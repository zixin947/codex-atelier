/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* list_macos_db_holder_processes_for @ 1007c6c44 requested 1007c6c44 */

/* codexmate_lib::platform::process::list_macos_db_holder_processes_for */

void codexmate_lib::platform::process::list_macos_db_holder_processes_for(long *param_1,undefined8 *******param_2,code *param_3)

{
  int *piVar1;
  undefined1 *puVar2;
  byte bVar3;
  uint uVar4;
  ushort uVar5;
  char **ppcVar6;
  pid_t pVar7;
  undefined1 (*pauVar8) [16];
  long lVar9;
  undefined1 *puVar10;
  char **ppcVar11;
  char **ppcVar12;
  byte *pbVar13;
  ulong uVar14;
  uint uVar15;
  char **ppcVar16;
  char **ppcVar17;
  long lVar18;
  code *pcVar19;
  undefined8 uVar20;
  code *pcVar21;
  undefined8 *puVar22;
  undefined8 uVar23;
  ulong uVar24;
  undefined8 *puVar25;
  undefined1 auVar26 [16];
  undefined1 *local_3b0;
  undefined1 *local_3a8;
  undefined1 *local_3a0;
  char **local_370;
  char **ppcStack_368;
  char **local_360;
  undefined8 ******local_2a0;
  code *pcStack_298;
  char **ppcStack_290;
  undefined8 *local_288;
  undefined8 local_280;
  undefined8 uStack_278;
  undefined8 local_270;
  long local_268;
  char **local_260;
  char **ppcStack_258;
  char *local_250;
  long lStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined1 local_230 [16];
  long local_218;
  long lStack_210;
  long local_208;
  char **local_200;
  char **ppcStack_1f8;
  char **local_1f0;
  char **local_1e8;
  undefined8 uStack_1e0;
  char **local_1d8;
  undefined8 local_1d0;
  undefined1 local_1c8;
  undefined2 local_1c0;
  char **local_1b8;
  char **ppcStack_1b0;
  char **local_1a8;
  long local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  char **local_180;
  char **ppcStack_178;
  char **local_170;
  long local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 ******local_148;
  code *pcStack_140;
  char **local_138;
  undefined8 *local_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined1 auStack_7a [26];
  
  local_1b8 = (char **)0x0;
  ppcStack_1b0 = (char **)&SUB_00000008;
  local_1a8 = (char **)0x0;
  local_250 = (char *)0x1;
  lStack_248 = 0;
  std::path::Path::to_path_buf(&local_180,param_2,param_3);
  std::sys::fs::metadata(&local_148,ppcStack_178,local_170);
  ppcVar17 = local_1a8;
  if (((uint)local_148 & 1) == 0) {
    if (local_1a8 == local_1b8) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_1b8);
    }
    ppcVar11 = ppcStack_1b0 + (long)ppcVar17 * 3;
    *ppcVar11 = (char *)local_180;
    ppcVar11[1] = (char *)ppcStack_178;
    ppcVar11[2] = (char *)local_170;
    local_1a8 = (char **)((long)ppcVar17 + 1);
  }
  else {
    if (((ulong)pcStack_140 & 3) == 1) {
      uVar20 = *(undefined8 *)((long)pcStack_140 - 1U);
      puVar22 = *(undefined8 **)((long)pcStack_140 + 7);
      if ((code *)*puVar22 != (code *)0x0) {
        (*(code *)*puVar22)(uVar20);
      }
      if (puVar22[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar20,puVar22[1],puVar22[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)pcStack_140 - 1U),0x18,8);
    }
    if (local_180 != (char **)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppcStack_178,local_180,1);
    }
  }
  local_250 = "-wal-shm";
  lStack_248 = 4;
  local_148 = &local_2a0;
  pcVar19 = <std::path::Display_as_core::fmt::Display>::fmt;
  pcStack_140 = <std::path::Display_as_core::fmt::Display>::fmt;
  local_138 = &local_250;
  local_130 = &<&T_as_core::fmt::Display>::fmt;
  local_2a0 = param_2;
  pcStack_298 = param_3;
  alloc::fmt::format::format_inner(&local_200,s__10151fed1,&local_148);
  ppcVar11 = local_1f0;
  ppcVar17 = ppcStack_1f8;
  ppcStack_368 = ppcStack_1f8;
  local_370 = local_200;
  local_360 = local_1f0;
  ppcStack_178 = ppcStack_1f8;
  local_180 = local_200;
  local_170 = local_1f0;
  std::sys::fs::metadata(&local_148,ppcStack_1f8,local_1f0);
  ppcVar16 = local_180;
  ppcVar6 = local_1a8;
  if ((uint)local_148 == 1) {
    if (((ulong)pcStack_140 & 3) == 1) {
      pcVar21 = pcStack_140 + -1;
      uVar20 = *(undefined8 *)pcVar21;
      puVar22 = *(undefined8 **)(pcStack_140 + 7);
      if ((code *)*puVar22 != (code *)0x0) {
        (*(code *)*puVar22)(uVar20);
      }
      if (puVar22[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar20,puVar22[1],puVar22[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar21,0x18,8);
    }
    if (local_180 != (char **)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppcVar17,local_180,1);
    }
  }
  else {
    if (local_1a8 == local_1b8) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_1b8);
    }
    ppcVar12 = ppcStack_1b0 + (long)ppcVar6 * 3;
    *ppcVar12 = (char *)ppcVar16;
    ppcVar12[1] = (char *)ppcVar17;
    ppcVar12[2] = (char *)ppcVar11;
    local_1a8 = (char **)((long)ppcVar6 + 1);
  }
  local_250 = "-shm";
  lStack_248 = 4;
  local_148 = &local_2a0;
  pcStack_140 = <std::path::Display_as_core::fmt::Display>::fmt;
  local_138 = &local_250;
  local_130 = &<&T_as_core::fmt::Display>::fmt;
  local_2a0 = param_2;
  pcStack_298 = param_3;
  alloc::fmt::format::format_inner(&local_200,s__10151fed1,&local_148);
  ppcVar11 = local_1f0;
  ppcVar17 = ppcStack_1f8;
  ppcStack_368 = ppcStack_1f8;
  local_370 = local_200;
  local_360 = local_1f0;
  ppcStack_178 = ppcStack_1f8;
  local_180 = local_200;
  local_170 = local_1f0;
  std::sys::fs::metadata(&local_148,ppcStack_1f8,local_1f0);
  ppcVar16 = local_180;
  ppcVar6 = local_1a8;
  if ((uint)local_148 == 1) {
    if (((ulong)pcStack_140 & 3) == 1) {
      pcVar21 = pcStack_140 + -1;
      uVar20 = *(undefined8 *)pcVar21;
      puVar22 = *(undefined8 **)(pcStack_140 + 7);
      if ((code *)*puVar22 != (code *)0x0) {
        (*(code *)*puVar22)(uVar20);
      }
      if (puVar22[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar20,puVar22[1],puVar22[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar21,0x18,8);
    }
    if (local_180 != (char **)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppcVar17,local_180,1);
    }
  }
  else {
    if (local_1a8 == local_1b8) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_1b8);
    }
    ppcVar12 = ppcStack_1b0 + (long)ppcVar6 * 3;
    *ppcVar12 = (char *)ppcVar16;
    ppcVar12[1] = (char *)ppcVar17;
    ppcVar12[2] = (char *)ppcVar11;
    local_1a8 = (char **)((long)ppcVar6 + 1);
  }
  ppcVar17 = local_1a8;
  ppcVar6 = ppcStack_1b0;
  ppcVar11 = local_1b8;
  if (local_1a8 == (char **)0x0) {
    *param_1 = 0;
    param_1[1] = 8;
    param_1[2] = 0;
  }
  else {
    std::sys::process::unix::common::Command::new(&local_370,"lsofNullRealTextBlobRgba",4);
    std::sys::process::unix::common::Command::arg
              (&local_370,
               "-t--Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codexContents/Resources/codexa tuple of size 2a sequencesignala tuple of size 3",2)
    ;
    std::sys::process::unix::common::Command::arg
              (&local_370,
               "--Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codexContents/Resources/codexa tuple of size 2a sequencesignala tuple of size 3",2);
    lVar18 = (long)ppcVar17 * 0x18;
    ppcVar16 = ppcVar6 + 2;
    do {
      std::sys::process::unix::common::Command::arg(&local_370,ppcVar16[-1],*ppcVar16);
      ppcVar16 = ppcVar16 + 3;
      lVar18 = lVar18 + -0x18;
    } while (lVar18 != 0);
    std::process::Command::output(&local_148,&local_370);
    if (local_148 == (undefined8 ******)&DAT_8000000000000000) {
      if (((ulong)pcStack_140 & 3) == 1) {
        pcVar19 = pcStack_140 + -1;
        uVar20 = *(undefined8 *)pcVar19;
        puVar22 = *(undefined8 **)(pcStack_140 + 7);
        if ((code *)*puVar22 != (code *)0x0) {
          (*(code *)*puVar22)(uVar20);
        }
        if (puVar22[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar20,puVar22[1],puVar22[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar19,0x18,8);
      }
      *param_1 = 0;
      param_1[1] = 8;
      param_1[2] = 0;
      ::core::ptr::drop_in_place<std::process::Command>(&local_370);
      ppcVar16 = ppcVar6 + 1;
      do {
        if (ppcVar16[-1] != (char *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppcVar16,ppcVar16[-1],1);
        }
        ppcVar16 = ppcVar16 + 3;
        ppcVar17 = (char **)((long)ppcVar17 + -1);
      } while (ppcVar17 != (char **)0x0);
    }
    else {
      pcStack_298 = pcStack_140;
      local_2a0 = local_148;
      local_288 = local_130;
      ppcStack_290 = local_138;
      uStack_278 = uStack_120;
      local_280 = local_128;
      local_270 = local_118;
      pVar7 = std::process::id();
      alloc::string::String::from_utf8_lossy(&local_268,pcStack_298,ppcStack_290);
      pauVar8 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
      if (pauVar8[1][0] == '\x01') {
        local_230 = *pauVar8;
      }
      else {
        local_230 = std::sys::random::hashmap_random_keys();
        *pauVar8 = local_230;
        pauVar8[1][0] = 1;
      }
      *(long *)*pauVar8 = local_230._0_8_ + 1;
      lStack_248 = 0;
      local_250 = &_anon_6408540cdffbe318d49f52812e99783f_28;
      uStack_238 = 0;
      uStack_240 = 0;
      local_218 = 0;
      lStack_210 = 8;
      local_208 = 0;
      local_200 = (char **)0x0;
      ppcStack_1f8 = ppcStack_258;
      local_1f0 = local_260;
      uVar20 = 1;
      local_1e8 = ppcStack_258;
      uStack_1e0 = 0;
      local_1d8 = ppcStack_258;
      local_1d0 = 0xa0000000a;
      local_1c8 = 1;
      local_1c0 = 0;
      do {
        ppcVar16 = local_1f0;
        <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(&local_148,&local_1f0);
        uVar5 = local_1c0;
        if ((uint)local_148 == 1) {
          lVar18 = (long)local_138 - (long)local_200;
          ppcVar12 = local_138;
        }
        else {
          if ((local_1c0 & 0x100) != 0) break;
          local_1c0 = CONCAT11(1,(undefined1)local_1c0);
          if (((uVar5 & 1) == 0) && (ppcStack_1f8 == local_200)) break;
          lVar18 = (long)ppcStack_1f8 - (long)local_200;
          ppcVar12 = local_200;
          ppcVar16 = local_1f0;
        }
        puVar10 = (undefined1 *)((long)ppcVar16 + (long)local_200);
        lVar9 = lVar18;
        if ((lVar18 != 0) && (puVar10[lVar18 + -1] == '\n')) {
          if (lVar18 + -1 == 0) {
            lVar9 = 0;
          }
          else {
            lVar9 = lVar18 + -1;
            if (puVar10[lVar18 + -2] == '\r') {
              lVar9 = lVar18 + -2;
            }
          }
        }
        local_200 = ppcVar12;
        auVar26 = ::core::str::<impl_str>::trim_matches(puVar10,lVar9);
        lVar18 = auVar26._8_8_;
        pbVar13 = auVar26._0_8_;
        if (lVar18 != 0) {
          if (lVar18 == 1) {
            bVar3 = *pbVar13;
            if ((bVar3 != 0x2b) && (bVar3 != 0x2d)) {
              uVar14 = (ulong)(bVar3 != 0x2b);
              if (bVar3 == 0x2b) {
                pbVar13 = pbVar13 + 1;
              }
              if (8 < uVar14) goto LAB_1007c7354;
LAB_1007c72ac:
              uVar24 = 0;
              for (; uVar14 != 0; uVar14 = uVar14 - 1) {
                if (9 < *pbVar13 - 0x30) goto LAB_1007c71cc;
                uVar24 = (ulong)((*pbVar13 - 0x30) + (int)uVar24 * 10);
                pbVar13 = pbVar13 + 1;
              }
LAB_1007c72d0:
              if (pVar7 != (int)uVar24) {
                uVar14 = hashbrown::map::HashMap<K,V,S,A>::insert(&local_250,uVar24);
                if ((uVar14 & 1) == 0) {
                  std::sys::process::unix::common::Command::new
                            (&local_148,
                             "ps-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                             ,2);
                  auVar26 = ::core::fmt::num::imp::<impl_u32>::_fmt(uVar24,auStack_7a,10);
                  pcVar21 = auVar26._8_8_;
                  if ((long)pcVar21 < 0) {
                    uVar20 = 0;
                    pcVar21 = pcVar19;
LAB_1007c76f4:
                    /* WARNING: Subroutine does not return */
                    alloc::raw_vec::handle_error(uVar20,pcVar21);
                  }
                  if (pcVar21 == (code *)0x0) {
                    pcVar19 = (code *)0x1;
                  }
                  else {
                    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                    pcVar19 = (code *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(pcVar21,1);
                    if (pcVar19 == (code *)0x0) goto LAB_1007c76f4;
                  }
                  _memcpy(pcVar19,auVar26._0_8_,(size_t)pcVar21);
                  std::sys::process::unix::common::Command::arg
                            (&local_148,
                             "-p-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                             ,2);
                  std::sys::process::unix::common::Command::arg(&local_148,pcVar19,pcVar21);
                  std::sys::process::unix::common::Command::arg
                            (&local_148,
                             "-ocodexmate_lib::platform::processcodex-transition-fenceapplication id \"com.openai.codex\" is runningcodex_process_lifecycleforce_close_complete/ApplicationsApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                             ,2);
                  std::sys::process::unix::common::Command::arg(&local_148,&DAT_1013af2e0,8);
                  std::process::Command::output(&local_180,&local_148);
                  ppcVar16 = local_180;
                  if (local_180 == (char **)&DAT_8000000000000000) {
                    if (((ulong)ppcStack_178 & 3) == 1) {
                      puVar22 = (undefined8 *)((long)ppcStack_178 + -1);
                      uVar23 = *puVar22;
                      puVar25 = *(undefined8 **)((long)ppcStack_178 + 7);
                      if ((code *)*puVar25 != (code *)0x0) {
                        (*(code *)*puVar25)(uVar23);
                      }
                      if (puVar25[1] != 0) {
                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar23,puVar25[1],puVar25[2]);
                      }
                      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar22,0x18,8);
                    }
                    if (pcVar21 != (code *)0x0) {
                      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar19,pcVar21,1);
                    }
                    ::core::ptr::drop_in_place<std::process::Command>(&local_148);
                  }
                  else {
                    local_1a8 = local_170;
                    ppcStack_1b0 = ppcStack_178;
                    uStack_198 = uStack_160;
                    local_1a0 = local_168;
                    uStack_188 = uStack_150;
                    local_190 = local_158;
                    local_1b8 = local_180;
                    if (pcVar21 != (code *)0x0) {
                      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar19,pcVar21,1);
                    }
                    ::core::ptr::drop_in_place<std::process::Command>(&local_148);
                    ppcVar12 = ppcStack_1b0;
                    if ((int)uStack_188 == 0) {
                      alloc::string::String::from_utf8_lossy(&local_148,ppcStack_1b0,local_1a8);
                      pcVar21 = pcStack_140;
                      auVar26 = ::core::str::<impl_str>::trim_matches(pcStack_140,local_138);
                      puVar10 = auVar26._8_8_;
                      if ((long)puVar10 < 0) {
                        uVar20 = 0;
                        puVar10 = local_3b0;
LAB_1007c7708:
                        local_3b0 = puVar10;
                    /* WARNING: Subroutine does not return */
                        alloc::raw_vec::handle_error(uVar20,local_3b0);
                      }
                      if (puVar10 == (undefined1 *)0x0) {
                        local_3b0 = (undefined1 *)0x1;
                      }
                      else {
                        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                        local_3b0 = (undefined1 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(puVar10,1);
                        if (local_3b0 == (undefined1 *)0x0) goto LAB_1007c7708;
                      }
                      _memcpy(local_3b0,auVar26._0_8_,(size_t)puVar10);
                      if (((ulong)local_148 & 0x7fffffffffffffff) != 0) {
                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar21,local_148,1);
                      }
                      if (puVar10 != (undefined1 *)0x0) {
                        local_3a0 = local_3b0;
                        local_3a8 = puVar10;
                      }
                      puVar2 = &DAT_8000000000000000;
                      if (puVar10 != (undefined1 *)0x0) {
                        puVar2 = puVar10;
                      }
                      if (ppcVar16 != (char **)0x0) {
                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppcVar12,ppcVar16,1);
                      }
                      if (local_1a0 != 0) {
                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_198,local_1a0,1);
                      }
                      lVar18 = local_208;
                      if (puVar10 != (undefined1 *)0x0) {
                        if (local_3a8 == (undefined1 *)0x0) {
                          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_3a0,puVar2,1);
                          local_3a8 = (undefined1 *)0x0;
                        }
                        else {
                          if (local_208 == local_218) {
                            alloc::raw_vec::RawVec<T,A>::grow_one(&local_218);
                          }
                          piVar1 = (int *)(lStack_210 + lVar18 * 0x20);
                          *piVar1 = (int)uVar24;
                          *(undefined1 **)(piVar1 + 2) = puVar2;
                          *(undefined1 **)(piVar1 + 4) = local_3a0;
                          *(undefined1 **)(piVar1 + 6) = local_3a8;
                          local_208 = lVar18 + 1;
                        }
                      }
                    }
                    else {
                      if (ppcVar16 != (char **)0x0) {
                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppcStack_1b0,ppcVar16,1);
                      }
                      if (local_1a0 != 0) {
                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_198,local_1a0,1);
                      }
                    }
                  }
                }
              }
            }
          }
          else {
            bVar3 = *pbVar13;
            if (bVar3 == 0x2b) {
              pbVar13 = pbVar13 + 1;
            }
            uVar14 = lVar18 - (ulong)(bVar3 == 0x2b);
            if (uVar14 < 9) goto LAB_1007c72ac;
LAB_1007c7354:
            uVar24 = 0;
            while( true ) {
              if (uVar14 == 0) goto LAB_1007c72d0;
              if ((uVar24 * 10 & 0xffffffff00000000) != 0) break;
              uVar4 = *pbVar13 - 0x30;
              uVar15 = (uint)(uVar24 * 10);
              uVar24 = (ulong)(uVar4 + uVar15);
              if ((9 < uVar4) || (uVar14 = uVar14 - 1, pbVar13 = pbVar13 + 1, CARRY4(uVar4,uVar15))) break;
            }
          }
        }
LAB_1007c71cc:
      } while ((local_1c0 & 0x100) == 0);
      param_1[1] = lStack_210;
      *param_1 = local_218;
      param_1[2] = local_208;
      uVar14 = lStack_248 * 4 + 0xbU & 0xfffffffffffffff8;
      lVar18 = lStack_248 + uVar14 + 9;
      if (lStack_248 != 0 && lVar18 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((long)local_250 - uVar14,lVar18,8);
      }
      if ((local_268 != -0x8000000000000000) && (local_268 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_260,local_268,1);
      }
      if ((undefined8 *******)local_2a0 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcStack_298,local_2a0,1);
      }
      if (local_288 != (undefined8 *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_280,local_288,1);
      }
      ::core::ptr::drop_in_place<std::process::Command>(&local_370);
      if (ppcVar17 != (char **)0x0) {
        ppcVar16 = ppcVar6 + 1;
        do {
          if (ppcVar16[-1] != (char *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*ppcVar16,ppcVar16[-1],1);
          }
          ppcVar16 = ppcVar16 + 3;
          ppcVar17 = (char **)((long)ppcVar17 + -1);
        } while (ppcVar17 != (char **)0x0);
      }
    }
  }
  if (ppcVar11 != (char **)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppcVar6,(long)ppcVar11 * 0x18,8);
  }
  return;
}

