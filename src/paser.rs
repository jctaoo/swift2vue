use std::{cell::RefCell, collections::HashMap, rc::Rc};

use tree_sitter::Node;

use crate::utils::log_node;
#[allow(unused)]
use crate::utils::log_node_tree;

#[derive(Debug, Clone)]
pub enum StructMember<'a> {
    Function(Node<'a>),
    Property {
        node: Node<'a>,
        modifier: Option<String>,
    },
}

#[derive(Debug, Default, Clone)]
pub struct StructInfo<'a> {
    pub name: String,
    pub members: HashMap<String, StructMember<'a>>,
    pub inheritance: Option<String>,
}

#[derive(Debug, Default)]
pub struct State<'a> {
    source: String,
    struct_def_level: usize,
    pub struct_list: Vec<Rc<RefCell<StructInfo<'a>>>>,
    current_struct: Option<Rc<RefCell<StructInfo<'a>>>>,
    pub verbose: bool,
}

impl<'a> State<'a> {
    pub fn new(source: String, verbose: bool) -> Self {
        let mut instance = Self::default();
        instance.source = source;
        instance.verbose = verbose;
        instance
    }
}

impl<'a> State<'a> {
    fn handle_struct_nodes(&mut self, cursor: &mut tree_sitter::TreeCursor<'a>) -> bool {
        let node = cursor.node();

        let mut struct_info = self.current_struct.as_ref().unwrap().borrow_mut();

        if struct_info.name.is_empty() {
            if node.kind() == "type_identifier" {
                let name = node.utf8_text(self.source.as_bytes()).unwrap();
                struct_info.name = name.to_string();
                return false;
            }
        }

        if struct_info.inheritance.is_none() {
            if node.kind() == "inheritance_specifier" {
                let inheritance = node.utf8_text(self.source.as_bytes()).unwrap();
                struct_info.inheritance = Some(inheritance.to_string());
                return false;
            }
        }

        if node.kind() == "property_declaration" {
            // log_node_tree(&node, 0);

            let mut name = String::new();
            let mut var_node: Option<Node> = None;
            let mut modifier: Option<String> = None;

            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "pattern" {
                    let idnode = child.child(0).unwrap();
                    if idnode.kind() == "simple_identifier" {
                        name = idnode
                            .utf8_text(self.source.as_bytes())
                            .unwrap()
                            .to_string();
                    }
                } else if child.kind() == "computed_property" {
                    let call_node = child.child(1).unwrap().child(0).unwrap();
                    if call_node.kind() == "call_expression" {
                        var_node = Some(call_node);
                    }
                } else if child
                    .prev_sibling()
                    .map(|x| x.kind() == "=")
                    .unwrap_or(false)
                {
                    let call_node = child;
                    var_node = Some(call_node);
                } else if child.kind() == "modifiers" {
                    // TODO: we only support one attribute for now
                    let attribute_node = child.child(0).unwrap();
                    if attribute_node.kind() == "attribute" {
                        let modifier_node = attribute_node.child(1).unwrap();
                        modifier = Some(
                            modifier_node
                                .utf8_text(self.source.as_bytes())
                                .unwrap()
                                .to_string(),
                        );
                    }
                } else {
                    continue;
                }
            }

            if let Some(var_node) = var_node {
                let var = StructMember::Property {
                    node: var_node,
                    modifier,
                };
                struct_info.members.insert(name, var);
            }

            return false;
        }

        if node.kind() == "function_declaration" {
            let mut name = String::new();
            let mut fn_node: Option<Node> = None;

            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "simple_identifier" {
                    name = child.utf8_text(self.source.as_bytes()).unwrap().to_string();
                } else if child.kind() == "function_body" {
                    fn_node = Some(child.child(1).unwrap());
                } else {
                    continue;
                }
            }

            if let Some(fn_node) = fn_node {
                struct_info
                    .members
                    .insert(name, StructMember::Function(fn_node));
            }

            return false;
        }

        // log_node(&node, cursor.depth() as u32);

        return true;
    }

    fn handle_node(&mut self, cursor: &mut tree_sitter::TreeCursor<'a>) -> bool {
        let node = cursor.node();
        if self.verbose {
            log_node(&node, cursor.depth(), &self.source);
        }

        if self.struct_def_level > 0 {
            return self.handle_struct_nodes(cursor);
        }

        if node.kind() == "class_declaration" {
            self.struct_def_level += 1;

            let struct_info = StructInfo::default();
            self.struct_list.push(Rc::new(RefCell::new(struct_info)));
            self.current_struct = self.struct_list.last().cloned();

            return self.handle_struct_nodes(cursor);
        }

        return true;
    }

    fn handle_node_post(&mut self, cursor: &mut tree_sitter::TreeCursor) {
        let node = cursor.node();

        if node.kind() == "class_declaration" {
            self.struct_def_level -= 1;

            if self.struct_def_level == 0 {
                self.current_struct = None;
            } else {
                self.current_struct = self.struct_list.first().cloned();
            }
        }
    }

    pub fn handle_source(&mut self, cursor: &mut tree_sitter::TreeCursor<'a>) {
        let go_on = self.handle_node(cursor);

        if go_on {
            if cursor.goto_first_child() {
                self.handle_source(cursor);
                while cursor.goto_next_sibling() {
                    self.handle_source(cursor);
                }
                cursor.goto_parent();
            }
        }

        self.handle_node_post(cursor);
    }
}
