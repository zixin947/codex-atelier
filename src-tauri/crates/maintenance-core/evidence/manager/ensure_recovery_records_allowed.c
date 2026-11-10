/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* ensure_recovery_records_allowed @ 10033ba1c requested 10033ba1c */

/* codexmate_lib::core::relay::router_transition::ensure_recovery_records_allowed */

void codexmate_lib::core::relay::router_transition::ensure_recovery_records_allowed(long *param_1,undefined8 param_2)

{
  ulong uVar1;
  long lVar2;
  long local_90;
  long lStack_88;
  long local_80;
  long lStack_78;
  ulong local_70;
  long lStack_68;
  long local_60;
  long lStack_58;
  long local_50;
  long lStack_48;
  long local_40;
  long lStack_38;

  transition_journal::read_checked(&local_90);
  lVar2 = lStack_68;
  uVar1 = local_70;
  if (local_90 != 0xb) {
    param_1[9] = lStack_48;
    param_1[8] = local_50;
    param_1[0xb] = lStack_38;
    param_1[10] = local_40;
    param_1[1] = lStack_88;
    param_1[2] = local_80;
    param_1[4] = local_70;
    param_1[3] = lStack_78;
    param_1[5] = lStack_68;
    param_1[7] = lStack_58;
    param_1[6] = local_60;
    *param_1 = local_90;
    return;
  }
  if (lStack_88 != -0x8000000000000000) {
    if (lStack_88 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_80,lStack_88,1);
    }
    if ((uVar1 & 0x7fffffffffffffff) != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar2,uVar1,1);
      codex_thread_visibility::validate_model_restore_journals(&local_90,param_2);
      goto joined_r0x00010033bafc;
    }
  }
  codex_thread_visibility::validate_model_restore_journals(&local_90,param_2);
joined_r0x00010033bafc:
  if (local_90 != 0xb) {
    param_1[5] = lStack_68;
    param_1[4] = local_70;
    param_1[7] = lStack_58;
    param_1[6] = local_60;
    param_1[9] = lStack_48;
    param_1[8] = local_50;
    param_1[0xb] = lStack_38;
    param_1[10] = local_40;
    param_1[1] = lStack_88;
    *param_1 = local_90;
    param_1[3] = lStack_78;
    param_1[2] = local_80;
    return;
  }
  config_takeover::validate_recovery_backups(param_1,param_2);
  return;
}
