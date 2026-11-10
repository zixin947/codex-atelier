/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* load_model_restore_journal @ 1008ab7c4 requested 1008ab7c4 */

/* codexmate_lib::core::relay::codex_thread_visibility::model_fallback::load_model_restore_journal */

ulong codexmate_lib::core::relay::codex_thread_visibility::model_fallback::load_model_restore_journal(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  ushort uVar1;
  uint uVar2;
  ulong uVar3;
  long lVar4;
  long lVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  undefined1 auVar9 [16];
  long local_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  long local_198;
  long local_190;
  ulong local_188;
  undefined8 local_180;
  undefined8 *puStack_178;
  long lStack_170;
  undefined8 local_168;
  undefined8 *local_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 **local_140;
  undefined8 *puStack_138;
  undefined8 ***local_120;
  undefined8 *puStack_118;
  long local_110;
  undefined8 uStack_108;
  undefined1 local_100 [16];
  uint local_e8;
  undefined2 uStack_e4;
  undefined1 uStack_e2;
  undefined1 uStack_e1;
  ushort uStack_e0;
  undefined2 uStack_de;
  uint uStack_dc;
  undefined4 local_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 local_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  ulong local_80;

  std::path::Path::_join(&local_1b0,param_2,param_3,&DAT_1013b5084,0x19);
  std::sys::fs::symlink_metadata(&local_e8,local_1a8,uStack_1a0);
  uVar1 = uStack_e0;
  lVar4 = CONCAT44(uStack_dc,CONCAT22(uStack_de,uStack_e0));
  if ((local_e8 & 1) == 0) {
    if ((uStack_dc & 0xf000) == 0x8000) {
      if (local_80 < 0x800001) {
        local_198 = 0;
        local_190 = 1;
        local_188 = 0;
        local_e8 = 0;
        uStack_e4 = 0x1b6;
        uStack_e1 = 0;
        uStack_e0 = 0;
        uStack_de = 0;
        uStack_e2 = 1;
        std::fs::OpenOptions::_open(&local_180,&local_e8,local_1a8,uStack_1a0);
        puVar7 = puStack_178;
        if ((int)local_180 == 1) {
          local_160 = (undefined8 *)&DAT_1013b504e;
          uStack_158 = 0xc;
          local_140 = &local_160;
          puStack_138 = &<&T_as_core::fmt::Display>::fmt;
          uVar3 = alloc::fmt::format::format_inner(&uStack_e0,s__10152f955,&local_140);
          if (((ulong)puVar7 & 3) == 1) {
            uVar6 = *(undefined8 *)((long)puVar7 + -1);
            puVar8 = *(undefined8 **)((long)puVar7 + 7);
            if ((code *)*puVar8 != (code *)0x0) {
              (*(code *)*puVar8)(uVar6);
            }
            if (puVar8[1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar6,puVar8[1],puVar8[2]);
            }
            uVar3 = __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)puVar7 + -1),0x18,8);
          }
          *(ulong *)((long)param_1 + 0x34) = CONCAT44(uStack_b0,uStack_b4);
          *(ulong *)((long)param_1 + 0x2c) = CONCAT44(local_b8,uStack_bc);
          *(ulong *)((long)param_1 + 0x44) = CONCAT44(uStack_a0,uStack_a4);
          *(ulong *)((long)param_1 + 0x3c) = CONCAT44(local_a8,uStack_ac);
          *(ulong *)((long)param_1 + 0x54) = CONCAT44(uStack_90,uStack_94);
          *(ulong *)((long)param_1 + 0x4c) = CONCAT44(local_98,uStack_9c);
          *(undefined4 *)((long)param_1 + 0x5c) = uStack_8c;
          *(ulong *)((long)param_1 + 0x14) = CONCAT44(uStack_d0,uStack_d4);
          *(ulong *)((long)param_1 + 0xc) = CONCAT44(local_d8,uStack_dc);
          *(ulong *)((long)param_1 + 0x24) = CONCAT44(uStack_c0,uStack_c4);
          *(ulong *)((long)param_1 + 0x1c) = CONCAT44(local_c8,uStack_cc);
          *param_1 = 10;
          *(uint *)(param_1 + 1) = CONCAT22(uStack_de,uStack_e0);
        }
        else {
          local_110 = CONCAT44(local_110._4_4_,local_180._4_4_);
          puStack_118 = (undefined8 *)0x800001;
          local_120 = (undefined8 ***)0x800001;
          auVar9 = std::io::default_read_to_end(&local_120,&local_198,0);
          uVar3 = auVar9._8_8_;
          if ((auVar9._0_8_ & 1) == 0) {
            _close((int)local_110);
            if (local_188 < 0x800001) {
              local_e8 = (uint)local_190;
              uStack_e4 = (undefined2)((ulong)local_190 >> 0x20);
              uStack_e2 = (undefined1)((ulong)local_190 >> 0x30);
              uStack_e1 = (undefined1)((ulong)local_190 >> 0x38);
              uStack_e0 = (ushort)local_188;
              uStack_de = (undefined2)(local_188 >> 0x10);
              uStack_dc = (uint)(local_188 >> 0x20);
              local_d8 = 0;
              uStack_d4 = 0;
              uStack_d0 = 0;
              uStack_cc = 0;
              serde_json::de::from_trait(&local_120,&local_e8);
              if (local_120 == (undefined8 ***)&DAT_8000000000000000) {
                uVar3 = load_model_restore_journal::__closure__(&local_e8,puStack_118);
                uStack_150 = CONCAT44(uStack_cc,uStack_d0);
                uStack_158 = CONCAT44(uStack_d4,local_d8);
                local_160 = (undefined8 *)CONCAT44(uStack_dc,CONCAT22(uStack_de,uStack_e0));
                param_1[6] = CONCAT44(uStack_b4,local_b8);
                param_1[5] = CONCAT44(uStack_bc,uStack_c0);
                param_1[8] = CONCAT44(uStack_a4,local_a8);
                param_1[7] = CONCAT44(uStack_ac,uStack_b0);
                param_1[10] = CONCAT44(uStack_94,local_98);
                param_1[9] = CONCAT44(uStack_9c,uStack_a0);
                param_1[0xb] = CONCAT44(uStack_8c,uStack_90);
                param_1[4] = CONCAT44(uStack_c4,local_c8);
                param_1[3] = uStack_150;
                param_1[2] = uStack_158;
                param_1[1] = local_160;
                *param_1 = CONCAT17(uStack_e1,CONCAT16(uStack_e2,CONCAT24(uStack_e4,local_e8)));
              }
              else {
                puStack_178 = puStack_118;
                local_180 = local_120;
                local_168 = uStack_108;
                lStack_170 = local_110;
                local_100 = std::thread::local::LocalKey<T>::with(&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_36);
                puStack_118 = (undefined8 *)0x0;
                local_120 = (undefined8 ***)&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_22;
                uStack_108 = 0;
                local_110 = 0;
                if ((int)local_168 == 1) {
                  puVar7 = puStack_178 + lStack_170 * 10;
                  local_e8 = (uint)puStack_178;
                  uStack_e4 = (undefined2)((ulong)puStack_178 >> 0x20);
                  uStack_e2 = (undefined1)((ulong)puStack_178 >> 0x30);
                  uStack_e1 = (undefined1)((ulong)puStack_178 >> 0x38);
                  uStack_e0 = (ushort)puVar7;
                  uStack_de = (undefined2)((ulong)puVar7 >> 0x10);
                  uStack_dc = (uint)((ulong)puVar7 >> 0x20);
                  uVar3 = <core::slice::iter::Iter<T>as_core::iter::traits::iterator::Iterator>::any(&local_e8,&local_120);
                  if ((uVar3 & 1) == 0) {
                    param_1[2] = puStack_178;
                    param_1[1] = local_180;
                    param_1[4] = local_168;
                    param_1[3] = lStack_170;
                    *param_1 = 0xb;
                    uVar3 = <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(&local_120);
                    goto LAB_1008abd80;
                  }
                }
                local_160 = &DAT_1013b500c;
                uStack_158 = 0x1e;
                local_140 = &local_160;
                puStack_138 = &<&T_as_core::fmt::Display>::fmt;
                alloc::fmt::format::format_inner(&uStack_e0,s__10152f955,&local_140);
                local_e8 = 10;
                uStack_e4 = 0;
                uStack_e2 = 0;
                uStack_e1 = 0;
                param_1[5] = CONCAT44(uStack_bc,uStack_c0);
                param_1[4] = CONCAT44(uStack_c4,local_c8);
                param_1[7] = CONCAT44(uStack_ac,uStack_b0);
                param_1[6] = CONCAT44(uStack_b4,local_b8);
                param_1[9] = CONCAT44(uStack_9c,uStack_a0);
                param_1[8] = CONCAT44(uStack_a4,local_a8);
                param_1[0xb] = CONCAT44(uStack_8c,uStack_90);
                param_1[10] = CONCAT44(uStack_94,local_98);
                param_1[1] = CONCAT44(uStack_dc,CONCAT22(uStack_de,uStack_e0));
                *param_1 = 10;
                param_1[3] = CONCAT44(uStack_cc,uStack_d0);
                param_1[2] = CONCAT44(uStack_d4,local_d8);
                <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(&local_120);
                uVar3 = ::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_thread_visibility::model_fallback::ModelRestoreJournal>(&local_180);
              }
            }
            else {
              local_140 = (undefined8 **)((long)&DAT_1013b5024 + 6);
              puStack_138 = (undefined8 *)0x12;
              local_120 = &local_140;
              puStack_118 = &<&T_as_core::fmt::Display>::fmt;
              uVar3 = alloc::fmt::format::format_inner(&uStack_e0,s__10152f955,&local_120);
              local_e8 = 10;
              uStack_e4 = 0;
              uStack_e2 = 0;
              uStack_e1 = 0;
              param_1[5] = CONCAT44(uStack_bc,uStack_c0);
              param_1[4] = CONCAT44(uStack_c4,local_c8);
              param_1[7] = CONCAT44(uStack_ac,uStack_b0);
              param_1[6] = CONCAT44(uStack_b4,local_b8);
              param_1[9] = CONCAT44(uStack_9c,uStack_a0);
              param_1[8] = CONCAT44(uStack_a4,local_a8);
              param_1[0xb] = CONCAT44(uStack_8c,uStack_90);
              param_1[10] = CONCAT44(uStack_94,local_98);
              param_1[1] = CONCAT44(uStack_dc,CONCAT22(uStack_de,uStack_e0));
              *param_1 = 10;
              param_1[3] = CONCAT44(uStack_cc,uStack_d0);
              param_1[2] = CONCAT44(uStack_d4,local_d8);
            }
          }
          else {
            local_160 = (undefined8 *)&DAT_1013b505a;
            uStack_158 = 0xc;
            local_140 = &local_160;
            puStack_138 = &<&T_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&uStack_e0,s__10152f955,&local_140);
            if ((uVar3 & 3) == 1) {
              uVar6 = *(undefined8 *)(uVar3 - 1);
              puVar7 = *(undefined8 **)(uVar3 + 7);
              if ((code *)*puVar7 != (code *)0x0) {
                (*(code *)*puVar7)(uVar6);
              }
              if (puVar7[1] != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar6,puVar7[1],puVar7[2]);
              }
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(uVar3 - 1),0x18,8);
            }
            param_1[7] = CONCAT44(uStack_ac,uStack_b0);
            param_1[6] = CONCAT44(uStack_b4,local_b8);
            param_1[9] = CONCAT44(uStack_9c,uStack_a0);
            param_1[8] = CONCAT44(uStack_a4,local_a8);
            param_1[0xb] = CONCAT44(uStack_8c,uStack_90);
            param_1[10] = CONCAT44(uStack_94,local_98);
            param_1[3] = CONCAT44(uStack_cc,uStack_d0);
            param_1[2] = CONCAT44(uStack_d4,local_d8);
            param_1[5] = CONCAT44(uStack_bc,uStack_c0);
            param_1[4] = CONCAT44(uStack_c4,local_c8);
            *param_1 = 10;
            param_1[1] = CONCAT44(uStack_dc,CONCAT22(uStack_de,uStack_e0));
            uVar2 = _close((int)local_110);
            uVar3 = (ulong)uVar2;
          }
        }
LAB_1008abd80:
        if (local_198 == 0) goto LAB_1008abd94;
        uVar6 = 1;
        lVar4 = local_190;
        lVar5 = local_198;
        goto LAB_1008abd90;
      }
      local_140 = (undefined8 **)((long)&DAT_1013b5024 + 6);
      puStack_138 = (undefined8 *)0x12;
      local_120 = &local_140;
      puStack_118 = &<&T_as_core::fmt::Display>::fmt;
      uVar3 = alloc::fmt::format::format_inner(&uStack_e0,s__10152f955,&local_120);
    }
    else {
      local_140 = (undefined8 **)&DAT_1013b503c;
      puStack_138 = (undefined8 *)0x12;
      local_120 = &local_140;
      puStack_118 = &<&T_as_core::fmt::Display>::fmt;
      uVar3 = alloc::fmt::format::format_inner(&uStack_e0,s__10152f955,&local_120);
    }
    local_e8 = 10;
    uStack_e4 = 0;
    uStack_e2 = 0;
    uStack_e1 = 0;
    param_1[5] = CONCAT44(uStack_bc,uStack_c0);
    param_1[4] = CONCAT44(uStack_c4,local_c8);
    param_1[7] = CONCAT44(uStack_ac,uStack_b0);
    param_1[6] = CONCAT44(uStack_b4,local_b8);
    param_1[9] = CONCAT44(uStack_9c,uStack_a0);
    param_1[8] = CONCAT44(uStack_a4,local_a8);
    param_1[0xb] = CONCAT44(uStack_8c,uStack_90);
    param_1[10] = CONCAT44(uStack_94,local_98);
    param_1[1] = CONCAT44(uStack_dc,CONCAT22(uStack_de,uStack_e0));
    *param_1 = 10;
    param_1[3] = CONCAT44(uStack_cc,uStack_d0);
    param_1[2] = CONCAT44(uStack_d4,local_d8);
  }
  else {
    uVar3 = std::io::error::Error::kind(lVar4);
    if ((uVar3 & 0xff) == 0) {
      param_1[2] = 8;
      param_1[3] = 0;
      *(undefined4 *)(param_1 + 4) = 1;
      param_1[1] = 0;
      *param_1 = 0xb;
      if ((uVar1 & 3) != 1) goto LAB_1008abd94;
      uVar6 = *(undefined8 *)(lVar4 + -1);
      puVar7 = *(undefined8 **)(lVar4 + 7);
      if ((code *)*puVar7 != (code *)0x0) {
        (*(code *)*puVar7)(uVar6);
      }
    }
    else {
      local_140 = (undefined8 **)&DAT_1013b504e;
      puStack_138 = (undefined8 *)0xc;
      local_120 = &local_140;
      puStack_118 = &<&T_as_core::fmt::Display>::fmt;
      uVar3 = alloc::fmt::format::format_inner(&uStack_e0,s__10152f955,&local_120);
      local_e8 = 10;
      uStack_e4 = 0;
      uStack_e2 = 0;
      uStack_e1 = 0;
      param_1[5] = CONCAT44(uStack_bc,uStack_c0);
      param_1[4] = CONCAT44(uStack_c4,local_c8);
      param_1[7] = CONCAT44(uStack_ac,uStack_b0);
      param_1[6] = CONCAT44(uStack_b4,local_b8);
      param_1[9] = CONCAT44(uStack_9c,uStack_a0);
      param_1[8] = CONCAT44(uStack_a4,local_a8);
      param_1[0xb] = CONCAT44(uStack_8c,uStack_90);
      param_1[10] = CONCAT44(uStack_94,local_98);
      param_1[1] = CONCAT44(uStack_dc,CONCAT22(uStack_de,uStack_e0));
      *param_1 = 10;
      param_1[3] = CONCAT44(uStack_cc,uStack_d0);
      param_1[2] = CONCAT44(uStack_d4,local_d8);
      if ((uVar1 & 3) != 1) goto LAB_1008abd94;
      uVar6 = *(undefined8 *)(lVar4 + -1);
      puVar7 = *(undefined8 **)(lVar4 + 7);
      if ((code *)*puVar7 != (code *)0x0) {
        (*(code *)*puVar7)(uVar6);
      }
    }
    if (puVar7[1] != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar6,puVar7[1],puVar7[2]);
    }
    uVar6 = 8;
    lVar4 = lVar4 + -1;
    lVar5 = 0x18;
LAB_1008abd90:
    uVar3 = __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar4,lVar5,uVar6);
  }
LAB_1008abd94:
  if (local_1b0 != 0) {
    uVar3 = __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1a8,local_1b0,1);
  }
  return uVar3;
}
