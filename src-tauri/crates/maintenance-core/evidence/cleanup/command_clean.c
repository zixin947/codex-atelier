/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* clean @ 1007b9084 requested 1007b9084 */

/* codexmate_lib::commands::system::clean */

int codexmate_lib::commands::system::clean(undefined8 *param_1,long *param_2)

{
  char cVar1;
  undefined **ppuVar2;
  undefined8 uVar3;
  undefined1 uVar4;
  int iVar5;
  uint uVar6;
  ulong uVar7;
  undefined ***local_170;
  undefined **local_168;
  undefined1 *puStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined4 local_108;
  undefined3 uStack_104;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined **local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 *puStack_68;
  undefined **local_60;
  undefined1 *local_58;

  if (*param_2 == 0) {
    std::sys::sync::once_box::OnceBox<T>::initialize(param_2);
    std::sys::pal::unix::sync::mutex::Mutex::lock();
  }
  else {
    std::sys::pal::unix::sync::mutex::Mutex::lock();
  }
  if ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) == 0) {
    uVar6 = 0;
    cVar1 = (char)param_2[1];
  }
  else {
    uVar6 = std::panicking::panic_count::is_zero_slow_path();
    uVar6 = uVar6 ^ 1;
    cVar1 = (char)param_2[1];
  }
  if (cVar1 == '\0') {
    core::repository::Repository::clean(&local_170,param_2 + 2);
    if ((int)local_170 == 1) {
      uStack_b8 = uStack_140;
      local_c0 = local_148;
      uStack_a8 = uStack_130;
      uStack_b0 = local_138;
      uStack_98 = uStack_120;
      local_a0 = local_128;
      uStack_88 = uStack_110;
      uStack_90 = local_118;
      uStack_d8 = puStack_160;
      local_e0 = local_168;
      uStack_c8 = uStack_150;
      local_d0 = local_158;
      local_80 = 0;
      uStack_78 = 1;
      local_58 = &DAT_60000020;
      puStack_68 = &local_80;
      local_70 = 0;
      local_60 = &_anon_6408540cdffbe318d49f52812e99783f_1640;
      iVar5 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_e0,&puStack_68);
      if (iVar5 != 0) {
                    /* WARNING: Subroutine does not return */
        ::core::result::unwrap_failed
                  ("a Display implementation returned an error unexpectedlycalled `Result::unwrap()` on an `Err` value",0x37,&local_100,
                   &_anon_6408540cdffbe318d49f52812e99783f_1652,&_anon_6408540cdffbe318d49f52812e99783f_1643);
      }
      uStack_f8 = uStack_78;
      local_100 = local_80;
      local_f0 = local_70;
      ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_168);
      param_1[2] = uStack_f8;
      param_1[1] = local_100;
      param_1[3] = local_f0;
      *param_1 = 0x8000000000000000;
    }
    else {
      param_1[5] = uStack_140;
      param_1[4] = local_148;
      param_1[7] = uStack_130;
      param_1[6] = local_138;
      param_1[9] = uStack_120;
      param_1[8] = local_128;
      param_1[0xb] = uStack_110;
      param_1[10] = local_118;
      param_1[1] = puStack_160;
      *param_1 = local_168;
      param_1[3] = uStack_150;
      param_1[2] = local_158;
    }
    if ((((uVar6 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
       (uVar7 = std::panicking::panic_count::is_zero_slow_path(), (uVar7 & 1) == 0)) {
      *(undefined1 *)(param_2 + 1) = 1;
    }
    iVar5 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*param_2);
  }
  else {
    local_e0 = (undefined **)0x0;
    uStack_d8 = 1;
    local_d0 = 0;
    puStack_160 = &DAT_60000020;
    local_170 = &local_e0;
    local_168 = &_anon_6408540cdffbe318d49f52812e99783f_1640;
    iVar5 = <str_as_core::fmt::Display>::fmt
                      ("poisoned lock: another task failed insidefatal runtime error: thread result panicked on drop, aborting\nnext_value_seed called before next_key_seedull"
                       ,0x29,&local_170);
    uVar3 = uStack_d8;
    ppuVar2 = local_e0;
    if (iVar5 != 0) {
                    /* WARNING: Subroutine does not return */
      ::core::result::unwrap_failed
                ("a Display implementation returned an error unexpectedlycalled `Result::unwrap()` on an `Err` value",0x37,&local_100,
                 &_anon_6408540cdffbe318d49f52812e99783f_1652,&_anon_6408540cdffbe318d49f52812e99783f_1643);
    }
    uVar4 = (undefined1)local_d0;
    uStack_104 = (undefined3)((ulong)local_d0 >> 0x28);
    local_108 = local_d0._1_4_;
    if ((((uVar6 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
       (uVar7 = std::panicking::panic_count::is_zero_slow_path(), (uVar7 & 1) == 0)) {
      *(undefined1 *)(param_2 + 1) = 1;
    }
    iVar5 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*param_2);
    param_1[1] = ppuVar2;
    param_1[2] = uVar3;
    *(undefined1 *)(param_1 + 3) = uVar4;
    *(undefined4 *)((long)param_1 + 0x19) = local_108;
    *(uint *)((long)param_1 + 0x1c) = CONCAT31(uStack_104,local_108._3_1_);
    *param_1 = 0x8000000000000000;
  }
  return iVar5;
}
