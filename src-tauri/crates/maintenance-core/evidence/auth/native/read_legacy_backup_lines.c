/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read_legacy_backup_lines @ 1001faac8 requested 1001faac8 */

/* codexmate_lib::core::relay::config_takeover::read_legacy_backup_lines */

void codexmate_lib::core::relay::config_takeover::read_legacy_backup_lines(long *param_1,undefined8 param_2,undefined8 param_3)

{
  long lVar1;
  long lVar2;
  long local_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  long local_f0;
  long lStack_e8;
  long local_e0;
  long local_d0;
  long lStack_c8;
  long local_c0;
  long lStack_b8;
  long local_b0;
  long lStack_a8;
  long lStack_a0;
  long lStack_98;
  long local_90;
  long lStack_88;
  long local_80;
  long lStack_78;
  char *local_70;
  undefined8 uStack_68;
  char **local_60;
  undefined8 *puStack_58;

  std::path::Path::_join(&local_108,param_2,param_3,"codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey",0x22)
  ;
  read_backup_bytes(&local_d0,local_100,uStack_f8,"codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey",0x22);
  lVar2 = local_c0;
  lVar1 = lStack_c8;
  if (local_d0 == 0xb) {
    if (lStack_c8 == -0x8000000000000000) {
      param_1[2] = 8;
      param_1[3] = 0;
      param_1[1] = 0;
      *param_1 = 0xb;
    }
    else {
      decode_backup(&local_d0,local_c0);
      lStack_e8 = lStack_b8;
      local_f0 = local_c0;
      local_e0 = local_b0;
      if (local_d0 == 0xb) {
        if (lStack_c8 == -0x8000000000000000) {
          local_70 = "codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey";
          uStack_68 = 0x22;
          local_60 = &local_70;
          puStack_58 = &<&T_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&lStack_c8,s___is_damaged_and_was_preserved_f_10151d21a,&local_60);
          local_d0 = 9;
          param_1[5] = lStack_a8;
          param_1[4] = local_b0;
          param_1[7] = lStack_98;
          param_1[6] = lStack_a0;
          param_1[9] = lStack_88;
          param_1[8] = local_90;
          param_1[0xb] = lStack_78;
          param_1[10] = local_80;
          param_1[1] = lStack_c8;
          *param_1 = 9;
          param_1[3] = lStack_b8;
          param_1[2] = local_c0;
        }
        else {
          param_1[3] = lStack_b8;
          param_1[2] = local_c0;
          *param_1 = 0xb;
          param_1[1] = lStack_c8;
          local_d0 = 0xb;
          lStack_e8 = lStack_b8;
          local_f0 = local_c0;
          local_e0 = local_b0;
        }
        if (lVar1 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar2,lVar1,1);
        }
        if (local_108 == 0) {
          return;
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_100,local_108,1);
        return;
      }
      param_1[6] = lStack_a0;
      param_1[5] = lStack_a8;
      param_1[8] = local_90;
      param_1[7] = lStack_98;
      param_1[10] = local_80;
      param_1[9] = lStack_88;
      param_1[0xb] = lStack_78;
      param_1[3] = lStack_b8;
      param_1[2] = local_c0;
      param_1[4] = local_b0;
      *param_1 = local_d0;
      param_1[1] = lStack_c8;
      if (lVar1 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar2,lVar1,1);
      }
    }
  }
  else {
    param_1[5] = lStack_a8;
    param_1[4] = local_b0;
    param_1[7] = lStack_98;
    param_1[6] = lStack_a0;
    param_1[9] = lStack_88;
    param_1[8] = local_90;
    param_1[0xb] = lStack_78;
    param_1[10] = local_80;
    param_1[2] = local_c0;
    param_1[3] = lStack_b8;
    *param_1 = local_d0;
    param_1[1] = lStack_c8;
  }
  if (local_108 == 0) {
    return;
  }
  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_100,local_108,1);
  return;
}
