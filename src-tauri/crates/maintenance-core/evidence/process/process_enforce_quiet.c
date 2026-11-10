/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* enforce_codex_quiet_window_with @ 1007c5178 requested 1007c5178 */

/* codexmate_lib::platform::process::enforce_codex_quiet_window_with */

void codexmate_lib::platform::process::enforce_codex_quiet_window_with(undefined8 *param_1,undefined8 param_2,undefined1 param_3,undefined4 param_4)

{
  int iVar1;
  undefined8 uVar2;
  code *pcVar3;
  bool bVar4;
  undefined1 auVar5 [12];
  undefined8 ****ppppuVar6;
  undefined8 ***pppuVar7;
  code *pcVar8;
  uint uVar9;
  int iVar10;
  undefined8 *****pppppuVar11;
  undefined8 *****pppppuVar12;
  undefined8 *****pppppuVar13;
  undefined8 unaff_x25;
  int iVar14;
  undefined1 auVar15 [16];
  undefined1 auVar16 [12];
  undefined1 auVar17 [12];
  undefined1 auVar18 [12];
  undefined8 local_280;
  undefined8 local_270;
  undefined8 ***local_258;
  long local_250;
  undefined8 local_248;
  undefined8 local_240;
  undefined1 local_238;
  undefined2 local_237;
  undefined8 ****local_230;
  undefined8 ****ppppuStack_228;
  undefined8 ****local_220;
  undefined1 local_211;
  undefined8 ****local_210;
  undefined8 ****ppppuStack_208;
  undefined8 ****local_200;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 local_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 ****local_1b0;
  undefined8 ****ppppuStack_1a8;
  undefined8 ****local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  byte local_141;
  undefined8 ***local_140;
  code *local_138;
  undefined *local_130;
  code *local_128;
  byte *pbStack_120;
  code *local_118;
  undefined8 ****ppppuStack_110;
  code *local_108;
  long local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 ****local_e8;
  undefined8 uStack_e0;
  undefined8 ****ppppuStack_d8;
  undefined8 ****ppppuStack_d0;
  char *local_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  char *local_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  char *local_98;
  undefined8 ***pppuStack_90;
  undefined1 auStack_81 [17];
  
  auVar15 = std::time::Instant::now();
  auVar16 = <std::time::Instant_as_core::ops::arith::Add<core::time::Duration>>::add(auVar15._0_8_,auVar15._8_8_,param_2,0);
  local_250 = 0;
  local_248 = 4;
  local_258 = (undefined8 ****)0x8000000000000000;
  local_240 = 0;
  local_237 = 0;
  iVar14 = 1000000000;
  iVar10 = 0;
  local_238 = param_3;
  do {
    auVar18._8_4_ = iVar14;
    auVar18._0_8_ = unaff_x25;
    auVar17 = std::time::Instant::now();
    pppppuVar11 = auVar17._0_8_;
    collect_codex_kill_targets(&local_e8);
    pppppuVar12 = (undefined8 *****)ppppuStack_d0;
    ppppuVar6 = ppppuStack_d8;
    ppppuStack_1a8 = ppppuStack_d8;
    local_1b0 = (undefined8 ****)uStack_e0;
    local_1a0 = ppppuStack_d0;
    if (local_e8 != (undefined8 ****)&DAT_0000000b) {
      param_1[5] = uStack_c0;
      param_1[4] = local_c8;
      param_1[7] = local_b0;
      param_1[6] = uStack_b8;
      param_1[9] = uStack_a0;
      param_1[8] = local_a8;
      param_1[0xb] = pppuStack_90;
      param_1[10] = local_98;
      param_1[2] = ppppuStack_d8;
      param_1[1] = uStack_e0;
      param_1[3] = ppppuStack_d0;
      *param_1 = local_e8;
LAB_1007c577c:
      if (((ulong)local_258 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_270,local_258,1);
      }
      if (local_250 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_248,local_250 << 2,4);
      }
      return;
    }
    ppppuStack_228 = ppppuStack_d8;
    local_230 = (undefined8 ****)uStack_e0;
    local_220 = ppppuStack_d0;
    uVar9 = WriterGenerationTracker::observe(&local_250,ppppuStack_d8,ppppuStack_d0);
    iVar1 = iVar10 + 1;
    if (iVar10 == -1) {
      iVar1 = -1;
    }
    if ((uVar9 & 0xff) != 4) {
      iVar1 = iVar10;
    }
    if ((uVar9 & 0xff) < 5 && (1 << (ulong)(uVar9 & 0x1f) & 0x16U) != 0) {
      uVar9 = 0xff;
      if (local_237._1_1_ + 1 < 0xff) {
        uVar9 = local_237._1_1_ + 1;
      }
      local_211 = (char)uVar9;
      quiesce_codex_transition_targets(&local_210,ppppuVar6,pppppuVar12,param_4);
      if (local_210 == (undefined8 ****)&DAT_0000000b) {
        local_237 = 1;
        if (((ulong)local_258 & 0x7fffffffffffffff) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_270,local_258,1);
        }
        local_258 = (undefined8 ****)0x8000000000000000;
        pppppuVar12 = (undefined8 *****)local_220;
        goto joined_r0x0001007c52fc;
      }
      uStack_188 = uStack_1e8;
      local_190 = local_1f0;
      uStack_178 = uStack_1d8;
      uStack_180 = uStack_1e0;
      uStack_168 = uStack_1c8;
      local_170 = local_1d0;
      uStack_158 = uStack_1b8;
      uStack_160 = uStack_1c0;
      ppppuStack_1a8 = ppppuStack_208;
      local_1b0 = local_210;
      uStack_198 = uStack_1f8;
      local_1a0 = local_200;
      local_237 = CONCAT11((char)uVar9,(undefined1)local_237);
      local_141 = 2 < (uVar9 & 0xff);
      if (1 < log::MAX_LOG_LEVEL_FILTER) {
        local_140 = (undefined8 ***)&local_211;
        local_138 = u8>::fmt;
        local_130 = &DAT_1013ade27;
        pbStack_120 = &local_141;
        local_128 = u8>::fmt;
        ppppuStack_110 = &local_1b0;
        local_118 = <bool_as_core::fmt::Display>::fmt;
        local_108 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        local_a8 = 0x20;
        pppuStack_90 = &local_140;
        local_98 = s_J_AiMaMi__codex_process__quiet_w_10152c08e;
        local_e8 = (undefined8 ****)0x0;
        uStack_e0 = (undefined **)0x1013ad807;
        ppppuStack_d0 = (undefined8 *****)0x0;
        ppppuStack_d8 = (undefined8 ****)0x20;
        local_b0 = 
        "codexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
        ;
        local_c8 = "src/platform/process.rs";
        uStack_b8 = 2;
        uStack_c0 = 0x17;
        uStack_a0 = 0x84800000001;
        <log::__private_api::GlobalLogger_as_log::Log>::log(auStack_81,&local_e8);
      }
      local_140 = (undefined8 ****)0x0;
      local_138 = (code *)0x1;
      local_130 = (undefined *)0x0;
      ppppuStack_d8 = (undefined8 ****)&DAT_60000020;
      local_e8 = &local_140;
      uStack_e0 = &_anon_6408540cdffbe318d49f52812e99783f_1640;
      iVar10 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_1b0,&local_e8);
      local_280 = local_130;
      pcVar8 = local_138;
      pppuVar7 = local_140;
      if (iVar10 != 0) {
                    /* WARNING: Subroutine does not return */
        ::core::result::unwrap_failed
                  ("a Display implementation returned an error unexpectedlycalled `Result::unwrap()` on an `Err` value",0x37,auStack_81,
                   &_anon_6408540cdffbe318d49f52812e99783f_1652,&_anon_6408540cdffbe318d49f52812e99783f_1643);
      }
      pppppuVar13 = (undefined8 *****)ppppuStack_228;
      pppppuVar12 = (undefined8 *****)local_220;
      if (((ulong)local_258 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_270,local_258,1);
        pppppuVar13 = (undefined8 *****)ppppuStack_228;
        pppppuVar12 = (undefined8 *****)local_220;
      }
      ppppuStack_228 = pppppuVar13;
      local_220 = pppppuVar12;
      local_258 = pppuVar7;
      local_270 = pcVar8;
      if ((local_141 & 1) == 0) {
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_1b0);
        pppppuVar12 = (undefined8 *****)local_220;
        goto joined_r0x0001007c52fc;
      }
      pcVar3 = (code *)0;
      if ((undefined8 ****)pppuVar7 != (undefined8 ****)0x8000000000000000) {
        pcVar3 = pcVar8;
      }
      codex_transition_quiet_timeout(&local_e8,pppppuVar13,pppppuVar12,pcVar3,local_280);
      param_1[5] = uStack_c0;
      param_1[4] = local_c8;
      param_1[7] = local_b0;
      param_1[6] = uStack_b8;
      param_1[9] = uStack_a0;
      param_1[8] = local_a8;
      param_1[0xb] = pppuStack_90;
      param_1[10] = local_98;
      param_1[1] = uStack_e0;
      *param_1 = local_e8;
      param_1[3] = ppppuStack_d0;
      param_1[2] = ppppuStack_d8;
      ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_1b0);
joined_r0x0001007c56fc:
      if (pppppuVar12 != (undefined8 *****)0x0) {
        pppppuVar11 = pppppuVar13 + 2;
        do {
          if (pppppuVar11[-1] != (undefined8 ****)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppuVar11,pppppuVar11[-1],1);
          }
          pppppuVar11 = pppppuVar11 + 4;
          pppppuVar12 = (undefined8 *****)((long)pppppuVar12 + -1);
        } while (pppppuVar12 != (undefined8 *****)0x0);
      }
      goto LAB_1007c5764;
    }
joined_r0x0001007c52fc:
    uStack_e0 = (undefined **)CONCAT44(uStack_e0._4_4_,auVar17._8_4_);
    local_e8 = pppppuVar11;
    if (pppppuVar12 == (undefined8 *****)0x0) {
      auVar5 = auVar17;
      if (iVar14 != 1000000000) {
        auVar5 = auVar18;
      }
      auVar18 = std::time::Instant::saturating_duration_since(&local_e8,auVar5._0_8_,auVar5._8_4_);
      if ((auVar18._0_8_ != 0) || (699999999 < auVar18._8_4_)) {
        protected_codex_db_holders(&local_210);
        if ((undefined8 *****)local_200 == (undefined8 *****)0x0) {
          *(int *)(param_1 + 1) = iVar1;
          *param_1 = 0xb;
        }
        else {
          alloc::str::join_generic_copy
                    (&local_e8,ppppuStack_208,local_200,
                     " | ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                     ,3);
          ppppuStack_1a8 = (undefined8 ****)uStack_e0;
          local_1b0 = local_e8;
          local_1a0 = ppppuStack_d8;
          local_e8 = &local_1b0;
          uStack_e0 = (undefined **)<alloc::string::String_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_100,&DAT_1013adcdc,&local_e8);
          if ((undefined8 *****)local_1b0 != (undefined8 *****)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuStack_1a8,local_1b0,1);
          }
          param_1[2] = uStack_f8;
          param_1[1] = local_100;
          param_1[3] = local_f0;
          *param_1 = 10;
          pppppuVar12 = (undefined8 *****)(ppppuStack_208 + 1);
          pppppuVar11 = (undefined8 *****)local_200;
          do {
            if (pppppuVar12[-1] != (undefined8 ****)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppuVar12,pppppuVar12[-1],1);
            }
            pppppuVar12 = pppppuVar12 + 3;
            pppppuVar11 = (undefined8 *****)((long)pppppuVar11 + -1);
          } while (pppppuVar11 != (undefined8 *****)0x0);
        }
        pppppuVar13 = (undefined8 *****)ppppuStack_228;
        if ((undefined8 *****)local_210 != (undefined8 *****)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuStack_208,(long)local_210 * 0x18,8);
          pppppuVar13 = (undefined8 *****)ppppuStack_228;
        }
LAB_1007c5764:
        if ((undefined8 *****)local_230 != (undefined8 *****)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppuVar13,(long)local_230 << 5,8);
        }
        goto LAB_1007c577c;
      }
    }
    else {
      auVar5._8_4_ = 1000000000;
      auVar5._0_8_ = unaff_x25;
    }
    pppppuVar13 = (undefined8 *****)ppppuStack_228;
    iVar14 = auVar5._8_4_;
    unaff_x25 = auVar5._0_8_;
    bVar4 = auVar16._8_4_ <= auVar17._8_4_;
    if (pppppuVar11 != auVar16._0_8_) {
      bVar4 = (long)auVar16._0_8_ <= (long)pppppuVar11;
    }
    if (bVar4) {
      uVar2 = 0;
      if ((undefined8 ****)local_258 != (undefined8 ****)0x8000000000000000) {
        uVar2 = local_270;
      }
      codex_transition_quiet_timeout(&local_e8,ppppuStack_228,pppppuVar12,uVar2,local_280);
      param_1[5] = uStack_c0;
      param_1[4] = local_c8;
      param_1[7] = local_b0;
      param_1[6] = uStack_b8;
      param_1[9] = uStack_a0;
      param_1[8] = local_a8;
      param_1[0xb] = pppuStack_90;
      param_1[10] = local_98;
      param_1[1] = uStack_e0;
      *param_1 = local_e8;
      param_1[3] = ppppuStack_d0;
      param_1[2] = ppppuStack_d8;
      goto joined_r0x0001007c56fc;
    }
    std::thread::functions::sleep(0,80000000);
    ppppuVar6 = ppppuStack_228;
    if (pppppuVar12 != (undefined8 *****)0x0) {
      pppppuVar11 = (undefined8 *****)(ppppuStack_228 + 2);
      do {
        if (pppppuVar11[-1] != (undefined8 ****)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppuVar11,pppppuVar11[-1],1);
        }
        pppppuVar11 = pppppuVar11 + 4;
        pppppuVar12 = (undefined8 *****)((long)pppppuVar12 + -1);
      } while (pppppuVar12 != (undefined8 *****)0x0);
    }
    iVar10 = iVar1;
    if ((undefined8 *****)local_230 != (undefined8 *****)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar6,(long)local_230 << 5,8);
    }
  } while( true );
}

