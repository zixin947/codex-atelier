/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* deserialize_struct @ 100269290 requested 100269290 */

/* WARNING: Type propagation algorithm not settling */
/* <&mut serde_json::de::Deserializer<R> as serde_core::de::Deserializer>::deserialize_struct */

void <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_struct(undefined8 *param_1,long param_2)

{
  byte bVar1;
  char cVar2;
  bool bVar3;
  long lVar4;
  undefined8 uVar5;
  char *pcVar6;
  uint uVar7;
  uint extraout_w8;
  uint extraout_w8_00;
  uint extraout_w8_01;
  uint extraout_w8_02;
  uint extraout_w8_03;
  uint extraout_w8_04;
  uint extraout_w8_05;
  ulong uVar8;
  char *extraout_x10;
  char *extraout_x10_00;
  char *extraout_x10_01;
  char *extraout_x10_02;
  char *extraout_x10_03;
  char *extraout_x10_04;
  char *extraout_x10_05;
  char *extraout_x10_06;
  char *extraout_x10_07;
  char *extraout_x10_08;
  char *extraout_x10_09;
  char *extraout_x10_10;
  char *extraout_x10_11;
  char *extraout_x10_12;
  char *extraout_x11;
  char *extraout_x11_00;
  char *extraout_x11_01;
  char *extraout_x11_02;
  char *extraout_x11_03;
  char *extraout_x11_04;
  char *extraout_x11_05;
  char *extraout_x11_06;
  char *extraout_x11_07;
  char *extraout_x11_08;
  char *extraout_x11_09;
  char *extraout_x11_10;
  char *extraout_x11_11;
  char *extraout_x11_12;
  char *extraout_x11_13;
  char *extraout_x11_14;
  char *extraout_x11_15;
  char *extraout_x11_16;
  char *extraout_x11_17;
  char *extraout_x11_18;
  char *extraout_x11_19;
  char *extraout_x11_20;
  char *extraout_x11_21;
  char *extraout_x11_22;
  char *extraout_x11_23;
  char *extraout_x11_24;
  char *extraout_x11_25;
  char *extraout_x11_26;
  char *extraout_x11_27;
  char *pcVar9;
  char *extraout_x11_28;
  char *pcVar10;
  char *extraout_x11_29;
  char *extraout_x11_30;
  char *extraout_x11_31;
  char *extraout_x11_32;
  char *extraout_x11_33;
  char *extraout_x11_34;
  char *extraout_x11_35;
  char *extraout_x11_36;
  char *extraout_x13;
  char *extraout_x13_00;
  char *extraout_x13_01;
  char *extraout_x13_02;
  char *extraout_x13_03;
  char *extraout_x13_04;
  char *extraout_x13_05;
  char *extraout_x13_06;
  char *extraout_x13_07;
  char *extraout_x13_08;
  char *extraout_x13_09;
  char *pcVar11;
  char *extraout_x13_10;
  char *extraout_x13_11;
  char *extraout_x13_12;
  char *pcVar12;
  char *pcVar13;
  char *pcVar14;
  char *unaff_x24;
  char *pcVar15;
  char *pcVar16;
  char *unaff_x25;
  char *unaff_x27;
  char *unaff_x28;
  char *local_138;
  char *local_130;
  char *local_128;
  char *local_120;
  char *local_118;
  char *local_108;
  char *local_100;
  char *pcStack_f8;
  undefined8 local_f0;
  char *local_e8;
  char *local_e0;
  char *pcStack_d8;
  char *local_d0;
  char *pcStack_c8;
  char *local_c0;
  char *pcStack_b8;
  char *local_b0;
  uint uStack_a8;
  undefined4 uStack_a4;
  long local_a0;
  undefined1 local_98;
  undefined8 local_90;
  char *pcStack_88;
  char *local_80;
  undefined1 auStack_69 [9];

  uVar8 = *(ulong *)(param_2 + 0x28);
  if (uVar8 < *(ulong *)(param_2 + 0x20)) {
    local_118 = *(char **)(param_2 + 0x18);
    do {
      bVar1 = local_118[uVar8];
      if (0x20 < bVar1 || (1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) {
        if (bVar1 == 0x5b) {
          cVar2 = *(char *)(param_2 + 0x38) + -1;
          *(char *)(param_2 + 0x38) = cVar2;
          if (cVar2 != '\0') {
            *(ulong *)(param_2 + 0x28) = uVar8 + 1;
            pcStack_88 = (char *)CONCAT71(pcStack_88._1_7_,1);
            local_90 = (char *)param_2;
            <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_f0,&local_90);
            local_130 = extraout_x10;
            pcVar10 = extraout_x11_06;
            local_108 = extraout_x13;
            uVar7 = (uint)local_f0 & 0xff;
            if ((char)local_f0 == '\x01') {
LAB_100269680:
              local_f0._4_4_ = uVar7;
              pcVar14 = &DAT_8000000000000000;
              local_120 = local_e8;
              local_138 = unaff_x24;
              local_128 = unaff_x25;
              pcVar15 = unaff_x28;
            }
            else {
              if (local_f0._1_1_ != '\x01') {
                local_e8 = (char *)serde_core::de::Error::invalid_length(0,&PTR_s_struct_ModelRestoreEntry_with_4_e_10130772a_0x439_101817c68,&DAT_101817930);
                local_130 = extraout_x10_05;
                pcVar10 = extraout_x11_18;
                local_108 = extraout_x13_05;
                uVar7 = extraout_w8_01;
                goto LAB_100269680;
              }
              deserialize_string(&local_f0,local_90);
              pcVar13 = local_e0;
              local_120 = local_e8;
              pcVar14 = local_f0;
              local_130 = extraout_x10_00;
              pcVar10 = extraout_x11_07;
              local_108 = extraout_x13_00;
              uVar7 = 0;
              if (local_f0 == &DAT_8000000000000000) goto LAB_100269680;
              <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_f0,&local_90);
              pcVar6 = extraout_x10_01;
              pcVar10 = extraout_x11_09;
              pcVar11 = extraout_x13_01;
              pcVar15 = unaff_x24;
              uVar7 = (uint)local_f0 & 0xff;
              if ((char)local_f0 == '\x01') {
LAB_100269b28:
                local_f0._4_4_ = uVar7;
                local_138 = pcVar15;
                local_130 = pcVar6;
                local_108 = pcVar11;
                if (pcVar14 != (char *)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,pcVar14,1);
                  local_130 = extraout_x10_11;
                  pcVar10 = extraout_x11_30;
                  local_108 = extraout_x13_11;
                  local_f0._4_4_ = extraout_w8_05;
                }
                pcVar14 = &DAT_8000000000000000;
                local_120 = local_e8;
                local_128 = local_e8;
                pcVar15 = pcVar13;
              }
              else {
                if (local_f0._1_1_ != '\x01') {
                  local_e8 = (char *)serde_core::de::Error::invalid_length(1,&PTR_s_struct_ModelRestoreEntry_with_4_e_10130772a_0x439_101817c68,&DAT_101817930);
                  pcVar6 = extraout_x10_04;
                  pcVar10 = extraout_x11_16;
                  pcVar11 = extraout_x13_04;
                  pcVar15 = unaff_x24;
                  uVar7 = extraout_w8_00;
                  goto LAB_100269b28;
                }
                deserialize_string(&local_f0,local_90);
                local_138 = local_e0;
                local_128 = local_e8;
                local_108 = local_f0;
                pcVar6 = extraout_x10_02;
                pcVar10 = extraout_x11_10;
                pcVar11 = extraout_x13_02;
                pcVar15 = unaff_x24;
                uVar7 = 0;
                if (local_f0 == &DAT_8000000000000000) goto LAB_100269b28;
                <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_f0,&local_90);
                uVar7 = (uint)local_f0 & 0xff;
                pcVar6 = extraout_x10_03;
                pcVar10 = extraout_x11_11;
                pcVar11 = extraout_x13_03;
                if ((char)local_f0 == '\x01') {
LAB_100269b10:
                  pcVar15 = local_138;
                  if (local_108 != (char *)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,local_108,1);
                    pcVar6 = extraout_x10_10;
                    pcVar10 = extraout_x11_29;
                    pcVar11 = extraout_x13_10;
                    uVar7 = extraout_w8_04;
                  }
                  goto LAB_100269b28;
                }
                if (local_f0._1_1_ != '\x01') {
                  local_e8 = (char *)serde_core::de::Error::invalid_length(2,&PTR_s_struct_ModelRestoreEntry_with_4_e_10130772a_0x439_101817c68,&DAT_101817930);
                  pcVar6 = extraout_x10_09;
                  pcVar10 = extraout_x11_28;
                  pcVar11 = extraout_x13_09;
                  uVar7 = extraout_w8_03;
                  goto LAB_100269b10;
                }
                deserialize_string(&local_f0,local_90);
                local_130 = local_e8;
                pcVar15 = local_f0;
                uVar7 = 0;
                pcVar6 = extraout_x10_06;
                pcVar10 = extraout_x11_22;
                pcVar11 = extraout_x13_06;
                if (local_f0 == &DAT_8000000000000000) goto LAB_100269b10;
                <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_f0,&local_90);
                uVar7 = (uint)local_f0 & 0xff;
                pcVar6 = extraout_x10_07;
                pcVar10 = extraout_x11_23;
                pcVar11 = extraout_x13_07;
                if ((char)local_f0 == '\x01') {
LAB_100269a14:
                  unaff_x27 = local_e8;
                  if (pcVar15 != (char *)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,pcVar15,1);
                    pcVar6 = extraout_x10_08;
                    pcVar10 = extraout_x11_24;
                    pcVar11 = extraout_x13_08;
                    uVar7 = extraout_w8_02;
                  }
                  goto LAB_100269b10;
                }
                unaff_x27 = local_e0;
                if (local_f0._1_1_ == '\x01') {
                  deserialize_u32(&local_f0,local_90);
                  uVar7 = 1;
                  pcVar6 = extraout_x10_12;
                  pcVar10 = extraout_x11_31;
                  pcVar11 = extraout_x13_12;
                  if ((int)local_f0 == 1) goto LAB_100269a14;
                  pcVar10 = pcVar13;
                }
                else {
                  local_f0._4_4_ = 0;
                  pcVar10 = pcVar13;
                }
              }
            }
            *(char *)(param_2 + 0x38) = *(char *)(param_2 + 0x38) + '\x01';
            uStack_a8 = local_f0._4_4_;
            local_f0 = pcVar14;
            local_e8 = local_120;
            pcStack_d8 = local_108;
            local_d0 = local_128;
            pcStack_c8 = local_138;
            local_c0 = pcVar15;
            pcStack_b8 = local_130;
            local_b0 = unaff_x27;
            pcVar13 = (char *)serde_json::de::Deserializer<R>::end_seq(param_2);
            pcStack_f8 = (char *)CONCAT44(uStack_a4,uStack_a8);
            local_90 = local_b0;
            pcStack_88 = pcStack_f8;
            if (pcVar14 == &DAT_8000000000000000) {
              if (pcVar13 == (char *)0x0) goto LAB_100269d98;
              core::ptr::drop_in_place<serde_json::error::ErrorCode>(pcVar13);
              goto LAB_100269bb8;
            }
            if (pcVar13 == (char *)0x0) {
              local_100 = local_b0;
              if (pcVar14 == &DAT_8000000000000000) goto LAB_100269d98;
              goto LAB_100269c30;
            }
            if (pcVar14 != (char *)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,pcVar14,1);
            }
            goto joined_r0x000100269bf8;
          }
        }
        else {
          if (bVar1 != 0x7b) {
            local_120 = (char *)serde_json::de::Deserializer<R>::peek_invalid_type(param_2,auStack_69,&DAT_1018189f0);
            goto LAB_100269d98;
          }
          cVar2 = *(char *)(param_2 + 0x38) + -1;
          *(char *)(param_2 + 0x38) = cVar2;
          if (cVar2 != '\0') {
            bVar3 = false;
            *(ulong *)(param_2 + 0x28) = uVar8 + 1;
            local_98 = 1;
            local_108 = &DAT_8000000000000000;
            pcVar15 = &DAT_8000000000000000;
            local_a0 = param_2;
            pcVar13 = local_120;
            pcVar14 = &DAT_8000000000000000;
            goto LAB_100269374;
          }
        }
        local_f0 = (char *)0x18;
        goto LAB_100269300;
      }
      uVar8 = uVar8 + 1;
      *(ulong *)(param_2 + 0x28) = uVar8;
    } while (*(ulong *)(param_2 + 0x20) != uVar8);
  }
  local_f0 = (char *)0x5;
LAB_100269300:
  uVar5 = serde_json::de::Deserializer<R>::peek_error(param_2,&local_f0);
  goto LAB_100269310;
LAB_100269374:
  do {
    pcVar10 = unaff_x28;
    local_120 = pcVar13;
    <serde_json::de::MapAccess<R>as_serde_core::de::MapAccess>::next_key_seed::has_next_key(&local_f0,&local_a0);
    lVar4 = local_a0;
    pcVar6 = local_e8;
    pcVar9 = extraout_x11;
    pcVar16 = pcVar15;
    pcVar11 = local_108;
    if ((char)local_f0 == '\x01') goto LAB_10026978c;
    if (local_f0._1_1_ != '\x01') {
      if (pcVar14 == &DAT_8000000000000000) {
        local_90 =
        "thread_idturn_idcwdthread_sourcesourceforked_from_idforked_from_ordinal_exclusivesubagent_history_start_ordinalhistory_baseend_ordinal_exclusiveend_byte_offsettspriorityrollout_idis_subagentincompleteincomplete_reasonsweb_search_coverage_versioncurrent_turnuncertain_inheritanceprevious_totalneeds_legacy_baselinepending_callsrecent_completed_callsturn_startssynthetic_turnsession_cwdturn_cwdsweb_search_coveragestarted_atcompleted_atfailedabortedduration_ms"
        ;
        pcStack_88 = &DAT_00000009;
        local_f0 = (char *)&local_90;
        local_e8 = (char *)&<&T_as_core::fmt::Display>::fmt;
        pcVar14 = &DAT_8000000000000000;
        pcVar6 = (char *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_f0);
        pcVar9 = extraout_x11_12;
LAB_100269778:
        pcVar12 = (char *)0x0;
        pcVar13 = (char *)0x1;
        goto LAB_100269798;
      }
      pcVar13 = (char *)(ulong)(local_108 == &DAT_8000000000000000);
      if (local_108 == &DAT_8000000000000000) {
        local_90 =
        "original_modelborrowed_modelstruct NoAccountModelRestoreEntry with 3 elementsentriesstruct NoAccountModelRestoreJournal with 2 elementsfallback_modelstruct ModelRestoreEntry with 4 elementsstruct ModelRestoreJournal with 2 elementsinternal error: entered unreachable code: invalid Once stateCompression method not supportedZipFileReader was in an invalid stateAES encrypted files cannot be decrypted without the aes-crypto feature.AES encryption without AES extra data fieldArchive header is too largeOversized central directorySupport for multi-disk files is not implementedPassword required to decrypt fileInvalid zip64 locator digital signature headerInvalid digital signature headerInvalid local file headerInvalid Central Directory headerInvalid EOCD64: inconsistent lengthInvalid EOCD64: inconsistency with Locator dataInvalid EOCD64: inconsistent number of filesCould not find EOCD64Multi-disk ZIP files are not supportedInvalid EOCD64 Locator CD offsetNo CDFH foundInvalid CDFH offset in EOCDInvalid EOCD comment lengthCould not find EOCDEOCD64 Locator does not fit in fileEOCD comment exceeds file boundaryEOCD comment length exceeds u16::MAXEOCD64 extends beyond EOCD64 locatorLow EOCD64 record sizeaxum_core::extract::default_body_limit::DefaultBodyLimitKinda Display implementation returned an error unexpectedlydescription() is deprecated; use Displaymid > lencalled `Result::unwrap()` on an `Err` valueErrorattempt to join into collection with len > usize::MAX"
        ;
        pcStack_88 = &DAT_0000000e;
        local_f0 = (char *)&local_90;
        local_e8 = (char *)&<&T_as_core::fmt::Display>::fmt;
        pcVar6 = (char *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_f0);
        pcVar9 = extraout_x11_17;
      }
      else {
        if (pcVar15 != &DAT_8000000000000000) {
          uStack_a8 = (uint)unaff_x25;
          if (!bVar3) {
            uStack_a8 = 0;
          }
          goto LAB_10026981c;
        }
        local_90 =
        "fallback_modelstruct ModelRestoreEntry with 4 elementsstruct ModelRestoreJournal with 2 elementsinternal error: entered unreachable code: invalid Once stateCompression method not supportedZipFileReader was in an invalid stateAES encrypted files cannot be decrypted without the aes-crypto feature.AES encryption without AES extra data fieldArchive header is too largeOversized central directorySupport for multi-disk files is not implementedPassword required to decrypt fileInvalid zip64 locator digital signature headerInvalid digital signature headerInvalid local file headerInvalid Central Directory headerInvalid EOCD64: inconsistent lengthInvalid EOCD64: inconsistency with Locator dataInvalid EOCD64: inconsistent number of filesCould not find EOCD64Multi-disk ZIP files are not supportedInvalid EOCD64 Locator CD offsetNo CDFH foundInvalid CDFH offset in EOCDInvalid EOCD comment lengthCould not find EOCDEOCD64 Locator does not fit in fileEOCD comment exceeds file boundaryEOCD comment length exceeds u16::MAXEOCD64 extends beyond EOCD64 locatorLow EOCD64 record sizeaxum_core::extract::default_body_limit::DefaultBodyLimitKinda Display implementation returned an error unexpectedlydescription() is deprecated; use Displaymid > lencalled `Result::unwrap()` on an `Err` valueErrorattempt to join into collection with len > usize::MAX"
        ;
        pcStack_88 = &DAT_0000000e;
        local_f0 = (char *)&local_90;
        local_e8 = (char *)&<&T_as_core::fmt::Display>::fmt;
        pcVar6 = (char *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_f0);
        pcVar9 = extraout_x11_19;
        if (local_108 != (char *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,local_108,1);
          pcVar9 = extraout_x11_20;
        }
      }
      pcVar12 = (char *)0x1;
      if (pcVar14 != (char *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,pcVar14,1);
        pcVar9 = extraout_x11_21;
        pcVar16 = pcVar6;
      }
      goto LAB_100269798;
    }
    *(long *)(local_a0 + 0x28) = *(long *)(local_a0 + 0x28) + 1;
    *(undefined8 *)(local_a0 + 0x10) = 0;
    <serde_json::read::SliceRead_as_serde_json::read::Read>::parse_str(&local_f0,local_a0 + 0x18,local_a0);
    pcVar6 = local_e8;
    pcVar9 = extraout_x11_00;
    if (local_f0 == (char *)0x2) {
LAB_10026978c:
      pcVar12 = (char *)0x0;
      pcVar13 = (char *)0x1;
      goto LAB_100269798;
    }
    codexmate_lib::core::relay::codex_thread_visibility::model_fallback::ModelRestoreEntry>::deserialize::__FieldVisitor_as_serde_core::de::Visitor>::visit_str
              (&local_90,local_e8,local_e0);
    pcVar6 = pcStack_88;
    pcVar9 = extraout_x11_01;
    if ((char)local_90 == '\x01') goto LAB_10026978c;
    pcVar13 = local_120;
    unaff_x28 = pcVar10;
    if (local_90._1_1_ == '\x03') {
      if (bVar3) {
        local_90 = "attemptsOverflow";
        pcStack_88 = &SUB_00000008;
LAB_100269d18:
        local_f0 = (char *)&local_90;
        local_e8 = (char *)&<&T_as_core::fmt::Display>::fmt;
        pcVar6 = (char *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_15,&local_f0);
        pcVar9 = extraout_x11_32;
        goto LAB_100269778;
      }
      uVar8 = *(ulong *)(lVar4 + 0x28);
      if (uVar8 < *(ulong *)(lVar4 + 0x20)) {
        while( true ) {
          bVar1 = *(byte *)(*(long *)(lVar4 + 0x18) + uVar8);
          if (0x3a < bVar1) goto LAB_100269d4c;
          if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) break;
          uVar8 = uVar8 + 1;
          *(ulong *)(lVar4 + 0x28) = uVar8;
          if (*(ulong *)(lVar4 + 0x20) == uVar8) goto LAB_100269688;
        }
        if ((ulong)bVar1 == 0x3a) {
          *(ulong *)(lVar4 + 0x28) = uVar8 + 1;
          deserialize_u32(&local_90,lVar4);
          pcVar6 = pcStack_88;
          pcVar9 = extraout_x11_02;
          if ((int)local_90 != 1) {
            unaff_x25 = (char *)((ulong)local_90 >> 0x20);
            bVar3 = true;
            goto LAB_100269374;
          }
        }
        else {
LAB_100269d4c:
          local_f0 = &SUB_00000006;
          pcVar6 = (char *)serde_json::de::Deserializer<R>::peek_error(lVar4,&local_f0);
          pcVar9 = extraout_x11_33;
        }
      }
      else {
LAB_100269688:
        local_f0 = (char *)0x3;
        pcVar6 = (char *)serde_json::de::Deserializer<R>::peek_error(lVar4,&local_f0);
        pcVar9 = extraout_x11_08;
      }
      pcVar12 = (char *)0x0;
      pcVar13 = (char *)0x1;
      goto LAB_100269798;
    }
    if (local_90._1_1_ != '\0') {
      if (local_90._1_1_ == '\x01') {
        if (local_108 == &DAT_8000000000000000) {
          uVar8 = *(ulong *)(lVar4 + 0x28);
          if (uVar8 < *(ulong *)(lVar4 + 0x20)) {
            while( true ) {
              bVar1 = *(byte *)(*(long *)(lVar4 + 0x18) + uVar8);
              if (0x3a < bVar1) goto LAB_100269dfc;
              if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) break;
              uVar8 = uVar8 + 1;
              *(ulong *)(lVar4 + 0x28) = uVar8;
              if (*(ulong *)(lVar4 + 0x20) == uVar8) goto LAB_100269ac4;
            }
            if ((ulong)bVar1 == 0x3a) {
              *(ulong *)(lVar4 + 0x28) = uVar8 + 1;
              deserialize_string(&local_90,lVar4);
              pcVar6 = pcStack_88;
              pcVar9 = extraout_x11_05;
              if (local_90 != &DAT_8000000000000000) {
                local_138 = local_80;
                local_108 = local_90;
                local_128 = pcStack_88;
                goto LAB_100269374;
              }
            }
            else {
LAB_100269dfc:
              local_f0 = &SUB_00000006;
              pcVar6 = (char *)serde_json::de::Deserializer<R>::peek_error(lVar4,&local_f0);
              pcVar9 = extraout_x11_35;
            }
          }
          else {
LAB_100269ac4:
            local_f0 = (char *)0x3;
            pcVar6 = (char *)serde_json::de::Deserializer<R>::peek_error(lVar4,&local_f0);
            pcVar9 = extraout_x11_27;
          }
          pcVar12 = (char *)0x0;
          pcVar13 = (char *)0x1;
          pcVar11 = &DAT_8000000000000000;
          goto LAB_100269798;
        }
        local_90 =
        "original_modelborrowed_modelstruct NoAccountModelRestoreEntry with 3 elementsentriesstruct NoAccountModelRestoreJournal with 2 elementsfallback_modelstruct ModelRestoreEntry with 4 elementsstruct ModelRestoreJournal with 2 elementsinternal error: entered unreachable code: invalid Once stateCompression method not supportedZipFileReader was in an invalid stateAES encrypted files cannot be decrypted without the aes-crypto feature.AES encryption without AES extra data fieldArchive header is too largeOversized central directorySupport for multi-disk files is not implementedPassword required to decrypt fileInvalid zip64 locator digital signature headerInvalid digital signature headerInvalid local file headerInvalid Central Directory headerInvalid EOCD64: inconsistent lengthInvalid EOCD64: inconsistency with Locator dataInvalid EOCD64: inconsistent number of filesCould not find EOCD64Multi-disk ZIP files are not supportedInvalid EOCD64 Locator CD offsetNo CDFH foundInvalid CDFH offset in EOCDInvalid EOCD comment lengthCould not find EOCDEOCD64 Locator does not fit in fileEOCD comment exceeds file boundaryEOCD comment length exceeds u16::MAXEOCD64 extends beyond EOCD64 locatorLow EOCD64 record sizeaxum_core::extract::default_body_limit::DefaultBodyLimitKinda Display implementation returned an error unexpectedlydescription() is deprecated; use Displaymid > lencalled `Result::unwrap()` on an `Err` valueErrorattempt to join into collection with len > usize::MAX"
        ;
      }
      else {
        if (pcVar15 == &DAT_8000000000000000) {
          uVar8 = *(ulong *)(lVar4 + 0x28);
          if (uVar8 < *(ulong *)(lVar4 + 0x20)) {
            while( true ) {
              bVar1 = *(byte *)(*(long *)(lVar4 + 0x18) + uVar8);
              if (0x3a < bVar1) goto LAB_100269e20;
              if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) break;
              uVar8 = uVar8 + 1;
              *(ulong *)(lVar4 + 0x28) = uVar8;
              if (*(ulong *)(lVar4 + 0x20) == uVar8) goto LAB_100269a54;
            }
            if ((ulong)bVar1 == 0x3a) {
              *(ulong *)(lVar4 + 0x28) = uVar8 + 1;
              deserialize_string(&local_90,lVar4);
              local_130 = pcStack_88;
              if (local_90 == &DAT_8000000000000000) {
                pcVar12 = (char *)0x0;
                local_138 = (char *)0x1;
                pcVar6 = pcStack_88;
                local_118 = extraout_x11_03;
                goto joined_r0x000100269a84;
              }
              local_118 = local_80;
              pcVar15 = local_90;
              goto LAB_100269374;
            }
LAB_100269e20:
            local_f0 = &SUB_00000006;
            pcVar6 = (char *)serde_json::de::Deserializer<R>::peek_error(lVar4,&local_f0);
            local_118 = extraout_x11_36;
          }
          else {
LAB_100269a54:
            local_f0 = (char *)0x3;
            pcVar6 = (char *)serde_json::de::Deserializer<R>::peek_error(lVar4,&local_f0);
            local_118 = extraout_x11_25;
          }
          pcVar12 = (char *)0x0;
          local_138 = (char *)0x1;
          goto joined_r0x000100269a84;
        }
        local_90 =
        "fallback_modelstruct ModelRestoreEntry with 4 elementsstruct ModelRestoreJournal with 2 elementsinternal error: entered unreachable code: invalid Once stateCompression method not supportedZipFileReader was in an invalid stateAES encrypted files cannot be decrypted without the aes-crypto feature.AES encryption without AES extra data fieldArchive header is too largeOversized central directorySupport for multi-disk files is not implementedPassword required to decrypt fileInvalid zip64 locator digital signature headerInvalid digital signature headerInvalid local file headerInvalid Central Directory headerInvalid EOCD64: inconsistent lengthInvalid EOCD64: inconsistency with Locator dataInvalid EOCD64: inconsistent number of filesCould not find EOCD64Multi-disk ZIP files are not supportedInvalid EOCD64 Locator CD offsetNo CDFH foundInvalid CDFH offset in EOCDInvalid EOCD comment lengthCould not find EOCDEOCD64 Locator does not fit in fileEOCD comment exceeds file boundaryEOCD comment length exceeds u16::MAXEOCD64 extends beyond EOCD64 locatorLow EOCD64 record sizeaxum_core::extract::default_body_limit::DefaultBodyLimitKinda Display implementation returned an error unexpectedlydescription() is deprecated; use Displaymid > lencalled `Result::unwrap()` on an `Err` valueErrorattempt to join into collection with len > usize::MAX"
        ;
      }
      pcStack_88 = &DAT_0000000e;
      goto LAB_100269d18;
    }
    if (pcVar14 != &DAT_8000000000000000) {
      local_90 =
      "thread_idturn_idcwdthread_sourcesourceforked_from_idforked_from_ordinal_exclusivesubagent_history_start_ordinalhistory_baseend_ordinal_exclusiveend_byte_offsettspriorityrollout_idis_subagentincompleteincomplete_reasonsweb_search_coverage_versioncurrent_turnuncertain_inheritanceprevious_totalneeds_legacy_baselinepending_callsrecent_completed_callsturn_startssynthetic_turnsession_cwdturn_cwdsweb_search_coveragestarted_atcompleted_atfailedabortedduration_ms"
      ;
      pcStack_88 = &DAT_00000009;
      goto LAB_100269d18;
    }
    uVar8 = *(ulong *)(lVar4 + 0x28);
    if (*(ulong *)(lVar4 + 0x20) <= uVar8) {
LAB_100269a8c:
      local_f0 = (char *)0x3;
      pcVar6 = (char *)serde_json::de::Deserializer<R>::peek_error(lVar4,&local_f0);
      pcVar9 = extraout_x11_26;
      break;
    }
    while( true ) {
      bVar1 = *(byte *)(*(long *)(lVar4 + 0x18) + uVar8);
      if (0x3a < bVar1) goto LAB_100269dd4;
      if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) break;
      uVar8 = uVar8 + 1;
      *(ulong *)(lVar4 + 0x28) = uVar8;
      if (*(ulong *)(lVar4 + 0x20) == uVar8) goto LAB_100269a8c;
    }
    if ((ulong)bVar1 != 0x3a) {
LAB_100269dd4:
      local_f0 = &SUB_00000006;
      pcVar6 = (char *)serde_json::de::Deserializer<R>::peek_error(lVar4,&local_f0);
      pcVar9 = extraout_x11_34;
      break;
    }
    *(ulong *)(lVar4 + 0x28) = uVar8 + 1;
    deserialize_string(&local_90,lVar4);
    pcVar6 = pcStack_88;
    pcVar9 = extraout_x11_04;
    pcVar13 = pcStack_88;
    unaff_x28 = local_80;
    pcVar14 = local_90;
  } while (local_90 != &DAT_8000000000000000);
  pcVar12 = (char *)0x0;
  pcVar13 = (char *)0x1;
  pcVar14 = &DAT_8000000000000000;
LAB_100269798:
  local_138 = pcVar13;
  local_118 = pcVar9;
  if (((ulong)pcVar15 & 0x7fffffffffffffff) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,pcVar15,1);
    local_118 = extraout_x11_13;
    pcVar16 = pcVar6;
  }
joined_r0x000100269a84:
  local_130 = pcVar16;
  if ((((ulong)pcVar11 & 0x7fffffffffffffff) != 0) && ((int)local_138 != 0)) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,pcVar11,1);
    local_118 = extraout_x11_14;
    local_138 = pcVar6;
  }
  uStack_a8 = (uint)pcVar12;
  if (((ulong)pcVar14 & 0x7fffffffffffffff) == 0) {
    uStack_a8 = 1;
  }
  if (uStack_a8 == 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,pcVar14,1);
    local_118 = extraout_x11_15;
    uStack_a8 = extraout_w8;
  }
  pcVar14 = &DAT_8000000000000000;
  local_120 = pcVar6;
  local_128 = pcVar12;
LAB_10026981c:
  *(char *)(param_2 + 0x38) = *(char *)(param_2 + 0x38) + '\x01';
  local_f0 = pcVar14;
  local_e8 = local_120;
  pcStack_d8 = local_108;
  local_d0 = local_128;
  pcStack_c8 = local_138;
  local_c0 = pcVar15;
  pcStack_b8 = local_130;
  local_b0 = local_118;
  pcVar13 = (char *)serde_json::de::Deserializer<R>::end_map(param_2);
  pcStack_f8 = (char *)CONCAT44(uStack_a4,uStack_a8);
  local_90 = local_b0;
  pcStack_88 = pcStack_f8;
  if (pcVar14 == &DAT_8000000000000000) {
    if (pcVar13 != (char *)0x0) {
      core::ptr::drop_in_place<serde_json::error::ErrorCode>(pcVar13);
LAB_100269bb8:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar13,0x28,8);
    }
  }
  else if (pcVar13 == (char *)0x0) {
    local_100 = local_b0;
    if (pcVar14 != &DAT_8000000000000000) {
LAB_100269c30:
      param_1[2] = pcVar10;
      param_1[3] = local_108;
      param_1[4] = local_128;
      param_1[5] = local_138;
      param_1[6] = pcVar15;
      param_1[7] = local_130;
      param_1[9] = pcStack_f8;
      param_1[8] = local_100;
      *param_1 = pcVar14;
      param_1[1] = local_120;
      return;
    }
  }
  else {
    if (pcVar14 != (char *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,pcVar14,1);
    }
joined_r0x000100269bf8:
    local_120 = pcVar13;
    if (local_108 != (char *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,local_108,1);
    }
    if (pcVar15 != (char *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,pcVar15,1);
    }
  }
LAB_100269d98:
  uVar5 = serde_json::error::Error::fix_position(local_120,param_2);
LAB_100269310:
  *param_1 = 0x8000000000000000;
  param_1[1] = uVar5;
  return;
}
