/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* ensure_transition_rollout_write_capacity @ 1008d54e8 requested 1008d54e8 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::codex_thread_visibility::ensure_transition_rollout_write_capacity */

void codexmate_lib::core::relay::codex_thread_visibility::ensure_transition_rollout_write_capacity(undefined8 *param_1,long param_2,uint param_3)

{
  ulong uVar1;
  long *******ppppppplVar2;
  undefined8 uVar3;
  int iVar4;
  long *******ppppppplVar5;
  long *******ppppppplVar6;
  long *******ppppppplVar7;
  long *******ppppppplVar8;
  long *******ppppppplVar9;
  long *******ppppppplVar10;
  long *******ppppppplVar11;
  long *******ppppppplVar12;
  long *******ppppppplVar13;
  long *******ppppppplVar14;
  bool bVar15;
  bool bVar16;
  char cVar17;
  undefined1 (*pauVar18) [16];
  ulong uVar19;
  ulong *puVar20;
  long ******pppppplVar21;
  long *******ppppppplVar22;
  long lVar23;
  long *******ppppppplVar24;
  ulong *puVar25;
  ulong *puVar26;
  long lVar27;
  undefined8 *puVar28;
  ulong uVar29;
  long *******ppppppplVar30;
  undefined8 *puVar31;
  long *******ppppppplVar32;
  long *******ppppppplVar33;
  undefined8 uVar34;
  long lVar35;
  long *******ppppppplVar36;
  long ******pppppplVar37;
  long *******ppppppplVar38;
  long *******ppppppplVar39;
  undefined8 uVar40;
  long *******ppppppplVar41;
  long *******ppppppplVar42;
  long *******ppppppplVar43;
  undefined1 auVar44 [16];
  long *******local_3d0;
  long *******ppppppplStack_3c8;
  long *******ppppppplStack_3c0;
  long *******ppppppplStack_3b8;
  long *******local_3b0;
  long *******ppppppplStack_3a8;
  long *******ppppppplStack_3a0;
  long *******ppppppplStack_398;
  long *******local_390;
  long *******ppppppplStack_388;
  undefined1 auStack_380 [24];
  undefined8 local_368;
  undefined8 uStack_360;
  undefined8 local_358;
  long *******local_350;
  long *******local_348;
  long *******local_340;
  long *******ppppppplStack_338;
  long *******local_330;
  long *******local_328;
  long *******local_320;
  long lStack_318;
  undefined8 uStack_310;
  long local_308;
  undefined1 local_300 [16];
  long *******local_2f0;
  long *******local_2e8;
  long *******local_2e0;
  long *******ppppppplStack_2d8;
  long *******local_2d0;
  long *******ppppppplStack_2c8;
  long *******ppppppplStack_2c0;
  long *******local_2b8;
  long *******local_2b0;
  long *******ppppppplStack_2a8;
  long *******ppppppplStack_2a0;
  long *******ppppppplStack_298;
  long *******local_290;
  long *******ppppppplStack_288;
  long *******ppppppplStack_280;
  long *******ppppppplStack_278;
  undefined8 *local_270;
  long ******pppppplStack_268;
  long *******local_260;
  long *******ppppppplStack_258;
  long *******local_250;
  long *******ppppppplStack_248;
  long *******local_240;
  long *******ppppppplStack_238;
  long *******local_230;
  long *******ppppppplStack_228;
  long *******local_220;
  long *******ppppppplStack_218;
  long *******local_210;
  long *******ppppppplStack_208;
  long *******local_200;
  long *******local_1f8;
  long *******local_1f0;
  long *******local_1e8;
  long *******local_1e0;
  long *******local_1d8;
  long *******local_1d0;
  long *******local_1c8;
  code *local_1c0;
  long *******local_1b8;
  code *local_1b0;
  long *******local_1a8;
  long *******ppppppplStack_1a0;
  long *******local_198;
  long *******ppppppplStack_190;
  long *******local_188;
  long *******ppppppplStack_180;
  long *******ppppppplStack_178;
  long *******ppppppplStack_170;
  long *******local_168;
  long *******ppppppplStack_160;
  long *******local_158;
  long *******ppppppplStack_150;
  undefined8 *local_148;
  long ******pppppplStack_140;
  long *******local_100;
  long *******ppppppplStack_f8;
  long *******ppppppplStack_f0;
  long *******ppppppplStack_e8;
  long *******ppppppplStack_e0;
  long *******ppppppplStack_d8;
  long *******ppppppplStack_d0;
  long *******local_c8;
  long *******ppppppplStack_c0;
  long *******local_b8;
  long *******ppppppplStack_b0;
  long *******local_a8;
  long *****appppplStack_91 [2];

  local_350 = (long *******)0x0;
  local_348 = (long *******)&SUB_00000008;
  ppppppplStack_338 = (long *******)0x0;
  local_340 = (long *******)0x0;
  local_328 = (long *******)0x0;
  local_330 = (long *******)0x0;
  collect_active_rollout_paths(&local_1a8);
  ppppppplVar6 = ppppppplStack_1a0;
  ppppppplVar5 = local_1a8;
  ppppppplVar39 = ppppppplStack_1a0 + (long)local_198 * 3;
  local_100 = ppppppplStack_1a0;
  ppppppplStack_f8 = ppppppplStack_1a0;
  ppppppplStack_f0 = local_1a8;
  ppppppplVar32 = ppppppplStack_1a0;
  ppppppplVar38 = ppppppplStack_1a0;
  ppppppplStack_e8 = ppppppplVar39;
  if (local_198 != (long *******)0x0) {
    do {
      ppppppplVar22 = ppppppplVar38 + 3;
      pppppplVar37 = *ppppppplVar38;
      ppppppplVar32 = ppppppplVar22;
      if (pppppplVar37 == (long ******)0x8000000000000000) break;
      ppppppplVar32 = (long *******)ppppppplVar38[1];
      ppppppplVar38 = (long *******)ppppppplVar38[2];
      auVar44 = rollout_history::logical_len(ppppppplVar32,ppppppplVar38);
      ppppppplVar2 = local_340;
      pppppplVar21 = auVar44._8_8_;
      if ((auVar44._0_8_ & 1) == 0) {
        if (local_340 == local_350) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_350);
        }
        local_348[(long)ppppppplVar2] = pppppplVar21;
        local_340 = (long *******)((long)ppppppplVar2 + 1);
      }
      else {
        cVar17 = std::io::error::Error::kind(pppppplVar21);
        if (cVar17 == '\0') {
          if (((ulong)pppppplVar21 & 3) == 1) {
            puVar28 = (undefined8 *)((long)pppppplVar21 + -1);
            uVar34 = *puVar28;
            puVar31 = *(undefined8 **)((long)pppppplVar21 + 7);
            if ((code *)*puVar31 != (code *)0x0) {
              (*(code *)*puVar31)(uVar34);
            }
            goto LAB_1008d5714;
          }
        }
        else {
          local_320 = (long *******)pppppplVar21;
          if (1 < log::MAX_LOG_LEVEL_FILTER) {
            local_2d0 = (long *******)&local_260;
            ppppppplStack_2c0 = (long *******)&local_320;
            ppppppplStack_2c8 = (long *******)<std::path::Display_as_core::fmt::Display>::fmt;
            local_2b8 = (long *******)<std::io::error::Error_as_core::fmt::Display>::fmt;
            local_168 = (long *******)&DAT_00000033;
            local_158 = (long *******)s___AiMaMi__rollout_capacity__cann_10153131f;
            ppppppplStack_150 = (long *******)&local_2d0;
            local_1a8 = (long *******)0x0;
            ppppppplStack_1a0 = (long *******)0x1013b537a;
            ppppppplStack_190 = (long *******)0x0;
            local_198 = (long *******)&DAT_00000033;
            ppppppplStack_170 = (long *******)0x1013b537a;
            local_188 = (long *******)0x10152fd89;
            ppppppplStack_178 = (long *******)0x2;
            ppppppplStack_180 = (long *******)&DAT_00000029;
            ppppppplStack_160 = (long *******)0x2ee00000001;
            local_260 = ppppppplVar32;
            ppppppplStack_258 = ppppppplVar38;
            <log::__private_api::GlobalLogger_as_log::Log>::log(appppplStack_91,&local_1a8);
          }
          if (((ulong)local_320 & 3) == 1) {
            puVar28 = (undefined8 *)((long)local_320 + -1);
            uVar34 = *puVar28;
            puVar31 = *(undefined8 **)((long)local_320 + 7);
            if ((code *)*puVar31 != (code *)0x0) {
              (*(code *)*puVar31)(uVar34);
            }
LAB_1008d5714:
            if (puVar31[1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar34,puVar31[1],puVar31[2]);
            }
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar28,0x18,8);
          }
        }
      }
      if (pppppplVar37 != (long ******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar32,pppppplVar37,1);
      }
      ppppppplVar32 = ppppppplVar39;
      ppppppplVar38 = ppppppplVar22;
    } while (ppppppplVar22 != ppppppplVar39);
  }
  ppppppplStack_f8 = ppppppplVar32;
  if ((long)ppppppplVar39 - (long)ppppppplVar32 != 0) {
    uVar29 = (ulong)((long)ppppppplVar39 - (long)ppppppplVar32) / 0x18;
    ppppppplVar32 = ppppppplVar32 + 1;
    do {
      if (ppppppplVar32[-1] != (long ******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppppppplVar32,ppppppplVar32[-1],1);
      }
      ppppppplVar32 = ppppppplVar32 + 3;
      uVar29 = uVar29 - 1;
    } while (uVar29 != 0);
  }
  if (ppppppplVar5 != (long *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar6,(long)ppppppplVar5 * 0x18,8);
  }
  pauVar18 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
  if (pauVar18[1][0] == '\x01') {
    local_300 = *pauVar18;
  }
  else {
    local_300 = std::sys::random::hashmap_random_keys();
    *pauVar18 = local_300;
    pauVar18[1][0] = 1;
  }
  *(long *)*pauVar18 = local_300._0_8_ + 1;
  lStack_318 = 0;
  local_320 = (long *******)&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_22;
  local_308 = 0;
  uStack_310 = 0;
  platform::paths::CodexPaths::all_codex_state_dbs(&local_1a8,param_2);
  ppppppplVar6 = ppppppplStack_1a0;
  ppppppplVar5 = local_1a8;
  uVar29 = (long)local_198 * 0x18;
  ppppppplVar38 = ppppppplStack_1a0 + (long)local_198 * 3;
  local_2f0 = ppppppplStack_1a0;
  local_2e0 = local_1a8;
  ppppppplVar32 = ppppppplStack_1a0;
  ppppppplStack_2d8 = ppppppplVar38;
  if (local_198 != (long *******)0x0) {
    uVar34 = *(undefined8 *)(param_2 + 0x158);
    uVar3 = *(undefined8 *)(param_2 + 0x160);
    ppppppplVar39 = ppppppplStack_1a0 + 4;
    ppppppplVar22 = ppppppplStack_1a0;
LAB_1008d589c:
    uVar29 = uVar29 - 0x18;
    ppppppplVar32 = ppppppplVar22 + 3;
    pppppplVar37 = *ppppppplVar22;
    if (pppppplVar37 != (long ******)0x8000000000000000) {
      ppppppplVar2 = (long *******)ppppppplVar22[1];
      ppppppplVar22 = (long *******)ppppppplVar22[2];
      local_2e8 = ppppppplVar32;
      std::sys::fs::metadata(&local_1a8,ppppppplVar2,ppppppplVar22);
      ppppppplVar24 = local_1a8;
      ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_1a8,ppppppplStack_1a0);
      if (((ulong)ppppppplVar24 & 1) != 0) goto LAB_1008d617c;
      open_codex_db_readonly(&local_1a8,ppppppplVar2,ppppppplVar22);
      if ((char)pppppplStack_140 != '\x03') {
        ppppppplStack_288 = ppppppplStack_160;
        local_290 = local_168;
        ppppppplStack_278 = ppppppplStack_150;
        ppppppplStack_280 = local_158;
        pppppplStack_268 = pppppplStack_140;
        local_270 = local_148;
        ppppppplStack_2c8 = ppppppplStack_1a0;
        local_2d0 = local_1a8;
        local_2b8 = ppppppplStack_190;
        ppppppplStack_2c0 = local_198;
        ppppppplStack_2a8 = ppppppplStack_180;
        local_2b0 = local_188;
        ppppppplStack_298 = ppppppplStack_170;
        ppppppplStack_2a0 = ppppppplStack_178;
        collect_provider_convergence_targets(&local_260,&local_2d0);
        if (local_260 == (long *******)&DAT_0000000b) {
          ppppppplVar24 = local_250 + (long)ppppppplStack_248 * 0xc;
          local_200 = local_250;
          local_1f8 = local_250;
          local_1f0 = ppppppplStack_258;
          local_1e8 = ppppppplVar24;
          if (ppppppplStack_248 == (long *******)0x0) {
            bVar16 = false;
            ppppppplVar36 = local_1f8;
          }
          else {
            ppppppplVar30 = local_250;
            if ((param_3 & 1) == 0) {
              do {
                ppppppplVar36 = ppppppplVar30 + 0xc;
                ppppppplVar41 = (long *******)*ppppppplVar30;
                ppppppplStack_238 = (long *******)ppppppplVar30[6];
                local_240 = (long *******)ppppppplVar30[5];
                ppppppplStack_228 = (long *******)ppppppplVar30[8];
                local_230 = (long *******)ppppppplVar30[7];
                ppppppplStack_218 = (long *******)ppppppplVar30[10];
                local_220 = (long *******)ppppppplVar30[9];
                local_210 = (long *******)ppppppplVar30[0xb];
                ppppppplStack_258 = (long *******)ppppppplVar30[2];
                local_260 = (long *******)ppppppplVar30[1];
                ppppppplStack_248 = (long *******)ppppppplVar30[4];
                local_250 = (long *******)ppppppplVar30[3];
                if (ppppppplVar41 == (long *******)&DAT_8000000000000000) {
                  bVar16 = false;
                  goto LAB_1008d6018;
                }
                local_100 = ppppppplVar41;
                ppppppplStack_f8 = local_260;
                ppppppplStack_f0 = ppppppplStack_258;
                ppppppplStack_e8 = local_250;
                ppppppplStack_e0 = ppppppplStack_248;
                ppppppplStack_d8 = local_240;
                ppppppplStack_d0 = ppppppplStack_238;
                local_c8 = local_230;
                ppppppplStack_c0 = ppppppplStack_228;
                local_b8 = local_220;
                ppppppplStack_b0 = ppppppplStack_218;
                local_a8 = local_210;
                require_existing_rollout(&local_1a8,&local_100);
                ppppppplVar42 = ppppppplStack_170;
                ppppppplVar43 = ppppppplStack_178;
                ppppppplVar7 = ppppppplStack_160;
                ppppppplVar8 = local_168;
                ppppppplVar9 = ppppppplStack_150;
                ppppppplVar10 = local_158;
                ppppppplVar11 = ppppppplStack_180;
                ppppppplVar12 = local_188;
                ppppppplVar13 = ppppppplStack_190;
                ppppppplVar14 = local_198;
                ppppppplVar33 = local_1a8;
                ppppppplVar30 = ppppppplStack_1a0;
                if (local_1a8 != (long *******)&DAT_0000000b) goto joined_r0x0001008d62e8;
                if (ppppppplVar41 != (long *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_f8,ppppppplVar41,1);
                }
                if ((ppppppplStack_d0 != (long *******)&DAT_8000000000000000) && (ppppppplStack_d0 != (long *******)0x0)) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_c8,ppppppplStack_d0,1);
                }
                if (ppppppplStack_e8 != (long *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_e0,ppppppplStack_e8,1);
                }
                if ((local_b8 != (long *******)&DAT_8000000000000000) && (local_b8 != (long *******)0x0)) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_b0,local_b8,1);
                }
                ppppppplVar30 = ppppppplVar36;
              } while (ppppppplVar36 != ppppppplVar24);
              bVar16 = false;
              ppppppplVar36 = ppppppplVar24;
            }
            else {
              bVar16 = false;
LAB_1008d5b34:
              do {
                ppppppplVar33 = ppppppplVar30 + 0xc;
                ppppppplVar41 = (long *******)*ppppppplVar30;
                ppppppplStack_258 = (long *******)ppppppplVar30[2];
                local_260 = (long *******)ppppppplVar30[1];
                ppppppplStack_248 = (long *******)ppppppplVar30[4];
                local_250 = (long *******)ppppppplVar30[3];
                ppppppplStack_238 = (long *******)ppppppplVar30[6];
                local_240 = (long *******)ppppppplVar30[5];
                ppppppplVar43 = (long *******)ppppppplVar30[8];
                ppppppplVar42 = (long *******)ppppppplVar30[7];
                ppppppplStack_218 = (long *******)ppppppplVar30[10];
                local_220 = (long *******)ppppppplVar30[9];
                local_210 = (long *******)ppppppplVar30[0xb];
                local_230 = ppppppplVar42;
                ppppppplStack_228 = ppppppplVar43;
                ppppppplVar36 = ppppppplVar33;
                if (ppppppplVar41 == (long *******)&DAT_8000000000000000) break;
                ppppppplVar30 = ppppppplVar33;
                local_100 = ppppppplVar41;
                ppppppplStack_f8 = local_260;
                ppppppplStack_f0 = ppppppplStack_258;
                ppppppplStack_e8 = local_250;
                ppppppplStack_e0 = ppppppplStack_248;
                ppppppplStack_d8 = local_240;
                ppppppplStack_d0 = ppppppplStack_238;
                local_c8 = ppppppplVar42;
                ppppppplStack_c0 = ppppppplVar43;
                local_b8 = local_220;
                ppppppplStack_b0 = ppppppplStack_218;
                local_a8 = local_210;
                if (ppppppplStack_238 == (long *******)&DAT_8000000000000000) {
                  bVar16 = true;
                  goto joined_r0x0001008d5e58;
                }
                rollout_history::resolve(&local_1a8,ppppppplVar42,ppppppplVar43);
                if (local_1a8 != (long *******)&DAT_8000000000000000) {
                  if (local_1a8 != (long *******)&DAT_8000000000000001) {
                    if (local_1a8 != (long *******)0x0) {
                      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_1a0,local_1a8,1);
                    }
                    goto joined_r0x0001008d5e58;
                  }
                  ppppppplVar42 = ppppppplStack_3a8;
                  ppppppplVar43 = local_3b0;
                  ppppppplVar7 = ppppppplStack_398;
                  ppppppplVar8 = ppppppplStack_3a0;
                  ppppppplVar9 = ppppppplStack_388;
                  ppppppplVar10 = local_390;
                  ppppppplVar11 = ppppppplStack_3b8;
                  ppppppplVar12 = ppppppplStack_3c0;
                  ppppppplVar13 = ppppppplStack_3c8;
                  ppppppplVar14 = local_3d0;
                  ppppppplVar33 = (long *******)0x2;
                  ppppppplVar30 = ppppppplStack_1a0;
joined_r0x0001008d62e8:
                  local_3d0 = ppppppplVar14;
                  ppppppplStack_3c8 = ppppppplVar13;
                  ppppppplStack_3c0 = ppppppplVar12;
                  ppppppplStack_3b8 = ppppppplVar11;
                  local_390 = ppppppplVar10;
                  ppppppplStack_388 = ppppppplVar9;
                  ppppppplStack_3a0 = ppppppplVar8;
                  ppppppplStack_398 = ppppppplVar7;
                  local_3b0 = ppppppplVar43;
                  ppppppplStack_3a8 = ppppppplVar42;
                  local_1f8 = ppppppplVar36;
                  ppppppplStack_1a0 = ppppppplVar30;
                  if (ppppppplVar41 != (long *******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_f8,ppppppplVar41,1);
                  }
                  if ((ppppppplStack_d0 != (long *******)&DAT_8000000000000000) && (ppppppplStack_d0 != (long *******)0x0)) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_c8,ppppppplStack_d0,1);
                  }
                  if (ppppppplStack_e8 != (long *******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_e0,ppppppplStack_e8,1);
                  }
                  if ((local_b8 != (long *******)&DAT_8000000000000000) && (local_b8 != (long *******)0x0)) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_b0,local_b8,1);
                  }
                  <alloc::vec::into_iter::IntoIter<T,A>as_core::ops::drop::Drop>::drop(&local_200);
                  goto LAB_1008d635c;
                }
                find_trusted_rollout_backup(&local_1a8,uVar34,uVar3,ppppppplStack_f8,ppppppplStack_f0,ppppppplVar42,ppppppplVar43);
                ppppppplVar8 = local_198;
                ppppppplVar7 = ppppppplStack_1a0;
                ppppppplVar36 = local_1a8;
                if (local_1a8 != (long *******)&DAT_8000000000000000) {
                  rollout_path_identity(&local_1a8,ppppppplVar42,ppppppplVar43);
                  uVar19 = hashbrown::map::HashMap<K,V,S,A>::insert(&local_320,&local_1a8);
                  if ((uVar19 & 1) != 0) {
                    if (ppppppplVar36 != (long *******)0x0) {
                      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar7,ppppppplVar36,1);
                    }
                    goto joined_r0x0001008d5e58;
                  }
                  std::sys::fs::metadata(&local_1a8,ppppppplVar7,ppppppplVar8);
                  pppppplVar21 = pppppplStack_140;
                  ppppppplVar42 = local_340;
                  if (((ulong)local_1a8 & 1) == 0) {
                    if (((ulong)ppppppplStack_1a0 & 0xf00000000000) == 0x800000000000) {
                      bVar15 = CARRY8((ulong)ppppppplStack_338,(ulong)pppppplStack_140);
                      ppppppplStack_338 = (long *******)((long)ppppppplStack_338 + (long)pppppplStack_140);
                      if (bVar15) {
                        ppppppplStack_338 = (long *******)0xffffffffffffffff;
                      }
                      if (local_340 == local_350) {
                        alloc::raw_vec::RawVec<T,A>::grow_one(&local_350);
                      }
                      local_348[(long)ppppppplVar42] = pppppplVar21;
                      local_340 = (long *******)((long)ppppppplVar42 + 1);
                      local_330 = (long *******)((long)local_330 + 1);
                    }
                  }
                  else {
                    local_1e0 = ppppppplStack_1a0;
                    if (1 < log::MAX_LOG_LEVEL_FILTER) {
                      local_1d8 = ppppppplVar7;
                      local_1d0 = ppppppplVar8;
                      local_1c8 = (long *******)&local_1d8;
                      local_1c0 = <std::path::Display_as_core::fmt::Display>::fmt;
                      local_1b8 = (long *******)&local_1e0;
                      local_1b0 = <std::io::error::Error_as_core::fmt::Display>::fmt;
                      local_168 = (long *******)&DAT_00000033;
                      local_158 = (long *******)s_A_AiMaMi__rollout_capacity__cann_101531250;
                      ppppppplStack_150 = (long *******)&local_1c8;
                      local_1a8 = (long *******)0x0;
                      ppppppplStack_1a0 = (long *******)0x1013b537a;
                      ppppppplStack_190 = (long *******)0x0;
                      local_198 = (long *******)&DAT_00000033;
                      ppppppplStack_170 = (long *******)0x1013b537a;
                      local_188 = (long *******)0x10152fd89;
                      ppppppplStack_178 = (long *******)0x2;
                      ppppppplStack_180 = (long *******)&DAT_00000029;
                      ppppppplStack_160 = (long *******)0x32f00000001;
                      <log::__private_api::GlobalLogger_as_log::Log>::log(appppplStack_91,&local_1a8);
                    }
                    if (((ulong)local_1e0 & 3) == 1) {
                      puVar31 = (undefined8 *)((long)local_1e0 + -1);
                      uVar40 = *puVar31;
                      puVar28 = *(undefined8 **)((long)local_1e0 + 7);
                      if ((code *)*puVar28 != (code *)0x0) {
                        (*(code *)*puVar28)(uVar40);
                      }
                      if (puVar28[1] != 0) {
                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar40,puVar28[1],puVar28[2]);
                      }
                      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar31,0x18,8);
                    }
                  }
                  if (ppppppplVar36 != (long *******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar7,ppppppplVar36,1);
                  }
                  if (ppppppplVar41 != (long *******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_f8,ppppppplVar41,1);
                  }
                  if ((ppppppplStack_d0 != (long *******)&DAT_8000000000000000) && (ppppppplStack_d0 != (long *******)0x0)) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_c8,ppppppplStack_d0,1);
                  }
                  if (ppppppplStack_e8 != (long *******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_e0,ppppppplStack_e8,1);
                  }
                  if ((local_b8 != (long *******)&DAT_8000000000000000) && (local_b8 != (long *******)0x0)) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_b0,local_b8,1);
                  }
                  ppppppplVar36 = ppppppplVar24;
                  if (ppppppplVar33 == ppppppplVar24) break;
                  goto LAB_1008d5b34;
                }
                bVar16 = true;
joined_r0x0001008d5e58:
                if (ppppppplVar41 != (long *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_f8,ppppppplVar41,1);
                }
                if ((ppppppplStack_d0 != (long *******)&DAT_8000000000000000) && (ppppppplStack_d0 != (long *******)0x0)) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_c8,ppppppplStack_d0,1);
                }
                if (ppppppplStack_e8 != (long *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_e0,ppppppplStack_e8,1);
                }
                if ((local_b8 != (long *******)&DAT_8000000000000000) && (local_b8 != (long *******)0x0)) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_b0,local_b8,1);
                }
                ppppppplVar36 = ppppppplVar24;
              } while (ppppppplVar33 != ppppppplVar24);
            }
          }
LAB_1008d6018:
          local_1f8 = ppppppplVar36;
          <alloc::vec::into_iter::IntoIter<T,A>as_core::ops::drop::Drop>::drop(&local_200);
          if (bVar16) {
            std::sys::fs::metadata(&local_1a8,ppppppplVar2,ppppppplVar22);
            if (((ulong)local_1a8 & 1) == 0) {
              if (((ulong)ppppppplStack_1a0 & 0xf00000000000) == 0x800000000000) {
                bVar16 = CARRY8((ulong)ppppppplStack_338,(ulong)pppppplStack_140);
                ppppppplStack_338 = (long *******)((long)ppppppplStack_338 + (long)pppppplStack_140);
                if (bVar16) {
                  ppppppplStack_338 = (long *******)0xffffffffffffffff;
                }
                local_328 = (long *******)((long)local_328 + 1);
              }
            }
            else {
              local_1c8 = ppppppplStack_1a0;
              if (1 < log::MAX_LOG_LEVEL_FILTER) {
                local_100 = (long *******)&local_260;
                ppppppplStack_f0 = (long *******)&local_1c8;
                ppppppplStack_f8 = (long *******)<std::path::Display_as_core::fmt::Display>::fmt;
                ppppppplStack_e8 = (long *******)<std::io::error::Error_as_core::fmt::Display>::fmt;
                local_168 = (long *******)&DAT_00000033;
                local_158 = (long *******)s_9_AiMaMi__rollout_capacity__cann_101531210;
                ppppppplStack_150 = (long *******)&local_100;
                local_1a8 = (long *******)0x0;
                ppppppplStack_1a0 = (long *******)0x1013b537a;
                ppppppplStack_190 = (long *******)0x0;
                local_198 = (long *******)&DAT_00000033;
                ppppppplStack_170 = (long *******)0x1013b537a;
                local_188 = (long *******)0x10152fd89;
                ppppppplStack_178 = (long *******)0x2;
                ppppppplStack_180 = (long *******)&DAT_00000029;
                ppppppplStack_160 = (long *******)0x33f00000001;
                local_260 = ppppppplVar2;
                ppppppplStack_258 = ppppppplVar22;
                <log::__private_api::GlobalLogger_as_log::Log>::log(appppplStack_91,&local_1a8);
              }
              if (((ulong)local_1c8 & 3) == 1) {
                puVar28 = (undefined8 *)((long)local_1c8 + -1);
                uVar40 = *puVar28;
                puVar31 = *(undefined8 **)((long)local_1c8 + 7);
                if ((code *)*puVar31 != (code *)0x0) {
                  (*(code *)*puVar31)(uVar40);
                }
                if (puVar31[1] != 0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar40,puVar31[1],puVar31[2]);
                }
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar28,0x18,8);
              }
            }
          }
        }
        else {
          if ((param_3 & 1) == 0) {
            ppppppplStack_3a8 = ppppppplStack_228;
            local_3b0 = local_230;
            ppppppplStack_398 = ppppppplStack_218;
            ppppppplStack_3a0 = local_220;
            ppppppplStack_388 = ppppppplStack_208;
            local_390 = local_210;
            ppppppplStack_3c8 = ppppppplStack_248;
            local_3d0 = local_250;
            ppppppplStack_3b8 = ppppppplStack_238;
            ppppppplStack_3c0 = local_240;
            ppppppplVar30 = ppppppplStack_258;
            ppppppplVar33 = local_260;
LAB_1008d635c:
            ::core::ptr::drop_in_place<rusqlite::Connection>(&local_2d0);
            ppppppplVar24 = local_3d0;
            ppppppplVar41 = ppppppplStack_3c8;
            ppppppplVar36 = ppppppplStack_3c0;
            ppppppplVar42 = ppppppplStack_3b8;
            ppppppplVar43 = local_3b0;
            ppppppplVar7 = ppppppplStack_3a8;
            ppppppplVar8 = ppppppplStack_3a0;
            ppppppplVar9 = ppppppplStack_398;
            ppppppplVar10 = local_390;
            ppppppplVar11 = ppppppplStack_388;
            goto joined_r0x0001008d6364;
          }
          ppppppplStack_d8 = ppppppplStack_238;
          ppppppplStack_e0 = local_240;
          local_c8 = ppppppplStack_228;
          ppppppplStack_d0 = local_230;
          local_b8 = ppppppplStack_218;
          ppppppplStack_c0 = local_220;
          local_a8 = ppppppplStack_208;
          ppppppplStack_b0 = local_210;
          ppppppplStack_f8 = ppppppplStack_258;
          local_100 = local_260;
          ppppppplStack_e8 = ppppppplStack_248;
          ppppppplStack_f0 = local_250;
          if (1 < log::MAX_LOG_LEVEL_FILTER) {
            local_1c8 = (long *******)&local_200;
            local_1c0 = <std::path::Display_as_core::fmt::Display>::fmt;
            local_1b8 = (long *******)&local_100;
            local_1b0 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
            local_168 = (long *******)&DAT_00000033;
            local_158 = (long *******)s___AiMaMi__rollout_capacity__cann_101531298;
            ppppppplStack_150 = (long *******)&local_1c8;
            local_1a8 = (long *******)0x0;
            ppppppplStack_1a0 = (long *******)0x1013b537a;
            ppppppplStack_190 = (long *******)0x0;
            local_198 = (long *******)&DAT_00000033;
            ppppppplStack_170 = (long *******)0x1013b537a;
            local_188 = (long *******)0x10152fd89;
            ppppppplStack_178 = (long *******)0x2;
            ppppppplStack_180 = (long *******)&DAT_00000029;
            ppppppplStack_160 = (long *******)0x30a00000001;
            local_200 = ppppppplVar2;
            local_1f8 = ppppppplVar22;
            <log::__private_api::GlobalLogger_as_log::Log>::log(appppplStack_91,&local_1a8);
          }
          ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_100);
        }
        ::core::ptr::drop_in_place<rusqlite::Connection>(&local_2d0);
        goto LAB_1008d617c;
      }
      ppppppplVar30 = ppppppplStack_1a0;
      ppppppplVar33 = local_1a8;
      ppppppplVar24 = local_198;
      ppppppplVar41 = ppppppplStack_190;
      ppppppplVar36 = local_188;
      ppppppplVar42 = ppppppplStack_180;
      ppppppplVar43 = ppppppplStack_178;
      ppppppplVar7 = ppppppplStack_170;
      ppppppplVar8 = local_168;
      ppppppplVar9 = ppppppplStack_160;
      ppppppplVar10 = local_158;
      ppppppplVar11 = ppppppplStack_150;
      if ((param_3 & 1) != 0) {
        ppppppplStack_238 = ppppppplStack_180;
        local_240 = local_188;
        ppppppplStack_228 = ppppppplStack_170;
        local_230 = ppppppplStack_178;
        ppppppplStack_218 = ppppppplStack_160;
        local_220 = local_168;
        ppppppplStack_208 = ppppppplStack_150;
        local_210 = local_158;
        ppppppplStack_258 = ppppppplStack_1a0;
        local_260 = local_1a8;
        ppppppplStack_248 = ppppppplStack_190;
        local_250 = local_198;
        if (1 < log::MAX_LOG_LEVEL_FILTER) {
          local_1c8 = (long *******)&local_200;
          local_1c0 = <std::path::Display_as_core::fmt::Display>::fmt;
          local_1b8 = (long *******)&local_260;
          local_1b0 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
          ppppppplStack_c0 = (long *******)&DAT_00000033;
          local_a8 = (long *******)&local_1c8;
          ppppppplStack_b0 = (long *******)s_>_AiMaMi__rollout_capacity__cann_1015312da;
          local_100 = (long *******)0x0;
          ppppppplStack_f8 = (long *******)0x1013b537a;
          ppppppplStack_e8 = (long *******)0x0;
          ppppppplStack_f0 = (long *******)&DAT_00000033;
          local_c8 = (long *******)0x1013b537a;
          ppppppplStack_e0 = (long *******)0x10152fd89;
          ppppppplStack_d0 = (long *******)0x2;
          ppppppplStack_d8 = (long *******)&DAT_00000029;
          local_b8 = (long *******)0x2ff00000001;
          local_200 = ppppppplVar2;
          local_1f8 = ppppppplVar22;
          <log::__private_api::GlobalLogger_as_log::Log>::log(appppplStack_91,&local_100);
        }
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_260);
        goto LAB_1008d617c;
      }
joined_r0x0001008d6364:
      ppppppplStack_388 = ppppppplVar11;
      local_390 = ppppppplVar10;
      ppppppplStack_398 = ppppppplVar9;
      ppppppplStack_3a0 = ppppppplVar8;
      ppppppplStack_3a8 = ppppppplVar7;
      local_3b0 = ppppppplVar43;
      ppppppplStack_3b8 = ppppppplVar42;
      ppppppplStack_3c0 = ppppppplVar36;
      ppppppplStack_3c8 = ppppppplVar41;
      local_3d0 = ppppppplVar24;
      if (pppppplVar37 != (long ******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar2,pppppplVar37,1);
      }
      if (ppppppplVar38 != ppppppplVar32) {
        uVar29 = uVar29 / 0x18;
        do {
          if (ppppppplVar39[-1] != (long ******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppppppplVar39,ppppppplVar39[-1],1);
          }
          ppppppplVar39 = ppppppplVar39 + 3;
          uVar29 = uVar29 - 1;
        } while (uVar29 != 0);
      }
      if (ppppppplVar5 != (long *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar6,(long)ppppppplVar5 * 0x18,8);
      }
      lVar23 = lStack_318;
      if (lStack_318 != 0) {
        if (local_308 != 0) {
          ppppppplVar38 = local_320 + 1;
          pppppplVar37 = *local_320;
          uVar29 = CONCAT17(-(-1 < (long)pppppplVar37),
                            CONCAT16(-(-1 < (char)((ulong)pppppplVar37 >> 0x30)),
                                     CONCAT15(-(-1 < (char)((ulong)pppppplVar37 >> 0x28)),
                                              CONCAT14(-(-1 < (char)((ulong)pppppplVar37 >> 0x20)),
                                                       CONCAT13(-(-1 < (char)((ulong)pppppplVar37 >> 0x18)),
                                                                CONCAT12(-(-1 < (char)((ulong)pppppplVar37 >> 0x10)),
                                                                         CONCAT11(-(-1 < (char)((ulong)pppppplVar37 >> 8)),-(-1 < (char)pppppplVar37)))))))) &
                   0x8080808080808080;
          ppppppplVar32 = local_320;
          lVar35 = local_308;
          do {
            while (uVar29 == 0) {
              pppppplVar37 = *ppppppplVar38;
              ppppppplVar32 = ppppppplVar32 + -0x18;
              ppppppplVar38 = ppppppplVar38 + 1;
              uVar29 = CONCAT17(-(-1 < (long)pppppplVar37),
                                CONCAT16(-(-1 < (char)((ulong)pppppplVar37 >> 0x30)),
                                         CONCAT15(-(-1 < (char)((ulong)pppppplVar37 >> 0x28)),
                                                  CONCAT14(-(-1 < (char)((ulong)pppppplVar37 >> 0x20)),
                                                           CONCAT13(-(-1 < (char)((ulong)pppppplVar37 >> 0x18)),
                                                                    CONCAT12(-(-1 < (char)((ulong)pppppplVar37 >> 0x10)),
                                                                             CONCAT11(-(-1 < (char)((ulong)pppppplVar37 >> 8)),-(-1 < (char)pppppplVar37))))))))
                       & 0x8080808080808080;
            }
            uVar19 = (uVar29 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar29 >> 7 & 0xff00ff00ff00ff) << 8;
            uVar19 = (uVar19 & 0xffff0000ffff0000) >> 0x10 | (uVar19 & 0xffff0000ffff) << 0x10;
            iVar4 = (int)((ulong)LZCOUNT(uVar19 >> 0x20 | uVar19 << 0x20) >> 3);
            if (ppppppplVar32[(long)-iVar4 * 3 + -3] != (long ******)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar32[(long)-iVar4 * 3 + -2],ppppppplVar32[(long)-iVar4 * 3 + -3],1);
            }
            uVar29 = uVar29 - 1 & uVar29;
            lVar35 = lVar35 + -1;
          } while (lVar35 != 0);
        }
        lVar35 = lVar23 * 0x19 + 0x21;
        if (lVar35 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_320 + lVar23 * -3 + -3,lVar35,8);
        }
      }
      if (local_350 != (long *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_348,(long)local_350 << 3,8);
      }
      if (ppppppplVar33 != (long *******)&DAT_0000000b) {
        param_1[8] = ppppppplStack_3a0;
        param_1[7] = ppppppplStack_3a8;
        param_1[10] = local_390;
        param_1[9] = ppppppplStack_398;
        param_1[0xb] = ppppppplStack_388;
        param_1[3] = ppppppplStack_3c8;
        param_1[2] = local_3d0;
        param_1[5] = ppppppplStack_3b8;
        param_1[4] = ppppppplStack_3c0;
        param_1[6] = local_3b0;
        *param_1 = ppppppplVar33;
        param_1[1] = ppppppplVar30;
        return;
      }
      goto LAB_1008d64a4;
    }
  }
  local_2e8 = ppppppplVar32;
  if ((long)ppppppplVar38 - (long)ppppppplVar32 != 0) {
    uVar29 = (ulong)((long)ppppppplVar38 - (long)ppppppplVar32) / 0x18;
    ppppppplVar32 = ppppppplVar32 + 1;
    do {
      if (ppppppplVar32[-1] != (long ******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppppppplVar32,ppppppplVar32[-1],1);
      }
      ppppppplVar32 = ppppppplVar32 + 3;
      uVar29 = uVar29 - 1;
    } while (uVar29 != 0);
  }
LAB_1008d61dc:
  if (ppppppplVar5 != (long *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar6,(long)ppppppplVar5 * 0x18,8);
  }
  lVar23 = lStack_318;
  ppppppplVar30 = local_350;
  ppppppplStack_3c8 = local_340;
  local_3d0 = local_348;
  ppppppplStack_3b8 = local_330;
  ppppppplStack_3c0 = ppppppplStack_338;
  local_3b0 = local_328;
  if (lStack_318 != 0) {
    if (local_308 != 0) {
      ppppppplVar38 = local_320 + 1;
      pppppplVar37 = *local_320;
      uVar29 = CONCAT17(-(-1 < (long)pppppplVar37),
                        CONCAT16(-(-1 < (char)((ulong)pppppplVar37 >> 0x30)),
                                 CONCAT15(-(-1 < (char)((ulong)pppppplVar37 >> 0x28)),
                                          CONCAT14(-(-1 < (char)((ulong)pppppplVar37 >> 0x20)),
                                                   CONCAT13(-(-1 < (char)((ulong)pppppplVar37 >> 0x18)),
                                                            CONCAT12(-(-1 < (char)((ulong)pppppplVar37 >> 0x10)),
                                                                     CONCAT11(-(-1 < (char)((ulong)pppppplVar37 >> 8)),-(-1 < (char)pppppplVar37)))))))) &
               0x8080808080808080;
      ppppppplVar32 = local_320;
      lVar35 = local_308;
      do {
        while (uVar29 == 0) {
          pppppplVar37 = *ppppppplVar38;
          ppppppplVar32 = ppppppplVar32 + -0x18;
          ppppppplVar38 = ppppppplVar38 + 1;
          uVar29 = CONCAT17(-(-1 < (long)pppppplVar37),
                            CONCAT16(-(-1 < (char)((ulong)pppppplVar37 >> 0x30)),
                                     CONCAT15(-(-1 < (char)((ulong)pppppplVar37 >> 0x28)),
                                              CONCAT14(-(-1 < (char)((ulong)pppppplVar37 >> 0x20)),
                                                       CONCAT13(-(-1 < (char)((ulong)pppppplVar37 >> 0x18)),
                                                                CONCAT12(-(-1 < (char)((ulong)pppppplVar37 >> 0x10)),
                                                                         CONCAT11(-(-1 < (char)((ulong)pppppplVar37 >> 8)),-(-1 < (char)pppppplVar37)))))))) &
                   0x8080808080808080;
        }
        uVar19 = (uVar29 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar29 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar19 = (uVar19 & 0xffff0000ffff0000) >> 0x10 | (uVar19 & 0xffff0000ffff) << 0x10;
        iVar4 = (int)((ulong)LZCOUNT(uVar19 >> 0x20 | uVar19 << 0x20) >> 3);
        if (ppppppplVar32[(long)-iVar4 * 3 + -3] != (long ******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar32[(long)-iVar4 * 3 + -2],ppppppplVar32[(long)-iVar4 * 3 + -3],1);
        }
        uVar29 = uVar29 - 1 & uVar29;
        lVar35 = lVar35 + -1;
      } while (lVar35 != 0);
    }
    lVar35 = lVar23 * 0x19 + 0x21;
    if (lVar35 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_320 + lVar23 * -3 + -3,lVar35,8);
    }
  }
LAB_1008d64a4:
  ppppppplStack_2c0 = ppppppplStack_3c8;
  ppppppplStack_2c8 = local_3d0;
  local_2b0 = ppppppplStack_3b8;
  local_2b8 = ppppppplStack_3c0;
  ppppppplStack_2a8 = local_3b0;
  local_2d0 = ppppppplVar30;
  fs2::unix::statvfs(&local_1a8,*(undefined8 *)(param_2 + 8),*(undefined8 *)(param_2 + 0x10));
  ppppppplVar6 = local_198;
  ppppppplVar5 = local_2b8;
  ppppppplVar38 = ppppppplStack_2c0;
  ppppppplVar32 = ppppppplStack_2c8;
  if (((ulong)local_1a8 & 1) != 0) {
    local_260 = ppppppplStack_1a0;
    if (1 < log::MAX_LOG_LEVEL_FILTER) {
      local_100 = (long *******)&local_260;
      ppppppplStack_f8 = (long *******)<std::io::error::Error_as_core::fmt::Display>::fmt;
      local_168 = (long *******)&DAT_00000033;
      ppppppplStack_150 = (long *******)&local_100;
      local_158 = (long *******)s_f_AiMaMi__rollout_capacity__free_1015317a4;
      local_1a8 = (long *******)0x0;
      ppppppplStack_1a0 = (long *******)0x1013b537a;
      ppppppplStack_190 = (long *******)0x0;
      local_198 = (long *******)&DAT_00000033;
      ppppppplStack_170 = (long *******)0x1013b537a;
      local_188 = (long *******)0x10152fd89;
      ppppppplStack_178 = (long *******)0x2;
      ppppppplStack_180 = (long *******)&DAT_00000029;
      ppppppplStack_160 = (long *******)0x2b500000001;
      <log::__private_api::GlobalLogger_as_log::Log>::log(appppplStack_91,&local_1a8);
    }
    *param_1 = 0xb;
    if (((ulong)local_260 & 3) == 1) {
      puVar28 = (undefined8 *)((long)local_260 + -1);
      uVar34 = *puVar28;
      puVar31 = *(undefined8 **)((long)local_260 + 7);
      if ((code *)*puVar31 != (code *)0x0) {
        (*(code *)*puVar31)(uVar34);
      }
      if (puVar31[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar34,puVar31[1],puVar31[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar28,0x18,8);
    }
    goto LAB_1008d684c;
  }
  uVar34 = 0;
  uVar29 = (long)ppppppplStack_2c0 * 8;
  if (((ulong)ppppppplStack_2c0 >> 0x3d != 0) || (0x7ffffffffffffff8 < uVar29)) {
LAB_1008d6628:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar34,uVar29);
  }
  if (uVar29 == 0) {
    puVar20 = (ulong *)&SUB_00000008;
    ppppppplVar39 = (long *******)0x0;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar34 = 8;
    puVar20 = (ulong *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(uVar29,8);
    ppppppplVar39 = ppppppplVar38;
    if (puVar20 == (ulong *)0x0) goto LAB_1008d6628;
  }
  _memcpy(puVar20,ppppppplVar32,uVar29);
  local_1a8 = (long *******)appppplStack_91;
  if (ppppppplVar38 < (long *******)0x2) {
LAB_1008d66a0:
    ppppppplVar32 = ppppppplVar5 + 0x2000000;
    if ((long *******)0xffffffffefffffff < ppppppplVar5) {
      ppppppplVar32 = (long *******)0xffffffffffffffff;
    }
    if (ppppppplVar38 != (long *******)0x0) {
      bVar16 = CARRY8((ulong)ppppppplVar32,*puVar20);
      ppppppplVar32 = (long *******)((long)ppppppplVar32 + *puVar20);
      if (bVar16) {
        ppppppplVar32 = (long *******)0xffffffffffffffff;
      }
      if (ppppppplVar38 != (long *******)0x1) {
        bVar16 = CARRY8((ulong)ppppppplVar32,puVar20[1]);
        ppppppplVar32 = (long *******)((long)ppppppplVar32 + puVar20[1]);
        if (bVar16) {
          ppppppplVar32 = (long *******)0xffffffffffffffff;
        }
        if (ppppppplVar38 != (long *******)0x2) {
          bVar16 = CARRY8((ulong)ppppppplVar32,puVar20[2]);
          ppppppplVar32 = (long *******)((long)ppppppplVar32 + puVar20[2]);
          if (bVar16) {
            ppppppplVar32 = (long *******)0xffffffffffffffff;
          }
          if (ppppppplVar38 != (long *******)0x3) goto LAB_1008d66ec;
        }
      }
    }
  }
  else {
    if (ppppppplVar38 < &DAT_00000015) {
      lVar23 = 0;
      puVar25 = puVar20 + 1;
      do {
        uVar29 = puVar25[-1];
        uVar19 = *puVar25;
        lVar35 = lVar23;
        if (uVar29 < uVar19) {
          do {
            lVar27 = lVar35;
            *(ulong *)((long)puVar20 + lVar27 + 8) = uVar29;
            puVar26 = puVar20;
            if (lVar27 == 0) goto LAB_1008d6924;
            uVar29 = *(ulong *)((long)puVar20 + lVar27 + -8);
            lVar35 = lVar27 + -8;
          } while (uVar29 < uVar19);
          puVar26 = (ulong *)((long)puVar20 + lVar27);
LAB_1008d6924:
          *puVar26 = uVar19;
        }
        puVar25 = puVar25 + 1;
        lVar23 = lVar23 + 8;
      } while (puVar25 != puVar20 + (long)ppppppplVar38);
      goto LAB_1008d66a0;
    }
    ::core::slice::sort::unstable::ipnsort(puVar20,ppppppplVar38,&local_1a8);
    ppppppplVar32 = ppppppplVar5 + 0x2000000;
    if ((long *******)0xffffffffefffffff < ppppppplVar5) {
      ppppppplVar32 = (long *******)0xffffffffffffffff;
    }
    uVar29 = (long)ppppppplVar32 + *puVar20;
    if (CARRY8((ulong)ppppppplVar32,*puVar20)) {
      uVar29 = 0xffffffffffffffff;
    }
    uVar19 = uVar29 + puVar20[1];
    if (CARRY8(uVar29,puVar20[1])) {
      uVar19 = 0xffffffffffffffff;
    }
    ppppppplVar32 = (long *******)(uVar19 + puVar20[2]);
    if (CARRY8(uVar19,puVar20[2])) {
      ppppppplVar32 = (long *******)0xffffffffffffffff;
    }
LAB_1008d66ec:
    bVar16 = CARRY8((ulong)ppppppplVar32,puVar20[3]);
    ppppppplVar32 = (long *******)((long)ppppppplVar32 + puVar20[3]);
    if (bVar16) {
      ppppppplVar32 = (long *******)0xffffffffffffffff;
    }
  }
  if (ppppppplVar39 != (long *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar20,(long)ppppppplVar39 << 3,8);
  }
  uVar29 = (long)ppppppplVar6 + 0xfffff;
  if ((long *******)0xfffffffffff00000 < ppppppplVar6) {
    uVar29 = 0xffffffffffffffff;
  }
  uVar19 = (long)ppppppplVar32 + 0xfffff;
  if ((long *******)0xfffffffffff00000 < ppppppplVar32) {
    uVar19 = 0xffffffffffffffff;
  }
  local_260 = ppppppplStack_2c0;
  uVar1 = (long)local_2b8 + 0xfffff;
  if ((long *******)0xfffffffffff00000 < local_2b8) {
    uVar1 = 0xffffffffffffffff;
  }
  local_100 = (long *******)(uVar1 >> 0x14);
  local_168 = (long *******)&local_2b0;
  local_158 = (long *******)&ppppppplStack_2a8;
  local_1a8 = (long *******)&local_320;
  ppppppplStack_1a0 = (long *******)usize>::fmt;
  local_198 = (long *******)&local_3d0;
  ppppppplStack_190 = (long *******)usize>::fmt;
  local_188 = (long *******)&local_260;
  ppppppplStack_180 = (long *******)usize>::fmt;
  ppppppplStack_178 = (long *******)&local_100;
  ppppppplStack_170 = (long *******)usize>::fmt;
  ppppppplStack_160 = (long *******)usize>::fmt;
  ppppppplStack_150 = (long *******)usize>::fmt;
  local_148 = &DAT_1013b3038;
  pppppplStack_140 = (long ******)usize>::fmt;
  local_3d0 = (long *******)(uVar19 >> 0x14);
  local_320 = (long *******)(uVar29 >> 0x14);
  alloc::fmt::format::format_inner(auStack_380,s_available_mib__required_mib__wri_101531637,&local_1a8);
  platform::debug_log::app_event
            ("router_transitioncapacity_preflightsession_meta changed while preparing router migrationsession_meta line length changed before replacementROUTER_HISTORY_LAYOUT_PROTECTED: native paginated record size would change; history preservedsession_meta replacements overlaprollout changed before replacement; history preservedrollout line length changedrollout changed before private reasoning cleanup; retry requiredrollout changed before cleanup publication; history preserved"
             ,0x11,
             "capacity_preflightsession_meta changed while preparing router migrationsession_meta line length changed before replacementROUTER_HISTORY_LAYOUT_PROTECTED: native paginated record size would change; history preservedsession_meta replacements overlaprollout changed before replacement; history preservedrollout line length changedrollout changed before private reasoning cleanup; retry requiredrollout changed before cleanup publication; history preserved"
             ,0x12,auStack_380);
  if (ppppppplVar6 < ppppppplVar32) {
    local_1a8 = (long *******)&local_260;
    ppppppplStack_1a0 = (long *******)usize>::fmt;
    local_198 = (long *******)&local_100;
    ppppppplStack_190 = (long *******)usize>::fmt;
    local_260 = (long *******)(uVar19 >> 0x14);
    local_100 = (long *******)(uVar29 >> 0x14);
    alloc::fmt::format::format_inner(&local_368,s_____MB___MB_ChatGPT_1015316c1,&local_1a8);
    param_1[2] = uStack_360;
    param_1[1] = local_368;
    param_1[3] = local_358;
    uVar34 = 10;
  }
  else {
    uVar34 = 0xb;
  }
  *param_1 = uVar34;
LAB_1008d684c:
  if (local_2d0 != (long *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_2c8,(long)local_2d0 << 3,8);
  }
  return;
LAB_1008d617c:
  if (pppppplVar37 != (long ******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar2,pppppplVar37,1);
  }
  ppppppplVar39 = ppppppplVar39 + 3;
  ppppppplVar22 = ppppppplVar32;
  if (ppppppplVar32 == ppppppplVar38) goto LAB_1008d61dc;
  goto LAB_1008d589c;
}
