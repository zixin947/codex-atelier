/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* fix_all @ 1003a69d8 requested 1003a69d8 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::codex_diagnostic::fix_all */

void codexmate_lib::core::relay::codex_diagnostic::fix_all
               (undefined8 *param_1,long ******param_2,long param_3,undefined8 param_4,undefined8 param_5,byte param_6,undefined8 param_7,undefined8 param_8)

{
  long ******pppppplVar1;
  long ******pppppplVar2;
  byte bVar3;
  uint uVar4;
  int iVar5;
  undefined1 (*pauVar6) [16];
  ulong uVar7;
  undefined8 *puVar8;
  long *******ppppppplVar9;
  long *******ppppppplVar10;
  undefined8 *puVar11;
  ulong uVar12;
  long ******pppppplVar13;
  long lVar14;
  ulong uVar15;
  long ******pppppplVar16;
  long lVar17;
  long *****ppppplVar18;
  long *******ppppppplVar19;
  long *******ppppppplVar20;
  size_t sVar21;
  long *****ppppplVar22;
  size_t *psVar23;
  undefined8 uVar24;
  undefined *puVar25;
  byte bVar26;
  byte bVar27;
  undefined8 uVar28;
  byte bVar29;
  byte bVar30;
  byte bVar31;
  byte bVar32;
  byte bVar33;
  long *****local_388;
  undefined8 uStack_348;
  long local_340;
  long local_338;
  long lStack_330;
  long *******local_320;
  long *******ppppppplStack_318;
  long *******ppppppplStack_310;
  long *******local_308;
  undefined1 local_300 [16];
  long *******local_2f0;
  long *******ppppppplStack_2e8;
  long *******local_2e0;
  long *******local_2d8;
  undefined1 *puStack_2d0;
  long *******local_2c8;
  long *******local_2c0;
  long *******ppppppplStack_2b8;
  long *******local_2b0;
  long local_2a8;
  long ******local_2a0;
  long local_298;
  long ******local_290;
  long ******pppppplStack_288;
  long ******local_280;
  long ******local_278;
  long ******pppppplStack_270;
  long ******local_268;
  long ******local_260;
  long ******pppppplStack_258;
  long ******local_250;
  long ******local_248;
  long ******pppppplStack_240;
  long ******local_238;
  long ******local_230;
  long ******pppppplStack_228;
  long ******local_220;
  long ******local_218;
  long ******pppppplStack_210;
  long ******local_208;
  long ******local_200;
  long ******pppppplStack_1f8;
  long ******local_1f0;
  long ******local_1e8;
  long ******pppppplStack_1e0;
  long ******local_1d8;
  long ******local_1d0;
  long ******local_1c8;
  long ******pppppplStack_1c0;
  long ******local_1b8;
  long ******local_1b0;
  long ******pppppplStack_1a8;
  long ******local_1a0;
  long ******local_198;
  long *******local_190;
  long *******ppppppplStack_188;
  long *******ppppppplStack_180;
  long *******local_178;
  long *******ppppppplStack_170;
  long *******local_168;
  long *******ppppppplStack_160;
  long *******local_158;
  long *******ppppppplStack_150;
  long *******local_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  long *******local_130;
  long *******ppppppplStack_128;
  long *******local_120;
  long *******ppppppplStack_118;
  undefined1 local_110 [16];
  long *******local_100;
  long *******ppppppplStack_f8;
  long *******local_f0;
  long *******ppppppplStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  long ******local_c8;
  long ******pppppplStack_c0;
  long ******local_b8;
  long *******local_b0;
  long *******ppppppplStack_a8;
  long *******local_a0;
  long *******local_98;
  byte local_89;
  undefined8 local_88 [3];

  uStack_348 = param_7;
  run_diagnostics_with_mode(&local_340,param_2,param_3,0);
  lVar14 = local_338;
  pauVar6 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
  if (pauVar6[1][0] == '\x01') {
    local_110 = *pauVar6;
  }
  else {
    local_110 = std::sys::random::hashmap_random_keys();
    *pauVar6 = local_110;
    pauVar6[1][0] = 1;
  }
  *(long *)*pauVar6 = local_110._0_8_ + 1;
  ppppppplStack_128 = (long *******)0x0;
  local_130 = (long *******)&_anon_164d03a2dfc98f969363eaf1c50504fd_13;
  ppppppplStack_118 = (long *******)0x0;
  local_120 = (long *******)0x0;
  ppppppplStack_318 = ppppppplStack_128;
  ppppppplVar10 = local_130;
  ppppppplVar9 = ppppppplStack_118;
  ppppppplStack_310 = local_120;
  for (lVar17 = lStack_330; local_130 = ppppppplVar10, ppppppplStack_128 = ppppppplStack_318, local_120 = ppppppplStack_310, ppppppplStack_118 = ppppppplVar9,
      lVar17 != 0; lVar17 = lVar17 + -1) {
    if ((*(char *)(lVar14 + 0x60) == '\x01') && ((*(long *)(lVar14 + 0x28) != 2 || (**(short **)(lVar14 + 0x20) != 0x6b6f)))) {
      <alloc::string::String_as_core::clone::Clone>::clone(&local_190,lVar14);
      hashbrown::map::HashMap<K,V,S,A>::insert(&local_130,&local_190);
    }
    lVar14 = lVar14 + 0x68;
    ppppppplStack_318 = ppppppplStack_128;
    ppppppplVar10 = local_130;
    ppppppplVar9 = ppppppplStack_118;
    ppppppplStack_310 = local_120;
  }
  local_320 = ppppppplVar10;
  local_308 = ppppppplVar9;
  local_300 = local_110;
  if (ppppppplVar9 == (long *******)0x0) {
    local_b0 = (long *******)0x0;
    ppppppplStack_a8 = (long *******)&SUB_00000008;
    local_a0 = (long *******)0x0;
    local_190 = (long *******)0x0;
    ppppppplStack_188 = (long *******)&SUB_00000008;
    ppppppplStack_180 = (long *******)0x0;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x21,1);
    if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x21);
    }
    *(undefined1 *)(puVar8 + 4) = 0x8d;
    puVar8[1] = 0xe5a3ade6ae9be7b9;
    *puVar8 = 0xa1e9899ce68089e6;
    puVar8[3] = 0xa4e5aebfe4809ce9;
    puVar8[2] = 0xa097e68cbcefb8b8;
    *puVar11 = 0x21;
    puVar11[1] = puVar8;
    puVar11[2] = 0x21;
    local_110._8_8_ = ppppppplStack_180;
    local_110._0_8_ = ppppppplStack_188;
    param_1[2] = ppppppplStack_a8;
    param_1[1] = local_b0;
    param_1[4] = local_190;
    param_1[3] = local_a0;
    param_1[6] = ppppppplStack_180;
    param_1[5] = ppppppplStack_188;
    param_1[7] = 1;
    param_1[8] = puVar11;
    param_1[9] = 1;
    *param_1 = 0xb;
    ppppppplStack_128 = ppppppplStack_a8;
    local_130 = local_b0;
    ppppppplStack_118 = local_190;
    local_120 = local_a0;
    ppppppplVar10 = ppppppplStack_318;
  }
  else {
    if ((param_6 & 1) != 0) {
LAB_1003a6b00:
      local_2f0 = (long *******)0x0;
      ppppppplStack_2e8 = (long *******)&SUB_00000008;
      local_2e0 = (long *******)0x0;
      local_2d8 = (long *******)0x0;
      puStack_2d0 = &SUB_00000008;
      local_2c8 = (long *******)0x0;
      local_2c0 = (long *******)0x0;
      ppppppplStack_2b8 = (long *******)&SUB_00000008;
      local_2b0 = (long *******)0x0;
      local_2a8 = 0;
      local_2a0 = (long ******)0x8;
      local_298 = 0;
      uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_418,0xe);
      lVar14 = 0;
      bVar3 = (byte)(uVar7 >> 0x39);
      uVar7 = uVar7 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
               0x8080808080808080;
joined_r0x0001003a6bcc:
      if (uVar12 == 0) {
        bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                     CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                              CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                       CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                                CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                         CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                  CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
        if ((bVar26 & 1) == 0) goto code_r0x0001003a6be0;
      }
      else {
        uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
        uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
        if ((ppppppplVar10[uVar15 * -3 + -1] != (long ******)0xe) ||
           (*ppppppplVar10[uVar15 * -3 + -2] != (long *****)0x746e695f68747561 || *(long *)((long)ppppppplVar10[uVar15 * -3 + -2] + 6) != 0x797469726765746e)) {
          uVar12 = uVar12 - 1 & uVar12;
          goto joined_r0x0001003a6bcc;
        }
        audit::RepairAudit::fix(&local_190,param_8,&_anon_164d03a2dfc98f969363eaf1c50504fd_418,0xe,param_2,param_3);
        ppppppplVar20 = local_178;
        ppppppplVar9 = ppppppplStack_180;
        ppppppplVar10 = ppppppplStack_188;
        if (local_190 == (long *******)&DAT_0000000b) {
          ppppppplStack_128 = ppppppplStack_180;
          local_130 = ppppppplStack_188;
          ppppppplStack_118 = ppppppplStack_170;
          local_120 = local_178;
          local_110._8_8_ = ppppppplStack_160;
          local_110._0_8_ = local_168;
          ppppppplStack_f8 = ppppppplStack_150;
          local_100 = local_158;
          local_f0 = local_148;
          sVar21 = (long)local_178 * 0x18;
          local_98 = ppppppplStack_180 + (long)local_178 * 3;
          local_b0 = ppppppplStack_180;
          ppppppplStack_a8 = ppppppplStack_180;
          local_a0 = ppppppplStack_188;
          if ((long *******)((long)local_2f0 - (long)local_2e0) < local_178) {
            alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2f0,local_2e0,local_178,8,0x18);
          }
          ppppppplVar19 = local_2e0;
          _memcpy(ppppppplStack_2e8 + (long)local_2e0 * 3,ppppppplVar9,sVar21);
          local_2e0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
          if (ppppppplVar10 != (long *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
          }
          ppppppplVar20 = local_f0;
          ppppppplVar9 = ppppppplStack_f8;
          ppppppplVar10 = local_100;
          sVar21 = (long)local_f0 * 0x18;
          local_98 = ppppppplStack_f8 + (long)local_f0 * 3;
          local_b0 = ppppppplStack_f8;
          ppppppplStack_a8 = ppppppplStack_f8;
          local_a0 = local_100;
          if ((long *******)((long)local_2c0 - (long)local_2b0) < local_f0) {
            alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2c0,local_2b0,local_f0,8,0x18);
          }
          ppppppplVar19 = local_2b0;
          _memcpy(ppppppplStack_2b8 + (long)local_2b0 * 3,ppppppplVar9,sVar21);
          local_2b0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
          if (ppppppplVar10 != (long *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
          }
          uVar28 = local_110._0_8_;
          if ((long *******)local_110._8_8_ != (long *******)0x0) {
            ppppppplVar10 = (long *******)(local_110._0_8_ + 8);
            ppppppplVar9 = (long *******)local_110._8_8_;
            do {
              if (ppppppplVar10[-1] != (long ******)0x0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppppppplVar10,ppppppplVar10[-1],1);
              }
              ppppppplVar10 = ppppppplVar10 + 3;
              ppppppplVar9 = (long *******)((long)ppppppplVar9 + -1);
            } while (ppppppplVar9 != (long *******)0x0);
          }
          if (ppppppplStack_118 != (long *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar28,(long)ppppppplStack_118 * 0x18,8);
          }
        }
        else {
          local_110._8_8_ = local_168;
          local_110._0_8_ = ppppppplStack_170;
          ppppppplStack_f8 = local_158;
          local_100 = ppppppplStack_160;
          ppppppplStack_e8 = local_148;
          local_f0 = ppppppplStack_150;
          uStack_d8 = uStack_138;
          uStack_e0 = uStack_140;
          ppppppplStack_128 = ppppppplStack_188;
          local_130 = local_190;
          ppppppplStack_118 = local_178;
          local_120 = ppppppplStack_180;
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
          ppppppplVar10 = local_2c8;
          if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(1,0xe);
          }
          *puVar11 = 0x746e695f68747561;
          *(undefined8 *)((long)puVar11 + 6) = 0x797469726765746e;
          if (local_2c8 == local_2d8) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
          }
          puVar8 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
          *puVar8 = 0xe;
          puVar8[1] = puVar11;
          puVar8[2] = 0xe;
          local_2c8 = (long *******)((long)ppppppplVar10 + 1);
          local_b0 = (long *******)&local_130;
          ppppppplStack_a8 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_290,s_auth_integrity__101522b79,&local_b0);
          ppppppplVar10 = local_2b0;
          if (local_2b0 == local_2c0) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
          }
          ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
          ppppppplVar9[1] = pppppplStack_288;
          *ppppppplVar9 = local_290;
          ppppppplVar9[2] = local_280;
          local_2b0 = (long *******)((long)ppppppplVar10 + 1);
          ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
        }
        if (local_308 == (long *******)0x0) goto LAB_1003a9744;
      }
      uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_632,0x1a);
      lVar14 = 0;
      bVar3 = (byte)(uVar7 >> 0x39);
      uVar7 = uVar7 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
               0x8080808080808080;
      while( true ) {
        while (uVar12 == 0) {
          bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                       CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                                CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                         CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                                  CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                           CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                    CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1)
          ;
          if ((bVar26 & 1) != 0) goto LAB_1003a7474;
          lVar14 = lVar14 + 8;
          uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
          uVar28 = *(undefined8 *)((long)local_320 + uVar7);
          uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                            CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                     CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                              CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                       CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                                CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                         CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
                   0x8080808080808080;
        }
        uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
        uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
        if ((local_320[uVar15 * -3 + -1] == (long ******)0x1a) &&
           (pppppplVar13 = local_320[uVar15 * -3 + -2],
           ((*pppppplVar13 == (long *****)0x755f726574756f72 && pppppplVar13[1] == (long *****)0x75615f6b636f6c6e) &&
           pppppplVar13[2] == (long *****)0x64697365725f6874) && *(short *)(pppppplVar13 + 3) == 0x6575)) break;
        uVar12 = uVar12 - 1 & uVar12;
      }
      audit::RepairAudit::fix(&local_190,param_8,&_anon_164d03a2dfc98f969363eaf1c50504fd_632,0x1a,param_2,param_3);
      ppppppplVar20 = local_178;
      ppppppplVar9 = ppppppplStack_180;
      ppppppplVar10 = ppppppplStack_188;
      if (local_190 == (long *******)&DAT_0000000b) {
        ppppppplStack_128 = ppppppplStack_180;
        local_130 = ppppppplStack_188;
        ppppppplStack_118 = ppppppplStack_170;
        local_120 = local_178;
        local_110._8_8_ = ppppppplStack_160;
        local_110._0_8_ = local_168;
        ppppppplStack_f8 = ppppppplStack_150;
        local_100 = local_158;
        local_f0 = local_148;
        sVar21 = (long)local_178 * 0x18;
        local_98 = ppppppplStack_180 + (long)local_178 * 3;
        local_b0 = ppppppplStack_180;
        ppppppplStack_a8 = ppppppplStack_180;
        local_a0 = ppppppplStack_188;
        if ((long *******)((long)local_2f0 - (long)local_2e0) < local_178) {
          alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2f0,local_2e0,local_178,8,0x18);
        }
        ppppppplVar19 = local_2e0;
        _memcpy(ppppppplStack_2e8 + (long)local_2e0 * 3,ppppppplVar9,sVar21);
        local_2e0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
        if (ppppppplVar10 != (long *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
        }
        uVar24 = local_110._8_8_;
        uVar28 = local_110._0_8_;
        ppppppplVar10 = ppppppplStack_118;
        sVar21 = local_110._8_8_ * 0x18;
        local_98 = (long *******)(local_110._0_8_ + local_110._8_8_ * 3 * 8);
        local_b0 = (long *******)local_110._0_8_;
        ppppppplStack_a8 = (long *******)local_110._0_8_;
        local_a0 = ppppppplStack_118;
        if ((long *******)((long)local_2d8 - (long)local_2c8) < (ulong)local_110._8_8_) {
          alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2d8,local_2c8,local_110._8_8_,8,0x18);
        }
        ppppppplVar9 = local_2c8;
        _memcpy(puStack_2d0 + (long)local_2c8 * 0x18,(void *)uVar28,sVar21);
        local_2c8 = (long *******)((long)ppppppplVar9 + uVar24);
        if (ppppppplVar10 != (long *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar28,(long)ppppppplVar10 * 0x18,8);
        }
        ppppppplVar20 = local_f0;
        ppppppplVar9 = ppppppplStack_f8;
        ppppppplVar10 = local_100;
        sVar21 = (long)local_f0 * 0x18;
        local_98 = ppppppplStack_f8 + (long)local_f0 * 3;
        local_b0 = ppppppplStack_f8;
        ppppppplStack_a8 = ppppppplStack_f8;
        local_a0 = local_100;
        if ((long *******)((long)local_2c0 - (long)local_2b0) < local_f0) {
          alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2c0,local_2b0,local_f0,8,0x18);
        }
        ppppppplVar19 = local_2b0;
        _memcpy(ppppppplStack_2b8 + (long)local_2b0 * 3,ppppppplVar9,sVar21);
        local_2b0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
        if (ppppppplVar10 != (long *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
        }
      }
      else {
        local_110._8_8_ = local_168;
        local_110._0_8_ = ppppppplStack_170;
        ppppppplStack_f8 = local_158;
        local_100 = ppppppplStack_160;
        ppppppplStack_e8 = local_148;
        local_f0 = ppppppplStack_150;
        uStack_d8 = uStack_138;
        uStack_e0 = uStack_140;
        ppppppplStack_128 = ppppppplStack_188;
        local_130 = local_190;
        ppppppplStack_118 = local_178;
        local_120 = ppppppplStack_180;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1a,1);
        ppppppplVar10 = local_2c8;
        if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x1a);
        }
        puVar11[1] = 0x75615f6b636f6c6e;
        *puVar11 = 0x755f726574756f72;
        *(undefined8 *)((long)puVar11 + 0x12) = 0x657564697365725f;
        *(undefined8 *)((long)puVar11 + 10) = 0x687475615f6b636f;
        if (local_2c8 == local_2d8) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
        }
        puVar8 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
        *puVar8 = 0x1a;
        puVar8[1] = puVar11;
        puVar8[2] = 0x1a;
        local_2c8 = (long *******)((long)ppppppplVar10 + 1);
        local_b0 = (long *******)&local_130;
        ppppppplStack_a8 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_278,s_router_unlock_auth_residue__101522b8c,&local_b0);
        ppppppplVar10 = local_2b0;
        if (local_2b0 == local_2c0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
        }
        ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
        ppppppplVar9[1] = pppppplStack_270;
        *ppppppplVar9 = local_278;
        ppppppplVar9[2] = local_268;
        local_2b0 = (long *******)((long)ppppppplVar10 + 1);
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
      }
      if (local_308 == (long *******)0x0) goto LAB_1003a9744;
LAB_1003a7474:
      uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_631,0x16);
      lVar14 = 0;
      bVar3 = (byte)(uVar7 >> 0x39);
      uVar7 = uVar7 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
               0x8080808080808080;
      while( true ) {
        while (uVar12 == 0) {
          bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                       CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                                CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                         CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                                  CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                           CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                    CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1)
          ;
          if ((bVar26 & 1) != 0) goto LAB_1003a77f4;
          lVar14 = lVar14 + 8;
          uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
          uVar28 = *(undefined8 *)((long)local_320 + uVar7);
          uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                            CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                     CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                              CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                       CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                                CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                         CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
                   0x8080808080808080;
        }
        uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
        uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
        if ((local_320[uVar15 * -3 + -1] == (long ******)0x16) &&
           (pppppplVar13 = local_320[uVar15 * -3 + -2],
           (*pppppplVar13 == (long *****)0x7265766f656b6174 && pppppplVar13[1] == (long *****)0x5f70756b6361625f) &&
           *(long *)((long)pppppplVar13 + 0xe) == 0x6e616870726f5f70)) break;
        uVar12 = uVar12 - 1 & uVar12;
      }
      audit::RepairAudit::fix(&local_190,param_8,&_anon_164d03a2dfc98f969363eaf1c50504fd_631,0x16,param_2,param_3);
      ppppppplVar20 = local_178;
      ppppppplVar9 = ppppppplStack_180;
      ppppppplVar10 = ppppppplStack_188;
      if (local_190 == (long *******)&DAT_0000000b) {
        ppppppplStack_128 = ppppppplStack_180;
        local_130 = ppppppplStack_188;
        ppppppplStack_118 = ppppppplStack_170;
        local_120 = local_178;
        local_110._8_8_ = ppppppplStack_160;
        local_110._0_8_ = local_168;
        ppppppplStack_f8 = ppppppplStack_150;
        local_100 = local_158;
        local_f0 = local_148;
        sVar21 = (long)local_178 * 0x18;
        local_98 = ppppppplStack_180 + (long)local_178 * 3;
        local_b0 = ppppppplStack_180;
        ppppppplStack_a8 = ppppppplStack_180;
        local_a0 = ppppppplStack_188;
        if ((long *******)((long)local_2f0 - (long)local_2e0) < local_178) {
          alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2f0,local_2e0,local_178,8,0x18);
        }
        ppppppplVar19 = local_2e0;
        _memcpy(ppppppplStack_2e8 + (long)local_2e0 * 3,ppppppplVar9,sVar21);
        local_2e0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
        if (ppppppplVar10 != (long *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
        }
        uVar24 = local_110._8_8_;
        uVar28 = local_110._0_8_;
        ppppppplVar10 = ppppppplStack_118;
        sVar21 = local_110._8_8_ * 0x18;
        local_98 = (long *******)(local_110._0_8_ + local_110._8_8_ * 3 * 8);
        local_b0 = (long *******)local_110._0_8_;
        ppppppplStack_a8 = (long *******)local_110._0_8_;
        local_a0 = ppppppplStack_118;
        if ((long *******)((long)local_2d8 - (long)local_2c8) < (ulong)local_110._8_8_) {
          alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2d8,local_2c8,local_110._8_8_,8,0x18);
        }
        ppppppplVar9 = local_2c8;
        _memcpy(puStack_2d0 + (long)local_2c8 * 0x18,(void *)uVar28,sVar21);
        local_2c8 = (long *******)((long)ppppppplVar9 + uVar24);
        if (ppppppplVar10 != (long *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar28,(long)ppppppplVar10 * 0x18,8);
        }
        ppppppplVar20 = local_f0;
        ppppppplVar9 = ppppppplStack_f8;
        ppppppplVar10 = local_100;
        sVar21 = (long)local_f0 * 0x18;
        local_98 = ppppppplStack_f8 + (long)local_f0 * 3;
        local_b0 = ppppppplStack_f8;
        ppppppplStack_a8 = ppppppplStack_f8;
        local_a0 = local_100;
        if ((long *******)((long)local_2c0 - (long)local_2b0) < local_f0) {
          alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2c0,local_2b0,local_f0,8,0x18);
        }
        ppppppplVar19 = local_2b0;
        _memcpy(ppppppplStack_2b8 + (long)local_2b0 * 3,ppppppplVar9,sVar21);
        local_2b0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
        if (ppppppplVar10 != (long *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
        }
      }
      else {
        local_110._8_8_ = local_168;
        local_110._0_8_ = ppppppplStack_170;
        ppppppplStack_f8 = local_158;
        local_100 = ppppppplStack_160;
        ppppppplStack_e8 = local_148;
        local_f0 = ppppppplStack_150;
        uStack_d8 = uStack_138;
        uStack_e0 = uStack_140;
        ppppppplStack_128 = ppppppplStack_188;
        local_130 = local_190;
        ppppppplStack_118 = local_178;
        local_120 = ppppppplStack_180;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x16,1);
        ppppppplVar10 = local_2c8;
        if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x16);
        }
        puVar11[1] = 0x5f70756b6361625f;
        *puVar11 = 0x7265766f656b6174;
        *(undefined8 *)((long)puVar11 + 0xe) = 0x6e616870726f5f70;
        if (local_2c8 == local_2d8) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
        }
        puVar8 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
        *puVar8 = 0x16;
        puVar8[1] = puVar11;
        puVar8[2] = 0x16;
        local_2c8 = (long *******)((long)ppppppplVar10 + 1);
        local_b0 = (long *******)&local_130;
        ppppppplStack_a8 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_260,s_takeover_backup_orphan__101522bab,&local_b0);
        ppppppplVar10 = local_2b0;
        if (local_2b0 == local_2c0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
        }
        ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
        ppppppplVar9[1] = pppppplStack_258;
        *ppppppplVar9 = local_260;
        ppppppplVar9[2] = local_250;
        local_2b0 = (long *******)((long)ppppppplVar10 + 1);
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
      }
      if (local_308 == (long *******)0x0) goto LAB_1003a9744;
LAB_1003a77f4:
      uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_499,0x12);
      lVar14 = 0;
      bVar3 = (byte)(uVar7 >> 0x39);
      uVar7 = uVar7 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
               0x8080808080808080;
joined_r0x0001003a7860:
      if (uVar12 == 0) {
        bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                     CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                              CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                       CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                                CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                         CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                  CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
        if ((bVar26 & 1) != 0) goto LAB_1003a7a24;
        lVar14 = lVar14 + 8;
        uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
        uVar28 = *(undefined8 *)((long)local_320 + uVar7);
        uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                          CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                   CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                            CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                     CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                              CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                       CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
                 0x8080808080808080;
        goto joined_r0x0001003a7860;
      }
      uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
      uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
      uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
      if ((local_320[uVar15 * -3 + -1] != (long ******)0x12) ||
         (pppppplVar13 = local_320[uVar15 * -3 + -2],
         (*pppppplVar13 != (long *****)0x745f6769666e6f63 || pppppplVar13[1] != (long *****)0x746e79735f6c6d6f) || *(short *)(pppppplVar13 + 2) != 0x7861)) {
        uVar12 = uVar12 - 1 & uVar12;
        goto joined_r0x0001003a7860;
      }
      audit::RepairAudit::step(&local_190,param_8,&_anon_164d03a2dfc98f969363eaf1c50504fd_499,0x12,&DAT_101310885,0xe,param_2);
      ppppppplVar10 = local_190;
      bVar3 = local_190 == (long *******)&DAT_0000000b;
      if ((bool)bVar3) {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        ppppppplVar9 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x12,1);
        if (ppppppplVar9 == (long *******)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x12);
        }
        *(undefined2 *)(ppppppplVar9 + 2) = 0x7861;
        ppppppplVar9[1] = (long ******)0x746e79735f6c6d6f;
        *ppppppplVar9 = (long ******)0x745f6769666e6f63;
        local_b0 = (long *******)&DAT_00000012;
        local_a0 = (long *******)&DAT_00000012;
        ppppppplStack_a8 = ppppppplVar9;
        <alloc::string::String_as_core::clone::Clone>::clone(&local_130,&local_b0);
        ppppppplVar20 = local_2e0;
        if (local_2e0 == local_2f0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2f0);
        }
        lVar14 = local_298;
        ppppppplVar19 = ppppppplStack_2e8 + (long)ppppppplVar20 * 3;
        ppppppplVar19[1] = (long ******)ppppppplStack_128;
        *ppppppplVar19 = (long ******)local_130;
        ppppppplVar19[2] = (long ******)local_120;
        local_2e0 = (long *******)((long)ppppppplVar20 + 1);
        if (local_298 == local_2a8) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2a8);
        }
        ppppppplVar20 = local_2b0;
        pppppplVar13 = local_2a0 + lVar14 * 3;
        *pppppplVar13 = (long *****)0x12;
        pppppplVar13[1] = (long *****)ppppppplVar9;
        pppppplVar13[2] = (long *****)0x12;
        local_298 = lVar14 + 1;
        if (local_2b0 == local_2c0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
        }
        ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar20 * 3;
        ppppppplVar9[1] = (long ******)ppppppplStack_180;
        *ppppppplVar9 = (long ******)ppppppplStack_188;
        ppppppplVar9[2] = (long ******)local_178;
        local_2b0 = (long *******)((long)ppppppplVar20 + 1);
      }
      else {
        local_110._8_8_ = local_168;
        local_110._0_8_ = ppppppplStack_170;
        ppppppplStack_f8 = local_158;
        local_100 = ppppppplStack_160;
        ppppppplStack_e8 = local_148;
        local_f0 = ppppppplStack_150;
        uStack_d8 = uStack_138;
        uStack_e0 = uStack_140;
        ppppppplStack_128 = ppppppplStack_188;
        local_130 = local_190;
        ppppppplStack_118 = local_178;
        local_120 = ppppppplStack_180;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x12,1);
        ppppppplVar9 = local_2c8;
        if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x12);
        }
        *(undefined2 *)(puVar11 + 2) = 0x7861;
        puVar11[1] = 0x746e79735f6c6d6f;
        *puVar11 = 0x745f6769666e6f63;
        if (local_2c8 == local_2d8) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
        }
        puVar8 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar9 * 0x18);
        *puVar8 = 0x12;
        puVar8[1] = puVar11;
        puVar8[2] = 0x12;
        local_2c8 = (long *******)((long)ppppppplVar9 + 1);
        local_b0 = (long *******)&local_130;
        ppppppplStack_a8 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_248,s_config_toml_syntax__101522bc6,&local_b0);
        ppppppplVar9 = local_2b0;
        if (local_2b0 == local_2c0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
        }
        ppppppplVar20 = ppppppplStack_2b8 + (long)ppppppplVar9 * 3;
        ppppppplVar20[1] = pppppplStack_240;
        *ppppppplVar20 = local_248;
        ppppppplVar20[2] = local_238;
        local_2b0 = (long *******)((long)ppppppplVar9 + 1);
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
      }
      if (local_308 != (long *******)0x0) goto LAB_1003a7b1c;
      if (ppppppplVar10 != (long *******)&DAT_0000000b) goto LAB_1003a9744;
LAB_1003a8c28:
      audit::RepairAudit::step(&local_190,param_8,&DAT_1013108e8,3,&DAT_1013108eb,0xd,param_4,param_5);
      if (local_190 != (long *******)&DAT_0000000b) {
        local_110._8_8_ = local_168;
        local_110._0_8_ = ppppppplStack_170;
        ppppppplStack_f8 = local_158;
        local_100 = ppppppplStack_160;
        ppppppplStack_e8 = local_148;
        local_f0 = ppppppplStack_150;
        uStack_d8 = uStack_138;
        uStack_e0 = uStack_140;
        ppppppplStack_128 = ppppppplStack_188;
        local_130 = local_190;
        ppppppplStack_118 = local_178;
        local_120 = ppppppplStack_180;
        local_b0 = (long *******)&local_130;
        ppppppplStack_a8 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_1e8,s_resync_config___101522c21,&local_b0);
        ppppppplVar10 = local_2b0;
        if (local_2b0 == local_2c0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
        }
        ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
        ppppppplVar9[1] = pppppplStack_1e0;
        *ppppppplVar9 = local_1e8;
        ppppppplVar9[2] = local_1d8;
        local_2b0 = (long *******)((long)ppppppplVar10 + 1);
        if (local_298 != 0) {
          pppppplVar16 = local_2a0 + local_298 * 3;
          pppppplVar13 = local_2a0;
          ppppppplVar10 = local_2c8;
          do {
            if (ppppppplVar10 != (long *******)0x0) {
              lVar14 = (long)ppppppplVar10 * 0x18;
              ppppplVar22 = pppppplVar13[1];
              ppppplVar18 = pppppplVar13[2];
              psVar23 = (size_t *)(puStack_2d0 + 0x10);
              do {
                if ((long *****)*psVar23 == ppppplVar18) {
                  iVar5 = _memcmp((void *)psVar23[-1],ppppplVar22,(size_t)ppppplVar18);
                  if (iVar5 == 0) goto LAB_1003a8ed8;
                }
                psVar23 = psVar23 + 3;
                lVar14 = lVar14 + -0x18;
              } while (lVar14 != 0);
            }
            <alloc::string::String_as_core::clone::Clone>::clone(&local_b0,pppppplVar13);
            ppppppplVar10 = local_2c8;
            if (local_2c8 == local_2d8) {
              alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
            }
            puVar11 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
            puVar11[1] = ppppppplStack_a8;
            *puVar11 = local_b0;
            puVar11[2] = local_a0;
            ppppppplVar10 = (long *******)((long)ppppppplVar10 + 1);
            local_2c8 = ppppppplVar10;
LAB_1003a8ed8:
            pppppplVar13 = pppppplVar13 + 3;
          } while (pppppplVar13 != pppppplVar16);
        }
        alloc::vec::Vec<T,A>::retain(&local_2f0,&local_2a8);
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
        goto joined_r0x0001003a8f70;
      }
      run_diagnostics_with_mode(&local_130,param_2,param_3,0);
      if (local_298 == 0) goto LAB_1003a8f98;
      if (local_120 == (long *******)0x0) {
        local_1d0 = local_2a0 + ((local_298 * 0x18 - 0x18U) / 0x18) * 3;
        goto LAB_1003a8f98;
      }
      pppppplVar13 = local_2a0 + local_298 * 3;
      lVar14 = (long)local_120 * 0x68;
      ppppppplVar10 = ppppppplStack_128 + 4;
      pppppplVar16 = local_2a0;
LAB_1003a8cd0:
      pppppplVar1 = pppppplVar16 + 3;
      ppppplVar22 = pppppplVar16[1];
      pppppplVar2 = (long ******)pppppplVar16[2];
      ppppppplVar9 = ppppppplVar10;
      lVar17 = lVar14;
      local_1d0 = pppppplVar16;
      do {
        if (ppppppplVar9[-2] == pppppplVar2) {
          iVar5 = _memcmp(ppppppplVar9[-3],ppppplVar22,(size_t)pppppplVar2);
          if (iVar5 == 0) {
            if ((ppppppplVar9[1] != (long ******)0x2) || (*(short *)*ppppppplVar9 != 0x6b6f)) {
              if (local_2c8 == (long *******)0x0) goto LAB_1003a8d84;
              lVar17 = (long)local_2c8 * 0x18;
              psVar23 = (size_t *)(puStack_2d0 + 0x10);
              goto LAB_1003a8d60;
            }
            break;
          }
        }
        ppppppplVar9 = ppppppplVar9 + 0xd;
        lVar17 = lVar17 + -0x68;
      } while (lVar17 != 0);
      goto LAB_1003a8cc8;
    }
    ppppppplVar20 = ppppppplVar10 + 1;
    pppppplVar13 = *ppppppplVar10;
    uVar12 = CONCAT17(-(-1 < (long)pppppplVar13),
                      CONCAT16(-(-1 < (char)((ulong)pppppplVar13 >> 0x30)),
                               CONCAT15(-(-1 < (char)((ulong)pppppplVar13 >> 0x28)),
                                        CONCAT14(-(-1 < (char)((ulong)pppppplVar13 >> 0x20)),
                                                 CONCAT13(-(-1 < (char)((ulong)pppppplVar13 >> 0x18)),
                                                          CONCAT12(-(-1 < (char)((ulong)pppppplVar13 >> 0x10)),
                                                                   CONCAT11(-(-1 < (char)((ulong)pppppplVar13 >> 8)),-(-1 < (char)pppppplVar13)))))))) &
             0x8080808080808080;
    ppppppplVar19 = ppppppplVar10;
    do {
      if (ppppppplVar9 == (long *******)0x0) goto LAB_1003a6b00;
      while (uVar12 == 0) {
        pppppplVar13 = *ppppppplVar20;
        ppppppplVar19 = ppppppplVar19 + -0x18;
        ppppppplVar20 = ppppppplVar20 + 1;
        uVar12 = CONCAT17(-(-1 < (long)pppppplVar13),
                          CONCAT16(-(-1 < (char)((ulong)pppppplVar13 >> 0x30)),
                                   CONCAT15(-(-1 < (char)((ulong)pppppplVar13 >> 0x28)),
                                            CONCAT14(-(-1 < (char)((ulong)pppppplVar13 >> 0x20)),
                                                     CONCAT13(-(-1 < (char)((ulong)pppppplVar13 >> 0x18)),
                                                              CONCAT12(-(-1 < (char)((ulong)pppppplVar13 >> 0x10)),
                                                                       CONCAT11(-(-1 < (char)((ulong)pppppplVar13 >> 8)),-(-1 < (char)pppppplVar13)))))))) &
                 0x8080808080808080;
      }
      uVar7 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
      uVar7 = (uVar7 & 0xffff0000ffff0000) >> 0x10 | (uVar7 & 0xffff0000ffff) << 0x10;
      uVar12 = uVar12 - 1 & uVar12;
      iVar5 = (int)((ulong)LZCOUNT(uVar7 >> 0x20 | uVar7 << 0x20) >> 3);
      ppppppplVar9 = (long *******)((long)ppppppplVar9 + -1);
      uVar4 = fix_requires_quiet_window(ppppppplVar19[(long)-iVar5 * 3 + -2],ppppppplVar19[(long)-iVar5 * 3 + -1]);
    } while ((uVar4 & 0xfd) != 1);
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x3b,1);
    if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x3b);
    }
    puVar11[1] = 0x612d786966206369;
    *puVar11 = 0x74736f6e67616964;
    puVar11[3] = 0x6576206120736572;
    puVar11[2] = 0x6975716572206c6c;
    puVar11[5] = 0x7120545047746168;
    puVar11[4] = 0x4320646569666972;
    *(undefined8 *)((long)puVar11 + 0x33) = 0x776f646e69772074;
    *(undefined8 *)((long)puVar11 + 0x2b) = 0x6569757120545047;
    param_1[1] = 0x3b;
    *param_1 = 10;
    param_1[2] = puVar11;
    param_1[3] = 0x3b;
    ppppppplVar10 = ppppppplStack_318;
  }
  ppppppplStack_318 = ppppppplVar10;
  if (ppppppplVar10 != (long *******)0x0) {
    if (local_308 != (long *******)0x0) {
      ppppppplVar20 = local_320 + 1;
      pppppplVar13 = *local_320;
      uVar12 = CONCAT17(-(-1 < (long)pppppplVar13),
                        CONCAT16(-(-1 < (char)((ulong)pppppplVar13 >> 0x30)),
                                 CONCAT15(-(-1 < (char)((ulong)pppppplVar13 >> 0x28)),
                                          CONCAT14(-(-1 < (char)((ulong)pppppplVar13 >> 0x20)),
                                                   CONCAT13(-(-1 < (char)((ulong)pppppplVar13 >> 0x18)),
                                                            CONCAT12(-(-1 < (char)((ulong)pppppplVar13 >> 0x10)),
                                                                     CONCAT11(-(-1 < (char)((ulong)pppppplVar13 >> 8)),-(-1 < (char)pppppplVar13)))))))) &
               0x8080808080808080;
      ppppppplVar9 = local_320;
      ppppppplVar19 = local_308;
      do {
        while (uVar12 == 0) {
          pppppplVar13 = *ppppppplVar20;
          ppppppplVar9 = ppppppplVar9 + -0x18;
          ppppppplVar20 = ppppppplVar20 + 1;
          uVar12 = CONCAT17(-(-1 < (long)pppppplVar13),
                            CONCAT16(-(-1 < (char)((ulong)pppppplVar13 >> 0x30)),
                                     CONCAT15(-(-1 < (char)((ulong)pppppplVar13 >> 0x28)),
                                              CONCAT14(-(-1 < (char)((ulong)pppppplVar13 >> 0x20)),
                                                       CONCAT13(-(-1 < (char)((ulong)pppppplVar13 >> 0x18)),
                                                                CONCAT12(-(-1 < (char)((ulong)pppppplVar13 >> 0x10)),
                                                                         CONCAT11(-(-1 < (char)((ulong)pppppplVar13 >> 8)),-(-1 < (char)pppppplVar13)))))))) &
                   0x8080808080808080;
        }
        uVar7 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar7 = (uVar7 & 0xffff0000ffff0000) >> 0x10 | (uVar7 & 0xffff0000ffff) << 0x10;
        iVar5 = (int)((ulong)LZCOUNT(uVar7 >> 0x20 | uVar7 << 0x20) >> 3);
        if (ppppppplVar9[(long)-iVar5 * 3 + -3] != (long ******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9[(long)-iVar5 * 3 + -2],ppppppplVar9[(long)-iVar5 * 3 + -3],1);
        }
        uVar12 = uVar12 - 1 & uVar12;
        ppppppplVar19 = (long *******)((long)ppppppplVar19 + -1);
      } while (ppppppplVar19 != (long *******)0x0);
    }
    lVar14 = (long)ppppppplVar10 * 0x19 + 0x21;
    if (lVar14 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_320 + (long)ppppppplVar10 * -3 + -3,lVar14,8);
    }
  }
  <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_340);
  goto LAB_1003a6f94;
code_r0x0001003a6be0:
  lVar14 = lVar14 + 8;
  uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
  uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
  uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                    CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                             CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                      CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                               CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                        CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                 CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
           0x8080808080808080;
  goto joined_r0x0001003a6bcc;
LAB_1003a7a24:
  bVar3 = false;
LAB_1003a7b1c:
  uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_497,0x12);
  lVar14 = 0;
  bVar26 = (byte)(uVar7 >> 0x39);
  uVar7 = uVar7 & (ulong)ppppppplStack_318;
  uVar28 = *(undefined8 *)((long)local_320 + uVar7);
  uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                    CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                             CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                      CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                               CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                        CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                 CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
           0x8080808080808080;
  do {
    for (; uVar12 != 0; uVar12 = uVar12 - 1 & uVar12) {
      uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
      uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
      uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
      if ((local_320[uVar15 * -3 + -1] == (long ******)0x12) &&
         (pppppplVar13 = local_320[uVar15 * -3 + -2],
         (*pppppplVar13 == (long *****)0x745f6769666e6f63 && pppppplVar13[1] == (long *****)0x7261705f64726968) && *(short *)(pppppplVar13 + 2) == 0x7974)) {
        audit::RepairAudit::step(&local_190,param_8,&_anon_164d03a2dfc98f969363eaf1c50504fd_497,0x12,&DAT_101310885,0xe,param_2,param_3);
        if (local_190 == (long *******)&DAT_0000000b) {
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          ppppppplVar10 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x12,1);
          if (ppppppplVar10 == (long *******)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(1,0x12);
          }
          *(undefined2 *)(ppppppplVar10 + 2) = 0x7974;
          ppppppplVar10[1] = (long ******)0x7261705f64726968;
          *ppppppplVar10 = (long ******)0x745f6769666e6f63;
          local_b0 = (long *******)&DAT_00000012;
          local_a0 = (long *******)&DAT_00000012;
          ppppppplStack_a8 = ppppppplVar10;
          <alloc::string::String_as_core::clone::Clone>::clone(&local_130,&local_b0);
          ppppppplVar9 = local_2e0;
          if (local_2e0 == local_2f0) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_2f0);
          }
          lVar14 = local_298;
          ppppppplVar20 = ppppppplStack_2e8 + (long)ppppppplVar9 * 3;
          ppppppplVar20[1] = (long ******)ppppppplStack_128;
          *ppppppplVar20 = (long ******)local_130;
          ppppppplVar20[2] = (long ******)local_120;
          local_2e0 = (long *******)((long)ppppppplVar9 + 1);
          if (local_298 == local_2a8) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_2a8);
          }
          ppppppplVar9 = local_2b0;
          pppppplVar13 = local_2a0 + lVar14 * 3;
          *pppppplVar13 = (long *****)0x12;
          pppppplVar13[1] = (long *****)ppppppplVar10;
          pppppplVar13[2] = (long *****)0x12;
          local_298 = lVar14 + 1;
          if (local_2b0 == local_2c0) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
          }
          ppppppplVar10 = ppppppplStack_2b8 + (long)ppppppplVar9 * 3;
          ppppppplVar10[1] = (long ******)ppppppplStack_180;
          *ppppppplVar10 = (long ******)ppppppplStack_188;
          ppppppplVar10[2] = (long ******)local_178;
          local_2b0 = (long *******)((long)ppppppplVar9 + 1);
          bVar3 = true;
        }
        else {
          local_110._8_8_ = local_168;
          local_110._0_8_ = ppppppplStack_170;
          ppppppplStack_f8 = local_158;
          local_100 = ppppppplStack_160;
          ppppppplStack_e8 = local_148;
          local_f0 = ppppppplStack_150;
          uStack_d8 = uStack_138;
          uStack_e0 = uStack_140;
          ppppppplStack_128 = ppppppplStack_188;
          local_130 = local_190;
          ppppppplStack_118 = local_178;
          local_120 = ppppppplStack_180;
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x12,1);
          ppppppplVar10 = local_2c8;
          if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(1,0x12);
          }
          *(undefined2 *)(puVar11 + 2) = 0x7974;
          puVar11[1] = 0x7261705f64726968;
          *puVar11 = 0x745f6769666e6f63;
          if (local_2c8 == local_2d8) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
          }
          puVar8 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
          *puVar8 = 0x12;
          puVar8[1] = puVar11;
          puVar8[2] = 0x12;
          local_2c8 = (long *******)((long)ppppppplVar10 + 1);
          local_b0 = (long *******)&local_130;
          ppppppplStack_a8 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_230,s_config_third_party__101522bdd,&local_b0);
          ppppppplVar10 = local_2b0;
          if (local_2b0 == local_2c0) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
          }
          ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
          ppppppplVar9[1] = pppppplStack_228;
          *ppppppplVar9 = local_230;
          ppppppplVar9[2] = local_220;
          local_2b0 = (long *******)((long)ppppppplVar10 + 1);
          ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
        }
        if (local_308 != (long *******)0x0) goto LAB_1003a7e30;
LAB_1003a8584:
        if ((bVar3 & 1) == 0) goto LAB_1003a9744;
        goto LAB_1003a8c28;
      }
    }
    bVar27 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                 CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                          CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                   CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                            CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                     CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                              CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
    if ((bVar27 & 1) != 0) break;
    lVar14 = lVar14 + 8;
    uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
    uVar28 = *(undefined8 *)((long)local_320 + uVar7);
    uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                      CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                               CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                        CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                                 CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                          CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                   CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
             0x8080808080808080;
  } while( true );
LAB_1003a7e30:
  uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_630,0x17);
  lVar14 = 0;
  bVar26 = (byte)(uVar7 >> 0x39);
  uVar7 = uVar7 & (ulong)ppppppplStack_318;
  uVar28 = *(undefined8 *)((long)local_320 + uVar7);
  uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                    CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                             CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                      CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                               CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                        CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                 CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
           0x8080808080808080;
  while( true ) {
    while (uVar12 == 0) {
      bVar27 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                   CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                            CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                     CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                              CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                       CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
      if ((bVar27 & 1) != 0) goto LAB_1003a8168;
      lVar14 = lVar14 + 8;
      uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
               0x8080808080808080;
    }
    uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
    uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
    uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
    if ((local_320[uVar15 * -3 + -1] == (long ******)0x17) &&
       (pppppplVar13 = local_320[uVar15 * -3 + -2],
       (*pppppplVar13 == (long *****)0x705f6769666e6f63 && pppppplVar13[1] == (long *****)0x635f656c69666f72) &&
       *(long *)((long)pppppplVar13 + 0xf) == 0x7463696c666e6f63)) break;
    uVar12 = uVar12 - 1 & uVar12;
  }
  audit::RepairAudit::step(&local_190,param_8,&_anon_164d03a2dfc98f969363eaf1c50504fd_630,0x17,&DAT_101310885,0xe,param_2);
  if (local_190 == (long *******)&DAT_0000000b) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    ppppppplVar10 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x17,1);
    if (ppppppplVar10 == (long *******)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x17);
    }
    ppppppplVar10[1] = (long ******)0x635f656c69666f72;
    *ppppppplVar10 = (long ******)0x705f6769666e6f63;
    *(undefined8 *)((long)ppppppplVar10 + 0xf) = 0x7463696c666e6f63;
    local_b0 = (long *******)&DAT_00000017;
    local_a0 = (long *******)&DAT_00000017;
    ppppppplStack_a8 = ppppppplVar10;
    <alloc::string::String_as_core::clone::Clone>::clone(&local_130,&local_b0);
    ppppppplVar9 = local_2e0;
    if (local_2e0 == local_2f0) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2f0);
    }
    lVar14 = local_298;
    ppppppplVar20 = ppppppplStack_2e8 + (long)ppppppplVar9 * 3;
    ppppppplVar20[1] = (long ******)ppppppplStack_128;
    *ppppppplVar20 = (long ******)local_130;
    ppppppplVar20[2] = (long ******)local_120;
    local_2e0 = (long *******)((long)ppppppplVar9 + 1);
    if (local_298 == local_2a8) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2a8);
    }
    ppppppplVar9 = local_2b0;
    pppppplVar13 = local_2a0 + lVar14 * 3;
    *pppppplVar13 = (long *****)0x17;
    pppppplVar13[1] = (long *****)ppppppplVar10;
    pppppplVar13[2] = (long *****)0x17;
    local_298 = lVar14 + 1;
    if (local_2b0 == local_2c0) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
    }
    ppppppplVar10 = ppppppplStack_2b8 + (long)ppppppplVar9 * 3;
    ppppppplVar10[1] = (long ******)ppppppplStack_180;
    *ppppppplVar10 = (long ******)ppppppplStack_188;
    ppppppplVar10[2] = (long ******)local_178;
    local_2b0 = (long *******)((long)ppppppplVar9 + 1);
    bVar3 = true;
  }
  else {
    local_110._8_8_ = local_168;
    local_110._0_8_ = ppppppplStack_170;
    ppppppplStack_f8 = local_158;
    local_100 = ppppppplStack_160;
    ppppppplStack_e8 = local_148;
    local_f0 = ppppppplStack_150;
    uStack_d8 = uStack_138;
    uStack_e0 = uStack_140;
    ppppppplStack_128 = ppppppplStack_188;
    local_130 = local_190;
    ppppppplStack_118 = local_178;
    local_120 = ppppppplStack_180;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x17,1);
    ppppppplVar10 = local_2c8;
    if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x17);
    }
    puVar11[1] = 0x635f656c69666f72;
    *puVar11 = 0x705f6769666e6f63;
    *(undefined8 *)((long)puVar11 + 0xf) = 0x7463696c666e6f63;
    if (local_2c8 == local_2d8) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
    }
    puVar8 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
    *puVar8 = 0x17;
    puVar8[1] = puVar11;
    puVar8[2] = 0x17;
    local_2c8 = (long *******)((long)ppppppplVar10 + 1);
    local_b0 = (long *******)&local_130;
    ppppppplStack_a8 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_218,s_config_profile_conflict__101522bf4,&local_b0);
    ppppppplVar10 = local_2b0;
    if (local_2b0 == local_2c0) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
    }
    ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
    ppppppplVar9[1] = pppppplStack_210;
    *ppppppplVar9 = local_218;
    ppppppplVar9[2] = local_208;
    local_2b0 = (long *******)((long)ppppppplVar10 + 1);
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
  }
  if (local_308 == (long *******)0x0) goto LAB_1003a8584;
LAB_1003a8168:
  uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_399,0xc);
  lVar14 = 0;
  bVar26 = (byte)(uVar7 >> 0x39);
  uVar7 = uVar7 & (ulong)ppppppplStack_318;
  uVar28 = *(undefined8 *)((long)local_320 + uVar7);
  uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                    CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                             CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                      CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                               CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                        CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                 CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
           0x8080808080808080;
  while( true ) {
    while (uVar12 == 0) {
      bVar27 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                   CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                            CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                     CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                              CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                       CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
      if ((bVar27 & 1) != 0) goto LAB_1003a8488;
      lVar14 = lVar14 + 8;
      uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
               0x8080808080808080;
    }
    uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
    uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
    uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
    if ((local_320[uVar15 * -3 + -1] == (long ******)0xc) &&
       (*local_320[uVar15 * -3 + -2] == (long *****)0x735f6769666e6f63 && *(int *)(local_320[uVar15 * -3 + -2] + 1) == 0x656c6174)) break;
    uVar12 = uVar12 - 1 & uVar12;
  }
  audit::RepairAudit::step(&local_190,param_8,&_anon_164d03a2dfc98f969363eaf1c50504fd_399,0xc,&DAT_101310885,0xe,param_2);
  if (local_190 == (long *******)&DAT_0000000b) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    ppppppplVar10 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xc,1);
    if (ppppppplVar10 == (long *******)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0xc);
    }
    *(undefined4 *)(ppppppplVar10 + 1) = 0x656c6174;
    *ppppppplVar10 = (long ******)0x735f6769666e6f63;
    local_b0 = (long *******)&DAT_0000000c;
    local_a0 = (long *******)&DAT_0000000c;
    ppppppplStack_a8 = ppppppplVar10;
    <alloc::string::String_as_core::clone::Clone>::clone(&local_130,&local_b0);
    ppppppplVar9 = local_2e0;
    if (local_2e0 == local_2f0) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2f0);
    }
    lVar14 = local_298;
    ppppppplVar20 = ppppppplStack_2e8 + (long)ppppppplVar9 * 3;
    ppppppplVar20[1] = (long ******)ppppppplStack_128;
    *ppppppplVar20 = (long ******)local_130;
    ppppppplVar20[2] = (long ******)local_120;
    local_2e0 = (long *******)((long)ppppppplVar9 + 1);
    if (local_298 == local_2a8) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2a8);
    }
    ppppppplVar9 = local_2b0;
    pppppplVar13 = local_2a0 + lVar14 * 3;
    *pppppplVar13 = (long *****)0xc;
    pppppplVar13[1] = (long *****)ppppppplVar10;
    pppppplVar13[2] = (long *****)0xc;
    local_298 = lVar14 + 1;
    if (local_2b0 == local_2c0) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
    }
    ppppppplVar10 = ppppppplStack_2b8 + (long)ppppppplVar9 * 3;
    ppppppplVar10[1] = (long ******)ppppppplStack_180;
    *ppppppplVar10 = (long ******)ppppppplStack_188;
    ppppppplVar10[2] = (long ******)local_178;
    local_2b0 = (long *******)((long)ppppppplVar9 + 1);
    bVar3 = bVar3 | *(byte *)(param_3 + 0x16c);
  }
  else {
    local_110._8_8_ = local_168;
    local_110._0_8_ = ppppppplStack_170;
    ppppppplStack_f8 = local_158;
    local_100 = ppppppplStack_160;
    ppppppplStack_e8 = local_148;
    local_f0 = ppppppplStack_150;
    uStack_d8 = uStack_138;
    uStack_e0 = uStack_140;
    ppppppplStack_128 = ppppppplStack_188;
    local_130 = local_190;
    ppppppplStack_118 = local_178;
    local_120 = ppppppplStack_180;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xc,1);
    ppppppplVar10 = local_2c8;
    if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0xc);
    }
    *(undefined4 *)(puVar11 + 1) = 0x656c6174;
    *puVar11 = 0x735f6769666e6f63;
    if (local_2c8 == local_2d8) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
    }
    puVar8 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
    *puVar8 = 0xc;
    puVar8[1] = puVar11;
    puVar8[2] = 0xc;
    local_2c8 = (long *******)((long)ppppppplVar10 + 1);
    local_b0 = (long *******)&local_130;
    ppppppplStack_a8 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_200,s_config_stale__101522c10,&local_b0);
    ppppppplVar10 = local_2b0;
    if (local_2b0 == local_2c0) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
    }
    ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
    ppppppplVar9[1] = pppppplStack_1f8;
    *ppppppplVar9 = local_200;
    ppppppplVar9[2] = local_1f0;
    local_2b0 = (long *******)((long)ppppppplVar10 + 1);
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
  }
  if (local_308 == (long *******)0x0) goto LAB_1003a8584;
LAB_1003a8488:
  uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_597,0x14);
  lVar14 = 0;
  bVar26 = (byte)(uVar7 >> 0x39);
  uVar7 = uVar7 & (ulong)ppppppplStack_318;
  uVar28 = *(undefined8 *)((long)local_320 + uVar7);
  uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                    CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                             CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                      CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                               CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                        CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                 CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
           0x8080808080808080;
  while( true ) {
    while (uVar12 == 0) {
      bVar27 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                   CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                            CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                     CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                              CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                       CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
      if ((bVar27 & 1) != 0) goto LAB_1003a86c0;
      lVar14 = lVar14 + 8;
      uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
               0x8080808080808080;
    }
    uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
    uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
    uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
    if ((local_320[uVar15 * -3 + -1] == (long ******)0x14) &&
       (pppppplVar13 = local_320[uVar15 * -3 + -2],
       (*pppppplVar13 == (long *****)0x685f726574756f72 && pppppplVar13[1] == (long *****)0x746e6f635f707474) && *(int *)(pppppplVar13 + 2) == 0x74636172))
    break;
    uVar12 = uVar12 - 1 & uVar12;
  }
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  uVar28 = 0x14;
  ppppppplVar10 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x14,1);
  if (ppppppplVar10 == (long *******)0x0) goto LAB_1003a9eb8;
  *(undefined4 *)(ppppppplVar10 + 2) = 0x74636172;
  ppppppplVar10[1] = (long ******)0x746e6f635f707474;
  *ppppppplVar10 = (long ******)0x685f726574756f72;
  local_190 = (long *******)&DAT_00000014;
  ppppppplStack_180 = (long *******)&DAT_00000014;
  ppppppplStack_188 = ppppppplVar10;
  <alloc::string::String_as_core::clone::Clone>::clone(&local_130,&local_190);
  ppppppplVar9 = local_2e0;
  if (local_2e0 == local_2f0) {
    alloc::raw_vec::RawVec<T,A>::grow_one(&local_2f0);
  }
  lVar14 = local_298;
  ppppppplVar20 = ppppppplStack_2e8 + (long)ppppppplVar9 * 3;
  ppppppplVar20[1] = (long ******)ppppppplStack_128;
  *ppppppplVar20 = (long ******)local_130;
  ppppppplVar20[2] = (long ******)local_120;
  local_2e0 = (long *******)((long)ppppppplVar9 + 1);
  if (local_298 == local_2a8) {
    alloc::raw_vec::RawVec<T,A>::grow_one(&local_2a8);
  }
  pppppplVar13 = local_2a0 + lVar14 * 3;
  *pppppplVar13 = (long *****)0x14;
  pppppplVar13[1] = (long *****)ppppppplVar10;
  pppppplVar13[2] = (long *****)0x14;
  local_298 = lVar14 + 1;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  pppppplVar13 = (long ******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x2d,1);
  ppppppplVar10 = local_2b0;
  if (pppppplVar13 == (long ******)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x2d);
  }
  pppppplVar13[1] = (long *****)0x20a585e59986e5b0;
  *pppppplVar13 = (long *****)0x96e68d87e986b0e5;
  pppppplVar13[3] = (long *****)0x796c6e6f2d505454;
  pppppplVar13[2] = (long *****)0x48203169616d6961;
  *(undefined8 *)((long)pppppplVar13 + 0x25) = 0xa6bae791a5e5b194;
  *(undefined8 *)((long)pppppplVar13 + 0x1d) = 0xe7afb7e820796c6e;
  if (local_2b0 == local_2c0) {
    alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
  }
  ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
  *ppppppplVar9 = (long ******)0x2d;
  ppppppplVar9[1] = pppppplVar13;
  ppppppplVar9[2] = (long ******)0x2d;
  local_2b0 = (long *******)((long)ppppppplVar10 + 1);
  if (local_308 == (long *******)0x0) goto LAB_1003a8c28;
  bVar3 = 1;
LAB_1003a86c0:
  uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_486,0x11);
  ppppppplVar9 = ppppppplStack_318;
  ppppppplVar10 = local_320;
  lVar14 = 0;
  bVar26 = (byte)(uVar7 >> 0x39);
  uVar7 = uVar7 & (ulong)ppppppplStack_318;
  uVar28 = *(undefined8 *)((long)local_320 + uVar7);
  uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                    CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                             CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                      CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                               CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                        CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                 CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
           0x8080808080808080;
  while( true ) {
    while (uVar12 == 0) {
      bVar27 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                   CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                            CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                     CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                              CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                       CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
      if ((bVar27 & 1) != 0) {
        uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_627,0x15);
        lVar14 = 0;
        bVar26 = (byte)(uVar7 >> 0x39);
        uVar7 = uVar7 & (ulong)ppppppplVar9;
        uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
        uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                          CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                                   CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                            CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                                     CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                              CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                       CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
                 0x8080808080808080;
        while( true ) {
          for (; uVar12 != 0; uVar12 = uVar12 - 1 & uVar12) {
            uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
            uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
            uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplVar9;
            if ((ppppppplVar10[uVar15 * -3 + -1] == (long ******)0x15) &&
               (pppppplVar13 = ppppppplVar10[uVar15 * -3 + -2],
               (*pppppplVar13 == (long *****)0x5f676f6c61746163 && pppppplVar13[1] == (long *****)0x6c61765f68746170) &&
               *(long *)((long)pppppplVar13 + 0xd) == 0x79746964696c6176)) goto LAB_1003a8894;
          }
          bVar27 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                       CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                                CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                         CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                                  CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                           CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                    CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1)
          ;
          if ((bVar27 & 1) != 0) break;
          lVar14 = lVar14 + 8;
          uVar7 = uVar7 + lVar14 & (ulong)ppppppplVar9;
          uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
          uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                            CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                                     CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                              CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                                       CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                                CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                         CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
                   0x8080808080808080;
        }
        if ((bVar3 & 1) != 0) goto LAB_1003a8c28;
        goto LAB_1003a8fc8;
      }
      lVar14 = lVar14 + 8;
      uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar26),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar26),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar26),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar26),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar26),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar26),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar26),-((byte)uVar28 == bVar26)))))))) &
               0x8080808080808080;
    }
    uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
    uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
    uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
    if ((local_320[uVar15 * -3 + -1] == (long ******)0x11) &&
       (pppppplVar13 = local_320[uVar15 * -3 + -2],
       (*pppppplVar13 == (long *****)0x5f676f6c61746163 && pppppplVar13[1] == (long *****)0x7469726765746e69) && *(char *)(pppppplVar13 + 2) == 'y')) break;
    uVar12 = uVar12 - 1 & uVar12;
  }
LAB_1003a8894:
  uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_486,0x11);
  lVar14 = 0;
  bVar3 = (byte)(uVar7 >> 0x39);
  uVar7 = uVar7 & (ulong)ppppppplVar9;
  uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
  uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                    CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                             CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                      CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                               CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                        CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                 CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
           0x8080808080808080;
  while( true ) {
    for (; uVar12 != 0; uVar12 = uVar12 - 1 & uVar12) {
      uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
      uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
      uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplVar9;
      if ((ppppppplVar10[uVar15 * -3 + -1] == (long ******)0x11) &&
         (pppppplVar13 = ppppppplVar10[uVar15 * -3 + -2],
         (*pppppplVar13 == (long *****)0x5f676f6c61746163 && pppppplVar13[1] == (long *****)0x7469726765746e69) && *(char *)(pppppplVar13 + 2) == 'y')) {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar28 = 0x11;
        ppppppplVar10 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x11,1);
        if (ppppppplVar10 == (long *******)0x0) goto LAB_1003a9eb8;
        *(undefined1 *)(ppppppplVar10 + 2) = 0x79;
        ppppppplVar10[1] = (long ******)0x7469726765746e69;
        *ppppppplVar10 = (long ******)0x5f676f6c61746163;
        local_190 = (long *******)&DAT_00000011;
        ppppppplStack_180 = (long *******)&DAT_00000011;
        ppppppplStack_188 = ppppppplVar10;
        <alloc::string::String_as_core::clone::Clone>::clone(&local_130,&local_190);
        ppppppplVar9 = local_2e0;
        if (local_2e0 == local_2f0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2f0);
        }
        lVar14 = local_298;
        ppppppplVar20 = ppppppplStack_2e8 + (long)ppppppplVar9 * 3;
        ppppppplVar20[1] = (long ******)ppppppplStack_128;
        *ppppppplVar20 = (long ******)local_130;
        ppppppplVar20[2] = (long ******)local_120;
        local_2e0 = (long *******)((long)ppppppplVar9 + 1);
        if (local_298 == local_2a8) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2a8);
        }
        pppppplVar13 = local_2a0 + lVar14 * 3;
        *pppppplVar13 = (long *****)0x11;
        pppppplVar13[1] = (long *****)ppppppplVar10;
        pppppplVar13[2] = (long *****)0x11;
        local_298 = lVar14 + 1;
        if (local_308 != (long *******)0x0) goto LAB_1003a8a28;
        goto LAB_1003a8bb8;
      }
    }
    bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                 CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                          CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                   CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                            CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                     CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                              CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
    if ((bVar26 & 1) != 0) break;
    lVar14 = lVar14 + 8;
    uVar7 = uVar7 + lVar14 & (ulong)ppppppplVar9;
    uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
    uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                      CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                               CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                        CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                 CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                          CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                   CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
             0x8080808080808080;
  }
LAB_1003a8a28:
  uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_627,0x15);
  lVar14 = 0;
  bVar3 = (byte)(uVar7 >> 0x39);
  uVar7 = uVar7 & (ulong)ppppppplStack_318;
  uVar28 = *(undefined8 *)((long)local_320 + uVar7);
  uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                    CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                             CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                      CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                               CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                        CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                 CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
           0x8080808080808080;
  do {
    while (uVar12 == 0) {
      bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                   CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                            CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                     CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                              CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                       CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
      if ((bVar26 & 1) != 0) {
LAB_1003a8bb8:
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar28 = 0x28;
        pppppplVar13 = (long ******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x28,1);
        ppppppplVar10 = local_2b0;
        if (pppppplVar13 == (long ******)0x0) {
LAB_1003a9eb8:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,uVar28);
        }
        pppppplVar13[1] = (long *****)0xb8e4206769666e6f;
        *pppppplVar13 = (long *****)0x63208f9ae986b0e5;
        pppppplVar13[3] = (long *****)0x9088e69f94e7b096;
        pppppplVar13[2] = (long *****)0xe68d87e9b7b5e880;
        pppppplVar13[4] = (long *****)0x676f6c6174614320;
        if (local_2b0 == local_2c0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
        }
        ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
        *ppppppplVar9 = (long ******)0x28;
        ppppppplVar9[1] = pppppplVar13;
        ppppppplVar9[2] = (long ******)0x28;
        local_2b0 = (long *******)((long)ppppppplVar10 + 1);
        goto LAB_1003a8c28;
      }
      lVar14 = lVar14 + 8;
      uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
               0x8080808080808080;
    }
    uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
    uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
    uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
    if ((local_320[uVar15 * -3 + -1] == (long ******)0x15) &&
       (pppppplVar13 = local_320[uVar15 * -3 + -2],
       (*pppppplVar13 == (long *****)0x5f676f6c61746163 && pppppplVar13[1] == (long *****)0x6c61765f68746170) &&
       *(long *)((long)pppppplVar13 + 0xd) == 0x79746964696c6176)) {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar28 = 0x15;
      ppppppplVar10 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x15,1);
      if (ppppppplVar10 == (long *******)0x0) goto LAB_1003a9eb8;
      ppppppplVar10[1] = (long ******)0x6c61765f68746170;
      *ppppppplVar10 = (long ******)0x5f676f6c61746163;
      *(undefined8 *)((long)ppppppplVar10 + 0xd) = 0x79746964696c6176;
      local_190 = (long *******)&DAT_00000015;
      ppppppplStack_180 = (long *******)&DAT_00000015;
      ppppppplStack_188 = ppppppplVar10;
      <alloc::string::String_as_core::clone::Clone>::clone(&local_130,&local_190);
      ppppppplVar9 = local_2e0;
      if (local_2e0 == local_2f0) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&local_2f0);
      }
      lVar14 = local_298;
      ppppppplVar20 = ppppppplStack_2e8 + (long)ppppppplVar9 * 3;
      ppppppplVar20[1] = (long ******)ppppppplStack_128;
      *ppppppplVar20 = (long ******)local_130;
      ppppppplVar20[2] = (long ******)local_120;
      local_2e0 = (long *******)((long)ppppppplVar9 + 1);
      if (local_298 == local_2a8) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&local_2a8);
      }
      pppppplVar13 = local_2a0 + lVar14 * 3;
      *pppppplVar13 = (long *****)0x15;
      pppppplVar13[1] = (long *****)ppppppplVar10;
      pppppplVar13[2] = (long *****)0x15;
      local_298 = lVar14 + 1;
      goto LAB_1003a8bb8;
    }
    uVar12 = uVar12 - 1 & uVar12;
  } while( true );
LAB_1003a8d60:
  do {
    if ((long ******)*psVar23 == pppppplVar2) {
      iVar5 = _memcmp((void *)psVar23[-1],ppppplVar22,(size_t)pppppplVar2);
      if (iVar5 == 0) goto LAB_1003a8dcc;
    }
    psVar23 = psVar23 + 3;
    lVar17 = lVar17 + -0x18;
  } while (lVar17 != 0);
LAB_1003a8d84:
  <alloc::string::String_as_core::clone::Clone>::clone(&local_b0,pppppplVar16);
  ppppppplVar9 = local_2c8;
  if (local_2c8 == local_2d8) {
    alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
  }
  puVar11 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar9 * 0x18);
  puVar11[1] = ppppppplStack_a8;
  *puVar11 = local_b0;
  puVar11[2] = local_a0;
  local_2c8 = (long *******)((long)ppppppplVar9 + 1);
LAB_1003a8dcc:
  alloc::vec::Vec<T,A>::retain(&local_2f0,&local_1d0);
  local_b0 = &local_1d0;
  ppppppplStack_a8 = (long *******)<&T_as_core::fmt::Display>::fmt;
  alloc::fmt::format::format_inner(&local_1c8,s___resync_101522c3a,&local_b0);
  ppppppplVar9 = local_2b0;
  if (local_2b0 == local_2c0) {
    alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
  }
  ppppppplVar20 = ppppppplStack_2b8 + (long)ppppppplVar9 * 3;
  ppppppplVar20[1] = pppppplStack_1c0;
  *ppppppplVar20 = local_1c8;
  ppppppplVar20[2] = local_1b8;
  local_2b0 = (long *******)((long)ppppppplVar9 + 1);
LAB_1003a8cc8:
  pppppplVar16 = pppppplVar1;
  if (pppppplVar1 == pppppplVar13) goto LAB_1003a8f98;
  goto LAB_1003a8cd0;
LAB_1003a8f98:
  <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_130);
  if (local_130 != (long *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_128,(long)local_130 * 0x68,8);
  }
joined_r0x0001003a8f70:
  if (local_308 != (long *******)0x0) {
LAB_1003a8fc8:
    uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_576,0x13);
    ppppppplVar9 = ppppppplStack_318;
    ppppppplVar10 = local_320;
    lVar14 = 0;
    bVar3 = (byte)(uVar7 >> 0x39);
    uVar7 = uVar7 & (ulong)ppppppplStack_318;
    uVar28 = *(undefined8 *)((long)local_320 + uVar7);
    uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                      CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                               CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                        CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                 CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                          CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                   CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
             0x8080808080808080;
    while( true ) {
      for (; uVar12 != 0; uVar12 = uVar12 - 1 & uVar12) {
        uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
        uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
        if ((local_320[uVar15 * -3 + -1] == (long ******)0x13) &&
           (pppppplVar13 = local_320[uVar15 * -3 + -2],
           (*pppppplVar13 == (long *****)0x616870726f5f6264 && pppppplVar13[1] == (long *****)0x6469766f72705f6e) &&
           *(long *)((long)pppppplVar13 + 0xb) == 0x7372656469766f72)) goto LAB_1003a91a8;
      }
      bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                   CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                            CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                     CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                              CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                       CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
      if ((bVar26 & 1) != 0) break;
      lVar14 = lVar14 + 8;
      uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
               0x8080808080808080;
    }
    uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_577,0x18);
    lVar14 = 0;
    bVar3 = (byte)(uVar7 >> 0x39);
    uVar7 = uVar7 & (ulong)ppppppplVar9;
    uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
    uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                      CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                               CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                        CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                 CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                          CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                   CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
             0x8080808080808080;
    while( true ) {
      for (; uVar12 != 0; uVar12 = uVar12 - 1 & uVar12) {
        uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
        uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplVar9;
        if ((ppppppplVar10[uVar15 * -3 + -1] == (long ******)0x18) &&
           (pppppplVar13 = ppppppplVar10[uVar15 * -3 + -2],
           (*pppppplVar13 == (long *****)0x5f74756f6c6c6f72 && pppppplVar13[1] == (long *****)0x705f6e616870726f) &&
           pppppplVar13[2] == (long *****)0x7372656469766f72)) goto LAB_1003a91a8;
      }
      bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                   CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                            CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                     CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                              CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                       CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
      if ((bVar26 & 1) != 0) break;
      lVar14 = lVar14 + 8;
      uVar7 = uVar7 + lVar14 & (ulong)ppppppplVar9;
      uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
               0x8080808080808080;
    }
  }
  goto LAB_1003a9744;
LAB_1003a91a8:
  uVar24 = 0x13;
  uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_576,0x13);
  lVar14 = 0;
  bVar3 = (byte)(uVar7 >> 0x39);
  uVar7 = uVar7 & (ulong)ppppppplVar9;
  uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
  uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                    CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                             CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                      CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                               CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                        CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                 CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
           0x8080808080808080;
  while( true ) {
    for (; uVar12 != 0; uVar12 = uVar12 - 1 & uVar12) {
      uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
      uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
      uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplVar9;
      if ((ppppppplVar10[uVar15 * -3 + -1] == (long ******)0x13) &&
         (pppppplVar13 = ppppppplVar10[uVar15 * -3 + -2], puVar25 = &_anon_164d03a2dfc98f969363eaf1c50504fd_576,
         (*pppppplVar13 == (long *****)0x616870726f5f6264 && pppppplVar13[1] == (long *****)0x6469766f72705f6e) &&
         *(long *)((long)pppppplVar13 + 0xb) == 0x7372656469766f72)) goto LAB_1003a9284;
    }
    bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                 CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                          CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                   CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                            CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                     CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                              CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1);
    if ((bVar26 & 1) != 0) break;
    lVar14 = lVar14 + 8;
    uVar7 = uVar7 + lVar14 & (ulong)ppppppplVar9;
    uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar7);
    uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                      CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                               CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                        CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                 CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                          CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                   CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
             0x8080808080808080;
  }
  uVar24 = 0x18;
  puVar25 = &_anon_164d03a2dfc98f969363eaf1c50504fd_577;
LAB_1003a9284:
  local_120 = (long *******)&uStack_348;
  local_130 = (long *******)param_2;
  ppppppplStack_128 = (long *******)param_3;
  audit::RepairAudit::fix(&local_190,param_8,puVar25,uVar24,&local_130);
  ppppppplVar20 = local_178;
  ppppppplVar9 = ppppppplStack_180;
  ppppppplVar10 = ppppppplStack_188;
  if (local_190 == (long *******)&DAT_0000000b) {
    ppppppplStack_128 = ppppppplStack_180;
    local_130 = ppppppplStack_188;
    ppppppplStack_118 = ppppppplStack_170;
    local_120 = local_178;
    local_110._8_8_ = ppppppplStack_160;
    local_110._0_8_ = local_168;
    ppppppplStack_f8 = ppppppplStack_150;
    local_100 = local_158;
    local_f0 = local_148;
    sVar21 = (long)local_178 * 0x18;
    local_98 = ppppppplStack_180 + (long)local_178 * 3;
    local_b0 = ppppppplStack_180;
    ppppppplStack_a8 = ppppppplStack_180;
    local_a0 = ppppppplStack_188;
    if ((long *******)((long)local_2f0 - (long)local_2e0) < local_178) {
      alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2f0,local_2e0,local_178,8,0x18);
    }
    ppppppplVar19 = local_2e0;
    _memcpy(ppppppplStack_2e8 + (long)local_2e0 * 3,ppppppplVar9,sVar21);
    local_2e0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
    if (ppppppplVar10 != (long *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
    }
    uVar24 = local_110._8_8_;
    uVar28 = local_110._0_8_;
    ppppppplVar10 = ppppppplStack_118;
    sVar21 = local_110._8_8_ * 0x18;
    local_98 = (long *******)(local_110._0_8_ + local_110._8_8_ * 3 * 8);
    local_b0 = (long *******)local_110._0_8_;
    ppppppplStack_a8 = (long *******)local_110._0_8_;
    local_a0 = ppppppplStack_118;
    if ((long *******)((long)local_2d8 - (long)local_2c8) < (ulong)local_110._8_8_) {
      alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2d8,local_2c8,local_110._8_8_,8,0x18);
    }
    ppppppplVar9 = local_2c8;
    _memcpy(puStack_2d0 + (long)local_2c8 * 0x18,(void *)uVar28,sVar21);
    local_2c8 = (long *******)((long)ppppppplVar9 + uVar24);
    if (ppppppplVar10 != (long *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar28,(long)ppppppplVar10 * 0x18,8);
    }
    ppppppplVar20 = local_f0;
    ppppppplVar9 = ppppppplStack_f8;
    ppppppplVar10 = local_100;
    sVar21 = (long)local_f0 * 0x18;
    local_98 = ppppppplStack_f8 + (long)local_f0 * 3;
    local_b0 = ppppppplStack_f8;
    ppppppplStack_a8 = ppppppplStack_f8;
    local_a0 = local_100;
    if ((long *******)((long)local_2c0 - (long)local_2b0) < local_f0) {
      alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2c0,local_2b0,local_f0,8,0x18);
    }
    ppppppplVar19 = local_2b0;
    _memcpy(ppppppplStack_2b8 + (long)local_2b0 * 3,ppppppplVar9,sVar21);
    local_2b0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
    if (ppppppplVar10 != (long *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
    }
  }
  else {
    local_110._8_8_ = local_168;
    local_110._0_8_ = ppppppplStack_170;
    ppppppplStack_f8 = local_158;
    local_100 = ppppppplStack_160;
    ppppppplStack_e8 = local_148;
    local_f0 = ppppppplStack_150;
    uStack_d8 = uStack_138;
    uStack_e0 = uStack_140;
    ppppppplStack_128 = ppppppplStack_188;
    local_130 = local_190;
    ppppppplStack_118 = local_178;
    local_120 = ppppppplStack_180;
    if (local_308 != (long *******)0x0) {
      uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_576,0x13);
      lVar14 = 0;
      bVar3 = (byte)(uVar7 >> 0x39);
      uVar7 = uVar7 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
               0x8080808080808080;
      while( true ) {
        while (uVar12 == 0) {
          bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                       CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                                CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                         CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                                  CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                           CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                    CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1)
          ;
          if ((bVar26 & 1) != 0) goto LAB_1003a9584;
          lVar14 = lVar14 + 8;
          uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
          uVar28 = *(undefined8 *)((long)local_320 + uVar7);
          uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                            CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                     CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                              CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                       CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                                CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                         CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
                   0x8080808080808080;
        }
        uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
        uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
        if ((local_320[uVar15 * -3 + -1] == (long ******)0x13) &&
           (pppppplVar13 = local_320[uVar15 * -3 + -2],
           (*pppppplVar13 == (long *****)0x616870726f5f6264 && pppppplVar13[1] == (long *****)0x6469766f72705f6e) &&
           *(long *)((long)pppppplVar13 + 0xb) == 0x7372656469766f72)) break;
        uVar12 = uVar12 - 1 & uVar12;
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar28 = 0x13;
      puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x13,1);
      ppppppplVar10 = local_2c8;
      if (puVar11 == (undefined8 *)0x0) goto LAB_1003a9f78;
      *(undefined4 *)((long)puVar11 + 0xf) = 0x73726564;
      puVar11[1] = 0x6469766f72705f6e;
      *puVar11 = 0x616870726f5f6264;
      if (local_2c8 == local_2d8) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
      }
      puVar8 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
      *puVar8 = 0x13;
      puVar8[1] = puVar11;
      puVar8[2] = 0x13;
      local_2c8 = (long *******)((long)ppppppplVar10 + 1);
      if (local_308 == (long *******)0x0) goto LAB_1003a96d4;
LAB_1003a9584:
      uVar7 = ::core::hash::BuildHasher::hash_one(local_300,&_anon_164d03a2dfc98f969363eaf1c50504fd_577,0x18);
      lVar14 = 0;
      bVar3 = (byte)(uVar7 >> 0x39);
      uVar7 = uVar7 & (ulong)ppppppplStack_318;
      uVar28 = *(undefined8 *)((long)local_320 + uVar7);
      uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                        CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                 CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                          CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                   CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                            CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                     CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
               0x8080808080808080;
      while( true ) {
        while (uVar12 == 0) {
          bVar26 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar28 >> 0x38) == -1),
                                       CONCAT16(-((char)((ulong)uVar28 >> 0x30) == -1),
                                                CONCAT15(-((char)((ulong)uVar28 >> 0x28) == -1),
                                                         CONCAT14(-((char)((ulong)uVar28 >> 0x20) == -1),
                                                                  CONCAT13(-((char)((ulong)uVar28 >> 0x18) == -1),
                                                                           CONCAT12(-((char)((ulong)uVar28 >> 0x10) == -1),
                                                                                    CONCAT11(-((char)((ulong)uVar28 >> 8) == -1),-((char)uVar28 == -1)))))))),1)
          ;
          if ((bVar26 & 1) != 0) goto LAB_1003a96d4;
          lVar14 = lVar14 + 8;
          uVar7 = uVar7 + lVar14 & (ulong)ppppppplStack_318;
          uVar28 = *(undefined8 *)((long)local_320 + uVar7);
          uVar12 = CONCAT17(-((byte)((ulong)uVar28 >> 0x38) == bVar3),
                            CONCAT16(-((byte)((ulong)uVar28 >> 0x30) == bVar3),
                                     CONCAT15(-((byte)((ulong)uVar28 >> 0x28) == bVar3),
                                              CONCAT14(-((byte)((ulong)uVar28 >> 0x20) == bVar3),
                                                       CONCAT13(-((byte)((ulong)uVar28 >> 0x18) == bVar3),
                                                                CONCAT12(-((byte)((ulong)uVar28 >> 0x10) == bVar3),
                                                                         CONCAT11(-((byte)((ulong)uVar28 >> 8) == bVar3),-((byte)uVar28 == bVar3)))))))) &
                   0x8080808080808080;
        }
        uVar15 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
        uVar15 = uVar7 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplStack_318;
        if ((local_320[uVar15 * -3 + -1] == (long ******)0x18) &&
           (pppppplVar13 = local_320[uVar15 * -3 + -2],
           (*pppppplVar13 == (long *****)0x5f74756f6c6c6f72 && pppppplVar13[1] == (long *****)0x705f6e616870726f) &&
           pppppplVar13[2] == (long *****)0x7372656469766f72)) break;
        uVar12 = uVar12 - 1 & uVar12;
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar28 = 0x18;
      puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,1);
      ppppppplVar10 = local_2c8;
      if (puVar11 == (undefined8 *)0x0) {
LAB_1003a9f78:
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,uVar28);
      }
      puVar11[1] = 0x705f6e616870726f;
      *puVar11 = 0x5f74756f6c6c6f72;
      puVar11[2] = 0x7372656469766f72;
      if (local_2c8 == local_2d8) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
      }
      puVar8 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
      *puVar8 = 0x18;
      puVar8[1] = puVar11;
      puVar8[2] = 0x18;
      local_2c8 = (long *******)((long)ppppppplVar10 + 1);
    }
LAB_1003a96d4:
    local_b0 = (long *******)&local_130;
    ppppppplStack_a8 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_1b0,s_orphan_thread_adoption__101522c58,&local_b0);
    ppppppplVar10 = local_2b0;
    if (local_2b0 == local_2c0) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
    }
    ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
    ppppppplVar9[1] = pppppplStack_1a8;
    *ppppppplVar9 = local_1b0;
    ppppppplVar9[2] = local_1a0;
    local_2b0 = (long *******)((long)ppppppplVar10 + 1);
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
  }
LAB_1003a9744:
  lVar14 = 0;
  do {
    local_198 = (long ******)((long)&PTR__anon_164d03a2dfc98f969363eaf1c50504fd_633_10181b9d0 + lVar14);
    if (local_308 != (long *******)0x0) {
      pppppplVar13 = (long ******)*local_198;
      pppppplVar16 = *(long *******)((long)&DAT_10181b9d8 + lVar14);
      uVar12 = ::core::hash::BuildHasher::hash_one(local_300,pppppplVar13,pppppplVar16);
      ppppppplVar9 = ppppppplStack_318;
      ppppppplVar10 = local_320;
      lVar17 = 0;
      bVar3 = (byte)(uVar12 >> 0x39);
      do {
        uVar12 = uVar12 & (ulong)ppppppplVar9;
        uVar28 = *(undefined8 *)((long)ppppppplVar10 + uVar12);
        bVar26 = (byte)((ulong)uVar28 >> 8);
        bVar27 = (byte)((ulong)uVar28 >> 0x10);
        bVar29 = (byte)((ulong)uVar28 >> 0x18);
        bVar30 = (byte)((ulong)uVar28 >> 0x20);
        bVar31 = (byte)((ulong)uVar28 >> 0x28);
        bVar32 = (byte)((ulong)uVar28 >> 0x30);
        bVar33 = (byte)((ulong)uVar28 >> 0x38);
        for (uVar7 = CONCAT17(-(bVar33 == bVar3),
                              CONCAT16(-(bVar32 == bVar3),
                                       CONCAT15(-(bVar31 == bVar3),
                                                CONCAT14(-(bVar30 == bVar3),
                                                         CONCAT13(-(bVar29 == bVar3),
                                                                  CONCAT12(-(bVar27 == bVar3),CONCAT11(-(bVar26 == bVar3),-((byte)uVar28 == bVar3)))))))) &
                     0x8080808080808080; uVar7 != 0; uVar7 = uVar7 - 1 & uVar7) {
          uVar15 = (uVar7 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar7 >> 7 & 0xff00ff00ff00ff) << 8;
          uVar15 = (uVar15 & 0xffff0000ffff0000) >> 0x10 | (uVar15 & 0xffff0000ffff) << 0x10;
          uVar15 = uVar12 + ((ulong)LZCOUNT(uVar15 >> 0x20 | uVar15 << 0x20) >> 3) & (ulong)ppppppplVar9;
          if (pppppplVar16 == ppppppplVar10[uVar15 * -3 + -1]) {
            iVar5 = _memcmp(pppppplVar13,ppppppplVar10[uVar15 * -3 + -2],(size_t)pppppplVar16);
            if (iVar5 == 0) {
              local_88[0] = uStack_348;
              local_130 = (long *******)&local_b0;
              ppppppplStack_128 = (long *******)&local_89;
              local_100 = (long *******)local_88;
              local_120 = (long *******)param_2;
              ppppppplStack_118 = (long *******)param_3;
              local_110._0_8_ = param_4;
              local_110._8_8_ = param_5;
              local_b0 = (long *******)pppppplVar13;
              ppppppplStack_a8 = (long *******)pppppplVar16;
              local_89 = param_6;
              audit::RepairAudit::fix(&local_190,param_8,pppppplVar13,pppppplVar16,&local_130);
              ppppppplVar20 = local_178;
              ppppppplVar9 = ppppppplStack_180;
              ppppppplVar10 = ppppppplStack_188;
              if (local_190 == (long *******)&DAT_0000000b) {
                ppppppplStack_128 = ppppppplStack_180;
                local_130 = ppppppplStack_188;
                ppppppplStack_118 = ppppppplStack_170;
                local_120 = local_178;
                local_110._8_8_ = ppppppplStack_160;
                local_110._0_8_ = local_168;
                ppppppplStack_f8 = ppppppplStack_150;
                local_100 = local_158;
                local_f0 = local_148;
                sVar21 = (long)local_178 * 0x18;
                local_98 = ppppppplStack_180 + (long)local_178 * 3;
                local_b0 = ppppppplStack_180;
                ppppppplStack_a8 = ppppppplStack_180;
                local_a0 = ppppppplStack_188;
                if ((long *******)((long)local_2f0 - (long)local_2e0) < local_178) {
                  alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2f0,local_2e0,local_178,8,0x18);
                }
                ppppppplVar19 = local_2e0;
                _memcpy(ppppppplStack_2e8 + (long)local_2e0 * 3,ppppppplVar9,sVar21);
                local_2e0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
                if (ppppppplVar10 != (long *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
                }
                uVar24 = local_110._8_8_;
                uVar28 = local_110._0_8_;
                ppppppplVar10 = ppppppplStack_118;
                sVar21 = local_110._8_8_ * 0x18;
                local_98 = (long *******)(local_110._0_8_ + local_110._8_8_ * 3 * 8);
                local_b0 = (long *******)local_110._0_8_;
                ppppppplStack_a8 = (long *******)local_110._0_8_;
                local_a0 = ppppppplStack_118;
                if ((long *******)((long)local_2d8 - (long)local_2c8) < (ulong)local_110._8_8_) {
                  alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2d8,local_2c8,local_110._8_8_,8,0x18);
                }
                ppppppplVar9 = local_2c8;
                _memcpy(puStack_2d0 + (long)local_2c8 * 0x18,(void *)uVar28,sVar21);
                local_2c8 = (long *******)((long)ppppppplVar9 + uVar24);
                if (ppppppplVar10 != (long *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar28,(long)ppppppplVar10 * 0x18,8);
                }
                ppppppplVar20 = local_f0;
                ppppppplVar9 = ppppppplStack_f8;
                ppppppplVar10 = local_100;
                sVar21 = (long)local_f0 * 0x18;
                local_98 = ppppppplStack_f8 + (long)local_f0 * 3;
                local_b0 = ppppppplStack_f8;
                ppppppplStack_a8 = ppppppplStack_f8;
                local_a0 = local_100;
                if ((long *******)((long)local_2c0 - (long)local_2b0) < local_f0) {
                  alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_2c0,local_2b0,local_f0,8,0x18);
                }
                ppppppplVar19 = local_2b0;
                _memcpy(ppppppplStack_2b8 + (long)local_2b0 * 3,ppppppplVar9,sVar21);
                local_2b0 = (long *******)((long)ppppppplVar19 + (long)ppppppplVar20);
                if (ppppppplVar10 != (long *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,(long)ppppppplVar10 * 0x18,8);
                }
              }
              else {
                ppppppplStack_128 = ppppppplStack_188;
                local_130 = local_190;
                ppppppplStack_118 = local_178;
                local_120 = ppppppplStack_180;
                ppppppplStack_e8 = local_148;
                local_f0 = ppppppplStack_150;
                uStack_d8 = uStack_138;
                uStack_e0 = uStack_140;
                local_110._8_8_ = local_168;
                local_110._0_8_ = ppppppplStack_170;
                ppppppplStack_f8 = local_158;
                local_100 = ppppppplStack_160;
                ppppplVar22 = local_198[1];
                if ((long)ppppplVar22 < 0) {
                  uVar28 = 0;
LAB_1003a9b58:
                    /* WARNING: Subroutine does not return */
                  alloc::raw_vec::handle_error(uVar28,local_388);
                }
                ppppplVar18 = *local_198;
                if (ppppplVar22 == (long *****)0x0) {
                  local_388 = (long *****)0x1;
                }
                else {
                  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                  local_388 = (long *****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(ppppplVar22,1);
                  if (local_388 == (long *****)0x0) {
                    uVar28 = 1;
                    local_388 = ppppplVar22;
                    goto LAB_1003a9b58;
                  }
                }
                _memcpy(local_388,ppppplVar18,(size_t)ppppplVar22);
                ppppppplVar10 = local_2c8;
                if (local_2c8 == local_2d8) {
                  alloc::raw_vec::RawVec<T,A>::grow_one(&local_2d8);
                }
                puVar11 = (undefined8 *)(puStack_2d0 + (long)ppppppplVar10 * 0x18);
                *puVar11 = ppppplVar22;
                puVar11[1] = local_388;
                puVar11[2] = ppppplVar22;
                local_2c8 = (long *******)((long)ppppppplVar10 + 1);
                local_b0 = &local_198;
                local_a0 = (long *******)&local_130;
                ppppppplStack_a8 = (long *******)((long)&DAT_1007dd6df + 1);
                local_98 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
                alloc::fmt::format::format_inner(&local_c8,_anon_164d03a2dfc98f969363eaf1c50504fd_17,&local_b0);
                ppppppplVar10 = local_2b0;
                if (local_2b0 == local_2c0) {
                  alloc::raw_vec::RawVec<T,A>::grow_one(&local_2c0);
                }
                ppppppplVar9 = ppppppplStack_2b8 + (long)ppppppplVar10 * 3;
                ppppppplVar9[1] = pppppplStack_c0;
                *ppppppplVar9 = local_c8;
                ppppppplVar9[2] = local_b8;
                local_2b0 = (long *******)((long)ppppppplVar10 + 1);
                ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_130);
              }
              goto LAB_1003a9998;
            }
          }
        }
        bVar26 = NEON_umaxv(CONCAT17(-(bVar33 == 0xff),
                                     CONCAT16(-(bVar32 == 0xff),
                                              CONCAT15(-(bVar31 == 0xff),
                                                       CONCAT14(-(bVar30 == 0xff),
                                                                CONCAT13(-(bVar29 == 0xff),
                                                                         CONCAT12(-(bVar27 == 0xff),CONCAT11(-(bVar26 == 0xff),-((byte)uVar28 == 0xff)))))))),1)
        ;
        if ((bVar26 & 1) != 0) break;
        lVar17 = lVar17 + 8;
        uVar12 = uVar12 + lVar17;
      } while( true );
    }
LAB_1003a9998:
    lVar14 = lVar14 + 0x10;
  } while (lVar14 != 0xb0);
  ppppppplStack_128 = ppppppplStack_2e8;
  local_130 = local_2f0;
  local_110._0_8_ = puStack_2d0;
  ppppppplStack_118 = local_2d8;
  local_120 = local_2e0;
  local_110._8_8_ = local_2c8;
  ppppppplStack_f8 = ppppppplStack_2b8;
  local_100 = local_2c0;
  local_f0 = local_2b0;
  param_1[8] = ppppppplStack_2b8;
  param_1[7] = local_2c0;
  param_1[9] = local_2b0;
  param_1[2] = ppppppplStack_2e8;
  param_1[1] = local_2f0;
  param_1[4] = local_2d8;
  param_1[3] = local_2e0;
  param_1[6] = local_2c8;
  param_1[5] = puStack_2d0;
  *param_1 = 0xb;
  if (local_298 != 0) {
    pppppplVar13 = local_2a0 + 1;
    lVar14 = local_298;
    do {
      if (pppppplVar13[-1] != (long *****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppplVar13,pppppplVar13[-1],1);
      }
      pppppplVar13 = pppppplVar13 + 3;
      lVar14 = lVar14 + -1;
    } while (lVar14 != 0);
  }
  if (local_2a8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2a0,local_2a8 * 0x18,8);
  }
  ppppppplVar10 = ppppppplStack_318;
  if (ppppppplStack_318 != (long *******)0x0) {
    if (local_308 != (long *******)0x0) {
      ppppppplVar20 = local_320 + 1;
      pppppplVar13 = *local_320;
      uVar12 = CONCAT17(-(-1 < (long)pppppplVar13),
                        CONCAT16(-(-1 < (char)((ulong)pppppplVar13 >> 0x30)),
                                 CONCAT15(-(-1 < (char)((ulong)pppppplVar13 >> 0x28)),
                                          CONCAT14(-(-1 < (char)((ulong)pppppplVar13 >> 0x20)),
                                                   CONCAT13(-(-1 < (char)((ulong)pppppplVar13 >> 0x18)),
                                                            CONCAT12(-(-1 < (char)((ulong)pppppplVar13 >> 0x10)),
                                                                     CONCAT11(-(-1 < (char)((ulong)pppppplVar13 >> 8)),-(-1 < (char)pppppplVar13)))))))) &
               0x8080808080808080;
      ppppppplVar9 = local_320;
      ppppppplVar19 = local_308;
      do {
        while (uVar12 == 0) {
          pppppplVar13 = *ppppppplVar20;
          ppppppplVar9 = ppppppplVar9 + -0x18;
          ppppppplVar20 = ppppppplVar20 + 1;
          uVar12 = CONCAT17(-(-1 < (long)pppppplVar13),
                            CONCAT16(-(-1 < (char)((ulong)pppppplVar13 >> 0x30)),
                                     CONCAT15(-(-1 < (char)((ulong)pppppplVar13 >> 0x28)),
                                              CONCAT14(-(-1 < (char)((ulong)pppppplVar13 >> 0x20)),
                                                       CONCAT13(-(-1 < (char)((ulong)pppppplVar13 >> 0x18)),
                                                                CONCAT12(-(-1 < (char)((ulong)pppppplVar13 >> 0x10)),
                                                                         CONCAT11(-(-1 < (char)((ulong)pppppplVar13 >> 8)),-(-1 < (char)pppppplVar13)))))))) &
                   0x8080808080808080;
        }
        uVar7 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
        uVar7 = (uVar7 & 0xffff0000ffff0000) >> 0x10 | (uVar7 & 0xffff0000ffff) << 0x10;
        iVar5 = (int)((ulong)LZCOUNT(uVar7 >> 0x20 | uVar7 << 0x20) >> 3);
        if (ppppppplVar9[(long)-iVar5 * 3 + -3] != (long ******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9[(long)-iVar5 * 3 + -2],ppppppplVar9[(long)-iVar5 * 3 + -3],1);
        }
        uVar12 = uVar12 - 1 & uVar12;
        ppppppplVar19 = (long *******)((long)ppppppplVar19 + -1);
      } while (ppppppplVar19 != (long *******)0x0);
    }
    lVar14 = (long)ppppppplVar10 * 0x19 + 0x21;
    if (lVar14 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_320 + (long)ppppppplVar10 * -3 + -3,lVar14,8);
    }
  }
  <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_340);
LAB_1003a6f94:
  if (local_340 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_338,local_340 * 0x68,8);
  }
  return;
}
