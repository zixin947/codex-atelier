/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* load_registry @ 10080c338 requested 10080c338 */

/* codexmate_lib::core::repository::Repository::load_registry */

void codexmate_lib::core::repository::Repository::load_registry(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  long lVar1;
  undefined8 uVar2;
  long local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined8 uStack_48;
  undefined8 local_40;
  undefined8 uStack_38;

  std::fs::read_to_string::inner(&local_b8,param_2,param_3);
  uVar2 = uStack_b0;
  lVar1 = local_b8;
  if (local_b8 == -0x8000000000000000) {
    *param_1 = 2;
    param_1[1] = uStack_b0;
  }
  else {
    local_60 = uStack_b0;
    uStack_58 = local_a8;
    local_50 = 0;
    uStack_48 = 0;
    local_40 = uStack_b0;
    uStack_38 = local_a8;
    serde_json::de::from_trait(&local_b8,&local_60);
    if (local_b8 == -0x8000000000000000) {
      *param_1 = 3;
      param_1[1] = uStack_b0;
    }
    else {
      param_1[6] = uStack_90;
      param_1[5] = local_98;
      param_1[8] = uStack_80;
      param_1[7] = local_88;
      param_1[10] = uStack_70;
      param_1[9] = local_78;
      param_1[0xb] = local_68;
      param_1[4] = uStack_a0;
      param_1[3] = local_a8;
      param_1[1] = local_b8;
      param_1[2] = uStack_b0;
      *param_1 = 0xb;
    }
    if (lVar1 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar2,lVar1,1);
    }
  }
  return;
}
