/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* deserialize_struct @ 10027b0e0 requested 10027b0e0 */

/* WARNING: Removing unreachable block (ram,0x00010027b698) */
/* WARNING: Removing unreachable block (ram,0x00010027b4f8) */
/* WARNING: Type propagation algorithm not settling */
/* <&mut serde_json::de::Deserializer<R> as serde_core::de::Deserializer>::deserialize_struct */

void <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_struct(undefined8 *param_1,long *******param_2)

{
  char cVar1;
  byte bVar2;
  uint uVar3;
  long *******ppppppplVar4;
  undefined8 uVar5;
  long *******ppppppplVar6;
  long ******pppppplVar7;
  long *******extraout_x9;
  long *******extraout_x9_00;
  long *******extraout_x9_01;
  long *******extraout_x9_02;
  long *******extraout_x9_03;
  long *******extraout_x9_04;
  long *******extraout_x9_05;
  long *******extraout_x9_06;
  long *******extraout_x9_07;
  long *******extraout_x9_08;
  long *******extraout_x9_09;
  long *******extraout_x9_10;
  long *******extraout_x9_11;
  long *******extraout_x9_12;
  long *******extraout_x9_13;
  long *******extraout_x9_14;
  long *******extraout_x9_15;
  long *******extraout_x9_16;
  long *******extraout_x9_17;
  long *******extraout_x9_18;
  long *******extraout_x9_19;
  long *******extraout_x9_20;
  long *******ppppppplVar8;
  long *******extraout_x9_21;
  long *******extraout_x9_22;
  long *******extraout_x9_23;
  long *******extraout_x9_24;
  long *******extraout_x9_25;
  long *******extraout_x9_26;
  long *******extraout_x9_27;
  long *******extraout_x9_28;
  long *******extraout_x10;
  long *******extraout_x10_00;
  long *******extraout_x10_01;
  long *******extraout_x10_02;
  long *******extraout_x10_03;
  long *******extraout_x10_04;
  long *******extraout_x10_05;
  long *******extraout_x10_06;
  long *******extraout_x10_07;
  long *******extraout_x10_08;
  long *******extraout_x10_09;
  long *******extraout_x10_10;
  long *******extraout_x10_11;
  long *******extraout_x10_12;
  long *******extraout_x10_13;
  long *******extraout_x10_14;
  long *******ppppppplVar9;
  long *******extraout_x10_15;
  long *******extraout_x10_16;
  long *******extraout_x10_17;
  long *******extraout_x10_18;
  long *******extraout_x10_19;
  long *******extraout_x10_20;
  long *******extraout_x10_21;
  long *******extraout_x10_22;
  long *******extraout_x11;
  long *******extraout_x11_00;
  long *******extraout_x11_01;
  long *******extraout_x11_02;
  long *******extraout_x11_03;
  long *******extraout_x11_04;
  long *******extraout_x11_05;
  long *******extraout_x11_06;
  long *******extraout_x11_07;
  long *******extraout_x11_08;
  long *******extraout_x11_09;
  long *******ppppppplVar10;
  long *******ppppppplVar11;
  bool bVar12;
  long *******unaff_x24;
  long *******unaff_x25;
  long *******unaff_x26;
  long *******unaff_x28;
  long *******ppppppplVar13;
  long *******ppppppplVar14;
  long *******local_130;
  long *******local_128;
  long *******local_120;
  long *******local_118;
  long *******local_110;
  undefined8 local_f8;
  long *******local_f0;
  long *******local_e8;
  long *******ppppppplStack_e0;
  long *******local_d8;
  long *******local_c8;
  long *******ppppppplStack_c0;
  long *******local_b8;
  long *******local_b0;
  undefined1 local_a8;
  long *******local_a0;
  long *******ppppppplStack_98;
  long *******local_90;
  long *******local_88;
  long *******local_80;
  long *******local_78;
  undefined1 auStack_69 [9];

  pppppplVar7 = param_2[5];
  if (pppppplVar7 < param_2[4]) {
    do {
      bVar2 = *(byte *)((long)param_2[3] + (long)pppppplVar7);
      if (0x20 < bVar2 || (1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) {
        if (bVar2 == 0x5b) {
          cVar1 = *(char *)(param_2 + 7);
          *(char *)(param_2 + 7) = cVar1 + -1;
          if ((char)(cVar1 + -1) != '\0') {
            param_2[5] = (long ******)((long)pppppplVar7 + 1);
            local_80 = (long *******)CONCAT71(local_80._1_7_,1);
            local_88 = param_2;
            <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_f8,&local_88);
            local_130 = extraout_x9_02;
            local_128 = extraout_x10_04;
            ppppppplVar10 = extraout_x11;
            if ((char)local_f8 == '\x01') {
LAB_10027b48c:
              ppppppplVar11 = (long *******)&DAT_8000000000000000;
              local_120 = local_f0;
              ppppppplVar14 = unaff_x28;
            }
            else {
              if (local_f8._1_1_ != '\x01') {
                local_f0 = (long *******)
                           serde_core::de::Error::invalid_length(0,&PTR_s_struct_NoAccountModelRestoreEntr_10130772a_0x3c0_101817c48,&DAT_101817930);
                local_130 = extraout_x9_14;
                local_128 = extraout_x10_12;
                ppppppplVar10 = extraout_x11_05;
                goto LAB_10027b48c;
              }
              deserialize_string(&local_f8,local_88);
              ppppppplVar14 = local_e8;
              local_120 = local_f0;
              ppppppplVar11 = local_f8;
              local_130 = extraout_x9_03;
              local_128 = extraout_x10_05;
              ppppppplVar10 = extraout_x11_00;
              if (local_f8 == (long *******)&DAT_8000000000000000) goto LAB_10027b48c;
              <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_f8,&local_88);
              ppppppplVar13 = extraout_x9_06;
              ppppppplVar6 = extraout_x11_01;
              if ((char)local_f8 == '\x01') {
LAB_10027b75c:
                local_128 = local_f0;
                if (ppppppplVar11 != (long *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,ppppppplVar11,1);
                  ppppppplVar13 = extraout_x9_19;
                  ppppppplVar6 = extraout_x11_09;
                }
                ppppppplVar11 = (long *******)&DAT_8000000000000000;
                local_130 = ppppppplVar13;
                ppppppplVar10 = ppppppplVar6;
                local_120 = local_128;
              }
              else {
                if (local_f8._1_1_ != '\x01') {
                  local_f0 = (long *******)
                             serde_core::de::Error::invalid_length(1,&PTR_s_struct_NoAccountModelRestoreEntr_10130772a_0x3c0_101817c48,&DAT_101817930);
                  ppppppplVar13 = extraout_x9_13;
                  ppppppplVar6 = extraout_x11_04;
                  goto LAB_10027b75c;
                }
                codexmate_lib::core::relay::codex_thread_visibility::NoAccountModelRestoreEntry>::deserialize::__Visitor_as_serde_core::de::Visitor>::visit_map
                ::__DeserializeWith_as_serde_core::de::Deserialize>::deserialize(&local_f8,local_88);
                local_130 = local_e8;
                local_128 = local_f0;
                unaff_x25 = local_f8;
                ppppppplVar13 = extraout_x9_08;
                ppppppplVar6 = extraout_x11_02;
                if (local_f8 == (long *******)&DAT_8000000000000001) goto LAB_10027b75c;
                <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed::has_next_element(&local_f8,&local_88);
                ppppppplVar13 = extraout_x9_09;
                ppppppplVar6 = extraout_x11_03;
                unaff_x26 = local_f0;
                if ((char)local_f8 == '\x01') {
LAB_10027b740:
                  local_f0 = unaff_x26;
                  if (((ulong)unaff_x25 & 0x7fffffffffffffff) != 0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,unaff_x25,1);
                    ppppppplVar13 = extraout_x9_18;
                    ppppppplVar6 = extraout_x11_08;
                  }
                  goto LAB_10027b75c;
                }
                if (local_f8._1_1_ != '\x01') {
                  unaff_x26 = (long *******)
                              serde_core::de::Error::invalid_length(2,&PTR_s_struct_NoAccountModelRestoreEntr_10130772a_0x3c0_101817c48,&DAT_101817930);
                  ppppppplVar13 = extraout_x9_17;
                  ppppppplVar6 = extraout_x11_07;
                  goto LAB_10027b740;
                }
                deserialize_string(&local_f8,local_88);
                ppppppplVar13 = extraout_x9_15;
                ppppppplVar6 = extraout_x11_06;
                ppppppplVar10 = local_e8;
                unaff_x24 = local_f8;
                unaff_x26 = local_f0;
                if (local_f8 == (long *******)&DAT_8000000000000000) goto LAB_10027b740;
              }
            }
            *(char *)(param_2 + 7) = *(char *)(param_2 + 7) + '\x01';
            local_f8 = ppppppplVar11;
            local_f0 = local_120;
            local_e8 = ppppppplVar14;
            ppppppplStack_e0 = unaff_x24;
            local_d8 = unaff_x26;
            local_c8 = unaff_x25;
            ppppppplStack_c0 = local_128;
            local_b8 = local_130;
            ppppppplVar6 = (long *******)serde_json::de::Deserializer<R>::end_seq(param_2);
            ppppppplVar13 = local_120;
            if (ppppppplVar11 != (long *******)&DAT_8000000000000000) goto joined_r0x00010027b7c0;
            if (ppppppplVar6 == (long *******)0x0) goto LAB_10027b980;
            core::ptr::drop_in_place<serde_json::error::ErrorCode>();
            goto LAB_10027b96c;
          }
        }
        else {
          if (bVar2 != 0x7b) {
            local_120 = (long *******)serde_json::de::Deserializer<R>::peek_invalid_type(param_2,auStack_69,&DAT_101818d30);
            goto LAB_10027b980;
          }
          uVar3 = *(byte *)(param_2 + 7) - 1;
          *(char *)(param_2 + 7) = (char)uVar3;
          if ((uVar3 & 0xff) != 0) {
            param_2[5] = (long ******)((long)pppppplVar7 + 1);
            local_a8 = 1;
            local_118 = (long *******)&DAT_8000000000000001;
            ppppppplVar13 = (long *******)&DAT_8000000000000000;
            local_b0 = param_2;
            ppppppplVar6 = local_120;
            ppppppplVar11 = (long *******)&DAT_8000000000000000;
            ppppppplVar14 = (long *******)(ulong)uVar3;
            goto LAB_10027b1c8;
          }
        }
        local_f8 = (long *******)0x18;
        goto LAB_10027b150;
      }
      pppppplVar7 = (long ******)((long)pppppplVar7 + 1);
      param_2[5] = pppppplVar7;
    } while (param_2[4] != pppppplVar7);
  }
  local_f8 = (long *******)0x5;
LAB_10027b150:
  uVar5 = serde_json::de::Deserializer<R>::peek_error(param_2,&local_f8);
  goto LAB_10027b988;
LAB_10027b1c8:
  local_120 = ppppppplVar6;
  ppppppplVar10 = unaff_x25;
  unaff_x24 = ppppppplVar13;
  <serde_json::de::MapAccess<R>as_serde_core::de::MapAccess>::next_key_seed::has_next_key(&local_f8,&local_b0);
  ppppppplVar4 = local_b0;
  ppppppplVar8 = extraout_x9;
  ppppppplVar9 = extraout_x10;
  ppppppplVar6 = local_f0;
  if ((char)local_f8 == '\x01') {
LAB_10027b82c:
    bVar12 = false;
    ppppppplVar13 = (long *******)0x1;
    goto LAB_10027b834;
  }
  if (local_f8._1_1_ != '\x01') {
    if (ppppppplVar11 != (long *******)&DAT_8000000000000000) {
      ppppppplVar13 = (long *******)(ulong)(local_118 == (long *******)&DAT_8000000000000001);
      if (local_118 == (long *******)&DAT_8000000000000001) {
        local_88 = (long *******)0x101307ace;
        local_80 = (long *******)&DAT_0000000e;
        local_f8 = (long *******)&local_88;
        local_f0 = (long *******)&<&T_as_core::fmt::Display>::fmt;
        ppppppplVar6 = (long *******)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_f8);
        ppppppplVar8 = extraout_x9_27;
        ppppppplVar9 = extraout_x10_21;
      }
      else {
        if (unaff_x24 != (long *******)&DAT_8000000000000000) goto LAB_10027b89c;
        local_88 = (long *******)0x101307adc;
        local_80 = (long *******)&DAT_0000000e;
        local_f8 = (long *******)&local_88;
        local_f0 = (long *******)&<&T_as_core::fmt::Display>::fmt;
        ppppppplVar6 = (long *******)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_f8);
        ppppppplVar8 = extraout_x9_10;
        ppppppplVar9 = extraout_x10_09;
        if (((ulong)local_118 & 0x7fffffffffffffff) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,local_118,1);
          ppppppplVar8 = extraout_x9_11;
          ppppppplVar9 = extraout_x10_10;
        }
      }
      bVar12 = true;
      if (ppppppplVar11 != (long *******)0x0) {
        bVar12 = true;
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,ppppppplVar11,1);
        ppppppplVar8 = extraout_x9_12;
        ppppppplVar9 = extraout_x10_11;
      }
      goto LAB_10027b834;
    }
    local_88 = (long *******)0x1013068bb;
    local_80 = (long *******)&DAT_00000009;
    local_f8 = (long *******)&local_88;
    local_f0 = (long *******)&<&T_as_core::fmt::Display>::fmt;
    ppppppplVar11 = (long *******)&DAT_8000000000000000;
    ppppppplVar6 = (long *******)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_f8);
    ppppppplVar8 = extraout_x9_07;
    ppppppplVar9 = extraout_x10_08;
LAB_10027b574:
    bVar12 = false;
    ppppppplVar13 = (long *******)0x1;
    goto LAB_10027b834;
  }
  local_b0[5] = (long ******)((long)local_b0[5] + 1);
  local_b0[2] = (long ******)0x0;
  <serde_json::read::SliceRead_as_serde_json::read::Read>::parse_str(&local_f8,local_b0 + 3,local_b0);
  ppppppplVar8 = extraout_x9_00;
  ppppppplVar9 = extraout_x10_00;
  ppppppplVar6 = local_f0;
  if (local_f8 == (long *******)0x2) goto LAB_10027b82c;
  unaff_x25 = ppppppplVar10;
  ppppppplVar13 = unaff_x24;
  if (local_e8 == (long *******)&DAT_00000009) {
    if (*local_f0 != (long ******)0x695f646165726874 || *(char *)(local_f0 + 1) != 'd') goto LAB_10027b810;
    if (ppppppplVar11 != (long *******)&DAT_8000000000000000) {
      local_88 = (long *******)0x1013068bb;
      goto LAB_10027b9cc;
    }
    pppppplVar7 = ppppppplVar4[5];
    if (ppppppplVar4[4] <= pppppplVar7) {
LAB_10027b494:
      local_f8 = (long *******)0x3;
      ppppppplVar6 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar4,&local_f8);
      ppppppplVar8 = extraout_x9_04;
      ppppppplVar9 = extraout_x10_06;
      goto LAB_10027b4b8;
    }
    while( true ) {
      bVar2 = *(byte *)((long)ppppppplVar4[3] + (long)pppppplVar7);
      if (0x3a < bVar2) goto LAB_10027b9f8;
      if ((1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) break;
      pppppplVar7 = (long ******)((long)pppppplVar7 + 1);
      ppppppplVar4[5] = pppppplVar7;
      if (ppppppplVar4[4] == pppppplVar7) goto LAB_10027b494;
    }
    if ((ulong)bVar2 != 0x3a) {
LAB_10027b9f8:
      local_f8 = (long *******)&SUB_00000006;
      ppppppplVar6 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar4,&local_f8);
      ppppppplVar8 = extraout_x9_25;
      ppppppplVar9 = extraout_x10_19;
      goto LAB_10027b4b8;
    }
    ppppppplVar4[5] = (long ******)((long)pppppplVar7 + 1);
    deserialize_string(&local_88,ppppppplVar4);
    ppppppplVar8 = extraout_x9_01;
    ppppppplVar9 = extraout_x10_03;
    ppppppplVar6 = local_80;
    ppppppplVar11 = local_88;
    ppppppplVar14 = local_78;
    if (local_88 == (long *******)&DAT_8000000000000000) goto LAB_10027b4b8;
    goto LAB_10027b1c8;
  }
  if (local_e8 != (long *******)&DAT_0000000e) {
LAB_10027b810:
    ppppppplVar6 = (long *******)serde_core::de::Error::unknown_field(local_f0,local_e8,&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_753,3);
    ppppppplVar8 = extraout_x9_20;
    ppppppplVar9 = extraout_x10_14;
    goto LAB_10027b82c;
  }
  ppppppplVar6 = local_120;
  if (*local_f0 == (long ******)0x6c616e696769726f && *(long *)((long)local_f0 + 6) == 0x6c65646f6d5f6c61) {
    if (local_118 == (long *******)&DAT_8000000000000001) {
      pppppplVar7 = ppppppplVar4[5];
      if (ppppppplVar4[4] <= pppppplVar7) {
LAB_10027b704:
        local_f8 = (long *******)0x3;
        local_118 = (long *******)&DAT_8000000000000001;
        ppppppplVar6 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar4,&local_f8);
        ppppppplVar8 = extraout_x9_16;
        ppppppplVar9 = extraout_x10_13;
        goto LAB_10027b574;
      }
      while( true ) {
        bVar2 = *(byte *)((long)ppppppplVar4[3] + (long)pppppplVar7);
        if (0x3a < bVar2) goto LAB_10027baa8;
        if ((1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) break;
        pppppplVar7 = (long ******)((long)pppppplVar7 + 1);
        ppppppplVar4[5] = pppppplVar7;
        if (ppppppplVar4[4] == pppppplVar7) goto LAB_10027b704;
      }
      if ((ulong)bVar2 == 0x3a) {
        ppppppplVar4[5] = (long ******)((long)pppppplVar7 + 1);
        codexmate_lib::core::relay::codex_thread_visibility::NoAccountModelRestoreEntry>::deserialize::__Visitor_as_serde_core::de::Visitor>::visit_map::
        __DeserializeWith_as_serde_core::de::Deserialize>::deserialize(&local_a0,ppppppplVar4);
        local_118 = local_a0;
        if (local_a0 == (long *******)&DAT_8000000000000001) {
          bVar12 = false;
          ppppppplVar13 = (long *******)0x1;
          local_118 = (long *******)&DAT_8000000000000001;
          ppppppplVar8 = (long *******)&DAT_8000000000000001;
          ppppppplVar9 = extraout_x10_02;
          ppppppplVar6 = ppppppplStack_98;
          goto LAB_10027b834;
        }
        local_130 = local_90;
        local_128 = ppppppplStack_98;
        goto LAB_10027b1c8;
      }
LAB_10027baa8:
      local_f8 = (long *******)&SUB_00000006;
      local_118 = (long *******)&DAT_8000000000000001;
      ppppppplVar6 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar4,&local_f8);
      ppppppplVar8 = extraout_x9_28;
      ppppppplVar9 = extraout_x10_22;
      goto LAB_10027b574;
    }
    local_88 = (long *******)0x101307ace;
  }
  else {
    if (*local_f0 != (long ******)0x6465776f72726f62 || *(long *)((long)local_f0 + 6) != 0x6c65646f6d5f6465) goto LAB_10027b810;
    if (unaff_x24 == (long *******)&DAT_8000000000000000) {
      pppppplVar7 = ppppppplVar4[5];
      if (pppppplVar7 < ppppppplVar4[4]) {
        while( true ) {
          bVar2 = *(byte *)((long)ppppppplVar4[3] + (long)pppppplVar7);
          if (0x3a < bVar2) goto LAB_10027ba1c;
          if ((1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) break;
          pppppplVar7 = (long ******)((long)pppppplVar7 + 1);
          ppppppplVar4[5] = pppppplVar7;
          if (ppppppplVar4[4] == pppppplVar7) goto LAB_10027b4cc;
        }
        if ((ulong)bVar2 == 0x3a) {
          ppppppplVar4[5] = (long ******)((long)pppppplVar7 + 1);
          deserialize_string(&local_88,ppppppplVar4);
          local_110 = local_80;
          ppppppplVar13 = local_88;
          unaff_x25 = local_78;
          if (local_88 == (long *******)&DAT_8000000000000000) {
            bVar12 = false;
            ppppppplVar13 = (long *******)0x1;
            local_130 = local_80;
            ppppppplVar9 = extraout_x10_01;
            local_110 = (long *******)0x100002600;
            ppppppplVar6 = local_80;
            goto LAB_10027b840;
          }
          goto LAB_10027b1c8;
        }
LAB_10027ba1c:
        local_f8 = (long *******)&SUB_00000006;
        ppppppplVar6 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar4,&local_f8);
        local_130 = extraout_x9_26;
        ppppppplVar9 = extraout_x10_20;
      }
      else {
LAB_10027b4cc:
        local_f8 = (long *******)0x3;
        ppppppplVar6 = (long *******)serde_json::de::Deserializer<R>::peek_error(ppppppplVar4,&local_f8);
        local_130 = extraout_x9_05;
        ppppppplVar9 = extraout_x10_07;
      }
      bVar12 = false;
      ppppppplVar13 = (long *******)0x1;
      local_110 = (long *******)0x100002600;
      goto LAB_10027b840;
    }
    local_88 = (long *******)0x101307adc;
  }
LAB_10027b9cc:
  local_80 = local_e8;
  local_f8 = (long *******)&local_88;
  local_f0 = (long *******)&<&T_as_core::fmt::Display>::fmt;
  ppppppplVar6 = (long *******)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_15,&local_f8);
  ppppppplVar8 = extraout_x9_24;
  ppppppplVar9 = extraout_x10_18;
  goto LAB_10027b574;
LAB_10027b4b8:
  bVar12 = false;
  ppppppplVar13 = (long *******)0x1;
  ppppppplVar11 = (long *******)&DAT_8000000000000000;
LAB_10027b834:
  local_130 = ppppppplVar8;
  if (((ulong)unaff_x24 & 0x7fffffffffffffff) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_110,unaff_x24,1);
    local_130 = extraout_x9_22;
    ppppppplVar9 = extraout_x10_16;
  }
  if ((int)ppppppplVar13 != 0) {
LAB_10027b840:
    if ((-0x7fffffffffffffff < (long)local_118) && (local_118 != (long *******)0x0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,local_118,1);
      local_130 = extraout_x9_21;
      ppppppplVar9 = extraout_x10_15;
    }
  }
  if (((ulong)ppppppplVar11 & 0x7fffffffffffffff) == 0) {
    bVar12 = true;
  }
  local_128 = ppppppplVar9;
  if (!bVar12) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,ppppppplVar11,1);
    local_130 = extraout_x9_23;
    local_128 = extraout_x10_17;
  }
  ppppppplVar11 = (long *******)&DAT_8000000000000000;
  local_120 = ppppppplVar6;
  ppppppplVar14 = ppppppplVar13;
LAB_10027b89c:
  *(char *)(param_2 + 7) = *(char *)(param_2 + 7) + '\x01';
  local_f8 = ppppppplVar11;
  local_f0 = local_120;
  local_e8 = ppppppplVar14;
  ppppppplStack_e0 = unaff_x24;
  local_d8 = local_110;
  local_c8 = local_118;
  ppppppplStack_c0 = local_128;
  local_b8 = local_130;
  ppppppplVar6 = (long *******)serde_json::de::Deserializer<R>::end_map(param_2);
  unaff_x25 = local_118;
  unaff_x26 = local_110;
  ppppppplVar13 = local_120;
  if (ppppppplVar11 == (long *******)&DAT_8000000000000000) {
    if (ppppppplVar6 != (long *******)0x0) {
      core::ptr::drop_in_place<serde_json::error::ErrorCode>();
LAB_10027b96c:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar6,0x28,8);
    }
  }
  else {
joined_r0x00010027b7c0:
    local_120 = ppppppplVar6;
    if (local_120 == (long *******)0x0) {
      param_1[2] = ppppppplVar14;
      param_1[3] = unaff_x24;
      param_1[4] = unaff_x26;
      param_1[5] = ppppppplVar10;
      param_1[6] = unaff_x25;
      param_1[7] = local_128;
      param_1[8] = local_130;
      *param_1 = ppppppplVar11;
      param_1[1] = ppppppplVar13;
      return;
    }
    if (ppppppplVar11 != (long *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar13,ppppppplVar11,1);
    }
    if (((ulong)unaff_x25 & 0x7fffffffffffffff) != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,unaff_x25,1);
    }
    if (unaff_x24 != (long *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x26,unaff_x24,1);
    }
  }
LAB_10027b980:
  uVar5 = serde_json::error::Error::fix_position(local_120,param_2);
LAB_10027b988:
  *param_1 = 0x8000000000000000;
  param_1[1] = uVar5;
  return;
}
