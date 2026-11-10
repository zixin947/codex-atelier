/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* configured_sqlite_home @ 1008ee934 requested 1008ee934 */

/* WARNING: Removing unreachable block (ram,0x0001008eeb94) */
/* codexmate_lib::platform::paths::CodexPaths::configured_sqlite_home */

void codexmate_lib::platform::paths::CodexPaths::configured_sqlite_home(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  size_t sVar2;
  long lVar3;
  ulong uVar4;
  byte bVar5;
  undefined7 uVar6;
  char *pcVar7;
  char *pcVar8;
  char *pcVar9;
  int iVar10;
  ulong uVar11;
  long lVar12;
  undefined8 *puVar13;
  char *pcVar14;
  undefined8 *puVar15;
  char *pcVar16;
  char *pcVar17;
  undefined1 *puVar18;
  undefined1 *puVar19;
  long lVar20;
  undefined8 uVar21;
  undefined1 *puVar22;
  long lVar23;
  long lVar24;
  undefined1 auVar25 [16];
  undefined8 local_2c0;
  undefined8 local_2b8;
  long local_2a0;
  undefined8 local_298;
  long lStack_290;
  undefined7 local_288;
  byte bStack_281;
  undefined7 uStack_280;
  uint local_258;
  undefined4 uStack_254;
  byte local_250;
  undefined7 uStack_24f;
  byte local_248;
  undefined7 uStack_247;
  byte local_240;
  undefined7 uStack_23f;
  long local_238;
  long local_230;
  char *local_228;
  char *local_220;
  char *local_218;
  char *local_210;
  char *local_208;
  char *local_200;
  undefined8 local_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined8 local_1e0;
  undefined8 uStack_1d8;
  undefined8 local_1d0;
  undefined8 uStack_1c8;
  undefined8 local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  undefined8 uStack_1a8;
  undefined8 local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  byte local_160;
  undefined7 uStack_15f;
  byte bStack_158;
  undefined7 uStack_157;
  byte local_150;
  undefined7 uStack_14f;
  byte local_148;
  undefined7 uStack_147;
  long lStack_140;
  long local_138;
  char *pcStack_130;
  char *local_128;
  char *pcStack_120;
  char *local_118;
  char *pcStack_110;
  char *local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  
  std::env::current_dir(&local_160);
  uVar4 = CONCAT71(uStack_15f,local_160);
  if (uVar4 == 0x8000000000000000) {
    if ((bStack_158 & 3) == 1) {
      puVar15 = (undefined8 *)(CONCAT71(uStack_157,bStack_158) + -1);
      uVar21 = *puVar15;
      puVar13 = *(undefined8 **)(CONCAT71(uStack_157,bStack_158) + 7);
      if ((code *)*puVar13 != (code *)0x0) {
        (*(code *)*puVar13)(uVar21);
      }
      if (puVar13[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar21,puVar13[1],puVar13[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar15,0x18,8);
    }
  }
  else {
    local_2b8 = CONCAT71(uStack_157,bStack_158);
    local_2c0 = CONCAT71(uStack_14f,local_150);
  }
  std::env::_var_os(&local_2a0,
                    "CODEX_SQLITE_HOMEstate_5.sqliteLibrary/LaunchAgents/dev.aimami.auto-switch.plistconfig.tomlsqlite_homecodexmatecustom-instructionsrelayauth.jsonsession_index.jsonlarchived_sessionsskillsregistry.jsonsnapshotsbackupsauto-switch.logskill-backupsquota-history.jsonlsettings.jsonbootstrap-cache.jsonauto-switch-pending.jsonauto-switch-snooze.jsonvoice-workspace.jsonvoice-runtime.jsonstate.jsonAGENTS.mdhistorymobiledesktopstruct ExportFilestruct ExportedAccountschemehostpath-patternpathPatternpath-prefixpathPrefixpath-suffixpathSuffixapp-linkappLinkDeepLinkProtocolschemesdomainsnamedata did not match any variant of untagged enum DesktopProtocolAssociatedDomainConfig://struct RouterUnlockAuthMarkerstruct NoAccountModelRestoreEntrystruct NoAccountModelRestoreJournalstruct ModelRestoreEntrystruct ModelRestoreJournalfailed to write whole bufferinternal error: entered unreachable codeinternal error: entered unreachable code: invalid Once stateNo file has been startedwrite(): ZipWriter was already closedLarge file option has not been set()a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` value/ErrorIoInvalidArchiveUnsupportedArchiveFileNotFoundInvalidPasswordassertion failed: self.is_char_boundary(new_len)assertion failed: vec.capacity() - start >= lenConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedTauriPathForbiddenGlobPatternInvalidPathUrlUnsafePathBufTryFromIntErrorInvalidUncertain"
                    ,0x11);
  std::path::Path::_join
            (&local_160,param_2,param_3,
             "config.tomlsqlite_homecodexmatecustom-instructionsrelayauth.jsonsession_index.jsonlarchived_sessionsskillsregistry.jsonsnapshotsbackupsauto-switch.logskill-backupsquota-history.jsonlsettings.jsonbootstrap-cache.jsonauto-switch-pending.jsonauto-switch-snooze.jsonvoice-workspace.jsonvoice-runtime.jsonstate.jsonAGENTS.mdhistorymobiledesktopstruct ExportFilestruct ExportedAccountschemehostpath-patternpathPatternpath-prefixpathPrefixpath-suffixpathSuffixapp-linkappLinkDeepLinkProtocolschemesdomainsnamedata did not match any variant of untagged enum DesktopProtocolAssociatedDomainConfig://struct RouterUnlockAuthMarkerstruct NoAccountModelRestoreEntrystruct NoAccountModelRestoreJournalstruct ModelRestoreEntrystruct ModelRestoreJournalfailed to write whole bufferinternal error: entered unreachable codeinternal error: entered unreachable code: invalid Once stateNo file has been startedwrite(): ZipWriter was already closedLarge file option has not been set()a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` value/ErrorIoInvalidArchiveUnsupportedArchiveFileNotFoundInvalidPasswordassertion failed: self.is_char_boundary(new_len)assertion failed: vec.capacity() - start >= lenConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedTauriPathForbiddenGlobPatternInvalidPathUrlUnsafePathBufTryFromIntErrorInvalidUncertain"
             ,0xb);
  uVar21 = CONCAT71(uStack_157,bStack_158);
  std::fs::read_to_string::inner(&local_258,uVar21,CONCAT71(uStack_14f,local_150));
  if (CONCAT71(uStack_15f,local_160) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar21,CONCAT71(uStack_15f,local_160),1);
  }
  puVar22 = (undefined1 *)CONCAT44(uStack_254,local_258);
  uVar21 = CONCAT71(uStack_24f,local_250);
  if (puVar22 == &DAT_8000000000000000) {
    ::core::ptr::drop_in_place<core::result::Result<alloc::string::String,std::io::error::Error>>(0x8000000000000000,uVar21);
  }
  else {
    <toml_edit::de::Deserializer_as_core::str::traits::FromStr>::from_str(&local_258,uVar21,CONCAT71(uStack_247,local_248));
    lVar20 = CONCAT71(uStack_24f,local_250);
    uVar6 = uStack_247;
    lVar23 = local_238;
    lVar12 = local_230;
    pcVar7 = local_220;
    pcVar8 = local_218;
    pcVar16 = local_210;
    pcVar14 = local_208;
    pcVar9 = local_200;
    bVar5 = local_248;
    if (CONCAT44(uStack_254,local_258) == 2) {
joined_r0x0001008eeba8:
      pcVar17 = pcVar9;
      if (lVar20 != 2) {
        if (lVar23 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar12,lVar23,1);
        }
        if (((ulong)pcVar14 & 0x7fffffffffffffff) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar9,pcVar14,1);
        }
        if (pcVar16 != (char *)0x0) {
          pcVar14 = pcVar8 + 8;
          do {
            if (*(long *)(pcVar14 + -8) != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)pcVar14,*(long *)(pcVar14 + -8),1);
            }
            pcVar14 = pcVar14 + 0x18;
            pcVar16 = pcVar16 + -1;
          } while (pcVar16 != (char *)0x0);
        }
        if (pcVar7 != (char *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar8,(long)pcVar7 * 0x18,8);
        }
        uVar6 = local_288;
        local_240 = bStack_281;
        uStack_23f = uStack_280;
        bVar5 = 7;
      }
    }
    else {
      uStack_a0 = uStack_198;
      local_a8 = local_1a0;
      uStack_90 = uStack_188;
      local_98 = local_190;
      uStack_80 = uStack_178;
      local_88 = local_180;
      uStack_70 = uStack_168;
      local_78 = local_170;
      uStack_e0 = uStack_1d8;
      local_e8 = local_1e0;
      uStack_d0 = uStack_1c8;
      local_d8 = local_1d0;
      uStack_c0 = uStack_1b8;
      local_c8 = local_1c0;
      uStack_b0 = uStack_1a8;
      local_b8 = local_1b0;
      uStack_f0 = uStack_1e8;
      local_f8 = local_1f0;
      local_160 = (byte)local_258;
      uStack_15f = (undefined7)((ulong)CONCAT44(uStack_254,local_258) >> 8);
      bStack_158 = local_250;
      uStack_157 = uStack_24f;
      local_150 = local_248;
      uStack_14f = uStack_247;
      local_148 = local_240;
      uStack_147 = uStack_23f;
      lStack_140 = local_238;
      local_138 = local_230;
      pcStack_130 = local_228;
      local_128 = local_220;
      pcStack_120 = local_218;
      local_118 = local_210;
      pcStack_110 = local_208;
      local_108 = local_200;
      uStack_100 = local_1f8;
      <toml_edit::de::Deserializer_as_serde_core::de::Deserializer>::deserialize_any(&local_258,&local_160);
      lVar20 = CONCAT44(uStack_254,local_258);
      lVar23 = CONCAT71(uStack_23f,local_240);
      uVar6 = uStack_24f;
      local_240 = local_248;
      uStack_23f = uStack_247;
      lVar12 = local_238;
      pcVar7 = local_228;
      pcVar8 = local_220;
      pcVar16 = local_218;
      pcVar14 = local_210;
      pcVar9 = local_208;
      bVar5 = local_250;
      pcVar17 = local_200;
      if (lVar20 != 2) goto joined_r0x0001008eeba8;
    }
    uStack_280 = uStack_23f;
    bStack_281 = local_240;
    local_288 = uVar6;
    if (puVar22 != (undefined1 *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar21,puVar22,1);
    }
    if (bVar5 != 7) {
      uStack_15f = local_288;
      bStack_158 = bStack_281;
      uStack_157 = uStack_280;
      local_150 = (byte)lVar23;
      uStack_14f = (undefined7)((ulong)lVar23 >> 8);
      local_148 = (byte)lVar12;
      uStack_147 = (undefined7)((ulong)lVar12 >> 8);
      local_160 = bVar5;
      if ((bVar5 == 6) && (lVar20 = CONCAT71(uStack_280,bStack_281), lVar20 != 0)) {
        pcVar17 = 
        "sqlite_homecodexmatecustom-instructionsrelayauth.jsonsession_index.jsonlarchived_sessionsskillsregistry.jsonsnapshotsbackupsauto-switch.logskill-backupsquota-history.jsonlsettings.jsonbootstrap-cache.jsonauto-switch-pending.jsonauto-switch-snooze.jsonvoice-workspace.jsonvoice-runtime.jsonstate.jsonAGENTS.mdhistorymobiledesktopstruct ExportFilestruct ExportedAccountschemehostpath-patternpathPatternpath-prefixpathPrefixpath-suffixpathSuffixapp-linkappLinkDeepLinkProtocolschemesdomainsnamedata did not match any variant of untagged enum DesktopProtocolAssociatedDomainConfig://struct RouterUnlockAuthMarkerstruct NoAccountModelRestoreEntrystruct NoAccountModelRestoreJournalstruct ModelRestoreEntrystruct ModelRestoreJournalfailed to write whole bufferinternal error: entered unreachable codeinternal error: entered unreachable code: invalid Once stateNo file has been startedwrite(): ZipWriter was already closedLarge file option has not been set()a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` value/ErrorIoInvalidArchiveUnsupportedArchiveFileNotFoundInvalidPasswordassertion failed: self.is_char_boundary(new_len)assertion failed: vec.capacity() - start >= lenConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedTauriPathForbiddenGlobPatternInvalidPathUrlUnsafePathBufTryFromIntErrorInvalidUncertain"
        ;
        do {
          puVar22 = (undefined1 *)(ulong)*(ushort *)(lVar20 + 0x272);
          lVar24 = (long)puVar22 * 0x18;
          puVar18 = (undefined1 *)0xffffffffffffffff;
          lVar12 = lVar20 + 0x168;
          puVar13 = (undefined8 *)(lVar20 + -0x10);
          do {
            puVar15 = puVar13;
            puVar19 = puVar22;
            if (lVar24 == 0) goto LAB_1008eed00;
            sVar2 = *(ulong *)(lVar12 + 0x10);
            lVar3 = 0xb - sVar2;
            if (0xb < sVar2 || lVar3 == 0) {
              sVar2 = 0xb;
            }
            iVar10 = _memcmp("sqlite_homecodexmatecustom-instructionsrelayauth.jsonsession_index.jsonlarchived_sessionsskillsregistry.jsonsnapshotsbackupsauto-switch.logskill-backupsquota-history.jsonlsettings.jsonbootstrap-cache.jsonauto-switch-pending.jsonauto-switch-snooze.jsonvoice-workspace.jsonvoice-runtime.jsonstate.jsonAGENTS.mdhistorymobiledesktopstruct ExportFilestruct ExportedAccountschemehostpath-patternpathPatternpath-prefixpathPrefixpath-suffixpathSuffixapp-linkappLinkDeepLinkProtocolschemesdomainsnamedata did not match any variant of untagged enum DesktopProtocolAssociatedDomainConfig://struct RouterUnlockAuthMarkerstruct NoAccountModelRestoreEntrystruct NoAccountModelRestoreJournalstruct ModelRestoreEntrystruct ModelRestoreJournalfailed to write whole bufferinternal error: entered unreachable codeinternal error: entered unreachable code: invalid Once stateNo file has been startedwrite(): ZipWriter was already closedLarge file option has not been set()a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` value/ErrorIoInvalidArchiveUnsupportedArchiveFileNotFoundInvalidPasswordassertion failed: self.is_char_boundary(new_len)assertion failed: vec.capacity() - start >= lenConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedTauriPathForbiddenGlobPatternInvalidPathUrlUnsafePathBufTryFromIntErrorInvalidUncertain"
                             ,*(void **)(lVar12 + 8),sVar2);
            if (iVar10 != 0) {
              lVar3 = (long)iVar10;
            }
            cVar1 = 0 < lVar3;
            if (lVar3 < 0) {
              cVar1 = -1;
            }
            puVar18 = puVar18 + 1;
            lVar24 = lVar24 + -0x18;
            lVar12 = lVar12 + 0x18;
            puVar13 = puVar15 + 4;
          } while (cVar1 == '\x01');
          puVar19 = puVar18;
          if (cVar1 == '\0') {
            if (*(char *)(puVar15 + 2) == '\0') {
              auVar25 = ::core::str::<impl_str>::trim_matches(puVar15[4],puVar15[5]);
              pcVar16 = auVar25._8_8_;
              lVar20 = -0x8000000000000000;
              if ((auVar25._0_8_ != (void *)0x0) && (pcVar17 = pcVar16, pcVar16 != (char *)0x0)) {
                alloc::raw_vec::RawVecInner<A>::try_allocate_in(&local_258,pcVar16,0,1,1);
                lVar20 = CONCAT71(uStack_24f,local_250);
                if ((local_258 & 1) != 0) {
                    /* WARNING: Subroutine does not return */
                  alloc::raw_vec::handle_error(lVar20,CONCAT71(uStack_247,local_248));
                }
                puVar22 = (undefined1 *)CONCAT71(uStack_247,local_248);
                _memcpy(puVar22,auVar25._0_8_,(size_t)pcVar16);
              }
            }
            else {
              lVar20 = -0x8000000000000000;
            }
            goto LAB_1008eed1c;
          }
LAB_1008eed00:
          if (lVar23 == 0) break;
          lVar20 = *(long *)(lVar20 + (long)puVar19 * 8 + 0x278);
          lVar23 = lVar23 + -1;
        } while( true );
      }
      lVar20 = -0x8000000000000000;
LAB_1008eed1c:
      ::core::ptr::drop_in_place<toml::value::Value>(&local_160);
      if (lVar20 != -0x8000000000000000) {
        uVar11 = std::path::Path::is_absolute(puVar22,pcVar17);
        if (((uVar11 & 1) == 0) && (uVar4 != 0x8000000000000000)) {
          std::path::Path::_join(&local_160,local_2b8,local_2c0,puVar22,pcVar17);
          if (lVar20 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar22,lVar20,1);
          }
        }
        else {
          local_160 = (byte)lVar20;
          uStack_15f = (undefined7)((ulong)lVar20 >> 8);
          bStack_158 = (byte)puVar22;
          uStack_157 = (undefined7)((ulong)puVar22 >> 8);
          local_150 = (byte)pcVar17;
          uStack_14f = (undefined7)((ulong)pcVar17 >> 8);
        }
        param_1[1] = CONCAT71(uStack_157,bStack_158);
        *param_1 = CONCAT71(uStack_15f,local_160);
        param_1[2] = CONCAT71(uStack_14f,local_150);
        if ((local_2a0 != -0x8000000000000000) && (local_2a0 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_298,local_2a0,1);
        }
        goto LAB_1008eed84;
      }
    }
  }
  if (local_2a0 != -0x8000000000000000) {
    if (lStack_290 != 0) {
      uVar11 = std::path::Path::is_absolute(local_298,lStack_290);
      if (((uVar11 & 1) == 0) && (uVar4 != 0x8000000000000000)) {
        std::path::Path::_join(&local_160,local_2b8,local_2c0,local_298,lStack_290);
        if (local_2a0 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_298,local_2a0,1);
        }
      }
      else {
        local_160 = (byte)local_2a0;
        uStack_15f = (undefined7)((ulong)local_2a0 >> 8);
        bStack_158 = (byte)local_298;
        uStack_157 = (undefined7)((ulong)local_298 >> 8);
        local_150 = (byte)lStack_290;
        uStack_14f = (undefined7)((ulong)lStack_290 >> 8);
      }
      param_1[1] = CONCAT71(uStack_157,bStack_158);
      *param_1 = CONCAT71(uStack_15f,local_160);
      param_1[2] = CONCAT71(uStack_14f,local_150);
      goto LAB_1008eed84;
    }
    if (local_2a0 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_298,local_2a0,1);
    }
  }
  *param_1 = 0x8000000000000000;
LAB_1008eed84:
  if ((uVar4 & 0x7fffffffffffffff) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2b8,uVar4,1);
  }
  return;
}

