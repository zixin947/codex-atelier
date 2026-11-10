===BEGIN backup_exists @ 1001f5724=== (GhidraScript)

/* codexmate_lib::core::relay::config_takeover::backup_exists */

uint codexmate_lib::core::relay::config_takeover::backup_exists(long param_1)

{
  undefined8 uVar1;
  ulong uVar2;
  uint uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long local_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  ulong local_c8;
  undefined8 uStack_c0;

  uVar4 = *(undefined8 *)(param_1 + 0x248);
  uVar5 = *(undefined8 *)(param_1 + 0x250);
  std::path::Path::_join(&local_e0,uVar4,uVar5,"router-takeover-backup.json",0x1b);
  uVar1 = local_d8;
  std::sys::fs::metadata(&local_c8,local_d8,uStack_d0);
  uVar2 = local_c8;
  ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_c8,uStack_c0);
  if ((uVar2 & 1) == 0) {
    if (local_e0 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar1,local_e0,1);
    }
    uVar3 = 1;
  }
  else {
    if (local_e0 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar1,local_e0,1);
    }
    std::path::Path::_join(&local_e0,uVar4,uVar5,"codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey",0x22);
    std::sys::fs::metadata(&local_c8,local_d8,uStack_d0);
    ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_c8,uStack_c0);
    uVar3 = (uint)local_c8 ^ 1;
    if (local_e0 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_d8,local_e0,1);
    }
  }
  return uVar3;
}

 (GhidraScript)
===END=== (GhidraScript)
