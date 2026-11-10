/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* __rust_begin_short_backtrace @ 10044aaec requested 10044aaec */

/* std::sys::backtrace::__rust_begin_short_backtrace */

void std::sys::backtrace::__rust_begin_short_backtrace(long *param_1)

{
  char cVar1;
  code *pcVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  code *pcVar6;
  undefined8 *puVar7;
  int unaff_w21;
  undefined1 *puVar8;
  code *pcVar9;
  undefined1 *puVar10;
  long local_2d0;
  undefined8 local_2c8;
  undefined8 local_2c0;
  undefined1 local_2b8 [2];
  byte local_2b6;
  undefined8 *****local_2b0;
  code *pcStack_2a8;
  undefined1 *puStack_2a0;
  code *local_298;
  undefined *local_290;
  code *pcStack_288;
  undefined8 *****pppppuStack_280;
  code *pcStack_278;
  undefined8 local_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  code *local_250;
  undefined1 *puStack_248;
  code *local_240;
  undefined1 local_231;
  undefined8 *****local_230;
  code *pcStack_228;
  undefined1 *local_220;
  undefined8 *****local_210;
  code *pcStack_208;
  undefined1 *local_200;
  code *pcStack_1f8;
  undefined *local_1f0;
  code *pcStack_1e8;
  undefined8 *****pppppuStack_1e0;
  code *pcStack_1d8;
  undefined8 local_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined1 local_1a1;
  undefined8 ****local_1a0;
  code *local_198;
  undefined *puStack_190;
  code *local_188;
  undefined1 *puStack_180;
  code *local_178;
  undefined8 *****pppppuStack_170;
  code *local_168;
  undefined8 *****local_160;
  code *pcStack_158;
  undefined1 *puStack_150;
  code *local_148;
  undefined *local_140;
  code *pcStack_138;
  undefined8 *****pppppuStack_130;
  code *local_128;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 *****local_f8;
  code *pcStack_f0;
  undefined1 *local_e8;
  undefined8 uStack_e0;
  char *local_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined *local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  char *local_a8;
  undefined8 *****pppppuStack_a0;
  undefined1 auStack_91 [17];
  
  local_2d0 = 0;
  local_2c8 = 4;
  local_2c0 = 0;
  _local_2b8 = 1;
  cVar1 = *(char *)(*param_1 + 0x10);
  do {
    if (cVar1 != '\0') {
      if (local_2d0 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2c8,local_2d0 << 2,4);
      }
      core::ptr::drop_in_place<codexmate_lib::platform::process::CodexTransitionFence::acquire_with_policy::__closure__>(param_1);
      return;
    }
    codexmate_lib::platform::process::collect_codex_kill_targets(&local_2b0);
    pcVar9 = local_298;
    puVar8 = puStack_2a0;
    if (local_2b0 == (undefined8 *****)&DAT_0000000b) {
      puStack_248 = puStack_2a0;
      local_250 = pcStack_2a8;
      local_240 = local_298;
      uVar4 = codexmate_lib::platform::process::WriterGenerationTracker::observe(&local_2d0,puStack_2a0,local_298);
      uVar5 = uVar4 & 0xff;
      if (uVar5 < 5 && (1 << (ulong)(uVar4 & 0x1f) & 0x16U) != 0) {
        if (uVar5 == 4) {
          LOAcquire();
          iVar3 = *(int *)(param_1[1] + 0x10);
          *(int *)(param_1[1] + 0x10) = iVar3 + 1;
          LORelease();
          unaff_w21 = iVar3 + 1;
          puVar8 = puStack_248;
          pcVar9 = local_240;
        }
        local_231 = -1;
        if (local_2b6 + 1 < 0xff) {
          local_231 = local_2b6 + 1;
        }
        <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_160,puVar8,puVar8 + (long)pcVar9 * 0x20);
        puVar10 = puStack_150;
        pcVar2 = pcStack_158;
        ::alloc::str::join_generic_copy(&local_f8,pcStack_158,puStack_150,&DAT_101317e9f,3);
        pcStack_228 = pcStack_f0;
        local_230 = local_f8;
        local_220 = local_e8;
        if (puVar10 != (undefined1 *)0x0) {
          pcVar6 = pcVar2 + 8;
          do {
            if (*(long *)(pcVar6 + -8) != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)pcVar6,*(long *)(pcVar6 + -8),1);
            }
            pcVar6 = pcVar6 + 0x18;
            puVar10 = puVar10 + -1;
          } while (puVar10 != (undefined1 *)0x0);
        }
        if ((undefined8 ******)local_160 != (undefined8 ******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar2,(long)local_160 * 0x18,8);
        }
        if (1 < log::MAX_LOG_LEVEL_FILTER) {
          if (uVar5 == 4) {
            local_210 = (undefined8 *****)CONCAT44(local_210._4_4_,unaff_w21);
            local_160 = &local_210;
            pcStack_158 = u32>::fmt;
            ::alloc::fmt::format::format_inner(&local_f8,s_re_entry__101524e4f,&local_160);
            pcStack_208 = pcStack_f0;
            local_210 = local_f8;
            local_200 = local_e8;
          }
          else {
            local_210 = (undefined8 ******)0x0;
            pcStack_208 = (code *)0x1;
            local_200 = (undefined1 *)0x0;
          }
          local_160 = &local_210;
          puStack_150 = &local_231;
          pcStack_158 = <alloc::string::String_as_core::fmt::Display>::fmt;
          local_148 = u8>::fmt;
          local_140 = &DAT_101317ea2;
          pppppuStack_130 = &local_230;
          pcStack_138 = u8>::fmt;
          local_128 = <alloc::string::String_as_core::fmt::Display>::fmt;
          local_b8 = 0x20;
          pppppuStack_a0 = &local_160;
          local_a8 = s_2_AiMaMi__codex_transition_fence_101524d35;
          local_f8 = (undefined8 ******)0x0;
          pcStack_f0 = (code *)&DAT_101317ea3;
          uStack_e0 = 0;
          local_e8 = &DAT_00000020;
          local_c0 = &DAT_101317ea3;
          local_d8 = "src/platform/process.rs";
          uStack_c8 = 2;
          local_d0 = 0x17;
          local_b0 = 0x70700000001;
          <log::__private_api::GlobalLogger_as_log::Log>::log(auStack_91,&local_f8);
          if ((undefined8 ******)local_210 != (undefined8 ******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcStack_208,local_210,1);
          }
        }
        codexmate_lib::platform::process::quiesce_codex_transition_targets(&local_210,puVar8,pcVar9,(char)param_1[3],0);
        if (local_210 == (undefined8 *****)&DAT_0000000b) {
          _local_2b8 = CONCAT21(1,local_2b8[0]);
        }
        else {
          pcStack_138 = pcStack_1e8;
          local_140 = local_1f0;
          local_128 = pcStack_1d8;
          pppppuStack_130 = pppppuStack_1e0;
          uStack_118 = uStack_1c8;
          local_120 = local_1d0;
          uStack_108 = uStack_1b8;
          uStack_110 = uStack_1c0;
          pcStack_158 = pcStack_208;
          local_160 = local_210;
          local_148 = pcStack_1f8;
          puStack_150 = local_200;
          uVar5 = 0xff;
          if (local_2b6 + 1 < 0xff) {
            uVar5 = local_2b6 + 1;
          }
          _local_2b8 = CONCAT12((char)uVar5,local_2b8);
          local_1a1 = 2 < uVar5;
          *(undefined1 *)(param_1[2] + 0x10) = 1;
          if (1 < log::MAX_LOG_LEVEL_FILTER) {
            local_1a0 = (undefined8 ****)&local_231;
            local_198 = u8>::fmt;
            puStack_190 = &DAT_101317ea2;
            puStack_180 = &local_1a1;
            local_188 = u8>::fmt;
            pppppuStack_170 = &local_160;
            local_178 = <bool_as_core::fmt::Display>::fmt;
            local_168 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
            local_b8 = 0x20;
            pppppuStack_a0 = &local_1a0;
            local_a8 = s_F_AiMaMi__codex_transition_fence_101524db9;
            local_f8 = (undefined8 ******)0x0;
            pcStack_f0 = (code *)&DAT_101317ea3;
            uStack_e0 = 0;
            local_e8 = &DAT_00000020;
            local_c0 = &DAT_101317ea3;
            local_d8 = "src/platform/process.rs";
            uStack_c8 = 2;
            local_d0 = 0x17;
            local_b0 = 0x71800000001;
            <log::__private_api::GlobalLogger_as_log::Log>::log(auStack_91,&local_f8);
          }
          core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_160);
        }
        if ((undefined8 ******)local_230 != (undefined8 ******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcStack_228,local_230,1);
        }
        if (pcVar9 != (code *)0x0) {
          puVar7 = (undefined8 *)(puVar8 + 0x10);
          do {
            if (puVar7[-1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar7,puVar7[-1],1);
            }
            puVar7 = puVar7 + 4;
            pcVar9 = pcVar9 + -1;
          } while (pcVar9 != (code *)0x0);
        }
        if (local_250 != (code *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar8,(long)local_250 << 5,8);
        }
        goto LAB_10044abcc;
      }
      std::thread::functions::sleep(0,80000000);
      if (pcVar9 != (code *)0x0) {
        puVar7 = (undefined8 *)(puVar8 + 0x10);
        do {
          if (puVar7[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar7,puVar7[-1],1);
          }
          puVar7 = puVar7 + 4;
          pcVar9 = pcVar9 + -1;
        } while (pcVar9 != (code *)0x0);
      }
      if (local_250 != (code *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar8,(long)local_250 << 5,8);
      }
    }
    else {
      pcStack_138 = pcStack_288;
      local_140 = local_290;
      local_128 = pcStack_278;
      pppppuStack_130 = pppppuStack_280;
      uStack_118 = uStack_268;
      local_120 = local_270;
      uStack_108 = uStack_258;
      uStack_110 = uStack_260;
      pcStack_158 = pcStack_2a8;
      local_160 = local_2b0;
      local_148 = local_298;
      puStack_150 = puStack_2a0;
      *(undefined1 *)(param_1[2] + 0x10) = 1;
      if (1 < log::MAX_LOG_LEVEL_FILTER) {
        local_210 = &local_160;
        pcStack_208 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        local_b8 = 0x20;
        pppppuStack_a0 = &local_210;
        local_a8 = s_6_AiMaMi__codex_transition_fence_101524e16;
        local_f8 = (undefined8 ******)0x0;
        pcStack_f0 = (code *)&DAT_101317ea3;
        uStack_e0 = 0;
        local_e8 = &DAT_00000020;
        local_c0 = &DAT_101317ea3;
        local_d8 = "src/platform/process.rs";
        uStack_c8 = 2;
        local_d0 = 0x17;
        local_b0 = 0x72100000001;
        <log::__private_api::GlobalLogger_as_log::Log>::log(auStack_91,&local_f8);
      }
      core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_160);
LAB_10044abcc:
      std::thread::functions::sleep(0,80000000);
    }
    cVar1 = *(char *)(*param_1 + 0x10);
  } while( true );
}

