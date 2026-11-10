/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* deserialize_struct @ 10026c184 requested 10026c184 */

/* <&mut serde_json::de::Deserializer<R> as serde_core::de::Deserializer>::deserialize_struct */

void <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_struct(undefined8 *param_1,char *param_2)

{
  char cVar1;
  byte bVar2;
  bool bVar3;
  char *pcVar4;
  undefined8 uVar5;
  int *piVar6;
  int *piVar7;
  ulong uVar8;
  undefined4 unaff_w19;
  int *unaff_x21;
  char *pcVar9;
  long unaff_x25;
  long lVar10;
  long lVar11;
  undefined4 local_cc;
  undefined8 local_c8;
  int *local_c0;
  long local_b8;
  undefined4 local_b0;
  undefined4 uStack_ac;
  char *local_a8;
  undefined1 local_a0;
  char *local_98;
  undefined8 local_80;
  int *local_78;
  long local_70;
  undefined1 uStack_61;

  uVar8 = *(ulong *)(param_2 + 0x28);
  if (uVar8 < *(ulong *)(param_2 + 0x20)) {
    do {
      bVar2 = *(byte *)(*(long *)(param_2 + 0x18) + uVar8);
      if (0x20 < bVar2 || (1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) {
        if (bVar2 == 0x5b) {
          cVar1 = param_2[0x38];
          param_2[0x38] = cVar1 + -1;
          if ((char)(cVar1 + -1) != '\0') {
            *(ulong *)(param_2 + 0x28) = uVar8 + 1;
            local_78 = (int *)CONCAT71(local_78._1_7_,1);
            local_80 = param_2;
            <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_c8,&local_80);
            if ((char)local_c8 == '\x01') {
LAB_10026c47c:
              unaff_x21 = local_c0;
              pcVar9 = &DAT_8000000000000000;
              lVar11 = unaff_x25;
            }
            else {
              if (local_c8._1_1_ != '\x01') {
                uVar5 = 0;
LAB_10026c7bc:
                local_c0 = (int *)serde_core::de::Error::invalid_length(uVar5,&PTR_s_struct_ModelRestoreJournal_with_2_10130772a_0x461_101817c78,&DAT_101817930)
                ;
                goto LAB_10026c47c;
              }
              deserialize_u32(&local_c8,local_80);
              if ((int)local_c8 == 1) goto LAB_10026c47c;
              unaff_w19 = local_c8._4_4_;
              <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_c8,&local_80);
              if ((char)local_c8 == '\x01') goto LAB_10026c47c;
              if (local_c8._1_1_ != '\x01') {
                uVar5 = 1;
                goto LAB_10026c7bc;
              }
              deserialize_seq(&local_c8,local_80);
              unaff_x21 = local_c0;
              pcVar9 = local_c8;
              lVar11 = local_b8;
              if (local_c8 == &DAT_8000000000000000) goto LAB_10026c47c;
            }
            param_2[0x38] = param_2[0x38] + '\x01';
            local_c8 = pcVar9;
            local_c0 = unaff_x21;
            local_b8 = lVar11;
            local_b0 = unaff_w19;
            piVar6 = (int *)serde_json::de::Deserializer<R>::end_seq(param_2);
            if (pcVar9 != &DAT_8000000000000000) {
              if (piVar6 == (int *)0x0) goto LAB_10026c714;
              if (lVar11 != 0) {
                piVar7 = unaff_x21 + 8;
                do {
                  if (*(long *)(piVar7 + -8) != 0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)(piVar7 + -6),*(long *)(piVar7 + -8),1);
                  }
                  if (*(long *)(piVar7 + -2) != 0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)piVar7,*(long *)(piVar7 + -2),1);
                  }
                  if (*(long *)(piVar7 + 4) != 0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)(piVar7 + 6),*(long *)(piVar7 + 4),1);
                  }
                  piVar7 = piVar7 + 0x14;
                  lVar11 = lVar11 + -1;
                } while (lVar11 != 0);
              }
              goto LAB_10026c778;
            }
            piVar7 = unaff_x21;
            if (piVar6 == (int *)0x0) goto LAB_10026c790;
            core::ptr::drop_in_place<serde_json::error::ErrorCode>(piVar6);
            goto LAB_10026c75c;
          }
        }
        else {
          if (bVar2 != 0x7b) {
            piVar7 = (int *)serde_json::de::Deserializer<R>::peek_invalid_type(param_2,&uStack_61,&DAT_101818a70);
            goto LAB_10026c790;
          }
          cVar1 = param_2[0x38];
          param_2[0x38] = cVar1 + -1;
          if ((char)(cVar1 + -1) != '\0') {
            bVar3 = false;
            *(ulong *)(param_2 + 0x28) = uVar8 + 1;
            local_a0 = 1;
            pcVar9 = &DAT_8000000000000000;
            local_a8 = param_2;
            goto LAB_10026c25c;
          }
        }
        local_c8 = (char *)0x18;
        goto LAB_10026c1f4;
      }
      uVar8 = uVar8 + 1;
      *(ulong *)(param_2 + 0x28) = uVar8;
    } while (*(ulong *)(param_2 + 0x20) != uVar8);
  }
  local_c8 = (char *)0x5;
LAB_10026c1f4:
  uVar5 = serde_json::de::Deserializer<R>::peek_error(param_2,&local_c8);
  goto LAB_10026c79c;
LAB_10026c25c:
  lVar11 = unaff_x25;
  <serde_json::de::MapAccess<R>as_serde_core::de::MapAccess>::next_key_seed::has_next_key(&local_c8,&local_a8);
  pcVar4 = local_a8;
  local_98 = pcVar9;
  piVar6 = local_c0;
  if ((char)local_c8 != '\x01') {
    if (local_c8._1_1_ != '\x01') {
      if (!bVar3) {
        local_80 = "versionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey";
        local_78 = (int *)&DAT_00000007;
        local_c8 = (char *)&local_80;
        local_c0 = (int *)&<&T_as_core::fmt::Display>::fmt;
        piVar6 = (int *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_c8);
        goto joined_r0x00010026c5c8;
      }
      if (pcVar9 != &DAT_8000000000000000) goto LAB_10026c6dc;
      local_80 =
      "entriesstruct NoAccountModelRestoreJournal with 2 elementsfallback_modelstruct ModelRestoreEntry with 4 elementsstruct ModelRestoreJournal with 2 elementsinternal error: entered unreachable code: invalid Once stateCompression method not supportedZipFileReader was in an invalid stateAES encrypted files cannot be decrypted without the aes-crypto feature.AES encryption without AES extra data fieldArchive header is too largeOversized central directorySupport for multi-disk files is not implementedPassword required to decrypt fileInvalid zip64 locator digital signature headerInvalid digital signature headerInvalid local file headerInvalid Central Directory headerInvalid EOCD64: inconsistent lengthInvalid EOCD64: inconsistency with Locator dataInvalid EOCD64: inconsistent number of filesCould not find EOCD64Multi-disk ZIP files are not supportedInvalid EOCD64 Locator CD offsetNo CDFH foundInvalid CDFH offset in EOCDInvalid EOCD comment lengthCould not find EOCDEOCD64 Locator does not fit in fileEOCD comment exceeds file boundaryEOCD comment length exceeds u16::MAXEOCD64 extends beyond EOCD64 locatorLow EOCD64 record sizeaxum_core::extract::default_body_limit::DefaultBodyLimitKinda Display implementation returned an error unexpectedlydescription() is deprecated; use Displaymid > lencalled `Result::unwrap()` on an `Err` valueErrorattempt to join into collection with len > usize::MAX"
      ;
      local_78 = (int *)&DAT_00000007;
      local_c8 = (char *)&local_80;
      local_c0 = (int *)&<&T_as_core::fmt::Display>::fmt;
      unaff_x21 = (int *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_c8);
      goto LAB_10026c844;
    }
    *(long *)(local_a8 + 0x28) = *(long *)(local_a8 + 0x28) + 1;
    local_a8[0x10] = '\0';
    local_a8[0x11] = '\0';
    local_a8[0x12] = '\0';
    local_a8[0x13] = '\0';
    local_a8[0x14] = '\0';
    local_a8[0x15] = '\0';
    local_a8[0x16] = '\0';
    local_a8[0x17] = '\0';
    <serde_json::read::SliceRead_as_serde_json::read::Read>::parse_str(&local_c8,local_a8 + 0x18,local_a8);
    piVar6 = local_c0;
    if (local_c8 == (char *)0x2) goto joined_r0x00010026c5c8;
    if (local_b8 != 7) {
LAB_10026c874:
      piVar6 = (int *)serde_core::de::Error::unknown_field(local_c0,local_b8,&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_666,2);
      goto joined_r0x00010026c5c8;
    }
    if (*local_c0 != 0x73726576 || *(int *)((long)local_c0 + 3) != 0x6e6f6973) {
      if (*local_c0 != 0x72746e65 || *(int *)((long)local_c0 + 3) != 0x73656972) goto LAB_10026c874;
      if (pcVar9 != &DAT_8000000000000000) {
        local_80 =
        "entriesstruct NoAccountModelRestoreJournal with 2 elementsfallback_modelstruct ModelRestoreEntry with 4 elementsstruct ModelRestoreJournal with 2 elementsinternal error: entered unreachable code: invalid Once stateCompression method not supportedZipFileReader was in an invalid stateAES encrypted files cannot be decrypted without the aes-crypto feature.AES encryption without AES extra data fieldArchive header is too largeOversized central directorySupport for multi-disk files is not implementedPassword required to decrypt fileInvalid zip64 locator digital signature headerInvalid digital signature headerInvalid local file headerInvalid Central Directory headerInvalid EOCD64: inconsistent lengthInvalid EOCD64: inconsistency with Locator dataInvalid EOCD64: inconsistent number of filesCould not find EOCD64Multi-disk ZIP files are not supportedInvalid EOCD64 Locator CD offsetNo CDFH foundInvalid CDFH offset in EOCDInvalid EOCD comment lengthCould not find EOCDEOCD64 Locator does not fit in fileEOCD comment exceeds file boundaryEOCD comment length exceeds u16::MAXEOCD64 extends beyond EOCD64 locatorLow EOCD64 record sizeaxum_core::extract::default_body_limit::DefaultBodyLimitKinda Display implementation returned an error unexpectedlydescription() is deprecated; use Displaymid > lencalled `Result::unwrap()` on an `Err` valueErrorattempt to join into collection with len > usize::MAX"
        ;
        goto LAB_10026c8a0;
      }
      uVar8 = *(ulong *)(pcVar4 + 0x28);
      if (uVar8 < *(ulong *)(pcVar4 + 0x20)) {
        while( true ) {
          bVar2 = *(byte *)(*(long *)(pcVar4 + 0x18) + uVar8);
          if (0x3a < bVar2) goto LAB_10026c928;
          if ((1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) break;
          uVar8 = uVar8 + 1;
          *(ulong *)(pcVar4 + 0x28) = uVar8;
          if (*(ulong *)(pcVar4 + 0x20) == uVar8) goto LAB_10026c81c;
        }
        if ((ulong)bVar2 == 0x3a) goto code_r0x00010026c3c4;
LAB_10026c928:
        local_c8 = (char *)0x6;
        unaff_x21 = (int *)serde_json::de::Deserializer<R>::peek_error(pcVar4,&local_c8);
      }
      else {
LAB_10026c81c:
        local_c8 = (char *)0x3;
        unaff_x21 = (int *)serde_json::de::Deserializer<R>::peek_error(pcVar4,&local_c8);
      }
      goto LAB_10026c844;
    }
    if (bVar3) {
      local_80 = "versionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey";
LAB_10026c8a0:
      local_78 = (int *)&DAT_00000007;
      local_c8 = (char *)&local_80;
      local_c0 = (int *)&<&T_as_core::fmt::Display>::fmt;
      piVar6 = (int *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_15,&local_c8);
    }
    else {
      uVar8 = *(ulong *)(pcVar4 + 0x28);
      if (uVar8 < *(ulong *)(pcVar4 + 0x20)) {
        while( true ) {
          bVar2 = *(byte *)(*(long *)(pcVar4 + 0x18) + uVar8);
          if (0x3a < bVar2) goto LAB_10026c8d8;
          if ((1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) break;
          uVar8 = uVar8 + 1;
          *(ulong *)(pcVar4 + 0x28) = uVar8;
          if (*(ulong *)(pcVar4 + 0x20) == uVar8) goto LAB_10026c50c;
        }
        if ((ulong)bVar2 == 0x3a) {
          *(ulong *)(pcVar4 + 0x28) = uVar8 + 1;
          deserialize_u32(&local_80,pcVar4);
          piVar6 = local_78;
          if ((int)local_80 == 1) goto joined_r0x00010026c5c8;
          local_cc = local_80._4_4_;
          bVar3 = true;
          unaff_x25 = lVar11;
          goto LAB_10026c25c;
        }
LAB_10026c8d8:
        local_c8 = &SUB_00000006;
        piVar6 = (int *)serde_json::de::Deserializer<R>::peek_error(pcVar4,&local_c8);
      }
      else {
LAB_10026c50c:
        local_c8 = (char *)0x3;
        piVar6 = (int *)serde_json::de::Deserializer<R>::peek_error(pcVar4,&local_c8);
      }
    }
  }
joined_r0x00010026c5c8:
  if (pcVar9 != &DAT_8000000000000000) {
    if (lVar11 != 0) {
      piVar7 = unaff_x21 + 8;
      lVar10 = lVar11;
      do {
        if (*(long *)(piVar7 + -8) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)(piVar7 + -6),*(long *)(piVar7 + -8),1);
        }
        if (*(long *)(piVar7 + -2) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)piVar7,*(long *)(piVar7 + -2),1);
        }
        if (*(long *)(piVar7 + 4) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)(piVar7 + 6),*(long *)(piVar7 + 4),1);
        }
        piVar7 = piVar7 + 0x14;
        lVar10 = lVar10 + -1;
        lVar11 = 0;
      } while (lVar10 != 0);
    }
    if (pcVar9 != (char *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x21,(long)pcVar9 * 0x50,8);
    }
  }
  pcVar9 = &DAT_8000000000000000;
  unaff_x21 = piVar6;
  goto LAB_10026c6dc;
code_r0x00010026c3c4:
  *(ulong *)(pcVar4 + 0x28) = uVar8 + 1;
  deserialize_seq(&local_80,pcVar4);
  unaff_x21 = local_78;
  pcVar9 = local_80;
  unaff_x25 = local_70;
  if (local_80 == &DAT_8000000000000000) goto LAB_10026c844;
  goto LAB_10026c25c;
LAB_10026c844:
  pcVar9 = &DAT_8000000000000000;
LAB_10026c6dc:
  param_2[0x38] = param_2[0x38] + '\x01';
  local_b0 = local_cc;
  local_c8 = pcVar9;
  local_c0 = unaff_x21;
  local_b8 = lVar11;
  piVar6 = (int *)serde_json::de::Deserializer<R>::end_map(param_2);
  if (pcVar9 == &DAT_8000000000000000) {
    piVar7 = unaff_x21;
    if (piVar6 != (int *)0x0) {
      core::ptr::drop_in_place<serde_json::error::ErrorCode>(piVar6);
LAB_10026c75c:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(piVar6,0x28,8);
      piVar7 = unaff_x21;
    }
  }
  else {
    if (piVar6 == (int *)0x0) {
LAB_10026c714:
      param_1[2] = lVar11;
      param_1[3] = CONCAT44(uStack_ac,local_b0);
      *param_1 = pcVar9;
      param_1[1] = unaff_x21;
      return;
    }
    if (lVar11 != 0) {
      piVar7 = unaff_x21 + 8;
      do {
        if (*(long *)(piVar7 + -8) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)(piVar7 + -6),*(long *)(piVar7 + -8),1);
        }
        if (*(long *)(piVar7 + -2) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)piVar7,*(long *)(piVar7 + -2),1);
        }
        if (*(long *)(piVar7 + 4) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)(piVar7 + 6),*(long *)(piVar7 + 4),1);
        }
        piVar7 = piVar7 + 0x14;
        lVar11 = lVar11 + -1;
      } while (lVar11 != 0);
    }
LAB_10026c778:
    piVar7 = piVar6;
    if (pcVar9 != (char *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x21,(long)pcVar9 * 0x50,8);
    }
  }
LAB_10026c790:
  uVar5 = serde_json::error::Error::fix_position(piVar7,param_2);
LAB_10026c79c:
  *param_1 = 0x8000000000000000;
  param_1[1] = uVar5;
  return;
}
