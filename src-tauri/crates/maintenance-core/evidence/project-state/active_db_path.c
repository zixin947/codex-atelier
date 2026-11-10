/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* active_db_path @ 1003b028c requested 1003b028c */

/* codexmate_lib::core::relay::codex_project_state::active_db_path */

void codexmate_lib::core::relay::codex_project_state::active_db_path(size_t *param_1,long param_2)

{
  void *pvVar1;
  size_t sVar2;
  void *pvVar3;
  undefined8 uVar4;
  size_t local_f0;
  void *pvStack_e8;
  size_t local_e0;
  long local_d8;
  undefined8 uStack_d0;

  if (*(long *)(param_2 + 0x2d0) == -0x8000000000000000) {
    platform::paths::CodexPaths::resolve_codex_state_db(&local_f0,*(undefined8 *)(param_2 + 8),*(undefined8 *)(param_2 + 0x10));
    pvVar1 = pvStack_e8;
  }
  else {
    sVar2 = *(size_t *)(param_2 + 0x2e0);
    if ((long)sVar2 < 0) {
      uVar4 = 0;
LAB_1003b02dc:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(uVar4,sVar2);
    }
    pvVar3 = *(void **)(param_2 + 0x2d8);
    if (sVar2 == 0) {
      pvVar1 = (void *)0x1;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar4 = 1;
      pvVar1 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar2,1);
      if (pvVar1 == (void *)0x0) goto LAB_1003b02dc;
    }
    local_f0 = sVar2;
    pvStack_e8 = pvVar1;
    _memcpy(pvVar1,pvVar3,sVar2);
    local_e0 = sVar2;
  }
  std::sys::fs::metadata(&local_d8,pvVar1,local_e0);
  ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_d8,uStack_d0);
  if (local_d8 == 0) {
    param_1[1] = (size_t)pvStack_e8;
    *param_1 = local_f0;
    param_1[2] = local_e0;
  }
  else {
    *param_1 = 0x8000000000000000;
    if (local_f0 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar1,local_f0,1);
    }
  }
  return;
}
