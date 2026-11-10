/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* require_existing_rollout @ 1008b51a0 requested 1008b51a0 */

/* codexmate_lib::core::relay::codex_thread_visibility::require_existing_rollout */

void codexmate_lib::core::relay::codex_thread_visibility::require_existing_rollout(undefined8 *param_1,ulong param_2)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  long local_170;
  ulong local_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 uStack_118;
  ulong local_c8;
  undefined8 uStack_c0;
  ulong *local_b8;
  code *pcStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  ulong local_50 [2];

  if (*(long *)(param_2 + 0x30) == -0x8000000000000000) {
    uStack_c0 = 0x1009113b0;
    local_c8 = param_2;
    alloc::fmt::format::format_inner(&local_168,&DAT_1013b54e7,&local_c8);
    param_1[7] = uStack_138;
    param_1[6] = local_140;
    param_1[9] = uStack_128;
    param_1[8] = uStack_130;
    param_1[0xb] = uStack_118;
    param_1[10] = local_120;
    param_1[3] = uStack_158;
    param_1[2] = local_160;
    param_1[5] = uStack_148;
    param_1[4] = uStack_150;
    *param_1 = 10;
    param_1[1] = local_168;
  }
  else {
    rollout_history::resolve(&local_170,*(undefined8 *)(param_2 + 0x38),*(undefined8 *)(param_2 + 0x40));
    if (local_170 == -0x8000000000000000) {
      local_50[1] = 0x1009113b0;
      local_50[0] = param_2;
      alloc::fmt::format::format_inner(&uStack_c0,&DAT_1013b54e7,local_50);
      local_c8 = 10;
      param_1[5] = uStack_a0;
      param_1[4] = local_a8;
      param_1[7] = uStack_90;
      param_1[6] = uStack_98;
      param_1[9] = uStack_80;
      param_1[8] = local_88;
      param_1[0xb] = uStack_70;
      param_1[10] = uStack_78;
      param_1[1] = uStack_c0;
      *param_1 = 10;
      param_1[3] = pcStack_b0;
      param_1[2] = local_b8;
    }
    else {
      if (local_170 == -0x7fffffffffffffff) {
        local_50[0] = local_168;
        uStack_c0 = 0x1009113b0;
        local_b8 = local_50;
        pcStack_b0 = <std::io::error::Error_as_core::fmt::Display>::fmt;
        local_c8 = param_2;
        alloc::fmt::format::format_inner(&local_68,s__10152ff6c,&local_c8);
        param_1[2] = uStack_60;
        param_1[1] = local_68;
        param_1[3] = local_58;
        *param_1 = 10;
        if ((local_50[0] & 3) != 1) {
          return;
        }
        puVar1 = (undefined8 *)(local_50[0] - 1);
        uVar2 = *puVar1;
        puVar3 = *(undefined8 **)(local_50[0] + 7);
        if ((code *)*puVar3 != (code *)0x0) {
          (*(code *)*puVar3)(uVar2);
        }
        if (puVar3[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar2,puVar3[1],puVar3[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar1,0x18,8);
        return;
      }
      *param_1 = 0xb;
    }
    ::core::ptr::drop_in_place<core::result::Result<core::option::Option<(std::path::PathBuf,std::fs::Metadata)>,std::io::error::Error>>(local_170,local_168);
  }
  return;
}
