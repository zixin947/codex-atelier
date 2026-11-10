/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* load_registry_for_mutation @ 10081d2f8 requested 10081d2f8 */

/* codexmate_lib::core::repository::Repository::load_registry_for_mutation */

void codexmate_lib::core::repository::Repository::load_registry_for_mutation(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  ulong uVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  int local_d8 [2];
  ulong local_d0;
  undefined1 local_40 [12];

  std::sys::fs::metadata(local_d8,param_2,param_3);
  if (local_d8[0] == 1) {
    if ((local_d0 & 3) == 1) {
      puVar2 = (undefined8 *)(local_d0 - 1);
      uVar3 = *puVar2;
      puVar4 = *(undefined8 **)(local_d0 + 7);
      if ((code *)*puVar4 != (code *)0x0) {
        (*(code *)*puVar4)(uVar3);
      }
      if (puVar4[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar3,puVar4[1],puVar4[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,0x18,8);
    }
    local_40 = std::time::SystemTime::now();
    std::time::SystemTime::duration_since(local_d8,local_40,0,0);
    uVar1 = 0;
    if (local_d8[0] == 0) {
      uVar1 = local_d0;
    }
    param_1[2] = 8;
    param_1[4] = 0x8000000000000000;
    param_1[3] = 0;
    param_1[7] = 0;
    param_1[8] = 0xa0000000f;
    *(undefined1 *)(param_1 + 9) = 0;
    param_1[10] = uVar1;
    *(undefined4 *)(param_1 + 0xb) = 4;
    *(undefined1 *)((long)param_1 + 0x5c) = 1;
    param_1[1] = 0;
    *param_1 = 0xb;
    return;
  }
  load_registry(param_1,param_2,param_3);
  return;
}
