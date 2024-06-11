use crate::paser::SOURCE;

///! Note that child is a special key, means child str content instead of modifier

fn compute_text(node: &tree_sitter::Node) -> Option<(String, String)> {
  let arg_node = node.child(0).unwrap();
  if arg_node.kind() == "line_string_literal" {
    let content_node = arg_node.child(1).unwrap();
    assert_eq!(content_node.kind(), "line_str_text");
    let content = content_node.utf8_text(SOURCE.as_bytes()).unwrap().to_string();
    return Some(("child".to_string(), content));
  };

  None
}

fn compute_button(node: &tree_sitter::Node) -> Option<(String, String)> {
  let arg_node = node.child(0).unwrap();
  if arg_node.kind() == "line_string_literal" {
    let content_node = arg_node.child(1).unwrap();
    assert_eq!(content_node.kind(), "line_str_text");
    let content = content_node.utf8_text(SOURCE.as_bytes()).unwrap().to_string();
    return Some(("child".to_string(), content));
  };

  None
}

fn common_compute(node: &tree_sitter::Node) -> Option<(String, String)> {
  let arg_node = node.child(0).unwrap();
  if arg_node.kind() == "value_argument_label" {
    let value_node = node.child(2).unwrap();
    let value_content = value_node.utf8_text(SOURCE.as_bytes()).unwrap().to_string();
    let arg_content = arg_node.utf8_text(SOURCE.as_bytes()).unwrap().to_string();

    return Some((arg_content, value_content));
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
