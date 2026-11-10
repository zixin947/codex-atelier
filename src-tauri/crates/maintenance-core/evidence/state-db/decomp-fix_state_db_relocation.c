===BEGIN fix_state_db_relocation @ 1003789f4=== (GhidraScript)  

/* codexmate_lib::core::relay::codex_diagnostic::fix_state_db_relocation */

void codexmate_lib::core::relay::codex_diagnostic::fix_state_db_relocation(undefined8 *param_1,long param_2)

{
  undefined8 uVar1;
  undefined8 ****ppppuVar2;
  undefined8 ****ppppuVar3;
  undefined8 ****ppppuVar4;
  int iVar5;
  void *pvVar6;
  undefined8 *****pppppuVar7;
  undefined8 ****ppppuVar8;
  undefined8 *puVar9;
  long lVar10;
  ulong uVar11;
  undefined8 *****pppppuVar12;
  long lVar13;
  undefined8 *****pppppuVar14;
  void *pvVar15;
  long lVar16;
  long lVar17;
  undefined8 uVar18;
  undefined8 *puVar19;
  undefined8 uVar20;
  void *pvVar21;
  long lVar22;
  undefined8 ***pppuVar23;
  size_t sVar24;
  size_t *psVar25;
  undefined1 auVar26 [12];
  undefined1 auVar27 [16];
  undefined8 **local_358;
  undefined8 ****local_340;
  undefined8 ****local_330;
  undefined8 local_328;
  long local_320;
  long local_300;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  long local_2e8;
  undefined8 local_2e0;
  undefined8 uStack_2d8;
  size_t local_2d0;
  void *local_2c8;
  size_t local_2c0;
  long local_2b8;
  long local_2b0;
  long local_2a8;
  long local_2a0;
  undefined8 local_298;
  undefined8 ****ppppuStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 **local_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 local_250;
  undefined8 uStack_248;
  undefined8 local_240;
  undefined8 **local_238;
  undefined8 ****local_230;
  undefined8 ****ppppuStack_228;
  char local_220;
  undefined7 uStack_21f;
  undefined1 uStack_218;
  undefined8 ****ppppuStack_217;
  undefined8 **local_20f;
  undefined7 uStack_207;
  undefined1 uStack_200;
  undefined7 uStack_1ff;
  undefined2 local_1f8;
  undefined1 local_1f6;
  undefined8 ****local_1f0;
  undefined8 ****ppppuStack_1e8;
  char local_1e0;
  undefined7 uStack_1df;
  undefined1 uStack_1d8;
  undefined8 uStack_1d7;
  undefined8 local_1cf;
  undefined2 local_1b8;
  undefined1 local_1b6;
  undefined8 ****local_1b0;
  undefined8 ****ppppuStack_1a8;
  undefined8 **local_1a0;
  undefined7 uStack_198;
  undefined1 local_191;
  undefined7 uStack_190;
  undefined8 ****local_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 ****local_150;
  undefined8 ****ppppuStack_148;
  char local_140;
  undefined7 uStack_13f;
  size_t sStack_138;
  undefined1 uStack_130;
  undefined7 uStack_12f;
  undefined1 uStack_128;
  undefined7 uStack_127;
  undefined1 uStack_120;
  undefined7 uStack_11f;
  undefined8 local_118;
  undefined8 ****local_110;
  undefined8 ****ppppuStack_108;
  char local_100;
  undefined7 uStack_ff;
  undefined1 uStack_f8;
  undefined7 uStack_f7;
  undefined1 uStack_f0;
  undefined7 uStack_ef;
  char cStack_e8;
  undefined7 uStack_e7;
  undefined2 local_d8;
  undefined1 local_d6;
  
  uVar18 = *(undefined8 *)(param_2 + 8);
  uVar1 = *(undefined8 *)(param_2 + 0x10);
  std::path::Path::_join(&local_300,uVar18,uVar1,&DAT_10130d9c1,6);
  platform::paths::CodexPaths::configured_sqlite_home(&local_2e8,uVar18,uVar1);
  if (*(long *)(param_2 + 0x2d0) == -0x8000000000000000) {
    platform::paths::CodexPaths::resolve_codex_state_db(&local_2d0,uVar18,uVar1);
    pvVar6 = local_2c8;
  }
  else {
    sVar24 = *(size_t *)(param_2 + 0x2e0);
    if ((long)sVar24 < 0) {
      uVar20 = 0;
LAB_100378a80:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(uVar20,sVar24);
    }
    pvVar15 = *(void **)(param_2 + 0x2d8);
    if (sVar24 == 0) {
      pvVar6 = (void *)0x1;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar20 = 1;
      pvVar6 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar24,1);
      if (pvVar6 == (void *)0x0) goto LAB_100378a80;
    }
    local_2d0 = sVar24;
    local_2c8 = pvVar6;
    _memcpy(pvVar6,pvVar15,sVar24);
    local_2c0 = sVar24;
  }
  iVar5 = std::path::Path::_starts_with(pvVar6,local_2c0,local_2f8,uStack_2f0);
  uVar20 = local_2e0;
  lVar22 = local_2e8;
  if (iVar5 == 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    pppppuVar7 = (undefined8 *****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (pppppuVar7 == (undefined8 *****)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    ppppuVar8 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x13,1);
    if (ppppuVar8 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x13);
    }
    *(undefined4 *)((long)ppppuVar8 + 0xf) = 0x6e6f6974;
    ppppuVar8[1] = (undefined8 ***)0x7461636f6c65725f;
    *ppppuVar8 = (undefined8 ***)0x62645f6574617473;
    *pppppuVar7 = (undefined8 ****)0x13;
    pppppuVar7[1] = ppppuVar8;
    pppppuVar7[2] = (undefined8 ****)0x13;
    local_1f0 = (undefined8 *****)0x1;
    local_1e0 = '\x01';
    uStack_1df = 0;
    local_150 = (undefined8 *****)0x0;
    ppppuStack_148 = (undefined8 ****)&SUB_00000008;
    local_140 = '\0';
    uStack_13f = 0;
    ppppuStack_1e8 = pppppuVar7;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar9 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar19 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x2d,1);
    if (puVar19 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x2d);
    }
    puVar19[1] = 0x6c717320a89ce58d;
    *puVar19 = 0xb8e493bae5bbb4e6;
    puVar19[3] = 0xbcef95bde5ae9be7;
    puVar19[2] = 0x90ade5202f657469;
    *(undefined8 *)((long)puVar19 + 0x25) = 0x8690e785b8e6809c;
    *(undefined8 *)((long)puVar19 + 0x1d) = 0xe9a097e68cbcef95;
    uStack_f0 = SUB81(ppppuStack_148,0);
    uStack_ef = (undefined7)((ulong)ppppuStack_148 >> 8);
    uStack_f8 = SUB81(local_150,0);
    uStack_f7 = (undefined7)((ulong)local_150 >> 8);
    param_1[2] = ppppuStack_1e8;
    param_1[1] = local_1f0;
    *puVar9 = 0x2d;
    puVar9[1] = puVar19;
    puVar9[2] = 0x2d;
    param_1[4] = local_150;
    param_1[3] = CONCAT71(uStack_1df,local_1e0);
    param_1[6] = CONCAT71(uStack_13f,local_140);
    param_1[5] = ppppuStack_148;
LAB_10037901c:
    param_1[7] = 1;
    param_1[8] = puVar9;
    param_1[9] = 1;
    *param_1 = 0xb;
    local_110 = local_1f0;
    ppppuStack_108 = ppppuStack_1e8;
    local_100 = local_1e0;
    uStack_ff = uStack_1df;
    cStack_e8 = local_140;
    uStack_e7 = uStack_13f;
  }
  else {
    if (local_2e8 == -0x8000000000000000) {
LAB_100378f1c:
      local_1f0 = (undefined8 *****)0x0;
      ppppuStack_1e8 = (undefined8 ****)&SUB_00000008;
      local_1e0 = '\0';
      uStack_1df = 0;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      pppppuVar7 = (undefined8 *****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
      if (pppppuVar7 == (undefined8 *****)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error(8,0x18);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      ppppuVar8 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x13,1);
      if (ppppuVar8 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x13);
      }
      *(undefined4 *)((long)ppppuVar8 + 0xf) = 0x6e6f6974;
      ppppuVar8[1] = (undefined8 ***)0x7461636f6c65725f;
      *ppppuVar8 = (undefined8 ***)0x62645f6574617473;
      *pppppuVar7 = (undefined8 ****)0x13;
      pppppuVar7[1] = ppppuVar8;
      pppppuVar7[2] = (undefined8 ****)0x13;
      local_150 = (undefined8 *****)0x1;
      local_140 = '\x01';
      uStack_13f = 0;
      ppppuStack_148 = pppppuVar7;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
      if (puVar9 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error(8,0x18);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar19 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x6c,1);
      if (puVar19 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x6c);
      }
      puVar19[9] = 0xbcef93bae5bf84e5;
      puVar19[8] = 0xa4ade5af98e693ba;
      puVar19[0xb] = 0xa883e9a885e59995;
      puVar19[10] = 0xe79dbfe4b2b7e59b;
      *(undefined8 *)((long)puVar19 + 100) = 0x8280e393bae5ae8d;
      *(undefined8 *)((long)puVar19 + 0x5c) = 0xe6b095e6a883e9a8;
      puVar19[1] = 0xe9aea1e78e98e6aa;
      *puVar19 = 0x9ce6207865646f43;
      puVar19[3] = 0x6d6f685f6574696c;
      puVar19[2] = 0x717320aebde78d85;
      puVar19[5] = 0x8e98e681afe895b3;
      puVar19[4] = 0xe6a097e68cbcef65;
      puVar19[7] = 0xe5ae8de6b095e695;
      puVar19[6] = 0xbde5ae9be7b9a0e6;
      *puVar9 = 0x6c;
      puVar9[1] = puVar19;
      puVar9[2] = 0x6c;
      uStack_f0 = SUB81(ppppuStack_148,0);
      uStack_ef = (undefined7)((ulong)ppppuStack_148 >> 8);
      uStack_f8 = SUB81(local_150,0);
      uStack_f7 = (undefined7)((ulong)local_150 >> 8);
      param_1[2] = ppppuStack_1e8;
      param_1[1] = local_1f0;
      param_1[4] = local_150;
      param_1[3] = CONCAT71(uStack_1df,local_1e0);
      param_1[6] = CONCAT71(uStack_13f,local_140);
      param_1[5] = ppppuStack_148;
      goto LAB_10037901c;
    }
    std::path::Path::components(&local_230,local_2e0,uStack_2d8);
    std::path::Path::components(&local_1f0,local_2f8,uStack_2f0);
    ppppuVar4 = ppppuStack_1e8;
    ppppuVar3 = local_1f0;
    ppppuVar2 = ppppuStack_228;
    ppppuVar8 = local_230;
    if (ppppuStack_228 == ppppuStack_1e8) {
      if (((((char)local_1f8 != (char)local_1b8) || (local_1f8._1_1_ != '\x02')) || (local_1b8._1_1_ != '\x02')) ||
         (iVar5 = _memcmp(local_230,local_1f0,(size_t)ppppuStack_228), iVar5 != 0)) goto LAB_100378b74;
    }
    else {
LAB_100378b74:
      if (local_220 != '\x06') {
        local_1b0 = (undefined8 ****)CONCAT17(uStack_218,uStack_21f);
        ppppuStack_1a8 = ppppuStack_217;
        uStack_198 = uStack_207;
        local_1a0 = local_20f;
        local_191 = uStack_200;
        uStack_190 = uStack_1ff;
      }
      sStack_138._1_7_ = SUB87(ppppuStack_1a8,0);
      uStack_130 = (undefined1)((ulong)ppppuStack_1a8 >> 0x38);
      uStack_13f = SUB87(local_1b0,0);
      sStack_138._0_1_ = (undefined1)((ulong)local_1b0 >> 0x38);
      local_150 = ppppuVar8;
      ppppuStack_148 = ppppuVar2;
      local_140 = local_220;
      uStack_127 = uStack_198;
      uStack_12f = SUB87(local_1a0,0);
      uStack_128 = (undefined1)((ulong)local_1a0 >> 0x38);
      uStack_120 = local_191;
      uStack_11f = uStack_190;
      local_118._0_3_ = CONCAT12(local_1f6,local_1f8);
      if (local_1e0 != '\x06') {
        local_180 = (undefined8 ****)CONCAT17(uStack_1d8,uStack_1df);
        uStack_178 = uStack_1d7;
        uStack_170 = local_1cf;
      }
      local_110 = ppppuVar3;
      ppppuStack_108 = ppppuVar4;
      local_100 = local_1e0;
      uStack_f7 = (undefined7)uStack_178;
      uStack_f0 = (undefined1)((ulong)uStack_178 >> 0x38);
      uStack_ff = SUB87(local_180,0);
      uStack_f8 = (undefined1)((ulong)local_180 >> 0x38);
      uStack_ef = (undefined7)uStack_170;
      cStack_e8 = (char)((ulong)uStack_170 >> 0x38);
      local_d8 = local_1b8;
      local_d6 = local_1b6;
      iVar5 = ::core::iter::traits::iterator::Iterator::eq_by(&local_150,&local_110);
      if (iVar5 == 0) goto LAB_100378f1c;
    }
    list_state_dbs_in(&local_2b8,uVar18,uVar1);
    if (local_2a8 == 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      pppppuVar7 = (undefined8 *****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
      if (pppppuVar7 == (undefined8 *****)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error(8,0x18);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      ppppuVar8 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x13,1);
      if (ppppuVar8 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x13);
      }
      *(undefined4 *)((long)ppppuVar8 + 0xf) = 0x6e6f6974;
      ppppuVar8[1] = (undefined8 ***)0x7461636f6c65725f;
      *ppppuVar8 = (undefined8 ***)0x62645f6574617473;
      *pppppuVar7 = (undefined8 ****)0x13;
      pppppuVar7[1] = ppppuVar8;
      pppppuVar7[2] = (undefined8 ****)0x13;
      local_1f0 = (undefined8 *****)0x1;
      local_1e0 = '\x01';
      uStack_1df = 0;
      local_150 = (undefined8 *****)0x0;
      ppppuStack_148 = (undefined8 ****)&SUB_00000008;
      local_140 = '\0';
      uStack_13f = 0;
      ppppuStack_1e8 = pppppuVar7;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
      if (puVar9 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error(8,0x18);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar19 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xf,1);
      if (puVar19 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0xf);
      }
      *puVar19 = 0xbae5a797e6a097e6;
      *(undefined8 *)((long)puVar19 + 7) = 0x9995e78baee693ba;
      *puVar9 = 0xf;
      puVar9[1] = puVar19;
      puVar9[2] = 0xf;
      ppppuStack_108 = ppppuStack_1e8;
      local_110 = local_1f0;
      uStack_f0 = SUB81(ppppuStack_148,0);
      uStack_ef = (undefined7)((ulong)ppppuStack_148 >> 8);
      uStack_f8 = SUB81(local_150,0);
      uStack_f7 = (undefined7)((ulong)local_150 >> 8);
      local_100 = local_1e0;
      uStack_ff = uStack_1df;
      cStack_e8 = local_140;
      uStack_e7 = uStack_13f;
      param_1[2] = ppppuStack_1e8;
      param_1[1] = local_1f0;
      param_1[4] = local_150;
      param_1[3] = CONCAT71(uStack_1df,local_1e0);
      param_1[6] = CONCAT71(uStack_13f,local_140);
      param_1[5] = ppppuStack_148;
      param_1[7] = 1;
      param_1[8] = puVar9;
      param_1[9] = 1;
      *param_1 = 0xb;
    }
    else {
      std::path::Path::_join(&local_2a0,*(undefined8 *)(param_2 + 0x248),*(undefined8 *)(param_2 + 0x250),&DAT_10130b5b0,0x10);
      local_110 = (undefined8 ****)CONCAT53(local_110._3_5_,0x101ff);
      local_330 = ppppuStack_290;
      pppppuVar7 = (undefined8 *****)std::fs::DirBuilder::_create(&local_110,local_298);
      if (pppppuVar7 == (undefined8 *****)0x0) {
        auVar26 = std::time::SystemTime::now();
        local_150 = auVar26._0_8_;
        ppppuStack_148 = (undefined8 ****)CONCAT44(ppppuStack_148._4_4_,auVar26._8_4_);
        std::time::SystemTime::duration_since(&local_110,&local_150,0,0);
        local_1b0 = (undefined8 *****)0x0;
        if ((int)local_110 == 0) {
          local_1b0 = ppppuStack_108;
        }
        lVar13 = local_2b0 + local_2a8 * 0x18;
        local_358 = (undefined8 ***)0x0;
        lVar17 = local_2b0;
LAB_100379234:
        lVar16 = 0;
        local_140 = '\x01';
        uStack_13f = 0;
        sStack_138._0_1_ = 0;
        sStack_138._1_7_ = 0;
        uStack_130 = 0x48;
        uStack_12f = 0x101312f;
        uStack_128 = 4;
        uStack_127 = 0;
        uStack_120 = 0x4c;
        uStack_11f = 0x101312f;
        local_118 = 4;
        pvVar15 = *(void **)(lVar17 + 8);
        pppppuVar7 = *(undefined8 ******)(lVar17 + 0x10);
        lVar17 = lVar17 + 0x18;
LAB_100379260:
        psVar25 = &sStack_138 + lVar16 * 2;
        lVar16 = lVar16 + 1;
        do {
          sVar24 = *psVar25;
          if (sVar24 == 0) {
            std::path::Path::to_path_buf(&local_230,pvVar15);
          }
          else {
            if ((long)pppppuVar7 < 0) {
              uVar18 = 0;
LAB_100379924:
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error(uVar18,local_340);
            }
            pvVar21 = (void *)psVar25[-1];
            if (pppppuVar7 == (undefined8 *****)0x0) {
              local_340 = (undefined8 *****)0x1;
            }
            else {
              __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
              local_340 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(pppppuVar7,1);
              if ((undefined8 *****)local_340 == (undefined8 *****)0x0) {
                uVar18 = 1;
                local_340 = pppppuVar7;
                goto LAB_100379924;
              }
            }
            _memcpy(local_340,pvVar15,(size_t)pppppuVar7);
            local_100 = (char)pppppuVar7;
            uStack_ff = (undefined7)((ulong)pppppuVar7 >> 8);
            local_110 = pppppuVar7;
            ppppuStack_108 = local_340;
            alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_110,pppppuVar7,sVar24,1,1);
            lVar10 = CONCAT71(uStack_ff,local_100);
            _memcpy((undefined1 *)((long)ppppuStack_108 + lVar10),pvVar21,sVar24);
            lVar10 = lVar10 + sVar24;
            local_220 = (char)lVar10;
            uStack_21f = (undefined7)((ulong)lVar10 >> 8);
            ppppuStack_228 = ppppuStack_108;
            local_230 = local_110;
          }
          ppppuVar8 = ppppuStack_228;
          uVar18 = CONCAT71(uStack_21f,local_220);
          std::sys::fs::metadata(&local_110,ppppuStack_228,uVar18);
          if (((ulong)local_110 & 1) == 0) goto LAB_1003793a0;
          if (((ulong)ppppuStack_108 & 3) == 1) {
            puVar19 = (undefined8 *)((long)ppppuStack_108 + -1);
            uVar18 = *puVar19;
            puVar9 = *(undefined8 **)((long)ppppuStack_108 + 7);
            if ((code *)*puVar9 != (code *)0x0) {
              (*(code *)*puVar9)(uVar18);
            }
            if (puVar9[1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar18,puVar9[1],puVar9[2]);
            }
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar19,0x18,8);
          }
          if ((undefined8 *****)local_230 != (undefined8 *****)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar8,local_230,1);
          }
          psVar25 = psVar25 + 2;
          lVar16 = lVar16 + 1;
          if (lVar16 == 4) goto LAB_100379588;
        } while( true );
      }
      local_1f0 = (undefined8 *****)0x0;
      ppppuStack_1e8 = (undefined8 ****)&SUB_00000008;
      local_1e0 = '\0';
      uStack_1df = 0;
      local_230 = pppppuVar7;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      pppppuVar7 = (undefined8 *****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
      if (pppppuVar7 == (undefined8 *****)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error(8,0x18);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      ppppuVar8 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x13,1);
      if (ppppuVar8 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x13);
      }
      *(undefined4 *)((long)ppppuVar8 + 0xf) = 0x6e6f6974;
      ppppuVar8[1] = (undefined8 ***)0x7461636f6c65725f;
      *ppppuVar8 = (undefined8 ***)0x62645f6574617473;
      *pppppuVar7 = (undefined8 ****)0x13;
      pppppuVar7[1] = ppppuVar8;
      pppppuVar7[2] = (undefined8 ****)0x13;
      local_150 = (undefined8 *****)0x1;
      local_140 = '\x01';
      uStack_13f = 0;
      ppppuStack_148 = pppppuVar7;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
      if (puVar9 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error(8,0x18);
      }
      local_110 = &local_230;
      ppppuStack_108 = (undefined8 ****)<std::io::error::Error_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_288,s__1015205f5,&local_110);
      puVar9[1] = uStack_280;
      *puVar9 = local_288;
      puVar9[2] = local_278;
      ppppuStack_108 = ppppuStack_1e8;
      local_110 = local_1f0;
      uStack_f0 = SUB81(ppppuStack_148,0);
      uStack_ef = (undefined7)((ulong)ppppuStack_148 >> 8);
      uStack_f8 = SUB81(local_150,0);
      uStack_f7 = (undefined7)((ulong)local_150 >> 8);
      local_100 = local_1e0;
      uStack_ff = uStack_1df;
      cStack_e8 = local_140;
      uStack_e7 = uStack_13f;
      param_1[2] = ppppuStack_1e8;
      param_1[1] = local_1f0;
      param_1[4] = local_150;
      param_1[3] = CONCAT71(uStack_1df,local_1e0);
      param_1[6] = CONCAT71(uStack_13f,local_140);
      param_1[5] = ppppuStack_148;
      param_1[7] = 1;
      param_1[8] = puVar9;
      param_1[9] = 1;
      *param_1 = 0xb;
      if (((ulong)local_230 & 3) == 1) {
        puVar9 = (undefined8 *)((long)local_230 + -1);
        uVar18 = *puVar9;
        puVar19 = *(undefined8 **)((long)local_230 + 7);
        if ((code *)*puVar19 != (code *)0x0) {
          (*(code *)*puVar19)(uVar18);
        }
        if (puVar19[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar18,puVar19[1],puVar19[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar9,0x18,8);
      }
      if (local_2a0 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_298,local_2a0,1);
      }
      puVar9 = (undefined8 *)(local_2b0 + 8);
      lVar22 = local_2a8;
      do {
        if (puVar9[-1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar9,puVar9[-1],1);
        }
        puVar9 = puVar9 + 3;
        lVar22 = lVar22 + -1;
      } while (lVar22 != 0);
    }
    if (local_2b8 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2b0,local_2b8 * 0x18,8);
    }
  }
  if (local_2d0 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar6,local_2d0,1);
  }
  if (local_2e8 == -0x8000000000000000) goto LAB_100379064;
  goto joined_r0x000100379bcc;
LAB_1003793a0:
  lVar10 = std::path::Path::file_name(ppppuVar8,uVar18);
  if (lVar10 == 0) {
    local_180 = (undefined8 ****)&DAT_10130f8de;
    uStack_178 = 0xc;
  }
  else {
    ::core::str::converts::from_utf8(&local_110);
    local_180 = (undefined8 ****)&DAT_10130f8de;
    if ((int)local_110 == 0) {
      local_180 = ppppuStack_108;
    }
    uStack_178 = 0xc;
    if ((int)local_110 == 0) {
      uStack_178 = CONCAT71(uStack_ff,local_100);
    }
  }
  local_110 = &local_1b0;
  ppppuStack_108 = (undefined8 ****)usize>::fmt;
  local_100 = (char)&local_180;
  uStack_ff = (undefined7)((ulong)&local_180 >> 8);
  uStack_f8 = 0xc0;
  uStack_f7 = 0x1007dd6;
  alloc::fmt::format::format_inner(&local_1f0,s___10152210f,&local_110);
  ppppuVar2 = ppppuStack_1e8;
  std::path::Path::_join(&local_110,local_298,ppppuStack_290,ppppuStack_1e8,CONCAT71(uStack_1df,local_1e0));
  if ((undefined8 *****)local_1f0 != (undefined8 *****)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar2,local_1f0,1);
  }
  ppppuVar2 = ppppuStack_108;
  auVar27 = std::sys::fs::copy(ppppuVar8,uVar18,ppppuStack_108,CONCAT71(uStack_ff,local_100));
  uVar11 = auVar27._8_8_;
  if (auVar27._0_8_ == 0) {
    uVar11 = std::sys::fs::remove_file(ppppuVar8,uVar18);
    if (uVar11 == 0) {
      if (sVar24 == 0) {
        local_358 = (undefined8 **)((long)local_358 + 1);
      }
    }
    else if ((uVar11 & 3) == 1) {
      uVar18 = *(undefined8 *)(uVar11 - 1);
      puVar9 = *(undefined8 **)(uVar11 + 7);
      if ((code *)*puVar9 != (code *)0x0) {
        (*(code *)*puVar9)(uVar18);
      }
      if (puVar9[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar18,puVar9[1],puVar9[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(uVar11 - 1),0x18,8);
    }
  }
  else if ((uVar11 & 3) == 1) {
    uVar18 = *(undefined8 *)(uVar11 - 1);
    puVar9 = *(undefined8 **)(uVar11 + 7);
    if ((code *)*puVar9 != (code *)0x0) {
      (*(code *)*puVar9)(uVar18);
    }
    if (puVar9[1] != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar18,puVar9[1],puVar9[2]);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(uVar11 - 1),0x18,8);
  }
  if ((undefined8 *****)local_110 != (undefined8 *****)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar2,local_110,1);
  }
  if ((undefined8 *****)local_230 != (undefined8 *****)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar8,local_230,1);
  }
  if (lVar16 == 3) goto LAB_100379588;
  goto LAB_100379260;
LAB_100379588:
  if (lVar17 == lVar13) goto code_r0x0001003795b0;
  goto LAB_100379234;
code_r0x0001003795b0:
  local_270 = local_358;
  local_230 = (undefined8 *****)0x0;
  ppppuStack_228 = (undefined8 ****)&SUB_00000008;
  local_220 = '\0';
  uStack_21f = 0;
  local_328 = 1;
  local_320 = local_2b0;
  do {
    lVar17 = 0;
    local_140 = '\x01';
    uStack_13f = 0;
    sStack_138._0_1_ = 0;
    sStack_138._1_7_ = 0;
    uStack_130 = 0x48;
    uStack_12f = 0x101312f;
    uStack_128 = 4;
    uStack_127 = 0;
    uStack_120 = 0x4c;
    uStack_11f = 0x101312f;
    local_118 = 4;
    pvVar15 = *(void **)(local_320 + 8);
    pppppuVar7 = *(undefined8 ******)(local_320 + 0x10);
    local_320 = local_320 + 0x18;
    do {
      sVar24 = *(size_t *)((long)&sStack_138 + lVar17);
      if (sVar24 == 0) {
        std::path::Path::to_path_buf(&local_1f0,pvVar15,pppppuVar7);
      }
      else {
        if ((long)pppppuVar7 < 0) {
          local_328 = 0;
LAB_10037997c:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(local_328,local_330);
        }
        pvVar21 = *(void **)(&local_140 + lVar17);
        if (pppppuVar7 == (undefined8 *****)0x0) {
          local_330 = (undefined8 *****)0x1;
          pppppuVar12 = (undefined8 *****)local_330;
        }
        else {
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          pppppuVar12 = (undefined8 *****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(pppppuVar7,1);
          local_330 = pppppuVar7;
          if (pppppuVar12 == (undefined8 *****)0x0) goto LAB_10037997c;
        }
        local_330 = pppppuVar12;
        _memcpy(local_330,pvVar15,(size_t)pppppuVar7);
        ppppuStack_108 = local_330;
        local_100 = (char)pppppuVar7;
        uStack_ff = (undefined7)((ulong)pppppuVar7 >> 8);
        local_110 = pppppuVar7;
        alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_110,pppppuVar7,sVar24,1,1);
        lVar16 = CONCAT71(uStack_ff,local_100);
        _memcpy((undefined1 *)((long)ppppuStack_108 + lVar16),pvVar21,sVar24);
        lVar16 = lVar16 + sVar24;
        local_1e0 = (char)lVar16;
        uStack_1df = (undefined7)((ulong)lVar16 >> 8);
        ppppuStack_1e8 = ppppuStack_108;
        local_1f0 = local_110;
      }
      ppppuVar3 = ppppuStack_1e8;
      ppppuVar8 = (undefined8 ****)CONCAT71(uStack_1df,local_1e0);
      std::sys::fs::metadata(&local_110,ppppuStack_1e8,ppppuVar8);
      ppppuVar2 = local_1f0;
      if (((ulong)local_110 & 1) == 0) {
        pppppuVar12 = (undefined8 *****)CONCAT71(uStack_21f,local_220);
        if (pppppuVar12 == (undefined8 *****)local_230) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_230);
        }
        pppppuVar14 = (undefined8 *****)(ppppuStack_228 + (long)pppppuVar12 * 3);
        *pppppuVar14 = ppppuVar2;
        pppppuVar14[1] = ppppuVar3;
        pppppuVar14[2] = ppppuVar8;
        local_220 = (char)(undefined1 *)((long)pppppuVar12 + 1);
        uStack_21f = (undefined7)((ulong)((long)pppppuVar12 + 1) >> 8);
      }
      else {
        if (((ulong)ppppuStack_108 & 3) == 1) {
          puVar19 = (undefined8 *)((long)ppppuStack_108 + -1);
          uVar18 = *puVar19;
          puVar9 = *(undefined8 **)((long)ppppuStack_108 + 7);
          if ((code *)*puVar9 != (code *)0x0) {
            (*(code *)*puVar9)(uVar18);
          }
          if (puVar9[1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar18,puVar9[1],puVar9[2]);
          }
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar19,0x18,8);
        }
        if ((undefined8 *****)local_1f0 != (undefined8 *****)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar3,local_1f0,1);
        }
      }
      pppppuVar12 = (undefined8 *****)ppppuStack_228;
      lVar17 = lVar17 + 0x10;
    } while (lVar17 != 0x30);
  } while (local_320 != lVar13);
  ppppuStack_1a8 = ppppuStack_228;
  local_1b0 = local_230;
  pppuVar23 = (undefined8 ***)CONCAT71(uStack_21f,local_220);
  local_1a0 = pppuVar23;
  if (pppuVar23 == (undefined8 ***)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    pppppuVar7 = (undefined8 *****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (pppppuVar7 == (undefined8 *****)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    ppppuVar8 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x13,1);
    if (ppppuVar8 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x13);
    }
    *(undefined4 *)((long)ppppuVar8 + 0xf) = 0x6e6f6974;
    ppppuVar8[1] = (undefined8 ***)0x7461636f6c65725f;
    *ppppuVar8 = (undefined8 ***)0x62645f6574617473;
    *pppppuVar7 = (undefined8 ****)0x13;
    pppppuVar7[1] = ppppuVar8;
    pppppuVar7[2] = (undefined8 ****)0x13;
    local_1f0 = (undefined8 *****)0x1;
    local_1e0 = '\x01';
    uStack_1df = 0;
    local_150 = (undefined8 *****)0x0;
    ppppuStack_148 = (undefined8 ****)&SUB_00000008;
    local_140 = '\0';
    uStack_13f = 0;
    ppppuStack_1e8 = pppppuVar7;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar9 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    local_110 = (undefined8 ****)&local_270;
    ppppuStack_108 = (undefined8 ****)usize>::fmt;
    alloc::fmt::format::format_inner(&local_268,s___relay_state_db_backups__101520613,&local_110);
    puVar9[1] = uStack_260;
    *puVar9 = local_268;
    puVar9[2] = local_258;
    ppppuStack_108 = ppppuStack_1e8;
    local_110 = local_1f0;
    uStack_f0 = SUB81(ppppuStack_148,0);
    uStack_ef = (undefined7)((ulong)ppppuStack_148 >> 8);
    uStack_f8 = SUB81(local_150,0);
    uStack_f7 = (undefined7)((ulong)local_150 >> 8);
    local_100 = local_1e0;
    uStack_ff = uStack_1df;
    cStack_e8 = local_140;
    uStack_e7 = uStack_13f;
    param_1[2] = ppppuStack_1e8;
    param_1[1] = local_1f0;
    param_1[4] = local_150;
    param_1[3] = CONCAT71(uStack_1df,local_1e0);
    param_1[6] = CONCAT71(uStack_13f,local_140);
    param_1[5] = ppppuStack_148;
    param_1[7] = 1;
    param_1[8] = puVar9;
    param_1[9] = 1;
    *param_1 = 0xb;
    pppppuVar12 = (undefined8 *****)ppppuStack_1a8;
  }
  else {
    <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_180,ppppuStack_228,ppppuStack_228 + (long)pppuVar23 * 3)
    ;
    local_230 = (undefined8 *****)0x0;
    ppppuStack_228 = (undefined8 ****)&SUB_00000008;
    local_220 = '\0';
    uStack_21f = 0;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    pppppuVar7 = (undefined8 *****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (pppppuVar7 == (undefined8 *****)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    ppppuVar8 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x13,1);
    if (ppppuVar8 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x13);
    }
    *(undefined4 *)((long)ppppuVar8 + 0xf) = 0x6e6f6974;
    ppppuVar8[1] = (undefined8 ***)0x7461636f6c65725f;
    *ppppuVar8 = (undefined8 ***)0x62645f6574617473;
    *pppppuVar7 = (undefined8 ****)0x13;
    pppppuVar7[1] = ppppuVar8;
    pppppuVar7[2] = (undefined8 ****)0x13;
    local_1f0 = (undefined8 *****)0x1;
    local_1e0 = '\x01';
    uStack_1df = 0;
    ppppuStack_1e8 = pppppuVar7;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar9 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    local_238 = pppuVar23;
    alloc::str::join_generic_copy(&local_110,uStack_178,uStack_170,&DAT_10130d4af,2);
    ppppuStack_148 = ppppuStack_108;
    local_150 = local_110;
    local_140 = local_100;
    uStack_13f = uStack_ff;
    local_110 = (undefined8 ****)&local_238;
    ppppuStack_108 = (undefined8 ****)usize>::fmt;
    local_100 = (char)&local_150;
    uStack_ff = (undefined7)((ulong)&local_150 >> 8);
    uStack_f8 = 0xf8;
    uStack_f7 = 0x1003edc;
    alloc::fmt::format::format_inner(&local_250,s_1_t_Codex_computer_use___node_re_10152066a,&local_110);
    if ((undefined8 *****)local_150 != (undefined8 *****)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuStack_148,local_150,1);
    }
    puVar9[1] = uStack_248;
    *puVar9 = local_250;
    puVar9[2] = local_240;
    ppppuStack_108 = ppppuStack_228;
    local_110 = local_230;
    uStack_f0 = SUB81(ppppuStack_1e8,0);
    uStack_ef = (undefined7)((ulong)ppppuStack_1e8 >> 8);
    uStack_f8 = SUB81(local_1f0,0);
    uStack_f7 = (undefined7)((ulong)local_1f0 >> 8);
    local_100 = local_220;
    uStack_ff = uStack_21f;
    cStack_e8 = local_1e0;
    uStack_e7 = uStack_1df;
    param_1[2] = ppppuStack_228;
    param_1[1] = local_230;
    param_1[4] = local_1f0;
    param_1[3] = CONCAT71(uStack_21f,local_220);
    param_1[6] = CONCAT71(uStack_1df,local_1e0);
    param_1[5] = ppppuStack_1e8;
    param_1[7] = 1;
    param_1[8] = puVar9;
    param_1[9] = 1;
    *param_1 = 0xb;
    ::core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>(&local_180);
    pppppuVar7 = pppppuVar12 + 1;
    do {
      if (pppppuVar7[-1] != (undefined8 ****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppuVar7,pppppuVar7[-1],1);
      }
      pppppuVar7 = pppppuVar7 + 3;
      pppuVar23 = (undefined8 ***)((long)pppuVar23 + -1);
    } while (pppuVar23 != (undefined8 ***)0x0);
  }
  if ((undefined8 *****)local_1b0 != (undefined8 *****)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppuVar12,(long)local_1b0 * 0x18,8);
  }
  if (local_2a0 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_298,local_2a0,1);
  }
  puVar9 = (undefined8 *)(local_2b0 + 8);
  lVar17 = local_2a8;
  do {
    if (puVar9[-1] != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar9,puVar9[-1],1);
    }
    puVar9 = puVar9 + 3;
    lVar17 = lVar17 + -1;
  } while (lVar17 != 0);
  if (local_2b8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2b0,local_2b8 * 0x18,8);
  }
  local_2e8 = lVar22;
  local_2e0 = uVar20;
  if (local_2d0 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar6,local_2d0,1);
  }
joined_r0x000100379bcc:
  if (local_2e8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2e0,local_2e8,1);
  }
LAB_100379064:
  if (local_300 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2f8,local_300,1);
  }
  return;
}

 (GhidraScript)  
===END=== (GhidraScript)  
