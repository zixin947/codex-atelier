/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read_checked @ 100790b68 requested 100790b68 */

/* codexmate_lib::core::relay::transition_journal::read_checked */

void codexmate_lib::core::relay::transition_journal::read_checked(undefined8 *param_1,long param_2)

{
  char *pcVar1;
  byte bVar2;
  char cVar3;
  undefined1 uVar4;
  char **ppcVar5;
  long lVar6;
  code *pcVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  undefined1 auVar10 [16];
  long local_178;
  undefined8 local_170;
  undefined8 uStack_168;
  long local_160;
  undefined8 local_158;
  long local_148;
  char **local_140;
  undefined8 *local_138;
  undefined8 local_130;
  code *pcStack_128;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 *local_110;
  char *local_108;
  undefined8 local_100;
  uint local_f0;
  undefined2 uStack_ec;
  undefined1 uStack_ea;
  undefined1 uStack_e9;
  byte bStack_e8;
  undefined1 uStack_e7;
  undefined2 uStack_e6;
  uint uStack_e4;
  int local_e0;
  undefined4 uStack_dc;
  undefined1 uStack_d8;
  undefined7 uStack_d7;
  undefined1 local_d0;
  undefined7 uStack_cf;
  undefined1 uStack_c8;
  undefined7 uStack_c7;
  undefined1 local_c0;
  undefined7 uStack_bf;
  undefined1 uStack_b8;
  undefined7 uStack_b7;
  undefined1 local_b0;
  undefined7 uStack_af;
  undefined1 uStack_a8;
  undefined7 uStack_a7;
  undefined1 uStack_a0;
  undefined7 uStack_9f;
  undefined1 local_98;
  undefined7 uStack_97;
  
  std::path::Path::_join
            (&local_178,*(undefined8 *)(param_2 + 0x248),*(undefined8 *)(param_2 + 0x250),
             "router-transition-journal.jsonunsupported transition record versionphase record exceeds the size limitphase record is not a regular file",0x1e);
  std::sys::fs::symlink_metadata(&local_f0,local_170,uStack_168);
  bVar2 = bStack_e8;
  pcVar1 = (char *)CONCAT44(uStack_e4,CONCAT22(uStack_e6,CONCAT11(uStack_e7,bStack_e8)));
  if ((local_f0 & 1) == 0) {
    if ((uStack_e4 & 0xf000) != 0x8000) {
      local_108 = "phase record is not a regular file";
      local_100 = 0x22;
      local_130 = &local_108;
      pcStack_128 = (code *)&<&T_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&bStack_e8,_anon_6408540cdffbe318d49f52812e99783f_436,&local_130);
      local_f0 = 9;
      uStack_ec = 0;
      uStack_ea = 0;
      uStack_e9 = 0;
      param_1[5] = CONCAT71(uStack_c7,uStack_c8);
      param_1[4] = CONCAT71(uStack_cf,local_d0);
      param_1[7] = CONCAT71(uStack_b7,uStack_b8);
      param_1[6] = CONCAT71(uStack_bf,local_c0);
      param_1[9] = CONCAT71(uStack_a7,uStack_a8);
      param_1[8] = CONCAT71(uStack_af,local_b0);
      param_1[0xb] = CONCAT71(uStack_97,local_98);
      param_1[10] = CONCAT71(uStack_9f,uStack_a0);
      param_1[1] = CONCAT44(uStack_e4,CONCAT22(uStack_e6,CONCAT11(uStack_e7,bStack_e8)));
      *param_1 = 9;
      param_1[3] = CONCAT71(uStack_d7,uStack_d8);
      param_1[2] = CONCAT44(uStack_dc,local_e0);
      goto LAB_100790edc;
    }
    local_148 = 0;
    local_140 = (char **)0x1;
    local_138 = (undefined8 *)0x0;
    local_f0 = 0;
    uStack_ec = 0x1b6;
    uStack_e9 = 0;
    bStack_e8 = 0;
    uStack_e7 = 0;
    uStack_e6 = 0;
    uStack_ea = 1;
    std::fs::OpenOptions::_open(&local_130,&local_f0,local_170,uStack_168);
    pcVar7 = pcStack_128;
    if ((int)local_130 == 1) {
LAB_100790de4:
      local_110 = (undefined8 *)pcVar7;
      uVar4 = std::io::error::Error::kind(pcVar7);
      local_130 = (char **)CONCAT71(local_130._1_7_,uVar4);
      local_f0 = (uint)&local_130;
      uStack_ec = (undefined2)((ulong)&local_130 >> 0x20);
      uStack_ea = (undefined1)((ulong)&local_130 >> 0x30);
      uStack_e9 = (undefined1)((ulong)&local_130 >> 0x38);
      bStack_e8 = 0x88;
      uStack_e7 = 0xbb;
      uStack_e6 = 0x7f;
      uStack_e4 = 1;
      alloc::fmt::format::format_inner(&local_108,s_record_read_failed_____10152b2bd,&local_f0);
      local_f0 = (uint)&local_108;
      uStack_ec = (undefined2)((ulong)&local_108 >> 0x20);
      uStack_ea = (undefined1)((ulong)&local_108 >> 0x30);
      uStack_e9 = (undefined1)((ulong)&local_108 >> 0x38);
      bStack_e8 = 0x54;
      uStack_e7 = 0xb2;
      uStack_e6 = 0x7f;
      uStack_e4 = 1;
      alloc::fmt::format::format_inner(&local_130,_anon_6408540cdffbe318d49f52812e99783f_436,&local_f0);
      ppcVar5 = local_130;
      bStack_e8 = (byte)local_120;
      uStack_e7 = (undefined1)((ulong)local_120 >> 8);
      uStack_e6 = (undefined2)((ulong)local_120 >> 0x10);
      uStack_e4 = (uint)((ulong)local_120 >> 0x20);
      local_f0 = (uint)pcStack_128;
      uStack_ec = (undefined2)((ulong)pcStack_128 >> 0x20);
      uStack_ea = (undefined1)((ulong)pcStack_128 >> 0x30);
      uStack_e9 = (undefined1)((ulong)pcStack_128 >> 0x38);
      if (local_108 != (char *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_100,local_108,1);
      }
      if (((ulong)pcVar7 & 3) == 1) {
        uVar8 = *(undefined8 *)((long)pcVar7 + -1);
        puVar9 = *(undefined8 **)((long)pcVar7 + 7);
        if ((code *)*puVar9 != (code *)0x0) {
          (*(code *)*puVar9)(uVar8);
        }
        if (puVar9[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar8,puVar9[1],puVar9[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)pcVar7 + -1),0x18,8);
      }
      param_1[7] = CONCAT71(uStack_c7,uStack_c8);
      param_1[6] = CONCAT71(uStack_cf,local_d0);
      param_1[9] = CONCAT71(uStack_b7,uStack_b8);
      param_1[8] = CONCAT71(uStack_bf,local_c0);
      param_1[0xb] = CONCAT71(uStack_a7,uStack_a8);
      param_1[10] = CONCAT71(uStack_af,local_b0);
      param_1[3] = CONCAT44(uStack_e4,CONCAT22(uStack_e6,CONCAT11(uStack_e7,bStack_e8)));
      param_1[2] = CONCAT17(uStack_e9,CONCAT16(uStack_ea,CONCAT24(uStack_ec,local_f0)));
      param_1[5] = CONCAT71(uStack_d7,uStack_d8);
      param_1[4] = CONCAT44(uStack_dc,local_e0);
      *param_1 = 9;
      param_1[1] = ppcVar5;
      lVar6 = local_148;
      ppcVar5 = local_140;
    }
    else {
      local_e0 = local_130._4_4_;
      bStack_e8 = 1;
      uStack_e7 = 0;
      uStack_e6 = 0x10;
      uStack_e4 = 0;
      local_f0 = 0x100001;
      uStack_ec = 0;
      uStack_ea = 0;
      uStack_e9 = 0;
      auVar10 = std::io::default_read_to_end(&local_f0,&local_148,0);
      _close(local_e0);
      puVar9 = local_138;
      ppcVar5 = local_140;
      pcVar7 = (code *)auVar10._8_8_;
      if ((auVar10._0_8_ & 1) != 0) goto LAB_100790de4;
      if (local_138 < (undefined8 *)0x100001) {
        local_f0 = (uint)local_140;
        uStack_ec = (undefined2)((ulong)local_140 >> 0x20);
        uStack_ea = (undefined1)((ulong)local_140 >> 0x30);
        uStack_e9 = (undefined1)((ulong)local_140 >> 0x38);
        bStack_e8 = (byte)local_138;
        uStack_e7 = (undefined1)((ulong)local_138 >> 8);
        uStack_e6 = (undefined2)((ulong)local_138 >> 0x10);
        uStack_e4 = (uint)((ulong)local_138 >> 0x20);
        local_e0 = 0;
        uStack_dc = 0;
        uStack_d8 = 0;
        uStack_d7 = 0;
        serde_json::de::from_trait(&local_130,&local_f0);
        if ((char)local_130 == '\x01') {
          ::core::ops::function::FnOnce::call_once(&local_f0,pcStack_128);
          *(ulong *)((long)param_1 + 0x31) = CONCAT17(uStack_b8,uStack_bf);
          *(ulong *)((long)param_1 + 0x29) = CONCAT17(local_c0,uStack_c7);
          *(ulong *)((long)param_1 + 0x41) = CONCAT17(uStack_a8,uStack_af);
          *(ulong *)((long)param_1 + 0x39) = CONCAT17(local_b0,uStack_b7);
          *(ulong *)((long)param_1 + 0x51) = CONCAT17(local_98,uStack_9f);
          *(ulong *)((long)param_1 + 0x49) = CONCAT17(uStack_a0,uStack_a7);
          param_1[0xb] = CONCAT71(uStack_97,local_98);
          *(ulong *)((long)param_1 + 0x11) = CONCAT17(uStack_d8,CONCAT43(uStack_dc,local_e0._1_3_));
          *(ulong *)((long)param_1 + 9) = CONCAT17((undefined1)local_e0,CONCAT43(uStack_e4,CONCAT21(uStack_e6,uStack_e7)));
          *(ulong *)((long)param_1 + 0x21) = CONCAT17(uStack_c8,uStack_cf);
          *(ulong *)((long)param_1 + 0x19) = CONCAT17(local_d0,uStack_d7);
          *param_1 = CONCAT17(uStack_e9,CONCAT16(uStack_ea,CONCAT24(uStack_ec,local_f0)));
          *(byte *)(param_1 + 1) = bStack_e8;
          lVar6 = local_148;
          ppcVar5 = local_140;
        }
        else {
          if (local_130._1_1_ != '\x01') {
            local_108 = "unsupported transition record versionphase record exceeds the size limitphase record is not a regular file";
            local_100 = 0x25;
            local_130 = &local_108;
            pcStack_128 = (code *)&<&T_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&bStack_e8,_anon_6408540cdffbe318d49f52812e99783f_436,&local_130);
            goto LAB_100790f50;
          }
          local_130 = ppcVar5;
          pcStack_128 = (code *)puVar9;
          local_120 = 0;
          uStack_118 = 0;
          serde_json::de::from_trait(&local_f0,&local_130);
          lVar6 = CONCAT17(uStack_e9,CONCAT16(uStack_ea,CONCAT24(uStack_ec,local_f0)));
          if (lVar6 == -0x8000000000000000) {
            ::core::ops::function::FnOnce::call_once(&local_f0);
            param_1[5] = CONCAT71(uStack_c7,uStack_c8);
            param_1[4] = CONCAT71(uStack_cf,local_d0);
            param_1[7] = CONCAT71(uStack_b7,uStack_b8);
            param_1[6] = CONCAT71(uStack_bf,local_c0);
            param_1[9] = CONCAT71(uStack_a7,uStack_a8);
            param_1[8] = CONCAT71(uStack_af,local_b0);
            param_1[0xb] = CONCAT71(uStack_97,local_98);
            param_1[10] = CONCAT71(uStack_9f,uStack_a0);
            param_1[1] = CONCAT44(uStack_e4,CONCAT22(uStack_e6,CONCAT11(uStack_e7,bStack_e8)));
            *param_1 = CONCAT17(uStack_e9,CONCAT16(uStack_ea,CONCAT24(uStack_ec,local_f0)));
            param_1[3] = CONCAT71(uStack_d7,uStack_d8);
            param_1[2] = CONCAT44(uStack_dc,local_e0);
            lVar6 = local_148;
          }
          else {
            param_1[4] = CONCAT71(uStack_d7,uStack_d8);
            param_1[3] = CONCAT44(uStack_dc,local_e0);
            param_1[6] = CONCAT71(uStack_c7,uStack_c8);
            param_1[5] = CONCAT71(uStack_cf,local_d0);
            param_1[8] = CONCAT71(uStack_b7,uStack_b8);
            param_1[7] = CONCAT71(uStack_bf,local_c0);
            param_1[9] = CONCAT71(uStack_af,local_b0);
            param_1[1] = lVar6;
            param_1[2] = CONCAT44(uStack_e4,CONCAT22(uStack_e6,CONCAT11(uStack_e7,bStack_e8)));
            *param_1 = 0xb;
            lVar6 = local_148;
          }
        }
      }
      else {
        local_108 = "phase record exceeds the size limitphase record is not a regular file";
        local_100 = 0x23;
        local_130 = &local_108;
        pcStack_128 = (code *)&<&T_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&bStack_e8,_anon_6408540cdffbe318d49f52812e99783f_436,&local_130);
LAB_100790f50:
        local_f0 = 9;
        uStack_ec = 0;
        uStack_ea = 0;
        uStack_e9 = 0;
        param_1[5] = CONCAT71(uStack_c7,uStack_c8);
        param_1[4] = CONCAT71(uStack_cf,local_d0);
        param_1[7] = CONCAT71(uStack_b7,uStack_b8);
        param_1[6] = CONCAT71(uStack_bf,local_c0);
        param_1[9] = CONCAT71(uStack_a7,uStack_a8);
        param_1[8] = CONCAT71(uStack_af,local_b0);
        param_1[0xb] = CONCAT71(uStack_97,local_98);
        param_1[10] = CONCAT71(uStack_9f,uStack_a0);
        param_1[1] = CONCAT44(uStack_e4,CONCAT22(uStack_e6,CONCAT11(uStack_e7,bStack_e8)));
        *param_1 = 9;
        param_1[3] = CONCAT71(uStack_d7,uStack_d8);
        param_1[2] = CONCAT44(uStack_dc,local_e0);
        lVar6 = local_148;
        ppcVar5 = local_140;
      }
    }
    local_148 = lVar6;
    if (lVar6 == 0) goto LAB_100790edc;
    uVar8 = 1;
  }
  else {
    cVar3 = std::io::error::Error::kind(pcVar1);
    if (cVar3 == '\0') {
      param_1[1] = 0x8000000000000000;
      *param_1 = 0xb;
      if ((bVar2 & 3) != 1) goto LAB_100790edc;
      uVar8 = *(undefined8 *)(pcVar1 + -1);
      puVar9 = *(undefined8 **)(pcVar1 + 7);
      if ((code *)*puVar9 != (code *)0x0) {
        (*(code *)*puVar9)(uVar8);
      }
    }
    else {
      local_108 = pcVar1;
      uVar4 = std::io::error::Error::kind(pcVar1);
      local_130 = (char **)CONCAT71(local_130._1_7_,uVar4);
      local_f0 = (uint)&local_130;
      uStack_ec = (undefined2)((ulong)&local_130 >> 0x20);
      uStack_ea = (undefined1)((ulong)&local_130 >> 0x30);
      uStack_e9 = (undefined1)((ulong)&local_130 >> 0x38);
      bStack_e8 = 0x88;
      uStack_e7 = 0xbb;
      uStack_e6 = 0x7f;
      uStack_e4 = 1;
      alloc::fmt::format::format_inner(&local_160,s_metadata_read_failed_____10152b2a2,&local_f0);
      pcStack_128 = <alloc::string::String_as_core::fmt::Display>::fmt;
      local_130 = (char **)&local_160;
      alloc::fmt::format::format_inner(&local_f0,_anon_6408540cdffbe318d49f52812e99783f_436,&local_130);
      pcStack_128 = (code *)CONCAT44(uStack_e4,CONCAT22(uStack_e6,CONCAT11(uStack_e7,bStack_e8)));
      local_130 = (char **)CONCAT17(uStack_e9,CONCAT16(uStack_ea,CONCAT24(uStack_ec,local_f0)));
      local_120 = CONCAT44(uStack_dc,local_e0);
      if (local_160 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_158,local_160,1);
      }
      *param_1 = 9;
      param_1[2] = pcStack_128;
      param_1[1] = local_130;
      param_1[3] = local_120;
      if ((bVar2 & 3) != 1) goto LAB_100790edc;
      uVar8 = *(undefined8 *)(pcVar1 + -1);
      puVar9 = *(undefined8 **)(pcVar1 + 7);
      if ((code *)*puVar9 != (code *)0x0) {
        (*(code *)*puVar9)(uVar8);
      }
    }
    ppcVar5 = (char **)(pcVar1 + -1);
    if (puVar9[1] != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar8,puVar9[1],puVar9[2]);
    }
    lVar6 = 0x18;
    uVar8 = 8;
  }
  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppcVar5,lVar6,uVar8);
LAB_100790edc:
  if (local_178 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_170,local_178,1);
  }
  return;
}

