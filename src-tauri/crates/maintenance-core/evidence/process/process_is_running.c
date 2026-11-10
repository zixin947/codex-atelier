/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* is_codex_app_running @ 1007be1e4 requested 1007be1e4 */

/* codexmate_lib::platform::process::is_codex_app_running */

bool codexmate_lib::platform::process::is_codex_app_running(void)

{
  uint uVar1;
  long lVar2;
  ulong uVar3;
  bool bVar4;
  int iVar5;
  byte *pbVar6;
  size_t sVar7;
  uint uVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  undefined1 auVar11 [16];
  long local_150;
  ulong local_148;
  undefined8 uStack_140;
  long local_138;
  undefined8 uStack_130;
  int iStack_120;
  ulong local_118;
  undefined8 local_110;
  undefined8 uStack_108;
  
  codex_app_bundle_path(&local_118);
  uVar9 = local_110;
  uVar3 = local_118;
  if (local_118 != 0x8000000000000000) {
    codex_macos_bundle_main_executable(&local_150,local_110,uStack_108);
    if (uVar3 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar9,uVar3,1);
    }
    uVar3 = local_148;
    lVar2 = local_150;
    if (local_150 != -0x8000000000000000) {
      iVar5 = codex_macos_main_process_pid(local_148,uStack_140);
      if (lVar2 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar3,lVar2,1);
      }
      if (iVar5 == 1) {
        return true;
      }
    }
  }
  std::sys::process::unix::common::Command::new
            (&local_118,
             "/usr/bin/osascript-eCFBundleShortVersionString | ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,0x12);
  std::sys::process::unix::common::Command::arg
            (&local_118,
             "-eCFBundleShortVersionString | ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,2);
  std::sys::process::unix::common::Command::arg
            (&local_118,
             "application id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,0x2c);
  std::process::Command::output(&local_150,&local_118);
  if (local_150 == -0x8000000000000000) {
    if ((local_148 & 3) == 1) {
      uVar9 = *(undefined8 *)(local_148 - 1);
      puVar10 = *(undefined8 **)(local_148 + 7);
      if ((code *)*puVar10 != (code *)0x0) {
        (*(code *)*puVar10)(uVar9);
      }
      if (puVar10[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar9,puVar10[1],puVar10[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(local_148 - 1),0x18,8);
    }
    ::core::ptr::drop_in_place<std::process::Command>(&local_118);
    return false;
  }
  ::core::ptr::drop_in_place<std::process::Command>(&local_118);
  if (iStack_120 != 0) {
    if (local_150 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_148,local_150,1);
    }
    if (local_138 == 0) {
      return false;
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_130,local_138,1);
    return false;
  }
  alloc::string::String::from_utf8_lossy(&local_118,local_148,uStack_140);
  auVar11 = ::core::str::<impl_str>::trim_matches(local_110,uStack_108);
  sVar7 = auVar11._8_8_;
  if ((long)sVar7 < 0) {
    uVar9 = 0;
LAB_1007be3cc:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar9,sVar7);
  }
  if (sVar7 == 0) {
    pbVar6 = (byte *)0x1;
    _memcpy((void *)0x1,auVar11._0_8_,0);
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar9 = 1;
    pbVar6 = (byte *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar7,1);
    if (pbVar6 == (byte *)0x0) goto LAB_1007be3cc;
    _memcpy(pbVar6,auVar11._0_8_,sVar7);
  }
  if (local_150 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_148,local_150,1);
  }
  if (local_138 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_130,local_138,1);
  }
  if ((local_118 & 0x7fffffffffffffff) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_110,local_118,1);
  }
  if (sVar7 == 4) {
    uVar8 = 0x20;
    uVar1 = uVar8;
    if (0x19 < *pbVar6 - 0x41) {
      uVar1 = 0;
    }
    if ((uVar1 | *pbVar6) == 0x74) {
      if (0x19 < pbVar6[1] - 0x41) {
        uVar8 = 0;
      }
      if ((uVar8 | pbVar6[1]) == 0x72) {
        uVar8 = 0x20;
        uVar1 = uVar8;
        if (0x19 < pbVar6[2] - 0x41) {
          uVar1 = 0;
        }
        if ((uVar1 | pbVar6[2]) == 0x75) {
          if (0x19 < pbVar6[3] - 0x41) {
            uVar8 = 0;
          }
          bVar4 = (uVar8 | pbVar6[3]) == 0x65;
          goto LAB_1007be4fc;
        }
      }
    }
  }
  bVar4 = false;
LAB_1007be4fc:
  if (sVar7 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pbVar6,sVar7,1);
    return bVar4;
  }
  return bVar4;
}

