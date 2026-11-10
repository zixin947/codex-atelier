/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* transition_paths @ 10067d054 requested 10067d054 */

/* codexmate_lib::core::relay::manager::RelayManager::transition_paths */

void codexmate_lib::core::relay::manager::RelayManager::transition_paths(undefined8 *param_1,long *param_2)

{
  long lVar1;
  undefined8 *local_6d8;
  undefined8 uStack_6d0;
  undefined8 uStack_6c8;
  undefined8 uStack_6c0;
  undefined8 uStack_6b8;
  undefined8 local_6b0;
  undefined8 uStack_6a8;
  undefined8 uStack_6a0;
  undefined8 uStack_698;
  undefined8 local_690;
  undefined8 uStack_688;
  undefined8 uStack_680;
  undefined8 uStack_678;
  undefined1 auStack_670 [640];
  long local_3f0;
  undefined8 local_3e8;
  undefined8 local_3e0;
  byte local_3d8;
  undefined8 local_3d0;
  undefined8 uStack_3c8;
  undefined8 uStack_3c0;
  undefined8 uStack_3b8;
  undefined8 local_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 local_390;
  undefined8 uStack_388;
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined **local_370;
  undefined8 *puStack_368;
  undefined8 *puStack_360;
  code *local_358;
  undefined8 uStack_350;
  undefined8 local_348;
  undefined8 uStack_340;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined8 local_328;
  undefined8 uStack_320;
  undefined8 local_318;
  undefined8 uStack_310;
  undefined1 auStack_308 [672];
  undefined1 auStack_68 [24];
  undefined *local_50;
  undefined8 uStack_48;

  lVar1 = *param_2;
  router_transition::ensure_recovery_records_allowed(&local_370,lVar1 + 0x10);
  if (local_370 == (undefined **)0xb) {
    snapshot(&local_370,param_2);
    ensure_pending_commit_matches(&local_6d8,lVar1,&local_370);
    if (local_6d8 != (undefined8 *)0xb) {
      param_1[6] = local_6b0;
      param_1[5] = uStack_6b8;
      param_1[8] = uStack_6a0;
      param_1[7] = uStack_6a8;
      param_1[10] = local_690;
      param_1[9] = uStack_698;
      param_1[0xc] = uStack_680;
      param_1[0xb] = uStack_688;
      param_1[2] = uStack_6d0;
      param_1[1] = local_6d8;
      param_1[4] = uStack_6c0;
      param_1[3] = uStack_6c8;
      *param_1 = 0x8000000000000000;
      ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayState>(&local_370);
      return;
    }
    ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayState>(&local_370);
    codex_runtime::CodexRuntimeContext::resolve_inner(&local_370,lVar1 + 0x10,1);
    uStack_3a8 = uStack_340;
    local_3b0 = local_348;
    uStack_398 = uStack_330;
    uStack_3a0 = local_338;
    uStack_388 = uStack_320;
    local_390 = local_328;
    uStack_378 = uStack_310;
    uStack_380 = local_318;
    uStack_3c8 = puStack_360;
    local_3d0 = puStack_368;
    uStack_3b8 = uStack_350;
    uStack_3c0 = local_358;
    if (local_370 != (undefined **)0x8000000000000000) {
      _memcpy(auStack_670,auStack_308,0x2a0);
      uStack_6a8 = uStack_3a8;
      local_6b0 = local_3b0;
      uStack_698 = uStack_398;
      uStack_6a0 = uStack_3a0;
      uStack_688 = uStack_388;
      local_690 = local_390;
      uStack_678 = uStack_378;
      uStack_680 = uStack_380;
      uStack_6c8 = uStack_3c8;
      uStack_6d0 = local_3d0;
      uStack_6b8 = uStack_3b8;
      uStack_6c0 = uStack_3c0;
      local_6d8 = local_370;
      uStack_48 = *(undefined8 *)(&DAT_1013a7ae8 + (ulong)local_3d8 * 8);
      local_50 = (&PTR_DAT_101829638)[local_3d8];
      local_3d0 = local_3e8;
      uStack_3c8 = local_3e0;
      local_370 = &local_50;
      puStack_368 = &<&T_as_core::fmt::Display>::fmt;
      puStack_360 = &local_3d0;
      local_358 = <std::path::Display_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(auStack_68,s_source__path__1015284e7,&local_370);
      platform::debug_log::app_event(&DAT_1013a31a9,0x11,&DAT_1013a31ba,0x11,auStack_68);
      <codexmate_lib::platform::paths::CodexPaths_as_core::clone::Clone>::clone(&local_370,&local_6d8);
      _memcpy(param_1,&local_370,0x2e8);
      ::core::ptr::drop_in_place<codexmate_lib::platform::paths::CodexPaths>(&local_6d8);
      if (local_3f0 == 0) {
        return;
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_3e8,local_3f0,1);
      return;
    }
    param_1[6] = uStack_340;
    param_1[5] = local_348;
    param_1[8] = uStack_330;
    param_1[7] = local_338;
    param_1[10] = uStack_320;
    param_1[9] = local_328;
    param_1[0xc] = uStack_310;
    param_1[0xb] = local_318;
    param_1[2] = puStack_360;
    param_1[1] = puStack_368;
    param_1[4] = uStack_350;
    param_1[3] = local_358;
  }
  else {
    param_1[6] = local_348;
    param_1[5] = uStack_350;
    param_1[8] = local_338;
    param_1[7] = uStack_340;
    param_1[10] = local_328;
    param_1[9] = uStack_330;
    param_1[0xc] = local_318;
    param_1[0xb] = uStack_320;
    param_1[2] = puStack_368;
    param_1[1] = local_370;
    param_1[4] = local_358;
    param_1[3] = puStack_360;
  }
  *param_1 = 0x8000000000000000;
  return;
}
