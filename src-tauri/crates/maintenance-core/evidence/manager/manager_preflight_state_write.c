/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* preflight_repair_state_write @ 100684350 requested 100684350 */

/* codexmate_lib::core::relay::manager::RelayManager::preflight_repair_state_write */

void codexmate_lib::core::relay::manager::RelayManager::preflight_repair_state_write(long *param_1,long param_2,undefined8 param_3)

{
  undefined *local_130;
  undefined8 uStack_128;
  long local_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  long local_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  long local_58;
  long lStack_50;
  long local_48;
  undefined1 *local_40;
  undefined8 *puStack_38;
  long *local_30;
  code *pcStack_28;

  local_130 = &DAT_1013a3582;
  uStack_128 = 0x11;
  storage::preflight_save(&local_120,param_2 + 0x10,param_3);
  if (local_120 != 0xb) {
    uStack_98 = uStack_f8;
    local_a0 = local_100;
    uStack_88 = uStack_e8;
    uStack_90 = uStack_f0;
    uStack_78 = uStack_d8;
    local_80 = local_e0;
    uStack_68 = uStack_c8;
    uStack_70 = uStack_d0;
    uStack_b8 = uStack_118;
    local_c0 = local_120;
    uStack_a8 = uStack_108;
    uStack_b0 = uStack_110;
    puStack_38 = &<&T_as_core::fmt::Display>::fmt;
    local_30 = &local_c0;
    pcStack_28 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
    local_40 = (undefined1 *)&local_130;
    alloc::fmt::format::format_inner(&local_58,s___cannot_start_because_relay_sta_101528ae5,&local_40);
    param_1[2] = lStack_50;
    param_1[1] = local_58;
    param_1[3] = local_48;
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_c0);
    local_120 = 10;
  }
  *param_1 = local_120;
  return;
}
