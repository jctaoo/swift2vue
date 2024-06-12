use handlebars::Handlebars;

pub static APP_JS_TEMPLATE: &'static str = include_str!("app.js.hbs");
pub static TEMPLATE: &'static str = include_str!("template.hbs");

pub fn generate_template_html(styles: Vec<String>, script: String) -> String {
  let reg = Handlebars::new();

  let context = serde_json::json!({
    "styles": styles,
    "script": script
  });

  reg.render_template(TEMPLATE, &context).unwrap()
}

pub fn generate_app_js(imports: Vec<String>, index_template: String) -> String {
  let reg = Handlebars::new();

  let context = serde_json::json!({
    "imports": imports,
    "index_template": index_template
  });

  reg.render_template(APP_JS_TEMPLATE, &context).unwrap()
}