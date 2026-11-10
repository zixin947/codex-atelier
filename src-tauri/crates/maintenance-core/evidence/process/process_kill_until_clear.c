/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* kill_codex_processes_until_clear @ 1007c5960 requested 1007c5960 */

/* codexmate_lib::platform::process::kill_codex_processes_until_clear */

void codexmate_lib::platform::process::kill_codex_processes_until_clear(undefined8 *param_1)

{
  undefined8 ***pppuVar1;
  code *pcVar2;
  int iVar3;
  undefined8 ****ppppuVar4;
  undefined8 ****ppppuVar5;
  undefined8 *puVar6;
  undefined8 ****ppppuVar7;
  long lVar8;
  undefined1 auVar9 [16];
  undefined1 auVar10 [12];
  undefined1 auVar11 [12];
  undefined8 ***local_240;
  long lStack_238;
  long local_230;
  undefined8 ***local_220;
  undefined8 ***pppuStack_218;
  undefined8 ***local_210;
  undefined8 local_200;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 ***local_1d0;
  undefined8 ***pppuStack_1c8;
  undefined8 ***local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 ***local_170;
  undefined8 ***pppuStack_168;
  undefined8 ***local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 ***local_110;
  long lStack_108;
  long local_100;
  undefined8 ***local_f8;
  code *pcStack_f0;
  long local_e8;
  undefined8 ***local_e0;
  undefined8 ***pppuStack_d8;
  undefined8 ***pppuStack_d0;
  undefined8 ***pppuStack_c8;
  char *local_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  char *local_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  char *local_90;
  undefined8 ***pppuStack_88;
  
  auVar9 = std::time::Instant::now();
  auVar10 = <std::time::Instant_as_core::ops::arith::Add<core::time::Duration>>::add(auVar9._0_8_,auVar9._8_8_,3,0);
  local_240 = (undefined8 ***)&DAT_8000000000000000;
  do {
    collect_codex_kill_targets(&local_e0);
    ppppuVar4 = (undefined8 ****)pppuStack_c8;
    pppuStack_168 = pppuStack_d0;
    local_170 = pppuStack_d8;
    local_160 = pppuStack_c8;
    if (local_e0 != (undefined8 ***)&DAT_0000000b) {
      param_1[5] = uStack_b8;
      param_1[4] = local_c0;
      param_1[7] = local_a8;
      param_1[6] = uStack_b0;
      param_1[9] = uStack_98;
      param_1[8] = local_a0;
      param_1[0xb] = pppuStack_88;
      param_1[10] = local_90;
      param_1[2] = pppuStack_d0;
      param_1[1] = pppuStack_d8;
      param_1[3] = pppuStack_c8;
      *param_1 = local_e0;
LAB_1007c5e88:
      if (local_240 != (undefined8 ***)&DAT_8000000000000000 && (undefined8 ****)local_240 != (undefined8 ****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_238,local_240,1);
      }
      return;
    }
    pppuStack_218 = pppuStack_d0;
    local_220 = pppuStack_d8;
    local_210 = pppuStack_c8;
    if ((undefined8 ****)pppuStack_c8 == (undefined8 ****)0x0) {
      protected_codex_db_holders(&local_1d0);
      ppppuVar7 = (undefined8 ****)local_1c0;
      pppuVar1 = pppuStack_1c8;
      if ((undefined8 ****)local_1c0 == (undefined8 ****)0x0) {
        *param_1 = 0xb;
      }
      else {
        alloc::str::join_generic_copy
                  (&local_e0,pppuStack_1c8,local_1c0,
                   " | ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                   ,3);
        pppuStack_168 = pppuStack_d8;
        local_170 = local_e0;
        local_160 = pppuStack_d0;
        local_e0 = &local_170;
        pppuStack_d8 = (undefined8 ***)<alloc::string::String_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_200,&DAT_1013adcdc,&local_e0);
        if ((undefined8 ****)local_170 != (undefined8 ****)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppuStack_168,local_170,1);
        }
        param_1[2] = uStack_1f8;
        param_1[1] = local_200;
        param_1[3] = local_1f0;
        *param_1 = 10;
        ppppuVar5 = (undefined8 ****)(pppuVar1 + 1);
        do {
          if (ppppuVar5[-1] != (undefined8 ***)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppppuVar5,ppppuVar5[-1],1);
          }
          ppppuVar5 = ppppuVar5 + 3;
          ppppuVar7 = (undefined8 ****)((long)ppppuVar7 + -1);
        } while (ppppuVar7 != (undefined8 ****)0x0);
      }
      ppppuVar7 = (undefined8 ****)pppuStack_218;
      if ((undefined8 ****)local_1d0 != (undefined8 ****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppuVar1,(long)local_1d0 * 0x18,8);
        ppppuVar7 = (undefined8 ****)pppuStack_218;
      }
LAB_1007c5e40:
      if (ppppuVar4 != (undefined8 ****)0x0) {
        ppppuVar5 = ppppuVar7 + 2;
        do {
          if (ppppuVar5[-1] != (undefined8 ***)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppppuVar5,ppppuVar5[-1],1);
          }
          ppppuVar5 = ppppuVar5 + 4;
          ppppuVar4 = (undefined8 ****)((long)ppppuVar4 + -1);
        } while (ppppuVar4 != (undefined8 ****)0x0);
      }
      if ((undefined8 ****)local_220 != (undefined8 ****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar7,(long)local_220 << 5,8);
      }
      goto LAB_1007c5e88;
    }
    auVar11 = std::time::Instant::now();
    ppppuVar7 = (undefined8 ****)pppuStack_218;
    if (auVar11._0_8_ == auVar10._0_8_) {
      if (auVar10._8_4_ <= auVar11._8_4_) {
LAB_1007c5cb0:
        <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
                  (&local_f8,pppuStack_218,pppuStack_218 + (long)ppppuVar4 * 4);
        lVar8 = local_e8;
        pcVar2 = pcStack_f0;
        alloc::str::join_generic_copy
                  (&local_e0,pcStack_f0,local_e8,
                   " | ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                   ,3);
        pppuStack_1c8 = pppuStack_d8;
        local_1d0 = local_e0;
        local_1c0 = pppuStack_d0;
        if (local_240 == (undefined8 ***)&DAT_8000000000000000) {
          local_170 = (undefined8 ****)0x0;
          pppuStack_168 = (undefined8 ****)0x1;
          local_160 = (undefined8 ****)0x0;
        }
        else {
          local_110 = &local_240;
          local_170 = &local_110;
          pppuStack_168 = (undefined8 ***)<&T_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_e0,s__10152c185,&local_170);
          pppuStack_168 = pppuStack_d8;
          local_170 = local_e0;
          local_160 = pppuStack_d0;
        }
        local_e0 = &local_1d0;
        pppuStack_d8 = (undefined8 ***)<alloc::string::String_as_core::fmt::Display>::fmt;
        pppuStack_d0 = &local_170;
        pppuStack_c8 = (undefined8 ***)<alloc::string::String_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_1e8,s__CODEX_FORCE_KILL_FAILED__Codex_10152c125,&local_e0);
        if ((undefined8 ****)local_170 != (undefined8 ****)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppuStack_168,local_170,1);
        }
        if ((undefined8 ****)local_1d0 != (undefined8 ****)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppuStack_1c8,local_1d0,1);
        }
        param_1[2] = uStack_1e0;
        param_1[1] = local_1e8;
        param_1[3] = local_1d8;
        *param_1 = 10;
        if (lVar8 != 0) {
          puVar6 = (undefined8 *)((long)pcVar2 + 8);
          do {
            if (puVar6[-1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar6,puVar6[-1],1);
            }
            puVar6 = puVar6 + 3;
            lVar8 = lVar8 + -1;
          } while (lVar8 != 0);
        }
        if ((undefined8 ****)local_f8 != (undefined8 ****)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar2,(long)local_f8 * 0x18,8);
        }
        goto LAB_1007c5e40;
      }
    }
    else if (auVar10._0_8_ <= auVar11._0_8_) goto LAB_1007c5cb0;
    force_kill_targets(&local_1d0,pppuStack_218,ppppuVar4);
    if (local_1d0 == (undefined8 ***)&DAT_0000000b) {
      if ((local_240 != (undefined8 ***)&DAT_8000000000000000) && ((undefined8 ****)local_240 != (undefined8 ****)0x0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_238,local_240,1);
      }
      local_240 = (undefined8 ***)&DAT_8000000000000000;
    }
    else {
      uStack_148 = uStack_1a8;
      local_150 = local_1b0;
      uStack_138 = uStack_198;
      uStack_140 = uStack_1a0;
      uStack_128 = uStack_188;
      local_130 = local_190;
      uStack_118 = uStack_178;
      uStack_120 = uStack_180;
      pppuStack_168 = pppuStack_1c8;
      local_170 = local_1d0;
      uStack_158 = uStack_1b8;
      local_160 = local_1c0;
      if (1 < log::MAX_LOG_LEVEL_FILTER) {
        local_f8 = &local_170;
        pcStack_f0 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        local_a0 = 0x20;
        local_90 = s_3_AiMaMi__codex_process__force_k_10152c0ef;
        local_e0 = (undefined8 ***)0x0;
        pppuStack_d8 = (undefined8 ***)0x1013ad807;
        pppuStack_c8 = (undefined8 ****)0x0;
        pppuStack_d0 = (undefined8 ***)0x20;
        local_a8 = 
        "codexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
        ;
        local_c0 = "src/platform/process.rs";
        uStack_b0 = 2;
        uStack_b8 = 0x17;
        uStack_98 = 0x60a00000001;
        pppuStack_88 = &local_f8;
        <log::__private_api::GlobalLogger_as_log::Log>::log(&local_110,&local_e0);
      }
      local_f8 = (undefined8 ****)0x0;
      pcStack_f0 = (code *)0x1;
      local_e8 = 0;
      pppuStack_d8 = (undefined8 ***)&_anon_6408540cdffbe318d49f52812e99783f_1640;
      pppuStack_d0 = (undefined8 ***)&DAT_60000020;
      local_e0 = &local_f8;
      iVar3 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_170,&local_e0);
      if (iVar3 != 0) {
                    /* WARNING: Subroutine does not return */
        ::core::result::unwrap_failed
                  ("a Display implementation returned an error unexpectedlycalled `Result::unwrap()` on an `Err` value",0x37,&local_110,
                   &_anon_6408540cdffbe318d49f52812e99783f_1652,&_anon_6408540cdffbe318d49f52812e99783f_1643);
      }
      lStack_108 = (long)pcStack_f0;
      local_110 = local_f8;
      local_100 = local_e8;
      if ((local_240 != (undefined8 ***)&DAT_8000000000000000) && ((undefined8 ****)local_240 != (undefined8 ****)0x0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_238,local_240,1);
      }
      lStack_238 = lStack_108;
      local_240 = local_110;
      local_230 = local_100;
      ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_170);
    }
    std::thread::functions::sleep(0,300000000);
    ppppuVar5 = ppppuVar7 + 2;
    do {
      if (ppppuVar5[-1] != (undefined8 ***)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppppuVar5,ppppuVar5[-1],1);
      }
      ppppuVar5 = ppppuVar5 + 4;
      ppppuVar4 = (undefined8 ****)((long)ppppuVar4 + -1);
    } while (ppppuVar4 != (undefined8 ****)0x0);
    if ((undefined8 ****)local_220 != (undefined8 ****)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar7,(long)local_220 << 5,8);
    }
  } while( true );
}

