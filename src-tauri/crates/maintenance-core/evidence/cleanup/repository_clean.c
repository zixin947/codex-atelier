/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* clean @ 10082600c requested 10082600c */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::repository::Repository::clean */

ulong codexmate_lib::core::repository::Repository::clean(undefined8 *param_1,long param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  ulong uVar3;
  int iVar4;
  void *pvVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  uint uVar8;
  int iVar9;
  ulong uVar10;
  undefined1 (*pauVar11) [16];
  undefined8 *puVar12;
  byte ******ppppppbVar13;
  code *pcVar14;
  long lVar15;
  byte *******pppppppbVar16;
  long lVar17;
  byte *******pppppppbVar18;
  undefined1 auVar19 [12];
  int local_2c4;
  undefined1 local_2c0 [8];
  code *pcStack_2b8;
  byte *******local_2b0;
  long local_2a8;
  void *local_2a0;
  size_t sStack_298;
  undefined8 uStack_278;
  byte *******local_260;
  long lStack_258;
  undefined8 uStack_250;
  long local_248;
  undefined1 local_240 [16];
  code *local_230;
  byte *******pppppppbStack_228;
  long local_220;
  void *local_218;
  size_t local_210;
  undefined8 local_200;
  undefined8 *puStack_1f8;
  undefined8 local_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined *local_1d0;
  void *pvStack_1c8;
  undefined *local_1c0;
  undefined8 uStack_1b8;
  undefined **local_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  byte *******local_160;
  code *pcStack_158;
  byte *******local_150;
  undefined4 local_13c;
  undefined4 local_138;
  int local_134;
  byte *******local_130;
  undefined8 uStack_128;
  byte *******local_120;
  long lStack_118;
  void *local_110;
  size_t sStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  byte *******local_d0;
  int iStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  int iStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 local_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 local_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 local_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;

  std::path::Path::_join
            (&local_1d0,*(undefined8 *)(param_2 + 200),*(undefined8 *)(param_2 + 0xd0),
             ".account-domain.lockaccount state.oauth-refresh.lockaccount.jsonaccount credential.account-transition.lockaccount transition",0x14);
  pvVar5 = pvStack_1c8;
  uVar10 = account_coordination::FileLock::acquire
                     (&local_d0,pvStack_1c8,local_1c0,"account state.oauth-refresh.lockaccount.jsonaccount credential.account-transition.lockaccount transition"
                      ,0xd);
  local_2c4 = iStack_c8;
  pppppppbVar16 = local_d0;
  if (local_d0 != (byte *******)&DAT_0000000b) {
    sStack_108 = CONCAT44(uStack_98,uStack_9c);
    local_110 = (void *)CONCAT44(uStack_a0,local_a4);
    uStack_f8 = CONCAT44(uStack_88,uStack_8c);
    uStack_100 = CONCAT44(uStack_90,local_94);
    uStack_e8 = CONCAT44(uStack_78,uStack_7c);
    local_f0 = CONCAT44(uStack_80,local_84);
    local_e0 = CONCAT44(local_e0._4_4_,uStack_74);
    uStack_128 = (code *)CONCAT44(iStack_b8,uStack_bc);
    local_130 = (byte *******)CONCAT44(uStack_c0,uStack_c4);
    lStack_118 = CONCAT44(uStack_a8,uStack_ac);
    local_120 = (byte *******)CONCAT44(uStack_b0,uStack_b4);
  }
  if (local_1d0 != (undefined *)0x0) {
    uVar10 = __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar5,local_1d0,1);
  }
  if (pppppppbVar16 != (byte *******)&DAT_0000000b) {
    *(size_t *)((long)param_1 + 0x3c) = sStack_108;
    *(void **)((long)param_1 + 0x34) = local_110;
    *(undefined8 *)((long)param_1 + 0x4c) = uStack_f8;
    *(undefined8 *)((long)param_1 + 0x44) = uStack_100;
    *(undefined8 *)((long)param_1 + 0x5c) = uStack_e8;
    *(undefined8 *)((long)param_1 + 0x54) = local_f0;
    *(code **)((long)param_1 + 0x1c) = uStack_128;
    *(byte ********)((long)param_1 + 0x14) = local_130;
    *(undefined4 *)((long)param_1 + 100) = (undefined4)local_e0;
    *(long *)((long)param_1 + 0x2c) = lStack_118;
    *(byte ********)((long)param_1 + 0x24) = local_120;
    *(int *)(param_1 + 2) = local_2c4;
    *param_1 = 1;
    param_1[1] = pppppppbVar16;
    return uVar10;
  }
  load_registry_for_mutation(&local_d0,*(undefined8 *)(param_2 + 0xe0),*(undefined8 *)(param_2 + 0xe8));
  iVar4 = iStack_b8;
  sStack_298 = CONCAT44(uStack_9c,uStack_a0);
  local_2a0 = (void *)CONCAT44(local_a4,uStack_a8);
  uStack_f8 = CONCAT44(uStack_8c,uStack_90);
  uStack_100 = CONCAT44(local_94,uStack_98);
  uStack_e8 = CONCAT44(uStack_7c,uStack_80);
  local_f0 = CONCAT44(local_84,uStack_88);
  local_e0 = CONCAT44(uStack_74,uStack_78);
  pcStack_2b8 = (code *)CONCAT44(uStack_bc,uStack_c0);
  local_130 = (byte *******)CONCAT44(uStack_c4,iStack_c8);
  lStack_118 = CONCAT44(uStack_ac,uStack_b0);
  local_2b0 = (byte *******)CONCAT44(uStack_b4,iStack_b8);
  uStack_128 = pcStack_2b8;
  local_120 = local_2b0;
  if (local_d0 == (byte *******)&DAT_0000000b) {
    local_2a8 = lStack_118;
    local_110 = local_2a0;
    sStack_108 = sStack_298;
    alloc::vec::Vec<T,A>::retain(local_2c0,param_2);
    pppppppbVar16 = local_2b0;
    pauVar11 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
    if (pauVar11[1][0] == '\x01') {
      local_240 = *pauVar11;
    }
    else {
      local_240 = std::sys::random::hashmap_random_keys();
      *pauVar11 = local_240;
      pauVar11[1][0] = 1;
    }
    *(long *)*pauVar11 = local_240._0_8_ + 1;
    iStack_c8 = 0;
    uStack_c4 = 0;
    local_d0 = (byte *******)&_anon_973b090bee24c06fdb949486dbea4242_10;
    iStack_b8 = 0;
    uStack_b4 = 0;
    uStack_c0 = 0;
    uStack_bc = 0;
    uStack_b0 = local_240._0_4_;
    uStack_ac = local_240._4_4_;
    uStack_a8 = local_240._8_4_;
    local_a4 = local_240._12_4_;
    if (pppppppbVar16 == (byte *******)0x0) {
      lStack_258 = 0;
      local_248 = 0;
      uStack_250 = 0;
      local_260 = (byte *******)&_anon_973b090bee24c06fdb949486dbea4242_10;
      lVar15 = local_2a8;
joined_r0x000100826950:
      if ((lVar15 != -0x8000000000000000) && (lVar15 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2a0,lVar15,1);
      }
    }
    else {
      hashbrown::raw::RawTable<T,A>::reserve_rehash(&local_d0,pppppppbVar16,&uStack_b0,1);
      pcVar14 = pcStack_2b8 + 0xc0;
      pppppppbVar18 = pppppppbVar16;
      do {
        <alloc::string::String_as_core::clone::Clone>::clone(&local_130,pcVar14);
        hashbrown::map::HashMap<K,V,S,A>::insert(&local_d0,&local_130);
        pvVar5 = local_2a0;
        lVar15 = local_2a8;
        pcVar14 = pcVar14 + 0x1a8;
        pppppppbVar18 = (byte *******)((long)pppppppbVar18 + -1);
      } while (pppppppbVar18 != (byte *******)0x0);
      lStack_258 = CONCAT44(uStack_c4,iStack_c8);
      local_260 = local_d0;
      local_248 = CONCAT44(uStack_b4,iStack_b8);
      uStack_250 = CONCAT44(uStack_bc,uStack_c0);
      local_240._8_8_ = CONCAT44(local_a4,uStack_a8);
      local_240._0_8_ = CONCAT44(uStack_ac,uStack_b0);
      if (local_2a8 != -0x8000000000000000) {
        lVar17 = (long)pppppppbVar16 * 0x1a8;
        pcVar14 = pcStack_2b8 + 0xd0;
        do {
          if (*(size_t *)pcVar14 == sStack_298) {
            iVar9 = _memcmp(*(void **)(pcVar14 + -8),pvVar5,sStack_298);
            if (iVar9 == 0) goto LAB_1008261bc;
          }
          pcVar14 = pcVar14 + 0x1a8;
          lVar17 = lVar17 + -0x1a8;
        } while (lVar17 != 0);
        goto joined_r0x000100826950;
      }
    }
    local_2a8 = -0x8000000000000000;
LAB_1008261bc:
    auVar19 = std::time::SystemTime::now();
    local_130 = auVar19._0_8_;
    uStack_128 = (code *)CONCAT44(uStack_128._4_4_,auVar19._8_4_);
    std::time::SystemTime::duration_since(&local_d0,&local_130,0,0);
    uStack_278 = 0;
    if ((int)local_d0 == 0) {
      uStack_278 = CONCAT44(uStack_c4,iStack_c8);
    }
    persist_registry(&local_d0,param_2,local_2c0,0);
    lVar15 = lStack_258;
    if (local_d0 == (byte *******)&DAT_0000000b) {
      uVar1 = *(undefined8 *)(param_2 + 0x1a0);
      uVar2 = *(undefined8 *)(param_2 + 0x1a8);
      quota_store::load(&local_130,uVar1,uVar2);
      if (local_130 == (byte *******)&DAT_0000000b) {
        pppppppbStack_228 = local_120;
        local_230 = uStack_128;
        local_218 = local_110;
        local_220 = lStack_118;
        local_210 = sStack_108;
      }
      else {
        uStack_a8 = (undefined4)sStack_108;
        local_a4 = (undefined4)(sStack_108 >> 0x20);
        uStack_b0 = SUB84(local_110,0);
        uStack_ac = (undefined4)((ulong)local_110 >> 0x20);
        uStack_98 = (undefined4)uStack_f8;
        local_94 = (undefined4)((ulong)uStack_f8 >> 0x20);
        uStack_a0 = (undefined4)uStack_100;
        uStack_9c = (undefined4)((ulong)uStack_100 >> 0x20);
        uStack_88 = (undefined4)uStack_e8;
        local_84 = (undefined4)((ulong)uStack_e8 >> 0x20);
        uStack_90 = (undefined4)local_f0;
        uStack_8c = (undefined4)((ulong)local_f0 >> 0x20);
        uStack_78 = (undefined4)uStack_d8;
        uStack_74 = (undefined4)((ulong)uStack_d8 >> 0x20);
        uStack_80 = (undefined4)local_e0;
        uStack_7c = (undefined4)((ulong)local_e0 >> 0x20);
        iStack_c8 = (int)uStack_128;
        uStack_c4 = (undefined4)((ulong)uStack_128 >> 0x20);
        local_d0 = local_130;
        iStack_b8 = (int)lStack_118;
        uStack_b4 = (undefined4)((ulong)lStack_118 >> 0x20);
        uStack_c0 = SUB84(local_120,0);
        uStack_bc = (undefined4)((ulong)local_120 >> 0x20);
        local_210 = CONCAT44(local_210._4_4_,3);
        local_230 = (code *)0x0;
        pppppppbStack_228 = (byte *******)&SUB_00000008;
        local_220 = 0;
        local_218 = (void *)0x0;
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_d0);
      }
      lVar15 = local_220;
      alloc::vec::Vec<T,A>::retain(&local_230,&local_260);
      lVar17 = -0x8000000000000000;
      local_200 = 0x8000000000000000;
      if (local_220 != lVar15) {
        auVar19 = std::time::SystemTime::now();
        local_130 = auVar19._0_8_;
        uStack_128 = (code *)CONCAT44(uStack_128._4_4_,auVar19._8_4_);
        std::time::SystemTime::duration_since(&local_d0,&local_130,0,0);
        local_218 = (void *)0x0;
        if ((int)local_d0 == 0) {
          local_218 = (void *)CONCAT44(uStack_c4,iStack_c8);
        }
        quota_store::save(&local_130,uVar1,uVar2,&local_230);
        if (local_130 != (byte *******)&DAT_0000000b) {
          uStack_a8 = (undefined4)sStack_108;
          local_a4 = (undefined4)(sStack_108 >> 0x20);
          uStack_b0 = SUB84(local_110,0);
          uStack_ac = (undefined4)((ulong)local_110 >> 0x20);
          uStack_98 = (undefined4)uStack_f8;
          local_94 = (undefined4)((ulong)uStack_f8 >> 0x20);
          uStack_a0 = (undefined4)uStack_100;
          uStack_9c = (undefined4)((ulong)uStack_100 >> 0x20);
          uStack_88 = (undefined4)uStack_e8;
          local_84 = (undefined4)((ulong)uStack_e8 >> 0x20);
          uStack_90 = (undefined4)local_f0;
          uStack_8c = (undefined4)((ulong)local_f0 >> 0x20);
          uStack_78 = (undefined4)uStack_d8;
          uStack_74 = (undefined4)((ulong)uStack_d8 >> 0x20);
          uStack_80 = (undefined4)local_e0;
          uStack_7c = (undefined4)((ulong)local_e0 >> 0x20);
          iStack_c8 = (int)uStack_128;
          uStack_c4 = (undefined4)((ulong)uStack_128 >> 0x20);
          local_d0 = local_130;
          iStack_b8 = (int)lStack_118;
          uStack_b4 = (undefined4)((ulong)lStack_118 >> 0x20);
          uStack_c0 = SUB84(local_120,0);
          uStack_bc = (undefined4)((ulong)local_120 >> 0x20);
          if (1 < log::MAX_LOG_LEVEL_FILTER) {
            local_160 = (byte *******)&local_d0;
            pcStack_158 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
            local_1d0 = &DAT_1013af968;
            pvStack_1c8 = (void *)0x1f;
            local_1c0 = &DAT_1013af968;
            uStack_1b8 = 0x1f;
            local_1b0 = &PTR_s_src_core_repository_rs_10182c8b0;
            log::__private_api::log(s___AiMaMi__accounts__quota_store_w_10152cb9c,&local_160,2,&local_1d0);
          }
          uStack_1a8 = CONCAT44(uStack_9c,uStack_a0);
          local_1b0 = (undefined **)CONCAT44(local_a4,uStack_a8);
          uStack_198 = CONCAT44(uStack_8c,uStack_90);
          uStack_1a0 = CONCAT44(local_94,uStack_98);
          uStack_188 = CONCAT44(uStack_7c,uStack_80);
          local_190 = CONCAT44(local_84,uStack_88);
          local_180 = CONCAT44(uStack_74,uStack_78);
          pvStack_1c8 = (void *)CONCAT44(uStack_bc,uStack_c0);
          local_1d0 = (undefined *)CONCAT44(uStack_c4,iStack_c8);
          uStack_1b8 = CONCAT44(uStack_ac,uStack_b0);
          local_1c0 = (undefined *)CONCAT44(uStack_b4,iStack_b8);
          if (local_d0 != (byte *******)&DAT_0000000b) {
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            puVar12 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1e,1);
            if (puVar12 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error(1,0x1e);
            }
            puVar12[1] = 0x41454c435f454843;
            *puVar12 = 0x41435f41544f5551;
            *(undefined8 *)((long)puVar12 + 0x16) = 0x4554454c504d4f43;
            *(undefined8 *)((long)puVar12 + 0xe) = 0x4e495f50554e4145;
            local_130 = (byte *******)&local_d0;
            uStack_128 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&local_178,s_PAccount_cleanup_completed__but_t_10152d598,&local_130);
            uStack_1e0 = uStack_170;
            local_1e8 = local_178;
            local_1d8 = local_168;
            local_200 = 0x1e;
            local_1f0 = 0x1e;
            puStack_1f8 = puVar12;
            ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_d0);
            lVar17 = 0x1e;
          }
        }
      }
      remove_account_backup_files(&local_d0,*(undefined8 *)(param_2 + 0x110),*(undefined8 *)(param_2 + 0x118),&DAT_1013b0226,0x1e,&DAT_1013b0244,0xe);
      uVar6 = (int)local_d0;
      pcStack_158 = (code *)CONCAT44(uStack_bc,uStack_c0);
      local_160 = (byte *******)CONCAT44(uStack_c4,iStack_c8);
      local_150 = (byte *******)CONCAT44(uStack_b4,iStack_b8);
      remove_account_backup_files(&local_d0,*(undefined8 *)(param_2 + 0x128),*(undefined8 *)(param_2 + 0x130),&DAT_1013b0252,0x22,&DAT_1013af718,8);
      uVar7 = (int)local_d0;
      pvVar5 = (void *)CONCAT44(uStack_bc,uStack_c0);
      local_1d0 = (undefined *)CONCAT44(uStack_c4,iStack_c8);
      uVar10 = CONCAT44(uStack_b4,iStack_b8);
      pvStack_1c8 = pvVar5;
      local_1c0 = (undefined *)uVar10;
      if ((ulong)((long)local_160 - (long)local_150) < uVar10) {
        alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_160,local_150,uVar10,8,0x30);
      }
      _memcpy(pcStack_158 + (long)local_150 * 0x30,pvVar5,uVar10 * 0x30);
      pppppppbVar16 = (byte *******)((long)local_150 + uVar10);
      local_1c0 = (undefined *)0x0;
      local_150 = pppppppbVar16;
      if (lVar17 != -0x8000000000000000) {
        if (pppppppbVar16 == local_160) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_160);
        }
        pcVar14 = pcStack_158 + (long)pppppppbVar16 * 0x30;
        *(undefined8 **)(pcVar14 + 8) = puStack_1f8;
        *(undefined8 *)pcVar14 = local_200;
        *(undefined8 *)(pcVar14 + 0x18) = local_1e8;
        *(undefined8 *)(pcVar14 + 0x10) = local_1f0;
        *(undefined8 *)(pcVar14 + 0x28) = local_1d8;
        *(undefined8 *)(pcVar14 + 0x20) = uStack_1e0;
        local_150 = (byte *******)((long)pppppppbVar16 + 1);
      }
      local_134 = iVar4 - (int)local_2b0;
      local_13c = uVar6;
      local_138 = uVar7;
      uStack_128 = pcStack_158;
      local_130 = local_160;
      local_120 = local_150;
      models::CoreEnvelope<T>::ok_with_warnings(&local_d0,&local_13c,&local_130);
      param_1[6] = CONCAT44(local_a4,uStack_a8);
      param_1[5] = CONCAT44(uStack_ac,uStack_b0);
      param_1[8] = CONCAT44(local_94,uStack_98);
      param_1[7] = CONCAT44(uStack_9c,uStack_a0);
      param_1[10] = CONCAT44(local_84,uStack_88);
      param_1[9] = CONCAT44(uStack_8c,uStack_90);
      param_1[0xc] = CONCAT44(uStack_74,uStack_78);
      param_1[0xb] = CONCAT44(uStack_7c,uStack_80);
      param_1[2] = CONCAT44(uStack_c4,iStack_c8);
      param_1[1] = local_d0;
      param_1[4] = CONCAT44(uStack_b4,iStack_b8);
      param_1[3] = CONCAT44(uStack_bc,uStack_c0);
      *param_1 = 0;
      if (local_1d0 != (undefined *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar5,(long)local_1d0 * 0x30,8);
      }
      if (local_220 != 0) {
        pppppppbVar16 = pppppppbStack_228 + 0x13;
        lVar15 = local_220;
        do {
          if (pppppppbVar16[-1] != (byte ******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppbVar16,pppppppbVar16[-1],1);
          }
          if ((pppppppbVar16[-7] != (byte ******)0x2) &&
             (ppppppbVar13 = pppppppbVar16[-5], ppppppbVar13 != (byte ******)0x8000000000000000 && ppppppbVar13 != (byte ******)0x0)) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppbVar16[-4],ppppppbVar13,1);
          }
          pppppppbVar16 = pppppppbVar16 + 0x18;
          lVar15 = lVar15 + -1;
        } while (lVar15 != 0);
      }
      if (local_230 != (code *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppbStack_228,(long)local_230 * 0xc0,8);
      }
      lVar15 = lStack_258;
      if (lStack_258 != 0) {
        if (local_248 != 0) {
          pppppppbVar18 = local_260 + 1;
          ppppppbVar13 = *local_260;
          uVar10 = CONCAT17(-(-1 < (long)ppppppbVar13),
                            CONCAT16(-(-1 < (char)((ulong)ppppppbVar13 >> 0x30)),
                                     CONCAT15(-(-1 < (char)((ulong)ppppppbVar13 >> 0x28)),
                                              CONCAT14(-(-1 < (char)((ulong)ppppppbVar13 >> 0x20)),
                                                       CONCAT13(-(-1 < (char)((ulong)ppppppbVar13 >> 0x18)),
                                                                CONCAT12(-(-1 < (char)((ulong)ppppppbVar13 >> 0x10)),
                                                                         CONCAT11(-(-1 < (char)((ulong)ppppppbVar13 >> 8)),-(-1 < (char)ppppppbVar13)))))))) &
                   0x8080808080808080;
          pppppppbVar16 = local_260;
          lVar17 = local_248;
          do {
            while (uVar10 == 0) {
              ppppppbVar13 = *pppppppbVar18;
              pppppppbVar16 = pppppppbVar16 + -0x18;
              pppppppbVar18 = pppppppbVar18 + 1;
              uVar10 = CONCAT17(-(-1 < (long)ppppppbVar13),
                                CONCAT16(-(-1 < (char)((ulong)ppppppbVar13 >> 0x30)),
                                         CONCAT15(-(-1 < (char)((ulong)ppppppbVar13 >> 0x28)),
                                                  CONCAT14(-(-1 < (char)((ulong)ppppppbVar13 >> 0x20)),
                                                           CONCAT13(-(-1 < (char)((ulong)ppppppbVar13 >> 0x18)),
                                                                    CONCAT12(-(-1 < (char)((ulong)ppppppbVar13 >> 0x10)),
                                                                             CONCAT11(-(-1 < (char)((ulong)ppppppbVar13 >> 8)),-(-1 < (char)ppppppbVar13))))))))
                       & 0x8080808080808080;
            }
            uVar3 = (uVar10 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar10 >> 7 & 0xff00ff00ff00ff) << 8;
            uVar3 = (uVar3 & 0xffff0000ffff0000) >> 0x10 | (uVar3 & 0xffff0000ffff) << 0x10;
            iVar4 = (int)((ulong)LZCOUNT(uVar3 >> 0x20 | uVar3 << 0x20) >> 3);
            if (pppppppbVar16[(long)-iVar4 * 3 + -3] != (byte ******)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppbVar16[(long)-iVar4 * 3 + -2],pppppppbVar16[(long)-iVar4 * 3 + -3],1);
            }
            uVar10 = uVar10 - 1 & uVar10;
            lVar17 = lVar17 + -1;
          } while (lVar17 != 0);
        }
        lVar17 = lVar15 * 0x19 + 0x21;
        if (lVar17 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_260 + lVar15 * -3 + -3,lVar17,8);
        }
      }
      ::core::ptr::drop_in_place<codexmate_lib::core::repository::RegistryFile>(local_2c0);
      <codexmate_lib::core::account_coordination::FileLock_as_core::ops::drop::Drop>::drop(&local_2c4);
      goto LAB_100826844;
    }
    param_1[6] = CONCAT44(local_a4,uStack_a8);
    param_1[5] = CONCAT44(uStack_ac,uStack_b0);
    param_1[8] = CONCAT44(local_94,uStack_98);
    param_1[7] = CONCAT44(uStack_9c,uStack_a0);
    param_1[10] = CONCAT44(local_84,uStack_88);
    param_1[9] = CONCAT44(uStack_8c,uStack_90);
    param_1[0xc] = CONCAT44(uStack_74,uStack_78);
    param_1[0xb] = CONCAT44(uStack_7c,uStack_80);
    param_1[2] = CONCAT44(uStack_c4,iStack_c8);
    param_1[1] = local_d0;
    param_1[4] = CONCAT44(uStack_b4,iStack_b8);
    param_1[3] = CONCAT44(uStack_bc,uStack_c0);
    *param_1 = 1;
    if (lStack_258 != 0) {
      if (local_248 != 0) {
        pppppppbVar18 = local_260 + 1;
        ppppppbVar13 = *local_260;
        uVar10 = CONCAT17(-(-1 < (long)ppppppbVar13),
                          CONCAT16(-(-1 < (char)((ulong)ppppppbVar13 >> 0x30)),
                                   CONCAT15(-(-1 < (char)((ulong)ppppppbVar13 >> 0x28)),
                                            CONCAT14(-(-1 < (char)((ulong)ppppppbVar13 >> 0x20)),
                                                     CONCAT13(-(-1 < (char)((ulong)ppppppbVar13 >> 0x18)),
                                                              CONCAT12(-(-1 < (char)((ulong)ppppppbVar13 >> 0x10)),
                                                                       CONCAT11(-(-1 < (char)((ulong)ppppppbVar13 >> 8)),-(-1 < (char)ppppppbVar13)))))))) &
                 0x8080808080808080;
        pppppppbVar16 = local_260;
        lVar17 = local_248;
        do {
          while (uVar10 == 0) {
            ppppppbVar13 = *pppppppbVar18;
            pppppppbVar16 = pppppppbVar16 + -0x18;
            pppppppbVar18 = pppppppbVar18 + 1;
            uVar10 = CONCAT17(-(-1 < (long)ppppppbVar13),
                              CONCAT16(-(-1 < (char)((ulong)ppppppbVar13 >> 0x30)),
                                       CONCAT15(-(-1 < (char)((ulong)ppppppbVar13 >> 0x28)),
                                                CONCAT14(-(-1 < (char)((ulong)ppppppbVar13 >> 0x20)),
                                                         CONCAT13(-(-1 < (char)((ulong)ppppppbVar13 >> 0x18)),
                                                                  CONCAT12(-(-1 < (char)((ulong)ppppppbVar13 >> 0x10)),
                                                                           CONCAT11(-(-1 < (char)((ulong)ppppppbVar13 >> 8)),-(-1 < (char)ppppppbVar13)))))))) &
                     0x8080808080808080;
          }
          uVar3 = (uVar10 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar10 >> 7 & 0xff00ff00ff00ff) << 8;
          uVar3 = (uVar3 & 0xffff0000ffff0000) >> 0x10 | (uVar3 & 0xffff0000ffff) << 0x10;
          iVar4 = (int)((ulong)LZCOUNT(uVar3 >> 0x20 | uVar3 << 0x20) >> 3);
          if (pppppppbVar16[(long)-iVar4 * 3 + -3] != (byte ******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppbVar16[(long)-iVar4 * 3 + -2],pppppppbVar16[(long)-iVar4 * 3 + -3],1);
          }
          uVar10 = uVar10 - 1 & uVar10;
          lVar17 = lVar17 + -1;
        } while (lVar17 != 0);
      }
      lVar17 = lVar15 * 0x19 + 0x21;
      if (lVar17 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_260 + lVar15 * -3 + -3,lVar17,8);
      }
    }
    ::core::ptr::drop_in_place<codexmate_lib::core::repository::RegistryFile>(local_2c0);
  }
  else {
    param_1[7] = sStack_298;
    param_1[6] = local_2a0;
    param_1[9] = uStack_f8;
    param_1[8] = uStack_100;
    param_1[0xb] = uStack_e8;
    param_1[10] = local_f0;
    param_1[0xc] = local_e0;
    param_1[3] = pcStack_2b8;
    param_1[2] = local_130;
    param_1[5] = lStack_118;
    param_1[4] = local_2b0;
    *param_1 = 1;
    param_1[1] = local_d0;
  }
  <codexmate_lib::core::account_coordination::FileLock_as_core::ops::drop::Drop>::drop(&local_2c4);
LAB_100826844:
  uVar8 = _close(local_2c4);
  return (ulong)uVar8;
}
