use paser::{State, SOURCE};

mod component;
mod paser;
mod template;
mod utils;
mod view;

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

    // using ./output
    let out_dir = std::path::Path::new("./output");

    // if the dir is not empty, remove all files
    if out_dir.exists() {
        let _ = std::fs::remove_dir_all(out_dir);
    }

    // create the dir
    std::fs::create_dir(out_dir).unwrap();

    let mut view_imports = Vec::new();
    let mut builtin_imports = Vec::new();
    let mut index_template = String::new();

    // handle runtime
    let runtime_dir = std::path::Path::new("./runtime");
    // add runtime dir's content to view_imports and copy to output dir
    for entry in std::fs::read_dir(runtime_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let base_name = path.file_stem().unwrap().to_str().unwrap().to_string();

        builtin_imports.push(base_name.clone());

        let out_file = format!("{}/{}", out_dir.display(), file_name);
        std::fs::copy(path, out_file).unwrap();
    }

    for st in state.struct_list {
        let st = st.borrow().clone();
        println!("{:?}", st);

        let st_name = st.name.clone();

        if st.inheritance == Some("View".to_string()) {
            let view = view::ViewParser::from_struct(st);
            let cmp_code = view.generate_component_code(builtin_imports.clone());

            let file_name = format!("{}/{}.js", out_dir.display(), st_name);
            std::fs::write(file_name, cmp_code).unwrap();

            view_imports.push(st_name);
        } else if st.inheritance == Some("PreviewProvider".to_string()) {
            let mut transformed = st.clone();
            transformed.inheritance = Some("View".to_string());

            let previews = transformed.members.get("previews").expect("No previews found");
            transformed.members.insert("body".to_string(), previews.clone());
            transformed.members.remove("previews");

            let mut view = view::ViewParser::from_struct(transformed);
            let template = view.generate_template();

            index_template = template;
        }
    }

    // copy styles
    let styles_dir = std::path::Path::new("./styles");
    let mut styles: Vec<String> = Vec::new();
    let styles_out_dir = format!("{}/styles", out_dir.display());

    std::fs::create_dir(&styles_out_dir).unwrap();

    for entry in std::fs::read_dir(styles_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let base_name = path.file_stem().unwrap().to_str().unwrap().to_string();

        styles.push(base_name.clone());

        let out_file = format!("{}/{}", styles_out_dir, file_name);
        std::fs::copy(path, out_file).unwrap();
    }

    view_imports.extend(builtin_imports);

    let index_html = template::generate_template_html(view_imports, styles, index_template);
    let file_name = format!("{}/index.html", out_dir.display());
    std::fs::write(file_name, index_html).unwrap();
}
