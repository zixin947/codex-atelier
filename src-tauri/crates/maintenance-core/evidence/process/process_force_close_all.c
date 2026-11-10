/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* force_close_all @ 1007be5c8 requested 1007be5c8 */

/* codexmate_lib::platform::process::CodexProcessLifecycle::force_close_all */

void codexmate_lib::platform::process::CodexProcessLifecycle::force_close_all(long *param_1)

{
  ulong uVar1;
  undefined1 auVar2 [16];
  long lVar3;
  ulong uVar4;
  undefined8 *puVar5;
  long lVar6;
  long lVar7;
  undefined1 auVar8 [16];
  undefined1 local_120 [12];
  long local_110;
  long local_f0;
  long lStack_e8;
  long local_e0;
  undefined4 local_cc;
  undefined8 local_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  long local_a8;
  long lStack_a0;
  long local_98;
  long lStack_90;
  long lStack_88;
  long lStack_80;
  long local_78;
  long lStack_70;
  undefined1 auStack_68 [24];
  long local_50;
  long lStack_48;
  
  local_120 = std::time::Instant::now();
  collect_codex_kill_targets(&local_c8);
  local_cc = uStack_b0;
  lVar3 = CONCAT44(uStack_b8._4_4_,(undefined4)uStack_b8);
  local_110 = CONCAT44(uStack_bc,uStack_c0);
  lVar6 = CONCAT44(uStack_ac,uStack_b0);
  if (CONCAT44(local_c8._4_4_,(undefined4)local_c8) == 0xb) {
    local_f0 = local_110;
    lStack_e8 = lVar3;
    local_e0 = lVar6;
    <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_f0,lVar3,lVar3 + lVar6 * 0x20);
    kill_codex_processes_until_clear(&local_c8);
    if (CONCAT44(local_c8._4_4_,(undefined4)local_c8) == 0xb) {
      auVar8 = std::time::Instant::elapsed(local_120);
      auVar2._8_8_ = 0;
      auVar2._0_8_ = auVar8._0_8_;
      lStack_48 = SUB168(auVar2 * ZEXT816(1000),8);
      uVar4 = auVar8._0_8_ * 1000;
      uVar1 = (auVar8._8_8_ & 0xffffffff) / 1000000;
      local_50 = uVar4 + uVar1;
      if (CARRY8(uVar4,uVar1)) {
        lStack_48 = lStack_48 + 1;
      }
      local_c8 = &local_cc;
      uStack_c0 = 0x117b2ec;
      uStack_bc = 1;
      uStack_b8 = &local_50;
      uStack_b0 = 0x117b8e8;
      uStack_ac = 1;
      alloc::fmt::format::format_inner(auStack_68,s_count__elapsed_ms__10152bd64,&local_c8);
      debug_log::app_event
                ("codex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                 ,0x17,
                 "force_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                 ,0x14,auStack_68);
      uStack_bc = (undefined4)lStack_e8;
      uStack_b8._0_4_ = (undefined4)((ulong)lStack_e8 >> 0x20);
      local_c8._4_4_ = (undefined4)local_f0;
      uStack_c0 = (undefined4)((ulong)local_f0 >> 0x20);
      uStack_b8._4_4_ = (undefined4)local_e0;
      uStack_b0 = (undefined4)((ulong)local_e0 >> 0x20);
      *(undefined4 *)(param_1 + 1) = local_cc;
      *(ulong *)((long)param_1 + 0x14) = CONCAT44(uStack_bc,uStack_c0);
      *(ulong *)((long)param_1 + 0xc) = CONCAT44(local_c8._4_4_,(undefined4)local_c8);
      param_1[4] = local_e0;
      param_1[3] = lStack_e8;
      *param_1 = 0xb;
      if (lVar6 != 0) {
        puVar5 = (undefined8 *)(lVar3 + 0x10);
        do {
          if (puVar5[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar5,puVar5[-1],1);
          }
          puVar5 = puVar5 + 4;
          lVar6 = lVar6 + -1;
        } while (lVar6 != 0);
      }
      if (local_110 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar3,local_110 << 5,8);
      }
    }
    else {
      param_1[5] = lStack_a0;
      param_1[4] = local_a8;
      param_1[7] = lStack_90;
      param_1[6] = local_98;
      param_1[9] = lStack_80;
      param_1[8] = lStack_88;
      param_1[0xb] = lStack_70;
      param_1[10] = local_78;
      param_1[1] = CONCAT44(uStack_bc,uStack_c0);
      *param_1 = CONCAT44(local_c8._4_4_,(undefined4)local_c8);
      param_1[3] = CONCAT44(uStack_ac,uStack_b0);
      param_1[2] = CONCAT44(uStack_b8._4_4_,(undefined4)uStack_b8);
      if (local_e0 != 0) {
        puVar5 = (undefined8 *)(lStack_e8 + 8);
        lVar7 = local_e0;
        do {
          if (puVar5[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar5,puVar5[-1],1);
          }
          puVar5 = puVar5 + 3;
          lVar7 = lVar7 + -1;
        } while (lVar7 != 0);
      }
      if (local_f0 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_e8,local_f0 * 0x18,8);
      }
      if (lVar6 != 0) {
        puVar5 = (undefined8 *)(lVar3 + 0x10);
        do {
          if (puVar5[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar5,puVar5[-1],1);
          }
          puVar5 = puVar5 + 4;
          lVar6 = lVar6 + -1;
        } while (lVar6 != 0);
      }
      if (local_110 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar3,local_110 << 5,8);
        return;
      }
    }
  }
  else {
    param_1[5] = lStack_a0;
    param_1[4] = local_a8;
    param_1[7] = lStack_90;
    param_1[6] = local_98;
    param_1[9] = lStack_80;
    param_1[8] = lStack_88;
    param_1[0xb] = lStack_70;
    param_1[10] = local_78;
    param_1[2] = lVar3;
    param_1[1] = local_110;
    param_1[3] = lVar6;
    *param_1 = CONCAT44(local_c8._4_4_,(undefined4)local_c8);
  }
  return;
}

