/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* launch @ 1007bf414 requested 1007bf414 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::platform::process::CodexProcessLifecycle::launch */

void codexmate_lib::platform::process::CodexProcessLifecycle::launch(undefined8 *param_1)

{
  undefined8 *******pppppppuVar1;
  undefined8 uVar2;
  code *pcVar3;
  undefined4 *puVar4;
  undefined8 *******pppppppuVar5;
  undefined4 uVar6;
  char *pcVar7;
  ulong uVar8;
  ulong uVar9;
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [12];
  undefined1 auVar13 [12];
  long local_1a8;
  undefined8 *******local_1a0;
  code *pcStack_198;
  undefined1 local_190 [12];
  undefined8 ******local_180 [3];
  undefined4 local_164;
  undefined8 *******local_160;
  code *pcStack_158;
  undefined8 ******local_140 [2];
  undefined8 *******local_130;
  code *local_128;
  undefined4 *local_120;
  code *pcStack_118;
  undefined8 *******local_110;
  code *pcStack_108;
  undefined8 *******local_100;
  undefined8 *puStack_f8;
  undefined8 *******local_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined8 uStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined8 uStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 local_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 local_94;
  undefined8 *******local_90;
  code *pcStack_88;
  undefined8 *******local_80;
  code *pcStack_78;
  
  codex_app_bundle_path(&local_f0);
  pppppppuVar5 = local_f0;
  if (local_f0 == (undefined8 *******)&DAT_8000000000000000) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    pcVar7 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x22,1);
    if (pcVar7 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x22);
    }
    builtin_strncpy(pcVar7,"Codex/ChatGPT app bundle not found",0x22);
    param_1[1] = 0x22;
    *param_1 = 8;
    param_1[2] = pcVar7;
    param_1[3] = 0x22;
  }
  else {
    pppppppuVar1 = (undefined8 *******)CONCAT44(uStack_e4,uStack_e8);
    pcVar3 = (code *)CONCAT44(uStack_e0._4_4_,(undefined4)uStack_e0);
    read_macos_bundle_value
              (&local_130,pppppppuVar1,pcVar3,
               "CFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
               ,0x12);
    puVar4 = local_120;
    if (local_130 == (undefined8 *******)&DAT_8000000000000000) {
      local_160 = local_180;
      pcStack_158 = <std::path::Display_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&uStack_e8,s_0Codex_ChatGPT_bundle_has_no_CFB_10152be53,&local_160);
      param_1[5] = CONCAT44(uStack_c4,uStack_c8);
      param_1[4] = uStack_d0;
      param_1[7] = CONCAT44(uStack_b4,uStack_b8);
      param_1[6] = CONCAT44(uStack_bc,uStack_c0);
      param_1[9] = CONCAT44(uStack_a4,uStack_a8);
      param_1[8] = CONCAT44(uStack_ac,local_b0);
      param_1[0xb] = CONCAT44(local_94,uStack_98);
      param_1[10] = CONCAT44(uStack_9c,uStack_a0);
      param_1[2] = uStack_e0;
      param_1[1] = CONCAT44(uStack_e4,uStack_e8);
      param_1[3] = CONCAT44(uStack_d4,uStack_d8);
      *param_1 = 9;
    }
    else {
      std::path::Path::_join
                (&local_f0,pppppppuVar1,pcVar3,
                 "Contents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                 ,0xe);
      uVar2 = CONCAT44(uStack_e4,uStack_e8);
      std::path::Path::_join(&local_1a8,uVar2,CONCAT44(uStack_e0._4_4_,(undefined4)uStack_e0),local_128,puVar4);
      if (local_130 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,local_130,1);
      }
      if (local_f0 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar2,local_f0,1);
      }
      uVar8 = std::path::Path::is_file(local_1a0,pcStack_198);
      if ((uVar8 & 1) == 0) {
        local_160 = local_1a0;
        pcStack_158 = pcStack_198;
        local_130 = &local_160;
        local_128 = <std::path::Display_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_f0,s__Codex_ChatGPT_main_executable_n_10152bde2,&local_130);
        param_1[2] = CONCAT44(uStack_e4,uStack_e8);
        param_1[1] = local_f0;
        param_1[3] = uStack_e0;
        *param_1 = 8;
      }
      else {
        local_190 = std::time::Instant::now();
        local_130 = local_1a0;
        local_128 = pcStack_198;
        local_f0 = &local_160;
        uStack_e8 = 0x115ed44;
        uStack_e4 = 1;
        uStack_e0 = &local_130;
        uStack_d8 = 0x115ed44;
        uStack_d4 = 1;
        local_160 = pppppppuVar1;
        pcStack_158 = pcVar3;
        alloc::fmt::format::format_inner(local_180,s_bundle__executable__10152be0e,&local_f0);
        debug_log::app_event
                  ("codex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                   ,0xd,&DAT_1013af190,0x10,local_180);
        open_macos_app_bundle(&local_f0,pppppppuVar1,pcVar3);
        if (local_f0 == (undefined8 *******)&DAT_0000000b) {
          auVar10 = std::time::Instant::now();
          auVar12 = <std::time::Instant_as_core::ops::arith::Add<core::time::Duration>>::add(auVar10._0_8_,auVar10._8_8_,10,0);
LAB_1007bf628:
          auVar13 = std::time::Instant::now();
          if (auVar13._0_8_ == auVar12._0_8_) {
            if (auVar12._8_4_ <= auVar13._8_4_) goto LAB_1007bf6f4;
LAB_1007bf648:
            auVar13 = codex_macos_main_process_pid(local_1a0,pcStack_198);
            uVar6 = auVar13._8_4_;
            if ((auVar13._0_8_ & 1) != 0) goto LAB_1007bf704;
            std::thread::functions::sleep(0,100000000);
            goto LAB_1007bf628;
          }
          if (auVar13._0_8_ < auVar12._0_8_) goto LAB_1007bf648;
LAB_1007bf6f4:
          auVar12 = codex_macos_main_process_pid(local_1a0,pcStack_198);
          uVar6 = auVar12._8_4_;
          if ((auVar12._0_8_ & 1) == 0) {
            local_80 = local_1a0;
            pcStack_78 = pcStack_198;
            local_90 = pppppppuVar1;
            pcStack_88 = pcVar3;
            auVar11 = std::time::Instant::elapsed(local_190);
            auVar10._8_8_ = 0;
            auVar10._0_8_ = auVar11._0_8_;
            pcStack_158 = SUB168(auVar10 * ZEXT816(1000),8);
            uVar9 = auVar11._0_8_ * 1000;
            uVar8 = (auVar11._8_8_ & 0xffffffff) / 1000000;
            local_160 = (undefined8 *******)(uVar9 + uVar8);
            if (CARRY8(uVar9,uVar8)) {
              pcStack_158 = (code *)((long)pcStack_158 + 1);
            }
            local_f0 = &local_90;
            uStack_e8 = 0x115ed44;
            uStack_e4 = 1;
            uStack_e0 = &local_80;
            uStack_d8 = 0x115ed44;
            uStack_d4 = 1;
            uStack_d0 = &local_160;
            uStack_c8 = 0x117b8e8;
            uStack_c4 = 1;
            alloc::fmt::format::format_inner(&local_130,s_bundle__executable__elapsed_ms__10152be86,&local_f0);
            debug_log::app_event
                      ("codex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                       ,0xd,
                       "launch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                       ,0xe,&local_130);
            local_80 = local_1a0;
            pcStack_78 = pcStack_198;
            local_160 = &local_80;
            pcStack_158 = <std::path::Display_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&uStack_e8,s_5Codex_ChatGPT_main_process_did_n_10152beac,&local_160);
            *(ulong *)((long)param_1 + 0x34) = CONCAT44(uStack_b8,uStack_bc);
            *(ulong *)((long)param_1 + 0x2c) = CONCAT44(uStack_c0,uStack_c4);
            *(ulong *)((long)param_1 + 0x44) = CONCAT44(uStack_a8,uStack_ac);
            *(ulong *)((long)param_1 + 0x3c) = CONCAT44(local_b0,uStack_b4);
            *(ulong *)((long)param_1 + 0x54) = CONCAT44(uStack_98,uStack_9c);
            *(ulong *)((long)param_1 + 0x4c) = CONCAT44(uStack_a0,uStack_a4);
            *(undefined4 *)((long)param_1 + 0x5c) = local_94;
            *(ulong *)((long)param_1 + 0x14) = CONCAT44(uStack_d8,uStack_e0._4_4_);
            *(ulong *)((long)param_1 + 0xc) = CONCAT44((undefined4)uStack_e0,uStack_e4);
            *(ulong *)((long)param_1 + 0x24) = CONCAT44(uStack_c8,uStack_d0._4_4_);
            *(ulong *)((long)param_1 + 0x1c) = CONCAT44((undefined4)uStack_d0,uStack_d4);
            *param_1 = 10;
            *(undefined4 *)(param_1 + 1) = uStack_e8;
            goto joined_r0x0001007bf8f4;
          }
LAB_1007bf704:
          local_164 = uVar6;
          open_macos_app_bundle(&local_f0,pppppppuVar1,pcVar3);
          auVar10 = std::time::Instant::elapsed(local_190);
          auVar11._8_8_ = 0;
          auVar11._0_8_ = auVar10._0_8_;
          pcStack_88 = SUB168(auVar11 * ZEXT816(1000),8);
          uVar9 = auVar10._0_8_ * 1000;
          uVar8 = (auVar10._8_8_ & 0xffffffff) / 1000000;
          local_90 = (undefined8 *******)(uVar9 + uVar8);
          if (CARRY8(uVar9,uVar8)) {
            pcStack_88 = pcStack_88 + 1;
          }
          local_80 = (undefined8 *******)0x1013ac400;
          if (local_f0 != (undefined8 *******)&DAT_0000000b) {
            local_80 = (undefined8 *******)"errorerror_code";
          }
          pcStack_78 = (code *)0x2;
          if (local_f0 != (undefined8 *******)&DAT_0000000b) {
            pcStack_78 = (code *)0x5;
          }
          local_130 = local_140;
          local_128 = <std::path::Display_as_core::fmt::Display>::fmt;
          local_120 = &local_164;
          pcStack_118 = u32>::fmt;
          local_110 = &local_90;
          pcStack_108 = u128>::fmt;
          local_100 = &local_80;
          puStack_f8 = &<&T_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_160,s_bundle__pid__elapsed_ms__activat_10152be26,&local_130);
          debug_log::app_event
                    ("codex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                     ,0xd,
                     "launch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                     ,0xc,&local_160);
        }
        param_1[5] = CONCAT44(uStack_c4,uStack_c8);
        param_1[4] = uStack_d0;
        param_1[7] = CONCAT44(uStack_b4,uStack_b8);
        param_1[6] = CONCAT44(uStack_bc,uStack_c0);
        param_1[9] = CONCAT44(uStack_a4,uStack_a8);
        param_1[8] = CONCAT44(uStack_ac,local_b0);
        param_1[0xb] = CONCAT44(local_94,uStack_98);
        param_1[10] = CONCAT44(uStack_9c,uStack_a0);
        param_1[1] = CONCAT44(uStack_e4,uStack_e8);
        *param_1 = local_f0;
        param_1[3] = CONCAT44(uStack_d4,uStack_d8);
        param_1[2] = uStack_e0;
      }
joined_r0x0001007bf8f4:
      if (local_1a8 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1a0,local_1a8,1);
      }
    }
    if (pppppppuVar5 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar1,pppppppuVar5,1);
    }
  }
  return;
}

