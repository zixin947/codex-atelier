===BEGIN check_project_state_integrity @ 100391c44=== (GhidraScript)

/* codexmate_lib::core::relay::codex_diagnostic::check_project_state_integrity */

void codexmate_lib::core::relay::codex_diagnostic::check_project_state_integrity(undefined8 *param_1,undefined8 param_2)

{
  undefined8 *puVar1;
  undefined4 *puVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  undefined1 auStack_210 [32];
  undefined8 local_1f0;
  long local_1e8;
  long local_1c8;
  undefined8 **local_1c0;
  undefined8 *puStack_1b8;
  long local_198;
  undefined8 **ppuStack_190;
  undefined8 *local_188;
  undefined1 auStack_180 [16];
  undefined1 auStack_170 [16];
  undefined1 auStack_160 [8];
  undefined1 auStack_158 [8];
  undefined1 auStack_150 [8];
  undefined1 auStack_148 [8];
  undefined1 auStack_140 [10];
  byte local_136;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 **local_100;
  undefined8 *puStack_f8;
  undefined1 *local_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 **local_d0;
  undefined8 *puStack_c8;
  undefined1 *local_c0;
  code *pcStack_b8;
  undefined1 *local_b0;
  code *pcStack_a8;
  undefined1 *local_a0;
  code *pcStack_98;
  undefined1 *local_90;
  code *pcStack_88;
  undefined1 *local_80;
  code *pcStack_78;
  undefined1 *local_70;
  code *pcStack_68;
  undefined1 *local_60;
  code *pcStack_58;

  codex_project_state::inspect(auStack_210,param_2);
  if (local_1c8 == -0x8000000000000000) {
    if (local_1e8 == 0) {
      if ((local_136 & 1) == 0) {
        ok_item(param_1,&_anon_164d03a2dfc98f969363eaf1c50504fd_640,0x17,&DAT_10130f062,0x18);
      }
      else {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x17,1);
        if (puVar1 == (undefined8 *)0x0) goto LAB_10039206c;
        puVar1[1] = 0x6e695f6574617473;
        *puVar1 = 0x5f7463656a6f7270;
        *(undefined8 *)((long)puVar1 + 0xf) = 0x797469726765746e;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar2 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
        if (puVar2 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,5);
        }
        *(undefined1 *)(puVar2 + 1) = 0x72;
        *puVar2 = 0x6f727265;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,1);
        if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x18);
        }
        puVar3[1] = 0xe5a2b4e7ae9be7b9;
        *puVar3 = 0xa1e9207865646f43;
        puVar3[2] = 0xb8b8e582bce595bc;
        local_100 = (undefined8 **)&DAT_10130f092;
        if (local_198 != -0x8000000000000000) {
          local_100 = ppuStack_190;
        }
        puStack_f8 = (undefined8 *)0x18;
        if (local_198 != -0x8000000000000000) {
          puStack_f8 = local_188;
        }
        local_c0 = auStack_180;
        local_b0 = auStack_170;
        local_a0 = auStack_160;
        local_90 = auStack_150;
        local_80 = auStack_148;
        local_70 = auStack_158;
        local_60 = auStack_140;
        local_d0 = &local_100;
        puStack_c8 = &<&T_as_core::fmt::Display>::fmt;
        pcStack_b8 = usize>::fmt;
        pcStack_a8 = usize>::fmt;
        pcStack_98 = usize>::fmt;
        pcStack_88 = usize>::fmt;
        pcStack_78 = usize>::fmt;
        pcStack_68 = usize>::fmt;
        pcStack_58 = usize>::fmt;
        alloc::fmt::format::format_inner(&local_e8,s_roots__active_roots__hints__root_1015212c3,&local_d0);
        param_1[10] = uStack_e0;
        param_1[9] = local_e8;
        param_1[0xb] = local_d8;
        *param_1 = 0x17;
        param_1[1] = puVar1;
        param_1[3] = 5;
        param_1[2] = 0x17;
        param_1[4] = puVar2;
        param_1[6] = 0x18;
        param_1[5] = 5;
        param_1[7] = puVar3;
        param_1[8] = 0x18;
        *(undefined1 *)(param_1 + 0xc) = 1;
      }
      goto LAB_100391eb8;
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x17,1);
    if (puVar1 == (undefined8 *)0x0) goto LAB_10039206c;
    puVar1[1] = 0x6e695f6574617473;
    *puVar1 = 0x5f7463656a6f7270;
    *(undefined8 *)((long)puVar1 + 0xf) = 0x797469726765746e;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar2 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(7,1);
    if (puVar2 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,7);
    }
    *(undefined4 *)((long)puVar2 + 3) = 0x676e696e;
    *puVar2 = 0x6e726177;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x27,1);
    if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x27);
    }
    puVar3[1] = 0xe5a2b4e7ae9be7b9;
    *puVar3 = 0xa1e9207865646f43;
    puVar3[3] = 0xbae780a3e693bde4;
    puVar3[2] = 0x95b3e6a097e695bc;
    *(undefined8 *)((long)puVar3 + 0x1f) = 0x93bae58ba8e7bfba;
    alloc::str::join_generic_copy(&local_d0,local_1f0,local_1e8,&DAT_10130f0d1,3);
    puStack_f8 = puStack_c8;
    local_100 = local_d0;
    local_f0 = local_c0;
    local_d0 = &local_100;
    puStack_c8 = &<alloc::string::String_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_118,s__101521332,&local_d0);
    if ((undefined8 ***)local_100 != (undefined8 ***)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_f8,local_100,1);
    }
    param_1[10] = uStack_110;
    param_1[9] = local_118;
    param_1[0xb] = local_108;
    *param_1 = 0x17;
    param_1[1] = puVar1;
    param_1[3] = 7;
    param_1[2] = 0x17;
    param_1[4] = puVar2;
    param_1[6] = 0x27;
    param_1[5] = 7;
    uVar4 = 0x27;
  }
  else {
    local_100 = local_1c0;
    puStack_f8 = puStack_1b8;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x17,1);
    if (puVar1 == (undefined8 *)0x0) {
LAB_10039206c:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x17);
    }
    puVar1[1] = 0x6e695f6574617473;
    *puVar1 = 0x5f7463656a6f7270;
    *(undefined8 *)((long)puVar1 + 0xf) = 0x797469726765746e;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar2 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(7,1);
    if (puVar2 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,7);
    }
    *(undefined4 *)((long)puVar2 + 3) = 0x676e696e;
    *puVar2 = 0x6e726177;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1e,1);
    if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x1e);
    }
    puVar3[1] = 0xe5a2b4e7ae9be7b9;
    *puVar3 = 0xa1e9207865646f43;
    *(undefined8 *)((long)puVar3 + 0x16) = 0x968fe5bbafe895b3;
    *(undefined8 *)((long)puVar3 + 0xe) = 0xe6a097e695bce5a2;
    local_d0 = &local_100;
    puStack_c8 = &<&T_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_130,s___codex_global_state_json_____10152134d,&local_d0);
    param_1[10] = uStack_128;
    param_1[9] = local_130;
    param_1[0xb] = local_120;
    *param_1 = 0x17;
    param_1[1] = puVar1;
    param_1[3] = 7;
    param_1[2] = 0x17;
    param_1[4] = puVar2;
    param_1[6] = 0x1e;
    param_1[5] = 7;
    uVar4 = 0x1e;
  }
  param_1[7] = puVar3;
  param_1[8] = uVar4;
  *(undefined1 *)(param_1 + 0xc) = 0;
LAB_100391eb8:
  ::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_project_state::ProjectStateInspection>(auStack_210);
  return;
}

 (GhidraScript)
===END=== (GhidraScript)
