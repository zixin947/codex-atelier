/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* is_protected_process_command @ 1007c34bc requested 1007c34bc */

/* codexmate_lib::platform::process::is_protected_process_command */

uint codexmate_lib::platform::process::is_protected_process_command(long *param_1,ulong param_2)

{
  char cVar1;
  undefined8 uVar2;
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
  int *piVar53;
  int iVar54;
  void *pvVar55;
  long *plVar56;
  void *pvVar57;
  ulong uVar58;
  char *pcVar59;
  byte *pbVar60;
  long lVar61;
  long lVar62;
  byte *pbVar63;
  ulong *puVar64;
  char *pcVar65;
  undefined8 *puVar66;
  ulong *puVar67;
  undefined8 uVar68;
  uint uVar69;
  ulong uVar70;
  byte bVar71;
  byte bVar72;
  byte bVar73;
  byte bVar74;
  byte bVar75;
  byte bVar76;
  byte bVar77;
  ulong uVar78;
  byte bVar79;
  byte bVar80;
  byte bVar81;
  byte bVar82;
  byte bVar83;
  byte bVar84;
  byte bVar85;
  undefined1 auVar86 [16];
  undefined1 auVar87 [16];
  uint local_d0 [6];
  undefined8 local_b8;
  ulong uStack_b0;
  long *local_a8;
  ulong uStack_a0;
  long *local_98;
  ulong uStack_90;
  undefined8 local_88;
  undefined2 local_80;
  undefined2 local_78;
  
  auVar87._8_8_ = param_2;
  auVar87._0_8_ = param_1;
  if ((long)param_2 < 0) {
    uVar68 = 0;
LAB_1007c34e4:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar68,param_2);
  }
  if (param_2 == 0) {
    plVar56 = (long *)0x1;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar68 = 1;
    pvVar55 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_2,1);
    if (pvVar55 == (void *)0x0) goto LAB_1007c34e4;
    if (param_2 < 8) {
      uVar58 = 0;
LAB_1007c35dc:
      lVar61 = param_2 - uVar58;
      pcVar59 = (char *)((long)param_1 + uVar58);
      pcVar65 = (char *)((long)pvVar55 + uVar58);
      do {
        cVar1 = '/';
        if (*pcVar59 != '\\') {
          cVar1 = *pcVar59;
        }
        *pcVar65 = cVar1;
        lVar61 = lVar61 + -1;
        pcVar59 = pcVar59 + 1;
        pcVar65 = pcVar65 + 1;
      } while (lVar61 != 0);
    }
    else if (param_2 < 0x40) {
      uVar78 = 0;
LAB_1007c35a0:
      uVar58 = param_2 & 0x7ffffffffffffff8;
      lVar61 = uVar78 - uVar58;
      puVar64 = (ulong *)((long)pvVar55 + uVar78);
      puVar67 = (ulong *)((long)param_1 + uVar78);
      do {
        uVar78 = *puVar67;
        *puVar64 = uVar78 ^ (uVar78 ^ 0x2f2f2f2f2f2f2f2f) &
                            CONCAT17(-((char)(uVar78 >> 0x38) == '\\'),
                                     CONCAT16(-((char)(uVar78 >> 0x30) == '\\'),
                                              CONCAT15(-((char)(uVar78 >> 0x28) == '\\'),
                                                       CONCAT14(-((char)(uVar78 >> 0x20) == '\\'),
                                                                CONCAT13(-((char)(uVar78 >> 0x18) == '\\'),
                                                                         CONCAT12(-((char)(uVar78 >> 0x10) == '\\'),
                                                                                  CONCAT11(-((char)(uVar78 >> 8) == '\\'),-((char)uVar78 == '\\'))))))));
        lVar61 = lVar61 + 8;
        puVar64 = puVar64 + 1;
        puVar67 = puVar67 + 1;
      } while (lVar61 != 0);
      if (param_2 != uVar58) goto LAB_1007c35dc;
    }
    else {
      uVar58 = param_2 & 0x7fffffffffffffc0;
      pbVar60 = (byte *)(param_1 + 4);
      pbVar63 = (byte *)((long)pvVar55 + 0x20);
      uVar78 = uVar58;
      do {
        bVar71 = pbVar60[-0x20];
        bVar72 = pbVar60[-0x1f];
        bVar73 = pbVar60[-0x1e];
        bVar74 = pbVar60[-0x1d];
        bVar75 = pbVar60[-0x1c];
        bVar76 = pbVar60[-0x1b];
        bVar77 = pbVar60[-0x1a];
        bVar79 = pbVar60[-0x19];
        bVar80 = pbVar60[-0x18];
        bVar81 = pbVar60[-0x17];
        bVar82 = pbVar60[-0x16];
        bVar83 = pbVar60[-0x15];
        bVar84 = pbVar60[-0x14];
        bVar85 = pbVar60[-0x13];
        bVar3 = pbVar60[-0x12];
        bVar4 = pbVar60[-0x11];
        bVar5 = pbVar60[-0x10];
        bVar6 = pbVar60[-0xf];
        bVar7 = pbVar60[-0xe];
        bVar8 = pbVar60[-0xd];
        bVar9 = pbVar60[-0xc];
        bVar10 = pbVar60[-0xb];
        bVar11 = pbVar60[-10];
        bVar12 = pbVar60[-9];
        bVar13 = pbVar60[-8];
        bVar14 = pbVar60[-7];
        bVar15 = pbVar60[-6];
        bVar16 = pbVar60[-5];
        bVar17 = pbVar60[-4];
        bVar18 = pbVar60[-3];
        bVar19 = pbVar60[-2];
        bVar20 = pbVar60[-1];
        bVar21 = *pbVar60;
        bVar22 = pbVar60[1];
        bVar23 = pbVar60[2];
        bVar24 = pbVar60[3];
        bVar25 = pbVar60[4];
        bVar26 = pbVar60[5];
        bVar27 = pbVar60[6];
        bVar28 = pbVar60[7];
        bVar29 = pbVar60[8];
        bVar30 = pbVar60[9];
        bVar31 = pbVar60[10];
        bVar32 = pbVar60[0xb];
        bVar33 = pbVar60[0xc];
        bVar34 = pbVar60[0xd];
        bVar35 = pbVar60[0xe];
        bVar36 = pbVar60[0xf];
        bVar37 = pbVar60[0x10];
        bVar38 = pbVar60[0x11];
        bVar39 = pbVar60[0x12];
        bVar40 = pbVar60[0x13];
        bVar41 = pbVar60[0x14];
        bVar42 = pbVar60[0x15];
        bVar43 = pbVar60[0x16];
        bVar44 = pbVar60[0x17];
        bVar45 = pbVar60[0x18];
        bVar46 = pbVar60[0x19];
        bVar47 = pbVar60[0x1a];
        bVar48 = pbVar60[0x1b];
        bVar49 = pbVar60[0x1c];
        bVar50 = pbVar60[0x1d];
        bVar51 = pbVar60[0x1e];
        bVar52 = pbVar60[0x1f];
        pbVar63[-0x18] = bVar80 ^ (bVar80 ^ 0x2f) & -(bVar80 == 0x5c);
        pbVar63[-0x17] = bVar81 ^ (bVar81 ^ 0x2f) & -(bVar81 == 0x5c);
        pbVar63[-0x16] = bVar82 ^ (bVar82 ^ 0x2f) & -(bVar82 == 0x5c);
        pbVar63[-0x15] = bVar83 ^ (bVar83 ^ 0x2f) & -(bVar83 == 0x5c);
        pbVar63[-0x14] = bVar84 ^ (bVar84 ^ 0x2f) & -(bVar84 == 0x5c);
        pbVar63[-0x13] = bVar85 ^ (bVar85 ^ 0x2f) & -(bVar85 == 0x5c);
        pbVar63[-0x12] = bVar3 ^ (bVar3 ^ 0x2f) & -(bVar3 == 0x5c);
        pbVar63[-0x11] = bVar4 ^ (bVar4 ^ 0x2f) & -(bVar4 == 0x5c);
        pbVar63[-0x20] = bVar71 ^ (bVar71 ^ 0x2f) & -(bVar71 == 0x5c);
        pbVar63[-0x1f] = bVar72 ^ (bVar72 ^ 0x2f) & -(bVar72 == 0x5c);
        pbVar63[-0x1e] = bVar73 ^ (bVar73 ^ 0x2f) & -(bVar73 == 0x5c);
        pbVar63[-0x1d] = bVar74 ^ (bVar74 ^ 0x2f) & -(bVar74 == 0x5c);
        pbVar63[-0x1c] = bVar75 ^ (bVar75 ^ 0x2f) & -(bVar75 == 0x5c);
        pbVar63[-0x1b] = bVar76 ^ (bVar76 ^ 0x2f) & -(bVar76 == 0x5c);
        pbVar63[-0x1a] = bVar77 ^ (bVar77 ^ 0x2f) & -(bVar77 == 0x5c);
        pbVar63[-0x19] = bVar79 ^ (bVar79 ^ 0x2f) & -(bVar79 == 0x5c);
        pbVar63[-8] = bVar13 ^ (bVar13 ^ 0x2f) & -(bVar13 == 0x5c);
        pbVar63[-7] = bVar14 ^ (bVar14 ^ 0x2f) & -(bVar14 == 0x5c);
        pbVar63[-6] = bVar15 ^ (bVar15 ^ 0x2f) & -(bVar15 == 0x5c);
        pbVar63[-5] = bVar16 ^ (bVar16 ^ 0x2f) & -(bVar16 == 0x5c);
        pbVar63[-4] = bVar17 ^ (bVar17 ^ 0x2f) & -(bVar17 == 0x5c);
        pbVar63[-3] = bVar18 ^ (bVar18 ^ 0x2f) & -(bVar18 == 0x5c);
        pbVar63[-2] = bVar19 ^ (bVar19 ^ 0x2f) & -(bVar19 == 0x5c);
        pbVar63[-1] = bVar20 ^ (bVar20 ^ 0x2f) & -(bVar20 == 0x5c);
        pbVar63[-0x10] = bVar5 ^ (bVar5 ^ 0x2f) & -(bVar5 == 0x5c);
        pbVar63[-0xf] = bVar6 ^ (bVar6 ^ 0x2f) & -(bVar6 == 0x5c);
        pbVar63[-0xe] = bVar7 ^ (bVar7 ^ 0x2f) & -(bVar7 == 0x5c);
        pbVar63[-0xd] = bVar8 ^ (bVar8 ^ 0x2f) & -(bVar8 == 0x5c);
        pbVar63[-0xc] = bVar9 ^ (bVar9 ^ 0x2f) & -(bVar9 == 0x5c);
        pbVar63[-0xb] = bVar10 ^ (bVar10 ^ 0x2f) & -(bVar10 == 0x5c);
        pbVar63[-10] = bVar11 ^ (bVar11 ^ 0x2f) & -(bVar11 == 0x5c);
        pbVar63[-9] = bVar12 ^ (bVar12 ^ 0x2f) & -(bVar12 == 0x5c);
        pbVar63[8] = bVar29 ^ (bVar29 ^ 0x2f) & -(bVar29 == 0x5c);
        pbVar63[9] = bVar30 ^ (bVar30 ^ 0x2f) & -(bVar30 == 0x5c);
        pbVar63[10] = bVar31 ^ (bVar31 ^ 0x2f) & -(bVar31 == 0x5c);
        pbVar63[0xb] = bVar32 ^ (bVar32 ^ 0x2f) & -(bVar32 == 0x5c);
        pbVar63[0xc] = bVar33 ^ (bVar33 ^ 0x2f) & -(bVar33 == 0x5c);
        pbVar63[0xd] = bVar34 ^ (bVar34 ^ 0x2f) & -(bVar34 == 0x5c);
        pbVar63[0xe] = bVar35 ^ (bVar35 ^ 0x2f) & -(bVar35 == 0x5c);
        pbVar63[0xf] = bVar36 ^ (bVar36 ^ 0x2f) & -(bVar36 == 0x5c);
        *pbVar63 = bVar21 ^ (bVar21 ^ 0x2f) & -(bVar21 == 0x5c);
        pbVar63[1] = bVar22 ^ (bVar22 ^ 0x2f) & -(bVar22 == 0x5c);
        pbVar63[2] = bVar23 ^ (bVar23 ^ 0x2f) & -(bVar23 == 0x5c);
        pbVar63[3] = bVar24 ^ (bVar24 ^ 0x2f) & -(bVar24 == 0x5c);
        pbVar63[4] = bVar25 ^ (bVar25 ^ 0x2f) & -(bVar25 == 0x5c);
        pbVar63[5] = bVar26 ^ (bVar26 ^ 0x2f) & -(bVar26 == 0x5c);
        pbVar63[6] = bVar27 ^ (bVar27 ^ 0x2f) & -(bVar27 == 0x5c);
        pbVar63[7] = bVar28 ^ (bVar28 ^ 0x2f) & -(bVar28 == 0x5c);
        pbVar63[0x18] = bVar45 ^ (bVar45 ^ 0x2f) & -(bVar45 == 0x5c);
        pbVar63[0x19] = bVar46 ^ (bVar46 ^ 0x2f) & -(bVar46 == 0x5c);
        pbVar63[0x1a] = bVar47 ^ (bVar47 ^ 0x2f) & -(bVar47 == 0x5c);
        pbVar63[0x1b] = bVar48 ^ (bVar48 ^ 0x2f) & -(bVar48 == 0x5c);
        pbVar63[0x1c] = bVar49 ^ (bVar49 ^ 0x2f) & -(bVar49 == 0x5c);
        pbVar63[0x1d] = bVar50 ^ (bVar50 ^ 0x2f) & -(bVar50 == 0x5c);
        pbVar63[0x1e] = bVar51 ^ (bVar51 ^ 0x2f) & -(bVar51 == 0x5c);
        pbVar63[0x1f] = bVar52 ^ (bVar52 ^ 0x2f) & -(bVar52 == 0x5c);
        pbVar63[0x10] = bVar37 ^ (bVar37 ^ 0x2f) & -(bVar37 == 0x5c);
        pbVar63[0x11] = bVar38 ^ (bVar38 ^ 0x2f) & -(bVar38 == 0x5c);
        pbVar63[0x12] = bVar39 ^ (bVar39 ^ 0x2f) & -(bVar39 == 0x5c);
        pbVar63[0x13] = bVar40 ^ (bVar40 ^ 0x2f) & -(bVar40 == 0x5c);
        pbVar63[0x14] = bVar41 ^ (bVar41 ^ 0x2f) & -(bVar41 == 0x5c);
        pbVar63[0x15] = bVar42 ^ (bVar42 ^ 0x2f) & -(bVar42 == 0x5c);
        pbVar63[0x16] = bVar43 ^ (bVar43 ^ 0x2f) & -(bVar43 == 0x5c);
        pbVar63[0x17] = bVar44 ^ (bVar44 ^ 0x2f) & -(bVar44 == 0x5c);
        uVar78 = uVar78 - 0x40;
        pbVar60 = pbVar60 + 0x40;
        pbVar63 = pbVar63 + 0x40;
      } while (uVar78 != 0);
      if (param_2 != uVar58) {
        uVar78 = uVar58;
        if ((param_2 & 0x38) == 0) goto LAB_1007c35dc;
        goto LAB_1007c35a0;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    plVar56 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_2,1);
    if (plVar56 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,param_2);
    }
    _memcpy(plVar56,pvVar55,param_2);
    if (param_2 < 8) {
      uVar58 = 0;
LAB_1007c370c:
      lVar61 = param_2 - uVar58;
      pbVar60 = (byte *)((long)plVar56 + uVar58);
      do {
        bVar71 = 0x20;
        if (0x19 < *pbVar60 - 0x41) {
          bVar71 = 0;
        }
        *pbVar60 = bVar71 | *pbVar60;
        lVar61 = lVar61 + -1;
        pbVar60 = pbVar60 + 1;
      } while (lVar61 != 0);
    }
    else if (param_2 < 0x40) {
      uVar78 = 0;
LAB_1007c36c8:
      uVar58 = param_2 & 0x7ffffffffffffff8;
      lVar61 = uVar78 - uVar58;
      puVar66 = (undefined8 *)((long)plVar56 + uVar78);
      do {
        uVar68 = *puVar66;
        bVar71 = (byte)((ulong)uVar68 >> 8);
        bVar72 = (byte)((ulong)uVar68 >> 0x10);
        bVar73 = (byte)((ulong)uVar68 >> 0x18);
        bVar74 = (byte)((ulong)uVar68 >> 0x20);
        bVar75 = (byte)((ulong)uVar68 >> 0x28);
        bVar76 = (byte)((ulong)uVar68 >> 0x30);
        bVar77 = (byte)((ulong)uVar68 >> 0x38);
        uVar78 = CONCAT17(-((byte)(bVar77 + 0xbf) < 0x1a),
                          CONCAT16(-((byte)(bVar76 + 0xbf) < 0x1a),
                                   CONCAT15(-((byte)(bVar75 + 0xbf) < 0x1a),
                                            CONCAT14(-((byte)(bVar74 + 0xbf) < 0x1a),
                                                     CONCAT13(-((byte)(bVar73 + 0xbf) < 0x1a),
                                                              CONCAT12(-((byte)(bVar72 + 0xbf) < 0x1a),
                                                                       CONCAT11(-((byte)(bVar71 + 0xbf) < 0x1a),-((byte)((byte)uVar68 + 0xbf) < 0x1a)))))))) &
                 0x2020202020202020;
        *puVar66 = CONCAT17((byte)(uVar78 >> 0x38) | bVar77,
                            CONCAT16((byte)(uVar78 >> 0x30) | bVar76,
                                     CONCAT15((byte)(uVar78 >> 0x28) | bVar75,
                                              CONCAT14((byte)(uVar78 >> 0x20) | bVar74,
                                                       CONCAT13((byte)(uVar78 >> 0x18) | bVar73,
                                                                CONCAT12((byte)(uVar78 >> 0x10) | bVar72,
                                                                         CONCAT11((byte)(uVar78 >> 8) | bVar71,(byte)uVar78 | (byte)uVar68)))))));
        lVar61 = lVar61 + 8;
        puVar66 = puVar66 + 1;
      } while (lVar61 != 0);
      if (param_2 != uVar58) goto LAB_1007c370c;
    }
    else {
      uVar58 = param_2 & 0x7fffffffffffffc0;
      pbVar60 = (byte *)(plVar56 + 4);
      uVar78 = uVar58;
      do {
        lVar62 = *(long *)(pbVar60 + 0x18);
        bVar79 = (byte)((ulong)lVar62 >> 8);
        bVar80 = (byte)((ulong)lVar62 >> 0x10);
        bVar81 = (byte)((ulong)lVar62 >> 0x18);
        bVar82 = (byte)((ulong)lVar62 >> 0x20);
        bVar83 = (byte)((ulong)lVar62 >> 0x28);
        bVar84 = (byte)((ulong)lVar62 >> 0x30);
        bVar85 = (byte)((ulong)lVar62 >> 0x38);
        lVar61 = *(long *)(pbVar60 + 0x10);
        bVar71 = (byte)((ulong)lVar61 >> 8);
        bVar72 = (byte)((ulong)lVar61 >> 0x10);
        bVar73 = (byte)((ulong)lVar61 >> 0x18);
        bVar74 = (byte)((ulong)lVar61 >> 0x20);
        bVar75 = (byte)((ulong)lVar61 >> 0x28);
        bVar76 = (byte)((ulong)lVar61 >> 0x30);
        bVar77 = (byte)((ulong)lVar61 >> 0x38);
        pbVar60[-0x18] = -((byte)(pbVar60[-0x18] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x18];
        pbVar60[-0x17] = -((byte)(pbVar60[-0x17] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x17];
        pbVar60[-0x16] = -((byte)(pbVar60[-0x16] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x16];
        pbVar60[-0x15] = -((byte)(pbVar60[-0x15] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x15];
        pbVar60[-0x14] = -((byte)(pbVar60[-0x14] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x14];
        pbVar60[-0x13] = -((byte)(pbVar60[-0x13] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x13];
        pbVar60[-0x12] = -((byte)(pbVar60[-0x12] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x12];
        pbVar60[-0x11] = -((byte)(pbVar60[-0x11] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x11];
        pbVar60[-0x20] = -((byte)(pbVar60[-0x20] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x20];
        pbVar60[-0x1f] = -((byte)(pbVar60[-0x1f] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1f];
        pbVar60[-0x1e] = -((byte)(pbVar60[-0x1e] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1e];
        pbVar60[-0x1d] = -((byte)(pbVar60[-0x1d] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1d];
        pbVar60[-0x1c] = -((byte)(pbVar60[-0x1c] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1c];
        pbVar60[-0x1b] = -((byte)(pbVar60[-0x1b] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1b];
        pbVar60[-0x1a] = -((byte)(pbVar60[-0x1a] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1a];
        pbVar60[-0x19] = -((byte)(pbVar60[-0x19] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x19];
        pbVar60[-8] = -((byte)(pbVar60[-8] + 0xbf) < 0x1a) & 0x20U | pbVar60[-8];
        pbVar60[-7] = -((byte)(pbVar60[-7] + 0xbf) < 0x1a) & 0x20U | pbVar60[-7];
        pbVar60[-6] = -((byte)(pbVar60[-6] + 0xbf) < 0x1a) & 0x20U | pbVar60[-6];
        pbVar60[-5] = -((byte)(pbVar60[-5] + 0xbf) < 0x1a) & 0x20U | pbVar60[-5];
        pbVar60[-4] = -((byte)(pbVar60[-4] + 0xbf) < 0x1a) & 0x20U | pbVar60[-4];
        pbVar60[-3] = -((byte)(pbVar60[-3] + 0xbf) < 0x1a) & 0x20U | pbVar60[-3];
        pbVar60[-2] = -((byte)(pbVar60[-2] + 0xbf) < 0x1a) & 0x20U | pbVar60[-2];
        pbVar60[-1] = -((byte)(pbVar60[-1] + 0xbf) < 0x1a) & 0x20U | pbVar60[-1];
        pbVar60[-0x10] = -((byte)(pbVar60[-0x10] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x10];
        pbVar60[-0xf] = -((byte)(pbVar60[-0xf] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xf];
        pbVar60[-0xe] = -((byte)(pbVar60[-0xe] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xe];
        pbVar60[-0xd] = -((byte)(pbVar60[-0xd] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xd];
        pbVar60[-0xc] = -((byte)(pbVar60[-0xc] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xc];
        pbVar60[-0xb] = -((byte)(pbVar60[-0xb] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xb];
        pbVar60[-10] = -((byte)(pbVar60[-10] + 0xbf) < 0x1a) & 0x20U | pbVar60[-10];
        pbVar60[-9] = -((byte)(pbVar60[-9] + 0xbf) < 0x1a) & 0x20U | pbVar60[-9];
        pbVar60[8] = -((byte)(pbVar60[8] + 0xbf) < 0x1a) & 0x20U | pbVar60[8];
        pbVar60[9] = -((byte)(pbVar60[9] + 0xbf) < 0x1a) & 0x20U | pbVar60[9];
        pbVar60[10] = -((byte)(pbVar60[10] + 0xbf) < 0x1a) & 0x20U | pbVar60[10];
        pbVar60[0xb] = -((byte)(pbVar60[0xb] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xb];
        pbVar60[0xc] = -((byte)(pbVar60[0xc] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xc];
        pbVar60[0xd] = -((byte)(pbVar60[0xd] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xd];
        pbVar60[0xe] = -((byte)(pbVar60[0xe] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xe];
        pbVar60[0xf] = -((byte)(pbVar60[0xf] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xf];
        *pbVar60 = -((byte)(*pbVar60 + 0xbf) < 0x1a) & 0x20U | *pbVar60;
        pbVar60[1] = -((byte)(pbVar60[1] + 0xbf) < 0x1a) & 0x20U | pbVar60[1];
        pbVar60[2] = -((byte)(pbVar60[2] + 0xbf) < 0x1a) & 0x20U | pbVar60[2];
        pbVar60[3] = -((byte)(pbVar60[3] + 0xbf) < 0x1a) & 0x20U | pbVar60[3];
        pbVar60[4] = -((byte)(pbVar60[4] + 0xbf) < 0x1a) & 0x20U | pbVar60[4];
        pbVar60[5] = -((byte)(pbVar60[5] + 0xbf) < 0x1a) & 0x20U | pbVar60[5];
        pbVar60[6] = -((byte)(pbVar60[6] + 0xbf) < 0x1a) & 0x20U | pbVar60[6];
        pbVar60[7] = -((byte)(pbVar60[7] + 0xbf) < 0x1a) & 0x20U | pbVar60[7];
        pbVar60[0x18] = -((byte)((byte)lVar62 + 0xbf) < 0x1a) & 0x20U | (byte)lVar62;
        pbVar60[0x19] = -((byte)(bVar79 + 0xbf) < 0x1a) & 0x20U | bVar79;
        pbVar60[0x1a] = -((byte)(bVar80 + 0xbf) < 0x1a) & 0x20U | bVar80;
        pbVar60[0x1b] = -((byte)(bVar81 + 0xbf) < 0x1a) & 0x20U | bVar81;
        pbVar60[0x1c] = -((byte)(bVar82 + 0xbf) < 0x1a) & 0x20U | bVar82;
        pbVar60[0x1d] = -((byte)(bVar83 + 0xbf) < 0x1a) & 0x20U | bVar83;
        pbVar60[0x1e] = -((byte)(bVar84 + 0xbf) < 0x1a) & 0x20U | bVar84;
        pbVar60[0x1f] = -((byte)(bVar85 + 0xbf) < 0x1a) & 0x20U | bVar85;
        pbVar60[0x10] = -((byte)((byte)lVar61 + 0xbf) < 0x1a) & 0x20U | (byte)lVar61;
        pbVar60[0x11] = -((byte)(bVar71 + 0xbf) < 0x1a) & 0x20U | bVar71;
        pbVar60[0x12] = -((byte)(bVar72 + 0xbf) < 0x1a) & 0x20U | bVar72;
        pbVar60[0x13] = -((byte)(bVar73 + 0xbf) < 0x1a) & 0x20U | bVar73;
        pbVar60[0x14] = -((byte)(bVar74 + 0xbf) < 0x1a) & 0x20U | bVar74;
        pbVar60[0x15] = -((byte)(bVar75 + 0xbf) < 0x1a) & 0x20U | bVar75;
        pbVar60[0x16] = -((byte)(bVar76 + 0xbf) < 0x1a) & 0x20U | bVar76;
        pbVar60[0x17] = -((byte)(bVar77 + 0xbf) < 0x1a) & 0x20U | bVar77;
        uVar78 = uVar78 - 0x40;
        pbVar60 = pbVar60 + 0x40;
      } while (uVar78 != 0);
      if (param_2 != uVar58) {
        uVar78 = uVar58;
        if ((param_2 & 0x38) == 0) goto LAB_1007c370c;
        goto LAB_1007c36c8;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar55,param_2,1);
  }
  uStack_90 = (long)param_1 + param_2;
  local_b8 = 0;
  local_88 = 0;
  local_80 = 1;
  uStack_b0 = param_2;
  local_a8 = param_1;
  uStack_a0 = param_2;
  local_98 = param_1;
  auVar86 = ::core::iter::traits::iterator::Iterator::try_fold(&local_b8);
  uVar68 = 0;
  if (auVar86._0_8_ != 0) {
    auVar87 = auVar86;
  }
  uVar78 = auVar87._8_8_;
  lVar61 = auVar87._0_8_;
  if ((long)uVar78 < 0) {
LAB_1007c377c:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar68,uVar78);
  }
  if (uVar78 == 0) {
    pvVar55 = (void *)0x1;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar68 = 1;
    pvVar57 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(uVar78,1);
    if (pvVar57 == (void *)0x0) goto LAB_1007c377c;
    uVar58 = 0;
    if ((uVar78 < 8) || ((ulong)((long)pvVar57 - lVar61) < 0x40)) {
LAB_1007c387c:
      lVar62 = uVar78 - uVar58;
      pcVar59 = (char *)(lVar61 + uVar58);
      pcVar65 = (char *)((long)pvVar57 + uVar58);
      do {
        cVar1 = '/';
        if (*pcVar59 != '\\') {
          cVar1 = *pcVar59;
        }
        *pcVar65 = cVar1;
        lVar62 = lVar62 + -1;
        pcVar59 = pcVar59 + 1;
        pcVar65 = pcVar65 + 1;
      } while (lVar62 != 0);
    }
    else if (uVar78 < 0x40) {
      uVar70 = 0;
LAB_1007c3840:
      uVar58 = uVar78 & 0x7ffffffffffffff8;
      lVar62 = uVar70 - uVar58;
      puVar64 = (ulong *)((long)pvVar57 + uVar70);
      puVar67 = (ulong *)(lVar61 + uVar70);
      do {
        uVar70 = *puVar67;
        *puVar64 = uVar70 ^ (uVar70 ^ 0x2f2f2f2f2f2f2f2f) &
                            CONCAT17(-((char)(uVar70 >> 0x38) == '\\'),
                                     CONCAT16(-((char)(uVar70 >> 0x30) == '\\'),
                                              CONCAT15(-((char)(uVar70 >> 0x28) == '\\'),
                                                       CONCAT14(-((char)(uVar70 >> 0x20) == '\\'),
                                                                CONCAT13(-((char)(uVar70 >> 0x18) == '\\'),
                                                                         CONCAT12(-((char)(uVar70 >> 0x10) == '\\'),
                                                                                  CONCAT11(-((char)(uVar70 >> 8) == '\\'),-((char)uVar70 == '\\'))))))));
        lVar62 = lVar62 + 8;
        puVar64 = puVar64 + 1;
        puVar67 = puVar67 + 1;
      } while (lVar62 != 0);
      if (uVar78 != uVar58) goto LAB_1007c387c;
    }
    else {
      uVar58 = uVar78 & 0x7fffffffffffffc0;
      pbVar60 = (byte *)(lVar61 + 0x20);
      pbVar63 = (byte *)((long)pvVar57 + 0x20);
      uVar70 = uVar58;
      do {
        bVar71 = pbVar60[-0x20];
        bVar72 = pbVar60[-0x1f];
        bVar73 = pbVar60[-0x1e];
        bVar74 = pbVar60[-0x1d];
        bVar75 = pbVar60[-0x1c];
        bVar76 = pbVar60[-0x1b];
        bVar77 = pbVar60[-0x1a];
        bVar79 = pbVar60[-0x19];
        bVar80 = pbVar60[-0x18];
        bVar81 = pbVar60[-0x17];
        bVar82 = pbVar60[-0x16];
        bVar83 = pbVar60[-0x15];
        bVar84 = pbVar60[-0x14];
        bVar85 = pbVar60[-0x13];
        bVar3 = pbVar60[-0x12];
        bVar4 = pbVar60[-0x11];
        bVar5 = pbVar60[-0x10];
        bVar6 = pbVar60[-0xf];
        bVar7 = pbVar60[-0xe];
        bVar8 = pbVar60[-0xd];
        bVar9 = pbVar60[-0xc];
        bVar10 = pbVar60[-0xb];
        bVar11 = pbVar60[-10];
        bVar12 = pbVar60[-9];
        bVar13 = pbVar60[-8];
        bVar14 = pbVar60[-7];
        bVar15 = pbVar60[-6];
        bVar16 = pbVar60[-5];
        bVar17 = pbVar60[-4];
        bVar18 = pbVar60[-3];
        bVar19 = pbVar60[-2];
        bVar20 = pbVar60[-1];
        bVar21 = *pbVar60;
        bVar22 = pbVar60[1];
        bVar23 = pbVar60[2];
        bVar24 = pbVar60[3];
        bVar25 = pbVar60[4];
        bVar26 = pbVar60[5];
        bVar27 = pbVar60[6];
        bVar28 = pbVar60[7];
        bVar29 = pbVar60[8];
        bVar30 = pbVar60[9];
        bVar31 = pbVar60[10];
        bVar32 = pbVar60[0xb];
        bVar33 = pbVar60[0xc];
        bVar34 = pbVar60[0xd];
        bVar35 = pbVar60[0xe];
        bVar36 = pbVar60[0xf];
        bVar37 = pbVar60[0x10];
        bVar38 = pbVar60[0x11];
        bVar39 = pbVar60[0x12];
        bVar40 = pbVar60[0x13];
        bVar41 = pbVar60[0x14];
        bVar42 = pbVar60[0x15];
        bVar43 = pbVar60[0x16];
        bVar44 = pbVar60[0x17];
        bVar45 = pbVar60[0x18];
        bVar46 = pbVar60[0x19];
        bVar47 = pbVar60[0x1a];
        bVar48 = pbVar60[0x1b];
        bVar49 = pbVar60[0x1c];
        bVar50 = pbVar60[0x1d];
        bVar51 = pbVar60[0x1e];
        bVar52 = pbVar60[0x1f];
        pbVar63[-0x18] = bVar80 ^ (bVar80 ^ 0x2f) & -(bVar80 == 0x5c);
        pbVar63[-0x17] = bVar81 ^ (bVar81 ^ 0x2f) & -(bVar81 == 0x5c);
        pbVar63[-0x16] = bVar82 ^ (bVar82 ^ 0x2f) & -(bVar82 == 0x5c);
        pbVar63[-0x15] = bVar83 ^ (bVar83 ^ 0x2f) & -(bVar83 == 0x5c);
        pbVar63[-0x14] = bVar84 ^ (bVar84 ^ 0x2f) & -(bVar84 == 0x5c);
        pbVar63[-0x13] = bVar85 ^ (bVar85 ^ 0x2f) & -(bVar85 == 0x5c);
        pbVar63[-0x12] = bVar3 ^ (bVar3 ^ 0x2f) & -(bVar3 == 0x5c);
        pbVar63[-0x11] = bVar4 ^ (bVar4 ^ 0x2f) & -(bVar4 == 0x5c);
        pbVar63[-0x20] = bVar71 ^ (bVar71 ^ 0x2f) & -(bVar71 == 0x5c);
        pbVar63[-0x1f] = bVar72 ^ (bVar72 ^ 0x2f) & -(bVar72 == 0x5c);
        pbVar63[-0x1e] = bVar73 ^ (bVar73 ^ 0x2f) & -(bVar73 == 0x5c);
        pbVar63[-0x1d] = bVar74 ^ (bVar74 ^ 0x2f) & -(bVar74 == 0x5c);
        pbVar63[-0x1c] = bVar75 ^ (bVar75 ^ 0x2f) & -(bVar75 == 0x5c);
        pbVar63[-0x1b] = bVar76 ^ (bVar76 ^ 0x2f) & -(bVar76 == 0x5c);
        pbVar63[-0x1a] = bVar77 ^ (bVar77 ^ 0x2f) & -(bVar77 == 0x5c);
        pbVar63[-0x19] = bVar79 ^ (bVar79 ^ 0x2f) & -(bVar79 == 0x5c);
        pbVar63[-8] = bVar13 ^ (bVar13 ^ 0x2f) & -(bVar13 == 0x5c);
        pbVar63[-7] = bVar14 ^ (bVar14 ^ 0x2f) & -(bVar14 == 0x5c);
        pbVar63[-6] = bVar15 ^ (bVar15 ^ 0x2f) & -(bVar15 == 0x5c);
        pbVar63[-5] = bVar16 ^ (bVar16 ^ 0x2f) & -(bVar16 == 0x5c);
        pbVar63[-4] = bVar17 ^ (bVar17 ^ 0x2f) & -(bVar17 == 0x5c);
        pbVar63[-3] = bVar18 ^ (bVar18 ^ 0x2f) & -(bVar18 == 0x5c);
        pbVar63[-2] = bVar19 ^ (bVar19 ^ 0x2f) & -(bVar19 == 0x5c);
        pbVar63[-1] = bVar20 ^ (bVar20 ^ 0x2f) & -(bVar20 == 0x5c);
        pbVar63[-0x10] = bVar5 ^ (bVar5 ^ 0x2f) & -(bVar5 == 0x5c);
        pbVar63[-0xf] = bVar6 ^ (bVar6 ^ 0x2f) & -(bVar6 == 0x5c);
        pbVar63[-0xe] = bVar7 ^ (bVar7 ^ 0x2f) & -(bVar7 == 0x5c);
        pbVar63[-0xd] = bVar8 ^ (bVar8 ^ 0x2f) & -(bVar8 == 0x5c);
        pbVar63[-0xc] = bVar9 ^ (bVar9 ^ 0x2f) & -(bVar9 == 0x5c);
        pbVar63[-0xb] = bVar10 ^ (bVar10 ^ 0x2f) & -(bVar10 == 0x5c);
        pbVar63[-10] = bVar11 ^ (bVar11 ^ 0x2f) & -(bVar11 == 0x5c);
        pbVar63[-9] = bVar12 ^ (bVar12 ^ 0x2f) & -(bVar12 == 0x5c);
        pbVar63[8] = bVar29 ^ (bVar29 ^ 0x2f) & -(bVar29 == 0x5c);
        pbVar63[9] = bVar30 ^ (bVar30 ^ 0x2f) & -(bVar30 == 0x5c);
        pbVar63[10] = bVar31 ^ (bVar31 ^ 0x2f) & -(bVar31 == 0x5c);
        pbVar63[0xb] = bVar32 ^ (bVar32 ^ 0x2f) & -(bVar32 == 0x5c);
        pbVar63[0xc] = bVar33 ^ (bVar33 ^ 0x2f) & -(bVar33 == 0x5c);
        pbVar63[0xd] = bVar34 ^ (bVar34 ^ 0x2f) & -(bVar34 == 0x5c);
        pbVar63[0xe] = bVar35 ^ (bVar35 ^ 0x2f) & -(bVar35 == 0x5c);
        pbVar63[0xf] = bVar36 ^ (bVar36 ^ 0x2f) & -(bVar36 == 0x5c);
        *pbVar63 = bVar21 ^ (bVar21 ^ 0x2f) & -(bVar21 == 0x5c);
        pbVar63[1] = bVar22 ^ (bVar22 ^ 0x2f) & -(bVar22 == 0x5c);
        pbVar63[2] = bVar23 ^ (bVar23 ^ 0x2f) & -(bVar23 == 0x5c);
        pbVar63[3] = bVar24 ^ (bVar24 ^ 0x2f) & -(bVar24 == 0x5c);
        pbVar63[4] = bVar25 ^ (bVar25 ^ 0x2f) & -(bVar25 == 0x5c);
        pbVar63[5] = bVar26 ^ (bVar26 ^ 0x2f) & -(bVar26 == 0x5c);
        pbVar63[6] = bVar27 ^ (bVar27 ^ 0x2f) & -(bVar27 == 0x5c);
        pbVar63[7] = bVar28 ^ (bVar28 ^ 0x2f) & -(bVar28 == 0x5c);
        pbVar63[0x18] = bVar45 ^ (bVar45 ^ 0x2f) & -(bVar45 == 0x5c);
        pbVar63[0x19] = bVar46 ^ (bVar46 ^ 0x2f) & -(bVar46 == 0x5c);
        pbVar63[0x1a] = bVar47 ^ (bVar47 ^ 0x2f) & -(bVar47 == 0x5c);
        pbVar63[0x1b] = bVar48 ^ (bVar48 ^ 0x2f) & -(bVar48 == 0x5c);
        pbVar63[0x1c] = bVar49 ^ (bVar49 ^ 0x2f) & -(bVar49 == 0x5c);
        pbVar63[0x1d] = bVar50 ^ (bVar50 ^ 0x2f) & -(bVar50 == 0x5c);
        pbVar63[0x1e] = bVar51 ^ (bVar51 ^ 0x2f) & -(bVar51 == 0x5c);
        pbVar63[0x1f] = bVar52 ^ (bVar52 ^ 0x2f) & -(bVar52 == 0x5c);
        pbVar63[0x10] = bVar37 ^ (bVar37 ^ 0x2f) & -(bVar37 == 0x5c);
        pbVar63[0x11] = bVar38 ^ (bVar38 ^ 0x2f) & -(bVar38 == 0x5c);
        pbVar63[0x12] = bVar39 ^ (bVar39 ^ 0x2f) & -(bVar39 == 0x5c);
        pbVar63[0x13] = bVar40 ^ (bVar40 ^ 0x2f) & -(bVar40 == 0x5c);
        pbVar63[0x14] = bVar41 ^ (bVar41 ^ 0x2f) & -(bVar41 == 0x5c);
        pbVar63[0x15] = bVar42 ^ (bVar42 ^ 0x2f) & -(bVar42 == 0x5c);
        pbVar63[0x16] = bVar43 ^ (bVar43 ^ 0x2f) & -(bVar43 == 0x5c);
        pbVar63[0x17] = bVar44 ^ (bVar44 ^ 0x2f) & -(bVar44 == 0x5c);
        uVar70 = uVar70 - 0x40;
        pbVar60 = pbVar60 + 0x40;
        pbVar63 = pbVar63 + 0x40;
      } while (uVar70 != 0);
      if (uVar78 != uVar58) {
        uVar70 = uVar58;
        if ((uVar78 & 0x38) == 0) goto LAB_1007c387c;
        goto LAB_1007c3840;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    pvVar55 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(uVar78,1);
    if (pvVar55 == (void *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,uVar78);
    }
    _memcpy(pvVar55,pvVar57,uVar78);
    if (uVar78 < 8) {
      uVar70 = 0;
LAB_1007c39ac:
      lVar61 = uVar78 - uVar70;
      pbVar60 = (byte *)((long)pvVar55 + uVar70);
      do {
        bVar71 = 0x20;
        if (0x19 < *pbVar60 - 0x41) {
          bVar71 = 0;
        }
        *pbVar60 = bVar71 | *pbVar60;
        lVar61 = lVar61 + -1;
        pbVar60 = pbVar60 + 1;
      } while (lVar61 != 0);
    }
    else if (uVar78 < 0x40) {
      uVar58 = 0;
LAB_1007c3968:
      uVar70 = uVar78 & 0x7ffffffffffffff8;
      lVar61 = uVar58 - uVar70;
      puVar66 = (undefined8 *)((long)pvVar55 + uVar58);
      do {
        uVar68 = *puVar66;
        bVar71 = (byte)((ulong)uVar68 >> 8);
        bVar72 = (byte)((ulong)uVar68 >> 0x10);
        bVar73 = (byte)((ulong)uVar68 >> 0x18);
        bVar74 = (byte)((ulong)uVar68 >> 0x20);
        bVar75 = (byte)((ulong)uVar68 >> 0x28);
        bVar76 = (byte)((ulong)uVar68 >> 0x30);
        bVar77 = (byte)((ulong)uVar68 >> 0x38);
        uVar58 = CONCAT17(-((byte)(bVar77 + 0xbf) < 0x1a),
                          CONCAT16(-((byte)(bVar76 + 0xbf) < 0x1a),
                                   CONCAT15(-((byte)(bVar75 + 0xbf) < 0x1a),
                                            CONCAT14(-((byte)(bVar74 + 0xbf) < 0x1a),
                                                     CONCAT13(-((byte)(bVar73 + 0xbf) < 0x1a),
                                                              CONCAT12(-((byte)(bVar72 + 0xbf) < 0x1a),
                                                                       CONCAT11(-((byte)(bVar71 + 0xbf) < 0x1a),-((byte)((byte)uVar68 + 0xbf) < 0x1a)))))))) &
                 0x2020202020202020;
        *puVar66 = CONCAT17((byte)(uVar58 >> 0x38) | bVar77,
                            CONCAT16((byte)(uVar58 >> 0x30) | bVar76,
                                     CONCAT15((byte)(uVar58 >> 0x28) | bVar75,
                                              CONCAT14((byte)(uVar58 >> 0x20) | bVar74,
                                                       CONCAT13((byte)(uVar58 >> 0x18) | bVar73,
                                                                CONCAT12((byte)(uVar58 >> 0x10) | bVar72,
                                                                         CONCAT11((byte)(uVar58 >> 8) | bVar71,(byte)uVar58 | (byte)uVar68)))))));
        lVar61 = lVar61 + 8;
        puVar66 = puVar66 + 1;
      } while (lVar61 != 0);
      if (uVar78 != uVar70) goto LAB_1007c39ac;
    }
    else {
      uVar70 = uVar78 & 0x7fffffffffffffc0;
      pbVar60 = (byte *)((long)pvVar55 + 0x20);
      uVar58 = uVar70;
      do {
        uVar2 = *(undefined8 *)(pbVar60 + 0x18);
        bVar79 = (byte)((ulong)uVar2 >> 8);
        bVar80 = (byte)((ulong)uVar2 >> 0x10);
        bVar81 = (byte)((ulong)uVar2 >> 0x18);
        bVar82 = (byte)((ulong)uVar2 >> 0x20);
        bVar83 = (byte)((ulong)uVar2 >> 0x28);
        bVar84 = (byte)((ulong)uVar2 >> 0x30);
        bVar85 = (byte)((ulong)uVar2 >> 0x38);
        uVar68 = *(undefined8 *)(pbVar60 + 0x10);
        bVar71 = (byte)((ulong)uVar68 >> 8);
        bVar72 = (byte)((ulong)uVar68 >> 0x10);
        bVar73 = (byte)((ulong)uVar68 >> 0x18);
        bVar74 = (byte)((ulong)uVar68 >> 0x20);
        bVar75 = (byte)((ulong)uVar68 >> 0x28);
        bVar76 = (byte)((ulong)uVar68 >> 0x30);
        bVar77 = (byte)((ulong)uVar68 >> 0x38);
        pbVar60[-0x18] = -((byte)(pbVar60[-0x18] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x18];
        pbVar60[-0x17] = -((byte)(pbVar60[-0x17] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x17];
        pbVar60[-0x16] = -((byte)(pbVar60[-0x16] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x16];
        pbVar60[-0x15] = -((byte)(pbVar60[-0x15] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x15];
        pbVar60[-0x14] = -((byte)(pbVar60[-0x14] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x14];
        pbVar60[-0x13] = -((byte)(pbVar60[-0x13] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x13];
        pbVar60[-0x12] = -((byte)(pbVar60[-0x12] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x12];
        pbVar60[-0x11] = -((byte)(pbVar60[-0x11] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x11];
        pbVar60[-0x20] = -((byte)(pbVar60[-0x20] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x20];
        pbVar60[-0x1f] = -((byte)(pbVar60[-0x1f] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1f];
        pbVar60[-0x1e] = -((byte)(pbVar60[-0x1e] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1e];
        pbVar60[-0x1d] = -((byte)(pbVar60[-0x1d] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1d];
        pbVar60[-0x1c] = -((byte)(pbVar60[-0x1c] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1c];
        pbVar60[-0x1b] = -((byte)(pbVar60[-0x1b] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1b];
        pbVar60[-0x1a] = -((byte)(pbVar60[-0x1a] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x1a];
        pbVar60[-0x19] = -((byte)(pbVar60[-0x19] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x19];
        pbVar60[-8] = -((byte)(pbVar60[-8] + 0xbf) < 0x1a) & 0x20U | pbVar60[-8];
        pbVar60[-7] = -((byte)(pbVar60[-7] + 0xbf) < 0x1a) & 0x20U | pbVar60[-7];
        pbVar60[-6] = -((byte)(pbVar60[-6] + 0xbf) < 0x1a) & 0x20U | pbVar60[-6];
        pbVar60[-5] = -((byte)(pbVar60[-5] + 0xbf) < 0x1a) & 0x20U | pbVar60[-5];
        pbVar60[-4] = -((byte)(pbVar60[-4] + 0xbf) < 0x1a) & 0x20U | pbVar60[-4];
        pbVar60[-3] = -((byte)(pbVar60[-3] + 0xbf) < 0x1a) & 0x20U | pbVar60[-3];
        pbVar60[-2] = -((byte)(pbVar60[-2] + 0xbf) < 0x1a) & 0x20U | pbVar60[-2];
        pbVar60[-1] = -((byte)(pbVar60[-1] + 0xbf) < 0x1a) & 0x20U | pbVar60[-1];
        pbVar60[-0x10] = -((byte)(pbVar60[-0x10] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0x10];
        pbVar60[-0xf] = -((byte)(pbVar60[-0xf] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xf];
        pbVar60[-0xe] = -((byte)(pbVar60[-0xe] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xe];
        pbVar60[-0xd] = -((byte)(pbVar60[-0xd] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xd];
        pbVar60[-0xc] = -((byte)(pbVar60[-0xc] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xc];
        pbVar60[-0xb] = -((byte)(pbVar60[-0xb] + 0xbf) < 0x1a) & 0x20U | pbVar60[-0xb];
        pbVar60[-10] = -((byte)(pbVar60[-10] + 0xbf) < 0x1a) & 0x20U | pbVar60[-10];
        pbVar60[-9] = -((byte)(pbVar60[-9] + 0xbf) < 0x1a) & 0x20U | pbVar60[-9];
        pbVar60[8] = -((byte)(pbVar60[8] + 0xbf) < 0x1a) & 0x20U | pbVar60[8];
        pbVar60[9] = -((byte)(pbVar60[9] + 0xbf) < 0x1a) & 0x20U | pbVar60[9];
        pbVar60[10] = -((byte)(pbVar60[10] + 0xbf) < 0x1a) & 0x20U | pbVar60[10];
        pbVar60[0xb] = -((byte)(pbVar60[0xb] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xb];
        pbVar60[0xc] = -((byte)(pbVar60[0xc] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xc];
        pbVar60[0xd] = -((byte)(pbVar60[0xd] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xd];
        pbVar60[0xe] = -((byte)(pbVar60[0xe] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xe];
        pbVar60[0xf] = -((byte)(pbVar60[0xf] + 0xbf) < 0x1a) & 0x20U | pbVar60[0xf];
        *pbVar60 = -((byte)(*pbVar60 + 0xbf) < 0x1a) & 0x20U | *pbVar60;
        pbVar60[1] = -((byte)(pbVar60[1] + 0xbf) < 0x1a) & 0x20U | pbVar60[1];
        pbVar60[2] = -((byte)(pbVar60[2] + 0xbf) < 0x1a) & 0x20U | pbVar60[2];
        pbVar60[3] = -((byte)(pbVar60[3] + 0xbf) < 0x1a) & 0x20U | pbVar60[3];
        pbVar60[4] = -((byte)(pbVar60[4] + 0xbf) < 0x1a) & 0x20U | pbVar60[4];
        pbVar60[5] = -((byte)(pbVar60[5] + 0xbf) < 0x1a) & 0x20U | pbVar60[5];
        pbVar60[6] = -((byte)(pbVar60[6] + 0xbf) < 0x1a) & 0x20U | pbVar60[6];
        pbVar60[7] = -((byte)(pbVar60[7] + 0xbf) < 0x1a) & 0x20U | pbVar60[7];
        pbVar60[0x18] = -((byte)((byte)uVar2 + 0xbf) < 0x1a) & 0x20U | (byte)uVar2;
        pbVar60[0x19] = -((byte)(bVar79 + 0xbf) < 0x1a) & 0x20U | bVar79;
        pbVar60[0x1a] = -((byte)(bVar80 + 0xbf) < 0x1a) & 0x20U | bVar80;
        pbVar60[0x1b] = -((byte)(bVar81 + 0xbf) < 0x1a) & 0x20U | bVar81;
        pbVar60[0x1c] = -((byte)(bVar82 + 0xbf) < 0x1a) & 0x20U | bVar82;
        pbVar60[0x1d] = -((byte)(bVar83 + 0xbf) < 0x1a) & 0x20U | bVar83;
        pbVar60[0x1e] = -((byte)(bVar84 + 0xbf) < 0x1a) & 0x20U | bVar84;
        pbVar60[0x1f] = -((byte)(bVar85 + 0xbf) < 0x1a) & 0x20U | bVar85;
        pbVar60[0x10] = -((byte)((byte)uVar68 + 0xbf) < 0x1a) & 0x20U | (byte)uVar68;
        pbVar60[0x11] = -((byte)(bVar71 + 0xbf) < 0x1a) & 0x20U | bVar71;
        pbVar60[0x12] = -((byte)(bVar72 + 0xbf) < 0x1a) & 0x20U | bVar72;
        pbVar60[0x13] = -((byte)(bVar73 + 0xbf) < 0x1a) & 0x20U | bVar73;
        pbVar60[0x14] = -((byte)(bVar74 + 0xbf) < 0x1a) & 0x20U | bVar74;
        pbVar60[0x15] = -((byte)(bVar75 + 0xbf) < 0x1a) & 0x20U | bVar75;
        pbVar60[0x16] = -((byte)(bVar76 + 0xbf) < 0x1a) & 0x20U | bVar76;
        pbVar60[0x17] = -((byte)(bVar77 + 0xbf) < 0x1a) & 0x20U | bVar77;
        uVar58 = uVar58 - 0x40;
        pbVar60 = pbVar60 + 0x40;
      } while (uVar58 != 0);
      if (uVar78 != uVar70) {
        uVar58 = uVar70;
        if ((uVar78 & 0x38) == 0) goto LAB_1007c39ac;
        goto LAB_1007c3968;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar57,uVar78,1);
  }
  auVar86._8_8_ = uVar78;
  auVar86._0_8_ = pvVar55;
  local_b8 = 0;
  local_98 = (long *)0x0;
  local_88 = 0x2f0000002f;
  local_80 = CONCAT11(local_80._1_1_,1);
  local_78 = 1;
  uStack_b0 = uVar78;
  local_a8 = pvVar55;
  uStack_a0 = uVar78;
  uStack_90 = uVar78;
  auVar87 = ::core::str::iter::SplitInternal<P>::next_back(&local_b8);
  if (auVar87._0_8_ != 0) {
    auVar86 = auVar87;
  }
  piVar53 = auVar86._0_8_;
  iVar54 = is_openai_ide_extension_writer_path(plVar56,param_2);
  if (iVar54 == 0) {
LAB_1007c3a94:
    if (param_2 < 0x15) {
      if (param_2 != 0x14) goto LAB_1007c3b7c;
      if (((*plVar56 == 0x65646f6373762e2f && plVar56[1] == 0x69736e657478652f) && (int)plVar56[2] == 0x2f736e6f) ||
         ((*plVar56 == 0x726f737275632e2f && plVar56[1] == 0x69736e657478652f) && (int)plVar56[2] == 0x2f736e6f)) goto LAB_1007c3f70;
      goto LAB_1007c3b84;
    }
    ::core::str::pattern::StrSearcher::new
              (&local_b8,plVar56,param_2,
               "/.vscode/extensions//.vscode-insiders/extensions//.vscode-oss/extensions//.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt",
               0x14);
    <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
    if ((local_d0[0] & 1) == 0) {
      if (param_2 < 0x1e) {
        if (param_2 == 0x1d) {
          if (((*plVar56 == 0x65646f6373762e2f && plVar56[1] == 0x72656469736e692d) && plVar56[2] == 0x736e657478652f73) &&
              *(long *)((long)plVar56 + 0x15) == 0x2f736e6f69736e65) goto LAB_1007c3f70;
LAB_1007c3d7c:
          ::core::str::pattern::StrSearcher::new
                    (&local_b8,plVar56,param_2,"/.vscode-oss/extensions//.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt",0x18);
          <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
          if ((local_d0[0] & 1) != 0) goto LAB_1007c3f70;
        }
        else {
          if (0x18 < param_2) goto LAB_1007c3d7c;
          if ((param_2 == 0x18) && ((*plVar56 == 0x65646f6373762e2f && plVar56[1] == 0x7478652f73736f2d) && plVar56[2] == 0x2f736e6f69736e65))
          goto LAB_1007c3f70;
        }
        ::core::str::pattern::StrSearcher::new(&local_b8,plVar56,param_2,"/.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt",0x14);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
        if ((local_d0[0] & 1) == 0) {
          if (param_2 < 0x17) {
            if ((param_2 != 0x16) ||
               ((*plVar56 != 0x7573646e69772e2f || plVar56[1] != 0x6e657478652f6672) || *(long *)((long)plVar56 + 0xe) != 0x2f736e6f69736e65))
            goto LAB_1007c3b84;
          }
          else {
            ::core::str::pattern::StrSearcher::new(&local_b8,plVar56,param_2,"/.windsurf/extensions//extensions/openai.chatgpt",0x16);
            <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
            if ((local_d0[0] & 1) == 0) {
LAB_1007c3b84:
              ::core::str::pattern::StrSearcher::new(&local_b8,param_1,param_2,&DAT_1013adb4a,0xc);
              <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
              if ((local_d0[0] & 1) == 0) {
                if (param_2 < 0x16) {
                  if ((param_2 != 0x15) ||
                     ((*param_1 != 0x2f7865646f632e2f || param_1[1] != 0x72657475706d6f63) || *(long *)((long)param_1 + 0xd) != 0x2f6573752d726574))
                  goto LAB_1007c3f10;
                }
                else {
                  ::core::str::pattern::StrSearcher::new(&local_b8,param_1,param_2,&DAT_1013adb56,0x15);
                  <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
                  if ((local_d0[0] & 1) == 0) {
                    if (param_2 < 0x19) {
                      if ((param_2 != 0x18) || ((*param_1 != 0x43207865646f432f || param_1[1] != 0x2072657475706d6f) || param_1[2] != 0x2f7070612e657355))
                      goto LAB_1007c3f10;
                    }
                    else {
                      ::core::str::pattern::StrSearcher::new(&local_b8,param_1,param_2,&DAT_1013adb6b,0x18);
                      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
                      if ((local_d0[0] & 1) == 0) {
LAB_1007c3f10:
                        ::core::str::pattern::StrSearcher::new(&local_b8,param_1,param_2,&DAT_1013adb83,0xc);
                        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
                        if ((local_d0[0] & 1) == 0) goto LAB_1007c3f40;
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      else {
        ::core::str::pattern::StrSearcher::new
                  (&local_b8,plVar56,param_2,
                   "/.vscode-insiders/extensions//.vscode-oss/extensions//.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt",0x1d);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
        if ((local_d0[0] & 1) == 0) goto LAB_1007c3d7c;
      }
    }
LAB_1007c3f70:
    uVar69 = 1;
  }
  else {
    if (auVar86._8_8_ == 5) {
      if (*piVar53 != 0x65646f63 || (char)piVar53[1] != 'x') goto LAB_1007c3a94;
    }
    else if ((auVar86._8_8_ < 6) || ((*piVar53 != 0x65646f63 || (short)piVar53[1] != 0x2d78 && (*piVar53 != 0x65646f63 || (short)piVar53[1] != 0x5f78))))
    goto LAB_1007c3a94;
LAB_1007c3b7c:
    if (0xc < param_2) goto LAB_1007c3b84;
    if (param_2 == 0xc) {
      if ((*param_1 == 0x2e726f737275432f && (int)param_1[1] == 0x2f707061) || (*param_1 == 0x2e694d614d69412f && (int)param_1[1] == 0x2f707061))
      goto LAB_1007c3f70;
LAB_1007c3f40:
      ::core::str::pattern::StrSearcher::new(&local_b8,param_1,param_2,&DAT_1013af2f8,8);
      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
      if ((local_d0[0] & 1) != 0) goto LAB_1007c3f70;
      if (0x10 < param_2) {
        ::core::str::pattern::StrSearcher::new(&local_b8,param_1,param_2,&DAT_1013af1b0,0x10);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(local_d0,&local_b8);
        uVar69 = local_d0[0];
        goto LAB_1007c3f74;
      }
      if (param_2 == 0x10) {
        uVar69 = (uint)(*param_1 == 0x6461706873617263 && param_1[1] == 0x72656c646e61685f);
        goto LAB_1007c3f74;
      }
    }
    else {
      if (8 < param_2) goto LAB_1007c3f40;
      if (param_2 == 8) {
        uVar69 = (uint)(*param_1 == 0x20694d614d69412f);
        goto LAB_1007c3f74;
      }
    }
    uVar69 = 0;
  }
LAB_1007c3f74:
  if (uVar78 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar55,uVar78,1);
  }
  if (param_2 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar56,param_2,1);
  }
  return uVar69;
}

