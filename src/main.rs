static SOURCE: &'static str = include_str!("test.swift");
static RUNTIME: &'static str = include_str!("runtime.js");

use std::fmt::Write;

use colored::Colorize;
use deno_core::{serde_json, serde_v8, v8, JsRuntime, RuntimeOptions};
use tree_sitter::Parser;

#[allow(dead_code)]
fn log_node(node: &tree_sitter::Node, level: u32) {
    let content = node.utf8_text(SOURCE.as_bytes()).unwrap().blue();
    // log self
    println!(
        "{:indent$}{} --> {content}",
        "",
        node.kind(),
        indent = level as usize * 2
    );
    for i in 0..node.child_count() {
        log_node(&node.child(i).unwrap(), level + 1);
    }
}

#[derive(Debug, Default)]
struct State {
    in_call_expression: usize,
    cur_navigation_exp_name: Option<String>,

    temp_navigation_expression_level: usize,
    navigation_expression_level: Vec<usize>,

    rest_call_args: Vec<usize>,
}

impl State {
    fn new() -> Self {
        Self::default()
    }

    fn is_in_fn_call(&self) -> bool {
        self.in_call_expression > 0
    }
}

impl State {
    fn handle_node_cursor(&mut self, cursor: &mut tree_sitter::TreeCursor, out: &mut String) {
        let node = cursor.node();
        let node_source = node.utf8_text(SOURCE.as_bytes()).unwrap();
        let kind = node.kind();
        #[allow(unused_variables)]
        let tab = cursor.depth();
        let mut skip_child = false;

        // println!(
        //     "{:indent$}{} --> {}",
        //     "",
        //     kind,
        //     node_source.blue(),
        //     indent = tab as usize * 2
        // );

        match kind {
            "call_expression" => {
                self.in_call_expression += 1;
            }
            _ => {}
        }

        if self.is_in_fn_call() {
            match kind {
                "navigation_expression" => {
                    self.temp_navigation_expression_level += 1;
                }
                "simple_identifier" => {
                    let parent_kind = cursor.node().parent().unwrap().kind();

                    if parent_kind == "call_expression" {
                        out.push_str(format!("new {node_source}").as_str());
                        self.navigation_expression_level
                            .push(self.temp_navigation_expression_level);
                        self.temp_navigation_expression_level = 0;
                    }
                }
                "navigation_suffix" => {
                    out.write_str(node_source).unwrap();

                    // find a simple_identifier child
                    for i in 0..node.child_count() {
                        let child = node.child(i).unwrap();
                        if child.kind() == "simple_identifier" {
                            let child_source = child.utf8_text(SOURCE.as_bytes()).unwrap();
                            self.cur_navigation_exp_name = Some(child_source.to_string());
                            break;
                        }
                    }

                    return;
                }
                "value_arguments" => {
                    out.push_str("(");

                    let mut args_count = 0;
                    for i in 0..node.child_count() {
                        let child = node.child(i).unwrap();
                        if child.kind() == "value_argument" {
                            args_count += 1;
                        }
                    }
                    self.rest_call_args.push(args_count);
                }
                "value_argument" => {
                    let child = node.child(0).unwrap();
                    let child_source = child.utf8_text(SOURCE.as_bytes()).unwrap();

                    match child.kind() {
                        "line_string_literal" | "navigation_expression" | "integer_literal" => {
                            out.push_str(child_source);
                        }
                        "prefix_expression" => {
                            let call_name = self.cur_navigation_exp_name.as_ref().unwrap();
                            let full_name = format!("{}PreExp{}", call_name, child_source);
                            out.push_str(&full_name);
                        }
                        _ => {}
                    }

                    skip_child = true;
                }
                "lambda_literal" => {
                    let parent = cursor.node().parent().unwrap();
                    let parent_kind = parent.kind();

                    // its a swift trailing closure
                    if parent_kind == "call_suffix" {
                        if let Some(pre) = node.prev_sibling() {
                            if pre.kind() != "value_arguments" {
                                out.push_str("(");
                            }
                        } else {
                            out.push_str("(");
                        }
                    }

                    out.push_str("() => {");
                }
                _ => {}
            }
        }

        if !skip_child {
            // deepth-first
            if cursor.goto_first_child() {
                self.handle_node_cursor(cursor, out);
                while cursor.goto_next_sibling() {
                    self.handle_node_cursor(cursor, out);
                }
                cursor.goto_parent();
            }
        }

        match kind {
            "call_expression" => {
                self.in_call_expression -= 1;

                if self.navigation_expression_level.last().unwrap() == &0 {
                    out.push_str(";");
                    self.navigation_expression_level.pop();
                }
            }
            "navigation_expression" => {
                if self.temp_navigation_expression_level > 0 {
                    self.temp_navigation_expression_level -= 1;
                } else {
                    let mut last = self.navigation_expression_level.pop().unwrap();
                    last -= 1;
                    self.navigation_expression_level.push(last);
                }
            }
            "value_arguments" => {
                if let Some(next) = node.next_sibling() {
                    if next.kind() == "lambda_literal" {
                        // swift trailing closure
                        out.push_str(",");
                        return;
                    }
                }

                out.push_str(")");
            }
            "value_argument" => {
                let mut last = self.rest_call_args.pop().unwrap();
                last -= 1;

                if last > 0 {
                    out.push_str(",");
                    self.rest_call_args.push(last);
                }
            }
            "lambda_literal" => {
                out.push_str("}");

                let parent = cursor.node().parent().unwrap();
                let parent_kind = parent.kind();

                // its a swift trailing closure
                if parent_kind == "call_suffix" {
                    out.push_str(")");
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_swift::language())
        .expect("Error loading Rust grammar");

    let tree = parser.parse(SOURCE, None).unwrap();
    let root_node = tree.root_node().child(0).unwrap();

    let mut cursor = root_node.walk();
    let mut output = String::new();

    let mut state = State::new();
    state.handle_node_cursor(&mut cursor, &mut output);

    // write output to file
    // std::fs::write("output.js", output).expect("Unable to write file");

    let mid_code = output;

    let script = format!(
        r#"
        {RUNTIME}
        const root = {mid_code}

        root.render()
        "#
    );

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![],
        ..Default::default()
    });
    let global = runtime.execute_script("<generate>", script).unwrap();

    let scope = &mut runtime.handle_scope();
    let local = v8::Local::new(scope, global);
    // Deserialize a `v8` object into a Rust type using `serde_v8`,
    // in this case deserialize to a JSON `Value`.
    let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local);

    let result = deserialized_value.unwrap();
    let result = result.as_str().unwrap();

    // read template.html and replace <%SLOT%>
    let template = std::fs::read_to_string("template.html").unwrap();
    let result = template.replace("<%SLOT%>", result);

    // write to index.html
    std::fs::write("index.html", result).expect("Unable to write file");
}
