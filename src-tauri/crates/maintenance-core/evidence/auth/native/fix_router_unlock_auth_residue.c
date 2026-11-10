===BEGIN fix_router_unlock_auth_residue @ 100393114=== (GhidraScript)

/* codexmate_lib::core::relay::codex_diagnostic::fix_router_unlock_auth_residue */

void codexmate_lib::core::relay::codex_diagnostic::fix_router_unlock_auth_residue(long *param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  byte bVar2;
  long lVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  long local_130;
  undefined1 uStack_128;
  undefined7 uStack_127;
  undefined1 local_120;
  undefined7 uStack_11f;
  undefined1 uStack_118;
  undefined7 uStack_117;
  undefined1 uStack_110;
  undefined7 uStack_10f;
  undefined1 uStack_108;
  undefined7 uStack_107;
  undefined1 uStack_100;
  undefined7 uStack_ff;
  undefined1 local_f8;
  undefined7 uStack_f7;
  undefined1 uStack_f0;
  undefined7 uStack_ef;
  undefined1 uStack_e8;
  undefined7 uStack_e7;
  undefined1 uStack_e0;
  undefined7 uStack_df;
  undefined1 uStack_d8;
  undefined7 uStack_d7;
  undefined1 *local_c0;
  undefined8 *puStack_b8;
  long local_b0;
  long lStack_a8;
  undefined8 *puStack_a0;
  long lStack_98;
  undefined1 *local_88;
  undefined8 *puStack_80;
  long local_78;
  long local_70;
  undefined8 *puStack_68;
  long local_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;

  cVar1 = *(char *)(param_3 + 0x16c);
  bVar2 = *(byte *)(param_3 + 0x17e);
  if ((cVar1 == '\x01') && ((bVar2 & 1) != 0)) {
    local_70 = 0;
    puStack_68 = (undefined8 *)0x8;
    local_60 = 0;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1a,1);
    if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x1a);
    }
    puVar5[1] = 0x75615f6b636f6c6e;
    *puVar5 = 0x755f726574756f72;
    *(undefined8 *)((long)puVar5 + 0x12) = 0x657564697365725f;
    *(undefined8 *)((long)puVar5 + 10) = 0x687475615f6b636f;
    *puVar4 = 0x1a;
    puVar4[1] = puVar5;
    puVar4[2] = 0x1a;
    local_c0 = (undefined1 *)0x1;
    local_b0 = 1;
    puStack_b8 = puVar4;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x67,1);
    if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x67);
    }
    puVar5[9] = 0x9ce68cbcef9b95e6;
    puVar5[8] = 0xb694e68d87e98180;
    puVar5[0xb] = 0x7561208690e785b8;
    puVar5[10] = 0xe6a58ee6b49be7aa;
    *(undefined8 *)((long)puVar5 + 0x5f) = 0x6e6f736a2e687475;
    puVar5[1] = 0xe5a6b4e8a097e620;
    *puVar5 = 0x6e4f726574756f52;
    puVar5[3] = 0xaee7978fe5849ae7;
    puVar5[2] = 0x8fbce5a1a8e6b78f;
    puVar5[5] = 0x9a80e9bd83e8aa8f;
    puVar5[4] = 0xe581afe8a4aee8a1;
    puVar5[7] = 0xe687a0e6ae9be7b4;
    puVar5[6] = 0x95e68caee587bfe8;
    *puVar4 = 0x67;
    puVar4[1] = puVar5;
    puVar4[2] = 0x67;
    param_1[2] = (long)puStack_68;
    param_1[1] = local_70;
    param_1[4] = (long)local_c0;
    param_1[3] = local_60;
    param_1[6] = local_b0;
    param_1[5] = (long)puStack_b8;
    param_1[7] = 1;
    param_1[8] = (long)puVar4;
    param_1[9] = 1;
    *param_1 = 0xb;
  }
  else {
    router_unlock_auth::cleanup_for_native_off(&local_130,param_2);
    if (local_130 == 0xb) {
      check_router_unlock_auth_residue(&local_130,param_2,cVar1,bVar2);
      if ((CONCAT71(uStack_107,uStack_108) == 2) && (*(short *)CONCAT71(uStack_10f,uStack_110) == 0x6b6f)) {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
        if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error(8,0x18);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1a,1);
        if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x1a);
        }
        puVar5[1] = 0x75615f6b636f6c6e;
        *puVar5 = 0x755f726574756f72;
        *(undefined8 *)((long)puVar5 + 0x12) = 0x657564697365725f;
        *(undefined8 *)((long)puVar5 + 10) = 0x687475615f6b636f;
        *puVar4 = 0x1a;
        puVar4[1] = puVar5;
        puVar4[2] = 0x1a;
        local_88 = (undefined1 *)0x1;
        local_78 = 1;
        local_70 = 0;
        puStack_68 = (undefined8 *)&SUB_00000008;
        local_60 = 0;
        puStack_80 = puVar4;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
        if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error(8,0x18);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x4b,1);
        if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x4b);
        }
        puVar5[5] = 0x4d614d6941208690;
        puVar5[4] = 0xe785b8e6b6b9e58c;
        puVar5[7] = 0x616d2f6874756120;
        puVar5[6] = 0xa1aee7978fe52069;
        *(undefined8 *)((long)puVar5 + 0x43) = 0x9995e78baee62072;
        *(undefined8 *)((long)puVar5 + 0x3b) = 0x656b72616d2f6874;
        puVar5[1] = 0xe79dbfe49688e68d;
        *puVar5 = 0xa4e5a281e6b2b7e5;
        puVar5[3] = 0xbcef81afe8a4aee8;
        puVar5[2] = 0xb788e6a894e79995;
        param_1[2] = (long)puStack_80;
        param_1[1] = (long)local_88;
        *puVar4 = 0x4b;
        puVar4[1] = puVar5;
        puVar4[2] = 0x4b;
        param_1[4] = local_70;
        param_1[3] = local_78;
        param_1[6] = local_60;
        param_1[5] = (long)puStack_68;
      }
      else {
        local_88 = (undefined1 *)0x0;
        puStack_80 = (undefined8 *)&SUB_00000008;
        local_78 = 0;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
        if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error(8,0x18);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1a,1);
        if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x1a);
        }
        puVar5[1] = 0x75615f6b636f6c6e;
        *puVar5 = 0x755f726574756f72;
        *(undefined8 *)((long)puVar5 + 0x12) = 0x657564697365725f;
        *(undefined8 *)((long)puVar5 + 10) = 0x687475615f6b636f;
        *puVar4 = 0x1a;
        puVar4[1] = puVar5;
        puVar4[2] = 0x1a;
        local_70 = 1;
        local_60 = 1;
        puStack_68 = puVar4;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
        if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error(8,0x18);
        }
        local_c0 = &uStack_100;
        puStack_b8 = &<alloc::string::String_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_58,s___101521b1e,&local_c0);
        puVar4[1] = uStack_50;
        *puVar4 = local_58;
        puVar4[2] = local_48;
        param_1[2] = (long)puStack_80;
        param_1[1] = (long)local_88;
        param_1[4] = local_70;
        param_1[3] = local_78;
        param_1[6] = local_60;
        param_1[5] = (long)puStack_68;
      }
      param_1[7] = 1;
      param_1[8] = (long)puVar4;
      param_1[9] = 1;
      *param_1 = 0xb;
      local_c0 = local_88;
      puStack_b8 = puStack_80;
      local_b0 = local_78;
      lStack_a8 = local_70;
      puStack_a0 = puStack_68;
      lStack_98 = local_60;
      if (local_130 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(CONCAT71(uStack_127,uStack_128),local_130,1);
      }
      if (CONCAT71(uStack_117,uStack_118) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(CONCAT71(uStack_10f,uStack_110),CONCAT71(uStack_117,uStack_118),1);
      }
      if (CONCAT71(uStack_ff,uStack_100) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(CONCAT71(uStack_f7,local_f8),CONCAT71(uStack_ff,uStack_100),1);
      }
      lVar3 = CONCAT71(uStack_e7,uStack_e8);
      if ((lVar3 != -0x8000000000000000) && (lVar3 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(CONCAT71(uStack_df,uStack_e0),lVar3,1);
      }
    }
    else {
      *(ulong *)((long)param_1 + 0x31) = CONCAT17(local_f8,uStack_ff);
      *(ulong *)((long)param_1 + 0x29) = CONCAT17(uStack_100,uStack_107);
      *(ulong *)((long)param_1 + 0x41) = CONCAT17(uStack_e8,uStack_ef);
      *(ulong *)((long)param_1 + 0x39) = CONCAT17(uStack_f0,uStack_f7);
      *(ulong *)((long)param_1 + 0x51) = CONCAT17(uStack_d8,uStack_df);
      *(ulong *)((long)param_1 + 0x49) = CONCAT17(uStack_e0,uStack_e7);
      param_1[0xb] = CONCAT71(uStack_d7,uStack_d8);
      *(ulong *)((long)param_1 + 0x11) = CONCAT17(uStack_118,uStack_11f);
      *(ulong *)((long)param_1 + 9) = CONCAT17(local_120,uStack_127);
      *(ulong *)((long)param_1 + 0x21) = CONCAT17(uStack_108,uStack_10f);
      *(ulong *)((long)param_1 + 0x19) = CONCAT17(uStack_110,uStack_117);
      *param_1 = local_130;
      *(undefined1 *)(param_1 + 1) = uStack_128;
    }
  }
  return;
}

 (GhidraScript)
===END=== (GhidraScript)
