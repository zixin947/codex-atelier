===BEGIN check_router_unlock_auth_residue @ 100397070=== (GhidraScript)

/* codexmate_lib::core::relay::codex_diagnostic::check_router_unlock_auth_residue */

void codexmate_lib::core::relay::codex_diagnostic::check_router_unlock_auth_residue(undefined8 *param_1,undefined8 param_2,ulong param_3,ulong param_4)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  undefined8 *puVar4;
  undefined4 *puVar5;
  undefined8 *puVar6;
  undefined8 *puVar7;
  undefined *puVar8;
  undefined8 uVar9;

  uVar2 = router_unlock_auth::live_auth_state(param_2);
  uVar1 = (uVar2 & 0xff) - 1;
  uVar2 = router_unlock_auth::has_residue(param_2);
  uVar3 = legacy_virtual_auth::marker_exists(param_2);
  if ((param_3 & 1) == 0) {
    if (uVar1 < 2) {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1a,1);
      if (puVar4 == (undefined8 *)0x0) goto LAB_1003975cc;
      puVar4[1] = 0x75615f6b636f6c6e;
      *puVar4 = 0x755f726574756f72;
      *(undefined8 *)((long)puVar4 + 0x12) = 0x657564697365725f;
      *(undefined8 *)((long)puVar4 + 10) = 0x687475615f6b636f;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar5 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
      if (puVar5 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,5);
      }
      *(undefined1 *)(puVar5 + 1) = 0x72;
      *puVar5 = 0x6f727265;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar6 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x2d,1);
      if (puVar6 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x2d);
      }
      puVar6[1] = 0xe4ad97e9b385e5b2;
      *puVar6 = 0xb7e5b194e7afb7e8;
      puVar6[3] = 0x6120b78fe5a6b4e8;
      puVar6[2] = 0x9f8be69a99e886bd;
      *(undefined8 *)((long)puVar6 + 0x25) = 0x9995e78baee68dbb;
      *(undefined8 *)((long)puVar6 + 0x1d) = 0xe4206874756120b7;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x7a,1);
      if (puVar7 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x7a);
      }
      puVar7[9] = 0xbae59bbcefa5b4e8;
      puVar7[8] = 0xb1a4e59abce4a0bc;
      puVar7[0xb] = 0x7561209eaee59f9c;
      puVar7[10] = 0xe78da4e5a281e694;
      puVar7[0xd] = 0xe69a99e8a499e9a0;
      puVar7[0xc] = 0x88e59688e6206874;
      *(undefined8 *)((long)puVar7 + 0x72) = 0x8280e36874756120;
      *(undefined8 *)((long)puVar7 + 0x6a) = 0x9f8be69a99e8a499;
      puVar7[1] = 0xe4afafe8bd83e8af;
      *puVar7 = 0x8fe5207865646f43;
      puVar7[3] = 0x8fe5af98e68d89e5;
      puVar7[2] = 0x93bde5bab8e4a5bb;
      puVar7[5] = 0x86bde48cbcefb78f;
      puVar7[4] = 0xe5a6b4e8a894e7af;
      puVar7[7] = 0xe48f80e98b9ee5a1;
      puVar7[6] = 0xa8e6b996e698aee5;
      *param_1 = 0x1a;
      param_1[1] = puVar4;
      param_1[3] = 5;
      param_1[2] = 0x1a;
      param_1[4] = puVar5;
      param_1[6] = 0x2d;
      param_1[5] = 5;
      param_1[7] = puVar6;
      param_1[9] = 0x7a;
      param_1[8] = 0x2d;
      uVar9 = 0x7a;
    }
    else {
      if (((uVar2 | uVar3) & 1) == 0) {
        puVar8 = &DAT_10130f9e8;
        uVar9 = 0x15;
        goto LAB_1003975b8;
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1a,1);
      if (puVar4 == (undefined8 *)0x0) goto LAB_1003975cc;
      puVar4[1] = 0x75615f6b636f6c6e;
      *puVar4 = 0x755f726574756f72;
      *(undefined8 *)((long)puVar4 + 0x12) = 0x657564697365725f;
      *(undefined8 *)((long)puVar4 + 10) = 0x687475615f6b636f;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar5 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
      if (puVar5 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,5);
      }
      *(undefined1 *)(puVar5 + 1) = 0x72;
      *puVar5 = 0x6f727265;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar6 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x33,1);
      if (puVar6 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x33);
      }
      *(undefined4 *)((long)puVar6 + 0x2f) = 0x9995e78b;
      puVar6[1] = 0xe4ad97e9b385e5b2;
      *puVar6 = 0xb7e5b194e7afb7e8;
      puVar6[3] = 0x81e6b78fe5a6b4e8;
      puVar6[2] = 0x9f8be69a99e886bd;
      puVar6[5] = 0x8baee68dbbe4b6bb;
      puVar6[4] = 0xe48796e68da4e5a2;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x82,1);
      if (puVar7 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x82);
      }
      *(undefined2 *)(puVar7 + 0x10) = 0x8280;
      puVar7[9] = 0x9bbcef8690e785b8;
      puVar7[8] = 0xe69088e68caee5aa;
      puVar7[0xb] = 0xe48ca1e8a789e6a8;
      puVar7[10] = 0x85e589aee5af8fe5;
      puVar7[0xd] = 0x4e209b95e6b694e6;
      puVar7[0xc] = 0xa5bbe48da4e5aebf;
      puVar7[0xf] = 0xe38180e6b68ae720;
      puVar7[0xe] = 0x66664f6576697461;
      puVar7[1] = 0x6e6f736a2e687475;
      *puVar7 = 0x61208d89e593bde5;
      puVar7[3] = 0xa6b4e89f8be69a99;
      puVar7[2] = 0xe8aba2e8aa9ce620;
      puVar7[5] = 0x2086bde48cbcefa8;
      puVar7[4] = 0x94e7a08de5b78fe5;
      puVar7[7] = 0x9ce62070756b6361;
      puVar7[6] = 0x622f72656b72616d;
      *param_1 = 0x1a;
      param_1[1] = puVar4;
      param_1[3] = 5;
      param_1[2] = 0x1a;
      param_1[4] = puVar5;
      param_1[6] = 0x33;
      param_1[5] = 5;
      param_1[7] = puVar6;
      param_1[9] = 0x82;
      param_1[8] = 0x33;
      uVar9 = 0x82;
    }
LAB_100397540:
    param_1[10] = puVar7;
    param_1[0xb] = uVar9;
    *(undefined1 *)(param_1 + 0xc) = 1;
    return;
  }
  if ((param_4 & 1) == 0) {
    if (uVar1 < 2) {
      uVar2 = 1;
    }
    if (((uVar3 | uVar2) & 1) == 0) {
      puVar8 = &DAT_10130fb59;
      uVar9 = 0x27;
LAB_1003975b8:
      ok_item(param_1,&_anon_164d03a2dfc98f969363eaf1c50504fd_632,0x1a,puVar8,uVar9);
      return;
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1a,1);
    if (puVar4 != (undefined8 *)0x0) {
      puVar4[1] = 0x75615f6b636f6c6e;
      *puVar4 = 0x755f726574756f72;
      *(undefined8 *)((long)puVar4 + 0x12) = 0x657564697365725f;
      *(undefined8 *)((long)puVar4 + 10) = 0x687475615f6b636f;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar5 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
      if (puVar5 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,5);
      }
      *(undefined1 *)(puVar5 + 1) = 0x72;
      *puVar5 = 0x6f727265;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar6 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x32,1);
      if (puVar6 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x32);
      }
      *(undefined2 *)(puVar6 + 6) = 0x9995;
      puVar6[1] = 0xe5a1a8e6b78fe5a6;
      *puVar6 = 0xb4e89eaee59f9ce7;
      puVar6[3] = 0x20694d614d694120;
      puVar6[2] = 0x899ce68dbbe48fbc;
      puVar6[5] = 0xe78baee681afe8a4;
      puVar6[4] = 0xaee8a1aee7978fe5;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x87,1);
      if (puVar7 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x87);
      }
      puVar7[0xd] = 0x208690e785b8e6b6;
      puVar7[0xc] = 0xb9e58cbcef95bde5;
      puVar7[0xf] = 0x61622f72656b7261;
      puVar7[0xe] = 0x6d20694d614d6941;
      *(undefined8 *)((long)puVar7 + 0x7f) = 0x8280e370756b6361;
      puVar7[5] = 0x20687475414f2054;
      puVar7[4] = 0x504774616843208d;
      puVar7[7] = 0xe589aee5af8fe59b;
      puVar7[6] = 0xbcef95bde5bb99e7;
      puVar7[9] = 0x95e79dbfe49688e6;
      puVar7[8] = 0x8da4e5a281e6a885;
      puVar7[0xb] = 0xbb99e7b096e6809c;
      puVar7[10] = 0xe6b788e6a894e799;
      puVar7[1] = 0xe5a1a8e6b78fe5a6;
      *puVar7 = 0xb4e89eaee59f9ce7;
      puVar7[3] = 0x89e593bde5a894e7;
      puVar7[2] = 0xbfbde494bae58fbc;
      *param_1 = 0x1a;
      param_1[1] = puVar4;
      param_1[3] = 5;
      param_1[2] = 0x1a;
      param_1[4] = puVar5;
      param_1[6] = 0x32;
      param_1[5] = 5;
      param_1[7] = puVar6;
      param_1[9] = 0x87;
      param_1[8] = 0x32;
      uVar9 = 0x87;
      goto LAB_100397540;
    }
  }
  else {
    if (((uVar2 | uVar3) & 1) == 0) {
      puVar8 = &DAT_10130fc39;
      uVar9 = 0x33;
      goto LAB_1003975b8;
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1a,1);
    if (puVar4 != (undefined8 *)0x0) {
      puVar4[1] = 0x75615f6b636f6c6e;
      *puVar4 = 0x755f726574756f72;
      *(undefined8 *)((long)puVar4 + 0x12) = 0x657564697365725f;
      *(undefined8 *)((long)puVar4 + 10) = 0x687475615f6b636f;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar5 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
      if (puVar5 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,5);
      }
      *(undefined1 *)(puVar5 + 1) = 0x72;
      *puVar5 = 0x6f727265;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar6 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x2d,1);
      if (puVar6 != (undefined8 *)0x0) {
        puVar6[1] = 0xe48fbce5a1a8e6b7;
        *puVar6 = 0x8fe5a6b4e8a097e6;
        puVar6[3] = 0xaee7978fe58889e7;
        puVar6[2] = 0xa797e6899ce68dbb;
        *(undefined8 *)((long)puVar6 + 0x25) = 0x9995e78baee681af;
        *(undefined8 *)((long)puVar6 + 0x1d) = 0xe8a4aee8a1aee797;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x83,1);
        if (puVar7 != (undefined8 *)0x0) {
          *(undefined4 *)((long)puVar7 + 0x7f) = 0x8280e36e;
          puVar7[9] = 0xe589aee58cbcef8d;
          puVar7[8] = 0xa4e5aebfe48180e6;
          puVar7[0xb] = 0xafe8a4aee8a797e6;
          puVar7[10] = 0x8da4e5a281e6a885;
          puVar7[0xd] = 0xa58ee6b49be7bd83;
          puVar7[0xc] = 0xe88db8e48cbcef81;
          puVar7[0xf] = 0x6e6f736a2e687475;
          puVar7[0xe] = 0x6120a499e9a088e5;
          puVar7[1] = 0xe6b78fe5a6b4e8a0;
          *puVar7 = 0x97e69f94e79f8ee5;
          puVar7[3] = 0x8de581a6e8809ce9;
          puVar7[2] = 0x8db8e48fbce5a1a8;
          puVar7[5] = 0x89e6b7afe89bbcef;
          puVar7[4] = 0x79654b208dbde4a0;
          puVar7[7] = 0x87a0e6ae9be7b194;
          puVar7[6] = 0xe7afb7e88ca1e8a7;
          *param_1 = 0x1a;
          param_1[1] = puVar4;
          param_1[3] = 5;
          param_1[2] = 0x1a;
          param_1[4] = puVar5;
          param_1[6] = 0x2d;
          param_1[5] = 5;
          param_1[7] = puVar6;
          param_1[9] = 0x83;
          param_1[8] = 0x2d;
          param_1[10] = puVar7;
          param_1[0xb] = 0x83;
          *(undefined1 *)(param_1 + 0xc) = 0;
          return;
        }
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x83);
      }
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x2d);
    }
  }
LAB_1003975cc:
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error(1,0x1a);
}

 (GhidraScript)
===END=== (GhidraScript)
