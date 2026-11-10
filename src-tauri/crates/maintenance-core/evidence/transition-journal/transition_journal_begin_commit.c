/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* begin_commit @ 100790988 requested 100790988 */

/* codexmate_lib::core::relay::transition_journal::begin_commit */

void codexmate_lib::core::relay::transition_journal::begin_commit(long *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5)

{
  undefined1 uVar1;
  ulong uVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  undefined8 ***local_d0;
  code *pcStack_c8;
  long local_c0;
  long lStack_b8;
  long lStack_b0;
  long lStack_a8;
  long local_a0;
  long lStack_98;
  long lStack_90;
  long lStack_88;
  long local_80;
  long lStack_78;
  undefined8 ***local_70;
  code *pcStack_68;
  long local_60;
  ulong local_50;
  undefined8 *local_48;
  undefined8 local_40;
  
  write_inner(&local_d0,param_2,param_3,
              "committingrouter-transition-journal.jsonunsupported transition record versionphase record exceeds the size limitphase record is not a regular file"
              ,10,param_4,param_5,0);
  if (local_d0 == (undefined8 ***)0xb) {
    uVar2 = atomic_write::Publication::require_sync();
    if (uVar2 == 0) {
      *param_1 = 0xb;
    }
    else {
      local_50 = uVar2;
      uVar1 = std::io::error::Error::kind();
      local_70 = (undefined8 ***)CONCAT71(local_70._1_7_,uVar1);
      local_d0 = &local_70;
      pcStack_c8 = <std::io::error::ErrorKind_as_core::fmt::Debug>::fmt;
      alloc::fmt::format::format_inner(&local_48,_anon_6408540cdffbe318d49f52812e99783f_425,&local_d0);
      pcStack_68 = <alloc::string::String_as_core::fmt::Display>::fmt;
      local_70 = (undefined8 ***)&local_48;
      alloc::fmt::format::format_inner(&local_d0,_anon_6408540cdffbe318d49f52812e99783f_436,&local_70);
      pcStack_68 = pcStack_c8;
      local_70 = local_d0;
      local_60 = local_c0;
      if ((undefined8 **)local_48 != (undefined8 **)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_40,local_48,1);
      }
      if ((uVar2 & 3) == 1) {
        uVar3 = *(undefined8 *)(uVar2 - 1);
        puVar4 = *(undefined8 **)(uVar2 + 7);
        if ((code *)*puVar4 != (code *)0x0) {
          (*(code *)*puVar4)(uVar3);
        }
        if (puVar4[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar3,puVar4[1],puVar4[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(uVar2 - 1),0x18,8);
      }
      *param_1 = 9;
      param_1[2] = (long)pcStack_68;
      param_1[1] = (long)local_70;
      param_1[3] = local_60;
    }
  }
  else {
    param_1[7] = lStack_98;
    param_1[6] = local_a0;
    param_1[9] = lStack_88;
    param_1[8] = lStack_90;
    param_1[0xb] = lStack_78;
    param_1[10] = local_80;
    param_1[3] = lStack_b8;
    param_1[2] = local_c0;
    param_1[5] = lStack_a8;
    param_1[4] = lStack_b0;
    *param_1 = (long)local_d0;
    param_1[1] = (long)pcStack_c8;
  }
  return;
}

