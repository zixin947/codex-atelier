/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* state_db_search_dirs @ 1008ee7b8 requested 1008ee7b8 */

/* codexmate_lib::platform::paths::CodexPaths::state_db_search_dirs */

void codexmate_lib::platform::paths::CodexPaths::state_db_search_dirs(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  long lVar1;
  long lVar2;
  long lVar3;
  long *plVar4;
  undefined8 uVar5;
  long local_68;
  long lStack_60;
  long local_58;
  
  configured_sqlite_home(&local_68);
  if (local_68 == -0x8000000000000000) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    plVar4 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x40,8);
    if (plVar4 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x40);
    }
    std::path::Path::_join
              (&local_68,param_2,param_3,
               "sqlitestate_.sqliteSELECT COALESCE(MAX(updated_at), 0) FROM threadsSELECT COALESCE(MAX(updated_at_ms), 0) FROM threadsSELECT COUNT(*) FROM threadsSELECT COUNT(*) FROM threads WHERE COALESCE(archived, 0) = 0"
               ,6);
    lVar3 = local_58;
    lVar2 = lStack_60;
    lVar1 = local_68;
    std::path::Path::to_path_buf(&local_68,param_2,param_3);
    plVar4[5] = lStack_60;
    plVar4[4] = local_68;
    plVar4[6] = local_58;
    *plVar4 = lVar1;
    plVar4[1] = lVar2;
    plVar4[2] = lVar3;
    *(undefined1 *)(plVar4 + 3) = 1;
    *(undefined1 *)(plVar4 + 7) = 0;
    uVar5 = 2;
    *param_1 = 2;
    param_1[1] = plVar4;
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    plVar4 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x20,8);
    if (plVar4 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error(8,0x20);
    }
    plVar4[1] = lStack_60;
    *plVar4 = local_68;
    plVar4[2] = local_58;
    uVar5 = 1;
    *(undefined1 *)(plVar4 + 3) = 1;
    *param_1 = 1;
    param_1[1] = plVar4;
  }
  param_1[2] = uVar5;
  return;
}

