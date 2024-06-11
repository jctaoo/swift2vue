#![allow(unused_imports)]
use crate::{paser::SOURCE, utils::log_node_tree};

///! Note that child is a special key, means child str content instead of modifier

fn compute_line_string_literal_for_str_child(node: &tree_sitter::Node) -> String {
  let mut out = String::new();

  for i in 0..node.child_count() {
    let child = node.child(i).unwrap();
    let content = child.utf8_text(SOURCE.as_bytes()).unwrap();

    match child.kind() {
        // "\"" => {
        //     out.push_str(content);
        // },
        "\\(" => {
            out.push_str("{{");
        },
        ")" => {
            out.push_str("}}");
        },
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

fn compute_text(node: &tree_sitter::Node) -> Option<(String, String)> {
  let arg_node = node.child(0).unwrap();
  if arg_node.kind() == "line_string_literal" {
    let content = compute_line_string_literal_for_str_child(&arg_node);
    return Some(("child".to_string(), content));
  };

  None
}

fn compute_button(node: &tree_sitter::Node) -> Option<(String, String)> {

  let arg_node = node.child(0).unwrap();
  if arg_node.kind() == "line_string_literal" {
    let content = compute_line_string_literal_for_str_child(&arg_node);
    return Some(("child".to_string(), content));
  };

  None
}

fn common_compute(node: &tree_sitter::Node) -> Option<(String, String)> {
  let arg_node = node.child(0).unwrap();
  if arg_node.kind() == "value_argument_label" {
    let value_node = node.child(2).unwrap();
    let arg_content = arg_node.utf8_text(SOURCE.as_bytes()).unwrap().to_string();

    if arg_node.kind().ends_with("_literal") {
      let value_content = value_node.utf8_text(SOURCE.as_bytes()).unwrap().to_string();
      return Some((arg_content, value_content));
    } else {
      let value_content = value_node.utf8_text(SOURCE.as_bytes()).unwrap().to_string();
      let arg_content = format!("v-bind:{}", arg_content);
      return Some((arg_content, value_content));
    }
  };

  None
}

pub fn compute_modifier(tag: String, node: &tree_sitter::Node) -> Option<(String, String)> {
  let res = match tag.as_str() {
    "Text" => {
      compute_text(node)
    },
    "Button" => {
      compute_button(node)
    },
    _ => {
      None
    }
  };

  if res.is_none() {
    common_compute(node)
  } else {
    res
  }
}
