===BEGIN set_image_compat @ 1007b4204=== (GhidraScript)  

/* codexmate_lib::commands::system::set_image_compat */

ulong codexmate_lib::commands::system::set_image_compat(long *param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  long lVar2;
  void *pvVar3;
  size_t sVar4;
  long *plVar5;
  undefined **ppuVar6;
  undefined1 uVar7;
  int iVar8;
  uint uVar9;
  void *pvVar10;
  ulong uVar11;
  undefined8 uVar12;
  long **local_450;
  undefined8 *puStack_448;
  ulong uStack_440;
  long lStack_438;
  long local_430;
  long lStack_428;
  long lStack_420;
  long local_418;
  long local_410;
  long lStack_408;
  undefined8 uStack_400;
  undefined8 uStack_3f8;
  undefined1 local_161;
  long **local_160;
  undefined **ppuStack_158;
  undefined1 *local_150;
  long lStack_148;
  long local_140;
  long lStack_138;
  long lStack_130;
  long lStack_128;
  long local_120;
  long lStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  long **local_100;
  undefined8 *puStack_f8;
  ulong local_f0;
  long lStack_e8;
  long local_e0;
  long lStack_d8;
  long lStack_d0;
  long lStack_c8;
  long local_c0;
  long lStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  long local_98;
  void *pvStack_90;
  size_t local_88;
  long *local_80;
  undefined **local_78;
  undefined8 local_70;
  undefined1 uStack_61;
  
  core::relay::codex_mutation::CodexMutationGate::lock
            (&local_100,param_2,"updating image compatibilityosVersionupdate image compatibility[features]image_generation = false",0x1c);
  uVar11 = local_f0;
  puVar1 = puStack_f8;
  if (local_100 != (long **)0xb) {
    lStack_428 = lStack_d8;
    local_430 = local_e0;
    local_418 = lStack_c8;
    lStack_420 = lStack_d0;
    lStack_408 = lStack_b8;
    local_410 = local_c0;
    uStack_3f8 = uStack_a8;
    uStack_400 = uStack_b0;
    puStack_448 = puStack_f8;
    local_450 = local_100;
    lStack_438 = lStack_e8;
    uStack_440 = local_f0;
    local_80 = (long *)0x0;
    local_78 = (undefined **)0x1;
    local_70 = (undefined1 *)0x0;
    local_150 = &DAT_60000020;
    local_160 = &local_80;
    ppuStack_158 = &_anon_6408540cdffbe318d49f52812e99783f_1640;
    iVar8 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_450,&local_160);
    ppuVar6 = local_78;
    plVar5 = local_80;
    if (iVar8 == 0) {
      uVar7 = (undefined1)local_70;
      *(undefined4 *)((long)param_1 + 0x19) = local_70._1_4_;
      *(undefined4 *)((long)param_1 + 0x1c) = local_70._4_4_;
      uVar11 = ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_100);
      param_1[1] = (long)plVar5;
      param_1[2] = (long)ppuVar6;
      *(undefined1 *)(param_1 + 3) = uVar7;
      *param_1 = -0x8000000000000000;
      return uVar11;
    }
                    /* WARNING: Subroutine does not return */
    ::core::result::unwrap_failed
              ("a Display implementation returned an error unexpectedlycalled `Result::unwrap()` on an `Err` value",0x37,&uStack_61,
               &_anon_6408540cdffbe318d49f52812e99783f_1652,&_anon_6408540cdffbe318d49f52812e99783f_1643);
  }
  platform::paths::CodexPaths::resolve_codex_home(&local_100);
  platform::paths::CodexPaths::from_home(&local_450,&local_100);
  local_161 = (undefined1)param_3;
  core::codex_config::update_text(&local_160,local_418,local_410,"update image compatibility[features]image_generation = false",0x1a,&local_161);
  if (local_160 != (long **)0xb) {
    lStack_d8 = lStack_138;
    local_e0 = local_140;
    lStack_c8 = lStack_128;
    lStack_d0 = lStack_130;
    lStack_b8 = lStack_118;
    local_c0 = local_120;
    uStack_a8 = uStack_108;
    uStack_b0 = uStack_110;
    puStack_f8 = ppuStack_158;
    local_100 = local_160;
    lStack_e8 = lStack_148;
    local_f0 = (ulong)local_150;
    local_98 = 0;
    pvStack_90 = (void *)0x1;
    local_70 = &DAT_60000020;
    local_80 = &local_98;
    local_88 = 0;
    local_78 = &_anon_6408540cdffbe318d49f52812e99783f_1640;
    iVar8 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_100,&local_80);
    sVar4 = local_88;
    pvVar3 = pvStack_90;
    lVar2 = local_98;
    if (iVar8 != 0) {
                    /* WARNING: Subroutine does not return */
      ::core::result::unwrap_failed
                ("a Display implementation returned an error unexpectedlycalled `Result::unwrap()` on an `Err` value",0x37,&uStack_61,
                 &_anon_6408540cdffbe318d49f52812e99783f_1652,&_anon_6408540cdffbe318d49f52812e99783f_1643);
    }
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_160);
    if (lVar2 != -0x8000000000000000) {
      if ((long)sVar4 < 0) {
        uVar12 = 0;
LAB_1007b43fc:
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(uVar12,sVar4);
      }
      if (sVar4 == 0) {
        pvVar10 = (void *)0x1;
        _memcpy((void *)0x1,pvVar3,0);
      }
      else {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar12 = 1;
        pvVar10 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar4,1);
        if (pvVar10 == (void *)0x0) goto LAB_1007b43fc;
        _memcpy(pvVar10,pvVar3,sVar4);
      }
      if (lVar2 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar3,lVar2,1);
      }
      param_1[2] = (long)pvVar10;
      param_1[3] = sVar4;
      *param_1 = -0x8000000000000000;
      param_1[1] = sVar4;
      ::core::ptr::drop_in_place<codexmate_lib::platform::paths::CodexPaths>(&local_450);
      goto joined_r0x0001007b432c;
    }
  }
  core::models::CoreEnvelope<T>::ok(&local_100,param_3);
  param_1[5] = lStack_d8;
  param_1[4] = local_e0;
  param_1[7] = lStack_c8;
  param_1[6] = lStack_d0;
  param_1[9] = lStack_b8;
  param_1[8] = local_c0;
  param_1[1] = (long)puStack_f8;
  *param_1 = (long)local_100;
  param_1[3] = lStack_e8;
  param_1[2] = local_f0;
  ::core::ptr::drop_in_place<codexmate_lib::platform::paths::CodexPaths>(&local_450);
joined_r0x0001007b432c:
  if ((((uVar11 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
     (uVar11 = std::panicking::panic_count::is_zero_slow_path(), (uVar11 & 1) == 0)) {
    *(undefined1 *)(puVar1 + 1) = 1;
  }
  uVar9 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*puVar1);
  return (ulong)uVar9;
}

 (GhidraScript)  
===END=== (GhidraScript)  
