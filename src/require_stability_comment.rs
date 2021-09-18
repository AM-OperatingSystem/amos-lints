use rustc_session::{declare_lint, declare_lint_pass};
use rustc_hir::{Item,ImplItem};

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Symbol;
use rustc_hir::{ItemKind,ImplItemKind};

declare_lint! {
    pub REQUIRE_STABILITY_COMMENT,
    Warn,
    "All functions must be annotated with a stability comment"
}

declare_lint_pass!(RequireStabilityComment => [REQUIRE_STABILITY_COMMENT]);

impl<'tcx> LateLintPass<'tcx> for RequireStabilityComment {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if let ItemKind::Fn(_,_,_) = item.kind {
            let attrs = cx.tcx.hir().attrs(item.hir_id());
            let mut has_stability_comment = false;
            let symbol_cfg = Symbol::intern("cfg");
            let symbol_feature = Symbol::intern("feature");
            let symbol_stable = Symbol::intern("stable");
            let symbol_unstable = Symbol::intern("unstable");
            for attr in attrs {
                if attr.has_name(symbol_cfg) {
                    if let Some(items) = attr.meta_item_list() {
                        for i in items {
                            if let Some(ident) = i.ident() {
                                if ident.name == symbol_feature {
                                    if let Some(v) = i.value_str() {
                                        if v==symbol_stable || v==symbol_unstable{
                                            has_stability_comment=true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !has_stability_comment {
                span_lint_and_help(cx,
                    REQUIRE_STABILITY_COMMENT,
                    item.span,
                    "Stability attribute required for all functions",
                    None,
                    "Add a #[cfg(feature=\"stable\")] or #[cfg(feature=\"unstable\")] attribute to this function");
            }
        }
    }

    fn check_impl_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx ImplItem<'tcx>) {
        if let ImplItemKind::Fn(_,_) = item.kind {
            let attrs = cx.tcx.hir().attrs(item.hir_id());
            let mut has_stability_comment = false;
            let symbol_cfg = Symbol::intern("cfg");
            let symbol_feature = Symbol::intern("feature");
            let symbol_stable = Symbol::intern("stable");
            let symbol_unstable = Symbol::intern("unstable");
            for attr in attrs {
                if attr.has_name(symbol_cfg) {
                    if let Some(items) = attr.meta_item_list() {
                        for i in items {
                            if let Some(ident) = i.ident() {
                                if ident.name == symbol_feature {
                                    if let Some(v) = i.value_str() {
                                        if v==symbol_stable || v==symbol_unstable{
                                            has_stability_comment=true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !has_stability_comment {
                span_lint_and_help(cx,
                    REQUIRE_STABILITY_COMMENT,
                    item.span,
                    "Stability attribute required for all functions",
                    None,
                    "Add a #[cfg(feature=\"stable\")] or #[cfg(feature=\"unstable\")] attribute to this function");
            }
        }
    }
}