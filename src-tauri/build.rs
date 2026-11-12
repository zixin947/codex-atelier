fn main() {
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rerun-if-changed=native-hud");
        println!("cargo:rerun-if-changed=../vendor/endfield-native");
        assert!(
            std::process::Command::new("node")
                .arg("../scripts/build-native-hud.mjs")
                .status()
                .expect("Node.js is required to build the native HUD")
                .success(),
            "Native HUD compilation failed"
        );
    }
    tauri_build::build()
}
