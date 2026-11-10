//! AiMaMi 1.2.7 image compatibility command and relay declaration filter.
//! All paths are explicit. This module never discovers a home or controls a process.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex, OnceLock,
    },
    time::SystemTime,
};

const FLAG: &str = "image_generation = false";
/// The native lexical matcher intentionally does not parse TOML or quoted keys.
fn image_assignment(line: &str) -> Option<&str> {
    line.trim()
        .strip_prefix("image_generation")?
        .trim_start()
        .strip_prefix('=')
        .map(str::trim)
}
/// @100366dbc: any exact `false` assignment in an exact `[features]` section enables it.
/// Errors, missing files, invalid UTF-8 and absent/other forms all mean false.
pub fn read_image_compat_enabled_text(text: &str) -> bool {
    let mut in_features = false;
    for line in text.lines().map(str::trim) {
        if line.starts_with('[') {
            in_features = line == "[features]";
        } else if in_features && image_assignment(line) == Some("false") {
            return true;
        }
    }
    false
}
pub fn read_image_compat_enabled(config_path: &Path) -> bool {
    fs::read_to_string(config_path).is_ok_and(|s| read_image_compat_enabled_text(&s))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageCompatPayload {
    pub enabled: bool,
}
/// @1007b3e60 calls the direct reader, not the relay cache or a persisted utility preference.
pub fn get_image_compat(config_path: &Path) -> ImageCompatPayload {
    ImageCompatPayload {
        enabled: read_image_compat_enabled(config_path),
    }
}
/// @1007b5bf0, including native LF normalization and section/comment behavior.
pub fn compose_image_compat_flag(text: &str, enabled: bool) -> String {
    if enabled {
        let mut lines = text.lines().collect::<Vec<_>>();
        let mut section = None;
        let mut in_features = false;
        let mut assignment = None;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                if in_features {
                    break;
                }
                in_features = trimmed == "[features]";
                if in_features {
                    section = Some(i);
                }
            } else if in_features && image_assignment(trimmed).is_some() {
                // Every matching line updates this index: only the last is rewritten.
                assignment = Some(i);
            }
        }
        if let Some(i) = assignment {
            lines[i] = FLAG;
        } else if let Some(i) = section {
            lines.insert(i + 1, FLAG);
        } else {
            if !text.is_empty() && !text.ends_with('\n') {
                lines.push("");
            }
            lines.push("[features]");
            lines.push(FLAG);
        }
        let mut output = lines.join("\n");
        output.push('\n');
        output
    } else {
        let mut output: Vec<&str> = vec![];
        let mut in_features = false;
        let mut has_content = false;
        let mut header = None;
        fn remove_empty_section(
            lines: &mut Vec<&str>,
            header: Option<usize>,
            active: bool,
            has_content: bool,
        ) {
            if active && !has_content {
                if let Some(i) = header.filter(|i| *i < lines.len()) {
                    lines.remove(i);
                    while i < lines.len() && lines[i].trim().is_empty() {
                        lines.remove(i);
                    }
                }
            }
        }
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                remove_empty_section(&mut output, header, in_features, has_content);
                in_features = trimmed == "[features]";
                if in_features {
                    has_content = false;
                    header = Some(output.len());
                }
            } else if in_features && !trimmed.is_empty() {
                if image_assignment(trimmed).is_some() {
                    continue;
                }
                if !trimmed.starts_with('#') {
                    has_content = true;
                }
            }
            output.push(line);
        }
        remove_empty_section(&mut output, header, in_features, has_content);
        let mut output = output.join("\n");
        if !output.is_empty() && !output.ends_with('\n') {
            output.push('\n');
        }
        output
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CacheKey {
    path: PathBuf,
    modified: Option<SystemTime>,
    length: Option<u64>,
}
impl CacheKey {
    fn for_path(path: &Path) -> Self {
        let metadata = fs::metadata(path).ok();
        Self {
            path: path.to_path_buf(),
            modified: metadata.as_ref().and_then(|m| m.modified().ok()),
            length: metadata.map(|m| m.len()),
        }
    }
}
/// @100366934 caches one path + Option<mtime> + Option<len>; no TTL or content digest.
#[derive(Default)]
pub struct ImageCompatCache(Option<(CacheKey, bool)>);
impl ImageCompatCache {
    pub fn enabled(&mut self, config_path: &Path) -> bool {
        let key = CacheKey::for_path(config_path);
        if let Some((prior, enabled)) = &self.0 {
            if prior == &key {
                return *enabled;
            }
        }
        let enabled = read_image_compat_enabled(config_path);
        self.0 = Some((key, enabled));
        enabled
    }
}
/// Native poison fallback reads the file directly without trusting/updating the cache.
pub fn image_compat_enabled(config_path: &Path) -> bool {
    static CACHE: OnceLock<Mutex<ImageCompatCache>> = OnceLock::new();
    match CACHE.get_or_init(Default::default).lock() {
        Ok(mut cache) => cache.enabled(config_path),
        Err(_) => read_image_compat_enabled(config_path),
    }
}
fn image_tool(tool: &Value) -> bool {
    tool.get("type").and_then(Value::as_str) == Some("image_generation")
        || (tool.get("type").and_then(Value::as_str) == Some("namespace")
            && tool.get("name").and_then(Value::as_str) == Some("image_gen"))
}
fn strip_array(tools: &mut Vec<Value>) -> usize {
    let before = tools.len();
    tools.retain(|v| !image_tool(v));
    before - tools.len()
}
/// @100367028 only rewrites tools and input.additional_tools. tool_choice is unchanged.
/// Count is removed declarations, not removed empty additional_tools records.
pub fn strip_image_generation_tools(body: &mut Value) -> usize {
    let mut removed = 0;
    if let Some(tools) = body.get_mut("tools").and_then(Value::as_array_mut) {
        removed += strip_array(tools);
    }
    if let Some(input) = body.get_mut("input").and_then(Value::as_array_mut) {
        input.retain_mut(|item| {
            if item.get("type").and_then(Value::as_str) == Some("additional_tools") {
                if let Some(tools) = item.get_mut("tools").and_then(Value::as_array_mut) {
                    removed += strip_array(tools);
                    return !tools.is_empty();
                }
            }
            true
        });
    }
    removed
}

#[derive(Debug)]
pub enum ImageCompatError {
    Io {
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    },
    Contention,
}
impl std::fmt::Display for ImageCompatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io{operation,path,source}=>write!(f,"{operation} {}: {source}",path.display()),
            Self::Contention=>write!(f,"could not update config.toml because it kept changing during update image compatibility"),
        }
    }
}
impl std::error::Error for ImageCompatError {}
/// FS seam for actual contention/error tests; StdImageCompatFs implements real publication.
pub trait ImageCompatFs {
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn read_text(&self, path: &Path) -> io::Result<String>;
    fn write_atomic_private(&self, path: &Path, text: &str) -> io::Result<()>;
}
pub struct StdImageCompatFs;
impl ImageCompatFs for StdImageCompatFs {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }
    fn read_text(&self, path: &Path) -> io::Result<String> {
        match fs::read_to_string(path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(String::new()),
            other => other,
        }
    }
    fn write_atomic_private(&self, path: &Path, text: &str) -> io::Result<()> {
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let parent = path
            .parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "config has no parent"))?;
        let temporary = parent.join(format!(
            ".image-compat-{}-{}.tmp",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        let publish = || {
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(&temporary)?;
            file.write_all(text.as_bytes())?;
            file.sync_all()?;
            drop(file);
            fs::rename(&temporary, path)?;
            fs::File::open(parent)?.sync_all()
        };
        let result = publish();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetImageCompatReport {
    /// Matches the original setter envelope: requested flag, no independent preference write.
    pub enabled: bool,
    pub changed: bool,
    pub attempts: usize,
}
/// @1005c1834: parent creation, raw text read, compose, repeat read/CAS, <=3 attempts.
/// Caller holds the shared mutation gate. There is no TOML parse or native backup here.
pub fn set_image_compat_with_fs(
    fs: &impl ImageCompatFs,
    config_path: &Path,
    enabled: bool,
) -> Result<SetImageCompatReport, ImageCompatError> {
    let error = |operation, source| ImageCompatError::Io {
        operation,
        path: config_path.to_path_buf(),
        source,
    };
    if let Some(parent) = config_path.parent() {
        fs.create_dir_all(parent)
            .map_err(|e| error("create config directory", e))?;
    }
    for attempts in 1..=3 {
        let before = fs
            .read_text(config_path)
            .map_err(|e| error("read config.toml", e))?;
        let after = compose_image_compat_flag(&before, enabled);
        if after == before {
            return Ok(SetImageCompatReport {
                enabled,
                changed: false,
                attempts,
            });
        }
        let current = fs
            .read_text(config_path)
            .map_err(|e| error("read config.toml", e))?;
        if current != before {
            continue;
        }
        fs.write_atomic_private(config_path, &after)
            .map_err(|e| error("failed to write config.toml", e))?;
        return Ok(SetImageCompatReport {
            enabled,
            changed: true,
            attempts,
        });
    }
    Err(ImageCompatError::Contention)
}
pub fn set_image_compat(
    config_path: &Path,
    enabled: bool,
) -> Result<SetImageCompatReport, ImageCompatError> {
    set_image_compat_with_fs(&StdImageCompatFs, config_path, enabled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::{
        cell::{Cell, RefCell},
        collections::VecDeque,
    };
    #[test]
    fn reader_preserves_native_lexical_boundaries_and_duplicate_behavior() {
        for text in [
            "[features]\nimage_generation=false\n",
            "\u{2003}[features]\u{2003}\nimage_generation\t= false\r\n",
            "[features]\nimage_generation=true\nimage_generation=false\n",
        ] {
            assert!(read_image_compat_enabled_text(text), "{text:?}");
        }
        for text in [
            "image_generation=false",
            "[features] # comment\nimage_generation=false",
            "[ features ]\nimage_generation=false",
            "[features]\nimage_generation=false # inline",
            "[features]\n'image_generation'=false",
            "[features]\nimage_generation=\"false\"",
            "[features]\nimage_generation_extra=false",
            "[features.x]\nimage_generation=false",
        ] {
            assert!(!read_image_compat_enabled_text(text), "{text:?}");
        }
        assert!(read_image_compat_enabled_text(
            "invalid TOML here\n[features]\nimage_generation=false"
        ));
    }
    #[test]
    fn enable_rewrites_only_last_assignment_in_first_exact_section() {
        let input="# keep\r\n[features]\r\nimage_generation=true # old\r\n  image_generation = 'odd' # chosen\r\n[other]\r\nimage_generation=true\r\n[features]\r\nimage_generation=true\r\n";
        assert_eq!(compose_image_compat_flag(input,true),"# keep\n[features]\nimage_generation=true # old\nimage_generation = false\n[other]\nimage_generation=true\n[features]\nimage_generation=true\n");
        assert_eq!(
            compose_image_compat_flag("[features]\nimage_generation=\n", true),
            "[features]\nimage_generation = false\n"
        );
    }
    #[test]
    fn enable_inserts_after_exact_header_or_appends_with_original_newline_rule() {
        assert_eq!(
            compose_image_compat_flag("[features]\n# comment\nweb=true", true),
            "[features]\nimage_generation = false\n# comment\nweb=true\n"
        );
        assert_eq!(
            compose_image_compat_flag("model='x'", true),
            "model='x'\n\n[features]\nimage_generation = false\n"
        );
        assert_eq!(
            compose_image_compat_flag("model='x'\n", true),
            "model='x'\n[features]\nimage_generation = false\n"
        );
        assert_eq!(
            compose_image_compat_flag("", true),
            "[features]\nimage_generation = false\n"
        );
        assert_eq!(
            compose_image_compat_flag("[features] # comment\nweb=true", true),
            "[features] # comment\nweb=true\n\n[features]\nimage_generation = false\n"
        );
    }
    #[test]
    fn disable_removes_all_matching_assignments_and_only_empty_feature_sections() {
        let input="model='x'\n[features]\n\n# feature comment\nimage_generation=false\n\n[other]\nx=true\n[features]\nimage_generation=true\nweb=true\n";
        assert_eq!(
            compose_image_compat_flag(input, false),
            "model='x'\n# feature comment\n\n[other]\nx=true\n[features]\nweb=true\n"
        );
        assert_eq!(
            compose_image_compat_flag("[features]\nimage_generation=false\n", false),
            ""
        );
        assert_eq!(
            compose_image_compat_flag(
                "[features]\n# keep comment\nimage_generation=false\n",
                false
            ),
            "# keep comment\n"
        );
        assert_eq!(compose_image_compat_flag("[features]\n", false), "");
        assert_eq!(
            compose_image_compat_flag("model='x'\r\n", false),
            "model='x'\n"
        );
    }
    #[test]
    fn filter_only_native_declaration_locations_leaves_tool_choice_and_input_images() {
        let mut body = json!({"tools":[{"type":"image_generation"},{"type":"namespace","name":"image_gen"},{"type":"function","name":"image_gen"}],"input":[{"type":"input_image","image_url":"keep"},{"type":"additional_tools","tools":[{"type":"namespace","name":"image_gen"}]},{"type":"additional_tools","tools":[]},{"type":"additional_tools","tools":[{"type":"function","name":"keep"},{"type":"image_generation"}]},{"type":"other","tools":[{"type":"image_generation"}]}],"tool_choice":{"type":"image_generation"}});
        assert_eq!(strip_image_generation_tools(&mut body), 4);
        assert_eq!(
            body["tools"],
            json!([{"type":"function","name":"image_gen"}])
        );
        assert_eq!(body["input"].as_array().unwrap().len(), 3);
        assert_eq!(body["input"][0]["image_url"], "keep");
        assert_eq!(body["tool_choice"]["type"], "image_generation");
        assert_eq!(body["input"][2]["tools"][0]["type"], "image_generation");
        assert_eq!(strip_image_generation_tools(&mut body), 0);
    }
    #[test]
    fn filter_malformed_shapes_and_case_are_preserved() {
        for mut value in [
            json!(null),
            json!("text"),
            json!({"tools":null,"input":[{"type":"additional_tools","tools":"keep"}]}),
            json!({"tools":[{"type":"Image_generation"},{"type":"namespace","name":"IMAGE_GEN"},null]}),
        ] {
            let original = value.clone();
            assert_eq!(strip_image_generation_tools(&mut value), 0);
            assert_eq!(value, original);
        }
    }
    #[test]
    fn actual_fs_creates_private_config_no_preference_and_disable_missing_is_noop() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("codex/config.toml");
        assert!(!get_image_compat(&path).enabled);
        let disabled = set_image_compat(&path, false).unwrap();
        assert!(!disabled.changed);
        assert!(!path.exists());
        assert!(path.parent().unwrap().is_dir());
        let set = set_image_compat(&path, true).unwrap();
        assert!(set.changed);
        assert!(get_image_compat(&path).enabled);
        assert!(!set_image_compat(&path, true).unwrap().changed);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
        assert_eq!(fs::read_dir(path.parent().unwrap()).unwrap().count(), 1);
        set_image_compat(&path, false).unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"");
        fs::write(&path, [0xff]).unwrap();
        assert!(!get_image_compat(&path).enabled);
        assert!(set_image_compat(&path, true).is_err());
        assert_eq!(fs::read(path).unwrap(), [0xff]);
    }
    #[test]
    fn cache_uses_path_mtime_and_len_not_a_content_digest_or_persisted_setting() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("config.toml");
        fs::write(&p, "[features]\nimage_generation=false\n").unwrap();
        let mut cache = ImageCompatCache::default();
        assert!(cache.enabled(&p));
        let modified = fs::metadata(&p).unwrap().modified().unwrap();
        fs::write(&p, "[features]\nimage_generation=true \n").unwrap();
        fs::File::options()
            .write(true)
            .open(&p)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(modified))
            .unwrap();
        assert!(!read_image_compat_enabled(&p));
        assert!(cache.enabled(&p));
        fs::write(&p, "[features]\nimage_generation=true\n").unwrap();
        assert!(!cache.enabled(&p));
        let other = d.path().join("other.toml");
        fs::write(&other, "[features]\nimage_generation=false\n").unwrap();
        assert!(cache.enabled(&other));
        fs::remove_file(&other).unwrap();
        assert!(!cache.enabled(&other));
    }
    struct Scripted {
        reads: RefCell<VecDeque<io::Result<String>>>,
        writes: RefCell<Vec<String>>,
        created: Cell<usize>,
        fail_write: bool,
    }
    impl ImageCompatFs for Scripted {
        fn create_dir_all(&self, _: &Path) -> io::Result<()> {
            self.created.set(self.created.get() + 1);
            Ok(())
        }
        fn read_text(&self, _: &Path) -> io::Result<String> {
            self.reads
                .borrow_mut()
                .pop_front()
                .expect("real algorithm expected another read")
        }
        fn write_atomic_private(&self, _: &Path, s: &str) -> io::Result<()> {
            if self.fail_write {
                Err(io::Error::other("synthetic publish failure"))
            } else {
                self.writes.borrow_mut().push(s.into());
                Ok(())
            }
        }
    }
    fn script(reads: &[&str]) -> Scripted {
        Scripted {
            reads: RefCell::new(reads.iter().map(|s| Ok((*s).into())).collect()),
            writes: RefCell::new(vec![]),
            created: Cell::new(0),
            fail_write: false,
        }
    }
    #[test]
    fn real_update_algorithm_rebases_contention_and_reports_three_attempt_limit() {
        let f = script(&[
            "model='old'\n",
            "model='external'\n",
            "model='external'\n",
            "model='external'\n",
        ]);
        let r = set_image_compat_with_fs(&f, Path::new("/fixture/config.toml"), true).unwrap();
        assert_eq!(r.attempts, 2);
        assert_eq!(f.created.get(), 1);
        assert_eq!(f.writes.borrow().len(), 1);
        assert!(f.writes.borrow()[0].contains("model='external'"));
        let f = script(&["a", "b", "c", "d", "e", "f"]);
        assert!(matches!(
            set_image_compat_with_fs(&f, Path::new("/fixture/config.toml"), true),
            Err(ImageCompatError::Contention)
        ));
        assert!(f.writes.borrow().is_empty());
        let f = script(&["[features]\nimage_generation = false\n"]);
        assert!(
            !set_image_compat_with_fs(&f, Path::new("/fixture/config.toml"), true)
                .unwrap()
                .changed
        );
        assert!(f.reads.borrow().is_empty());
    }
    #[test]
    fn read_or_publication_errors_are_not_success_and_no_toml_validation_is_invented() {
        let mut f = script(&["", " "]);
        f.reads.borrow_mut().clear();
        f.reads.borrow_mut().push_back(Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "fixture denied",
        )));
        assert!(set_image_compat_with_fs(&f, Path::new("/fixture/config.toml"), true).is_err());
        assert!(f.writes.borrow().is_empty());
        f = script(&["broken = [\n", "broken = [\n"]);
        f.fail_write = true;
        assert!(set_image_compat_with_fs(&f, Path::new("/fixture/config.toml"), true).is_err());
        let f = script(&["broken = [\n", "broken = [\n"]);
        assert!(
            set_image_compat_with_fs(&f, Path::new("/fixture/config.toml"), true)
                .unwrap()
                .changed
        );
        assert!(f.writes.borrow()[0].starts_with("broken = [\n"));
    }
}
