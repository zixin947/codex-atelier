/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* collect_external_codex_writers @ 1007c4890 requested 1007c4890 */

/* codexmate_lib::platform::process::collect_external_codex_writers */

void codexmate_lib::platform::process::collect_external_codex_writers(undefined8 *param_1)

{
  undefined8 uVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long local_240;
  long lStack_238;
  ulong local_230;
  long lStack_228;
  undefined8 *local_220;
  ulong uStack_218;
  undefined8 local_210;
  long local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 *local_1e8;
  undefined8 *local_1e0;
  undefined8 local_1d8;
  long local_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  ulong local_1b0;
  long local_1a8;
  long local_1a0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 *local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  ulong uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined1 local_90;
  undefined2 local_88;
  ulong local_80;
  undefined1 local_78 [16];
  undefined8 local_68;
  ulong *local_60;
  code *pcStack_58;
  
  std::sys::process::unix::common::Command::new
            (&local_1b0,
             "ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,2);
  std::sys::process::unix::common::Command::arg(&local_1b0,&DAT_1013adb08,3);
  std::sys::process::unix::common::Command::arg
            (&local_1b0,
             "-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,2);
  std::sys::process::unix::common::Command::arg(&local_1b0,&DAT_1013adb0b,0xd);
  std::process::Command::output(&local_1e8,&local_1b0);
  uVar3 = local_1d8;
  puVar4 = local_1e0;
  puVar2 = local_1e8;
  if (local_1e8 == (undefined8 *)0x8000000000000000) {
    local_80 = (ulong)local_1e0;
    local_60 = &local_80;
    pcStack_58 = <std::io::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(local_78,s_ps_failed__10152c1a0,&local_60);
    if ((local_80 & 3) == 1) {
      puVar2 = (undefined8 *)(local_80 - 1);
      uVar3 = *puVar2;
      puVar4 = *(undefined8 **)(local_80 + 7);
      if ((code *)*puVar4 != (code *)0x0) {
        (*(code *)*puVar4)(uVar3);
      }
      if (puVar4[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar3,puVar4[1],puVar4[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,0x18,8);
    }
    ::core::ptr::drop_in_place<std::process::Command>(&local_1b0);
    uVar3 = local_78._0_8_;
    uVar5 = local_78._8_8_;
    uVar6 = local_68;
  }
  else {
    uStack_200 = uStack_1c8;
    local_208 = local_1d0;
    uStack_1f0 = uStack_1b8;
    local_1f8 = uStack_1c0;
    local_220 = local_1e8;
    uStack_218 = (ulong)local_1e0;
    local_210 = local_1d8;
    ::core::ptr::drop_in_place<std::process::Command>(&local_1b0);
    uVar1 = uStack_200;
    if ((int)uStack_1f0 == 0) {
      local_78._0_4_ = std::process::id();
      alloc::string::String::from_utf8_lossy(&local_1e8,puVar4,uVar3);
      local_c0 = local_1d8;
      uStack_b8 = (ulong)local_1e0;
      local_b0 = local_1d8;
      uStack_a8 = 0;
      local_a0 = local_1d8;
      local_98 = 0xa0000000a;
      local_90 = 1;
      local_88 = 0;
      local_d0 = (undefined8 *)local_78;
      uStack_c8 = 0;
      <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_1b0,&local_d0);
      if (((ulong)local_1e8 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1e0,local_1e8,1);
      }
      if (puVar2 != (undefined8 *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar4,puVar2,1);
      }
      if (local_208 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_200,local_208,1);
      }
      local_240 = local_1a8;
      lStack_238 = local_1a8;
      lStack_228 = local_1a8 + local_1a0 * 0x20;
      local_230 = local_1b0;
      alloc::vec::in_place_collect::from_iter_in_place(param_1 + 1,&local_240);
      *param_1 = 0xb;
      return;
    }
    alloc::string::String::from_utf8_lossy(&local_1b0,uStack_200,local_1f8);
    local_78 = ::core::str::<impl_str>::trim_matches(local_1a8,local_1a0);
    local_1e8 = (undefined8 *)local_78;
    local_1e0 = &<&T_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_e8,s_ps_failed__10152c1a0,&local_1e8);
    uVar3 = local_e8;
    uVar5 = uStack_e0;
    uVar6 = local_d8;
    if ((local_1b0 & 0x7fffffffffffffff) != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1a8,local_1b0,1);
      uVar3 = local_e8;
      uVar5 = uStack_e0;
      uVar6 = local_d8;
    }
    if (puVar2 != (undefined8 *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar4,puVar2,1);
    }
    if (local_208 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar1,local_208,1);
    }
  }
  *param_1 = 10;
  param_1[1] = uVar3;
  param_1[2] = uVar5;
  param_1[3] = uVar6;
  return;
}

