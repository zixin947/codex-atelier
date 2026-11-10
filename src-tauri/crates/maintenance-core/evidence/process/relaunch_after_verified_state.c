/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* relaunch_after_verified_state @ 10068e464 requested 10068e464 */

/* codexmate_lib::core::relay::manager::CodexRelaunchPolicy::relaunch_after_verified_state */

void codexmate_lib::core::relay::manager::CodexRelaunchPolicy::relaunch_after_verified_state(undefined8 *param_1,uint param_2,ulong param_3,int param_4)

{
  int iVar1;
  
  if ((((param_4 != 0) && ((param_2 & 1) != 0)) && (iVar1 = platform::process::is_codex_app_running(), (param_3 & 1) != 0)) && (iVar1 == 0)) {
    launch_codex_app_warning(param_1);
    return;
  }
  *param_1 = 0x8000000000000000;
  return;
}

