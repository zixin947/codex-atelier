use tauri::{Emitter, Manager};

#[derive(Default)]
pub struct HudNativeState(std::sync::Arc<std::sync::Mutex<Option<NativeHud>>>);

#[cfg(target_os = "macos")]
type Create = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    extern "C" fn(*mut libc::c_void, i32),
    *mut libc::c_void,
) -> *mut libc::c_void;
#[cfg(target_os = "macos")]
type Update = unsafe extern "C" fn(*mut libc::c_void, f64, f64, f64, f64, bool);
#[cfg(target_os = "macos")]
type Destroy = unsafe extern "C" fn(*mut libc::c_void);
#[cfg(target_os = "macos")]
struct NativeHud {
    view: usize,
    update: Update,
    destroy: Destroy,
    _context: Box<tauri::WebviewWindow>,
}
#[cfg(not(target_os = "macos"))]
struct NativeHud;

#[tauri::command]
pub fn hud_native_supported() -> bool {
    cfg!(target_os = "macos")
}

#[cfg(target_os = "macos")]
extern "C" fn navigate(context: *mut libc::c_void, index: i32) {
    if (0..14).contains(&index) {
        let window = unsafe { &*(context as *const tauri::WebviewWindow) };
        let _ = window.emit("atelier:hud-navigate", index);
    }
}
#[cfg(target_os = "macos")]
unsafe fn symbol<T: Copy>(handle: *mut libc::c_void, name: &std::ffi::CStr) -> Result<T, String> {
    let value = libc::dlsym(handle, name.as_ptr());
    if value.is_null() {
        return Err(format!("原生 HUD 缺少符号 {}", name.to_string_lossy()));
    }
    Ok(std::mem::transmute_copy(&value))
}

#[tauri::command]
pub async fn hud_native_update(
    window: tauri::WebviewWindow,
    state: tauri::State<'_, HudNativeState>,
    frame: [f64; 4],
    labels: Vec<String>,
    visible: bool,
) -> Result<(), String> {
    if labels.len() != 14
        || frame.iter().any(|v| !v.is_finite())
        || frame[2] <= 0.0
        || frame[3] <= 0.0
    {
        return Err("原生 HUD 展示参数无效".into());
    }
    #[cfg(target_os = "macos")]
    {
        let state = state.0.clone();
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let host = window.clone();
        window
            .run_on_main_thread(move || {
                let result = (|| -> Result<(), String> {
                    let mut current = state.lock().map_err(|_| "原生 HUD 状态锁不可用")?;
                    if current.is_none() && !visible {
                        return Ok(());
                    }
                    if current.is_none() {
                        let path = host
                            .path()
                            .resource_dir()
                            .map_err(|e| e.to_string())?
                            .join("hud-native/libAtelierHUD.dylib");
                        let path = std::ffi::CString::new(path.to_string_lossy().as_bytes())
                            .map_err(|e| e.to_string())?;
                        let handle = unsafe {
                            libc::dlopen(path.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL)
                        };
                        if handle.is_null() {
                            let reason = unsafe { std::ffi::CStr::from_ptr(libc::dlerror()) }
                                .to_string_lossy();
                            return Err(format!("无法加载 EndfieldHUD 原生呈现库：{reason}"));
                        }
                        let create: Create = unsafe { symbol(handle, c"atelier_hud_create")? };
                        let update: Update = unsafe { symbol(handle, c"atelier_hud_update")? };
                        let destroy: Destroy = unsafe { symbol(handle, c"atelier_hud_destroy")? };
                        let mut context = Box::new(host.clone());
                        let labels = std::ffi::CString::new(
                            serde_json::to_string(&labels).map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;
                        let view = unsafe {
                            create(
                                host.ns_window().map_err(|e| e.to_string())?,
                                labels.as_ptr(),
                                navigate,
                                (&mut *context as *mut tauri::WebviewWindow).cast(),
                            )
                        };
                        if view.is_null() {
                            return Err("原生 HUD 地图资源初始化失败".into());
                        }
                        *current = Some(NativeHud {
                            view: view as usize,
                            update,
                            destroy,
                            _context: context,
                        });
                    }
                    let hud = current.as_ref().unwrap();
                    unsafe {
                        (hud.update)(
                            hud.view as *mut libc::c_void,
                            frame[0],
                            frame[1],
                            frame[2],
                            frame[3],
                            visible,
                        );
                    }
                    Ok(())
                })();
                let _ = sender.send(result);
            })
            .map_err(|e| e.to_string())?;
        receiver.await.map_err(|e| e.to_string())?
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (window, state, frame, labels, visible);
        Err("原生 EndfieldHUD 呈现仅支持 macOS".into())
    }
}

// Swift registers Objective-C classes and owns asynchronous paint callbacks.
// Keep the library loaded for process lifetime; destroy only its view.
pub fn cleanup(window: &tauri::Window) {
    #[cfg(target_os = "macos")]
    if let Some(hud) = window.state::<HudNativeState>().0.lock().unwrap().take() {
        unsafe {
            (hud.destroy)(hud.view as *mut libc::c_void);
        }
    }
}
