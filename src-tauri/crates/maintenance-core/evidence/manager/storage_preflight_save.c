/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* preflight_save @ 1002040f8 requested 1002040f8 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::storage::preflight_save */

void codexmate_lib::core::relay::storage::preflight_save(undefined8 *param_1,long param_2,undefined8 param_3)

{
  uint7 uVar1;
  ulong *puVar2;
  code *pcVar3;
  long lVar4;
  ulong uVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  undefined1 auVar10 [16];
  undefined8 local_168;
  undefined2 uStack_160;
  undefined2 local_15e;
  int local_158;
  int iStack_154;
  ulong local_150;
  undefined8 uStack_148;
  long local_140;
  undefined8 local_138;
  undefined8 uStack_130;
  ulong local_128;
  undefined8 local_120;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 local_110;
  undefined4 uStack_10c;
  ulong *local_100;
  code *pcStack_f8;
  undefined8 *local_f0;
  code *pcStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined4 local_b0;
  undefined4 uStack_ac;
  undefined8 uStack_a8;

  lVar4 = platform::paths::CodexPaths::ensure_directories();
  if (lVar4 == 0) {
    uVar6 = *(undefined8 *)(param_2 + 0x260);
    uVar8 = *(undefined8 *)(param_2 + 0x268);
    uVar5 = std::path::Path::is_dir(uVar6,uVar8);
    if ((uVar5 & 1) == 0) {
      std::sys::fs::metadata(&local_100,uVar6,uVar8);
      if ((int)local_100 == 1) {
        ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(1,pcStack_f8);
      }
      else {
        ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(0,pcStack_f8);
        local_168._0_4_ = 0;
        local_168._4_2_ = 0x1b6;
        local_168._6_1_ = 0;
        uStack_160 = 0;
        local_15e = 0;
        local_168._7_1_ = 1;
        std::fs::OpenOptions::_open(&local_158,&local_168,uVar6,uVar8);
        if (local_158 == 1) {
          local_128 = local_150;
          local_100 = &local_128;
          pcStack_f8 = <std::io::error::Error_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_120,s__preflight_relay_state_target_ac_10151d6d9,&local_100);
          pcStack_f8 = (code *)CONCAT44(local_110,uStack_114);
          local_100 = (ulong *)CONCAT44(uStack_118,local_120._4_4_);
          local_f0 = (undefined8 *)CONCAT44(local_f0._4_4_,uStack_10c);
          if ((local_128 & 3) == 1) {
            puVar7 = (undefined8 *)(local_128 - 1);
            uVar6 = *puVar7;
            puVar9 = *(undefined8 **)(local_128 + 7);
            if ((code *)*puVar9 != (code *)0x0) {
              (*(code *)*puVar9)(uVar6);
            }
            if (puVar9[1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar6,puVar9[1],puVar9[2]);
            }
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,0x18,8);
          }
          *(undefined8 *)((long)param_1 + 0x34) = uStack_d8;
          *(undefined8 *)((long)param_1 + 0x2c) = local_e0;
          *(undefined8 *)((long)param_1 + 0x44) = uStack_c8;
          *(undefined8 *)((long)param_1 + 0x3c) = uStack_d0;
          *(undefined8 *)((long)param_1 + 0x54) = uStack_b8;
          *(undefined8 *)((long)param_1 + 0x4c) = local_c0;
          *(code **)((long)param_1 + 0x14) = pcStack_f8;
          *(ulong **)((long)param_1 + 0xc) = local_100;
          *(undefined4 *)((long)param_1 + 0x5c) = local_b0;
          *(code **)((long)param_1 + 0x24) = pcStack_e8;
          *(undefined8 **)((long)param_1 + 0x1c) = local_f0;
          *param_1 = 10;
          *(undefined4 *)(param_1 + 1) = (undefined4)local_120;
          return;
        }
        _close(iStack_154);
      }
      serde_json::ser::to_vec_pretty(&local_100,param_3);
      puVar7 = local_f0;
      pcVar3 = pcStack_f8;
      puVar2 = local_100;
      if (local_100 == (ulong *)&DAT_8000000000000000) {
        *param_1 = 3;
        param_1[1] = pcStack_f8;
      }
      else {
        auVar10 = std::path::Path::parent(uVar6,uVar8);
        if (auVar10._0_8_ == 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x28,1);
          if (puVar7 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(1,0x28);
          }
          puVar7[1] = 0x6874617020657461;
          *puVar7 = 0x74732079616c6572;
          puVar7[3] = 0x6420746e65726170;
          puVar7[2] = 0x206f6e2073616820;
          puVar7[4] = 0x79726f7463657269;
          *param_1 = 10;
          param_1[1] = 0x28;
          param_1[2] = puVar7;
          param_1[3] = 0x28;
        }
        else {
          local_168._0_4_ = std::process::id();
          uuid::v4::<impl_uuid::Uuid>::new_v4(&local_100);
          uStack_118 = SUB84(pcStack_f8,0);
          uStack_114 = (undefined4)((ulong)pcStack_f8 >> 0x20);
          local_120._0_4_ = SUB84(local_100,0);
          local_120._4_4_ = (undefined4)((ulong)local_100 >> 0x20);
          local_100 = &local_168;
          pcStack_f8 = u32>::fmt;
          local_f0 = &local_120;
          pcStack_e8 = <uuid::fmt::Simple_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_140,s__relay_state_preflight____10151d65e,&local_100);
          std::path::Path::_join(&local_158,auVar10._0_8_,auVar10._8_8_,local_138,uStack_130);
          if (local_140 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_138,local_140,1);
          }
          auVar10 = atomic_write::write_atomic_private_report(local_150,uStack_148,pcVar3,puVar7);
          if ((auVar10._0_8_ & 1) == 0) {
            atomic_write::Publication::acknowledge(auVar10._8_8_);
            lVar4 = std::sys::fs::remove_file(local_150,uStack_148);
            if (lVar4 == 0) {
              *param_1 = 0xb;
            }
            else {
              preflight_save::__closure__(&local_100,local_150,uStack_148,lVar4);
              param_1[5] = uStack_d8;
              param_1[4] = local_e0;
              param_1[7] = uStack_c8;
              param_1[6] = uStack_d0;
              param_1[9] = uStack_b8;
              param_1[8] = local_c0;
              param_1[0xb] = uStack_a8;
              param_1[10] = CONCAT44(uStack_ac,local_b0);
              param_1[1] = pcStack_f8;
              *param_1 = local_100;
              param_1[3] = pcStack_e8;
              param_1[2] = local_f0;
            }
            if (CONCAT44(iStack_154,local_158) != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_150,CONCAT44(iStack_154,local_158),1);
            }
          }
          else {
            local_168._0_4_ = auVar10._8_4_;
            local_168._4_2_ = auVar10._12_2_;
            local_168._6_1_ = auVar10[0xe];
            local_168._7_1_ = auVar10[0xf];
            local_120 = (pid_t *)&local_168;
            uStack_118 = 0x1169844;
            uStack_114 = 1;
            alloc::fmt::format::format_inner(&local_100,s__preflight_relay_state_write_fai_10151d6b2,&local_120);
            uStack_118 = SUB84(pcStack_f8,0);
            uStack_114 = (undefined4)((ulong)pcStack_f8 >> 0x20);
            local_120._0_4_ = SUB84(local_100,0);
            local_120._4_4_ = (undefined4)((ulong)local_100 >> 0x20);
            local_110 = SUB84(local_f0,0);
            uStack_10c = (undefined4)((ulong)local_f0 >> 0x20);
            uVar1 = CONCAT16(local_168._6_1_,CONCAT24(local_168._4_2_,(pid_t)local_168));
            lVar4 = CONCAT17(local_168._7_1_,uVar1);
            if (((ulong)uVar1 & 3) == 1) {
              puVar7 = (undefined8 *)(lVar4 + -1);
              uVar6 = *puVar7;
              puVar9 = *(undefined8 **)(lVar4 + 7);
              if ((code *)*puVar9 != (code *)0x0) {
                (*(code *)*puVar9)(uVar6);
              }
              if (puVar9[1] != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar6,puVar9[1],puVar9[2]);
              }
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,0x18,8);
            }
            param_1[2] = CONCAT44(uStack_114,uStack_118);
            param_1[1] = CONCAT44(local_120._4_4_,(undefined4)local_120);
            param_1[3] = CONCAT44(uStack_10c,local_110);
            *param_1 = 10;
            if (CONCAT44(iStack_154,local_158) != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_150,CONCAT44(iStack_154,local_158),1);
            }
          }
        }
        if (puVar2 != (ulong *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar3,puVar2,1);
        }
      }
    }
    else {
      local_120._0_4_ = (undefined4)uVar6;
      local_120._4_4_ = (undefined4)((ulong)uVar6 >> 0x20);
      uStack_118 = (undefined4)uVar8;
      uStack_114 = (undefined4)((ulong)uVar8 >> 0x20);
      local_100 = &local_120;
      pcStack_f8 = <std::path::Display_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(param_1 + 1,s__preflight_relay_state_write_fai_10151d67b,&local_100);
      *param_1 = 10;
    }
  }
  else {
    *param_1 = 2;
    param_1[1] = lVar4;
  }
  return;
}
