/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* acquire_with_policy @ 1007bd574 requested 1007bd574 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::platform::process::CodexTransitionFence::acquire_with_policy */

void codexmate_lib::platform::process::CodexTransitionFence::acquire_with_policy
               (undefined8 *param_1,undefined8 param_2,undefined8 param_3,byte param_4,undefined8 param_5)

{
  long *plVar1;
  long *plVar2;
  code *pcVar3;
  long lVar4;
  long *plVar5;
  char *pcVar6;
  undefined8 uVar7;
  bool bVar8;
  undefined1 uVar9;
  undefined8 *puVar10;
  int local_1d4;
  char ******local_1d0;
  char *****pppppcStack_1c8;
  char ******local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  ulong uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  char ******local_170;
  code *pcStack_168;
  char ******local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  ulong uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  long *local_110;
  long *local_108;
  long *local_100;
  long *local_f8;
  long *local_f0;
  long *local_e8;
  long *local_e0;
  long *plStack_d8;
  long *local_d0;
  undefined8 uStack_c8;
  char *local_c0;
  ulong local_b8;
  undefined8 uStack_b0;
  char *local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  char *local_90;
  char *******pppppppcStack_88;
  char *******local_80;
  code *pcStack_78;
  undefined8 local_70;
  
  CodexProcessLifecycle::prepare_transition(&local_1d0,param_2,param_3,param_5);
  bVar8 = local_1d0 != (char ******)&DAT_0000000b;
  if (local_1d0 == (char ******)&DAT_0000000b) {
    if ((param_4 & 1) != 0) goto LAB_1007bd66c;
    enforce_codex_quiet_window_with(&local_1d0,0xc,0,param_5);
    if (local_1d0 != (char ******)&DAT_0000000b) goto LAB_1007bd820;
  }
  else {
    if ((param_4 & 1) == 0) {
LAB_1007bd820:
      param_1[5] = uStack_1a8;
      param_1[4] = local_1b0;
      param_1[7] = uStack_198;
      param_1[6] = uStack_1a0;
      param_1[9] = uStack_188;
      param_1[8] = local_190;
      param_1[0xb] = uStack_178;
      param_1[10] = uStack_180;
      param_1[1] = pppppcStack_1c8;
      *param_1 = local_1d0;
      param_1[3] = uStack_1b8;
      param_1[2] = local_1c0;
      return;
    }
    uStack_148 = uStack_1a8;
    local_150 = local_1b0;
    uStack_138 = uStack_198;
    uStack_140 = uStack_1a0;
    uStack_128 = uStack_188;
    local_130 = local_190;
    uStack_118 = uStack_178;
    uStack_120 = uStack_180;
    pcStack_168 = (code *)pppppcStack_1c8;
    local_170 = local_1d0;
    uStack_158 = uStack_1b8;
    local_160 = local_1c0;
    if (1 < log::MAX_LOG_LEVEL_FILTER) {
      pcStack_78 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
      local_a8 = 
      "codexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
      ;
      uStack_a0 = 0x20;
      pppppppcStack_88 = (char *******)&local_80;
      local_90 = s___AiMaMi__codex_transition_fence_10152bb84;
      local_e0 = (long *)0x0;
      plStack_d8 = (long *)0x1013ad807;
      uStack_c8 = 0;
      local_d0 = (long *)0x20;
      local_c0 = "src/platform/process.rs";
      uStack_b0 = 2;
      local_b8 = 0x17;
      local_98 = 0x6c500000001;
      local_80 = &local_170;
      <log::__private_api::GlobalLogger_as_log::Log>::log(&local_e8,&local_e0);
    }
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_170);
LAB_1007bd66c:
    enforce_codex_quiet_window_with(&local_1d0,2,0,param_5);
    if (local_1d0 != (char ******)&DAT_0000000b) {
      uStack_148 = uStack_1a8;
      local_150 = local_1b0;
      uStack_138 = uStack_198;
      uStack_140 = uStack_1a0;
      uStack_128 = uStack_188;
      local_130 = local_190;
      uStack_118 = uStack_178;
      uStack_120 = uStack_180;
      pcStack_168 = (code *)pppppcStack_1c8;
      local_170 = local_1d0;
      uStack_158 = uStack_1b8;
      local_160 = local_1c0;
      if (1 < log::MAX_LOG_LEVEL_FILTER) {
        pcStack_78 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        local_a8 = 
        "codexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
        ;
        uStack_a0 = 0x20;
        pppppppcStack_88 = (char *******)&local_80;
        local_90 = s_r_AiMaMi__codex_transition_fence_10152bbe6;
        local_e0 = (long *)0x0;
        plStack_d8 = (long *)0x1013ad807;
        uStack_c8 = 0;
        local_d0 = (long *)0x20;
        local_c0 = "src/platform/process.rs";
        uStack_b0 = 2;
        local_b8 = 0x17;
        local_98 = 0x6de00000001;
        local_80 = &local_170;
        <log::__private_api::GlobalLogger_as_log::Log>::log(&local_e8,&local_e0);
      }
      ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_170);
      uVar9 = 0;
      local_1d4 = 0;
      bVar8 = true;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      local_110 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
      goto joined_r0x0001007bd818;
    }
  }
  local_1d4 = (int)pppppcStack_1c8;
  if (((int)pppppcStack_1c8 == 0) || (log::MAX_LOG_LEVEL_FILTER < 2)) {
    uVar9 = 1;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    local_110 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
  }
  else {
    local_170 = (char ******)&local_1d4;
    pcStack_168 = u32>::fmt;
    local_a8 = 
    "codexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
    ;
    uStack_a0 = 0x20;
    local_90 = s_l_AiMaMi__codex_transition_fence_10152bc5b;
    local_e0 = (long *)0x0;
    plStack_d8 = (long *)0x1013ad807;
    uStack_c8 = 0;
    local_d0 = (long *)0x20;
    local_c0 = "src/platform/process.rs";
    uStack_b0 = 2;
    local_b8 = 0x17;
    local_98 = 0x6e600000001;
    pppppppcStack_88 = &local_170;
    <log::__private_api::GlobalLogger_as_log::Log>::log(&local_e8,&local_e0);
    uVar9 = 1;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    local_110 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
  }
joined_r0x0001007bd818:
  if (local_110 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error(8,0x18);
  }
  local_110[1] = 1;
  *local_110 = 1;
  *(undefined1 *)(local_110 + 2) = 0;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  local_108 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
  if (local_108 != (long *)0x0) {
    local_108[1] = 1;
    *local_108 = 1;
    *(undefined4 *)(local_108 + 2) = 0;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    plVar5 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    plVar2 = local_108;
    plVar1 = local_110;
    if (plVar5 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    plVar5[1] = 1;
    *plVar5 = 1;
    *(bool *)(plVar5 + 2) = bVar8;
    lVar4 = *local_110;
    *local_110 = lVar4 + 1;
    local_100 = plVar5;
    if (-1 < lVar4) {
      local_f8 = local_110;
      lVar4 = *local_108;
      *local_108 = lVar4 + 1;
      if (-1 < lVar4) {
        local_f0 = local_108;
        lVar4 = *plVar5;
        *plVar5 = lVar4 + 1;
        if (-1 < lVar4) {
          local_d0 = (long *)0x8000000000000000;
          local_e0 = (long *)0x0;
          local_b8 = local_b8 & 0xffffffffffffff00;
          local_e8 = plVar5;
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          pppppcStack_1c8 = (char *****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x16,1);
          if (pppppcStack_1c8 != (char *****)0x0) {
            pppppcStack_1c8[1] = (char ****)0x6e6f697469736e61;
            *pppppcStack_1c8 = (char ****)0x72742d7865646f63;
            builtin_strncpy((char *)((long)pppppcStack_1c8 + 0xe),"on-fence",8);
            local_1d0 = (char ******)0x16;
            local_1c0 = (char ******)0x16;
            std::thread::builder::Builder::name(&local_170,&local_e0,&local_1d0);
            local_e0 = plVar1;
            plStack_d8 = plVar2;
            uStack_c8 = CONCAT71(uStack_c8._1_7_,(char)param_5);
            pcStack_78 = (code *)uStack_158;
            local_80 = (char *******)local_160;
            local_70 = local_150;
            local_d0 = plVar5;
            std::thread::lifecycle::spawn_unchecked(&local_1d0,&local_80,local_170,pcStack_168,uStack_148 & 0xff,0,&local_e0);
            if (local_1d0 == (char ******)0x0) {
              local_1d0 = (char ******)pppppcStack_1c8;
              local_170 = (char ******)&local_1d0;
              pcStack_168 = <std::io::error::Error_as_core::fmt::Display>::fmt;
              alloc::fmt::format::format_inner(&local_e0,s__CODEX_TRANSITION_FENCE_START_FA_10152bcca,&local_170);
              plVar5 = local_d0;
              plVar2 = plStack_d8;
              plVar1 = local_e0;
              if (((ulong)local_1d0 & 3) == 1) {
                pcVar6 = (char *)((long)local_1d0 + -1);
                uVar7 = *(undefined8 *)pcVar6;
                puVar10 = *(undefined8 **)((long)local_1d0 + 7);
                if ((code *)*puVar10 != (code *)0x0) {
                  (*(code *)*puVar10)(uVar7);
                }
                if (puVar10[1] != 0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar7,puVar10[1],puVar10[2]);
                }
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar6,0x18,8);
              }
              param_1[2] = plVar2;
              param_1[1] = plVar1;
              param_1[3] = plVar5;
              *param_1 = 10;
              lVar4 = *local_100;
              *local_100 = lVar4 + -1;
              LORelease();
              if (lVar4 == 1) {
                DataMemoryBarrier(2,1);
                alloc::sync::Arc<T,A>::drop_slow(&local_100);
              }
              lVar4 = *local_108;
              *local_108 = lVar4 + -1;
              LORelease();
              if (lVar4 == 1) {
                DataMemoryBarrier(2,1);
                alloc::sync::Arc<T,A>::drop_slow(&local_108);
              }
              lVar4 = *local_110;
              *local_110 = lVar4 + -1;
              LORelease();
              if (lVar4 == 1) {
                DataMemoryBarrier(2,1);
                alloc::sync::Arc<T,A>::drop_slow(&local_110);
              }
            }
            else {
              param_1[2] = pppppcStack_1c8;
              param_1[3] = local_1c0;
              param_1[4] = local_110;
              param_1[5] = local_108;
              param_1[6] = local_100;
              *(undefined1 *)(param_1 + 7) = uVar9;
              *(byte *)((long)param_1 + 0x39) = param_4;
              *(char *)((long)param_1 + 0x3a) = (char)param_5;
              *param_1 = 0xb;
              param_1[1] = local_1d0;
            }
            return;
          }
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x16);
        }
      }
    }
                    /* WARNING: Does not return */
    pcVar3 = (code *)SoftwareBreakpoint(1,0x1007bdaf8);
    (*pcVar3)();
  }
                    /* WARNING: Subroutine does not return */
  alloc::alloc::handle_alloc_error(8,0x18);
}

