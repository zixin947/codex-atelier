/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* prepare_transition @ 1007be88c requested 1007be88c */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::platform::process::CodexProcessLifecycle::prepare_transition */

void codexmate_lib::platform::process::CodexProcessLifecycle::prepare_transition(undefined8 *******param_1,undefined8 param_2,undefined8 param_3,uint param_4)

{
  undefined1 auVar1 [8];
  undefined8 *******pppppppuVar2;
  int iVar3;
  ulong uVar4;
  undefined8 *******pppppppuVar5;
  undefined8 *******pppppppuVar6;
  undefined8 *******pppppppuVar7;
  undefined8 *******pppppppuVar8;
  undefined8 *******pppppppuVar9;
  undefined1 auVar10 [8];
  undefined8 *******pppppppuVar11;
  char *pcVar12;
  undefined8 uVar13;
  undefined8 *puVar14;
  undefined8 *******unaff_x28;
  undefined1 auVar15 [16];
  undefined1 auVar16 [12];
  undefined1 auVar17 [12];
  int local_2a0;
  undefined8 *******local_290;
  undefined8 *******pppppppuStack_288;
  undefined8 *******local_280;
  undefined8 *******pppppppuStack_278;
  undefined8 *******local_270;
  undefined8 *******pppppppuStack_268;
  undefined8 *******local_260;
  undefined8 *******local_250;
  undefined8 *******pppppppuStack_248;
  undefined8 *******local_240;
  undefined8 *******local_230;
  undefined8 *******pppppppuStack_228;
  undefined1 local_210 [8];
  undefined8 *******pppppppuStack_208;
  undefined8 *******local_200;
  undefined8 *******pppppppuStack_1f8;
  undefined8 *******local_1f0;
  undefined8 *******pppppppuStack_1e8;
  undefined8 *******local_1e0;
  undefined8 *******local_1d0;
  undefined8 *******pppppppuStack_1c8;
  undefined8 *******pppppppuStack_1c0;
  undefined8 *******local_1b8;
  undefined8 *******pppppppuStack_1b0;
  undefined8 *******local_1a8;
  undefined8 *******pppppppuStack_1a0;
  undefined8 *******local_198;
  undefined8 ******ppppppuStack_190;
  undefined8 ******local_188;
  undefined8 ******ppppppuStack_180;
  undefined8 *******local_178;
  undefined8 *******local_108;
  undefined8 *******pppppppuStack_100;
  undefined8 *******local_f8;
  undefined8 *******local_f0;
  undefined8 *******local_e8;
  undefined8 *******pppppppuStack_e0;
  undefined8 *******local_d8;
  undefined8 *******local_d0;
  undefined8 *******pppppppuStack_c8;
  undefined8 *******local_c0;
  undefined8 *******pppppppuStack_b8;
  undefined8 *******pppppppuStack_b0;
  undefined8 *******local_a8;
  undefined8 *******pppppppuStack_a0;
  undefined8 *******local_98;
  undefined8 ******ppppppuStack_90;
  undefined8 ******local_88;
  undefined8 ******ppppppuStack_80;
  undefined8 *******local_78;
  
  if (param_4 != 0) {
    collect_external_codex_writers(&local_1d0);
    pppppppuVar8 = local_1b8;
    pppppppuVar5 = pppppppuStack_1c0;
    pppppppuStack_c8 = pppppppuStack_1c0;
    local_d0 = pppppppuStack_1c8;
    local_c0 = local_1b8;
    if (local_1d0 != (undefined8 *******)&DAT_0000000b) {
      param_1[5] = local_1a8;
      param_1[4] = pppppppuStack_1b0;
      param_1[7] = local_198;
      param_1[6] = pppppppuStack_1a0;
      param_1[9] = local_188;
      param_1[8] = ppppppuStack_190;
      param_1[0xb] = local_178;
      param_1[10] = ppppppuStack_180;
      param_1[2] = pppppppuStack_1c0;
      param_1[1] = pppppppuStack_1c8;
      param_1[3] = local_1b8;
      *param_1 = local_1d0;
      return;
    }
    pppppppuStack_208 = pppppppuStack_1c0;
    local_210 = (undefined1  [8])pppppppuStack_1c8;
    local_200 = local_1b8;
    <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
              (&local_d0,pppppppuStack_1c0,pppppppuStack_1c0 + (long)local_1b8 * 4);
    reject_external_codex_writers(&local_1d0,&local_d0);
    if (local_1d0 != (undefined8 *******)&DAT_0000000b) {
      param_1[5] = local_1a8;
      param_1[4] = pppppppuStack_1b0;
      param_1[7] = local_198;
      param_1[6] = pppppppuStack_1a0;
      param_1[9] = local_188;
      param_1[8] = ppppppuStack_190;
      param_1[0xb] = local_178;
      param_1[10] = ppppppuStack_180;
      param_1[1] = pppppppuStack_1c8;
      *param_1 = local_1d0;
      param_1[3] = local_1b8;
      param_1[2] = pppppppuStack_1c0;
      auVar1 = local_210;
      if (pppppppuVar8 != (undefined8 *******)0x0) {
        pppppppuVar6 = pppppppuVar5 + 1;
        do {
          if (pppppppuVar6[-1] != (undefined8 ******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppuVar6,pppppppuVar6[-1],1);
          }
          pppppppuVar6 = pppppppuVar6 + 4;
          pppppppuVar8 = (undefined8 *******)((long)pppppppuVar8 + -1);
          auVar1 = local_210;
        } while (pppppppuVar8 != (undefined8 *******)0x0);
      }
joined_r0x0001007bf220:
      if (auVar1 == (undefined1  [8])0x0) {
        return;
      }
      pppppppuVar6 = (undefined8 *******)((long)auVar1 << 5);
      uVar13 = 8;
      goto LAB_1007bf230;
    }
    if (pppppppuVar8 != (undefined8 *******)0x0) {
      pppppppuVar6 = pppppppuVar5 + 1;
      do {
        if (pppppppuVar6[-1] != (undefined8 ******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppuVar6,pppppppuVar6[-1],1);
        }
        pppppppuVar6 = pppppppuVar6 + 4;
        pppppppuVar8 = (undefined8 *******)((long)pppppppuVar8 + -1);
      } while (pppppppuVar8 != (undefined8 *******)0x0);
    }
    if (local_210 != (undefined1  [8])0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar5,(long)local_210 << 5,8);
    }
  }
  uVar4 = is_codex_app_running();
  if ((uVar4 & 1) == 0) {
LAB_1007bee04:
    if ((param_4 & 1) == 0) {
      kill_codex_processes_until_clear(param_1);
      return;
    }
    auVar15 = std::time::Instant::now();
    auVar16 = <std::time::Instant_as_core::ops::arith::Add<core::time::Duration>>::add(auVar15._0_8_,auVar15._8_8_,3,0);
    pppppppuVar6 = (undefined8 *******)&DAT_8000000000000000;
    pppppppuVar8 = param_1;
    pppppppuVar5 = unaff_x28;
    while (list_codex_processes_matching(&local_1d0), local_1d0 == (undefined8 *******)&DAT_0000000b) {
      local_210 = (undefined1  [8])pppppppuStack_1c0;
      pppppppuStack_208 = pppppppuStack_1c0;
      pppppppuStack_1f8 = pppppppuStack_1c0 + (long)local_1b8 * 4;
      local_200 = pppppppuStack_1c8;
      alloc::vec::in_place_collect::from_iter_in_place(&pppppppuStack_c8,local_210);
      pppppppuVar11 = pppppppuStack_b8;
      pppppppuVar7 = local_c0;
      auVar1 = (undefined1  [8])pppppppuStack_c8;
      if (pppppppuStack_b8 == (undefined8 *******)0x0) {
        *param_1 = (undefined8 ******)0xb;
        if (pppppppuStack_c8 != (undefined8 *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_c0,(long)pppppppuStack_c8 << 5,8);
        }
        goto LAB_1007bf048;
      }
      auVar17 = std::time::Instant::now();
      if (auVar17._0_8_ != auVar16._0_8_) {
        if (auVar17._0_8_ < auVar16._0_8_) goto LAB_1007beea0;
LAB_1007bf060:
        <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
                  (&local_250,pppppppuVar7,pppppppuVar7 + (long)pppppppuVar11 * 4);
        pppppppuVar9 = local_240;
        pppppppuVar2 = pppppppuStack_248;
        alloc::str::join_generic_copy
                  (&local_1d0,pppppppuStack_248,local_240,
                   " | ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                   ,3);
        pppppppuStack_288 = pppppppuStack_1c8;
        local_290 = local_1d0;
        local_280 = pppppppuStack_1c0;
        if (pppppppuVar6 == (undefined8 *******)&DAT_8000000000000000) {
          local_210 = (undefined1  [8])0x0;
          pppppppuStack_208 = (undefined8 *******)0x1;
          local_200 = (undefined8 *******)0x0;
        }
        else {
          local_210 = (undefined1  [8])&local_1d0;
          pppppppuStack_208 = (undefined8 *******)<alloc::string::String_as_core::fmt::Display>::fmt;
          local_1d0 = pppppppuVar6;
          pppppppuStack_1c8 = pppppppuVar5;
          pppppppuStack_1c0 = pppppppuVar8;
          alloc::fmt::format::format_inner(&local_d0,s__10152c185,local_210);
          if (local_1d0 != (undefined8 *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuStack_1c8,local_1d0,1);
          }
          pppppppuStack_208 = pppppppuStack_c8;
          local_210 = (undefined1  [8])local_d0;
          local_200 = local_c0;
        }
        local_1d0 = &local_290;
        pppppppuStack_1c8 = (undefined8 *******)<alloc::string::String_as_core::fmt::Display>::fmt;
        pppppppuStack_1c0 = (undefined8 *******)local_210;
        local_1b8 = (undefined8 *******)<alloc::string::String_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_d0,s__CHATGPT_DESKTOP_FORCE_KILL_FAIL_10152c1ae,&local_1d0);
        if (local_210 != (undefined1  [8])0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuStack_208,local_210,1);
        }
        if (local_290 != (undefined8 *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuStack_288,local_290,1);
        }
        if (pppppppuVar9 != (undefined8 *******)0x0) {
          pppppppuVar8 = pppppppuVar2 + 1;
          do {
            if (pppppppuVar8[-1] != (undefined8 ******)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppuVar8,pppppppuVar8[-1],1);
            }
            pppppppuVar8 = pppppppuVar8 + 3;
            pppppppuVar9 = (undefined8 *******)((long)pppppppuVar9 + -1);
          } while (pppppppuVar9 != (undefined8 *******)0x0);
        }
        if (local_250 != (undefined8 *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar2,(long)local_250 * 0x18,8);
        }
        param_1[2] = pppppppuStack_c8;
        param_1[1] = local_d0;
        param_1[3] = local_c0;
        *param_1 = (undefined8 ******)0xa;
        pppppppuVar8 = pppppppuVar7 + 2;
        do {
          if (pppppppuVar8[-1] != (undefined8 ******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppuVar8,pppppppuVar8[-1],1);
          }
          pppppppuVar8 = pppppppuVar8 + 4;
          pppppppuVar11 = (undefined8 *******)((long)pppppppuVar11 + -1);
          pppppppuVar5 = pppppppuVar7;
        } while (pppppppuVar11 != (undefined8 *******)0x0);
        goto joined_r0x0001007bf220;
      }
      if (auVar16._8_4_ <= auVar17._8_4_) goto LAB_1007bf060;
LAB_1007beea0:
      force_kill_targets(&local_d0,pppppppuVar7,pppppppuVar11);
      if (local_d0 == (undefined8 *******)&DAT_0000000b) {
        auVar10 = (undefined1  [8])&DAT_8000000000000000;
        pppppppuVar9 = pppppppuVar5;
      }
      else {
        pppppppuStack_1a0 = pppppppuStack_a0;
        local_1a8 = local_a8;
        ppppppuStack_190 = ppppppuStack_90;
        local_198 = local_98;
        ppppppuStack_180 = ppppppuStack_80;
        local_188 = local_88;
        local_178 = local_78;
        pppppppuStack_1c0 = local_c0;
        pppppppuStack_1c8 = pppppppuStack_c8;
        pppppppuStack_1b0 = pppppppuStack_b0;
        local_1b8 = pppppppuStack_b8;
        local_1d0 = local_d0;
        local_210 = (undefined1  [8])0x0;
        pppppppuStack_208 = (undefined8 *******)0x1;
        local_200 = (undefined8 *******)0x0;
        local_c0 = (undefined8 *******)&DAT_60000020;
        local_d0 = (undefined8 *******)local_210;
        pppppppuStack_c8 = (undefined8 *******)&_anon_6408540cdffbe318d49f52812e99783f_1640;
        iVar3 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_1d0,&local_d0);
        pppppppuVar8 = local_200;
        pppppppuVar9 = pppppppuStack_208;
        auVar10 = local_210;
        if (iVar3 != 0) {
                    /* WARNING: Subroutine does not return */
          ::core::result::unwrap_failed
                    ("a Display implementation returned an error unexpectedlycalled `Result::unwrap()` on an `Err` value",0x37,&local_290,
                     &_anon_6408540cdffbe318d49f52812e99783f_1652,&_anon_6408540cdffbe318d49f52812e99783f_1643);
        }
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_1d0);
      }
      if (((ulong)pppppppuVar6 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar5,pppppppuVar6,1);
      }
      std::thread::functions::sleep(0,300000000);
      if (pppppppuVar11 != (undefined8 *******)0x0) {
        pppppppuVar5 = pppppppuVar7 + 2;
        do {
          if (pppppppuVar5[-1] != (undefined8 ******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppuVar5,pppppppuVar5[-1],1);
          }
          pppppppuVar5 = pppppppuVar5 + 4;
          pppppppuVar11 = (undefined8 *******)((long)pppppppuVar11 + -1);
        } while (pppppppuVar11 != (undefined8 *******)0x0);
      }
      pppppppuVar6 = (undefined8 *******)auVar10;
      pppppppuVar5 = pppppppuVar9;
      if (auVar1 != (undefined1  [8])0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar7,(long)auVar1 << 5,8);
      }
    }
    local_a8 = local_1a8;
    pppppppuStack_b0 = pppppppuStack_1b0;
    local_98 = local_198;
    pppppppuStack_a0 = pppppppuStack_1a0;
    local_88 = local_188;
    ppppppuStack_90 = ppppppuStack_190;
    local_78 = local_178;
    ppppppuStack_80 = ppppppuStack_180;
    pppppppuStack_c8 = pppppppuStack_1c8;
    local_c0 = pppppppuStack_1c0;
    pppppppuStack_b8 = local_1b8;
    pppppppuStack_248 = pppppppuStack_1c0;
    local_250 = pppppppuStack_1c8;
    local_240 = local_1b8;
    param_1[5] = local_1a8;
    param_1[4] = pppppppuStack_1b0;
    param_1[7] = local_198;
    param_1[6] = pppppppuStack_1a0;
    param_1[9] = local_188;
    param_1[8] = ppppppuStack_190;
    param_1[0xb] = local_178;
    param_1[10] = ppppppuStack_180;
    param_1[2] = pppppppuStack_1c0;
    param_1[1] = pppppppuStack_1c8;
    param_1[3] = local_1b8;
    *param_1 = local_1d0;
LAB_1007bf048:
    if (((ulong)pppppppuVar6 & 0x7fffffffffffffff) == 0) {
      return;
    }
    uVar13 = 1;
LAB_1007bf230:
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar5,pppppppuVar6,uVar13);
    return;
  }
  std::sys::process::unix::common::Command::new
            (&local_1d0,
             "/usr/bin/osascript-eCFBundleShortVersionString | ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,0x12);
  std::sys::process::unix::common::Command::arg
            (&local_1d0,
             "-eCFBundleShortVersionString | ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,2);
  std::sys::process::unix::common::Command::arg
            (&local_1d0,
             "if application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,0x6e);
  std::process::Command::output(local_210,&local_1d0);
  pppppppuVar7 = local_1e0;
  pppppppuVar11 = pppppppuStack_1e8;
  pppppppuVar6 = local_1f0;
  pppppppuVar5 = pppppppuStack_1f8;
  pppppppuVar8 = pppppppuStack_208;
  auVar1 = local_210;
  if (local_210 == (undefined1  [8])&DAT_8000000000000000) {
    local_f0 = pppppppuStack_208;
    local_d0 = &local_f0;
    pppppppuStack_c8 = (undefined8 *******)<std::io::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_e8,s__osascript_quit_Codex_host_faile_10152bf13,&local_d0);
    pppppppuStack_c8 = pppppppuStack_e0;
    local_d0 = local_e8;
    local_c0 = local_d8;
    if (((ulong)local_f0 & 3) == 1) {
      pcVar12 = (char *)((long)local_f0 + -1);
      uVar13 = *(undefined8 *)pcVar12;
      puVar14 = *(undefined8 **)((long)local_f0 + 7);
      if ((code *)*puVar14 != (code *)0x0) {
        (*(code *)*puVar14)(uVar13);
      }
      if (puVar14[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar13,puVar14[1],puVar14[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar12,0x18,8);
    }
    pppppppuStack_228 = local_a8;
    local_230 = pppppppuStack_b0;
    local_260 = pppppppuStack_a0;
    pppppppuStack_278 = pppppppuStack_b8;
    local_280 = local_c0;
    pppppppuStack_268 = local_a8;
    local_270 = pppppppuStack_b0;
    pppppppuStack_288 = pppppppuStack_c8;
    local_290 = local_d0;
    pppppppuStack_1f8 = pppppppuStack_b8;
    local_200 = local_c0;
    pppppppuStack_1e8 = local_a8;
    local_1f0 = pppppppuStack_b0;
    local_1e0 = pppppppuStack_a0;
    pppppppuStack_208 = pppppppuStack_c8;
    local_210 = (undefined1  [8])local_d0;
    ::core::ptr::drop_in_place<std::process::Command>(&local_1d0);
  }
  else {
    pppppppuStack_228 = pppppppuStack_1e8;
    local_230 = local_1f0;
    pppppppuStack_288 = pppppppuStack_208;
    local_290 = (undefined8 *******)local_210;
    pppppppuStack_278 = pppppppuStack_1f8;
    local_280 = local_200;
    pppppppuStack_268 = pppppppuStack_1e8;
    local_270 = local_1f0;
    local_260 = local_1e0;
    ::core::ptr::drop_in_place<std::process::Command>(&local_1d0);
    local_2a0 = (int)pppppppuVar7;
    if (local_2a0 == 0) {
      if (auVar1 != (undefined1  [8])0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar8,auVar1,1);
      }
      if (pppppppuVar5 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar6,pppppppuVar5,1);
      }
      goto LAB_1007bec7c;
    }
    alloc::string::String::from_utf8_lossy(&local_1d0,pppppppuVar6,pppppppuVar11);
    pppppppuVar11 = pppppppuStack_1c8;
    _local_210 = ::core::str::<impl_str>::trim_matches(pppppppuStack_1c8,pppppppuStack_1c0);
    local_d0 = (undefined8 *******)local_210;
    pppppppuStack_c8 = (undefined8 *******)&<&T_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_108,s__osascript_quit_Codex_host_faile_10152bf13,&local_d0);
    if (((ulong)local_1d0 & 0x7fffffffffffffff) != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar11,local_1d0,1);
    }
    pppppppuStack_208 = pppppppuStack_100;
    local_210 = (undefined1  [8])local_108;
    local_200 = local_f8;
    if (auVar1 != (undefined1  [8])0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar8,auVar1,1);
    }
    if (pppppppuVar5 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar6,pppppppuVar5,1);
    }
  }
  local_c0 = pppppppuStack_208;
  pppppppuStack_c8 = (undefined8 *******)local_210;
  pppppppuStack_b0 = pppppppuStack_1f8;
  pppppppuStack_b8 = local_200;
  pppppppuStack_a0 = pppppppuStack_1e8;
  local_a8 = local_1f0;
  local_d0 = (undefined8 *******)&DAT_0000000a;
  local_98 = local_1e0;
  if (1 < log::MAX_LOG_LEVEL_FILTER) {
    local_250 = &local_d0;
    pppppppuStack_248 = (undefined8 *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
    ppppppuStack_190 = (undefined8 ******)0x20;
    local_178 = &local_250;
    ppppppuStack_180 = (undefined8 ******)s_d_AiMaMi__codex_process_lifecycl_10152bd7b;
    local_1d0 = (undefined8 *******)0x0;
    pppppppuStack_1c8 = (undefined8 *******)0x1013ad807;
    local_1b8 = (undefined8 *******)0x0;
    pppppppuStack_1c0 = (undefined8 *******)&DAT_00000020;
    local_198 = (undefined8 *******)0x1013ad807;
    pppppppuStack_1b0 = (undefined8 *******)0x101524da1;
    pppppppuStack_1a0 = (undefined8 *******)0x2;
    local_1a8 = (undefined8 *******)&DAT_00000017;
    local_188 = (undefined8 ******)0x66400000001;
    <log::__private_api::GlobalLogger_as_log::Log>::log(&local_290,&local_1d0);
  }
  ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_d0);
LAB_1007bec7c:
  auVar15 = std::time::Instant::now();
  auVar16 = <std::time::Instant_as_core::ops::arith::Add<core::time::Duration>>::add(auVar15._0_8_,auVar15._8_8_,param_2,param_3);
  do {
    auVar17 = std::time::Instant::now();
    if (auVar17._0_8_ == auVar16._0_8_) {
      if (auVar16._8_4_ <= auVar17._8_4_) {
LAB_1007bed68:
        collect_codex_kill_targets(&local_1d0);
        pppppppuVar5 = pppppppuStack_1c0;
        pppppppuVar8 = pppppppuStack_1c8;
        if (local_1d0 == (undefined8 *******)&DAT_0000000b) {
          if (local_1b8 != (undefined8 *******)0x0) {
            pppppppuVar6 = pppppppuStack_1c0 + 2;
            pppppppuVar11 = local_1b8;
            do {
              if (pppppppuVar6[-1] != (undefined8 ******)0x0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppuVar6,pppppppuVar6[-1],1);
              }
              pppppppuVar6 = pppppppuVar6 + 4;
              pppppppuVar11 = (undefined8 *******)((long)pppppppuVar11 + -1);
            } while (pppppppuVar11 != (undefined8 *******)0x0);
          }
          if (pppppppuVar8 != (undefined8 *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar5,(long)pppppppuVar8 << 5,8);
          }
        }
        else {
          local_a8 = local_1a8;
          pppppppuStack_b0 = pppppppuStack_1b0;
          local_98 = local_198;
          pppppppuStack_a0 = pppppppuStack_1a0;
          local_88 = local_188;
          ppppppuStack_90 = ppppppuStack_190;
          local_78 = local_178;
          ppppppuStack_80 = ppppppuStack_180;
          pppppppuStack_c8 = pppppppuStack_1c8;
          local_d0 = local_1d0;
          pppppppuStack_b8 = local_1b8;
          local_c0 = pppppppuStack_1c0;
          if (local_1d0 != (undefined8 *******)&DAT_0000000b) {
            ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_d0);
          }
        }
        goto LAB_1007bee04;
      }
    }
    else if (auVar16._0_8_ <= auVar17._0_8_) goto LAB_1007bed68;
    collect_codex_kill_targets(&local_1d0);
    pppppppuVar6 = local_1b8;
    pppppppuVar5 = pppppppuStack_1c0;
    pppppppuVar8 = pppppppuStack_1c8;
    if (local_1d0 == (undefined8 *******)&DAT_0000000b) {
      if (local_1b8 != (undefined8 *******)0x0) {
        pppppppuVar11 = pppppppuStack_1c0 + 2;
        pppppppuVar7 = local_1b8;
        do {
          if (pppppppuVar11[-1] != (undefined8 ******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppuVar11,pppppppuVar11[-1],1);
          }
          pppppppuVar11 = pppppppuVar11 + 4;
          pppppppuVar7 = (undefined8 *******)((long)pppppppuVar7 + -1);
        } while (pppppppuVar7 != (undefined8 *******)0x0);
      }
      if (pppppppuVar8 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar5,(long)pppppppuVar8 << 5,8);
      }
      if (pppppppuVar6 == (undefined8 *******)0x0) goto LAB_1007bee04;
    }
    else {
      local_a8 = local_1a8;
      pppppppuStack_b0 = pppppppuStack_1b0;
      local_98 = local_198;
      pppppppuStack_a0 = pppppppuStack_1a0;
      local_88 = local_188;
      ppppppuStack_90 = ppppppuStack_190;
      local_78 = local_178;
      ppppppuStack_80 = ppppppuStack_180;
      pppppppuStack_c8 = pppppppuStack_1c8;
      local_d0 = local_1d0;
      pppppppuStack_b8 = local_1b8;
      local_c0 = pppppppuStack_1c0;
      if (local_1d0 == (undefined8 *******)&DAT_0000000b) {
        if (((ulong)pppppppuStack_1c8 & 1) != 0) goto LAB_1007bee04;
      }
      else {
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_d0);
      }
    }
    std::thread::functions::sleep(0,100000000);
  } while( true );
}

