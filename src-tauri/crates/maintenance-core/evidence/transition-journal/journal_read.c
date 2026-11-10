/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read @ 100792574 requested 100792574 */

/* codexmate_lib::core::relay::transition_journal::read */

void codexmate_lib::core::relay::transition_journal::read(undefined8 *param_1)

{
  long local_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  long local_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  long *local_98;
  code *pcStack_90;
  undefined8 local_88;
  undefined *puStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  char *local_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined *local_50;
  undefined8 local_48;
  undefined8 local_40;
  char *local_38;
  undefined8 **ppuStack_30;
  undefined1 uStack_21;
  
  read_checked(&local_160);
  if (local_160 == 0xb) {
    param_1[5] = uStack_130;
    param_1[4] = local_138;
    param_1[7] = uStack_120;
    param_1[6] = local_128;
    param_1[8] = local_118;
    param_1[1] = uStack_150;
    *param_1 = uStack_158;
    param_1[3] = uStack_140;
    param_1[2] = local_148;
  }
  else {
    uStack_d8 = local_138;
    local_e0 = uStack_140;
    uStack_c8 = local_128;
    uStack_d0 = uStack_130;
    uStack_b8 = local_118;
    local_c0 = uStack_120;
    uStack_a8 = uStack_108;
    uStack_b0 = uStack_110;
    uStack_f8 = uStack_158;
    local_100 = local_160;
    uStack_e8 = local_148;
    uStack_f0 = uStack_150;
    if (1 < log::MAX_LOG_LEVEL_FILTER) {
      local_98 = &local_100;
      pcStack_90 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
      local_48 = 0x2e;
      ppuStack_30 = &local_98;
      local_38 = _anon_6408540cdffbe318d49f52812e99783f_550;
      local_88 = 0;
      puStack_80 = &_anon_6408540cdffbe318d49f52812e99783f_553;
      uStack_70 = 0;
      local_78 = 0x2e;
      local_50 = &_anon_6408540cdffbe318d49f52812e99783f_553;
      local_68 = "src/core/relay/transition_journal.rs";
      uStack_58 = 2;
      local_60 = 0x24;
      local_40 = 0x12b00000001;
      <log::__private_api::GlobalLogger_as_log::Log>::log(&uStack_21,&local_88);
    }
    *param_1 = 0x8000000000000000;
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_100);
  }
  return;
}

