use paser::{State, SOURCE};

mod paser;
mod view;
mod utils;
mod component;

fn main() {
    use tree_sitter::Parser;

    let lang = tree_sitter_swift::language();

    let mut parser = Parser::new();
    parser
        .set_language(&lang)
        .expect("Error loading Rust grammar");

    let tree = parser.parse(SOURCE, None).unwrap();
    let root_node = tree.root_node();

    // log_node(&root_node, 0);

    let mut cursor = root_node.walk();

    let mut state = State::new();
    state.handle_source(&mut cursor);

    for st in state.struct_list {
        // log current struct
        let st = st.borrow().clone();
        println!("got struct info: {:?}", st);

        let view = view::ViewParser::from_struct(st);
        let cmp_code = view.generate_component_code();
        println!("{}", cmp_code);
    }


    // println!("index.html generated");

    // state.handle_node_cursor(&mut cursor, &mut output);

    // write output to file
    // std::fs::write("output.js", output).expect("Unable to write file");

    // let mid_code = output;
    // // println!("{}", mid_code);

    // let script = format!(
    //     r#"
    //     {RUNTIME}
    //     const root = {mid_code}

    //     const out = {{
    //         render: root.render(),
    //         script: root.rendererScript()
    //     }};

    //     out
    //     "#
    // );

    // write out.js
    // std::fs::write("out.js", &script).expect("Unable to write file");

    // let mut runtime = JsRuntime::new(RuntimeOptions {
    //     extensions: vec![],
    //     ..Default::default()
    // });
    // let global = runtime.execute_script("<generate>", script).unwrap();

    // let scope = &mut runtime.handle_scope();
    // let local = v8::Local::new(scope, global);
    // // Deserialize a `v8` object into a Rust type using `serde_v8`,
    // // in this case deserialize to a JSON `Value`.
    // let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local);

    // let result = deserialized_value.unwrap();
    // let render_str = result["render"].as_str().unwrap();
    // let script_str = result["script"].as_str().unwrap();

    // // read template.html and replace <%SLOT%>
    // let template = std::fs::read_to_string("template.html").unwrap();
    // let result = template.replace("<%SLOT%>", render_str).replace("<%SCRIPT%>", script_str);

    // // write to index.html
    // std::fs::write("index.html", result).expect("Unable to write file");
    // println!("Generated index.html")
}
