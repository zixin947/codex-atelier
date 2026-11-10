/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* classify_macos_codex_writer @ 1007c1870 requested 1007c1870 */

/* codexmate_lib::platform::process::classify_macos_codex_writer */

undefined8 codexmate_lib::platform::process::classify_macos_codex_writer(long param_1,ulong param_2)

{
  char cVar1;
  long *plVar2;
  undefined8 uVar3;
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
  undefined1 auVar54 [16];
  bool bVar55;
  uint uVar56;
  void *pvVar57;
  long *plVar58;
  ulong uVar59;
  long *plVar60;
  uint uVar61;
  ulong uVar62;
  ulong uVar63;
  char *pcVar64;
  byte *pbVar65;
  long lVar66;
  long lVar67;
  byte *pbVar68;
  ulong *puVar69;
  char *pcVar70;
  undefined8 *puVar71;
  ulong *puVar72;
  undefined8 uVar73;
  ulong uVar74;
  byte bVar75;
  byte bVar76;
  byte bVar77;
  byte bVar78;
  byte bVar79;
  byte bVar80;
  byte bVar81;
  byte bVar82;
  byte bVar83;
  byte bVar84;
  byte bVar85;
  byte bVar86;
  byte bVar87;
  byte bVar88;
  undefined1 auVar89 [16];
  undefined1 auVar90 [16];
  ulong local_128;
  ulong uStack_120;
  long local_118;
  ulong uStack_110;
  long local_108;
  long lStack_100;
  undefined8 local_f8;
  undefined2 local_f0;
  long local_e8;
  long *local_e0;
  ulong uStack_d8;
  long local_d0;
  ulong local_c8;
  long *local_c0;
  ulong uStack_b8;
  long local_b0;
  ulong uStack_a8;
  undefined8 local_a0;
  undefined2 local_98;
  undefined2 local_90;
  
  auVar90._8_8_ = param_2;
  auVar90._0_8_ = param_1;
  if ((long)param_2 < 0) {
    uVar73 = 0;
LAB_1007c189c:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar73,param_2);
  }
  if (param_2 == 0) {
    uVar59 = is_openai_ide_extension_writer_path(param_1,0);
    plVar58 = (long *)0x1;
    if ((uVar59 & 1) != 0) {
      return 1;
    }
    uVar59 = 0;
LAB_1007c18f8:
    bVar55 = false;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar73 = 1;
    pvVar57 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_2,1);
    if (pvVar57 == (void *)0x0) goto LAB_1007c189c;
    if (param_2 < 8) {
      uVar62 = 0;
LAB_1007c19f0:
      lVar66 = param_2 - uVar62;
      pcVar64 = (char *)(param_1 + uVar62);
      pcVar70 = (char *)((long)pvVar57 + uVar62);
      do {
        cVar1 = '/';
        if (*pcVar64 != '\\') {
          cVar1 = *pcVar64;
        }
        *pcVar70 = cVar1;
        lVar66 = lVar66 + -1;
        pcVar64 = pcVar64 + 1;
        pcVar70 = pcVar70 + 1;
      } while (lVar66 != 0);
    }
    else if (param_2 < 0x40) {
      uVar59 = 0;
LAB_1007c19b4:
      uVar62 = param_2 & 0x7ffffffffffffff8;
      lVar66 = uVar59 - uVar62;
      puVar69 = (ulong *)((long)pvVar57 + uVar59);
      puVar72 = (ulong *)(param_1 + uVar59);
      do {
        uVar59 = *puVar72;
        *puVar69 = uVar59 ^ (uVar59 ^ 0x2f2f2f2f2f2f2f2f) &
                            CONCAT17(-((char)(uVar59 >> 0x38) == '\\'),
                                     CONCAT16(-((char)(uVar59 >> 0x30) == '\\'),
                                              CONCAT15(-((char)(uVar59 >> 0x28) == '\\'),
                                                       CONCAT14(-((char)(uVar59 >> 0x20) == '\\'),
                                                                CONCAT13(-((char)(uVar59 >> 0x18) == '\\'),
                                                                         CONCAT12(-((char)(uVar59 >> 0x10) == '\\'),
                                                                                  CONCAT11(-((char)(uVar59 >> 8) == '\\'),-((char)uVar59 == '\\'))))))));
        lVar66 = lVar66 + 8;
        puVar69 = puVar69 + 1;
        puVar72 = puVar72 + 1;
      } while (lVar66 != 0);
      if (param_2 != uVar62) goto LAB_1007c19f0;
    }
    else {
      uVar62 = param_2 & 0x7fffffffffffffc0;
      pbVar65 = (byte *)(param_1 + 0x20);
      pbVar68 = (byte *)((long)pvVar57 + 0x20);
      uVar59 = uVar62;
      do {
        bVar75 = pbVar65[-0x20];
        bVar76 = pbVar65[-0x1f];
        bVar77 = pbVar65[-0x1e];
        bVar78 = pbVar65[-0x1d];
        bVar79 = pbVar65[-0x1c];
        bVar80 = pbVar65[-0x1b];
        bVar81 = pbVar65[-0x1a];
        bVar82 = pbVar65[-0x19];
        bVar83 = pbVar65[-0x18];
        bVar84 = pbVar65[-0x17];
        bVar85 = pbVar65[-0x16];
        bVar86 = pbVar65[-0x15];
        bVar87 = pbVar65[-0x14];
        bVar88 = pbVar65[-0x13];
        bVar4 = pbVar65[-0x12];
        bVar5 = pbVar65[-0x11];
        bVar6 = pbVar65[-0x10];
        bVar7 = pbVar65[-0xf];
        bVar8 = pbVar65[-0xe];
        bVar9 = pbVar65[-0xd];
        bVar10 = pbVar65[-0xc];
        bVar11 = pbVar65[-0xb];
        bVar12 = pbVar65[-10];
        bVar13 = pbVar65[-9];
        bVar14 = pbVar65[-8];
        bVar15 = pbVar65[-7];
        bVar16 = pbVar65[-6];
        bVar17 = pbVar65[-5];
        bVar18 = pbVar65[-4];
        bVar19 = pbVar65[-3];
        bVar20 = pbVar65[-2];
        bVar21 = pbVar65[-1];
        bVar22 = *pbVar65;
        bVar23 = pbVar65[1];
        bVar24 = pbVar65[2];
        bVar25 = pbVar65[3];
        bVar26 = pbVar65[4];
        bVar27 = pbVar65[5];
        bVar28 = pbVar65[6];
        bVar29 = pbVar65[7];
        bVar30 = pbVar65[8];
        bVar31 = pbVar65[9];
        bVar32 = pbVar65[10];
        bVar33 = pbVar65[0xb];
        bVar34 = pbVar65[0xc];
        bVar35 = pbVar65[0xd];
        bVar36 = pbVar65[0xe];
        bVar37 = pbVar65[0xf];
        bVar38 = pbVar65[0x10];
        bVar39 = pbVar65[0x11];
        bVar40 = pbVar65[0x12];
        bVar41 = pbVar65[0x13];
        bVar42 = pbVar65[0x14];
        bVar43 = pbVar65[0x15];
        bVar44 = pbVar65[0x16];
        bVar45 = pbVar65[0x17];
        bVar46 = pbVar65[0x18];
        bVar47 = pbVar65[0x19];
        bVar48 = pbVar65[0x1a];
        bVar49 = pbVar65[0x1b];
        bVar50 = pbVar65[0x1c];
        bVar51 = pbVar65[0x1d];
        bVar52 = pbVar65[0x1e];
        bVar53 = pbVar65[0x1f];
        pbVar68[-0x18] = bVar83 ^ (bVar83 ^ 0x2f) & -(bVar83 == 0x5c);
        pbVar68[-0x17] = bVar84 ^ (bVar84 ^ 0x2f) & -(bVar84 == 0x5c);
        pbVar68[-0x16] = bVar85 ^ (bVar85 ^ 0x2f) & -(bVar85 == 0x5c);
        pbVar68[-0x15] = bVar86 ^ (bVar86 ^ 0x2f) & -(bVar86 == 0x5c);
        pbVar68[-0x14] = bVar87 ^ (bVar87 ^ 0x2f) & -(bVar87 == 0x5c);
        pbVar68[-0x13] = bVar88 ^ (bVar88 ^ 0x2f) & -(bVar88 == 0x5c);
        pbVar68[-0x12] = bVar4 ^ (bVar4 ^ 0x2f) & -(bVar4 == 0x5c);
        pbVar68[-0x11] = bVar5 ^ (bVar5 ^ 0x2f) & -(bVar5 == 0x5c);
        pbVar68[-0x20] = bVar75 ^ (bVar75 ^ 0x2f) & -(bVar75 == 0x5c);
        pbVar68[-0x1f] = bVar76 ^ (bVar76 ^ 0x2f) & -(bVar76 == 0x5c);
        pbVar68[-0x1e] = bVar77 ^ (bVar77 ^ 0x2f) & -(bVar77 == 0x5c);
        pbVar68[-0x1d] = bVar78 ^ (bVar78 ^ 0x2f) & -(bVar78 == 0x5c);
        pbVar68[-0x1c] = bVar79 ^ (bVar79 ^ 0x2f) & -(bVar79 == 0x5c);
        pbVar68[-0x1b] = bVar80 ^ (bVar80 ^ 0x2f) & -(bVar80 == 0x5c);
        pbVar68[-0x1a] = bVar81 ^ (bVar81 ^ 0x2f) & -(bVar81 == 0x5c);
        pbVar68[-0x19] = bVar82 ^ (bVar82 ^ 0x2f) & -(bVar82 == 0x5c);
        pbVar68[-8] = bVar14 ^ (bVar14 ^ 0x2f) & -(bVar14 == 0x5c);
        pbVar68[-7] = bVar15 ^ (bVar15 ^ 0x2f) & -(bVar15 == 0x5c);
        pbVar68[-6] = bVar16 ^ (bVar16 ^ 0x2f) & -(bVar16 == 0x5c);
        pbVar68[-5] = bVar17 ^ (bVar17 ^ 0x2f) & -(bVar17 == 0x5c);
        pbVar68[-4] = bVar18 ^ (bVar18 ^ 0x2f) & -(bVar18 == 0x5c);
        pbVar68[-3] = bVar19 ^ (bVar19 ^ 0x2f) & -(bVar19 == 0x5c);
        pbVar68[-2] = bVar20 ^ (bVar20 ^ 0x2f) & -(bVar20 == 0x5c);
        pbVar68[-1] = bVar21 ^ (bVar21 ^ 0x2f) & -(bVar21 == 0x5c);
        pbVar68[-0x10] = bVar6 ^ (bVar6 ^ 0x2f) & -(bVar6 == 0x5c);
        pbVar68[-0xf] = bVar7 ^ (bVar7 ^ 0x2f) & -(bVar7 == 0x5c);
        pbVar68[-0xe] = bVar8 ^ (bVar8 ^ 0x2f) & -(bVar8 == 0x5c);
        pbVar68[-0xd] = bVar9 ^ (bVar9 ^ 0x2f) & -(bVar9 == 0x5c);
        pbVar68[-0xc] = bVar10 ^ (bVar10 ^ 0x2f) & -(bVar10 == 0x5c);
        pbVar68[-0xb] = bVar11 ^ (bVar11 ^ 0x2f) & -(bVar11 == 0x5c);
        pbVar68[-10] = bVar12 ^ (bVar12 ^ 0x2f) & -(bVar12 == 0x5c);
        pbVar68[-9] = bVar13 ^ (bVar13 ^ 0x2f) & -(bVar13 == 0x5c);
        pbVar68[8] = bVar30 ^ (bVar30 ^ 0x2f) & -(bVar30 == 0x5c);
        pbVar68[9] = bVar31 ^ (bVar31 ^ 0x2f) & -(bVar31 == 0x5c);
        pbVar68[10] = bVar32 ^ (bVar32 ^ 0x2f) & -(bVar32 == 0x5c);
        pbVar68[0xb] = bVar33 ^ (bVar33 ^ 0x2f) & -(bVar33 == 0x5c);
        pbVar68[0xc] = bVar34 ^ (bVar34 ^ 0x2f) & -(bVar34 == 0x5c);
        pbVar68[0xd] = bVar35 ^ (bVar35 ^ 0x2f) & -(bVar35 == 0x5c);
        pbVar68[0xe] = bVar36 ^ (bVar36 ^ 0x2f) & -(bVar36 == 0x5c);
        pbVar68[0xf] = bVar37 ^ (bVar37 ^ 0x2f) & -(bVar37 == 0x5c);
        *pbVar68 = bVar22 ^ (bVar22 ^ 0x2f) & -(bVar22 == 0x5c);
        pbVar68[1] = bVar23 ^ (bVar23 ^ 0x2f) & -(bVar23 == 0x5c);
        pbVar68[2] = bVar24 ^ (bVar24 ^ 0x2f) & -(bVar24 == 0x5c);
        pbVar68[3] = bVar25 ^ (bVar25 ^ 0x2f) & -(bVar25 == 0x5c);
        pbVar68[4] = bVar26 ^ (bVar26 ^ 0x2f) & -(bVar26 == 0x5c);
        pbVar68[5] = bVar27 ^ (bVar27 ^ 0x2f) & -(bVar27 == 0x5c);
        pbVar68[6] = bVar28 ^ (bVar28 ^ 0x2f) & -(bVar28 == 0x5c);
        pbVar68[7] = bVar29 ^ (bVar29 ^ 0x2f) & -(bVar29 == 0x5c);
        pbVar68[0x18] = bVar46 ^ (bVar46 ^ 0x2f) & -(bVar46 == 0x5c);
        pbVar68[0x19] = bVar47 ^ (bVar47 ^ 0x2f) & -(bVar47 == 0x5c);
        pbVar68[0x1a] = bVar48 ^ (bVar48 ^ 0x2f) & -(bVar48 == 0x5c);
        pbVar68[0x1b] = bVar49 ^ (bVar49 ^ 0x2f) & -(bVar49 == 0x5c);
        pbVar68[0x1c] = bVar50 ^ (bVar50 ^ 0x2f) & -(bVar50 == 0x5c);
        pbVar68[0x1d] = bVar51 ^ (bVar51 ^ 0x2f) & -(bVar51 == 0x5c);
        pbVar68[0x1e] = bVar52 ^ (bVar52 ^ 0x2f) & -(bVar52 == 0x5c);
        pbVar68[0x1f] = bVar53 ^ (bVar53 ^ 0x2f) & -(bVar53 == 0x5c);
        pbVar68[0x10] = bVar38 ^ (bVar38 ^ 0x2f) & -(bVar38 == 0x5c);
        pbVar68[0x11] = bVar39 ^ (bVar39 ^ 0x2f) & -(bVar39 == 0x5c);
        pbVar68[0x12] = bVar40 ^ (bVar40 ^ 0x2f) & -(bVar40 == 0x5c);
        pbVar68[0x13] = bVar41 ^ (bVar41 ^ 0x2f) & -(bVar41 == 0x5c);
        pbVar68[0x14] = bVar42 ^ (bVar42 ^ 0x2f) & -(bVar42 == 0x5c);
        pbVar68[0x15] = bVar43 ^ (bVar43 ^ 0x2f) & -(bVar43 == 0x5c);
        pbVar68[0x16] = bVar44 ^ (bVar44 ^ 0x2f) & -(bVar44 == 0x5c);
        pbVar68[0x17] = bVar45 ^ (bVar45 ^ 0x2f) & -(bVar45 == 0x5c);
        uVar59 = uVar59 - 0x40;
        pbVar65 = pbVar65 + 0x40;
        pbVar68 = pbVar68 + 0x40;
      } while (uVar59 != 0);
      if (param_2 != uVar62) {
        uVar59 = uVar62;
        if ((param_2 & 0x38) == 0) goto LAB_1007c19f0;
        goto LAB_1007c19b4;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    plVar58 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_2,1);
    if (plVar58 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,param_2);
    }
    _memcpy(plVar58,pvVar57,param_2);
    if (param_2 < 8) {
      uVar62 = 0;
LAB_1007c1b20:
      lVar66 = param_2 - uVar62;
      pbVar65 = (byte *)((long)plVar58 + uVar62);
      do {
        bVar75 = 0x20;
        if (0x19 < *pbVar65 - 0x41) {
          bVar75 = 0;
        }
        *pbVar65 = bVar75 | *pbVar65;
        lVar66 = lVar66 + -1;
        pbVar65 = pbVar65 + 1;
      } while (lVar66 != 0);
    }
    else if (param_2 < 0x40) {
      uVar59 = 0;
LAB_1007c1adc:
      uVar62 = param_2 & 0x7ffffffffffffff8;
      lVar66 = uVar59 - uVar62;
      puVar71 = (undefined8 *)((long)plVar58 + uVar59);
      do {
        uVar73 = *puVar71;
        bVar75 = (byte)((ulong)uVar73 >> 8);
        bVar76 = (byte)((ulong)uVar73 >> 0x10);
        bVar77 = (byte)((ulong)uVar73 >> 0x18);
        bVar78 = (byte)((ulong)uVar73 >> 0x20);
        bVar79 = (byte)((ulong)uVar73 >> 0x28);
        bVar80 = (byte)((ulong)uVar73 >> 0x30);
        bVar81 = (byte)((ulong)uVar73 >> 0x38);
        uVar59 = CONCAT17(-((byte)(bVar81 + 0xbf) < 0x1a),
                          CONCAT16(-((byte)(bVar80 + 0xbf) < 0x1a),
                                   CONCAT15(-((byte)(bVar79 + 0xbf) < 0x1a),
                                            CONCAT14(-((byte)(bVar78 + 0xbf) < 0x1a),
                                                     CONCAT13(-((byte)(bVar77 + 0xbf) < 0x1a),
                                                              CONCAT12(-((byte)(bVar76 + 0xbf) < 0x1a),
                                                                       CONCAT11(-((byte)(bVar75 + 0xbf) < 0x1a),-((byte)((byte)uVar73 + 0xbf) < 0x1a)))))))) &
                 0x2020202020202020;
        *puVar71 = CONCAT17((byte)(uVar59 >> 0x38) | bVar81,
                            CONCAT16((byte)(uVar59 >> 0x30) | bVar80,
                                     CONCAT15((byte)(uVar59 >> 0x28) | bVar79,
                                              CONCAT14((byte)(uVar59 >> 0x20) | bVar78,
                                                       CONCAT13((byte)(uVar59 >> 0x18) | bVar77,
                                                                CONCAT12((byte)(uVar59 >> 0x10) | bVar76,
                                                                         CONCAT11((byte)(uVar59 >> 8) | bVar75,(byte)uVar59 | (byte)uVar73)))))));
        lVar66 = lVar66 + 8;
        puVar71 = puVar71 + 1;
      } while (lVar66 != 0);
      if (param_2 != uVar62) goto LAB_1007c1b20;
    }
    else {
      uVar62 = param_2 & 0x7fffffffffffffc0;
      pbVar65 = (byte *)(plVar58 + 4);
      uVar59 = uVar62;
      do {
        lVar67 = *(long *)(pbVar65 + 0x18);
        bVar82 = (byte)((ulong)lVar67 >> 8);
        bVar83 = (byte)((ulong)lVar67 >> 0x10);
        bVar84 = (byte)((ulong)lVar67 >> 0x18);
        bVar85 = (byte)((ulong)lVar67 >> 0x20);
        bVar86 = (byte)((ulong)lVar67 >> 0x28);
        bVar87 = (byte)((ulong)lVar67 >> 0x30);
        bVar88 = (byte)((ulong)lVar67 >> 0x38);
        lVar66 = *(long *)(pbVar65 + 0x10);
        bVar75 = (byte)((ulong)lVar66 >> 8);
        bVar76 = (byte)((ulong)lVar66 >> 0x10);
        bVar77 = (byte)((ulong)lVar66 >> 0x18);
        bVar78 = (byte)((ulong)lVar66 >> 0x20);
        bVar79 = (byte)((ulong)lVar66 >> 0x28);
        bVar80 = (byte)((ulong)lVar66 >> 0x30);
        bVar81 = (byte)((ulong)lVar66 >> 0x38);
        pbVar65[-0x18] = -((byte)(pbVar65[-0x18] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x18];
        pbVar65[-0x17] = -((byte)(pbVar65[-0x17] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x17];
        pbVar65[-0x16] = -((byte)(pbVar65[-0x16] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x16];
        pbVar65[-0x15] = -((byte)(pbVar65[-0x15] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x15];
        pbVar65[-0x14] = -((byte)(pbVar65[-0x14] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x14];
        pbVar65[-0x13] = -((byte)(pbVar65[-0x13] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x13];
        pbVar65[-0x12] = -((byte)(pbVar65[-0x12] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x12];
        pbVar65[-0x11] = -((byte)(pbVar65[-0x11] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x11];
        pbVar65[-0x20] = -((byte)(pbVar65[-0x20] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x20];
        pbVar65[-0x1f] = -((byte)(pbVar65[-0x1f] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1f];
        pbVar65[-0x1e] = -((byte)(pbVar65[-0x1e] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1e];
        pbVar65[-0x1d] = -((byte)(pbVar65[-0x1d] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1d];
        pbVar65[-0x1c] = -((byte)(pbVar65[-0x1c] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1c];
        pbVar65[-0x1b] = -((byte)(pbVar65[-0x1b] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1b];
        pbVar65[-0x1a] = -((byte)(pbVar65[-0x1a] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1a];
        pbVar65[-0x19] = -((byte)(pbVar65[-0x19] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x19];
        pbVar65[-8] = -((byte)(pbVar65[-8] + 0xbf) < 0x1a) & 0x20U | pbVar65[-8];
        pbVar65[-7] = -((byte)(pbVar65[-7] + 0xbf) < 0x1a) & 0x20U | pbVar65[-7];
        pbVar65[-6] = -((byte)(pbVar65[-6] + 0xbf) < 0x1a) & 0x20U | pbVar65[-6];
        pbVar65[-5] = -((byte)(pbVar65[-5] + 0xbf) < 0x1a) & 0x20U | pbVar65[-5];
        pbVar65[-4] = -((byte)(pbVar65[-4] + 0xbf) < 0x1a) & 0x20U | pbVar65[-4];
        pbVar65[-3] = -((byte)(pbVar65[-3] + 0xbf) < 0x1a) & 0x20U | pbVar65[-3];
        pbVar65[-2] = -((byte)(pbVar65[-2] + 0xbf) < 0x1a) & 0x20U | pbVar65[-2];
        pbVar65[-1] = -((byte)(pbVar65[-1] + 0xbf) < 0x1a) & 0x20U | pbVar65[-1];
        pbVar65[-0x10] = -((byte)(pbVar65[-0x10] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x10];
        pbVar65[-0xf] = -((byte)(pbVar65[-0xf] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xf];
        pbVar65[-0xe] = -((byte)(pbVar65[-0xe] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xe];
        pbVar65[-0xd] = -((byte)(pbVar65[-0xd] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xd];
        pbVar65[-0xc] = -((byte)(pbVar65[-0xc] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xc];
        pbVar65[-0xb] = -((byte)(pbVar65[-0xb] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xb];
        pbVar65[-10] = -((byte)(pbVar65[-10] + 0xbf) < 0x1a) & 0x20U | pbVar65[-10];
        pbVar65[-9] = -((byte)(pbVar65[-9] + 0xbf) < 0x1a) & 0x20U | pbVar65[-9];
        pbVar65[8] = -((byte)(pbVar65[8] + 0xbf) < 0x1a) & 0x20U | pbVar65[8];
        pbVar65[9] = -((byte)(pbVar65[9] + 0xbf) < 0x1a) & 0x20U | pbVar65[9];
        pbVar65[10] = -((byte)(pbVar65[10] + 0xbf) < 0x1a) & 0x20U | pbVar65[10];
        pbVar65[0xb] = -((byte)(pbVar65[0xb] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xb];
        pbVar65[0xc] = -((byte)(pbVar65[0xc] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xc];
        pbVar65[0xd] = -((byte)(pbVar65[0xd] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xd];
        pbVar65[0xe] = -((byte)(pbVar65[0xe] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xe];
        pbVar65[0xf] = -((byte)(pbVar65[0xf] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xf];
        *pbVar65 = -((byte)(*pbVar65 + 0xbf) < 0x1a) & 0x20U | *pbVar65;
        pbVar65[1] = -((byte)(pbVar65[1] + 0xbf) < 0x1a) & 0x20U | pbVar65[1];
        pbVar65[2] = -((byte)(pbVar65[2] + 0xbf) < 0x1a) & 0x20U | pbVar65[2];
        pbVar65[3] = -((byte)(pbVar65[3] + 0xbf) < 0x1a) & 0x20U | pbVar65[3];
        pbVar65[4] = -((byte)(pbVar65[4] + 0xbf) < 0x1a) & 0x20U | pbVar65[4];
        pbVar65[5] = -((byte)(pbVar65[5] + 0xbf) < 0x1a) & 0x20U | pbVar65[5];
        pbVar65[6] = -((byte)(pbVar65[6] + 0xbf) < 0x1a) & 0x20U | pbVar65[6];
        pbVar65[7] = -((byte)(pbVar65[7] + 0xbf) < 0x1a) & 0x20U | pbVar65[7];
        pbVar65[0x18] = -((byte)((byte)lVar67 + 0xbf) < 0x1a) & 0x20U | (byte)lVar67;
        pbVar65[0x19] = -((byte)(bVar82 + 0xbf) < 0x1a) & 0x20U | bVar82;
        pbVar65[0x1a] = -((byte)(bVar83 + 0xbf) < 0x1a) & 0x20U | bVar83;
        pbVar65[0x1b] = -((byte)(bVar84 + 0xbf) < 0x1a) & 0x20U | bVar84;
        pbVar65[0x1c] = -((byte)(bVar85 + 0xbf) < 0x1a) & 0x20U | bVar85;
        pbVar65[0x1d] = -((byte)(bVar86 + 0xbf) < 0x1a) & 0x20U | bVar86;
        pbVar65[0x1e] = -((byte)(bVar87 + 0xbf) < 0x1a) & 0x20U | bVar87;
        pbVar65[0x1f] = -((byte)(bVar88 + 0xbf) < 0x1a) & 0x20U | bVar88;
        pbVar65[0x10] = -((byte)((byte)lVar66 + 0xbf) < 0x1a) & 0x20U | (byte)lVar66;
        pbVar65[0x11] = -((byte)(bVar75 + 0xbf) < 0x1a) & 0x20U | bVar75;
        pbVar65[0x12] = -((byte)(bVar76 + 0xbf) < 0x1a) & 0x20U | bVar76;
        pbVar65[0x13] = -((byte)(bVar77 + 0xbf) < 0x1a) & 0x20U | bVar77;
        pbVar65[0x14] = -((byte)(bVar78 + 0xbf) < 0x1a) & 0x20U | bVar78;
        pbVar65[0x15] = -((byte)(bVar79 + 0xbf) < 0x1a) & 0x20U | bVar79;
        pbVar65[0x16] = -((byte)(bVar80 + 0xbf) < 0x1a) & 0x20U | bVar80;
        pbVar65[0x17] = -((byte)(bVar81 + 0xbf) < 0x1a) & 0x20U | bVar81;
        uVar59 = uVar59 - 0x40;
        pbVar65 = pbVar65 + 0x40;
      } while (uVar59 != 0);
      if (param_2 != uVar62) {
        uVar59 = uVar62;
        if ((param_2 & 0x38) == 0) goto LAB_1007c1b20;
        goto LAB_1007c1adc;
      }
    }
    if (param_2 < 0x17) {
      if ((param_2 == 0x16) && ((*plVar58 == 0x747067746168632f && plVar58[1] == 0x6e6f632f7070612e) && *(long *)((long)plVar58 + 0xe) == 0x2f73746e65746e6f))
      goto LAB_1007c1c34;
    }
    else {
      ::core::str::pattern::StrSearcher::new(&local_d0,plVar58,param_2,"/chatgpt.app/contents//node_modules/@openai/codex/",0x16);
      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
      if ((local_128 & 1) != 0) {
LAB_1007c1c34:
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar58,param_2,1);
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar57,param_2,1);
        return 0;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar58,param_2,1);
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar57,param_2,1);
    uVar59 = is_openai_ide_extension_writer_path(param_1,param_2);
    if ((uVar59 & 1) != 0) {
      return 1;
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    pvVar57 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_2,1);
    if (pvVar57 == (void *)0x0) goto LAB_1007c2e2c;
    if (param_2 < 8) {
      uVar62 = 0;
LAB_1007c1d54:
      lVar66 = param_2 - uVar62;
      pcVar64 = (char *)(param_1 + uVar62);
      pcVar70 = (char *)((long)pvVar57 + uVar62);
      do {
        cVar1 = '/';
        if (*pcVar64 != '\\') {
          cVar1 = *pcVar64;
        }
        *pcVar70 = cVar1;
        lVar66 = lVar66 + -1;
        pcVar64 = pcVar64 + 1;
        pcVar70 = pcVar70 + 1;
      } while (lVar66 != 0);
    }
    else if (param_2 < 0x40) {
      uVar59 = 0;
LAB_1007c1d18:
      uVar62 = param_2 & 0x7ffffffffffffff8;
      lVar66 = uVar59 - uVar62;
      puVar69 = (ulong *)((long)pvVar57 + uVar59);
      puVar72 = (ulong *)(param_1 + uVar59);
      do {
        uVar59 = *puVar72;
        *puVar69 = uVar59 ^ (uVar59 ^ 0x2f2f2f2f2f2f2f2f) &
                            CONCAT17(-((char)(uVar59 >> 0x38) == '\\'),
                                     CONCAT16(-((char)(uVar59 >> 0x30) == '\\'),
                                              CONCAT15(-((char)(uVar59 >> 0x28) == '\\'),
                                                       CONCAT14(-((char)(uVar59 >> 0x20) == '\\'),
                                                                CONCAT13(-((char)(uVar59 >> 0x18) == '\\'),
                                                                         CONCAT12(-((char)(uVar59 >> 0x10) == '\\'),
                                                                                  CONCAT11(-((char)(uVar59 >> 8) == '\\'),-((char)uVar59 == '\\'))))))));
        lVar66 = lVar66 + 8;
        puVar69 = puVar69 + 1;
        puVar72 = puVar72 + 1;
      } while (lVar66 != 0);
      if (param_2 != uVar62) goto LAB_1007c1d54;
    }
    else {
      uVar62 = param_2 & 0x7fffffffffffffc0;
      pbVar65 = (byte *)(param_1 + 0x20);
      pbVar68 = (byte *)((long)pvVar57 + 0x20);
      uVar59 = uVar62;
      do {
        bVar75 = pbVar65[-0x20];
        bVar76 = pbVar65[-0x1f];
        bVar77 = pbVar65[-0x1e];
        bVar78 = pbVar65[-0x1d];
        bVar79 = pbVar65[-0x1c];
        bVar80 = pbVar65[-0x1b];
        bVar81 = pbVar65[-0x1a];
        bVar82 = pbVar65[-0x19];
        bVar83 = pbVar65[-0x18];
        bVar84 = pbVar65[-0x17];
        bVar85 = pbVar65[-0x16];
        bVar86 = pbVar65[-0x15];
        bVar87 = pbVar65[-0x14];
        bVar88 = pbVar65[-0x13];
        bVar4 = pbVar65[-0x12];
        bVar5 = pbVar65[-0x11];
        bVar6 = pbVar65[-0x10];
        bVar7 = pbVar65[-0xf];
        bVar8 = pbVar65[-0xe];
        bVar9 = pbVar65[-0xd];
        bVar10 = pbVar65[-0xc];
        bVar11 = pbVar65[-0xb];
        bVar12 = pbVar65[-10];
        bVar13 = pbVar65[-9];
        bVar14 = pbVar65[-8];
        bVar15 = pbVar65[-7];
        bVar16 = pbVar65[-6];
        bVar17 = pbVar65[-5];
        bVar18 = pbVar65[-4];
        bVar19 = pbVar65[-3];
        bVar20 = pbVar65[-2];
        bVar21 = pbVar65[-1];
        bVar22 = *pbVar65;
        bVar23 = pbVar65[1];
        bVar24 = pbVar65[2];
        bVar25 = pbVar65[3];
        bVar26 = pbVar65[4];
        bVar27 = pbVar65[5];
        bVar28 = pbVar65[6];
        bVar29 = pbVar65[7];
        bVar30 = pbVar65[8];
        bVar31 = pbVar65[9];
        bVar32 = pbVar65[10];
        bVar33 = pbVar65[0xb];
        bVar34 = pbVar65[0xc];
        bVar35 = pbVar65[0xd];
        bVar36 = pbVar65[0xe];
        bVar37 = pbVar65[0xf];
        bVar38 = pbVar65[0x10];
        bVar39 = pbVar65[0x11];
        bVar40 = pbVar65[0x12];
        bVar41 = pbVar65[0x13];
        bVar42 = pbVar65[0x14];
        bVar43 = pbVar65[0x15];
        bVar44 = pbVar65[0x16];
        bVar45 = pbVar65[0x17];
        bVar46 = pbVar65[0x18];
        bVar47 = pbVar65[0x19];
        bVar48 = pbVar65[0x1a];
        bVar49 = pbVar65[0x1b];
        bVar50 = pbVar65[0x1c];
        bVar51 = pbVar65[0x1d];
        bVar52 = pbVar65[0x1e];
        bVar53 = pbVar65[0x1f];
        pbVar68[-0x18] = bVar83 ^ (bVar83 ^ 0x2f) & -(bVar83 == 0x5c);
        pbVar68[-0x17] = bVar84 ^ (bVar84 ^ 0x2f) & -(bVar84 == 0x5c);
        pbVar68[-0x16] = bVar85 ^ (bVar85 ^ 0x2f) & -(bVar85 == 0x5c);
        pbVar68[-0x15] = bVar86 ^ (bVar86 ^ 0x2f) & -(bVar86 == 0x5c);
        pbVar68[-0x14] = bVar87 ^ (bVar87 ^ 0x2f) & -(bVar87 == 0x5c);
        pbVar68[-0x13] = bVar88 ^ (bVar88 ^ 0x2f) & -(bVar88 == 0x5c);
        pbVar68[-0x12] = bVar4 ^ (bVar4 ^ 0x2f) & -(bVar4 == 0x5c);
        pbVar68[-0x11] = bVar5 ^ (bVar5 ^ 0x2f) & -(bVar5 == 0x5c);
        pbVar68[-0x20] = bVar75 ^ (bVar75 ^ 0x2f) & -(bVar75 == 0x5c);
        pbVar68[-0x1f] = bVar76 ^ (bVar76 ^ 0x2f) & -(bVar76 == 0x5c);
        pbVar68[-0x1e] = bVar77 ^ (bVar77 ^ 0x2f) & -(bVar77 == 0x5c);
        pbVar68[-0x1d] = bVar78 ^ (bVar78 ^ 0x2f) & -(bVar78 == 0x5c);
        pbVar68[-0x1c] = bVar79 ^ (bVar79 ^ 0x2f) & -(bVar79 == 0x5c);
        pbVar68[-0x1b] = bVar80 ^ (bVar80 ^ 0x2f) & -(bVar80 == 0x5c);
        pbVar68[-0x1a] = bVar81 ^ (bVar81 ^ 0x2f) & -(bVar81 == 0x5c);
        pbVar68[-0x19] = bVar82 ^ (bVar82 ^ 0x2f) & -(bVar82 == 0x5c);
        pbVar68[-8] = bVar14 ^ (bVar14 ^ 0x2f) & -(bVar14 == 0x5c);
        pbVar68[-7] = bVar15 ^ (bVar15 ^ 0x2f) & -(bVar15 == 0x5c);
        pbVar68[-6] = bVar16 ^ (bVar16 ^ 0x2f) & -(bVar16 == 0x5c);
        pbVar68[-5] = bVar17 ^ (bVar17 ^ 0x2f) & -(bVar17 == 0x5c);
        pbVar68[-4] = bVar18 ^ (bVar18 ^ 0x2f) & -(bVar18 == 0x5c);
        pbVar68[-3] = bVar19 ^ (bVar19 ^ 0x2f) & -(bVar19 == 0x5c);
        pbVar68[-2] = bVar20 ^ (bVar20 ^ 0x2f) & -(bVar20 == 0x5c);
        pbVar68[-1] = bVar21 ^ (bVar21 ^ 0x2f) & -(bVar21 == 0x5c);
        pbVar68[-0x10] = bVar6 ^ (bVar6 ^ 0x2f) & -(bVar6 == 0x5c);
        pbVar68[-0xf] = bVar7 ^ (bVar7 ^ 0x2f) & -(bVar7 == 0x5c);
        pbVar68[-0xe] = bVar8 ^ (bVar8 ^ 0x2f) & -(bVar8 == 0x5c);
        pbVar68[-0xd] = bVar9 ^ (bVar9 ^ 0x2f) & -(bVar9 == 0x5c);
        pbVar68[-0xc] = bVar10 ^ (bVar10 ^ 0x2f) & -(bVar10 == 0x5c);
        pbVar68[-0xb] = bVar11 ^ (bVar11 ^ 0x2f) & -(bVar11 == 0x5c);
        pbVar68[-10] = bVar12 ^ (bVar12 ^ 0x2f) & -(bVar12 == 0x5c);
        pbVar68[-9] = bVar13 ^ (bVar13 ^ 0x2f) & -(bVar13 == 0x5c);
        pbVar68[8] = bVar30 ^ (bVar30 ^ 0x2f) & -(bVar30 == 0x5c);
        pbVar68[9] = bVar31 ^ (bVar31 ^ 0x2f) & -(bVar31 == 0x5c);
        pbVar68[10] = bVar32 ^ (bVar32 ^ 0x2f) & -(bVar32 == 0x5c);
        pbVar68[0xb] = bVar33 ^ (bVar33 ^ 0x2f) & -(bVar33 == 0x5c);
        pbVar68[0xc] = bVar34 ^ (bVar34 ^ 0x2f) & -(bVar34 == 0x5c);
        pbVar68[0xd] = bVar35 ^ (bVar35 ^ 0x2f) & -(bVar35 == 0x5c);
        pbVar68[0xe] = bVar36 ^ (bVar36 ^ 0x2f) & -(bVar36 == 0x5c);
        pbVar68[0xf] = bVar37 ^ (bVar37 ^ 0x2f) & -(bVar37 == 0x5c);
        *pbVar68 = bVar22 ^ (bVar22 ^ 0x2f) & -(bVar22 == 0x5c);
        pbVar68[1] = bVar23 ^ (bVar23 ^ 0x2f) & -(bVar23 == 0x5c);
        pbVar68[2] = bVar24 ^ (bVar24 ^ 0x2f) & -(bVar24 == 0x5c);
        pbVar68[3] = bVar25 ^ (bVar25 ^ 0x2f) & -(bVar25 == 0x5c);
        pbVar68[4] = bVar26 ^ (bVar26 ^ 0x2f) & -(bVar26 == 0x5c);
        pbVar68[5] = bVar27 ^ (bVar27 ^ 0x2f) & -(bVar27 == 0x5c);
        pbVar68[6] = bVar28 ^ (bVar28 ^ 0x2f) & -(bVar28 == 0x5c);
        pbVar68[7] = bVar29 ^ (bVar29 ^ 0x2f) & -(bVar29 == 0x5c);
        pbVar68[0x18] = bVar46 ^ (bVar46 ^ 0x2f) & -(bVar46 == 0x5c);
        pbVar68[0x19] = bVar47 ^ (bVar47 ^ 0x2f) & -(bVar47 == 0x5c);
        pbVar68[0x1a] = bVar48 ^ (bVar48 ^ 0x2f) & -(bVar48 == 0x5c);
        pbVar68[0x1b] = bVar49 ^ (bVar49 ^ 0x2f) & -(bVar49 == 0x5c);
        pbVar68[0x1c] = bVar50 ^ (bVar50 ^ 0x2f) & -(bVar50 == 0x5c);
        pbVar68[0x1d] = bVar51 ^ (bVar51 ^ 0x2f) & -(bVar51 == 0x5c);
        pbVar68[0x1e] = bVar52 ^ (bVar52 ^ 0x2f) & -(bVar52 == 0x5c);
        pbVar68[0x1f] = bVar53 ^ (bVar53 ^ 0x2f) & -(bVar53 == 0x5c);
        pbVar68[0x10] = bVar38 ^ (bVar38 ^ 0x2f) & -(bVar38 == 0x5c);
        pbVar68[0x11] = bVar39 ^ (bVar39 ^ 0x2f) & -(bVar39 == 0x5c);
        pbVar68[0x12] = bVar40 ^ (bVar40 ^ 0x2f) & -(bVar40 == 0x5c);
        pbVar68[0x13] = bVar41 ^ (bVar41 ^ 0x2f) & -(bVar41 == 0x5c);
        pbVar68[0x14] = bVar42 ^ (bVar42 ^ 0x2f) & -(bVar42 == 0x5c);
        pbVar68[0x15] = bVar43 ^ (bVar43 ^ 0x2f) & -(bVar43 == 0x5c);
        pbVar68[0x16] = bVar44 ^ (bVar44 ^ 0x2f) & -(bVar44 == 0x5c);
        pbVar68[0x17] = bVar45 ^ (bVar45 ^ 0x2f) & -(bVar45 == 0x5c);
        uVar59 = uVar59 - 0x40;
        pbVar65 = pbVar65 + 0x40;
        pbVar68 = pbVar68 + 0x40;
      } while (uVar59 != 0);
      if (param_2 != uVar62) {
        uVar59 = uVar62;
        if ((param_2 & 0x38) == 0) goto LAB_1007c1d54;
        goto LAB_1007c1d18;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    plVar58 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_2,1);
    if (plVar58 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,param_2);
    }
    _memcpy(plVar58,pvVar57,param_2);
    if (param_2 < 8) {
      uVar62 = 0;
LAB_1007c1e84:
      lVar66 = param_2 - uVar62;
      pbVar65 = (byte *)((long)plVar58 + uVar62);
      do {
        bVar75 = 0x20;
        if (0x19 < *pbVar65 - 0x41) {
          bVar75 = 0;
        }
        *pbVar65 = bVar75 | *pbVar65;
        lVar66 = lVar66 + -1;
        pbVar65 = pbVar65 + 1;
      } while (lVar66 != 0);
    }
    else if (param_2 < 0x40) {
      uVar59 = 0;
LAB_1007c1e40:
      uVar62 = param_2 & 0x7ffffffffffffff8;
      lVar66 = uVar59 - uVar62;
      puVar71 = (undefined8 *)((long)plVar58 + uVar59);
      do {
        uVar73 = *puVar71;
        bVar75 = (byte)((ulong)uVar73 >> 8);
        bVar76 = (byte)((ulong)uVar73 >> 0x10);
        bVar77 = (byte)((ulong)uVar73 >> 0x18);
        bVar78 = (byte)((ulong)uVar73 >> 0x20);
        bVar79 = (byte)((ulong)uVar73 >> 0x28);
        bVar80 = (byte)((ulong)uVar73 >> 0x30);
        bVar81 = (byte)((ulong)uVar73 >> 0x38);
        uVar59 = CONCAT17(-((byte)(bVar81 + 0xbf) < 0x1a),
                          CONCAT16(-((byte)(bVar80 + 0xbf) < 0x1a),
                                   CONCAT15(-((byte)(bVar79 + 0xbf) < 0x1a),
                                            CONCAT14(-((byte)(bVar78 + 0xbf) < 0x1a),
                                                     CONCAT13(-((byte)(bVar77 + 0xbf) < 0x1a),
                                                              CONCAT12(-((byte)(bVar76 + 0xbf) < 0x1a),
                                                                       CONCAT11(-((byte)(bVar75 + 0xbf) < 0x1a),-((byte)((byte)uVar73 + 0xbf) < 0x1a)))))))) &
                 0x2020202020202020;
        *puVar71 = CONCAT17((byte)(uVar59 >> 0x38) | bVar81,
                            CONCAT16((byte)(uVar59 >> 0x30) | bVar80,
                                     CONCAT15((byte)(uVar59 >> 0x28) | bVar79,
                                              CONCAT14((byte)(uVar59 >> 0x20) | bVar78,
                                                       CONCAT13((byte)(uVar59 >> 0x18) | bVar77,
                                                                CONCAT12((byte)(uVar59 >> 0x10) | bVar76,
                                                                         CONCAT11((byte)(uVar59 >> 8) | bVar75,(byte)uVar59 | (byte)uVar73)))))));
        lVar66 = lVar66 + 8;
        puVar71 = puVar71 + 1;
      } while (lVar66 != 0);
      if (param_2 != uVar62) goto LAB_1007c1e84;
    }
    else {
      uVar62 = param_2 & 0x7fffffffffffffc0;
      pbVar65 = (byte *)(plVar58 + 4);
      uVar59 = uVar62;
      do {
        lVar67 = *(long *)(pbVar65 + 0x18);
        bVar82 = (byte)((ulong)lVar67 >> 8);
        bVar83 = (byte)((ulong)lVar67 >> 0x10);
        bVar84 = (byte)((ulong)lVar67 >> 0x18);
        bVar85 = (byte)((ulong)lVar67 >> 0x20);
        bVar86 = (byte)((ulong)lVar67 >> 0x28);
        bVar87 = (byte)((ulong)lVar67 >> 0x30);
        bVar88 = (byte)((ulong)lVar67 >> 0x38);
        lVar66 = *(long *)(pbVar65 + 0x10);
        bVar75 = (byte)((ulong)lVar66 >> 8);
        bVar76 = (byte)((ulong)lVar66 >> 0x10);
        bVar77 = (byte)((ulong)lVar66 >> 0x18);
        bVar78 = (byte)((ulong)lVar66 >> 0x20);
        bVar79 = (byte)((ulong)lVar66 >> 0x28);
        bVar80 = (byte)((ulong)lVar66 >> 0x30);
        bVar81 = (byte)((ulong)lVar66 >> 0x38);
        pbVar65[-0x18] = -((byte)(pbVar65[-0x18] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x18];
        pbVar65[-0x17] = -((byte)(pbVar65[-0x17] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x17];
        pbVar65[-0x16] = -((byte)(pbVar65[-0x16] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x16];
        pbVar65[-0x15] = -((byte)(pbVar65[-0x15] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x15];
        pbVar65[-0x14] = -((byte)(pbVar65[-0x14] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x14];
        pbVar65[-0x13] = -((byte)(pbVar65[-0x13] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x13];
        pbVar65[-0x12] = -((byte)(pbVar65[-0x12] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x12];
        pbVar65[-0x11] = -((byte)(pbVar65[-0x11] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x11];
        pbVar65[-0x20] = -((byte)(pbVar65[-0x20] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x20];
        pbVar65[-0x1f] = -((byte)(pbVar65[-0x1f] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1f];
        pbVar65[-0x1e] = -((byte)(pbVar65[-0x1e] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1e];
        pbVar65[-0x1d] = -((byte)(pbVar65[-0x1d] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1d];
        pbVar65[-0x1c] = -((byte)(pbVar65[-0x1c] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1c];
        pbVar65[-0x1b] = -((byte)(pbVar65[-0x1b] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1b];
        pbVar65[-0x1a] = -((byte)(pbVar65[-0x1a] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1a];
        pbVar65[-0x19] = -((byte)(pbVar65[-0x19] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x19];
        pbVar65[-8] = -((byte)(pbVar65[-8] + 0xbf) < 0x1a) & 0x20U | pbVar65[-8];
        pbVar65[-7] = -((byte)(pbVar65[-7] + 0xbf) < 0x1a) & 0x20U | pbVar65[-7];
        pbVar65[-6] = -((byte)(pbVar65[-6] + 0xbf) < 0x1a) & 0x20U | pbVar65[-6];
        pbVar65[-5] = -((byte)(pbVar65[-5] + 0xbf) < 0x1a) & 0x20U | pbVar65[-5];
        pbVar65[-4] = -((byte)(pbVar65[-4] + 0xbf) < 0x1a) & 0x20U | pbVar65[-4];
        pbVar65[-3] = -((byte)(pbVar65[-3] + 0xbf) < 0x1a) & 0x20U | pbVar65[-3];
        pbVar65[-2] = -((byte)(pbVar65[-2] + 0xbf) < 0x1a) & 0x20U | pbVar65[-2];
        pbVar65[-1] = -((byte)(pbVar65[-1] + 0xbf) < 0x1a) & 0x20U | pbVar65[-1];
        pbVar65[-0x10] = -((byte)(pbVar65[-0x10] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x10];
        pbVar65[-0xf] = -((byte)(pbVar65[-0xf] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xf];
        pbVar65[-0xe] = -((byte)(pbVar65[-0xe] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xe];
        pbVar65[-0xd] = -((byte)(pbVar65[-0xd] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xd];
        pbVar65[-0xc] = -((byte)(pbVar65[-0xc] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xc];
        pbVar65[-0xb] = -((byte)(pbVar65[-0xb] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xb];
        pbVar65[-10] = -((byte)(pbVar65[-10] + 0xbf) < 0x1a) & 0x20U | pbVar65[-10];
        pbVar65[-9] = -((byte)(pbVar65[-9] + 0xbf) < 0x1a) & 0x20U | pbVar65[-9];
        pbVar65[8] = -((byte)(pbVar65[8] + 0xbf) < 0x1a) & 0x20U | pbVar65[8];
        pbVar65[9] = -((byte)(pbVar65[9] + 0xbf) < 0x1a) & 0x20U | pbVar65[9];
        pbVar65[10] = -((byte)(pbVar65[10] + 0xbf) < 0x1a) & 0x20U | pbVar65[10];
        pbVar65[0xb] = -((byte)(pbVar65[0xb] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xb];
        pbVar65[0xc] = -((byte)(pbVar65[0xc] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xc];
        pbVar65[0xd] = -((byte)(pbVar65[0xd] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xd];
        pbVar65[0xe] = -((byte)(pbVar65[0xe] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xe];
        pbVar65[0xf] = -((byte)(pbVar65[0xf] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xf];
        *pbVar65 = -((byte)(*pbVar65 + 0xbf) < 0x1a) & 0x20U | *pbVar65;
        pbVar65[1] = -((byte)(pbVar65[1] + 0xbf) < 0x1a) & 0x20U | pbVar65[1];
        pbVar65[2] = -((byte)(pbVar65[2] + 0xbf) < 0x1a) & 0x20U | pbVar65[2];
        pbVar65[3] = -((byte)(pbVar65[3] + 0xbf) < 0x1a) & 0x20U | pbVar65[3];
        pbVar65[4] = -((byte)(pbVar65[4] + 0xbf) < 0x1a) & 0x20U | pbVar65[4];
        pbVar65[5] = -((byte)(pbVar65[5] + 0xbf) < 0x1a) & 0x20U | pbVar65[5];
        pbVar65[6] = -((byte)(pbVar65[6] + 0xbf) < 0x1a) & 0x20U | pbVar65[6];
        pbVar65[7] = -((byte)(pbVar65[7] + 0xbf) < 0x1a) & 0x20U | pbVar65[7];
        pbVar65[0x18] = -((byte)((byte)lVar67 + 0xbf) < 0x1a) & 0x20U | (byte)lVar67;
        pbVar65[0x19] = -((byte)(bVar82 + 0xbf) < 0x1a) & 0x20U | bVar82;
        pbVar65[0x1a] = -((byte)(bVar83 + 0xbf) < 0x1a) & 0x20U | bVar83;
        pbVar65[0x1b] = -((byte)(bVar84 + 0xbf) < 0x1a) & 0x20U | bVar84;
        pbVar65[0x1c] = -((byte)(bVar85 + 0xbf) < 0x1a) & 0x20U | bVar85;
        pbVar65[0x1d] = -((byte)(bVar86 + 0xbf) < 0x1a) & 0x20U | bVar86;
        pbVar65[0x1e] = -((byte)(bVar87 + 0xbf) < 0x1a) & 0x20U | bVar87;
        pbVar65[0x1f] = -((byte)(bVar88 + 0xbf) < 0x1a) & 0x20U | bVar88;
        pbVar65[0x10] = -((byte)((byte)lVar66 + 0xbf) < 0x1a) & 0x20U | (byte)lVar66;
        pbVar65[0x11] = -((byte)(bVar75 + 0xbf) < 0x1a) & 0x20U | bVar75;
        pbVar65[0x12] = -((byte)(bVar76 + 0xbf) < 0x1a) & 0x20U | bVar76;
        pbVar65[0x13] = -((byte)(bVar77 + 0xbf) < 0x1a) & 0x20U | bVar77;
        pbVar65[0x14] = -((byte)(bVar78 + 0xbf) < 0x1a) & 0x20U | bVar78;
        pbVar65[0x15] = -((byte)(bVar79 + 0xbf) < 0x1a) & 0x20U | bVar79;
        pbVar65[0x16] = -((byte)(bVar80 + 0xbf) < 0x1a) & 0x20U | bVar80;
        pbVar65[0x17] = -((byte)(bVar81 + 0xbf) < 0x1a) & 0x20U | bVar81;
        uVar59 = uVar59 - 0x40;
        pbVar65 = pbVar65 + 0x40;
      } while (uVar59 != 0);
      if (param_2 != uVar62) {
        uVar59 = uVar62;
        if ((param_2 & 0x38) == 0) goto LAB_1007c1e84;
        goto LAB_1007c1e40;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar57,param_2,1);
    uVar59 = param_2;
    if (param_2 < 0x15) {
      if (param_2 == 0x14) {
        if (((*plVar58 == 0x65646f6373762e2f && plVar58[1] == 0x69736e657478652f) && (int)plVar58[2] == 0x2f736e6f) ||
           ((*plVar58 == 0x726f737275632e2f && plVar58[1] == 0x69736e657478652f) && (int)plVar58[2] == 0x2f736e6f)) goto LAB_1007c2d40;
        goto LAB_1007c1fac;
      }
      if (0xc < param_2) goto LAB_1007c1fac;
      if ((param_2 == 0xc) && (*plVar58 == 0x2e726f737275632f && (int)plVar58[1] == 0x2f707061)) goto LAB_1007c2d40;
LAB_1007c26b8:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar58,param_2,1);
    }
    else {
      ::core::str::pattern::StrSearcher::new
                (&local_d0,plVar58,param_2,
                 "/.vscode/extensions//.vscode-insiders/extensions//.vscode-oss/extensions//.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt"
                 ,0x14);
      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
      if ((local_128 & 1) != 0) goto LAB_1007c2d40;
      if (param_2 < 0x1e) {
        if (param_2 == 0x1d) {
          if (((*plVar58 == 0x65646f6373762e2f && plVar58[1] == 0x72656469736e692d) && plVar58[2] == 0x736e657478652f73) &&
              *(long *)((long)plVar58 + 0x15) == 0x2f736e6f69736e65) goto LAB_1007c2d40;
          goto LAB_1007c25f8;
        }
        if (0x18 < param_2) goto LAB_1007c25f8;
        if ((param_2 == 0x18) && ((*plVar58 == 0x65646f6373762e2f && plVar58[1] == 0x7478652f73736f2d) && plVar58[2] == 0x2f736e6f69736e65)) goto LAB_1007c2d40;
      }
      else {
        ::core::str::pattern::StrSearcher::new
                  (&local_d0,plVar58,param_2,
                   "/.vscode-insiders/extensions//.vscode-oss/extensions//.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt",0x1d);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
        if ((local_128 & 1) != 0) goto LAB_1007c2d40;
LAB_1007c25f8:
        ::core::str::pattern::StrSearcher::new
                  (&local_d0,plVar58,param_2,"/.vscode-oss/extensions//.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt",0x18);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
        if ((local_128 & 1) != 0) goto LAB_1007c2d40;
      }
      ::core::str::pattern::StrSearcher::new(&local_d0,plVar58,param_2,"/.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt",0x14);
      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
      if ((local_128 & 1) != 0) goto LAB_1007c2d40;
      if (param_2 < 0x17) {
        if ((param_2 == 0x16) && ((*plVar58 == 0x7573646e69772e2f && plVar58[1] == 0x6e657478652f6672) && *(long *)((long)plVar58 + 0xe) == 0x2f736e6f69736e65))
        goto LAB_1007c2d40;
      }
      else {
        ::core::str::pattern::StrSearcher::new(&local_d0,plVar58,param_2,"/.windsurf/extensions//extensions/openai.chatgpt",0x16);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
        if ((local_128 & 1) != 0) goto LAB_1007c2d40;
        if (param_2 < 0x1b) {
          if (param_2 == 0x1a) {
            if (((*plVar58 == 0x69736e657478652f && plVar58[1] == 0x6e65706f2f736e6f) && plVar58[2] == 0x67746168632e6961) && (short)plVar58[3] == 0x7470)
            goto LAB_1007c2d40;
          }
          else if (param_2 < 0x19) {
            if ((param_2 == 0x18) && ((*plVar58 == 0x206c61757369762f && plVar58[1] == 0x63206f6964757473) && plVar58[2] == 0x2f7070612e65646f))
            goto LAB_1007c2d40;
            goto LAB_1007c1fac;
          }
        }
        else {
          ::core::str::pattern::StrSearcher::new(&local_d0,plVar58,param_2,"/extensions/openai.chatgpt",0x1a);
          <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
          if ((local_128 & 1) != 0) goto LAB_1007c2d40;
        }
        ::core::str::pattern::StrSearcher::new(&local_d0,plVar58,param_2,&DAT_1013adb18,0x18);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
        if ((local_128 & 1) != 0) goto LAB_1007c2d40;
      }
LAB_1007c1fac:
      ::core::str::pattern::StrSearcher::new(&local_d0,plVar58,param_2,&DAT_1013adb30,0xc);
      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
      if ((local_128 & 1) != 0) goto LAB_1007c2d40;
      if (param_2 < 0xf) {
        if (param_2 != 0xe) goto LAB_1007c26b8;
        lVar66 = *plVar58;
        lVar67 = *(long *)((long)plVar58 + 6);
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar58,0xe,1);
        if (lVar66 == 0x727573646e69772f && lVar67 == 0x2f7070612e667275) {
          return 2;
        }
      }
      else {
        ::core::str::pattern::StrSearcher::new(&local_d0,plVar58,param_2,&DAT_1013adb3c,0xe);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
        uVar62 = local_128;
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar58,param_2,1);
        if ((uVar62 & 1) != 0) {
          return 2;
        }
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    pvVar57 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_2,1);
    if (pvVar57 == (void *)0x0) {
LAB_1007c2e2c:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,param_2);
    }
    if (param_2 < 8) {
      uVar63 = 0;
LAB_1007c2810:
      lVar66 = param_2 - uVar63;
      pcVar64 = (char *)(param_1 + uVar63);
      pcVar70 = (char *)((long)pvVar57 + uVar63);
      do {
        cVar1 = '/';
        if (*pcVar64 != '\\') {
          cVar1 = *pcVar64;
        }
        *pcVar70 = cVar1;
        lVar66 = lVar66 + -1;
        pcVar64 = pcVar64 + 1;
        pcVar70 = pcVar70 + 1;
      } while (lVar66 != 0);
    }
    else if (param_2 < 0x40) {
      uVar62 = 0;
LAB_1007c27d4:
      uVar63 = param_2 & 0x7ffffffffffffff8;
      lVar66 = uVar62 - uVar63;
      puVar69 = (ulong *)((long)pvVar57 + uVar62);
      puVar72 = (ulong *)(param_1 + uVar62);
      do {
        uVar62 = *puVar72;
        *puVar69 = uVar62 ^ (uVar62 ^ 0x2f2f2f2f2f2f2f2f) &
                            CONCAT17(-((char)(uVar62 >> 0x38) == '\\'),
                                     CONCAT16(-((char)(uVar62 >> 0x30) == '\\'),
                                              CONCAT15(-((char)(uVar62 >> 0x28) == '\\'),
                                                       CONCAT14(-((char)(uVar62 >> 0x20) == '\\'),
                                                                CONCAT13(-((char)(uVar62 >> 0x18) == '\\'),
                                                                         CONCAT12(-((char)(uVar62 >> 0x10) == '\\'),
                                                                                  CONCAT11(-((char)(uVar62 >> 8) == '\\'),-((char)uVar62 == '\\'))))))));
        lVar66 = lVar66 + 8;
        puVar69 = puVar69 + 1;
        puVar72 = puVar72 + 1;
      } while (lVar66 != 0);
      if (param_2 != uVar63) goto LAB_1007c2810;
    }
    else {
      uVar63 = param_2 & 0x7fffffffffffffc0;
      pbVar65 = (byte *)(param_1 + 0x20);
      pbVar68 = (byte *)((long)pvVar57 + 0x20);
      uVar62 = uVar63;
      do {
        bVar75 = pbVar65[-0x20];
        bVar76 = pbVar65[-0x1f];
        bVar77 = pbVar65[-0x1e];
        bVar78 = pbVar65[-0x1d];
        bVar79 = pbVar65[-0x1c];
        bVar80 = pbVar65[-0x1b];
        bVar81 = pbVar65[-0x1a];
        bVar82 = pbVar65[-0x19];
        bVar83 = pbVar65[-0x18];
        bVar84 = pbVar65[-0x17];
        bVar85 = pbVar65[-0x16];
        bVar86 = pbVar65[-0x15];
        bVar87 = pbVar65[-0x14];
        bVar88 = pbVar65[-0x13];
        bVar4 = pbVar65[-0x12];
        bVar5 = pbVar65[-0x11];
        bVar6 = pbVar65[-0x10];
        bVar7 = pbVar65[-0xf];
        bVar8 = pbVar65[-0xe];
        bVar9 = pbVar65[-0xd];
        bVar10 = pbVar65[-0xc];
        bVar11 = pbVar65[-0xb];
        bVar12 = pbVar65[-10];
        bVar13 = pbVar65[-9];
        bVar14 = pbVar65[-8];
        bVar15 = pbVar65[-7];
        bVar16 = pbVar65[-6];
        bVar17 = pbVar65[-5];
        bVar18 = pbVar65[-4];
        bVar19 = pbVar65[-3];
        bVar20 = pbVar65[-2];
        bVar21 = pbVar65[-1];
        bVar22 = *pbVar65;
        bVar23 = pbVar65[1];
        bVar24 = pbVar65[2];
        bVar25 = pbVar65[3];
        bVar26 = pbVar65[4];
        bVar27 = pbVar65[5];
        bVar28 = pbVar65[6];
        bVar29 = pbVar65[7];
        bVar30 = pbVar65[8];
        bVar31 = pbVar65[9];
        bVar32 = pbVar65[10];
        bVar33 = pbVar65[0xb];
        bVar34 = pbVar65[0xc];
        bVar35 = pbVar65[0xd];
        bVar36 = pbVar65[0xe];
        bVar37 = pbVar65[0xf];
        bVar38 = pbVar65[0x10];
        bVar39 = pbVar65[0x11];
        bVar40 = pbVar65[0x12];
        bVar41 = pbVar65[0x13];
        bVar42 = pbVar65[0x14];
        bVar43 = pbVar65[0x15];
        bVar44 = pbVar65[0x16];
        bVar45 = pbVar65[0x17];
        bVar46 = pbVar65[0x18];
        bVar47 = pbVar65[0x19];
        bVar48 = pbVar65[0x1a];
        bVar49 = pbVar65[0x1b];
        bVar50 = pbVar65[0x1c];
        bVar51 = pbVar65[0x1d];
        bVar52 = pbVar65[0x1e];
        bVar53 = pbVar65[0x1f];
        pbVar68[-0x18] = bVar83 ^ (bVar83 ^ 0x2f) & -(bVar83 == 0x5c);
        pbVar68[-0x17] = bVar84 ^ (bVar84 ^ 0x2f) & -(bVar84 == 0x5c);
        pbVar68[-0x16] = bVar85 ^ (bVar85 ^ 0x2f) & -(bVar85 == 0x5c);
        pbVar68[-0x15] = bVar86 ^ (bVar86 ^ 0x2f) & -(bVar86 == 0x5c);
        pbVar68[-0x14] = bVar87 ^ (bVar87 ^ 0x2f) & -(bVar87 == 0x5c);
        pbVar68[-0x13] = bVar88 ^ (bVar88 ^ 0x2f) & -(bVar88 == 0x5c);
        pbVar68[-0x12] = bVar4 ^ (bVar4 ^ 0x2f) & -(bVar4 == 0x5c);
        pbVar68[-0x11] = bVar5 ^ (bVar5 ^ 0x2f) & -(bVar5 == 0x5c);
        pbVar68[-0x20] = bVar75 ^ (bVar75 ^ 0x2f) & -(bVar75 == 0x5c);
        pbVar68[-0x1f] = bVar76 ^ (bVar76 ^ 0x2f) & -(bVar76 == 0x5c);
        pbVar68[-0x1e] = bVar77 ^ (bVar77 ^ 0x2f) & -(bVar77 == 0x5c);
        pbVar68[-0x1d] = bVar78 ^ (bVar78 ^ 0x2f) & -(bVar78 == 0x5c);
        pbVar68[-0x1c] = bVar79 ^ (bVar79 ^ 0x2f) & -(bVar79 == 0x5c);
        pbVar68[-0x1b] = bVar80 ^ (bVar80 ^ 0x2f) & -(bVar80 == 0x5c);
        pbVar68[-0x1a] = bVar81 ^ (bVar81 ^ 0x2f) & -(bVar81 == 0x5c);
        pbVar68[-0x19] = bVar82 ^ (bVar82 ^ 0x2f) & -(bVar82 == 0x5c);
        pbVar68[-8] = bVar14 ^ (bVar14 ^ 0x2f) & -(bVar14 == 0x5c);
        pbVar68[-7] = bVar15 ^ (bVar15 ^ 0x2f) & -(bVar15 == 0x5c);
        pbVar68[-6] = bVar16 ^ (bVar16 ^ 0x2f) & -(bVar16 == 0x5c);
        pbVar68[-5] = bVar17 ^ (bVar17 ^ 0x2f) & -(bVar17 == 0x5c);
        pbVar68[-4] = bVar18 ^ (bVar18 ^ 0x2f) & -(bVar18 == 0x5c);
        pbVar68[-3] = bVar19 ^ (bVar19 ^ 0x2f) & -(bVar19 == 0x5c);
        pbVar68[-2] = bVar20 ^ (bVar20 ^ 0x2f) & -(bVar20 == 0x5c);
        pbVar68[-1] = bVar21 ^ (bVar21 ^ 0x2f) & -(bVar21 == 0x5c);
        pbVar68[-0x10] = bVar6 ^ (bVar6 ^ 0x2f) & -(bVar6 == 0x5c);
        pbVar68[-0xf] = bVar7 ^ (bVar7 ^ 0x2f) & -(bVar7 == 0x5c);
        pbVar68[-0xe] = bVar8 ^ (bVar8 ^ 0x2f) & -(bVar8 == 0x5c);
        pbVar68[-0xd] = bVar9 ^ (bVar9 ^ 0x2f) & -(bVar9 == 0x5c);
        pbVar68[-0xc] = bVar10 ^ (bVar10 ^ 0x2f) & -(bVar10 == 0x5c);
        pbVar68[-0xb] = bVar11 ^ (bVar11 ^ 0x2f) & -(bVar11 == 0x5c);
        pbVar68[-10] = bVar12 ^ (bVar12 ^ 0x2f) & -(bVar12 == 0x5c);
        pbVar68[-9] = bVar13 ^ (bVar13 ^ 0x2f) & -(bVar13 == 0x5c);
        pbVar68[8] = bVar30 ^ (bVar30 ^ 0x2f) & -(bVar30 == 0x5c);
        pbVar68[9] = bVar31 ^ (bVar31 ^ 0x2f) & -(bVar31 == 0x5c);
        pbVar68[10] = bVar32 ^ (bVar32 ^ 0x2f) & -(bVar32 == 0x5c);
        pbVar68[0xb] = bVar33 ^ (bVar33 ^ 0x2f) & -(bVar33 == 0x5c);
        pbVar68[0xc] = bVar34 ^ (bVar34 ^ 0x2f) & -(bVar34 == 0x5c);
        pbVar68[0xd] = bVar35 ^ (bVar35 ^ 0x2f) & -(bVar35 == 0x5c);
        pbVar68[0xe] = bVar36 ^ (bVar36 ^ 0x2f) & -(bVar36 == 0x5c);
        pbVar68[0xf] = bVar37 ^ (bVar37 ^ 0x2f) & -(bVar37 == 0x5c);
        *pbVar68 = bVar22 ^ (bVar22 ^ 0x2f) & -(bVar22 == 0x5c);
        pbVar68[1] = bVar23 ^ (bVar23 ^ 0x2f) & -(bVar23 == 0x5c);
        pbVar68[2] = bVar24 ^ (bVar24 ^ 0x2f) & -(bVar24 == 0x5c);
        pbVar68[3] = bVar25 ^ (bVar25 ^ 0x2f) & -(bVar25 == 0x5c);
        pbVar68[4] = bVar26 ^ (bVar26 ^ 0x2f) & -(bVar26 == 0x5c);
        pbVar68[5] = bVar27 ^ (bVar27 ^ 0x2f) & -(bVar27 == 0x5c);
        pbVar68[6] = bVar28 ^ (bVar28 ^ 0x2f) & -(bVar28 == 0x5c);
        pbVar68[7] = bVar29 ^ (bVar29 ^ 0x2f) & -(bVar29 == 0x5c);
        pbVar68[0x18] = bVar46 ^ (bVar46 ^ 0x2f) & -(bVar46 == 0x5c);
        pbVar68[0x19] = bVar47 ^ (bVar47 ^ 0x2f) & -(bVar47 == 0x5c);
        pbVar68[0x1a] = bVar48 ^ (bVar48 ^ 0x2f) & -(bVar48 == 0x5c);
        pbVar68[0x1b] = bVar49 ^ (bVar49 ^ 0x2f) & -(bVar49 == 0x5c);
        pbVar68[0x1c] = bVar50 ^ (bVar50 ^ 0x2f) & -(bVar50 == 0x5c);
        pbVar68[0x1d] = bVar51 ^ (bVar51 ^ 0x2f) & -(bVar51 == 0x5c);
        pbVar68[0x1e] = bVar52 ^ (bVar52 ^ 0x2f) & -(bVar52 == 0x5c);
        pbVar68[0x1f] = bVar53 ^ (bVar53 ^ 0x2f) & -(bVar53 == 0x5c);
        pbVar68[0x10] = bVar38 ^ (bVar38 ^ 0x2f) & -(bVar38 == 0x5c);
        pbVar68[0x11] = bVar39 ^ (bVar39 ^ 0x2f) & -(bVar39 == 0x5c);
        pbVar68[0x12] = bVar40 ^ (bVar40 ^ 0x2f) & -(bVar40 == 0x5c);
        pbVar68[0x13] = bVar41 ^ (bVar41 ^ 0x2f) & -(bVar41 == 0x5c);
        pbVar68[0x14] = bVar42 ^ (bVar42 ^ 0x2f) & -(bVar42 == 0x5c);
        pbVar68[0x15] = bVar43 ^ (bVar43 ^ 0x2f) & -(bVar43 == 0x5c);
        pbVar68[0x16] = bVar44 ^ (bVar44 ^ 0x2f) & -(bVar44 == 0x5c);
        pbVar68[0x17] = bVar45 ^ (bVar45 ^ 0x2f) & -(bVar45 == 0x5c);
        uVar62 = uVar62 - 0x40;
        pbVar65 = pbVar65 + 0x40;
        pbVar68 = pbVar68 + 0x40;
      } while (uVar62 != 0);
      if (param_2 != uVar63) {
        uVar62 = uVar63;
        if ((param_2 & 0x38) == 0) goto LAB_1007c2810;
        goto LAB_1007c27d4;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    plVar58 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_2,1);
    if (plVar58 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,param_2);
    }
    _memcpy(plVar58,pvVar57,param_2);
    if (param_2 < 8) {
      uVar63 = 0;
LAB_1007c2940:
      lVar66 = param_2 - uVar63;
      pbVar65 = (byte *)((long)plVar58 + uVar63);
      do {
        bVar75 = 0x20;
        if (0x19 < *pbVar65 - 0x41) {
          bVar75 = 0;
        }
        *pbVar65 = bVar75 | *pbVar65;
        lVar66 = lVar66 + -1;
        pbVar65 = pbVar65 + 1;
      } while (lVar66 != 0);
    }
    else if (param_2 < 0x40) {
      uVar62 = 0;
LAB_1007c28fc:
      uVar63 = param_2 & 0x7ffffffffffffff8;
      lVar66 = uVar62 - uVar63;
      puVar71 = (undefined8 *)((long)plVar58 + uVar62);
      do {
        uVar73 = *puVar71;
        bVar75 = (byte)((ulong)uVar73 >> 8);
        bVar76 = (byte)((ulong)uVar73 >> 0x10);
        bVar77 = (byte)((ulong)uVar73 >> 0x18);
        bVar78 = (byte)((ulong)uVar73 >> 0x20);
        bVar79 = (byte)((ulong)uVar73 >> 0x28);
        bVar80 = (byte)((ulong)uVar73 >> 0x30);
        bVar81 = (byte)((ulong)uVar73 >> 0x38);
        uVar62 = CONCAT17(-((byte)(bVar81 + 0xbf) < 0x1a),
                          CONCAT16(-((byte)(bVar80 + 0xbf) < 0x1a),
                                   CONCAT15(-((byte)(bVar79 + 0xbf) < 0x1a),
                                            CONCAT14(-((byte)(bVar78 + 0xbf) < 0x1a),
                                                     CONCAT13(-((byte)(bVar77 + 0xbf) < 0x1a),
                                                              CONCAT12(-((byte)(bVar76 + 0xbf) < 0x1a),
                                                                       CONCAT11(-((byte)(bVar75 + 0xbf) < 0x1a),-((byte)((byte)uVar73 + 0xbf) < 0x1a)))))))) &
                 0x2020202020202020;
        *puVar71 = CONCAT17((byte)(uVar62 >> 0x38) | bVar81,
                            CONCAT16((byte)(uVar62 >> 0x30) | bVar80,
                                     CONCAT15((byte)(uVar62 >> 0x28) | bVar79,
                                              CONCAT14((byte)(uVar62 >> 0x20) | bVar78,
                                                       CONCAT13((byte)(uVar62 >> 0x18) | bVar77,
                                                                CONCAT12((byte)(uVar62 >> 0x10) | bVar76,
                                                                         CONCAT11((byte)(uVar62 >> 8) | bVar75,(byte)uVar62 | (byte)uVar73)))))));
        lVar66 = lVar66 + 8;
        puVar71 = puVar71 + 1;
      } while (lVar66 != 0);
      if (param_2 != uVar63) goto LAB_1007c2940;
    }
    else {
      uVar63 = param_2 & 0x7fffffffffffffc0;
      pbVar65 = (byte *)(plVar58 + 4);
      uVar62 = uVar63;
      do {
        lVar67 = *(long *)(pbVar65 + 0x18);
        bVar82 = (byte)((ulong)lVar67 >> 8);
        bVar83 = (byte)((ulong)lVar67 >> 0x10);
        bVar84 = (byte)((ulong)lVar67 >> 0x18);
        bVar85 = (byte)((ulong)lVar67 >> 0x20);
        bVar86 = (byte)((ulong)lVar67 >> 0x28);
        bVar87 = (byte)((ulong)lVar67 >> 0x30);
        bVar88 = (byte)((ulong)lVar67 >> 0x38);
        lVar66 = *(long *)(pbVar65 + 0x10);
        bVar75 = (byte)((ulong)lVar66 >> 8);
        bVar76 = (byte)((ulong)lVar66 >> 0x10);
        bVar77 = (byte)((ulong)lVar66 >> 0x18);
        bVar78 = (byte)((ulong)lVar66 >> 0x20);
        bVar79 = (byte)((ulong)lVar66 >> 0x28);
        bVar80 = (byte)((ulong)lVar66 >> 0x30);
        bVar81 = (byte)((ulong)lVar66 >> 0x38);
        pbVar65[-0x18] = -((byte)(pbVar65[-0x18] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x18];
        pbVar65[-0x17] = -((byte)(pbVar65[-0x17] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x17];
        pbVar65[-0x16] = -((byte)(pbVar65[-0x16] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x16];
        pbVar65[-0x15] = -((byte)(pbVar65[-0x15] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x15];
        pbVar65[-0x14] = -((byte)(pbVar65[-0x14] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x14];
        pbVar65[-0x13] = -((byte)(pbVar65[-0x13] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x13];
        pbVar65[-0x12] = -((byte)(pbVar65[-0x12] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x12];
        pbVar65[-0x11] = -((byte)(pbVar65[-0x11] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x11];
        pbVar65[-0x20] = -((byte)(pbVar65[-0x20] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x20];
        pbVar65[-0x1f] = -((byte)(pbVar65[-0x1f] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1f];
        pbVar65[-0x1e] = -((byte)(pbVar65[-0x1e] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1e];
        pbVar65[-0x1d] = -((byte)(pbVar65[-0x1d] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1d];
        pbVar65[-0x1c] = -((byte)(pbVar65[-0x1c] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1c];
        pbVar65[-0x1b] = -((byte)(pbVar65[-0x1b] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1b];
        pbVar65[-0x1a] = -((byte)(pbVar65[-0x1a] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1a];
        pbVar65[-0x19] = -((byte)(pbVar65[-0x19] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x19];
        pbVar65[-8] = -((byte)(pbVar65[-8] + 0xbf) < 0x1a) & 0x20U | pbVar65[-8];
        pbVar65[-7] = -((byte)(pbVar65[-7] + 0xbf) < 0x1a) & 0x20U | pbVar65[-7];
        pbVar65[-6] = -((byte)(pbVar65[-6] + 0xbf) < 0x1a) & 0x20U | pbVar65[-6];
        pbVar65[-5] = -((byte)(pbVar65[-5] + 0xbf) < 0x1a) & 0x20U | pbVar65[-5];
        pbVar65[-4] = -((byte)(pbVar65[-4] + 0xbf) < 0x1a) & 0x20U | pbVar65[-4];
        pbVar65[-3] = -((byte)(pbVar65[-3] + 0xbf) < 0x1a) & 0x20U | pbVar65[-3];
        pbVar65[-2] = -((byte)(pbVar65[-2] + 0xbf) < 0x1a) & 0x20U | pbVar65[-2];
        pbVar65[-1] = -((byte)(pbVar65[-1] + 0xbf) < 0x1a) & 0x20U | pbVar65[-1];
        pbVar65[-0x10] = -((byte)(pbVar65[-0x10] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x10];
        pbVar65[-0xf] = -((byte)(pbVar65[-0xf] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xf];
        pbVar65[-0xe] = -((byte)(pbVar65[-0xe] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xe];
        pbVar65[-0xd] = -((byte)(pbVar65[-0xd] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xd];
        pbVar65[-0xc] = -((byte)(pbVar65[-0xc] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xc];
        pbVar65[-0xb] = -((byte)(pbVar65[-0xb] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xb];
        pbVar65[-10] = -((byte)(pbVar65[-10] + 0xbf) < 0x1a) & 0x20U | pbVar65[-10];
        pbVar65[-9] = -((byte)(pbVar65[-9] + 0xbf) < 0x1a) & 0x20U | pbVar65[-9];
        pbVar65[8] = -((byte)(pbVar65[8] + 0xbf) < 0x1a) & 0x20U | pbVar65[8];
        pbVar65[9] = -((byte)(pbVar65[9] + 0xbf) < 0x1a) & 0x20U | pbVar65[9];
        pbVar65[10] = -((byte)(pbVar65[10] + 0xbf) < 0x1a) & 0x20U | pbVar65[10];
        pbVar65[0xb] = -((byte)(pbVar65[0xb] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xb];
        pbVar65[0xc] = -((byte)(pbVar65[0xc] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xc];
        pbVar65[0xd] = -((byte)(pbVar65[0xd] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xd];
        pbVar65[0xe] = -((byte)(pbVar65[0xe] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xe];
        pbVar65[0xf] = -((byte)(pbVar65[0xf] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xf];
        *pbVar65 = -((byte)(*pbVar65 + 0xbf) < 0x1a) & 0x20U | *pbVar65;
        pbVar65[1] = -((byte)(pbVar65[1] + 0xbf) < 0x1a) & 0x20U | pbVar65[1];
        pbVar65[2] = -((byte)(pbVar65[2] + 0xbf) < 0x1a) & 0x20U | pbVar65[2];
        pbVar65[3] = -((byte)(pbVar65[3] + 0xbf) < 0x1a) & 0x20U | pbVar65[3];
        pbVar65[4] = -((byte)(pbVar65[4] + 0xbf) < 0x1a) & 0x20U | pbVar65[4];
        pbVar65[5] = -((byte)(pbVar65[5] + 0xbf) < 0x1a) & 0x20U | pbVar65[5];
        pbVar65[6] = -((byte)(pbVar65[6] + 0xbf) < 0x1a) & 0x20U | pbVar65[6];
        pbVar65[7] = -((byte)(pbVar65[7] + 0xbf) < 0x1a) & 0x20U | pbVar65[7];
        pbVar65[0x18] = -((byte)((byte)lVar67 + 0xbf) < 0x1a) & 0x20U | (byte)lVar67;
        pbVar65[0x19] = -((byte)(bVar82 + 0xbf) < 0x1a) & 0x20U | bVar82;
        pbVar65[0x1a] = -((byte)(bVar83 + 0xbf) < 0x1a) & 0x20U | bVar83;
        pbVar65[0x1b] = -((byte)(bVar84 + 0xbf) < 0x1a) & 0x20U | bVar84;
        pbVar65[0x1c] = -((byte)(bVar85 + 0xbf) < 0x1a) & 0x20U | bVar85;
        pbVar65[0x1d] = -((byte)(bVar86 + 0xbf) < 0x1a) & 0x20U | bVar86;
        pbVar65[0x1e] = -((byte)(bVar87 + 0xbf) < 0x1a) & 0x20U | bVar87;
        pbVar65[0x1f] = -((byte)(bVar88 + 0xbf) < 0x1a) & 0x20U | bVar88;
        pbVar65[0x10] = -((byte)((byte)lVar66 + 0xbf) < 0x1a) & 0x20U | (byte)lVar66;
        pbVar65[0x11] = -((byte)(bVar75 + 0xbf) < 0x1a) & 0x20U | bVar75;
        pbVar65[0x12] = -((byte)(bVar76 + 0xbf) < 0x1a) & 0x20U | bVar76;
        pbVar65[0x13] = -((byte)(bVar77 + 0xbf) < 0x1a) & 0x20U | bVar77;
        pbVar65[0x14] = -((byte)(bVar78 + 0xbf) < 0x1a) & 0x20U | bVar78;
        pbVar65[0x15] = -((byte)(bVar79 + 0xbf) < 0x1a) & 0x20U | bVar79;
        pbVar65[0x16] = -((byte)(bVar80 + 0xbf) < 0x1a) & 0x20U | bVar80;
        pbVar65[0x17] = -((byte)(bVar81 + 0xbf) < 0x1a) & 0x20U | bVar81;
        uVar62 = uVar62 - 0x40;
        pbVar65 = pbVar65 + 0x40;
      } while (uVar62 != 0);
      if (param_2 != uVar63) {
        uVar62 = uVar63;
        if ((param_2 & 0x38) == 0) goto LAB_1007c2940;
        goto LAB_1007c28fc;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar57,param_2,1);
    if (param_2 < 0x1d) {
      if (param_2 == 0x1c) {
        uVar59 = 0x1c;
        if (((*plVar58 == 0x6f6d5f65646f6e2f && plVar58[1] == 0x6f402f73656c7564) && plVar58[2] == 0x6f632f69616e6570) && (int)plVar58[3] == 0x2f786564)
        goto LAB_1007c2ad4;
        goto LAB_1007c18f8;
      }
      bVar55 = false;
    }
    else {
      ::core::str::pattern::StrSearcher::new(&local_d0,plVar58,param_2,"/node_modules/@openai/codex/",0x1c);
      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
      if ((local_128 & 1) != 0) goto LAB_1007c2ad4;
      bVar55 = true;
    }
  }
  local_d0 = 0;
  local_a0 = 0;
  local_98 = 1;
  local_c8 = param_2;
  local_c0 = (long *)param_1;
  uStack_b8 = param_2;
  local_b0 = param_1;
  uStack_a8 = param_1 + param_2;
  auVar89 = ::core::iter::traits::iterator::Iterator::try_fold(&local_d0);
  uVar73 = 0;
  if (auVar89._0_8_ != 0) {
    auVar90 = auVar89;
  }
  uVar62 = auVar90._8_8_;
  lVar66 = auVar90._0_8_;
  if ((long)uVar62 < 0) {
LAB_1007c1934:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar73,uVar62);
  }
  if (uVar62 == 0) {
    plVar60 = (long *)0x1;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar73 = 1;
    pvVar57 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(uVar62,1);
    if (pvVar57 == (void *)0x0) goto LAB_1007c1934;
    uVar63 = 0;
    if ((uVar62 < 8) || ((ulong)((long)pvVar57 - lVar66) < 0x40)) {
LAB_1007c2170:
      lVar67 = uVar62 - uVar63;
      pcVar64 = (char *)(lVar66 + uVar63);
      pcVar70 = (char *)((long)pvVar57 + uVar63);
      do {
        cVar1 = '/';
        if (*pcVar64 != '\\') {
          cVar1 = *pcVar64;
        }
        *pcVar70 = cVar1;
        lVar67 = lVar67 + -1;
        pcVar64 = pcVar64 + 1;
        pcVar70 = pcVar70 + 1;
      } while (lVar67 != 0);
    }
    else if (uVar62 < 0x40) {
      uVar74 = 0;
LAB_1007c2134:
      uVar63 = uVar62 & 0x7ffffffffffffff8;
      lVar67 = uVar74 - uVar63;
      puVar69 = (ulong *)((long)pvVar57 + uVar74);
      puVar72 = (ulong *)(lVar66 + uVar74);
      do {
        uVar74 = *puVar72;
        *puVar69 = uVar74 ^ (uVar74 ^ 0x2f2f2f2f2f2f2f2f) &
                            CONCAT17(-((char)(uVar74 >> 0x38) == '\\'),
                                     CONCAT16(-((char)(uVar74 >> 0x30) == '\\'),
                                              CONCAT15(-((char)(uVar74 >> 0x28) == '\\'),
                                                       CONCAT14(-((char)(uVar74 >> 0x20) == '\\'),
                                                                CONCAT13(-((char)(uVar74 >> 0x18) == '\\'),
                                                                         CONCAT12(-((char)(uVar74 >> 0x10) == '\\'),
                                                                                  CONCAT11(-((char)(uVar74 >> 8) == '\\'),-((char)uVar74 == '\\'))))))));
        lVar67 = lVar67 + 8;
        puVar69 = puVar69 + 1;
        puVar72 = puVar72 + 1;
      } while (lVar67 != 0);
      if (uVar62 != uVar63) goto LAB_1007c2170;
    }
    else {
      uVar63 = uVar62 & 0x7fffffffffffffc0;
      pbVar65 = (byte *)(lVar66 + 0x20);
      pbVar68 = (byte *)((long)pvVar57 + 0x20);
      uVar74 = uVar63;
      do {
        bVar75 = pbVar65[-0x20];
        bVar76 = pbVar65[-0x1f];
        bVar77 = pbVar65[-0x1e];
        bVar78 = pbVar65[-0x1d];
        bVar79 = pbVar65[-0x1c];
        bVar80 = pbVar65[-0x1b];
        bVar81 = pbVar65[-0x1a];
        bVar82 = pbVar65[-0x19];
        bVar83 = pbVar65[-0x18];
        bVar84 = pbVar65[-0x17];
        bVar85 = pbVar65[-0x16];
        bVar86 = pbVar65[-0x15];
        bVar87 = pbVar65[-0x14];
        bVar88 = pbVar65[-0x13];
        bVar4 = pbVar65[-0x12];
        bVar5 = pbVar65[-0x11];
        bVar6 = pbVar65[-0x10];
        bVar7 = pbVar65[-0xf];
        bVar8 = pbVar65[-0xe];
        bVar9 = pbVar65[-0xd];
        bVar10 = pbVar65[-0xc];
        bVar11 = pbVar65[-0xb];
        bVar12 = pbVar65[-10];
        bVar13 = pbVar65[-9];
        bVar14 = pbVar65[-8];
        bVar15 = pbVar65[-7];
        bVar16 = pbVar65[-6];
        bVar17 = pbVar65[-5];
        bVar18 = pbVar65[-4];
        bVar19 = pbVar65[-3];
        bVar20 = pbVar65[-2];
        bVar21 = pbVar65[-1];
        bVar22 = *pbVar65;
        bVar23 = pbVar65[1];
        bVar24 = pbVar65[2];
        bVar25 = pbVar65[3];
        bVar26 = pbVar65[4];
        bVar27 = pbVar65[5];
        bVar28 = pbVar65[6];
        bVar29 = pbVar65[7];
        bVar30 = pbVar65[8];
        bVar31 = pbVar65[9];
        bVar32 = pbVar65[10];
        bVar33 = pbVar65[0xb];
        bVar34 = pbVar65[0xc];
        bVar35 = pbVar65[0xd];
        bVar36 = pbVar65[0xe];
        bVar37 = pbVar65[0xf];
        bVar38 = pbVar65[0x10];
        bVar39 = pbVar65[0x11];
        bVar40 = pbVar65[0x12];
        bVar41 = pbVar65[0x13];
        bVar42 = pbVar65[0x14];
        bVar43 = pbVar65[0x15];
        bVar44 = pbVar65[0x16];
        bVar45 = pbVar65[0x17];
        bVar46 = pbVar65[0x18];
        bVar47 = pbVar65[0x19];
        bVar48 = pbVar65[0x1a];
        bVar49 = pbVar65[0x1b];
        bVar50 = pbVar65[0x1c];
        bVar51 = pbVar65[0x1d];
        bVar52 = pbVar65[0x1e];
        bVar53 = pbVar65[0x1f];
        pbVar68[-0x18] = bVar83 ^ (bVar83 ^ 0x2f) & -(bVar83 == 0x5c);
        pbVar68[-0x17] = bVar84 ^ (bVar84 ^ 0x2f) & -(bVar84 == 0x5c);
        pbVar68[-0x16] = bVar85 ^ (bVar85 ^ 0x2f) & -(bVar85 == 0x5c);
        pbVar68[-0x15] = bVar86 ^ (bVar86 ^ 0x2f) & -(bVar86 == 0x5c);
        pbVar68[-0x14] = bVar87 ^ (bVar87 ^ 0x2f) & -(bVar87 == 0x5c);
        pbVar68[-0x13] = bVar88 ^ (bVar88 ^ 0x2f) & -(bVar88 == 0x5c);
        pbVar68[-0x12] = bVar4 ^ (bVar4 ^ 0x2f) & -(bVar4 == 0x5c);
        pbVar68[-0x11] = bVar5 ^ (bVar5 ^ 0x2f) & -(bVar5 == 0x5c);
        pbVar68[-0x20] = bVar75 ^ (bVar75 ^ 0x2f) & -(bVar75 == 0x5c);
        pbVar68[-0x1f] = bVar76 ^ (bVar76 ^ 0x2f) & -(bVar76 == 0x5c);
        pbVar68[-0x1e] = bVar77 ^ (bVar77 ^ 0x2f) & -(bVar77 == 0x5c);
        pbVar68[-0x1d] = bVar78 ^ (bVar78 ^ 0x2f) & -(bVar78 == 0x5c);
        pbVar68[-0x1c] = bVar79 ^ (bVar79 ^ 0x2f) & -(bVar79 == 0x5c);
        pbVar68[-0x1b] = bVar80 ^ (bVar80 ^ 0x2f) & -(bVar80 == 0x5c);
        pbVar68[-0x1a] = bVar81 ^ (bVar81 ^ 0x2f) & -(bVar81 == 0x5c);
        pbVar68[-0x19] = bVar82 ^ (bVar82 ^ 0x2f) & -(bVar82 == 0x5c);
        pbVar68[-8] = bVar14 ^ (bVar14 ^ 0x2f) & -(bVar14 == 0x5c);
        pbVar68[-7] = bVar15 ^ (bVar15 ^ 0x2f) & -(bVar15 == 0x5c);
        pbVar68[-6] = bVar16 ^ (bVar16 ^ 0x2f) & -(bVar16 == 0x5c);
        pbVar68[-5] = bVar17 ^ (bVar17 ^ 0x2f) & -(bVar17 == 0x5c);
        pbVar68[-4] = bVar18 ^ (bVar18 ^ 0x2f) & -(bVar18 == 0x5c);
        pbVar68[-3] = bVar19 ^ (bVar19 ^ 0x2f) & -(bVar19 == 0x5c);
        pbVar68[-2] = bVar20 ^ (bVar20 ^ 0x2f) & -(bVar20 == 0x5c);
        pbVar68[-1] = bVar21 ^ (bVar21 ^ 0x2f) & -(bVar21 == 0x5c);
        pbVar68[-0x10] = bVar6 ^ (bVar6 ^ 0x2f) & -(bVar6 == 0x5c);
        pbVar68[-0xf] = bVar7 ^ (bVar7 ^ 0x2f) & -(bVar7 == 0x5c);
        pbVar68[-0xe] = bVar8 ^ (bVar8 ^ 0x2f) & -(bVar8 == 0x5c);
        pbVar68[-0xd] = bVar9 ^ (bVar9 ^ 0x2f) & -(bVar9 == 0x5c);
        pbVar68[-0xc] = bVar10 ^ (bVar10 ^ 0x2f) & -(bVar10 == 0x5c);
        pbVar68[-0xb] = bVar11 ^ (bVar11 ^ 0x2f) & -(bVar11 == 0x5c);
        pbVar68[-10] = bVar12 ^ (bVar12 ^ 0x2f) & -(bVar12 == 0x5c);
        pbVar68[-9] = bVar13 ^ (bVar13 ^ 0x2f) & -(bVar13 == 0x5c);
        pbVar68[8] = bVar30 ^ (bVar30 ^ 0x2f) & -(bVar30 == 0x5c);
        pbVar68[9] = bVar31 ^ (bVar31 ^ 0x2f) & -(bVar31 == 0x5c);
        pbVar68[10] = bVar32 ^ (bVar32 ^ 0x2f) & -(bVar32 == 0x5c);
        pbVar68[0xb] = bVar33 ^ (bVar33 ^ 0x2f) & -(bVar33 == 0x5c);
        pbVar68[0xc] = bVar34 ^ (bVar34 ^ 0x2f) & -(bVar34 == 0x5c);
        pbVar68[0xd] = bVar35 ^ (bVar35 ^ 0x2f) & -(bVar35 == 0x5c);
        pbVar68[0xe] = bVar36 ^ (bVar36 ^ 0x2f) & -(bVar36 == 0x5c);
        pbVar68[0xf] = bVar37 ^ (bVar37 ^ 0x2f) & -(bVar37 == 0x5c);
        *pbVar68 = bVar22 ^ (bVar22 ^ 0x2f) & -(bVar22 == 0x5c);
        pbVar68[1] = bVar23 ^ (bVar23 ^ 0x2f) & -(bVar23 == 0x5c);
        pbVar68[2] = bVar24 ^ (bVar24 ^ 0x2f) & -(bVar24 == 0x5c);
        pbVar68[3] = bVar25 ^ (bVar25 ^ 0x2f) & -(bVar25 == 0x5c);
        pbVar68[4] = bVar26 ^ (bVar26 ^ 0x2f) & -(bVar26 == 0x5c);
        pbVar68[5] = bVar27 ^ (bVar27 ^ 0x2f) & -(bVar27 == 0x5c);
        pbVar68[6] = bVar28 ^ (bVar28 ^ 0x2f) & -(bVar28 == 0x5c);
        pbVar68[7] = bVar29 ^ (bVar29 ^ 0x2f) & -(bVar29 == 0x5c);
        pbVar68[0x18] = bVar46 ^ (bVar46 ^ 0x2f) & -(bVar46 == 0x5c);
        pbVar68[0x19] = bVar47 ^ (bVar47 ^ 0x2f) & -(bVar47 == 0x5c);
        pbVar68[0x1a] = bVar48 ^ (bVar48 ^ 0x2f) & -(bVar48 == 0x5c);
        pbVar68[0x1b] = bVar49 ^ (bVar49 ^ 0x2f) & -(bVar49 == 0x5c);
        pbVar68[0x1c] = bVar50 ^ (bVar50 ^ 0x2f) & -(bVar50 == 0x5c);
        pbVar68[0x1d] = bVar51 ^ (bVar51 ^ 0x2f) & -(bVar51 == 0x5c);
        pbVar68[0x1e] = bVar52 ^ (bVar52 ^ 0x2f) & -(bVar52 == 0x5c);
        pbVar68[0x1f] = bVar53 ^ (bVar53 ^ 0x2f) & -(bVar53 == 0x5c);
        pbVar68[0x10] = bVar38 ^ (bVar38 ^ 0x2f) & -(bVar38 == 0x5c);
        pbVar68[0x11] = bVar39 ^ (bVar39 ^ 0x2f) & -(bVar39 == 0x5c);
        pbVar68[0x12] = bVar40 ^ (bVar40 ^ 0x2f) & -(bVar40 == 0x5c);
        pbVar68[0x13] = bVar41 ^ (bVar41 ^ 0x2f) & -(bVar41 == 0x5c);
        pbVar68[0x14] = bVar42 ^ (bVar42 ^ 0x2f) & -(bVar42 == 0x5c);
        pbVar68[0x15] = bVar43 ^ (bVar43 ^ 0x2f) & -(bVar43 == 0x5c);
        pbVar68[0x16] = bVar44 ^ (bVar44 ^ 0x2f) & -(bVar44 == 0x5c);
        pbVar68[0x17] = bVar45 ^ (bVar45 ^ 0x2f) & -(bVar45 == 0x5c);
        uVar74 = uVar74 - 0x40;
        pbVar65 = pbVar65 + 0x40;
        pbVar68 = pbVar68 + 0x40;
      } while (uVar74 != 0);
      if (uVar62 != uVar63) {
        uVar74 = uVar63;
        if ((uVar62 & 0x38) == 0) goto LAB_1007c2170;
        goto LAB_1007c2134;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    plVar60 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(uVar62,1);
    if (plVar60 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,uVar62);
    }
    _memcpy(plVar60,pvVar57,uVar62);
    if (uVar62 < 8) {
      uVar74 = 0;
LAB_1007c22a0:
      lVar66 = uVar62 - uVar74;
      pbVar65 = (byte *)((long)plVar60 + uVar74);
      do {
        bVar75 = 0x20;
        if (0x19 < *pbVar65 - 0x41) {
          bVar75 = 0;
        }
        *pbVar65 = bVar75 | *pbVar65;
        lVar66 = lVar66 + -1;
        pbVar65 = pbVar65 + 1;
      } while (lVar66 != 0);
    }
    else if (uVar62 < 0x40) {
      uVar63 = 0;
LAB_1007c225c:
      uVar74 = uVar62 & 0x7ffffffffffffff8;
      lVar66 = uVar63 - uVar74;
      puVar71 = (undefined8 *)((long)plVar60 + uVar63);
      do {
        uVar73 = *puVar71;
        bVar75 = (byte)((ulong)uVar73 >> 8);
        bVar76 = (byte)((ulong)uVar73 >> 0x10);
        bVar77 = (byte)((ulong)uVar73 >> 0x18);
        bVar78 = (byte)((ulong)uVar73 >> 0x20);
        bVar79 = (byte)((ulong)uVar73 >> 0x28);
        bVar80 = (byte)((ulong)uVar73 >> 0x30);
        bVar81 = (byte)((ulong)uVar73 >> 0x38);
        uVar63 = CONCAT17(-((byte)(bVar81 + 0xbf) < 0x1a),
                          CONCAT16(-((byte)(bVar80 + 0xbf) < 0x1a),
                                   CONCAT15(-((byte)(bVar79 + 0xbf) < 0x1a),
                                            CONCAT14(-((byte)(bVar78 + 0xbf) < 0x1a),
                                                     CONCAT13(-((byte)(bVar77 + 0xbf) < 0x1a),
                                                              CONCAT12(-((byte)(bVar76 + 0xbf) < 0x1a),
                                                                       CONCAT11(-((byte)(bVar75 + 0xbf) < 0x1a),-((byte)((byte)uVar73 + 0xbf) < 0x1a)))))))) &
                 0x2020202020202020;
        *puVar71 = CONCAT17((byte)(uVar63 >> 0x38) | bVar81,
                            CONCAT16((byte)(uVar63 >> 0x30) | bVar80,
                                     CONCAT15((byte)(uVar63 >> 0x28) | bVar79,
                                              CONCAT14((byte)(uVar63 >> 0x20) | bVar78,
                                                       CONCAT13((byte)(uVar63 >> 0x18) | bVar77,
                                                                CONCAT12((byte)(uVar63 >> 0x10) | bVar76,
                                                                         CONCAT11((byte)(uVar63 >> 8) | bVar75,(byte)uVar63 | (byte)uVar73)))))));
        lVar66 = lVar66 + 8;
        puVar71 = puVar71 + 1;
      } while (lVar66 != 0);
      if (uVar62 != uVar74) goto LAB_1007c22a0;
    }
    else {
      uVar74 = uVar62 & 0x7fffffffffffffc0;
      pbVar65 = (byte *)(plVar60 + 4);
      uVar63 = uVar74;
      do {
        uVar3 = *(undefined8 *)(pbVar65 + 0x18);
        bVar82 = (byte)((ulong)uVar3 >> 8);
        bVar83 = (byte)((ulong)uVar3 >> 0x10);
        bVar84 = (byte)((ulong)uVar3 >> 0x18);
        bVar85 = (byte)((ulong)uVar3 >> 0x20);
        bVar86 = (byte)((ulong)uVar3 >> 0x28);
        bVar87 = (byte)((ulong)uVar3 >> 0x30);
        bVar88 = (byte)((ulong)uVar3 >> 0x38);
        uVar73 = *(undefined8 *)(pbVar65 + 0x10);
        bVar75 = (byte)((ulong)uVar73 >> 8);
        bVar76 = (byte)((ulong)uVar73 >> 0x10);
        bVar77 = (byte)((ulong)uVar73 >> 0x18);
        bVar78 = (byte)((ulong)uVar73 >> 0x20);
        bVar79 = (byte)((ulong)uVar73 >> 0x28);
        bVar80 = (byte)((ulong)uVar73 >> 0x30);
        bVar81 = (byte)((ulong)uVar73 >> 0x38);
        pbVar65[-0x18] = -((byte)(pbVar65[-0x18] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x18];
        pbVar65[-0x17] = -((byte)(pbVar65[-0x17] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x17];
        pbVar65[-0x16] = -((byte)(pbVar65[-0x16] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x16];
        pbVar65[-0x15] = -((byte)(pbVar65[-0x15] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x15];
        pbVar65[-0x14] = -((byte)(pbVar65[-0x14] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x14];
        pbVar65[-0x13] = -((byte)(pbVar65[-0x13] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x13];
        pbVar65[-0x12] = -((byte)(pbVar65[-0x12] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x12];
        pbVar65[-0x11] = -((byte)(pbVar65[-0x11] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x11];
        pbVar65[-0x20] = -((byte)(pbVar65[-0x20] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x20];
        pbVar65[-0x1f] = -((byte)(pbVar65[-0x1f] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1f];
        pbVar65[-0x1e] = -((byte)(pbVar65[-0x1e] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1e];
        pbVar65[-0x1d] = -((byte)(pbVar65[-0x1d] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1d];
        pbVar65[-0x1c] = -((byte)(pbVar65[-0x1c] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1c];
        pbVar65[-0x1b] = -((byte)(pbVar65[-0x1b] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1b];
        pbVar65[-0x1a] = -((byte)(pbVar65[-0x1a] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x1a];
        pbVar65[-0x19] = -((byte)(pbVar65[-0x19] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x19];
        pbVar65[-8] = -((byte)(pbVar65[-8] + 0xbf) < 0x1a) & 0x20U | pbVar65[-8];
        pbVar65[-7] = -((byte)(pbVar65[-7] + 0xbf) < 0x1a) & 0x20U | pbVar65[-7];
        pbVar65[-6] = -((byte)(pbVar65[-6] + 0xbf) < 0x1a) & 0x20U | pbVar65[-6];
        pbVar65[-5] = -((byte)(pbVar65[-5] + 0xbf) < 0x1a) & 0x20U | pbVar65[-5];
        pbVar65[-4] = -((byte)(pbVar65[-4] + 0xbf) < 0x1a) & 0x20U | pbVar65[-4];
        pbVar65[-3] = -((byte)(pbVar65[-3] + 0xbf) < 0x1a) & 0x20U | pbVar65[-3];
        pbVar65[-2] = -((byte)(pbVar65[-2] + 0xbf) < 0x1a) & 0x20U | pbVar65[-2];
        pbVar65[-1] = -((byte)(pbVar65[-1] + 0xbf) < 0x1a) & 0x20U | pbVar65[-1];
        pbVar65[-0x10] = -((byte)(pbVar65[-0x10] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0x10];
        pbVar65[-0xf] = -((byte)(pbVar65[-0xf] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xf];
        pbVar65[-0xe] = -((byte)(pbVar65[-0xe] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xe];
        pbVar65[-0xd] = -((byte)(pbVar65[-0xd] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xd];
        pbVar65[-0xc] = -((byte)(pbVar65[-0xc] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xc];
        pbVar65[-0xb] = -((byte)(pbVar65[-0xb] + 0xbf) < 0x1a) & 0x20U | pbVar65[-0xb];
        pbVar65[-10] = -((byte)(pbVar65[-10] + 0xbf) < 0x1a) & 0x20U | pbVar65[-10];
        pbVar65[-9] = -((byte)(pbVar65[-9] + 0xbf) < 0x1a) & 0x20U | pbVar65[-9];
        pbVar65[8] = -((byte)(pbVar65[8] + 0xbf) < 0x1a) & 0x20U | pbVar65[8];
        pbVar65[9] = -((byte)(pbVar65[9] + 0xbf) < 0x1a) & 0x20U | pbVar65[9];
        pbVar65[10] = -((byte)(pbVar65[10] + 0xbf) < 0x1a) & 0x20U | pbVar65[10];
        pbVar65[0xb] = -((byte)(pbVar65[0xb] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xb];
        pbVar65[0xc] = -((byte)(pbVar65[0xc] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xc];
        pbVar65[0xd] = -((byte)(pbVar65[0xd] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xd];
        pbVar65[0xe] = -((byte)(pbVar65[0xe] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xe];
        pbVar65[0xf] = -((byte)(pbVar65[0xf] + 0xbf) < 0x1a) & 0x20U | pbVar65[0xf];
        *pbVar65 = -((byte)(*pbVar65 + 0xbf) < 0x1a) & 0x20U | *pbVar65;
        pbVar65[1] = -((byte)(pbVar65[1] + 0xbf) < 0x1a) & 0x20U | pbVar65[1];
        pbVar65[2] = -((byte)(pbVar65[2] + 0xbf) < 0x1a) & 0x20U | pbVar65[2];
        pbVar65[3] = -((byte)(pbVar65[3] + 0xbf) < 0x1a) & 0x20U | pbVar65[3];
        pbVar65[4] = -((byte)(pbVar65[4] + 0xbf) < 0x1a) & 0x20U | pbVar65[4];
        pbVar65[5] = -((byte)(pbVar65[5] + 0xbf) < 0x1a) & 0x20U | pbVar65[5];
        pbVar65[6] = -((byte)(pbVar65[6] + 0xbf) < 0x1a) & 0x20U | pbVar65[6];
        pbVar65[7] = -((byte)(pbVar65[7] + 0xbf) < 0x1a) & 0x20U | pbVar65[7];
        pbVar65[0x18] = -((byte)((byte)uVar3 + 0xbf) < 0x1a) & 0x20U | (byte)uVar3;
        pbVar65[0x19] = -((byte)(bVar82 + 0xbf) < 0x1a) & 0x20U | bVar82;
        pbVar65[0x1a] = -((byte)(bVar83 + 0xbf) < 0x1a) & 0x20U | bVar83;
        pbVar65[0x1b] = -((byte)(bVar84 + 0xbf) < 0x1a) & 0x20U | bVar84;
        pbVar65[0x1c] = -((byte)(bVar85 + 0xbf) < 0x1a) & 0x20U | bVar85;
        pbVar65[0x1d] = -((byte)(bVar86 + 0xbf) < 0x1a) & 0x20U | bVar86;
        pbVar65[0x1e] = -((byte)(bVar87 + 0xbf) < 0x1a) & 0x20U | bVar87;
        pbVar65[0x1f] = -((byte)(bVar88 + 0xbf) < 0x1a) & 0x20U | bVar88;
        pbVar65[0x10] = -((byte)((byte)uVar73 + 0xbf) < 0x1a) & 0x20U | (byte)uVar73;
        pbVar65[0x11] = -((byte)(bVar75 + 0xbf) < 0x1a) & 0x20U | bVar75;
        pbVar65[0x12] = -((byte)(bVar76 + 0xbf) < 0x1a) & 0x20U | bVar76;
        pbVar65[0x13] = -((byte)(bVar77 + 0xbf) < 0x1a) & 0x20U | bVar77;
        pbVar65[0x14] = -((byte)(bVar78 + 0xbf) < 0x1a) & 0x20U | bVar78;
        pbVar65[0x15] = -((byte)(bVar79 + 0xbf) < 0x1a) & 0x20U | bVar79;
        pbVar65[0x16] = -((byte)(bVar80 + 0xbf) < 0x1a) & 0x20U | bVar80;
        pbVar65[0x17] = -((byte)(bVar81 + 0xbf) < 0x1a) & 0x20U | bVar81;
        uVar63 = uVar63 - 0x40;
        pbVar65 = pbVar65 + 0x40;
      } while (uVar63 != 0);
      if (uVar62 != uVar74) {
        uVar63 = uVar74;
        if ((uVar62 & 0x38) == 0) goto LAB_1007c22a0;
        goto LAB_1007c225c;
      }
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar57,uVar62,1);
  }
  auVar89._8_8_ = uVar62;
  auVar89._0_8_ = plVar60;
  local_d0 = 0;
  local_b0 = 0;
  local_a0 = 0x2f0000002f;
  local_98 = CONCAT11(local_98._1_1_,1);
  local_90 = 1;
  local_c8 = uVar62;
  local_c0 = plVar60;
  uStack_b8 = uVar62;
  uStack_a8 = uVar62;
  auVar90 = ::core::str::iter::SplitInternal<P>::next_back(&local_d0);
  if (auVar90._0_8_ != 0) {
    auVar89 = auVar90;
  }
  lVar66 = auVar89._8_8_;
  plVar2 = auVar89._0_8_;
  if (bVar55) {
    ::core::str::pattern::StrSearcher::new(&local_d0,plVar58,param_2,"/node_modules/@openai/codex/",0x1c);
    <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_128,&local_d0);
    if ((local_128 & 1) != 0) goto LAB_1007c2478;
LAB_1007c23a8:
    if (0xd < uVar62) {
      if (*plVar60 != 0x6d6f682f74706f2f || *(long *)((long)plVar60 + 6) != 0x2f77657262656d6f) goto LAB_1007c23f0;
      goto LAB_1007c2478;
    }
    if (10 < uVar62) {
LAB_1007c23f0:
      if (*plVar60 == 0x636f6c2f7273752f && *(long *)((long)plVar60 + 3) == 0x2f6c61636f6c2f72) goto LAB_1007c2478;
    }
    uVar63 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013adae3,0xc,plVar60,uVar62);
    if ((((uVar63 & 1) != 0) || (uVar63 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013af2f0,8,plVar60,uVar62), (uVar63 & 1) != 0)) ||
       (uVar63 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013adaef,6,plVar60,uVar62), (uVar63 & 1) != 0)) goto LAB_1007c2478;
    uVar56 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013adaf5,0x13,plVar60,uVar62);
  }
  else {
    if ((param_2 != 0x1c) ||
       (((*plVar58 != 0x6f6d5f65646f6e2f || plVar58[1] != 0x6f402f73656c7564) || plVar58[2] != 0x6f632f69616e6570) || (int)plVar58[3] != 0x2f786564))
    goto LAB_1007c23a8;
LAB_1007c2478:
    uVar56 = 1;
  }
  if (lVar66 == 4) {
    if ((int)*plVar2 == 0x65646f6e) {
LAB_1007c2510:
      local_128 = 0;
      local_f8 = 0;
      local_f0 = 1;
      uStack_120 = param_2;
      local_118 = param_1;
      uStack_110 = param_2;
      local_108 = param_1;
      lStack_100 = param_1 + param_2;
      lVar66 = ::core::iter::traits::iterator::Iterator::try_fold(&local_128);
      if (lVar66 != 0) {
        auVar90 = ::core::iter::traits::iterator::Iterator::try_fold(&local_128);
        if (auVar90._0_8_ != 0) {
          alloc::str::<impl_str>::replace(&local_d0,auVar90._0_8_,auVar90._8_8_,0x5c,&DAT_1013a9c3e,1);
          uVar63 = local_c8;
          alloc::str::<impl_str>::to_ascii_lowercase(&local_e8,local_c8,local_c0);
          if (local_d0 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar63,local_d0,1);
          }
          auVar54._8_8_ = uStack_d8;
          auVar54._0_8_ = local_e0;
          local_d0 = 0;
          local_c8 = uStack_d8;
          local_c0 = local_e0;
          uStack_b8 = uStack_d8;
          local_b0 = 0;
          uStack_a8 = uStack_d8;
          local_a0 = 0x2f0000002f;
          local_98 = CONCAT11(local_98._1_1_,1);
          local_90 = 1;
          auVar90 = ::core::str::iter::SplitInternal<P>::next_back(&local_d0);
          if (auVar90._0_8_ != 0) {
            auVar54 = auVar90;
          }
          plVar2 = auVar54._0_8_;
          uVar63 = <&str_as_core::str::pattern::Pattern>::is_contained_in("/node_modules/@openai/codex/",0x1c,local_e0,uStack_d8);
          if ((uVar63 & 1) == 0) {
            if (0xd < uStack_d8) {
              if (*local_e0 != 0x6d6f682f74706f2f || *(long *)((long)local_e0 + 6) != 0x2f77657262656d6f) goto LAB_1007c2b7c;
              goto LAB_1007c2c04;
            }
            if (10 < uStack_d8) {
LAB_1007c2b7c:
              if (*local_e0 == 0x636f6c2f7273752f && *(long *)((long)local_e0 + 3) == 0x2f6c61636f6c2f72) goto LAB_1007c2c04;
            }
            uVar63 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013adae3,0xc,local_e0,uStack_d8);
            if ((((uVar63 & 1) != 0) ||
                (uVar63 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013af2f0,8,local_e0,uStack_d8), (uVar63 & 1) != 0)) ||
               (uVar63 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013adaef,6,local_e0,uStack_d8), (uVar63 & 1) != 0)) goto LAB_1007c2c04;
            uVar56 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013adaf5,0x13,local_e0,uStack_d8);
          }
          else {
LAB_1007c2c04:
            uVar56 = 1;
          }
          if (auVar54._8_8_ == 8) {
            if (*plVar2 == 0x736a2e7865646f63) goto LAB_1007c2c58;
          }
          else if ((auVar54._8_8_ == 5) && ((int)*plVar2 == 0x65646f63 && *(char *)((long)plVar2 + 4) == 'x')) {
LAB_1007c2c58:
            if (uStack_d8 == 5) {
              uVar61 = (uint)((int)*local_e0 == 0x65646f63 && *(char *)((long)local_e0 + 4) == 'x');
            }
            else {
              uVar61 = 0;
            }
            if ((uVar56 | uVar61) == 1) {
              if (local_e8 != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_e0,local_e8,1);
              }
              goto LAB_1007c2abc;
            }
          }
          if (local_e8 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_e0,local_e8,1);
          }
        }
      }
    }
  }
  else if (lVar66 == 8) {
    if (*plVar2 == 0x6578652e65646f6e) goto LAB_1007c2510;
  }
  else if ((lVar66 == 5) && ((int)*plVar2 == 0x65646f63 && *(char *)((long)plVar2 + 4) == 'x')) {
    if (uVar62 == 5) {
      bVar55 = (int)*plVar60 == 0x65646f63 && *(char *)((long)plVar60 + 4) == 'x';
    }
    else {
      bVar55 = false;
    }
    if ((uVar56 & 1) != 0 || bVar55) {
LAB_1007c2abc:
      if (uVar62 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar60,uVar62,1);
      }
      if (uVar59 == 0) {
        return 1;
      }
LAB_1007c2ad4:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar58,uVar59,1);
      return 1;
    }
  }
  if (uVar62 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar60,uVar62,1);
  }
  if (uVar59 == 0) {
    return 2;
  }
LAB_1007c2d40:
  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar58,uVar59,1);
  return 2;
}

