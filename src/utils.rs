use colored::Colorize;
use xmltree::{Element, EmitterConfig};

pub fn prettify_xml(xml: String) -> String {
  let el = Element::parse(xml.as_bytes()).expect("parsexml");
  let mut cfg = EmitterConfig::new();
  cfg.perform_indent = true;

  let mut out = Vec::new();
  let _ = el.write_with_config(&mut out, cfg);

  let output = String::from_utf8(out).unwrap();

  // return without first line
  output.lines().skip(1).collect::<Vec<&str>>().join("\n")
}

pub fn inline_str(s: &str) -> String {
  s.replace("\n", "")
}

#[allow(dead_code)]
pub fn log_node(node: &tree_sitter::Node, level: u32, source: &String) {
  let content = node.utf8_text(source.as_bytes()).unwrap();
  // content without new line
  let content = inline_str(content).blue();

  // log self
  println!(
      "{:indent$}- 节点类型: {} --> {content}",
      "",
      node.kind(),
      indent = level as usize * 3
  );

  // log all attributes
  println!(
      "{:indent$}  节点信息: {} id={}",
      "",
      format!("{:?}", node),
      node.id(),
      indent = level as usize * 3
  );

  // for i in 0..node.child_count() {
  //     log_node(&node.child(i).unwrap(), level + 1);
  // }
}

#[allow(dead_code)]
pub fn log_node_tree(node: &tree_sitter::Node, level: u32, source: &String) {
  let content = node.utf8_text(source.as_bytes()).unwrap();
  // content without new line
  let content = inline_str(content).blue();

  // log self
  println!(
      "{:indent$}- 节点类型: {} --> {content}",
      "",
      node.kind(),
      indent = level as usize * 3
  );

  // log all attributes
  println!(
      "{:indent$}  节点信息: {} id={}",
      "",
      format!("{:?}", node),
      node.id(),
      indent = level as usize * 3
  );

  for i in 0..node.child_count() {
    log_node_tree(&node.child(i).unwrap(), level + 1, source);
  }
}

pub fn find_first_node<'a>(node: tree_sitter::Node<'a>, kind: &str, source: &String) -> Option<tree_sitter::Node<'a>> {
  for i in 0..node.child_count() {
    let child = node.child(i).unwrap();
    if child.kind() == kind {
      return Some(child);
    }
    if let Some(r) = find_first_node(child, kind, source) {
      return Some(r);
    }
  }

  None
}

pub fn find_first_simple_identifier(node: &tree_sitter::Node, source: &String) -> Option<String> {
  for i in 0..node.child_count() {
    let child = node.child(i).unwrap();
    if child.kind() == "simple_identifier" {
      return Some(child.utf8_text(source.as_bytes()).unwrap().to_string());
    }
    return find_first_simple_identifier(&child, source);
  }

  None
}
