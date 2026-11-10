/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* persist_with_commit @ 10067d67c requested 10067d67c */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::manager::RelayManager::persist_with_commit */

void codexmate_lib::core::relay::manager::RelayManager::persist_with_commit(long *param_1,long *param_2,void *param_3,byte *param_4)

{
  undefined8 *puVar1;
  char cVar2;
  byte bVar3;
  void *pvVar4;
  void *pvVar5;
  void *pvVar6;
  int iVar7;
  uint uVar8;
  undefined8 *puVar9;
  ulong uVar10;
  undefined8 uVar11;
  long lVar12;
  long lVar13;
  undefined8 local_300;
  long lStack_2f8;
  long lStack_2f0;
  long lStack_2e8;
  long local_2e0;
  long lStack_2d8;
  undefined8 *local_2d0;
  void *pvStack_2c8;
  size_t local_2c0;
  long lStack_2b8;
  long local_2b0;
  long lStack_2a8;
  long local_2a0;
  long local_298;
  long local_290;
  long lStack_288;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 *local_1c8;
  void *pvStack_1c0;
  size_t sStack_1b8;
  long lStack_1b0;
  long local_1a8;
  long lStack_1a0;
  char local_194;
  byte local_193;
  undefined7 uStack_192;
  undefined1 uStack_18b;
  undefined7 uStack_18a;
  byte bStack_183;
  char local_182;
  undefined8 local_180;
  undefined8 uStack_178;
  byte local_170;
  void *local_168;
  undefined8 *local_160;
  void *pvStack_158;
  size_t local_150;
  long lStack_148;
  long local_140;
  long lStack_138;
  long lStack_130;
  long lStack_128;
  long local_120;
  long lStack_118;
  long lStack_110;
  long lStack_108;
  undefined8 *local_100;
  void *pvStack_f8;
  size_t local_f0;
  long lStack_e8;
  long local_e0;
  long lStack_d8;
  long local_d0;
  long lStack_c8;
  long local_c0;
  long lStack_b8;
  long local_b0;
  long lStack_a8;
  undefined8 **local_a0;
  code *pcStack_98;
  undefined8 *local_90;
  undefined8 uStack_88;
  undefined8 *local_80;
  undefined8 uStack_78;
  undefined **local_70;

  if (*(long *)(param_2[2] + 0x10) != -0x8000000000000000) {
    <alloc::string::String_as_core::clone::Clone>::clone(&lStack_2f8);
    local_300 = 9;
    param_1[5] = lStack_2d8;
    param_1[4] = local_2e0;
    param_1[7] = (long)pvStack_2c8;
    param_1[6] = (long)local_2d0;
    param_1[9] = lStack_2b8;
    param_1[8] = local_2c0;
    param_1[0xb] = lStack_2a8;
    param_1[10] = local_2b0;
    param_1[1] = lStack_2f8;
    *param_1 = 9;
    param_1[3] = lStack_2e8;
    param_1[2] = lStack_2f0;
    ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayState>(param_3);
    return;
  }
  _memcpy(&local_300,param_3,0x180);
  codex_catalog::validate_legacy_no_account_aliases(&local_100,local_278,uStack_270);
  if (local_100 != (undefined8 *)0xb) {
    param_1[5] = lStack_d8;
    param_1[4] = local_e0;
    param_1[7] = lStack_c8;
    param_1[6] = local_d0;
    param_1[9] = lStack_b8;
    param_1[8] = local_c0;
    param_1[0xb] = lStack_a8;
    param_1[10] = local_b0;
    param_1[1] = (long)pvStack_f8;
    *param_1 = (long)local_100;
    param_1[3] = lStack_e8;
    param_1[2] = local_f0;
    goto LAB_10067dbb4;
  }
  compose_proxy_status(&local_100,param_2[3]);
  if (local_2d0 != (undefined8 *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvStack_2c8,local_2d0,1);
  }
  if (lStack_2b8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2b0,lStack_2b8,1);
  }
  if ((local_2a0 != -0x8000000000000000) && (local_2a0 != 0)) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_298,local_2a0,1);
  }
  lStack_2a8 = lStack_d8;
  local_2b0 = local_e0;
  local_298 = lStack_c8;
  local_2a0 = local_d0;
  lStack_288 = lStack_b8;
  local_290 = local_c0;
  pvStack_2c8 = pvStack_f8;
  local_2d0 = local_100;
  lStack_2b8 = lStack_e8;
  local_2c0 = local_f0;
  lVar13 = param_2[1];
  puVar1 = (undefined8 *)(lVar13 + 0x10);
  if (*(long *)(lVar13 + 0x10) == 0) {
    std::sys::sync::once_box::OnceBox<T>::initialize(puVar1);
  }
  std::sys::pal::unix::sync::mutex::Mutex::lock();
  if ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) == 0) {
    uVar8 = 0;
    cVar2 = *(char *)(lVar13 + 0x18);
  }
  else {
    uVar8 = std::panicking::panic_count::is_zero_slow_path();
    uVar8 = uVar8 ^ 1;
    cVar2 = *(char *)(lVar13 + 0x18);
  }
  if (cVar2 != '\0') {
    uVar10 = (ulong)pvStack_f8 >> 8;
    pvStack_f8 = (void *)CONCAT71((int7)uVar10,(char)uVar8);
    local_100 = puVar1;
                    /* WARNING: Subroutine does not return */
    ::core::result::unwrap_failed
              ("relay state poisonednative catalog is not activeinvalid client_versioncatalog configuration changed; reload clientclient_versionrelay.key_unavailableparam error_type= error_code= error_param="
               ,0x14,&local_100,&_anon_1f4a05fd1dc96b9982818105670d4dce_959,&PTR__anon_1f4a05fd1dc96b9982818105670d4dce_576_101826e08);
  }
  lVar12 = *param_2;
  ensure_pending_commit_matches(&local_100,lVar12,lVar13 + 0x20);
  if (local_100 == (undefined8 *)0xb) {
    bVar3 = *param_4;
    if (bVar3 == 1) {
      router_transition::ensure_recovery_records_allowed(&local_100,lVar12 + 0x10);
      if (local_100 != (undefined8 *)0xb) goto LAB_10067d7d4;
    }
    else if ((local_194 != *(char *)(lVar13 + 0x18c)) || ((local_194 != '\0' && (local_182 != *(char *)(lVar13 + 0x19e))))) {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar11 = 0x47;
      puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x47,1);
      if (puVar9 == (undefined8 *)0x0) goto LAB_10067dc68;
      puVar9[5] = 0x7571657220736567;
      puVar9[4] = 0x6e61686320657475;
      puVar9[7] = 0x6e6f697469736e61;
      puVar9[6] = 0x7274206120657269;
      *(undefined8 *)((long)puVar9 + 0x3f) = 0x74696d6d6f63206e;
      puVar9[1] = 0x4e4f435f45544154;
      *puVar9 = 0x535f524554554f52;
      puVar9[3] = 0x6f72206576697463;
      puVar9[2] = 0x61203a5443494c46;
      param_1[1] = 0x47;
      *param_1 = 10;
      param_1[2] = (long)puVar9;
      param_1[3] = 0x47;
      goto joined_r0x00010067d984;
    }
    if ((local_193 & 1) == 0) {
      if (*(byte *)(lVar13 + 0x18d) != 0) goto LAB_10067d8b4;
LAB_10067d990:
      uStack_178 = *(undefined8 *)(lVar13 + 0x195);
      local_180 = *(undefined8 *)(lVar13 + 0x18d);
      local_170 = *(byte *)(lVar13 + 0x19d);
      if ((bVar3 & 1) != 0) goto LAB_10067d9a4;
    }
    else {
      if (((*(byte *)(lVar13 + 0x18d) & 1) != 0) &&
         (CONCAT17(bStack_183,uStack_18a) == *(long *)(lVar13 + 0x196) && CONCAT17(uStack_18b,uStack_192) == *(long *)(lVar13 + 0x18e))) goto LAB_10067d990;
LAB_10067d8b4:
      if (((bVar3 & 1) != 0) || ((local_194 != *(char *)(lVar13 + 0x18c) || (param_4 = (byte *)(lVar13 + 0x18d), local_182 != *(char *)(lVar13 + 0x19e))))) {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        uVar11 = 0x58;
        puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x58,1);
        if (puVar9 == (undefined8 *)0x0) {
LAB_10067dc68:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,uVar11);
        }
        puVar9[5] = 0x2073696874207265;
        puVar9[4] = 0x746661206465676e;
        puVar9[7] = 0x203b746f68737061;
        puVar9[6] = 0x6e73206574617473;
        puVar9[9] = 0x6572727563206d6f;
        puVar9[8] = 0x7266207972746572;
        puVar9[10] = 0x657461747320746e;
        puVar9[1] = 0x4e4f435f45544154;
        *puVar9 = 0x535f524554554f52;
        puVar9[3] = 0x616863206574756f;
        puVar9[2] = 0x72203a5443494c46;
        param_1[1] = 0x58;
        *param_1 = 10;
        param_1[2] = (long)puVar9;
        param_1[3] = 0x58;
        goto joined_r0x00010067d984;
      }
LAB_10067d9a4:
      uStack_178 = *(undefined8 *)(param_4 + 8);
      local_180 = *(undefined8 *)param_4;
      local_170 = param_4[0x10];
    }
    uStack_18b = (undefined1)uStack_178;
    uStack_18a = (undefined7)((ulong)uStack_178 >> 8);
    local_193 = (byte)local_180;
    uStack_192 = (undefined7)((ulong)local_180 >> 8);
    bStack_183 = local_170;
    storage::save_with_publication(&local_100,lVar12 + 0x10,&local_300);
    pvVar5 = pvStack_f8;
    if (local_100 == (undefined8 *)0xb) {
      local_168 = pvStack_f8;
      if ((pvStack_f8 != (void *)0x0) && (storage::verify_router_commit(&local_160,lVar12 + 0x10,&local_300), local_160 != (undefined8 *)0xb)) {
        lStack_d8 = lStack_138;
        local_e0 = local_140;
        lStack_c8 = lStack_128;
        local_d0 = lStack_130;
        lStack_b8 = lStack_118;
        local_c0 = local_120;
        lStack_a8 = lStack_108;
        local_b0 = lStack_110;
        pvStack_f8 = pvStack_158;
        local_100 = local_160;
        lStack_e8 = lStack_148;
        local_f0 = local_150;
        if (1 < log::MAX_LOG_LEVEL_FILTER) {
          local_a0 = &local_100;
          pcStack_98 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
          local_90 = &DAT_1013a31cb;
          uStack_88 = 0x23;
          local_80 = &DAT_1013a31cb;
          uStack_78 = 0x23;
          local_70 = &PTR__anon_1f4a05fd1dc96b9982818105670d4dce_576_101826e20;
          log::__private_api::log(s_9_AiMaMi__router_commit__publish_1015284f9,&local_a0,2,&local_90);
        }
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_100);
      }
      <codexmate_lib::core::relay::native_catalog::CatalogRuntime_as_core::clone::Clone>::clone(&local_100,lVar13 + 0x158);
      pvStack_1c0 = pvStack_f8;
      local_1c8 = local_100;
      lStack_1b0 = lStack_e8;
      sStack_1b8 = local_f0;
      lStack_1a0 = lStack_d8;
      local_1a8 = local_e0;
      if ((local_194 == *(char *)(lVar13 + 0x18c)) && (local_182 == *(char *)(lVar13 + 0x19e))) {
        native_catalog::revision(&local_160,lStack_2f8,lStack_2f0,local_2e0,lStack_2d8);
        native_catalog::revision
                  (&local_100,*(undefined8 *)(lVar13 + 0x28),*(undefined8 *)(lVar13 + 0x30),*(undefined8 *)(lVar13 + 0x40),*(undefined8 *)(lVar13 + 0x48));
        pvVar6 = pvStack_f8;
        pvVar4 = pvStack_158;
        if ((local_150 == local_f0) && (iVar7 = _memcmp(pvStack_158,pvStack_f8,local_150), iVar7 == 0)) {
          if (local_100 != (undefined8 *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar6,local_100,1);
          }
          if (local_160 != (undefined8 *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar4,local_160,1);
          }
          goto LAB_10067db50;
        }
        if (local_100 != (undefined8 *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvStack_f8,local_100,1);
        }
        if (local_160 != (undefined8 *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvStack_158,local_160,1);
        }
      }
      local_1c8 = (undefined8 *)((long)local_1c8 + 1);
      pvStack_1c0 = (void *)((long)pvStack_1c0 + 1);
      lStack_1a0 = CONCAT62(lStack_1a0._2_6_,7);
LAB_10067db50:
      ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayState>(lVar13 + 0x20);
      _memmove((void *)(lVar13 + 0x20),&local_300,0x180);
      *param_1 = 0xb;
      param_1[1] = (long)pvVar5;
      ::core::ptr::
      drop_in_place<std::sync::poison::mutex::MutexGuard<std::collections::hash::map::HashMap<alloc::string::String,alloc::sync::Arc<tauri::manager::webview::UriSchemeProtocol<tauri_runtime_wry::Wry<tauri::EventLoopMessage>>>>>>
                (puVar1,uVar8);
      return;
    }
    param_1[7] = lStack_c8;
    param_1[6] = local_d0;
    param_1[9] = lStack_b8;
    param_1[8] = local_c0;
    param_1[0xb] = lStack_a8;
    param_1[10] = local_b0;
    param_1[3] = lStack_e8;
    param_1[2] = local_f0;
    param_1[5] = lStack_d8;
    param_1[4] = local_e0;
    *param_1 = (long)local_100;
    param_1[1] = (long)pvStack_f8;
  }
  else {
LAB_10067d7d4:
    param_1[5] = lStack_d8;
    param_1[4] = local_e0;
    param_1[7] = lStack_c8;
    param_1[6] = local_d0;
    param_1[9] = lStack_b8;
    param_1[8] = local_c0;
    param_1[0xb] = lStack_a8;
    param_1[10] = local_b0;
    param_1[1] = (long)pvStack_f8;
    *param_1 = (long)local_100;
    param_1[3] = lStack_e8;
    param_1[2] = local_f0;
  }
joined_r0x00010067d984:
  if ((((uVar8 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
     (uVar10 = std::panicking::panic_count::is_zero_slow_path(), (uVar10 & 1) == 0)) {
    *(undefined1 *)(lVar13 + 0x18) = 1;
  }
  std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*puVar1);
LAB_10067dbb4:
  ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayState>(&local_300);
  return;
}
