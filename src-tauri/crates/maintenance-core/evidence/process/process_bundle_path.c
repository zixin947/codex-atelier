/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* codex_app_bundle_path @ 1007bf9a4 requested 1007bf9a4 */

/* codexmate_lib::platform::process::codex_app_bundle_path */

void codexmate_lib::platform::process::codex_app_bundle_path(ulong *param_1)

{
  long lVar1;
  long *plVar2;
  char *pcVar3;
  long *plVar4;
  ulong uVar5;
  ulong uVar6;
  undefined8 *puVar7;
  long lVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  long *plVar11;
  long lVar12;
  long lVar13;
  long local_1d8;
  long *local_1d0;
  long local_1c8;
  ulong local_1c0;
  ulong uStack_1b8;
  ulong local_1b0;
  long local_1a8;
  undefined8 local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  ulong local_180;
  ulong uStack_178;
  ulong local_170;
  long lStack_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  long *local_148;
  long *plStack_140;
  ulong local_138;
  long *local_130;
  undefined8 local_128;
  ulong uStack_120;
  undefined8 local_118;
  undefined1 local_110;
  undefined2 local_108;
  long local_80;
  long *local_78;
  ulong local_70;
  
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  plVar2 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
  if (plVar2 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error(8,0x18);
  }
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  pcVar3 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xd,1);
  if (pcVar3 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0xd);
  }
  builtin_strncpy(pcVar3,"/Applications",0xd);
  *plVar2 = 0xd;
  plVar2[1] = (long)pcVar3;
  plVar2[2] = 0xd;
  local_1d8 = 1;
  local_1c8 = 1;
  local_1d0 = plVar2;
  dirs::home_dir(&local_180);
  uVar5 = uStack_178;
  uVar6 = local_180;
  if (local_180 != 0x8000000000000000) {
    std::path::Path::_join
              (&local_148,uStack_178,local_170,
               "ApplicationsChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
               ,0xc);
    lVar12 = local_1c8;
    if (local_1c8 == local_1d8) {
      alloc::raw_vec::RawVec<T,A>::grow_one(&local_1d8);
    }
    plVar2 = local_1d0 + lVar12 * 3;
    plVar2[1] = (long)plStack_140;
    *plVar2 = (long)local_148;
    plVar2[2] = local_138;
    local_1c8 = lVar12 + 1;
    if (uVar6 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar5,uVar6,1);
    }
  }
  plVar4 = local_1d0;
  uVar6 = local_1d8;
  lVar12 = local_1c8 * 0x18;
  plVar11 = local_1d0 + local_1c8 * 3;
  local_148 = local_1d0;
  plStack_140 = local_1d0;
  local_138 = local_1d8;
  plVar2 = local_1d0;
  local_130 = plVar11;
  if (local_1c8 != 0) {
    lVar13 = 0;
LAB_1007bfaf0:
    plVar2 = (long *)((long)plVar4 + lVar13);
    lVar8 = *plVar2;
    if (lVar8 != -0x8000000000000000) {
      lVar1 = plVar2[1];
      std::path::Path::_join
                (&local_1c0,lVar1,plVar2[2],
                 "ChatGPT.appCFBundleIdentifierCFBundleExecutableContents/MacOScodex_processlaunch_readyCodex/ChatGPT app bundle not foundlaunch_timeout/usr/bin/openif application id \"com.openai.codex\" is running then\n    tell application id \"com.openai.codex\" to quit\nend if/codex framework.framework//contents/resources/native/bare-modifier-monitor"
                 ,0xb);
      uVar5 = uStack_1b8;
      codex_macos_bundle_main_executable(&local_180,uStack_1b8,local_1b0);
      if (local_180 == 0x8000000000000000) goto code_r0x0001007bfb34;
      if (local_180 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_178,local_180,1);
      }
      param_1[1] = uStack_1b8;
      *param_1 = local_1c0;
      param_1[2] = local_1b0;
      if (lVar8 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar1,lVar8,1);
      }
      if (lVar12 + -0x18 != lVar13) {
        puVar7 = (undefined8 *)((long)plVar4 + lVar13 + 0x20);
        uVar5 = ((lVar12 - lVar13) - 0x18U) / 0x18;
        do {
          if (puVar7[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar7,puVar7[-1],1);
          }
          puVar7 = puVar7 + 3;
          uVar5 = uVar5 - 1;
        } while (uVar5 != 0);
      }
      if (uVar6 == 0) {
        return;
      }
      goto LAB_1007bfe1c;
    }
    plVar2 = plVar2 + 3;
  }
LAB_1007bfc14:
  plStack_140 = plVar2;
  if ((long)plVar11 - (long)plVar2 != 0) {
    uVar5 = (ulong)((long)plVar11 - (long)plVar2) / 0x18;
    plVar2 = plVar2 + 1;
    do {
      if (plVar2[-1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*plVar2,plVar2[-1],1);
      }
      plVar2 = plVar2 + 3;
      uVar5 = uVar5 - 1;
    } while (uVar5 != 0);
  }
  if (uVar6 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar4,uVar6 * 0x18,8);
  }
  std::sys::process::unix::common::Command::new
            (&local_148,
             "/usr/bin/mdfindkMDItemCFBundleIdentifier == \'com.openai.codex\'Codex.app/Codex Helper-t--Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codexContents/Resources/codexa tuple of size 2a sequencesignala tuple of size 3"
             ,0xf);
  std::sys::process::unix::common::Command::arg
            (&local_148,
             "kMDItemCFBundleIdentifier == \'com.openai.codex\'Codex.app/Codex Helper-t--Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codexContents/Resources/codexa tuple of size 2a sequencesignala tuple of size 3"
             ,0x2f);
  std::process::Command::output(&local_180,&local_148);
  if (local_180 == 0x8000000000000000) {
    if ((uStack_178 & 3) == 1) {
      puVar7 = (undefined8 *)(uStack_178 - 1);
      uVar9 = *puVar7;
      puVar10 = *(undefined8 **)(uStack_178 + 7);
      if ((code *)*puVar10 != (code *)0x0) {
        (*(code *)*puVar10)(uVar9);
      }
      if (puVar10[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar9,puVar10[1],puVar10[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,0x18,8);
    }
    ::core::ptr::drop_in_place<std::process::Command>(&local_148);
  }
  else {
    uStack_1b8 = uStack_178;
    local_1c0 = local_180;
    local_1a8 = lStack_168;
    local_1b0 = local_170;
    uStack_198 = uStack_158;
    local_1a0 = local_160;
    local_190 = local_150;
    ::core::ptr::drop_in_place<std::process::Command>(&local_148);
    uVar6 = uStack_1b8;
    if ((int)local_190 == 0) {
      alloc::string::String::from_utf8_lossy(&local_180,uStack_1b8,local_1b0);
      uVar5 = uStack_178;
      local_148 = (long *)0x0;
      plStack_140 = (long *)local_170;
      local_138 = uStack_178;
      local_130 = (long *)local_170;
      local_128 = 0;
      uStack_120 = local_170;
      local_118 = 0xa0000000a;
      local_110 = 1;
      local_108 = 0;
      <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_80,&local_148);
      if ((local_180 & 0x7fffffffffffffff) != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar5,local_180,1);
      }
      plVar2 = local_78;
      if (1 < local_70) {
        if (local_70 < 0x15) {
          lVar12 = local_70 * 0x18 + -0x18;
          plVar11 = local_78;
          do {
            plVar11 = plVar11 + 3;
            ::core::slice::sort::shared::smallsort::insert_tail(plVar2,plVar11);
            lVar12 = lVar12 + -0x18;
          } while (lVar12 != 0);
        }
        else {
          ::core::slice::sort::stable::driftsort_main(local_78,local_70,&local_180);
        }
      }
      alloc::vec::Vec<T,A>::dedup_by(&local_80);
      if (local_1c0 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar6,local_1c0,1);
      }
      if (local_1a8 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1a0,local_1a8,1);
      }
      goto LAB_1007bfd68;
    }
    if (local_1c0 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_1b8,local_1c0,1);
    }
    if (local_1a8 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1a0,local_1a8,1);
    }
  }
  local_80 = 0;
  local_70 = 0;
  local_78 = (long *)&SUB_00000008;
LAB_1007bfd68:
  local_130 = local_78 + local_70 * 3;
  local_148 = local_78;
  plStack_140 = local_78;
  local_138 = local_80;
  <alloc::vec::into_iter::IntoIter<T,A>as_core::iter::traits::iterator::Iterator>::try_fold(&local_180,&local_148);
  if (local_180 == 0x8000000000000000) {
    *param_1 = 0x8000000000000000;
  }
  else {
    param_1[1] = uStack_178;
    *param_1 = local_180;
    param_1[2] = local_170;
  }
  if ((long)local_130 - (long)plStack_140 != 0) {
    uVar6 = (ulong)((long)local_130 - (long)plStack_140) / 0x18;
    plVar2 = plStack_140 + 1;
    do {
      if (plVar2[-1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*plVar2,plVar2[-1],1);
      }
      plVar2 = plVar2 + 3;
      uVar6 = uVar6 - 1;
    } while (uVar6 != 0);
  }
  plVar4 = local_148;
  uVar6 = local_138;
  if (local_138 != 0) {
LAB_1007bfe1c:
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar4,uVar6 * 0x18,8);
  }
  return;
code_r0x0001007bfb34:
  if (local_1c0 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar5,local_1c0,1);
  }
  if (lVar8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar1,lVar8,1);
  }
  lVar13 = lVar13 + 0x18;
  plVar2 = plVar11;
  if (lVar12 - lVar13 == 0) goto LAB_1007bfc14;
  goto LAB_1007bfaf0;
}

