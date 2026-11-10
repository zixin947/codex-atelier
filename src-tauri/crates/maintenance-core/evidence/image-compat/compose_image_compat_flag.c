/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* compose_image_compat_flag @ 1007b5bf0 requested 1007b5bf0 */

/* codexmate_lib::commands::system::compose_image_compat_flag */

void codexmate_lib::commands::system::compose_image_compat_flag(ulong *param_1,ulong param_2,ulong param_3,int param_4)

{
  int iVar1;
  uint uVar2;
  bool bVar3;
  bool bVar4;
  long lVar5;
  ushort uVar6;
  code *pcVar7;
  bool bVar8;
  bool bVar9;
  bool bVar10;
  ulong *puVar11;
  long lVar12;
  long *plVar13;
  undefined8 *puVar14;
  long extraout_x1;
  long extraout_x1_00;
  ulong uVar15;
  ulong uVar16;
  char cVar17;
  undefined8 *puVar18;
  undefined8 *puVar19;
  long lVar20;
  bool bVar21;
  ulong unaff_x25;
  ulong uVar22;
  undefined1 auVar23 [16];
  ulong local_158;
  undefined8 local_138;
  ulong uStack_130;
  ulong local_128;
  ulong uStack_120;
  undefined8 local_118;
  ulong uStack_110;
  undefined8 local_108;
  undefined1 local_100;
  undefined2 local_f8;
  ulong local_f0;
  undefined8 *local_e8;
  ulong local_e0;
  ulong local_d8;
  ulong uStack_d0;
  ulong local_c8;
  ulong local_c0;
  undefined8 uStack_b8;
  ulong local_b0;
  undefined8 local_a8;
  undefined1 local_a0;
  undefined2 local_98;
  ulong local_90;
  ulong uStack_88;
  ulong local_80;
  
  if (param_4 == 0) {
    bVar21 = false;
    bVar8 = false;
    bVar3 = false;
    local_f0 = 0;
    local_e8 = (undefined8 *)0x8;
    local_e0 = 0;
    local_d8 = 0;
    uStack_b8 = 0;
    local_a8 = 0xa0000000a;
    local_a0 = 1;
    local_98 = 0;
    uStack_d0 = param_3;
    local_c8 = param_2;
    local_c0 = param_3;
    local_b0 = param_3;
    do {
      while( true ) {
        uVar22 = local_c8;
        <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(&local_90,&local_c8);
        uVar6 = local_98;
        if ((int)local_90 == 1) {
          lVar12 = local_80 - local_d8;
          uVar16 = local_80;
        }
        else {
          if ((local_98 & 0x100) != 0) goto LAB_1007b6230;
          local_98 = CONCAT11(1,(undefined1)local_98);
          if (((uVar6 & 1) == 0) && (uStack_d0 == local_d8)) goto LAB_1007b6230;
          lVar12 = uStack_d0 - local_d8;
          uVar16 = local_d8;
          uVar22 = local_c8;
        }
        lVar5 = uVar22 + local_d8;
        lVar20 = lVar12;
        if ((lVar12 != 0) && (*(char *)(lVar5 + lVar12 + -1) == '\n')) {
          if (lVar12 + -1 == 0) {
            lVar20 = 0;
          }
          else {
            lVar20 = lVar12 + -1;
            if (*(char *)(lVar5 + lVar12 + -2) == '\r') {
              lVar20 = lVar12 + -2;
            }
          }
        }
        local_d8 = uVar16;
        auVar23 = ::core::str::<impl_str>::trim_matches(lVar5,lVar20);
        uVar22 = auVar23._8_8_;
        plVar13 = auVar23._0_8_;
        if (uVar22 != 0) break;
LAB_1007b60e0:
        uVar16 = local_e0;
        if (local_e0 == local_f0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_f0);
        }
LAB_1007b5fe0:
        local_e8[uVar16 * 2] = lVar5;
        (local_e8 + uVar16 * 2)[1] = lVar20;
        local_e0 = uVar16 + 1;
        if ((local_98 & 0x100) != 0) goto LAB_1007b6230;
      }
      cVar17 = (char)*plVar13;
      if (cVar17 == '[') {
        if (((bool)(bVar8 & (bVar3 ^ 1U))) && (bVar21)) {
          if (local_e0 <= param_3) {
                    /* WARNING: Subroutine does not return */
            alloc::vec::Vec<T,A>::remove::assert_failed(param_3,local_e0,&PTR_s_src_commands_system_rs_10182b968);
          }
          puVar14 = local_e8 + param_3 * 2;
          while( true ) {
            uVar16 = local_e0;
            _memmove(puVar14,puVar14 + 2,(local_e0 + ~param_3) * 0x10);
            local_e0 = uVar16 - 1;
            if ((local_e0 <= param_3) || (::core::str::<impl_str>::trim_matches(local_e8[param_3 * 2],(local_e8 + param_3 * 2)[1]), extraout_x1 != 0)) break;
            if (local_e0 <= param_3) {
                    /* WARNING: Subroutine does not return */
              alloc::vec::Vec<T,A>::remove::assert_failed(param_3,local_e0,&PTR_s_src_commands_system_rs_10182b980);
            }
            puVar14 = local_e8 + param_3 * 2;
          }
        }
        uVar16 = local_e0;
        if (uVar22 == 10) {
          bVar9 = *plVar13 == 0x657275746165665b;
          bVar10 = (short)plVar13[1] == 0x5d73;
          bVar8 = bVar9 && bVar10;
          bVar4 = false;
          if (!bVar9 || !bVar10) {
            bVar4 = bVar3;
          }
          uVar22 = local_e0;
          bVar3 = bVar4;
          bVar4 = true;
          if (!bVar9 || !bVar10) {
            uVar22 = param_3;
            bVar4 = bVar21;
          }
        }
        else {
          bVar8 = false;
          uVar22 = param_3;
          bVar4 = bVar21;
        }
        bVar21 = bVar4;
        param_3 = uVar22;
        if (local_e0 == local_f0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_f0);
        }
        goto LAB_1007b5fe0;
      }
      if (!bVar8) goto LAB_1007b60e0;
      if ((uVar22 < 0x10) || (*plVar13 != 0x65675f6567616d69 || plVar13[1] != 0x6e6f69746172656e)) {
LAB_1007b60d4:
        bVar3 = (bool)(cVar17 != '#' | bVar3);
        goto LAB_1007b60e0;
      }
      auVar23 = ::core::str::<impl_str>::trim_start_matches(plVar13 + 2,uVar22 - 0x10);
      if ((auVar23._8_8_ == 0) || (*auVar23._0_8_ != '=')) goto LAB_1007b60d4;
      lVar12 = ::core::str::<impl_str>::trim_matches(auVar23._0_8_ + 1,auVar23._8_8_ + -1);
      if (lVar12 == 0) {
        cVar17 = (char)*plVar13;
        goto LAB_1007b60d4;
      }
      bVar8 = true;
    } while ((local_98 & 0x100) == 0);
LAB_1007b6230:
    uVar22 = local_e0;
    if ((((bVar8) && (!bVar3)) && (bVar21)) && (param_3 < local_e0)) {
      _memmove(local_e8 + param_3 * 2,local_e8 + param_3 * 2 + 2,(local_e0 + ~param_3) * 0x10);
      while ((local_e0 = uVar22 - 1, param_3 < local_e0 &&
             (::core::str::<impl_str>::trim_matches(local_e8[param_3 * 2],(local_e8 + param_3 * 2)[1]), uVar22 = local_e0, extraout_x1_00 == 0))) {
        if (local_e0 <= param_3) {
                    /* WARNING: Subroutine does not return */
          alloc::vec::Vec<T,A>::remove::assert_failed(param_3,local_e0,&PTR_s_src_commands_system_rs_10182b950);
        }
        _memmove(local_e8 + param_3 * 2,local_e8 + param_3 * 2 + 2,(local_e0 + ~param_3) * 0x10);
      }
    }
    alloc::str::join_generic_copy(&local_d8,local_e8,local_e0,&_anon_6408540cdffbe318d49f52812e99783f_174,1);
    uVar22 = local_c8;
    uStack_88 = uStack_d0;
    local_90 = local_d8;
    local_80 = local_c8;
    if ((local_c8 != 0) && (*(char *)(uStack_d0 + local_c8 + -1) != '\n')) {
      if (local_d8 == local_c8) {
        alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_90,local_c8,1,1,1);
      }
      *(undefined1 *)(uStack_88 + local_80) = 10;
      local_80 = uVar22 + 1;
    }
    param_1[1] = uStack_88;
    *param_1 = local_90;
    param_1[2] = local_80;
    goto joined_r0x0001007b634c;
  }
  local_138 = 0;
  local_118 = 0;
  local_108 = 0xa0000000a;
  local_100 = 1;
  local_f8 = 0;
  uStack_130 = param_3;
  local_128 = param_2;
  uStack_120 = param_3;
  uStack_110 = param_3;
  <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_f0,&local_138);
  if (local_e0 == 0) {
LAB_1007b5eb8:
    if (param_3 == 0) {
      uVar22 = local_e0;
      if (local_e0 == local_f0) goto LAB_1007b5f20;
    }
    else {
      if (*(char *)(param_2 + param_3 + -1) != '\n') {
        if (local_e0 == local_f0) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_f0);
        }
        local_e8[local_e0 * 2] = 1;
        (local_e8 + local_e0 * 2)[1] = 0;
        local_e0 = local_e0 + 1;
      }
      uVar22 = local_e0;
      if (local_e0 == local_f0) {
LAB_1007b5f20:
        uVar22 = local_e0;
        alloc::raw_vec::RawVec<T,A>::grow_one(&local_f0);
      }
    }
    local_e8[uVar22 * 2] = "[features]image_generation = false";
    (local_e8 + uVar22 * 2)[1] = 10;
    uVar16 = uVar22 + 1;
    if (uVar16 == local_f0) {
      local_e0 = uVar16;
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_f0);
    }
    local_e8[uVar16 * 2] = "image_generation = false";
    (local_e8 + uVar16 * 2)[1] = 0x18;
    local_e0 = uVar22 + 2;
  }
  else {
    bVar3 = false;
    bVar21 = false;
    bVar8 = false;
    uVar22 = 0;
    puVar14 = local_e8 + local_e0 * 2;
    puVar19 = local_e8;
LAB_1007b5cb8:
    do {
      puVar18 = puVar19 + 2;
      auVar23 = ::core::str::<impl_str>::trim_matches(*puVar19,puVar19[1]);
      uVar16 = auVar23._8_8_;
      puVar11 = auVar23._0_8_;
      puVar19 = puVar18;
      if (uVar16 != 0) {
        if ((char)*puVar11 == '[') {
          if (bVar3) break;
          if (uVar16 == 10) {
            uVar16 = (*puVar11 & 0xff00ff00ff00ff00) >> 8 | (*puVar11 & 0xff00ff00ff00ff) << 8;
            uVar16 = (uVar16 & 0xffff0000ffff0000) >> 0x10 | (uVar16 & 0xffff0000ffff) << 0x10;
            uVar15 = uVar16 >> 0x20 | uVar16 << 0x20;
            uVar16 = 0x5b66656174757265;
            if (uVar15 == 0x5b66656174757265) {
              uVar2 = (uint)(ushort)((ushort)puVar11[1] >> 8) | ((ushort)puVar11[1] & 0xff00ff) << 8;
              uVar15 = (ulong)uVar2;
              if (uVar2 != 0x735d) {
                uVar16 = 0x735d;
                goto LAB_1007b5c94;
              }
            }
            else {
LAB_1007b5c94:
              iVar1 = 1;
              if (uVar15 < uVar16) {
                iVar1 = -1;
              }
              if (iVar1 != 0) goto LAB_1007b5ca0;
            }
            bVar3 = true;
            bVar21 = true;
            unaff_x25 = uVar22;
          }
          else {
LAB_1007b5ca0:
            bVar3 = false;
          }
          uVar22 = uVar22 + 1;
          if (puVar18 == puVar14) break;
          goto LAB_1007b5cb8;
        }
        if (bVar3) {
          if ((0xf < uVar16) && (*puVar11 == 0x65675f6567616d69 && puVar11[1] == 0x6e6f69746172656e)) {
            auVar23 = ::core::str::<impl_str>::trim_start_matches(puVar11 + 2,uVar16 - 0x10);
            if ((auVar23._8_8_ != 0) && (*auVar23._0_8_ == '=')) {
              lVar12 = ::core::str::<impl_str>::trim_matches(auVar23._0_8_ + 1,auVar23._8_8_ + -1);
              uVar16 = uVar22;
              if (lVar12 == 0) {
                uVar16 = local_158;
              }
              bVar8 = (bool)(lVar12 != 0 | bVar8);
              local_158 = uVar16;
            }
          }
          bVar3 = true;
        }
        else {
          bVar3 = false;
        }
      }
      uVar22 = uVar22 + 1;
    } while (puVar18 != puVar14);
    if (bVar8) {
      if (local_e0 <= local_158) {
                    /* WARNING: Subroutine does not return */
        ::core::panicking::panic_bounds_check(local_158,local_e0,&PTR_s_src_commands_system_rs_10182b998);
      }
      local_e8[local_158 * 2] = "image_generation = false";
      (local_e8 + local_158 * 2)[1] = 0x18;
    }
    else {
      if (!bVar21) goto LAB_1007b5eb8;
      uVar22 = unaff_x25 + 1;
      if (local_e0 < uVar22) {
        alloc::vec::Vec<T,A>::insert_mut::assert_failed(uVar22,local_e0,&PTR_s_src_commands_system_rs_10182b9b0);
                    /* WARNING: Does not return */
        pcVar7 = (code *)SoftwareBreakpoint(1,0x1007b6434);
        (*pcVar7)();
      }
      if (local_e0 == local_f0) {
        alloc::raw_vec::RawVec<T,A>::grow_one(&local_f0);
      }
      puVar14 = local_e8 + uVar22 * 2;
      if (uVar22 <= local_e0 && local_e0 - uVar22 != 0) {
        _memmove(puVar14 + 2,puVar14,(local_e0 - uVar22) * 0x10);
      }
      *puVar14 = "image_generation = false";
      puVar14[1] = 0x18;
      local_e0 = local_e0 + 1;
    }
  }
  alloc::str::join_generic_copy(&local_d8,local_e8,local_e0,&_anon_6408540cdffbe318d49f52812e99783f_174,1);
  uStack_88 = uStack_d0;
  local_90 = local_d8;
  local_80 = local_c8;
  if (local_d8 == local_c8) {
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_90,local_c8,1,1,1);
  }
  *(undefined1 *)(uStack_88 + local_80) = 10;
  param_1[1] = uStack_88;
  *param_1 = local_90;
  param_1[2] = local_80 + 1;
joined_r0x0001007b634c:
  if (local_f0 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_e8,local_f0 << 4,8);
  }
  return;
}

