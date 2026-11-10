/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* new @ 100688540 requested 100688540 */

/* codexmate_lib::core::relay::manager::RelayManager::new */

void codexmate_lib::core::relay::manager::RelayManager::new(long *param_1,long param_2)

{
  undefined8 ****ppppuVar1;
  undefined8 ***pppuVar2;
  int iVar3;
  ulong uVar4;
  long lVar5;
  undefined8 ****ppppuVar6;
  void *pvVar7;
  undefined8 ****ppppuVar8;
  undefined8 *puVar9;
  undefined8 *****pppppuVar10;
  code *pcVar11;
  undefined8 ****ppppuVar12;
  undefined8 uVar13;
  undefined8 *****unaff_x26;
  undefined8 ****unaff_x27;
  undefined8 ***local_730;
  undefined8 ****ppppuStack_728;
  undefined8 ***pppuStack_720;
  undefined8 ****ppppuStack_718;
  undefined8 **local_710;
  undefined8 **ppuStack_708;
  undefined8 **ppuStack_700;
  undefined8 **ppuStack_6f8;
  undefined8 **local_6f0;
  undefined8 **ppuStack_6e8;
  undefined8 ****ppppuStack_6e0;
  undefined8 **ppuStack_6d8;
  undefined8 ****local_6d0;
  undefined8 ***pppuStack_6c8;
  undefined8 ****local_6c0;
  undefined8 ****local_5b0;
  undefined8 ***pppuStack_5a8;
  undefined8 ****local_5a0;
  undefined1 auStack_590 [384];
  undefined8 ***local_410;
  undefined8 ****local_408;
  undefined8 ***pppuStack_400;
  undefined8 ****local_3f8;
  undefined8 **ppuStack_3f0;
  undefined8 **local_3e8;
  undefined8 **ppuStack_3e0;
  undefined8 **local_3d8;
  undefined8 **ppuStack_3d0;
  undefined8 **local_3c8;
  undefined8 ****ppppuStack_3c0;
  undefined8 **local_3b8;
  long local_3b0;
  void *local_3a8;
  undefined8 local_3a0;
  undefined8 ***pppuStack_398;
  undefined8 ****ppppuStack_390;
  undefined8 ***local_388;
  undefined8 ****ppppuStack_380;
  undefined8 **local_378;
  undefined8 **ppuStack_370;
  undefined8 **local_368;
  undefined8 **ppuStack_360;
  undefined8 **local_358;
  undefined8 **ppuStack_350;
  undefined8 ****local_348;
  undefined8 **ppuStack_340;
  undefined8 *local_218;
  undefined8 ****local_210;
  undefined8 ***pppuStack_208;
  undefined8 ****local_200;
  undefined8 uStack_1f8;
  undefined8 ****local_1f0;
  undefined8 **ppuStack_1e8;
  undefined8 **ppuStack_1e0;
  undefined8 **local_1d8;
  undefined8 **ppuStack_1d0;
  undefined8 **local_1c8;
  undefined8 **ppuStack_1c0;
  undefined8 ****local_1b8;
  undefined8 ****ppppuStack_1b0;
  undefined8 ***pppuStack_1a8;
  undefined8 ****local_1a0;
  undefined2 local_198;
  undefined1 local_196;
  undefined8 local_190;
  undefined8 local_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined2 local_b0;
  undefined4 local_a8;
  undefined2 local_a4;
  undefined2 local_92;

  storage::load(&local_3a0,param_2 + 0x10);
  if (local_3a0 == (undefined8 *****)&DAT_8000000000000000) {
    ppuStack_708 = ppuStack_370;
    local_710 = local_378;
    ppuStack_6f8 = ppuStack_360;
    ppuStack_700 = local_368;
    ppuStack_6e8 = ppuStack_350;
    local_6f0 = local_358;
    ppuStack_6d8 = ppuStack_340;
    ppppuStack_6e0 = local_348;
    ppppuStack_728 = ppppuStack_390;
    local_730 = pppuStack_398;
    ppppuStack_718 = ppppuStack_380;
    pppuStack_720 = local_388;
    local_408 = &local_730;
    pppuStack_400 = (undefined8 ***)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_210,&DAT_1013a37c2,&local_408);
    pppuStack_5a8 = pppuStack_208;
    local_5b0 = local_210;
    local_5a0 = local_200;
    if (log::MAX_LOG_LEVEL_FILTER != 0) {
      local_408 = &local_5b0;
      pppuStack_400 = (undefined8 ***)<alloc::string::String_as_core::fmt::Display>::fmt;
      ppuStack_1d0 = (undefined8 **)0x23;
      local_1b8 = &local_408;
      ppuStack_1c0 = (undefined8 **)s__AiMaMi__relay_state__101528f97;
      local_210 = (undefined8 ****)0x0;
      pppuStack_208 = (undefined8 ***)&DAT_1013a31cb;
      uStack_1f8 = (undefined8 ****)0x0;
      local_200 = (undefined8 ****)0x23;
      local_1d8 = (undefined8 **)&DAT_1013a31cb;
      local_1f0 = (undefined8 ****)0x1015284cd;
      ppuStack_1e0 = (undefined8 **)0x1;
      ppuStack_1e8 = (undefined8 **)0x19;
      local_1c8 = (undefined8 **)0xc200000001;
      <log::__private_api::GlobalLogger_as_log::Log>::log(&local_218,&local_210);
    }
    local_a8 = 0x10;
    local_210 = (undefined8 *****)0x0;
    pppuStack_208 = (undefined8 ***)&SUB_00000008;
    uStack_1f8 = (undefined8 ****)0x0;
    local_200 = (undefined8 *****)0x0;
    local_1f0 = (undefined8 ****)&SUB_00000008;
    ppuStack_1e8 = (undefined8 ***)0x0;
    ppuStack_1e0 = (undefined8 ***)0x0;
    local_1d8 = (undefined8 ***)0x1;
    local_1c8 = (undefined8 ***)0x0;
    ppuStack_1d0 = (undefined8 ***)0x0;
    ppuStack_1c0 = (undefined8 ***)0x1;
    ppppuStack_1b0 = (undefined8 ****)0x8000000000000000;
    local_1b8 = (undefined8 *****)0x0;
    local_198 = 0;
    local_196 = 0;
    local_a4 = 0;
    local_92 = 0x100;
    local_190 = 0;
    local_188 = 8;
    uStack_178 = 0x8000000000000000;
    local_180 = 0;
    local_d0 = 0;
    local_d8 = 0;
    local_b0 = 7;
    <alloc::string::String_as_core::clone::Clone>::clone(&local_408,&local_5b0);
    pppuStack_1a8 = pppuStack_400;
    ppppuStack_1b0 = local_408;
    local_1a0 = local_3f8;
    _memcpy(auStack_590,&local_210,0x180);
    unaff_x26 = (undefined8 *****)local_5a0;
    unaff_x27 = (undefined8 ****)pppuStack_5a8;
    pppppuVar10 = (undefined8 *****)local_5b0;
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_730);
    _memcpy(&local_730,auStack_590,0x180);
    if (pppppuVar10 != (undefined8 *****)&DAT_8000000000000000) goto LAB_10068896c;
  }
  else {
    _memcpy(auStack_590,&local_3a0,0x180);
    _memcpy(&local_730,&local_3a0,0x180);
  }
  pppppuVar10 = (undefined8 *****)&DAT_8000000000000000;
  uVar4 = storage::hydrate_secrets(param_2 + 0x10,&local_730);
  if ((uVar4 & 1) != 0) {
    storage::save_with_publication(&local_210,param_2 + 0x10,&local_730);
    if (local_210 == (undefined8 ****)&DAT_0000000b) {
      atomic_write::Publication::acknowledge(pppuStack_208);
    }
    else {
      ppuStack_370 = ppuStack_1e0;
      local_378 = ppuStack_1e8;
      ppuStack_360 = ppuStack_1d0;
      local_368 = local_1d8;
      ppuStack_350 = ppuStack_1c0;
      local_358 = local_1c8;
      ppppuStack_390 = local_200;
      pppuStack_398 = pppuStack_208;
      ppppuStack_380 = local_1f0;
      local_388 = uStack_1f8;
      local_348 = local_1b8;
      local_3a0 = (undefined8 *****)local_210;
      if (1 < log::MAX_LOG_LEVEL_FILTER) {
        local_408 = (undefined8 ****)&local_3a0;
        pppuStack_400 = (undefined8 ***)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        ppuStack_1d0 = (undefined8 ***)0x23;
        local_1b8 = &local_408;
        ppuStack_1c0 = (undefined8 **)s___AiMaMi__failed_to_persist_hydr_101528fb0;
        local_210 = (undefined8 *****)0x0;
        pppuStack_208 = (undefined8 ***)&DAT_1013a31cb;
        uStack_1f8 = (undefined8 ****)0x0;
        local_200 = (undefined8 ****)&DAT_00000023;
        local_1d8 = (undefined8 **)&DAT_1013a31cb;
        local_1f0 = (undefined8 ****)0x1015284cd;
        ppuStack_1e0 = (undefined8 ***)0x2;
        ppuStack_1e8 = (undefined8 ***)0x19;
        local_1c8 = (undefined8 ***)0xce00000001;
        <log::__private_api::GlobalLogger_as_log::Log>::log(&local_218,&local_210);
      }
      ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_3a0);
    }
  }
  iVar3 = std::path::Path::is_file(*(undefined8 *)(param_2 + 0x270),*(undefined8 *)(param_2 + 0x278));
  if ((iVar3 != 0) && (keychain::revisions::cleanup_unreferenced(&local_408,param_2 + 0x10,&local_730), local_408 != (undefined8 ****)&DAT_8000000000000000)) {
    pppuStack_398 = pppuStack_400;
    local_3a0 = (undefined8 *****)local_408;
    ppppuStack_390 = local_3f8;
    if (1 < log::MAX_LOG_LEVEL_FILTER) {
      local_5b0 = (undefined8 ****)&local_3a0;
      pppuStack_5a8 = (undefined8 ***)<alloc::string::String_as_core::fmt::Display>::fmt;
      ppuStack_1d0 = (undefined8 ***)0x23;
      local_1b8 = &local_5b0;
      ppuStack_1c0 = (undefined8 **)s___AiMaMi__relay_credential_clean_101528fe1;
      local_210 = (undefined8 *****)0x0;
      pppuStack_208 = (undefined8 ***)&DAT_1013a31cb;
      uStack_1f8 = (undefined8 ****)0x0;
      local_200 = (undefined8 ****)&DAT_00000023;
      local_1d8 = (undefined8 **)&DAT_1013a31cb;
      local_1f0 = (undefined8 ****)0x1015284cd;
      ppuStack_1e0 = (undefined8 ***)0x2;
      ppuStack_1e8 = (undefined8 ***)0x19;
      local_1c8 = (undefined8 ***)0xd300000001;
      <log::__private_api::GlobalLogger_as_log::Log>::log(&local_218,&local_210);
    }
    if (local_3a0 != (undefined8 *****)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppuStack_398,local_3a0,1);
    }
  }
LAB_10068896c:
  tokio::runtime::builder::Builder::new_multi_thread(&local_210);
  lVar5 = tokio::runtime::builder::Builder::worker_threads(&local_210,2,&PTR__anon_1f4a05fd1dc96b9982818105670d4dce_576_101826f88);
  *(undefined2 *)(lVar5 + 0xd5) = 0x101;
  tokio::runtime::builder::Builder::thread_name(lVar5,&DAT_1013a3849,0xf);
  tokio::runtime::builder::Builder::build(&local_408);
  if ((undefined8 *****)local_408 == (undefined8 *****)0x2) {
    ppppuVar12 = (undefined8 ****)0x0;
    local_410 = (undefined8 ****)0x0;
    if (((ulong)pppuStack_400 & 3) == 1) {
      pcVar11 = (code *)((long)pppuStack_400 + -1);
      uVar13 = *(undefined8 *)pcVar11;
      puVar9 = *(undefined8 **)((long)pppuStack_400 + 7);
      if ((code *)*puVar9 != (code *)0x0) {
        (*(code *)*puVar9)(uVar13);
      }
      if (puVar9[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar13,puVar9[1],puVar9[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar11,0x18,8);
      ppppuVar12 = (undefined8 ****)0x0;
    }
  }
  else {
    local_358 = ppuStack_3d0;
    ppuStack_360 = local_3d8;
    local_348 = ppppuStack_3c0;
    ppuStack_350 = local_3c8;
    ppuStack_340 = local_3b8;
    local_378 = ppuStack_3f0;
    ppppuStack_380 = local_3f8;
    local_368 = ppuStack_3e0;
    ppuStack_370 = local_3e8;
    pppuStack_398 = (undefined8 ****)0x1;
    local_3a0 = (undefined8 *****)0x1;
    local_388 = pppuStack_400;
    ppppuStack_390 = local_408;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    ppppuVar12 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x68,8);
    if (ppppuVar12 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x68);
    }
    ppppuVar12[9] = (undefined8 ***)local_358;
    ppppuVar12[8] = (undefined8 ***)ppuStack_360;
    ppppuVar12[0xb] = local_348;
    ppppuVar12[10] = (undefined8 ***)ppuStack_350;
    ppppuVar12[0xc] = (undefined8 ***)ppuStack_340;
    ppppuVar12[1] = pppuStack_398;
    *ppppuVar12 = local_3a0;
    ppppuVar12[3] = local_388;
    ppppuVar12[2] = ppppuStack_390;
    ppppuVar12[5] = (undefined8 ***)local_378;
    ppppuVar12[4] = ppppuStack_380;
    ppppuVar12[7] = (undefined8 ***)local_368;
    ppppuVar12[6] = (undefined8 ***)ppuStack_370;
    local_410 = ppppuVar12;
  }
  ::core::ptr::drop_in_place<tokio::runtime::builder::Builder>(&local_210);
  if (ppppuVar12 == (undefined8 ****)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    ppppuVar6 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x2f,1);
    if (ppppuVar6 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x2f);
    }
    ppppuVar6[1] = (undefined8 ***)0x746e7572206f696b;
    *ppppuVar6 = (undefined8 ***)0x6f742079616c6572;
    ppppuVar6[3] = (undefined8 ***)0x3b64656c69616620;
    ppppuVar6[2] = (undefined8 ***)0x74696e6920656d69;
    *(undefined8 *)((long)ppppuVar6 + 0x27) = 0x64656c6261736964;
    *(undefined8 *)((long)ppppuVar6 + 0x1f) = 0x2079786f7270203b;
    local_3a0 = (undefined8 *****)&DAT_0000002f;
    ppppuStack_390 = (undefined8 ****)&DAT_0000002f;
    pppuStack_398 = ppppuVar6;
    if (log::MAX_LOG_LEVEL_FILTER != 0) {
      local_408 = (undefined8 ****)&local_3a0;
      pppuStack_400 = (undefined8 ***)<alloc::string::String_as_core::fmt::Display>::fmt;
      ppuStack_1d0 = (undefined8 ***)0x23;
      local_1b8 = &local_408;
      ppuStack_1c0 = (undefined8 **)s__AiMaMi__101529010;
      local_210 = (undefined8 *****)0x0;
      pppuStack_208 = (undefined8 ***)&DAT_1013a31cb;
      uStack_1f8 = (undefined8 ****)0x0;
      local_200 = (undefined8 ****)&DAT_00000023;
      local_1d8 = (undefined8 **)&DAT_1013a31cb;
      local_1f0 = (undefined8 ****)0x1015284cd;
      ppuStack_1e0 = (undefined8 ***)0x1;
      ppuStack_1e8 = (undefined8 ***)0x19;
      local_1c8 = (undefined8 ***)0xe000000001;
      <log::__private_api::GlobalLogger_as_log::Log>::log(&local_218,&local_210);
    }
    if (local_6d0 == (undefined8 ****)&DAT_8000000000000000) {
      pppuStack_6c8 = pppuStack_398;
      local_6d0 = local_3a0;
      local_6c0 = ppppuStack_390;
    }
    else if (local_3a0 != (undefined8 *****)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppuStack_398,local_3a0,1);
    }
  }
  local_3b0 = param_2;
  _memcpy((void *)((long)&local_3a0 + 7),&local_730,0x180);
  pppuStack_208 = (undefined8 ***)0x1;
  local_210 = (undefined8 ****)0x1;
  local_200 = (undefined8 ****)0x0;
  uStack_1f8 = (undefined8 ****)((ulong)uStack_1f8 & 0xffffffffffffff00);
  _memcpy((void *)((long)&uStack_1f8 + 1),&local_3a0,0x187);
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  pvVar7 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1a0,8);
  if (pvVar7 == (void *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error(8,0x1a0);
  }
  _memcpy(pvVar7,&local_210,0x1a0);
  local_3a8 = pvVar7;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x28,8);
  if (puVar9 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error(8,0x28);
  }
  puVar9[1] = 1;
  *puVar9 = 1;
  puVar9[2] = pppppuVar10;
  puVar9[3] = unaff_x27;
  puVar9[4] = unaff_x26;
  pppuStack_208 = (undefined8 ***)0x1;
  local_210 = (undefined8 ****)0x1;
  local_200 = (undefined8 ****)0x0;
  uStack_1f8 = (undefined8 ****)((ulong)uStack_1f8 & 0xffffffffffffff00);
  local_1f0 = (undefined8 ****)0x0;
  local_218 = puVar9;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  ppppuVar6 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x38,8);
  if (ppppuVar6 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error(8,0x38);
  }
  ppppuVar6[1] = pppuStack_208;
  *ppppuVar6 = local_210;
  ppppuVar6[3] = uStack_1f8;
  ppppuVar6[2] = local_200;
  ppppuVar6[5] = (undefined8 ***)ppuStack_1e8;
  ppppuVar6[4] = local_1f0;
  ppppuVar6[6] = (undefined8 ***)ppuStack_1e0;
  pppuStack_208 = (undefined8 ***)0x1;
  local_210 = (undefined8 ****)0x1;
  local_200 = (undefined8 ****)0x0;
  uStack_1f8 = (undefined8 ****)((ulong)uStack_1f8 & 0xffffffffffffff00);
  local_5b0 = ppppuVar6;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  ppppuVar8 = (undefined8 ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x20,8);
  if (ppppuVar8 == (undefined8 ****)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error(8,0x20);
  }
  ppppuVar8[1] = pppuStack_208;
  *ppppuVar8 = local_210;
  ppppuVar8[3] = uStack_1f8;
  ppppuVar8[2] = local_200;
  local_408 = ppppuVar8;
  if (<codexmate_lib::core::relay::codex_mutation::CodexMutationGate_as_core::default::Default>::default::GATE != 0) {
    std::sync::once_lock::OnceLock<T>::initialize(&<codexmate_lib::core::relay::codex_mutation::CodexMutationGate_as_core::default::Default>::default::GATE);
  }
  pppuVar2 = *DAT_1018ec1b0;
  *DAT_1018ec1b0 = (undefined8 ***)((long)pppuVar2 + 1);
  ppppuVar1 = DAT_1018ec1b0;
  if ((long)pppuVar2 < 0) {
                    /* WARNING: Does not return */
    pcVar11 = (code *)SoftwareBreakpoint(1,0x100688da4);
    (*pcVar11)();
  }
  local_3a0 = (undefined8 *****)DAT_1018ec1b0;
  local_210 = ppppuVar12;
  lVar5 = breaker::new_store();
  *param_1 = param_2;
  param_1[1] = (long)pvVar7;
  param_1[2] = (long)puVar9;
  param_1[3] = (long)ppppuVar6;
  param_1[4] = (long)ppppuVar8;
  param_1[5] = (long)ppppuVar1;
  param_1[6] = lVar5;
  param_1[7] = (long)local_210;
  return;
}
