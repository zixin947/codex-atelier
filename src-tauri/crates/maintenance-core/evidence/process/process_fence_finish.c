/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* finish @ 1007bdc3c requested 1007bdc3c */

/* codexmate_lib::platform::process::CodexTransitionFence::finish */

void codexmate_lib::platform::process::CodexTransitionFence::finish(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  char cVar2;
  undefined1 uVar3;
  undefined8 ***pppuVar4;
  uint uVar5;
  undefined8 ****ppppuVar6;
  long lVar7;
  undefined8 ****ppppuVar8;
  undefined8 *puVar9;
  undefined1 auVar10 [16];
  undefined8 ***local_1c0;
  undefined8 ***pppuStack_1b8;
  undefined8 ***pppuStack_1b0;
  undefined8 ***local_1a8;
  char *local_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  char *pcStack_188;
  undefined8 local_180;
  undefined8 uStack_178;
  char *pcStack_170;
  undefined8 ***pppuStack_168;
  undefined8 ***local_160;
  undefined8 ***pppuStack_158;
  undefined8 ***local_150;
  undefined8 ***pppuStack_148;
  char *local_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  char *pcStack_128;
  undefined8 local_120;
  undefined8 uStack_118;
  char *pcStack_110;
  undefined8 ***pppuStack_108;
  undefined8 ***local_100;
  undefined8 ***pppuStack_f8;
  undefined8 ***local_f0;
  long local_e0;
  long local_d8;
  long local_d0;
  undefined8 ***local_c8;
  undefined8 ***pppuStack_c0;
  undefined8 ***local_b8;
  undefined8 ***local_b0;
  undefined8 ***pppuStack_a8;
  undefined8 ***pppuStack_a0;
  undefined8 ***local_98;
  char *local_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  char *local_78;
  undefined8 local_70;
  undefined8 uStack_68;
  char *local_60;
  undefined8 ***pppuStack_58;
  
  *(undefined1 *)(param_2[3] + 0x10) = 1;
  ppppuVar6 = (undefined8 ****)*param_2;
  *param_2 = 0;
  if (ppppuVar6 != (undefined8 ****)0x0) {
    pppuStack_a0 = (undefined8 ***)param_2[2];
    pppuStack_a8 = (undefined8 ***)param_2[1];
    local_b0 = ppppuVar6;
    auVar10 = std::thread::lifecycle::JoinInner<T>::join(&local_b0);
    puVar9 = auVar10._8_8_;
    lVar7 = auVar10._0_8_;
    if (lVar7 != 0) {
      if ((code *)*puVar9 != (code *)0x0) {
        (*(code *)*puVar9)(lVar7);
      }
      if (puVar9[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar7,puVar9[1],puVar9[2]);
      }
      *(undefined1 *)(param_2[5] + 0x10) = 1;
    }
  }
  uVar3 = *(undefined1 *)((long)param_2 + 0x32);
  collect_codex_kill_targets(&local_b0);
  ppppuVar6 = (undefined8 ****)local_98;
  pppuStack_158 = pppuStack_a0;
  local_160 = pppuStack_a8;
  local_150 = local_98;
  if (local_b0 == (undefined8 ***)&DAT_0000000b) {
    pppuStack_f8 = pppuStack_a0;
    local_100 = pppuStack_a8;
    local_f0 = local_98;
    if ((undefined8 ****)local_98 == (undefined8 ****)0x0) {
      protected_codex_db_holders(&local_e0);
      if (local_d0 == 0) {
        pppuStack_1b8 = (undefined8 ***)((ulong)pppuStack_1b8 & 0xffffffff00000000);
        ppppuVar6 = (undefined8 ****)&DAT_0000000b;
        local_1c0 = (undefined8 ***)&DAT_0000000b;
      }
      else {
        alloc::str::join_generic_copy
                  (&local_b0,local_d8,local_d0,
                   " | ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                   ,3);
        pppuStack_158 = pppuStack_a8;
        local_160 = local_b0;
        local_150 = pppuStack_a0;
        local_b0 = &local_160;
        pppuStack_a8 = (undefined8 ***)<alloc::string::String_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_c8,&DAT_1013adcdc,&local_b0);
        if ((undefined8 ****)local_160 != (undefined8 ****)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppuStack_158,local_160,1);
        }
        pppuStack_1b0 = pppuStack_c0;
        pppuStack_1b8 = local_c8;
        local_1a8 = local_b8;
        ppppuVar6 = (undefined8 ****)&DAT_0000000a;
        local_1c0 = (undefined8 ***)&DAT_0000000a;
        puVar9 = (undefined8 *)(local_d8 + 8);
        lVar7 = local_d0;
        do {
          if (puVar9[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar9,puVar9[-1],1);
          }
          puVar9 = puVar9 + 3;
          lVar7 = lVar7 + -1;
        } while (lVar7 != 0);
      }
      if (local_e0 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_d8,local_e0 * 0x18,8);
      }
      if ((undefined8 ****)local_100 != (undefined8 ****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppuStack_f8,(long)local_100 << 5,8);
      }
    }
    else {
      enforce_codex_quiet_window_with(&local_1c0,4,1,uVar3);
      pppuVar4 = pppuStack_f8;
      ppppuVar8 = (undefined8 ****)(pppuStack_f8 + 2);
      do {
        if (ppppuVar8[-1] != (undefined8 ***)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppppuVar8,ppppuVar8[-1],1);
        }
        ppppuVar8 = ppppuVar8 + 4;
        ppppuVar6 = (undefined8 ****)((long)ppppuVar6 + -1);
      } while (ppppuVar6 != (undefined8 ****)0x0);
      ppppuVar6 = (undefined8 ****)local_1c0;
      if ((undefined8 ****)local_100 != (undefined8 ****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppuVar4,(long)local_100 << 5,8);
        ppppuVar6 = (undefined8 ****)local_1c0;
      }
    }
    if (ppppuVar6 != (undefined8 ****)&DAT_0000000b) goto LAB_1007bdea4;
    uVar5 = (uint)pppuStack_1b8;
  }
  else {
    uStack_198 = uStack_88;
    local_1a0 = local_90;
    pcStack_188 = local_78;
    uStack_190 = uStack_80;
    uStack_178 = uStack_68;
    local_180 = local_70;
    pppuStack_168 = pppuStack_58;
    pcStack_170 = local_60;
    pppuStack_1b0 = pppuStack_a0;
    pppuStack_1b8 = pppuStack_a8;
    local_1a8 = local_98;
    local_1c0 = local_b0;
LAB_1007bdea4:
    if ((*(byte *)((long)param_2 + 0x31) & 1) == 0) {
      param_1[5] = uStack_198;
      param_1[4] = local_1a0;
      param_1[7] = pcStack_188;
      param_1[6] = uStack_190;
      param_1[9] = uStack_178;
      param_1[8] = local_180;
      param_1[0xb] = pppuStack_168;
      param_1[10] = pcStack_170;
      param_1[1] = pppuStack_1b8;
      *param_1 = local_1c0;
      param_1[3] = local_1a8;
      param_1[2] = pppuStack_1b0;
      goto LAB_1007bdfac;
    }
    uStack_138 = uStack_198;
    local_140 = local_1a0;
    pcStack_128 = pcStack_188;
    uStack_130 = uStack_190;
    uStack_118 = uStack_178;
    local_120 = local_180;
    pppuStack_108 = pppuStack_168;
    pcStack_110 = pcStack_170;
    pppuStack_158 = pppuStack_1b8;
    local_160 = local_1c0;
    pppuStack_148 = local_1a8;
    local_150 = pppuStack_1b0;
    *(undefined1 *)(param_2[5] + 0x10) = 1;
    if (1 < log::MAX_LOG_LEVEL_FILTER) {
      local_c8 = &local_160;
      pppuStack_c0 = (undefined8 ***)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
      local_70 = 0x20;
      pppuStack_58 = &local_c8;
      local_60 = s_o_AiMaMi__codex_transition_fence_10152bcf2;
      local_b0 = (undefined8 ****)0x0;
      pppuStack_a8 = (undefined8 ***)0x1013ad807;
      local_98 = (undefined8 ****)0x0;
      pppuStack_a0 = (undefined8 ***)&DAT_00000020;
      local_78 = 
      "codexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
      ;
      local_90 = "src/platform/process.rs";
      uStack_80 = 2;
      uStack_88 = 0x17;
      uStack_68 = 0x74a00000001;
      <log::__private_api::GlobalLogger_as_log::Log>::log(&local_e0,&local_b0);
    }
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_160);
    uVar5 = 0;
  }
  cVar2 = *(char *)(param_2[5] + 0x10);
  iVar1 = *(uint *)(param_2[4] + 0x10) + uVar5;
  if (CARRY4(*(uint *)(param_2[4] + 0x10),uVar5)) {
    iVar1 = -1;
  }
  *(int *)(param_1 + 1) = iVar1;
  *(bool *)((long)param_1 + 0xc) = cVar2 != '\0';
  *param_1 = 0xb;
LAB_1007bdfac:
  ::core::ptr::drop_in_place<codexmate_lib::platform::process::CodexTransitionFence>(param_2);
  return;
}

