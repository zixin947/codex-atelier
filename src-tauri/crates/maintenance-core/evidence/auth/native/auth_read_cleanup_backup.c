/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read_cleanup_backup @ 1008a9f94 requested 1008a9f94 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::router_unlock_auth::read_cleanup_backup */

void codexmate_lib::core::relay::router_unlock_auth::read_cleanup_backup
               (undefined8 *param_1,undefined8 *******param_2,undefined8 *******param_3,undefined8 ******param_4,undefined8 param_5)

{
  char cVar1;
  bool bVar2;
  undefined8 ******ppppppuVar3;
  long lVar4;
  undefined8 *******pppppppuVar5;
  undefined8 *******pppppppuVar6;
  undefined8 *******pppppppuVar7;
  code *pcVar8;
  bool bVar9;
  int iVar10;
  undefined8 *******pppppppuVar11;
  long *plVar12;
  undefined8 *******pppppppuVar13;
  char *pcVar14;
  undefined8 uVar15;
  long lVar16;
  ulong uVar17;
  char *unaff_x24;
  undefined8 *puVar18;
  undefined8 *******pppppppuVar19;
  undefined8 *******pppppppuVar20;
  ulong uVar21;
  undefined8 *******pppppppuVar22;
  undefined8 *******pppppppuVar23;
  undefined1 auVar24 [16];
  undefined8 *******local_288;
  undefined8 *******local_280;
  undefined8 ******local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  undefined8 *******local_240;
  undefined8 ******local_238;
  char *local_230;
  undefined8 local_228;
  undefined8 local_220;
  undefined8 *******pppppppuStack_218;
  undefined8 *******pppppppuStack_210;
  code *local_208;
  undefined8 local_200;
  undefined8 *******pppppppuStack_1f8;
  undefined8 *******local_1f0;
  code *local_1e8;
  char *local_1e0;
  undefined8 uStack_1d8;
  long local_1d0;
  char *local_1c8;
  undefined8 local_1c0;
  long lStack_1b8;
  char *local_1b0;
  undefined8 *******pppppppuStack_1a8;
  long local_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  long local_188;
  undefined8 local_180;
  undefined8 uStack_178;
  long local_170;
  undefined8 local_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 *******local_150;
  undefined8 *******pppppppuStack_148;
  undefined8 *******local_140;
  code *pcStack_138;
  undefined8 *******local_130;
  undefined8 *******pppppppuStack_128;
  undefined8 *******local_120;
  code *pcStack_118;
  char *local_110;
  undefined8 uStack_108;
  long lStack_100;
  char *local_f8;
  undefined8 local_f0;
  long lStack_e8;
  char *local_e0;
  undefined8 *******pppppppuStack_d8;
  long local_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  long local_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  long local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined1 auStack_71 [17];

  local_268 = param_4;
  uStack_260 = param_5;
  std::fs::read::inner(&local_150,param_2,param_3);
  pppppppuVar7 = local_140;
  pppppppuVar6 = pppppppuStack_148;
  pppppppuVar5 = local_150;
  if (local_150 == (undefined8 *******)0x8000000000000000) {
    local_220 = pppppppuStack_148;
    local_130 = &local_268;
    pppppppuStack_128 = (undefined8 *******)&<&T_as_core::fmt::Display>::fmt;
    local_120 = (undefined8 *******)&local_220;
    pcStack_118 = <std::io::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_200,s_read___router_unlock_auth_backup_10152f902,&local_130);
    if (((ulong)local_220 & 3) == 1) {
      pcVar14 = (char *)((long)local_220 + -1);
      uVar15 = *(undefined8 *)pcVar14;
      puVar18 = *(undefined8 **)((long)local_220 + 7);
      if ((code *)*puVar18 != (code *)0x0) {
        (*(code *)*puVar18)(uVar15);
      }
      if (puVar18[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar15,puVar18[1],puVar18[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar14,0x18,8);
    }
    param_1[2] = pppppppuStack_1f8;
    param_1[3] = local_1f0;
    uVar15 = 10;
    param_1[1] = local_200;
    goto LAB_1008aa114;
  }
  local_130 = pppppppuStack_148;
  pppppppuStack_128 = local_140;
  local_120 = (undefined8 *******)0x0;
  pcStack_118 = (code *)0x0;
  serde_json::de::from_trait(&local_200,&local_130);
  pppppppuVar19 = pppppppuStack_1f8;
  if ((char)local_200 == '\x06') {
    local_220 = pppppppuStack_1f8;
    local_150 = (undefined8 *******)0x0;
    pppppppuStack_148 = (undefined8 *******)0x1;
    local_140 = (undefined8 *******)0x0;
    local_120 = (undefined8 *******)&DAT_60000020;
    local_130 = &local_150;
    pppppppuStack_128 = (undefined8 *******)&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_1109;
    iVar10 = <serde_json::error::Error_as_core::fmt::Display>::fmt(&local_220,&local_130);
    pppppppuVar22 = local_140;
    pppppppuVar11 = pppppppuStack_148;
    pppppppuVar19 = local_150;
    if (iVar10 != 0) {
                    /* WARNING: Subroutine does not return */
      ::core::result::unwrap_failed
                ("a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` value/ErrorIoInvalidArchiveUnsupportedArchiveFileNotFoundInvalidPasswordassertion failed: self.is_char_boundary(new_len)assertion failed: vec.capacity() - start >= lenConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedTauriPathForbiddenGlobPatternInvalidPathUrlUnsafePathBufTryFromIntErrorInvalidUncertain"
                 ,0x37,auStack_71,&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_1445,&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_1429);
    }
    ::core::ptr::drop_in_place<serde_json::error::Error>(&local_220);
joined_r0x0001008aaad0:
    if (pppppppuVar19 == (undefined8 *******)0x8000000000000000) {
      param_1[2] = pppppppuVar6;
      param_1[3] = pppppppuVar7;
      uVar15 = 0xb;
      param_1[1] = pppppppuVar5;
LAB_1008aa114:
      *param_1 = uVar15;
      return;
    }
  }
  else {
    local_220._0_4_ = local_200._1_4_ << 8;
    local_220 = (undefined8 *******)CONCAT44(local_200._4_4_,(int)local_220);
    pppppppuStack_218 = pppppppuStack_1f8;
    pppppppuStack_210 = local_1f0;
    local_208 = local_1e8;
    local_220 = (undefined8 *******)CONCAT71(local_220._1_7_,(char)local_200);
    if ((char)local_200 != '\x05') {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar15 = 0x21;
      pppppppuVar11 = (undefined8 *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x21,1);
      if (pppppppuVar11 == (undefined8 *******)0x0) {
LAB_1008aab70:
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,uVar15);
      }
      builtin_strncpy((char *)pppppppuVar11,"auth backup root is not an object",0x21);
      pppppppuVar22 = (undefined8 *******)0x21;
      pppppppuVar19 = (undefined8 *******)0x21;
LAB_1008aa25c:
      ::core::ptr::drop_in_place<serde_json::value::Value>(&local_220);
      goto joined_r0x0001008aaad0;
    }
    if (pppppppuStack_1f8 != (undefined8 *******)0x0) {
      local_288 = local_1f0;
      local_280 = local_1f0;
      pppppppuVar11 = pppppppuStack_1f8;
      while( true ) {
        uVar17 = (ulong)*(ushort *)((long)pppppppuVar11 + 0x272);
        lVar16 = uVar17 * 0x18;
        uVar21 = 0xffffffffffffffff;
        pppppppuVar13 = pppppppuVar11 + 0x2d;
        pppppppuVar22 = pppppppuVar11 + -4;
        do {
          pppppppuVar23 = pppppppuVar22;
          if (lVar16 == 0) goto LAB_1008aa200;
          ppppppuVar3 = pppppppuVar13[2];
          lVar4 = 0x19 - (long)ppppppuVar3;
          if ((undefined8 ******)0x19 < ppppppuVar3 || lVar4 == 0) {
            ppppppuVar3 = (undefined8 ******)0x19;
          }
          iVar10 = _memcmp("aimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
                           ,pppppppuVar13[1],(size_t)ppppppuVar3);
          if (iVar10 != 0) {
            lVar4 = (long)iVar10;
          }
          cVar1 = 0 < lVar4;
          if (lVar4 < 0) {
            cVar1 = -1;
          }
          pppppppuVar22 = pppppppuVar23 + 4;
          uVar21 = uVar21 + 1;
          lVar16 = lVar16 + -0x18;
          pppppppuVar13 = pppppppuVar13 + 3;
        } while (cVar1 == '\x01');
        uVar17 = uVar21;
        if (cVar1 == '\0') break;
LAB_1008aa200:
        if (local_280 == (undefined8 *******)0x0) goto LAB_1008aa2c8;
        pppppppuVar11 = (undefined8 *******)pppppppuVar11[uVar17 + 0x4f];
        local_280 = (undefined8 *******)((long)local_280 + -1);
      }
      if ((*(char *)pppppppuVar22 == '\x01') && ((*(byte *)((long)pppppppuVar23 + 0x21) & 1) != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar15 = 0x29;
        pppppppuVar11 = (undefined8 *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x29,1);
        if (pppppppuVar11 == (undefined8 *******)0x0) goto LAB_1008aab70;
        builtin_strncpy((char *)pppppppuVar11,"backup contains AiMaMi router unlock auth",0x29);
        pppppppuVar22 = (undefined8 *******)0x29;
        pppppppuVar19 = (undefined8 *******)0x29;
        goto LAB_1008aa25c;
      }
LAB_1008aa2c8:
      pppppppuVar22 = (undefined8 *******)0xe;
      pppppppuVar11 = (undefined8 *******)0x1013b4d72;
      while( true ) {
        uVar17 = (ulong)*(ushort *)((long)pppppppuVar19 + 0x272);
        lVar16 = uVar17 * 0x18;
        uVar21 = 0xffffffffffffffff;
        pppppppuVar13 = pppppppuVar19 + 0x2d;
        pppppppuVar23 = pppppppuVar19 + -2;
        do {
          pppppppuVar20 = pppppppuVar23;
          if (lVar16 == 0) goto LAB_1008aa354;
          ppppppuVar3 = pppppppuVar13[2];
          lVar4 = 0xe - (long)ppppppuVar3;
          if ((undefined8 ******)0xe < ppppppuVar3 || lVar4 == 0) {
            ppppppuVar3 = (undefined8 ******)0xe;
          }
          iVar10 = _memcmp("OPENAI_API_KEY",pppppppuVar13[1],(size_t)ppppppuVar3);
          if (iVar10 != 0) {
            lVar4 = (long)iVar10;
          }
          cVar1 = 0 < lVar4;
          if (lVar4 < 0) {
            cVar1 = -1;
          }
          uVar21 = uVar21 + 1;
          lVar16 = lVar16 + -0x18;
          pppppppuVar13 = pppppppuVar13 + 3;
          pppppppuVar23 = pppppppuVar20 + 4;
        } while (cVar1 == '\x01');
        uVar17 = uVar21;
        if (cVar1 == '\0') break;
LAB_1008aa354:
        unaff_x24 = "OPENAI_API_KEY";
        if (local_288 == (undefined8 *******)0x0) goto LAB_1008aa3ec;
        pppppppuVar19 = (undefined8 *******)pppppppuVar19[uVar17 + 0x4f];
        local_288 = (undefined8 *******)((long)local_288 + -1);
      }
      unaff_x24 = (char *)pppppppuVar11;
      if (*(char *)(pppppppuVar20 + 2) == '\x03') {
        auVar24 = ::core::str::<impl_str>::trim_matches(pppppppuVar20[4],pppppppuVar20[5]);
        plVar12 = auVar24._0_8_;
        if ((plVar12 != (long *)0x0) && (unaff_x24 = "OPENAI_API_KEY", auVar24._8_8_ != 0)) {
          if ((auVar24._8_8_ < 0x17) ||
             ((*plVar12 != 0x6d616d69612d6b73 || plVar12[1] != 0x2d79616c65722d69) || *(long *)((long)plVar12 + 0xf) != 0x6c6175747269762d)) {
            pppppppuVar19 = (undefined8 *******)0x8000000000000000;
          }
          else {
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            uVar15 = 0x27;
            pppppppuVar11 = (undefined8 *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x27,1);
            if (pppppppuVar11 == (undefined8 *******)0x0) goto LAB_1008aab70;
            builtin_strncpy((char *)pppppppuVar11,"backup contains AiMaMi virtual API auth",0x27);
            pppppppuVar22 = (undefined8 *******)0x27;
            pppppppuVar19 = (undefined8 *******)0x27;
          }
          goto LAB_1008aa25c;
        }
      }
    }
LAB_1008aa3ec:
    pppppppuStack_148 = pppppppuStack_218;
    local_150 = local_220;
    pcStack_138 = local_208;
    local_140 = pppppppuStack_210;
    serde_json::value::Value>::deserialize_struct
              (&local_130,&local_150,"AuthFile~/.codexccswitchdeecodexarchivedSKILL.mdmarkdown",8,&PTR_s_auth_modeOPENAI_API_KEY_10182dad8,5);
    pcVar8 = pcStack_118;
    pppppppuVar13 = pppppppuStack_128;
    pppppppuVar22 = local_130;
    if (local_130 == (undefined8 *******)&DAT_8000000000000001) {
      local_240 = pppppppuStack_128;
      local_238 = (undefined8 ******)0x0;
      local_230 = (char *)0x1;
      local_228 = 0;
      local_140 = (undefined8 *******)&DAT_60000020;
      local_150 = &local_238;
      pppppppuStack_148 = (undefined8 *******)&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_1109;
      iVar10 = <serde_json::error::Error_as_core::fmt::Display>::fmt(&local_240,&local_150);
      pppppppuVar22 = (undefined8 *******)local_228;
      pppppppuVar11 = (undefined8 *******)local_230;
      pppppppuVar19 = (undefined8 *******)local_238;
      if (iVar10 != 0) {
                    /* WARNING: Subroutine does not return */
        ::core::result::unwrap_failed
                  ("a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` value/ErrorIoInvalidArchiveUnsupportedArchiveFileNotFoundInvalidPasswordassertion failed: self.is_char_boundary(new_len)assertion failed: vec.capacity() - start >= lenConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressOtherUncategorizedTauriPathForbiddenGlobPatternInvalidPathUrlUnsafePathBufTryFromIntErrorInvalidUncertain"
                   ,0x37,auStack_71,&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_1445,&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_1429);
      }
      ::core::ptr::drop_in_place<serde_json::error::Error>(&local_240);
      goto joined_r0x0001008aaad0;
    }
    uStack_178 = uStack_a8;
    local_180 = local_b0;
    local_168 = local_98;
    local_170 = local_a0;
    uStack_158 = uStack_88;
    local_160 = local_90;
    lStack_1b8 = lStack_e8;
    local_1c0 = local_f0;
    pppppppuStack_1a8 = pppppppuStack_d8;
    local_1b0 = local_e0;
    uStack_198 = uStack_c8;
    local_1a0 = local_d0;
    local_188 = local_b8;
    uStack_190 = uStack_c0;
    uStack_1d8 = uStack_108;
    local_1e0 = local_110;
    local_1c8 = local_f8;
    local_1d0 = lStack_100;
    local_1f0 = local_120;
    local_1e8 = pcStack_118;
    local_200 = local_130;
    pppppppuStack_1f8 = pppppppuStack_128;
    uVar21 = auth::is_aimami_managed_virtual_auth(&local_200);
    if ((uVar21 & 1) == 0) {
      uVar21 = auth::is_switchable_oauth_auth(&local_200);
      if ((uVar21 & 1) == 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar15 = 0x3d;
        unaff_x24 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x3d,1);
        if (unaff_x24 == (char *)0x0) goto LAB_1008aab84;
        builtin_strncpy(unaff_x24,"backup contains neither user OAuth auth nor user API-key auth",0x3d);
        pppppppuVar19 = (undefined8 *******)0x3d;
      }
      else {
        pppppppuVar19 = (undefined8 *******)0x8000000000000000;
      }
      if (((ulong)pppppppuVar22 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar13,pppppppuVar22,1);
      }
      if ((pcVar8 != (code *)0x8000000000000000) && (pcVar8 != (code *)0x0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1e0,pcVar8,1);
      }
      if ((local_1d0 != -0x8000000000000000) && (local_1d0 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1c8,local_1d0,1);
      }
      if ((lStack_1b8 != -0x8000000000000000) && (lStack_1b8 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1b0,lStack_1b8,1);
      }
      if ((local_1a0 != -0x8000000000000000) && (local_1a0 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_198,local_1a0,1);
      }
      if ((local_188 != -0x8000000000000000) && (local_188 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_180,local_188,1);
      }
      if ((local_170 != -0x8000000000000000) && (local_170 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_168,local_170,1);
      }
      pppppppuVar22 = (undefined8 *******)0x3d;
      pppppppuVar11 = (undefined8 *******)unaff_x24;
      goto joined_r0x0001008aaad0;
    }
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar15 = 0x2b;
    pppppppuVar11 = (undefined8 *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x2b,1);
    if (pppppppuVar11 == (undefined8 *******)0x0) {
LAB_1008aab84:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,uVar15);
    }
    builtin_strncpy((char *)pppppppuVar11,"backup contains AiMaMi managed virtual auth",0x2b);
    if (((ulong)pppppppuVar22 & 0x7fffffffffffffff) != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar13,pppppppuVar22,1);
    }
    if ((pcVar8 != (code *)0x8000000000000000) && (pcVar8 != (code *)0x0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1e0,pcVar8,1);
    }
    if ((local_1d0 != -0x8000000000000000) && (local_1d0 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1c8,local_1d0,1);
    }
    if ((lStack_1b8 != -0x8000000000000000) && (lStack_1b8 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1b0,lStack_1b8,1);
    }
    if ((local_1a0 != -0x8000000000000000) && (local_1a0 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_198,local_1a0,1);
    }
    if ((local_188 != -0x8000000000000000) && (local_188 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_180,local_188,1);
    }
    if ((local_170 != -0x8000000000000000) && (local_170 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_168,local_170,1);
    }
    pppppppuVar22 = (undefined8 *******)0x2b;
    pppppppuVar19 = (undefined8 *******)0x2b;
  }
  pppppppuStack_148 = pppppppuVar7;
  local_140 = (undefined8 *******)0x0;
  pcStack_138 = (code *)0x0;
  local_150 = pppppppuVar6;
  serde_json::de::from_trait(&local_130,&local_150);
  if ((char)local_130 == '\x06') {
    ::core::ptr::drop_in_place<serde_json::error::Error>(&pppppppuStack_128);
LAB_1008aa608:
    local_130 = &local_268;
    pppppppuStack_128 = (undefined8 *******)&<&T_as_core::fmt::Display>::fmt;
    local_120 = &local_150;
    pcStack_118 = <std::path::Display_as_core::fmt::Display>::fmt;
    local_110 = (char *)&local_200;
    uStack_108 = 0x1009113b0;
    local_200 = pppppppuVar19;
    pppppppuStack_1f8 = pppppppuVar11;
    local_1f0 = pppppppuVar22;
    local_150 = param_2;
    pppppppuStack_148 = param_3;
    alloc::fmt::format::format_inner(&local_258,s_I_router_unlock_auth_backup_is_n_10152f863,&local_130);
    param_1[2] = uStack_250;
    param_1[1] = local_258;
    param_1[3] = local_248;
    *param_1 = 10;
    goto joined_r0x0001008aa934;
  }
  pppppppuStack_218 = pppppppuStack_128;
  local_220 = local_130;
  local_208 = pcStack_118;
  pppppppuStack_210 = local_120;
  pcVar14 = (char *)<str_as_serde_json::value::index::Index>::index_into
                              ("aimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
                               ,0x19,&local_220);
  if (((pcVar14 == (char *)0x0) || (*pcVar14 != '\x01')) || ((pcVar14[1] & 1U) == 0)) {
    pcVar14 = (char *)<str_as_serde_json::value::index::Index>::index_into("OPENAI_API_KEY",0xe,&local_220);
    if ((pcVar14 == (char *)0x0) || (*pcVar14 != '\x03')) {
LAB_1008aa6f0:
      bVar9 = false;
    }
    else {
      auVar24 = ::core::str::<impl_str>::trim_matches(*(undefined8 *)(pcVar14 + 0x10),*(undefined8 *)(pcVar14 + 0x18));
      plVar12 = auVar24._0_8_;
      if (auVar24._8_8_ < 0x17) goto LAB_1008aa6f0;
      bVar9 = (*plVar12 == 0x6d616d69612d6b73 && plVar12[1] == 0x2d79616c65722d69) && *(long *)((long)plVar12 + 0xf) == 0x6c6175747269762d;
    }
    pcVar14 = (char *)<str_as_serde_json::value::index::Index>::index_into(&DAT_1013b375d,6,&local_220);
    if ((pcVar14 == (char *)0x0) || (*pcVar14 != '\x05')) {
      if (!bVar9) goto LAB_1008aa728;
      goto LAB_1008aa76c;
    }
    bVar2 = false;
    if (*(long *)(pcVar14 + 0x18) == 0) {
      bVar2 = bVar9;
    }
    if (bVar2) goto LAB_1008aa76c;
LAB_1008aa728:
    serde_json::de::from_trait(&local_200,&local_150);
    if (local_200 == (undefined8 *******)&DAT_8000000000000001) {
      pppppppuStack_128 = pppppppuStack_1f8;
      local_130 = (undefined8 *******)CONCAT71(local_130._1_7_,1);
      ::core::ptr::drop_in_place<serde_json::error::Error>(&pppppppuStack_128);
      uVar21 = 0;
    }
    else {
      uStack_a8 = uStack_178;
      local_b0 = local_180;
      local_98 = local_168;
      local_a0 = local_170;
      uStack_88 = uStack_158;
      local_90 = local_160;
      lStack_e8 = lStack_1b8;
      local_f0 = local_1c0;
      pppppppuStack_d8 = pppppppuStack_1a8;
      local_e0 = local_1b0;
      uStack_c8 = uStack_198;
      local_d0 = local_1a0;
      local_b8 = local_188;
      uStack_c0 = uStack_190;
      pppppppuStack_128 = pppppppuStack_1f8;
      local_130 = local_200;
      pcStack_118 = local_1e8;
      local_120 = local_1f0;
      uStack_108 = uStack_1d8;
      local_110 = local_1e0;
      local_f8 = local_1c8;
      lStack_100 = local_1d0;
      uVar21 = auth::is_aimami_managed_virtual_auth(&local_130);
      if ((local_130 != (undefined8 *******)&DAT_8000000000000000) && (local_130 != (undefined8 *******)0x0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuStack_128,local_130,1);
      }
      if ((pcStack_118 != (code *)0x8000000000000000) && (pcStack_118 != (code *)0x0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_110,pcStack_118,1);
      }
      if ((lStack_100 != -0x8000000000000000) && (lStack_100 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_f8,lStack_100,1);
      }
      if ((lStack_e8 != -0x8000000000000000) && (lStack_e8 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_e0,lStack_e8,1);
      }
      if ((local_d0 != -0x8000000000000000) && (local_d0 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_c8,local_d0,1);
      }
      if ((local_b8 != -0x8000000000000000) && (local_b8 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_b0,local_b8,1);
      }
      if ((local_a0 != -0x8000000000000000) && (local_a0 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_98,local_a0,1);
      }
    }
    ::core::ptr::drop_in_place<serde_json::value::Value>(&local_220);
    if ((uVar21 & 1) == 0) goto LAB_1008aa608;
  }
  else {
LAB_1008aa76c:
    ::core::ptr::drop_in_place<serde_json::value::Value>(&local_220);
  }
  local_200 = pppppppuVar19;
  pppppppuStack_1f8 = pppppppuVar11;
  local_1f0 = pppppppuVar22;
  if (1 < log::MAX_LOG_LEVEL_FILTER) {
    local_150 = (undefined8 *******)&local_200;
    pppppppuStack_148 = (undefined8 *******)((long)&DAT_1009113af + 1);
    local_f0 = 0x2e;
    pppppppuStack_d8 = &local_150;
    local_e0 = s_K_AiMaMi__router_unlock_auth__di_10152f8b4;
    local_130 = (undefined8 *******)0x0;
    pppppppuStack_128 = (undefined8 *******)0x1013b4b0a;
    pcStack_118 = (code *)0x0;
    local_120 = (undefined8 *******)&DAT_0000002e;
    local_f8 =
    "codexmate_lib::core::relay::router_unlock_authaimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
    ;
    local_110 = "src/core/relay/router_unlock_auth.rs";
    lStack_100 = 2;
    uStack_108 = 0x24;
    lStack_e8 = 0x14600000001;
    <log::__private_api::GlobalLogger_as_log::Log>::log(auStack_71,&local_130);
  }
  param_1[1] = 0x8000000000000000;
  *param_1 = 0xb;
joined_r0x0001008aa934:
  if (local_200 != (undefined8 *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuStack_1f8,local_200,1);
  }
  if (pppppppuVar5 != (undefined8 *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar6,pppppppuVar5,1);
  }
  return;
}
