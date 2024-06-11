use handlebars::Handlebars;

pub static TEMPLATE: &'static str = include_str!("template.hbs");

pub fn generate_template_html(imports: Vec<String>, index_template: String) -> String {
  let reg = Handlebars::new();

  let context = serde_json::json!({
    "imports": imports,
    "index_template": index_template
  });

  reg.render_template(TEMPLATE, &context).unwrap()
}