#![feature(quote, plugin_registrar, rustc_private)]
#![feature(stmt_expr_attributes)]
extern crate external_mixin_umbrella as emu;

extern crate syntax;
extern crate rustc_plugin;

use std::path::Path;
use std::env;
use syntax::codemap;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;

use rustc_plugin::Registry;

const NAME: &'static str = "rust_mixin";
const TARGET: &'static str = "rust_mixin_output_binary";

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(registrar: &mut Registry) {
    if let Ok(synext) = emu::MixinExpander::new(registrar, NAME.to_string(), mixin) {
        registrar.register_syntax_extension(token::intern(NAME), synext)
    }
}

fn mixin(cx: &mut ExtCtxt, sp: codemap::Span,
         options: emu::Options,
         dir: &Path,
         file: &Path) -> Result<emu::Output, ()> {
    let mut args: Vec<String> = match options.get("arg") {
        None => vec![],
        Some(args) => args.iter().map(|t| t.0.clone()).collect()
    };

    for (k, vals) in &options {
        match &**k {
            "arg" => {}
            _ => {
                let span = vals[0].1;
                cx.span_err(span, &format!("`{}!`: unknown option `{}`", NAME, k));
            }
        }
    }

    args.push("--crate-name".to_string());
    args.push(TARGET.to_string());
    args.push("-o".to_string());
    args.push(TARGET.to_string());
    emu::run_mixin_command(cx, sp, NAME, dir, "rustc", &args, Some(&file))
        .map(|o| {
            #[cfg(any(windows))]
            return emu::Output::Compiled(o, format!("{}\\{}\\{}.exe",env::temp_dir().as_path().to_string_lossy(),dir.file_name().unwrap().to_string_lossy(), TARGET));
            #[cfg(any(not(windows)))]
            return emu::Output::Compiled(o, format!("./{}", TARGET));
        })
}
