/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* list_codex_processes_matching @ 1007c40b8 requested 1007c40b8 */

/* codexmate_lib::platform::process::list_codex_processes_matching */

void codexmate_lib::platform::process::list_codex_processes_matching(undefined8 *param_1)

{
  ulong uVar1;
  undefined1 *puVar2;
  pid_t pVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  ulong local_258;
  undefined8 local_250;
  undefined8 uStack_248;
  undefined8 local_240;
  undefined8 uStack_238;
  undefined1 local_230 [16];
  undefined8 local_220;
  long lStack_218;
  undefined8 local_210;
  undefined8 uStack_208;
  undefined8 local_200;
  undefined8 *local_1f0;
  undefined8 *local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d0;
  undefined8 uStack_1c8;
  ulong *local_1b0;
  ulong uStack_1a8;
  undefined8 local_1a0;
  long lStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 *local_178;
  undefined1 *local_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 *local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined1 local_130;
  undefined2 local_128;
  ulong *local_b0;
  code *pcStack_a8;
  undefined8 local_a0;
  long lStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  ulong local_70;
  ulong *local_68;
  ulong uStack_60;
  undefined8 local_58;
  
  codex_app_bundle_path(&local_178);
  puVar2 = local_170;
  puVar4 = local_178;
  if (local_178 == (undefined8 *)&DAT_8000000000000000) {
    local_258 = 0x8000000000000000;
  }
  else {
    codex_macos_bundle_main_executable(&local_258,local_170,local_168);
    if (puVar4 != (undefined8 *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,puVar4,1);
    }
  }
  uVar1 = local_258;
  local_240 = 0;
  if (local_258 != 0x8000000000000000) {
    local_240 = local_250;
  }
  uStack_238 = uStack_248;
  std::sys::process::unix::common::Command::new
            (&local_178,
             "ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,2);
  std::sys::process::unix::common::Command::arg(&local_178,&DAT_1013adb08,3);
  std::sys::process::unix::common::Command::arg
            (&local_178,
             "-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,2);
  std::sys::process::unix::common::Command::arg(&local_178,&DAT_1013adb0b,0xd);
  std::process::Command::output(&local_1b0,&local_178);
  if (local_1b0 == (ulong *)0x8000000000000000) {
    local_70 = uStack_1a8;
    local_b0 = &local_70;
    pcStack_a8 = <std::io::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_68,s_ps_failed__10152c1a0,&local_b0);
    pcStack_a8 = (code *)uStack_60;
    local_b0 = local_68;
    local_a0 = local_58;
    if ((local_70 & 3) == 1) {
      puVar4 = (undefined8 *)(local_70 - 1);
      uVar5 = *puVar4;
      puVar6 = *(undefined8 **)(local_70 + 7);
      if ((code *)*puVar6 != (code *)0x0) {
        (*(code *)*puVar6)(uVar5);
      }
      if (puVar6[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar5,puVar6[1],puVar6[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar4,0x18,8);
    }
    uStack_1c8 = uStack_88;
    local_1d0 = local_90;
    local_200 = local_80;
    lStack_218 = lStack_98;
    local_220 = local_a0;
    uStack_208 = uStack_88;
    local_210 = local_90;
    local_230._8_8_ = pcStack_a8;
    local_230._0_8_ = local_b0;
    param_1[2] = pcStack_a8;
    param_1[1] = local_b0;
    param_1[4] = lStack_98;
    param_1[3] = local_a0;
    param_1[6] = uStack_88;
    param_1[5] = local_90;
    param_1[7] = local_80;
    *param_1 = 10;
    ::core::ptr::drop_in_place<std::process::Command>(&local_178);
  }
  else {
    uStack_1c8 = uStack_188;
    local_1d0 = uStack_190;
    local_230._8_8_ = uStack_1a8;
    local_230._0_8_ = local_1b0;
    lStack_218 = lStack_198;
    local_220 = local_1a0;
    uStack_208 = uStack_188;
    local_210 = uStack_190;
    local_200 = local_180;
    local_80 = local_180;
    lStack_98 = lStack_198;
    local_a0 = local_1a0;
    uStack_88 = uStack_188;
    local_90 = uStack_190;
    pcStack_a8 = (code *)uStack_1a8;
    local_b0 = local_1b0;
    ::core::ptr::drop_in_place<std::process::Command>(&local_178);
    if ((int)local_80 == 0) {
      pVar3 = std::process::id();
      local_230._0_4_ = pVar3;
      alloc::string::String::from_utf8_lossy(&local_1f0,pcStack_a8,local_a0);
      puVar4 = local_1e8;
      local_168 = 0;
      uStack_160 = uStack_1e0;
      local_158 = local_1e8;
      uStack_150 = uStack_1e0;
      local_148 = 0;
      uStack_140 = uStack_1e0;
      local_138 = 0xa0000000a;
      local_130 = 1;
      local_128 = 0;
      local_178 = &local_240;
      local_170 = local_230;
      <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_1b0,&local_178);
      param_1[2] = uStack_1a8;
      param_1[1] = local_1b0;
      param_1[3] = local_1a0;
      *param_1 = 0xb;
      if (((ulong)local_1f0 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar4,local_1f0,1);
      }
    }
    else {
      alloc::string::String::from_utf8_lossy(&local_178,local_90,uStack_88);
      puVar2 = local_170;
      local_230 = ::core::str::<impl_str>::trim_matches(local_170,local_168);
      local_1f0 = (undefined8 *)local_230;
      local_1e8 = &<&T_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_1b0,s_ps_failed__10152c1a0,&local_1f0);
      if (((ulong)local_178 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,local_178,1);
      }
      param_1[2] = uStack_1a8;
      param_1[1] = local_1b0;
      param_1[3] = local_1a0;
      *param_1 = 10;
    }
    if (local_b0 != (ulong *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcStack_a8,local_b0,1);
    }
    if (lStack_98 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_90,lStack_98,1);
    }
  }
  if ((uVar1 & 0x7fffffffffffffff) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_250,uVar1,1);
  }
  return;
}

