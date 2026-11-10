# AiMaMi 1.2.7 macOS process lifecycle

Input: static `AiMaMi.arm64`, SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`. Addresses below belong to that exact installed binary. C files are read-only Ghidra exports from the parent batch, copied into this evidence directory. ASM files are local `llvm-objdump` exports. No real process enumeration, `lsof`, signal, quit, open, user auth/config/session/keychain access was executed during recovery or tests.

Implementation: `../../src/process.rs`. Tests: `../../tests/process_native.rs`; all runner outputs, process clocks, directory inventories and file contents are synthetic. The actual named Rust worker is also tested with a mutex-wrapped synthetic runner. `SystemRunner` compiles on macOS but its external operations were deliberately not invoked. Non-macOS output/signal methods return Unsupported; this binary provides evidence for macOS only.

## Native implementation map

| Recovered behavior | Native evidence |
| --- | --- |
| Exact executable match requires equality or whitespace boundary; process name is first whitespace word's final slash component | `command_runs_executable` @ `1007c09ac`; `extract_process_name` @ `1007be050`; `process_related_command.c` @ `1007c65cc` |
| Writer classification: normalized separators + ASCII lower; ChatGPT.app/Contents first; official extension CLI, other IDE roots, global CLI executable/script origin paths | `process_classify_writer.c` @ `1007c1870` (normalization line 139; Desktop match line 483; package-origin checks lines 1284, 1673, 1734) |
| Protection includes official extension Codex basename, IDE extension roots, Cursor, computer-use, AiMaMi and crashpad exclusions; related matching then checks validated main/bundle prefix, legacy Codex.app/Helper and Codex/Node script names | `process_protected_command.c` @ `1007c34bc`; `process_related_command.c` @ `1007c65cc`; official extension path helper @ `1007c7910` |
| `ps -ax -o pid=,command=` is checked, self PID excluded, full command retained for ownership; all related rows become kill targets with extracted display names; DB holders are merged/deduped | `process_list_matching.c` @ `1007c40b8`; `process_collect_targets.c` @ `1007c1318`; `process_target_filter_fold.c` @ `100510efc` |
| External CLI collection is separate from protection. Official extension CLI can be protected against kill and still reject policy1 | `process_collect_external.c` @ `1007c4890`; classification above |
| Policy0 includes every native related target, including external CLI. Policy1 rejects external CLI first, then only keeps raw command ASCII-lower containing `/chatgpt.app/contents/`; it does not broaden/narrow using another classifier | `process_quiesce_targets.c` @ `1007c5fe0`; `desktop_filter.asm` @ `10050e63c`, called through @ `100610730` |
| Protected DB holders mean **all** holders failing native related-with-main, including unknown ownership, not just explicit protection matches | `protected_db_filter.asm` @ `100511130`, called via @ `10060f818` by @ `1007c1770` |
| Runtime DB search is configured sqlite_home exclusively, otherwise home/sqlite then home; config valid nonempty TOML string wins over nonempty CODEX_SQLITE_HOME; relative config/env paths use cwd; no tilde/canonicalization | `state_db_search_dirs.c` @ `1008ee7b8`; `configured_sqlite_home.c` @ `1008ee934`; `runtime_state_db_candidates_for_home.c` @ `1008ef428` |
| Runtime DB candidates require is_file + exact `state_<u32>.sqlite`, permit Rust u32 leading plus, dedup Path equality, sort numeric version descending then Path ascending | `runtime_state_db_candidates_for_home.c` lines 175–204, 318–328; `runtime_sort.asm` @ `1008fc818` |
| Holder probe includes existing main/-wal/-shm paths, fixed `lsof -t --`; lsof launch failure is empty, nonzero status stdout still parsed; PID dedup/self exclusion; fixed per-PID ps lookup skips failure/empty | `process_db_holders_for.c` @ `1007c6c44`; `process_list_db_holders.c` @ `1007c4db0` |
| Actual kill is libc `kill(pid as i32,9)`; errno 3/ESRCH succeeds; others aggregated after attempting every target | `kill_signal.asm` @ `1007e6ec0`; `process_force_kill_targets_iter.c` @ `10053adf0` |
| Force close snapshots report targets then 3s deadline, 300ms retries; clears cached signal error after success, checks protected holders only once related writers empty | `process_force_close_all.c` @ `1007be5c8`; `process_kill_until_clear.c` @ `1007c5960` |
| Prepare rejects policy1 CLI before mutation; only if app running sends AppleScript quit and waits requested grace at 100ms; quit errors ignored; then native policy-specific 3s/300ms force fallback | `process_prepare_transition.c` @ `1007be88c` (policy branch lines 80–158; graceful loop lines 471–561) |
| Generation uses sorted unique PID set: empty resets, first after seen-empty/new PID is respawn, successful quiesce suppresses repeated signals, failed same generation retries at most 3 | `process_generation_observe.c` @ `1007c05c4`; `process_enforce_quiet.c` @ `1007c5178`, lines 127–173 |
| Quiet window requires continuous 700ms empty, 80ms samples, protected holder check before success, pre-collection deadline timestamp; empty timeout/unreleased host/orphan writers have separate actual error codes | `process_enforce_quiet.c` @ `1007c5178`; `quiet_timeout.asm` @ `1007c4644` |
| Fence strict prepare+12s quiet; best effort absorbs prepare/2s quiet error into taint; initial quiet respawns are only logged, not reported in final counter; worker starts named codex-transition-fence or returns start error | `process_fence_acquire.c` @ `1007bd574` |
| Worker starts seen_empty=true, samples at 80ms, marks taint for any collection or quiesce failure, wraps u32 respawn count, continues observing without repeatedly signaling persistent successful/exhausted generation | `process_fence_worker.c` + `fence_worker.asm` @ `10044aaec` (actual inlined closure); spawn vtable `10181cf60` slot 4 -> `10045b538` -> `10044aaec` |
| Finish stops/joins (panic taints), final empty checks holders or nonempty 4s quiet with initially_quiet=true; strict final errors propagate, best effort taints; background + final counters saturating-add. Drop only stops/joins | `process_fence_finish.c` @ `1007bdc3c`; `fence_drop.asm` @ `100465a28` |
| Bundle roots /Applications then explicit user home/Applications, file ChatGPT.app; fallback exact mdfind bundle-ID query sorted/deduped; validates case-insensitive com.openai.codex and CFBundleExecutable is_file | `process_bundle_path.c` @ `1007bf9a4`; `process_bundle_main_executable.c` @ `1007c6254`; `process_read_bundle_value.c` @ `1007c0b4c` |
| Running probe validates main process first, fallback exact osascript application-id query; launch first open, up to10s main-PID observation at100ms (one final probe), second open activation; either open failure propagates | `process_is_running.c` @ `1007be1e4`; `process_main_pid.c` @ `1007c2f64`; `process_launch.c` @ `1007bf414` lines 218–274; `process_open_bundle.c` @ `1007c00cc` |
| Relaunch policy captures requested flag and measured prior running state; only after actual verification success + requested + prior running + currently stopped does it call launch-warning | `relaunch_capture.asm` @ `100680d90` (requested from function parameter2 via sp+0x77 -> sp+0x50; is_running -> sp+0x4c); `relaunch_after_verified_state.c` + `relaunch_policy.asm` @ `10068e464`; launch warning @ `10068ee30` |

## Fixed external commands

`Operation::argv()` is a closed enum: no caller-supplied script or executable, no shell invocation. Bundle/path arguments remain individual OsString argv elements even with spaces/metacharacters.

- `ps -ax -o pid=,command=`
- `ps -p <u32> -o command=`
- `/usr/bin/mdfind "kMDItemCFBundleIdentifier == 'com.openai.codex'"`
- `/usr/bin/plutil -extract CFBundleIdentifier|CFBundleExecutable raw -o - <bundle>/Contents/Info.plist`
- `/usr/bin/osascript -e 'application id "com.openai.codex" is running'`
- `/usr/bin/osascript -e 'if application id "com.openai.codex" is running then\n    tell application id "com.openai.codex" to quit\nend if'`
- `/usr/bin/open <validated bundle>`
- `lsof -t -- <existing DB and WAL/SHM paths>`
- libc `kill(<native selected PID>,9)`.

## Integration contract

Use an explicit ProcessContext; the constructor does not discover/read a real home. Production adapter supplies codex_home, optional actual user_home, optional captured CODEX_SQLITE_HOME, and cwd. Config parsing and runtime DB enumeration execute inside the engine rather than trusting a caller's known-status bool. `preview_stop` inventories native targets, external CLI and protected DB holders. An external CLI listed under policy0 is an actual native kill target when it also passes related/protection rules; policy1 rejects CLI before quit/signal.

`CodexRelaunchPolicy::capture` measures prior running state before stopping. `CodexTransitionFence::acquire(Arc<Runner>, context, grace, Strict|BestEffort, ForceCloseAll|DesktopTransition)` executes stop/initial quiet and starts the worker. Manager retains it while applying its reviewed transaction and checking its actual post-state; `finish` returns the real native respawn/taint metadata. Pass that Manager verification Result to `after_verification_result`; failed verification is never converted into a launch. Relaunch failure returns Warning because native Manager presents launch failure as a warning after an otherwise verified mutation.

Business post-state verification, preview confirmation, UI presentation and transaction rollback remain the Manager's responsibility. This module does not claim that a completed fence verifies business state. It preserves native behavior where a strict fence can finish successfully with taint metadata from an earlier background failure. It does not invent a stronger success gate.

## Version boundary and remaining limits

The existing public reference at `aimami-unpacked/src-tauri/src/platform/process.rs` declares package **1.0.0**, and uses older name-kill/discovery behavior. It does not contain this 1.2.7 classifier/fence/policy implementation and was not treated as native proof. The installed implementation specifically searches ChatGPT.app with com.openai.codex bundle validation; legacy Codex.app command matching is retained separately.

Remaining presentation differences: original debug_log/log messages, CoreError enum transport, localized force-kill/external-writer display text and launch-warning string formatting are represented by typed ProcessError/Outcome rather than byte-identical logging/UI output. The exact control-flow error boundaries and distinct quiet timeout codes are implemented. SystemRunner was compile-checked only; actual macOS command execution, app discovery/signals/quit/launch and Desktop E2E are untested by instruction. Library/app registration is intentionally left to root's shared adapter ownership; this task only writes the assigned new module, tests and evidence directory.

Validation: `cargo test --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test process_native` (34 tests); rustfmt check on the two new Rust files; isolated whitespace checks on the new Rust files and README. No real process operations were used by these tests.
