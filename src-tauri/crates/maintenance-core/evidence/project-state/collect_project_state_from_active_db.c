/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* collect_project_state_from_active_db @ 1003b25a4 requested 1003b25a4 */

/* codexmate_lib::core::relay::codex_project_state::collect_project_state_from_active_db */

void codexmate_lib::core::relay::codex_project_state::collect_project_state_from_active_db
               (undefined8 *param_1,undefined8 param_2,sqlite3_stmt *param_3,undefined8 param_4)

{
  char cVar1;
  sqlite3_stmt *psVar2;
  long lVar3;
  sqlite3_stmt *psVar4;
  sqlite3_stmt *psVar5;
  sqlite3_stmt *psVar6;
  sqlite3_stmt *psVar7;
  sqlite3_stmt *psVar8;
  sqlite3_stmt *psVar9;
  sqlite3_stmt *psVar10;
  long lVar11;
  sqlite3_stmt *psVar12;
  sqlite3_stmt *psVar13;
  int iVar14;
  uint uVar15;
  undefined8 *puVar16;
  sqlite3_stmt *psVar17;
  sqlite3_stmt *psVar18;
  sqlite3_stmt *psVar19;
  ulong uVar20;
  sqlite3_stmt *psVar21;
  sqlite3_stmt *psVar22;
  sqlite3_stmt *psVar23;
  sqlite3_stmt *psVar24;
  sqlite3_stmt *psVar25;
  ulong uVar26;
  long lVar27;
  sqlite3_stmt *psVar28;
  undefined8 uVar29;
  sqlite3_stmt *psVar30;
  sqlite3_stmt *local_370;
  sqlite3_stmt *local_350;
  sqlite3_stmt *local_348;
  sqlite3_stmt *local_330;
  sqlite3_stmt *local_310;
  sqlite3_stmt *psStack_308;
  sqlite3_stmt *psStack_300;
  sqlite3_stmt *psStack_2f8;
  sqlite3_stmt *local_2f0;
  sqlite3_stmt *psStack_2e8;
  sqlite3_stmt *psStack_2e0;
  sqlite3_stmt *psStack_2d8;
  sqlite3_stmt *local_2d0;
  undefined8 uStack_2c8;
  undefined8 uStack_2c0;
  undefined8 uStack_2b8;
  undefined8 local_2b0;
  sqlite3_stmt *local_2a0;
  sqlite3_stmt *psStack_298;
  sqlite3_stmt *local_290;
  sqlite3_stmt *local_288;
  sqlite3_stmt *psStack_280;
  sqlite3_stmt *psStack_278;
  sqlite3_stmt *psStack_270;
  sqlite3_stmt *local_268;
  sqlite3_stmt *local_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 local_240;
  char cStack_238;
  undefined8 local_230;
  undefined8 uStack_228;
  undefined8 local_220;
  sqlite3_stmt *local_218;
  sqlite3_stmt *psStack_210;
  sqlite3_stmt *local_208;
  undefined *local_200;
  undefined8 uStack_1f8;
  undefined *local_1f0;
  undefined8 uStack_1e8;
  sqlite3_stmt *local_1e0;
  sqlite3_stmt *psStack_1d8;
  sqlite3_stmt *local_1d0;
  sqlite3_stmt *psStack_1c8;
  sqlite3_stmt *local_1c0;
  sqlite3_stmt *psStack_1b8;
  sqlite3_stmt *local_1b0;
  sqlite3_stmt *psStack_1a8;
  sqlite3_stmt *local_198;
  sqlite3_stmt *local_190;
  sqlite3_stmt *local_188;
  undefined8 auStack_180 [2];
  undefined8 local_170;
  sqlite3_stmt *local_168;
  sqlite3_stmt *local_160;
  undefined8 local_158;
  long local_150;
  sqlite3_stmt **local_148;
  long local_140;
  long local_138;
  undefined8 local_130;
  long local_128;
  sqlite3_stmt *local_120;
  sqlite3_stmt *local_118;
  sqlite3_stmt *local_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  sqlite3_stmt *local_f0;
  sqlite3_stmt *psStack_e8;
  sqlite3_stmt *local_e0;
  sqlite3_stmt *local_d8;
  sqlite3_stmt *local_d0;
  sqlite3_stmt *psStack_c8;
  sqlite3_stmt *local_c0;
  sqlite3_stmt *psStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  sqlite3_stmt *local_90;
  sqlite3_stmt *psStack_88;

  active_db_path(&local_2a0);
  psVar9 = local_290;
  psVar8 = psStack_298;
  psVar7 = local_2a0;
  if (local_2a0 == (sqlite3_stmt *)&DAT_8000000000000000) {
    *param_1 = 0;
    param_1[1] = 8;
    param_1[2] = 0;
    param_1[3] = 0;
    param_1[9] = 0;
    param_1[10] = 0;
    param_1[6] = 0;
    param_1[7] = 0;
    param_1[4] = 8;
    param_1[5] = 0;
    return;
  }
  rusqlite::Connection::open_with_flags(&local_2a0,psStack_298,local_290,0x8001);
  if (cStack_238 == '\x03') {
    psStack_e8 = psStack_298;
    local_f0 = local_2a0;
    local_d8 = local_288;
    local_e0 = local_290;
    psStack_c8 = psStack_278;
    local_d0 = psStack_280;
    psStack_b8 = local_268;
    local_c0 = psStack_270;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar16 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar16 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    local_90 = psVar8;
    psStack_88 = psVar9;
    local_1e0 = (sqlite3_stmt *)&local_90;
    psStack_1d8 = (sqlite3_stmt *)<std::path::Display_as_core::fmt::Display>::fmt;
    local_1d0 = (sqlite3_stmt *)&local_f0;
    psStack_1c8 = (sqlite3_stmt *)<rusqlite::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_230,s___sqlite_open_readonly__101523bf1,&local_1e0);
    puVar16[1] = uStack_228;
    *puVar16 = local_230;
    puVar16[2] = local_220;
    psStack_1d8 = (sqlite3_stmt *)0x0;
    *param_1 = 0;
    param_1[1] = 8;
    param_1[7] = 0;
    param_1[9] = 0;
    param_1[8] = local_1e0;
    param_1[10] = 0;
    param_1[3] = 1;
    param_1[2] = 0;
    param_1[4] = puVar16;
    param_1[6] = 0;
    param_1[5] = 1;
    ::core::ptr::drop_in_place<rusqlite::error::Error>(&local_f0);
    goto LAB_1003b3290;
  }
  uStack_2c8 = uStack_258;
  local_2d0 = local_260;
  uStack_2b8 = uStack_248;
  uStack_2c0 = uStack_250;
  local_2b0 = local_240;
  psStack_308 = psStack_298;
  local_310 = local_2a0;
  psStack_2f8 = local_288;
  psStack_300 = local_290;
  psStack_2e8 = psStack_278;
  local_2f0 = psStack_280;
  psStack_2d8 = local_268;
  psStack_2e0 = psStack_270;
  rusqlite::busy::<impl_rusqlite::Connection>::busy_timeout(&local_2a0,&local_310,0,500000000);
  if (local_2a0 != (sqlite3_stmt *)0x8000000000000016) {
    ::core::ptr::drop_in_place<rusqlite::error::Error>(&local_2a0);
  }
  sqlite_table_columns(&local_2a0,&local_310);
  psVar18 = local_288;
  psVar10 = local_290;
  psVar22 = psStack_298;
  if (local_2a0 == (sqlite3_stmt *)0x8000000000000016) {
    local_218 = psStack_298;
    psStack_210 = local_290;
    local_208 = local_288;
    psVar30 = local_290;
    psVar25 = psStack_298;
    local_350 = param_3;
    if (psStack_298 != (sqlite3_stmt *)0x0) {
      do {
        uVar26 = (ulong)*(ushort *)(psVar25 + 0x112);
        lVar27 = uVar26 * 0x18;
        uVar20 = 0xffffffffffffffff;
        psVar17 = psVar25 + 8;
        do {
          if (lVar27 == 0) goto LAB_1003b27f4;
          psVar23 = *(sqlite3_stmt **)(psVar17 + 0x10);
          lVar3 = 2 - (long)psVar23;
          if ((sqlite3_stmt *)0x2 < psVar23 || lVar3 == 0) {
            psVar23 = (sqlite3_stmt *)0x2;
          }
          iVar14 = _memcmp("id/price/monthly_amount/price/amount/unit_amount/monthly_amount/amount",*(sqlite3_stmt **)(psVar17 + 8),(size_t)psVar23);
          if (iVar14 != 0) {
            lVar3 = (long)iVar14;
          }
          cVar1 = 0 < lVar3;
          if (lVar3 < 0) {
            cVar1 = -1;
          }
          uVar20 = uVar20 + 1;
          lVar27 = lVar27 + -0x18;
          psVar17 = psVar17 + 0x18;
        } while (cVar1 == '\x01');
        uVar26 = uVar20;
        psVar17 = psVar10;
        psVar23 = psVar22;
        if (cVar1 == '\0') goto LAB_1003b282c;
LAB_1003b27f4:
        if (psVar30 == (sqlite3_stmt *)0x0) goto LAB_1003b2e74;
        psVar30 = psVar30 + -1;
        psVar25 = *(sqlite3_stmt **)(psVar25 + uVar26 * 8 + 0x118);
      } while( true );
    }
    local_348 = (sqlite3_stmt *)0x0;
    psVar19 = (sqlite3_stmt *)0x0;
    psVar25 = (sqlite3_stmt *)0x0;
    psVar30 = (sqlite3_stmt *)0x0;
    psVar28 = (sqlite3_stmt *)0x0;
    local_2a0 = (sqlite3_stmt *)0x0;
    psVar18 = (sqlite3_stmt *)0x0;
    psVar24 = (sqlite3_stmt *)&SUB_00000008;
    local_330 = (sqlite3_stmt *)&SUB_00000008;
    psVar21 = psStack_298;
    goto LAB_1003b30b8;
  }
  goto LAB_1003b3138;
LAB_1003b282c:
  uVar26 = (ulong)*(ushort *)(psVar23 + 0x112);
  lVar27 = uVar26 * 0x18;
  uVar20 = 0xffffffffffffffff;
  psVar30 = psVar23 + 8;
  do {
    if (lVar27 == 0) goto LAB_1003b2898;
    psVar25 = *(sqlite3_stmt **)(psVar30 + 0x10);
    lVar3 = 3 - (long)psVar25;
    if ((sqlite3_stmt *)0x3 < psVar25 || lVar3 == 0) {
      psVar25 = (sqlite3_stmt *)0x3;
    }
    iVar14 = _memcmp(&DAT_10131110e,*(sqlite3_stmt **)(psVar30 + 8),(size_t)psVar25);
    if (iVar14 != 0) {
      lVar3 = (long)iVar14;
    }
    cVar1 = 0 < lVar3;
    if (lVar3 < 0) {
      cVar1 = -1;
    }
    uVar20 = uVar20 + 1;
    lVar27 = lVar27 + -0x18;
    psVar30 = psVar30 + 0x18;
  } while (cVar1 == '\x01');
  psVar30 = psVar22;
  uVar26 = uVar20;
  psVar25 = psVar10;
  if (cVar1 == '\0') goto LAB_1003b28bc;
LAB_1003b2898:
  if (psVar17 == (sqlite3_stmt *)0x0) goto LAB_1003b2e74;
  psVar17 = psVar17 + -1;
  psVar23 = *(sqlite3_stmt **)(psVar23 + uVar26 * 8 + 0x118);
  goto LAB_1003b282c;
LAB_1003b28bc:
  uVar26 = (ulong)*(ushort *)(psVar30 + 0x112);
  lVar27 = uVar26 * 0x18;
  uVar20 = 0xffffffffffffffff;
  psVar17 = psVar30 + 8;
  do {
    psVar23 = psVar10;
    psVar19 = psVar22;
    if (lVar27 == 0) goto LAB_1003b2928;
    psVar2 = *(sqlite3_stmt **)(psVar17 + 0x10);
    lVar3 = 8 - (long)psVar2;
    if ((sqlite3_stmt *)0x8 < psVar2 || lVar3 == 0) {
      psVar2 = (sqlite3_stmt *)0x8;
    }
    iVar14 = _memcmp("archivedSKILL.mdmarkdown",*(sqlite3_stmt **)(psVar17 + 8),(size_t)psVar2);
    if (iVar14 != 0) {
      lVar3 = (long)iVar14;
    }
    cVar1 = 0 < lVar3;
    if (lVar3 < 0) {
      cVar1 = -1;
    }
    uVar20 = uVar20 + 1;
    lVar27 = lVar27 + -0x18;
    psVar17 = psVar17 + 0x18;
  } while (cVar1 == '\x01');
  uVar26 = uVar20;
  if (cVar1 == '\0') {
    local_200 = &DAT_1013111e0;
    uStack_1f8 = 0x19;
    goto LAB_1003b295c;
  }
LAB_1003b2928:
  if (psVar25 == (sqlite3_stmt *)0x0) goto LAB_1003b2e94;
  psVar30 = *(sqlite3_stmt **)(psVar30 + uVar26 * 8 + 0x118);
  psVar25 = psVar25 + -1;
  goto LAB_1003b28bc;
LAB_1003b2e94:
  local_200 = &DAT_1013111f9;
  uStack_1f8 = 5;
LAB_1003b295c:
  do {
    uVar26 = (ulong)*(ushort *)(psVar19 + 0x112);
    lVar27 = uVar26 * 0x18;
    uVar20 = 0xffffffffffffffff;
    psVar30 = psVar19 + 8;
    do {
      if (lVar27 == 0) goto LAB_1003b29c8;
      psVar25 = *(sqlite3_stmt **)(psVar30 + 0x10);
      lVar3 = 0xd - (long)psVar25;
      if ((sqlite3_stmt *)0xd < psVar25 || lVar3 == 0) {
        psVar25 = (sqlite3_stmt *)0xd;
      }
      iVar14 = _memcmp(&DAT_101311224,*(sqlite3_stmt **)(psVar30 + 8),(size_t)psVar25);
      if (iVar14 != 0) {
        lVar3 = (long)iVar14;
      }
      cVar1 = 0 < lVar3;
      if (lVar3 < 0) {
        cVar1 = -1;
      }
      uVar20 = uVar20 + 1;
      lVar27 = lVar27 + -0x18;
      psVar30 = psVar30 + 0x18;
    } while (cVar1 == '\x01');
    uVar26 = uVar20;
    if (cVar1 == '\0') {
      local_1f0 = &DAT_101311231;
      uStack_1e8 = 0x1a;
      goto LAB_1003b29e8;
    }
LAB_1003b29c8:
    if (psVar23 == (sqlite3_stmt *)0x0) goto LAB_1003b2ea4;
    psVar23 = psVar23 + -1;
    psVar19 = *(sqlite3_stmt **)(psVar19 + uVar26 * 8 + 0x118);
  } while( true );
LAB_1003b2e74:
  local_330 = (sqlite3_stmt *)&SUB_00000008;
  psVar21 = (sqlite3_stmt *)0x0;
  psVar28 = (sqlite3_stmt *)0x0;
  psVar30 = (sqlite3_stmt *)0x0;
  psVar25 = (sqlite3_stmt *)0x0;
  psVar19 = (sqlite3_stmt *)0x0;
  local_348 = (sqlite3_stmt *)0x0;
  psVar24 = (sqlite3_stmt *)&SUB_00000008;
  goto LAB_1003b3090;
LAB_1003b2ea4:
  lVar27 = alloc::collections::btree::map::BTreeMap<K,V,A>::get(&local_218,&DAT_10131124b,10);
  local_1f0 = &UNK_10131126c;
  if (lVar27 != 0) {
    local_1f0 = &DAT_101311255;
  }
  uStack_1e8 = 0x17;
  if (lVar27 == 0) {
    uStack_1e8 = 1;
  }
LAB_1003b29e8:
  local_f0 = (sqlite3_stmt *)&local_200;
  psStack_e8 = (sqlite3_stmt *)&<&T_as_core::fmt::Display>::fmt;
  alloc::fmt::format::format_inner(&local_2a0,s__SELECT_COUNT____FROM_threads_WH_1015239bc,&local_f0);
  psVar25 = psStack_298;
  psVar30 = local_2a0;
  rusqlite::Connection::prepare_with_flags(&local_2a0,&local_310,psStack_298,local_290);
  psVar2 = local_268;
  psVar19 = psStack_278;
  psVar23 = local_288;
  psVar17 = psStack_298;
  psStack_e8 = local_290;
  local_f0 = psStack_298;
  local_d8 = psStack_280;
  local_e0 = local_288;
  psStack_c8 = psStack_270;
  local_d0 = psStack_278;
  psStack_b8 = local_260;
  local_c0 = local_268;
  if ((int)local_2a0 == 1) {
    psStack_1d8 = local_290;
    local_1e0 = psStack_298;
    psStack_1c8 = psStack_280;
    local_1d0 = local_288;
    psStack_1b8 = psStack_270;
    local_1c0 = psStack_278;
    psStack_1a8 = local_260;
    local_1b0 = local_268;
  }
  else {
    psStack_298 = local_290;
    local_2a0 = psVar17;
    local_288 = psStack_280;
    local_290 = psVar23;
    psStack_278 = psStack_270;
    psStack_280 = psVar19;
    local_268 = local_260;
    psStack_270 = psVar2;
    rusqlite::statement::Statement::query_row(&local_1e0,&local_2a0);
    ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_2a0);
  }
  psVar17 = psStack_1d8;
  psVar23 = local_1e0;
  if (local_1e0 != (sqlite3_stmt *)0x8000000000000016) {
    ::core::ptr::drop_in_place<rusqlite::error::Error>(&local_1e0);
  }
  psVar17 = (sqlite3_stmt *)((ulong)psVar17 & ((long)psVar17 >> 0x3f ^ 0xffffffffffffffffU));
  psVar19 = (sqlite3_stmt *)0x8000000000000016;
  if (psVar23 != (sqlite3_stmt *)0x8000000000000016) {
    psVar17 = (sqlite3_stmt *)0x0;
  }
  if (psVar30 != (sqlite3_stmt *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psVar25,psVar30,1);
  }
  local_2a0 = (sqlite3_stmt *)&local_200;
  psStack_298 = (sqlite3_stmt *)&<&T_as_core::fmt::Display>::fmt;
  local_290 = (sqlite3_stmt *)&local_1f0;
  local_288 = (sqlite3_stmt *)&<&T_as_core::fmt::Display>::fmt;
  alloc::fmt::format::format_inner(&local_f0,s__SELECT_id__cwd_FROM_threads_WHE_1015239e2,&local_2a0);
  psVar2 = psStack_e8;
  psVar23 = local_f0;
  rusqlite::Connection::prepare_with_flags(&local_2a0,&local_310,psStack_e8,local_e0);
  psVar25 = psStack_270;
  psVar30 = psStack_278;
  local_330 = psStack_280;
  if (((ulong)local_2a0 & 1) == 0) {
    local_1e0 = psStack_298;
    psStack_1d8 = local_290;
    local_1d0 = local_288;
    psStack_1c8 = psStack_280;
    local_1c0 = psStack_278;
    psStack_1b8 = psStack_270;
    local_1b0 = local_268;
    psStack_1a8 = local_260;
    iVar14 = _sqlite3_bind_parameter_count(local_260);
    psVar28 = (sqlite3_stmt *)(long)iVar14;
    if (iVar14 == 0) {
      lVar27 = 0;
      local_198 = (sqlite3_stmt *)0x0;
      local_190 = (sqlite3_stmt *)&SUB_00000008;
      local_188 = (sqlite3_stmt *)0x0;
      auStack_180[0] = 0;
      local_170 = 0;
      local_148 = &local_1e0;
      local_168 = (sqlite3_stmt *)0x0;
      local_158 = 0;
      local_150 = 0;
      local_140 = 0;
      while (<rusqlite::row::Rows_as_fallible_streaming_iterator::FallibleStreamingIterator>::advance(&local_2a0,&local_148), lVar3 = local_140,
            local_2a0 == (sqlite3_stmt *)0x8000000000000016) {
        if (local_140 == 0) {
          ::core::ptr::drop_in_place<rusqlite::row::Rows>(&local_148);
          if ((lVar27 != 0) && (1 < log::MAX_LOG_LEVEL_FILTER)) {
            local_f0 = (sqlite3_stmt *)&local_150;
            psStack_e8 = (sqlite3_stmt *)usize>::fmt;
            local_2a0 = (sqlite3_stmt *)&DAT_10131126d;
            psStack_298 = (sqlite3_stmt *)&DAT_0000002f;
            local_290 = (sqlite3_stmt *)&DAT_10131126d;
            local_288 = (sqlite3_stmt *)&DAT_0000002f;
            psStack_280 = (sqlite3_stmt *)&PTR_s_src_core_relay_codex_project_sta_10181bb10;
            log::__private_api::log(s__AiMaMi__project_state__skipped_5_101523a35,&local_f0,2,&local_2a0,param_3,param_4);
          }
          collect_cwd_backfill_candidates(&local_2a0,&local_310,psVar22,psVar10,param_3,param_4);
          psVar22 = local_288;
          if (local_2a0 == (sqlite3_stmt *)0x8000000000000016) {
            ::core::ptr::drop_in_place<alloc::vec::Vec<codexmate_lib::core::relay::codex_project_state::CwdBackfillCandidate>>(&psStack_298);
          }
          else {
            psStack_e8 = psStack_298;
            local_f0 = local_2a0;
            local_d8 = local_288;
            local_e0 = local_290;
            psStack_c8 = psStack_278;
            local_d0 = psStack_280;
            psStack_b8 = local_268;
            local_c0 = psStack_270;
            psVar22 = psStack_298;
            if (local_2a0 != (sqlite3_stmt *)0x8000000000000016) {
              ::core::ptr::drop_in_place<rusqlite::error::Error>(&local_f0);
              psVar22 = (sqlite3_stmt *)0x0;
            }
          }
          uVar29 = local_158;
          psVar25 = local_168;
          psVar18 = local_188;
          psVar10 = local_190;
          local_348 = local_198;
          <alloc::collections::btree::map::BTreeMap<K,V,A>as_core::ops::drop::Drop>::drop(auStack_180);
          ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_1e0);
          if (psVar23 != (sqlite3_stmt *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psVar2,psVar23,1);
          }
          <alloc::collections::btree::map::BTreeMap<K,V,A>as_core::ops::drop::Drop>::drop(&local_218);
          psVar28 = (sqlite3_stmt *)0x0;
          psVar30 = (sqlite3_stmt *)0x0;
          psVar12 = (sqlite3_stmt *)&SUB_00000008;
          psVar13 = local_160;
          psVar23 = local_2a0;
          psVar19 = psStack_298;
          psVar2 = local_290;
          psVar21 = local_288;
          psVar24 = psStack_280;
          psVar4 = psStack_278;
          psVar5 = psStack_270;
          psVar6 = local_268;
          local_2a0 = psVar10;
          psStack_298 = psVar18;
          psStack_270 = psVar25;
          goto joined_r0x0001003b3264;
        }
        rusqlite::row::Row::get(&local_2a0,&local_140,0,&PTR_s_src_core_relay_codex_project_sta_10181bb28);
        psVar4 = local_288;
        psVar24 = local_290;
        psVar21 = psStack_298;
        if (local_2a0 != (sqlite3_stmt *)0x8000000000000016) {
          local_350 = local_268;
          psVar19 = psStack_270;
          local_330 = local_288;
          psVar25 = psStack_278;
          psVar24 = local_2a0;
          psVar28 = local_290;
          psVar30 = psStack_280;
          goto LAB_1003b302c;
        }
        uVar15 = _sqlite3_column_count(*(sqlite3_stmt **)(lVar3 + 0x38));
        psVar19 = psStack_270;
        psVar25 = psStack_278;
        psVar30 = psStack_280;
        local_330 = local_288;
        psVar28 = local_290;
        if (uVar15 < 2) {
          local_350 = local_268;
          if (psVar21 != (sqlite3_stmt *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psVar24,psVar21,1);
          }
          psVar21 = (sqlite3_stmt *)0x1;
          psVar24 = (sqlite3_stmt *)0x800000000000000b;
          goto LAB_1003b302c;
        }
        rusqlite::statement::Statement::value_ref(&psStack_298,lVar3,1);
        if (psStack_298 < (sqlite3_stmt *)0x3) {
LAB_1003b2c60:
          if (psVar21 != (sqlite3_stmt *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psVar24,psVar21,1);
          }
          lVar27 = lVar27 + 1;
          local_150 = lVar27;
        }
        else {
          ::core::str::converts::from_utf8(&local_2a0,local_290,local_288);
          psVar25 = local_290;
          psVar30 = psStack_298;
          if ((int)local_2a0 == 1) goto LAB_1003b2c60;
          if ((long)local_290 < 0) {
            uVar29 = 0;
LAB_1003b32c8:
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(uVar29,local_370);
          }
          if (local_290 == (sqlite3_stmt *)0x0) {
            local_370 = (sqlite3_stmt *)0x1;
          }
          else {
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            local_370 = (sqlite3_stmt *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(psVar25,1);
            if (local_370 == (sqlite3_stmt *)0x0) {
              uVar29 = 1;
              local_370 = psVar25;
              goto LAB_1003b32c8;
            }
          }
          _memcpy(local_370,psVar30,(size_t)psVar25);
          normalize_codex_path(&local_138,local_370,psVar25);
          if (local_128 == 0) {
            if (local_138 != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,local_138,1);
            }
            if (psVar25 != (sqlite3_stmt *)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_370,psVar25,1);
            }
          }
          else {
            local_120 = psVar21;
            local_118 = psVar24;
            local_110 = psVar4;
            <alloc::string::String_as_core::clone::Clone>::clone(&local_108,&local_138);
            alloc::collections::btree::map::BTreeMap<K,V,A>::entry(&local_f0,&local_168,&local_120);
            if (local_f0 == (sqlite3_stmt *)&DAT_8000000000000000) {
              uVar20 = *(ulong *)(psStack_e8 + (long)local_d8 * 0x18 + 0x110);
              uVar29 = *(undefined8 *)(psStack_e8 + (long)local_d8 * 0x18 + 0x118);
              *(undefined8 *)(psStack_e8 + (long)local_d8 * 0x18 + 0x120) = local_f8;
              *(undefined8 *)(psStack_e8 + (long)local_d8 * 0x18 + 0x118) = uStack_100;
              *(undefined8 *)(psStack_e8 + (long)local_d8 * 0x18 + 0x110) = local_108;
              if ((uVar20 & 0x7fffffffffffffff) != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar29,uVar20,1);
              }
            }
            else {
              psStack_298 = psStack_e8;
              local_2a0 = local_f0;
              local_288 = local_d8;
              local_290 = local_e0;
              psStack_278 = psStack_c8;
              psStack_280 = local_d0;
              psStack_270 = local_c0;
              uStack_a8 = uStack_100;
              local_b0 = local_108;
              local_a0 = local_f8;
              alloc::collections::btree::map::entry::VacantEntry<K,V,A>::insert_entry(&local_90,&local_2a0,&local_b0);
            }
            lVar3 = local_128;
            uVar29 = local_130;
            uVar20 = is_codex_generated_projectless_dir(local_130,local_128);
            psVar21 = psVar25;
            psVar24 = local_370;
            if ((uVar20 & 1) == 0) {
              <alloc::string::String_as_core::clone::Clone>::clone(&local_2a0,&local_138);
              uVar20 = alloc::collections::btree::map::BTreeMap<K,V,A>::insert(auStack_180,&local_2a0);
              lVar11 = local_138;
              psVar30 = local_188;
              if ((uVar20 & 1) == 0) {
                if (local_188 == local_198) {
                  alloc::raw_vec::RawVec<T,A>::grow_one(&local_198);
                }
                psVar25 = local_190 + (long)psVar30 * 0x18;
                *(long *)psVar25 = lVar11;
                *(undefined8 *)(psVar25 + 8) = uVar29;
                *(long *)(psVar25 + 0x10) = lVar3;
                local_188 = psVar30 + 1;
                goto joined_r0x0001003b2d7c;
              }
            }
            if (local_138 != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar29,local_138,1);
            }
          }
joined_r0x0001003b2d7c:
          if (psVar21 != (sqlite3_stmt *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psVar24,psVar21,1);
          }
        }
      }
      local_350 = local_268;
      psVar19 = psStack_270;
      local_330 = local_288;
      psVar25 = psStack_278;
      psVar21 = psStack_298;
      psVar24 = local_2a0;
      psVar28 = local_290;
      psVar30 = psStack_280;
LAB_1003b302c:
      ::core::ptr::drop_in_place<rusqlite::row::Rows>(&local_148);
      <alloc::collections::btree::map::BTreeMap<K,V,A>as_core::ops::drop::Drop>::drop(&local_168);
      <alloc::collections::btree::map::BTreeMap<K,V,A>as_core::ops::drop::Drop>::drop(auStack_180);
      <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_198);
      if (local_198 != (sqlite3_stmt *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_190,(long)local_198 * 0x18,8);
      }
    }
    else {
      psVar21 = (sqlite3_stmt *)0x0;
      psVar24 = (sqlite3_stmt *)0x8000000000000013;
    }
    ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_1e0);
  }
  else {
    local_350 = local_260;
    psVar19 = local_268;
    psVar21 = local_290;
    psVar24 = psStack_298;
    psVar28 = local_288;
  }
  if (psVar23 != (sqlite3_stmt *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psVar2,psVar23,1);
  }
  local_348 = (sqlite3_stmt *)&DAT_8000000000000000;
LAB_1003b3090:
  psStack_298 = (sqlite3_stmt *)0x0;
  local_290 = psVar22;
  local_288 = psVar10;
  local_2a0 = (sqlite3_stmt *)0x1;
  psStack_270 = psVar22;
  local_268 = psVar10;
  psStack_278 = (sqlite3_stmt *)0x0;
LAB_1003b30b8:
  psStack_280 = local_2a0;
  local_260 = psVar18;
  alloc::collections::btree::map::IntoIter<K,V,A>::dying_next(&local_f0,&local_2a0);
  while (local_f0 != (sqlite3_stmt *)0x0) {
    if (*(long *)(local_f0 + (long)local_e0 * 0x18 + 8) != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)(local_f0 + (long)local_e0 * 0x18 + 0x10),*(long *)(local_f0 + (long)local_e0 * 0x18 + 8),1);
    }
    alloc::collections::btree::map::IntoIter<K,V,A>::dying_next(&local_f0,&local_2a0);
  }
  uVar29 = 0;
  psVar22 = (sqlite3_stmt *)0x0;
  local_f0 = (sqlite3_stmt *)0x0;
  psVar12 = local_330;
  psVar17 = psVar25;
  psVar13 = local_350;
  psVar23 = local_2a0;
  psVar19 = psStack_298;
  psVar2 = local_290;
  psVar21 = local_288;
  psVar24 = psStack_280;
  psVar4 = psStack_278;
  psVar5 = psStack_270;
  psVar6 = local_268;
  local_2a0 = psVar24;
  psStack_298 = psVar21;
  psStack_270 = psVar19;
joined_r0x0001003b3264:
  local_268 = psVar13;
  psStack_278 = psVar17;
  local_288 = psVar12;
  psStack_280 = psVar30;
  local_290 = psVar28;
  if (local_348 == (sqlite3_stmt *)&DAT_8000000000000000) {
LAB_1003b3138:
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar16 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
    if (puVar16 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x18);
    }
    local_90 = psVar8;
    psStack_88 = psVar9;
    local_f0 = (sqlite3_stmt *)&local_90;
    psStack_e8 = (sqlite3_stmt *)<std::path::Display_as_core::fmt::Display>::fmt;
    local_e0 = (sqlite3_stmt *)&local_2a0;
    local_d8 = (sqlite3_stmt *)<rusqlite::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_1e0,_anon_164d03a2dfc98f969363eaf1c50504fd_17,&local_f0);
    puVar16[1] = psStack_1d8;
    *puVar16 = local_1e0;
    puVar16[2] = local_1d0;
    psStack_e8 = (sqlite3_stmt *)0x0;
    *param_1 = 0;
    param_1[1] = 8;
    param_1[7] = 0;
    param_1[9] = 0;
    param_1[8] = local_f0;
    param_1[10] = 0;
    param_1[3] = 1;
    param_1[2] = 0;
    param_1[4] = puVar16;
    param_1[6] = 0;
    param_1[5] = 1;
    ::core::ptr::drop_in_place<rusqlite::error::Error>(&local_2a0);
  }
  else {
    *param_1 = local_348;
    param_1[1] = local_2a0;
    param_1[2] = psStack_298;
    param_1[3] = local_290;
    param_1[4] = local_288;
    param_1[5] = psStack_280;
    param_1[6] = psStack_278;
    param_1[7] = psStack_270;
    param_1[8] = local_268;
    param_1[9] = uVar29;
    param_1[10] = psVar22;
    local_2a0 = psVar23;
    psStack_298 = psVar19;
    local_290 = psVar2;
    local_288 = psVar21;
    psStack_280 = psVar24;
    psStack_278 = psVar4;
    psStack_270 = psVar5;
    local_268 = psVar6;
  }
  ::core::ptr::drop_in_place<rusqlite::Connection>(&local_310);
LAB_1003b3290:
  if (psVar7 != (sqlite3_stmt *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(psVar8,psVar7,1);
  }
  return;
}
