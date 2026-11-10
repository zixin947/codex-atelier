/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* state_db_score @ 1008ebccc requested 1008ebccc */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::platform::paths::CodexPaths::state_db_score */

void codexmate_lib::platform::paths::CodexPaths::state_db_score(long *param_1,undefined8 *******param_2,undefined8 param_3,undefined1 param_4)

{
  uint uVar1;
  undefined1 auVar2 [16];
  bool bVar3;
  undefined8 *******pppppppuVar4;
  undefined8 *******pppppppuVar5;
  undefined3 uVar6;
  bool bVar7;
  int iVar8;
  uint uVar9;
  long lVar10;
  long lVar11;
  long lVar12;
  long lVar13;
  long lVar14;
  undefined8 *******pppppppuVar15;
  long lVar16;
  undefined8 *******pppppppuVar17;
  undefined8 *******pppppppuVar18;
  ulong uVar19;
  undefined8 *******pppppppuVar20;
  undefined8 *******pppppppuVar21;
  uint uVar22;
  undefined8 *puVar23;
  undefined8 uVar24;
  uint uVar25;
  undefined8 uVar26;
  undefined4 uVar27;
  undefined8 *******pppppppuVar28;
  undefined1 auVar29 [12];
  undefined4 local_308;
  int local_2e4;
  undefined1 uStack_2bb;
  undefined1 uStack_2ba;
  undefined1 uStack_2b9;
  long lStack_2b8;
  undefined8 *******local_2b0;
  undefined3 uStack_2a8;
  int5 iStack_2a5;
  undefined3 uStack_2a0;
  sqlite3_stmt *psStack_29d;
  undefined8 *******local_288;
  undefined8 *******local_280;
  undefined8 *******pppppppuStack_278;
  undefined8 *******pppppppuStack_270;
  undefined4 local_268;
  uint local_264;
  int5 local_260;
  undefined3 uStack_25b;
  int5 iStack_258;
  undefined3 uStack_253;
  sqlite3_stmt *local_250;
  sqlite3_stmt *psStack_248;
  sqlite3_stmt *local_240;
  sqlite3_stmt *psStack_238;
  sqlite3_stmt *local_230;
  sqlite3_stmt *psStack_228;
  undefined8 local_220;
  undefined8 uStack_218;
  undefined8 local_210;
  undefined8 uStack_208;
  undefined8 local_200;
  undefined8 *******local_1f8;
  undefined8 *******pppppppuStack_1f0;
  undefined8 *******local_1e8;
  undefined8 *******local_1e0;
  undefined3 uStack_1d8;
  int5 iStack_1d5;
  undefined3 local_1d0;
  int5 iStack_1cd;
  undefined3 uStack_1c8;
  int5 iStack_1c5;
  undefined4 local_1c0;
  uint uStack_1bc;
  int5 iStack_1b8;
  undefined3 uStack_1b3;
  int5 local_1b0;
  undefined3 uStack_1ab;
  sqlite3_stmt *psStack_1a8;
  undefined8 *******local_1a0;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined3 uStack_190;
  int5 iStack_18d;
  undefined3 uStack_188;
  int5 iStack_185;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  sqlite3_stmt *local_168;
  undefined8 *******local_160;
  undefined3 uStack_158;
  int5 iStack_155;
  undefined3 local_150;
  int5 iStack_14d;
  undefined3 local_148;
  int5 iStack_145;
  undefined8 local_140;
  int5 iStack_138;
  undefined3 uStack_133;
  int5 local_130;
  undefined3 uStack_12b;
  sqlite3_stmt *psStack_128;
  undefined8 *******local_118;
  undefined8 ******local_110;
  undefined8 *******local_108;
  undefined8 *******local_100;
  undefined8 *******pppppppuStack_f8;
  undefined8 *******pppppppuStack_f0;
  undefined4 uStack_e8;
  uint uStack_e4;
  undefined4 uStack_e0;
  uint uStack_dc;
  int5 local_d8;
  undefined3 uStack_d3;
  int5 iStack_d0;
  undefined3 uStack_cb;
  sqlite3_stmt *local_c8;
  sqlite3_stmt *psStack_c0;
  sqlite3_stmt *local_b8;
  sqlite3_stmt *psStack_b0;
  sqlite3_stmt *local_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;

  auVar29 = state_db_version(param_2,param_3);
  uVar9 = auVar29._8_4_;
  if ((auVar29._0_8_ & 1) == 0) {
    uVar9 = 0;
  }
  std::sys::fs::metadata(&local_108,param_2,param_3);
  pppppppuVar17 = local_100;
  if ((int)local_108 == 1) {
joined_r0x0001008ebdc8:
    if (((ulong)pppppppuVar17 & 3) == 1) {
      uVar24 = *(undefined8 *)((long)pppppppuVar17 + -1);
      puVar23 = *(undefined8 **)((long)pppppppuVar17 + 7);
      if ((code *)*puVar23 != (code *)0x0) {
        (*(code *)*puVar23)(uVar24);
      }
      if (puVar23[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar24,puVar23[1],puVar23[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)pppppppuVar17 + -1),0x18,8);
    }
    bVar3 = true;
    uVar22 = 1000000000;
  }
  else {
    psStack_228 = (sqlite3_stmt *)CONCAT44(uStack_9c,uStack_a0);
    local_230 = local_a8;
    uStack_218 = uStack_90;
    local_220 = local_98;
    uStack_208 = uStack_80;
    local_210 = local_88;
    local_200 = local_78;
    pppppppuStack_270 = (undefined8 *******)CONCAT44(uStack_e4,uStack_e8);
    iStack_258 = iStack_d0;
    uStack_253 = uStack_cb;
    local_260 = local_d8;
    uStack_25b = uStack_d3;
    psStack_248 = psStack_c0;
    local_250 = local_c8;
    psStack_238 = psStack_b0;
    local_240 = local_b8;
    pppppppuStack_278 = pppppppuStack_f0;
    local_280 = pppppppuStack_f8;
    local_268 = uStack_e0;
    local_264 = uStack_dc;
    local_288 = local_100;
    std::fs::Metadata::modified(&local_108,&local_288);
    pppppppuVar17 = local_108;
    if ((uint)local_100 == 1000000000) goto joined_r0x0001008ebdc8;
    bVar3 = false;
    uVar22 = (uint)local_100;
  }
  std::path::Path::_with_extension
            (&local_160,param_2,param_3,
             "sqlite-walCODEX_HOME.codexCODEX_SQLITE_HOMEstate_5.sqliteLibrary/LaunchAgents/dev.aimami.auto-switch.plistconfig.tomlsqlite_homecodexmatecustom-instructionsrelayauth.jsonsession_index.jsonlarchived_sessionsskillsregistry.jsonsnapshotsbackupsauto-switch.logskill-backupsquota-history.jsonlsettings.jsonbootstrap-cache.jsonauto-switch-pending.jsonauto-switch-snooze.jsonvoice-workspace.jsonvoice-runtime.jsonstate.jsonAGENTS.mdhistorymobiledesktopstruct ExportFilestruct ExportedAccountschemehostpath-patternpathPatternpath-prefixpathPrefixpath-suffixpathSuffixapp-linkappLinkDeepLinkProtocolschemesdomainsnamedata did not match any variant of untagged enum DesktopProtocolAssociatedDomainConfig://struct RouterUnlockAuthMarkerstruct NoAccountModelRestoreEntrystruct NoAccountModelRestoreJournalstruct ModelRestoreEntrystruct ModelRestoreJournalfailed to write whole bufferinternal error: entered unreachable codeinternal error: entered unreachable code: invalid Once stateNo file has been startedwrite(): ZipWriter was already closedLarge file option has not been set()a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` value/ErrorIoInvalidArchiveUnsupportedArchiveFileNotFoundInvalidPasswordassertion failed: self.is_char_boundary(new_len)assertion failed: vec.capacity() - start >= lenConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedTauriPathForbiddenGlobPatternInvalidPathUrlUnsafePathBufTryFromIntErrorInvalidUncertain"
             ,10);
  uVar24 = CONCAT53(iStack_155,uStack_158);
  std::sys::fs::metadata(&local_108,uVar24,CONCAT53(iStack_14d,local_150));
  pppppppuVar20 = local_100;
  if (((ulong)local_108 & 1) == 0) {
    psStack_228 = (sqlite3_stmt *)CONCAT44(uStack_9c,uStack_a0);
    local_230 = local_a8;
    uStack_218 = uStack_90;
    local_220 = local_98;
    uStack_208 = uStack_80;
    local_210 = local_88;
    local_200 = local_78;
    pppppppuStack_270 = (undefined8 *******)CONCAT44(uStack_e4,uStack_e8);
    iStack_258 = iStack_d0;
    uStack_253 = uStack_cb;
    local_260 = local_d8;
    uStack_25b = uStack_d3;
    psStack_248 = psStack_c0;
    local_250 = local_c8;
    psStack_238 = psStack_b0;
    local_240 = local_b8;
    pppppppuStack_278 = pppppppuStack_f0;
    local_280 = pppppppuStack_f8;
    local_268 = uStack_e0;
    local_264 = uStack_dc;
    local_288 = local_100;
    std::fs::Metadata::modified(&local_1a0,&local_288);
    pppppppuVar20 = local_1a0;
    if (uStack_198 != 1000000000) {
      pppppppuVar20 = (undefined8 *******)0x0;
      if (!bVar3) {
        pppppppuVar20 = pppppppuVar17;
      }
      bVar7 = local_1a0 == pppppppuVar20;
      pppppppuVar17 = local_1a0;
      uVar25 = uStack_198;
      goto LAB_1008ebf24;
    }
  }
  if (((ulong)pppppppuVar20 & 3) == 1) {
    uVar26 = *(undefined8 *)((long)pppppppuVar20 + -1);
    puVar23 = *(undefined8 **)((long)pppppppuVar20 + 7);
    if ((code *)*puVar23 != (code *)0x0) {
      (*(code *)*puVar23)(uVar26);
    }
    if (puVar23[1] != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar26,puVar23[1],puVar23[2]);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)pppppppuVar20 + -1),0x18,8);
  }
  pppppppuVar20 = (undefined8 *******)0x0;
  if (!bVar3) {
    pppppppuVar20 = pppppppuVar17;
  }
  bVar7 = pppppppuVar20 == (undefined8 *******)0x0;
  pppppppuVar17 = (undefined8 *******)0x0;
  uVar25 = 0;
LAB_1008ebf24:
  uVar1 = 0;
  if (!bVar3) {
    uVar1 = uVar22;
  }
  bVar3 = uVar25 < uVar1;
  if (!bVar7) {
    bVar3 = (long)pppppppuVar17 < (long)pppppppuVar20;
  }
  if (!bVar3) {
    pppppppuVar20 = pppppppuVar17;
    uVar1 = uVar25;
  }
  if (local_160 != (undefined8 *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar24,local_160,1);
  }
  local_280 = (undefined8 *******)CONCAT44(local_280._4_4_,uVar1);
  local_288 = pppppppuVar20;
  std::time::SystemTime::duration_since(&local_108,&local_288,0,0);
  if (((ulong)local_108 & 1) == 0) {
    auVar2._8_8_ = 0;
    auVar2._0_8_ = local_100;
    lVar14 = SUB168(auVar2 * ZEXT816(1000000000),8);
    lVar10 = (long)local_100 * 1000000000 + ((ulong)pppppppuStack_f8 & 0xffffffff);
    if (CARRY8((long)local_100 * 1000000000,(ulong)pppppppuStack_f8 & 0xffffffff)) {
      lVar14 = lVar14 + 1;
    }
  }
  else {
    lVar10 = 0;
    lVar14 = 0;
  }
  uVar22 = 0;
  uStack_2bb = 1;
  local_2e4 = 1;
  uStack_2ba = param_4;
  do {
    rusqlite::Connection::open_with_flags(&local_108,param_2,param_3,0x8001);
    uVar25 = uStack_e4 & 0xff;
    local_160 = (undefined8 *******)CONCAT17((undefined1)uStack_dc,CONCAT43(uStack_e0,uStack_e4._1_3_));
    uStack_158 = uStack_dc._1_3_;
    iStack_14d = iStack_d0;
    local_148 = uStack_cb;
    iStack_155 = local_d8;
    local_150 = uStack_d3;
    if ((char)uStack_a0 == '\x03') {
      uVar19 = (ulong)pppppppuStack_f0 & 0xff;
      uStack_2a8 = uStack_dc._1_3_;
      psStack_29d = (sqlite3_stmt *)CONCAT35(uStack_cb,iStack_d0);
      iStack_2a5 = local_d8;
      uStack_2a0 = uStack_d3;
      pppppppuVar17 = pppppppuStack_f8;
      pppppppuVar28 = local_100;
      if (local_108 == (undefined8 *******)0x8000000000000016) goto LAB_1008ecb90;
      pppppppuVar20 = pppppppuStack_f8;
      pppppppuVar28 = local_108;
      pppppppuVar15 = pppppppuStack_f0;
      pppppppuVar17 = local_100;
      uVar27 = uStack_e8;
      local_2b0 = local_160;
      if (local_108 != (undefined8 *******)&DAT_8000000000000000) goto LAB_1008eca54;
    }
    else {
      local_240 = psStack_c0;
      psStack_248 = local_c8;
      local_230 = psStack_b0;
      psStack_238 = local_b8;
      psStack_228 = local_a8;
      local_288 = local_108;
      local_280 = local_100;
      pppppppuStack_278 = pppppppuStack_f8;
      pppppppuStack_270 = pppppppuStack_f0;
      local_268 = uStack_e8;
      uStack_25b = uStack_dc._1_3_;
      local_264 = uStack_e4;
      local_260 = (int5)((ulong)local_160 >> 0x18);
      local_250 = (sqlite3_stmt *)CONCAT35(uStack_cb,iStack_d0);
      iStack_258 = local_d8;
      uStack_253 = uStack_d3;
      local_220 = CONCAT71((int7)(CONCAT44(uStack_9c,uStack_a0) >> 8),(char)uStack_a0);
      rusqlite::busy::<impl_rusqlite::Connection>::busy_timeout(&local_108,&local_288,0,75000000);
      if (local_108 == (undefined8 *******)0x8000000000000016) {
        rusqlite::Connection::prepare_with_flags(&local_108,&local_288,"PRAGMA table_info(threads)idrollout_pathtargetPathfileNamesummaryhasIssues, ",0x1a);
        uVar6 = uStack_1d8;
        pppppppuVar15 = (undefined8 *******)CONCAT44(uStack_e4,uStack_e8);
        local_1a0 = (undefined8 *******)CONCAT53(local_d8,uStack_dc._1_3_);
        iStack_18d = (int5)local_c8;
        uStack_188 = (undefined3)((ulong)local_c8 >> 0x28);
        uStack_198 = CONCAT13((char)iStack_d0,uStack_d3);
        uStack_194 = (undefined4)((uint5)iStack_d0 >> 8);
        uStack_190 = uStack_cb;
        uStack_1d8 = uStack_d3;
        if (((ulong)local_108 & 1) == 0) {
          local_150 = SUB83(pppppppuStack_f0,0);
          iStack_14d = (int5)((ulong)pppppppuStack_f0 >> 0x18);
          uStack_158 = SUB83(pppppppuStack_f8,0);
          iStack_155 = (int5)((ulong)pppppppuStack_f8 >> 0x18);
          local_160 = local_100;
          local_148 = (undefined3)uStack_e8;
          iStack_145 = (int5)((ulong)pppppppuVar15 >> 0x18);
          local_140._0_4_ = uStack_e0;
          psStack_128 = local_c8;
          uStack_12b = uStack_cb;
          local_130 = (int5)(CONCAT44(uStack_194,uStack_198) >> 0x18);
          local_140._4_4_ = uStack_dc;
          iStack_138 = local_d8;
          uStack_133 = uStack_1d8;
          uStack_1d8 = uVar6;
          iVar8 = _sqlite3_bind_parameter_count(local_c8);
          pppppppuVar20 = (undefined8 *******)(long)iVar8;
          if (iVar8 == 0) {
            <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_108,&local_160,0);
            pppppppuVar18 = pppppppuStack_f8;
            pppppppuVar5 = local_100;
            pppppppuVar4 = local_108;
            ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_160);
            goto joined_r0x0001008ec244;
          }
          ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_160);
          pppppppuVar17 = (undefined8 *******)0x0;
          pppppppuVar28 = (undefined8 *******)0x8000000000000013;
          uVar27 = local_308;
        }
        else {
          iStack_1d5 = iStack_d0;
          local_1d0 = uStack_cb;
          pppppppuVar20 = pppppppuStack_f0;
          pppppppuVar28 = local_100;
          pppppppuVar17 = pppppppuStack_f8;
          uVar27 = uStack_e0;
          uVar22 = uStack_dc & 0xff;
          local_1e0 = local_1a0;
          iStack_1cd = iStack_18d;
          uStack_1c8 = uStack_188;
          pppppppuVar4 = pppppppuStack_f8;
          pppppppuVar5 = pppppppuStack_f0;
          local_308 = uStack_e0;
          pppppppuVar18 = pppppppuVar15;
          if (local_100 == (undefined8 *******)0x8000000000000016) {
joined_r0x0001008ec244:
            local_1f8 = pppppppuVar4;
            pppppppuStack_1f0 = pppppppuVar5;
            local_1e8 = pppppppuVar18;
            if (pppppppuVar18 == (undefined8 *******)0x0) {
              pppppppuVar17 = (undefined8 *******)0x0;
              uVar19 = 0;
              pppppppuVar28 = (undefined8 *******)0x0;
            }
            else {
              lVar16 = (long)pppppppuVar18 * 0x18;
              pppppppuVar20 = (undefined8 *******)((long)pppppppuVar5 + 0x10);
              lVar13 = lVar16;
              pppppppuVar17 = pppppppuVar20;
              do {
                lVar12 = lVar16;
                pppppppuVar15 = pppppppuVar20;
                if ((*pppppppuVar17 == (undefined8 ******)0xd) &&
                   (*pppppppuVar17[-1] == (undefined8 *****)0x5f79636e65636572 && *(long *)((long)pppppppuVar17[-1] + 5) == 0x736d5f74615f7963)) {
                  uVar19 = 2;
                  goto LAB_1008ec354;
                }
                pppppppuVar17 = pppppppuVar17 + 3;
                lVar13 = lVar13 + -0x18;
                lVar11 = lVar16;
                pppppppuVar28 = pppppppuVar20;
              } while (lVar13 != 0);
              do {
                if ((*pppppppuVar28 == (undefined8 ******)0xd) &&
                   (*pppppppuVar28[-1] == (undefined8 *****)0x5f64657461647075 && *(long *)((long)pppppppuVar28[-1] + 5) == 0x736d5f74615f6465)) {
                  uVar19 = 1;
                  goto LAB_1008ec354;
                }
                lVar11 = lVar11 + -0x18;
                pppppppuVar28 = pppppppuVar28 + 3;
              } while (lVar11 != 0);
              uVar19 = 0;
LAB_1008ec354:
              do {
                if ((*pppppppuVar15 == (undefined8 ******)0xd) &&
                   (*pppppppuVar15[-1] == (undefined8 *****)0x5f64657461647075 && *(long *)((long)pppppppuVar15[-1] + 5) == 0x736d5f74615f6465)) {
                  rusqlite::Connection::prepare_with_flags
                            (&local_108,&local_288,
                             "SELECT COALESCE(MAX(updated_at_ms), 0) FROM threadsSELECT COUNT(*) FROM threadsSELECT COUNT(*) FROM threads WHERE COALESCE(archived, 0) = 0"
                             ,0x33);
                  uStack_158 = SUB83(pppppppuStack_f8,0);
                  iStack_155 = (int5)((ulong)pppppppuStack_f8 >> 0x18);
                  local_160 = local_100;
                  local_148 = (undefined3)uStack_e8;
                  iStack_145 = (int5)(CONCAT44(uStack_e4,uStack_e8) >> 0x18);
                  local_150 = SUB83(pppppppuStack_f0,0);
                  iStack_14d = (int5)((ulong)pppppppuStack_f0 >> 0x18);
                  iStack_138 = local_d8;
                  uStack_133 = uStack_d3;
                  local_140._0_4_ = uStack_e0;
                  local_140._4_4_ = uStack_dc;
                  psStack_128 = local_c8;
                  local_130 = iStack_d0;
                  uStack_12b = uStack_cb;
                  if ((int)local_108 == 1) {
                    local_1e0 = local_100;
                    uStack_1c8 = local_148;
                    iStack_1c5 = iStack_145;
                    iStack_1b8 = local_d8;
                    uStack_1b3 = uStack_d3;
                    local_1c0 = uStack_e0;
                    uStack_1bc = uStack_dc;
                    psStack_1a8 = local_c8;
                    local_1b0 = iStack_d0;
                    uStack_1ab = uStack_cb;
                    uStack_1d8 = uStack_158;
                    iStack_1d5 = iStack_155;
                    local_1d0 = local_150;
                    iStack_1cd = iStack_14d;
                  }
                  else {
                    uStack_198 = (uint)pppppppuStack_f8;
                    uStack_194 = (undefined4)((ulong)pppppppuStack_f8 >> 0x20);
                    local_1a0 = local_100;
                    uStack_188 = local_148;
                    iStack_185 = iStack_145;
                    uStack_178 = CONCAT35(uStack_d3,local_d8);
                    local_180 = CONCAT44(uStack_dc,uStack_e0);
                    uStack_170 = CONCAT35(uStack_cb,iStack_d0);
                    local_168 = local_c8;
                    uStack_190 = local_150;
                    iStack_18d = iStack_14d;
                    iVar8 = _sqlite3_bind_parameter_count(local_c8);
                    if (iVar8 == 0) {
                      local_118 = &local_1a0;
                      local_110 = (undefined8 ******)0x0;
                      <rusqlite::row::Rows_as_fallible_streaming_iterator::FallibleStreamingIterator>::advance(&local_108,&local_118);
                      if (local_108 == (undefined8 *******)0x8000000000000016) {
                        if (local_110 == (undefined8 ******)0x0) {
                          local_1e0 = (undefined8 *******)&DAT_8000000000000009;
                          pppppppuVar17 = &local_110;
                          goto LAB_1008ec5ac;
                        }
                        rusqlite::row::Row::get(&local_1e0,&local_110,0,&PTR_s_src_platform_paths_rs_10182e658);
                      }
                      else {
                        uStack_158 = SUB83(pppppppuStack_f0,0);
                        iStack_155 = (int5)((ulong)pppppppuStack_f0 >> 0x18);
                        local_160 = pppppppuStack_f8;
                        local_148 = (undefined3)uStack_e0;
                        iStack_145 = (int5)(CONCAT44(uStack_dc,uStack_e0) >> 0x18);
                        local_150 = (undefined3)uStack_e8;
                        iStack_14d = (int5)(CONCAT44(uStack_e4,uStack_e8) >> 0x18);
                        iStack_138 = iStack_d0;
                        uStack_133 = uStack_cb;
                        local_140._0_4_ = (undefined4)local_d8;
                        local_140._4_4_ = (uint)(CONCAT35(uStack_d3,local_d8) >> 0x20);
                        local_1e0 = local_108;
                        pppppppuVar17 = local_100;
LAB_1008ec5ac:
                        uStack_1d8 = SUB83(pppppppuVar17,0);
                        iStack_1d5 = (int5)((ulong)pppppppuVar17 >> 0x18);
                        uStack_1c8 = uStack_158;
                        iStack_1c5 = iStack_155;
                        local_1d0 = SUB83(local_160,0);
                        iStack_1cd = (int5)((ulong)local_160 >> 0x18);
                        iStack_1b8 = (int5)CONCAT53(iStack_145,local_148);
                        uStack_1b3 = (undefined3)((uint5)iStack_145 >> 0x10);
                        local_1c0 = (undefined4)CONCAT53(iStack_14d,local_150);
                        uStack_1bc = (uint)((uint5)iStack_14d >> 8);
                        psStack_1a8 = (sqlite3_stmt *)CONCAT35(uStack_133,iStack_138);
                        local_1b0 = (int5)CONCAT44(local_140._4_4_,(undefined4)local_140);
                        uStack_1ab = (undefined3)(local_140._4_4_ >> 8);
                      }
                      ::core::ptr::drop_in_place<rusqlite::row::Rows>(&local_118);
                    }
                    else {
                      local_1e0 = (undefined8 *******)0x8000000000000013;
                      uStack_1d8 = 0;
                      iStack_1d5 = 0;
                      local_1d0 = (undefined3)iVar8;
                      iStack_1cd = (int5)(char)((uint)iVar8 >> 0x18);
                    }
                    ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_1a0);
                  }
                  pppppppuVar28 = (undefined8 *******)CONCAT53(iStack_1d5,uStack_1d8);
                  if (local_1e0 == (undefined8 *******)0x8000000000000016) goto LAB_1008ec6a8;
                  pppppppuVar20 = (undefined8 *******)CONCAT53(iStack_1cd,local_1d0);
                  pppppppuVar15 = (undefined8 *******)CONCAT53(iStack_1c5,uStack_1c8);
                  uVar25 = uStack_1bc & 0xff;
                  local_2b0 = (undefined8 *******)CONCAT53(iStack_1b8,uStack_1bc._1_3_);
                  pppppppuVar17 = pppppppuVar28;
                  goto LAB_1008ec968;
                }
                lVar12 = lVar12 + -0x18;
                lVar13 = lVar16;
                pppppppuVar17 = pppppppuVar20;
                pppppppuVar15 = pppppppuVar15 + 3;
              } while (lVar12 != 0);
              do {
                if ((*pppppppuVar17 == (undefined8 ******)0xa) &&
                   (*pppppppuVar17[-1] == (undefined8 *****)0x5f64657461647075 && *(short *)(pppppppuVar17[-1] + 1) == 0x7461)) {
                  rusqlite::Connection::prepare_with_flags
                            (&local_108,&local_288,
                             "SELECT COALESCE(MAX(updated_at), 0) FROM threadsSELECT COALESCE(MAX(updated_at_ms), 0) FROM threadsSELECT COUNT(*) FROM threadsSELECT COUNT(*) FROM threads WHERE COALESCE(archived, 0) = 0"
                             ,0x30);
                  uStack_158 = SUB83(pppppppuStack_f8,0);
                  iStack_155 = (int5)((ulong)pppppppuStack_f8 >> 0x18);
                  local_160 = local_100;
                  local_148 = (undefined3)uStack_e8;
                  iStack_145 = (int5)(CONCAT44(uStack_e4,uStack_e8) >> 0x18);
                  local_150 = SUB83(pppppppuStack_f0,0);
                  iStack_14d = (int5)((ulong)pppppppuStack_f0 >> 0x18);
                  iStack_138 = local_d8;
                  uStack_133 = uStack_d3;
                  local_140._0_4_ = uStack_e0;
                  local_140._4_4_ = uStack_dc;
                  psStack_128 = local_c8;
                  local_130 = iStack_d0;
                  uStack_12b = uStack_cb;
                  if ((int)local_108 == 1) {
                    local_1e0 = local_100;
                    uStack_1c8 = local_148;
                    iStack_1c5 = iStack_145;
                    iStack_1b8 = local_d8;
                    uStack_1b3 = uStack_d3;
                    local_1c0 = uStack_e0;
                    uStack_1bc = uStack_dc;
                    psStack_1a8 = local_c8;
                    local_1b0 = iStack_d0;
                    uStack_1ab = uStack_cb;
                    uStack_1d8 = uStack_158;
                    iStack_1d5 = iStack_155;
                    local_1d0 = local_150;
                    iStack_1cd = iStack_14d;
                  }
                  else {
                    uStack_198 = (uint)pppppppuStack_f8;
                    uStack_194 = (undefined4)((ulong)pppppppuStack_f8 >> 0x20);
                    local_1a0 = local_100;
                    uStack_188 = local_148;
                    iStack_185 = iStack_145;
                    uStack_178 = CONCAT35(uStack_d3,local_d8);
                    local_180 = CONCAT44(uStack_dc,uStack_e0);
                    uStack_170 = CONCAT35(uStack_cb,iStack_d0);
                    local_168 = local_c8;
                    uStack_190 = local_150;
                    iStack_18d = iStack_14d;
                    iVar8 = _sqlite3_bind_parameter_count(local_c8);
                    if (iVar8 == 0) {
                      local_118 = &local_1a0;
                      local_110 = (undefined8 ******)0x0;
                      <rusqlite::row::Rows_as_fallible_streaming_iterator::FallibleStreamingIterator>::advance(&local_108,&local_118);
                      if (local_108 == (undefined8 *******)0x8000000000000016) {
                        if (local_110 == (undefined8 ******)0x0) {
                          local_1e0 = (undefined8 *******)&DAT_8000000000000009;
                          pppppppuVar17 = &local_110;
                          goto LAB_1008ec630;
                        }
                        rusqlite::row::Row::get(&local_1e0,&local_110,0,&PTR_s_src_platform_paths_rs_10182e628);
                      }
                      else {
                        uStack_158 = SUB83(pppppppuStack_f0,0);
                        iStack_155 = (int5)((ulong)pppppppuStack_f0 >> 0x18);
                        local_160 = pppppppuStack_f8;
                        local_148 = (undefined3)uStack_e0;
                        iStack_145 = (int5)(CONCAT44(uStack_dc,uStack_e0) >> 0x18);
                        local_150 = (undefined3)uStack_e8;
                        iStack_14d = (int5)(CONCAT44(uStack_e4,uStack_e8) >> 0x18);
                        iStack_138 = iStack_d0;
                        uStack_133 = uStack_cb;
                        local_140._0_4_ = (undefined4)local_d8;
                        local_140._4_4_ = (uint)(CONCAT35(uStack_d3,local_d8) >> 0x20);
                        local_1e0 = local_108;
                        pppppppuVar17 = local_100;
LAB_1008ec630:
                        uStack_1d8 = SUB83(pppppppuVar17,0);
                        iStack_1d5 = (int5)((ulong)pppppppuVar17 >> 0x18);
                        uStack_1c8 = uStack_158;
                        iStack_1c5 = iStack_155;
                        local_1d0 = SUB83(local_160,0);
                        iStack_1cd = (int5)((ulong)local_160 >> 0x18);
                        iStack_1b8 = (int5)CONCAT53(iStack_145,local_148);
                        uStack_1b3 = (undefined3)((uint5)iStack_145 >> 0x10);
                        local_1c0 = (undefined4)CONCAT53(iStack_14d,local_150);
                        uStack_1bc = (uint)((uint5)iStack_14d >> 8);
                        psStack_1a8 = (sqlite3_stmt *)CONCAT35(uStack_133,iStack_138);
                        local_1b0 = (int5)CONCAT44(local_140._4_4_,(undefined4)local_140);
                        uStack_1ab = (undefined3)(local_140._4_4_ >> 8);
                      }
                      ::core::ptr::drop_in_place<rusqlite::row::Rows>(&local_118);
                    }
                    else {
                      local_1e0 = (undefined8 *******)0x8000000000000013;
                      uStack_1d8 = 0;
                      iStack_1d5 = 0;
                      local_1d0 = (undefined3)iVar8;
                      iStack_1cd = (int5)(char)((uint)iVar8 >> 0x18);
                    }
                    ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_1a0);
                  }
                  pppppppuVar17 = (undefined8 *******)CONCAT53(iStack_1d5,uStack_1d8);
                  if (local_1e0 == (undefined8 *******)0x8000000000000016) {
                    pppppppuVar28 = (undefined8 *******)((long)pppppppuVar17 * 1000);
                    if (SUB168(SEXT816((long)pppppppuVar17) * SEXT816(1000),8) != (long)pppppppuVar28 >> 0x3f) {
                      pppppppuVar28 = (undefined8 *******)((long)iStack_1d5 >> 0x27 ^ 0x7fffffffffffffff);
                    }
                    goto LAB_1008ec6a8;
                  }
                  pppppppuVar20 = (undefined8 *******)CONCAT53(iStack_1cd,local_1d0);
                  pppppppuVar15 = (undefined8 *******)CONCAT53(iStack_1c5,uStack_1c8);
                  uVar25 = uStack_1bc & 0xff;
                  local_2b0 = (undefined8 *******)CONCAT53(iStack_1b8,uStack_1bc._1_3_);
                  goto LAB_1008ec968;
                }
                lVar13 = lVar13 + -0x18;
                pppppppuVar17 = pppppppuVar17 + 3;
              } while (lVar13 != 0);
              pppppppuVar28 = (undefined8 *******)0x0;
LAB_1008ec6a8:
              do {
                if ((*pppppppuVar20 == (undefined8 ******)0x8) && (*pppppppuVar20[-1] == (undefined8 *****)0x6465766968637261)) {
                  rusqlite::Connection::prepare_with_flags(&local_108,&local_288,"SELECT COUNT(*) FROM threads WHERE COALESCE(archived, 0) = 0",0x3c);
                  uStack_158 = SUB83(pppppppuStack_f8,0);
                  iStack_155 = (int5)((ulong)pppppppuStack_f8 >> 0x18);
                  local_160 = local_100;
                  local_148 = (undefined3)uStack_e8;
                  iStack_145 = (int5)(CONCAT44(uStack_e4,uStack_e8) >> 0x18);
                  local_150 = SUB83(pppppppuStack_f0,0);
                  iStack_14d = (int5)((ulong)pppppppuStack_f0 >> 0x18);
                  iStack_138 = local_d8;
                  uStack_133 = uStack_d3;
                  local_140._0_4_ = uStack_e0;
                  local_140._4_4_ = uStack_dc;
                  psStack_128 = local_c8;
                  local_130 = iStack_d0;
                  uStack_12b = uStack_cb;
                  if ((int)local_108 == 1) goto LAB_1008ec774;
                  uStack_198 = (uint)pppppppuStack_f8;
                  uStack_194 = (undefined4)((ulong)pppppppuStack_f8 >> 0x20);
                  local_1a0 = local_100;
                  uStack_188 = local_148;
                  iStack_185 = iStack_145;
                  uStack_178 = CONCAT35(uStack_d3,local_d8);
                  local_180 = CONCAT44(uStack_dc,uStack_e0);
                  uStack_170 = CONCAT35(uStack_cb,iStack_d0);
                  local_168 = local_c8;
                  uStack_190 = local_150;
                  iStack_18d = iStack_14d;
                  iVar8 = _sqlite3_bind_parameter_count(local_c8);
                  if (iVar8 == 0) {
                    local_118 = &local_1a0;
                    local_110 = (undefined8 ******)0x0;
                    <rusqlite::row::Rows_as_fallible_streaming_iterator::FallibleStreamingIterator>::advance(&local_108,&local_118);
                    if (local_108 == (undefined8 *******)0x8000000000000016) {
                      if (local_110 == (undefined8 ******)0x0) {
                        local_1e0 = (undefined8 *******)&DAT_8000000000000009;
                        pppppppuVar17 = &local_110;
                        goto LAB_1008ec904;
                      }
                      rusqlite::row::Row::get(&local_1e0,&local_110,0,&PTR_s_src_platform_paths_rs_10182e640);
                    }
                    else {
                      uStack_158 = SUB83(pppppppuStack_f0,0);
                      iStack_155 = (int5)((ulong)pppppppuStack_f0 >> 0x18);
                      local_160 = pppppppuStack_f8;
                      local_148 = (undefined3)uStack_e0;
                      iStack_145 = (int5)(CONCAT44(uStack_dc,uStack_e0) >> 0x18);
                      local_150 = (undefined3)uStack_e8;
                      iStack_14d = (int5)(CONCAT44(uStack_e4,uStack_e8) >> 0x18);
                      iStack_138 = iStack_d0;
                      uStack_133 = uStack_cb;
                      local_140._0_4_ = (undefined4)local_d8;
                      local_140._4_4_ = (uint)(CONCAT35(uStack_d3,local_d8) >> 0x20);
                      local_1e0 = local_108;
                      pppppppuVar17 = local_100;
LAB_1008ec904:
                      uStack_1d8 = SUB83(pppppppuVar17,0);
                      iStack_1d5 = (int5)((ulong)pppppppuVar17 >> 0x18);
                      uStack_1c8 = uStack_158;
                      iStack_1c5 = iStack_155;
                      local_1d0 = SUB83(local_160,0);
                      iStack_1cd = (int5)((ulong)local_160 >> 0x18);
                      iStack_1b8 = (int5)CONCAT53(iStack_145,local_148);
                      uStack_1b3 = (undefined3)((uint5)iStack_145 >> 0x10);
                      local_1c0 = (undefined4)CONCAT53(iStack_14d,local_150);
                      uStack_1bc = (uint)((uint5)iStack_14d >> 8);
                      psStack_1a8 = (sqlite3_stmt *)CONCAT35(uStack_133,iStack_138);
                      local_1b0 = (int5)CONCAT44(local_140._4_4_,(undefined4)local_140);
                      uStack_1ab = (undefined3)(local_140._4_4_ >> 8);
                    }
                    ::core::ptr::drop_in_place<rusqlite::row::Rows>(&local_118);
                  }
                  else {
                    local_1e0 = (undefined8 *******)0x8000000000000013;
                    uStack_1d8 = 0;
                    iStack_1d5 = 0;
                    local_1d0 = (undefined3)iVar8;
                    iStack_1cd = (int5)(char)((uint)iVar8 >> 0x18);
                  }
                  ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_1a0);
                  goto LAB_1008ec92c;
                }
                pppppppuVar20 = pppppppuVar20 + 3;
                lVar16 = lVar16 + -0x18;
              } while (lVar16 != 0);
              rusqlite::Connection::prepare_with_flags
                        (&local_108,&local_288,"SELECT COUNT(*) FROM threadsSELECT COUNT(*) FROM threads WHERE COALESCE(archived, 0) = 0",0x1c);
              uStack_158 = SUB83(pppppppuStack_f8,0);
              iStack_155 = (int5)((ulong)pppppppuStack_f8 >> 0x18);
              local_160 = local_100;
              local_148 = (undefined3)uStack_e8;
              iStack_145 = (int5)(CONCAT44(uStack_e4,uStack_e8) >> 0x18);
              local_150 = SUB83(pppppppuStack_f0,0);
              iStack_14d = (int5)((ulong)pppppppuStack_f0 >> 0x18);
              iStack_138 = local_d8;
              uStack_133 = uStack_d3;
              local_140._0_4_ = uStack_e0;
              local_140._4_4_ = uStack_dc;
              psStack_128 = local_c8;
              local_130 = iStack_d0;
              uStack_12b = uStack_cb;
              if ((int)local_108 == 1) {
LAB_1008ec774:
                uStack_1d8 = uStack_158;
                iStack_1d5 = iStack_155;
                local_1e0 = local_100;
                uStack_1c8 = local_148;
                iStack_1c5 = iStack_145;
                local_1d0 = local_150;
                iStack_1cd = iStack_14d;
                iStack_1b8 = local_d8;
                uStack_1b3 = uStack_d3;
                local_1c0 = uStack_e0;
                uStack_1bc = uStack_dc;
                psStack_1a8 = local_c8;
                local_1b0 = iStack_d0;
                uStack_1ab = uStack_cb;
                local_160 = local_100;
                local_140._0_4_ = uStack_e0;
                local_140._4_4_ = uStack_dc;
                iStack_138 = local_d8;
                uStack_133 = uStack_d3;
                local_130 = iStack_d0;
                uStack_12b = uStack_cb;
                psStack_128 = local_c8;
              }
              else {
                uStack_198 = (uint)pppppppuStack_f8;
                uStack_194 = (undefined4)((ulong)pppppppuStack_f8 >> 0x20);
                local_1a0 = local_100;
                uStack_188 = local_148;
                iStack_185 = iStack_145;
                uStack_178 = CONCAT35(uStack_d3,local_d8);
                local_180 = CONCAT44(uStack_dc,uStack_e0);
                uStack_170 = CONCAT35(uStack_cb,iStack_d0);
                local_168 = local_c8;
                uStack_190 = local_150;
                iStack_18d = iStack_14d;
                iVar8 = _sqlite3_bind_parameter_count(local_c8);
                if (iVar8 == 0) {
                  local_118 = &local_1a0;
                  local_110 = (undefined8 ******)0x0;
                  <rusqlite::row::Rows_as_fallible_streaming_iterator::FallibleStreamingIterator>::advance(&local_108,&local_118);
                  if (local_108 == (undefined8 *******)0x8000000000000016) {
                    if (local_110 == (undefined8 ******)0x0) {
                      local_1e0 = (undefined8 *******)&DAT_8000000000000009;
                      pppppppuVar17 = &local_110;
                      goto LAB_1008ec8c8;
                    }
                    rusqlite::row::Row::get(&local_1e0,&local_110,0,&PTR_s_src_platform_paths_rs_10182e610);
                  }
                  else {
                    uStack_158 = SUB83(pppppppuStack_f0,0);
                    iStack_155 = (int5)((ulong)pppppppuStack_f0 >> 0x18);
                    local_160 = pppppppuStack_f8;
                    local_148 = (undefined3)uStack_e0;
                    iStack_145 = (int5)(CONCAT44(uStack_dc,uStack_e0) >> 0x18);
                    local_150 = (undefined3)uStack_e8;
                    iStack_14d = (int5)(CONCAT44(uStack_e4,uStack_e8) >> 0x18);
                    iStack_138 = iStack_d0;
                    uStack_133 = uStack_cb;
                    local_140._0_4_ = (undefined4)local_d8;
                    local_140._4_4_ = (uint)(CONCAT35(uStack_d3,local_d8) >> 0x20);
                    local_1e0 = local_108;
                    pppppppuVar17 = local_100;
LAB_1008ec8c8:
                    uStack_1d8 = SUB83(pppppppuVar17,0);
                    iStack_1d5 = (int5)((ulong)pppppppuVar17 >> 0x18);
                    uStack_1c8 = uStack_158;
                    iStack_1c5 = iStack_155;
                    local_1d0 = SUB83(local_160,0);
                    iStack_1cd = (int5)((ulong)local_160 >> 0x18);
                    iStack_1b8 = (int5)CONCAT53(iStack_145,local_148);
                    uStack_1b3 = (undefined3)((uint5)iStack_145 >> 0x10);
                    local_1c0 = (undefined4)CONCAT53(iStack_14d,local_150);
                    uStack_1bc = (uint)((uint5)iStack_14d >> 8);
                    psStack_1a8 = (sqlite3_stmt *)CONCAT35(uStack_133,iStack_138);
                    local_1b0 = (int5)CONCAT44(local_140._4_4_,(undefined4)local_140);
                    uStack_1ab = (undefined3)(local_140._4_4_ >> 8);
                  }
                  ::core::ptr::drop_in_place<rusqlite::row::Rows>(&local_118);
                }
                else {
                  local_1e0 = (undefined8 *******)0x8000000000000013;
                  uStack_1d8 = 0;
                  iStack_1d5 = 0;
                  local_1d0 = (undefined3)iVar8;
                  iStack_1cd = (int5)(char)((uint)iVar8 >> 0x18);
                }
                ::core::ptr::drop_in_place<rusqlite::statement::Statement>(&local_1a0);
              }
LAB_1008ec92c:
              pppppppuVar17 = (undefined8 *******)CONCAT53(iStack_1d5,uStack_1d8);
              if (local_1e0 != (undefined8 *******)0x8000000000000016) {
                pppppppuVar20 = (undefined8 *******)CONCAT53(iStack_1cd,local_1d0);
                pppppppuVar15 = (undefined8 *******)CONCAT53(iStack_1c5,uStack_1c8);
                uVar25 = uStack_1bc & 0xff;
                local_2b0 = (undefined8 *******)CONCAT53(iStack_1b8,uStack_1bc._1_3_);
LAB_1008ec968:
                uVar27 = local_1c0;
                pppppppuVar28 = local_1e0;
                pppppppuVar21 = (undefined8 *******)((long)pppppppuVar5 + 8);
                uStack_2a8 = uStack_1b3;
                iStack_2a5 = local_1b0;
                uStack_2a0 = uStack_1ab;
                psStack_29d = psStack_1a8;
                do {
                  if (pppppppuVar21[-1] != (undefined8 ******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppuVar21,pppppppuVar21[-1],1);
                  }
                  pppppppuVar21 = pppppppuVar21 + 3;
                  pppppppuVar18 = (undefined8 *******)((long)pppppppuVar18 + -1);
                } while (pppppppuVar18 != (undefined8 *******)0x0);
                if (pppppppuVar4 != (undefined8 *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar5,(long)pppppppuVar4 * 0x18,8);
                }
                goto LAB_1008ec9c4;
              }
              pppppppuVar20 = (undefined8 *******)((long)pppppppuVar5 + 8);
              do {
                if (pppppppuVar20[-1] != (undefined8 ******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*pppppppuVar20,pppppppuVar20[-1],1);
                }
                pppppppuVar20 = pppppppuVar20 + 3;
                pppppppuVar18 = (undefined8 *******)((long)pppppppuVar18 + -1);
              } while (pppppppuVar18 != (undefined8 *******)0x0);
            }
            if (pppppppuVar4 != (undefined8 *******)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar5,(long)pppppppuVar4 * 0x18,8);
            }
            ::core::ptr::drop_in_place<rusqlite::Connection>(&local_288);
LAB_1008ecb90:
            param_1[1] = lVar14;
            *param_1 = lVar10;
            param_1[3] = (long)pppppppuVar17;
            param_1[2] = (long)pppppppuVar28;
            param_1[5] = lStack_2b8;
            param_1[4] = CONCAT17(uStack_2b9,CONCAT16(uStack_2ba,CONCAT15(2,CONCAT14((char)uVar19,uVar9))));
            return;
          }
        }
        uStack_2a8 = uStack_1d8;
        local_2b0 = local_1e0;
        psStack_29d = (sqlite3_stmt *)CONCAT35(uStack_1c8,iStack_1cd);
        iStack_2a5 = iStack_1d5;
        uStack_2a0 = local_1d0;
        uVar25 = uVar22;
        local_308 = uVar27;
      }
      else {
        local_2b0 = (undefined8 *******)CONCAT17((undefined1)uStack_dc,CONCAT43(uStack_e0,uStack_e4._1_3_));
        uStack_2a8 = uStack_dc._1_3_;
        psStack_29d = (sqlite3_stmt *)CONCAT35(uStack_cb,iStack_d0);
        iStack_2a5 = local_d8;
        uStack_2a0 = uStack_d3;
        pppppppuVar20 = pppppppuStack_f8;
        pppppppuVar28 = local_108;
        pppppppuVar15 = pppppppuStack_f0;
        pppppppuVar17 = local_100;
        uVar27 = uStack_e8;
        uVar25 = uStack_e4 & 0xff;
      }
LAB_1008ec9c4:
      uVar19 = (ulong)pppppppuVar15 & 0xff;
      ::core::ptr::drop_in_place<rusqlite::Connection>(&local_288);
      if (pppppppuVar28 != (undefined8 *******)&DAT_8000000000000000) {
LAB_1008eca54:
        pppppppuStack_270 = (undefined8 *******)((ulong)pppppppuVar15 & 0xffffffffffffff00 | uVar19);
        uStack_25b = uStack_2a8;
        local_264._1_3_ = SUB83(local_2b0,0);
        local_264 = CONCAT31(local_264._1_3_,(char)uVar25);
        local_260 = (int5)((ulong)local_2b0 >> 0x18);
        local_250 = psStack_29d;
        iStack_258 = iStack_2a5;
        uStack_253 = uStack_2a0;
        uStack_2bb = uVar25 != 9 && uVar25 != 0x16 || pppppppuVar28 != (undefined8 *******)&DAT_8000000000000000;
        local_288 = pppppppuVar28;
        local_280 = pppppppuVar17;
        pppppppuStack_278 = pppppppuVar20;
        local_268 = uVar27;
        if (1 < log::MAX_LOG_LEVEL_FILTER) {
          uStack_198 = (uint)param_3;
          uStack_194 = (undefined4)((ulong)param_3 >> 0x20);
          local_160 = &local_1a0;
          uStack_158 = 0x15ed44;
          iStack_155 = 0x101;
          local_150 = SUB83(&uStack_2bb,0);
          iStack_14d = (int5)((ulong)&uStack_2bb >> 0x18);
          local_148 = 0x913464;
          iStack_145 = 0x100;
          local_140 = &local_288;
          iStack_138 = 0x100d8066c;
          uStack_133 = 0;
          iStack_d0 = 0x1013b7823;
          uStack_cb = 0;
          local_c8 = (sqlite3_stmt *)&DAT_0000001e;
          psStack_b0 = (sqlite3_stmt *)&local_160;
          local_b8 = (sqlite3_stmt *)s_2_AiMaMi__state_db__semantic_pro_10153210d;
          local_108 = (undefined8 *******)0x0;
          local_100 = (undefined8 *******)0x1013b7823;
          pppppppuStack_f0 = (undefined8 *******)0x0;
          pppppppuStack_f8 = (undefined8 *******)&DAT_0000001e;
          uStack_e8 = 0x1525f09;
          uStack_e4 = 1;
          local_d8 = 2;
          uStack_d3 = 0;
          uStack_e0 = 0x15;
          uStack_dc = 0;
          psStack_c0 = (sqlite3_stmt *)0x11700000001;
          local_1a0 = param_2;
          <log::__private_api::GlobalLogger_as_log::Log>::log(&local_1e0,&local_108);
        }
        param_1[1] = lVar14;
        *param_1 = lVar10;
        param_1[3] = 0;
        param_1[2] = 0;
        param_1[5] = lStack_2b8;
        param_1[4] = CONCAT17(uStack_2b9,CONCAT16(uStack_2ba,CONCAT15(uStack_2bb,(uint5)uVar9)));
        ::core::ptr::drop_in_place<rusqlite::error::Error>(&local_288);
        return;
      }
    }
    if (((0xe < uVar25) || ((1 << (ulong)(uVar25 & 0x1f) & 0x7118U) == 0)) || (local_2e4 == 3)) goto LAB_1008eca54;
    local_108 = (undefined8 *******)&DAT_8000000000000000;
    pppppppuStack_f0 = (undefined8 *******)((ulong)pppppppuVar15 & 0xffffffffffffff00 | uVar19);
    uStack_e4._1_3_ = SUB83(local_2b0,0);
    uStack_e4 = CONCAT31(uStack_e4._1_3_,(char)uVar25);
    uStack_e0 = (undefined4)((ulong)local_2b0 >> 0x18);
    uStack_dc._0_1_ = (undefined1)((ulong)local_2b0 >> 0x38);
    uStack_dc = CONCAT31(uStack_2a8,(undefined1)uStack_dc);
    iStack_d0 = (int5)psStack_29d;
    uStack_cb = (undefined3)((ulong)psStack_29d >> 0x28);
    local_d8 = iStack_2a5;
    uStack_d3 = uStack_2a0;
    local_100 = pppppppuVar17;
    pppppppuStack_f8 = pppppppuVar20;
    uStack_e8 = uVar27;
    std::thread::functions::sleep(0,25000000);
    local_2e4 = local_2e4 + 1;
    ::core::ptr::drop_in_place<rusqlite::error::Error>(&local_108);
  } while( true );
}
