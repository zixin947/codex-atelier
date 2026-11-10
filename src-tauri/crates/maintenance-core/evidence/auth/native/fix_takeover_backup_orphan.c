===BEGIN fix_takeover_backup_orphan @ 1003885e8=== (GhidraScript)

/* codexmate_lib::core::relay::codex_diagnostic::fix_takeover_backup_orphan */

void codexmate_lib::core::relay::codex_diagnostic::fix_takeover_backup_orphan(long *param_1,undefined8 param_2,long param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  long *plVar4;
  long *plVar5;
  long lVar6;
  long *local_1c0;
  undefined8 *puStack_1b8;
  long local_1b0;
  short *psStack_1a8;
  long local_1a0;
  long lStack_198;
  long lStack_190;
  long lStack_188;
  long local_180;
  long lStack_178;
  long local_170;
  long local_168;
  long *local_160;
  undefined8 *puStack_158;
  long local_150;
  short *psStack_148;
  undefined8 *local_140;
  long lStack_138;
  long lStack_130;
  long lStack_128;
  long local_120;
  long lStack_118;
  long lStack_110;
  long lStack_108;
  long local_100;
  long *plStack_f8;
  undefined8 *local_f0;
  long local_e8;
  short *psStack_e0;
  long local_d8;
  long lStack_d0;
  long local_c8;
  long lStack_c0;
  long local_b8;
  long lStack_b0;
  long local_a8;
  long lStack_a0;
  undefined8 **local_98;
  undefined8 *puStack_90;
  long local_88;
  long *local_80;
  undefined8 *puStack_78;
  long local_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;

  if ((*(byte *)(param_3 + 0x16c) & 1) == 0) {
    config_takeover::restore(&local_100);
    lStack_138 = lStack_d0;
    local_140 = (undefined8 *)local_d8;
    lStack_128 = lStack_c0;
    lStack_130 = local_c8;
    lStack_118 = lStack_b0;
    local_120 = local_b8;
    lStack_108 = lStack_a0;
    lStack_110 = local_a8;
    puStack_158 = local_f0;
    local_160 = plStack_f8;
    psStack_148 = psStack_e0;
    local_150 = local_e8;
    if ((int)local_100 == 1) {
      param_1[5] = lStack_d0;
      param_1[4] = local_d8;
      param_1[7] = lStack_c0;
      param_1[6] = local_c8;
      param_1[9] = lStack_b0;
      param_1[8] = local_b8;
      param_1[0xb] = lStack_a0;
      param_1[10] = local_a8;
      param_1[1] = (long)local_f0;
      *param_1 = (long)plStack_f8;
      param_1[3] = (long)psStack_e0;
      param_1[2] = local_e8;
    }
    else {
      lStack_198 = lStack_d0;
      local_1a0 = local_d8;
      lStack_188 = lStack_c0;
      lStack_190 = local_c8;
      lStack_178 = lStack_b0;
      local_180 = local_b8;
      local_168 = lStack_a0;
      local_170 = local_a8;
      puStack_1b8 = local_f0;
      local_1c0 = plStack_f8;
      psStack_1a8 = psStack_e0;
      local_1b0 = local_e8;
      check_takeover_backup_orphan(&local_100,param_2,0);
      if ((local_d8 == 2) && (*psStack_e0 == 0x6b6f)) {
        if (local_168 == 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x34,1);
          if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(1,0x34);
          }
          *(undefined4 *)(puVar1 + 6) = 0xb6bbe487;
          puVar1[1] = 0x766f656b6174209f;
          *puVar1 = 0x8ee598bfe8b2b7e5;
          puVar1[3] = 0xe9a088e5b6b9e5bd;
          puVar1[2] = 0xbbe487a4e5207265;
          puVar1[5] = 0x96e6bdbbe487a4e5;
          puVar1[4] = 0x9995e78baee6a499;
          plVar5 = (long *)0x34;
          lVar6 = 0x34;
        }
        else {
          alloc::str::join_generic_copy(&local_160,local_170,local_168,&DAT_10130dcc1,3);
          puStack_78 = puStack_158;
          local_80 = local_160;
          local_70 = local_150;
          local_98 = &local_80;
          puStack_90 = &<alloc::string::String_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_160,s_C_takeover_101520c83,&local_98);
          puVar1 = puStack_158;
          plVar5 = local_160;
          lVar6 = local_150;
          if (local_80 != (long *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_78,local_80,1);
            puVar1 = puStack_158;
            plVar5 = local_160;
            lVar6 = local_150;
          }
        }
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar2 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
        if (puVar2 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error(8,0x18);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x16,1);
        if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x16);
        }
        puVar3[1] = 0x5f70756b6361625f;
        *puVar3 = 0x7265766f656b6174;
        *(undefined8 *)((long)puVar3 + 0xe) = 0x6e616870726f5f70;
        *puVar2 = 0x16;
        puVar2[1] = puVar3;
        puVar2[2] = 0x16;
        local_98 = (undefined8 **)0x1;
        local_88 = 1;
        local_80 = (long *)0x0;
        puStack_78 = (undefined8 *)&SUB_00000008;
        local_70 = 0;
        puStack_90 = puVar2;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        plVar4 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
        if (plVar4 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error(8,0x18);
        }
        *plVar4 = (long)plVar5;
        plVar4[1] = (long)puVar1;
        plVar4[2] = lVar6;
        param_1[2] = (long)puStack_90;
        param_1[1] = (long)local_98;
        param_1[4] = (long)local_80;
        param_1[3] = local_88;
        param_1[6] = local_70;
        param_1[5] = (long)puStack_78;
        param_1[7] = 1;
        param_1[8] = (long)plVar4;
        param_1[9] = 1;
        *param_1 = 0xb;
        local_160 = (long *)local_98;
        puStack_158 = puStack_90;
        local_150 = local_88;
        psStack_148 = (short *)local_80;
        local_140 = puStack_78;
        lStack_138 = local_70;
      }
      else {
        local_98 = (undefined8 **)0x0;
        puStack_90 = (undefined8 *)&SUB_00000008;
        local_88 = 0;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
        if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error(8,0x18);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar2 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x16,1);
        if (puVar2 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x16);
        }
        puVar2[1] = 0x5f70756b6361625f;
        *puVar2 = 0x7265766f656b6174;
        *(undefined8 *)((long)puVar2 + 0xe) = 0x6e616870726f5f70;
        *puVar1 = 0x16;
        puVar1[1] = puVar2;
        puVar1[2] = 0x16;
        local_80 = (long *)0x1;
        local_70 = 1;
        puStack_78 = puVar1;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
        if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error(8,0x18);
        }
        local_160 = &lStack_d0;
        puStack_158 = &<alloc::string::String_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_68,s__takeover_101520c5f,&local_160);
        puVar1[1] = uStack_60;
        *puVar1 = local_68;
        puVar1[2] = local_58;
        param_1[2] = (long)puStack_90;
        param_1[1] = (long)local_98;
        param_1[4] = (long)local_80;
        param_1[3] = local_88;
        param_1[6] = local_70;
        param_1[5] = (long)puStack_78;
        param_1[7] = 1;
        param_1[8] = (long)puVar1;
        param_1[9] = 1;
        *param_1 = 0xb;
        local_160 = (long *)local_98;
        puStack_158 = puStack_90;
        local_150 = local_88;
        psStack_148 = (short *)local_80;
        local_140 = puStack_78;
        lStack_138 = local_70;
      }
      local_98 = (undefined8 **)local_160;
      puStack_90 = puStack_158;
      local_88 = local_150;
      local_80 = (long *)psStack_148;
      puStack_78 = local_140;
      local_70 = lStack_138;
      if (local_100 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_f8,local_100,1);
      }
      if (local_e8 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psStack_e0,local_e8,1);
      }
      if (lStack_d0 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_c8,lStack_d0,1);
      }
      if ((local_b8 != -0x8000000000000000) && (local_b8 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_b0,local_b8,1);
      }
      ::core::ptr::drop_in_place<codexmate_lib::core::relay::config_takeover::RestoreSummary>(&local_1c0);
    }
  }
  else {
    local_1c0 = (long *)0x0;
    puStack_1b8 = (undefined8 *)0x8;
    local_1b0 = 0;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar2 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x16,1);
    if (puVar2 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x16);
    }
    puVar2[1] = 0x5f70756b6361625f;
    *puVar2 = 0x7265766f656b6174;
    *(undefined8 *)((long)puVar2 + 0xe) = 0x6e616870726f5f70;
    *puVar1 = 0x16;
    puVar1[1] = puVar2;
    puVar1[2] = 0x16;
    local_160 = (long *)0x1;
    local_150 = 1;
    puStack_158 = puVar1;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar2 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x49,1);
    if (puVar2 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x49);
    }
    puVar2[5] = 0xaee7a58ee6b8b8e5;
    puVar2[4] = 0xa3ade68ebae49eb1;
    puVar2[7] = 0x8ca1e8a789e68db8;
    puVar2[6] = 0xe48cbcef8180e6a1;
    *(undefined8 *)((long)puVar2 + 0x41) = 0x8280e39f8ee598bf;
    *(undefined8 *)((long)puVar2 + 0x39) = 0xe88ca1e8a789e68d;
    puVar2[1] = 0x20adb8e4a894e7af;
    *puVar2 = 0x90e5b194e7afb7e8;
    puVar2[3] = 0xe5bdbbe487a4e520;
    puVar2[2] = 0x7265766f656b6174;
    param_1[2] = (long)puStack_1b8;
    param_1[1] = (long)local_1c0;
    *puVar1 = 0x49;
    puVar1[1] = puVar2;
    puVar1[2] = 0x49;
    param_1[4] = (long)local_160;
    param_1[3] = local_1b0;
    param_1[6] = local_150;
    param_1[5] = (long)puStack_158;
    param_1[7] = 1;
    param_1[8] = (long)puVar1;
    param_1[9] = 1;
    *param_1 = 0xb;
  }
  return;
}

 (GhidraScript)
===END=== (GhidraScript)
