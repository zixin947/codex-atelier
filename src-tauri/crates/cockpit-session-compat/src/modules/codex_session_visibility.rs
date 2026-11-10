include!("../../upstream/codex_session_visibility.rs");
pub type Report = CodexSessionVisibilityRepairSummary;
pub fn run(
    home: &Path,
    target: &str,
    ids: Vec<String>,
    deep: bool,
    dry_run: bool,
) -> Result<Report, String> {
    let _lock = acquire_session_visibility_repair_lock()?;
    if ids.is_empty() {
        return Err("必须显式选择线程，不允许空范围隐式全量修复".into());
    }
    let mode = if deep {
        CodexSessionVisibilityRepairMode::Deep
    } else {
        CodexSessionVisibilityRepairMode::Quick
    };
    let selection = RepairTargetSelection::from_inputs(Some(target.into()), Some(ids), None)?;
    let instance = CodexSyncInstance {
        id: "atelier".into(),
        name: "Atelier 指定的 Codex".into(),
        data_dir: home.into(),
        last_pid: None,
    };
    repair_session_visibility_for_instances_with_options(
        CodexSessionVisibilityRepairOptions::for_mode(mode).with_dry_run(dry_run),
        None,
        None,
        selection,
        vec![instance],
    )
}
pub fn restore(home: &Path, backup: &Path) -> Result<(), String> {
    let _lock = acquire_session_visibility_repair_lock()?;
    restore_instance_files_from_backup(home, backup, true)
}
