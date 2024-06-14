#[allow(unused_imports)]
use crate::utils::log_node_tree;
use crate::utils::{find_first_node, log_node};

use super::object::callexp2object;

#[derive(Debug)]
struct Var {
    name: String,
    default: Option<String>,
}

#[derive(Default, Debug)]
struct State {
    name: String,
    source: String,
    vars: Vec<Var>,
}

impl State {
    fn collect(&mut self, node: &tree_sitter::Node) {
        let node_code = node.utf8_text(self.source.as_bytes()).unwrap();

        match node.kind() {
            "type_identifier" => {
              if self.name.is_empty() {
                self.name = node_code.to_string();
              }
            }
            "property_declaration" => {
                let name_node = find_first_node(*node, "pattern", &self.source).unwrap();
                let name_code = name_node.utf8_text(self.source.as_bytes()).unwrap();

                if let Some(eq_node) = find_first_node(*node, "=", &self.source) {
                    let default_node = eq_node.next_sibling().unwrap();
                    let default_code = default_node.utf8_text(self.source.as_bytes()).unwrap();

                    self.vars.push(Var {
                        name: name_code.to_string(),
                        default: Some(default_code.to_string()),
                    });
                } else {
                    self.vars.push(Var {
                        name: name_code.to_string(),
                        default: None,
                    });
                }
                return;
            }
            _ => {}
        }

        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            self.collect(&child);
        }
    }

    fn generate(self) -> String {
        let mut out = format!("function {}(arg) {{\n", self.name);
        out.push_str("    const { ");
        for var in &self.vars {
            out.push_str(&format!("{}, ", var.name));
        }
        out.push_str("} = arg ?? {};\n");

        out.push_str(format!("{:indent$}return {{\n", "", indent = 4).as_str());
        for var in &self.vars {
            out.push_str(&format!("{:indent$}{}: {} ?? {},\n", "", var.name, var.name, var.default.as_ref().unwrap_or(&"false".to_string()), indent = 8));
        }
        out.push_str(format!("{:indent$}}};\n", "", indent = 4).as_str());
        out.push_str("}\n");
        out
    }
}

#[allow(dead_code)]
pub fn date_model2js_fn(node: &tree_sitter::Node, source: &String) -> String {
    assert_eq!(node.kind(), "class_declaration");
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
    const SOURCE3: &str = r#"
    struct ToggleStates {
      var oneIsOn: Bool = false
      var twoIsOn: Bool = true
    }
    "#;

    // will convert to
    // function ToggleStates(arg) {
    //     const { oneIsOn, twoIsOn } = arg ?? {};
    //     return {
    //         oneIsOn: oneIsOn || false,
    //         twoIsOn: twoIsOn || true,
    //     };
    // }

    #[test]
    fn test_array2js_call() {
        let lang = tree_sitter_swift::language();

        let mut parser = Parser::new();
        parser
            .set_language(&lang)
            .expect("Error loading Rust grammar");

        let tree = parser.parse(SOURCE3, None).unwrap();
        let node = tree.root_node().child(0).unwrap();

        // log_node_tree(&node, 0, &SOURCE3.to_string());

        let result = super::date_model2js_fn(&node, &SOURCE3.to_string());
        println!("result: {}", result);
        // assert_eq!(result, "[DatePicker.red, DatePicker.blue, SwiftColor({red: 22})]".to_string());
    }
}
