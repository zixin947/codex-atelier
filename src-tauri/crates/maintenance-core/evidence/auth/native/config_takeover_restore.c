/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* restore @ 1001fd5cc requested 1001fd5cc */

/* codexmate_lib::core::relay::config_takeover::restore */

void codexmate_lib::core::relay::config_takeover::restore(undefined8 *param_1,long param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  void *pvVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  bool bVar6;
  undefined1 *puVar7;
  bool bVar8;
  int iVar9;
  ulong uVar10;
  char *pcVar11;
  long extraout_x1;
  undefined **ppuVar12;
  undefined8 *puVar13;
  long lVar14;
  long lVar15;
  long lVar16;
  ulong uVar17;
  undefined8 uVar18;
  long lVar19;
  undefined1 *puVar20;
  undefined1 *puVar21;
  long lVar22;
  undefined1 *puVar23;
  undefined8 uVar24;
  undefined1 *puVar25;
  long lVar26;
  ulong *puVar27;
  undefined1 *puVar28;
  long *plVar29;
  undefined1 *puVar30;
  long lVar31;
  undefined1 *puVar32;
  undefined1 auVar33 [16];
  undefined8 local_2d0;
  undefined8 uStack_2c8;
  undefined8 local_2c0;
  long local_2b8;
  long local_2b0;
  long lStack_2a8;
  long local_2a0;
  long local_298;
  long local_290;
  undefined1 *puStack_288;
  undefined1 *puStack_280;
  undefined1 *local_278;
  undefined1 *local_270;
  undefined1 *puStack_268;
  undefined1 *puStack_260;
  undefined1 *puStack_258;
  undefined1 *local_250;
  long lStack_248;
  undefined8 local_230;
  undefined8 uStack_228;
  undefined8 local_220;
  undefined1 *local_210;
  undefined1 *puStack_208;
  undefined1 *local_200;
  undefined1 *puStack_1f8;
  undefined1 *local_1f0;
  long lStack_1e8;
  undefined8 local_1d0;
  undefined8 uStack_1c8;
  undefined8 local_1c0;
  undefined1 *local_1b0;
  undefined1 *puStack_1a8;
  undefined1 *local_1a0;
  undefined1 *local_190;
  long lStack_188;
  undefined1 *local_180;
  undefined1 *local_170;
  undefined1 *puStack_168;
  undefined1 *local_160;
  long local_158;
  long local_150;
  long local_148;
  long local_140;
  long local_138;
  long local_130;
  undefined1 *local_128;
  undefined1 *local_120;
  undefined1 *local_118;
  undefined8 local_110;
  undefined1 *puStack_108;
  undefined1 *puStack_100;
  undefined1 *puStack_f8;
  undefined1 *puStack_f0;
  undefined1 *local_e8;
  long lStack_e0;
  undefined1 local_d8;
  undefined7 uStack_d7;
  byte bStack_d0;
  byte bStack_cf;
  undefined6 uStack_ce;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined1 *local_a8;
  long local_a0;
  undefined1 *local_98;
  undefined1 *local_90;
  undefined1 *puStack_88;
  undefined1 *local_80;
  undefined1 local_71 [17];

  quarantine_damaged_backups(&local_110,param_2);
  puStack_208 = puStack_100;
  local_210 = puStack_108;
  local_200 = puStack_f8;
  if (local_110 != &DAT_0000000b) {
    param_1[6] = local_e8;
    param_1[5] = puStack_f0;
    param_1[8] = CONCAT71(uStack_d7,local_d8);
    param_1[7] = lStack_e0;
    param_1[10] = local_c8;
    param_1[9] = CONCAT62(uStack_ce,CONCAT11(bStack_cf,bStack_d0));
    param_1[0xc] = local_b8;
    param_1[0xb] = uStack_c0;
    param_1[3] = puStack_100;
    param_1[2] = puStack_108;
    param_1[4] = puStack_f8;
    *param_1 = 1;
    param_1[1] = local_110;
    return;
  }
  puStack_280 = puStack_100;
  puStack_288 = puStack_108;
  local_278 = puStack_f8;
  local_2d0 = 0;
  uStack_2c8 = 8;
  local_2c0 = 0;
  local_2b8 = 0;
  local_2b0 = 8;
  lStack_2a8 = 0;
  local_2a0 = 0;
  local_298 = 8;
  local_290 = 0;
  uVar18 = *(undefined8 *)(param_2 + 0x248);
  uVar24 = *(undefined8 *)(param_2 + 0x250);
  read_backup(&local_110,uVar18,uVar24);
  lStack_1e8 = lStack_e0;
  local_1f0 = local_e8;
  uStack_1c8 = uStack_c0;
  local_1d0 = local_c8;
  local_1c0 = local_b8;
  puStack_208 = puStack_100;
  local_210 = puStack_108;
  puStack_1f8 = puStack_f0;
  local_200 = puStack_f8;
  if (local_110 != &DAT_0000000b) {
    param_1[7] = lStack_e0;
    param_1[6] = local_e8;
    param_1[9] = CONCAT62(uStack_ce,CONCAT11(bStack_cf,bStack_d0));
    param_1[8] = CONCAT71(uStack_d7,local_d8);
    param_1[0xb] = uStack_c0;
    param_1[10] = local_c8;
    param_1[0xc] = local_b8;
    param_1[3] = puStack_100;
    param_1[2] = puStack_108;
    param_1[5] = puStack_f0;
    param_1[4] = puStack_f8;
    *param_1 = 1;
    param_1[1] = local_110;
    goto LAB_1001fdb4c;
  }
  lStack_248 = lStack_e0;
  local_250 = local_e8;
  uStack_228 = uStack_c0;
  local_230 = local_c8;
  local_220 = local_b8;
  puStack_268 = puStack_100;
  local_270 = puStack_108;
  puStack_258 = puStack_f0;
  puStack_260 = puStack_f8;
  read_legacy_backup_lines(&local_110,uVar18,uVar24);
  puVar32 = local_270;
  puStack_208 = puStack_100;
  local_210 = puStack_108;
  local_200 = puStack_f8;
  if (local_110 != &DAT_0000000b) {
    param_1[6] = local_e8;
    param_1[5] = puStack_f0;
    param_1[8] = CONCAT71(uStack_d7,local_d8);
    param_1[7] = lStack_e0;
    param_1[10] = local_c8;
    param_1[9] = CONCAT62(uStack_ce,CONCAT11(bStack_cf,bStack_d0));
    param_1[0xc] = local_b8;
    param_1[0xb] = uStack_c0;
    param_1[3] = puStack_100;
    param_1[2] = puStack_108;
    param_1[4] = puStack_f8;
    *param_1 = 1;
    param_1[1] = local_110;
    if (local_270 != &DAT_8000000000000000) {
      ::core::ptr::drop_in_place<codexmate_lib::core::relay::config_takeover::TakeoverBackup>(&local_270);
    }
    goto LAB_1001fdb4c;
  }
  puStack_1a8 = puStack_100;
  local_1b0 = puStack_108;
  local_1a0 = puStack_f8;
  if ((local_270 == &DAT_8000000000000000) && (puStack_f8 == (undefined1 *)0x0)) {
    bVar6 = false;
    param_1[6] = lStack_2a8;
    param_1[5] = local_2b0;
    param_1[8] = local_298;
    param_1[7] = local_2a0;
    param_1[10] = puStack_288;
    param_1[9] = local_290;
    param_1[0xc] = local_278;
    param_1[0xb] = puStack_280;
    param_1[2] = uStack_2c8;
    param_1[1] = local_2d0;
    param_1[4] = local_2b8;
    param_1[3] = local_2c0;
    *param_1 = 0;
  }
  else {
    uVar1 = *(undefined8 *)(param_2 + 0x38);
    uVar4 = *(undefined8 *)(param_2 + 0x40);
    read_optional_text(&local_110,uVar1,uVar4);
    puVar21 = puStack_f8;
    puVar7 = puStack_100;
    puVar23 = puStack_108;
    if (local_110 == &DAT_0000000b) {
      bVar6 = false;
      bVar8 = false;
      local_90 = (undefined1 *)0x0;
      puStack_88 = (undefined1 *)0x8;
      local_80 = (undefined1 *)0x0;
      local_a8 = (undefined1 *)CONCAT71(local_a8._1_7_,2);
      local_110 = (undefined1 *)0x0;
      puStack_108 = puStack_f8;
      puStack_f0 = (undefined1 *)0x0;
      local_e8 = puStack_f8;
      lStack_e0 = 0xa0000000a;
      local_d8 = 1;
      bStack_d0 = 0;
      bStack_cf = 0;
      do {
        puVar30 = puStack_100;
        <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(&local_210,&puStack_100);
        if ((int)local_210 == 1) {
          lVar31 = (long)local_200 - (long)local_110;
          puVar20 = local_200;
        }
        else {
          if (((bStack_cf & 1) != 0) || ((bStack_cf = 1, (bStack_d0 & 1) == 0 && (puStack_108 == local_110)))) break;
          lVar31 = (long)puStack_108 - (long)local_110;
          puVar20 = local_110;
          puVar30 = puStack_100;
        }
        puVar30 = local_110 + (long)puVar30;
        lVar26 = lVar31;
        if ((lVar31 != 0) && (puVar30[lVar31 + -1] == '\n')) {
          if (lVar31 + -1 == 0) {
            lVar26 = 0;
          }
          else {
            lVar26 = lVar31 + -1;
            if (puVar30[lVar31 + -2] == '\r') {
              lVar26 = lVar31 + -2;
            }
          }
        }
        local_110 = puVar20;
        auVar33 = ::core::str::<impl_str>::trim_matches(puVar30,lVar26);
        lVar31 = auVar33._8_8_;
        plVar29 = auVar33._0_8_;
        uVar10 = managed_blocks::TomlLexicalState::line_is_structural(&local_a8,puVar30,lVar26);
        if ((uVar10 & 1) == 0) {
          if (bVar6) goto LAB_1001fd7c0;
LAB_1001fdb1c:
          bVar6 = false;
        }
        else {
          if (lVar31 < 0x37) {
            if (lVar31 == 0x1e) {
              if (((*plVar29 != 0x6961203c3c3c2023 || plVar29[1] != 0x6c65722d696d616d) || plVar29[2] != 0x67616e616d207961) ||
                  *(long *)((long)plVar29 + 0x16) != 0x646e652064656761) goto LAB_1001fda6c;
            }
            else if (lVar31 == 0x24) {
              if ((((*plVar29 != 0x6961203c3c3c2023 || plVar29[1] != 0x6c65722d696d616d) || plVar29[2] != 0x67616e616d207961) ||
                  plVar29[3] != 0x2820646e65206465) || (int)plVar29[4] != 0x29706f74) goto LAB_1001fda6c;
            }
            else if ((lVar31 != 0x27) ||
                    ((((*plVar29 != 0x6961203c3c3c2023 || plVar29[1] != 0x6c65722d696d616d) || plVar29[2] != 0x7865646f63207961) ||
                     plVar29[3] != 0x20726574756f722d) || *(long *)((long)plVar29 + 0x1f) != 0x646e6520706f7420)) {
LAB_1001fda6c:
              if (bVar6) goto LAB_1001fd7c0;
              if (((lVar31 == 0) || ((char)*plVar29 != '[')) || (*(char *)((long)plVar29 + lVar31 + -1) != ']')) {
                if (!bVar8) {
                  top_level_assignment_key(&local_210,plVar29,lVar31);
                  puVar30 = local_80;
                  if (local_210 != &DAT_8000000000000000) {
                    if (local_80 == local_90) {
                      alloc::raw_vec::RawVec<T,A>::grow_one(&local_90);
                    }
                    puVar13 = (undefined8 *)((long)puStack_88 + (long)puVar30 * 0x18);
                    puVar13[1] = puStack_208;
                    *puVar13 = local_210;
                    puVar13[2] = local_200;
                    local_80 = puVar30 + 1;
                  }
                  bVar8 = false;
                  goto LAB_1001fdb1c;
                }
              }
              else if ((lVar31 + -1 == 0) || (*(char *)((long)plVar29 + 1) < -0x40)) {
                    /* WARNING: Subroutine does not return */
                ::core::str::slice_error_fail(plVar29,lVar31,1,lVar31 + -1,&_anon_6408540cdffbe318d49f52812e99783f_417);
              }
              bVar6 = false;
              bVar8 = true;
              goto LAB_1001fd7c4;
            }
            goto LAB_1001fdb1c;
          }
          if (lVar31 == 0x40) {
            if (((((((*plVar29 != 0x6961203e3e3e2023 || plVar29[1] != 0x6c65722d696d616d) || plVar29[2] != 0x7865646f63207961) ||
                   plVar29[3] != 0x20726574756f722d) || plVar29[4] != 0x7261747320706f74) || plVar29[5] != 0x4f4e204f44282074) ||
                plVar29[6] != 0x4d20544944452054) || plVar29[7] != 0x29594c4c41554e41) goto LAB_1001fda6c;
          }
          else if (lVar31 == 0x3c) {
            if (((((((*plVar29 != 0x6961203e3e3e2023 || plVar29[1] != 0x6c65722d696d616d) || plVar29[2] != 0x67616e616d207961) ||
                   plVar29[3] != 0x7472617473206465) || plVar29[4] != 0x44202c706f742820) || plVar29[5] != 0x444520544f4e204f) ||
                plVar29[6] != 0x41554e414d205449) || (int)plVar29[7] != 0x29594c4c) goto LAB_1001fda6c;
          }
          else if ((lVar31 != 0x37) ||
                  ((((((*plVar29 != 0x6961203e3e3e2023 || plVar29[1] != 0x6c65722d696d616d) || plVar29[2] != 0x67616e616d207961) ||
                     plVar29[3] != 0x7472617473206465) || plVar29[4] != 0x544f4e204f442820) || plVar29[5] != 0x414d205449444520) ||
                   *(long *)((long)plVar29 + 0x2f) != 0x29594c4c41554e41)) goto LAB_1001fda6c;
LAB_1001fd7c0:
          bVar6 = true;
        }
LAB_1001fd7c4:
      } while ((bStack_cf & 1) == 0);
      bVar6 = false;
      lStack_188 = (long)puStack_88;
      local_190 = local_90;
      local_180 = local_80;
      local_90 = (undefined1 *)0x0;
      puStack_88 = &SUB_00000008;
      local_80 = (undefined1 *)0x0;
      local_a8 = (undefined1 *)CONCAT71(local_a8._1_7_,2);
      local_110 = (undefined1 *)0x0;
      puStack_108 = puStack_f8;
      puStack_f0 = (undefined1 *)0x0;
      local_e8 = puStack_f8;
      lStack_e0 = 0xa0000000a;
      local_d8 = 1;
      bStack_d0 = 0;
      bStack_cf = 0;
      puStack_100 = puVar7;
      do {
        puVar30 = puStack_100;
        <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(&local_210,&puStack_100);
        if ((int)local_210 == 1) {
          lVar31 = (long)local_200 - (long)local_110;
          puVar20 = local_200;
        }
        else {
          if (((bStack_cf & 1) != 0) || ((bStack_cf = 1, (bStack_d0 & 1) == 0 && (puStack_108 == local_110)))) break;
          lVar31 = (long)puStack_108 - (long)local_110;
          puVar20 = local_110;
          puVar30 = puStack_100;
        }
        puVar30 = local_110 + (long)puVar30;
        lVar26 = lVar31;
        if ((lVar31 != 0) && (puVar30[lVar31 + -1] == '\n')) {
          if (lVar31 + -1 == 0) {
            lVar26 = 0;
          }
          else {
            lVar26 = lVar31 + -1;
            if (puVar30[lVar31 + -2] == '\r') {
              lVar26 = lVar31 + -2;
            }
          }
        }
        local_110 = puVar20;
        auVar33 = ::core::str::<impl_str>::trim_matches(puVar30,lVar26);
        lVar31 = auVar33._8_8_;
        plVar29 = auVar33._0_8_;
        uVar10 = managed_blocks::TomlLexicalState::line_is_structural(&local_a8,puVar30,lVar26);
        if ((uVar10 & 1) == 0) {
          if (bVar6) goto LAB_1001fddb8;
        }
        else {
          bVar8 = false;
          if (lVar31 < 0x37) {
            if (lVar31 == 0x1e) {
              bVar8 = ((*plVar29 == 0x6961203c3c3c2023 && plVar29[1] == 0x6c65722d696d616d) && plVar29[2] == 0x67616e616d207961) &&
                      *(long *)((long)plVar29 + 0x16) == 0x646e652064656761;
            }
            else {
              if (lVar31 == 0x24) {
                lVar26 = *plVar29;
                lVar14 = plVar29[1];
                lVar22 = plVar29[2];
                lVar15 = plVar29[3];
                lVar19._0_1_ = 'a';
                lVar19._1_1_ = 'y';
                lVar19._2_1_ = ' ';
                lVar19._3_1_ = 'm';
                lVar19._4_1_ = 'a';
                lVar19._5_1_ = 'n';
                lVar19._6_1_ = 'a';
                lVar19._7_1_ = 'g';
                lVar16._0_1_ = 'e';
                lVar16._1_1_ = 'd';
                lVar16._2_1_ = ' ';
                lVar16._3_1_ = 'e';
                lVar16._4_1_ = 'n';
                lVar16._5_1_ = 'd';
                lVar16._6_1_ = ' ';
                lVar16._7_1_ = '(';
                uVar10 = (ulong)*(uint *)(plVar29 + 4);
                uVar17 = 0x29706f74;
              }
              else {
                if (lVar31 != 0x27) goto LAB_1001fde90;
                lVar26 = *plVar29;
                lVar14 = plVar29[1];
                lVar22 = plVar29[2];
                lVar15 = plVar29[3];
                lVar19._0_1_ = 'a';
                lVar19._1_1_ = 'y';
                lVar19._2_1_ = ' ';
                lVar19._3_1_ = 'c';
                lVar19._4_1_ = 'o';
                lVar19._5_1_ = 'd';
                lVar19._6_1_ = 'e';
                lVar19._7_1_ = 'x';
                lVar16._0_1_ = '-';
                lVar16._1_1_ = 'r';
                lVar16._2_1_ = 'o';
                lVar16._3_1_ = 'u';
                lVar16._4_1_ = 't';
                lVar16._5_1_ = 'e';
                lVar16._6_1_ = 'r';
                lVar16._7_1_ = ' ';
                uVar10 = *(ulong *)((long)plVar29 + 0x1f);
                uVar17._0_1_ = ' ';
                uVar17._1_1_ = 't';
                uVar17._2_1_ = 'o';
                uVar17._3_1_ = 'p';
                uVar17._4_1_ = ' ';
                uVar17._5_1_ = 'e';
                uVar17._6_1_ = 'n';
                uVar17._7_1_ = 'd';
              }
              bVar8 = false;
              if (((lVar26 == 0x6961203c3c3c2023 && lVar14 == 0x6c65722d696d616d) && lVar22 == lVar19) && lVar15 == lVar16) {
                bVar8 = uVar10 == uVar17;
              }
            }
LAB_1001fde90:
            if (bVar8 || bVar6) {
LAB_1001fde4c:
              bVar6 = (bool)((bVar8 ^ 1U) & bVar6);
              goto LAB_1001fdc44;
            }
          }
          else {
            if (lVar31 == 0x40) {
              if (((((((*plVar29 == 0x6961203e3e3e2023 && plVar29[1] == 0x6c65722d696d616d) && plVar29[2] == 0x7865646f63207961) &&
                     plVar29[3] == 0x20726574756f722d) && plVar29[4] == 0x7261747320706f74) && plVar29[5] == 0x4f4e204f44282074) &&
                  plVar29[6] == 0x4d20544944452054) && plVar29[7] == 0x29594c4c41554e41) {
LAB_1001fddb8:
                bVar6 = true;
                goto LAB_1001fdc44;
              }
            }
            else if (lVar31 == 0x3c) {
              if (((((((*plVar29 == 0x6961203e3e3e2023 && plVar29[1] == 0x6c65722d696d616d) && plVar29[2] == 0x67616e616d207961) &&
                     plVar29[3] == 0x7472617473206465) && plVar29[4] == 0x44202c706f742820) && plVar29[5] == 0x444520544f4e204f) &&
                  plVar29[6] == 0x41554e414d205449) && (int)plVar29[7] == 0x29594c4c) goto LAB_1001fddb8;
            }
            else {
              if (lVar31 != 0x37) goto LAB_1001fde90;
              if ((((((*plVar29 == 0x6961203e3e3e2023 && plVar29[1] == 0x6c65722d696d616d) && plVar29[2] == 0x67616e616d207961) &&
                    plVar29[3] == 0x7472617473206465) && plVar29[4] == 0x544f4e204f442820) && plVar29[5] == 0x414d205449444520) &&
                  *(long *)((long)plVar29 + 0x2f) == 0x29594c4c41554e41) goto LAB_1001fddb8;
            }
            bVar8 = false;
            if (bVar6) goto LAB_1001fde4c;
          }
          if (((lVar31 != 0) && ((char)*plVar29 == '[')) && (*(char *)((long)plVar29 + lVar31 + -1) == ']')) {
            if ((lVar31 + -1 == 0) || (*(char *)((long)plVar29 + 1) < -0x40)) {
                    /* WARNING: Subroutine does not return */
              ::core::str::slice_error_fail(plVar29,lVar31,1,lVar31 + -1,&_anon_6408540cdffbe318d49f52812e99783f_417);
            }
            normalize_section_key(&local_210,(char *)((long)plVar29 + 1),lVar31 + -2);
            puVar30 = local_80;
            if (local_80 == local_90) {
              alloc::raw_vec::RawVec<T,A>::grow_one(&local_90);
            }
            puVar13 = (undefined8 *)(puStack_88 + (long)puVar30 * 0x18);
            puVar13[1] = puStack_208;
            *puVar13 = local_210;
            puVar13[2] = local_200;
            local_80 = puVar30 + 1;
          }
        }
LAB_1001fdc44:
      } while ((bStack_cf & 1) == 0);
      puVar30 = local_180;
      lVar31 = lStack_188;
      puStack_168 = puStack_88;
      local_170 = local_90;
      local_160 = local_80;
      local_158 = 0;
      local_150 = 8;
      local_148 = 0;
      local_140 = 0;
      local_138 = 8;
      local_130 = 0;
      if (puVar32 != &DAT_8000000000000000) {
        puVar13 = (undefined8 *)(puStack_268 + 0x10);
        puVar32 = puStack_268;
        for (lVar26 = (long)puStack_260 * 0x18; puVar20 = puStack_168, lVar26 != 0; lVar26 = lVar26 + -0x18) {
          iVar9 = is_aimami_owned_top_level_line(param_2,*(undefined8 *)(puVar32 + 8),*(undefined8 *)(puVar32 + 0x10));
          if (iVar9 == 0) {
            restore::__closure__(lVar31,puVar30,puVar13[-1],*puVar13,&local_158,&local_2d0);
          }
          puVar32 = puVar32 + 0x18;
          puVar13 = puVar13 + 3;
        }
        if (lStack_248 != 0) {
          puVar25 = local_250 + lStack_248 * 0x30;
          lVar26 = (long)local_160 * 0x18;
          puVar32 = local_250;
          do {
            uVar10 = is_aimami_owned_model_section(*(undefined8 *)(puVar32 + 8),*(undefined8 *)(puVar32 + 0x10));
            if ((uVar10 & 1) == 0) {
              local_110 = (undefined1 *)CONCAT71(local_110._1_7_,2);
              if (*(long *)(puVar32 + 0x28) != 0) {
                lVar22 = *(long *)(puVar32 + 0x28) * 0x18;
                puVar13 = (undefined8 *)(*(long *)(puVar32 + 0x20) + 0x10);
                do {
                  uVar2 = puVar13[-1];
                  uVar5 = *puVar13;
                  iVar9 = managed_blocks::TomlLexicalState::line_is_structural(&local_110,uVar2,uVar5);
                  if (iVar9 != 0) {
                    auVar33 = ::core::str::<impl_str>::trim_matches(uVar2,uVar5);
                    lVar19 = auVar33._8_8_;
                    pcVar11 = auVar33._0_8_;
                    if (((lVar19 != 0) && (*pcVar11 == '[')) && (pcVar11[lVar19 + -1] == ']')) {
                      if ((lVar19 + -1 == 0) || (pcVar11[1] < -0x40)) {
                    /* WARNING: Subroutine does not return */
                        ::core::str::slice_error_fail(pcVar11,lVar19,1,lVar19 + -1,&_anon_6408540cdffbe318d49f52812e99783f_417);
                      }
                      uVar10 = is_aimami_owned_model_section(pcVar11 + 1,lVar19 + -2);
                      if ((uVar10 & 1) != 0) goto LAB_1001fe04c;
                    }
                  }
                  puVar13 = puVar13 + 3;
                  lVar22 = lVar22 + -0x18;
                } while (lVar22 != 0);
              }
              uVar2 = *(undefined8 *)(puVar32 + 8);
              uVar5 = *(undefined8 *)(puVar32 + 0x10);
              lVar22 = lVar26;
              puVar28 = puVar20;
              do {
                if (lVar22 == 0) {
                  <alloc::string::String_as_core::clone::Clone>::clone(&local_110,puVar32);
                  lVar22 = lStack_2a8;
                  if (lStack_2a8 == local_2b8) {
                    alloc::raw_vec::RawVec<T,A>::grow_one(&local_2b8);
                  }
                  puVar13 = (undefined8 *)(local_2b0 + lVar22 * 0x18);
                  puVar13[1] = puStack_108;
                  *puVar13 = local_110;
                  puVar13[2] = puStack_100;
                  lStack_2a8 = lVar22 + 1;
                  <alloc::string::String_as_core::clone::Clone>::clone(&local_90,puVar32);
                  <alloc::vec::Vec<T,A>as_core::clone::Clone>::clone(&local_210,puVar32 + 0x18);
                  lVar22 = local_130;
                  puStack_108 = puStack_88;
                  local_110 = local_90;
                  puStack_100 = local_80;
                  puStack_f0 = puStack_208;
                  puStack_f8 = local_210;
                  local_e8 = local_200;
                  if (local_130 == local_140) {
                    alloc::raw_vec::RawVec<T,A>::grow_one(&local_140);
                  }
                  puVar13 = (undefined8 *)(local_138 + lVar22 * 0x30);
                  puVar13[1] = puStack_108;
                  *puVar13 = local_110;
                  puVar13[3] = puStack_f8;
                  puVar13[2] = puStack_100;
                  puVar13[5] = local_e8;
                  puVar13[4] = puStack_f0;
                  local_130 = lVar22 + 1;
                  goto LAB_1001fe04c;
                }
                uVar10 = section_overlaps(*(undefined8 *)(puVar28 + 8),*(undefined8 *)(puVar28 + 0x10),uVar2,uVar5);
                puVar28 = puVar28 + 0x18;
                lVar22 = lVar22 + -0x18;
              } while ((uVar10 & 1) == 0);
              <alloc::string::String_as_core::clone::Clone>::clone(&local_110,puVar32);
              lVar22 = local_290;
              if (local_290 == local_2a0) {
                alloc::raw_vec::RawVec<T,A>::grow_one(&local_2a0);
              }
              puVar13 = (undefined8 *)(local_298 + lVar22 * 0x18);
              puVar13[1] = puStack_108;
              *puVar13 = local_110;
              puVar13[2] = puStack_100;
              local_290 = lVar22 + 1;
            }
LAB_1001fe04c:
            puVar32 = puVar32 + 0x30;
          } while (puVar32 != puVar25);
        }
      }
      puVar13 = (undefined8 *)(puStack_1a8 + 0x10);
      puVar32 = puStack_1a8;
      for (lVar26 = (long)local_1a0 * 0x18; lVar15 = local_130, lVar22 = local_138, lVar19 = local_148, lVar14 = local_150, lVar26 != 0; lVar26 = lVar26 + -0x18
          ) {
        iVar9 = is_aimami_owned_top_level_line(param_2,*(undefined8 *)(puVar32 + 8),*(undefined8 *)(puVar32 + 0x10));
        if (iVar9 == 0) {
          restore::__closure__(lVar31,puVar30,puVar13[-1],*puVar13,&local_158,&local_2d0);
        }
        puVar32 = puVar32 + 0x18;
        puVar13 = puVar13 + 3;
      }
      if (local_130 == 0 && local_148 == 0) {
        alloc::raw_vec::RawVecInner<A>::try_allocate_in(&local_110,puVar21,0,1,1);
        local_120 = puStack_100;
        local_128 = puStack_108;
        puVar32 = puStack_100;
        puVar30 = puStack_108;
        if ((int)local_110 == 1) goto LAB_1001fe294;
        _memcpy(puStack_100,puVar7,(size_t)puVar21);
        local_118 = puVar21;
      }
      else {
        puVar32 = puVar21 + 0x100;
        if ((long)puVar32 < 0) {
          puVar30 = (undefined1 *)0x0;
LAB_1001fe294:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(puVar30,puVar32);
        }
        if (puVar32 == (undefined1 *)0x0) {
          local_a0 = 1;
        }
        else {
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          puVar30 = (undefined1 *)0x1;
          local_a0 = __RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(puVar32,1);
          if (local_a0 == 0) goto LAB_1001fe294;
        }
        local_98 = (undefined1 *)0x0;
        local_a8 = puVar32;
        if (lVar19 != 0) {
          lVar19 = lVar19 * 0x18;
          puVar27 = (ulong *)(lVar14 + 0x10);
          do {
            pvVar3 = (void *)puVar27[-1];
            uVar10 = *puVar27;
            if ((ulong)((long)local_a8 - (long)local_98) < uVar10) {
              alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,local_98,uVar10,1,1);
            }
            puVar32 = local_98;
            _memcpy(local_98 + local_a0,pvVar3,uVar10);
            puVar32 = puVar32 + uVar10;
            local_98 = puVar32;
            if (local_a8 == puVar32) {
              alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,puVar32,1,1,1);
            }
            puVar27 = puVar27 + 3;
            local_98[local_a0] = 10;
            local_98 = puVar32 + 1;
            lVar19 = lVar19 + -0x18;
          } while (lVar19 != 0);
        }
        local_90 = (undefined1 *)0x0;
        puStack_88 = (undefined1 *)0x1;
        local_80 = (undefined1 *)0x0;
        puVar32 = (undefined1 *)0x0;
        if (lVar15 != 0) {
          puVar30 = (undefined1 *)0x0;
          lVar31 = lVar22 + lVar15 * 0x30;
          puVar20 = (undefined1 *)0x1;
          do {
            if (puVar30 != (undefined1 *)0x0) {
              if (local_90 == puVar32) {
                alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_90,puVar32,1,1,1);
                puVar20 = puStack_88;
                puVar32 = local_80;
              }
              puVar20[(long)puVar32] = 10;
              puVar30 = puVar30 + 1;
              puVar32 = puVar30;
              local_80 = puVar30;
            }
            if (*(long *)(lVar22 + 0x28) != 0) {
              lVar26 = *(long *)(lVar22 + 0x28) * 0x18;
              puVar27 = (ulong *)(*(long *)(lVar22 + 0x20) + 0x10);
              puVar30 = puVar32;
              do {
                pvVar3 = (void *)puVar27[-1];
                uVar10 = *puVar27;
                if ((ulong)((long)local_90 - (long)puVar30) < uVar10) {
                  alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_90,puVar30,uVar10,1,1);
                  puVar30 = local_80;
                }
                puVar20 = puStack_88;
                puVar32 = local_90;
                _memcpy(puStack_88 + (long)puVar30,pvVar3,uVar10);
                puVar30 = puVar30 + uVar10;
                local_80 = puVar30;
                if (puVar32 == puVar30) {
                  alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_90,puVar32,1,1,1);
                  puVar20 = puStack_88;
                }
                puVar27 = puVar27 + 3;
                puVar20[(long)local_80] = 10;
                puVar30 = puVar30 + 1;
                lVar26 = lVar26 + -0x18;
                puVar32 = puVar30;
                local_80 = puVar30;
              } while (lVar26 != 0);
            }
            lVar22 = lVar22 + 0x30;
          } while (lVar22 != lVar31);
        }
        puVar30 = (undefined1 *)0x0;
        local_71[0] = 2;
        local_110 = (undefined1 *)0x0;
        puStack_108 = puVar21;
        puStack_f0 = (undefined1 *)0x0;
        local_e8 = puVar21;
        lStack_e0 = 0xa0000000a;
        local_d8 = 1;
        bStack_d0 = 0;
        bStack_cf = 0;
        puStack_100 = puVar7;
        puStack_f8 = puVar21;
        do {
          puVar20 = puStack_100;
          <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(&local_210,&puStack_100);
          if ((int)local_210 == 1) {
            lVar31 = (long)local_200 - (long)local_110;
            puVar20 = puVar20 + (long)local_110;
            local_110 = local_200;
            if (lVar31 != 0) goto LAB_1001fe5f8;
LAB_1001fe654:
            lVar26 = 0;
            iVar9 = managed_blocks::TomlLexicalState::line_is_structural(local_71,puVar20,lVar31);
          }
          else {
            if (((bStack_cf & 1) != 0) || ((bStack_cf = 1, (bStack_d0 & 1) == 0 && (puStack_108 == local_110)))) break;
            lVar31 = (long)puStack_108 - (long)local_110;
            puVar20 = puStack_100 + (long)local_110;
            if (lVar31 == 0) goto LAB_1001fe654;
LAB_1001fe5f8:
            lVar26 = -(ulong)(puVar20[lVar31 + -1] == '\n');
            iVar9 = managed_blocks::TomlLexicalState::line_is_structural(local_71,puVar20,lVar26 + lVar31);
          }
          if (iVar9 != 0) {
            auVar33 = ::core::str::<impl_str>::trim_matches(puVar20,lVar26 + lVar31);
            puVar20 = local_98;
            plVar29 = auVar33._0_8_;
            if ((auVar33._8_8_ == 0x37) &&
               ((((((*plVar29 == 0x6961203e3e3e2023 && plVar29[1] == 0x6c65722d696d616d) && plVar29[2] == 0x67616e616d207961) &&
                  plVar29[3] == 0x7472617473206465) && plVar29[4] == 0x544f4e204f442820) && plVar29[5] == 0x414d205449444520) &&
                *(long *)((long)plVar29 + 0x2f) == 0x29594c4c41554e41)) {
              if (puVar32 != (undefined1 *)0x0) {
                if (local_98 != (undefined1 *)0x0) {
                  if (local_a8 == local_98) {
                    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,local_98,1,1,1);
                  }
                  local_98[local_a0] = 10;
                  local_98 = puVar20 + 1;
                }
                if (puVar30 != (undefined1 *)0x0) {
                  if (puVar30 < puVar21) {
                    if ((char)puVar7[(long)puVar30] < -0x40) {
LAB_1001fe740:
                      ppuVar12 = &PTR_s_src_core_relay_config_takeover_r_1018171e0;
                      puVar32 = (undefined1 *)0x0;
                      goto LAB_1001ff064;
                    }
                  }
                  else if (puVar21 != puVar30) goto LAB_1001fe740;
                }
                if (local_a8 + -(long)local_98 < puVar30) {
                  alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,local_98,puVar30,1,1);
                }
                puVar20 = local_98;
                lVar31 = local_a0;
                _memcpy(local_98 + local_a0,puVar7,(size_t)puVar30);
                puVar20 = puVar20 + (long)puVar30;
                local_98 = puVar20;
                if (puVar20 < (undefined1 *)0x2) {
                  if (puVar20 != (undefined1 *)0x0) {
LAB_1001fedd8:
                    if (local_a8 == puVar20) {
                      alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,puVar20,1,1,1);
                      lVar31 = local_a0;
                    }
                    local_98[lVar31] = 10;
                    local_98 = puVar20 + 1;
                  }
                }
                else if (*(short *)(puVar20 + lVar31 + -2) != 0xa0a) goto LAB_1001fedd8;
                puVar20 = puStack_88;
                if (local_a8 + -(long)local_98 < puVar32) {
                  alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,local_98,puVar32,1,1);
                  lVar31 = local_a0;
                }
                puVar25 = local_98;
                _memcpy(local_98 + lVar31,puVar20,(size_t)puVar32);
                puVar25 = puVar25 + (long)puVar32;
                if (local_a8 == puVar25) {
                  local_98 = puVar25;
                  alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,puVar25,1,1,1);
                  local_98[local_a0] = 10;
                }
                else {
                  puVar25[local_a0] = 10;
                }
                if (puVar30 != (undefined1 *)0x0) {
                  local_98 = puVar25 + 1;
                  if (puVar30 < puVar21) {
                    if ((char)puVar7[(long)puVar30] < -0x40) {
LAB_1001ff054:
                      ppuVar12 = &PTR_s_src_core_relay_config_takeover_r_1018171f8;
                      puVar32 = puVar30;
                      puVar30 = puVar21;
LAB_1001ff064:
                    /* WARNING: Subroutine does not return */
                      ::core::str::slice_error_fail(puVar7,puVar21,puVar32,puVar30,ppuVar12);
                    }
                  }
                  else if (puVar21 != puVar30) goto LAB_1001ff054;
                }
                local_98 = puVar25 + 1;
                puVar32 = puVar21 + -(long)puVar30;
                if (local_a8 + -(long)local_98 < puVar32) {
                  alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,local_98,puVar32,1,1);
                }
                puVar20 = local_98 + local_a0;
                puVar30 = puVar7 + (long)puVar30;
                goto LAB_1001fe854;
              }
              break;
            }
          }
          puVar30 = puVar30 + lVar31;
        } while ((bStack_cf & 1) == 0);
        puVar25 = local_98;
        if ((local_98 != (undefined1 *)0x0) && (::core::str::<impl_str>::trim_matches(puVar7,puVar21), extraout_x1 != 0)) {
          puVar30 = puVar25;
          if (local_a8 == puVar25) {
            alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,puVar25,1,1,1);
            puVar30 = local_98;
          }
          puVar30[local_a0] = 10;
          puVar25 = puVar25 + 1;
          local_98 = puVar25;
        }
        if (local_a8 + -(long)puVar25 < puVar21) {
          alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,puVar25,puVar21,1,1);
          puVar25 = local_98;
        }
        lVar31 = local_a0;
        _memcpy(puVar25 + local_a0,puVar7,(size_t)puVar21);
        puVar25 = puVar25 + (long)puVar21;
        if (puVar32 != (undefined1 *)0x0) {
          local_98 = puVar25;
          if (puVar25 != (undefined1 *)0x0) {
            if (puVar25[lVar31 + -1] == '\n') {
              if (puVar25 != (undefined1 *)0x1) goto LAB_1001fe804;
            }
            else {
              if (local_a8 == puVar25) {
                alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,puVar25,1,1,1);
                lVar31 = local_a0;
              }
              local_98[lVar31] = 10;
              puVar25 = puVar25 + 1;
LAB_1001fe804:
              local_98 = puVar25;
              if (*(short *)(puVar25 + lVar31 + -2) == 0xa0a) goto LAB_1001fe834;
            }
            puVar30 = local_98;
            if (local_a8 == local_98) {
              alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,local_98,1,1,1);
              lVar31 = local_a0;
            }
            local_98[lVar31] = 10;
            local_98 = puVar30 + 1;
          }
LAB_1001fe834:
          puVar30 = puStack_88;
          if (local_a8 + -(long)local_98 < puVar32) {
            alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,local_98,puVar32,1,1);
          }
          puVar20 = local_98 + local_a0;
LAB_1001fe854:
          puVar25 = local_98;
          lVar31 = local_a0;
          _memcpy(puVar20,puVar30,(size_t)puVar32);
          puVar25 = puVar25 + (long)puVar32;
        }
        local_98 = puVar25;
        tidy_blank_lines(&local_128,lVar31,puVar25);
        if (local_90 != (undefined1 *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_88,local_90,1);
        }
        if (local_a8 != (undefined1 *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar31,local_a8,1);
        }
      }
      puVar30 = local_118;
      puVar32 = local_120;
      codex_config::validate_candidate(&local_110,local_120,local_118);
      if (local_110 == &DAT_0000000b) {
        if ((puVar30 == puVar21) && (iVar9 = _memcmp(puVar32,puVar7,(size_t)puVar21), iVar9 == 0)) {
          std::sys::fs::exists(&local_110,uVar1,uVar4);
          if (((ulong)local_110 & 1) != 0) {
            param_1[2] = puStack_108;
            param_1[1] = 2;
            *param_1 = 1;
            goto LAB_1001fe934;
          }
          if ((local_110._1_1_ == '\x01') && (lVar31 = atomic_write::confirm_existing_sync(uVar1,uVar4), lVar31 != 0)) {
            restore::__closure__(&local_110);
            goto LAB_1001fe908;
          }
        }
        else {
          auVar33 = atomic_write::write_atomic_private_report(uVar1,uVar4,puVar32,puVar30);
          if (((auVar33._0_8_ & 1) != 0) || (lVar31 = atomic_write::Publication::require_sync(auVar33._8_8_), lVar31 != 0)) {
            restore::__closure__(&local_110);
            goto LAB_1001fe908;
          }
        }
        std::path::Path::_join(&local_210,uVar18,uVar24,"router-takeover-backup.json",0x1b);
        puVar21 = puStack_208;
        remove_backup_file(&local_110,puStack_208,local_200,"router-takeover-backup.json",0x1b);
        if (local_110 == &DAT_0000000b) {
          if (local_210 != (undefined1 *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar21,local_210,1);
          }
          std::path::Path::_join
                    (&local_210,uVar18,uVar24,"codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey",0x22);
          puVar21 = puStack_208;
          remove_backup_file(&local_110,puStack_208,local_200,
                             "codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey",0x22);
          if (local_110 == &DAT_0000000b) {
            if (local_210 != (undefined1 *)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar21,local_210,1);
            }
            param_1[6] = lStack_2a8;
            param_1[5] = local_2b0;
            param_1[8] = local_298;
            param_1[7] = local_2a0;
            param_1[10] = puStack_288;
            param_1[9] = local_290;
            param_1[0xc] = local_278;
            param_1[0xb] = puStack_280;
            param_1[2] = uStack_2c8;
            param_1[1] = local_2d0;
            param_1[4] = local_2b8;
            param_1[3] = local_2c0;
            *param_1 = 0;
            if (local_128 != (undefined1 *)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar32,local_128,1);
            }
            ::core::ptr::drop_in_place<alloc::vec::Vec<codexmate_lib::core::relay::config_takeover::TakeoverSection>>(&local_140);
            ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayActiveByIde>(&local_158);
            ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayActiveByIde>(&local_170);
            ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayActiveByIde>(&local_190);
            if (puVar23 != (undefined1 *)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,puVar23,1);
            }
            ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayActiveByIde>(&local_1b0);
            if (local_270 == &DAT_8000000000000000) {
              return;
            }
            ::core::ptr::drop_in_place<codexmate_lib::core::relay::config_takeover::TakeoverBackup>(&local_270);
            return;
          }
          param_1[6] = local_e8;
          param_1[5] = puStack_f0;
          param_1[8] = CONCAT71(uStack_d7,local_d8);
          param_1[7] = lStack_e0;
          param_1[10] = local_c8;
          param_1[9] = CONCAT62(uStack_ce,CONCAT11(bStack_cf,bStack_d0));
          param_1[0xc] = local_b8;
          param_1[0xb] = uStack_c0;
          param_1[2] = puStack_108;
          param_1[1] = local_110;
          param_1[4] = puStack_f8;
          param_1[3] = puStack_100;
          *param_1 = 1;
        }
        else {
          param_1[6] = local_e8;
          param_1[5] = puStack_f0;
          param_1[8] = CONCAT71(uStack_d7,local_d8);
          param_1[7] = lStack_e0;
          param_1[10] = local_c8;
          param_1[9] = CONCAT62(uStack_ce,CONCAT11(bStack_cf,bStack_d0));
          param_1[0xc] = local_b8;
          param_1[0xb] = uStack_c0;
          param_1[2] = puStack_108;
          param_1[1] = local_110;
          param_1[4] = puStack_f8;
          param_1[3] = puStack_100;
          *param_1 = 1;
        }
        if (local_210 != (undefined1 *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar21,local_210,1);
        }
      }
      else {
LAB_1001fe908:
        param_1[6] = local_e8;
        param_1[5] = puStack_f0;
        param_1[8] = CONCAT71(uStack_d7,local_d8);
        param_1[7] = lStack_e0;
        param_1[10] = local_c8;
        param_1[9] = CONCAT62(uStack_ce,CONCAT11(bStack_cf,bStack_d0));
        param_1[0xc] = local_b8;
        param_1[0xb] = uStack_c0;
        param_1[2] = puStack_108;
        param_1[1] = local_110;
        param_1[4] = puStack_f8;
        param_1[3] = puStack_100;
        *param_1 = 1;
      }
LAB_1001fe934:
      if (local_128 != (undefined1 *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar32,local_128,1);
      }
      lVar26 = local_130;
      lVar31 = local_138;
      if (local_130 != 0) {
        lVar22 = 0;
        do {
          plVar29 = (long *)(lVar31 + lVar22 * 0x30);
          if (*plVar29 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar29[1],*plVar29,1);
          }
          lVar19 = plVar29[4];
          lVar14 = plVar29[5];
          if (lVar14 != 0) {
            puVar13 = (undefined8 *)(lVar19 + 8);
            do {
              if (puVar13[-1] != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar13,puVar13[-1],1);
              }
              puVar13 = puVar13 + 3;
              lVar14 = lVar14 + -1;
            } while (lVar14 != 0);
          }
          if (plVar29[3] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar19,plVar29[3] * 0x18,8);
          }
          lVar22 = lVar22 + 1;
        } while (lVar22 != lVar26);
      }
      if (local_140 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_138,local_140 * 0x30,8);
      }
      if (local_148 != 0) {
        puVar13 = (undefined8 *)(local_150 + 8);
        lVar31 = local_148;
        do {
          if (puVar13[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar13,puVar13[-1],1);
          }
          puVar13 = puVar13 + 3;
          lVar31 = lVar31 + -1;
        } while (lVar31 != 0);
      }
      if (local_158 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_150,local_158 * 0x18,8);
      }
      puVar32 = puStack_168;
      if (local_160 != (undefined1 *)0x0) {
        puVar13 = (undefined8 *)(puStack_168 + 8);
        puVar21 = local_160;
        do {
          if (puVar13[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar13,puVar13[-1],1);
          }
          puVar13 = puVar13 + 3;
          puVar21 = puVar21 + -1;
        } while (puVar21 != (undefined1 *)0x0);
      }
      if (local_170 != (undefined1 *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar32,(long)local_170 * 0x18,8);
      }
      lVar31 = lStack_188;
      if (local_180 != (undefined1 *)0x0) {
        puVar13 = (undefined8 *)(lStack_188 + 8);
        puVar32 = local_180;
        do {
          if (puVar13[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar13,puVar13[-1],1);
          }
          puVar13 = puVar13 + 3;
          puVar32 = puVar32 + -1;
        } while (puVar32 != (undefined1 *)0x0);
      }
      if (local_190 != (undefined1 *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar31,(long)local_190 * 0x18,8);
      }
      bVar6 = true;
      if (puVar23 != (undefined1 *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,puVar23,1);
      }
    }
    else {
      param_1[6] = local_e8;
      param_1[5] = puStack_f0;
      param_1[8] = CONCAT71(uStack_d7,local_d8);
      param_1[7] = lStack_e0;
      param_1[10] = local_c8;
      param_1[9] = CONCAT62(uStack_ce,CONCAT11(bStack_cf,bStack_d0));
      param_1[0xc] = local_b8;
      param_1[0xb] = uStack_c0;
      param_1[2] = puStack_108;
      param_1[3] = puStack_100;
      param_1[4] = puStack_f8;
      bVar6 = true;
      *param_1 = 1;
      param_1[1] = local_110;
    }
  }
  puVar32 = puStack_1a8;
  if (local_1a0 != (undefined1 *)0x0) {
    puVar13 = (undefined8 *)(puStack_1a8 + 8);
    puVar23 = local_1a0;
    do {
      if (puVar13[-1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar13,puVar13[-1],1);
      }
      puVar13 = puVar13 + 3;
      puVar23 = puVar23 + -1;
    } while (puVar23 != (undefined1 *)0x0);
  }
  if (local_1b0 != (undefined1 *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar32,(long)local_1b0 * 0x18,8);
  }
  if (local_270 != &DAT_8000000000000000) {
    ::core::ptr::drop_in_place<codexmate_lib::core::relay::config_takeover::TakeoverBackup>(&local_270);
  }
  if (!bVar6) {
    return;
  }
LAB_1001fdb4c:
  ::core::ptr::drop_in_place<codexmate_lib::core::relay::config_takeover::RestoreSummary>(&local_2d0);
  return;
}
