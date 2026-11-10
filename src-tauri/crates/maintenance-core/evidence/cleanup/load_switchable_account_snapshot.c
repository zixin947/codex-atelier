/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* load_switchable_account_snapshot @ 1008313e8 requested 1008313e8 */

/* codexmate_lib::core::repository::load_switchable_account_snapshot */

void codexmate_lib::core::repository::load_switchable_account_snapshot(long *param_1,undefined8 param_2,undefined8 param_3)

{
  ulong uVar1;
  char *pcVar2;
  long local_250;
  long lStack_248;
  long lStack_240;
  long local_238;
  long local_230;
  long lStack_228;
  long local_220;
  long local_218;
  long local_210;
  long lStack_208;
  long local_200;
  long local_1f8;
  long lStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  long local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  long lStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  long local_1a0;
  long local_198;
  long lStack_190;
  long local_188;
  long lStack_180;
  long local_178;
  long lStack_170;
  long local_168;
  long lStack_160;
  long local_158;
  long lStack_150;
  long local_148;
  long lStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  long local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  long lStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  long *local_f0;
  code *pcStack_e8;
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
  long lStack_80;
  long lStack_78;
  long *local_68;
  code *pcStack_60;
  long local_58;

  auth::load_auth_file(&local_1a0,param_2,param_3);
  if (local_1a0 == -0x7fffffffffffffff) {
    lStack_228 = lStack_170;
    local_230 = local_178;
    local_218 = lStack_160;
    local_220 = local_168;
    lStack_208 = lStack_150;
    local_210 = local_158;
    local_1f8 = lStack_140;
    local_200 = local_148;
    lStack_248 = lStack_190;
    local_250 = local_198;
    local_238 = lStack_180;
    lStack_240 = local_188;
    pcStack_60 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
    local_68 = &local_250;
    alloc::fmt::format::format_inner(&local_d0,s__Account_snapshot_is_not_a_switc_10152d9fc,&local_68);
    local_188 = lStack_c8;
    lStack_190 = local_d0;
    lStack_180 = local_c0;
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_250);
    param_1[6] = lStack_170;
    param_1[5] = local_178;
    param_1[8] = lStack_160;
    param_1[7] = local_168;
    param_1[10] = lStack_150;
    param_1[9] = local_158;
    param_1[0xc] = lStack_140;
    param_1[0xb] = local_148;
    param_1[2] = lStack_190;
    param_1[1] = 9;
    param_1[4] = lStack_180;
    param_1[3] = local_188;
    *param_1 = 2;
  }
  else {
    uStack_1d0 = uStack_120;
    local_1d8 = local_128;
    lStack_1c0 = lStack_110;
    local_1c8 = local_118;
    uStack_1b0 = uStack_100;
    local_1b8 = local_108;
    local_1a8 = local_f8;
    uStack_1e0 = uStack_130;
    local_1e8 = local_138;
    local_220 = lStack_170;
    lStack_228 = local_178;
    local_210 = lStack_160;
    local_218 = local_168;
    local_200 = lStack_150;
    lStack_208 = local_158;
    lStack_1f0 = lStack_140;
    local_1f8 = local_148;
    lStack_240 = lStack_190;
    lStack_248 = local_198;
    local_230 = lStack_180;
    local_238 = local_188;
    local_250 = local_1a0;
    uVar1 = auth::is_switchable_oauth_auth(&local_250);
    if ((uVar1 & 1) == 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      pcVar2 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x38,1);
      if (pcVar2 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x38);
      }
      builtin_strncpy(pcVar2,"Account snapshot is not a switchable ChatGPT OAuth login",0x38);
      param_1[2] = 0x38;
      param_1[3] = (long)pcVar2;
      param_1[4] = 0x38;
      param_1[1] = 9;
      *param_1 = 2;
    }
    else {
      auth::make_auth_snapshot(param_1,&local_250,param_2,param_3);
      if (*param_1 == 2) {
        lStack_a8 = param_1[6];
        local_b0 = param_1[5];
        lStack_98 = param_1[8];
        lStack_a0 = param_1[7];
        lStack_88 = param_1[10];
        local_90 = param_1[9];
        lStack_78 = param_1[0xc];
        lStack_80 = param_1[0xb];
        lStack_c8 = param_1[2];
        local_d0 = param_1[1];
        lStack_b8 = param_1[4];
        local_c0 = param_1[3];
        local_f0 = &local_d0;
        pcStack_e8 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_68,s__Account_snapshot_is_not_a_switc_10152d9fc,&local_f0);
        pcStack_e8 = pcStack_60;
        local_f0 = local_68;
        local_e0 = local_58;
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_d0);
        param_1[4] = local_e0;
        param_1[1] = 9;
        *param_1 = 2;
        param_1[3] = (long)pcStack_e8;
        param_1[2] = (long)local_f0;
      }
    }
    if ((local_1a0 != -0x8000000000000000) && (local_1a0 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_248,local_1a0,1);
    }
    if ((local_238 != -0x8000000000000000) && (local_238 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_230,local_238,1);
    }
    if ((local_220 != -0x8000000000000000) && (local_220 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_218,local_220,1);
    }
    if ((lStack_208 != -0x8000000000000000) && (lStack_208 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_200,lStack_208,1);
    }
    if ((lStack_1f0 != -0x8000000000000000) && (lStack_1f0 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1e8,lStack_1f0,1);
    }
    if ((local_1d8 != -0x8000000000000000) && (local_1d8 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_1d0,local_1d8,1);
    }
    if ((lStack_1c0 != -0x8000000000000000) && (lStack_1c0 != 0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1b8,lStack_1c0,1);
    }
  }
  return;
}
