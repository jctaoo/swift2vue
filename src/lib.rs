use paser::State;

mod component;
mod paser;
mod template;
mod utils;
mod view;
mod bundler;
use napi_derive::napi;

use include_dir::{include_dir, Dir};

static RUNTIME_DIR: Dir = include_dir!("./runtime");
static STYLES_DIR: Dir = include_dir!("./styles");

#[allow(unused)]
#[napi]
fn generate(source: String, outdir: String, verbose: Option<bool>) {
    use tree_sitter::Parser;

    let lang = tree_sitter_swift::language();
    let verbose = verbose.unwrap_or(false);

    let mut parser = Parser::new();
    parser
        .set_language(&lang)
        .expect("Error loading Rust grammar");

    let tree = parser.parse(source.clone(), None).unwrap();
    let root_node = tree.root_node();

    // log_node(&root_node, 0);

    let mut cursor = root_node.walk();

    let mut state = State::new(source.clone(), verbose);
    state.handle_source(&mut cursor);

    // using ./output
    let out_dir = std::path::Path::new(outdir.as_str());
    if out_dir.is_relative() {
        panic!("Output directory must be an absolute path");
    }

    // if the dir is not empty, check is a empty dir or not
    if out_dir.exists() {
        let dir = std::fs::read_dir(out_dir).unwrap();
        let mut is_empty = true;
        for _ in dir {
            is_empty = false;
            break;
        }

        if !is_empty {
            panic!("Output directory is not empty");
        }
    }

    // create the dir
    std::fs::create_dir(out_dir).unwrap();

    let mut view_imports = Vec::new();
    let mut builtin_imports = Vec::new();

    let temp_dir = out_dir.join("temp");
    std::fs::create_dir(&temp_dir).unwrap();

    // add runtime dir's content to view_imports and copy to output dir
    for file in RUNTIME_DIR.files() {
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let base_name = path.file_stem().unwrap().to_str().unwrap().to_string();

        builtin_imports.push(base_name.clone());

        let out_file = format!("{}/{}", temp_dir.display(), file_name);
        std::fs::write(out_file, file.contents());
    }

    for st in state.struct_list {
        let st = st.borrow().clone();
        println!("{:?}", st);

        let st_name = st.name.clone();

        if st.inheritance == Some("View".to_string()) {
            let view = view::ViewParser::from_struct(st, source.clone());
            let cmp_code = view.generate_component_code(builtin_imports.clone());

            let file_name = format!("{}/{}.js", temp_dir.display(), st_name);
            std::fs::write(file_name, cmp_code).unwrap();

            view_imports.push(st_name);
        } else if st.inheritance == Some("PreviewProvider".to_string()) {
            let mut transformed = st.clone();
            transformed.inheritance = Some("View".to_string());

            let previews = transformed.members.get("previews").expect("No previews found");
            transformed.members.insert("body".to_string(), previews.clone());
            transformed.members.remove("previews");

            let mut view = view::ViewParser::from_struct(transformed, source.clone());
            let template = view.generate_template();

            let mut imports = view_imports.clone();
            imports.extend(builtin_imports.clone());

            let app_js = template::generate_app_js(imports, template);

            let file_name = format!("{}/{}.js", temp_dir.display(), "app");
            std::fs::write(file_name, app_js).unwrap();
        }
    }

    // copy styles
    let mut styles: Vec<String> = Vec::new();

    for file in STYLES_DIR.files() {
        let code = file.contents_utf8().unwrap().to_string();
        styles.push(code);
    }

    // do bundle
    let app_js_path = temp_dir.join("app.js");
    let code = bundler::bundle(app_js_path.as_path(), true, false);

    // clear temp dir
    std::fs::remove_dir_all(temp_dir).unwrap();

    // generate html
    let index_html = template::generate_template_html(styles, code);
    println!("index.html generated ({}kb)", index_html.len() / 1024);

    let file_name = format!("{}/index.html", out_dir.display());
    std::fs::write(file_name, index_html).unwrap();
}
