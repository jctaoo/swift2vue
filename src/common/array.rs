#[allow(unused_imports)]
use crate::utils::log_node_tree;

use super::object::callexp2object_with_context;

#[derive(Debug)]
enum ItemType {
    Literal { value: String },
    PrefixValue { value: String },
    Object { code: String },
    Sub { code: String },
}

#[derive(Default, Debug)]
struct State {
    source: String,
    context: String,
    obj_ctx: Vec<String>,
    args: Vec<ItemType>,

    in_root: bool,
}

impl State {
    fn collect(&mut self, node: &tree_sitter::Node) {
        let node_code = node.utf8_text(self.source.as_bytes()).unwrap();

        if !self.in_root {
            match node.kind() {
                "prefix_expression" => {
                    let name = node.child(1).unwrap();
                    let name_code = name.utf8_text(self.source.as_bytes()).unwrap();

                    self.args.push(ItemType::PrefixValue {
                        value: name_code.to_string(),
                    });
                    return;
                }
                "call_expression" => {
                    let code = callexp2object_with_context(&node, &self.source, self.obj_ctx.clone());
                    self.args.push(ItemType::Object { code });
                    return;
                }
                "array_literal" => {
                    let code = array2js_call(&node, &self.source, self.context.clone());
                    self.args.push(ItemType::Sub { code });
                    return;
                }
                _ => {
                    // TODO: 细致处理
                    if node.kind().ends_with("_literal") {
                        self.args.push(ItemType::Literal {
                            value: node_code.to_string(),
                        });
                    }
                }
            }
        } else {
            self.in_root = false;
        }

        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            self.collect(&child);
        }
    }

    fn generate(self) -> String {
        let mut out = format!("[");

        for arg in &self.args {
            match arg {
                ItemType::Literal { value } => {
                    out.push_str(&format!("{}, ", value));
                }
                ItemType::PrefixValue { value } => {
                    out.push_str(&format!("{}.{}, ", self.context, value));
                }
                ItemType::Object { code } => {
                    out.push_str(&format!("{}, ", code));
                }
                ItemType::Sub { code } => {
                    out.push_str(&format!("{}, ", code));
                }
            }
        }

        if self.args.len() > 0 {
            out.pop();
            out.pop();
        }

        out.push_str("]");
        out
    }
}

#[allow(dead_code)]
pub fn array2js_call(node: &tree_sitter::Node, source: &String, context: String) -> String {
    assert_eq!(node.kind(), "array_literal");
    let mut state = State::default();
    state.source = source.clone();
    state.context = context;
    state.in_root = true;
    state.collect(node);
    state.generate()
}

#[allow(dead_code)]
pub fn array2js_call_with_obj_context(node: &tree_sitter::Node, source: &String, context: String, obj_ctx: Vec<String>) -> String {
    assert_eq!(node.kind(), "array_literal");
    let mut state = State::default();
    state.source = source.clone();
    state.context = context;
    state.obj_ctx = obj_ctx;
    state.in_root = true;
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
    const SOURCE3: &str = r#"[.red, .blue, Color(red: 22)]"#;

    #[test]
    fn test_array2js_call() {
        let lang = tree_sitter_swift::language();

        let mut parser = Parser::new();
        parser
            .set_language(&lang)
            .expect("Error loading Rust grammar");

        let tree = parser.parse(SOURCE3, None).unwrap();
        let node = tree.root_node().child(0).unwrap();

        let result = super::array2js_call(&node, &SOURCE3.to_string(), "DatePicker".to_string());
        // println!("result: {:?}", result);
        assert_eq!(result, "[DatePicker.red, DatePicker.blue, SwiftColor({red: 22})]".to_string());
    }
}
