/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* commit_reconciled_target @ 100680790 requested 100680790 */

/* codexmate_lib::core::relay::manager::RelayManager::commit_reconciled_target */

void codexmate_lib::core::relay::manager::RelayManager::commit_reconciled_target
               (undefined8 *param_1,undefined8 param_2,int *param_3,undefined1 param_4,undefined8 *param_5)

{
  undefined8 *puVar1;
  undefined1 *puVar2;
  undefined1 auStack_3b0 [364];
  undefined1 local_244;
  undefined1 local_232;
  undefined8 local_230;
  undefined8 uStack_228;
  undefined8 local_220;
  undefined1 auStack_218 [364];
  undefined1 local_ac;
  undefined1 local_9a;
  long local_98 [3];
  undefined1 local_80;
  undefined7 uStack_7f;
  undefined1 uStack_78;
  undefined7 uStack_77;
  undefined1 uStack_70;
  undefined7 uStack_6f;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined1 *local_50;
  code *pcStack_48;

  puVar2 = auStack_3b0;
  if (*param_3 == 1) {
    if ((*(byte *)(param_3 + 0xe) & 1) == 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x50,1);
      if (puVar1 != (undefined8 *)0x0) {
        puVar1[5] = 0x6843206465696669;
        puVar1[4] = 0x726576206f6e2065;
        puVar1[7] = 0x646e697720657469;
        puVar1[6] = 0x7277205450477461;
        puVar1[9] = 0x656c62616c696176;
        puVar1[8] = 0x612073617720776f;
        puVar1[1] = 0x6f6e207361772066;
        *puVar1 = 0x664f65766974614e;
        puVar1[3] = 0x7375616365622064;
        puVar1[2] = 0x65696c7070612074;
        param_1[1] = 0x50;
        *param_1 = 10;
        param_1[2] = puVar1;
        param_1[3] = 0x50;
        return;
      }
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x50);
    }
    snapshot(auStack_218,param_2);
    local_ac = 0;
    local_9a = param_4;
    codex_config_stale_reason_for_snapshot(local_98,param_2,auStack_218);
    if (local_98[0] != -0x8000000000000000) {
      uStack_78 = (undefined1)local_98[1];
      uStack_77 = (undefined7)((ulong)local_98[1] >> 8);
      local_80 = (undefined1)local_98[0];
      uStack_7f = (undefined7)((ulong)local_98[0] >> 8);
      uStack_70 = (undefined1)local_98[2];
      uStack_6f = (undefined7)((ulong)local_98[2] >> 8);
      local_50 = &local_80;
      pcStack_48 = <alloc::string::String_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_68,s_0NativeOff_config_still_contains_101528786,&local_50);
      param_1[2] = uStack_60;
      param_1[1] = local_68;
      param_1[3] = local_58;
      *param_1 = 10;
      if (CONCAT71(uStack_7f,local_80) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(CONCAT71(uStack_77,uStack_78),CONCAT71(uStack_7f,local_80),1);
      }
      puVar2 = auStack_218;
LAB_10068099c:
      ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayState>(puVar2);
      return;
    }
    uStack_77 = (undefined7)param_5[1];
    uStack_70 = (undefined1)((ulong)param_5[1] >> 0x38);
    uStack_7f = (undefined7)*param_5;
    uStack_78 = (undefined1)((ulong)*param_5 >> 0x38);
    puVar2 = auStack_218;
  }
  else {
    snapshot(auStack_3b0,param_2);
    local_244 = 1;
    local_232 = (undefined1)param_3[0xe];
    codex_config_stale_reason_for_snapshot(local_98,param_2,auStack_3b0);
    if (local_98[0] != -0x8000000000000000) {
      uStack_78 = (undefined1)local_98[1];
      uStack_77 = (undefined7)((ulong)local_98[1] >> 8);
      local_80 = (undefined1)local_98[0];
      uStack_7f = (undefined7)((ulong)local_98[0] >> 8);
      uStack_70 = (undefined1)local_98[2];
      uStack_6f = (undefined7)((ulong)local_98[2] >> 8);
      local_50 = &local_80;
      pcStack_48 = <alloc::string::String_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_230,s_9RouterOn_config_does_not_match_t_10152874a,&local_50);
      param_1[2] = uStack_228;
      param_1[1] = local_230;
      param_1[3] = local_220;
      *param_1 = 10;
      puVar2 = auStack_3b0;
      if (CONCAT71(uStack_7f,local_80) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(CONCAT71(uStack_77,uStack_78),CONCAT71(uStack_7f,local_80),1);
        puVar2 = auStack_3b0;
      }
      goto LAB_10068099c;
    }
    uStack_77 = (undefined7)param_5[1];
    uStack_70 = (undefined1)((ulong)param_5[1] >> 0x38);
    uStack_7f = (undefined7)*param_5;
    uStack_78 = (undefined1)((ulong)*param_5 >> 0x38);
  }
  local_80 = 1;
  persist_with_commit(param_1,param_2,puVar2,&local_80);
  return;
}
