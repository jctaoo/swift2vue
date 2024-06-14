#[allow(unused_imports)]
use crate::utils::log_node_tree;

#[derive(Debug)]
enum ArgType {
    Label { label: String, value: String },
    Value { value: String },
    PrefixValue { value: String },

    Sub { code: String },
    LabelSub { label: String, code: String },
}

impl ArgType {
    fn is_label(&self) -> bool {
        match self {
            ArgType::Label { .. } => true,
            ArgType::LabelSub { .. } => true,
            _ => false,
        }
    }
}

fn handle_value_arg(node: &tree_sitter::Node, source: &String) -> ArgType {
    let first_child = node.child(0).unwrap();
    let node_code = first_child.utf8_text(source.as_bytes()).unwrap();

    match first_child.kind() {
        "prefix_expression" => {
            let name = first_child.child(1).unwrap();
            let name_code = name.utf8_text(source.as_bytes()).unwrap();

            ArgType::PrefixValue {
                value: name_code.to_string(),
            }
        }
        "call_expression" => {
            let code = callexp2object(&first_child, source);
            ArgType::Sub { code }
        }
        "value_argument_label" => {
            let label = node_code;
            let value = node.child(2).unwrap();

            if value.kind() == "call_expression" {
                let code = callexp2object(&value, source);
                return ArgType::LabelSub {
                    label: label.to_string(),
                    code,
                };
            }

            let value_code = value.utf8_text(source.as_bytes()).unwrap();

            ArgType::Label {
                label: label.to_string(),
                value: value_code.to_string(),
            }
        }
        _ => ArgType::Value {
            value: node_code.to_string(),
        },
    }
}

#[derive(Default, Debug)]
struct State {
    source: String,
    args: Vec<ArgType>,
    name: String,
}

impl State {
    fn collect(&mut self, node: &tree_sitter::Node) {
        let node_code = node.utf8_text(self.source.as_bytes()).unwrap();

        match node.kind() {
            "simple_identifier" => {
                let parent_is_call = node.parent().unwrap().kind() == "call_expression";
                if parent_is_call {
                    self.name = node_code.to_string();
                }
            }
            "value_argument" => {
                let arg = handle_value_arg(node, &self.source);
                self.args.push(arg);
                return;
            }
            _ => {}
        }

        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            self.collect(&child);
        }
    }

    fn generate(mut self) -> String {
        let name = format!("Swift{}", self.name);
        let mut out = format!("{}(", name);

        // sort args, all value args first, then label args
        self.args.sort_by(|a, b| {
            if a.is_label() && !b.is_label() {
                std::cmp::Ordering::Greater
            } else if !a.is_label() && b.is_label() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        let mut into_labels = false;

        for arg in self.args {
            match arg {
                ArgType::Value { value } => {
                    out.push_str(&format!("{}, ", value));
                }
                ArgType::PrefixValue { value } => {
                    out.push_str(&format!("{name}.{}, ", value));
                }
                ArgType::Label { label, value } => {
                    if !into_labels {
                        out.push_str("{");
                        into_labels = true;
                    }

                    out.push_str(&format!("{}: {}, ", label, value));
                }
                ArgType::Sub { code } => {
                    out.push_str(&format!("{}, ", code));
                }
                ArgType::LabelSub { label, code } => {
                    if !into_labels {
                        out.push_str("{");
                        into_labels = true;
                    }

                    out.push_str(&format!("{}: {}, ", label, code));
                }
            }
        }

        if into_labels {
            out.pop();
            out.pop();
            out.push_str("}");
        }

        out.push_str(")");
        out
    }
}

/// Convert `call_experssion` to js object literal
/// Color(.sRGB, red: 0.98, green: 0.9, blue: 0.2) -->
/// SwiftColor("sRGB", {red: 0.98, green: 0.9, blue: 0.2})
#[allow(dead_code)]
pub fn callexp2object(node: &tree_sitter::Node, source: &String) -> String {
    assert_eq!(node.kind(), "call_expression");
    let mut state = State::default();
    state.source = source.clone();
    state.collect(node);
    state.generate()
}

#[cfg(test)]
mod test {
    use tree_sitter::Parser;

    #[allow(unused_imports)]
    use crate::utils::log_node_tree;

    // const SOURCE1: &str = r#"Color(.sRGB, red: 0.98, green: 0.9, blue: 0.2)"#;
    // const SOURCE2: &str = r#"Color(red: 0.98, .sRGB, green: 0.9, blue: 0.2)"#;
    const SOURCE3: &str = r#"Color(red: 0.98, .sRGB, sub: Test(2, a: 123), Test(2, a: 123))"#;

    #[test]
    fn test_callexp2object() {
        let lang = tree_sitter_swift::language();

        let mut parser = Parser::new();
        parser
            .set_language(&lang)
            .expect("Error loading Rust grammar");

        let tree = parser.parse(SOURCE3, None).unwrap();
        let node = tree.root_node().child(0).unwrap();

        let result = super::callexp2object(&node, &SOURCE3.to_string());
        assert_eq!(result, "SwiftColor(SwiftColor.sRGB, SwiftTest(2, {a: 123}), {red: 0.98, sub: SwiftTest(2, {a: 123})})".to_string());
    }
}
