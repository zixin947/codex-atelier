/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* into_verified_parts @ 10033b7fc requested 10033b7fc */

/* codexmate_lib::core::relay::router_transition::TransitionOutcome<T>::into_verified_parts */

void codexmate_lib::core::relay::router_transition::TransitionOutcome<T>::into_verified_parts(long *param_1,long *param_2)

{
  long lVar1;
  long lVar2;
  long lVar3;
  undefined8 *puVar4;
  long lVar5;
  long lVar6;
  long lVar7;
  long lVar8;
  long lVar9;
  long lVar10;
  long lVar11;
  long lVar12;
  long lVar13;
  long lVar14;
  long lVar15;
  long lVar16;
  long lVar17;
  long lVar18;
  long lVar19;
  long lVar20;
  long *local_130;
  code *pcStack_128;
  undefined8 local_120;
  long local_88;
  long lStack_80;
  long local_78;
  undefined8 *local_70;
  undefined8 uStack_68;
  undefined8 local_60;

  if (*param_2 == 0) {
    lVar3 = param_2[0xc];
    lVar2 = param_2[0xb];
    lVar1 = param_2[0xd];
    lVar6 = param_2[0x10];
    lVar5 = param_2[0xf];
    param_1[0xd] = param_2[0xe];
    param_1[0xc] = lVar1;
    param_1[0xf] = lVar6;
    param_1[0xe] = lVar5;
    lVar1 = param_2[0x11];
    lVar6 = param_2[4];
    lVar5 = param_2[3];
    lVar7 = param_2[5];
    lVar9 = param_2[8];
    lVar8 = param_2[7];
    param_1[5] = param_2[6];
    param_1[4] = lVar7;
    param_1[7] = lVar9;
    param_1[6] = lVar8;
    lVar7 = param_2[9];
    param_1[9] = param_2[10];
    param_1[8] = lVar7;
    param_1[0xb] = lVar3;
    param_1[10] = lVar2;
    lVar2 = param_2[1];
    param_1[1] = param_2[2];
    *param_1 = lVar2;
    param_1[3] = lVar6;
    param_1[2] = lVar5;
    param_1[0x10] = lVar1;
    param_1[0x11] = 0;
    param_1[0x12] = 8;
    param_1[0x13] = 0;
  }
  else if (*param_2 == 1) {
    lVar3 = param_2[0x10];
    lVar6 = param_2[0x13];
    lVar5 = param_2[0x12];
    lVar1 = param_2[0x14];
    lVar8 = param_2[9];
    lVar7 = param_2[8];
    lVar10 = param_2[0xb];
    lVar9 = param_2[10];
    lVar14 = param_2[0xd];
    lVar13 = param_2[0xc];
    lVar12 = param_2[0xf];
    lVar11 = param_2[0xe];
    lVar18 = param_2[5];
    lVar17 = param_2[4];
    lVar16 = param_2[7];
    lVar15 = param_2[6];
    lVar20 = param_2[2];
    lVar19 = param_2[1];
    lVar2 = param_2[3];
    param_1[0xd] = param_2[0x11];
    param_1[0xc] = lVar3;
    param_1[0xf] = lVar6;
    param_1[0xe] = lVar5;
    param_1[5] = lVar8;
    param_1[4] = lVar7;
    param_1[7] = lVar10;
    param_1[6] = lVar9;
    param_1[9] = lVar14;
    param_1[8] = lVar13;
    param_1[0xb] = lVar12;
    param_1[10] = lVar11;
    param_1[1] = lVar18;
    *param_1 = lVar17;
    param_1[3] = lVar16;
    param_1[2] = lVar15;
    param_1[0x11] = lVar19;
    param_1[0x10] = lVar1;
    param_1[0x13] = lVar2;
    param_1[0x12] = lVar20;
  }
  else {
    lVar1 = param_2[1];
    lVar2 = param_2[2];
    lVar3 = param_2[3];
    alloc::str::join_generic_copy(&local_130,lVar2,lVar3,&_anon_81615f03ac42cf09f1e5e0315a2480f1_254,3);
    uStack_68 = pcStack_128;
    local_70 = local_130;
    local_60 = local_120;
    local_130 = (long *)&local_70;
    pcStack_128 = <alloc::string::String_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_88,_anon_81615f03ac42cf09f1e5e0315a2480f1_289,&local_130);
    if (local_70 != (long *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_68,local_70,1);
    }
    param_1[4] = local_78;
    param_1[1] = 10;
    *param_1 = 3;
    param_1[3] = lStack_80;
    param_1[2] = local_88;
    if (lVar3 != 0) {
      puVar4 = (undefined8 *)(lVar2 + 8);
      do {
        if (puVar4[-1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar4,puVar4[-1],1);
        }
        puVar4 = puVar4 + 3;
        lVar3 = lVar3 + -1;
      } while (lVar3 != 0);
    }
    if (lVar1 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar2,lVar1 * 0x18,8);
    }
    ::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_diagnostic::FixResult>(param_2 + 0xc);
    if (param_2[4] != 2) {
      ::core::ptr::drop_in_place<codexmate_lib::core::relay::manager::ReconciledRouterTarget>(param_2 + 4);
    }
  }
  return;
}
