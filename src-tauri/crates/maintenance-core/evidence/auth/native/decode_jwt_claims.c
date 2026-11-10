/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* decode_jwt_claims @ 1001ec270 requested 1001ec270 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::auth::decode_jwt_claims */

void codexmate_lib::core::auth::decode_jwt_claims(undefined8 *param_1,long param_2,long param_3)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  byte bVar7;
  byte bVar8;
  byte bVar9;
  byte bVar10;
  byte bVar11;
  byte bVar12;
  byte bVar13;
  byte bVar14;
  byte bVar15;
  byte bVar16;
  byte bVar17;
  byte bVar18;
  byte bVar19;
  byte bVar20;
  byte bVar21;
  byte bVar22;
  byte bVar23;
  byte bVar24;
  byte bVar25;
  byte bVar26;
  byte bVar27;
  byte bVar28;
  byte bVar29;
  byte bVar30;
  byte bVar31;
  byte bVar32;
  byte bVar33;
  byte bVar34;
  byte bVar35;
  byte bVar36;
  byte bVar37;
  byte bVar38;
  byte bVar39;
  byte bVar40;
  byte bVar41;
  byte bVar42;
  byte bVar43;
  byte bVar44;
  byte bVar45;
  byte bVar46;
  byte bVar47;
  byte bVar48;
  byte bVar49;
  byte bVar50;
  byte bVar51;
  byte bVar52;
  byte bVar53;
  byte bVar54;
  byte bVar55;
  byte bVar56;
  byte bVar57;
  byte bVar58;
  byte bVar59;
  byte bVar60;
  byte bVar61;
  byte bVar62;
  byte bVar63;
  byte bVar64;
  long lVar65;
  undefined1 (*pauVar66) [16];
  long lVar67;
  uint uVar68;
  undefined8 *******pppppppuVar69;
  undefined8 *******pppppppuVar70;
  long lVar71;
  byte *pbVar72;
  uint uVar73;
  ulong *puVar74;
  byte *pbVar75;
  ulong *puVar76;
  undefined8 uVar77;
  undefined8 *******pppppppuVar78;
  long lVar79;
  undefined8 *******pppppppuVar80;
  ulong uVar81;
  undefined1 auVar82 [16];
  long local_198;
  long local_190;
  ulong local_188;
  undefined8 local_180;
  long lStack_178;
  long local_170;
  long lStack_168;
  undefined8 local_160;
  long lStack_158;
  undefined8 local_150;
  undefined1 local_148;
  undefined2 local_140;
  undefined8 *******local_138;
  long local_130;
  undefined8 *******local_128;
  undefined8 *******local_120;
  long lStack_118;
  undefined8 *******local_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 *******local_f0;
  long lStack_e8;
  undefined8 *******local_e0;
  long local_d8;
  undefined8 *******local_d0;
  undefined8 *******local_c8;
  undefined8 uStack_c0;
  undefined1 local_b8;
  undefined8 *******local_b0;
  long lStack_a8;
  undefined8 *******local_a0;
  undefined8 uStack_98;
  undefined8 local_78 [3];

  if ((param_2 == 0) || (param_3 == 0)) {
    pauVar66 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
    if (pauVar66[1][0] == '\x01') {
      auVar82 = *pauVar66;
    }
    else {
      auVar82 = std::sys::random::hashmap_random_keys();
      *pauVar66 = auVar82;
      pauVar66[1][0] = 1;
    }
    *(long *)*pauVar66 = auVar82._0_8_ + 1;
    param_1[1] = 0;
    *param_1 = &DAT_101303e80;
    param_1[3] = 0;
    param_1[2] = 0;
    *(undefined1 (*) [16])(param_1 + 4) = auVar82;
    return;
  }
  local_180 = 0;
  local_160 = 0;
  local_150 = 0x2e0000002e;
  local_148 = 1;
  local_140 = 1;
  lStack_178 = param_3;
  local_170 = param_2;
  lStack_168 = param_3;
  lStack_158 = param_3;
  <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_198,&local_180);
  lVar65 = local_190;
  if (local_188 < 2) {
    pauVar66 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
    if (pauVar66[1][0] == '\x01') {
      auVar82 = *pauVar66;
    }
    else {
      auVar82 = std::sys::random::hashmap_random_keys();
      *pauVar66 = auVar82;
      pauVar66[1][0] = 1;
    }
    *(long *)*pauVar66 = auVar82._0_8_ + 1;
    param_1[1] = 0;
    *param_1 = &DAT_101303e80;
    param_1[3] = 0;
    param_1[2] = 0;
    *(undefined1 (*) [16])(param_1 + 4) = auVar82;
    goto joined_r0x0001001ec884;
  }
  pppppppuVar78 = *(undefined8 ********)(local_190 + 0x18);
  if ((long)pppppppuVar78 < 0) {
    uVar77 = 0;
LAB_1001ec378:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar77,pppppppuVar78);
  }
  local_138 = pppppppuVar78;
  local_128 = pppppppuVar78;
  if (pppppppuVar78 == (undefined8 *******)0x0) {
    local_130 = 1;
    uVar81 = 0;
    pppppppuVar69 = pppppppuVar78;
    lVar67 = local_130;
    pppppppuVar70 = pppppppuVar78;
  }
  else {
    lVar79 = *(long *)(local_190 + 0x10);
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar77 = 1;
    lVar67 = __RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(pppppppuVar78,1);
    if (lVar67 == 0) goto LAB_1001ec378;
    pppppppuVar69 = (undefined8 *******)0x0;
    if ((pppppppuVar78 < &SUB_00000008) || ((ulong)(lVar67 - lVar79) < 0x40)) {
LAB_1001ec490:
      lVar71 = (long)pppppppuVar78 - (long)pppppppuVar69;
      pbVar72 = (byte *)(lVar79 + (long)pppppppuVar69);
      pbVar75 = (byte *)(lVar67 + (long)pppppppuVar69);
      do {
        bVar1 = 0x2b;
        if (*pbVar72 != 0x2d) {
          bVar1 = *pbVar72;
        }
        *pbVar75 = bVar1;
        lVar71 = lVar71 + -1;
        pbVar72 = pbVar72 + 1;
        pbVar75 = pbVar75 + 1;
      } while (lVar71 != 0);
    }
    else if (&DAT_0000003f < pppppppuVar78) {
      pppppppuVar69 = (undefined8 *******)((ulong)pppppppuVar78 & 0x7fffffffffffffc0);
      pbVar72 = (byte *)(lVar79 + 0x20);
      pbVar75 = (byte *)(lVar67 + 0x20);
      pppppppuVar70 = pppppppuVar69;
      do {
        bVar1 = pbVar72[-0x20];
        bVar2 = pbVar72[-0x1f];
        bVar3 = pbVar72[-0x1e];
        bVar4 = pbVar72[-0x1d];
        bVar5 = pbVar72[-0x1c];
        bVar6 = pbVar72[-0x1b];
        bVar7 = pbVar72[-0x1a];
        bVar8 = pbVar72[-0x19];
        bVar9 = pbVar72[-0x18];
        bVar10 = pbVar72[-0x17];
        bVar11 = pbVar72[-0x16];
        bVar12 = pbVar72[-0x15];
        bVar13 = pbVar72[-0x14];
        bVar14 = pbVar72[-0x13];
        bVar15 = pbVar72[-0x12];
        bVar16 = pbVar72[-0x11];
        bVar17 = pbVar72[-0x10];
        bVar18 = pbVar72[-0xf];
        bVar19 = pbVar72[-0xe];
        bVar20 = pbVar72[-0xd];
        bVar21 = pbVar72[-0xc];
        bVar22 = pbVar72[-0xb];
        bVar23 = pbVar72[-10];
        bVar24 = pbVar72[-9];
        bVar25 = pbVar72[-8];
        bVar26 = pbVar72[-7];
        bVar27 = pbVar72[-6];
        bVar28 = pbVar72[-5];
        bVar29 = pbVar72[-4];
        bVar30 = pbVar72[-3];
        bVar31 = pbVar72[-2];
        bVar32 = pbVar72[-1];
        bVar33 = *pbVar72;
        bVar34 = pbVar72[1];
        bVar35 = pbVar72[2];
        bVar36 = pbVar72[3];
        bVar37 = pbVar72[4];
        bVar38 = pbVar72[5];
        bVar39 = pbVar72[6];
        bVar40 = pbVar72[7];
        bVar41 = pbVar72[8];
        bVar42 = pbVar72[9];
        bVar43 = pbVar72[10];
        bVar44 = pbVar72[0xb];
        bVar45 = pbVar72[0xc];
        bVar46 = pbVar72[0xd];
        bVar47 = pbVar72[0xe];
        bVar48 = pbVar72[0xf];
        bVar49 = pbVar72[0x10];
        bVar50 = pbVar72[0x11];
        bVar51 = pbVar72[0x12];
        bVar52 = pbVar72[0x13];
        bVar53 = pbVar72[0x14];
        bVar54 = pbVar72[0x15];
        bVar55 = pbVar72[0x16];
        bVar56 = pbVar72[0x17];
        bVar57 = pbVar72[0x18];
        bVar58 = pbVar72[0x19];
        bVar59 = pbVar72[0x1a];
        bVar60 = pbVar72[0x1b];
        bVar61 = pbVar72[0x1c];
        bVar62 = pbVar72[0x1d];
        bVar63 = pbVar72[0x1e];
        bVar64 = pbVar72[0x1f];
        pbVar75[-0x18] = bVar9 ^ (bVar9 ^ 0x2b) & -(bVar9 == 0x2d);
        pbVar75[-0x17] = bVar10 ^ (bVar10 ^ 0x2b) & -(bVar10 == 0x2d);
        pbVar75[-0x16] = bVar11 ^ (bVar11 ^ 0x2b) & -(bVar11 == 0x2d);
        pbVar75[-0x15] = bVar12 ^ (bVar12 ^ 0x2b) & -(bVar12 == 0x2d);
        pbVar75[-0x14] = bVar13 ^ (bVar13 ^ 0x2b) & -(bVar13 == 0x2d);
        pbVar75[-0x13] = bVar14 ^ (bVar14 ^ 0x2b) & -(bVar14 == 0x2d);
        pbVar75[-0x12] = bVar15 ^ (bVar15 ^ 0x2b) & -(bVar15 == 0x2d);
        pbVar75[-0x11] = bVar16 ^ (bVar16 ^ 0x2b) & -(bVar16 == 0x2d);
        pbVar75[-0x20] = bVar1 ^ (bVar1 ^ 0x2b) & -(bVar1 == 0x2d);
        pbVar75[-0x1f] = bVar2 ^ (bVar2 ^ 0x2b) & -(bVar2 == 0x2d);
        pbVar75[-0x1e] = bVar3 ^ (bVar3 ^ 0x2b) & -(bVar3 == 0x2d);
        pbVar75[-0x1d] = bVar4 ^ (bVar4 ^ 0x2b) & -(bVar4 == 0x2d);
        pbVar75[-0x1c] = bVar5 ^ (bVar5 ^ 0x2b) & -(bVar5 == 0x2d);
        pbVar75[-0x1b] = bVar6 ^ (bVar6 ^ 0x2b) & -(bVar6 == 0x2d);
        pbVar75[-0x1a] = bVar7 ^ (bVar7 ^ 0x2b) & -(bVar7 == 0x2d);
        pbVar75[-0x19] = bVar8 ^ (bVar8 ^ 0x2b) & -(bVar8 == 0x2d);
        pbVar75[-8] = bVar25 ^ (bVar25 ^ 0x2b) & -(bVar25 == 0x2d);
        pbVar75[-7] = bVar26 ^ (bVar26 ^ 0x2b) & -(bVar26 == 0x2d);
        pbVar75[-6] = bVar27 ^ (bVar27 ^ 0x2b) & -(bVar27 == 0x2d);
        pbVar75[-5] = bVar28 ^ (bVar28 ^ 0x2b) & -(bVar28 == 0x2d);
        pbVar75[-4] = bVar29 ^ (bVar29 ^ 0x2b) & -(bVar29 == 0x2d);
        pbVar75[-3] = bVar30 ^ (bVar30 ^ 0x2b) & -(bVar30 == 0x2d);
        pbVar75[-2] = bVar31 ^ (bVar31 ^ 0x2b) & -(bVar31 == 0x2d);
        pbVar75[-1] = bVar32 ^ (bVar32 ^ 0x2b) & -(bVar32 == 0x2d);
        pbVar75[-0x10] = bVar17 ^ (bVar17 ^ 0x2b) & -(bVar17 == 0x2d);
        pbVar75[-0xf] = bVar18 ^ (bVar18 ^ 0x2b) & -(bVar18 == 0x2d);
        pbVar75[-0xe] = bVar19 ^ (bVar19 ^ 0x2b) & -(bVar19 == 0x2d);
        pbVar75[-0xd] = bVar20 ^ (bVar20 ^ 0x2b) & -(bVar20 == 0x2d);
        pbVar75[-0xc] = bVar21 ^ (bVar21 ^ 0x2b) & -(bVar21 == 0x2d);
        pbVar75[-0xb] = bVar22 ^ (bVar22 ^ 0x2b) & -(bVar22 == 0x2d);
        pbVar75[-10] = bVar23 ^ (bVar23 ^ 0x2b) & -(bVar23 == 0x2d);
        pbVar75[-9] = bVar24 ^ (bVar24 ^ 0x2b) & -(bVar24 == 0x2d);
        pbVar75[8] = bVar41 ^ (bVar41 ^ 0x2b) & -(bVar41 == 0x2d);
        pbVar75[9] = bVar42 ^ (bVar42 ^ 0x2b) & -(bVar42 == 0x2d);
        pbVar75[10] = bVar43 ^ (bVar43 ^ 0x2b) & -(bVar43 == 0x2d);
        pbVar75[0xb] = bVar44 ^ (bVar44 ^ 0x2b) & -(bVar44 == 0x2d);
        pbVar75[0xc] = bVar45 ^ (bVar45 ^ 0x2b) & -(bVar45 == 0x2d);
        pbVar75[0xd] = bVar46 ^ (bVar46 ^ 0x2b) & -(bVar46 == 0x2d);
        pbVar75[0xe] = bVar47 ^ (bVar47 ^ 0x2b) & -(bVar47 == 0x2d);
        pbVar75[0xf] = bVar48 ^ (bVar48 ^ 0x2b) & -(bVar48 == 0x2d);
        *pbVar75 = bVar33 ^ (bVar33 ^ 0x2b) & -(bVar33 == 0x2d);
        pbVar75[1] = bVar34 ^ (bVar34 ^ 0x2b) & -(bVar34 == 0x2d);
        pbVar75[2] = bVar35 ^ (bVar35 ^ 0x2b) & -(bVar35 == 0x2d);
        pbVar75[3] = bVar36 ^ (bVar36 ^ 0x2b) & -(bVar36 == 0x2d);
        pbVar75[4] = bVar37 ^ (bVar37 ^ 0x2b) & -(bVar37 == 0x2d);
        pbVar75[5] = bVar38 ^ (bVar38 ^ 0x2b) & -(bVar38 == 0x2d);
        pbVar75[6] = bVar39 ^ (bVar39 ^ 0x2b) & -(bVar39 == 0x2d);
        pbVar75[7] = bVar40 ^ (bVar40 ^ 0x2b) & -(bVar40 == 0x2d);
        pbVar75[0x18] = bVar57 ^ (bVar57 ^ 0x2b) & -(bVar57 == 0x2d);
        pbVar75[0x19] = bVar58 ^ (bVar58 ^ 0x2b) & -(bVar58 == 0x2d);
        pbVar75[0x1a] = bVar59 ^ (bVar59 ^ 0x2b) & -(bVar59 == 0x2d);
        pbVar75[0x1b] = bVar60 ^ (bVar60 ^ 0x2b) & -(bVar60 == 0x2d);
        pbVar75[0x1c] = bVar61 ^ (bVar61 ^ 0x2b) & -(bVar61 == 0x2d);
        pbVar75[0x1d] = bVar62 ^ (bVar62 ^ 0x2b) & -(bVar62 == 0x2d);
        pbVar75[0x1e] = bVar63 ^ (bVar63 ^ 0x2b) & -(bVar63 == 0x2d);
        pbVar75[0x1f] = bVar64 ^ (bVar64 ^ 0x2b) & -(bVar64 == 0x2d);
        pbVar75[0x10] = bVar49 ^ (bVar49 ^ 0x2b) & -(bVar49 == 0x2d);
        pbVar75[0x11] = bVar50 ^ (bVar50 ^ 0x2b) & -(bVar50 == 0x2d);
        pbVar75[0x12] = bVar51 ^ (bVar51 ^ 0x2b) & -(bVar51 == 0x2d);
        pbVar75[0x13] = bVar52 ^ (bVar52 ^ 0x2b) & -(bVar52 == 0x2d);
        pbVar75[0x14] = bVar53 ^ (bVar53 ^ 0x2b) & -(bVar53 == 0x2d);
        pbVar75[0x15] = bVar54 ^ (bVar54 ^ 0x2b) & -(bVar54 == 0x2d);
        pbVar75[0x16] = bVar55 ^ (bVar55 ^ 0x2b) & -(bVar55 == 0x2d);
        pbVar75[0x17] = bVar56 ^ (bVar56 ^ 0x2b) & -(bVar56 == 0x2d);
        pppppppuVar70 = pppppppuVar70 + -8;
        pbVar72 = pbVar72 + 0x40;
        pbVar75 = pbVar75 + 0x40;
      } while (pppppppuVar70 != (undefined8 *******)0x0);
      if (pppppppuVar78 != pppppppuVar69) {
        pppppppuVar70 = pppppppuVar69;
        if (((ulong)pppppppuVar78 & 0x38) == 0) goto LAB_1001ec490;
        goto LAB_1001ec454;
      }
    }
    else {
      pppppppuVar70 = (undefined8 *******)0x0;
LAB_1001ec454:
      pppppppuVar69 = (undefined8 *******)((ulong)pppppppuVar78 & 0x7ffffffffffffff8);
      lVar71 = (long)pppppppuVar70 - (long)pppppppuVar69;
      puVar74 = (ulong *)(lVar67 + (long)pppppppuVar70);
      puVar76 = (ulong *)(lVar79 + (long)pppppppuVar70);
      do {
        uVar81 = *puVar76;
        *puVar74 = uVar81 ^ (uVar81 ^ 0x2b2b2b2b2b2b2b2b) &
                            CONCAT17(-((char)(uVar81 >> 0x38) == '-'),
                                     CONCAT16(-((char)(uVar81 >> 0x30) == '-'),
                                              CONCAT15(-((char)(uVar81 >> 0x28) == '-'),
                                                       CONCAT14(-((char)(uVar81 >> 0x20) == '-'),
                                                                CONCAT13(-((char)(uVar81 >> 0x18) == '-'),
                                                                         CONCAT12(-((char)(uVar81 >> 0x10) == '-'),
                                                                                  CONCAT11(-((char)(uVar81 >> 8) == '-'),-((char)uVar81 == '-'))))))));
        lVar71 = lVar71 + 8;
        puVar74 = puVar74 + 1;
        puVar76 = puVar76 + 1;
      } while (lVar71 != 0);
      if (pppppppuVar78 != pppppppuVar69) goto LAB_1001ec490;
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    local_130 = __RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(pppppppuVar78,1);
    if (local_130 == 0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,pppppppuVar78);
    }
    if (&DAT_00000007 < pppppppuVar78) {
      if (&DAT_0000003f < pppppppuVar78) {
        pppppppuVar70 = (undefined8 *******)((ulong)pppppppuVar78 & 0x7fffffffffffffc0);
        pbVar72 = (byte *)(lVar67 + 0x20);
        pbVar75 = (byte *)(local_130 + 0x20);
        pppppppuVar69 = pppppppuVar70;
        do {
          bVar1 = pbVar72[-0x20];
          bVar2 = pbVar72[-0x1f];
          bVar3 = pbVar72[-0x1e];
          bVar4 = pbVar72[-0x1d];
          bVar5 = pbVar72[-0x1c];
          bVar6 = pbVar72[-0x1b];
          bVar7 = pbVar72[-0x1a];
          bVar8 = pbVar72[-0x19];
          bVar9 = pbVar72[-0x18];
          bVar10 = pbVar72[-0x17];
          bVar11 = pbVar72[-0x16];
          bVar12 = pbVar72[-0x15];
          bVar13 = pbVar72[-0x14];
          bVar14 = pbVar72[-0x13];
          bVar15 = pbVar72[-0x12];
          bVar16 = pbVar72[-0x11];
          bVar17 = pbVar72[-0x10];
          bVar18 = pbVar72[-0xf];
          bVar19 = pbVar72[-0xe];
          bVar20 = pbVar72[-0xd];
          bVar21 = pbVar72[-0xc];
          bVar22 = pbVar72[-0xb];
          bVar23 = pbVar72[-10];
          bVar24 = pbVar72[-9];
          bVar25 = pbVar72[-8];
          bVar26 = pbVar72[-7];
          bVar27 = pbVar72[-6];
          bVar28 = pbVar72[-5];
          bVar29 = pbVar72[-4];
          bVar30 = pbVar72[-3];
          bVar31 = pbVar72[-2];
          bVar32 = pbVar72[-1];
          bVar33 = *pbVar72;
          bVar34 = pbVar72[1];
          bVar35 = pbVar72[2];
          bVar36 = pbVar72[3];
          bVar37 = pbVar72[4];
          bVar38 = pbVar72[5];
          bVar39 = pbVar72[6];
          bVar40 = pbVar72[7];
          bVar41 = pbVar72[8];
          bVar42 = pbVar72[9];
          bVar43 = pbVar72[10];
          bVar44 = pbVar72[0xb];
          bVar45 = pbVar72[0xc];
          bVar46 = pbVar72[0xd];
          bVar47 = pbVar72[0xe];
          bVar48 = pbVar72[0xf];
          bVar49 = pbVar72[0x10];
          bVar50 = pbVar72[0x11];
          bVar51 = pbVar72[0x12];
          bVar52 = pbVar72[0x13];
          bVar53 = pbVar72[0x14];
          bVar54 = pbVar72[0x15];
          bVar55 = pbVar72[0x16];
          bVar56 = pbVar72[0x17];
          bVar57 = pbVar72[0x18];
          bVar58 = pbVar72[0x19];
          bVar59 = pbVar72[0x1a];
          bVar60 = pbVar72[0x1b];
          bVar61 = pbVar72[0x1c];
          bVar62 = pbVar72[0x1d];
          bVar63 = pbVar72[0x1e];
          bVar64 = pbVar72[0x1f];
          pbVar75[-0x18] = bVar9 ^ (bVar9 ^ 0x2f) & -(bVar9 == 0x5f);
          pbVar75[-0x17] = bVar10 ^ (bVar10 ^ 0x2f) & -(bVar10 == 0x5f);
          pbVar75[-0x16] = bVar11 ^ (bVar11 ^ 0x2f) & -(bVar11 == 0x5f);
          pbVar75[-0x15] = bVar12 ^ (bVar12 ^ 0x2f) & -(bVar12 == 0x5f);
          pbVar75[-0x14] = bVar13 ^ (bVar13 ^ 0x2f) & -(bVar13 == 0x5f);
          pbVar75[-0x13] = bVar14 ^ (bVar14 ^ 0x2f) & -(bVar14 == 0x5f);
          pbVar75[-0x12] = bVar15 ^ (bVar15 ^ 0x2f) & -(bVar15 == 0x5f);
          pbVar75[-0x11] = bVar16 ^ (bVar16 ^ 0x2f) & -(bVar16 == 0x5f);
          pbVar75[-0x20] = bVar1 ^ (bVar1 ^ 0x2f) & -(bVar1 == 0x5f);
          pbVar75[-0x1f] = bVar2 ^ (bVar2 ^ 0x2f) & -(bVar2 == 0x5f);
          pbVar75[-0x1e] = bVar3 ^ (bVar3 ^ 0x2f) & -(bVar3 == 0x5f);
          pbVar75[-0x1d] = bVar4 ^ (bVar4 ^ 0x2f) & -(bVar4 == 0x5f);
          pbVar75[-0x1c] = bVar5 ^ (bVar5 ^ 0x2f) & -(bVar5 == 0x5f);
          pbVar75[-0x1b] = bVar6 ^ (bVar6 ^ 0x2f) & -(bVar6 == 0x5f);
          pbVar75[-0x1a] = bVar7 ^ (bVar7 ^ 0x2f) & -(bVar7 == 0x5f);
          pbVar75[-0x19] = bVar8 ^ (bVar8 ^ 0x2f) & -(bVar8 == 0x5f);
          pbVar75[-8] = bVar25 ^ (bVar25 ^ 0x2f) & -(bVar25 == 0x5f);
          pbVar75[-7] = bVar26 ^ (bVar26 ^ 0x2f) & -(bVar26 == 0x5f);
          pbVar75[-6] = bVar27 ^ (bVar27 ^ 0x2f) & -(bVar27 == 0x5f);
          pbVar75[-5] = bVar28 ^ (bVar28 ^ 0x2f) & -(bVar28 == 0x5f);
          pbVar75[-4] = bVar29 ^ (bVar29 ^ 0x2f) & -(bVar29 == 0x5f);
          pbVar75[-3] = bVar30 ^ (bVar30 ^ 0x2f) & -(bVar30 == 0x5f);
          pbVar75[-2] = bVar31 ^ (bVar31 ^ 0x2f) & -(bVar31 == 0x5f);
          pbVar75[-1] = bVar32 ^ (bVar32 ^ 0x2f) & -(bVar32 == 0x5f);
          pbVar75[-0x10] = bVar17 ^ (bVar17 ^ 0x2f) & -(bVar17 == 0x5f);
          pbVar75[-0xf] = bVar18 ^ (bVar18 ^ 0x2f) & -(bVar18 == 0x5f);
          pbVar75[-0xe] = bVar19 ^ (bVar19 ^ 0x2f) & -(bVar19 == 0x5f);
          pbVar75[-0xd] = bVar20 ^ (bVar20 ^ 0x2f) & -(bVar20 == 0x5f);
          pbVar75[-0xc] = bVar21 ^ (bVar21 ^ 0x2f) & -(bVar21 == 0x5f);
          pbVar75[-0xb] = bVar22 ^ (bVar22 ^ 0x2f) & -(bVar22 == 0x5f);
          pbVar75[-10] = bVar23 ^ (bVar23 ^ 0x2f) & -(bVar23 == 0x5f);
          pbVar75[-9] = bVar24 ^ (bVar24 ^ 0x2f) & -(bVar24 == 0x5f);
          pbVar75[8] = bVar41 ^ (bVar41 ^ 0x2f) & -(bVar41 == 0x5f);
          pbVar75[9] = bVar42 ^ (bVar42 ^ 0x2f) & -(bVar42 == 0x5f);
          pbVar75[10] = bVar43 ^ (bVar43 ^ 0x2f) & -(bVar43 == 0x5f);
          pbVar75[0xb] = bVar44 ^ (bVar44 ^ 0x2f) & -(bVar44 == 0x5f);
          pbVar75[0xc] = bVar45 ^ (bVar45 ^ 0x2f) & -(bVar45 == 0x5f);
          pbVar75[0xd] = bVar46 ^ (bVar46 ^ 0x2f) & -(bVar46 == 0x5f);
          pbVar75[0xe] = bVar47 ^ (bVar47 ^ 0x2f) & -(bVar47 == 0x5f);
          pbVar75[0xf] = bVar48 ^ (bVar48 ^ 0x2f) & -(bVar48 == 0x5f);
          *pbVar75 = bVar33 ^ (bVar33 ^ 0x2f) & -(bVar33 == 0x5f);
          pbVar75[1] = bVar34 ^ (bVar34 ^ 0x2f) & -(bVar34 == 0x5f);
          pbVar75[2] = bVar35 ^ (bVar35 ^ 0x2f) & -(bVar35 == 0x5f);
          pbVar75[3] = bVar36 ^ (bVar36 ^ 0x2f) & -(bVar36 == 0x5f);
          pbVar75[4] = bVar37 ^ (bVar37 ^ 0x2f) & -(bVar37 == 0x5f);
          pbVar75[5] = bVar38 ^ (bVar38 ^ 0x2f) & -(bVar38 == 0x5f);
          pbVar75[6] = bVar39 ^ (bVar39 ^ 0x2f) & -(bVar39 == 0x5f);
          pbVar75[7] = bVar40 ^ (bVar40 ^ 0x2f) & -(bVar40 == 0x5f);
          pbVar75[0x18] = bVar57 ^ (bVar57 ^ 0x2f) & -(bVar57 == 0x5f);
          pbVar75[0x19] = bVar58 ^ (bVar58 ^ 0x2f) & -(bVar58 == 0x5f);
          pbVar75[0x1a] = bVar59 ^ (bVar59 ^ 0x2f) & -(bVar59 == 0x5f);
          pbVar75[0x1b] = bVar60 ^ (bVar60 ^ 0x2f) & -(bVar60 == 0x5f);
          pbVar75[0x1c] = bVar61 ^ (bVar61 ^ 0x2f) & -(bVar61 == 0x5f);
          pbVar75[0x1d] = bVar62 ^ (bVar62 ^ 0x2f) & -(bVar62 == 0x5f);
          pbVar75[0x1e] = bVar63 ^ (bVar63 ^ 0x2f) & -(bVar63 == 0x5f);
          pbVar75[0x1f] = bVar64 ^ (bVar64 ^ 0x2f) & -(bVar64 == 0x5f);
          pbVar75[0x10] = bVar49 ^ (bVar49 ^ 0x2f) & -(bVar49 == 0x5f);
          pbVar75[0x11] = bVar50 ^ (bVar50 ^ 0x2f) & -(bVar50 == 0x5f);
          pbVar75[0x12] = bVar51 ^ (bVar51 ^ 0x2f) & -(bVar51 == 0x5f);
          pbVar75[0x13] = bVar52 ^ (bVar52 ^ 0x2f) & -(bVar52 == 0x5f);
          pbVar75[0x14] = bVar53 ^ (bVar53 ^ 0x2f) & -(bVar53 == 0x5f);
          pbVar75[0x15] = bVar54 ^ (bVar54 ^ 0x2f) & -(bVar54 == 0x5f);
          pbVar75[0x16] = bVar55 ^ (bVar55 ^ 0x2f) & -(bVar55 == 0x5f);
          pbVar75[0x17] = bVar56 ^ (bVar56 ^ 0x2f) & -(bVar56 == 0x5f);
          pppppppuVar69 = pppppppuVar69 + -8;
          pbVar72 = pbVar72 + 0x40;
          pbVar75 = pbVar75 + 0x40;
        } while (pppppppuVar69 != (undefined8 *******)0x0);
        if (pppppppuVar78 != pppppppuVar70) {
          pppppppuVar69 = pppppppuVar70;
          if (((ulong)pppppppuVar78 & 0x38) == 0) goto LAB_1001ec588;
          goto LAB_1001ec54c;
        }
      }
      else {
        pppppppuVar69 = (undefined8 *******)0x0;
LAB_1001ec54c:
        pppppppuVar70 = (undefined8 *******)((ulong)pppppppuVar78 & 0x7ffffffffffffff8);
        lVar79 = (long)pppppppuVar69 - (long)pppppppuVar70;
        puVar74 = (ulong *)(local_130 + (long)pppppppuVar69);
        puVar76 = (ulong *)(lVar67 + (long)pppppppuVar69);
        do {
          uVar81 = *puVar76;
          *puVar74 = uVar81 ^ (uVar81 ^ 0x2f2f2f2f2f2f2f2f) &
                              CONCAT17(-((char)(uVar81 >> 0x38) == '_'),
                                       CONCAT16(-((char)(uVar81 >> 0x30) == '_'),
                                                CONCAT15(-((char)(uVar81 >> 0x28) == '_'),
                                                         CONCAT14(-((char)(uVar81 >> 0x20) == '_'),
                                                                  CONCAT13(-((char)(uVar81 >> 0x18) == '_'),
                                                                           CONCAT12(-((char)(uVar81 >> 0x10) == '_'),
                                                                                    CONCAT11(-((char)(uVar81 >> 8) == '_'),-((char)uVar81 == '_'))))))));
          lVar79 = lVar79 + 8;
          puVar74 = puVar74 + 1;
          puVar76 = puVar76 + 1;
        } while (lVar79 != 0);
        if (pppppppuVar78 != pppppppuVar70) goto LAB_1001ec588;
      }
    }
    else {
      pppppppuVar70 = (undefined8 *******)0x0;
LAB_1001ec588:
      lVar79 = (long)pppppppuVar78 - (long)pppppppuVar70;
      pbVar72 = (byte *)(lVar67 + (long)pppppppuVar70);
      pbVar75 = (byte *)(local_130 + (long)pppppppuVar70);
      do {
        bVar1 = 0x2f;
        if (*pbVar72 != 0x5f) {
          bVar1 = *pbVar72;
        }
        *pbVar75 = bVar1;
        lVar79 = lVar79 + -1;
        pbVar72 = pbVar72 + 1;
        pbVar75 = pbVar75 + 1;
      } while (lVar79 != 0);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar67,pppppppuVar78,1);
    uVar81 = (ulong)local_128 & 3;
    pppppppuVar78 = local_128;
    pppppppuVar69 = local_128;
    lVar67 = local_130;
    pppppppuVar70 = local_138;
    if (uVar81 != 0) {
      alloc::slice::<impl[T]>::repeat(&local_f0,&DAT_10130572a,1,4 - uVar81);
      lStack_a8 = lStack_e8;
      local_b0 = local_f0;
      local_a0 = local_e0;
      local_f0 = &local_138;
      lStack_e8 = 0x100233af8;
      local_e0 = &local_b0;
      local_d8 = 0x100233af8;
      alloc::fmt::format::format_inner(&local_120,_anon_dc5b8adae996cb58fd4173e320c2d1cf_144,&local_f0);
      pppppppuVar78 = local_110;
      pppppppuVar69 = local_110;
      lVar67 = lStack_118;
      pppppppuVar70 = local_120;
      if (local_b0 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_a8,local_b0,1);
        pppppppuVar78 = local_110;
        pppppppuVar69 = local_110;
        lVar67 = lStack_118;
        pppppppuVar70 = local_120;
      }
    }
  }
  do {
    if (pppppppuVar78 == (undefined8 *******)0x0) break;
    pbVar72 = (byte *)((long)pppppppuVar78 + lVar67 + -1);
    uVar68 = (uint)(char)*pbVar72;
    if ((int)uVar68 < 0) {
      pbVar72 = (byte *)((long)pppppppuVar78 + lVar67 + -2);
      bVar1 = *pbVar72;
      if ((char)bVar1 < -0x40) {
        pbVar72 = (byte *)((long)pppppppuVar78 + lVar67 + -3);
        bVar2 = *pbVar72;
        if ((char)bVar2 < -0x40) {
          pbVar72 = (byte *)((long)pppppppuVar78 + lVar67 + -4);
          uVar73 = bVar2 & 0x3f | (*pbVar72 & 7) << 6;
        }
        else {
          uVar73 = (int)(char)bVar2 & 0xf;
        }
        uVar73 = bVar1 & 0x3f | uVar73 << 6;
      }
      else {
        uVar73 = (int)(char)bVar1 & 0x1f;
      }
      uVar68 = uVar68 & 0x3f | uVar73 << 6;
    }
    pppppppuVar78 = (undefined8 *******)(pbVar72 + -lVar67);
  } while (uVar68 == 0x3d);
  base64::engine::Engine::decode::inner(&local_b0,&DAT_10130572b,lVar67);
  pppppppuVar78 = local_a0;
  lVar79 = lStack_a8;
  pppppppuVar80 = local_b0;
  if ((local_b0 == (undefined8 *******)&DAT_8000000000000000) &&
     (base64::engine::Engine::decode::inner(&local_f0,&DAT_10130586e,lVar67,pppppppuVar69), pppppppuVar78 = local_e0, lVar79 = lStack_e8,
     pppppppuVar80 = local_f0, local_f0 == (undefined8 *******)&DAT_8000000000000000)) {
    pauVar66 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
    if (pauVar66[1][0] == '\x01') {
      auVar82 = *pauVar66;
    }
    else {
      auVar82 = std::sys::random::hashmap_random_keys();
      *pauVar66 = auVar82;
      pauVar66[1][0] = 1;
    }
    *(long *)*pauVar66 = auVar82._0_8_ + 1;
    param_1[1] = 0;
    *param_1 = &DAT_101303e80;
    param_1[3] = 0;
    param_1[2] = 0;
    *(undefined1 (*) [16])(param_1 + 4) = auVar82;
    if (pppppppuVar70 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar67,pppppppuVar70,1);
    }
    if ((uVar81 != 0) && (local_138 != (undefined8 *******)0x0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,local_138,1);
    }
  }
  else {
    local_c8 = (undefined8 *******)0x0;
    uStack_c0 = 0;
    local_f0 = (undefined8 *******)0x0;
    lStack_e8 = 1;
    local_e0 = (undefined8 *******)0x0;
    local_b8 = 0x80;
    local_d8 = lVar79;
    local_d0 = pppppppuVar78;
    <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_map(&local_120,&local_f0);
    if (local_120 == (undefined8 *******)0x0) {
LAB_1001ec8d8:
      if (local_f0 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_e8,local_f0,1);
      }
      pauVar66 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
      if (pauVar66[1][0] == '\x01') {
        auVar82 = *pauVar66;
      }
      else {
        auVar82 = std::sys::random::hashmap_random_keys();
        *pauVar66 = auVar82;
        pauVar66[1][0] = 1;
      }
      lVar71 = lStack_118;
      *(long *)*pauVar66 = auVar82._0_8_ + 1;
      param_1[1] = 0;
      *param_1 = &DAT_101303e80;
      param_1[3] = 0;
      param_1[2] = 0;
      *(undefined1 (*) [16])(param_1 + 4) = auVar82;
      ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(lStack_118);
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar71,0x28,8);
    }
    else {
      lStack_a8 = lStack_118;
      local_b0 = local_120;
      uStack_98 = uStack_108;
      local_a0 = local_110;
      if (local_c8 < local_d0) {
        do {
          if (0x20 < *(byte *)(local_d8 + (long)local_c8) || (1L << ((ulong)*(byte *)(local_d8 + (long)local_c8) & 0x3f) & 0x100002600U) == 0) {
            local_78[0] = 0x16;
            lStack_118 = serde_json::de::Deserializer<R>::peek_error(&local_f0,local_78);
            local_120 = (undefined8 *******)0x0;
            <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(&local_b0);
            goto LAB_1001ec8d8;
          }
          local_c8 = (undefined8 *******)((long)local_c8 + 1);
        } while (local_d0 != local_c8);
      }
      if (local_f0 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_e8,local_f0,1);
      }
      param_1[1] = lStack_118;
      *param_1 = local_120;
      param_1[3] = uStack_108;
      param_1[2] = local_110;
      param_1[5] = uStack_f8;
      param_1[4] = uStack_100;
    }
    if (pppppppuVar80 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar79,pppppppuVar80,1);
    }
    if (pppppppuVar70 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar67,pppppppuVar70,1);
    }
    local_190 = lVar65;
    if ((uVar81 != 0) && (local_138 != (undefined8 *******)0x0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,local_138,1);
    }
  }
joined_r0x0001001ec884:
  if (local_198 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_190,local_198 << 4,8);
  }
  return;
}
