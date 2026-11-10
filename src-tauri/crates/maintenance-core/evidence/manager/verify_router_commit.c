/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* verify_router_commit @ 100205568 requested 100205568 */

/* WARNING: Removing unreachable block (ram,0x0001002062a8) */
/* WARNING: Removing unreachable block (ram,0x000100206404) */
/* WARNING: Removing unreachable block (ram,0x0001002062e0) */
/* WARNING: Removing unreachable block (ram,0x0001002063a4) */
/* WARNING: Removing unreachable block (ram,0x00010020636c) */
/* WARNING: Removing unreachable block (ram,0x0001002064f8) */
/* WARNING: Removing unreachable block (ram,0x0001002064c0) */
/* WARNING: Removing unreachable block (ram,0x00010020661c) */
/* WARNING: Removing unreachable block (ram,0x0001002065bc) */
/* WARNING: Removing unreachable block (ram,0x000100206584) */
/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::storage::verify_router_commit */

void codexmate_lib::core::relay::storage::verify_router_commit(undefined8 *param_1,long param_2,ulong param_3)

{
  byte bVar1;
  undefined1 uVar2;
  byte bVar3;
  undefined2 uVar4;
  undefined3 uVar5;
  undefined4 uVar6;
  bool bVar7;
  bool bVar8;
  byte bVar9;
  bool bVar10;
  char cVar11;
  uint uVar12;
  undefined8 *puVar13;
  long *plVar14;
  ulong uVar15;
  long *plVar16;
  undefined **ppuVar17;
  undefined8 uVar18;
  byte bVar19;
  int iVar20;
  uint uVar21;
  byte bVar22;
  uint uVar23;
  undefined *puVar24;
  long lVar25;
  undefined *puVar26;
  long *local_1b0;
  long *local_1a8;
  uint local_19c;
  undefined8 local_198;
  char *local_190;
  ushort local_184;
  ulong local_178;
  byte local_164;
  undefined8 local_160;
  undefined3 uStack_14b;
  undefined2 local_145;
  undefined6 uStack_143;
  undefined8 local_138;
  ulong local_130;
  long local_128;
  long lStack_120;
  long local_118;
  long local_110;
  long local_108;
  long local_100;
  long local_f8;
  long lStack_f0;
  undefined8 local_e8;
  int local_e0;
  long local_d8;
  long local_d0;
  long local_c8;
  byte local_c0;
  byte local_bf;
  char local_b8;
  undefined2 local_b0;
  undefined1 local_ae;
  undefined2 local_a8;
  undefined1 local_a6;
  undefined2 local_a0;
  undefined1 local_9e;
  undefined2 local_9c;
  undefined1 local_9a;
  byte local_98;
  byte bStack_97;
  undefined2 uStack_96;
  undefined1 uStack_94;
  undefined1 uStack_93;
  undefined1 uStack_92;
  undefined1 uStack_91;
  undefined2 uStack_90;
  undefined2 uStack_8e;
  undefined4 uStack_8c;
  ushort local_88;
  undefined6 uStack_86;
  undefined8 local_80;
  undefined8 local_78;
  undefined1 auStack_69 [9];

  local_98 = 0;
  bStack_97 = 0;
  uStack_96 = 0;
  uStack_94 = 0xb6;
  uStack_93 = 1;
  uStack_91 = 0;
  uStack_90 = 0;
  uStack_8e = 0;
  uStack_92 = 1;
  std::fs::OpenOptions::_open(&local_138,&local_98,*(undefined8 *)(param_2 + 0x260),*(undefined8 *)(param_2 + 0x268));
  if ((int)local_138 == 1) {
    cVar11 = std::io::error::Error::kind(local_130);
    if (((cVar11 == '\0') && ((*(byte *)(param_3 + 0x16d) & 1) == 0)) && ((*(byte *)(param_3 + 0x16c) & 1) == 0)) {
      *param_1 = 0xb;
      if ((local_130 & 3) != 1) {
        return;
      }
      uVar18 = *(undefined8 *)(local_130 - 1);
      puVar13 = *(undefined8 **)(local_130 + 7);
      if ((code *)*puVar13 != (code *)0x0) {
        (*(code *)*puVar13)(uVar18);
      }
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar13 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x8b,1);
      if (puVar13 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x8b);
      }
      puVar13[0xd] = 0xe7a59fe680a3e6b7;
      puVar13[0xc] = 0xafe89bbcefa585e5;
      puVar13[0xf] = 0x87e9b6b9e5b6bbe4;
      puVar13[0xe] = 0x8796e68180e6b68a;
      *(undefined8 *)((long)puVar13 + 0x83) = 0x694d614d694120af;
      *(undefined8 *)((long)puVar13 + 0x7b) = 0x90e58d87e9b6b9e5;
      puVar13[5] = 0x989be781a3e7b9af;
      puVar13[4] = 0xe5b8a0e695b3e6a0;
      puVar13[7] = 0xe58eb8e4a789e69e;
      puVar13[6] = 0x9be5b194e7afb7e8;
      puVar13[9] = 0xb7e58cbcef8fbce5;
      puVar13[8] = 0xa1a8e68d89e593bd;
      puVar13[0xb] = 0x9986e5adbbe7a7bb;
      puVar13[10] = 0xe7a2ade69c81e5b2;
      puVar13[1] = 0x54535f54494d4d4f;
      *puVar13 = 0x435f524554554f52;
      puVar13[3] = 0x97e6203a5443494c;
      puVar13[2] = 0x464e4f435f455441;
      param_1[1] = 0x8b;
      *param_1 = 10;
      param_1[2] = puVar13;
      param_1[3] = 0x8b;
      if ((local_130 & 3) != 1) {
        return;
      }
      uVar18 = *(undefined8 *)(local_130 - 1);
      puVar13 = *(undefined8 **)(local_130 + 7);
      if ((code *)*puVar13 != (code *)0x0) {
        (*(code *)*puVar13)(uVar18);
      }
    }
    if (puVar13[1] != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar18,puVar13[1],puVar13[2]);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130 - 1,0x18,8);
    return;
  }
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  local_108 = __RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x2000,1);
  if (local_108 == 0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x2000);
  }
  local_f8 = 0;
  lStack_f0 = 0;
  local_e8 = 0;
  local_100 = 0x2000;
  local_e0 = local_138._4_4_;
  local_d0 = 0;
  local_c8 = 0;
  local_d8 = 1;
  local_c0 = 0;
  local_138 = 0;
  local_130 = 1;
  local_b8 = -0x80;
  lStack_120 = -0x8000000000000000;
  local_128 = 0;
  do {
    if (lStack_f0 == local_f8) {
      std::io::uninlined_slow_read_byte(&local_98,&local_108);
      if ((local_98 == 2) || ((local_98 & 1) != 0)) {
        if (local_98 == 2) {
          local_98 = 5;
          bStack_97 = 0;
          uStack_96 = 0;
          uStack_94 = 0;
          uStack_93 = 0;
          uStack_92 = 0;
          uStack_91 = 0;
          uVar18 = serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
        }
        else {
          uVar18 = serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
        }
        goto LAB_100206fdc;
      }
    }
    else {
      bStack_97 = *(byte *)(local_108 + local_f8);
      local_f8 = local_f8 + 1;
      local_98 = 0;
    }
    local_d0 = local_d0 + 1;
    if (bStack_97 == 10) {
      local_c8 = local_d0 + local_c8;
      local_d0 = 0;
      local_d8 = local_d8 + 1;
      uVar15 = 10;
      local_c0 = 1;
      local_bf = 10;
      goto LAB_1002057a4;
    }
    local_c0 = 1;
    local_bf = bStack_97;
    do {
      uVar12 = (uint)local_bf;
      uVar15 = (ulong)local_bf;
      if (0x20 < local_bf) {
LAB_100205800:
        lVar25 = local_110;
        if (uVar12 != 0x5b) {
          if (uVar12 != 0x7b) {
            plVar16 = (long *)serde_json::de::Deserializer<R>::peek_invalid_type(&local_138,auStack_69,&DAT_101818910);
            goto LAB_100206fd4;
          }
          local_b8 = local_b8 + -1;
          if (local_b8 == '\0') {
            local_98 = 0x18;
            bStack_97 = 0;
            uStack_96 = 0;
            uStack_94 = 0;
            uStack_93 = 0;
            uStack_92 = 0;
            uStack_91 = 0;
            uVar18 = serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
            goto LAB_100206fdc;
          }
          local_c0 = 0;
          if (lStack_120 != -0x8000000000000000) {
            if (local_110 == lStack_120) {
              alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
            }
            *(undefined1 *)(local_118 + lVar25) = 0x7b;
            local_110 = lVar25 + 1;
          }
          bVar19 = 2;
          local_198 = 0xd;
          local_190 =
          "schemaVersionupdatedAtitemsaccountKeycapturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
          ;
          local_178 = 0xd;
          bVar10 = true;
          local_160 = 2;
          bVar3 = 2;
          bVar1 = 0;
          uVar12 = 2;
          goto LAB_1002058c8;
        }
        local_b8 = local_b8 + -1;
        if (local_b8 == '\0') {
          local_98 = 0x18;
          bStack_97 = 0;
          uStack_96 = 0;
          uStack_94 = 0;
          uStack_93 = 0;
          uStack_92 = 0;
          uStack_91 = 0;
          uVar18 = serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
          goto LAB_100206fdc;
        }
        local_c0 = 0;
        if (lStack_120 != -0x8000000000000000) {
          if (local_110 == lStack_120) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
          }
          *(undefined1 *)(local_118 + lVar25) = 0x5b;
          local_110 = lVar25 + 1;
        }
        local_80 = (char *)&local_138;
        local_78 = CONCAT71(local_78._1_7_,1);
        <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_98,&local_80);
        uVar15 = param_3;
        if (local_98 == 1) goto LAB_1002067f8;
        if (bStack_97 != 1) {
          uVar18 = 0;
          goto LAB_100206a08;
        }
        <core::marker::PhantomData<T>as_serde_core::de::DeserializeSeed>::deserialize(&local_98,local_80);
        if (CONCAT22(uStack_96,CONCAT11(bStack_97,local_98)) == 1) {
LAB_1002067f8:
          local_184 = (ushort)uVar15;
          local_1b0 = (long *)CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90));
          bVar1 = local_164;
LAB_100206a20:
          bVar19 = 2;
        }
        else {
          uVar6 = CONCAT13(uStack_91,CONCAT12(uStack_92,CONCAT11(uStack_93,uStack_94)));
          <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_98,&local_80);
          if (local_98 == 1) goto LAB_1002067f8;
          if (bStack_97 != 1) {
LAB_100206e1c:
            uVar18 = 1;
LAB_100206a08:
            local_184 = (ushort)uVar15;
            local_1b0 = (long *)serde_core::de::Error::invalid_length(uVar18,&PTR_s_struct_Receipt_with_4_elementsst_10130772a_0x35e_101817c28,&DAT_101817930);
            bVar1 = local_164;
            goto LAB_100206a20;
          }
          ::core::option::Option<T>>::deserialize(&local_98,local_80);
          bVar19 = bStack_97;
          if (local_98 == 1) goto LAB_1002067f8;
          uVar5 = CONCAT12(uStack_94,uStack_96);
          local_9c = CONCAT11(uStack_92,uStack_93);
          local_9a = uStack_91;
          if (bStack_97 == 2) goto LAB_100206e1c;
          local_160 = CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90));
          uVar15 = (ulong)local_88;
          <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_98,&local_80);
          if (local_98 == 1) goto LAB_1002067f8;
          if (bStack_97 != 1) {
            uVar18 = 2;
            goto LAB_100206a08;
          }
          <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_bool(&local_98,local_80);
          bVar1 = bStack_97;
          if ((local_98 == 1) ||
             (<serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_98,&local_80), local_98 == 1))
          goto LAB_1002067f8;
          if (bStack_97 != 1) {
            uVar18 = 3;
            goto LAB_100206a08;
          }
          <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_bool(&local_98,local_80);
          local_164 = bVar1;
          if (local_98 == 1) goto LAB_1002067f8;
          local_b0 = local_9c;
          local_ae = local_9a;
          local_1b0 = (long *)CONCAT35(uVar5,CONCAT14(bVar19,uVar6));
          bVar19 = bStack_97;
          local_184 = local_88;
        }
        local_b8 = local_b8 + '\x01';
        goto LAB_100206a54;
      }
LAB_1002057a4:
      lVar25 = local_110;
      uVar12 = (uint)uVar15;
      if ((1L << (uVar15 & 0x3f) & 0x100002600U) == 0) goto LAB_100205800;
      local_c0 = 0;
      if (lStack_120 == -0x8000000000000000) break;
      if (local_110 == lStack_120) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
      }
      *(char *)(local_118 + lVar25) = (char)uVar15;
      local_110 = lVar25 + 1;
    } while ((local_c0 & 1) != 0);
  } while( true );
LAB_1002058c8:
  if ((local_c0 & 1) == 0) {
    if (lStack_f0 == local_f8) {
      std::io::uninlined_slow_read_byte(&local_98,&local_108);
      if ((local_98 == 2) || ((local_98 & 1) != 0)) {
        if (local_98 == 2) {
          local_98 = 3;
          bStack_97 = 0;
          uStack_96 = 0;
          uStack_94 = 0;
          uStack_93 = 0;
          uStack_92 = 0;
          uStack_91 = 0;
          plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
        }
        else {
          plVar16 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
        }
        goto LAB_100206c8c;
      }
      local_d0 = local_d0 + 1;
      if (bStack_97 == 10) goto LAB_100205984;
LAB_100205950:
      uVar15 = (ulong)bStack_97;
    }
    else {
      bStack_97 = *(byte *)(local_108 + local_f8);
      local_f8 = local_f8 + 1;
      local_98 = 0;
      local_d0 = local_d0 + 1;
      if (bStack_97 != 10) goto LAB_100205950;
LAB_100205984:
      local_c8 = local_d0 + local_c8;
      local_d0 = 0;
      local_d8 = local_d8 + 1;
      uVar15 = 10;
    }
    local_c0 = 1;
    uVar21 = (uint)uVar15;
    local_bf = (byte)uVar15;
    if (0x20 < uVar21 || (1L << (uVar15 & 0x3f) & 0x100002600U) == 0) goto LAB_1002059c0;
LAB_1002058ec:
    bVar9 = local_bf;
    lVar25 = local_110;
    local_c0 = 0;
    if (lStack_120 != -0x8000000000000000) {
      if (local_110 == lStack_120) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
      }
      *(byte *)(local_118 + lVar25) = bVar9;
      local_110 = lVar25 + 1;
    }
    goto LAB_1002058c8;
  }
  uVar21 = (uint)local_bf;
  if (local_bf < 0x21 && (1L << ((ulong)local_bf & 0x3f) & 0x100002600U) != 0) goto LAB_1002058ec;
LAB_1002059c0:
  lVar25 = local_110;
  if (uVar21 == 0x7d) {
    if (bVar1 == 0) {
      local_80 =
      "schemaVersionupdatedAtitemsaccountKeycapturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
      ;
      local_78 = 0xd;
      local_98 = (byte)&local_80;
      bStack_97 = (byte)((ulong)&local_80 >> 8);
      uStack_96 = (undefined2)((ulong)&local_80 >> 0x10);
      uStack_94 = (undefined1)((ulong)&local_80 >> 0x20);
      uStack_93 = (undefined1)((ulong)&local_80 >> 0x28);
      uStack_92 = (undefined1)((ulong)&local_80 >> 0x30);
      uStack_91 = (undefined1)((ulong)&local_80 >> 0x38);
      uStack_90 = 0xd6c0;
      uStack_8e = 0x7d;
      uStack_8c = 1;
      plVar16 = (long *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_98);
      goto LAB_100206c8c;
    }
    if (uVar12 == 2) {
      local_178 = 0;
      uVar15 = 0;
    }
    else {
      local_178 = local_178 << 0x28;
      uVar15 = (ulong)uVar12 << 0x20;
      local_9c = local_a0;
      local_9a = local_9e;
    }
    if (bVar3 == 2) {
      local_80 = &DAT_10130651a;
      local_78 = 0x12;
      local_98 = (byte)&local_80;
      bStack_97 = (byte)((ulong)&local_80 >> 8);
      uStack_96 = (undefined2)((ulong)&local_80 >> 0x10);
      uStack_94 = (undefined1)((ulong)&local_80 >> 0x20);
      uStack_93 = (undefined1)((ulong)&local_80 >> 0x28);
      uStack_92 = (undefined1)((ulong)&local_80 >> 0x30);
      uStack_91 = (undefined1)((ulong)&local_80 >> 0x38);
      uStack_90 = 0xd6c0;
      uStack_8e = 0x7d;
      uStack_8c = 1;
      plVar16 = (long *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_98);
      goto LAB_100206c8c;
    }
    if (bVar19 == 2) {
      local_80 = &DAT_10130652c;
      local_78 = 0x18;
      local_98 = (byte)&local_80;
      bStack_97 = (byte)((ulong)&local_80 >> 8);
      uStack_96 = (undefined2)((ulong)&local_80 >> 0x10);
      uStack_94 = (undefined1)((ulong)&local_80 >> 0x20);
      uStack_93 = (undefined1)((ulong)&local_80 >> 0x28);
      uStack_92 = (undefined1)((ulong)&local_80 >> 0x30);
      uStack_91 = (undefined1)((ulong)&local_80 >> 0x38);
      uStack_90 = 0xd6c0;
      uStack_8e = 0x7d;
      uStack_8c = 1;
      plVar16 = (long *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_98);
      goto LAB_100206c8c;
    }
    local_a8 = local_9c;
    local_a6 = local_9a;
    bVar1 = bVar3 & 1;
    bVar19 = bVar19 & 1;
    local_1b0 = (long *)(local_178 | uVar15 | (ulong)local_19c);
    goto LAB_100206c90;
  }
  if (!bVar10) {
    if (uVar21 == 0x2c) {
      local_c0 = 0;
      if (lStack_120 == -0x8000000000000000) goto LAB_100205a18;
      if (local_110 == lStack_120) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
      }
      *(undefined1 *)(local_118 + lVar25) = 0x2c;
      do {
        local_110 = lVar25 + 1;
LAB_100205a18:
        do {
          if ((local_c0 & 1) == 0) {
            if (lStack_f0 == local_f8) {
              std::io::uninlined_slow_read_byte(&local_98,&local_108);
              if ((local_98 == 2) || ((local_98 & 1) != 0)) {
                if (local_98 == 2) {
                  local_98 = 5;
                  bStack_97 = 0;
                  uStack_96 = 0;
                  uStack_94 = 0;
                  uStack_93 = 0;
                  uStack_92 = 0;
                  uStack_91 = 0;
                  plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
                }
                else {
                  plVar16 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
                }
                goto LAB_100206c8c;
              }
              local_d0 = local_d0 + 1;
              if (bStack_97 == 10) goto LAB_100205a98;
LAB_100205a64:
              uVar21 = (uint)bStack_97;
            }
            else {
              bStack_97 = *(byte *)(local_108 + local_f8);
              local_f8 = local_f8 + 1;
              local_98 = 0;
              local_d0 = local_d0 + 1;
              if (bStack_97 != 10) goto LAB_100205a64;
LAB_100205a98:
              local_c8 = local_d0 + local_c8;
              local_d0 = 0;
              local_d8 = local_d8 + 1;
              uVar21 = 10;
            }
            local_c0 = 1;
            local_bf = (byte)uVar21;
          }
          else {
            uVar21 = (uint)local_bf;
          }
          lVar25 = local_110;
          if (0x22 < uVar21) {
LAB_100207450:
            if (uVar21 == 0x7d) {
              local_98 = 0x15;
              bStack_97 = 0;
              uStack_96 = 0;
              uStack_94 = 0;
              uStack_93 = 0;
              uStack_92 = 0;
              uStack_91 = 0;
              plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
            }
            else {
              local_98 = 0x11;
              bStack_97 = 0;
              uStack_96 = 0;
              uStack_94 = 0;
              uStack_93 = 0;
              uStack_92 = 0;
              uStack_91 = 0;
              plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
            }
            goto LAB_100206c8c;
          }
          if ((1L << ((ulong)uVar21 & 0x3f) & 0x100002600U) == 0) {
            if ((ulong)uVar21 != 0x22) goto LAB_100207450;
            goto LAB_100205b10;
          }
          local_c0 = 0;
        } while (lStack_120 == -0x8000000000000000);
        if (local_110 == lStack_120) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
        }
        *(char *)(local_118 + lVar25) = (char)uVar21;
      } while( true );
    }
    local_98 = 8;
    bStack_97 = 0;
    uStack_96 = 0;
    uStack_94 = 0;
    uStack_93 = 0;
    uStack_92 = 0;
    uStack_91 = 0;
    plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
    goto LAB_100206c8c;
  }
  if (uVar21 != 0x22) {
    local_98 = 0x11;
    bStack_97 = 0;
    uStack_96 = 0;
    uStack_94 = 0;
    uStack_93 = 0;
    uStack_92 = 0;
    uStack_91 = 0;
    plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
    goto LAB_100206c8c;
  }
LAB_100205b10:
  lVar25 = local_110;
  local_c0 = 0;
  if (lStack_120 != -0x8000000000000000) {
    if (local_110 == lStack_120) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
    }
    *(undefined1 *)(local_118 + lVar25) = 0x22;
    local_110 = lVar25 + 1;
  }
  local_128 = 0;
  <serde_json::read::IoRead<R>as_serde_json::read::Read>::parse_str(&local_98,&lStack_120,&local_138);
  plVar16 = (long *)CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90));
  if (CONCAT17(uStack_91,CONCAT16(uStack_92,CONCAT15(uStack_93,CONCAT14(uStack_94,CONCAT22(uStack_96,CONCAT11(bStack_97,local_98)))))) == 2) goto LAB_100206c8c;
  lVar25 = CONCAT62(uStack_86,local_88);
  if (lVar25 < 0x12) {
    if (lVar25 == 0xd) {
      if (*plVar16 != 0x6556616d65686373 || *(long *)((long)plVar16 + 5) != 0x6e6f697372655661) goto LAB_100205d14;
      if (bVar1 == 0) {
        plVar16 = (long *)serde_json::de::Deserializer<R>::parse_object_colon(&local_138);
        if (plVar16 != (long *)0x0) goto LAB_100206c8c;
        <core::marker::PhantomData<T>as_serde_core::de::DeserializeSeed>::deserialize(&local_98,&local_138);
        if (CONCAT22(uStack_96,CONCAT11(bStack_97,local_98)) == 1) goto LAB_1002073ec;
        bVar10 = false;
        local_19c = CONCAT13(uStack_91,CONCAT12(uStack_92,CONCAT11(uStack_93,uStack_94)));
        bVar1 = 1;
        goto LAB_1002058c8;
      }
    }
    else {
      if ((lVar25 != 0xe) || (*plVar16 != 0x6f43726574756f72 || *(long *)((long)plVar16 + 6) != 0x644974696d6d6f43)) {
LAB_100205d14:
        plVar16 = (long *)serde_json::de::Deserializer<R>::parse_object_colon(&local_138);
        if (plVar16 == (long *)0x0) {
          iVar20 = 0;
          local_128 = 0;
          puVar26 = &DAT_10130651a;
LAB_100205d40:
          while ((local_c0 & 1) == 0) {
            if (lStack_f0 == local_f8) {
              std::io::uninlined_slow_read_byte(&local_98,&local_108);
              if ((local_98 == 2) || ((local_98 & 1) != 0)) {
                if (local_98 == 2) {
                  local_98 = 5;
                  bStack_97 = 0;
                  uStack_96 = 0;
                  uStack_94 = 0;
                  uStack_93 = 0;
                  uStack_92 = 0;
                  uStack_91 = 0;
                  plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
                }
                else {
                  plVar16 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
                }
                goto LAB_100206c8c;
              }
            }
            else {
              bStack_97 = *(byte *)(local_108 + local_f8);
              local_f8 = local_f8 + 1;
              local_98 = 0;
            }
            local_d0 = local_d0 + 1;
            if (bStack_97 == 10) {
              local_c8 = local_d0 + local_c8;
              local_d0 = 0;
              local_d8 = local_d8 + 1;
              puVar24 = (undefined *)0xa;
              local_c0 = 1;
              local_bf = 10;
            }
            else {
              puVar24 = (undefined *)(ulong)bStack_97;
              local_c0 = 1;
              local_bf = bStack_97;
              if (0x2c < bStack_97) goto LAB_100205e40;
            }
LAB_100205d54:
            lVar25 = local_110;
            if (0x22 < (uint)puVar24) {
LAB_100206058:
              if ((int)puVar24 - 0x30U < 10) {
                plVar16 = (long *)serde_json::de::Deserializer<R>::ignore_integer(&local_138);
                goto LAB_10020606c;
              }
              local_98 = 10;
              bStack_97 = 0;
              uStack_96 = 0;
              uStack_94 = 0;
              uStack_93 = 0;
              uStack_92 = 0;
              uStack_91 = 0;
              plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
              goto LAB_100206c8c;
            }
            if ((1L << ((ulong)puVar24 & 0x3f) & 0x100002600U) == 0) {
              if (puVar24 == (undefined *)0x22) {
                local_c0 = 0;
                if (lStack_120 != -0x8000000000000000) {
                  if (local_110 == lStack_120) {
                    alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
                  }
                  *(undefined1 *)(local_118 + lVar25) = 0x22;
                  local_110 = lVar25 + 1;
                }
                plVar16 = (long *)<serde_json::read::IoRead<R>as_serde_json::read::Read>::ignore_str(&lStack_120);
                goto LAB_10020606c;
              }
              goto LAB_100206058;
            }
            local_c0 = 0;
            if (lStack_120 != -0x8000000000000000) {
              if (local_110 == lStack_120) {
                alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
              }
              *(char *)(local_118 + lVar25) = (char)puVar24;
              local_110 = lVar25 + 1;
            }
          }
          puVar24 = (undefined *)(ulong)local_bf;
          if (local_bf < 0x2d) goto LAB_100205d54;
LAB_100205e40:
          lVar25 = local_110;
          uVar21 = (uint)puVar24;
          if (uVar21 < 0x6e) {
            if (uVar21 == 0x2d) {
              local_c0 = 0;
              if (lStack_120 != -0x8000000000000000) {
                if (local_110 == lStack_120) {
                  alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
                }
                *(undefined1 *)(local_118 + lVar25) = 0x2d;
                local_110 = lVar25 + 1;
              }
              plVar16 = (long *)serde_json::de::Deserializer<R>::ignore_integer(&local_138);
              goto LAB_10020606c;
            }
            if (uVar21 == 0x5b) {
LAB_100205ec8:
              alloc::vec::Vec<T,A>::extend_trusted(&local_138,iVar20,puVar26);
              bVar22 = local_bf;
              bVar9 = local_c0;
              lVar25 = local_110;
              bVar10 = false;
              local_c0 = 0;
              if ((bVar9 == 1) && (lStack_120 != -0x8000000000000000)) {
                if (local_110 == lStack_120) {
                  alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
                }
                bVar10 = false;
                *(byte *)(local_118 + lVar25) = bVar22;
                local_110 = lVar25 + 1;
              }
              goto LAB_1002060b8;
            }
            if (uVar21 == 0x66) {
              local_c0 = 0;
              if (lStack_120 != -0x8000000000000000) {
                if (local_110 == lStack_120) {
                  alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
                }
                *(undefined1 *)(local_118 + lVar25) = 0x66;
                local_110 = lVar25 + 1;
              }
              plVar16 = (long *)serde_json::de::Deserializer<R>::parse_ident(&local_138,&DAT_1013082a0,4);
              goto LAB_10020606c;
            }
            goto LAB_100206058;
          }
          if (uVar21 == 0x6e) {
            local_c0 = 0;
            if (lStack_120 != -0x8000000000000000) {
              if (local_110 == lStack_120) {
                alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
              }
              *(undefined1 *)(local_118 + lVar25) = 0x6e;
              local_110 = lVar25 + 1;
            }
            plVar16 = (long *)serde_json::de::Deserializer<R>::parse_ident(&local_138,&BYTE_1013045de,3);
          }
          else {
            if (uVar21 != 0x74) {
              if (uVar21 != 0x7b) goto LAB_100206058;
              goto LAB_100205ec8;
            }
            local_c0 = 0;
            if (lStack_120 != -0x8000000000000000) {
              if (local_110 == lStack_120) {
                alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
              }
              *(undefined1 *)(local_118 + lVar25) = 0x74;
              local_110 = lVar25 + 1;
            }
            plVar16 = (long *)serde_json::de::Deserializer<R>::parse_ident(&local_138,&BYTE_1013045e1,3);
          }
LAB_10020606c:
          if (plVar16 != (long *)0x0) goto LAB_100206c8c;
          if (iVar20 == 0) {
            if (local_128 == 0) goto LAB_1002066bc;
            local_128 = local_128 + -1;
            puVar24 = (undefined *)(ulong)*(byte *)(local_130 + local_128);
            bVar10 = true;
          }
          else {
            bVar10 = true;
            puVar24 = puVar26;
          }
LAB_1002060b8:
          do {
            while (uVar21 = (uint)puVar24, (local_c0 & 1) == 0) {
              if (lStack_f0 == local_f8) {
                std::io::uninlined_slow_read_byte(&local_98,&local_108);
                if ((local_98 == 2) || ((local_98 & 1) != 0)) {
                  if (local_98 != 2) {
                    plVar16 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
                    goto LAB_100206c8c;
                  }
                  if ((uVar21 & 0xff) == 0x5b) {
                    local_98 = 2;
                  }
                  else {
                    if ((uVar21 & 0xff) != 0x7b) {
                      ppuVar17 = &PTR_s__Users_borawong__cargo_registry__101816be8;
                      goto LAB_100207608;
                    }
                    local_98 = 3;
                  }
                  bStack_97 = 0;
                  uStack_96 = 0;
                  uStack_94 = 0;
                  uStack_93 = 0;
                  uStack_92 = 0;
                  uStack_91 = 0;
                  plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
                  goto LAB_100206c8c;
                }
              }
              else {
                bStack_97 = *(byte *)(local_108 + local_f8);
                local_f8 = local_f8 + 1;
                local_98 = 0;
              }
              local_d0 = local_d0 + 1;
              if (bStack_97 == 10) {
                local_c8 = local_d0 + local_c8;
                local_d0 = 0;
                local_d8 = local_d8 + 1;
                uVar15 = 10;
                local_c0 = 1;
                local_bf = 10;
              }
              else {
                uVar15 = (ulong)bStack_97;
                uVar23 = (uint)bStack_97;
                local_c0 = 1;
                local_bf = bStack_97;
                if (0x2c < bStack_97) goto LAB_1002061bc;
              }
LAB_10020617c:
              lVar25 = local_110;
              uVar23 = (uint)uVar15;
              if ((1L << (uVar15 & 0x3f) & 0x100002600U) == 0) {
                if (uVar15 != 0x2c) goto LAB_1002061bc;
                if ((bVar10) && (local_c0 = 0, lStack_120 != -0x8000000000000000)) {
                  if (local_110 == lStack_120) {
                    alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
                  }
                  *(undefined1 *)(local_118 + lVar25) = 0x2c;
                  local_110 = lVar25 + 1;
                }
                goto LAB_100206280;
              }
              local_c0 = 0;
              if (lStack_120 != -0x8000000000000000) {
                if (local_110 == lStack_120) {
                  alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
                }
                *(char *)(local_118 + lVar25) = (char)uVar15;
                local_110 = lVar25 + 1;
              }
            }
            uVar15 = (ulong)local_bf;
            uVar23 = (uint)local_bf;
            if (local_bf < 0x2d) goto LAB_10020617c;
LAB_1002061bc:
            lVar25 = local_110;
            if (uVar23 == 0x5d) {
              if ((uVar21 & 0xff) != 0x5b) goto LAB_10020623c;
            }
            else if ((uVar23 != 0x7d) || ((uVar21 & 0xff) != 0x7b)) goto LAB_10020623c;
            local_c0 = 0;
            if (lStack_120 != -0x8000000000000000) {
              if (local_110 == lStack_120) {
                alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
              }
              *(char *)(local_118 + lVar25) = (char)uVar23;
              local_110 = lVar25 + 1;
            }
            if (local_128 == 0) goto LAB_1002066bc;
            local_128 = local_128 + -1;
            bVar10 = true;
            puVar24 = (undefined *)(ulong)*(byte *)(local_130 + local_128);
          } while( true );
        }
        goto LAB_100206c8c;
      }
      if (uVar12 == 2) {
        plVar16 = (long *)serde_json::de::Deserializer<R>::parse_object_colon(&local_138);
        if (plVar16 != (long *)0x0) goto LAB_100206c8c;
        ::core::option::Option<T>>::deserialize(&local_98,&local_138);
        if (local_98 != 1) {
          bVar10 = false;
          uVar12 = (uint)bStack_97;
          local_178 = (ulong)CONCAT12(uStack_94,uStack_96);
          local_a0 = *(undefined2 *)((ulong)&local_98 | 5);
          local_9e = *(undefined1 *)((undefined2 *)((ulong)&local_98 | 5) + 1);
          local_160 = CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90));
          local_184 = local_88;
          goto LAB_1002058c8;
        }
LAB_1002073ec:
        plVar16 = (long *)CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90));
LAB_100206c8c:
        local_1b0 = plVar16;
        bVar19 = 2;
LAB_100206c90:
        local_b8 = local_b8 + '\x01';
        do {
          if ((local_c0 & 1) == 0) {
            if (lStack_f0 == local_f8) {
              std::io::uninlined_slow_read_byte(&local_98,&local_108);
              if ((local_98 == 2) || ((local_98 & 1) != 0)) {
                if (local_98 == 2) {
                  local_98 = 3;
                  bStack_97 = 0;
                  uStack_96 = 0;
                  uStack_94 = 0;
                  uStack_93 = 0;
                  uStack_92 = 0;
                  uStack_91 = 0;
                  plVar14 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
                }
                else {
                  plVar14 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
                }
                goto LAB_100206fa0;
              }
            }
            else {
              bStack_97 = *(byte *)(local_108 + local_f8);
              local_f8 = local_f8 + 1;
              local_98 = 0;
            }
            local_d0 = local_d0 + 1;
            if (bStack_97 != 10) {
              local_c0 = 1;
              local_bf = bStack_97;
              goto joined_r0x000100206d28;
            }
            local_c8 = local_d0 + local_c8;
            local_d0 = 0;
            local_d8 = local_d8 + 1;
            uVar15 = 10;
            local_c0 = 1;
            local_bf = 10;
          }
          else {
joined_r0x000100206d28:
            uVar12 = (uint)local_bf;
            uVar15 = (ulong)local_bf;
            if (0x2c < local_bf) goto LAB_100206f50;
          }
          lVar25 = local_110;
          uVar12 = (uint)uVar15;
          if ((1L << (uVar15 & 0x3f) & 0x100002600U) == 0) goto LAB_100206f30;
          local_c0 = 0;
          if (lStack_120 != -0x8000000000000000) {
            if (local_110 == lStack_120) {
              alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
            }
            *(char *)(local_118 + lVar25) = (char)uVar15;
            local_110 = lVar25 + 1;
          }
        } while( true );
      }
      local_190 = &DAT_10130650c;
      local_198 = 0xe;
    }
  }
  else if (lVar25 == 0x12) {
    if ((*plVar16 != 0x756f527865646f63 || plVar16[1] != 0x6c62616e45726574) || (short)plVar16[2] != 0x6465) goto LAB_100205d14;
    if (bVar3 == 2) {
      plVar16 = (long *)serde_json::de::Deserializer<R>::parse_object_colon(&local_138);
      if (plVar16 != (long *)0x0) goto LAB_100206c8c;
      <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_bool(&local_98,&local_138);
      if (local_98 == 1) goto LAB_1002073ec;
      bVar10 = false;
      bVar3 = bStack_97;
      goto LAB_1002058c8;
    }
    local_190 = &DAT_10130651a;
    local_198 = 0x12;
  }
  else {
    if ((lVar25 != 0x18) || ((*plVar16 != 0x756f527865646f63 || plVar16[1] != 0x6363416f4e726574) || plVar16[2] != 0x65646f4d746e756f)) goto LAB_100205d14;
    if (bVar19 == 2) {
      plVar16 = (long *)serde_json::de::Deserializer<R>::parse_object_colon(&local_138);
      if (plVar16 != (long *)0x0) goto LAB_100206c8c;
      <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_bool(&local_98,&local_138);
      if (local_98 == 1) goto LAB_1002073ec;
      bVar10 = false;
      bVar19 = bStack_97;
      goto LAB_1002058c8;
    }
    local_190 = &DAT_10130652c;
    local_198 = 0x18;
  }
  local_80 = local_190;
  local_78 = local_198;
  local_98 = (byte)&local_80;
  bStack_97 = (byte)((ulong)&local_80 >> 8);
  uStack_96 = (undefined2)((ulong)&local_80 >> 0x10);
  uStack_94 = (undefined1)((ulong)&local_80 >> 0x20);
  uStack_93 = (undefined1)((ulong)&local_80 >> 0x28);
  uStack_92 = (undefined1)((ulong)&local_80 >> 0x30);
  uStack_91 = (undefined1)((ulong)&local_80 >> 0x38);
  uStack_90 = 0xd6c0;
  uStack_8e = 0x7d;
  uStack_8c = 1;
  plVar16 = (long *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_15,&local_98);
  goto LAB_100206c8c;
LAB_10020623c:
  if (bVar10) {
    if ((uVar21 & 0xff) == 0x5b) {
      local_98 = 7;
    }
    else {
      if ((uVar21 & 0xff) != 0x7b) {
        ppuVar17 = &PTR_s__Users_borawong__cargo_registry__101816c00;
LAB_100207608:
                    /* WARNING: Subroutine does not return */
        ::core::panicking::panic(&_anon_dc5b8adae996cb58fd4173e320c2d1cf_4,0x28,ppuVar17);
      }
      local_98 = 8;
    }
    bStack_97 = 0;
    uStack_96 = 0;
    uStack_94 = 0;
    uStack_93 = 0;
    uStack_92 = 0;
    uStack_91 = 0;
    plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
    goto LAB_100206c8c;
  }
LAB_100206280:
  iVar20 = 1;
  puVar26 = puVar24;
  if ((uVar21 & 0xff) != 0x7b) goto LAB_100205d40;
  if ((local_c0 & 1) == 0) {
    <serde_json::iter::LineColIterator<I>as_core::iter::traits::iterator::Iterator>::next(&local_98,&local_108);
    bVar10 = local_98 != 2;
    if (bVar10) {
      if ((local_98 & 1) == 0) {
        bVar8 = false;
        uVar21 = (uint)bStack_97;
        local_c0 = 1;
        local_bf = bStack_97;
      }
      else {
        local_1a8 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
        bVar8 = true;
        uVar21 = 0x22;
      }
      goto LAB_1002062f0;
    }
    bVar7 = false;
    uVar21 = 0x22;
LAB_100206444:
    if (bVar7) {
      bVar19 = 2;
      local_1b0 = local_1a8;
      goto LAB_100206c90;
    }
    if (!bVar10) {
      local_98 = 3;
      bStack_97 = 0;
      uStack_96 = 0;
      uStack_94 = 0;
      uStack_93 = 0;
      uStack_92 = 0;
      uStack_91 = 0;
      plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
      goto LAB_100206c8c;
    }
    if (uVar21 != 0x22) {
LAB_100207504:
      local_98 = 0x11;
      bStack_97 = 0;
      uStack_96 = 0;
      uStack_94 = 0;
      uStack_93 = 0;
      uStack_92 = 0;
      uStack_91 = 0;
      plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
      goto LAB_100206c8c;
    }
  }
  else {
    bVar8 = false;
    uVar21 = (uint)local_bf;
    bVar10 = true;
LAB_1002062f0:
    bVar22 = local_bf;
    bVar9 = local_c0;
    lVar25 = local_110;
    bVar7 = true;
    if (bVar8) goto LAB_100206444;
    if (0x22 < uVar21) goto LAB_100207504;
    uVar15 = (ulong)uVar21;
    if ((1L << (uVar15 & 0x3f) & 0x100002600U) != 0) {
      local_c0 = 0;
      if (bVar9 != 1) goto LAB_100206350;
      if (lStack_120 == -0x8000000000000000) goto LAB_100206350;
      if (local_110 == lStack_120) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
      }
      do {
        *(byte *)(local_118 + lVar25) = bVar22;
        local_110 = lVar25 + 1;
LAB_100206350:
        do {
          if (local_c0 == 1) {
            bVar8 = false;
            uVar21 = (uint)local_bf;
            bVar10 = true;
          }
          else {
            <serde_json::iter::LineColIterator<I>as_core::iter::traits::iterator::Iterator>::next(&local_98,&local_108);
            bVar10 = local_98 != 2;
            if (local_98 == 2) {
              bVar7 = false;
              goto LAB_100206444;
            }
            if ((local_98 & 1) == 0) {
              bVar8 = false;
              uVar21 = (uint)bStack_97;
              local_c0 = 1;
              local_bf = bStack_97;
            }
            else {
              local_1a8 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
              bVar8 = true;
            }
          }
          bVar22 = local_bf;
          bVar9 = local_c0;
          lVar25 = local_110;
          bVar7 = true;
          if (bVar8) goto LAB_100206444;
          if (0x22 < uVar21) goto LAB_100207504;
          uVar15 = (ulong)uVar21;
          if ((1L << (uVar15 & 0x3f) & 0x100002600U) == 0) goto LAB_100206408;
          local_c0 = 0;
        } while ((bVar9 != 1) || (lStack_120 == -0x8000000000000000));
        if (local_110 == lStack_120) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
        }
      } while( true );
    }
LAB_100206408:
    if (uVar15 != 0x22) goto LAB_100207504;
  }
  bVar22 = local_bf;
  bVar9 = local_c0;
  lVar25 = local_110;
  local_c0 = 0;
  if ((bVar9 == 1) && (lStack_120 != -0x8000000000000000)) {
    if (local_110 == lStack_120) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
    }
    *(byte *)(local_118 + lVar25) = bVar22;
    local_110 = lVar25 + 1;
  }
  plVar16 = (long *)<serde_json::read::IoRead<R>as_serde_json::read::Read>::ignore_str(&lStack_120);
  if (plVar16 != (long *)0x0) goto LAB_100206c8c;
  if ((local_c0 & 1) == 0) {
    <serde_json::iter::LineColIterator<I>as_core::iter::traits::iterator::Iterator>::next(&local_98,&local_108);
    bVar10 = local_98 != 2;
    if (bVar10) {
      if ((local_98 & 1) == 0) {
        bVar8 = false;
        uVar21 = (uint)bStack_97;
        local_c0 = 1;
        local_bf = bStack_97;
      }
      else {
        local_1b0 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
        bVar8 = true;
        uVar21 = 0x3a;
      }
      goto LAB_100206508;
    }
    bVar8 = false;
    uVar21 = 0x3a;
  }
  else {
    bVar8 = false;
    uVar21 = (uint)local_bf;
    bVar10 = true;
LAB_100206508:
    bVar22 = local_bf;
    bVar9 = local_c0;
    lVar25 = local_110;
    if (!bVar8) {
      if (0x3a < uVar21) goto LAB_10020751c;
      uVar15 = (ulong)uVar21;
      if ((1L << (uVar15 & 0x3f) & 0x100002600U) != 0) {
        local_c0 = 0;
        if (bVar9 != 1) goto LAB_100206568;
        if (lStack_120 == -0x8000000000000000) goto LAB_100206568;
        if (local_110 == lStack_120) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
        }
        do {
          *(byte *)(local_118 + lVar25) = bVar22;
          local_110 = lVar25 + 1;
LAB_100206568:
          do {
            if (local_c0 == 1) {
              bVar7 = false;
              uVar21 = (uint)local_bf;
              bVar10 = true;
            }
            else {
              <serde_json::iter::LineColIterator<I>as_core::iter::traits::iterator::Iterator>::next(&local_98,&local_108);
              bVar10 = local_98 != 2;
              if (local_98 == 2) {
                bVar8 = false;
                goto LAB_10020665c;
              }
              if ((local_98 & 1) == 0) {
                bVar7 = false;
                uVar21 = (uint)bStack_97;
                local_c0 = 1;
                local_bf = bStack_97;
              }
              else {
                local_1b0 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
                bVar7 = true;
              }
            }
            bVar22 = local_bf;
            bVar9 = local_c0;
            lVar25 = local_110;
            bVar8 = true;
            if (bVar7) goto LAB_10020665c;
            if (0x3a < uVar21) goto LAB_10020751c;
            uVar15 = (ulong)uVar21;
            if ((1L << (uVar15 & 0x3f) & 0x100002600U) == 0) goto LAB_100206620;
            local_c0 = 0;
          } while ((bVar9 != 1) || (lStack_120 == -0x8000000000000000));
          if (local_110 == lStack_120) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
          }
        } while( true );
      }
LAB_100206620:
      if (uVar15 != 0x3a) goto LAB_10020751c;
      goto LAB_10020666c;
    }
  }
LAB_10020665c:
  if (bVar8) {
    bVar19 = 2;
    goto LAB_100206c90;
  }
  if (!bVar10) {
    local_98 = 3;
    bStack_97 = 0;
    uStack_96 = 0;
    uStack_94 = 0;
    uStack_93 = 0;
    uStack_92 = 0;
    uStack_91 = 0;
    plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
    goto LAB_100206c8c;
  }
  if (uVar21 != 0x3a) {
LAB_10020751c:
    local_98 = 6;
    bStack_97 = 0;
    uStack_96 = 0;
    uStack_94 = 0;
    uStack_93 = 0;
    uStack_92 = 0;
    uStack_91 = 0;
    plVar16 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
    goto LAB_100206c8c;
  }
LAB_10020666c:
  bVar22 = local_bf;
  bVar9 = local_c0;
  lVar25 = local_110;
  local_c0 = 0;
  iVar20 = 1;
  if ((bVar9 == 1) && (lStack_120 != -0x8000000000000000)) {
    if (local_110 == lStack_120) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
    }
    *(byte *)(local_118 + lVar25) = bVar22;
    local_110 = lVar25 + 1;
    iVar20 = 1;
  }
  goto LAB_100205d40;
LAB_1002066bc:
  bVar10 = false;
  goto LAB_1002058c8;
LAB_100206f30:
  if (uVar15 == 0x2c) {
    local_98 = 0x15;
    bStack_97 = 0;
    uStack_96 = 0;
    uStack_94 = 0;
    uStack_93 = 0;
    uStack_92 = 0;
    uStack_91 = 0;
    plVar14 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
  }
  else {
LAB_100206f50:
    lVar25 = local_110;
    if (uVar12 == 0x7d) {
      local_c0 = 0;
      if (lStack_120 == -0x8000000000000000) {
        plVar14 = (long *)0x0;
      }
      else {
        if (local_110 == lStack_120) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
        }
        plVar14 = (long *)0x0;
        *(undefined1 *)(local_118 + lVar25) = 0x7d;
        local_110 = lVar25 + 1;
      }
    }
    else {
      local_98 = 0x16;
      bStack_97 = 0;
      uStack_96 = 0;
      uStack_94 = 0;
      uStack_93 = 0;
      uStack_92 = 0;
      uStack_91 = 0;
      plVar14 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
    }
  }
LAB_100206fa0:
  if (bVar19 == 2) {
    plVar16 = local_1b0;
    if (plVar14 != (long *)0x0) {
      ::core::ptr::drop_in_place<serde_json::error::ErrorCode>();
      goto LAB_100206fb8;
    }
  }
  else {
    plVar16 = plVar14;
    if (plVar14 == (long *)0x0) {
      lVar25 = -0x98;
      goto LAB_1002070ec;
    }
  }
  goto LAB_100206fd4;
LAB_10020722c:
  local_98 = 0x16;
  bStack_97 = 0;
  uStack_96 = 0;
  uStack_94 = 0;
  uStack_93 = 0;
  uStack_92 = 0;
  uStack_91 = 0;
  uVar18 = serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
  goto LAB_100206fdc;
LAB_100206a54:
  if ((local_c0 & 1) == 0) {
    if (lStack_f0 == local_f8) {
      std::io::uninlined_slow_read_byte(&local_98,&local_108);
      if ((local_98 == 2) || ((local_98 & 1) != 0)) {
        if (local_98 == 2) {
          local_98 = 2;
          bStack_97 = 0;
          uStack_96 = 0;
          uStack_94 = 0;
          uStack_93 = 0;
          uStack_92 = 0;
          uStack_91 = 0;
          plVar14 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
        }
        else {
          plVar14 = (long *)serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
        }
        goto LAB_100206f0c;
      }
    }
    else {
      bStack_97 = *(byte *)(local_108 + local_f8);
      local_f8 = local_f8 + 1;
      local_98 = 0;
    }
    local_d0 = local_d0 + 1;
    if (bStack_97 != 10) {
      local_c0 = 1;
      local_bf = bStack_97;
      goto joined_r0x000100206abc;
    }
    local_c8 = local_d0 + local_c8;
    local_d0 = 0;
    local_d8 = local_d8 + 1;
    uVar15 = 10;
    local_c0 = 1;
    local_bf = 10;
  }
  else {
joined_r0x000100206abc:
    uVar12 = (uint)local_bf;
    uVar15 = (ulong)local_bf;
    if (0x2c < local_bf) goto LAB_100206ebc;
  }
  lVar25 = local_110;
  uVar12 = (uint)uVar15;
  if ((1L << (uVar15 & 0x3f) & 0x100002600U) != 0) {
    local_c0 = 0;
    if (lStack_120 != -0x8000000000000000) {
      if (local_110 == lStack_120) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
      }
      *(char *)(local_118 + lVar25) = (char)uVar15;
      local_110 = lVar25 + 1;
    }
    goto LAB_100206a54;
  }
  if (uVar15 == 0x2c) {
    <serde_json::read::IoRead<R>as_serde_json::read::Read>::discard(&lStack_120);
    serde_json::de::Deserializer<R>::parse_whitespace(&local_80,&local_138);
    uVar15 = (ulong)local_80 & 0xff;
    if ((((char)local_80 == '\x01') || (local_80._1_1_ != '\x01')) || (local_80._2_1_ != ']')) {
      local_98 = 0x16;
      bStack_97 = 0;
      uStack_96 = 0;
      uStack_94 = 0;
      uStack_93 = 0;
      uStack_92 = 0;
      uStack_91 = 0;
      plVar14 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
    }
    else {
      local_98 = 0x15;
      bStack_97 = 0;
      uStack_96 = 0;
      uStack_94 = 0;
      uStack_93 = 0;
      uStack_92 = 0;
      uStack_91 = 0;
      plVar14 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
    }
    ::core::ptr::drop_in_place<core::result::Result<core::option::Option<u8>,serde_json::error::Error>>(uVar15,local_78);
  }
  else {
LAB_100206ebc:
    lVar25 = local_110;
    if (uVar12 == 0x5d) {
      local_c0 = 0;
      if (lStack_120 == -0x8000000000000000) {
        plVar14 = (long *)0x0;
      }
      else {
        if (local_110 == lStack_120) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
        }
        plVar14 = (long *)0x0;
        *(undefined1 *)(local_118 + lVar25) = 0x5d;
        local_110 = lVar25 + 1;
      }
    }
    else {
      local_98 = 0x16;
      bStack_97 = 0;
      uStack_96 = 0;
      uStack_94 = 0;
      uStack_93 = 0;
      uStack_92 = 0;
      uStack_91 = 0;
      plVar14 = (long *)serde_json::error::Error::syntax(&local_98,local_d8,local_d0);
    }
  }
LAB_100206f0c:
  if (bVar19 == 2) {
    plVar16 = local_1b0;
    if (plVar14 != (long *)0x0) {
      ::core::ptr::drop_in_place<serde_json::error::ErrorCode>();
LAB_100206fb8:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar14,0x28,8);
      plVar16 = local_1b0;
    }
  }
  else {
    plVar16 = plVar14;
    if (plVar14 == (long *)0x0) {
      lVar25 = -0xa0;
LAB_1002070ec:
      uVar4 = *(undefined2 *)(&stack0xfffffffffffffff0 + lVar25);
      uVar2 = (&stack0xfffffffffffffff2)[lVar25];
      uStack_14b = (undefined3)((ulong)local_1b0 >> 0x28);
      local_145 = (undefined2)local_160;
      uStack_143 = (undefined6)((ulong)local_160 >> 0x10);
LAB_100207130:
      if ((local_c0 & 1) == 0) {
        if (lStack_f0 == local_f8) {
          std::io::uninlined_slow_read_byte(&local_98,&local_108);
          if ((local_98 == 2) || ((local_98 & 1) != 0)) {
            if (local_98 == 2) {
              if (local_100 != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_108,local_100,1);
              }
              uVar12 = _close(local_e0);
              uVar15 = (ulong)uVar12;
              if ((lStack_120 != -0x8000000000000000) && (lStack_120 != 0)) {
                uVar15 = __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_118,lStack_120,1);
              }
              if (local_138 != 0) {
                uVar15 = __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,local_138,1);
              }
              if (*(int *)(param_3 + 0x168) == (int)local_1b0) {
                bVar3 = *(byte *)(param_3 + 0x16d);
                if (((ulong)local_1b0 >> 0x20 & 1) != 0) {
                  if ((bVar3 & 1) == 0) goto LAB_1002072f8;
                  bVar3 = CONCAT26(local_145,CONCAT15(uVar2,CONCAT23(uVar4,uStack_14b))) != *(long *)(param_3 + 0x16e) ||
                          CONCAT26(local_184,uStack_143) != *(long *)(param_3 + 0x176);
                }
                if ((((bVar3 & 1) == 0) && (*(byte *)(param_3 + 0x16c) == bVar1)) && (((bVar1 & 1) == 0 || (bVar19 == *(byte *)(param_3 + 0x17e))))) {
                  *param_1 = 0xb;
                  return;
                }
              }
LAB_1002072f8:
              __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2(uVar15);
              puVar13 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x8b,1);
              if (puVar13 != (undefined8 *)0x0) {
                puVar13[0xd] = 0xe7a59fe680a3e6b7;
                puVar13[0xc] = 0xafe89bbcefa585e5;
                puVar13[0xf] = 0x87e9b6b9e5b6bbe4;
                puVar13[0xe] = 0x8796e68180e6b68a;
                *(undefined8 *)((long)puVar13 + 0x83) = 0x694d614d694120af;
                *(undefined8 *)((long)puVar13 + 0x7b) = 0x90e58d87e9b6b9e5;
                puVar13[5] = 0x989be781a3e7b9af;
                puVar13[4] = 0xe5b8a0e695b3e6a0;
                puVar13[7] = 0xe58eb8e4a789e69e;
                puVar13[6] = 0x9be5b194e7afb7e8;
                puVar13[9] = 0xb7e58cbcef8fbce5;
                puVar13[8] = 0xa1a8e68d89e593bd;
                puVar13[0xb] = 0x9986e5adbbe7a7bb;
                puVar13[10] = 0xe7a2ade69c81e5b2;
                puVar13[1] = 0x54535f54494d4d4f;
                *puVar13 = 0x435f524554554f52;
                puVar13[3] = 0x97e6203a5443494c;
                puVar13[2] = 0x464e4f435f455441;
                param_1[1] = 0x8b;
                *param_1 = 10;
                param_1[2] = puVar13;
                param_1[3] = 0x8b;
                return;
              }
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error(1,0x8b);
            }
            uVar18 = serde_json::error::Error::io(CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90)));
            goto LAB_100206fdc;
          }
          local_d0 = local_d0 + 1;
          if (bStack_97 == 10) goto LAB_1002071c8;
LAB_100207194:
          uVar15 = (ulong)bStack_97;
        }
        else {
          bStack_97 = *(byte *)(local_108 + local_f8);
          local_f8 = local_f8 + 1;
          local_98 = 0;
          local_d0 = local_d0 + 1;
          if (bStack_97 != 10) goto LAB_100207194;
LAB_1002071c8:
          local_c8 = local_d0 + local_c8;
          local_d0 = 0;
          local_d8 = local_d8 + 1;
          uVar15 = 10;
        }
        local_c0 = 1;
        local_bf = (byte)uVar15;
        if (0x20 < (uint)uVar15 || (1L << (uVar15 & 0x3f) & 0x100002600U) == 0) goto LAB_10020722c;
      }
      else if (0x20 < local_bf || (1L << ((ulong)local_bf & 0x3f) & 0x100002600U) == 0) goto LAB_10020722c;
      bVar3 = local_bf;
      lVar25 = local_110;
      local_c0 = 0;
      if (lStack_120 != -0x8000000000000000) {
        if (local_110 == lStack_120) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&lStack_120);
        }
        *(byte *)(local_118 + lVar25) = bVar3;
        local_110 = lVar25 + 1;
      }
      goto LAB_100207130;
    }
  }
LAB_100206fd4:
  uVar18 = serde_json::error::Error::fix_position(plVar16,&local_138);
LAB_100206fdc:
  if (local_100 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_108,local_100,1);
  }
  uVar12 = _close(local_e0);
  uVar15 = (ulong)uVar12;
  if ((lStack_120 != -0x8000000000000000) && (lStack_120 != 0)) {
    uVar15 = __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_118,lStack_120,1);
  }
  if (local_138 != 0) {
    uVar15 = __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,local_138,1);
  }
  local_138 = uVar18;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2(uVar15);
  puVar13 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x8b,1);
  if (puVar13 != (undefined8 *)0x0) {
    puVar13[0xd] = 0xe7a59fe680a3e6b7;
    puVar13[0xc] = 0xafe89bbcefa585e5;
    puVar13[0xf] = 0x87e9b6b9e5b6bbe4;
    puVar13[0xe] = 0x8796e68180e6b68a;
    *(undefined8 *)((long)puVar13 + 0x83) = 0x694d614d694120af;
    *(undefined8 *)((long)puVar13 + 0x7b) = 0x90e58d87e9b6b9e5;
    puVar13[5] = 0x989be781a3e7b9af;
    puVar13[4] = 0xe5b8a0e695b3e6a0;
    puVar13[7] = 0xe58eb8e4a789e69e;
    puVar13[6] = 0x9be5b194e7afb7e8;
    puVar13[9] = 0xb7e58cbcef8fbce5;
    puVar13[8] = 0xa1a8e68d89e593bd;
    puVar13[0xb] = 0x9986e5adbbe7a7bb;
    puVar13[10] = 0xe7a2ade69c81e5b2;
    puVar13[1] = 0x54535f54494d4d4f;
    *puVar13 = 0x435f524554554f52;
    puVar13[3] = 0x97e6203a5443494c;
    puVar13[2] = 0x464e4f435f455441;
    ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(uVar18);
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar18,0x28,8);
    *param_1 = 10;
    *(undefined4 *)(param_1 + 1) = 0x8b;
    *(undefined1 *)((long)param_1 + 0xc) = 0;
    *(undefined1 *)((long)param_1 + 0xf) = 0;
    *(undefined2 *)((long)param_1 + 0xd) = 0;
    param_1[2] = puVar13;
    *(undefined1 *)((long)param_1 + 0x1c) = 0;
    *(undefined4 *)(param_1 + 3) = 0x8b;
    *(undefined2 *)((long)param_1 + 0x1d) = 0;
    *(undefined1 *)((long)param_1 + 0x1f) = 0;
    return;
  }
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error(1,0x8b);
}
