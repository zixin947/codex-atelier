/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read_user_owned_backup @ 100201ec4 requested 100201ec4 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::legacy_virtual_auth::read_user_owned_backup */

void codexmate_lib::core::relay::legacy_virtual_auth::read_user_owned_backup(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  ulong uVar1;
  ulong ****ppppuVar2;
  ulong ****ppppuVar3;
  ulong ****ppppuVar4;
  int iVar5;
  ulong uVar6;
  char *pcVar7;
  undefined8 *puVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  ulong local_230;
  ulong ****ppppuStack_228;
  ulong ****local_220;
  ulong ****ppppuStack_218;
  long *local_210;
  ulong local_208;
  long lStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  long local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  long lStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  long local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  ulong uStack_1a0;
  ulong ****local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  ulong ****local_180;
  ulong ****ppppuStack_178;
  long *local_170;
  ulong ****local_160;
  ulong ****ppppuStack_158;
  long *local_150;
  ulong local_140;
  ulong ****local_138;
  ulong ****local_130;
  ulong ****ppppuStack_128;
  long *local_120;
  ulong local_118;
  long lStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  long local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  long lStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  long local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  ulong uStack_b0;
  ulong ****local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 *****local_90;
  ulong ****ppppuStack_88;
  long *local_80;
  undefined8 uStack_78;
  undefined8 *****local_70;
  ulong ****ppppuStack_68;
  long *local_60;
  ulong ****local_58;

  std::fs::read::inner(&local_230,param_2,param_3);
  ppppuVar3 = local_220;
  ppppuVar2 = ppppuStack_228;
  uVar1 = local_230;
  if (local_230 == 0x8000000000000000) {
    local_70 = (undefined8 *****)ppppuStack_228;
    local_90 = &local_70;
    ppppuStack_88 = (ulong ****)<std::io::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_140,s_read_auth_backup_failed__10151d58b,&local_90);
    if (((ulong)local_70 & 3) == 1) {
      puVar8 = (undefined8 *)((long)local_70 - 1);
      uVar9 = *puVar8;
      puVar10 = *(undefined8 **)((long)local_70 + 7);
      if ((code *)*puVar10 != (code *)0x0) {
        (*(code *)*puVar10)(uVar9);
      }
      if (puVar10[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar9,puVar10[1],puVar10[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar8,0x18,8);
    }
    param_1[2] = local_138;
    param_1[3] = local_130;
    *param_1 = 10;
    param_1[1] = local_140;
  }
  else {
    local_90 = (undefined8 *****)ppppuStack_228;
    ppppuStack_88 = local_220;
    local_80 = (long *)0x0;
    uStack_78 = 0;
    serde_json::de::from_trait(&local_140,&local_90);
    if (local_140 == 0x8000000000000001) {
      local_58 = local_138;
      local_70 = &local_58;
      ppppuStack_68 = (ulong ****)<serde_json::error::Error_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_90,s_5legacy_auth_backup_is_invalid__b_10151d553,&local_70);
      ppppuVar3 = local_58;
      ppppuStack_68 = ppppuStack_88;
      local_70 = local_90;
      local_60 = local_80;
      ::core::ptr::drop_in_place<serde_json::error::ErrorCode>(local_58);
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar3,0x28,8);
      param_1[2] = ppppuStack_68;
      param_1[1] = local_70;
      param_1[3] = local_60;
      *param_1 = 10;
      local_180 = (ulong ****)local_70;
      ppppuStack_178 = ppppuStack_68;
      local_170 = local_60;
      local_160 = (ulong ****)local_70;
      ppppuStack_158 = ppppuStack_68;
      local_150 = local_60;
    }
    else {
      local_150 = local_120;
      ppppuStack_158 = ppppuStack_128;
      local_160 = local_130;
      lStack_200 = lStack_110;
      local_208 = local_118;
      uStack_1f0 = uStack_100;
      local_1f8 = local_108;
      uStack_1e0 = uStack_f0;
      local_1e8 = local_f8;
      lStack_1d0 = lStack_e0;
      local_1d8 = local_e8;
      uStack_1b0 = uStack_c0;
      local_1b8 = local_c8;
      uStack_1a0 = uStack_b0;
      local_1a8 = local_b8;
      uStack_190 = uStack_a0;
      local_198 = local_a8;
      local_188 = local_98;
      uStack_1c0 = uStack_d0;
      local_1c8 = local_d8;
      local_210 = local_120;
      ppppuStack_218 = ppppuStack_128;
      local_220 = local_130;
      local_230 = local_140;
      ppppuStack_228 = local_138;
      uVar6 = auth::is_aimami_managed_virtual_auth(&local_230);
      ppppuVar4 = ppppuStack_218;
      if (((uVar6 & 1) == 0) &&
         ((((ppppuStack_218 == (ulong ****)0x8000000000000000 || (local_208 < 0x17)) ||
           ((*local_210 != 0x6d616d69612d6b73 || local_210[1] != 0x2d79616c65722d69) || *(long *)((long)local_210 + 0xf) != 0x6c6175747269762d)) &&
          ((uVar6 = auth::is_switchable_oauth_auth(&local_230), (uVar6 & 1) != 0 || (iVar5 = auth::is_api_key_auth(&local_230), iVar5 != 0)))))) {
        param_1[2] = ppppuVar2;
        param_1[3] = ppppuVar3;
        *param_1 = 0xb;
        param_1[1] = uVar1;
        if ((local_140 & 0x7fffffffffffffff) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_138,local_140,1);
        }
        if ((ppppuVar4 != (ulong ****)0x8000000000000000) && (ppppuVar4 != (ulong ****)0x0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_210,ppppuVar4,1);
        }
        if ((lStack_200 != -0x8000000000000000) && (lStack_200 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1f8,lStack_200,1);
        }
        if ((local_1e8 != -0x8000000000000000) && (local_1e8 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_1e0,local_1e8,1);
        }
        if ((lStack_1d0 != -0x8000000000000000) && (lStack_1d0 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1c8,lStack_1d0,1);
        }
        if ((local_1b8 != -0x8000000000000000) && (local_1b8 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_1b0,local_1b8,1);
        }
        uVar1 = uStack_1a0;
        ppppuVar2 = local_198;
        if (uStack_1a0 == 0x8000000000000000) {
          return;
        }
      }
      else {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        pcVar7 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x3f,1);
        if (pcVar7 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x3f);
        }
        builtin_strncpy(pcVar7,"legacy auth backup is not user-owned auth; backup was preserved",0x3f);
        param_1[1] = 0x3f;
        *param_1 = 10;
        param_1[2] = pcVar7;
        param_1[3] = 0x3f;
        if ((local_140 & 0x7fffffffffffffff) != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_138,local_140,1);
        }
        if ((ppppuStack_218 != (ulong ****)0x8000000000000000) && (ppppuStack_218 != (ulong ****)0x0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_210,ppppuStack_218,1);
        }
        if ((lStack_200 != -0x8000000000000000) && (lStack_200 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1f8,lStack_200,1);
        }
        if ((local_1e8 != -0x8000000000000000) && (local_1e8 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_1e0,local_1e8,1);
        }
        if ((lStack_1d0 != -0x8000000000000000) && (lStack_1d0 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1c8,lStack_1d0,1);
        }
        if ((local_1b8 != -0x8000000000000000) && (local_1b8 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_1b0,local_1b8,1);
        }
        if ((uStack_1a0 != 0x8000000000000000) && (uStack_1a0 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_198,uStack_1a0,1);
        }
      }
    }
    if (uVar1 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar2,uVar1,1);
    }
  }
  return;
}
