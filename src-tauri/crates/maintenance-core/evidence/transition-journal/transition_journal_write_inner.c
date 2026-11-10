/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* write_inner @ 10078e254 requested 10078e254 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::transition_journal::write_inner */

void codexmate_lib::core::relay::transition_journal::write_inner
               (ulong *param_1,long param_2,uint param_3,void *param_4,code *param_5,undefined8 param_6,undefined8 param_7,void *param_8,size_t param_9,
               byte param_10)

{
  ulong uVar1;
  ulong uVar2;
  ulong uVar3;
  ulong uVar4;
  uint uVar5;
  code *pcVar6;
  long *plVar7;
  ulong uVar8;
  long *******ppppppplVar9;
  undefined8 *******pppppppuVar10;
  undefined8 *******pppppppuVar11;
  void *pvVar12;
  code *pcVar13;
  long *******ppppppplVar14;
  undefined8 uVar15;
  undefined8 *****pppppuVar16;
  size_t sVar17;
  undefined8 uVar18;
  undefined8 *puVar19;
  int iVar20;
  undefined8 uVar21;
  undefined8 *puVar22;
  int iVar23;
  undefined1 auVar24 [16];
  byte local_180;
  code *local_170;
  long *plStack_168;
  code *local_160;
  ulong uStack_158;
  code *local_150;
  ulong uStack_148;
  ulong local_140;
  ulong uStack_138;
  undefined8 local_130;
  long *******local_120;
  code *pcStack_118;
  long *plStack_110;
  ulong local_108;
  undefined1 uStack_100;
  undefined7 uStack_ff;
  ulong local_f8;
  ulong uStack_f0;
  ulong local_e8;
  ulong uStack_e0;
  ulong local_d8;
  ulong local_d0;
  ulong uStack_c8;
  undefined8 *****local_c0;
  long *******local_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  ushort local_a0 [4];
  undefined8 *******local_98;
  undefined8 *****local_90;
  undefined8 *******local_88;
  long *******local_80;
  code *pcStack_78;
  
  chrono::offset::utc::Utc::now(&local_120);
  pcVar13 = pcStack_118;
  uVar5 = (uint)local_120;
  iVar23 = (int)(uint)local_120 >> 0xd;
  iVar20 = iVar23 + -1;
  if (iVar23 < 1) {
    iVar23 = (1U - iVar23) / 400 + 1;
    iVar20 = iVar20 + iVar23 * 400;
    iVar23 = iVar23 * -0x23ab1;
  }
  else {
    iVar23 = 0;
  }
  uVar1 = (ulong)local_120 >> 0x20;
  read_checked(&local_120,param_2);
  uVar4 = local_e8;
  uVar8 = local_f8;
  uVar2 = CONCAT71(uStack_ff,uStack_100);
  uStack_148 = uStack_f0;
  local_150 = (code *)local_f8;
  uStack_138 = uStack_e0;
  local_140 = local_e8;
  local_130 = local_d8;
  plStack_168 = plStack_110;
  local_170 = pcStack_118;
  local_160 = (code *)local_108;
  if (local_120 != (long *******)0xb) {
    param_1[0xb] = uStack_c8;
    param_1[10] = local_d0;
    param_1[4] = uVar2;
    param_1[3] = local_108;
    param_1[6] = uStack_f0;
    param_1[5] = local_f8;
    param_1[8] = uStack_e0;
    param_1[7] = local_e8;
    param_1[9] = local_d8;
    param_1[2] = (ulong)plStack_110;
    param_1[1] = (ulong)pcStack_118;
    *param_1 = (ulong)local_120;
    return;
  }
  uVar1 = (uVar1 + (long)(int)((((uVar5 >> 4 & 0x1ff) + iVar23) - iVar20 / 100) + (iVar20 * 0x5b5 >> 2) + (iVar20 / 100 >> 2) + -0xaf93b) * 0x15180) * 1000 +
          ((ulong)pcVar13 & 0xffffffff) / 1000000;
  local_180 = (byte)local_d8;
  if (((param_10 & 1) == 0) && (pcStack_118 != (code *)0x8000000000000000)) {
    if ((local_108 == 10) && (*plStack_110 == 0x697474696d6d6f63 && (short)plStack_110[1] == 0x676e)) {
      append_event(*(undefined8 *)(param_2 + 0x248),*(undefined8 *)(param_2 + 0x250),param_3,param_4,param_5,param_6,param_7,param_8,param_9);
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar19 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xaa,1);
      if (puVar19 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0xaa);
      }
      puVar19[0x11] = 0xe69688e6b385e580;
      puVar19[0x10] = 0xbce5b194e7afb7e8;
      puVar19[0x13] = 0xa0e6ae9be7b194e7;
      puVar19[0x12] = 0xafb7e88ca1e8a789;
      *(undefined8 *)((long)puVar19 + 0xa2) = 0x8da4e5aebfe48180;
      *(undefined8 *)((long)puVar19 + 0x9a) = 0xe687a0e6ae9be7b1;
      puVar19[9] = 0x95e79dbfe4b2b7e5;
      puVar19[8] = 0x95bde5b0aee88da4;
      puVar19[0xb] = 0xe48db8e420545047;
      puVar19[10] = 0x746168438cbcef99;
      puVar19[0xd] = 0xbcefa88ae5af90e5;
      puVar19[0xc] = 0xa88ae5aa87e89abc;
      puVar19[0xf] = 0x8d89e593bde595af;
      puVar19[0xe] = 0xe88d87e9b7afe89b;
      puVar19[1] = 0x45525f54494d4d4f;
      *puVar19 = 0x435f524554554f52;
      puVar19[3] = 0x3a44455249555145;
      puVar19[2] = 0x525f595245564f43;
      puVar19[5] = 0x9ab0e5a4bae4908f;
      puVar19[4] = 0xe6b194e7afb7e820;
      puVar19[7] = 0xe5a281e68cbcefa4;
      puVar19[6] = 0xaee8aea1e7aa9ce6;
      param_1[1] = 0xaa;
      *param_1 = 10;
      param_1[2] = (ulong)puVar19;
      param_1[3] = 0xaa;
      if (pcStack_118 != (code *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_110,pcStack_118,1);
      }
      if (uVar2 == 0x8000000000000000) {
        return;
      }
      if (uVar2 == 0) {
        return;
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_f8,uVar2,1);
      return;
    }
joined_r0x00010078e494:
    if (local_180 == param_3) {
      if (pcStack_118 != (code *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_110,pcStack_118,1);
      }
    }
    else {
      uVar4 = uVar1;
      if (pcStack_118 != (code *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_110,pcStack_118,1);
      }
    }
    uVar3 = uVar4;
    if ((uVar2 & 0x7fffffffffffffff) != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar8,uVar2,1);
    }
  }
  else {
    uVar3 = uVar1;
    if (pcStack_118 != (code *)0x8000000000000000) goto joined_r0x00010078e494;
  }
  if ((long)param_5 < 0) {
    uVar21 = 0;
LAB_10078e4b8:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar21,param_5);
  }
  if (param_5 == (code *)0x0) {
    pvVar12 = (void *)0x1;
    _memcpy((void *)0x1,param_4,0);
    if (param_8 != (void *)0x0) goto LAB_10078e50c;
LAB_10078e540:
    sVar17 = 0x8000000000000000;
    pcVar13 = param_5;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar21 = 1;
    pvVar12 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_5,1);
    if (pvVar12 == (void *)0x0) goto LAB_10078e4b8;
    _memcpy(pvVar12,param_4,(size_t)param_5);
    if (param_8 == (void *)0x0) goto LAB_10078e540;
LAB_10078e50c:
    if ((long)param_9 < 0) {
      uVar21 = 0;
LAB_10078e518:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(uVar21,param_9);
    }
    if (param_9 == 0) {
      pcVar13 = (code *)0x1;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar21 = 1;
      pcVar13 = (code *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_9,1);
      if (pcVar13 == (code *)0x0) goto LAB_10078e518;
    }
    _memcpy(pcVar13,param_8,param_9);
    sVar17 = param_9;
  }
  local_130 = CONCAT62(local_130._2_6_,0x100);
  local_130 = CONCAT71(local_130._1_7_,(char)param_3);
  uStack_148 = param_9;
  local_170 = param_5;
  plStack_168 = pvVar12;
  local_160 = param_5;
  uStack_158 = sVar17;
  local_150 = pcVar13;
  local_140 = uVar3;
  uStack_138 = uVar1;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  local_80 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x80,1);
  if (local_80 == (long *******)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x80);
  }
  local_88 = (undefined8 *******)&DAT_00000080;
  local_120 = (long *******)&local_88;
  pcStack_118 = (code *)0x1013a9286;
  local_108 = 1;
  plStack_110 = (long *)0x2;
  uStack_100 = 0;
  *(char *)local_80 = '{';
  pcStack_78 = (code *)0x1;
  local_a0[0] = 0x100;
  local_98 = &local_120;
  ppppppplVar14 =
       (long *******)
       serde_core::ser::SerializeMap::serialize_entry
                 (local_a0,"schemaVersiontransition completed or already stableclearedcleanup_sync_unconfirmedrouter-events.jsonlfailed",0xd,
                  (long)&local_130 + 1);
  if (ppppppplVar14 == (long *******)0x0) {
    if ((local_a0[0] & 1) != 0) {
LAB_10078e71c:
      ppppppplVar14 = (long *******)serde_json::ser::invalid_raw_value();
      goto LAB_10078e748;
    }
    ppppppplVar14 = (long *******)serde_core::ser::SerializeMap::serialize_entry(local_a0,&_anon_6408540cdffbe318d49f52812e99783f_452,6,&local_130);
    if (ppppppplVar14 != (long *******)0x0) goto LAB_10078e748;
    if ((local_a0[0] & 1) != 0) goto LAB_10078e71c;
    ppppppplVar14 = (long *******)serde_core::ser::SerializeMap::serialize_entry(local_a0,&_anon_6408540cdffbe318d49f52812e99783f_453,5,&local_170);
    if (ppppppplVar14 != (long *******)0x0) goto LAB_10078e748;
    if ((local_a0[0] & 1) != 0) goto LAB_10078e71c;
    ppppppplVar14 = (long *******)serde_core::ser::SerializeMap::serialize_entry(local_a0,&_anon_6408540cdffbe318d49f52812e99783f_462,0xb,&local_140);
    if (ppppppplVar14 != (long *******)0x0) goto LAB_10078e748;
    if ((local_a0[0] & 1) != 0) goto LAB_10078e71c;
    ppppppplVar14 = (long *******)serde_core::ser::SerializeMap::serialize_entry(local_a0,&_anon_6408540cdffbe318d49f52812e99783f_463,0xb,&uStack_138);
    if (ppppppplVar14 != (long *******)0x0) goto LAB_10078e748;
    if ((local_a0[0] & 1) != 0) goto LAB_10078e71c;
    ppppppplVar14 = (long *******)serde_core::ser::SerializeMap::serialize_entry(local_a0,&_anon_6408540cdffbe318d49f52812e99783f_455,6,&uStack_158);
    if (ppppppplVar14 != (long *******)0x0) goto LAB_10078e748;
    <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::end(local_a0);
    pcVar6 = pcStack_78;
    ppppppplVar14 = local_80;
    pppppppuVar10 = local_88;
    if (local_88 != (undefined8 *******)&DAT_8000000000000000) {
      if (pcStack_78 < (code *)0x100001) {
        uVar15 = *(undefined8 *)(param_2 + 0x248);
        uVar18 = *(undefined8 *)(param_2 + 0x250);
        std::path::Path::_join
                  (&local_b8,uVar15,uVar18,
                   "router-transition-journal.jsonunsupported transition record versionphase record exceeds the size limitphase record is not a regular file",
                   0x1e);
        uVar21 = local_b0;
        auVar24 = atomic_write::write_atomic_private_report(local_b0,uStack_a8,ppppppplVar14,pcVar6);
        pppppuVar16 = auVar24._8_8_;
        if ((auVar24._0_8_ & 1) == 0) {
          local_c0 = pppppuVar16;
          if (local_b8 != (long *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar21,local_b8,1);
          }
          append_event(uVar15,uVar18,param_3,param_4,param_5,param_6,param_7,param_8,param_9);
          *param_1 = 0xb;
          param_1[1] = (ulong)pppppuVar16;
          if (pppppppuVar10 != (undefined8 *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar14,pppppppuVar10,1);
          }
          ::core::ptr::drop_in_place<codexmate_lib::core::relay::transition_journal::TransitionJournal>(&local_170);
          return;
        }
        local_120 = (long *******)&local_90;
        pcStack_118 = <std::io::error::Error_as_core::fmt::Display>::fmt;
        local_90 = pppppuVar16;
        alloc::fmt::format::format_inner(&local_88,s__write_router_transition_journal_10152b252,&local_120);
        pppppppuVar11 = local_88;
        pcStack_118 = pcStack_78;
        local_120 = local_80;
        if (((ulong)local_90 & 3) == 1) {
          puVar19 = (undefined8 *)((long)local_90 - 1);
          uVar15 = *puVar19;
          puVar22 = *(undefined8 **)((long)local_90 + 7);
          if ((code *)*puVar22 != (code *)0x0) {
            (*(code *)*puVar22)(uVar15);
          }
          if (puVar22[1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar15,puVar22[1],puVar22[2]);
          }
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar19,0x18,8);
        }
        param_1[7] = local_f8;
        param_1[6] = CONCAT71(uStack_ff,uStack_100);
        param_1[9] = local_e8;
        param_1[8] = uStack_f0;
        param_1[0xb] = local_d8;
        param_1[10] = uStack_e0;
        param_1[3] = (ulong)pcStack_118;
        param_1[2] = (ulong)local_120;
        param_1[5] = local_108;
        param_1[4] = (ulong)plStack_110;
        *param_1 = 10;
        param_1[1] = (ulong)pppppppuVar11;
        if (local_b8 != (long *******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar21,local_b8,1);
        }
      }
      else {
        local_b8 = (long *******)0x1013aad54;
        local_b0 = 0x27;
        local_88 = &local_b8;
        local_80 = (long *******)&<&T_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&pcStack_118,_anon_6408540cdffbe318d49f52812e99783f_436,&local_88);
        local_120 = (long *******)&DAT_00000009;
        param_1[5] = local_f8;
        param_1[4] = CONCAT71(uStack_ff,uStack_100);
        param_1[7] = local_e8;
        param_1[6] = uStack_f0;
        param_1[9] = local_d8;
        param_1[8] = uStack_e0;
        param_1[0xb] = uStack_c8;
        param_1[10] = local_d0;
        param_1[1] = (ulong)pcStack_118;
        *param_1 = 9;
        param_1[3] = local_108;
        param_1[2] = (ulong)plStack_110;
      }
      if (pppppppuVar10 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar14,pppppppuVar10,1);
      }
      goto LAB_10078e7dc;
    }
  }
  else {
LAB_10078e748:
    if (local_88 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_80,local_88,1);
    }
  }
  local_88 = &local_b8;
  local_80 = (long *******)<serde_json::error::Error_as_core::fmt::Display>::fmt;
  local_b8 = ppppppplVar14;
  alloc::fmt::format::format_inner(&local_120,s__serialize_router_transition_jou_10152b223,&local_88);
  ppppppplVar9 = local_b8;
  plVar7 = plStack_110;
  pcVar6 = pcStack_118;
  ppppppplVar14 = local_120;
  if (*local_b8 == (long ******)0x1) {
    ::core::ptr::drop_in_place<std::io::error::Error>(local_b8 + 1);
  }
  else if ((*local_b8 == (long ******)0x0) && (local_b8[2] != (long ******)0x0)) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_b8[1],local_b8[2],1);
  }
  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplVar9,0x28,8);
  *param_1 = 10;
  param_1[1] = (ulong)ppppppplVar14;
  param_1[2] = (ulong)pcVar6;
  param_1[3] = (ulong)plVar7;
LAB_10078e7dc:
  if (param_5 != (code *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar12,param_5,1);
  }
  if ((sVar17 & 0x7fffffffffffffff) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar13,sVar17,1);
  }
  return;
}

