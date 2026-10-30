use crate::gateway::{
    storage::{atomic_private, read_private},
    types::now_ms,
};
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};
use tokio::{
    sync::watch,
    time::{Instant, MissedTickBehavior},
};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Settings {
    interval_ms: u64,
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    pub finished_at: u64,
    pub updated: usize,
    pub failed: usize,
    pub error: Option<String>,
}
#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub version: u64,
    pub configured: bool,
    pub interval_ms: u64,
    pub running: bool,
    pub started_at: Option<u64>,
    pub next_refresh_at: Option<u64>,
    pub last_run: Option<RunResult>,
    pub error: Option<String>,
}
pub struct Controller {
    path: PathBuf,
    status: Mutex<Status>,
    changes: watch::Sender<u64>,
    pub started: AtomicBool,
}
fn valid_interval(value: u64) -> bool {
    matches!(value, 0 | 30_000 | 60_000 | 180_000 | 300_000)
}
impl Controller {
    pub fn open(path: PathBuf) -> Arc<Self> {
        let mut status = Status {
            version: 1,
            ..Status::default()
        };
        if path.exists() {
            status.configured = true;
            match read_private(&path, 4096).and_then(|raw| {
                let settings: Settings = serde_json::from_slice(&raw)
                    .map_err(|_| "自动刷新设置损坏，已停用；请重新选择频率。")?;
                if !valid_interval(settings.interval_ms) {
                    return Err("自动刷新频率不支持，已停用；请重新选择。".into());
                }
                Ok(settings)
            }) {
                Ok(settings) => status.interval_ms = settings.interval_ms,
                Err(error) => status.error = Some(error),
            }
        }
        let (changes, _) = watch::channel(status.interval_ms);
        Arc::new(Self {
            path,
            status: Mutex::new(status),
            changes,
            started: AtomicBool::new(false),
        })
    }
    pub fn status(&self) -> Result<Status, String> {
        self.status
            .lock()
            .map(|s| s.clone())
            .map_err(|_| "自动刷新状态锁异常。".into())
    }
    pub fn configure(&self, interval_ms: u64) -> Result<Status, String> {
        if !valid_interval(interval_ms) {
            return Err("不支持该自动刷新频率。".into());
        }
        let mut state = self.status.lock().map_err(|_| "自动刷新状态锁异常。")?;
        atomic_private(
            &self.path,
            &serde_json::to_vec(&Settings { interval_ms }).map_err(|_| "无法编码刷新设置。")?,
        )?;
        let changed = state.interval_ms != interval_ms;
        state.interval_ms = interval_ms;
        state.configured = true;
        state.error = None;
        state.version += 1;
        if changed {
            state.next_refresh_at = (interval_ms != 0).then(|| now_ms() + interval_ms);
            self.changes.send_replace(interval_ms);
        }
        Ok(state.clone())
    }
    fn update(&self, change: impl FnOnce(&mut Status), notify: &impl Fn(Status)) {
        if let Ok(mut state) = self.status.lock() {
            change(&mut state);
            state.version += 1;
            let value = state.clone();
            drop(state);
            notify(value);
        }
    }
    // Owned by the native application, never by a route or WebView timer.
    // Each batch is awaited before the next tick; missed ticks do not accumulate.
    pub async fn run<F, Fut>(self: Arc<Self>, refresh: F, notify: impl Fn(Status))
    where
        F: Fn() -> Fut,
        Fut: Future<Output = RunResult>,
    {
        let mut changes = self.changes.subscribe();
        'settings: loop {
            let interval = *changes.borrow_and_update();
            if interval == 0 {
                self.update(|s| s.next_refresh_at = None, &notify);
                if changes.changed().await.is_err() {
                    return;
                }
                continue;
            }
            let duration = Duration::from_millis(interval);
            let mut ticks = tokio::time::interval_at(Instant::now() + duration, duration);
            ticks.set_missed_tick_behavior(MissedTickBehavior::Skip);
            self.update(|s| s.next_refresh_at = Some(now_ms() + interval), &notify);
            loop {
                tokio::select! {
                    _ = ticks.tick() => {},
                    result = changes.changed() => { if result.is_err() { return } continue 'settings }
                }
                if changes.has_changed().unwrap_or(true) {
                    continue 'settings;
                }
                self.update(
                    |s| {
                        s.running = true;
                        s.started_at = Some(now_ms());
                        s.next_refresh_at = Some(now_ms() + interval);
                    },
                    &notify,
                );
                let result = refresh().await;
                self.update(
                    |s| {
                        s.running = false;
                        s.last_run = Some(result);
                    },
                    &notify,
                );
                if changes.has_changed().unwrap_or(true) {
                    continue 'settings;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    fn fixture() -> (Arc<Controller>, PathBuf) {
        let root =
            std::env::temp_dir().join(format!("atelier-auto-refresh-{}", uuid::Uuid::new_v4()));
        crate::gateway::storage::private_dir(&root).unwrap();
        (Controller::open(root.join("auto-refresh.json")), root)
    }
    #[tokio::test]
    #[ignore = "runs a real one-minute native timer with disposable data; no Desktop, credentials or network"]
    async fn real_minute_without_webview_updates_owned_cache() {
        let (controller, root) = fixture();
        controller.configure(60_000).unwrap();
        let cache = root.join("quota-fixture.json");
        std::fs::write(&cache, b"85").unwrap();
        let target = cache.clone();
        let started = Instant::now();
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let task = tokio::spawn(controller.clone().run(
            move || {
                let target = target.clone();
                let tx = tx.clone();
                async move {
                    std::fs::write(target, b"58").unwrap();
                    let _ = tx.send(started.elapsed()).await;
                    RunResult {
                        finished_at: now_ms(),
                        updated: 1,
                        ..RunResult::default()
                    }
                }
            },
            |_| {},
        ));
        let elapsed = tokio::time::timeout(Duration::from_secs(65), rx.recv())
            .await
            .unwrap()
            .unwrap();
        assert!(elapsed >= Duration::from_secs(60));
        assert_eq!(std::fs::read_to_string(cache).unwrap(), "58");
        println!("Native background refresh ran after {} ms with no WebView or account page; disposable cache changed 85 -> 58. No real account or Desktop was used.", elapsed.as_millis());
        controller.configure(0).unwrap();
        task.abort();
        let _ = task.await;
        std::fs::remove_dir_all(root).unwrap();
    }
    #[tokio::test(start_paused = true)]
    async fn one_minute_native_schedule_survives_absent_ui_and_reconfiguration() {
        let (controller, root) = fixture();
        controller.configure(60_000).unwrap();
        let count = Arc::new(AtomicUsize::new(0));
        let refreshed = count.clone();
        let task = tokio::spawn(controller.clone().run(
            move || {
                let count = refreshed.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    RunResult::default()
                }
            },
            |_| {},
        ));
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_secs(60)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.load(Ordering::SeqCst), 1);
        tokio::time::advance(Duration::from_secs(60)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.load(Ordering::SeqCst), 2);
        controller.configure(0).unwrap();
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_secs(180)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.load(Ordering::SeqCst), 2);
        assert!(
            Controller::open(root.join("auto-refresh.json"))
                .status()
                .unwrap()
                .configured
        );
        controller.configure(30_000).unwrap();
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_secs(30)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.load(Ordering::SeqCst), 3);
        assert_eq!(
            Controller::open(root.join("auto-refresh.json"))
                .status()
                .unwrap()
                .interval_ms,
            30_000
        );
        assert!(controller.configure(1).is_err());
        task.abort();
        let _ = task.await;
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn damaged_settings_stop_scheduling_without_overwriting_user_data() {
        let (controller, root) = fixture();
        std::fs::write(&controller.path, b"broken").unwrap();
        let restored = Controller::open(controller.path.clone());
        let state = restored.status().unwrap();
        assert_eq!(state.interval_ms, 0);
        assert!(state.configured && state.error.is_some());
        assert_eq!(std::fs::read_to_string(&controller.path).unwrap(), "broken");
        restored.configure(60_000).unwrap();
        assert_eq!(
            Controller::open(controller.path.clone())
                .status()
                .unwrap()
                .interval_ms,
            60_000
        );
        std::fs::remove_dir_all(root).unwrap();
    }
    #[tokio::test(start_paused = true)]
    async fn slow_requests_do_not_overlap_and_disabling_waits_only_for_inflight_work() {
        let (controller, root) = fixture();
        controller.configure(60_000).unwrap();
        let count = Arc::new(AtomicUsize::new(0));
        let refreshed = count.clone();
        let task = tokio::spawn(controller.clone().run(
            move || {
                let count = refreshed.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_secs(150)).await;
                    RunResult {
                        failed: 1,
                        error: Some("离线失败".into()),
                        ..RunResult::default()
                    }
                }
            },
            |_| {},
        ));
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_secs(60)).await;
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_secs(120)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.load(Ordering::SeqCst), 1);
        assert!(controller.status().unwrap().running);
        controller.configure(0).unwrap();
        tokio::time::advance(Duration::from_secs(30)).await;
        tokio::task::yield_now().await;
        assert!(!controller.status().unwrap().running);
        assert_eq!(
            controller
                .status()
                .unwrap()
                .last_run
                .unwrap()
                .error
                .as_deref(),
            Some("离线失败")
        );
        tokio::time::advance(Duration::from_secs(300)).await;
        tokio::task::yield_now().await;
        assert_eq!(count.load(Ordering::SeqCst), 1);
        task.abort();
        let _ = task.await;
        std::fs::remove_dir_all(root).unwrap();
    }
}
