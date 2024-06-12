use anyhow::Error;
use std::{collections::HashMap, path::Path};
use swc_bundler::{Bundle, Bundler, Load, ModuleData, ModuleRecord};
use swc_common::{sync::Lrc, FileName, FilePathMapping, SourceMap, Span};
use swc_ecma_ast::*;
use swc_ecma_codegen::{
    text_writer::{omit_trailing_semi, JsWriter, WriteJs},
    Emitter,
};
use swc_ecma_loader::{
    resolvers::{lru::CachingResolver, node::NodeModulesResolver},
    TargetEnv,
};
use swc_ecma_parser::{parse_file_as_module, Syntax};

pub struct Loader {
    pub cm: Lrc<SourceMap>,
}

impl Load for Loader {
    fn load(&self, f: &FileName) -> Result<ModuleData, Error> {
        let fm = match f {
            FileName::Real(path) => self.cm.load_file(path)?,
            _ => unreachable!(),
        };

        let module = parse_file_as_module(
            &fm,
            Syntax::Es(Default::default()),
            EsVersion::Es2020,
            None,
            &mut vec![],
        )
        .unwrap_or_else(|err| {
            panic!("failed to parse module: {:?}\n{}", err, fm.src);
        });

        Ok(ModuleData {
            fm,
            module,
            helpers: Default::default(),
        })
    }
}

#[allow(dead_code)]
struct Hook;

impl swc_bundler::Hook for Hook {
    fn get_import_meta_props(
        &self,
        span: Span,
        module_record: &ModuleRecord,
    ) -> Result<Vec<KeyValueProp>, Error> {
        let file_name = module_record.file_name.to_string();

        Ok(vec![
            KeyValueProp {
                key: PropName::Ident(Ident::new("url".into(), span)),
                value: Box::new(Expr::Lit(Lit::Str(Str {
                    span,
                    raw: None,
                    value: file_name.into(),
                }))),
            },
            KeyValueProp {
                key: PropName::Ident(Ident::new("main".into(), span)),
                value: Box::new(if module_record.is_entry {
                    Expr::Member(MemberExpr {
                        span,
                        obj: Box::new(Expr::MetaProp(MetaPropExpr {
                            span,
                            kind: MetaPropKind::ImportMeta,
                        })),
                        prop: MemberProp::Ident(Ident::new("main".into(), span)),
                    })
                } else {
                    Expr::Lit(Lit::Bool(Bool { span, value: false }))
                }),
            },
        ])
    }
}

fn print_bundles(cm: Lrc<SourceMap>, modules: Vec<Bundle>, minify: bool) -> String {
    for bundled in modules {
        let code = {
            let mut buf = vec![];

            {
                let wr = JsWriter::new(cm.clone(), "\n", &mut buf, None);
                let mut emitter = Emitter {
                    cfg: swc_ecma_codegen::Config::default().with_minify(true),
                    cm: cm.clone(),
                    comments: None,
                    wr: if minify {
                        Box::new(omit_trailing_semi(wr)) as Box<dyn WriteJs>
                    } else {
                        Box::new(wr) as Box<dyn WriteJs>
                    },
                };

                emitter.emit_module(&bundled.module).unwrap();
            }

            String::from_utf8_lossy(&buf).to_string()
        };

        println!("Created output.js ({}kb)", code.len() / 1024);

        // TODO: only support 1 bundle
        return code;
    }

    return String::new();
}

pub fn bundle(entry: &Path, inline: bool, minify: bool) -> String {
    let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
    let globals = Box::leak(Box::default());
    let mut bundler = Bundler::new(
        globals,
        cm.clone(),
        Loader { cm: cm.clone() },
        CachingResolver::new(
            4096,
            NodeModulesResolver::new(TargetEnv::Node, Default::default(), true),
        ),
        swc_bundler::Config {
            require: false,
            disable_inliner: !inline,
            external_modules: Default::default(),
            disable_fixer: minify,
            disable_hygiene: minify,
            disable_dce: false,
            module: Default::default(),
        },
        Box::new(Hook),
    );

    let mut entries = HashMap::new();
    entries.insert("main".to_string(), FileName::Real(entry.to_path_buf()));

    let modules = bundler
        .bundle(entries)
        .map_err(|err| println!("{:?}", err))
        .unwrap();
    // clean modules
    drop(bundler);

    println!("Bundled as {} modules", modules.len());

    return print_bundles(cm, modules, minify);
}
