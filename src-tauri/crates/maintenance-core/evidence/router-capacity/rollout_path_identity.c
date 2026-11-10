/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* rollout_path_identity @ 1008b3298 requested 1008b3298 */

/* codexmate_lib::core::relay::codex_thread_visibility::rollout_path_identity */

void codexmate_lib::core::relay::codex_thread_visibility::rollout_path_identity(ulong *param_1,ulong param_2,ulong param_3)

{
  ulong uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  ulong uVar5;
  undefined8 *puVar6;
  ulong local_e8;
  ulong uStack_e0;
  ulong local_d8;

  rollout_history::resolve(&local_e8,param_2,param_3);
  uVar1 = uStack_e0;
  if (local_e8 == 0x8000000000000001) {
    ::core::ptr::drop_in_place<core::result::Result<core::option::Option<(std::path::PathBuf,std::fs::Metadata)>,std::io::error::Error>>
              (0x8000000000000001,uStack_e0);
  }
  else {
    uVar2 = local_e8;
    uVar3 = local_d8;
    uVar5 = uStack_e0;
    if (local_e8 != 0x8000000000000000) goto LAB_1008b3308;
  }
  uVar2 = 0x8000000000000000;
  uVar3 = param_3;
  uVar5 = param_2;
LAB_1008b3308:
  std::sys::fs::canonicalize(&local_e8,uVar5,uVar3);
  if (local_e8 == 0x8000000000000000) {
    std::path::Path::to_path_buf(param_1,uVar5,uVar3);
    if ((uStack_e0 & 3) == 1) {
      uVar4 = *(undefined8 *)(uStack_e0 - 1);
      puVar6 = *(undefined8 **)(uStack_e0 + 7);
      if ((code *)*puVar6 != (code *)0x0) {
        (*(code *)*puVar6)(uVar4);
      }
      if (puVar6[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar4,puVar6[1],puVar6[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(uStack_e0 - 1),0x18,8);
    }
  }
  else {
    param_1[1] = uStack_e0;
    *param_1 = local_e8;
    param_1[2] = local_d8;
  }
  if ((uVar2 & 0x7fffffffffffffff) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar1,uVar2,1);
  }
  return;
}
