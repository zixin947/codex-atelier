/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* collect_cwd_backfill_candidates @ 1003b1444 requested 1003b1444 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::codex_project_state::collect_cwd_backfill_candidates */

void codexmate_lib::core::relay::codex_project_state::collect_cwd_backfill_candidates
               (undefined8 *param_1,undefined8 param_2,long param_3,long param_4,long param_5,long param_6)

{
  char cVar1;
  size_t sVar2;
  ushort uVar3;
  long lVar4;
  undefined **ppuVar5;
  undefined **ppuVar6;
  undefined **ppuVar7;
  undefined **ppuVar8;
  undefined **ppuVar9;
  undefined **ppuVar10;
  undefined **ppuVar11;
  int iVar12;
  char *pcVar13;
  char *pcVar14;
  long lVar15;
  undefined8 *puVar16;
  long lVar17;
  undefined8 *puVar18;
  undefined8 *puVar19;
  long lVar20;
  long lVar21;
  undefined *puVar22;
  undefined **ppuVar23;
  undefined **ppuVar24;
  ulong uVar25;
  ulong uVar26;
  undefined8 uVar27;
  undefined **ppuVar28;
  undefined **ppuVar29;
  undefined **extraout_d0;
  undefined **ppuVar30;
  undefined **ppuVar31;
  undefined **ppuVar32;
  undefined **ppuVar33;
  undefined **local_268;
  undefined **local_260;
  long local_1f0;
  undefined *local_1d8;
  undefined8 uStack_1d0;
  undefined *local_1c8;
  undefined8 uStack_1c0;
  long local_1b8;
  undefined8 local_1b0;
  undefined8 uStack_1a8;
  undefined **local_1a0;
  undefined **ppuStack_198;
  undefined **ppuStack_190;
  undefined **ppuStack_188;
  undefined **local_180;
  undefined **ppuStack_178;
  undefined **ppuStack_170;
  sqlite3_stmt *local_168;
  undefined **local_160;
  undefined **ppuStack_158;
  undefined **local_150;
  undefined **ppuStack_148;
  undefined **local_140;
  undefined **ppuStack_138;
  undefined **local_130;
  sqlite3_stmt *psStack_128;
  long local_120;
  long lStack_118;
  long local_110;
  undefined ***local_108;
  long local_100;
  undefined **local_f8;
  undefined **ppuStack_f0;
  undefined **ppuStack_e8;
  undefined **ppuStack_e0;
  undefined **local_d8;
  undefined **ppuStack_d0;
  undefined **ppuStack_c8;
  undefined **local_c0;
  sqlite3_stmt *psStack_b8;
  char local_90 [48];

  lVar15 = param_4;
  lVar21 = param_3;
  if (param_3 != 0) {
    do {
      uVar3 = *(ushort *)(lVar21 + 0x112);
      lVar20 = (ulong)uVar3 * 0x18;
      uVar26 = 0xffffffffffffffff;
      lVar17 = lVar21 + 8;
      do {
        uVar25 = (ulong)uVar3;
        if (lVar20 == 0) goto LAB_1003b14f8;
        sVar2 = *(ulong *)(lVar17 + 0x10);
        lVar4 = 2 - sVar2;
        if (2 < sVar2 || lVar4 == 0) {
          sVar2 = 2;
        }
        iVar12 = _memcmp("id/price/monthly_amount/price/amount/unit_amount/monthly_amount/amount",*(void **)(lVar17 + 8),sVar2);
        if (iVar12 != 0) {
          lVar4 = (long)iVar12;
        }
        cVar1 = 0 < lVar4;
        if (lVar4 < 0) {
          cVar1 = -1;
        }
        uVar26 = uVar26 + 1;
        lVar20 = lVar20 + -0x18;
        lVar17 = lVar17 + 0x18;
      } while (cVar1 == '\x01');
      uVar25 = uVar26;
      lVar17 = param_4;
      lVar20 = param_3;
      if (cVar1 == '\0') goto LAB_1003b1520;
LAB_1003b14f8:
      if (lVar15 == 0) break;
      lVar15 = lVar15 + -1;
      lVar21 = *(long *)(lVar21 + uVar25 * 8 + 0x118);
    } while( true );
  }
LAB_1003b1804:
  param_1[2] = 8;
  param_1[3] = 0;
  param_1[1] = 0;
  *param_1 = 0x8000000000000016;
  return;
LAB_1003b1520:
  uVar3 = *(ushort *)(lVar20 + 0x112);
  lVar21 = (ulong)uVar3 * 0x18;
  uVar26 = 0xffffffffffffffff;
  lVar15 = lVar20 + 8;
  do {
    uVar25 = (ulong)uVar3;
    if (lVar21 == 0) goto LAB_1003b158c;
    sVar2 = *(ulong *)(lVar15 + 0x10);
    lVar4 = 3 - sVar2;
    if (3 < sVar2 || lVar4 == 0) {
      sVar2 = 3;
    }
    iVar12 = _memcmp(&DAT_10131110e,*(void **)(lVar15 + 8),sVar2);
    if (iVar12 != 0) {
      lVar4 = (long)iVar12;
    }
    cVar1 = 0 < lVar4;
    if (lVar4 < 0) {
      cVar1 = -1;
    }
    uVar26 = uVar26 + 1;
    lVar21 = lVar21 + -0x18;
    lVar15 = lVar15 + 0x18;
  } while (cVar1 == '\x01');
  uVar25 = uVar26;
  lVar15 = param_4;
  lVar21 = param_3;
  if (cVar1 == '\0') goto LAB_1003b15b4;
LAB_1003b158c:
  if (lVar17 == 0) goto LAB_1003b1804;
  lVar17 = lVar17 + -1;
  lVar20 = *(long *)(lVar20 + uVar25 * 8 + 0x118);
  goto LAB_1003b1520;
LAB_1003b15b4:
  uVar3 = *(ushort *)(lVar21 + 0x112);
  lVar20 = (ulong)uVar3 * 0x18;
  uVar26 = 0xffffffffffffffff;
  lVar17 = lVar21 + 8;
  do {
    uVar25 = (ulong)uVar3;
    if (lVar20 == 0) goto LAB_1003b1620;
    sVar2 = *(ulong *)(lVar17 + 0x10);
    lVar4 = 0xc - sVar2;
    if (0xc < sVar2 || lVar4 == 0) {
      sVar2 = 0xc;
    }
    iVar12 = _memcmp(&DAT_1013111d4,*(void **)(lVar17 + 8),sVar2);
    if (iVar12 != 0) {
      lVar4 = (long)iVar12;
    }
    cVar1 = 0 < lVar4;
    if (lVar4 < 0) {
      cVar1 = -1;
    }
    uVar26 = uVar26 + 1;
    lVar20 = lVar20 + -0x18;
    lVar17 = lVar17 + 0x18;
  } while (cVar1 == '\x01');
  lVar17 = param_3;
  uVar25 = uVar26;
  lVar20 = param_4;
  if (cVar1 == '\0') goto LAB_1003b1648;
LAB_1003b1620:
  if (lVar15 == 0) goto LAB_1003b1804;
  lVar15 = lVar15 + -1;
  lVar21 = *(long *)(lVar21 + uVar25 * 8 + 0x118);
  goto LAB_1003b15b4;
LAB_1003b1648:
  uVar3 = *(ushort *)(lVar17 + 0x112);
  lVar21 = (ulong)uVar3 * 0x18;
  uVar26 = 0xffffffffffffffff;
  lVar15 = lVar17 + 8;
  do {
    uVar25 = (ulong)uVar3;
    if (lVar21 == 0) goto LAB_1003b16b4;
    sVar2 = *(ulong *)(lVar15 + 0x10);
    lVar4 = 8 - sVar2;
    if (8 < sVar2 || lVar4 == 0) {
      sVar2 = 8;
    }
    iVar12 = _memcmp("archivedSKILL.mdmarkdown",*(void **)(lVar15 + 8),sVar2);
    if (iVar12 != 0) {
      lVar4 = (long)iVar12;
    }
    cVar1 = 0 < lVar4;
    if (lVar4 < 0) {
      cVar1 = -1;
    }
    uVar26 = uVar26 + 1;
    lVar21 = lVar21 + -0x18;
    lVar15 = lVar15 + 0x18;
  } while (cVar1 == '\x01');
  uVar25 = uVar26;
  if (cVar1 == '\0') {
    local_1d8 = &DAT_1013111e0;
    uStack_1d0 = 0x19;
    goto LAB_1003b16d4;
  }
LAB_1003b16b4:
  if (lVar20 == 0) goto LAB_1003b1d8c;
  lVar17 = *(long *)(lVar17 + uVar25 * 8 + 0x118);
  lVar20 = lVar20 + -1;
  goto LAB_1003b1648;
LAB_1003b1d8c:
  local_1d8 = &DAT_1013111f9;
  uStack_1d0 = 5;
LAB_1003b16d4:
  puVar22 = &DAT_101311218;
  uVar27 = 2;
  local_1f0 = param_4;
  while( true ) {
    uVar3 = *(ushort *)(param_3 + 0x112);
    lVar21 = (ulong)uVar3 * 0x18;
    uVar26 = 0xffffffffffffffff;
    lVar15 = param_3 + 8;
    do {
      uVar25 = (ulong)uVar3;
      if (lVar21 == 0) goto LAB_1003b175c;
      sVar2 = *(ulong *)(lVar15 + 0x10);
      lVar17 = 6 - sVar2;
      if (6 < sVar2 || lVar17 == 0) {
        sVar2 = 6;
      }
      iVar12 = _memcmp(&DAT_1013111fe,*(void **)(lVar15 + 8),sVar2);
      if (iVar12 != 0) {
        lVar17 = (long)iVar12;
      }
      cVar1 = 0 < lVar17;
      if (lVar17 < 0) {
        cVar1 = -1;
      }
      uVar26 = uVar26 + 1;
      lVar21 = lVar21 + -0x18;
      lVar15 = lVar15 + 0x18;
    } while (cVar1 == '\x01');
    uVar25 = uVar26;
    if (cVar1 == '\0') break;
LAB_1003b175c:
    if (local_1f0 == 0) goto LAB_1003b1784;
    param_3 = *(long *)(param_3 + uVar25 * 8 + 0x118);
    local_1f0 = local_1f0 + -1;
  }
  puVar22 = &DAT_101311204;
  uVar27 = 0x14;
LAB_1003b1784:
  local_f8 = &local_1c8;
  ppuStack_f0 = (undefined **)&<&T_as_core::fmt::Display>::fmt;
  ppuStack_e8 = &local_1d8;
  ppuStack_e0 = (undefined **)&<&T_as_core::fmt::Display>::fmt;
  local_1c8 = puVar22;
  uStack_1c0 = uVar27;
  alloc::fmt::format::format_inner(&local_1b8,s_SELECT_id__rollout_path__FROM_th_101523949,&local_f8);
  rusqlite::Connection::prepare_with_flags(&local_f8,param_2,local_1b0,uStack_1a8);
  ppuStack_158 = ppuStack_e8;
  local_160 = ppuStack_f0;
  ppuStack_148 = local_d8;
  local_150 = ppuStack_e0;
  ppuStack_138 = ppuStack_c8;
  local_140 = ppuStack_d0;
  psStack_128 = psStack_b8;
  local_130 = local_c0;
  if ((int)local_f8 == 1) {
    param_1[1] = ppuStack_e8;
    *param_1 = ppuStack_f0;
    param_1[3] = local_d8;
    param_1[2] = ppuStack_e0;
    param_1[5] = ppuStack_c8;
    param_1[4] = ppuStack_d0;
    param_1[7] = psStack_b8;
    param_1[6] = local_c0;
  }
  else {
    ppuStack_198 = ppuStack_e8;
    local_1a0 = ppuStack_f0;
    ppuStack_188 = local_d8;
    ppuStack_190 = ppuStack_e0;
    ppuStack_178 = ppuStack_c8;
    local_180 = ppuStack_d0;
    local_168 = psStack_b8;
    ppuStack_170 = local_c0;
    iVar12 = _sqlite3_bind_parameter_count(psStack_b8);
    if (iVar12 == 0) {
      local_108 = &local_1a0;
      local_120 = 0;
      lStack_118 = 8;
      local_110 = 0;
      puVar19 = (undefined8 *)((ulong)&local_160 | 1);
      puVar16 = (undefined8 *)((ulong)local_90 | 1);
      local_100 = 0;
      while (<rusqlite::row::Rows_as_fallible_streaming_iterator::FallibleStreamingIterator>::advance(&local_f8,&local_108), ppuVar23 = local_c0,
            ppuVar24 = local_d8, ppuVar28 = ppuStack_e0, ppuVar29 = local_f8, ppuVar30 = ppuStack_f0, ppuVar31 = ppuStack_e8, ppuVar32 = ppuStack_d0,
            ppuVar33 = ppuStack_c8, local_f8 == (undefined **)0x8000000000000016) {
        if (local_100 == 0) {
LAB_1003b1d9c:
          ::core::ptr::drop_in_place<rusqlite::row::Rows>(&local_108);
          param_1[2] = lStack_118;
          param_1[1] = local_120;
          param_1[3] = local_110;
          *param_1 = 0x8000000000000016;
          ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_1a0);
          goto LAB_1003b1f28;
        }
        rusqlite::row::Row::get(&local_f8,&local_100,0,&PTR_s_src_core_relay_codex_project_sta_10181bac8);
        ppuVar9 = ppuStack_e0;
        ppuVar7 = ppuStack_e8;
        ppuVar6 = ppuStack_f0;
        ppuVar23 = local_c0;
        ppuVar24 = local_d8;
        ppuVar28 = ppuStack_e0;
        ppuVar29 = local_f8;
        ppuVar30 = ppuStack_f0;
        ppuVar31 = ppuStack_e8;
        ppuVar32 = ppuStack_d0;
        ppuVar33 = ppuStack_c8;
        if (local_f8 != (undefined **)0x8000000000000016) break;
        rusqlite::row::Row::get(&local_f8,&local_100,1,&PTR_s_src_core_relay_codex_project_sta_10181bae0);
        ppuVar32 = ppuStack_d0;
        ppuVar24 = local_d8;
        ppuVar10 = ppuStack_e0;
        ppuVar8 = ppuStack_e8;
        ppuVar31 = ppuStack_f0;
        ppuVar29 = local_f8;
        if (local_f8 != (undefined **)0x8000000000000016) {
          ppuVar23 = local_c0;
          ppuVar28 = ppuVar10;
          ppuVar30 = ppuVar31;
          ppuVar31 = ppuVar8;
          ppuVar33 = ppuStack_c8;
          if (ppuVar6 != (undefined **)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar7,ppuVar6,1);
          }
          break;
        }
        rusqlite::row::Row::get(&local_f8,&local_100,2,&PTR_s_src_core_relay_codex_project_sta_10181baf8);
        ppuVar32 = ppuStack_d0;
        ppuVar24 = local_d8;
        ppuVar11 = ppuStack_e0;
        ppuVar33 = ppuStack_e8;
        ppuVar5 = ppuStack_f0;
        ppuVar29 = local_f8;
        if (local_f8 != (undefined **)0x8000000000000016) {
          if (ppuVar31 != (undefined **)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar31,ppuVar8,ppuVar31,1);
          }
          ppuVar23 = local_c0;
          ppuVar28 = ppuVar11;
          ppuVar30 = ppuVar5;
          ppuVar31 = ppuVar33;
          ppuVar33 = ppuStack_c8;
          if (ppuVar6 != (undefined **)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar7,ppuVar6,1);
          }
          break;
        }
        ppuVar23 = ppuStack_e0;
        ppuVar24 = ppuVar10;
        ppuVar28 = ppuVar8;
        ppuVar29 = ppuVar7;
        ppuVar30 = ppuVar9;
        ppuVar32 = ppuStack_f0;
        if (ppuVar6 == (undefined **)&DAT_8000000000000000) break;
        if (ppuVar6 == (undefined **)&DAT_8000000000000001) goto LAB_1003b1d9c;
        lVar15 = param_5;
        lVar21 = param_6;
        if (param_5 != 0) {
          do {
            uVar3 = *(ushort *)(lVar15 + 0x112);
            lVar20 = (ulong)uVar3 * 0x18;
            uVar26 = 0xffffffffffffffff;
            lVar17 = lVar15 + 8;
            do {
              uVar25 = (ulong)uVar3;
              if (lVar20 == 0) goto LAB_1003b1a0c;
              ppuVar24 = *(undefined ***)(lVar17 + 0x10);
              ppuVar29 = ppuVar9;
              if (ppuVar24 <= ppuVar9) {
                ppuVar29 = ppuVar24;
              }
              iVar12 = _memcmp(ppuVar7,*(void **)(lVar17 + 8),(size_t)ppuVar29);
              lVar4 = (long)ppuVar9 - (long)ppuVar24;
              if (iVar12 != 0) {
                lVar4 = (long)iVar12;
              }
              cVar1 = 0 < lVar4;
              if (lVar4 < 0) {
                cVar1 = -1;
              }
              uVar26 = uVar26 + 1;
              lVar20 = lVar20 + -0x18;
              lVar17 = lVar17 + 0x18;
            } while (cVar1 == '\x01');
            uVar25 = uVar26;
            if (cVar1 == '\0') goto LAB_1003b1b14;
LAB_1003b1a0c:
            if (lVar21 == 0) break;
            lVar15 = *(long *)(lVar15 + uVar25 * 8 + 0x118);
            lVar21 = lVar21 + -1;
          } while( true );
        }
        if (ppuVar11 < &DAT_0000000b) {
          if ((ppuVar11 != (undefined **)&DAT_0000000a) || (*ppuVar33 != (undefined *)0x6e65676162757322 || *(short *)(ppuVar33 + 1) != 0x2274))
          goto LAB_1003b1a94;
LAB_1003b1b14:
          ppuVar29 = ppuVar31;
          ppuVar24 = ppuVar31;
          ppuVar32 = ppuVar8;
          if (ppuVar5 != (undefined **)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar5,ppuVar33,ppuVar5,1);
          }
joined_r0x0001003b1b6c:
          ppuVar5 = ppuVar6;
          ppuVar33 = ppuVar7;
          if (ppuVar24 != (undefined **)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar29,ppuVar32,ppuVar24,1);
          }
        }
        else {
          ::core::str::pattern::StrSearcher::new(&local_f8,ppuVar5,ppuVar33,ppuVar11,&DAT_10131121a,10);
          <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_160,&local_f8);
          if (((ulong)local_160 & 1) != 0) goto LAB_1003b1b14;
LAB_1003b1a94:
          ppuVar29 = (undefined **)codex_thread_visibility::find_first_session_meta_line(&local_f8,ppuVar31,ppuVar8,ppuVar10);
          ppuVar23 = ppuStack_e0;
          ppuVar32 = ppuStack_e8;
          ppuVar24 = ppuStack_f0;
          if (local_f8 != (undefined **)&DAT_0000000b) {
            ppuVar29 = (undefined **)::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_f8);
LAB_1003b1b58:
            ppuVar24 = ppuVar5;
            ppuVar32 = ppuVar33;
            if (ppuVar31 != (undefined **)0x0) {
              ppuVar29 = (undefined **)__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar8,ppuVar31,1);
            }
            goto joined_r0x0001003b1b6c;
          }
          if ((long)ppuStack_f0 < -0x7ffffffffffffffe) goto LAB_1003b1b58;
          local_f8 = ppuStack_e8;
          ppuStack_f0 = ppuStack_e0;
          ppuStack_e8 = (undefined **)0x0;
          ppuStack_e0 = (undefined **)0x0;
          local_d8 = ppuVar32;
          ppuStack_d0 = ppuVar23;
          serde_json::de::from_trait(&local_160,&local_f8);
          if ((char)local_160 == '\x06') {
            ppuVar29 = (undefined **)::core::ptr::drop_in_place<serde_json::error::Error>(&ppuStack_158);
LAB_1003b1c24:
            if (ppuVar24 != (undefined **)0x0) {
              ppuVar29 = (undefined **)__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar32,ppuVar24,1);
            }
            goto LAB_1003b1b58;
          }
          uVar27 = *puVar19;
          puVar16[1] = puVar19[1];
          *puVar16 = uVar27;
          uVar27 = *(undefined8 *)((long)puVar19 + 0xf);
          *(undefined8 *)((long)puVar16 + 0x17) = *(undefined8 *)((long)puVar19 + 0x17);
          *(undefined8 *)((long)puVar16 + 0xf) = uVar27;
          local_90[0] = (char)local_160;
          pcVar13 = (char *)<str_as_serde_json::value::index::Index>::index_into(&_anon_164d03a2dfc98f969363eaf1c50504fd_986,7,local_90);
          if (((((pcVar13 == (char *)0x0) || (*pcVar13 != '\x05')) ||
               (pcVar14 = (char *)alloc::collections::btree::map::BTreeMap<K,V,A>::get
                                            (pcVar13 + 8,"id/price/monthly_amount/price/amount/unit_amount/monthly_amount/amount",2), pcVar14 == (char *)0x0))
              || ((*pcVar14 != '\x03' || (*(undefined ***)(pcVar14 + 0x18) != ppuVar9)))) ||
             ((iVar12 = _memcmp(*(void **)(pcVar14 + 0x10),ppuVar7,(size_t)ppuVar9), iVar12 != 0 ||
              ((pcVar13 = (char *)alloc::collections::btree::map::BTreeMap<K,V,A>::get(pcVar13 + 8,&DAT_10131110e,3), pcVar13 == (char *)0x0 ||
               (*pcVar13 != '\x03')))))) {
            ppuVar29 = (undefined **)::core::ptr::drop_in_place<serde_json::value::Value>(local_90);
            goto LAB_1003b1c24;
          }
          normalize_codex_path(&local_f8,*(undefined8 *)(pcVar13 + 0x10),*(undefined8 *)(pcVar13 + 0x18));
          if (ppuStack_e8 == (undefined **)0x0) {
            if (local_f8 != (undefined **)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuStack_f0,local_f8,1);
            }
            ppuVar23 = (undefined **)&DAT_8000000000000000;
          }
          else {
            local_268 = ppuStack_e8;
            local_260 = ppuStack_f0;
            ppuVar23 = local_f8;
          }
          ppuVar29 = (undefined **)::core::ptr::drop_in_place<serde_json::value::Value>(local_90);
          if (ppuVar24 != (undefined **)0x0) {
            ppuVar29 = (undefined **)__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar32,ppuVar24,1);
          }
          if (ppuVar23 == (undefined **)&DAT_8000000000000000) goto LAB_1003b1b58;
          uVar26 = is_codex_generated_projectless_dir(local_260,local_268);
          lVar15 = local_110;
          if ((uVar26 & 1) != 0) {
            ppuVar29 = extraout_d0;
            if (ppuVar23 != (undefined **)0x0) {
              ppuVar29 = (undefined **)__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_260,ppuVar23,1);
            }
            goto LAB_1003b1b58;
          }
          local_f8 = ppuVar6;
          ppuStack_f0 = ppuVar7;
          ppuStack_e8 = ppuVar9;
          local_d8 = local_260;
          ppuStack_d0 = local_268;
          ppuStack_e0 = ppuVar23;
          if (local_110 == local_120) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_120);
          }
          puVar18 = (undefined8 *)(lStack_118 + lVar15 * 0x30);
          puVar18[1] = ppuStack_f0;
          *puVar18 = local_f8;
          puVar18[3] = ppuStack_e0;
          puVar18[2] = ppuStack_e8;
          puVar18[5] = ppuStack_d0;
          puVar18[4] = local_d8;
          local_110 = lVar15 + 1;
          if (ppuVar31 != (undefined **)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar8,ppuVar31,1);
          }
        }
        if (ppuVar5 != (undefined **)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar33,ppuVar5,1);
        }
      }
      *param_1 = ppuVar29;
      param_1[2] = ppuVar31;
      param_1[1] = ppuVar30;
      param_1[3] = ppuVar28;
      param_1[4] = ppuVar24;
      param_1[6] = ppuVar33;
      param_1[5] = ppuVar32;
      param_1[7] = ppuVar23;
      ::core::ptr::drop_in_place<rusqlite::row::Rows>(&local_108);
      if (local_110 != 0) {
        puVar16 = (undefined8 *)(lStack_118 + 0x20);
        lVar15 = local_110;
        do {
          if (puVar16[-4] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar16[-3],puVar16[-4],1);
          }
          if (puVar16[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar16,puVar16[-1],1);
          }
          puVar16 = puVar16 + 6;
          lVar15 = lVar15 + -1;
        } while (lVar15 != 0);
      }
      if (local_120 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_118,local_120 * 0x30,8);
      }
    }
    else {
      *param_1 = 0x8000000000000013;
      param_1[1] = 0;
      param_1[2] = (long)iVar12;
    }
    ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_1a0);
  }
LAB_1003b1f28:
  if (local_1b8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1b0,local_1b8,1);
  }
  return;
}
