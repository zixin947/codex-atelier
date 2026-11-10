/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* append_event @ 10078ebd8 requested 10078ebd8 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::transition_journal::append_event */

void codexmate_lib::core::relay::transition_journal::append_event
               (undefined8 param_1,undefined8 param_2,undefined1 param_3,void *param_4,size_t param_5,void *param_6,ulong param_7,long *param_8,size_t param_9)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  long *******ppppppplVar4;
  long *******ppppppplVar5;
  int iVar6;
  void *pvVar7;
  ulong *puVar8;
  long *plVar9;
  ulong uVar10;
  long *******ppppppplVar11;
  ulong uVar12;
  byte *pbVar13;
  long *******ppppppplVar14;
  ulong *puVar15;
  long lVar16;
  long *plVar17;
  int iVar18;
  int iVar19;
  undefined8 uVar20;
  size_t sVar21;
  char *pcVar22;
  ulong uVar23;
  size_t sVar24;
  long *******ppppppplVar25;
  size_t sVar26;
  undefined8 *puVar27;
  byte bVar29;
  byte bVar30;
  byte bVar31;
  byte bVar32;
  byte bVar33;
  byte bVar34;
  ulong uVar28;
  byte bVar35;
  byte bVar38;
  byte bVar39;
  byte bVar40;
  byte bVar41;
  byte bVar42;
  byte bVar43;
  ulong uVar36;
  long lVar37;
  byte bVar44;
  byte bVar47;
  byte bVar48;
  byte bVar49;
  byte bVar50;
  byte bVar51;
  byte bVar52;
  ulong uVar45;
  long lVar46;
  byte bVar53;
  byte bVar56;
  byte bVar57;
  byte bVar58;
  byte bVar59;
  byte bVar60;
  byte bVar61;
  ulong uVar54;
  long lVar55;
  byte bVar62;
  byte bVar65;
  byte bVar66;
  byte bVar67;
  byte bVar68;
  byte bVar69;
  byte bVar70;
  ulong uVar63;
  long lVar64;
  byte bVar71;
  byte bVar74;
  byte bVar75;
  byte bVar76;
  byte bVar77;
  byte bVar78;
  byte bVar79;
  ulong uVar72;
  long lVar73;
  byte bVar80;
  byte bVar83;
  byte bVar84;
  byte bVar85;
  byte bVar86;
  byte bVar87;
  byte bVar88;
  ulong uVar81;
  long lVar82;
  byte bVar89;
  byte bVar92;
  byte bVar93;
  byte bVar94;
  byte bVar95;
  byte bVar96;
  byte bVar97;
  ulong uVar90;
  long lVar91;
  byte bVar98;
  undefined1 auVar99 [16];
  void *local_168;
  long local_160;
  long *******local_158;
  undefined8 uStack_150;
  long *******local_148;
  long *******local_140;
  long *******local_138;
  undefined8 local_130;
  long *******ppppppplStack_128;
  long *******local_120;
  long *******local_118;
  long *******ppppppplStack_110;
  long *******local_108;
  long *******ppppppplStack_100;
  undefined8 local_f8;
  long *******ppppppplStack_f0;
  undefined8 local_e8;
  undefined1 local_e0;
  undefined2 local_d8;
  size_t local_d0;
  void *pvStack_c8;
  size_t local_c0;
  size_t sStack_b8;
  void *local_b0;
  size_t sStack_a8;
  size_t local_a0;
  long *plStack_98;
  size_t local_90;
  long lStack_88;
  undefined1 local_80;
  undefined1 local_7f [31];
  
  if ((long)param_5 < 0) {
    uVar20 = 0;
LAB_10078ec04:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar20,param_5);
  }
  if (param_5 == 0) {
    pvVar7 = (void *)0x1;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar20 = 1;
    pvVar7 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_5,1);
    if (pvVar7 == (void *)0x0) goto LAB_10078ec04;
  }
  _memcpy(pvVar7,param_4,param_5);
  chrono::offset::utc::Utc::now(&local_130);
  uVar3 = (uint)local_130;
  iVar19 = (int)(uint)local_130 >> 0xd;
  iVar18 = iVar19 + -1;
  uVar23 = (ulong)local_130 >> 0x20;
  if (iVar19 < 1) {
    iVar19 = (1U - iVar19) / 400 + 1;
    iVar18 = iVar18 + iVar19 * 400;
    iVar19 = iVar19 * -0x23ab1;
    if (param_6 != (void *)0x0) goto LAB_10078ec8c;
LAB_10078ecdc:
    sVar24 = 0x8000000000000000;
    if (param_8 != (long *)0x0) goto LAB_10078ece4;
LAB_10078f870:
    sVar21 = 0x8000000000000000;
  }
  else {
    iVar19 = 0;
    if (param_6 == (void *)0x0) goto LAB_10078ecdc;
LAB_10078ec8c:
    sVar24 = param_7;
    if ((long)param_7 < 0) {
      uVar20 = 0;
      goto LAB_10078ec94;
    }
    if (param_7 == 0) {
      _memcpy((void *)0x1,param_6,0);
      pcVar22 = &UNK_1013ab29a;
      sVar24 = 5;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar20 = 1;
      puVar8 = (ulong *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_7,1);
      if (puVar8 == (ulong *)0x0) goto LAB_10078ec94;
      _memcpy(puVar8,param_6,param_7);
      if (param_7 < 8) {
        uVar12 = 0;
LAB_10078eee8:
        lVar16 = param_7 - uVar12;
        pbVar13 = (byte *)((long)puVar8 + uVar12);
        do {
          bVar29 = 0x20;
          if (0x19 < *pbVar13 - 0x41) {
            bVar29 = 0;
          }
          *pbVar13 = bVar29 | *pbVar13;
          lVar16 = lVar16 + -1;
          pbVar13 = pbVar13 + 1;
        } while (lVar16 != 0);
      }
      else if (param_7 < 0x40) {
        uVar10 = 0;
LAB_10078eea4:
        uVar12 = param_7 & 0x7ffffffffffffff8;
        lVar16 = uVar10 - uVar12;
        puVar27 = (undefined8 *)((long)puVar8 + uVar10);
        do {
          uVar20 = *puVar27;
          bVar29 = (byte)((ulong)uVar20 >> 8);
          bVar30 = (byte)((ulong)uVar20 >> 0x10);
          bVar31 = (byte)((ulong)uVar20 >> 0x18);
          bVar32 = (byte)((ulong)uVar20 >> 0x20);
          bVar33 = (byte)((ulong)uVar20 >> 0x28);
          bVar34 = (byte)((ulong)uVar20 >> 0x30);
          bVar35 = (byte)((ulong)uVar20 >> 0x38);
          uVar10 = CONCAT17(-((byte)(bVar35 + 0xbf) < 0x1a),
                            CONCAT16(-((byte)(bVar34 + 0xbf) < 0x1a),
                                     CONCAT15(-((byte)(bVar33 + 0xbf) < 0x1a),
                                              CONCAT14(-((byte)(bVar32 + 0xbf) < 0x1a),
                                                       CONCAT13(-((byte)(bVar31 + 0xbf) < 0x1a),
                                                                CONCAT12(-((byte)(bVar30 + 0xbf) < 0x1a),
                                                                         CONCAT11(-((byte)(bVar29 + 0xbf) < 0x1a),-((byte)((byte)uVar20 + 0xbf) < 0x1a)))))))) &
                   0x2020202020202020;
          *puVar27 = CONCAT17((byte)(uVar10 >> 0x38) | bVar35,
                              CONCAT16((byte)(uVar10 >> 0x30) | bVar34,
                                       CONCAT15((byte)(uVar10 >> 0x28) | bVar33,
                                                CONCAT14((byte)(uVar10 >> 0x20) | bVar32,
                                                         CONCAT13((byte)(uVar10 >> 0x18) | bVar31,
                                                                  CONCAT12((byte)(uVar10 >> 0x10) | bVar30,
                                                                           CONCAT11((byte)(uVar10 >> 8) | bVar29,(byte)uVar10 | (byte)uVar20)))))));
          lVar16 = lVar16 + 8;
          puVar27 = puVar27 + 1;
        } while (lVar16 != 0);
        if (param_7 != uVar12) goto LAB_10078eee8;
      }
      else {
        uVar12 = param_7 & 0x7fffffffffffffc0;
        puVar15 = puVar8 + 4;
        uVar10 = uVar12;
        do {
          uVar36 = puVar15[-3];
          uVar28 = puVar15[-4];
          uVar54 = puVar15[-1];
          uVar45 = puVar15[-2];
          uVar72 = puVar15[1];
          uVar63 = *puVar15;
          uVar90 = puVar15[3];
          uVar81 = puVar15[2];
          bVar29 = (byte)(uVar28 >> 8);
          bVar30 = (byte)(uVar28 >> 0x10);
          bVar31 = (byte)(uVar28 >> 0x18);
          bVar32 = (byte)(uVar28 >> 0x20);
          bVar33 = (byte)(uVar28 >> 0x28);
          bVar34 = (byte)(uVar28 >> 0x30);
          bVar35 = (byte)(uVar28 >> 0x38);
          bVar38 = (byte)(uVar36 >> 8);
          bVar39 = (byte)(uVar36 >> 0x10);
          bVar40 = (byte)(uVar36 >> 0x18);
          bVar41 = (byte)(uVar36 >> 0x20);
          bVar42 = (byte)(uVar36 >> 0x28);
          bVar43 = (byte)(uVar36 >> 0x30);
          bVar44 = (byte)(uVar36 >> 0x38);
          bVar47 = (byte)(uVar45 >> 8);
          bVar48 = (byte)(uVar45 >> 0x10);
          bVar49 = (byte)(uVar45 >> 0x18);
          bVar50 = (byte)(uVar45 >> 0x20);
          bVar51 = (byte)(uVar45 >> 0x28);
          bVar52 = (byte)(uVar45 >> 0x30);
          bVar53 = (byte)(uVar45 >> 0x38);
          bVar56 = (byte)(uVar54 >> 8);
          bVar57 = (byte)(uVar54 >> 0x10);
          bVar58 = (byte)(uVar54 >> 0x18);
          bVar59 = (byte)(uVar54 >> 0x20);
          bVar60 = (byte)(uVar54 >> 0x28);
          bVar61 = (byte)(uVar54 >> 0x30);
          bVar62 = (byte)(uVar54 >> 0x38);
          bVar65 = (byte)(uVar63 >> 8);
          bVar66 = (byte)(uVar63 >> 0x10);
          bVar67 = (byte)(uVar63 >> 0x18);
          bVar68 = (byte)(uVar63 >> 0x20);
          bVar69 = (byte)(uVar63 >> 0x28);
          bVar70 = (byte)(uVar63 >> 0x30);
          bVar71 = (byte)(uVar63 >> 0x38);
          bVar74 = (byte)(uVar72 >> 8);
          bVar75 = (byte)(uVar72 >> 0x10);
          bVar76 = (byte)(uVar72 >> 0x18);
          bVar77 = (byte)(uVar72 >> 0x20);
          bVar78 = (byte)(uVar72 >> 0x28);
          bVar79 = (byte)(uVar72 >> 0x30);
          bVar80 = (byte)(uVar72 >> 0x38);
          bVar83 = (byte)(uVar81 >> 8);
          bVar84 = (byte)(uVar81 >> 0x10);
          bVar85 = (byte)(uVar81 >> 0x18);
          bVar86 = (byte)(uVar81 >> 0x20);
          bVar87 = (byte)(uVar81 >> 0x28);
          bVar88 = (byte)(uVar81 >> 0x30);
          bVar89 = (byte)(uVar81 >> 0x38);
          bVar92 = (byte)(uVar90 >> 8);
          bVar93 = (byte)(uVar90 >> 0x10);
          bVar94 = (byte)(uVar90 >> 0x18);
          bVar95 = (byte)(uVar90 >> 0x20);
          bVar96 = (byte)(uVar90 >> 0x28);
          bVar97 = (byte)(uVar90 >> 0x30);
          bVar98 = (byte)(uVar90 >> 0x38);
          puVar15[-3] = CONCAT17(-((byte)(bVar44 + 0xbf) < 0x1a) & 0x20U | bVar44,
                                 CONCAT16(-((byte)(bVar43 + 0xbf) < 0x1a) & 0x20U | bVar43,
                                          CONCAT15(-((byte)(bVar42 + 0xbf) < 0x1a) & 0x20U | bVar42,
                                                   CONCAT14(-((byte)(bVar41 + 0xbf) < 0x1a) & 0x20U | bVar41,
                                                            CONCAT13(-((byte)(bVar40 + 0xbf) < 0x1a) & 0x20U | bVar40,
                                                                     CONCAT12(-((byte)(bVar39 + 0xbf) < 0x1a) & 0x20U | bVar39,
                                                                              CONCAT11(-((byte)(bVar38 + 0xbf) < 0x1a) & 0x20U | bVar38,
                                                                                       -((byte)((byte)uVar36 + 0xbf) < 0x1a) & 0x20U | (byte)uVar36)))))));
          puVar15[-4] = CONCAT17(-((byte)(bVar35 + 0xbf) < 0x1a) & 0x20U | bVar35,
                                 CONCAT16(-((byte)(bVar34 + 0xbf) < 0x1a) & 0x20U | bVar34,
                                          CONCAT15(-((byte)(bVar33 + 0xbf) < 0x1a) & 0x20U | bVar33,
                                                   CONCAT14(-((byte)(bVar32 + 0xbf) < 0x1a) & 0x20U | bVar32,
                                                            CONCAT13(-((byte)(bVar31 + 0xbf) < 0x1a) & 0x20U | bVar31,
                                                                     CONCAT12(-((byte)(bVar30 + 0xbf) < 0x1a) & 0x20U | bVar30,
                                                                              CONCAT11(-((byte)(bVar29 + 0xbf) < 0x1a) & 0x20U | bVar29,
                                                                                       -((byte)((byte)uVar28 + 0xbf) < 0x1a) & 0x20U | (byte)uVar28)))))));
          puVar15[-1] = CONCAT17(-((byte)(bVar62 + 0xbf) < 0x1a) & 0x20U | bVar62,
                                 CONCAT16(-((byte)(bVar61 + 0xbf) < 0x1a) & 0x20U | bVar61,
                                          CONCAT15(-((byte)(bVar60 + 0xbf) < 0x1a) & 0x20U | bVar60,
                                                   CONCAT14(-((byte)(bVar59 + 0xbf) < 0x1a) & 0x20U | bVar59,
                                                            CONCAT13(-((byte)(bVar58 + 0xbf) < 0x1a) & 0x20U | bVar58,
                                                                     CONCAT12(-((byte)(bVar57 + 0xbf) < 0x1a) & 0x20U | bVar57,
                                                                              CONCAT11(-((byte)(bVar56 + 0xbf) < 0x1a) & 0x20U | bVar56,
                                                                                       -((byte)((byte)uVar54 + 0xbf) < 0x1a) & 0x20U | (byte)uVar54)))))));
          puVar15[-2] = CONCAT17(-((byte)(bVar53 + 0xbf) < 0x1a) & 0x20U | bVar53,
                                 CONCAT16(-((byte)(bVar52 + 0xbf) < 0x1a) & 0x20U | bVar52,
                                          CONCAT15(-((byte)(bVar51 + 0xbf) < 0x1a) & 0x20U | bVar51,
                                                   CONCAT14(-((byte)(bVar50 + 0xbf) < 0x1a) & 0x20U | bVar50,
                                                            CONCAT13(-((byte)(bVar49 + 0xbf) < 0x1a) & 0x20U | bVar49,
                                                                     CONCAT12(-((byte)(bVar48 + 0xbf) < 0x1a) & 0x20U | bVar48,
                                                                              CONCAT11(-((byte)(bVar47 + 0xbf) < 0x1a) & 0x20U | bVar47,
                                                                                       -((byte)((byte)uVar45 + 0xbf) < 0x1a) & 0x20U | (byte)uVar45)))))));
          puVar15[1] = CONCAT17(-((byte)(bVar80 + 0xbf) < 0x1a) & 0x20U | bVar80,
                                CONCAT16(-((byte)(bVar79 + 0xbf) < 0x1a) & 0x20U | bVar79,
                                         CONCAT15(-((byte)(bVar78 + 0xbf) < 0x1a) & 0x20U | bVar78,
                                                  CONCAT14(-((byte)(bVar77 + 0xbf) < 0x1a) & 0x20U | bVar77,
                                                           CONCAT13(-((byte)(bVar76 + 0xbf) < 0x1a) & 0x20U | bVar76,
                                                                    CONCAT12(-((byte)(bVar75 + 0xbf) < 0x1a) & 0x20U | bVar75,
                                                                             CONCAT11(-((byte)(bVar74 + 0xbf) < 0x1a) & 0x20U | bVar74,
                                                                                      -((byte)((byte)uVar72 + 0xbf) < 0x1a) & 0x20U | (byte)uVar72)))))));
          *puVar15 = CONCAT17(-((byte)(bVar71 + 0xbf) < 0x1a) & 0x20U | bVar71,
                              CONCAT16(-((byte)(bVar70 + 0xbf) < 0x1a) & 0x20U | bVar70,
                                       CONCAT15(-((byte)(bVar69 + 0xbf) < 0x1a) & 0x20U | bVar69,
                                                CONCAT14(-((byte)(bVar68 + 0xbf) < 0x1a) & 0x20U | bVar68,
                                                         CONCAT13(-((byte)(bVar67 + 0xbf) < 0x1a) & 0x20U | bVar67,
                                                                  CONCAT12(-((byte)(bVar66 + 0xbf) < 0x1a) & 0x20U | bVar66,
                                                                           CONCAT11(-((byte)(bVar65 + 0xbf) < 0x1a) & 0x20U | bVar65,
                                                                                    -((byte)((byte)uVar63 + 0xbf) < 0x1a) & 0x20U | (byte)uVar63)))))));
          puVar15[3] = CONCAT17(-((byte)(bVar98 + 0xbf) < 0x1a) & 0x20U | bVar98,
                                CONCAT16(-((byte)(bVar97 + 0xbf) < 0x1a) & 0x20U | bVar97,
                                         CONCAT15(-((byte)(bVar96 + 0xbf) < 0x1a) & 0x20U | bVar96,
                                                  CONCAT14(-((byte)(bVar95 + 0xbf) < 0x1a) & 0x20U | bVar95,
                                                           CONCAT13(-((byte)(bVar94 + 0xbf) < 0x1a) & 0x20U | bVar94,
                                                                    CONCAT12(-((byte)(bVar93 + 0xbf) < 0x1a) & 0x20U | bVar93,
                                                                             CONCAT11(-((byte)(bVar92 + 0xbf) < 0x1a) & 0x20U | bVar92,
                                                                                      -((byte)((byte)uVar90 + 0xbf) < 0x1a) & 0x20U | (byte)uVar90)))))));
          puVar15[2] = CONCAT17(-((byte)(bVar89 + 0xbf) < 0x1a) & 0x20U | bVar89,
                                CONCAT16(-((byte)(bVar88 + 0xbf) < 0x1a) & 0x20U | bVar88,
                                         CONCAT15(-((byte)(bVar87 + 0xbf) < 0x1a) & 0x20U | bVar87,
                                                  CONCAT14(-((byte)(bVar86 + 0xbf) < 0x1a) & 0x20U | bVar86,
                                                           CONCAT13(-((byte)(bVar85 + 0xbf) < 0x1a) & 0x20U | bVar85,
                                                                    CONCAT12(-((byte)(bVar84 + 0xbf) < 0x1a) & 0x20U | bVar84,
                                                                             CONCAT11(-((byte)(bVar83 + 0xbf) < 0x1a) & 0x20U | bVar83,
                                                                                      -((byte)((byte)uVar81 + 0xbf) < 0x1a) & 0x20U | (byte)uVar81)))))));
          uVar10 = uVar10 - 0x40;
          puVar15 = puVar15 + 8;
        } while (uVar10 != 0);
        if (param_7 != uVar12) {
          uVar10 = uVar12;
          if ((param_7 & 0x38) == 0) goto LAB_10078eee8;
          goto LAB_10078eea4;
        }
      }
      if (param_7 < 9) {
        if (param_7 != 8) {
          if (6 < param_7) goto LAB_10078efc4;
          if (param_7 == 6) {
            bVar1 = (int)*puVar8 == 0x61706572;
            bVar2 = *(short *)((long)puVar8 + 4) == 0x7269;
            sVar24 = 5;
            if (bVar1 && bVar2) {
              sVar24 = 6;
            }
            pcVar22 = &UNK_1013ab29a;
            if (bVar1 && bVar2) {
              pcVar22 = &DAT_1013ab28b;
            }
            goto LAB_10078f824;
          }
          goto LAB_10078f820;
        }
        if (*puVar8 == 0x6b6361626c6c6f72) goto LAB_10078efac;
LAB_10078efc4:
        ::core::str::pattern::StrSearcher::new(&local_d0,puVar8,param_7,&DAT_1013ab28b,6);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
        if (((ulong)local_118 & 1) == 0) {
          if (param_7 < 10) {
            if (param_7 != 9) goto LAB_10078f820;
            uVar10 = (*puVar8 & 0xff00ff00ff00ff00) >> 8 | (*puVar8 & 0xff00ff00ff00ff) << 8;
            uVar10 = (uVar10 & 0xffff0000ffff0000) >> 0x10 | (uVar10 & 0xffff0000ffff) << 0x10;
            uVar10 = uVar10 >> 0x20 | uVar10 << 0x20;
            if (uVar10 == 0x7265636f6e63696c) {
              if ((char)puVar8[1] != 'e') goto LAB_10078f820;
            }
            else {
              iVar6 = 1;
              if (0x7265636f6e63696c < uVar10) {
                iVar6 = -1;
              }
              if (iVar6 != 0) goto LAB_10078f820;
            }
          }
          else {
            ::core::str::pattern::StrSearcher::new(&local_d0,puVar8,param_7,&DAT_1013ab291,9);
            <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
            if (((ulong)local_118 & 1) == 0) {
LAB_10078f820:
              sVar24 = 5;
              pcVar22 = &UNK_1013ab29a;
              goto LAB_10078f824;
            }
          }
        }
        sVar24 = 6;
        pcVar22 = &DAT_1013ab28b;
      }
      else {
        ::core::str::pattern::StrSearcher::new(&local_d0,puVar8,param_7,"rollback",8);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
        if (((ulong)local_118 & 1) == 0) {
          if (param_7 < 0xd) {
            if (param_7 == 0xc) {
              if (*puVar8 == 0x61736e65706d6f63 && (int)puVar8[1] == 0x6e6f6974) goto LAB_10078efac;
              goto LAB_10078f53c;
            }
            if (10 < param_7) goto LAB_10078f53c;
            if (param_7 == 10) {
              if (*puVar8 == 0x74736f6e67616964 && (short)puVar8[1] == 0x6369) goto LAB_10078f56c;
              goto LAB_10078f694;
            }
            if (*puVar8 != 0x61727473746f6f62 || (char)puVar8[1] != 'p') {
              if (*puVar8 != 0x646f6d2d68747561 || (char)puVar8[1] != 'e') goto LAB_10078efc4;
              goto LAB_10078f704;
            }
LAB_10078f6c4:
            sVar24 = 9;
            pcVar22 = &DAT_1013ab261;
          }
          else {
            ::core::str::pattern::StrSearcher::new(&local_d0,puVar8,param_7,"compensationstopping_codexreconcilingretryingdegraded",0xc);
            <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
            if (((ulong)local_118 & 1) != 0) goto LAB_10078efac;
LAB_10078f53c:
            ::core::str::pattern::StrSearcher::new(&local_d0,puVar8,param_7,&DAT_1013ab257,10);
            <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
            if (((ulong)local_118 & 1) == 0) {
LAB_10078f694:
              ::core::str::pattern::StrSearcher::new(&local_d0,puVar8,param_7,&DAT_1013ab261,9);
              <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
              if (((ulong)local_118 & 1) != 0) goto LAB_10078f6c4;
              ::core::str::pattern::StrSearcher::new(&local_d0,puVar8,param_7,&DAT_1013ab26a,9);
              <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
              if (((ulong)local_118 & 1) == 0) {
                if (param_7 < 0xc) {
                  if ((param_7 != 0xb) || (*puVar8 != 0x6c73206c65646f6d || *(long *)((long)puVar8 + 3) != 0x73746f6c73206c65)) goto LAB_10078efc4;
                }
                else {
                  ::core::str::pattern::StrSearcher::new(&local_d0,puVar8,param_7,&DAT_1013ab273,0xb);
                  <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
                  if (((ulong)local_118 & 1) == 0) {
                    if (param_7 < 0xe) {
                      if ((param_7 == 0xd) && (*puVar8 == 0x7420726574756f72 && *(long *)((long)puVar8 + 5) == 0x656c67676f742072)) goto LAB_100790064;
                    }
                    else {
                      ::core::str::pattern::StrSearcher::new(&local_d0,puVar8,param_7,&DAT_1013ab27e,0xd);
                      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
                      if (((ulong)local_118 & 1) != 0) {
LAB_100790064:
                        sVar24 = 6;
                        pcVar22 = &UNK_1013ab29f;
                        goto LAB_10078f824;
                      }
                    }
                    goto LAB_10078efc4;
                  }
                }
                sVar24 = 0xb;
                pcVar22 = &UNK_1013ab2a5;
              }
              else {
LAB_10078f704:
                sVar24 = 9;
                pcVar22 = &UNK_1013ab2b0;
              }
            }
            else {
LAB_10078f56c:
              sVar24 = 10;
              pcVar22 = &DAT_1013ab257;
            }
          }
        }
        else {
LAB_10078efac:
          sVar24 = 0xc;
          pcVar22 = "compensationstopping_codexreconcilingretryingdegraded";
        }
      }
LAB_10078f824:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar8,param_7,1);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar20 = 1;
    local_168 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar24,1);
    if (local_168 == (void *)0x0) {
LAB_10078ec94:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(uVar20,sVar24);
    }
    _memcpy(local_168,pcVar22,sVar24);
    if (param_8 == (long *)0x0) goto LAB_10078f870;
LAB_10078ece4:
    sVar21 = param_9;
    if ((long)param_9 < 0) {
      uVar20 = 0;
      goto LAB_10078ecf4;
    }
    if (param_9 == 0) {
      plVar9 = (long *)0x1;
      _memcpy((void *)0x1,param_8,0);
      sVar26 = 0;
LAB_10078ede4:
      uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab093,0xf,param_8,param_9);
      if ((uVar10 & 1) == 0) {
        uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in("taskkillre-entrytakeoverrollback",8,plVar9,param_9);
        if ((((((uVar10 & 1) == 0) && (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab0a2,0xb,plVar9,param_9), (uVar10 & 1) == 0))
             && (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab0ad,0xb,plVar9,param_9), (uVar10 & 1) == 0)) &&
            ((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab0b8,0xc,plVar9,param_9), (uVar10 & 1) == 0 &&
             (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013af130,0x10,plVar9,param_9), (uVar10 & 1) == 0)))) &&
           ((((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in("re-entrytakeoverrollback",8,plVar9,param_9), (uVar10 & 1) == 0 &&
              ((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab0c4,7,plVar9,param_9), (uVar10 & 1) == 0 &&
               (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab0cb,0xe,plVar9,param_9), (uVar10 & 1) == 0)))) &&
             (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab0d9,0xd,plVar9,param_9), (uVar10 & 1) == 0)) &&
            (((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab0e6,0x12,param_8,param_9), (uVar10 & 1) == 0 &&
              (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab0f8,0x12,param_8,param_9), (uVar10 & 1) == 0)) &&
             (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab10a,0xc,param_8,param_9), (uVar10 & 1) == 0)))))) {
          uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&_anon_6408540cdffbe318d49f52812e99783f_267,6,plVar9,param_9);
          if ((((uVar10 & 1) == 0) && (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab116,7,plVar9,param_9), (uVar10 & 1) == 0)) &&
             (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab11d,6,param_8,param_9), (uVar10 & 1) == 0)) {
            uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in("takeoverrollback",8,plVar9,param_9);
            if ((((uVar10 & 1) != 0) ||
                (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&_anon_6408540cdffbe318d49f52812e99783f_498,6,plVar9,param_9),
                (uVar10 & 1) != 0)) ||
               ((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab129,6,param_8,param_9), (uVar10 & 1) != 0 ||
                (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab12f,6,param_8,param_9), (uVar10 & 1) != 0)))) {
              pcVar22 = &UNK_1013ab232;
              goto LAB_10078f15c;
            }
            uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab135,0xd,plVar9,param_9);
            if (((((uVar10 & 1) == 0) &&
                 (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in("model_catalog",0xd,plVar9,param_9), (uVar10 & 1) == 0)) &&
                (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab142,7,plVar9,param_9), (uVar10 & 1) == 0)) &&
               ((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab149,0xc,param_8,param_9), (uVar10 & 1) == 0 &&
                (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab155,0xc,param_8,param_9), (uVar10 & 1) == 0)))) {
              uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&_anon_6408540cdffbe318d49f52812e99783f_505,7,plVar9,param_9);
              if ((((uVar10 & 1) == 0) &&
                  (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab168,0xe,plVar9,param_9), (uVar10 & 1) == 0)) &&
                 ((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab176,6,param_8,param_9), (uVar10 & 1) == 0 &&
                  (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab17c,9,param_8,param_9), (uVar10 & 1) == 0)))) {
                uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab185,6,plVar9,param_9);
                if (((((uVar10 & 1) == 0) &&
                     (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013af318,4,plVar9,param_9), (uVar10 & 1) == 0)) &&
                    (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab18b,0xd,plVar9,param_9), (uVar10 & 1) == 0)) &&
                   (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab198,6,param_8,param_9), (uVar10 & 1) == 0)) {
                  uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013082d4,4,plVar9,param_9);
                  if ((((uVar10 & 1) == 0) &&
                      (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab19e,5,plVar9,param_9), (uVar10 & 1) == 0)) &&
                     (((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&_anon_6408540cdffbe318d49f52812e99783f_515,5,plVar9,param_9),
                       (uVar10 & 1) == 0 &&
                       ((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1a8,6,param_8,param_9), (uVar10 & 1) == 0 &&
                        (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1ae,6,param_8,param_9), (uVar10 & 1) == 0)))) &&
                      (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1b4,6,param_8,param_9), (uVar10 & 1) == 0)))) {
                    uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1ba,7,plVar9,param_9);
                    if (((((uVar10 & 1) != 0) ||
                         (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1c1,0xb,plVar9,param_9), (uVar10 & 1) != 0)) ||
                        (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1cc,9,param_8,param_9), (uVar10 & 1) != 0)) ||
                       (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1d5,0xc,param_8,param_9), (uVar10 & 1) != 0)) {
                      pcVar22 = &UNK_1013ab20d;
                      goto LAB_100790404;
                    }
                    uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&_anon_6408540cdffbe318d49f52812e99783f_523,5,plVar9,param_9);
                    if ((((uVar10 & 1) == 0) &&
                        (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&_anon_6408540cdffbe318d49f52812e99783f_524,4,plVar9,param_9),
                        (uVar10 & 1) == 0)) &&
                       ((uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1ea,6,param_8,param_9), (uVar10 & 1) == 0 &&
                        (uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1f0,6,param_8,param_9), (uVar10 & 1) == 0)))) {
                      uVar10 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1f6,9,plVar9,param_9);
                      if ((uVar10 & 1) == 0) {
                        iVar6 = <&str_as_core::str::pattern::Pattern>::is_contained_in(&DAT_1013ab1ff,0xe,plVar9,param_9);
                        sVar21 = 9;
                        if (iVar6 == 0) {
                          sVar21 = 7;
                        }
                        pcVar22 = &DAT_1013ab1f6;
                        if (iVar6 == 0) {
                          pcVar22 = (char *)&_anon_6408540cdffbe318d49f52812e99783f_224;
                        }
                      }
                      else {
                        sVar21 = 9;
                        pcVar22 = &DAT_1013ab1f6;
                      }
                    }
                    else {
                      sVar21 = 5;
                      pcVar22 = (char *)&_anon_6408540cdffbe318d49f52812e99783f_523;
                    }
                  }
                  else {
                    sVar21 = 4;
                    pcVar22 = "authportPK\x06\x06PK\x01\x02PK\x05\x06\x01\x01";
                  }
                }
                else {
                  sVar21 = 0xb;
                  pcVar22 = &UNK_1013ab21a;
                }
                goto LAB_10078f164;
              }
              pcVar22 = &UNK_1013ab225;
            }
            else {
              pcVar22 = "model_catalog";
            }
LAB_100790404:
            sVar21 = 0xd;
          }
          else {
            sVar21 = 0x10;
            pcVar22 = "thread_reconcilerestoring_configtakeover_restorerestore-rollbackBackup corruptedUntitled Sessioncodex-session.md";
          }
        }
        else {
          pcVar22 = &UNK_1013ab241;
LAB_10078f15c:
          sVar21 = 0xf;
        }
      }
      else {
        sVar21 = 7;
        pcVar22 = &UNK_1013ab250;
      }
LAB_10078f164:
      if (sVar26 != 0) goto LAB_10078fa1c;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar20 = 1;
      plVar9 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_9,1);
      if (plVar9 == (long *)0x0) goto LAB_10078ecf4;
      _memcpy(plVar9,param_8,param_9);
      if (param_9 < 8) {
        uVar12 = 0;
LAB_10078f230:
        lVar16 = param_9 - uVar12;
        pbVar13 = (byte *)((long)plVar9 + uVar12);
        do {
          bVar29 = 0x20;
          if (0x19 < *pbVar13 - 0x41) {
            bVar29 = 0;
          }
          *pbVar13 = bVar29 | *pbVar13;
          lVar16 = lVar16 + -1;
          pbVar13 = pbVar13 + 1;
        } while (lVar16 != 0);
      }
      else if (param_9 < 0x40) {
        uVar10 = 0;
LAB_10078f1ec:
        uVar12 = param_9 & 0x7ffffffffffffff8;
        lVar16 = uVar10 - uVar12;
        puVar27 = (undefined8 *)((long)plVar9 + uVar10);
        do {
          uVar20 = *puVar27;
          bVar29 = (byte)((ulong)uVar20 >> 8);
          bVar30 = (byte)((ulong)uVar20 >> 0x10);
          bVar31 = (byte)((ulong)uVar20 >> 0x18);
          bVar32 = (byte)((ulong)uVar20 >> 0x20);
          bVar33 = (byte)((ulong)uVar20 >> 0x28);
          bVar34 = (byte)((ulong)uVar20 >> 0x30);
          bVar35 = (byte)((ulong)uVar20 >> 0x38);
          uVar10 = CONCAT17(-((byte)(bVar35 + 0xbf) < 0x1a),
                            CONCAT16(-((byte)(bVar34 + 0xbf) < 0x1a),
                                     CONCAT15(-((byte)(bVar33 + 0xbf) < 0x1a),
                                              CONCAT14(-((byte)(bVar32 + 0xbf) < 0x1a),
                                                       CONCAT13(-((byte)(bVar31 + 0xbf) < 0x1a),
                                                                CONCAT12(-((byte)(bVar30 + 0xbf) < 0x1a),
                                                                         CONCAT11(-((byte)(bVar29 + 0xbf) < 0x1a),-((byte)((byte)uVar20 + 0xbf) < 0x1a)))))))) &
                   0x2020202020202020;
          *puVar27 = CONCAT17((byte)(uVar10 >> 0x38) | bVar35,
                              CONCAT16((byte)(uVar10 >> 0x30) | bVar34,
                                       CONCAT15((byte)(uVar10 >> 0x28) | bVar33,
                                                CONCAT14((byte)(uVar10 >> 0x20) | bVar32,
                                                         CONCAT13((byte)(uVar10 >> 0x18) | bVar31,
                                                                  CONCAT12((byte)(uVar10 >> 0x10) | bVar30,
                                                                           CONCAT11((byte)(uVar10 >> 8) | bVar29,(byte)uVar10 | (byte)uVar20)))))));
          lVar16 = lVar16 + 8;
          puVar27 = puVar27 + 1;
        } while (lVar16 != 0);
        if (param_9 != uVar12) goto LAB_10078f230;
      }
      else {
        uVar12 = param_9 & 0x7fffffffffffffc0;
        plVar17 = plVar9 + 4;
        uVar10 = uVar12;
        do {
          lVar37 = plVar17[-3];
          lVar16 = plVar17[-4];
          lVar55 = plVar17[-1];
          lVar46 = plVar17[-2];
          lVar73 = plVar17[1];
          lVar64 = *plVar17;
          lVar91 = plVar17[3];
          lVar82 = plVar17[2];
          bVar29 = (byte)((ulong)lVar16 >> 8);
          bVar30 = (byte)((ulong)lVar16 >> 0x10);
          bVar31 = (byte)((ulong)lVar16 >> 0x18);
          bVar32 = (byte)((ulong)lVar16 >> 0x20);
          bVar33 = (byte)((ulong)lVar16 >> 0x28);
          bVar34 = (byte)((ulong)lVar16 >> 0x30);
          bVar35 = (byte)((ulong)lVar16 >> 0x38);
          bVar38 = (byte)((ulong)lVar37 >> 8);
          bVar39 = (byte)((ulong)lVar37 >> 0x10);
          bVar40 = (byte)((ulong)lVar37 >> 0x18);
          bVar41 = (byte)((ulong)lVar37 >> 0x20);
          bVar42 = (byte)((ulong)lVar37 >> 0x28);
          bVar43 = (byte)((ulong)lVar37 >> 0x30);
          bVar44 = (byte)((ulong)lVar37 >> 0x38);
          bVar47 = (byte)((ulong)lVar46 >> 8);
          bVar48 = (byte)((ulong)lVar46 >> 0x10);
          bVar49 = (byte)((ulong)lVar46 >> 0x18);
          bVar50 = (byte)((ulong)lVar46 >> 0x20);
          bVar51 = (byte)((ulong)lVar46 >> 0x28);
          bVar52 = (byte)((ulong)lVar46 >> 0x30);
          bVar53 = (byte)((ulong)lVar46 >> 0x38);
          bVar56 = (byte)((ulong)lVar55 >> 8);
          bVar57 = (byte)((ulong)lVar55 >> 0x10);
          bVar58 = (byte)((ulong)lVar55 >> 0x18);
          bVar59 = (byte)((ulong)lVar55 >> 0x20);
          bVar60 = (byte)((ulong)lVar55 >> 0x28);
          bVar61 = (byte)((ulong)lVar55 >> 0x30);
          bVar62 = (byte)((ulong)lVar55 >> 0x38);
          bVar65 = (byte)((ulong)lVar64 >> 8);
          bVar66 = (byte)((ulong)lVar64 >> 0x10);
          bVar67 = (byte)((ulong)lVar64 >> 0x18);
          bVar68 = (byte)((ulong)lVar64 >> 0x20);
          bVar69 = (byte)((ulong)lVar64 >> 0x28);
          bVar70 = (byte)((ulong)lVar64 >> 0x30);
          bVar71 = (byte)((ulong)lVar64 >> 0x38);
          bVar74 = (byte)((ulong)lVar73 >> 8);
          bVar75 = (byte)((ulong)lVar73 >> 0x10);
          bVar76 = (byte)((ulong)lVar73 >> 0x18);
          bVar77 = (byte)((ulong)lVar73 >> 0x20);
          bVar78 = (byte)((ulong)lVar73 >> 0x28);
          bVar79 = (byte)((ulong)lVar73 >> 0x30);
          bVar80 = (byte)((ulong)lVar73 >> 0x38);
          bVar83 = (byte)((ulong)lVar82 >> 8);
          bVar84 = (byte)((ulong)lVar82 >> 0x10);
          bVar85 = (byte)((ulong)lVar82 >> 0x18);
          bVar86 = (byte)((ulong)lVar82 >> 0x20);
          bVar87 = (byte)((ulong)lVar82 >> 0x28);
          bVar88 = (byte)((ulong)lVar82 >> 0x30);
          bVar89 = (byte)((ulong)lVar82 >> 0x38);
          bVar92 = (byte)((ulong)lVar91 >> 8);
          bVar93 = (byte)((ulong)lVar91 >> 0x10);
          bVar94 = (byte)((ulong)lVar91 >> 0x18);
          bVar95 = (byte)((ulong)lVar91 >> 0x20);
          bVar96 = (byte)((ulong)lVar91 >> 0x28);
          bVar97 = (byte)((ulong)lVar91 >> 0x30);
          bVar98 = (byte)((ulong)lVar91 >> 0x38);
          plVar17[-3] = CONCAT17(-((byte)(bVar44 + 0xbf) < 0x1a) & 0x20U | bVar44,
                                 CONCAT16(-((byte)(bVar43 + 0xbf) < 0x1a) & 0x20U | bVar43,
                                          CONCAT15(-((byte)(bVar42 + 0xbf) < 0x1a) & 0x20U | bVar42,
                                                   CONCAT14(-((byte)(bVar41 + 0xbf) < 0x1a) & 0x20U | bVar41,
                                                            CONCAT13(-((byte)(bVar40 + 0xbf) < 0x1a) & 0x20U | bVar40,
                                                                     CONCAT12(-((byte)(bVar39 + 0xbf) < 0x1a) & 0x20U | bVar39,
                                                                              CONCAT11(-((byte)(bVar38 + 0xbf) < 0x1a) & 0x20U | bVar38,
                                                                                       -((byte)((byte)lVar37 + 0xbf) < 0x1a) & 0x20U | (byte)lVar37)))))));
          plVar17[-4] = CONCAT17(-((byte)(bVar35 + 0xbf) < 0x1a) & 0x20U | bVar35,
                                 CONCAT16(-((byte)(bVar34 + 0xbf) < 0x1a) & 0x20U | bVar34,
                                          CONCAT15(-((byte)(bVar33 + 0xbf) < 0x1a) & 0x20U | bVar33,
                                                   CONCAT14(-((byte)(bVar32 + 0xbf) < 0x1a) & 0x20U | bVar32,
                                                            CONCAT13(-((byte)(bVar31 + 0xbf) < 0x1a) & 0x20U | bVar31,
                                                                     CONCAT12(-((byte)(bVar30 + 0xbf) < 0x1a) & 0x20U | bVar30,
                                                                              CONCAT11(-((byte)(bVar29 + 0xbf) < 0x1a) & 0x20U | bVar29,
                                                                                       -((byte)((byte)lVar16 + 0xbf) < 0x1a) & 0x20U | (byte)lVar16)))))));
          plVar17[-1] = CONCAT17(-((byte)(bVar62 + 0xbf) < 0x1a) & 0x20U | bVar62,
                                 CONCAT16(-((byte)(bVar61 + 0xbf) < 0x1a) & 0x20U | bVar61,
                                          CONCAT15(-((byte)(bVar60 + 0xbf) < 0x1a) & 0x20U | bVar60,
                                                   CONCAT14(-((byte)(bVar59 + 0xbf) < 0x1a) & 0x20U | bVar59,
                                                            CONCAT13(-((byte)(bVar58 + 0xbf) < 0x1a) & 0x20U | bVar58,
                                                                     CONCAT12(-((byte)(bVar57 + 0xbf) < 0x1a) & 0x20U | bVar57,
                                                                              CONCAT11(-((byte)(bVar56 + 0xbf) < 0x1a) & 0x20U | bVar56,
                                                                                       -((byte)((byte)lVar55 + 0xbf) < 0x1a) & 0x20U | (byte)lVar55)))))));
          plVar17[-2] = CONCAT17(-((byte)(bVar53 + 0xbf) < 0x1a) & 0x20U | bVar53,
                                 CONCAT16(-((byte)(bVar52 + 0xbf) < 0x1a) & 0x20U | bVar52,
                                          CONCAT15(-((byte)(bVar51 + 0xbf) < 0x1a) & 0x20U | bVar51,
                                                   CONCAT14(-((byte)(bVar50 + 0xbf) < 0x1a) & 0x20U | bVar50,
                                                            CONCAT13(-((byte)(bVar49 + 0xbf) < 0x1a) & 0x20U | bVar49,
                                                                     CONCAT12(-((byte)(bVar48 + 0xbf) < 0x1a) & 0x20U | bVar48,
                                                                              CONCAT11(-((byte)(bVar47 + 0xbf) < 0x1a) & 0x20U | bVar47,
                                                                                       -((byte)((byte)lVar46 + 0xbf) < 0x1a) & 0x20U | (byte)lVar46)))))));
          plVar17[1] = CONCAT17(-((byte)(bVar80 + 0xbf) < 0x1a) & 0x20U | bVar80,
                                CONCAT16(-((byte)(bVar79 + 0xbf) < 0x1a) & 0x20U | bVar79,
                                         CONCAT15(-((byte)(bVar78 + 0xbf) < 0x1a) & 0x20U | bVar78,
                                                  CONCAT14(-((byte)(bVar77 + 0xbf) < 0x1a) & 0x20U | bVar77,
                                                           CONCAT13(-((byte)(bVar76 + 0xbf) < 0x1a) & 0x20U | bVar76,
                                                                    CONCAT12(-((byte)(bVar75 + 0xbf) < 0x1a) & 0x20U | bVar75,
                                                                             CONCAT11(-((byte)(bVar74 + 0xbf) < 0x1a) & 0x20U | bVar74,
                                                                                      -((byte)((byte)lVar73 + 0xbf) < 0x1a) & 0x20U | (byte)lVar73)))))));
          *plVar17 = CONCAT17(-((byte)(bVar71 + 0xbf) < 0x1a) & 0x20U | bVar71,
                              CONCAT16(-((byte)(bVar70 + 0xbf) < 0x1a) & 0x20U | bVar70,
                                       CONCAT15(-((byte)(bVar69 + 0xbf) < 0x1a) & 0x20U | bVar69,
                                                CONCAT14(-((byte)(bVar68 + 0xbf) < 0x1a) & 0x20U | bVar68,
                                                         CONCAT13(-((byte)(bVar67 + 0xbf) < 0x1a) & 0x20U | bVar67,
                                                                  CONCAT12(-((byte)(bVar66 + 0xbf) < 0x1a) & 0x20U | bVar66,
                                                                           CONCAT11(-((byte)(bVar65 + 0xbf) < 0x1a) & 0x20U | bVar65,
                                                                                    -((byte)((byte)lVar64 + 0xbf) < 0x1a) & 0x20U | (byte)lVar64)))))));
          plVar17[3] = CONCAT17(-((byte)(bVar98 + 0xbf) < 0x1a) & 0x20U | bVar98,
                                CONCAT16(-((byte)(bVar97 + 0xbf) < 0x1a) & 0x20U | bVar97,
                                         CONCAT15(-((byte)(bVar96 + 0xbf) < 0x1a) & 0x20U | bVar96,
                                                  CONCAT14(-((byte)(bVar95 + 0xbf) < 0x1a) & 0x20U | bVar95,
                                                           CONCAT13(-((byte)(bVar94 + 0xbf) < 0x1a) & 0x20U | bVar94,
                                                                    CONCAT12(-((byte)(bVar93 + 0xbf) < 0x1a) & 0x20U | bVar93,
                                                                             CONCAT11(-((byte)(bVar92 + 0xbf) < 0x1a) & 0x20U | bVar92,
                                                                                      -((byte)((byte)lVar91 + 0xbf) < 0x1a) & 0x20U | (byte)lVar91)))))));
          plVar17[2] = CONCAT17(-((byte)(bVar89 + 0xbf) < 0x1a) & 0x20U | bVar89,
                                CONCAT16(-((byte)(bVar88 + 0xbf) < 0x1a) & 0x20U | bVar88,
                                         CONCAT15(-((byte)(bVar87 + 0xbf) < 0x1a) & 0x20U | bVar87,
                                                  CONCAT14(-((byte)(bVar86 + 0xbf) < 0x1a) & 0x20U | bVar86,
                                                           CONCAT13(-((byte)(bVar85 + 0xbf) < 0x1a) & 0x20U | bVar85,
                                                                    CONCAT12(-((byte)(bVar84 + 0xbf) < 0x1a) & 0x20U | bVar84,
                                                                             CONCAT11(-((byte)(bVar83 + 0xbf) < 0x1a) & 0x20U | bVar83,
                                                                                      -((byte)((byte)lVar82 + 0xbf) < 0x1a) & 0x20U | (byte)lVar82)))))));
          uVar10 = uVar10 - 0x40;
          plVar17 = plVar17 + 8;
        } while (uVar10 != 0);
        if (param_9 != uVar12) {
          uVar10 = uVar12;
          if ((param_9 & 0x38) == 0) goto LAB_10078f230;
          goto LAB_10078f1ec;
        }
      }
      sVar26 = param_9;
      if (8 < param_9) {
        ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,"no spacetaskkillre-entrytakeoverrollback",8);
        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
        if (((ulong)local_118 & 1) == 0) {
          if (param_9 == 9) {
            if (*plVar9 != 0x6c7566206b736964 || (char)plVar9[1] != 'l') goto LAB_10078f438;
            param_9 = 9;
          }
          else {
            ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,&DAT_1013ab00f,9);
            <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
            if (((ulong)local_118 & 1) == 0) goto LAB_10078f438;
          }
        }
        goto LAB_10078fa18;
      }
      if (param_9 != 8) {
        if (6 < param_9) goto LAB_10078f438;
        if (param_9 == 6) {
          if ((int)*plVar9 == 0x736f6e65 && *(short *)((long)plVar9 + 4) == 0x6370) {
            param_9 = 6;
            goto LAB_10078fa18;
          }
          goto LAB_10078f584;
        }
        if (4 < param_9) goto LAB_10078f584;
        if ((param_9 != 4) || (sVar26 = 4, (int)*plVar9 != 0x79737562)) goto LAB_10078ede4;
        pcVar22 = &UNK_1013ab250;
        sVar21 = 7;
        sVar26 = 4;
        goto LAB_10078fa1c;
      }
      if (*plVar9 == 0x6563617073206f6e) {
        param_9 = 8;
        goto LAB_10078fa18;
      }
LAB_10078f438:
      ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,&DAT_1013ab018,6);
      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
      if (((ulong)local_118 & 1) == 0) {
        if (param_9 < 0x13) {
          if (param_9 == 0x12) {
            if ((*plVar9 == 0x6369666675736e69 && plVar9[1] == 0x61707320746e6569) && (short)plVar9[2] == 0x6563) {
              param_9 = 0x12;
              goto LAB_10078fa18;
            }
LAB_10078f5f8:
            ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,&DAT_1013ab030,0xe);
            <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
            if (((ulong)local_118 & 1) != 0) goto LAB_10078fa18;
            if (param_9 < 0x11) {
              if ((param_9 == 0x10) && (*plVar9 == 0x20656761726f7473 && plVar9[1] == 0x7974696361706163)) {
                pcVar22 = 
                "storage_capacityimage generationconnection resetproxy_last_errorlast_codex_routelaunch_requested/helpers/codex (crashpad_handlerPngEncodingErrorOperationAborted\x01"
                ;
                sVar21 = 0x10;
                sVar26 = 0x10;
                goto LAB_10078fa1c;
              }
            }
            else {
              ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,&DAT_1013af120,0x10);
              <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
              if (((ulong)local_118 & 1) != 0) goto LAB_10078fa18;
            }
          }
          else {
            if (0xe < param_9) goto LAB_10078f5f8;
            if (param_9 == 0xe) {
              if (*plVar9 != 0x6163206574697277 || *(long *)((long)plVar9 + 6) != 0x7974696361706163) goto LAB_10078f9b0;
              param_9 = 0xe;
              goto LAB_10078fa18;
            }
            if (param_9 < 0xd) {
              if ((param_9 != 0xc) ||
                 ((*param_8 != -0x471b4b6816455619 || (int)param_8[1] != -0x4c491773 && (*param_8 != -0x481a6764187e5c19 || (int)param_8[1] != -0x5e44194e))))
              goto LAB_10078f584;
              param_9 = 0xc;
              goto LAB_10078fa18;
            }
          }
LAB_10078f9b0:
          ::core::str::pattern::StrSearcher::new(&local_d0,param_8,param_9,&DAT_1013ab03e,0xc);
          <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
          if (((ulong)local_118 & 1) == 0) {
            ::core::str::pattern::StrSearcher::new(&local_d0,param_8,param_9,&DAT_1013ab04a,0xc);
            <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
            if (((ulong)local_118 & 1) == 0) {
              if (param_9 < 0x13) {
                if ((param_9 == 0x12) && ((*plVar9 == 0x6573616261746164 && plVar9[1] == 0x6b636f6c20736920) && (short)plVar9[2] == 0x6465)) {
                  pcVar22 = &UNK_1013ab250;
                  sVar26 = 0x12;
                  sVar21 = 7;
                  goto LAB_10078fa1c;
                }
LAB_10078f584:
                ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,&DAT_1013af314,4);
                <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
                if (((ulong)local_118 & 1) == 0) {
                  if (param_9 < 10) {
                    if (param_9 == 9) {
                      if (*plVar9 != 0x65646c6f68206264 || (char)plVar9[1] != 'r') {
LAB_10078fe38:
                        ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,&DAT_1013ab071,7);
                        <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
                        if (((ulong)local_118 & 1) != 0) goto LAB_10078fe68;
                        if (param_9 < 0x10) {
                          if (param_9 == 0xf) {
                            if (*param_8 != -0x451a5172194f6a1a || *(long *)((long)param_8 + 7) != -0x7e6b16545d176c46) {
LAB_10079007c:
                              ::core::str::pattern::StrSearcher::new(&local_d0,param_8,param_9,&DAT_1013ab087,0xc);
                              <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
                              if (((uint)local_118 & 1) == 0) goto LAB_10078ede4;
                            }
                          }
                          else {
                            if (0xc < param_9) goto LAB_10079007c;
                            if ((param_9 != 0xc) || (*param_8 != -0x451a5172194f6a1a || (int)param_8[1] != -0x7e6b166d)) goto LAB_10078ede4;
                          }
                        }
                        else {
                          ::core::str::pattern::StrSearcher::new(&local_d0,param_8,param_9,&DAT_1013ab078,0xf);
                          <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
                          if (((ulong)local_118 & 1) == 0) goto LAB_10079007c;
                        }
                        sVar21 = 7;
                        pcVar22 = &UNK_1013ab250;
                        goto LAB_10078f164;
                      }
                      pcVar22 = &UNK_1013ab250;
                      sVar26 = 9;
                      sVar21 = 7;
                    }
                    else {
                      if (7 < param_9) goto LAB_10078fe38;
                      if ((param_9 != 7) || (sVar21 = 7, sVar26 = sVar21, (int)*plVar9 != 0x646c6568 || *(int *)((long)plVar9 + 3) != 0x79622064))
                      goto LAB_10078ede4;
                      pcVar22 = &UNK_1013ab250;
                      sVar26 = 7;
                    }
                    goto LAB_10078fa1c;
                  }
                  ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,&DAT_1013ab068,9);
                  <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
                  if (((ulong)local_118 & 1) == 0) goto LAB_10078fe38;
                }
              }
              else {
                ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,&DAT_1013ab056,0x12);
                <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
                if (((ulong)local_118 & 1) == 0) goto LAB_10078f584;
              }
LAB_10078fe68:
              pcVar22 = &UNK_1013ab250;
              sVar21 = 7;
              goto LAB_10078fa1c;
            }
          }
        }
        else {
          ::core::str::pattern::StrSearcher::new(&local_d0,plVar9,param_9,&DAT_1013ab01e,0x12);
          <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_118,&local_d0);
          if (((ulong)local_118 & 1) == 0) goto LAB_10078f5f8;
        }
      }
LAB_10078fa18:
      pcVar22 = 
      "storage_capacityimage generationconnection resetproxy_last_errorlast_codex_routelaunch_requested/helpers/codex (crashpad_handlerPngEncodingErrorOperationAborted\x01"
      ;
      sVar21 = 0x10;
      sVar26 = param_9;
LAB_10078fa1c:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar9,sVar26,1);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar20 = 1;
    param_8 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar21,1);
    if (param_8 == (long *)0x0) {
LAB_10078ecf4:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(uVar20,sVar21);
    }
    _memcpy(param_8,pcVar22,sVar21);
  }
  local_7f[0] = 1;
  local_b0 = local_168;
  lStack_88 = (uVar23 + (long)(int)((((uVar3 >> 4 & 0x1ff) + iVar19) - iVar18 / 100) + (iVar18 * 0x5b5 >> 2) + (iVar18 / 100 >> 2) + -0xaf93b) * 0x15180) * 1000
              + ((ulong)ppppppplStack_128 & 0xffffffff) / 1000000;
  local_d0 = param_5;
  pvStack_c8 = pvVar7;
  local_c0 = param_5;
  sStack_b8 = sVar24;
  sStack_a8 = sVar24;
  local_a0 = sVar21;
  plStack_98 = param_8;
  local_90 = sVar21;
  local_80 = param_3;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  ppppppplStack_110 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x80,1);
  if (ppppppplStack_110 == (long *******)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x80);
  }
  local_118 = (long *******)&DAT_00000080;
  local_148 = (long *******)&local_118;
  *(undefined1 *)ppppppplStack_110 = 0x7b;
  local_108 = (long *******)0x1;
  local_130 = (long *******)CONCAT62(local_130._2_6_,0x100);
  ppppppplStack_128 = (long *******)&local_148;
  ppppppplVar11 =
       (long *******)
       serde_core::ser::SerializeMap::serialize_entry
                 (&local_130,"schemaVersiontransition completed or already stableclearedcleanup_sync_unconfirmedrouter-events.jsonlfailed",0xd,local_7f);
  if (ppppppplVar11 == (long *******)0x0) {
    if (((ulong)local_130 & 1) != 0) {
LAB_10078fb5c:
      ppppppplVar11 = (long *******)serde_json::ser::invalid_raw_value();
      goto LAB_10078fb88;
    }
    ppppppplVar11 = (long *******)serde_core::ser::SerializeMap::serialize_entry(&local_130,&_anon_6408540cdffbe318d49f52812e99783f_452,6,&local_80);
    if (ppppppplVar11 != (long *******)0x0) goto LAB_10078fb88;
    if (((ulong)local_130 & 1) != 0) goto LAB_10078fb5c;
    ppppppplVar11 = (long *******)serde_core::ser::SerializeMap::serialize_entry(&local_130,&_anon_6408540cdffbe318d49f52812e99783f_453,5,&local_d0);
    if (ppppppplVar11 != (long *******)0x0) goto LAB_10078fb88;
    if (((ulong)local_130 & 1) != 0) goto LAB_10078fb5c;
    ppppppplVar11 = (long *******)serde_core::ser::SerializeMap::serialize_entry(&local_130,&_anon_6408540cdffbe318d49f52812e99783f_454,0xc,&lStack_88);
    if (ppppppplVar11 != (long *******)0x0) goto LAB_10078fb88;
    if (sVar24 != 0x8000000000000000) {
      if (((ulong)local_130 & 1) != 0) goto LAB_10078fb5c;
      ppppppplVar11 = (long *******)serde_core::ser::SerializeMap::serialize_entry(&local_130,&_anon_6408540cdffbe318d49f52812e99783f_249,6,&sStack_b8);
      if (ppppppplVar11 != (long *******)0x0) goto LAB_10078fb88;
    }
    if (((ulong)local_130 & 1) != 0) goto LAB_10078fb5c;
    ppppppplVar11 = (long *******)serde_core::ser::SerializeMap::serialize_entry(&local_130,&_anon_6408540cdffbe318d49f52812e99783f_455,6,&local_a0);
    if (ppppppplVar11 != (long *******)0x0) goto LAB_10078fb88;
    if ((((ulong)local_130 & 1) == 0) && (local_130._1_1_ != '\0')) {
      alloc::vec::Vec<T,A>::extend_from_slice(*ppppppplStack_128,"}false{string or mapan array of length 16",1);
    }
    ppppppplVar5 = local_108;
    ppppppplVar11 = ppppppplStack_110;
    ppppppplVar4 = local_118;
    if (local_118 == (long *******)&DAT_8000000000000000) goto LAB_10078fb9c;
    std::path::Path::_join(&local_160,param_1,param_2,"router-events.jsonlfailed",0x13);
    std::fs::read_to_string::inner(&local_148,local_158,uStack_150);
    ppppppplVar14 = local_148;
    ppppppplVar25 = local_140;
    if (local_148 == (long *******)&DAT_8000000000000000) {
LAB_10078fd90:
      local_148 = (long *******)0x0;
      local_140 = (long *******)&SUB_00000008;
      local_138 = (long *******)0x0;
      if (((ulong)ppppppplVar25 & 3) == 1) {
        uVar20 = *(undefined8 *)((long)ppppppplVar25 + -1);
        puVar27 = *(undefined8 **)((long)ppppppplVar25 + 7);
        if ((code *)*puVar27 != (code *)0x0) {
          (*(code *)*puVar27)(uVar20);
        }
        if (puVar27[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar20,puVar27[1],puVar27[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)ppppppplVar25 + -1),0x18,8);
        goto LAB_100790108;
      }
LAB_100790110:
      ppppppplVar14 = local_148;
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_148);
    }
    else {
      local_118 = (long *******)0x0;
      ppppppplStack_110 = local_138;
      local_108 = local_140;
      ppppppplStack_100 = local_138;
      local_f8 = 0;
      ppppppplStack_f0 = local_138;
      local_e8 = 0xa0000000a;
      local_e0 = 1;
      local_d8 = 0;
      <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_130,&local_118);
      if (ppppppplVar14 != (long *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_140,ppppppplVar14,1);
      }
      ppppppplVar25 = ppppppplStack_128;
      if (local_130 == (long *******)&DAT_8000000000000000) goto LAB_10078fd90;
      local_148 = local_130;
      local_140 = ppppppplStack_128;
      local_138 = local_120;
LAB_100790108:
      ppppppplVar14 = local_138;
      if (local_138 == local_148) goto LAB_100790110;
    }
    ppppppplVar25 = local_140 + (long)ppppppplVar14 * 3;
    *ppppppplVar25 = (long ******)ppppppplVar4;
    ppppppplVar25[1] = (long ******)ppppppplVar11;
    ppppppplVar25[2] = (long ******)ppppppplVar5;
    local_138 = (long *******)((long)ppppppplVar14 + 1);
    if (&DAT_000000c8 < local_138) {
      alloc::vec::Vec<T,A>::drain(&local_118,&local_148,0,(undefined1 *)((long)ppppppplVar14 + -199));
      <alloc::vec::drain::Drain<T,A>as_core::ops::drop::Drop>::drop(&local_118);
    }
    alloc::str::join_generic_copy(&local_118,local_140,local_138,&_anon_6408540cdffbe318d49f52812e99783f_174,1);
    ppppppplVar11 = local_108;
    ppppppplStack_128 = ppppppplStack_110;
    local_130 = local_118;
    local_120 = local_108;
    if (local_118 == local_108) {
      alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_130,local_108,1,1,1);
    }
    ppppppplVar4 = ppppppplStack_128;
    *(undefined1 *)((long)ppppppplStack_128 + (long)local_120) = 10;
    auVar99 = atomic_write::write_atomic_report(local_158,uStack_150,ppppppplStack_128,(undefined1 *)((long)ppppppplVar11 + 1));
    uVar23 = auVar99._8_8_;
    if ((auVar99._0_8_ & 1) == 0) {
      atomic_write::Publication::acknowledge(uVar23);
      uVar23 = 0;
    }
    if ((uVar23 & 3) == 1) {
      uVar20 = *(undefined8 *)(uVar23 - 1);
      puVar27 = *(undefined8 **)(uVar23 + 7);
      if ((code *)*puVar27 != (code *)0x0) {
        (*(code *)*puVar27)(uVar20);
      }
      if (puVar27[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar20,puVar27[1],puVar27[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(uVar23 - 1),0x18,8);
    }
    if (local_130 != (long *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar4,local_130,1);
    }
    <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_148);
    if (local_148 != (long *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_140,(long)local_148 * 0x18,8);
    }
    if (local_160 == 0) goto LAB_10078fbdc;
    uVar20 = 1;
  }
  else {
LAB_10078fb88:
    if (local_118 != (long *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_110,local_118,1);
    }
LAB_10078fb9c:
    if (*ppppppplVar11 == (long ******)0x1) {
      ::core::ptr::drop_in_place<std::io::error::Error>(ppppppplVar11 + 1);
    }
    else if ((*ppppppplVar11 == (long ******)0x0) && (ppppppplVar11[2] != (long ******)0x0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar11[1],ppppppplVar11[2],1);
    }
    local_160 = 0x28;
    uVar20 = 8;
    local_158 = ppppppplVar11;
  }
  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_158,local_160,uVar20);
LAB_10078fbdc:
  if (param_5 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar7,param_5,1);
  }
  if (sVar24 != 0x8000000000000000) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_168,sVar24,1);
  }
  if (sVar21 != 0x8000000000000000) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(param_8,sVar21,1);
  }
  return;
}

