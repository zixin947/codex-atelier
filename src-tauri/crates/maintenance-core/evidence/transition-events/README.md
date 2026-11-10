# Transition event log recovery

Exact input: installed AiMaMi 1.2.7 arm64, SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`. Only static binary/C/ASM analysis and synthetic temporary-file tests were used. No real client, user home, auth, Keychain or session data was touched.

`../../src/transition_events.rs` implements the actual classification, JSON serialization, line retention, current UTC millisecond clock and local filesystem publication. It reuses the already recovered `catalog::publication::write_atomic_report` implementation, including inherited target permissions, atomic staging/rename/sync and acknowledge semantics. `../../tests/transition_events_native.rs` contains 13 synthetic tests, including real temporary file publication and fault-injected publication/directory-sync failures. No root journal code was edited by this task.

## Fields and side effects

| Conclusion | Exact native evidence |
| --- | --- |
| append_event takes explicit relay root, target, raw phase, optional source and optional reason. Phase is copied verbatim; chrono UTC now is converted to epoch milliseconds | `journal_append_event.c:146-178`, `:1051-1065`; function `10078ebd8` |
| schemaVersion is integer1; field order is schemaVersion,target,phase,occurredAtMs, optional source,reason | `journal_append_event.c:1066-1114`; `event-fields.asm` @ `10078fac0..10078fd1c` |
| Optional field249 is **source**, not status. Its 6 literal bytes at `1013a9e97` are `source` | `source-field.asm` @ `10078fffc..100790028` (load at `100790004/008`, length6 at `100790014`); `constants.json` |
| source=None omits the key, source=Some("") emits other; reason=None always emits JSON null; reason=Some("") emits unknown | `journal_append_event.c:168-186`, `:535-668`, `:1099-1114` |
| New source and reason are replaced by fixed classification labels, with ASCII-only lowercasing and substring matching. No raw source/reason text is truncated/copied into a new event | `journal_append_event.c:178-535`, `:535-1049`; classification details below |
| Filename is exactly router-events.jsonl (19 bytes) | `journal_append_event.c:1123`; binary literal at `1013aae66` |
| Any previous read_to_string error, including invalid UTF-8, means empty previous log | `journal_append_event.c:1124-1148`; call `10078fd28`, sentinel branch `10078fd34/38` |
| Existing text is only Rust str.lines().map(str::to_owned), preserving whitespace, empty and invalid-JSON lines. CRLF terminators lose CR, standalone CR remains. Prior lines are not JSON-decoded or sanitized | `existing-lines.asm` @ `10050b1d0..10050b318`, called through Vec from_iter `100532d80` by `10078fd68` |
| After appending one JSON line, drop the oldest entries until at most200 remain; blank lines count. Join with LF and add exactly one trailing LF | `journal_append_event.c:1154-1180`; count `0xc8` and drain at `:1160-1163` |
| write_atomic_report error is discarded; successful publication calls Publication::acknowledge (directory-sync warning is consumed/logged), does not require_sync | `journal_append_event.c:1181-1200`; native report implementation `1008a5300`, acknowledgement `1008a4e78` (existing catalog evidence) |
| Serialization failure is also discarded; event append does not fail the journal transition | `journal_append_event.c:1215-1226`; serialization error edges throughout `:1085-1117` |

## Source classification, in precedence order

All source tests use the ASCII-lowercased string. No word-boundary checks, whitespace normalization, underscore aliases or Unicode case folding are added.

| Any matching substring | Result | Evidence |
| --- | --- | --- |
| rollback, compensation | compensation | C `:438-464, :516-519` |
| diagnostic | diagnostic | C `:449-451, :465-469, :509-512` |
| bootstrap | bootstrap | C `:452-460, :470-472`; literal `1013ab261` |
| auth-mode | auth_mode | C `:473-475, :503-506`; literals `1013ab26a`, `1013ab2b0` |
| model slots | model_slots | C `:476-482, :499-501`; literals `1013ab273`, `1013ab2a5` |
| router toggle | toggle | C `:483-493`; literals `1013ab27e`, `1013ab29f` |
| repair, reconcile | repair | C `:388-436`; literals `1013ab28b`, `1013ab291` |
| otherwise, including empty | other | C `:183-186, :429-432`; literal `1013ab29a` |

For example, `auth_mode`, `model_slots` and `reconciling` are other, because native input matching uses `auth-mode`, `model slots` and `reconcile`. This is preserved rather than normalized into an invented alias.

## Reason classification, in precedence order

English tests use ASCII-lowercased input; Chinese tests use the original string (ASCII lower leaves these Chinese bytes unchanged). Each row's first matching category wins. `constants.json` contains decoded VA/byte-length/text evidence for all C constant pointers, including the anonymous thread/backup/project/token/proxy/port labels.

| Any matching substring | Result | Evidence |
| --- | --- | --- |
| no space; disk full; enospc; insufficient space; write capacity; storage capacity; 空间不足; 磁盘已满 | storage_capacity | C `:866-946, :1025-1036`; storage literal `1013af120`, keyword literals `1013ab00f..1013ab04a` |
| database is locked; busy; db holder; held by; 数据库被锁; 数据库锁; 数据库占用 | db_lock | C `:949-1022`, plus `:545-546`; output `1013ab250`, keywords `1013ab056..1013ab093`, `1013af314` |
| taskkill; force close; force-close; quiet window; transition fence; re-entry; reentry; failed to stop; still running; 无法关闭进程; 进程仍在运行; 反复启动 | process_control | C `:547-558, :655-659`; output `1013ab241`; transition-fence literal `1013af130` |
| thread; rollout; 线程 | thread_reconcile | C `:559-561, :650-653`; thread literal `1013a9f2f` |
| takeover; backup; 接管; 备份 | config_takeover | C `:562-569`; output `1013ab232` |
| model catalog; model_catalog; catalog; 模型目录; 模型列表 | model_catalog | C `:571-576, :644-648` |
| project; workspace root; 项目; 工作区 | project_state | C `:577-582, :641-648`; output `1013ab225` |
| config; toml; managed block; 配置 | config_sync | C `:582-587, :636-640`; output `1013ab21a`; toml `1013af318` |
| auth; login; token; 认证; 登录; 令牌 | auth | C `:587-594, :632-634`; auth `1013082d4` |
| persist; relay state; 持久化; 路由状态 | state_persist | C `:595-601, :644-648`; output `1013ab20d` |
| proxy; port; 代理; 端口 | proxy | C `:603-608, :626-629`; output `1013ab1e1` |
| completed; already stable | completed | C `:609-624`; output `1013ab1f6` |
| otherwise, including empty | unknown | C `:610-620`; output `1013a9d07` |

Substring quirks remain native: for example, `unsupported` contains `port` and therefore classifies proxy; no-space with a hyphen is not added as a storage-capacity alias. A source/reason's finite output labels do not imply that arbitrary preexisting log lines or raw phase text have been sanitized.

## API and integration

`append_event_at(relay_root,target,phase,source,reason,occurred_at_ms)` performs actual local file I/O with explicit time. `append_event` supplies the actual current epoch millisecond time without any client or network operation. `append_event_with_io` permits synthetic read/write failures, and `render_append` permits pure deterministic verification. TransitionTarget is reused from root's real transition_journal module.

AppendOutcome is intentionally not a Result that callers should propagate as a transition error. Written exposes the acknowledged publication and optional directory-sync warning kind; Dropped identifies an unpersisted audit event. This receipt makes the native best-effort loss visible without returning a false persistence success. No stronger serialization/file-read/concurrency gate is invented. Original root journal phase-write, pending-refusal and clear call sites are owned and connected by root.

Verification completed: `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test transition_events_native` — 13 passed; rustfmt check and owned-file whitespace checks. Actual filesystem tests use only newly allocated temporary roots. Complete outside-client E2E is unrun by instruction.
