/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* acquire @ 100785384 requested 100785384 */

/* codexmate_lib::core::account_coordination::FileLock::acquire */

ulong codexmate_lib::core::account_coordination::FileLock::acquire
                (undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5)

{
  int iVar1;
  char cVar2;
  uint uVar3;
  ulong uVar4;
  undefined8 *puVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [12];
  undefined1 auVar10 [12];
  undefined8 local_e8;
  undefined8 uStack_e0;
  int local_d4;
  undefined8 *local_d0;
  int iStack_c8;
  undefined4 uStack_c4;
  undefined8 local_c0;
  undefined4 uStack_b8;
  undefined4 local_b4;
  undefined4 uStack_b0;
  undefined8 uStack_ac;
  undefined8 local_a4;
  undefined8 uStack_9c;
  undefined8 local_94;
  undefined8 uStack_8c;
  undefined8 local_84;
  undefined8 uStack_7c;
  undefined4 local_74;
  ulong local_70;
  undefined8 *local_68;
  undefined8 *puStack_60;
  undefined8 local_58;

  local_e8 = param_4;
  uStack_e0 = param_5;
  uVar4 = open(&local_d0);
  iVar1 = iStack_c8;
  if (local_d0 == (undefined8 *)&DAT_0000000b) {
    local_d4 = iStack_c8;
    auVar8 = std::time::Instant::now();
    auVar9 = <std::time::Instant_as_core::ops::arith::Add<core::time::Duration>>::add(auVar8._0_8_,auVar8._8_8_,0x14,0);
    while (uVar4 = <std::fs::File_as_fs2::FileExt>::try_lock_exclusive(&local_d4), uVar4 != 0) {
      cVar2 = std::io::error::Error::kind(uVar4);
      if (((uVar4 & 0xffffffff00000003) == 0x2300000002) || (cVar2 == '\r')) {
        local_70 = uVar4;
        auVar10 = std::time::Instant::now();
        if (auVar10._0_8_ == auVar9._0_8_) {
          if (auVar9._8_4_ <= auVar10._8_4_) {
LAB_100785520:
            local_68 = &local_e8;
            puStack_60 = &<&T_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&local_d0,s_timed_out_waiting_for_another_op_10152b029,&local_68);
            param_1[2] = CONCAT44(uStack_c4,iStack_c8);
            param_1[1] = local_d0;
            param_1[3] = local_c0;
            *param_1 = 10;
            if ((uVar4 & 3) != 1) goto LAB_100785620;
            puVar7 = (undefined8 *)(uVar4 - 1);
            uVar6 = *puVar7;
            puVar5 = *(undefined8 **)(uVar4 + 7);
            if ((code *)*puVar5 != (code *)0x0) {
              (*(code *)*puVar5)(uVar6);
            }
            goto LAB_1007855fc;
          }
        }
        else if (auVar9._0_8_ <= auVar10._0_8_) goto LAB_100785520;
        std::thread::functions::sleep(0,25000000);
        if ((uVar4 & 3) == 1) {
          uVar6 = *(undefined8 *)(uVar4 - 1);
          puVar7 = *(undefined8 **)(uVar4 + 7);
          if ((code *)*puVar7 != (code *)0x0) {
            (*(code *)*puVar7)(uVar6);
          }
LAB_1007854bc:
          if (puVar7[1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar6,puVar7[1],puVar7[2]);
          }
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar4 - 1,0x18,8);
        }
      }
      else {
        cVar2 = std::io::error::Error::kind(uVar4);
        if (cVar2 != '#') {
          local_d0 = &local_e8;
          iStack_c8 = 0x7dd6c0;
          uStack_c4 = 1;
          local_c0 = &local_70;
          uStack_b8 = 0x1169844;
          local_b4 = 1;
          local_70 = uVar4;
          alloc::fmt::format::format_inner(&local_68,s_failed_to_acquire_coordination_l_10152affe,&local_d0);
          param_1[2] = puStack_60;
          param_1[1] = local_68;
          param_1[3] = local_58;
          *param_1 = 10;
          if ((local_70 & 3) != 1) goto LAB_100785620;
          puVar7 = (undefined8 *)(local_70 - 1);
          uVar6 = *puVar7;
          puVar5 = *(undefined8 **)(local_70 + 7);
          if ((code *)*puVar5 != (code *)0x0) {
            (*(code *)*puVar5)(uVar6);
          }
LAB_1007855fc:
          if (puVar5[1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar6,puVar5[1],puVar5[2]);
          }
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,0x18,8);
LAB_100785620:
          uVar3 = _close(iVar1);
          return (ulong)uVar3;
        }
        if ((uVar4 & 3) == 1) {
          uVar6 = *(undefined8 *)(uVar4 - 1);
          puVar7 = *(undefined8 **)(uVar4 + 7);
          if ((code *)*puVar7 != (code *)0x0) {
            (*(code *)*puVar7)(uVar6);
          }
          goto LAB_1007854bc;
        }
      }
    }
    *(int *)(param_1 + 1) = iVar1;
    *param_1 = 0xb;
    uVar4 = 0;
  }
  else {
    *(undefined8 *)((long)param_1 + 0x34) = uStack_9c;
    *(undefined8 *)((long)param_1 + 0x2c) = local_a4;
    *(undefined8 *)((long)param_1 + 0x44) = uStack_8c;
    *(undefined8 *)((long)param_1 + 0x3c) = local_94;
    *(undefined8 *)((long)param_1 + 0x54) = uStack_7c;
    *(undefined8 *)((long)param_1 + 0x4c) = local_84;
    *(undefined4 *)((long)param_1 + 0x5c) = local_74;
    *(ulong *)((long)param_1 + 0x14) = CONCAT44(uStack_b8,local_c0._4_4_);
    *(ulong *)((long)param_1 + 0xc) = CONCAT44((undefined4)local_c0,uStack_c4);
    *(undefined8 *)((long)param_1 + 0x24) = uStack_ac;
    *(ulong *)((long)param_1 + 0x1c) = CONCAT44(uStack_b0,local_b4);
    *param_1 = local_d0;
    *(int *)(param_1 + 1) = iStack_c8;
  }
  return uVar4;
}
