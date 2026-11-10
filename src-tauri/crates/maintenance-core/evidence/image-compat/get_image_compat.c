/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* get_image_compat @ 1007b3e60 requested 1007b3e60 */

/* codexmate_lib::commands::system::get_image_compat */

void codexmate_lib::commands::system::get_image_compat(undefined8 *param_1)

{
  undefined8 uVar1;
  undefined1 auStack_358 [56];
  undefined8 local_320;
  undefined8 uStack_318;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 local_30;
  undefined8 uStack_28;
  
  platform::paths::CodexPaths::resolve_codex_home(&local_70);
  platform::paths::CodexPaths::from_home(auStack_358,&local_70);
  uVar1 = core::relay::image_compat::read_image_compat_enabled(local_320,uStack_318);
  core::models::CoreEnvelope<T>::ok(&local_70,uVar1);
  param_1[5] = uStack_48;
  param_1[4] = local_50;
  param_1[7] = uStack_38;
  param_1[6] = uStack_40;
  param_1[9] = uStack_28;
  param_1[8] = local_30;
  param_1[1] = uStack_68;
  *param_1 = local_70;
  param_1[3] = uStack_58;
  param_1[2] = uStack_60;
  ::core::ptr::drop_in_place<codexmate_lib::platform::paths::CodexPaths>(auStack_358);
  return;
}

