/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read_backup_bytes @ 1001f7ab4 requested 1001f7ab4 */

/* codexmate_lib::core::relay::config_takeover::read_backup_bytes */

void codexmate_lib::core::relay::config_takeover::read_backup_bytes
               (undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 *local_100;
  undefined8 *local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 *local_e0;
  undefined8 *puStack_d8;
  undefined8 **local_d0;
  code *pcStack_c8;

  local_128 = param_4;
  uStack_120 = param_5;
  std::sys::fs::symlink_metadata(&local_e0,param_2,param_3);
  if ((int)local_e0 == 1) {
    cVar1 = std::io::error::Error::kind(puStack_d8);
    if (cVar1 == '\0') {
      param_1[1] = 0x8000000000000000;
      *param_1 = 0xb;
      if (((ulong)puStack_d8 & 3) == 1) {
        uVar3 = *(undefined8 *)((long)puStack_d8 - 1U);
        puVar2 = *(undefined8 **)((long)puStack_d8 + 7);
        if ((code *)*puVar2 != (code *)0x0) {
          (*(code *)*puVar2)(uVar3);
        }
        if (puVar2[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar3,puVar2[1],puVar2[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)puStack_d8 - 1U),0x18,8);
        return;
      }
    }
    else {
      local_f8 = puStack_d8;
      local_e0 = &local_128;
      puStack_d8 = &<&T_as_core::fmt::Display>::fmt;
      local_d0 = &local_f8;
      pcStack_c8 = <std::io::error::Error_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_118,s_inspect_for_recovery__10151d1ab,&local_e0);
      param_1[2] = uStack_110;
      param_1[1] = local_118;
      param_1[3] = local_108;
      *param_1 = 10;
      if (((ulong)local_f8 & 3) == 1) {
        puVar2 = (undefined8 *)((long)local_f8 - 1);
        uVar3 = *puVar2;
        puVar4 = *(undefined8 **)((long)local_f8 + 7);
        if ((code *)*puVar4 != (code *)0x0) {
          (*(code *)*puVar4)(uVar3);
        }
        if (puVar4[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar3,puVar4[1],puVar4[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,0x18,8);
      }
    }
  }
  else if (((ulong)puStack_d8 & 0xf00000000000) == 0x800000000000) {
    std::fs::read::inner(&local_e0,param_2,param_3);
    if (local_e0 == (undefined8 *)0x8000000000000000) {
      local_100 = puStack_d8;
      local_e0 = &local_128;
      puStack_d8 = &<&T_as_core::fmt::Display>::fmt;
      local_d0 = &local_100;
      pcStack_c8 = <std::io::error::Error_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_f8,s_read_for_recovery__10151d1c7,&local_e0);
      puStack_d8 = (undefined8 *)uStack_f0;
      local_e0 = local_f8;
      local_d0 = (undefined8 **)local_e8;
      if (((ulong)local_100 & 3) == 1) {
        puVar2 = (undefined8 *)((long)local_100 - 1);
        uVar3 = *puVar2;
        puVar4 = *(undefined8 **)((long)local_100 + 7);
        if ((code *)*puVar4 != (code *)0x0) {
          (*(code *)*puVar4)(uVar3);
        }
        if (puVar4[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar3,puVar4[1],puVar4[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,0x18,8);
      }
      *param_1 = 10;
      param_1[2] = puStack_d8;
      param_1[1] = local_e0;
      param_1[3] = local_d0;
    }
    else {
      param_1[2] = puStack_d8;
      param_1[3] = local_d0;
      *param_1 = 0xb;
      param_1[1] = local_e0;
    }
  }
  else {
    local_e0 = &local_128;
    puStack_d8 = &<&T_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(param_1 + 1,s___is_not_a_regular_recovery_file_10151d16e,&local_e0);
    *param_1 = 9;
  }
  return;
}
