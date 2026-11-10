/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* cleanup @ 100202408 requested 100202408 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::legacy_virtual_auth::cleanup */

void codexmate_lib::core::relay::legacy_virtual_auth::cleanup(undefined8 *param_1,long param_2)

{
  byte bVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  bool bVar4;
  bool bVar5;
  char cVar6;
  int iVar7;
  long *******ppppppplVar8;
  char *pcVar9;
  long *******ppppppplVar10;
  long *******ppppppplVar11;
  long ******pppppplVar12;
  undefined8 uVar13;
  long *******ppppppplVar14;
  undefined8 uVar15;
  undefined8 *puVar16;
  long *******unaff_x24;
  long *******unaff_x25;
  undefined8 *puVar17;
  long lVar18;
  long *******ppppppplVar19;
  undefined1 uVar20;
  long *******unaff_x26;
  ulong unaff_x27;
  undefined1 auVar21 [16];
  long *******local_1b8;
  long *******local_1b0;
  uint local_1a4;
  long local_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  long local_188;
  undefined8 local_180;
  undefined8 uStack_178;
  long local_170;
  undefined8 local_168;
  undefined8 uStack_160;
  long *******local_158;
  undefined1 local_150;
  undefined8 local_148;
  long *******local_140;
  long *******local_138;
  undefined8 local_130;
  long *******ppppppplStack_128;
  long *******local_120;
  long *******ppppppplStack_118;
  long *******local_110;
  long *******ppppppplStack_108;
  long *******local_100;
  long *******ppppppplStack_f8;
  long *******local_f0;
  long *******ppppppplStack_e8;
  undefined8 local_e0;
  long *******local_d8;
  long *******local_d0;
  undefined8 uStack_c8;
  char *local_c0;
  undefined8 uStack_b8;
  undefined1 auStack_69 [9];

  uVar13 = *(undefined8 *)(param_2 + 0x248);
  uVar15 = *(undefined8 *)(param_2 + 0x250);
  std::path::Path::_join
            (&local_1a0,uVar13,uVar15,
             "virtual-auth-marker.jsonapiKeylegacy auth backup is not user-owned auth; backup was preservedlegacy virtual auth backuplegacy virtual auth marker[AiMaMi][legacy-virtual-auth] auth.json is unreadable; preserving marker + backup for a later cleanupcodexmate_lib::core::relay::legacy_virtual_authtools"
             ,0x18);
  std::path::Path::_join
            (&local_188,uVar13,uVar15,
             "api-login-auth-backup.jsonvirtual-auth-marker.jsonapiKeylegacy auth backup is not user-owned auth; backup was preservedlegacy virtual auth backuplegacy virtual auth marker[AiMaMi][legacy-virtual-auth] auth.json is unreadable; preserving marker + backup for a later cleanupcodexmate_lib::core::relay::legacy_virtual_authtools"
             ,0x1a);
  std::path::Path::_join
            (&local_170,uVar13,uVar15,
             "virtual-auth-marker.jsonapiKeylegacy auth backup is not user-owned auth; backup was preservedlegacy virtual auth backuplegacy virtual auth marker[AiMaMi][legacy-virtual-auth] auth.json is unreadable; preserving marker + backup for a later cleanupcodexmate_lib::core::relay::legacy_virtual_authtools"
             ,0x18);
  std::fs::read_to_string::inner(&local_130,local_168,uStack_160);
  ppppppplVar19 = ppppppplStack_128;
  ppppppplVar10 = local_130;
  if (local_130 != (long *******)&DAT_8000000000000000) {
    local_f0 = local_120;
    ppppppplStack_e8 = (long *******)0x0;
    local_e0 = 0;
    local_d8 = ppppppplStack_128;
    local_d0 = local_120;
    local_110 = (long *******)0x0;
    ppppppplStack_108 = (long *******)0x1;
    local_100 = (long *******)0x0;
    ppppppplStack_f8 = ppppppplStack_128;
    uStack_c8 = CONCAT71(uStack_c8._1_7_,0x80);
    if (local_120 != (long *******)0x0) {
      do {
        bVar1 = *(byte *)((long)ppppppplStack_128 + (long)ppppppplStack_e8);
        if (0x20 < bVar1 || (1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) {
          if (bVar1 != 0x5b) {
            if (bVar1 != 0x7b) {
              unaff_x25 = (long *******)serde_json::de::Deserializer<R>::peek_invalid_type(&local_110,auStack_69,&DAT_1018186f0);
              goto LAB_100202a24;
            }
            uStack_c8 = CONCAT71(uStack_c8._1_7_,0x7f);
            ppppppplStack_e8 = (long *******)((long)ppppppplStack_e8 + 1);
            local_158 = (long *******)&local_110;
            local_1a4 = 2;
            local_150 = 1;
            local_1b8 = (long *******)&SUB_00000006;
            local_1b0 = (long *******)0x1013062d8;
            ppppppplVar14 = (long *******)&DAT_8000000000000000;
            goto LAB_10020260c;
          }
          uStack_c8 = CONCAT71(uStack_c8._1_7_,0x7f);
          ppppppplStack_e8 = (long *******)((long)ppppppplStack_e8 + 1);
          local_148 = (long *******)&local_110;
          local_140 = (long *******)CONCAT71(local_140._1_7_,1);
          <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_130,&local_148);
          unaff_x25 = ppppppplStack_128;
          if ((char)local_130 == '\x01') {
LAB_100202950:
            unaff_x27 = 0;
            ppppppplVar14 = (long *******)&DAT_8000000000000000;
          }
          else {
            if (local_130._1_1_ != '\x01') {
              unaff_x25 = (long *******)
                          serde_core::de::Error::invalid_length(0,&PTR_s_struct_LegacyVirtualAuthMarker_w_10130772a_0x322_101817c18,&DAT_101817930);
              goto LAB_100202950;
            }
            <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_string(&local_130,local_148);
            ppppppplVar8 = local_120;
            unaff_x25 = ppppppplStack_128;
            ppppppplVar14 = local_130;
            if (local_130 == (long *******)&DAT_8000000000000000) goto LAB_100202950;
            <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_130,&local_148);
            unaff_x26 = ppppppplVar8;
            if ((char)local_130 == '\x01') {
LAB_100202908:
              ppppppplVar8 = ppppppplStack_128;
              if (ppppppplVar14 != (long *******)0x0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x25,ppppppplVar14,1);
              }
              unaff_x27 = 0;
              ppppppplVar14 = (long *******)&DAT_8000000000000000;
              unaff_x25 = ppppppplVar8;
            }
            else if (local_130._1_1_ == '\x01') {
              <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_bool(&local_130,local_148);
              if ((char)local_130 == '\x01') goto LAB_100202908;
              unaff_x27 = (ulong)local_130 >> 8 & 0xff;
            }
            else {
              unaff_x27 = 0;
            }
          }
          uStack_c8 = CONCAT71(uStack_c8._1_7_,(char)uStack_c8 + '\x01');
          ppppppplVar8 = (long *******)serde_json::de::Deserializer<R>::end_seq(&local_110);
          unaff_x24 = unaff_x25;
          if (ppppppplVar14 != (long *******)&DAT_8000000000000000) goto LAB_100202a04;
          if (ppppppplVar8 == (long *******)0x0) goto LAB_100202a24;
          ::core::ptr::drop_in_place<serde_json::error::ErrorCode>();
          goto LAB_1002029ec;
        }
        ppppppplStack_e8 = (long *******)((long)ppppppplStack_e8 + 1);
      } while (local_120 != ppppppplStack_e8);
    }
    local_130 = (long *******)&DAT_00000005;
    unaff_x24 = (long *******)serde_json::de::Deserializer<R>::peek_error(&local_110,&local_130);
    goto LAB_100202a2c;
  }
  cVar6 = std::io::error::Error::kind(ppppppplStack_128);
  if (cVar6 == '\0') {
    if (((ulong)ppppppplVar19 & 3) == 1) {
      uVar13 = *(undefined8 *)((long)ppppppplVar19 + -1);
      puVar16 = *(undefined8 **)((long)ppppppplVar19 + 7);
      if ((code *)*puVar16 != (code *)0x0) {
        (*(code *)*puVar16)(uVar13);
      }
      if (puVar16[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar13,puVar16[1],puVar16[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)ppppppplVar19 + -1),0x18,8);
    }
    ppppppplVar14 = (long *******)&DAT_8000000000000000;
    lVar18 = 0xb;
  }
  else {
    local_158 = ppppppplVar19;
    local_148 = (long *******)&local_158;
    local_140 = (long *******)<std::io::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_110,s__read_legacy_virtual_auth_marker_10151d4c7,&local_148);
    unaff_x26 = local_100;
    unaff_x24 = ppppppplStack_108;
    ppppppplVar14 = local_110;
    if (((ulong)local_158 & 3) == 1) {
      puVar16 = (undefined8 *)((long)local_158 + -1);
      uVar13 = *puVar16;
      puVar17 = *(undefined8 **)((long)local_158 + 7);
      if ((code *)*puVar17 != (code *)0x0) {
        (*(code *)*puVar17)(uVar13);
      }
      if (puVar17[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar13,puVar17[1],puVar17[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar16,0x18,8);
    }
    lVar18 = 10;
  }
  goto LAB_100202ac4;
LAB_10020260c:
  <serde_json::de::MapAccess<R>as_serde_core::de::MapAccess>::next_key_seed::has_next_key(&local_130,&local_158);
  ppppppplVar8 = local_158;
  ppppppplVar11 = ppppppplStack_128;
  if ((char)local_130 == '\x01') goto LAB_100202988;
  if (local_130._1_1_ != '\x01') {
    if (ppppppplVar14 == (long *******)&DAT_8000000000000000) {
      local_148 = (long *******)0x1013062d8;
      local_140 = (long *******)&SUB_00000006;
      local_130 = (long *******)&local_148;
      ppppppplStack_128 = (long *******)&<&T_as_core::fmt::Display>::fmt;
      ppppppplVar14 = (long *******)&DAT_8000000000000000;
      unaff_x25 = (long *******)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_130);
      unaff_x27 = 0;
    }
    else {
      unaff_x27 = (ulong)(local_1a4 & 1);
    }
    goto LAB_1002029c0;
  }
  local_158[5] = (long ******)((long)local_158[5] + 1);
  local_158[2] = (long ******)0x0;
  <serde_json::read::StrRead_as_serde_json::read::Read>::parse_str(&local_130,local_158 + 3,local_158);
  ppppppplVar11 = ppppppplStack_128;
  if (local_130 != (long *******)0x2) {
    if (local_120 == (long *******)&DAT_00000010) {
      if (*ppppppplStack_128 != (long ******)0x705564656b636162 || ppppppplStack_128[1] != (long ******)0x687475416c616552) goto LAB_10020270c;
      if (local_1a4 != 2) {
        local_1b8 = (long *******)&DAT_00000010;
        local_1b0 = (long *******)&DAT_1013042b0;
        goto LAB_10020323c;
      }
      pppppplVar12 = ppppppplVar8[5];
      if (pppppplVar12 < ppppppplVar8[4]) {
        while( true ) {
          bVar1 = *(byte *)((long)ppppppplVar8[3] + (long)pppppplVar12);
          if (0x3a < bVar1) goto LAB_100203268;
          if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) break;
          pppppplVar12 = (long ******)((long)pppppplVar12 + 1);
          ppppppplVar8[5] = pppppplVar12;
          if (ppppppplVar8[4] == pppppplVar12) goto LAB_1002031dc;
        }
        if ((ulong)bVar1 == 0x3a) {
          ppppppplVar8[5] = (long ******)((long)pppppplVar12 + 1);
          <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_bool(&local_148,ppppppplVar8);
          ppppppplVar11 = local_140;
          if ((char)local_148 == '\x01') goto LAB_100202988;
          local_1a4 = (uint)local_148._1_1_;
          goto LAB_10020260c;
        }
LAB_100203268:
        local_130 = (long *******)&SUB_00000006;
        ppppppplVar11 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar8,&local_130);
      }
      else {
LAB_1002031dc:
        local_130 = (long *******)0x3;
        ppppppplVar11 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar8,&local_130);
      }
    }
    else {
      if ((local_120 == (long *******)&SUB_00000006) && (*(int *)ppppppplStack_128 == 0x4b697061 && *(short *)((long)ppppppplStack_128 + 4) == 0x7965)) {
        unaff_x26 = (long *******)&DAT_8000000000000000;
        if (ppppppplVar14 != (long *******)&DAT_8000000000000000) {
LAB_10020323c:
          local_148 = local_1b0;
          local_140 = local_1b8;
          local_130 = (long *******)&local_148;
          ppppppplStack_128 = (long *******)&<&T_as_core::fmt::Display>::fmt;
          ppppppplVar11 = (long *******)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_15,&local_130);
          goto LAB_100202988;
        }
        pppppplVar12 = ppppppplVar8[5];
        if (ppppppplVar8[4] <= pppppplVar12) {
LAB_1002031b4:
          local_130 = (long *******)0x3;
          unaff_x25 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar8,&local_130);
          goto LAB_1002031d0;
        }
        while( true ) {
          bVar1 = *(byte *)((long)ppppppplVar8[3] + (long)pppppplVar12);
          if (0x3a < bVar1) goto LAB_100203280;
          if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) break;
          pppppplVar12 = (long ******)((long)pppppplVar12 + 1);
          ppppppplVar8[5] = pppppplVar12;
          if (ppppppplVar8[4] == pppppplVar12) goto LAB_1002031b4;
        }
        if ((ulong)bVar1 != 0x3a) {
LAB_100203280:
          local_130 = (long *******)&SUB_00000006;
          unaff_x25 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar8,&local_130);
          goto LAB_1002031d0;
        }
        ppppppplVar8[5] = (long ******)((long)pppppplVar12 + 1);
        <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_string(&local_148,ppppppplVar8);
        ppppppplVar14 = local_148;
        unaff_x25 = local_140;
        unaff_x26 = local_138;
        if (local_148 == (long *******)&DAT_8000000000000000) goto LAB_1002031d0;
        goto LAB_10020260c;
      }
LAB_10020270c:
      pppppplVar12 = ppppppplVar8[5];
      if (pppppplVar12 < ppppppplVar8[4]) {
        while( true ) {
          bVar1 = *(byte *)((long)ppppppplVar8[3] + (long)pppppplVar12);
          if (0x3a < bVar1) goto LAB_1002031f4;
          if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) break;
          pppppplVar12 = (long ******)((long)pppppplVar12 + 1);
          ppppppplVar8[5] = pppppplVar12;
          if (ppppppplVar8[4] == pppppplVar12) goto LAB_1002028ac;
        }
        if ((ulong)bVar1 == 0x3a) {
          ppppppplVar8[5] = (long ******)((long)pppppplVar12 + 1);
          ppppppplVar11 = (long *******)serde_json::de::Deserializer<R>::ignore_value(ppppppplVar8);
          if (ppppppplVar11 != (long *******)0x0) goto LAB_100202988;
          goto LAB_10020260c;
        }
LAB_1002031f4:
        local_130 = (long *******)&SUB_00000006;
        ppppppplVar11 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar8,&local_130);
      }
      else {
LAB_1002028ac:
        local_130 = (long *******)0x3;
        ppppppplVar11 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar8,&local_130);
      }
    }
  }
LAB_100202988:
  if (((ulong)ppppppplVar14 & 0x7fffffffffffffff) == 0) {
    unaff_x27 = 0;
    ppppppplVar14 = (long *******)&DAT_8000000000000000;
    unaff_x25 = ppppppplVar11;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x25,ppppppplVar14,1);
    unaff_x27 = 0;
    ppppppplVar14 = (long *******)&DAT_8000000000000000;
    unaff_x25 = ppppppplVar11;
  }
  goto LAB_1002029c0;
LAB_1002031d0:
  unaff_x27 = 0;
  ppppppplVar14 = (long *******)&DAT_8000000000000000;
  unaff_x26 = (long *******)&DAT_8000000000000000;
LAB_1002029c0:
  uStack_c8 = CONCAT71(uStack_c8._1_7_,(char)uStack_c8 + '\x01');
  ppppppplVar8 = (long *******)serde_json::de::Deserializer<R>::end_map(&local_110);
  unaff_x24 = unaff_x25;
  if (ppppppplVar14 == (long *******)&DAT_8000000000000000) {
    if (ppppppplVar8 != (long *******)0x0) {
      ::core::ptr::drop_in_place<serde_json::error::ErrorCode>();
LAB_1002029ec:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar8,0x28,8);
    }
LAB_100202a24:
    unaff_x24 = (long *******)serde_json::error::Error::fix_position(unaff_x25,&local_110);
LAB_100202a2c:
    ppppppplVar14 = (long *******)&DAT_8000000000000000;
  }
  else {
LAB_100202a04:
    if (ppppppplVar8 != (long *******)0x0) {
      unaff_x25 = ppppppplVar8;
      if (ppppppplVar14 != (long *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x24,ppppppplVar14,1);
      }
      goto LAB_100202a24;
    }
    if (ppppppplStack_e8 < local_f0) {
      do {
        if (0x20 < *(byte *)((long)ppppppplStack_f8 + (long)ppppppplStack_e8) ||
            (1L << ((ulong)*(byte *)((long)ppppppplStack_f8 + (long)ppppppplStack_e8) & 0x3f) & 0x100002600U) == 0) {
          local_130 = (long *******)&DAT_00000016;
          ppppppplVar8 = (long *******)serde_json::de::Deserializer<R>::peek_error(&local_110,&local_130);
          if (ppppppplVar14 != (long *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x24,ppppppplVar14,1);
          }
          ppppppplVar14 = (long *******)&DAT_8000000000000000;
          unaff_x24 = ppppppplVar8;
          break;
        }
        ppppppplStack_e8 = (long *******)((long)ppppppplStack_e8 + 1);
      } while (local_f0 != ppppppplStack_e8);
    }
  }
  if (local_110 != (long *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_108,local_110,1);
  }
  if (ppppppplVar14 == (long *******)&DAT_8000000000000000) {
    local_130 = (long *******)&local_148;
    ppppppplStack_128 = (long *******)<serde_json::error::Error_as_core::fmt::Display>::fmt;
    local_148 = unaff_x24;
    alloc::fmt::format::format_inner(&local_110,s_Ilegacy_virtual_auth_marker_is_i_10151d4f2,&local_130);
    unaff_x26 = local_100;
    unaff_x24 = ppppppplStack_108;
    ppppppplVar14 = local_110;
    ppppppplVar8 = local_148;
    ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(local_148);
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar8,0x28,8);
    lVar18 = 10;
  }
  else {
    lVar18 = 0xb;
  }
  if (ppppppplVar10 != (long *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar19,ppppppplVar10,1);
  }
LAB_100202ac4:
  if (local_170 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_168,local_170,1);
  }
  if (lVar18 != 0xb) {
    *param_1 = 10;
    param_1[1] = ppppppplVar14;
    param_1[2] = unaff_x24;
    param_1[3] = unaff_x26;
    param_1[4] = unaff_x27;
    goto LAB_100202cdc;
  }
  if (ppppppplVar14 == (long *******)&DAT_8000000000000000) {
    *(undefined1 *)(param_1 + 1) = 0;
    *param_1 = 0xb;
    goto LAB_100202cdc;
  }
  uVar13 = *(undefined8 *)(param_2 + 0x20);
  uVar15 = *(undefined8 *)(param_2 + 0x28);
  std::sys::fs::metadata(&local_110,uVar13,uVar15);
  ppppppplVar10 = local_110;
  ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_110,ppppppplStack_108);
  if (((ulong)ppppppplVar10 & 1) == 0) {
    std::fs::read_to_string::inner(&local_148,uVar13,uVar15);
    ppppppplVar19 = local_140;
    ppppppplVar10 = local_148;
    if (local_148 == (long *******)&DAT_8000000000000000) {
      ::core::ptr::drop_in_place<core::result::Result<alloc::string::String,std::io::error::Error>>(0x8000000000000000,local_140);
    }
    else {
      local_110 = local_140;
      ppppppplStack_108 = local_138;
      local_100 = (long *******)0x0;
      ppppppplStack_f8 = (long *******)0x0;
      local_f0 = local_140;
      ppppppplStack_e8 = local_138;
      serde_json::de::from_trait(&local_130,&local_110);
      ppppppplVar8 = ppppppplStack_128;
      if ((char)local_130 != '\x06') {
        ::core::ptr::drop_in_place<serde_json::value::Value>(&local_130);
        if (ppppppplVar10 != (long *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar19,ppppppplVar10,1);
        }
        goto LAB_100202b34;
      }
      ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(ppppppplStack_128);
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar8,0x28,8);
      if (ppppppplVar10 != (long *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar19,ppppppplVar10,1);
      }
    }
    if (1 < log::MAX_LOG_LEVEL_FILTER) {
      local_d8 = (long *******)0x1013063b6;
      local_d0 = (long *******)&DAT_0000002f;
      local_c0 =
      "[AiMaMi][legacy-virtual-auth] auth.json is unreadable; preserving marker + backup for a later cleanupcodexmate_lib::core::relay::legacy_virtual_authtools"
      ;
      uStack_b8 = 0xcb;
      local_110 = (long *******)0x0;
      ppppppplStack_108 = (long *******)0x1013063b6;
      ppppppplStack_f8 = (long *******)0x0;
      local_100 = (long *******)&DAT_0000002f;
      local_f0 = (long *******)0x10151d5a7;
      local_e0 = 2;
      ppppppplStack_e8 = (long *******)&DAT_00000025;
      uStack_c8 = 0x7c00000001;
      <log::__private_api::GlobalLogger_as_log::Log>::log(auStack_69,&local_110);
    }
    *(undefined1 *)(param_1 + 1) = 0;
    *param_1 = 0xb;
    goto joined_r0x000100203180;
  }
LAB_100202b34:
  std::fs::read_to_string::inner(&local_110,uVar13,uVar15);
  ppppppplVar8 = local_100;
  ppppppplVar19 = ppppppplStack_108;
  ppppppplVar10 = local_110;
  if (local_110 == (long *******)&DAT_8000000000000000) {
    ::core::ptr::drop_in_place<core::result::Result<alloc::string::String,std::io::error::Error>>(0x8000000000000000,ppppppplStack_108);
LAB_100202f60:
    std::sys::fs::metadata(&local_110,uVar13,uVar15);
    ppppppplVar10 = local_110;
    ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_110,ppppppplStack_108);
    uVar3 = uStack_178;
    uVar2 = local_180;
    uVar20 = 0;
    if ((ppppppplVar10 == (long *******)0x0) || ((unaff_x27 & 1) == 0)) goto LAB_100202fb0;
    std::sys::fs::metadata(&local_110,local_180,uStack_178);
    ppppppplVar10 = local_110;
    ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_110,ppppppplStack_108);
    if (((ulong)ppppppplVar10 & 1) != 0) {
      uVar20 = 0;
      goto LAB_100202fb0;
    }
    read_user_owned_backup(&local_110,uVar2,uVar3);
    ppppppplVar19 = local_100;
    ppppppplVar10 = ppppppplStack_108;
    if (local_110 == (long *******)&DAT_0000000b) {
      auVar21 = atomic_write::write_atomic_private_report(uVar13,uVar15,local_100);
      if ((auVar21._0_8_ & 1) == 0) {
        atomic_write::Publication::acknowledge(auVar21._8_8_);
        uVar20 = 1;
        if (ppppppplVar10 != (long *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar19,ppppppplVar10,1);
        }
        goto LAB_100202fb0;
      }
      cleanup::__closure__(&local_110);
LAB_10020305c:
      param_1[5] = ppppppplStack_e8;
      param_1[4] = local_f0;
      param_1[7] = local_d8;
      param_1[6] = local_e0;
      param_1[9] = uStack_c8;
      param_1[8] = local_d0;
      param_1[0xb] = uStack_b8;
      param_1[10] = local_c0;
      param_1[1] = ppppppplStack_108;
      *param_1 = local_110;
      param_1[3] = ppppppplStack_f8;
      param_1[2] = local_100;
      if (ppppppplVar10 != (long *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar19,ppppppplVar10,1);
      }
    }
    else {
LAB_100203168:
      param_1[5] = ppppppplStack_e8;
      param_1[4] = local_f0;
      param_1[7] = local_d8;
      param_1[6] = local_e0;
      param_1[9] = uStack_c8;
      param_1[8] = local_d0;
      param_1[0xb] = uStack_b8;
      param_1[10] = local_c0;
      param_1[2] = local_100;
      param_1[3] = ppppppplStack_f8;
      *param_1 = local_110;
      param_1[1] = ppppppplStack_108;
    }
  }
  else {
    local_110 = ppppppplStack_108;
    ppppppplStack_108 = local_100;
    local_100 = (long *******)0x0;
    ppppppplStack_f8 = (long *******)0x0;
    local_f0 = ppppppplVar19;
    ppppppplStack_e8 = ppppppplVar8;
    serde_json::de::from_trait(&local_130,&local_110);
    ppppppplVar8 = ppppppplStack_128;
    if ((char)local_130 == '\x06') {
      ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(ppppppplStack_128);
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar8,0x28,8);
      if (ppppppplVar10 != (long *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar19,ppppppplVar10,1);
      }
      goto LAB_100202f60;
    }
    ppppppplStack_108 = ppppppplStack_128;
    local_110 = local_130;
    ppppppplStack_f8 = ppppppplStack_118;
    local_100 = local_120;
    pcVar9 = (char *)<str_as_serde_json::value::index::Index>::index_into("OPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens",0xe,&local_110);
    if (((pcVar9 == (char *)0x0) || (*pcVar9 != '\x03')) || (*(long ********)(pcVar9 + 0x18) != unaff_x26)) {
      bVar4 = false;
    }
    else {
      iVar7 = _memcmp(*(void **)(pcVar9 + 0x10),unaff_x24,(size_t)unaff_x26);
      bVar4 = iVar7 == 0;
    }
    pcVar9 = (char *)<str_as_serde_json::value::index::Index>::index_into("tokenslast_refreshAuthTokens",6,&local_110);
    if (pcVar9 == (char *)0x0) {
      bVar5 = true;
    }
    else {
      bVar5 = *pcVar9 == '\0';
    }
    ::core::ptr::drop_in_place<serde_json::value::Value>(&local_110);
    if (ppppppplVar10 != (long *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar19,ppppppplVar10,1);
    }
    uVar3 = uStack_178;
    uVar2 = local_180;
    if (!(bool)(bVar4 & bVar5)) goto LAB_100202f60;
    if ((unaff_x27 & 1) != 0) {
      std::sys::fs::metadata(&local_110,local_180,uStack_178);
      ppppppplVar10 = local_110;
      ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_110,ppppppplStack_108);
      if (((ulong)ppppppplVar10 & 1) != 0) goto LAB_100202eac;
      read_user_owned_backup(&local_110,uVar2,uVar3);
      ppppppplVar19 = local_100;
      ppppppplVar10 = ppppppplStack_108;
      if (local_110 == (long *******)&DAT_0000000b) {
        auVar21 = atomic_write::write_atomic_private_report(uVar13,uVar15,local_100);
        if ((auVar21._0_8_ & 1) == 0) {
          atomic_write::Publication::acknowledge(auVar21._8_8_);
          uVar20 = 1;
          if (ppppppplVar10 != (long *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar19,ppppppplVar10,1);
          }
          goto LAB_100202fb0;
        }
        cleanup::__closure__(&local_110);
        goto LAB_10020305c;
      }
      goto LAB_100203168;
    }
LAB_100202eac:
    ppppppplVar10 = (long *******)std::sys::fs::remove_file(uVar13,uVar15);
    if (ppppppplVar10 != (long *******)0x0) {
      local_130 = (long *******)&local_148;
      ppppppplStack_128 = (long *******)<std::io::error::Error_as_core::fmt::Display>::fmt;
      local_148 = ppppppplVar10;
      alloc::fmt::format::format_inner(&local_110,s__remove_legacy_virtual_auth_json_10151d5cd,&local_130);
      ppppppplStack_128 = ppppppplStack_108;
      local_130 = local_110;
      local_120 = local_100;
      if (((ulong)local_148 & 3) == 1) {
        puVar16 = (undefined8 *)((long)local_148 + -1);
        uVar13 = *puVar16;
        puVar17 = *(undefined8 **)((long)local_148 + 7);
        if ((code *)*puVar17 != (code *)0x0) {
          (*(code *)*puVar17)(uVar13);
        }
        if (puVar17[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar13,puVar17[1],puVar17[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar16,0x18,8);
      }
      param_1[2] = ppppppplStack_128;
      param_1[1] = local_130;
      param_1[3] = local_120;
      *param_1 = 10;
      goto joined_r0x000100203180;
    }
    uVar20 = 1;
LAB_100202fb0:
    remove_file_if_exists
              (&local_110,local_180,uStack_178,
               "legacy virtual auth backuplegacy virtual auth marker[AiMaMi][legacy-virtual-auth] auth.json is unreadable; preserving marker + backup for a later cleanupcodexmate_lib::core::relay::legacy_virtual_authtools"
              );
    if ((local_110 == (long *******)&DAT_0000000b) &&
       (remove_file_if_exists
                  (&local_110,local_198,uStack_190,
                   "legacy virtual auth marker[AiMaMi][legacy-virtual-auth] auth.json is unreadable; preserving marker + backup for a later cleanupcodexmate_lib::core::relay::legacy_virtual_authtools"
                  ), local_110 == (long *******)&DAT_0000000b)) {
      *(undefined1 *)(param_1 + 1) = uVar20;
      *param_1 = 0xb;
    }
    else {
      param_1[5] = ppppppplStack_e8;
      param_1[4] = local_f0;
      param_1[7] = local_d8;
      param_1[6] = local_e0;
      param_1[9] = uStack_c8;
      param_1[8] = local_d0;
      param_1[0xb] = uStack_b8;
      param_1[10] = local_c0;
      param_1[1] = ppppppplStack_108;
      *param_1 = local_110;
      param_1[3] = ppppppplStack_f8;
      param_1[2] = local_100;
    }
  }
joined_r0x000100203180:
  if (ppppppplVar14 != (long *******)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x24,ppppppplVar14,1);
  }
LAB_100202cdc:
  if (local_188 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_180,local_188,1);
  }
  if (local_1a0 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_198,local_1a0,1);
  }
  return;
}
