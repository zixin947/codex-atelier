//! 接入回归：仅创建隔离临时目录，不调用原生 Context 或真实客户端。
use super::*;

const ORIGINAL: &str = concat!(
    "model = \"gpt-test\"\n",
    "# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\n",
    "broken = [\n",
    "# <<< aimami-relay managed end\n",
);
const EXPECTED: &[u8] = b"model = \"gpt-test\"\n";

struct Fixture {
    root: PathBuf,
    ctx: Context,
}
impl Fixture {
    fn new() -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-syntax-workflow-{}", uuid::Uuid::new_v4()));
        let ctx = Context {
            home: root.join("codex"),
            app_root: root.join("atelier"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: None,
            sqlite_home_env: None,
        };
        std::fs::create_dir_all(&ctx.home).unwrap();
        std::fs::create_dir_all(&ctx.app_root).unwrap();
        std::fs::write(ctx.config(), ORIGINAL).unwrap();
        Self { root, ctx }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn planned_strip_text_matches_the_repair_preview_contract() {
    match config_syntax::plan_config_toml_syntax_rescue(ORIGINAL.as_bytes()) {
        config_syntax::RescuePlan::Plan { after, .. } => assert_eq!(
            String::from_utf8(after).unwrap(),
            std::str::from_utf8(EXPECTED).unwrap()
        ),
        config_syntax::RescuePlan::None => {
            panic!("broken configuration must produce a repair plan")
        }
    }
}

#[test]
fn preview_never_writes_config_or_creates_backups() {
    let f = Fixture::new();
    let before = std::fs::read_dir(&f.ctx.home).unwrap().count();
    repair::preview(&f.ctx, "config_toml_syntax").unwrap();
    assert_eq!(std::fs::read(f.ctx.config()).unwrap(), ORIGINAL.as_bytes());
    assert_eq!(std::fs::read_dir(&f.ctx.home).unwrap().count(), before);
    assert_eq!(std::fs::read_dir(&f.ctx.app_root).unwrap().count(), 0);
}

#[test]
fn confirmed_strip_repair_writes_planned_text_and_backs_up_original() {
    let f = Fixture::new();
    let ticket = repair::preview(&f.ctx, "config_toml_syntax").unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert_eq!(
        std::fs::read_to_string(f.ctx.config()).unwrap(),
        std::str::from_utf8(EXPECTED).unwrap()
    );
    let backup = PathBuf::from(result.backup_path.unwrap());
    assert_eq!(
        std::fs::read(backup.join("0000.bin")).unwrap(),
        ORIGINAL.as_bytes()
    );
    assert!(result.fixed_ids.iter().any(|id| id == "config_toml_syntax"));
    assert!(!result
        .failed_ids
        .iter()
        .any(|id| id == "config_toml_syntax"));
}

#[test]
fn external_update_between_preview_and_confirm_is_preserved() {
    let f = Fixture::new();
    let ticket = repair::preview(&f.ctx, "config_toml_syntax").unwrap();
    let external = b"model = \"external-change\"\n";
    std::fs::write(f.ctx.config(), external).unwrap();
    assert!(repair::apply(&f.ctx, ticket).is_err());
    assert_eq!(std::fs::read(f.ctx.config()).unwrap(), external);
    assert!(!f
        .ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .exists());
}

#[test]
fn repair_all_can_rescue_a_broken_config() {
    let f = Fixture::new();
    let ticket = repair::preview(&f.ctx, "all").unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert_eq!(
        std::fs::read_to_string(f.ctx.config()).unwrap(),
        std::str::from_utf8(EXPECTED).unwrap()
    );
    assert!(result.fixed_ids.iter().any(|id| id == "config_toml_syntax"));
}

#[test]
fn salvage_that_remains_invalid_is_not_reported_as_fixed() {
    let f = Fixture::new();
    let broken = b"model =\n";
    std::fs::write(f.ctx.config(), broken).unwrap();
    let ticket = repair::preview(&f.ctx, "config_toml_syntax").unwrap();
    let result = repair::apply(&f.ctx, ticket).unwrap();
    assert!(result
        .failed_ids
        .iter()
        .any(|id| id == "config_toml_syntax"));
    assert!(!result.fixed_ids.iter().any(|id| id == "config_toml_syntax"));
    assert!(result
        .report
        .items
        .iter()
        .any(|i| i.id == "config_toml_syntax" && i.status == "error"));
    let backup = PathBuf::from(result.backup_path.unwrap());
    assert_eq!(std::fs::read(backup.join("0000.bin")).unwrap(), broken);
}
