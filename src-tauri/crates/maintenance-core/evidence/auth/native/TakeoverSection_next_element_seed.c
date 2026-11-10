/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* next_element_seed @ 100237ac0 requested 100237ac0 */

/* WARNING: Type propagation algorithm not settling */
/* <serde_json::de::SeqAccess<R> as serde_core::de::SeqAccess>::next_element_seed */

void <serde_json::de::SeqAccess<R>as_serde_core::de::SeqAccess>::next_element_seed(undefined8 *param_1,undefined8 *param_2)

{
  char cVar1;
  byte bVar2;
  undefined8 *******pppppppuVar3;
  undefined8 uVar4;
  int *piVar5;
  int *piVar6;
  undefined8 ******ppppppuVar7;
  undefined8 uVar8;
  undefined8 *******pppppppuVar9;
  undefined8 *******pppppppuVar10;
  int *unaff_x25;
  undefined8 *******unaff_x26;
  long unaff_x27;
  long lVar11;
  long lVar12;
  long unaff_x28;
  long lVar13;
  int *local_e8;
  int *local_e0;
  undefined8 local_d8;
  int *local_d0;
  long local_c8;
  undefined8 *******pppppppuStack_c0;
  int *local_b8;
  long lStack_b0;
  undefined8 *******local_a8;
  undefined1 local_a0;
  undefined8 *******local_98;
  int *piStack_90;
  undefined8 *******local_80;
  int *local_78;
  long local_70;
  undefined1 uStack_61;

  next_element_seed::has_next_element(&local_d8,param_2);
  if ((char)local_d8 == '\x01') {
    *param_1 = 0x8000000000000001;
    param_1[1] = local_d0;
    return;
  }
  if (((ulong)local_d8 & 0x100) == 0) {
    uVar8 = 0x8000000000000000;
    goto LAB_100237b7c;
  }
  pppppppuVar9 = (undefined8 *******)*param_2;
  ppppppuVar7 = pppppppuVar9[5];
  if (ppppppuVar7 < pppppppuVar9[4]) {
    do {
      bVar2 = *(byte *)((long)pppppppuVar9[3] + (long)ppppppuVar7);
      if (0x20 < bVar2 || (1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) {
        if (bVar2 == 0x5b) {
          cVar1 = *(char *)(pppppppuVar9 + 7);
          *(char *)(pppppppuVar9 + 7) = cVar1 + -1;
          if ((char)(cVar1 + -1) != '\0') {
            pppppppuVar9[5] = (undefined8 ******)((long)ppppppuVar7 + 1);
            local_78 = (int *)CONCAT71(local_78._1_7_,1);
            local_80 = pppppppuVar9;
            next_element_seed::has_next_element(&local_d8,&local_80);
            lVar13 = unaff_x28;
            lVar12 = unaff_x27;
            if ((char)local_d8 == '\x01') {
              pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
              piVar5 = local_d0;
            }
            else if (local_d8._1_1_ == '\x01') {
              <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_string(&local_d8,local_80);
              lVar13 = local_c8;
              piVar5 = local_d0;
              pppppppuVar10 = local_d8;
              if (local_d8 == (undefined8 *******)&DAT_8000000000000000) {
                pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
                lVar13 = unaff_x28;
              }
              else {
                next_element_seed::has_next_element(&local_d8,&local_80);
                unaff_x25 = local_d0;
                if ((char)local_d8 != '\x01') {
                  if (local_d8._1_1_ == '\x01') {
                    <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_seq(&local_d8,local_80);
                    unaff_x25 = local_d0;
                    unaff_x26 = local_d8;
                    lVar12 = local_c8;
                    if (local_d8 != (undefined8 *******)&DAT_8000000000000000) goto LAB_100238008;
                  }
                  else {
                    unaff_x25 = (int *)serde_core::de::Error::invalid_length(1,&PTR_s_struct_TakeoverSection_with_2_el_10130772a_0x1e9_101817bc8,&DAT_101817930)
                    ;
                  }
                }
                if (pppppppuVar10 != (undefined8 *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(piVar5,pppppppuVar10,1);
                }
                pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
                piVar5 = unaff_x25;
                lVar12 = unaff_x27;
              }
            }
            else {
              piVar5 = (int *)serde_core::de::Error::invalid_length(0,&PTR_s_struct_TakeoverSection_with_2_el_10130772a_0x1e9_101817bc8,&DAT_101817930);
              pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
            }
LAB_100238008:
            *(char *)(pppppppuVar9 + 7) = *(char *)(pppppppuVar9 + 7) + '\x01';
            local_d8 = pppppppuVar10;
            local_d0 = piVar5;
            pppppppuStack_c0 = unaff_x26;
            local_b8 = unaff_x25;
            lStack_b0 = lVar12;
            piVar6 = (int *)serde_json::de::Deserializer<R>::end_seq(pppppppuVar9);
            if (pppppppuVar10 == (undefined8 *******)&DAT_8000000000000000) {
              if (piVar6 == (int *)0x0) goto LAB_1002381e4;
              core::ptr::drop_in_place<serde_json::error::ErrorCode>(piVar6);
              goto LAB_10023819c;
            }
            if (piVar6 == (int *)0x0) goto LAB_100238164;
            if (pppppppuVar10 != (undefined8 *******)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(piVar5,pppppppuVar10,1);
            }
            if (lVar12 != 0) {
              piVar5 = unaff_x25 + 2;
              do {
                if (*(long *)(piVar5 + -2) != 0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)piVar5,*(long *)(piVar5 + -2),1);
                }
                piVar5 = piVar5 + 6;
                lVar12 = lVar12 + -1;
              } while (lVar12 != 0);
            }
            goto LAB_1002381cc;
          }
        }
        else {
          if (bVar2 != 0x7b) {
            piVar5 = (int *)serde_json::de::Deserializer<R>::peek_invalid_type(pppppppuVar9,&uStack_61,&DAT_101818cf0);
            goto LAB_1002381e4;
          }
          cVar1 = *(char *)(pppppppuVar9 + 7);
          *(char *)(pppppppuVar9 + 7) = cVar1 + -1;
          if ((char)(cVar1 + -1) != '\0') {
            pppppppuVar9[5] = (undefined8 ******)((long)ppppppuVar7 + 1);
            local_a0 = 1;
            pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
            pppppppuVar3 = (undefined8 *******)&DAT_8000000000000000;
            piVar5 = local_e8;
            piVar6 = local_e0;
            local_a8 = pppppppuVar9;
            goto LAB_100237bcc;
          }
        }
        local_d8 = (undefined8 *******)0x18;
        goto LAB_100237b64;
      }
      ppppppuVar7 = (undefined8 ******)((long)ppppppuVar7 + 1);
      pppppppuVar9[5] = ppppppuVar7;
    } while (pppppppuVar9[4] != ppppppuVar7);
  }
  local_d8 = (undefined8 *******)0x5;
LAB_100237b64:
  uVar4 = serde_json::de::Deserializer<R>::peek_error(pppppppuVar9,&local_d8);
  goto LAB_1002381f0;
LAB_100237bcc:
  local_e0 = piVar6;
  local_e8 = piVar5;
  lVar13 = unaff_x28;
  lVar12 = unaff_x27;
  unaff_x26 = pppppppuVar3;
  <serde_json::de::MapAccess<R>as_serde_core::de::MapAccess>::next_key_seed::has_next_key(&local_d8,&local_a8);
  pppppppuVar3 = local_a8;
  local_98 = unaff_x26;
  piStack_90 = local_e0;
  piVar5 = local_d0;
  if ((char)local_d8 != '\x01') {
    if (local_d8._1_1_ != '\x01') {
      if (pppppppuVar10 == (undefined8 *******)&DAT_8000000000000000) {
        local_80 = (undefined8 *******)0x10130618d;
        local_78 = (int *)0x3;
        local_d8 = &local_80;
        local_d0 = (int *)&<&T_as_core::fmt::Display>::fmt;
        pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
        piVar5 = (int *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_d8);
        goto joined_r0x000100237f40;
      }
      piVar5 = local_e8;
      unaff_x25 = local_e0;
      if (unaff_x26 == (undefined8 *******)&DAT_8000000000000000) {
        local_80 = (undefined8 *******)0x101306155;
        local_78 = (int *)&DAT_00000005;
        local_d8 = &local_80;
        local_d0 = (int *)&<&T_as_core::fmt::Display>::fmt;
        piVar5 = (int *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_11,&local_d8);
        if (pppppppuVar10 != (undefined8 *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_e8,pppppppuVar10,1);
        }
        pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
      }
      goto LAB_100238130;
    }
    local_a8[5] = (undefined8 ******)((long)local_a8[5] + 1);
    local_a8[2] = (undefined8 ******)0x0;
    <serde_json::read::SliceRead_as_serde_json::read::Read>::parse_str(&local_d8,local_a8 + 3,local_a8);
    piVar5 = local_d0;
    if (local_d8 == (undefined8 *******)0x2) goto joined_r0x000100237f40;
    if (local_c8 != 3) {
      if (local_c8 != 5) {
LAB_100238260:
        piVar5 = (int *)serde_core::de::Error::unknown_field(local_d0,local_c8,&PTR__anon_dc5b8adae996cb58fd4173e320c2d1cf_377_101817280,2);
        goto joined_r0x000100237f40;
      }
      if (*local_d0 != 0x656e696c || (char)local_d0[1] != 's') {
        local_c8 = 5;
        goto LAB_100238260;
      }
      if (unaff_x26 != (undefined8 *******)&DAT_8000000000000000) {
        local_80 = (undefined8 *******)0x101306155;
        local_78 = (int *)&DAT_00000005;
        goto LAB_1002382b0;
      }
      ppppppuVar7 = pppppppuVar3[5];
      if (pppppppuVar3[4] <= ppppppuVar7) {
LAB_100237e40:
        local_d8 = (undefined8 *******)0x3;
        piVar5 = (int *)serde_json::de::Deserializer<R>::peek_error(pppppppuVar3,&local_d8);
        goto LAB_100238110;
      }
      while( true ) {
        bVar2 = *(byte *)((long)pppppppuVar3[3] + (long)ppppppuVar7);
        if (0x3a < bVar2) goto LAB_1002382ec;
        if ((1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) break;
        ppppppuVar7 = (undefined8 ******)((long)ppppppuVar7 + 1);
        pppppppuVar3[5] = ppppppuVar7;
        if (pppppppuVar3[4] == ppppppuVar7) goto LAB_100237e40;
      }
      if ((ulong)bVar2 != 0x3a) {
LAB_1002382ec:
        local_d8 = (undefined8 *******)&SUB_00000006;
        piVar5 = (int *)serde_json::de::Deserializer<R>::peek_error(pppppppuVar3,&local_d8);
        goto LAB_100238110;
      }
      pppppppuVar3[5] = (undefined8 ******)((long)ppppppuVar7 + 1);
      <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_seq(&local_80,pppppppuVar3);
      pppppppuVar3 = local_80;
      unaff_x27 = local_70;
      unaff_x28 = lVar13;
      piVar5 = local_e8;
      piVar6 = local_78;
      if (local_80 == (undefined8 *******)&DAT_8000000000000000) goto LAB_100238090;
      goto LAB_100237bcc;
    }
    if ((short)*local_d0 != 0x656b || *(char *)((long)local_d0 + 2) != 'y') {
      local_c8 = 3;
      goto LAB_100238260;
    }
    if (pppppppuVar10 == (undefined8 *******)&DAT_8000000000000000) {
      ppppppuVar7 = pppppppuVar3[5];
      if (ppppppuVar7 < pppppppuVar3[4]) {
        while( true ) {
          bVar2 = *(byte *)((long)pppppppuVar3[3] + (long)ppppppuVar7);
          if (0x3a < bVar2) goto LAB_10023831c;
          if ((1L << ((ulong)bVar2 & 0x3f) & 0x100002600U) == 0) break;
          ppppppuVar7 = (undefined8 ******)((long)ppppppuVar7 + 1);
          pppppppuVar3[5] = ppppppuVar7;
          if (pppppppuVar3[4] == ppppppuVar7) goto LAB_100237df8;
        }
        if ((ulong)bVar2 == 0x3a) {
          pppppppuVar3[5] = (undefined8 ******)((long)ppppppuVar7 + 1);
          <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_string(&local_80,pppppppuVar3);
          pppppppuVar10 = local_80;
          pppppppuVar3 = unaff_x26;
          unaff_x27 = lVar12;
          unaff_x28 = local_70;
          piVar5 = local_78;
          piVar6 = local_e0;
          if (local_80 == (undefined8 *******)&DAT_8000000000000000) {
            pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
            piVar5 = local_78;
            goto joined_r0x000100237f40;
          }
          goto LAB_100237bcc;
        }
LAB_10023831c:
        local_d8 = (undefined8 *******)&SUB_00000006;
        piVar5 = (int *)serde_json::de::Deserializer<R>::peek_error(pppppppuVar3,&local_d8);
      }
      else {
LAB_100237df8:
        local_d8 = (undefined8 *******)0x3;
        piVar5 = (int *)serde_json::de::Deserializer<R>::peek_error(pppppppuVar3,&local_d8);
      }
      pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
      goto joined_r0x000100237f40;
    }
    local_80 = (undefined8 *******)0x10130618d;
    local_78 = (int *)0x3;
LAB_1002382b0:
    local_d8 = &local_80;
    local_d0 = (int *)&<&T_as_core::fmt::Display>::fmt;
    piVar5 = (int *)<serde_json::error::Error_as_serde_core::de::Error>::custom(_anon_b5f536351c4bf1e454cf90283dd9e625_15,&local_d8);
  }
joined_r0x000100237f40:
  if (unaff_x26 != (undefined8 *******)&DAT_8000000000000000) {
    if (lVar12 != 0) {
      piVar6 = local_e0 + 2;
      lVar11 = lVar12;
      do {
        if (*(long *)(piVar6 + -2) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)piVar6,*(long *)(piVar6 + -2),1);
        }
        piVar6 = piVar6 + 6;
        lVar11 = lVar11 + -1;
        lVar12 = 0;
      } while (lVar11 != 0);
    }
    if (unaff_x26 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_e0,(long)unaff_x26 * 0x18,8);
    }
  }
  goto LAB_100238110;
LAB_100238090:
  piVar5 = local_78;
LAB_100238110:
  if (((ulong)pppppppuVar10 & 0x7fffffffffffffff) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_e8,pppppppuVar10,1);
  }
  pppppppuVar10 = (undefined8 *******)&DAT_8000000000000000;
  unaff_x25 = local_e8;
LAB_100238130:
  *(char *)(pppppppuVar9 + 7) = *(char *)(pppppppuVar9 + 7) + '\x01';
  local_d8 = pppppppuVar10;
  local_d0 = piVar5;
  pppppppuStack_c0 = unaff_x26;
  local_b8 = unaff_x25;
  lStack_b0 = lVar12;
  piVar6 = (int *)serde_json::de::Deserializer<R>::end_map(pppppppuVar9);
  if (pppppppuVar10 == (undefined8 *******)&DAT_8000000000000000) {
    if (piVar6 != (int *)0x0) {
      core::ptr::drop_in_place<serde_json::error::ErrorCode>(piVar6);
LAB_10023819c:
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(piVar6,0x28,8);
    }
  }
  else {
    if (piVar6 == (int *)0x0) {
LAB_100238164:
      param_1[2] = lVar13;
      param_1[3] = unaff_x26;
      param_1[4] = unaff_x25;
      param_1[5] = lVar12;
      *param_1 = pppppppuVar10;
      param_1[1] = piVar5;
      return;
    }
    if (pppppppuVar10 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(piVar5,pppppppuVar10,1);
    }
    if (lVar12 != 0) {
      piVar5 = unaff_x25 + 2;
      do {
        if (*(long *)(piVar5 + -2) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*(undefined8 *)piVar5,*(long *)(piVar5 + -2),1);
        }
        piVar5 = piVar5 + 6;
        lVar12 = lVar12 + -1;
      } while (lVar12 != 0);
    }
LAB_1002381cc:
    piVar5 = piVar6;
    if (unaff_x26 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(unaff_x25,(long)unaff_x26 * 0x18,8);
    }
  }
LAB_1002381e4:
  uVar4 = serde_json::error::Error::fix_position(piVar5,pppppppuVar9);
LAB_1002381f0:
  uVar8 = 0x8000000000000001;
  param_1[1] = uVar4;
LAB_100237b7c:
  *param_1 = uVar8;
  return;
}
