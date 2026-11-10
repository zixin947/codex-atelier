//! Native repair policy, including shared switch arms verified directly in ARM64.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum QuietWindow {
    NotRequired,
    Required,
    Unresolved,
}
pub(super) fn classify(id: &str) -> QuietWindow {
    use atelier_maintenance_core::policy::{quiet_window, QuietWindow as Native};
    match quiet_window(id) {
        Native::NotRequired => QuietWindow::NotRequired,
        Native::Required => QuietWindow::Required,
        Native::Unknown => QuietWindow::Unresolved,
    }
}
pub(super) fn can_run_with_desktop(ids: &[String]) -> bool {
    // Preserve the existing quiet-window requirement for unresolved cases.
    // Only the two confirmed exceptions relax it; full upstream dispatch is pending.
    !ids.is_empty()
        && ids
            .iter()
            .all(|id| classify(id) == QuietWindow::NotRequired)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn report_inventory_does_not_imply_a_known_repair_policy() {
        for id in [
            "config_access",
            "official_catalog_freshness",
            "warning",
            "all",
        ] {
            assert_eq!(classify(id), QuietWindow::Unresolved);
            assert!(!can_run_with_desktop(&[id.into()]));
        }
    }
}
