/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* inspect @ 1003b5b34 requested 1003b5b34 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::codex_project_state::inspect */

void codexmate_lib::core::relay::codex_project_state::inspect(undefined8 *param_1,long param_2)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  undefined8 uVar4;
  undefined1 **ppuVar5;
  undefined1 *puVar6;
  undefined **ppuVar7;
  int iVar8;
  char *pcVar9;
  undefined8 *puVar10;
  undefined1 **ppuVar11;
  undefined1 ***pppuVar12;
  long lVar13;
  undefined **ppuVar14;
  long lVar15;
  undefined1 *puVar16;
  long lVar17;
  undefined1 uVar18;
  undefined1 uVar19;
  long lVar20;
  long lVar21;
  undefined8 *puVar22;
  undefined8 uVar23;
  undefined **local_1f0;
  undefined1 **local_1e8;
  undefined8 local_1e0;
  undefined8 local_1d8;
  undefined1 *local_1a8;
  long local_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined1 **local_170;
  undefined7 uStack_168;
  undefined1 uStack_161;
  undefined7 local_160;
  undefined1 uStack_159;
  undefined7 uStack_158;
  char local_150;
  undefined7 uStack_14f;
  undefined1 uStack_148;
  undefined7 uStack_147;
  undefined1 uStack_140;
  undefined7 uStack_13f;
  undefined8 uStack_138;
  undefined1 **local_130;
  undefined1 *local_128;
  undefined1 **local_120;
  undefined **local_118;
  undefined1 **local_110;
  undefined **local_108;
  undefined1 *local_100;
  undefined1 **local_f8;
  undefined1 **ppuStack_f0;
  undefined **local_e8;
  undefined1 *puStack_e0;
  undefined1 **local_d8;
  undefined **local_d0;
  long local_c8;
  long lStack_c0;
  undefined1 auStack_71 [17];

  std::path::Path::_join(&local_188,*(undefined8 *)(param_2 + 8),*(undefined8 *)(param_2 + 0x10),&DAT_101310fb6,0x18);
  uVar4 = local_178;
  uVar23 = uStack_180;
  std::sys::fs::metadata(&local_110,uStack_180,local_178);
  ppuVar11 = local_110;
  ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_110,local_108);
  if (((ulong)ppuVar11 & 1) == 0) {
    std::fs::read_to_string::inner(&local_128,uVar23,uVar4);
    ppuVar11 = local_120;
    puVar6 = local_128;
    if (local_128 == (undefined1 *)0x8000000000000000) {
      local_130 = local_120;
      local_150 = 0;
      uStack_14f = 0;
      uStack_148 = 1;
      uStack_147 = 0;
      uStack_140 = 0;
      uStack_13f = 0;
      local_100 = &DAT_60000020;
      local_110 = (undefined1 **)&local_150;
      local_108 = &_anon_164d03a2dfc98f969363eaf1c50504fd_1497;
      iVar8 = <std::io::error::Error_as_core::fmt::Display>::fmt(&local_130,&local_110);
      if (iVar8 != 0) {
                    /* WARNING: Subroutine does not return */
        ::core::result::unwrap_failed
                  ("a Display implementation returned an error unexpectedlycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` valueErrorConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedFromUtf8ErrorVec is sized conservativelyInvalid UTF8"
                   ,0x37,auStack_71,&_anon_164d03a2dfc98f969363eaf1c50504fd_1512,&_anon_164d03a2dfc98f969363eaf1c50504fd_1500);
      }
      local_190 = CONCAT71(uStack_14f,local_150);
      local_1d8 = CONCAT71(uStack_147,uStack_148);
      local_1e0 = CONCAT71(uStack_13f,uStack_140);
      if (((ulong)ppuVar11 & 3) == 1) {
        uVar23 = *(undefined8 *)((long)ppuVar11 + -1);
        puVar22 = *(undefined8 **)((long)ppuVar11 + 7);
        if ((code *)*puVar22 != (code *)0x0) {
          (*(code *)*puVar22)(uVar23);
        }
        if (puVar22[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar23,puVar22[1],puVar22[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)ppuVar11 + -1),0x18,8);
      }
      bVar3 = true;
      goto LAB_1003b5c6c;
    }
    local_110 = local_120;
    local_108 = local_118;
    local_100 = (undefined1 *)0x0;
    local_f8 = (undefined1 **)0x0;
    ppuStack_f0 = local_120;
    local_e8 = local_118;
    serde_json::de::from_trait(&local_150,&local_110);
    if (local_150 != '\x05') {
      if (local_150 == '\x06') {
        local_130 = (undefined1 **)CONCAT71(uStack_147,uStack_148);
        local_128 = (undefined1 *)0x0;
        local_120 = (undefined1 **)0x1;
        local_118 = (undefined **)0x0;
        local_100 = &DAT_60000020;
        local_110 = &local_128;
        local_108 = &_anon_164d03a2dfc98f969363eaf1c50504fd_1497;
        iVar8 = <serde_json::error::Error_as_core::fmt::Display>::fmt(&local_130,&local_110);
        local_1f0 = local_118;
        local_1e8 = local_120;
        local_1a8 = local_128;
        if (iVar8 != 0) {
                    /* WARNING: Subroutine does not return */
          ::core::result::unwrap_failed
                    ("a Display implementation returned an error unexpectedlycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` valueErrorConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedFromUtf8ErrorVec is sized conservativelyInvalid UTF8"
                     ,0x37,auStack_71,&_anon_164d03a2dfc98f969363eaf1c50504fd_1512,&_anon_164d03a2dfc98f969363eaf1c50504fd_1500);
        }
        ::core::ptr::drop_in_place<serde_json::error::Error>(&local_130);
      }
      else {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        local_1e8 = (undefined1 **)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x22,1);
        if (local_1e8 == (undefined1 **)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x22);
        }
        *(undefined2 *)(local_1e8 + 4) = 0x7463;
        local_1e8[1] = (undefined1 *)0x6f6f722065746174;
        *local_1e8 = (undefined1 *)0x732d6c61626f6c67;
        local_1e8[3] = (undefined1 *)0x656a626f206e6120;
        local_1e8[2] = (undefined1 *)0x746f6e2073692074;
        local_1f0 = (undefined **)0x22;
        local_1a8 = (undefined1 *)0x22;
      }
      if (puVar6 != (undefined1 *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar11,puVar6,1);
      }
      if (local_150 != '\x06') {
        ::core::ptr::drop_in_place<serde_json::value::Value>(&local_150);
      }
      local_190 = -0x8000000000000000;
      bVar3 = true;
      goto LAB_1003b5c7c;
    }
    local_170 = (undefined1 **)CONCAT17(uStack_148,uStack_14f);
    uStack_168 = uStack_147;
    uStack_159 = (undefined1)uStack_138;
    uStack_158 = (undefined7)((ulong)uStack_138 >> 8);
    uStack_161 = uStack_140;
    local_160 = uStack_13f;
    if (puVar6 != (undefined1 *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar11,puVar6,1);
    }
    local_150 = 5;
    uStack_147 = uStack_168;
    uStack_14f = SUB87(local_170,0);
    uStack_148 = (undefined1)((ulong)local_170 >> 0x38);
    uStack_138 = CONCAT71(uStack_158,uStack_159);
    uStack_140 = uStack_161;
    uStack_13f = local_160;
    pcVar9 = (char *)<str_as_serde_json::value::index::Index>::index_into(&DAT_10131129c,0x16,&local_150);
    if ((pcVar9 == (char *)0x0) || (*pcVar9 != '\x04')) {
      bVar2 = false;
      bVar1 = true;
      local_1a8 = (undefined1 *)0x8000000000000000;
      bVar3 = true;
      uVar18 = 1;
      local_190 = -0x8000000000000000;
      goto LAB_1003b5c88;
    }
    <alloc::collections::btree::set::BTreeSet<T>as_core::iter::traits::collect::FromIterator<T>>::from_iter
              (&local_128,*(long *)(pcVar9 + 0x10),*(long *)(pcVar9 + 0x10) + *(long *)(pcVar9 + 0x18) * 0x20);
    bVar2 = false;
    bVar1 = true;
    local_1a8 = (undefined1 *)0x8000000000000000;
    bVar3 = true;
    uVar18 = 1;
    local_190 = -0x8000000000000000;
  }
  else {
    bVar3 = false;
    local_190 = -0x8000000000000000;
LAB_1003b5c6c:
    local_1a8 = (undefined1 *)0x8000000000000000;
LAB_1003b5c7c:
    local_150 = 6;
    bVar1 = false;
    uVar18 = 0;
    bVar2 = true;
LAB_1003b5c88:
    local_128 = (undefined1 *)0x0;
    local_118 = (undefined **)0x0;
  }
  puVar6 = local_128;
  collect_project_state_from_active_db(&local_110,param_2);
  if (bVar2) {
    puVar22 = (undefined8 *)0x0;
    lVar15 = 0;
    lVar17 = 0;
    lVar21 = 0;
LAB_1003b5f20:
    lVar13 = 0;
  }
  else {
    pcVar9 = (char *)<str_as_serde_json::value::index::Index>::index_into(&DAT_101311312,0x1e,&local_150);
    if ((pcVar9 == (char *)0x0) || (*pcVar9 != '\x04')) {
      lVar17 = 0;
      local_170 = (undefined1 **)0x0;
      lVar15 = 8;
    }
    else {
      <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
                (&local_170,*(long *)(pcVar9 + 0x10),*(long *)(pcVar9 + 0x10) + *(long *)(pcVar9 + 0x18) * 0x20);
      lVar15 = CONCAT17(uStack_161,uStack_168);
      lVar17 = CONCAT17(uStack_159,local_160);
      if (lVar17 != 0) {
        puVar22 = (undefined8 *)(lVar15 + 8);
        lVar21 = lVar17;
        do {
          if (puVar22[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar22,puVar22[-1],1);
          }
          puVar22 = puVar22 + 3;
          lVar21 = lVar21 + -1;
        } while (lVar21 != 0);
      }
    }
    if (local_170 != (undefined1 **)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar15,(long)local_170 * 0x18,8);
    }
    pcVar9 = (char *)<str_as_serde_json::value::index::Index>::index_into
                               ("project-orderactive-workspace-rootsthread-workspace-root-hintsreasoning_contentreasoningcontentsummaryreasoning_detailspartsskill-backupsreplaceInvalid skill after importInvalid skill sourceremoveExternal Skills are read-onlymetadata.jsonskillInvalid skill backup id"
                                ,0xd,&local_150);
    if ((pcVar9 == (char *)0x0) || (*pcVar9 != '\x04')) {
      lVar15 = 0;
      local_170 = (undefined1 **)0x0;
    }
    else {
      <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
                (&local_170,*(long *)(pcVar9 + 0x10),*(long *)(pcVar9 + 0x10) + *(long *)(pcVar9 + 0x18) * 0x20);
      lVar21 = CONCAT17(uStack_161,uStack_168);
      lVar15 = CONCAT17(uStack_159,local_160);
      if (lVar15 == 0) {
        lVar15 = 0;
      }
      else {
        puVar22 = (undefined8 *)(lVar21 + 8);
        lVar13 = lVar15;
        do {
          if (puVar22[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar22,puVar22[-1],1);
          }
          puVar22 = puVar22 + 3;
          lVar13 = lVar13 + -1;
        } while (lVar13 != 0);
      }
      if (local_170 != (undefined1 **)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar21,(long)local_170 * 0x18,8);
      }
    }
    pcVar9 = (char *)<str_as_serde_json::value::index::Index>::index_into
                               ("active-workspace-rootsthread-workspace-root-hintsreasoning_contentreasoningcontentsummaryreasoning_detailspartsskill-backupsreplaceInvalid skill after importInvalid skill sourceremoveExternal Skills are read-onlymetadata.jsonskillInvalid skill backup id"
                                ,0x16,&local_150);
    if ((pcVar9 == (char *)0x0) || (*pcVar9 != '\x04')) {
      lVar21 = 0;
      local_170 = (undefined1 **)0x0;
    }
    else {
      <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
                (&local_170,*(long *)(pcVar9 + 0x10),*(long *)(pcVar9 + 0x10) + *(long *)(pcVar9 + 0x18) * 0x20);
      lVar13 = CONCAT17(uStack_161,uStack_168);
      lVar21 = CONCAT17(uStack_159,local_160);
      if (lVar21 == 0) {
        lVar21 = 0;
      }
      else {
        puVar22 = (undefined8 *)(lVar13 + 8);
        lVar20 = lVar21;
        do {
          if (puVar22[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar22,puVar22[-1],1);
          }
          puVar22 = puVar22 + 3;
          lVar20 = lVar20 + -1;
        } while (lVar20 != 0);
      }
      if (local_170 != (undefined1 **)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar13,(long)local_170 * 0x18,8);
      }
    }
    pcVar9 = (char *)<str_as_serde_json::value::index::Index>::index_into(&DAT_10131129c,0x16,&local_150);
    if ((pcVar9 == (char *)0x0) || (*pcVar9 != '\x04')) {
      puVar22 = (undefined8 *)0x0;
    }
    else {
      puVar22 = *(undefined8 **)(pcVar9 + 0x18);
    }
    pcVar9 = (char *)<str_as_serde_json::value::index::Index>::index_into
                               ("thread-workspace-root-hintsreasoning_contentreasoningcontentsummaryreasoning_detailspartsskill-backupsreplaceInvalid skill after importInvalid skill sourceremoveExternal Skills are read-onlymetadata.jsonskillInvalid skill backup id"
                                ,0x1b,&local_150);
    if ((pcVar9 == (char *)0x0) || (*pcVar9 != '\x05')) goto LAB_1003b5f20;
    lVar13 = *(long *)(pcVar9 + 0x18);
  }
  ppuVar11 = (undefined1 **)0x8000000000000000;
  puVar10 = puVar22;
  if (local_190 != -0x8000000000000000) {
    uVar19 = 0;
    pppuVar12 = &local_170;
    goto LAB_1003b6354;
  }
  if (local_e8 == (undefined **)0x0) {
    if (bVar3) {
      if (bVar1) goto LAB_1003b6100;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar23 = 0x25;
      puVar10 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x25,1);
      if (puVar10 == (undefined8 *)0x0) goto LAB_1003b6568;
      puVar10[1] = 0x74732d6c61626f6c;
      *puVar10 = 0x672d7865646f632e;
      puVar10[3] = 0xe895b3e6a097e620;
      puVar10[2] = 0x6e6f736a2e657461;
      *(undefined8 *)((long)puVar10 + 0x1d) = 0x909ee6a3a7e895b3;
      ppuVar11 = (undefined1 **)0x25;
    }
    else {
      if (((local_100 == (undefined1 *)0x0) && (local_c8 == 0)) && (lStack_c0 == 0)) goto LAB_1003b5f54;
LAB_1003b6100:
      if ((lVar17 == 0) && (local_100 != (undefined1 *)0x0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar23 = 0x37;
        puVar10 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x37,1);
        if (puVar10 == (undefined8 *)0x0) {
LAB_1003b6568:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,uVar23);
        }
        puVar10[1] = 0xe5a585e5ae9be7b9;
        *puVar10 = 0xa1e9207865646f43;
        puVar10[3] = 0xbae786bde48cbcef;
        puVar10[2] = 0xbaa9e7bab8e4a38f;
        puVar10[5] = 0xb9a1e9a89ce598ad;
        puVar10[4] = 0xe593bae58ba8e7bf;
        *(undefined8 *)((long)puVar10 + 0x2f) = 0x64776320ae9be7b9;
        ppuVar11 = (undefined1 **)0x37;
      }
      else if (lStack_c0 == 0) {
        uVar19 = 0;
        ppuVar11 = (undefined1 **)0x8000000000000000;
        pppuVar12 = &local_170;
        if ((lVar13 != 0) || (local_c8 == 0)) goto LAB_1003b6354;
        if ((lVar17 == 0) && (local_100 == (undefined1 *)0x0)) goto LAB_1003b5f54;
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar23 = 0x24;
        puVar10 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x24,1);
        if (puVar10 == (undefined8 *)0x0) goto LAB_1003b6568;
        *(undefined4 *)(puVar10 + 4) = 0xbaa9e7ba;
        puVar10[1] = 0xe9b088e58ba8e7bf;
        *puVar10 = 0xbae7207865646f43;
        puVar10[3] = 0xb8e484b0e5a098e6;
        puVar10[2] = 0x849ae7ae9be7b9a1;
        ppuVar11 = (undefined1 **)0x24;
      }
      else {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar23 = 0x43;
        puVar10 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x43,1);
        if (puVar10 == (undefined8 *)0x0) goto LAB_1003b6568;
        *(undefined4 *)((long)puVar10 + 0x3f) = 0x84bee5af;
        puVar10[1] = 0xe9b1a4e5babce793;
        *puVar10 = 0xbae58ba8e7bfbae7;
        puVar10[3] = 0x2086bde48cbcef64;
        puVar10[2] = 0x776320ae9be7b9a1;
        puVar10[5] = 0xa1e59e9be5af8fe5;
        puVar10[4] = 0x2074756f6c6c6f72;
        puVar10[7] = 0xafb7e8ae9be7b9a1;
        puVar10[6] = 0xe99eaee59f9ce7ab;
        ppuVar11 = (undefined1 **)0x43;
      }
    }
    pppuVar12 = &local_130;
    uVar19 = 1;
    local_170 = ppuVar11;
  }
  else {
LAB_1003b5f54:
    uVar19 = 0;
    ppuVar11 = (undefined1 **)0x8000000000000000;
    pppuVar12 = &local_170;
  }
LAB_1003b6354:
  *pppuVar12 = ppuVar11;
  ppuVar5 = local_130;
  ppuVar11 = local_170;
  active_db_path(&local_170,param_2);
  ppuVar7 = local_108;
  param_1[4] = ppuStack_f0;
  param_1[3] = local_f8;
  param_1[0x10] = puVar10;
  param_1[0x11] = ppuVar5;
  param_1[1] = uStack_180;
  *param_1 = local_188;
  param_1[2] = local_178;
  *(bool *)(param_1 + 0x1b) = bVar3;
  *(undefined1 *)((long)param_1 + 0xd9) = uVar18;
  param_1[5] = local_e8;
  param_1[6] = local_1a8;
  param_1[7] = local_1e8;
  param_1[8] = local_1f0;
  param_1[9] = local_190;
  param_1[10] = local_1d8;
  param_1[0xb] = local_1e0;
  param_1[0x12] = lVar17;
  param_1[0x13] = lVar15;
  param_1[0x14] = lVar21;
  param_1[0x15] = puVar22;
  param_1[0xe] = CONCAT17(uStack_159,local_160);
  param_1[0xf] = ppuVar11;
  param_1[0xd] = CONCAT17(uStack_161,uStack_168);
  param_1[0xc] = local_170;
  param_1[0x16] = lVar13;
  param_1[0x17] = puStack_e0;
  param_1[0x18] = local_100;
  param_1[0x1a] = lStack_c0;
  param_1[0x19] = local_c8;
  *(undefined1 *)((long)param_1 + 0xda) = uVar19;
  if (local_100 != (undefined1 *)0x0) {
    ppuVar14 = local_108 + 1;
    puVar16 = local_100;
    do {
      if (ppuVar14[-1] != (undefined *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppuVar14,ppuVar14[-1],1);
      }
      ppuVar14 = ppuVar14 + 3;
      puVar16 = puVar16 + -1;
    } while (puVar16 != (undefined1 *)0x0);
  }
  if (local_110 != (undefined1 **)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppuVar7,(long)local_110 * 0x18,8);
  }
  <alloc::collections::btree::map::BTreeMap<K,V,A>as_core::ops::drop::Drop>::drop(&local_d8);
  if (puVar6 == (undefined1 *)0x0) {
    local_d0 = (undefined **)0x0;
  }
  else {
    local_108 = (undefined **)0x0;
    local_f8 = local_120;
    local_e8 = (undefined **)0x0;
    local_d8 = local_120;
    local_d0 = local_118;
    local_100 = puVar6;
    puStack_e0 = puVar6;
  }
  local_110 = (undefined1 **)(ulong)(puVar6 != (undefined1 *)0x0);
  ppuStack_f0 = local_110;
  alloc::collections::btree::map::IntoIter<K,V,A>::dying_next(&local_170,&local_110);
  while (local_170 != (undefined1 **)0x0) {
    if (local_170[CONCAT17(uStack_159,local_160) * 3 + 1] != (undefined1 *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_170[CONCAT17(uStack_159,local_160) * 3 + 2],local_170[CONCAT17(uStack_159,local_160) * 3 + 1],1);
    }
    alloc::collections::btree::map::IntoIter<K,V,A>::dying_next(&local_170,&local_110);
  }
  if (!bVar2) {
    ::core::ptr::drop_in_place<serde_json::value::Value>(&local_150);
  }
  return;
}
