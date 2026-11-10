/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read_macos_bundle_value @ 1007c0b4c requested 1007c0b4c */

/* codexmate_lib::platform::process::read_macos_bundle_value */

void codexmate_lib::platform::process::read_macos_bundle_value(size_t *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5)

{
  ulong uVar1;
  void *pvVar2;
  size_t sVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  undefined1 auVar6 [16];
  long local_1b0;
  undefined8 uStack_1a8;
  undefined8 local_1a0;
  long local_198;
  ulong local_190;
  undefined8 uStack_188;
  ulong local_180;
  undefined8 local_178;
  undefined8 local_170;
  undefined8 uStack_168;
  long local_160;
  ulong local_158;
  undefined8 uStack_150;
  ulong local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  ulong local_128;
  undefined8 local_120;
  undefined8 uStack_118;
  
  std::path::Path::_join
            (&local_1b0,param_2,param_3,
             "Contents/Info.plist/usr/bin/plutilraw/.vscode/extensions//.vscode-insiders/extensions//.vscode-oss/extensions//.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt"
             ,0x13);
  std::sys::process::unix::common::Command::new
            (&local_128,
             "/usr/bin/plutilraw/.vscode/extensions//.vscode-insiders/extensions//.vscode-oss/extensions//.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt"
             ,0xf);
  std::sys::process::unix::common::Command::arg(&local_128,&DAT_1013af2e8,8);
  std::sys::process::unix::common::Command::arg(&local_128,param_4,param_5);
  std::sys::process::unix::common::Command::arg
            (&local_128,
             "raw/.vscode/extensions//.vscode-insiders/extensions//.vscode-oss/extensions//.cursor/extensions//.windsurf/extensions//extensions/openai.chatgpt",
             3);
  std::sys::process::unix::common::Command::arg
            (&local_128,
             "-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,2);
  std::sys::process::unix::common::Command::arg(&local_128,"-size overflows MAX_SIZEextraHeaders must be a JSON object",1);
  std::sys::process::unix::common::Command::arg(&local_128,uStack_1a8,local_1a0);
  if (local_1b0 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_1a8,local_1b0,1);
  }
  std::process::Command::output(&local_160,&local_128);
  if (local_160 == -0x8000000000000000) {
    if ((local_158 & 3) == 1) {
      uVar4 = *(undefined8 *)(local_158 - 1);
      puVar5 = *(undefined8 **)(local_158 + 7);
      if ((code *)*puVar5 != (code *)0x0) {
        (*(code *)*puVar5)(uVar4);
      }
      if (puVar5[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar4,puVar5[1],puVar5[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(local_158 - 1),0x18,8);
    }
    *param_1 = 0x8000000000000000;
    ::core::ptr::drop_in_place<std::process::Command>(&local_128);
  }
  else {
    uStack_188 = uStack_150;
    local_190 = local_158;
    local_178 = uStack_140;
    local_180 = local_148;
    uStack_168 = uStack_130;
    local_170 = local_138;
    local_198 = local_160;
    ::core::ptr::drop_in_place<std::process::Command>(&local_128);
    uVar1 = local_190;
    if ((int)uStack_168 == 0) {
      alloc::string::String::from_utf8_lossy(&local_128,local_190,uStack_188);
      auVar6 = ::core::str::<impl_str>::trim_matches(local_120,uStack_118);
      sVar3 = auVar6._8_8_;
      if ((long)sVar3 < 0) {
        uVar4 = 0;
LAB_1007c0d50:
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(uVar4,sVar3);
      }
      if (sVar3 == 0) {
        pvVar2 = (void *)0x1;
      }
      else {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar4 = 1;
        pvVar2 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar3,1);
        if (pvVar2 == (void *)0x0) goto LAB_1007c0d50;
      }
      _memcpy(pvVar2,auVar6._0_8_,sVar3);
      *param_1 = sVar3;
      param_1[1] = (size_t)pvVar2;
      param_1[2] = sVar3;
      if (local_160 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar1,local_160,1);
      }
      if (local_180 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_178,local_180,1);
      }
      uVar1 = local_128 & 0x7fffffffffffffff;
    }
    else {
      *param_1 = 0x8000000000000000;
      local_128 = local_180;
      local_120 = local_178;
      uVar1 = local_180;
      if (local_160 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_190,local_160,1);
        local_128 = local_180;
        local_120 = local_178;
        uVar1 = local_180;
      }
    }
    if (uVar1 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_120,local_128,1);
    }
  }
  return;
}

