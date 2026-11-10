/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* normalize_section_key @ 1001f86b0 requested 1001f86b0 */

/* codexmate_lib::core::relay::config_takeover::normalize_section_key */

void codexmate_lib::core::relay::config_takeover::normalize_section_key(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  long local_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined1 local_48;
  undefined2 local_40;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined8 local_28;

  local_80 = 0;
  local_60 = 0;
  local_50 = 0x2e0000002e;
  local_48 = 1;
  local_40 = 1;
  uStack_78 = param_3;
  local_70 = param_2;
  uStack_68 = param_3;
  uStack_58 = param_3;
  <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_98,&local_80);
  alloc::str::join_generic_copy(&local_38,local_90,uStack_88,".",1);
  param_1[1] = uStack_30;
  *param_1 = local_38;
  param_1[2] = local_28;
  if (local_98 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_90,local_98 << 4,8);
  }
  return;
}
