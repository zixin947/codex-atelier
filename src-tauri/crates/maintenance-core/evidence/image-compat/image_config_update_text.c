/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* update_text @ 1005c1834 requested 1005c1834 */

/* codexmate_lib::core::codex_config::update_text */

void codexmate_lib::core::codex_config::update_text
               (undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5,undefined1 *param_6)

{
  undefined1 uVar1;
  ulong *puVar2;
  code *pcVar3;
  int iVar4;
  long lVar5;
  undefined8 *puVar6;
  ulong *puVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  code *pcVar11;
  undefined8 uVar12;
  undefined8 *puVar13;
  undefined1 auVar14 [16];
  undefined8 *local_130;
  undefined8 *local_128;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 *local_108;
  undefined8 *puStack_100;
  undefined8 *local_f8;
  code *pcStack_f0;
  undefined8 *local_e8;
  code *pcStack_e0;
  ulong local_d8;
  undefined8 *local_d0;
  undefined8 *puStack_c8;
  ulong *local_c0;
  code *pcStack_b8;
  char *local_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined *local_98;
  undefined8 local_90;
  undefined8 uStack_88;
  char *local_80;
  undefined8 **ppuStack_78;
  
  local_120 = param_4;
  uStack_118 = param_5;
  auVar14 = std::path::Path::parent();
  if (auVar14._0_8_ != 0) {
    local_d0 = (undefined8 *)CONCAT53(local_d0._3_5_,0x101ff);
    lVar5 = std::fs::DirBuilder::_create(&local_d0,auVar14._0_8_,auVar14._8_8_);
    if (lVar5 != 0) {
      *param_1 = 2;
      param_1[1] = lVar5;
      return;
    }
  }
  uVar1 = *param_6;
  local_110 = 1;
  read_text(&local_d0,param_2,param_3);
  pcVar3 = pcStack_b8;
  puVar7 = local_c0;
  puVar6 = puStack_c8;
  if (local_d0 == (undefined8 *)&DAT_0000000b) {
    commands::system::compose_image_compat_flag(&puStack_c8,local_c0,pcStack_b8,uVar1);
    pcVar11 = pcStack_b8;
    local_130 = local_c0;
    puVar8 = puStack_c8;
    if ((pcStack_b8 == pcVar3) && (iVar4 = _memcmp(local_c0,puVar7,(size_t)pcVar3), iVar4 == 0)) {
LAB_1005c1da0:
      *param_1 = 0xb;
    }
    else {
      read_text(&local_d0,param_2,param_3);
      puVar2 = local_c0;
      puVar10 = puStack_c8;
      local_128 = local_c0;
      if (local_d0 == (undefined8 *)&DAT_0000000b) {
        if ((pcStack_b8 != pcVar3) || (iVar4 = _memcmp(local_c0,puVar7,(size_t)pcVar3), iVar4 != 0)) {
          if (1 < log::MAX_LOG_LEVEL_FILTER) {
            local_108 = &local_120;
            puStack_100 = &<&T_as_core::fmt::Display>::fmt;
            local_f8 = &local_110;
            pcStack_f0 = usize>::fmt;
            local_e8 = &DAT_1013a14b8;
            pcStack_e0 = usize>::fmt;
            local_98 = &DAT_10139db69;
            local_90 = 0x21;
            ppuStack_78 = &local_108;
            local_80 = s___AiMaMi__codex_config__config_c_101526556;
            local_d0 = (undefined8 *)0x0;
            puStack_c8 = (undefined8 *)&DAT_10139db69;
            pcStack_b8 = (code *)0x0;
            local_c0 = (ulong *)&DAT_00000021;
            local_b0 = "src/core/codex_config.rs";
            uStack_a0 = 2;
            uStack_a8 = 0x18;
            uStack_88 = 0x2600000001;
            <log::__private_api::GlobalLogger_as_log::Log>::log(&local_d8,&local_d0);
          }
          if (puVar10 != (undefined8 *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,puVar10,1);
          }
          if (puVar8 != (undefined8 *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,puVar8,1);
          }
          if (puVar6 != (undefined8 *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,puVar6,1);
          }
          local_110 = 2;
          read_text(&local_d0,param_2,param_3);
          pcVar3 = pcStack_b8;
          puVar7 = local_c0;
          puVar6 = puStack_c8;
          if (local_d0 != (undefined8 *)&DAT_0000000b) goto LAB_1005c1d5c;
          commands::system::compose_image_compat_flag(&puStack_c8,local_c0,pcStack_b8,uVar1);
          pcVar11 = pcStack_b8;
          local_130 = local_c0;
          puVar8 = puStack_c8;
          if ((pcStack_b8 == pcVar3) && (iVar4 = _memcmp(local_c0,puVar7,(size_t)pcVar3), iVar4 == 0)) goto LAB_1005c1da0;
          read_text(&local_d0,param_2,param_3);
          puVar2 = local_c0;
          puVar10 = puStack_c8;
          local_128 = local_c0;
          if (local_d0 != (undefined8 *)&DAT_0000000b) goto LAB_1005c1d78;
          if ((pcStack_b8 != pcVar3) || (iVar4 = _memcmp(local_c0,puVar7,(size_t)pcVar3), iVar4 != 0)) {
            if (1 < log::MAX_LOG_LEVEL_FILTER) {
              local_108 = &local_120;
              puStack_100 = &<&T_as_core::fmt::Display>::fmt;
              local_f8 = &local_110;
              pcStack_f0 = usize>::fmt;
              local_e8 = &DAT_1013a14b8;
              pcStack_e0 = usize>::fmt;
              local_98 = &DAT_10139db69;
              local_90 = 0x21;
              ppuStack_78 = &local_108;
              local_80 = s___AiMaMi__codex_config__config_c_101526556;
              local_d0 = (undefined8 *)0x0;
              puStack_c8 = (undefined8 *)&DAT_10139db69;
              pcStack_b8 = (code *)0x0;
              local_c0 = (ulong *)&DAT_00000021;
              local_b0 = "src/core/codex_config.rs";
              uStack_a0 = 2;
              uStack_a8 = 0x18;
              uStack_88 = 0x2600000001;
              <log::__private_api::GlobalLogger_as_log::Log>::log(&local_d8,&local_d0);
            }
            if (puVar10 != (undefined8 *)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,puVar10,1);
            }
            if (puVar8 != (undefined8 *)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,puVar8,1);
            }
            if (puVar6 != (undefined8 *)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,puVar6,1);
            }
            local_110 = 3;
            read_text(&local_d0,param_2,param_3);
            pcVar3 = pcStack_b8;
            puVar7 = local_c0;
            puVar6 = puStack_c8;
            if (local_d0 != (undefined8 *)&DAT_0000000b) goto LAB_1005c1d5c;
            commands::system::compose_image_compat_flag(&puStack_c8,local_c0,pcStack_b8,uVar1);
            pcVar11 = pcStack_b8;
            local_130 = local_c0;
            puVar8 = puStack_c8;
            if ((pcStack_b8 == pcVar3) && (iVar4 = _memcmp(local_c0,puVar7,(size_t)pcVar3), iVar4 == 0)) goto LAB_1005c1da0;
            read_text(&local_d0,param_2,param_3);
            puVar2 = local_c0;
            puVar10 = puStack_c8;
            local_128 = local_c0;
            if (local_d0 != (undefined8 *)&DAT_0000000b) goto LAB_1005c1d78;
            if ((pcStack_b8 != pcVar3) || (iVar4 = _memcmp(local_c0,puVar7,(size_t)pcVar3), iVar4 != 0)) {
              if (1 < log::MAX_LOG_LEVEL_FILTER) {
                local_108 = &local_120;
                puStack_100 = &<&T_as_core::fmt::Display>::fmt;
                local_f8 = &local_110;
                pcStack_f0 = usize>::fmt;
                local_e8 = &DAT_1013a14b8;
                pcStack_e0 = usize>::fmt;
                local_98 = &DAT_10139db69;
                local_90 = 0x21;
                ppuStack_78 = &local_108;
                local_80 = s___AiMaMi__codex_config__config_c_101526556;
                local_d0 = (undefined8 *)0x0;
                puStack_c8 = (undefined8 *)&DAT_10139db69;
                pcStack_b8 = (code *)0x0;
                local_c0 = (ulong *)&DAT_00000021;
                local_b0 = "src/core/codex_config.rs";
                uStack_a0 = 2;
                uStack_a8 = 0x18;
                uStack_88 = 0x2600000001;
                <log::__private_api::GlobalLogger_as_log::Log>::log(&local_d8,&local_d0);
              }
              if (puVar10 != (undefined8 *)0x0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,puVar10,1);
              }
              if (puVar8 != (undefined8 *)0x0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,puVar8,1);
              }
              if (puVar6 != (undefined8 *)0x0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,puVar6,1);
              }
              local_d0 = &local_120;
              puStack_c8 = &<&T_as_core::fmt::Display>::fmt;
              alloc::fmt::format::format_inner(param_1 + 1,s_E_could_not_update_config_toml_b_10152650e,&local_d0);
              *param_1 = 10;
              return;
            }
          }
        }
        auVar14 = relay::atomic_write::write_atomic_private_report(param_2,param_3,local_130,pcVar11);
        if ((auVar14._0_8_ & 1) == 0) {
          relay::atomic_write::Publication::acknowledge(auVar14._8_8_);
          uVar12 = 0xb;
        }
        else {
          local_d0 = &local_120;
          puStack_c8 = &<&T_as_core::fmt::Display>::fmt;
          local_c0 = &local_d8;
          pcStack_b8 = <std::io::error::Error_as_core::fmt::Display>::fmt;
          local_d8 = auVar14._8_8_;
          alloc::fmt::format::format_inner(&local_108,s_failed_to_write_config_toml__1015265b5,&local_d0);
          puStack_c8 = puStack_100;
          local_d0 = local_108;
          local_c0 = local_f8;
          if ((local_d8 & 3) == 1) {
            puVar9 = (undefined8 *)(local_d8 - 1);
            uVar12 = *puVar9;
            puVar13 = *(undefined8 **)(local_d8 + 7);
            if ((code *)*puVar13 != (code *)0x0) {
              (*(code *)*puVar13)(uVar12);
            }
            if (puVar13[1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar12,puVar13[1],puVar13[2]);
            }
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar9,0x18,8);
          }
          param_1[2] = puStack_c8;
          param_1[1] = local_d0;
          param_1[3] = local_c0;
          uVar12 = 10;
        }
        *param_1 = uVar12;
        if (puVar10 != (undefined8 *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,puVar10,1);
        }
      }
      else {
LAB_1005c1d78:
        param_1[5] = uStack_a8;
        param_1[4] = local_b0;
        param_1[7] = local_98;
        param_1[6] = uStack_a0;
        param_1[9] = uStack_88;
        param_1[8] = local_90;
        param_1[0xb] = ppuStack_78;
        param_1[10] = local_80;
        param_1[2] = local_c0;
        param_1[3] = pcStack_b8;
        *param_1 = local_d0;
        param_1[1] = puStack_c8;
      }
    }
    if (puVar8 != (undefined8 *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,puVar8,1);
    }
    if (puVar6 != (undefined8 *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,puVar6,1);
    }
  }
  else {
LAB_1005c1d5c:
    param_1[5] = uStack_a8;
    param_1[4] = local_b0;
    param_1[7] = local_98;
    param_1[6] = uStack_a0;
    param_1[9] = uStack_88;
    param_1[8] = local_90;
    param_1[0xb] = ppuStack_78;
    param_1[10] = local_80;
    param_1[2] = local_c0;
    param_1[3] = pcStack_b8;
    *param_1 = local_d0;
    param_1[1] = puStack_c8;
  }
  return;
}

