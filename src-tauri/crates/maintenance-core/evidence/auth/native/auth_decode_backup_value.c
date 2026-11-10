/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* decode_backup @ 1001f584c requested 1001f584c */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::config_takeover::decode_backup */

void codexmate_lib::core::relay::config_takeover::decode_backup(undefined8 *param_1,long param_2,ulong param_3)

{
  byte bVar1;
  bool bVar2;
  char *****pppppcVar3;
  ulong *puVar4;
  long lVar5;
  int *piVar6;
  int *piVar7;
  char ****ppppcVar8;
  long extraout_x9;
  long extraout_x9_00;
  long extraout_x9_01;
  long extraout_x9_02;
  long extraout_x9_03;
  int *unaff_x20;
  char *******pppppppcVar9;
  long unaff_x25;
  long lVar10;
  char ****local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  long local_180;
  ulong uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  char *****local_160;
  undefined8 *local_158;
  undefined8 local_150;
  long local_148;
  ulong uStack_140;
  ulong local_138;
  undefined8 uStack_130;
  char local_128;
  char *******local_120;
  int *piStack_118;
  long lStack_110;
  undefined8 uStack_108;
  long local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  ulong *puStack_e8;
  long local_e0;
  undefined8 uStack_d8;
  int *local_d0;
  char *******local_c0;
  int *piStack_b8;
  long local_b0;
  undefined4 local_a8;
  undefined4 uStack_a4;
  char *****local_a0;
  undefined1 local_98;
  char *******local_90;
  undefined1 auStack_71 [17];

  local_170 = 0;
  uStack_168 = 0;
  local_1a8 = (char ****)0x10130612c;
  uStack_1a0 = 0x22;
  uStack_130 = 0;
  local_138 = 0;
  local_160 = (char *****)0x0;
  local_158 = (undefined8 *)0x1;
  local_150 = 0;
  local_128 = -0x80;
  local_180 = param_2;
  uStack_178 = param_3;
  local_148 = param_2;
  uStack_140 = param_3;
  if (param_3 != 0) {
    do {
      bVar1 = *(byte *)(param_2 + local_138);
      if (0x20 < bVar1 || (1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) {
        if (bVar1 != 0x5b) {
          if (bVar1 != 0x7b) {
            piVar7 = (int *)serde_json::de::Deserializer<R>::peek_invalid_type(&local_160,auStack_71,&DAT_101818b90);
            goto LAB_1001f60bc;
          }
          bVar2 = false;
          local_128 = '\x7f';
          local_138 = local_138 + 1;
          local_a0 = (char *****)&local_160;
          local_98 = 1;
          pppppppcVar9 = (char *******)&DAT_8000000000000000;
          goto LAB_1001f5a24;
        }
        local_128 = '\x7f';
        local_138 = local_138 + 1;
        local_c0 = (char *******)&local_160;
        piStack_b8 = (int *)CONCAT71(piStack_b8._1_7_,1);
        <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_f0,&local_c0);
        lVar5 = extraout_x9;
        if ((char)local_f0 == '\x01') {
LAB_1001f5c54:
          local_e0 = lVar5;
          local_f0 = (char *******)&DAT_8000000000000000;
        }
        else {
          if (local_f0._1_1_ == '\x01') {
            <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_u32(&local_f0,local_c0);
            lVar5 = extraout_x9_00;
            if ((int)local_f0 != 1) {
              if (local_f0._4_4_ == 1) goto LAB_1001f5c38;
              puStack_e8 = (ulong *)<serde_json::error::Error_as_serde_core::de::Error>::custom
                                              ("unsupported takeover backup versionCodex config.tomlsonnet-5-5fable-5-1mythos-5-1opus-5-5fable-5mythos-5mythos-previewopus-5opus-4-8sonnet-4-63-7-sonnetsonnet-4-5haiku-4-5opus-4mediumlowhighxhighmax"
                                               ,0x23);
              lVar5 = extraout_x9_03;
            }
            goto LAB_1001f5c54;
          }
LAB_1001f5c38:
          <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_f0,&local_c0);
          lVar5 = extraout_x9_01;
          if ((char)local_f0 == '\x01') goto LAB_1001f5c54;
          if (local_f0._1_1_ != '\x01') {
            puStack_e8 = (ulong *)serde_core::de::Error::invalid_length(1,&PTR_s_struct_LegacyBackup_with_2_eleme_10130772a_0x10e_101817b88,&DAT_101817930);
            lVar5 = extraout_x9_02;
            goto LAB_1001f5c54;
          }
          <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_seq(&local_f0,local_c0);
          lVar5 = -0x8000000000000000;
          if (local_f0 == (char *******)&DAT_8000000000000000) goto LAB_1001f5c54;
        }
        local_128 = local_128 + '\x01';
        local_a8 = 1;
        local_c0 = local_f0;
        piStack_b8 = (int *)puStack_e8;
        local_b0 = local_e0;
        piVar6 = (int *)serde_json::de::Deserializer<R>::end_seq(&local_160);
        piVar7 = piStack_b8;
        pppppppcVar9 = local_c0;
        uStack_d8 = CONCAT44(uStack_a4,local_a8);
        puStack_e8 = (ulong *)piStack_b8;
        local_f0 = local_c0;
        local_e0 = local_b0;
        local_d0 = piVar6;
        if (local_c0 != (char *******)&DAT_8000000000000000) {
          if (piVar6 == (int *)0x0) goto LAB_1001f5eb8;
          <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_f0);
          goto LAB_1001f5e88;
        }
        if (piVar6 == (int *)0x0) goto LAB_1001f60bc;
        ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(piVar6);
        goto LAB_1001f5e58;
      }
      local_138 = local_138 + 1;
    } while (param_3 != local_138);
  }
  local_f0 = (char *******)0x5;
  puVar4 = (ulong *)serde_json::de::Deserializer<R>::peek_error(&local_160,&local_f0);
  goto LAB_1001f5904;
LAB_1001f5a24:
  lVar5 = unaff_x25;
  <serde_json::de::MapAccess<R>as_serde_core::de::MapAccess>::next_key_seed::has_next_key(&local_f0,&local_a0);
  pppppcVar3 = local_a0;
  puVar4 = puStack_e8;
  local_90 = pppppppcVar9;
  if ((char)local_f0 == '\x01') goto LAB_1001f5da8;
  if (local_f0._1_1_ != '\x01') {
    piStack_b8 = unaff_x20;
    if (pppppppcVar9 == (char *******)&DAT_8000000000000000) {
      local_c0 = (char *******)0x101306155;
      piStack_b8 = (int *)0x5;
      local_f0 = (char *******)&local_c0;
      puStack_e8 = &<&T_as_core::fmt::Display>::fmt;
      pppppppcVar9 = (char *******)&DAT_8000000000000000;
      piStack_b8 = (int *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_f0);
    }
    goto LAB_1001f5e04;
  }
  local_a0[5] = (char ****)((long)local_a0[5] + 1);
  pppppcVar3[2] = (char ****)0x0;
  <serde_json::read::SliceRead_as_serde_json::read::Read>::parse_str(&local_f0,pppppcVar3 + 3,pppppcVar3);
  puVar4 = puStack_e8;
  if (local_f0 != (char *******)0x2) {
    if (local_e0 != 7) {
      if (local_e0 != 5) {
LAB_1001f5ff4:
        puVar4 = (ulong *)serde_core::de::Error::unknown_field(puStack_e8,local_e0,&PTR__anon_dc5b8adae996cb58fd4173e320c2d1cf_369_101817210,2);
        goto LAB_1001f5da8;
      }
      if ((int)*puStack_e8 != 0x656e696c || (char)*(int *)((long)puStack_e8 + 4) != 's') {
        local_e0 = 5;
        goto LAB_1001f5ff4;
      }
      if (pppppppcVar9 != (char *******)&DAT_8000000000000000) {
        local_c0 = (char *******)0x101306155;
        piStack_b8 = (int *)&DAT_00000005;
        goto LAB_1001f6030;
      }
      ppppcVar8 = pppppcVar3[5];
      if (ppppcVar8 < pppppcVar3[4]) {
        while( true ) {
          bVar1 = *(byte *)((long)pppppcVar3[3] + (long)ppppcVar8);
          if (0x3a < bVar1) goto LAB_1001f6080;
          if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) break;
          ppppcVar8 = (char ****)((long)ppppcVar8 + 1);
          pppppcVar3[5] = ppppcVar8;
          if (pppppcVar3[4] == ppppcVar8) goto LAB_1001f5ce4;
        }
        if ((ulong)bVar1 == 0x3a) goto code_r0x0001001f5b9c;
LAB_1001f6080:
        local_f0 = (char *******)&SUB_00000006;
        piStack_b8 = (int *)serde_json::de::Deserializer<R>::peek_error(pppppcVar3,&local_f0);
      }
      else {
LAB_1001f5ce4:
        local_f0 = (char *******)0x3;
        piStack_b8 = (int *)serde_json::de::Deserializer<R>::peek_error(pppppcVar3,&local_f0);
      }
      goto LAB_1001f5d04;
    }
    if ((int)*puStack_e8 != 0x73726576 || *(int *)((long)puStack_e8 + 3) != 0x6e6f6973) {
      local_e0 = 7;
      goto LAB_1001f5ff4;
    }
    if (bVar2) {
      local_c0 = (char *******)0x10130614e;
      piStack_b8 = (int *)&DAT_00000007;
LAB_1001f6030:
      local_f0 = (char *******)&local_c0;
      puStack_e8 = &<&T_as_core::fmt::Display>::fmt;
      puVar4 = (ulong *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_15,&local_f0);
    }
    else {
      ppppcVar8 = pppppcVar3[5];
      if (ppppcVar8 < pppppcVar3[4]) {
        while( true ) {
          bVar1 = *(byte *)((long)pppppcVar3[3] + (long)ppppcVar8);
          if (0x3a < bVar1) goto LAB_1001f6060;
          if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) break;
          ppppcVar8 = (char ****)((long)ppppcVar8 + 1);
          pppppcVar3[5] = ppppcVar8;
          if (pppppcVar3[4] == ppppcVar8) goto LAB_1001f5cc0;
        }
        if ((ulong)bVar1 == 0x3a) {
          pppppcVar3[5] = (char ****)((long)ppppcVar8 + 1);
          <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_u32(&local_f0,pppppcVar3);
          puVar4 = puStack_e8;
          if ((int)local_f0 != 1) {
            bVar2 = true;
            unaff_x25 = lVar5;
            if (local_f0._4_4_ != 1) {
              puVar4 = (ulong *)<serde_json::error::Error_as_serde_core::de::Error>::custom
                                          ("unsupported takeover backup versionCodex config.tomlsonnet-5-5fable-5-1mythos-5-1opus-5-5fable-5mythos-5mythos-previewopus-5opus-4-8sonnet-4-63-7-sonnetsonnet-4-5haiku-4-5opus-4mediumlowhighxhighmax"
                                           ,0x23);
              goto LAB_1001f5da8;
            }
            goto LAB_1001f5a24;
          }
        }
        else {
LAB_1001f6060:
          local_f0 = (char *******)&SUB_00000006;
          puVar4 = (ulong *)serde_json::de::Deserializer<R>::peek_error(pppppcVar3,&local_f0);
        }
      }
      else {
LAB_1001f5cc0:
        local_f0 = (char *******)0x3;
        puVar4 = (ulong *)serde_json::de::Deserializer<R>::peek_error(pppppcVar3,&local_f0);
      }
    }
  }
LAB_1001f5da8:
  if (pppppppcVar9 != (char *******)&DAT_8000000000000000) {
    if (lVar5 != 0) {
      piVar7 = unaff_x20 + 2;
      lVar10 = lVar5;
      do {
        if (*(long *)(piVar7 + -2) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)piVar7,*(long *)(piVar7 + -2),1);
        }
        piVar7 = piVar7 + 6;
        lVar10 = lVar10 + -1;
        lVar5 = 0;
      } while (lVar10 != 0);
    }
    if (pppppppcVar9 != (char *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x20,(long)pppppppcVar9 * 0x18,8);
    }
  }
  piStack_b8 = (int *)puVar4;
  pppppppcVar9 = (char *******)&DAT_8000000000000000;
  goto LAB_1001f5e04;
code_r0x0001001f5b9c:
  pppppcVar3[5] = (char ****)((long)ppppcVar8 + 1);
  <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_seq(&local_c0,pppppcVar3);
  unaff_x20 = piStack_b8;
  pppppppcVar9 = local_c0;
  unaff_x25 = local_b0;
  if (local_c0 == (char *******)&DAT_8000000000000000) goto LAB_1001f5d04;
  goto LAB_1001f5a24;
LAB_1001f5d04:
  pppppppcVar9 = (char *******)&DAT_8000000000000000;
LAB_1001f5e04:
  local_128 = local_128 + '\x01';
  local_a8 = 1;
  local_c0 = pppppppcVar9;
  local_b0 = lVar5;
  piVar6 = (int *)serde_json::de::Deserializer<R>::end_map(&local_160);
  piVar7 = piStack_b8;
  pppppppcVar9 = local_c0;
  uStack_d8 = CONCAT44(uStack_a4,local_a8);
  puStack_e8 = (ulong *)piStack_b8;
  local_f0 = local_c0;
  local_e0 = local_b0;
  local_d0 = piVar6;
  if (local_c0 == (char *******)&DAT_8000000000000000) {
    if (piVar6 != (int *)0x0) {
      ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(piVar6);
LAB_1001f5e58:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(piVar6,0x28,8);
    }
  }
  else if (piVar6 == (int *)0x0) {
LAB_1001f5eb8:
    uStack_f8 = uStack_d8;
    local_100 = local_b0;
    piVar7 = piStack_b8;
    local_f0 = local_c0;
    puStack_e8 = (ulong *)piStack_b8;
    local_e0 = local_b0;
    if (local_c0 != (char *******)&DAT_8000000000000000) {
      local_120 = local_c0;
      piStack_118 = piStack_b8;
      uStack_108 = uStack_d8;
      lStack_110 = local_b0;
      local_f0 = local_120;
      puStack_e8 = (ulong *)piStack_118;
      if (local_138 < uStack_140) {
        do {
          if (0x20 < *(byte *)(local_148 + local_138) || (1L << ((ulong)*(byte *)(local_148 + local_138) & 0x3f) & 0x100002600U) == 0) {
            local_c0 = (char *******)&DAT_00000016;
            puVar4 = (ulong *)serde_json::de::Deserializer<R>::peek_error(&local_160,&local_c0);
            local_f0 = (char *******)0x8000000000000000;
            puStack_e8 = puVar4;
            <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_120);
            if (local_120 != (char *******)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(piStack_118,(long)local_120 * 0x18,8);
            }
            goto LAB_1001f5910;
          }
          local_138 = local_138 + 1;
        } while (uStack_140 != local_138);
      }
      if (local_160 != (char *****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_158,local_160,1);
      }
      param_1[2] = puStack_e8;
      param_1[1] = local_f0;
      param_1[4] = uStack_d8;
      param_1[3] = local_e0;
      *param_1 = 0xb;
      return;
    }
  }
  else {
    <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_f0);
LAB_1001f5e88:
    piVar7 = piVar6;
    if (pppppppcVar9 != (char *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_e8,(long)pppppppcVar9 * 0x18,8);
    }
  }
LAB_1001f60bc:
  puVar4 = (ulong *)serde_json::error::Error::fix_position(piVar7,&local_160);
LAB_1001f5904:
  local_f0 = (char *******)0x8000000000000000;
  puStack_e8 = puVar4;
LAB_1001f5910:
  if (local_160 != (char *****)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_158,local_160,1);
  }
  if ((*puVar4 < 2) || (lVar5 = serde_json::de::from_trait(&local_180), lVar5 == 0)) {
    local_160 = &local_1a8;
    local_158 = &<&T_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_198,s_I_has_an_unsupported_recovery_sc_10151d100,&local_160);
    param_1[2] = uStack_190;
    param_1[1] = local_198;
    param_1[3] = local_188;
    *param_1 = 9;
    ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(puVar4);
  }
  else {
    ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(lVar5);
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar5,0x28,8);
    param_1[1] = 0x8000000000000000;
    *param_1 = 0xb;
    ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(puVar4);
  }
  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar4,0x28,8);
  return;
}
