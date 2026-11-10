/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* {{closure}} @ 1008abef4 requested 1008abef4 */

/* codexmate_lib::core::relay::codex_thread_visibility::model_fallback::load_model_restore_journal::{{closure}} */

void codexmate_lib::core::relay::codex_thread_visibility::model_fallback::load_model_restore_journal::__closure__(undefined8 *param_1,undefined8 param_2)

{
  undefined8 local_48;
  undefined *puStack_40;
  undefined8 local_38;
  undefined **local_30;
  undefined8 *puStack_28;

  puStack_40 = &DAT_1013b5066;
  local_38 = 0x1e;
  local_30 = &puStack_40;
  puStack_28 = &<&T_as_core::fmt::Display>::fmt;
  local_48 = param_2;
  alloc::fmt::format::format_inner(param_1 + 1,s__10152f955,&local_30);
  *param_1 = 10;
  ::core::ptr::drop_in_place<serde_json::error::Error>(&local_48);
  return;
}
