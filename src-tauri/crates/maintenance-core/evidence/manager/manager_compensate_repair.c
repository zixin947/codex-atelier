/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* compensate_repair_to_source_state @ 100686b58 requested 100686b58 */

/* codexmate_lib::core::relay::manager::RelayManager::compensate_repair_to_source_state */

void codexmate_lib::core::relay::manager::RelayManager::compensate_repair_to_source_state(long *param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  code *pcVar1;
  long lVar2;
  undefined8 *puVar3;
  long lVar4;
  long *local_e0;
  code *pcStack_d8;
  long local_d0;
  long lStack_c8;
  long local_c0;
  long lStack_b8;
  long lStack_b0;
  long lStack_a8;
  long local_a0;
  long lStack_98;
  long lStack_90;
  long lStack_88;
  long local_80;
  long local_78;
  long lStack_70;
  long local_68;
  long lStack_60;
  long local_58;
  undefined8 *local_50;
  long lStack_48;
  long local_40;

  if ((*(byte *)(param_4 + 0x16c) & 1) != 0) {
    restore_router_enabled_stably(param_1);
    return;
  }
  rollback_router_enabled_stably(&local_e0,param_2,param_3);
  lVar2 = local_d0;
  pcVar1 = pcStack_d8;
  if (local_e0 == (long *)0xb) {
    if (pcStack_d8 == (code *)0x8000000000000000) {
      *param_1 = 0xb;
    }
    else {
      local_80 = (long)pcStack_d8;
      lStack_70 = lStack_c8;
      local_78 = local_d0;
      alloc::str::join_generic_copy(&local_e0,local_d0,lStack_c8,&_anon_1f4a05fd1dc96b9982818105670d4dce_635,3);
      lStack_48 = (long)pcStack_d8;
      local_50 = local_e0;
      local_40 = local_d0;
      local_e0 = (long *)&local_50;
      pcStack_d8 = <alloc::string::String_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_68,_anon_1f4a05fd1dc96b9982818105670d4dce_690,&local_e0);
      if (local_50 != (long *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_48,local_50,1);
      }
      param_1[2] = lStack_60;
      param_1[1] = local_68;
      param_1[3] = local_58;
      *param_1 = 10;
      if (lStack_c8 != 0) {
        puVar3 = (undefined8 *)(lVar2 + 8);
        lVar4 = lStack_c8;
        do {
          if (puVar3[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar3,puVar3[-1],1);
          }
          puVar3 = puVar3 + 3;
          lVar4 = lVar4 + -1;
        } while (lVar4 != 0);
      }
      if (pcVar1 != (code *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar2,(long)pcVar1 * 0x18,8);
      }
    }
  }
  else {
    param_1[7] = lStack_a8;
    param_1[6] = lStack_b0;
    param_1[9] = lStack_98;
    param_1[8] = local_a0;
    param_1[0xb] = lStack_88;
    param_1[10] = lStack_90;
    param_1[3] = lStack_c8;
    param_1[2] = local_d0;
    param_1[5] = lStack_b8;
    param_1[4] = local_c0;
    *param_1 = (long)local_e0;
    param_1[1] = (long)pcStack_d8;
  }
  return;
}
