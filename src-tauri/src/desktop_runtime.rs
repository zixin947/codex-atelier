//! Desktop is the supported host. Never silently substitute a separately installed CLI.
use std::path::PathBuf;

pub fn applications() -> Vec<PathBuf> {
    let mut roots = vec![PathBuf::from("/Applications")];
    if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join("Applications"));
    }
    roots
        .into_iter()
        .flat_map(|root| [root.join("ChatGPT.app"), root.join("Codex.app")])
        .collect()
}
fn candidates() -> Vec<PathBuf> {
    applications()
        .into_iter()
        .flat_map(|app| {
            let root = app.join("Contents/Resources");
            [
                root.join("codex-cli/CodexCLI.app/Contents/MacOS/codex"),
                root.join("codex-cli/bin/codex"),
                root.join("codex"),
            ]
        })
        .collect()
}
pub fn binary() -> Option<PathBuf> {
    candidates().into_iter().find(|path| path.is_file())
}
pub fn owns_binary(path: &std::path::Path) -> bool {
    candidates()
        .into_iter()
        .any(|candidate| candidate.canonicalize().ok().as_deref() == Some(path))
}
