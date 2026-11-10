/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* observe @ 1007c05c4 requested 1007c05c4 */

/* codexmate_lib::platform::process::WriterGenerationTracker::observe */

undefined4 codexmate_lib::platform::process::WriterGenerationTracker::observe(ulong *param_1,long param_2,ulong param_3)

{
  uint uVar1;
  long lVar2;
  uint *puVar3;
  ulong uVar4;
  uint *puVar5;
  long lVar6;
  ulong uVar7;
  undefined4 *puVar8;
  uint *puVar9;
  uint uVar10;
  long lVar11;
  ulong uVar12;
  long lVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  undefined4 uVar20;
  undefined4 uVar21;
  undefined4 uVar22;
  undefined4 uVar23;
  undefined4 uVar24;
  undefined4 uVar25;
  undefined4 uVar26;
  undefined4 uVar27;
  undefined1 uStack_41;
  
  if (param_3 == 0) {
    param_1[2] = 0;
    *(undefined2 *)(param_1 + 3) = 1;
    *(undefined1 *)((long)param_1 + 0x1a) = 0;
    return 0;
  }
  lVar13 = param_3 << 2;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  puVar3 = (uint *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(lVar13,4);
  if (puVar3 == (uint *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(4,lVar13);
  }
  if (param_3 < 5) {
    uVar4 = 0;
  }
  else {
    if (param_3 < 0x11) {
      uVar12 = 0;
    }
    else {
      uVar7 = 0x10;
      if ((param_3 & 0xf) != 0) {
        uVar7 = param_3 & 0xf;
      }
      uVar4 = param_3 - uVar7;
      puVar8 = (undefined4 *)(param_2 + 0x100);
      puVar5 = puVar3 + 8;
      uVar12 = param_3;
      do {
        uVar14 = puVar8[-0x40];
        uVar15 = puVar8[-0x38];
        uVar16 = puVar8[-0x20];
        uVar17 = puVar8[-0x18];
        uVar18 = puVar8[-0x10];
        uVar19 = puVar8[-8];
        uVar20 = *puVar8;
        uVar21 = puVar8[8];
        uVar22 = puVar8[0x10];
        uVar23 = puVar8[0x18];
        uVar24 = puVar8[0x20];
        uVar25 = puVar8[0x28];
        uVar26 = puVar8[0x30];
        uVar27 = puVar8[0x38];
        *(ulong *)(puVar5 + -6) = CONCAT44(puVar8[-0x28],puVar8[-0x30]);
        *(ulong *)(puVar5 + -8) = CONCAT44(uVar15,uVar14);
        *(ulong *)(puVar5 + -2) = CONCAT44(uVar19,uVar18);
        *(ulong *)(puVar5 + -4) = CONCAT44(uVar17,uVar16);
        *(ulong *)(puVar5 + 2) = CONCAT44(uVar23,uVar22);
        *(ulong *)puVar5 = CONCAT44(uVar21,uVar20);
        *(ulong *)(puVar5 + 6) = CONCAT44(uVar27,uVar26);
        *(ulong *)(puVar5 + 4) = CONCAT44(uVar25,uVar24);
        uVar12 = uVar12 - 0x10;
        puVar8 = puVar8 + 0x80;
        puVar5 = puVar5 + 0x10;
      } while (uVar7 != uVar12);
      uVar12 = uVar4;
      if (uVar7 < 5) goto LAB_1007c073c;
    }
    uVar7 = 4;
    if ((param_3 & 3) != 0) {
      uVar7 = param_3 & 3;
    }
    uVar4 = param_3 - uVar7;
    lVar6 = (uVar7 + uVar12) - param_3;
    puVar8 = (undefined4 *)(param_2 + uVar12 * 0x20 + 0x40);
    puVar5 = puVar3 + uVar12;
    do {
      uVar14 = puVar8[-0x10];
      uVar15 = puVar8[-8];
      *(ulong *)(puVar5 + 2) = CONCAT44(puVar8[8],*puVar8);
      *(ulong *)puVar5 = CONCAT44(uVar15,uVar14);
      puVar8 = puVar8 + 0x20;
      lVar6 = lVar6 + 4;
      puVar5 = puVar5 + 4;
    } while (lVar6 != 0);
  }
LAB_1007c073c:
  puVar5 = (uint *)(param_2 + uVar4 * 0x20);
  do {
    puVar3[uVar4] = *puVar5;
    uVar4 = uVar4 + 1;
    puVar5 = puVar5 + 8;
  } while (param_3 != uVar4);
  if (param_3 == 1) {
    uVar4 = 1;
  }
  else {
    if (param_3 < 0x15) {
      lVar6 = 0;
      puVar5 = puVar3 + 1;
      do {
        uVar10 = puVar5[-1];
        uVar1 = *puVar5;
        lVar2 = lVar6;
        if (uVar1 < uVar10) {
          do {
            lVar11 = lVar2;
            *(uint *)((long)puVar3 + lVar11 + 4) = uVar10;
            puVar9 = puVar3;
            if (lVar11 == 0) goto LAB_1007c0894;
            uVar10 = *(uint *)((long)puVar3 + lVar11 + -4);
            lVar2 = lVar11 + -4;
          } while (uVar1 < uVar10);
          puVar9 = (uint *)((long)puVar3 + lVar11);
LAB_1007c0894:
          *puVar9 = uVar1;
        }
        puVar5 = puVar5 + 1;
        lVar6 = lVar6 + 4;
      } while (puVar5 != puVar3 + param_3);
    }
    else {
      ::core::slice::sort::unstable::ipnsort(puVar3,param_3,&uStack_41);
    }
    lVar6 = 0;
    do {
      if ((puVar3 + lVar6)[1] == puVar3[lVar6]) {
        uVar7 = lVar6 + 2;
        uVar4 = lVar6 + 1;
        if (uVar7 < param_3) {
          do {
            if (puVar3[uVar7] != (puVar3 + uVar4)[-1]) {
              puVar3[uVar4] = puVar3[uVar7];
              uVar4 = uVar4 + 1;
            }
            uVar7 = uVar7 + 1;
          } while (param_3 != uVar7);
          if (uVar4 == 0) {
            param_1[2] = 0;
            *(undefined2 *)(param_1 + 3) = 1;
            *(undefined1 *)((long)param_1 + 0x1a) = 0;
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar3,lVar13,4);
            return 0;
          }
        }
        break;
      }
      lVar6 = lVar6 + 1;
      uVar4 = param_3;
    } while (param_3 - 1 != lVar6);
  }
  uVar7 = param_1[2];
  if (uVar7 == 0) {
    *(undefined2 *)((long)param_1 + 0x19) = 0;
    uVar14 = 4;
    if ((char)param_1[3] == '\0') {
      uVar14 = 1;
    }
    uVar7 = *param_1;
  }
  else {
    puVar5 = (uint *)param_1[1];
    if (uVar7 == 1) {
      lVar13 = uVar4 << 2;
      puVar9 = puVar3;
      do {
        if (lVar13 == 0) goto LAB_1007c0818;
        uVar10 = *puVar9;
        lVar13 = lVar13 + -4;
        puVar9 = puVar9 + 1;
      } while (*puVar5 == uVar10);
    }
    else {
      puVar9 = puVar3;
      do {
        if (puVar9 == puVar3 + uVar4) goto LAB_1007c0818;
        lVar13 = 0;
        uVar10 = *puVar9;
        uVar12 = uVar7;
        do {
          lVar6 = (uVar12 >> 1) + lVar13;
          if (puVar5[lVar6] <= uVar10) {
            lVar13 = lVar6;
          }
          uVar12 = uVar12 - (uVar12 >> 1);
        } while (1 < uVar12);
        puVar9 = puVar9 + 1;
      } while (puVar5[lVar13] == uVar10);
    }
    *(undefined2 *)((long)param_1 + 0x19) = 0;
    uVar14 = 4;
    uVar7 = *param_1;
  }
joined_r0x0001007c0840:
  if (uVar7 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(param_1[1],uVar7 << 2,4);
  }
  *param_1 = param_3;
  param_1[1] = (ulong)puVar3;
  param_1[2] = uVar4;
  return uVar14;
LAB_1007c0818:
  uVar14 = 2;
  if ((*(byte *)((long)param_1 + 0x19) & 1) != 0 || 2 < *(byte *)((long)param_1 + 0x1a)) {
    uVar14 = 3;
  }
  uVar7 = *param_1;
  goto joined_r0x0001007c0840;
}

