/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* is_codex_related_command_with_main @ 1007c65cc requested 1007c65cc */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::platform::process::is_codex_related_command_with_main */

bool codexmate_lib::platform::process::is_codex_related_command_with_main(long *param_1,ulong param_2,long param_3,undefined8 param_4)

{
  long *plVar1;
  bool bVar2;
  int iVar3;
  ulong uVar4;
  long lVar5;
  long *plVar6;
  ulong uVar7;
  byte *pbVar8;
  long *plVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  uint uVar13;
  uint uVar14;
  byte bVar15;
  byte bVar16;
  byte bVar17;
  byte bVar18;
  byte bVar19;
  byte bVar20;
  byte bVar21;
  byte bVar23;
  byte bVar24;
  byte bVar25;
  byte bVar26;
  byte bVar27;
  byte bVar28;
  long lVar22;
  byte bVar29;
  byte bVar32;
  byte bVar33;
  byte bVar34;
  byte bVar35;
  byte bVar36;
  byte bVar37;
  long lVar30;
  byte bVar38;
  ulong uVar31;
  byte bVar40;
  byte bVar41;
  byte bVar42;
  byte bVar43;
  byte bVar44;
  byte bVar45;
  long lVar39;
  byte bVar46;
  byte bVar48;
  byte bVar49;
  byte bVar50;
  byte bVar51;
  byte bVar52;
  byte bVar53;
  long lVar47;
  byte bVar54;
  byte bVar56;
  byte bVar57;
  byte bVar58;
  byte bVar59;
  byte bVar60;
  byte bVar61;
  long lVar55;
  byte bVar62;
  byte bVar64;
  byte bVar65;
  byte bVar66;
  byte bVar67;
  byte bVar68;
  byte bVar69;
  long lVar63;
  byte bVar70;
  byte bVar72;
  byte bVar73;
  byte bVar74;
  byte bVar75;
  byte bVar76;
  byte bVar77;
  long lVar71;
  byte bVar78;
  undefined1 auVar79 [16];
  undefined1 auVar80 [16];
  ulong local_120;
  ulong uStack_118;
  long *local_110;
  ulong uStack_108;
  long *local_100;
  long lStack_f8;
  undefined8 local_f0;
  undefined2 local_e8;
  long local_e0;
  long *local_d8;
  ulong uStack_d0;
  ulong local_c8;
  byte *local_c0;
  undefined1 local_b8 [16];
  long *local_a8;
  ulong uStack_a0;
  undefined8 local_98;
  undefined2 local_90;
  undefined2 local_88;
  
  auVar79._8_8_ = param_4;
  auVar79._0_8_ = param_3;
  uVar4 = is_protected_process_command();
  if ((uVar4 & 1) != 0) {
    return false;
  }
  if (param_3 != 0) {
    uVar4 = command_runs_executable(param_1,param_2,param_3,param_4);
    if ((uVar4 & 1) != 0) {
      return true;
    }
    do {
      uVar12 = auVar79._8_8_;
      uVar11 = auVar79._0_8_;
      auVar79 = std::path::Path::parent(uVar11,uVar12);
      lVar5 = std::path::Path::extension(uVar11,uVar12);
      if (lVar5 != 0) {
        ::core::str::converts::from_utf8(&local_c8);
        if (((int)local_c8 != 1) && (local_b8._0_8_ == 3)) {
          uVar13 = 0x20;
          uVar14 = uVar13;
          if (0x19 < *local_c0 - 0x41) {
            uVar14 = 0;
          }
          if ((uVar14 | *local_c0) == 0x61) {
            if (0x19 < local_c0[1] - 0x41) {
              uVar13 = 0;
            }
            if ((uVar13 | local_c0[1]) == 0x70) {
              uVar14 = 0x20;
              if (0x19 < local_c0[2] - 0x41) {
                uVar14 = 0;
              }
              if ((uVar14 | local_c0[2]) == 0x70) {
                alloc::string::String::from_utf8_lossy(&local_c8,uVar11,uVar12);
                if (param_2 < (ulong)local_b8._0_8_) {
                  bVar2 = false;
                }
                else {
                  iVar3 = _memcmp(local_c0,param_1,local_b8._0_8_);
                  bVar2 = false;
                  if ((iVar3 == 0) && (param_2 != local_b8._0_8_)) {
                    bVar2 = *(char *)((long)param_1 + local_b8._0_8_) == '/';
                  }
                }
                if ((local_c8 & 0x7fffffffffffffff) != 0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_c0,local_c8,1);
                }
                if (bVar2) {
                  return true;
                }
                break;
              }
            }
          }
        }
      }
    } while (auVar79._0_8_ != 0);
  }
  if (param_2 < 0xb) {
    if ((param_2 == 10) && (*param_1 == 0x70612e7865646f43 && (short)param_1[1] == 0x2f70)) {
      return true;
    }
  }
  else {
    ::core::str::pattern::StrSearcher::new
              (&local_c8,param_1,param_2,
               "Codex.app/Codex Helper-t--Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codexContents/Resources/codexa tuple of size 2a sequencesignala tuple of size 3"
               ,10);
    <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_120,&local_c8);
    if ((local_120 & 1) != 0) {
      return true;
    }
    if (param_2 < 0xd) {
      if ((param_2 == 0xc) && (*param_1 == 0x6548207865646f43 && (int)param_1[1] == 0x7265706c)) {
        return true;
      }
    }
    else {
      ::core::str::pattern::StrSearcher::new
                (&local_c8,param_1,param_2,
                 "Codex Helper-t--Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codexContents/Resources/codexa tuple of size 2a sequencesignala tuple of size 3"
                 ,0xc);
      <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_120,&local_c8);
      if ((local_120 & 1) != 0) {
        return true;
      }
    }
  }
  local_c8 = 0;
  local_98 = 0;
  local_90 = 1;
  local_c0 = (byte *)param_2;
  local_b8._0_8_ = param_1;
  local_b8._8_8_ = param_2;
  local_a8 = param_1;
  uStack_a0 = (long)param_1 + param_2;
  auVar79 = ::core::iter::traits::iterator::Iterator::try_fold(&local_c8);
  plVar9 = auVar79._0_8_;
  uVar4 = param_2;
  if (plVar9 != (long *)0x0) {
    uVar4 = auVar79._8_8_;
  }
  local_c8 = 0;
  plVar1 = param_1;
  if (plVar9 != (long *)0x0) {
    plVar1 = plVar9;
  }
  local_a8 = (long *)0x0;
  local_98 = 0x2f0000002f;
  local_90._0_1_ = 1;
  local_88 = 1;
  local_c0 = (byte *)uVar4;
  local_b8._0_8_ = plVar1;
  local_b8._8_8_ = uVar4;
  uStack_a0 = uVar4;
  auVar79 = ::core::str::iter::SplitInternal<P>::next_back(&local_c8);
  plVar9 = auVar79._0_8_;
  uVar12 = 0;
  if (plVar9 != (long *)0x0) {
    uVar4 = auVar79._8_8_;
  }
  if ((long)uVar4 < 0) {
LAB_1007c68a4:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar12,uVar4);
  }
  if (uVar4 == 0) {
    if (plVar9 != (long *)0x0) {
      plVar1 = plVar9;
    }
    _memcpy((void *)0x1,plVar1,0);
    return false;
  }
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  uVar12 = 1;
  plVar6 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(uVar4,1);
  if (plVar6 == (long *)0x0) goto LAB_1007c68a4;
  if (plVar9 != (long *)0x0) {
    plVar1 = plVar9;
  }
  _memcpy(plVar6,plVar1,uVar4);
  if (uVar4 < 8) {
    uVar7 = 0;
LAB_1007c69e4:
    lVar5 = uVar4 - uVar7;
    pbVar8 = (byte *)((long)plVar6 + uVar7);
    do {
      bVar15 = 0x20;
      if (0x19 < *pbVar8 - 0x41) {
        bVar15 = 0;
      }
      *pbVar8 = bVar15 | *pbVar8;
      lVar5 = lVar5 + -1;
      pbVar8 = pbVar8 + 1;
    } while (lVar5 != 0);
  }
  else if (uVar4 < 0x40) {
    uVar31 = 0;
LAB_1007c69a0:
    uVar7 = uVar4 & 0x7ffffffffffffff8;
    lVar5 = uVar31 - uVar7;
    puVar10 = (undefined8 *)((long)plVar6 + uVar31);
    do {
      uVar12 = *puVar10;
      bVar15 = (byte)((ulong)uVar12 >> 8);
      bVar16 = (byte)((ulong)uVar12 >> 0x10);
      bVar17 = (byte)((ulong)uVar12 >> 0x18);
      bVar18 = (byte)((ulong)uVar12 >> 0x20);
      bVar19 = (byte)((ulong)uVar12 >> 0x28);
      bVar20 = (byte)((ulong)uVar12 >> 0x30);
      bVar21 = (byte)((ulong)uVar12 >> 0x38);
      uVar31 = CONCAT17(-((byte)(bVar21 + 0xbf) < 0x1a),
                        CONCAT16(-((byte)(bVar20 + 0xbf) < 0x1a),
                                 CONCAT15(-((byte)(bVar19 + 0xbf) < 0x1a),
                                          CONCAT14(-((byte)(bVar18 + 0xbf) < 0x1a),
                                                   CONCAT13(-((byte)(bVar17 + 0xbf) < 0x1a),
                                                            CONCAT12(-((byte)(bVar16 + 0xbf) < 0x1a),
                                                                     CONCAT11(-((byte)(bVar15 + 0xbf) < 0x1a),-((byte)((byte)uVar12 + 0xbf) < 0x1a)))))))) &
               0x2020202020202020;
      *puVar10 = CONCAT17((byte)(uVar31 >> 0x38) | bVar21,
                          CONCAT16((byte)(uVar31 >> 0x30) | bVar20,
                                   CONCAT15((byte)(uVar31 >> 0x28) | bVar19,
                                            CONCAT14((byte)(uVar31 >> 0x20) | bVar18,
                                                     CONCAT13((byte)(uVar31 >> 0x18) | bVar17,
                                                              CONCAT12((byte)(uVar31 >> 0x10) | bVar16,
                                                                       CONCAT11((byte)(uVar31 >> 8) | bVar15,(byte)uVar31 | (byte)uVar12)))))));
      lVar5 = lVar5 + 8;
      puVar10 = puVar10 + 1;
    } while (lVar5 != 0);
    if (uVar4 != uVar7) goto LAB_1007c69e4;
  }
  else {
    uVar7 = uVar4 & 0x7fffffffffffffc0;
    plVar9 = plVar6 + 4;
    uVar31 = uVar7;
    do {
      lVar22 = plVar9[-3];
      lVar5 = plVar9[-4];
      lVar39 = plVar9[-1];
      lVar30 = plVar9[-2];
      lVar55 = plVar9[1];
      lVar47 = *plVar9;
      lVar71 = plVar9[3];
      lVar63 = plVar9[2];
      bVar15 = (byte)((ulong)lVar5 >> 8);
      bVar16 = (byte)((ulong)lVar5 >> 0x10);
      bVar17 = (byte)((ulong)lVar5 >> 0x18);
      bVar18 = (byte)((ulong)lVar5 >> 0x20);
      bVar19 = (byte)((ulong)lVar5 >> 0x28);
      bVar20 = (byte)((ulong)lVar5 >> 0x30);
      bVar21 = (byte)((ulong)lVar5 >> 0x38);
      bVar23 = (byte)((ulong)lVar22 >> 8);
      bVar24 = (byte)((ulong)lVar22 >> 0x10);
      bVar25 = (byte)((ulong)lVar22 >> 0x18);
      bVar26 = (byte)((ulong)lVar22 >> 0x20);
      bVar27 = (byte)((ulong)lVar22 >> 0x28);
      bVar28 = (byte)((ulong)lVar22 >> 0x30);
      bVar29 = (byte)((ulong)lVar22 >> 0x38);
      bVar32 = (byte)((ulong)lVar30 >> 8);
      bVar33 = (byte)((ulong)lVar30 >> 0x10);
      bVar34 = (byte)((ulong)lVar30 >> 0x18);
      bVar35 = (byte)((ulong)lVar30 >> 0x20);
      bVar36 = (byte)((ulong)lVar30 >> 0x28);
      bVar37 = (byte)((ulong)lVar30 >> 0x30);
      bVar38 = (byte)((ulong)lVar30 >> 0x38);
      bVar40 = (byte)((ulong)lVar39 >> 8);
      bVar41 = (byte)((ulong)lVar39 >> 0x10);
      bVar42 = (byte)((ulong)lVar39 >> 0x18);
      bVar43 = (byte)((ulong)lVar39 >> 0x20);
      bVar44 = (byte)((ulong)lVar39 >> 0x28);
      bVar45 = (byte)((ulong)lVar39 >> 0x30);
      bVar46 = (byte)((ulong)lVar39 >> 0x38);
      bVar48 = (byte)((ulong)lVar47 >> 8);
      bVar49 = (byte)((ulong)lVar47 >> 0x10);
      bVar50 = (byte)((ulong)lVar47 >> 0x18);
      bVar51 = (byte)((ulong)lVar47 >> 0x20);
      bVar52 = (byte)((ulong)lVar47 >> 0x28);
      bVar53 = (byte)((ulong)lVar47 >> 0x30);
      bVar54 = (byte)((ulong)lVar47 >> 0x38);
      bVar56 = (byte)((ulong)lVar55 >> 8);
      bVar57 = (byte)((ulong)lVar55 >> 0x10);
      bVar58 = (byte)((ulong)lVar55 >> 0x18);
      bVar59 = (byte)((ulong)lVar55 >> 0x20);
      bVar60 = (byte)((ulong)lVar55 >> 0x28);
      bVar61 = (byte)((ulong)lVar55 >> 0x30);
      bVar62 = (byte)((ulong)lVar55 >> 0x38);
      bVar64 = (byte)((ulong)lVar63 >> 8);
      bVar65 = (byte)((ulong)lVar63 >> 0x10);
      bVar66 = (byte)((ulong)lVar63 >> 0x18);
      bVar67 = (byte)((ulong)lVar63 >> 0x20);
      bVar68 = (byte)((ulong)lVar63 >> 0x28);
      bVar69 = (byte)((ulong)lVar63 >> 0x30);
      bVar70 = (byte)((ulong)lVar63 >> 0x38);
      bVar72 = (byte)((ulong)lVar71 >> 8);
      bVar73 = (byte)((ulong)lVar71 >> 0x10);
      bVar74 = (byte)((ulong)lVar71 >> 0x18);
      bVar75 = (byte)((ulong)lVar71 >> 0x20);
      bVar76 = (byte)((ulong)lVar71 >> 0x28);
      bVar77 = (byte)((ulong)lVar71 >> 0x30);
      bVar78 = (byte)((ulong)lVar71 >> 0x38);
      plVar9[-3] = CONCAT17(-((byte)(bVar29 + 0xbf) < 0x1a) & 0x20U | bVar29,
                            CONCAT16(-((byte)(bVar28 + 0xbf) < 0x1a) & 0x20U | bVar28,
                                     CONCAT15(-((byte)(bVar27 + 0xbf) < 0x1a) & 0x20U | bVar27,
                                              CONCAT14(-((byte)(bVar26 + 0xbf) < 0x1a) & 0x20U | bVar26,
                                                       CONCAT13(-((byte)(bVar25 + 0xbf) < 0x1a) & 0x20U | bVar25,
                                                                CONCAT12(-((byte)(bVar24 + 0xbf) < 0x1a) & 0x20U | bVar24,
                                                                         CONCAT11(-((byte)(bVar23 + 0xbf) < 0x1a) & 0x20U | bVar23,
                                                                                  -((byte)((byte)lVar22 + 0xbf) < 0x1a) & 0x20U | (byte)lVar22)))))));
      plVar9[-4] = CONCAT17(-((byte)(bVar21 + 0xbf) < 0x1a) & 0x20U | bVar21,
                            CONCAT16(-((byte)(bVar20 + 0xbf) < 0x1a) & 0x20U | bVar20,
                                     CONCAT15(-((byte)(bVar19 + 0xbf) < 0x1a) & 0x20U | bVar19,
                                              CONCAT14(-((byte)(bVar18 + 0xbf) < 0x1a) & 0x20U | bVar18,
                                                       CONCAT13(-((byte)(bVar17 + 0xbf) < 0x1a) & 0x20U | bVar17,
                                                                CONCAT12(-((byte)(bVar16 + 0xbf) < 0x1a) & 0x20U | bVar16,
                                                                         CONCAT11(-((byte)(bVar15 + 0xbf) < 0x1a) & 0x20U | bVar15,
                                                                                  -((byte)((byte)lVar5 + 0xbf) < 0x1a) & 0x20U | (byte)lVar5)))))));
      plVar9[-1] = CONCAT17(-((byte)(bVar46 + 0xbf) < 0x1a) & 0x20U | bVar46,
                            CONCAT16(-((byte)(bVar45 + 0xbf) < 0x1a) & 0x20U | bVar45,
                                     CONCAT15(-((byte)(bVar44 + 0xbf) < 0x1a) & 0x20U | bVar44,
                                              CONCAT14(-((byte)(bVar43 + 0xbf) < 0x1a) & 0x20U | bVar43,
                                                       CONCAT13(-((byte)(bVar42 + 0xbf) < 0x1a) & 0x20U | bVar42,
                                                                CONCAT12(-((byte)(bVar41 + 0xbf) < 0x1a) & 0x20U | bVar41,
                                                                         CONCAT11(-((byte)(bVar40 + 0xbf) < 0x1a) & 0x20U | bVar40,
                                                                                  -((byte)((byte)lVar39 + 0xbf) < 0x1a) & 0x20U | (byte)lVar39)))))));
      plVar9[-2] = CONCAT17(-((byte)(bVar38 + 0xbf) < 0x1a) & 0x20U | bVar38,
                            CONCAT16(-((byte)(bVar37 + 0xbf) < 0x1a) & 0x20U | bVar37,
                                     CONCAT15(-((byte)(bVar36 + 0xbf) < 0x1a) & 0x20U | bVar36,
                                              CONCAT14(-((byte)(bVar35 + 0xbf) < 0x1a) & 0x20U | bVar35,
                                                       CONCAT13(-((byte)(bVar34 + 0xbf) < 0x1a) & 0x20U | bVar34,
                                                                CONCAT12(-((byte)(bVar33 + 0xbf) < 0x1a) & 0x20U | bVar33,
                                                                         CONCAT11(-((byte)(bVar32 + 0xbf) < 0x1a) & 0x20U | bVar32,
                                                                                  -((byte)((byte)lVar30 + 0xbf) < 0x1a) & 0x20U | (byte)lVar30)))))));
      plVar9[1] = CONCAT17(-((byte)(bVar62 + 0xbf) < 0x1a) & 0x20U | bVar62,
                           CONCAT16(-((byte)(bVar61 + 0xbf) < 0x1a) & 0x20U | bVar61,
                                    CONCAT15(-((byte)(bVar60 + 0xbf) < 0x1a) & 0x20U | bVar60,
                                             CONCAT14(-((byte)(bVar59 + 0xbf) < 0x1a) & 0x20U | bVar59,
                                                      CONCAT13(-((byte)(bVar58 + 0xbf) < 0x1a) & 0x20U | bVar58,
                                                               CONCAT12(-((byte)(bVar57 + 0xbf) < 0x1a) & 0x20U | bVar57,
                                                                        CONCAT11(-((byte)(bVar56 + 0xbf) < 0x1a) & 0x20U | bVar56,
                                                                                 -((byte)((byte)lVar55 + 0xbf) < 0x1a) & 0x20U | (byte)lVar55)))))));
      *plVar9 = CONCAT17(-((byte)(bVar54 + 0xbf) < 0x1a) & 0x20U | bVar54,
                         CONCAT16(-((byte)(bVar53 + 0xbf) < 0x1a) & 0x20U | bVar53,
                                  CONCAT15(-((byte)(bVar52 + 0xbf) < 0x1a) & 0x20U | bVar52,
                                           CONCAT14(-((byte)(bVar51 + 0xbf) < 0x1a) & 0x20U | bVar51,
                                                    CONCAT13(-((byte)(bVar50 + 0xbf) < 0x1a) & 0x20U | bVar50,
                                                             CONCAT12(-((byte)(bVar49 + 0xbf) < 0x1a) & 0x20U | bVar49,
                                                                      CONCAT11(-((byte)(bVar48 + 0xbf) < 0x1a) & 0x20U | bVar48,
                                                                               -((byte)((byte)lVar47 + 0xbf) < 0x1a) & 0x20U | (byte)lVar47)))))));
      plVar9[3] = CONCAT17(-((byte)(bVar78 + 0xbf) < 0x1a) & 0x20U | bVar78,
                           CONCAT16(-((byte)(bVar77 + 0xbf) < 0x1a) & 0x20U | bVar77,
                                    CONCAT15(-((byte)(bVar76 + 0xbf) < 0x1a) & 0x20U | bVar76,
                                             CONCAT14(-((byte)(bVar75 + 0xbf) < 0x1a) & 0x20U | bVar75,
                                                      CONCAT13(-((byte)(bVar74 + 0xbf) < 0x1a) & 0x20U | bVar74,
                                                               CONCAT12(-((byte)(bVar73 + 0xbf) < 0x1a) & 0x20U | bVar73,
                                                                        CONCAT11(-((byte)(bVar72 + 0xbf) < 0x1a) & 0x20U | bVar72,
                                                                                 -((byte)((byte)lVar71 + 0xbf) < 0x1a) & 0x20U | (byte)lVar71)))))));
      plVar9[2] = CONCAT17(-((byte)(bVar70 + 0xbf) < 0x1a) & 0x20U | bVar70,
                           CONCAT16(-((byte)(bVar69 + 0xbf) < 0x1a) & 0x20U | bVar69,
                                    CONCAT15(-((byte)(bVar68 + 0xbf) < 0x1a) & 0x20U | bVar68,
                                             CONCAT14(-((byte)(bVar67 + 0xbf) < 0x1a) & 0x20U | bVar67,
                                                      CONCAT13(-((byte)(bVar66 + 0xbf) < 0x1a) & 0x20U | bVar66,
                                                               CONCAT12(-((byte)(bVar65 + 0xbf) < 0x1a) & 0x20U | bVar65,
                                                                        CONCAT11(-((byte)(bVar64 + 0xbf) < 0x1a) & 0x20U | bVar64,
                                                                                 -((byte)((byte)lVar63 + 0xbf) < 0x1a) & 0x20U | (byte)lVar63)))))));
      uVar31 = uVar31 - 0x40;
      plVar9 = plVar9 + 8;
    } while (uVar31 != 0);
    if (uVar4 != uVar7) {
      uVar31 = uVar7;
      if ((uVar4 & 0x38) == 0) goto LAB_1007c69e4;
      goto LAB_1007c69a0;
    }
  }
  if (uVar4 == 5) {
    if ((int)*plVar6 != 0x65646f63 || *(char *)((long)plVar6 + 4) != 'x') goto LAB_1007c6ad4;
  }
  else if ((uVar4 < 6) ||
          (((int)*plVar6 != 0x65646f63 || *(short *)((long)plVar6 + 4) != 0x2d78 && ((int)*plVar6 != 0x65646f63 || *(short *)((long)plVar6 + 4) != 0x5f78)))) {
    if (uVar4 == 8) {
      if (*plVar6 != 0x6578652e65646f6e) {
        uVar4 = 8;
        goto LAB_1007c6ad4;
      }
    }
    else {
      if (uVar4 != 4) {
LAB_1007c6ad4:
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar6,uVar4,1);
        return false;
      }
      if ((int)*plVar6 != 0x65646f6e) {
        uVar4 = 4;
        goto LAB_1007c6ad4;
      }
    }
    local_120 = 0;
    local_f0 = 0;
    local_e8 = 1;
    uStack_118 = param_2;
    local_110 = param_1;
    uStack_108 = param_2;
    local_100 = param_1;
    lStack_f8 = (long)param_1 + param_2;
    lVar5 = ::core::iter::traits::iterator::Iterator::try_fold(&local_120);
    if (lVar5 != 0) {
      auVar79 = ::core::iter::traits::iterator::Iterator::try_fold(&local_120);
      if (auVar79._0_8_ != 0) {
        local_c8 = 0;
        local_a8 = (long *)0x0;
        local_98 = 0x2f0000002f;
        local_90._0_1_ = 1;
        local_88 = 1;
        local_c0 = (byte *)auVar79._8_8_;
        uStack_a0 = auVar79._8_8_;
        local_b8 = auVar79;
        auVar80 = ::core::str::iter::SplitInternal<P>::next_back(&local_c8);
        if (auVar80._0_8_ != 0) {
          auVar79 = auVar80;
        }
        alloc::str::<impl_str>::to_ascii_lowercase(&local_e0,auVar79._0_8_,auVar79._8_8_);
        if (uStack_d0 == 5) {
          bVar2 = false;
          if ((int)*local_d8 == 0x65646f63) {
            bVar2 = *(char *)((long)local_d8 + 4) == 'x';
          }
        }
        else if (uStack_d0 < 6) {
LAB_1007c6be8:
          bVar2 = false;
        }
        else if ((int)*local_d8 == 0x65646f63 && *(short *)((long)local_d8 + 4) == 0x2d78) {
          bVar2 = true;
        }
        else {
          if (uStack_d0 != 8) goto LAB_1007c6be8;
          bVar2 = *local_d8 == 0x736a2e7865646f63;
        }
        if (local_e0 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_d8,local_e0,1);
        }
        goto LAB_1007c6c10;
      }
    }
    bVar2 = false;
    goto LAB_1007c6c10;
  }
  bVar2 = true;
LAB_1007c6c10:
  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar6,uVar4,1);
  return bVar2;
}

