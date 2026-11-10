/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* codex_macos_bundle_main_executable @ 1007c6254 requested 1007c6254 */

/* codexmate_lib::platform::process::codex_macos_bundle_main_executable */

void codexmate_lib::platform::process::codex_macos_bundle_main_executable(long *param_1,undefined8 param_2,undefined8 param_3)

{
  uint uVar1;
  byte bVar2;
  long lVar3;
  ulong uVar4;
  uint uVar5;
  long local_88;
  long lStack_80;
  long local_78;
  long local_70;
  undefined8 local_68;
  undefined8 uStack_60;
  long local_58;
  byte *local_50;
  long local_48;
  
  read_macos_bundle_value
            (&local_58,param_2,param_3,
             "CFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,0x12);
  if (local_58 != -0x8000000000000000) {
    if (local_48 == 0x10) {
      uVar5 = 0x20;
      uVar1 = uVar5;
      if (0x19 < *local_50 - 0x41) {
        uVar1 = 0;
      }
      if ((uVar1 | *local_50) == 99) {
        if (0x19 < local_50[1] - 0x41) {
          uVar5 = 0;
        }
        if ((uVar5 | local_50[1]) == 0x6f) {
          uVar5 = 0x20;
          uVar1 = uVar5;
          if (0x19 < local_50[2] - 0x41) {
            uVar1 = 0;
          }
          if ((uVar1 | local_50[2]) == 0x6d) {
            if (0x19 < local_50[3] - 0x41) {
              uVar5 = 0;
            }
            if ((uVar5 | local_50[3]) == 0x2e) {
              uVar5 = 0x20;
              uVar1 = uVar5;
              if (0x19 < local_50[4] - 0x41) {
                uVar1 = 0;
              }
              if ((uVar1 | local_50[4]) == 0x6f) {
                if (0x19 < local_50[5] - 0x41) {
                  uVar5 = 0;
                }
                if ((uVar5 | local_50[5]) == 0x70) {
                  uVar5 = 0x20;
                  uVar1 = uVar5;
                  if (0x19 < local_50[6] - 0x41) {
                    uVar1 = 0;
                  }
                  if ((uVar1 | local_50[6]) == 0x65) {
                    if (0x19 < local_50[7] - 0x41) {
                      uVar5 = 0;
                    }
                    if ((uVar5 | local_50[7]) == 0x6e) {
                      uVar5 = 0x20;
                      uVar1 = uVar5;
                      if (0x19 < local_50[8] - 0x41) {
                        uVar1 = 0;
                      }
                      if ((uVar1 | local_50[8]) == 0x61) {
                        if (0x19 < local_50[9] - 0x41) {
                          uVar5 = 0;
                        }
                        if ((uVar5 | local_50[9]) == 0x69) {
                          uVar5 = 0x20;
                          uVar1 = uVar5;
                          if (0x19 < local_50[10] - 0x41) {
                            uVar1 = 0;
                          }
                          if ((uVar1 | local_50[10]) == 0x2e) {
                            if (0x19 < local_50[0xb] - 0x41) {
                              uVar5 = 0;
                            }
                            if ((uVar5 | local_50[0xb]) == 99) {
                              uVar5 = 0x20;
                              uVar1 = uVar5;
                              if (0x19 < local_50[0xc] - 0x41) {
                                uVar1 = 0;
                              }
                              if ((uVar1 | local_50[0xc]) == 0x6f) {
                                if (0x19 < local_50[0xd] - 0x41) {
                                  uVar5 = 0;
                                }
                                if ((uVar5 | local_50[0xd]) == 100) {
                                  uVar5 = 0x20;
                                  uVar1 = uVar5;
                                  if (0x19 < local_50[0xe] - 0x41) {
                                    uVar1 = 0;
                                  }
                                  if ((uVar1 | local_50[0xe]) == 0x65) {
                                    bVar2 = local_50[0xf];
                                    if (0x19 < bVar2 - 0x41) {
                                      uVar5 = 0;
                                    }
                                    if (local_58 != 0) {
                                      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_50,local_58,1);
                                    }
                                    if (((uVar5 | bVar2) == 0x78) &&
                                       (read_macos_bundle_value
                                                  (&local_58,param_2,param_3,
                                                   "CFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                                                   ,0x12), local_58 != -0x8000000000000000)) {
                                      std::path::Path::_join
                                                (&local_70,param_2,param_3,
                                                 "Contents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                                                 ,0xe);
                                      std::path::Path::join(&local_88,local_68,uStack_60,&local_58);
                                      if (local_70 != 0) {
                                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_68,local_70,1);
                                      }
                                      lVar3 = lStack_80;
                                      uVar4 = std::path::Path::is_file(lStack_80,local_78);
                                      if ((uVar4 & 1) != 0) {
                                        param_1[1] = lStack_80;
                                        *param_1 = local_88;
                                        param_1[2] = local_78;
                                        return;
                                      }
                                      *param_1 = -0x8000000000000000;
                                      if (local_88 == 0) {
                                        return;
                                      }
                                      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar3,local_88,1);
                                      return;
                                    }
                                    goto LAB_1007c654c;
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    if (local_58 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_50,local_58,1);
    }
  }
LAB_1007c654c:
  *param_1 = -0x8000000000000000;
  return;
}

