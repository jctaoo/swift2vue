#![allow(unused_imports)]
use crate::utils::{find_first_simple_identifier, log_node_tree};

///! Note that child is a special key, means child str content instead of modifier

fn compute_line_string_literal_for_str_child(node: &tree_sitter::Node, source: &String) -> String {
    let mut out = String::new();

    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        let content = child.utf8_text(source.as_bytes()).unwrap();

        match child.kind() {
            // "\"" => {
            //     out.push_str(content);
            // },
            "\\(" => {
                out.push_str("{{");
            }
            ")" => {
                out.push_str("}}");
            }
            "interpolated_expression" => {
                out.push_str(content);
            }
            "line_str_text" => {
                out.push_str(content);
            }
            _ => {}
        }
    }

    out
}

fn compute_text(node: &tree_sitter::Node, source: &String) -> Option<(String, String)> {
    let arg_node = node.child(0).unwrap();
    if arg_node.kind() == "line_string_literal" {
        let content = compute_line_string_literal_for_str_child(&arg_node, source);
        return Some(("child".to_string(), content));
    };

    if arg_node.kind() == "navigation_expression" || arg_node.kind() == "simple_identifier" {
        // TODO: 是否正确，这里是为了 for each 正常工作
        let content = arg_node.utf8_text(source.as_bytes()).unwrap().to_string();
        let code = format!("{{{{ {} }}}}", content);
        return Some(("child".to_string(), code));
    }

    log_node_tree(node, 0, source);

    None
}

fn compute_button(node: &tree_sitter::Node, source: &String) -> Option<(String, String)> {
    let arg_node = node.child(0).unwrap();
    if arg_node.kind() == "line_string_literal" {
        let content = compute_line_string_literal_for_str_child(&arg_node, source);
        return Some(("child".to_string(), content));
    };

    None
}

fn compute_foreach(node: &tree_sitter::Node, source: &String) -> Option<(String, String)> {
    let arg_node = node.child(0).unwrap();
    if arg_node.kind() == "simple_identifier" {
        let arg_node_code = arg_node.utf8_text(source.as_bytes()).unwrap();

        // TODO: 这里没有考虑不使用尾随闭包的情况
        // find foreach lambda
        let lambda_node = node.parent().unwrap().next_sibling().unwrap();
        assert_eq!(lambda_node.kind(), "lambda_literal");

        // TODO: 没有考虑使用 $0.xxx 的情况
        let item_name = lambda_node.child(1).unwrap();
        assert_eq!(item_name.kind(), "lambda_function_type");

        let item_name =
            find_first_simple_identifier(&item_name, source).expect("expect a item name");
        let v_for_code = format!("{} in {}", item_name, arg_node_code);

        return Some(("v-for".to_string(), v_for_code));
    };

    None
}

fn compute_color_picker(node: &tree_sitter::Node, source: &String) -> Option<(String, String)> {
    let arg_node = node.child(0).unwrap();
    if arg_node.kind() == "line_string_literal" {
        let content = compute_line_string_literal_for_str_child(&arg_node, source);
        return Some(("child".to_string(), content));
    };

    // TODO: go to common
    if arg_node.kind() == "value_argument_label" {
        let value_node = node.child(2).unwrap();
        let arg_content = arg_node.utf8_text(source.as_bytes()).unwrap().to_string();
        let value_content = value_node.utf8_text(source.as_bytes()).unwrap().to_string();

        if arg_content == "supportsOpacity" {
            let arg_content = format!("v-bind:{}", arg_content);
            return Some((arg_content, value_content));
        }
    }

    log_node_tree(node, 0, source);

    None
}


fn common_compute(node: &tree_sitter::Node, source: &String) -> Option<(String, String)> {
    let arg_node = node.child(0).unwrap();
    if arg_node.kind() == "value_argument_label" {
        let value_node = node.child(2).unwrap();
        let arg_content = arg_node.utf8_text(source.as_bytes()).unwrap().to_string();


        if value_node.kind() == "boolean_literal" {
            let value_content = value_node.utf8_text(source.as_bytes()).unwrap().to_string();

            if value_content == "true" {
                return Some((arg_content, "".to_string()));
            } else {
                return None;
            }
        } else if value_node.kind().ends_with("_literal") {
            let value_content = value_node.utf8_text(source.as_bytes()).unwrap().to_string();
            return Some((arg_content, value_content));
        } else if value_node.kind() == "simple_identifier" {
            let value_content = value_node.utf8_text(source.as_bytes()).unwrap().to_string();

            if value_content.starts_with("$") {
                let name_without_prefix = value_content.trim_start_matches("$");
                let arg_content = format!("v-model:{}", arg_content);
                return Some((arg_content, name_without_prefix.to_string()));
            }

            let arg_content = format!("v-bind:{}", arg_content);
            return Some((arg_content, value_content));
        } else {
            let value_content = value_node.utf8_text(source.as_bytes()).unwrap().to_string();
            let arg_content = format!("v-bind:{}", arg_content);
            return Some((arg_content, value_content));
        }
    };

    None
}

pub fn compute_modifier(
    tag: String,
    node: &tree_sitter::Node,
    source: &String,
) -> Option<(String, String)> {
    let res = match tag.as_str() {
        "Text" => compute_text(node, source),
        "Button" => compute_button(node, source),
        "ForEach" => compute_foreach(node, source),
        "ColorPicker" => compute_color_picker(node, source),
        _ => None,
    };

    if res.is_none() {
        common_compute(node, source)
    } else {
        res
    }
}
