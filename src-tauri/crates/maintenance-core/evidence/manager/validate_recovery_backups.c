/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* validate_recovery_backups @ 1001fb3c8 requested 1001fb3c8 */

/* codexmate_lib::core::relay::config_takeover::validate_recovery_backups */

void codexmate_lib::core::relay::config_takeover::validate_recovery_backups(long *param_1,long param_2)

{
  long lVar1;
  long lVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  long lVar7;
  long local_138;
  long local_130;
  undefined8 uStack_128;
  long local_120;
  long lStack_118;
  long lStack_110;
  long lStack_108;
  long local_100;
  long lStack_f8;
  long lStack_f0;
  long lStack_e8;
  long local_e0;
  long lStack_d8;
  long local_d0;
  long local_c0;
  long local_b8;
  long lStack_b0;
  long local_a8;
  long lStack_a0;
  long local_98;
  long lStack_90;
  long local_88;
  long lStack_80;
  long local_78;
  long lStack_70;
  long local_68;

  uVar4 = *(undefined8 *)(param_2 + 0x248);
  uVar5 = *(undefined8 *)(param_2 + 0x250);
  std::path::Path::_join(&local_138,uVar4,uVar5,"router-takeover-backup.json",0x1b);
  read_backup_bytes(&local_c0,local_130,uStack_128,"router-takeover-backup.json",0x1b);
  lVar3 = lStack_b0;
  lVar2 = local_b8;
  lVar1 = local_130;
  if (local_c0 == 0xb) {
    if (local_b8 != -0x8000000000000000) {
      decode_backup(&local_c0,lStack_b0);
      lStack_f8 = lStack_90;
      local_100 = local_98;
      lStack_e8 = lStack_80;
      lStack_f0 = local_88;
      lStack_d8 = lStack_70;
      local_e0 = local_78;
      local_d0 = local_68;
      lStack_118 = lStack_b0;
      local_120 = local_b8;
      lStack_108 = lStack_a0;
      lStack_110 = local_a8;
      if (local_c0 != 0xb) {
        param_1[6] = lStack_90;
        param_1[5] = local_98;
        param_1[8] = lStack_80;
        param_1[7] = local_88;
        param_1[10] = lStack_70;
        param_1[9] = local_78;
        param_1[0xb] = local_68;
        param_1[2] = lStack_b0;
        param_1[1] = local_b8;
        param_1[4] = lStack_a0;
        param_1[3] = local_a8;
        *param_1 = local_c0;
        if (lVar2 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar3,lVar2,1);
        }
        goto joined_r0x0001001fb570;
      }
      if (local_b8 != -0x8000000000000000) {
        ::core::ptr::drop_in_place<codexmate_lib::core::relay::config_takeover::TakeoverBackup>(&local_120);
      }
      if (lVar2 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar3,lVar2,1);
      }
    }
    if (local_138 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,local_138,1);
    }
    std::path::Path::_join(&local_120,uVar4,uVar5,"codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey",0x22);
    lVar1 = lStack_118;
    read_backup_bytes(&local_c0,lStack_118,lStack_110,"codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey",
                      0x22);
    lVar3 = lStack_b0;
    lVar2 = local_b8;
    if (local_c0 == 0xb) {
      if (local_b8 != -0x8000000000000000) {
        decode_backup(&local_c0,lStack_b0);
        if (local_c0 != 0xb) {
          param_1[6] = lStack_90;
          param_1[5] = local_98;
          param_1[8] = lStack_80;
          param_1[7] = local_88;
          param_1[10] = lStack_70;
          param_1[9] = local_78;
          param_1[0xb] = local_68;
          param_1[2] = lStack_b0;
          param_1[3] = local_a8;
          param_1[4] = lStack_a0;
          *param_1 = local_c0;
          param_1[1] = local_b8;
          local_138 = local_120;
          if (lVar2 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar3,lVar2,1);
            local_138 = local_120;
          }
          goto joined_r0x0001001fb570;
        }
        if (local_b8 != -0x8000000000000000) {
          if (local_a8 != 0) {
            puVar6 = (undefined8 *)(lStack_b0 + 8);
            lVar7 = local_a8;
            do {
              if (puVar6[-1] != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar6,puVar6[-1],1);
              }
              puVar6 = puVar6 + 3;
              lVar7 = lVar7 + -1;
            } while (lVar7 != 0);
          }
          if (local_b8 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_b0,local_b8 * 0x18,8);
          }
        }
        if (lVar2 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar3,lVar2,1);
        }
      }
      if (local_120 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar1,local_120,1);
      }
      *param_1 = 0xb;
      return;
    }
    param_1[5] = local_98;
    param_1[4] = lStack_a0;
    param_1[7] = local_88;
    param_1[6] = lStack_90;
    param_1[9] = local_78;
    param_1[8] = lStack_80;
    param_1[0xb] = local_68;
    param_1[10] = lStack_70;
    param_1[2] = lStack_b0;
    param_1[3] = local_a8;
    *param_1 = local_c0;
    param_1[1] = local_b8;
    local_138 = local_120;
  }
  else {
    param_1[5] = local_98;
    param_1[4] = lStack_a0;
    param_1[7] = local_88;
    param_1[6] = lStack_90;
    param_1[9] = local_78;
    param_1[8] = lStack_80;
    param_1[0xb] = local_68;
    param_1[10] = lStack_70;
    param_1[2] = lStack_b0;
    param_1[3] = local_a8;
    *param_1 = local_c0;
    param_1[1] = local_b8;
  }
joined_r0x0001001fb570:
  if (local_138 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar1,local_138,1);
  }
  return;
}
