use std::collections::HashMap;

use id_tree::{InsertBehavior, NodeId, Tree, TreeBuilder};

use crate::common;
#[allow(unused)]
use crate::utils::log_node;
#[allow(unused)]
use crate::{
    paser::StructMember,
    utils::{log_node_tree, prettify_xml},
};

#[derive(Debug)]
pub struct ViewParseError {
    pub message: String,
    pub node_code: String,
    pub row: usize,
    pub col: usize,
}

impl ViewParseError {
    pub fn new(message: String, node: &tree_sitter::Node, source: &String) -> Self {
        let row = node.start_position().row;
        let col = node.start_position().column;
        let node_code = node.utf8_text(source.as_bytes()).unwrap().to_string();
        Self {
            message,
            node_code,
            row,
            col,
        }
    }
}

#[derive(Debug)]
struct ViewNode {
    tag: String,
    modifier: HashMap<String, String>,
    str_content: Option<String>,
}

impl ViewNode {
    fn new(tag: String) -> Self {
        Self {
            tag,
            modifier: HashMap::new(),
            str_content: None,
        }
    }

    fn attr_str(&self) -> String {
        let mut attr_str = String::new();

        for (key, value) in self.modifier.iter() {
            if value.is_empty() {
                attr_str.push_str(&format!("{} ", key));
            } else {
                attr_str.push_str(&format!("{}=\"{}\" ", key, value));
            }
        }

        if !self.modifier.is_empty() {
            attr_str.pop();
        }

        attr_str
    }
}

pub struct ViewParser<'a> {
    source: String,

    struct_info: crate::paser::StructInfo<'a>,
    view_tree: Tree<ViewNode>,

    in_call_expression: usize,

    /// related call suffix node
    navigation_expression_level: Vec<tree_sitter::Node<'a>>,
    navigation_component_node_id: Option<usize>,

    ignore_nodes: Vec<tree_sitter::Node<'a>>,
    id_to_tree_id: HashMap<usize, NodeId>,

    parent_node_id: Option<NodeId>,
}

impl<'a> ViewParser<'a> {
    pub fn from_struct(st: crate::paser::StructInfo<'a>, source: String) -> Self {
        Self {
            source,
            view_tree: TreeBuilder::new().build(),
            struct_info: st,
            in_call_expression: 0,
            navigation_expression_level: vec![],
            parent_node_id: None,
            ignore_nodes: vec![],
            id_to_tree_id: HashMap::new(),
            navigation_component_node_id: None,
        }
    }
}

impl<'a> ViewParser<'a> {

    pub fn generate_template(&mut self) -> String {
        if self.struct_info.inheritance != Some("View".to_string()) {
            return "".to_string();
        }

        if let Some(StructMember::Property {
            node: body,
            modifier: _,
        }) = self.struct_info.members.get("body")
        {
            let mut cursor = body.walk();
            self.handle_struct(&mut cursor);
        }

        let mut fmt_tree = String::new();
        self.view_tree.write_formatted(&mut fmt_tree).unwrap();
        println!("{}", fmt_tree);

        return self.generate_code_from_tree();
    }

    /// 处理函数调用的转换
    fn handle_fn(&self, node: &tree_sitter::Node) -> String {
        let mut code = String::new();
        assert_eq!(node.kind(), "statements");

        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            let child_code = child.utf8_text(self.source.as_bytes()).unwrap();

            match child.kind() {
                "assignment" => {
                    let target = child.child(0).unwrap();
                    let target = target.utf8_text(self.source.as_bytes()).unwrap();
                    if let Some(StructMember::Property { node: _, modifier }) =
                        self.struct_info.members.get(target)
                    {
                        // 处理 vue3 ref
                        if modifier == &Some("State".to_string()) {
                            let target = format!("{}.value", target);
                            let op = child.child(1).unwrap();
                            let op = op.utf8_text(self.source.as_bytes()).unwrap();
                            let value = child.child(2).unwrap();
                            let value = value.utf8_text(self.source.as_bytes()).unwrap();

                            code.push_str(format!("{} {op} {};\n", target, value).as_str());
                        }
                    } else {
                        code.push_str(child_code);
                        code.push_str("\n");
                    }
                }
                _ => {
                    // TODO: avoid hardcode
                    // [Item(title: "A"), Item(title: "B")] --> [{ title: "A" }, { title: "B" }]
                    // using regex
                    let regex = regex::Regex::new(r#"(\w+)\(([^()]+:[^)]+)\)"#).unwrap();
                    let child_code = regex.replace_all(&child_code, r#"{ $2 }"#).to_string();

                    code.push_str(&child_code);
                    code.push_str("\n");
                }
            }
        }

        // TODO:
        // handle swift interpolated_expression like "Hello, \(name)!"
        // try handle this using regex

        code.to_string()
    }

    /// 处理属性 = 符号的右边
    fn handle_member_expression(&self, node: &tree_sitter::Node) -> String {
        // TODO: avoid hardcode
        if node.kind() == "array_literal" {
            // TODO: 根据类型标注获得前缀表达式 context
            let names = self.struct_info.sub_names();
            return common::array::array2js_call_with_obj_context(node, &self.source, "".to_string(), names);
        }

        if node.kind() == "call_expression" {
            let names = self.struct_info.sub_names();
            return common::object::callexp2object_with_context(node, &self.source, names);
        }

        return node.utf8_text(self.source.as_bytes()).unwrap().to_string();
    }

    fn generate_setup_code(&self, runtimes: Vec<String>) -> Result<String, ViewParseError> {
        if self.struct_info.inheritance != Some("View".to_string()) {
            return Ok("".to_string());
        }

        let mut setup_code = String::new();
        let mut exported_identifier: Vec<String> = vec![];

        for (key, value) in self.struct_info.members.iter() {
            if key.as_str() == "body" {
                continue;
            }

            let member_code = match value {
                StructMember::Property { node, modifier } => {
                    let var_name = key;
                    exported_identifier.push(var_name.clone());

                    let ref_literal = ["line_string_literal", "integer_literal"];

                    if modifier == &Some("State".to_string()) {
                        if ref_literal.contains(&node.kind()) {
                            let var_code = node.utf8_text(self.source.as_bytes()).unwrap();
                            format!(
                                "const {var_name} = ref({var_code});",
                                var_name = var_name,
                                var_code = var_code
                            )
                        } else {
                            let var_code = self.handle_member_expression(node);

                            let directive = if node.kind() == "array_literal" {
                                "reactive"
                            } else {
                                "ref"
                            };

                            format!(
                                // TODO: when to use ref? v-model not work with reactive
                                "const {var_name} = {directive}({var_code});",
                                var_name = var_name,
                                var_code = var_code
                            )
                        }
                    } else {
                        if node.kind() == "call_expression" {
                            if let Some(first_child) = node.child(0) {
                                if first_child.kind() == "lambda_literal" {
                                    let parent_node = node.parent().unwrap();
                                    let error = ViewParseError::new(
                                        "目前不支持闭包属性".to_string(),
                                        &parent_node,
                                        &self.source,
                                    );
                                    return Err(error);
                                }
                            }
                        }

                        let var_code = node.utf8_text(self.source.as_bytes()).unwrap();
                        format!(
                            "const {var_name} = {var_code};",
                            var_name = var_name,
                            var_code = var_code
                        )
                    }
                }
                StructMember::Function(node) => {
                    // log_node_tree(&node, 0);
                    let fn_name = key;
                    exported_identifier.push(fn_name.clone());

                    let fn_code = self.handle_fn(node);

                    let code_with_indent = fn_code
                        .split("\n")
                        .map(|x| x.trim())
                        .filter(|x| x.len() > 0)
                        .map(|line| format!("{:indent$}{}", "", line, indent = 12))
                        .collect::<Vec<String>>()
                        .join("\n");

                    format!(
                        "const {fn_name} = () => {{\n{code_with_indent}\n{:indent$}}};",
                        "",
                        indent = 8
                    )
                    .trim_end()
                    .to_string()
                }
            };

            // push indent
            setup_code.push_str(format!("{:indent$}", "", indent = 8).as_str());
            setup_code.push_str(&member_code);
            setup_code.push_str("\n");
        }

        exported_identifier.extend(runtimes);

        let defs = setup_code.trim_end().to_string();
        let exported = exported_identifier.join(", ");
        let exported_code = format!("{:indent$}return {{ {} }};", "", exported, indent = 8);

        Ok(format!("{}\n{}", defs, exported_code))
    }

    fn generate_sub_struct(&self) -> String {
        let mut out = String::new();

        for sub in self.struct_info.sub.iter() {
            let code = common::model::date_model2js_fn(&sub.borrow().node, &self.source);
            out.push_str(&code);
            out.push_str("\n");
        }

        out.trim().to_string()
    }

    /// It's usually used for error component
    pub fn generate_empty_component(self) -> String {
        if self.struct_info.inheritance != Some("View".to_string()) {
            return "".to_string();
        }

        let template_code = "<div></div>";

        format!(
            r#"
export default {{
    setup() {{
    }},
    template: `{template_code}`
}}
            "#
        ).trim().to_string()
    }

    pub fn generate_component_code(&mut self, builtin: Vec<String>, views: Vec<String>) -> Result<String, ViewParseError> {
        if self.struct_info.inheritance != Some("View".to_string()) {
            return Ok("".to_string());
        }

        let template_code = self.generate_template().replace("\n", "");
        let setup_code = self.generate_setup_code(builtin.clone())?;

        // example: my-component.js
        // import { ref } from 'vue'
        // export default {
        //   setup() {
        //     const count = ref(0)
        //     return { count }
        //   },
        //   template: `<div>Count is: {{ count }}</div>`
        // }

        let builtin_imports = builtin
            .iter()
            .map(|name| format!("import {} from './{}.js'", name, name))
            .collect::<Vec<String>>()
            .join("\n");
        let view_imports = views
            .iter()
            .map(|name| format!("import {} from './{}.js'", name, name))
            .collect::<Vec<String>>()
            .join("\n");
        let components = views.join(", ");

        let sub_struct_code = self.generate_sub_struct();

        Ok(format!(
            r#"
{builtin_imports}
{view_imports}

{sub_struct_code}

export default {{
    components: {{
        {components}
    }},
    setup() {{
{setup_code}
    }},
    template: `{template_code}`
}}
        "#
        )
        .trim()
        .to_string())
    }

    /// generate html code from view tree
    fn generate_code_from_tree(&self) -> String {
        let mut code = String::new();

        let root_id = self.view_tree.root_node_id().unwrap();
        self.handle_view_tree_node(root_id, &mut code);

        // prettify_xml(code)
        code
    }

    fn handle_view_tree_node(&self, id: &NodeId, code: &mut String) {
        let node = self.view_tree.get(id).unwrap();
        let view_node = node.data();

        // handle node pre
        code.push_str(&format!("<{}", view_node.tag));
        if view_node.modifier.len() > 0 {
            code.push_str(" ");
            code.push_str(&view_node.attr_str());
        }
        code.push_str(">");

        if let Some(str_child) = &view_node.str_content {
            code.push_str(str_child);
        } else {
            for child in self.view_tree.children_ids(id).unwrap() {
                self.handle_view_tree_node(&child, code);
            }
        }

        // handle node post
        code.push_str(&format!("</{}>\n", view_node.tag));
    }

    fn insert_view_node(&mut self, view_node: ViewNode) -> NodeId {
        let node_id: NodeId = match self.parent_node_id.as_ref() {
            Some(node_id) => self.view_tree.insert(
                id_tree::Node::new(view_node),
                InsertBehavior::UnderNode(&node_id),
            ),
            None => self
                .view_tree
                .insert(id_tree::Node::new(view_node), InsertBehavior::AsRoot),
        }
        .unwrap();
        self.parent_node_id = Some(node_id.clone());
        node_id
    }

    fn post_insert_view_node(&mut self) {
        if let Some(parent_node) = self.parent_node_id.as_ref() {
            if let Some(parent) = self.view_tree.ancestor_ids(parent_node).unwrap().next() {
                self.parent_node_id = Some(parent.clone());
            } else {
                self.parent_node_id = None;
            }
        }
    }

    fn extract_view_tag(&self, node: &tree_sitter::Node) -> Option<String> {
        assert_eq!(node.kind(), "call_expression");
        let child = node.child(0).unwrap();

        match child.kind() {
            "simple_identifier" => {
                let identifier_text = child.utf8_text(self.source.as_bytes()).unwrap();
                Some(identifier_text.to_string())
            }
            _ => None,
        }
    }

    fn handle_node(&mut self, cursor: &mut tree_sitter::TreeCursor<'a>) -> bool {
        let node = cursor.node();

        if node.kind() == "comment" {
            return false;
        }

        if node.kind() == "call_expression" {
            self.in_call_expression += 1;

            if let Some(tag) = self.extract_view_tag(&node) {
                let mut view_node = ViewNode::new(tag.clone());

                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind() == "call_suffix" {
                        if child.child(0).unwrap().kind() == "lambda_literal" {
                            continue;
                        }

                        for j in 0..child.child_count() {
                            let call_suffix_child = child.child(j).unwrap();
                            if call_suffix_child.kind() == "value_arguments" {
                                for k in 0..(call_suffix_child.child_count() - 2) {
                                    let arg_node = call_suffix_child.child(k + 1).unwrap();

                                    if arg_node.kind() == "," {
                                        continue;
                                    }

                                    if let Some((key, value)) = crate::component::compute_modifier(
                                        tag.clone(),
                                        &arg_node,
                                        &self.source,
                                    ) {
                                        // println!("{}: {}", key, value);
                                        if key.as_str() == "child" {
                                            view_node.str_content = Some(value);
                                        } else {
                                            view_node.modifier.insert(key, value);
                                        }
                                    }
                                    // log_node_tree(&arg_node, 0);
                                }

                                break;
                            }
                        }
                    }
                }
                // println!("{}", "====================".on_yellow());

                let insert_id = self.insert_view_node(view_node);
                self.id_to_tree_id.insert(node.id(), insert_id);

                if node.parent().unwrap().kind() == "navigation_expression" {
                    self.navigation_component_node_id = Some(node.id());
                }
            }
        }

        if node.kind() == "navigation_expression"
            && node.parent().unwrap().kind() == "call_expression"
        {
            let related_call_suffix = node.next_sibling().unwrap();
            assert_eq!(related_call_suffix.kind(), "call_suffix");
            self.navigation_expression_level.push(related_call_suffix);
        }

        if node.kind() == "navigation_suffix"
            && node.parent().unwrap().parent().unwrap().kind() == "call_expression"
        {
            let last_navigation = self.navigation_expression_level.pop().unwrap();
            self.ignore_nodes.push(last_navigation);

            let call_suffix_identifier = node.child(1).unwrap();
            let mut call_suffix_name = call_suffix_identifier
                .utf8_text(self.source.as_bytes())
                .unwrap();

            let args_node = last_navigation.child(0).unwrap();

            // TODO: only support one arg for now
            // TODO: no handling of lambda_literal for now, it's usually for children
            let arg_node = if args_node.child_count() > 2 && args_node.kind() != "lambda_literal" {
                let node = args_node.child(1).unwrap();
                let value_node = node.child(0).unwrap();
                if node.child(0).unwrap().kind() == "value_argument_label" {
                    Some(value_node.next_sibling().unwrap().next_sibling().unwrap())
                } else {
                    Some(value_node)
                }
            } else {
                None
            };

            let arg_value = if let Some(arg_node) = arg_node {
                if arg_node.kind() == "prefix_expression" {
                    arg_node
                        .child(1)
                        .unwrap()
                        .utf8_text(self.source.as_bytes())
                        .unwrap()
                } else {
                    arg_node.utf8_text(self.source.as_bytes()).unwrap()
                }
            } else {
                ""
            };

            // TODO: avoid hardcode
            if call_suffix_name == "onTapGesture" {
                call_suffix_name = "@click";
            }

            // TODO: ignroe contextMenu for now
            if call_suffix_name != "contextMenu" {
                let related_call_exp = self.navigation_component_node_id.unwrap();
                let related_tree_id = self.id_to_tree_id.get(&related_call_exp).unwrap();
                let realted_view_node = self.view_tree.get_mut(related_tree_id).unwrap();
                realted_view_node
                    .data_mut()
                    .modifier
                    .insert(call_suffix_name.to_string(), arg_value.to_string());
            }
        }

        if node.kind() == "call_suffix" {
            if self.ignore_nodes.contains(&node) {
                return false;
            }
        }

        // log_node(&node, cursor.depth());

        true
    }

    fn handle_node_post(&mut self, cursor: &mut tree_sitter::TreeCursor<'a>) {
        let node = cursor.node();

        if node.kind() == "call_expression" {
            self.in_call_expression -= 1;

            if let Some(_) = self.extract_view_tag(&node) {
                self.post_insert_view_node();
            }
        }
    }

    fn handle_struct(&mut self, cursor: &mut tree_sitter::TreeCursor<'a>) {
        let go_on = self.handle_node(cursor);

        if go_on {
            if cursor.goto_first_child() {
                self.handle_struct(cursor);
                while cursor.goto_next_sibling() {
                    self.handle_struct(cursor);
                }
                cursor.goto_parent();
            }
        }

        self.handle_node_post(cursor);
    }
}
