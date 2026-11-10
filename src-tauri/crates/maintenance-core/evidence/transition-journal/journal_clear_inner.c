/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* clear_inner @ 10078dd58 requested 10078dd58 */

/* codexmate_lib::core::relay::transition_journal::clear_inner */

void codexmate_lib::core::relay::transition_journal::clear_inner(long *param_1,long param_2,char param_3)

{
  code *pcVar1;
  long *plVar2;
  long lVar3;
  long lVar4;
  undefined1 *puVar5;
  undefined1 uVar6;
  undefined1 *puVar7;
  undefined8 in_x6;
  ulong uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 *puVar11;
  undefined1 auVar12 [16];
  code *pcVar13;
  char local_158;
  undefined1 **local_110;
  code *pcStack_108;
  long *local_100;
  long lStack_f8;
  long local_f0;
  long lStack_e8;
  long local_e0;
  long lStack_d8;
  long local_d0;
  ulong uStack_c8;
  long local_c0;
  long lStack_b8;
  undefined1 local_b0;
  undefined7 uStack_af;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined1 *local_98;
  undefined1 *local_90;
  undefined8 local_88;
  undefined1 *local_78;
  undefined1 **local_70;
  long lStack_68;
  
  read_checked(&local_110,param_2);
  lVar4 = lStack_e8;
  lVar3 = local_f0;
  plVar2 = local_100;
  pcVar1 = pcStack_108;
  if (local_110 != (undefined1 **)0xb) {
    param_1[3] = lStack_f8;
    param_1[2] = (long)local_100;
    param_1[5] = lStack_e8;
    param_1[4] = local_f0;
    param_1[7] = lStack_d8;
    param_1[6] = local_e0;
    param_1[9] = uStack_c8;
    param_1[8] = local_d0;
    param_1[0xb] = lStack_b8;
    param_1[10] = local_c0;
    *param_1 = (long)local_110;
    param_1[1] = (long)pcStack_108;
    return;
  }
  if (pcStack_108 == (code *)0x8000000000000000) {
    *param_1 = 0xb;
    return;
  }
  if (lStack_f8 == 10) {
    if ((param_3 != '\x02') || (*local_100 != 0x697474696d6d6f63 || (short)local_100[1] != 0x676e)) goto LAB_10078de60;
  }
  else {
LAB_10078de60:
    local_158 = (char)uStack_c8;
    if ((param_3 == '\x02') || (local_158 == param_3)) {
      uVar8 = uStack_c8 & 0xff;
      uVar10 = *(undefined8 *)(param_2 + 0x248);
      uVar9 = *(undefined8 *)(param_2 + 0x250);
      pcVar13 = pcStack_108;
      std::path::Path::_join
                (&local_b0,uVar10,uVar9,
                 "router-transition-journal.jsonunsupported transition record versionphase record exceeds the size limitphase record is not a regular file",0x1e
                );
      auVar12 = atomic_write::removal::remove_file_report(local_a8,uStack_a0);
      puVar7 = auVar12._8_8_;
      if ((auVar12._0_8_ & 1) == 0) {
        if (CONCAT71(uStack_af,local_b0) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_a8,CONCAT71(uStack_af,local_b0),1);
        }
        if (puVar7 == (undefined1 *)0x0) {
          append_event(uVar10,uVar9,uVar8,"clearedcleanup_sync_unconfirmedrouter-events.jsonlfailed",7,0,in_x6,
                       "transition completed or already stableclearedcleanup_sync_unconfirmedrouter-events.jsonlfailed",0x26,pcVar13);
          *param_1 = 0xb;
        }
        else {
          local_90 = puVar7;
          append_event(uVar10,uVar9,uVar8,"cleanup_sync_unconfirmedrouter-events.jsonlfailed",0x18,0,in_x6,0);
          local_b0 = std::io::error::Error::kind(puVar7);
          local_78 = &local_b0;
          local_70 = (undefined1 **)<std::io::error::ErrorKind_as_core::fmt::Debug>::fmt;
          alloc::fmt::format::format_inner(&local_110,s__ROUTER_JOURNAL_REMOVED_SYNC_UNC_10152b382,&local_78);
          param_1[2] = (long)pcStack_108;
          param_1[1] = (long)local_110;
          param_1[3] = (long)local_100;
          *param_1 = 10;
          if (((ulong)puVar7 & 3) == 1) {
            uVar9 = *(undefined8 *)(puVar7 + -1);
            puVar11 = *(undefined8 **)(puVar7 + 7);
            if ((code *)*puVar11 != (code *)0x0) {
              (*(code *)*puVar11)(uVar9);
            }
            if (puVar11[1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar9,puVar11[1],puVar11[2]);
            }
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7 + -1,0x18,8);
          }
        }
      }
      else {
        local_98 = puVar7;
        uVar6 = std::io::error::Error::kind(puVar7);
        local_78 = (undefined1 *)CONCAT71(local_78._1_7_,uVar6);
        local_110 = &local_78;
        pcStack_108 = <std::io::error::ErrorKind_as_core::fmt::Debug>::fmt;
        alloc::fmt::format::format_inner(&local_90,s_record_cleanup_failed_____10152b412,&local_110);
        pcStack_108 = <alloc::string::String_as_core::fmt::Display>::fmt;
        local_110 = &local_90;
        alloc::fmt::format::format_inner(&local_78,_anon_6408540cdffbe318d49f52812e99783f_436,&local_110);
        puVar5 = local_78;
        pcStack_108 = (code *)lStack_68;
        local_110 = local_70;
        if (local_90 != (undefined1 *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_88,local_90,1);
        }
        if (((ulong)puVar7 & 3) == 1) {
          uVar9 = *(undefined8 *)(puVar7 + -1);
          puVar11 = *(undefined8 **)(puVar7 + 7);
          if ((code *)*puVar11 != (code *)0x0) {
            (*(code *)*puVar11)(uVar9);
          }
          if (puVar11[1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar9,puVar11[1],puVar11[2]);
          }
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7 + -1,0x18,8);
        }
        param_1[7] = lStack_e8;
        param_1[6] = local_f0;
        param_1[9] = lStack_d8;
        param_1[8] = local_e0;
        param_1[0xb] = uStack_c8;
        param_1[10] = local_d0;
        param_1[3] = (long)pcStack_108;
        param_1[2] = (long)local_110;
        param_1[5] = lStack_f8;
        param_1[4] = (long)local_100;
        *param_1 = 9;
        param_1[1] = (long)puVar5;
        if (CONCAT71(uStack_af,local_b0) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_a8,CONCAT71(uStack_af,local_b0),1);
        }
      }
      goto joined_r0x00010078e124;
    }
  }
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  puVar11 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xaa,1);
  if (puVar11 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0xaa);
  }
  puVar11[0x11] = 0xe69688e6b385e580;
  puVar11[0x10] = 0xbce5b194e7afb7e8;
  puVar11[0x13] = 0xa0e6ae9be7b194e7;
  puVar11[0x12] = 0xafb7e88ca1e8a789;
  *(undefined8 *)((long)puVar11 + 0xa2) = 0x8da4e5aebfe48180;
  *(undefined8 *)((long)puVar11 + 0x9a) = 0xe687a0e6ae9be7b1;
  puVar11[9] = 0x95e79dbfe4b2b7e5;
  puVar11[8] = 0x95bde5b0aee88da4;
  puVar11[0xb] = 0xe48db8e420545047;
  puVar11[10] = 0x746168438cbcef99;
  puVar11[0xd] = 0xbcefa88ae5af90e5;
  puVar11[0xc] = 0xa88ae5aa87e89abc;
  puVar11[0xf] = 0x8d89e593bde595af;
  puVar11[0xe] = 0xe88d87e9b7afe89b;
  puVar11[1] = 0x45525f54494d4d4f;
  *puVar11 = 0x435f524554554f52;
  puVar11[3] = 0x3a44455249555145;
  puVar11[2] = 0x525f595245564f43;
  puVar11[5] = 0x9ab0e5a4bae4908f;
  puVar11[4] = 0xe6b194e7afb7e820;
  puVar11[7] = 0xe5a281e68cbcefa4;
  puVar11[6] = 0xaee8aea1e7aa9ce6;
  param_1[1] = 0xaa;
  *param_1 = 10;
  param_1[2] = (long)puVar11;
  param_1[3] = 0xaa;
joined_r0x00010078e124:
  if (pcVar1 != (code *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar2,pcVar1,1);
  }
  if ((lVar3 != -0x8000000000000000) && (lVar3 != 0)) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar4,lVar3,1);
  }
  return;
}

