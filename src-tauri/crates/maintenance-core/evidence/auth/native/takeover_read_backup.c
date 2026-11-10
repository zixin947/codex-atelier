/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read_backup @ 1001f550c requested 1001f550c */

/* codexmate_lib::core::relay::config_takeover::read_backup */

void codexmate_lib::core::relay::config_takeover::read_backup(long *param_1,undefined8 param_2,undefined8 param_3)

{
  long lVar1;
  long lVar2;
  long local_138;
  undefined8 local_130;
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
  long lStack_c8;
  long local_c0;
  long lStack_b8;
  long local_b0;
  long lStack_a8;
  long lStack_a0;
  long lStack_98;
  long local_90;
  long lStack_88;
  long lStack_80;
  long lStack_78;
  char *local_70;
  undefined8 uStack_68;
  char **local_60;
  undefined8 *puStack_58;

  std::path::Path::_join(&local_138,param_2,param_3,"router-takeover-backup.json",0x1b);
  read_backup_bytes(&local_d0,local_130,uStack_128,"router-takeover-backup.json",0x1b);
  lVar2 = local_c0;
  lVar1 = lStack_c8;
  if (local_d0 == 0xb) {
    if (lStack_c8 == -0x8000000000000000) {
      param_1[1] = -0x8000000000000000;
      *param_1 = 0xb;
    }
    else {
      decode_backup(&local_d0,local_c0);
      lStack_f8 = lStack_98;
      local_100 = lStack_a0;
      lStack_e8 = lStack_88;
      lStack_f0 = local_90;
      lStack_d8 = lStack_78;
      local_e0 = lStack_80;
      lStack_118 = lStack_b8;
      local_120 = local_c0;
      lStack_108 = lStack_a8;
      lStack_110 = local_b0;
      if (local_d0 == 0xb) {
        if (lStack_c8 == -0x8000000000000000) {
          local_70 = "router-takeover-backup.json";
          uStack_68 = 0x1b;
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
          param_1[10] = lStack_80;
          param_1[1] = lStack_c8;
          *param_1 = 9;
          param_1[3] = lStack_b8;
          param_1[2] = local_c0;
        }
        else {
          param_1[7] = lStack_98;
          param_1[6] = lStack_a0;
          param_1[9] = lStack_88;
          param_1[8] = local_90;
          param_1[0xb] = lStack_78;
          param_1[10] = lStack_80;
          param_1[3] = lStack_b8;
          param_1[2] = local_c0;
          param_1[5] = lStack_a8;
          param_1[4] = local_b0;
          *param_1 = 0xb;
          param_1[1] = lStack_c8;
        }
      }
      else {
        param_1[7] = lStack_98;
        param_1[6] = lStack_a0;
        param_1[9] = lStack_88;
        param_1[8] = local_90;
        param_1[0xb] = lStack_78;
        param_1[10] = lStack_80;
        param_1[3] = lStack_b8;
        param_1[2] = local_c0;
        param_1[5] = lStack_a8;
        param_1[4] = local_b0;
        *param_1 = local_d0;
        param_1[1] = lStack_c8;
      }
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
    param_1[10] = lStack_80;
    param_1[2] = local_c0;
    param_1[3] = lStack_b8;
    *param_1 = local_d0;
    param_1[1] = lStack_c8;
  }
  if (local_138 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_130,local_138,1);
  }
  return;
}
