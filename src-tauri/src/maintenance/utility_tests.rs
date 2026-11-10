use super::*;
use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    thread,
};
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!("atelier-utility-test-{}", Uuid::new_v4()));
        Self(p)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn settings_read_is_non_creating_and_config_is_injected_for_each_gateway_request() {
    let f = Fixture::new();
    assert!(!read_settings_at(&f.0).unwrap().image_compat);
    assert!(!f.0.exists());
    let config = f.0.join("codex/config.toml");
    let store =
        crate::gateway::storage::Store::new_with_config(f.0.join("gateway"), config.clone())
            .unwrap();
    let local_only = crate::gateway::storage::Store::new(f.0.join("local-only-gateway")).unwrap();
    assert!(!store.image_compat_enabled().unwrap());
    atelier_maintenance_core::image_compat::set_image_compat(&config, true).unwrap();
    assert!(store.image_compat_enabled().unwrap());
    assert!(
        read_settings_with_config(&f.0, &config)
            .unwrap()
            .image_compat
    );
    assert!(!local_only.image_compat_enabled().unwrap());
    assert!(!settings_path(&f.0).exists());
    atelier_maintenance_core::image_compat::set_image_compat(&config, false).unwrap();
    assert!(!store.image_compat_enabled().unwrap());
}
#[test]
fn legacy_image_preference_is_ignored_and_endpoint_updates_preserve_only_endpoint() {
    let f = Fixture::new();
    fs::create_dir_all(f.0.join("maintenance")).unwrap();
    atomic_private(
        &settings_path(&f.0),
        br#"{"imageCompat":true,"diagnosticEndpoint":"https://example.invalid/upload"}"#,
    )
    .unwrap();
    let config = f.0.join("codex/config.toml");
    let state = read_settings_with_config(&f.0, &config).unwrap();
    assert!(!state.image_compat);
    assert_eq!(
        state.diagnostic_endpoint.as_deref(),
        Some("https://example.invalid/upload")
    );
    atelier_maintenance_core::image_compat::set_image_compat(&config, true).unwrap();
    assert!(
        read_settings_with_config(&f.0, &config)
            .unwrap()
            .image_compat
    );
    update_settings(&f.0, |s| {
        s.diagnostic_endpoint = Some("https://example.invalid/new".into())
    })
    .unwrap();
    let stored: Value = serde_json::from_slice(&fs::read(settings_path(&f.0)).unwrap()).unwrap();
    assert!(stored.get("imageCompat").is_none());
    assert_eq!(stored["diagnosticEndpoint"], "https://example.invalid/new");
}
#[test]
fn image_filter_keeps_input_images_and_function_tools_and_restores_passthrough() {
    let request = json!({"model":"test","tools":[{"type":"image_generation"},{"type":"function","name":"image_generation"},{"type":"web_search"}],
        "tool_choice":{"type":"image_generation"},"input":[{"role":"user","content":[{"type":"input_image","image_url":"synthetic-photo"}]}]});
    let mut actual = request.clone();
    assert_eq!(filter_image_declarations(&mut actual, false), 0);
    assert_eq!(actual, request);
    assert_eq!(filter_image_declarations(&mut actual, true), 1);
    assert_eq!(actual["tools"].as_array().unwrap().len(), 2);
    assert_eq!(actual["tool_choice"], request["tool_choice"]);
    assert_eq!(actual["input"], request["input"]);
}
#[test]
fn bundle_is_valid_zip_and_contains_only_allowlisted_data() {
    let f = Fixture::new();
    update_settings(&f.0, |s| s.diagnostic_endpoint = None).unwrap();
    record_diagnostic_summary(&f.0,&json!({"checkedAt":1,"root":"/private/user/path","items":[
        {"id":"auth_integrity","status":"warning","label":"user@example.invalid","detail":"sk-SYNTHETIC_SECRET"},
        {"id":"secret-id","status":"ok"},{"id":"config_access","status":"sk-SYNTHETIC_SECRET"}]})).unwrap();
    // A raw credential file must not be read or packaged even if present in app data.
    fs::write(f.0.join("auth.json"), b"SYNTHETIC_DO_NOT_INCLUDE").unwrap();
    let bundle = export_at(&f.0, None).unwrap();
    let contents = String::from_utf8_lossy(&bundle.bytes);
    for secret in [
        "SYNTHETIC_DO_NOT_INCLUDE",
        "sk-SYNTHETIC_SECRET",
        "user@example.invalid",
        "/private/user/path",
        "secret-id",
    ] {
        assert!(!contents.contains(secret));
    }
    let target = f.0.join("fixture.zip");
    fs::write(&target, &bundle.bytes).unwrap();
    let result=std::process::Command::new("python3").args(["-c", "import zipfile,json,sys; z=zipfile.ZipFile(sys.argv[1]); assert z.testzip() is None; assert set(z.namelist())=={'manifest.json','diagnostics.json','README.txt'}; d=json.loads(z.read('diagnostics.json')); assert d=={'checkedAt':1,'items':[{'id':'auth_integrity','status':'warning'}]}"]).arg(target).status().unwrap();
    assert!(result.success());
}
#[test]
fn endpoint_rejects_credentials_queries_non_https_and_redirect_destinations() {
    for target in [
        "http://example.com/upload",
        "https://user:secret@example.com",
        "https://example.com/?key=secret",
        "https://example.com/#private",
        "file:///tmp/data",
    ] {
        assert!(endpoint(target).is_err(), "{target}");
    }
    for target in [
        "https://example.com/upload",
        "http://127.0.0.1:45678/upload",
        "http://[::1]:45678/upload",
    ] {
        assert!(endpoint(target).is_ok());
    }
}
fn receiver(status: &str, body: &str) -> (String, thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}/receive", listener.local_addr().unwrap());
    let response=format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len());
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut all = vec![];
        let mut chunk = [0u8; 4096];
        loop {
            let n = stream.read(&mut chunk).unwrap();
            if n == 0 {
                break;
            }
            all.extend_from_slice(&chunk[..n]);
            if let Some(end) = all.windows(4).position(|w| w == b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&all[..end]);
                let size = headers
                    .lines()
                    .find_map(|line| {
                        line.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .and_then(|v| v.trim().parse::<usize>().ok())
                    })
                    .unwrap_or(0);
                if all.len() >= end + 4 + size {
                    break;
                }
            }
        }
        stream.write_all(response.as_bytes()).unwrap();
        all
    });
    (address, handle)
}
#[tokio::test]
async fn diagnostic_upload_delivers_multipart_only_after_confirmation() {
    let f = Fixture::new();
    assert!(
        send_at(&f.0, "https://example.invalid/upload", "note", false, None)
            .await
            .is_err()
    );
    assert!(!f.0.exists());
    let (url, worker) = receiver("201 Created", r#"{"reportId":"fixture-123"}"#);
    let result = send_at(&f.0, &url, "Synthetic test only", true, None)
        .await
        .unwrap();
    assert_eq!(result.report_id, "fixture-123");
    let request = worker.join().unwrap();
    let request = String::from_utf8_lossy(&request);
    assert!(request.contains("multipart/form-data"));
    assert!(request.contains("Synthetic test only"));
    assert!(request.contains("application/zip"));
    assert!(!request.to_lowercase().contains("authorization:"));
    assert_eq!(
        read_settings_at(&f.0)
            .unwrap()
            .diagnostic_endpoint
            .as_deref(),
        Some(url.as_str())
    );
}
#[tokio::test]
async fn failed_upload_does_not_claim_success_or_save_receiver() {
    let f = Fixture::new();
    let (url, worker) = receiver("503 Service Unavailable", "{}");
    assert!(send_at(&f.0, &url, "", true, None)
        .await
        .unwrap_err()
        .contains("503"));
    worker.join().unwrap();
    assert!(read_settings_at(&f.0)
        .unwrap()
        .diagnostic_endpoint
        .is_none());
}

#[test]
fn filters_additional_tools_and_image_namespace_like_installed_release() {
    let mut request = json!({"tools":[{"type":"namespace","name":"image_gen","tools":[]},{"type":"namespace","name":"allowed","tools":[]}],
        "input":[{"type":"additional_tools","tools":[{"type":"image_generation"}]},
            {"type":"additional_tools","tools":[{"type":"namespace","name":"image_gen"},{"type":"function","name":"keep"}]},
            {"type":"message","content":[{"type":"input_image","image_url":"fixture"}]}]});
    assert_eq!(filter_image_declarations(&mut request, true), 3);
    assert_eq!(request["tools"].as_array().unwrap().len(), 1);
    assert_eq!(request["input"].as_array().unwrap().len(), 2);
    assert_eq!(request["input"][0]["tools"][0]["name"], "keep");
    assert_eq!(request["input"][1]["content"][0]["type"], "input_image");
}

#[test]
fn native_save_writes_a_valid_archive_and_reports_no_false_success() {
    let f = Fixture::new();
    fs::create_dir_all(&f.0).unwrap();
    let target = f.0.join("saved.zip");
    let saved = save_bundle_at(&f.0, &target, None).unwrap();
    assert!(saved.saved);
    assert_eq!(saved.path.as_deref(), target.to_str());
    assert_eq!(&fs::read(&target).unwrap()[..4], b"PK\x03\x04");
    assert!(save_bundle_at(&f.0, &f.0.join("missing-directory/out.zip"), None).is_err());
    assert!(!f.0.join("missing-directory").exists());
}

#[test]
fn exported_image_state_uses_explicit_config_and_absent_source_is_unknown() {
    let f = Fixture::new();
    let config = f.0.join("codex/config.toml");
    fn manifest(bundle: &BundleExport) -> Value {
        let bytes = &bundle.bytes;
        let length = u32::from_le_bytes(bytes[18..22].try_into().unwrap()) as usize;
        let name = u16::from_le_bytes(bytes[26..28].try_into().unwrap()) as usize;
        let extra = u16::from_le_bytes(bytes[28..30].try_into().unwrap()) as usize;
        serde_json::from_slice(&bytes[30 + name + extra..30 + name + extra + length]).unwrap()
    }
    let unknown = manifest(&export_at(&f.0, None).unwrap());
    assert!(unknown["settings"]["imageCompat"].is_null());
    assert_eq!(unknown["settings"]["imageCompatSource"], "unknown");
    for enabled in [false, true, false] {
        atelier_maintenance_core::image_compat::set_image_compat(&config, enabled).unwrap();
        let data = manifest(&export_at(&f.0, Some(&config)).unwrap());
        assert_eq!(data["settings"]["imageCompat"], enabled);
        assert_eq!(data["settings"]["imageCompatSource"], "config");
    }
    assert!(!settings_path(&f.0).exists());
}
