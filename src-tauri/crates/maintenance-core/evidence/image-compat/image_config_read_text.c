/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read_text @ 1005c223c requested 1005c223c */

/* codexmate_lib::core::codex_config::read_text */

void codexmate_lib::core::codex_config::read_text(undefined8 *param_1)

{
  ulong uVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  long local_48;
  ulong uStack_40;
  undefined8 local_38;
  
  std::fs::read_to_string::inner(&local_48);
  if (local_48 != -0x8000000000000000) {
    param_1[2] = uStack_40;
    param_1[1] = local_48;
    param_1[3] = local_38;
    *param_1 = 0xb;
    return;
  }
  uVar1 = uStack_40 & 3;
  if (uVar1 < 2) {
    if (uVar1 == 0) {
      if (*(char *)(uStack_40 + 0x10) == '\0') goto LAB_1005c2334;
    }
    else if (*(char *)(uStack_40 + 0xf) == '\0') {
      param_1[2] = 1;
      param_1[3] = 0;
      param_1[1] = 0;
      *param_1 = 0xb;
      uVar2 = *(undefined8 *)(uStack_40 - 1);
      puVar3 = *(undefined8 **)(uStack_40 + 7);
      if ((code *)*puVar3 != (code *)0x0) {
        (*(code *)*puVar3)(uVar2);
      }
      if (puVar3[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar2,puVar3[1],puVar3[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_40 - 1,0x18,8);
      return;
    }
  }
  else if (uVar1 == 2) {
    if (uStack_40 >> 0x20 == 2) {
LAB_1005c2334:
      param_1[2] = 1;
      param_1[3] = 0;
      param_1[1] = 0;
      *param_1 = 0xb;
      return;
    }
  }
  else if (uStack_40 >> 0x20 == 0) goto LAB_1005c2334;
  *param_1 = 2;
  param_1[1] = uStack_40;
  return;
}

