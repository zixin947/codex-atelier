/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* load @ 1001e85a8 requested 1001e85a8 */

/* codexmate_lib::core::quota_store::load */

void codexmate_lib::core::quota_store::load(undefined8 *param_1)

{
  long lVar1;
  uint uVar2;
  uint uVar3;
  byte bVar4;
  bool bVar5;
  bool bVar6;
  long lVar7;
  long lVar8;
  char *pcVar9;
  long *plVar10;
  undefined8 uVar11;
  long *plVar12;
  long *plVar13;
  long lVar14;
  long lVar15;
  long lVar16;
  int iVar17;
  ulong uVar18;
  long lVar19;
  char *extraout_x9;
  char *extraout_x9_00;
  char *extraout_x9_01;
  char *extraout_x9_02;
  char *extraout_x9_03;
  char *extraout_x9_04;
  char *pcVar20;
  long *extraout_x9_05;
  char *extraout_x9_06;
  long *extraout_x9_07;
  long *extraout_x9_08;
  long *extraout_x9_09;
  long *extraout_x9_10;
  long *extraout_x9_11;
  char *extraout_x9_12;
  char *extraout_x9_13;
  char *extraout_x9_14;
  char *extraout_x9_15;
  char *extraout_x9_16;
  char *extraout_x9_17;
  char *extraout_x9_18;
  char *extraout_x9_19;
  char *pcVar21;
  char *extraout_x9_20;
  char *extraout_x9_21;
  char *extraout_x9_22;
  long *extraout_x10;
  long *extraout_x10_00;
  long *extraout_x10_01;
  long *extraout_x10_02;
  long *extraout_x10_03;
  long *extraout_x10_04;
  long *extraout_x10_05;
  long *extraout_x10_06;
  char *extraout_x10_07;
  long *extraout_x10_08;
  char *extraout_x10_09;
  char *extraout_x10_10;
  char *extraout_x10_11;
  char *extraout_x10_12;
  char *extraout_x10_13;
  char *extraout_x10_14;
  long *extraout_x10_15;
  long *extraout_x10_16;
  long *extraout_x10_17;
  long *extraout_x10_18;
  long *extraout_x10_19;
  long *extraout_x10_20;
  long *extraout_x10_21;
  long *extraout_x10_22;
  long *extraout_x10_23;
  long *extraout_x10_24;
  long *extraout_x10_25;
  long extraout_x13;
  long lVar22;
  ulong extraout_x14;
  ulong extraout_x15;
  ulong uVar23;
  ulong extraout_x16;
  ulong uVar24;
  long extraout_x17;
  long lVar25;
  long *unaff_x22;
  undefined4 unaff_w23;
  char *pcVar26;
  undefined1 auVar27 [16];
  undefined1 auVar28 [12];
  long *local_1f0;
  long *local_1e8;
  char *local_1e0;
  undefined4 local_1d4;
  undefined8 local_198;
  char *local_190;
  long *plStack_188;
  undefined8 local_180;
  long local_170;
  long lStack_168;
  char *local_160;
  long local_158;
  char *local_150;
  char *local_148;
  undefined8 local_140;
  long lStack_138;
  char *local_130;
  char local_128;
  char *local_120;
  long *plStack_118;
  undefined8 local_110;
  undefined8 local_100;
  long *plStack_f8;
  char *local_f0;
  long *plStack_e8;
  undefined8 local_e0;
  long *plStack_d8;
  undefined8 local_c8;
  long *plStack_c0;
  char *local_b8;
  long *plStack_b0;
  undefined4 local_a8;
  undefined4 uStack_a4;
  long *local_a0;
  undefined1 local_98;
  char *local_90 [2];
  char *local_80;
  undefined1 auStack_71 [17];

  std::fs::read_to_string::inner(&local_170);
  pcVar21 = local_160;
  lVar8 = lStack_168;
  lVar7 = local_170;
  if (local_170 == -0x8000000000000000) {
    *param_1 = 2;
    param_1[1] = lStack_168;
    return;
  }
  local_150 = local_160;
  local_148 = (char *)0x0;
  local_140 = 0;
  lStack_138 = lStack_168;
  local_130 = local_160;
  local_170 = 0;
  lStack_168 = 1;
  local_160 = (char *)0x0;
  local_158 = lVar8;
  local_128 = -0x80;
  if (pcVar21 != (char *)0x0) {
    do {
      bVar4 = local_148[lVar8];
      if (0x20 < bVar4 || (1L << ((ulong)bVar4 & 0x3f) & 0x100002600U) == 0) {
        if (bVar4 != 0x5b) {
          if (bVar4 == 0x7b) {
            bVar6 = false;
            bVar5 = false;
            local_128 = '\x7f';
            local_a0 = &local_170;
            local_98 = 1;
            local_1e8 = (long *)&DAT_0000000d;
            local_1e0 =
            "schemaVersionupdatedAtitemsaccountKeycapturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
            ;
            pcVar9 = local_148 + 1;
            pcVar26 = &DAT_8000000000000000;
            pcVar21 = local_148;
            goto LAB_1001e88a4;
          }
          plVar13 = (long *)serde_json::de::Deserializer<R>::peek_invalid_type(&local_170,auStack_71,&DAT_101818ab0);
          goto LAB_1001e90ec;
        }
        local_128 = '\x7f';
        local_148 = local_148 + 1;
        local_c8 = &local_170;
        plStack_c0 = (long *)CONCAT71(plStack_c0._1_7_,1);
        <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_100,&local_c8);
        plVar13 = extraout_x9_05;
        pcVar21 = extraout_x10_07;
        plVar12 = plStack_f8;
        if ((char)local_100 == '\x01') {
LAB_1001e8cd8:
          plStack_f8 = plVar12;
          local_f0 = pcVar21;
          local_100 = &DAT_8000000000000000;
          plVar10 = plVar13;
        }
        else {
          if (local_100._1_1_ != '\x01') {
            uVar11 = 0;
LAB_1001e8cc0:
            plVar12 = (long *)serde_core::de::Error::invalid_length(uVar11,&PTR__anon_dc5b8adae996cb58fd4173e320c2d1cf_744_0x702_101817a78,&DAT_101817930);
            plVar13 = extraout_x9_11;
            pcVar21 = extraout_x10_14;
            goto LAB_1001e8cd8;
          }
          <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_i32(&local_100,local_c8);
          plVar13 = extraout_x9_07;
          pcVar21 = extraout_x10_09;
          plVar12 = plStack_f8;
          if ((int)local_100 == 1) goto LAB_1001e8cd8;
          unaff_w23 = local_100._4_4_;
          <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_100,&local_c8);
          plVar13 = extraout_x9_08;
          pcVar21 = extraout_x10_10;
          plVar12 = plStack_f8;
          if ((char)local_100 == '\x01') goto LAB_1001e8cd8;
          if (local_100._1_1_ != '\x01') {
            uVar11 = 1;
            goto LAB_1001e8cc0;
          }
          auVar27 = <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_i64(local_c8);
          plVar10 = auVar27._8_8_;
          plVar13 = extraout_x9_09;
          pcVar21 = extraout_x10_11;
          plVar12 = plVar10;
          if ((auVar27._0_8_ & 1) != 0) goto LAB_1001e8cd8;
          <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_100,&local_c8);
          plVar13 = extraout_x9_10;
          pcVar21 = extraout_x10_12;
          plVar12 = plStack_f8;
          if ((char)local_100 == '\x01') goto LAB_1001e8cd8;
          if (local_100._1_1_ != '\x01') {
            uVar11 = 2;
            goto LAB_1001e8cc0;
          }
          <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_seq(&local_100,local_c8);
          if (local_100 == &DAT_8000000000000000) {
            local_100 = &DAT_8000000000000000;
            plVar10 = (long *)&DAT_8000000000000000;
            local_f0 = extraout_x10_13;
          }
        }
        local_128 = local_128 + '\x01';
        local_c8 = (long *)local_100;
        plStack_c0 = plStack_f8;
        local_b8 = local_f0;
        plStack_b0 = plVar10;
        local_a8 = unaff_w23;
        plVar12 = (long *)serde_json::de::Deserializer<R>::end_seq(&local_170);
        plVar13 = plStack_c0;
        pcVar21 = (char *)local_c8;
        plStack_f8 = plStack_c0;
        local_100 = (char *)local_c8;
        plStack_e8 = plStack_b0;
        local_f0 = local_b8;
        local_e0 = CONCAT44(uStack_a4,local_a8);
        plStack_d8 = plVar12;
        if (local_c8 == (long *)&DAT_8000000000000000) {
          if (plVar12 == (long *)0x0) goto LAB_1001e90ec;
          ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(plVar12);
          goto LAB_1001e8f6c;
        }
        if (plVar12 == (long *)0x0) goto LAB_1001e8fcc;
        <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_100);
        goto LAB_1001e8f9c;
      }
      local_148 = local_148 + 1;
    } while (pcVar21 != local_148);
  }
  local_100 = &DAT_00000005;
  plVar10 = (long *)serde_json::de::Deserializer<R>::peek_error(&local_170,&local_100);
  goto LAB_1001e866c;
LAB_1001e88a4:
  local_148 = pcVar9;
  <serde_json::de::MapAccess<R>as_serde_core::de::MapAccess>::next_key_seed::has_next_key(&local_100,&local_a0);
  plVar13 = local_a0;
  local_90[0] = pcVar26;
  local_80 = pcVar21;
  if ((char)local_100 == '\x01') {
    pcVar20 = extraout_x9;
    plVar12 = extraout_x10;
    plVar10 = plStack_f8;
    if (pcVar26 == &DAT_8000000000000000) {
LAB_1001e8c0c:
      pcVar26 = &DAT_8000000000000000;
      pcVar21 = pcVar20;
      local_1f0 = plVar12;
      unaff_x22 = plVar10;
    }
    else {
LAB_1001e8ee8:
      <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(local_90);
      pcVar21 = extraout_x9_18;
      local_1f0 = extraout_x10_21;
      if (pcVar26 != (char *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x22,(long)pcVar26 * 0xc0,8);
        pcVar21 = extraout_x9_19;
        local_1f0 = extraout_x10_22;
      }
      pcVar26 = &DAT_8000000000000000;
      unaff_x22 = plVar10;
    }
    goto LAB_1001e8f10;
  }
  if (local_100._1_1_ == '\x01') {
    local_a0[5] = local_a0[5] + 1;
    plVar13[2] = 0;
    <serde_json::read::StrRead_as_serde_json::read::Read>::parse_str(&local_100,plVar13 + 3,plVar13);
    pcVar20 = local_100;
    plVar12 = extraout_x10_00;
    plVar10 = plStack_f8;
    if (local_100 == (char *)0x2) goto joined_r0x0001001e90cc;
    if (local_f0 == &DAT_00000005) {
      if ((int)*plStack_f8 != 0x6d657469 || *(char *)((long)plStack_f8 + 4) != 's') goto LAB_1001e8a34;
      if (pcVar26 != &DAT_8000000000000000) {
        local_1e0 =
        "itemsaccountKeycapturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
        ;
        local_1e8 = (long *)&DAT_00000005;
        goto LAB_1001e9124;
      }
      uVar18 = plVar13[5];
      if ((ulong)plVar13[4] <= uVar18) {
LAB_1001e8e3c:
        local_100 = (char *)0x3;
        plStack_c0 = (long *)serde_json::de::Deserializer<R>::peek_error(plVar13,&local_100);
        pcVar20 = extraout_x9_15;
        plVar13 = extraout_x10_18;
        goto LAB_1001e8e64;
      }
      while( true ) {
        bVar4 = *(byte *)(plVar13[3] + uVar18);
        if (0x3a < bVar4) goto LAB_1001e915c;
        if ((1L << ((ulong)bVar4 & 0x3f) & 0x100002600U) == 0) break;
        uVar18 = uVar18 + 1;
        plVar13[5] = uVar18;
        if (plVar13[4] == uVar18) goto LAB_1001e8e3c;
      }
      if ((ulong)bVar4 != 0x3a) {
LAB_1001e915c:
        local_100 = &SUB_00000006;
        plStack_c0 = (long *)serde_json::de::Deserializer<R>::peek_error(plVar13,&local_100);
        pcVar20 = extraout_x9_21;
        plVar13 = extraout_x10_24;
        goto LAB_1001e8e64;
      }
      plVar13[5] = uVar18 + 1;
      <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_seq(&local_c8,plVar13);
      pcVar20 = &DAT_8000000000000000;
      plVar13 = extraout_x10_06;
      pcVar9 = local_148;
      pcVar26 = (char *)local_c8;
      unaff_x22 = plStack_c0;
      pcVar21 = local_b8;
      if (local_c8 == (long *)&DAT_8000000000000000) goto LAB_1001e8e64;
      goto LAB_1001e88a4;
    }
    if (local_f0 == &DAT_00000009) {
      if (*plStack_f8 == 0x4164657461647075 && (char)plStack_f8[1] == 't') {
        if (bVar6) {
          local_1e0 =
          "updatedAtitemsaccountKeycapturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
          ;
          local_1e8 = (long *)&DAT_00000009;
LAB_1001e9124:
          local_c8 = (long *)local_1e0;
          plStack_c0 = local_1e8;
          local_100 = (char *)&local_c8;
          plStack_f8 = &<&T_as_core::fmt::Display>::fmt;
          plVar10 = (long *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_15,&local_100);
          pcVar20 = extraout_x9_20;
          plVar12 = extraout_x10_23;
        }
        else {
          uVar18 = plVar13[5];
          if (uVar18 < (ulong)plVar13[4]) {
            do {
              bVar4 = *(byte *)(plVar13[3] + uVar18);
              if (0x3a < bVar4) {
LAB_1001e8afc:
                local_100 = &SUB_00000006;
                local_1f0 = (long *)serde_json::de::Deserializer<R>::peek_error(plVar13,&local_100);
                pcVar20 = extraout_x9_04;
                plVar12 = extraout_x10_05;
                goto LAB_1001e8a0c;
              }
              if ((1L << ((ulong)bVar4 & 0x3f) & 0x100002600U) == 0) {
                if ((ulong)bVar4 != 0x3a) goto LAB_1001e8afc;
                plVar13[5] = uVar18 + 1;
                auVar27 = <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_i64(plVar13);
                pcVar20 = extraout_x9_03;
                plVar12 = extraout_x10_04;
                goto LAB_1001e8aec;
              }
              uVar18 = uVar18 + 1;
              plVar13[5] = uVar18;
            } while (plVar13[4] != uVar18);
          }
          local_100 = (char *)0x3;
          local_1f0 = (long *)serde_json::de::Deserializer<R>::peek_error(plVar13,&local_100);
          pcVar20 = extraout_x9_00;
          plVar12 = extraout_x10_01;
LAB_1001e8a0c:
          auVar27._8_8_ = local_1f0;
          auVar27._0_8_ = 1;
LAB_1001e8aec:
          local_1f0 = auVar27._8_8_;
          plVar10 = local_1f0;
          if ((auVar27._0_8_ & 1) == 0) {
            bVar6 = true;
            pcVar9 = local_148;
            goto LAB_1001e88a4;
          }
        }
      }
      else {
LAB_1001e8a34:
        uVar18 = plVar13[5];
        if (uVar18 < (ulong)plVar13[4]) {
          while( true ) {
            bVar4 = *(byte *)(plVar13[3] + uVar18);
            if (0x3a < bVar4) goto LAB_1001e8ea8;
            if ((1L << ((ulong)bVar4 & 0x3f) & 0x100002600U) == 0) break;
            uVar18 = uVar18 + 1;
            plVar13[5] = uVar18;
            if (plVar13[4] == uVar18) goto LAB_1001e8bdc;
          }
          if ((ulong)bVar4 != 0x3a) {
LAB_1001e8ea8:
            local_100 = &SUB_00000006;
            plVar10 = (long *)serde_json::de::Deserializer<R>::peek_error(plVar13,&local_100);
            pcVar20 = extraout_x9_17;
            plVar12 = extraout_x10_20;
            goto joined_r0x0001001e90cc;
          }
          plVar13[5] = uVar18 + 1;
          plVar10 = (long *)serde_json::de::Deserializer<R>::ignore_value(plVar13);
          pcVar9 = local_148;
          pcVar20 = extraout_x9_01;
          plVar12 = extraout_x10_02;
          if (plVar10 == (long *)0x0) goto LAB_1001e88a4;
        }
        else {
LAB_1001e8bdc:
          local_100 = (char *)0x3;
          plVar10 = (long *)serde_json::de::Deserializer<R>::peek_error(plVar13,&local_100);
          pcVar20 = extraout_x9_06;
          plVar12 = extraout_x10_08;
        }
      }
    }
    else {
      if ((local_f0 != &DAT_0000000d) || (*plStack_f8 != 0x6556616d65686373 || *(long *)((long)plStack_f8 + 5) != 0x6e6f697372655661)) goto LAB_1001e8a34;
      if (bVar5) goto LAB_1001e9124;
      uVar18 = plVar13[5];
      if (uVar18 < (ulong)plVar13[4]) {
        while( true ) {
          bVar4 = *(byte *)(plVar13[3] + uVar18);
          if (0x3a < bVar4) goto LAB_1001e9184;
          if ((1L << ((ulong)bVar4 & 0x3f) & 0x100002600U) == 0) break;
          uVar18 = uVar18 + 1;
          plVar13[5] = uVar18;
          if (plVar13[4] == uVar18) goto LAB_1001e8e6c;
        }
        if ((ulong)bVar4 == 0x3a) {
          plVar13[5] = uVar18 + 1;
          <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_i32(&local_c8,plVar13);
          pcVar20 = extraout_x9_02;
          plVar12 = extraout_x10_03;
          plVar10 = plStack_c0;
          if ((int)local_c8 != 1) {
            local_1d4 = local_c8._4_4_;
            bVar5 = true;
            pcVar9 = local_148;
            goto LAB_1001e88a4;
          }
        }
        else {
LAB_1001e9184:
          local_100 = &SUB_00000006;
          plVar10 = (long *)serde_json::de::Deserializer<R>::peek_error(plVar13,&local_100);
          pcVar20 = extraout_x9_22;
          plVar12 = extraout_x10_25;
        }
      }
      else {
LAB_1001e8e6c:
        local_100 = (char *)0x3;
        plVar10 = (long *)serde_json::de::Deserializer<R>::peek_error(plVar13,&local_100);
        pcVar20 = extraout_x9_16;
        plVar12 = extraout_x10_19;
      }
    }
  }
  else if (bVar5) {
    if (bVar6) {
      if (pcVar26 != &DAT_8000000000000000) goto LAB_1001e8f10;
      local_c8 = (long *)0x101305562;
      plStack_c0 = (long *)0x5;
      local_100 = (char *)&local_c8;
      plStack_f8 = &<&T_as_core::fmt::Display>::fmt;
      plStack_c0 = (long *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_100);
      pcVar20 = extraout_x9_12;
      plVar13 = extraout_x10_15;
      goto LAB_1001e8e64;
    }
    local_c8 = (long *)0x101305559;
    plStack_c0 = (long *)&DAT_00000009;
    local_100 = (char *)&local_c8;
    plStack_f8 = &<&T_as_core::fmt::Display>::fmt;
    plVar10 = (long *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_100);
    pcVar20 = extraout_x9_14;
    plVar12 = extraout_x10_17;
  }
  else {
    local_c8 = (long *)0x10130554c;
    plStack_c0 = (long *)&DAT_0000000d;
    local_100 = (char *)&local_c8;
    plStack_f8 = &<&T_as_core::fmt::Display>::fmt;
    plVar10 = (long *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_100);
    pcVar20 = extraout_x9_13;
    plVar12 = extraout_x10_16;
  }
joined_r0x0001001e90cc:
  if (pcVar26 != &DAT_8000000000000000) goto LAB_1001e8ee8;
  goto LAB_1001e8c0c;
LAB_1001e8e64:
  pcVar26 = &DAT_8000000000000000;
  pcVar21 = pcVar20;
  local_1f0 = plVar13;
  unaff_x22 = plStack_c0;
LAB_1001e8f10:
  local_128 = local_128 + '\x01';
  local_a8 = local_1d4;
  local_c8 = (long *)pcVar26;
  plStack_c0 = unaff_x22;
  local_b8 = pcVar21;
  plStack_b0 = local_1f0;
  plVar12 = (long *)serde_json::de::Deserializer<R>::end_map(&local_170);
  plVar13 = plStack_c0;
  pcVar21 = (char *)local_c8;
  plStack_f8 = plStack_c0;
  local_100 = (char *)local_c8;
  plStack_e8 = plStack_b0;
  local_f0 = local_b8;
  local_e0 = CONCAT44(uStack_a4,local_a8);
  plStack_d8 = plVar12;
  if (local_c8 == (long *)&DAT_8000000000000000) {
    if (plVar12 != (long *)0x0) {
      ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(plVar12);
LAB_1001e8f6c:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar12,0x28,8);
    }
  }
  else if (plVar12 == (long *)0x0) {
LAB_1001e8fcc:
    plVar12 = plStack_b0;
    pcVar21 = local_b8;
    plVar13 = plStack_c0;
    pcVar9 = (char *)local_c8;
    uVar11 = local_e0;
    plStack_118 = plStack_b0;
    local_120 = local_b8;
    local_110 = local_e0;
    local_100 = (char *)local_c8;
    plStack_f8 = plStack_c0;
    local_f0 = local_b8;
    plStack_e8 = plStack_b0;
    if (local_c8 != (long *)&DAT_8000000000000000) {
      plStack_188 = plStack_b0;
      local_190 = local_b8;
      local_180 = local_e0;
      plVar10 = plStack_c0;
      if (local_148 < local_150) {
        do {
          if (0x20 < (byte)local_148[local_158] || (1L << ((ulong)(byte)local_148[local_158] & 0x3f) & 0x100002600U) == 0) {
            local_c8 = (long *)&DAT_00000016;
            plVar10 = (long *)serde_json::de::Deserializer<R>::peek_error(&local_170,&local_c8);
            <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_100);
            if (pcVar9 == (char *)0x0) goto LAB_1001e866c;
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar13,(long)pcVar9 * 0xc0,8);
            pcVar9 = &DAT_8000000000000000;
            break;
          }
          local_148 = local_148 + 1;
        } while (local_150 != local_148);
      }
      goto joined_r0x0001001e909c;
    }
  }
  else {
    <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_100);
LAB_1001e8f9c:
    plVar13 = plVar12;
    if (pcVar21 != (char *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_f8,(long)pcVar21 * 0xc0,8);
    }
  }
LAB_1001e90ec:
  plVar10 = (long *)serde_json::error::Error::fix_position(plVar13,&local_170);
LAB_1001e866c:
  pcVar9 = &DAT_8000000000000000;
  plVar12 = plStack_188;
  pcVar21 = local_190;
  uVar11 = local_180;
joined_r0x0001001e909c:
  if (local_170 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_168,local_170,1);
  }
  if (pcVar9 == &DAT_8000000000000000) {
    *param_1 = 3;
    param_1[1] = plVar10;
  }
  else {
    iVar17 = (int)uVar11;
    if (iVar17 < 4) {
      iVar17 = 3;
    }
    local_198 = CONCAT44((int)((ulong)uVar11 >> 0x20),iVar17);
    auVar28 = std::time::SystemTime::now();
    local_100 = auVar28._0_8_;
    plStack_f8 = (long *)CONCAT44(plStack_f8._4_4_,auVar28._8_4_);
    std::time::SystemTime::duration_since(&local_170,&local_100,0,0);
    lVar1 = 0;
    if ((int)local_170 == 0) {
      lVar1 = lStack_168;
    }
    if (pcVar21 != (char *)0x0) {
      lVar19 = 0;
      lVar22 = extraout_x13;
      uVar18 = extraout_x14;
      uVar23 = extraout_x15;
      uVar24 = extraout_x16;
      lVar25 = extraout_x17;
      do {
        lVar14 = *(long *)((long)plVar10 + lVar19 + 0x10);
        lVar15 = *(long *)((long)plVar10 + lVar19 + 0x18);
        if (((lVar14 == 2) || (((uint)lVar14 & (uint)(lVar15 <= lVar1)) != 0)) || ((*(uint *)((long)plVar10 + lVar19 + 0x20) & 1) == 0)) {
          lVar16 = 2;
        }
        else {
          uVar2 = *(uint *)((long)plVar10 + lVar19 + 0x24);
          lVar16 = 2;
          if (uVar2 - 1 < 0x168) {
            lStack_168 = *(long *)((long)plVar10 + lVar19 + 0x30);
            local_170 = *(long *)((long)plVar10 + lVar19 + 0x28);
            lVar16 = lVar14;
            lVar22 = lVar15;
            uVar18 = (ulong)uVar2;
          }
        }
        *(long *)((long)plVar10 + lVar19 + 0x10) = lVar16;
        *(long *)((long)plVar10 + lVar19 + 0x18) = lVar22;
        *(undefined4 *)((long)plVar10 + lVar19 + 0x20) = 1;
        *(int *)((long)plVar10 + lVar19 + 0x24) = (int)uVar18;
        *(long *)((long)plVar10 + lVar19 + 0x30) = lStack_168;
        *(long *)((long)plVar10 + lVar19 + 0x28) = local_170;
        lVar14 = *(long *)((long)plVar10 + lVar19 + 0x38);
        if (lVar14 == 2) {
LAB_1001e8740:
          lVar16 = 2;
        }
        else {
          lVar15 = *(long *)((long)plVar10 + lVar19 + 0x40);
          uVar2 = *(uint *)((long)plVar10 + lVar19 + 0x4c);
          if (((uint)lVar14 & (uint)(lVar15 <= lVar1)) != 0) goto LAB_1001e8740;
          uVar3 = *(uint *)((long)plVar10 + lVar19 + 0x48);
          lVar16 = 2;
          if ((uVar3 & (int)uVar2 < 0x21c0) == 0) {
            lStack_168 = *(long *)((long)plVar10 + lVar19 + 0x58);
            local_170 = *(long *)((long)plVar10 + lVar19 + 0x50);
            lVar16 = lVar14;
            uVar23 = (ulong)uVar2;
            uVar24 = (ulong)uVar3;
            lVar25 = lVar15;
          }
        }
        *(long *)((long)plVar10 + lVar19 + 0x38) = lVar16;
        *(long *)((long)plVar10 + lVar19 + 0x40) = lVar25;
        *(int *)((long)plVar10 + lVar19 + 0x48) = (int)uVar24;
        *(int *)((long)plVar10 + lVar19 + 0x4c) = (int)uVar23;
        *(long *)((long)plVar10 + lVar19 + 0x58) = lStack_168;
        *(long *)((long)plVar10 + lVar19 + 0x50) = local_170;
        lVar19 = lVar19 + 0xc0;
      } while ((long)pcVar21 * 0xc0 - lVar19 != 0);
    }
    param_1[2] = plVar10;
    param_1[1] = pcVar9;
    param_1[4] = plVar12;
    param_1[3] = pcVar21;
    param_1[5] = local_198;
    *param_1 = 0xb;
  }
  if (lVar7 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar8,lVar7,1);
  }
  return;
}
