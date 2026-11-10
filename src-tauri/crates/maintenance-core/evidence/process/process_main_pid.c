/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* codex_macos_main_process_pid @ 1007c2f64 requested 1007c2f64 */

/* codexmate_lib::platform::process::codex_macos_main_process_pid */

undefined1  [16] codexmate_lib::platform::process::codex_macos_main_process_pid(undefined8 param_1,undefined8 param_2)

{
  byte *pbVar1;
  byte bVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  ushort uVar6;
  int iVar7;
  long lVar8;
  byte *pbVar10;
  byte *pbVar11;
  byte *pbVar12;
  uint uVar13;
  uint uVar14;
  ulong uVar15;
  undefined8 uVar16;
  undefined8 *puVar17;
  byte *unaff_x23;
  undefined1 auVar18 [16];
  int local_180;
  undefined4 uStack_17c;
  ulong uStack_178;
  long local_170;
  long lStack_168;
  undefined8 local_160;
  int local_150;
  long local_148;
  long lStack_140;
  long local_138;
  long lStack_130;
  undefined8 local_128;
  long lStack_120;
  undefined8 local_118;
  undefined1 local_110;
  undefined2 local_108;
  ulong local_80;
  long local_78;
  long lStack_70;
  long lVar9;
  
  std::sys::process::unix::common::Command::new
            (&local_148,
             "ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,2);
  std::sys::process::unix::common::Command::arg(&local_148,&DAT_1013adb08,3);
  std::sys::process::unix::common::Command::arg
            (&local_148,
             "-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
             ,2);
  std::sys::process::unix::common::Command::arg(&local_148,&DAT_1013adb0b,0xd);
  std::process::Command::output(&local_180,&local_148);
  lVar9 = local_170;
  if (CONCAT44(uStack_17c,local_180) == -0x8000000000000000) {
    if ((uStack_178 & 3) == 1) {
      uVar16 = *(undefined8 *)(uStack_178 - 1);
      puVar17 = *(undefined8 **)(uStack_178 + 7);
      if ((code *)*puVar17 != (code *)0x0) {
        (*(code *)*puVar17)(uVar16);
      }
      if (puVar17[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar16,puVar17[1],puVar17[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(uStack_178 - 1),0x18,8);
    }
    ::core::ptr::drop_in_place<std::process::Command>(&local_148);
  }
  else {
    lVar3 = CONCAT44(uStack_17c,local_180);
    ::core::ptr::drop_in_place<std::process::Command>(&local_148);
    if (local_150 == 0) {
      alloc::string::String::from_utf8_lossy(&local_80,uStack_178,lVar9);
      local_148 = 0;
      lStack_140 = lStack_70;
      local_138 = local_78;
      lStack_130 = lStack_70;
      local_128 = 0;
      lStack_120 = lStack_70;
      local_118 = 0xa0000000a;
      local_110 = 1;
      local_108 = 0;
      do {
        lVar9 = local_138;
        <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(&local_180,&local_138);
        uVar6 = local_108;
        if (local_180 != 1) {
          if ((local_108 & 0x100) == 0) {
            local_108 = CONCAT11(1,(undefined1)local_108);
            if (((uVar6 & 1) != 0) || (lStack_140 != local_148)) {
              lVar8 = lStack_140 - local_148;
              lVar4 = local_148;
              lVar9 = local_138;
              goto joined_r0x0001007c3154;
            }
          }
          uVar16 = 0;
          goto joined_r0x0001007c33b8;
        }
        lVar8 = local_170 - local_148;
        lVar4 = local_170;
joined_r0x0001007c3154:
        lVar9 = lVar9 + local_148;
        lVar5 = lVar8;
        if ((lVar8 != 0) && (*(char *)(lVar9 + lVar8 + -1) == '\n')) {
          if (lVar8 + -1 == 0) {
            lVar5 = 0;
          }
          else {
            lVar5 = lVar8 + -1;
            if (*(char *)(lVar9 + lVar8 + -2) == '\r') {
              lVar5 = lVar8 + -2;
            }
          }
        }
        local_148 = lVar4;
        if (lVar9 == 0) break;
        auVar18 = ::core::str::<impl_str>::trim_start_matches(lVar9,lVar5);
        lVar9 = auVar18._8_8_;
        pbVar10 = auVar18._0_8_;
        if (lVar9 != 0) {
          pbVar11 = pbVar10;
          unaff_x23 = (byte *)0x0;
          do {
            pbVar12 = pbVar11 + 1;
            uVar13 = (uint)*pbVar11;
            uVar15 = (ulong)uVar13;
            if ((char)*pbVar11 < '\0') {
              if (uVar13 < 0xe0) {
                pbVar12 = pbVar11 + 2;
                uVar15 = (ulong)(pbVar11[1] & 0x3f | (uVar13 & 0x1f) << 6);
              }
              else {
                uVar14 = pbVar11[2] & 0x3f | (pbVar11[1] & 0x3f) << 6;
                if (uVar13 < 0xf0) {
                  pbVar12 = pbVar11 + 3;
                  uVar15 = (ulong)(uVar14 | (uVar13 & 0x1f) << 0xc);
                }
                else {
                  pbVar12 = pbVar11 + 4;
                  uVar15 = (ulong)(pbVar11[3] & 0x3f | uVar14 << 6 | (uVar13 & 7) << 0x12);
                }
              }
            }
            pbVar1 = pbVar12 + ((long)unaff_x23 - (long)pbVar11);
            uVar13 = (uint)uVar15;
            if (uVar13 < 0x21 && (1L << (uVar15 & 0x3f) & 0x100003e00U) != 0) {
LAB_1007c32cc:
              if (pbVar10 == (byte *)0x0) break;
              auVar18 = ::core::str::<impl_str>::trim_matches(pbVar10 + (long)pbVar1,lVar9 - (long)pbVar1);
              iVar7 = command_runs_executable(auVar18._0_8_,auVar18._8_8_,param_1,param_2);
              if ((iVar7 == 0) || (unaff_x23 == (byte *)0x0)) break;
              if (unaff_x23 == (byte *)0x1) {
                bVar2 = *pbVar10;
                if ((bVar2 == 0x2b) || (bVar2 == 0x2d)) break;
                uVar15 = (ulong)(bVar2 != 0x2b);
                if (bVar2 == 0x2b) {
                  pbVar10 = pbVar10 + 1;
                }
              }
              else {
                bVar2 = *pbVar10;
                if (bVar2 == 0x2b) {
                  pbVar10 = pbVar10 + 1;
                }
                uVar15 = (long)unaff_x23 - (ulong)(bVar2 == 0x2b);
              }
              if (8 < uVar15) {
                unaff_x23 = (byte *)0x0;
                goto LAB_1007c337c;
              }
              unaff_x23 = (byte *)0x0;
              goto joined_r0x0001007c332c;
            }
            if (0x7f < uVar13) {
              uVar14 = uVar13 >> 8;
              if (uVar14 < 0x20) {
                if (uVar14 == 0) {
                  bVar2 = (&::core::unicode::unicode_data::white_space::WHITESPACE_MAP)[uVar15 & 0xff];
joined_r0x0001007c31dc:
                  if ((bVar2 & 1) != 0) goto LAB_1007c32cc;
                }
                else if ((uVar14 == 0x16) && (uVar13 == 0x1680)) goto LAB_1007c32cc;
              }
              else {
                if (uVar14 == 0x20) {
                  bVar2 = (byte)(&::core::unicode::unicode_data::white_space::WHITESPACE_MAP)[uVar15 & 0xff] >> 1;
                  goto joined_r0x0001007c31dc;
                }
                if ((uVar14 == 0x30) && (uVar13 == 0x3000)) goto LAB_1007c32cc;
              }
            }
            pbVar11 = pbVar12;
            unaff_x23 = pbVar1;
          } while (pbVar12 != pbVar10 + lVar9);
        }
LAB_1007c311c:
      } while ((local_108 & 0x100) == 0);
      uVar16 = 0;
      unaff_x23 = (byte *)0x0;
joined_r0x0001007c33b8:
      if (lVar3 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_178,lVar3,1);
      }
      if (lStack_168 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_160,lStack_168,1);
      }
      if ((local_80 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_78,local_80,1);
      }
      goto LAB_1007c30a4;
    }
    if (lVar3 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_178,lVar3,1);
    }
    if (lStack_168 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_160,lStack_168,1);
    }
  }
  uVar16 = 0;
LAB_1007c30a4:
  auVar18._8_8_ = unaff_x23;
  auVar18._0_8_ = uVar16;
  return auVar18;
joined_r0x0001007c332c:
  if (uVar15 == 0) goto LAB_1007c3428;
  if (9 < *pbVar10 - 0x30) goto LAB_1007c311c;
  unaff_x23 = (byte *)(ulong)((*pbVar10 - 0x30) + (int)unaff_x23 * 10);
  uVar15 = uVar15 - 1;
  pbVar10 = pbVar10 + 1;
  goto joined_r0x0001007c332c;
LAB_1007c337c:
  if (uVar15 == 0) goto LAB_1007c3428;
  if (((long)unaff_x23 * 10 & 0xffffffff00000000U) != 0) goto LAB_1007c311c;
  uVar13 = *pbVar10 - 0x30;
  uVar14 = (uint)((long)unaff_x23 * 10);
  unaff_x23 = (byte *)(ulong)(uVar13 + uVar14);
  if ((9 < uVar13) || (uVar15 = uVar15 - 1, pbVar10 = pbVar10 + 1, CARRY4(uVar13,uVar14))) goto LAB_1007c311c;
  goto LAB_1007c337c;
LAB_1007c3428:
  uVar16 = 1;
  goto joined_r0x0001007c33b8;
}

