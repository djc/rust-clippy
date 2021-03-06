use crate::methods::derefs_to_slice;
use crate::utils::{is_type_diagnostic_item, match_type, paths, snippet_with_applicability, span_lint_and_sugg};

use rustc_errors::Applicability;
use rustc_hir::Expr;
use rustc_lint::LateContext;
use rustc_span::sym;

use super::ITER_COUNT;

pub(crate) fn lints<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'_>, iter_args: &'tcx [Expr<'tcx>], iter_method: &str) {
    let ty = cx.typeck_results().expr_ty(&iter_args[0]);
    let caller_type = if derefs_to_slice(cx, &iter_args[0], ty).is_some() {
        "slice"
    } else if is_type_diagnostic_item(cx, ty, sym::vec_type) {
        "Vec"
    } else if is_type_diagnostic_item(cx, ty, sym!(vecdeque_type)) {
        "VecDeque"
    } else if is_type_diagnostic_item(cx, ty, sym!(hashset_type)) {
        "HashSet"
    } else if is_type_diagnostic_item(cx, ty, sym!(hashmap_type)) {
        "HashMap"
    } else if match_type(cx, ty, &paths::BTREEMAP) {
        "BTreeMap"
    } else if match_type(cx, ty, &paths::BTREESET) {
        "BTreeSet"
    } else if match_type(cx, ty, &paths::LINKED_LIST) {
        "LinkedList"
    } else if match_type(cx, ty, &paths::BINARY_HEAP) {
        "BinaryHeap"
    } else {
        return;
    };
    let mut applicability = Applicability::MachineApplicable;
    span_lint_and_sugg(
        cx,
        ITER_COUNT,
        expr.span,
        &format!("called `.{}().count()` on a `{}`", iter_method, caller_type),
        "try",
        format!(
            "{}.len()",
            snippet_with_applicability(cx, iter_args[0].span, "..", &mut applicability),
        ),
        applicability,
    );
}
