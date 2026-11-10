/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* open_macos_app_bundle @ 1007c00cc requested 1007c00cc */

/* codexmate_lib::platform::process::open_macos_app_bundle */

void codexmate_lib::platform::process::open_macos_app_bundle(undefined8 *param_1,ulong *param_2,ulong param_3)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  ulong *local_230;
  ulong uStack_228;
  undefined8 local_220;
  long lStack_218;
  undefined8 local_210;
  undefined8 uStack_208;
  undefined8 local_200;
  undefined1 local_1f0 [2] [16];
  undefined8 local_1d0;
  undefined8 uStack_1c8;
  ulong *local_1b8;
  ulong uStack_1b0;
  undefined8 local_1a8;
  long lStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined1 *local_180;
  code *pcStack_178;
  undefined1 (*local_170) [16];
  undefined8 *puStack_168;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  ulong *local_a0;
  code *pcStack_98;
  undefined8 local_90;
  long lStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  ulong local_60;
  ulong *local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  
  std::sys::process::unix::common::Command::new
            (&local_180,
             "/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,0xd);
  std::sys::process::unix::common::Command::arg(&local_180,param_2,param_3);
  std::process::Command::output(&local_1b8,&local_180);
  if (local_1b8 == (ulong *)0x8000000000000000) {
    local_60 = uStack_1b0;
    local_a0 = &local_60;
    pcStack_98 = <std::io::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_58,s_open_app_failed__10152beff,&local_a0);
    pcStack_98 = (code *)uStack_50;
    local_a0 = local_58;
    local_90 = local_48;
    if ((local_60 & 3) == 1) {
      puVar1 = (undefined8 *)(local_60 - 1);
      uVar2 = *puVar1;
      puVar3 = *(undefined8 **)(local_60 + 7);
      if ((code *)*puVar3 != (code *)0x0) {
        (*(code *)*puVar3)(uVar2);
      }
      if (puVar3[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar2,puVar3[1],puVar3[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar1,0x18,8);
    }
    uStack_1c8 = uStack_78;
    local_1d0 = local_80;
    local_200 = local_70;
    lStack_218 = lStack_88;
    local_220 = local_90;
    uStack_208 = uStack_78;
    local_210 = local_80;
    uStack_228 = (ulong)pcStack_98;
    local_230 = local_a0;
    param_1[2] = pcStack_98;
    param_1[1] = local_a0;
    param_1[4] = lStack_88;
    param_1[3] = local_90;
    param_1[6] = uStack_78;
    param_1[5] = local_80;
    param_1[7] = local_70;
    *param_1 = 10;
    ::core::ptr::drop_in_place<std::process::Command>(&local_180);
  }
  else {
    uStack_1c8 = uStack_190;
    local_1d0 = uStack_198;
    uStack_228 = uStack_1b0;
    local_230 = local_1b8;
    lStack_218 = lStack_1a0;
    local_220 = local_1a8;
    uStack_208 = uStack_190;
    local_210 = uStack_198;
    local_200 = local_188;
    local_70 = local_188;
    lStack_88 = lStack_1a0;
    local_90 = local_1a8;
    uStack_78 = uStack_190;
    local_80 = uStack_198;
    pcStack_98 = (code *)uStack_1b0;
    local_a0 = local_1b8;
    ::core::ptr::drop_in_place<std::process::Command>(&local_180);
    uVar2 = local_80;
    if ((int)local_70 == 0) {
      *param_1 = 0xb;
      if (local_a0 != (ulong *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcStack_98,local_a0,1);
      }
      uVar2 = local_80;
    }
    else {
      local_230 = param_2;
      uStack_228 = param_3;
      alloc::string::String::from_utf8_lossy(&local_1b8,local_80,uStack_78);
      local_1f0[0] = ::core::str::<impl_str>::trim_matches(uStack_1b0,local_1a8);
      pcStack_178 = <std::path::Display_as_core::fmt::Display>::fmt;
      local_170 = local_1f0;
      puStack_168 = &<&T_as_core::fmt::Display>::fmt;
      local_180 = (undefined1 *)&local_230;
      alloc::fmt::format::format_inner(&local_b8,s_open_app_failed_for___10152bee4,&local_180);
      if (((ulong)local_1b8 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_1b0,local_1b8,1);
      }
      param_1[2] = uStack_b0;
      param_1[1] = local_b8;
      param_1[3] = local_a8;
      *param_1 = 10;
      if (local_a0 != (ulong *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcStack_98,local_a0,1);
      }
    }
    if (lStack_88 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar2,lStack_88,1);
    }
  }
  return;
}

