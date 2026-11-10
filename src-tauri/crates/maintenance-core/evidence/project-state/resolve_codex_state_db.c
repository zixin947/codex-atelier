/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* resolve_codex_state_db @ 1008eeff8 requested 1008eeff8 */

/* codexmate_lib::platform::paths::CodexPaths::resolve_codex_state_db */

void codexmate_lib::platform::paths::CodexPaths::resolve_codex_state_db(long *param_1,undefined8 param_2,undefined8 param_3)

{
  long *plVar1;
  void *pvVar2;
  long lVar3;
  ulong uVar4;
  size_t sVar5;
  void *pvVar6;
  long *plVar7;
  undefined8 uVar8;
  long lVar9;
  undefined8 local_118;
  long local_110;
  long local_108;
  size_t local_100;
  void *local_f8;
  size_t local_f0;
  long local_e8;
  long *plStack_e0;
  ulong local_d8;
  undefined1 local_d0 [16];
  undefined8 local_c0;
  long lStack_b8;
  long local_b0;
  long lStack_a8;
  long local_a0;
  long local_90;
  long local_88;
  long lStack_80;
  long local_78;
  long *local_70;
  long *local_68;
  long local_60;
  long *local_58;

  state_db_search_dirs(&local_118);
  if (local_108 == 0) {
    std::path::Path::_join
              (&local_100,param_2,param_3,
               "sqlitestate_.sqliteSELECT COALESCE(MAX(updated_at), 0) FROM threadsSELECT COALESCE(MAX(updated_at_ms), 0) FROM threadsSELECT COUNT(*) FROM threadsSELECT COUNT(*) FROM threads WHERE COALESCE(archived, 0) = 0"
               ,6);
  }
  else {
    sVar5 = *(size_t *)(local_110 + 0x10);
    if ((long)sVar5 < 0) {
      uVar8 = 0;
LAB_1008ef040:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(uVar8,sVar5);
    }
    pvVar6 = *(void **)(local_110 + 8);
    if (sVar5 == 0) {
      pvVar2 = (void *)0x1;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar8 = 1;
      pvVar2 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar5,1);
      if (pvVar2 == (void *)0x0) goto LAB_1008ef040;
    }
    _memcpy(pvVar2,pvVar6,sVar5);
    local_100 = sVar5;
    local_f8 = pvVar2;
    local_f0 = sVar5;
  }
  lStack_b8 = local_110 + local_108 * 0x20;
  local_c0 = local_118;
  alloc::vec::in_place_collect::from_iter_in_place(&local_e8,local_d0);
  if (1 < local_d8) {
    if (*plStack_e0 == -0x8000000000000000) {
      uVar4 = local_d8 * 0x20 - 0x20 >> 5;
      plVar7 = plStack_e0 + 5;
      do {
        if (plVar7[-1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*plVar7,plVar7[-1],1);
        }
        plVar7 = plVar7 + 4;
        uVar4 = uVar4 - 1;
      } while (uVar4 != 0);
      if (local_e8 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_e0,local_e8 << 5,8);
      }
    }
    else {
      local_58 = plStack_e0 + local_d8 * 4;
      lStack_80 = plStack_e0[2];
      local_88 = plStack_e0[1];
      local_78 = plStack_e0[3];
      local_70 = plStack_e0;
      local_60 = local_e8;
      local_90 = *plStack_e0;
      local_68 = plStack_e0 + 4;
      <alloc::vec::into_iter::IntoIter<T,A>as_core::iter::traits::iterator::Iterator>::fold(&local_b0,&local_70,&local_90);
      if (local_b0 != -0x8000000000000000) {
        param_1[1] = lStack_a8;
        *param_1 = local_b0;
        param_1[2] = local_a0;
        goto LAB_1008ef260;
      }
    }
    std::path::Path::_join
              (param_1,local_f8,local_f0,
               "state_5.sqliteLibrary/LaunchAgents/dev.aimami.auto-switch.plistconfig.tomlsqlite_homecodexmatecustom-instructionsrelayauth.jsonsession_index.jsonlarchived_sessionsskillsregistry.jsonsnapshotsbackupsauto-switch.logskill-backupsquota-history.jsonlsettings.jsonbootstrap-cache.jsonauto-switch-pending.jsonauto-switch-snooze.jsonvoice-workspace.jsonvoice-runtime.jsonstate.jsonAGENTS.mdhistorymobiledesktopstruct ExportFilestruct ExportedAccountschemehostpath-patternpathPatternpath-prefixpathPrefixpath-suffixpathSuffixapp-linkappLinkDeepLinkProtocolschemesdomainsnamedata did not match any variant of untagged enum DesktopProtocolAssociatedDomainConfig://struct RouterUnlockAuthMarkerstruct NoAccountModelRestoreEntrystruct NoAccountModelRestoreJournalstruct ModelRestoreEntrystruct ModelRestoreJournalfailed to write whole bufferinternal error: entered unreachable codeinternal error: entered unreachable code: invalid Once stateNo file has been startedwrite(): ZipWriter was already closedLarge file option has not been set()a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` value/ErrorIoInvalidArchiveUnsupportedArchiveFileNotFoundInvalidPasswordassertion failed: self.is_char_boundary(new_len)assertion failed: vec.capacity() - start >= lenConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedTauriPathForbiddenGlobPatternInvalidPathUrlUnsafePathBufTryFromIntErrorInvalidUncertain"
               ,0xe);
    goto LAB_1008ef260;
  }
  plVar1 = plStack_e0 + local_d8 * 4;
  local_70 = plStack_e0;
  local_60 = local_e8;
  plVar7 = plStack_e0;
  local_58 = plVar1;
  if (local_d8 == 0) {
LAB_1008ef114:
    local_68 = plVar7;
    std::path::Path::_join
              (param_1,local_f8,local_f0,
               "state_5.sqliteLibrary/LaunchAgents/dev.aimami.auto-switch.plistconfig.tomlsqlite_homecodexmatecustom-instructionsrelayauth.jsonsession_index.jsonlarchived_sessionsskillsregistry.jsonsnapshotsbackupsauto-switch.logskill-backupsquota-history.jsonlsettings.jsonbootstrap-cache.jsonauto-switch-pending.jsonauto-switch-snooze.jsonvoice-workspace.jsonvoice-runtime.jsonstate.jsonAGENTS.mdhistorymobiledesktopstruct ExportFilestruct ExportedAccountschemehostpath-patternpathPatternpath-prefixpathPrefixpath-suffixpathSuffixapp-linkappLinkDeepLinkProtocolschemesdomainsnamedata did not match any variant of untagged enum DesktopProtocolAssociatedDomainConfig://struct RouterUnlockAuthMarkerstruct NoAccountModelRestoreEntrystruct NoAccountModelRestoreJournalstruct ModelRestoreEntrystruct ModelRestoreJournalfailed to write whole bufferinternal error: entered unreachable codeinternal error: entered unreachable code: invalid Once stateNo file has been startedwrite(): ZipWriter was already closedLarge file option has not been set()a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` value/ErrorIoInvalidArchiveUnsupportedArchiveFileNotFoundInvalidPasswordassertion failed: self.is_char_boundary(new_len)assertion failed: vec.capacity() - start >= lenConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedTauriPathForbiddenGlobPatternInvalidPathUrlUnsafePathBufTryFromIntErrorInvalidUncertain"
               ,0xe);
  }
  else {
    plVar7 = plStack_e0 + 4;
    lVar3 = *plStack_e0;
    if (lVar3 == -0x8000000000000000) goto LAB_1008ef114;
    lVar9 = plStack_e0[1];
    param_1[2] = plStack_e0[2];
    param_1[1] = lVar9;
    *param_1 = lVar3;
    local_68 = plVar7;
  }
  if ((long)plVar1 - (long)plVar7 != 0) {
    uVar4 = (ulong)((long)plVar1 - (long)plVar7) >> 5;
    plVar7 = plVar7 + 1;
    do {
      if (plVar7[-1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*plVar7,plVar7[-1],1);
      }
      plVar7 = plVar7 + 4;
      uVar4 = uVar4 - 1;
    } while (uVar4 != 0);
  }
  if (local_e8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_e0,local_e8 << 5,8);
  }
LAB_1008ef260:
  if (local_100 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_f8,local_100,1);
  }
  return;
}
