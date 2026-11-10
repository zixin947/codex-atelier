===BEGIN fix_project_state_integrity @ 10038e4b8=== (GhidraScript)

/* codexmate_lib::core::relay::codex_diagnostic::fix_project_state_integrity */

void codexmate_lib::core::relay::codex_diagnostic::fix_project_state_integrity(undefined8 *param_1,undefined8 param_2)

{
  short *psVar1;
  ulong uVar2;
  short *psVar3;
  short *psVar4;
  short *psVar5;
  long lVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  short *psVar10;
  ulong *puVar11;
  bool bVar12;
  undefined1 auStack_250 [218];
  char local_176;
  short *local_170;
  undefined8 *puStack_168;
  ulong local_160;
  short *local_158;
  short *psStack_150;
  long local_148;
  short *psStack_140;
  long local_138;
  short *local_130;
  short *psStack_128;
  long local_120;
  short *psStack_118;
  undefined8 *local_110;
  short *local_100;
  short *psStack_f8;
  short *local_f0;
  short *psStack_e8;
  undefined8 *puStack_e0;
  ulong local_d8;
  short *local_c8;
  short *psStack_c0;
  long local_b8;
  short *local_b0;
  undefined8 *puStack_a8;
  ulong local_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  short **local_80;
  undefined8 *puStack_78;

  codex_project_state::inspect(auStack_250);
  if (local_176 != '\x01') {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar7 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x17,1);
    if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x17);
    }
    puVar8[1] = 0x6e695f6574617473;
    *puVar8 = 0x5f7463656a6f7270;
    *(undefined8 *)((long)puVar8 + 0xf) = 0x797469726765746e;
    *puVar7 = 0x17;
    puVar7[1] = puVar8;
    puVar7[2] = 0x17;
    local_b0 = (short *)0x1;
    local_a0 = 1;
    local_100 = (short *)0x0;
    psStack_f8 = (short *)&SUB_00000008;
    local_f0 = (short *)0x0;
    puStack_a8 = puVar7;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar7 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x27,1);
    if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x27);
    }
    local_160 = local_a0;
    psStack_150 = psStack_f8;
    local_158 = local_100;
    puVar8[1] = 0xe5a2b4e7ae9be7b9;
    *puVar8 = 0xa1e9207865646f43;
    puVar8[3] = 0x9ce9a097e68cbcef;
    puVar8[2] = 0xb8b8e5a3ade695bc;
    puStack_168 = puStack_a8;
    local_170 = local_b0;
    param_1[2] = puStack_a8;
    param_1[1] = local_b0;
    *(undefined8 *)((long)puVar8 + 0x1f) = 0x8da4e5aebfe4809c;
    *puVar7 = 0x27;
    puVar7[1] = puVar8;
    puVar7[2] = 0x27;
    local_148 = (long)local_f0;
    param_1[4] = local_100;
    param_1[3] = local_a0;
    param_1[6] = local_f0;
    param_1[5] = psStack_f8;
    param_1[7] = 1;
    param_1[8] = puVar7;
    param_1[9] = 1;
    *param_1 = 0xb;
    goto LAB_10038e654;
  }
  codex_project_state::repair_if_needed(&local_170,param_2);
  lVar6 = local_138;
  psVar5 = psStack_140;
  psVar4 = psStack_150;
  psVar3 = local_158;
  uVar2 = local_160;
  puVar7 = puStack_168;
  psVar1 = local_170;
  if (local_170 == (short *)0x8000000000000000) {
    psStack_f8 = psStack_128;
    local_100 = local_130;
    psStack_e8 = psStack_118;
    local_f0 = (short *)local_120;
    puStack_e0 = local_110;
    *param_1 = puStack_168;
    param_1[1] = local_160;
    param_1[2] = local_158;
    param_1[3] = psStack_150;
    param_1[5] = psStack_140;
    param_1[4] = local_148;
    param_1[6] = local_138;
    param_1[8] = psStack_128;
    param_1[7] = local_130;
    param_1[10] = psStack_118;
    param_1[9] = local_120;
    param_1[0xb] = local_110;
    goto LAB_10038e654;
  }
  check_project_state_integrity(&local_170,param_2);
  if ((local_148 == 2) && (*psStack_150 == 0x6b6f)) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    psVar10 = (short *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (psVar10 == (short *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x17,1);
    if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x17);
    }
    puVar8[1] = 0x6e695f6574617473;
    *puVar8 = 0x5f7463656a6f7270;
    *(undefined8 *)((long)puVar8 + 0xf) = 0x797469726765746e;
    psVar10[0] = 0x17;
    psVar10[1] = 0;
    psVar10[2] = 0;
    psVar10[3] = 0;
    *(undefined8 **)(psVar10 + 4) = puVar8;
    psVar10[8] = 0x17;
    psVar10[9] = 0;
    psVar10[10] = 0;
    psVar10[0xb] = 0;
    local_c8 = (short *)0x1;
    local_b8 = 1;
    local_b0 = (short *)0x0;
    puStack_a8 = (undefined8 *)&SUB_00000008;
    local_a0 = 0;
    psStack_c0 = psVar10;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar11 = (ulong *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar11 == (ulong *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    *puVar11 = (ulong)psVar1;
    puVar11[1] = (ulong)puVar7;
    puVar11[2] = uVar2;
    param_1[2] = psStack_c0;
    param_1[1] = local_c8;
    param_1[4] = local_b0;
    param_1[3] = local_b8;
    param_1[6] = local_a0;
    param_1[5] = puStack_a8;
    bVar12 = true;
    param_1[7] = 1;
    param_1[8] = puVar11;
    param_1[9] = 1;
    *param_1 = 0xb;
    local_100 = local_c8;
    psStack_f8 = psStack_c0;
    local_f0 = (short *)local_b8;
    psStack_e8 = local_b0;
    puStack_e0 = puStack_a8;
    local_d8 = local_a0;
  }
  else {
    local_c8 = (short *)0x0;
    psStack_c0 = (short *)&SUB_00000008;
    local_b8 = 0;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x17,1);
    if (puVar9 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x17);
    }
    puVar9[1] = 0x6e695f6574617473;
    *puVar9 = 0x5f7463656a6f7270;
    *(undefined8 *)((long)puVar9 + 0xf) = 0x797469726765746e;
    *puVar8 = 0x17;
    puVar8[1] = puVar9;
    puVar8[2] = 0x17;
    local_b0 = (short *)0x1;
    local_a0 = 1;
    puStack_a8 = puVar8;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    if (psStack_128 == (short *)&DAT_8000000000000000) {
      psStack_f8 = (short *)local_138;
      local_100 = psStack_140;
      local_f0 = local_130;
    }
    else {
      local_100 = psStack_128;
      local_f0 = psStack_118;
      psStack_f8 = (short *)local_120;
      if (psStack_140 != (short *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_138,psStack_140,1);
      }
    }
    local_80 = &local_100;
    puStack_78 = &<alloc::string::String_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_98,s_4project_state_integrity__101520fee,&local_80);
    if (local_100 != (short *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psStack_f8,local_100,1);
    }
    bVar12 = false;
    puVar8[1] = uStack_90;
    *puVar8 = local_98;
    puVar8[2] = local_88;
    param_1[2] = psStack_c0;
    param_1[1] = local_c8;
    param_1[4] = local_b0;
    param_1[3] = local_b8;
    param_1[6] = local_a0;
    param_1[5] = puStack_a8;
    param_1[7] = 1;
    param_1[8] = puVar8;
    param_1[9] = 1;
    *param_1 = 0xb;
    local_100 = local_c8;
    psStack_f8 = psStack_c0;
    local_f0 = (short *)local_b8;
    psStack_e8 = local_b0;
    puStack_e0 = puStack_a8;
    local_d8 = local_a0;
  }
  local_c8 = local_100;
  psStack_c0 = psStack_f8;
  local_b8 = (long)local_f0;
  local_b0 = psStack_e8;
  puStack_a8 = puStack_e0;
  local_a0 = local_d8;
  if (local_170 != (short *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_168,local_170,1);
  }
  if (local_158 != (short *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psStack_150,local_158,1);
  }
  if (bVar12) {
    if (psStack_140 != (short *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_138,psStack_140,1);
    }
    if ((psStack_128 != (short *)&DAT_8000000000000000) && (psStack_128 != (short *)0x0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,psStack_128,1);
    }
  }
  if (((ulong)psVar3 & 0x7fffffffffffffff) == 0) {
    if (((ulong)psVar5 & 0x7fffffffffffffff) == 0) goto LAB_10038e878;
LAB_10038e8a0:
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar6,psVar5,1);
    if (psVar1 == (short *)0x0) {
      bVar12 = true;
    }
    if (bVar12) goto LAB_10038e654;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psVar4,psVar3,1);
    if (((ulong)psVar5 & 0x7fffffffffffffff) != 0) goto LAB_10038e8a0;
LAB_10038e878:
    if (psVar1 == (short *)0x0) {
      bVar12 = true;
    }
    if (bVar12) goto LAB_10038e654;
  }
  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,psVar1,1);
LAB_10038e654:
  ::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_project_state::ProjectStateInspection>(auStack_250);
  return;
}

 (GhidraScript)
===END=== (GhidraScript)
