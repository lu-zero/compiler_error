#![feature(plugin_registrar, rustc_private)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::tokenstream::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, DummyResult, MacResult};
use syntax::print::pprust::tt_to_string;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("compiler_error", compiler_error);
}

fn compiler_error<'a>(cx: &'a mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'a> {
    if args.len() != 1 {
        cx.span_err(sp,
                    &format!("argument should be a single identifier, but got {} arguments",
                             args.len()));
        return DummyResult::any(sp);
    }

    let text = tt_to_string(&args[0]);
    cx.span_err(sp, &format!("{}", text));
    return DummyResult::any(sp);
}
