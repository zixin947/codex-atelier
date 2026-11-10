/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* logical_len @ 1008afa38 requested 1008afa38 */

/* codexmate_lib::core::relay::codex_thread_visibility::rollout_history::logical_len */

undefined1  [16] codexmate_lib::core::relay::codex_thread_visibility::rollout_history::logical_len(undefined8 param_1,undefined8 param_2)

{
  ulong uVar1;
  undefined8 *puVar2;
  ulong uVar3;
  undefined8 *puVar4;
  bool bVar5;
  ulong uVar6;
  long lVar7;
  undefined8 *puVar8;
  undefined1 auVar9 [16];
  ulong local_110;
  undefined8 *local_108;
  ulong local_100;
  undefined8 *local_98;
  ulong local_68;
  undefined8 *puStack_60;
  undefined1 uStack_51;

  resolve(&local_110,param_1,param_2);
  uVar6 = local_100;
  puVar2 = local_108;
  uVar1 = local_110;
  if (local_110 != 0x8000000000000001) {
    if (local_110 != 0x8000000000000000) {
      alloc::string::String::from_utf8_lossy(&local_110,local_108,local_100);
      if (local_100 < 10) {
        bVar5 = false;
        if ((local_110 & 0x7fffffffffffffff) == 0) goto LAB_1008afb2c;
LAB_1008afbb8:
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_108,local_110,1);
        if (bVar5) goto LAB_1008afb30;
LAB_1008afbc4:
        uVar6 = 0;
        puVar8 = local_98;
      }
      else {
        bVar5 = *(long *)((long)local_108 + (local_100 - 10)) == 0x7a2e6c6e6f736a2e && *(short *)((long)local_108 + (local_100 - 2)) == 0x7473;
        if ((local_110 & 0x7fffffffffffffff) != 0) goto LAB_1008afbb8;
LAB_1008afb2c:
        if (!bVar5) goto LAB_1008afbc4;
LAB_1008afb30:
        reader(&local_110,puVar2,uVar6);
        if (local_110 == 0) {
          if (uVar1 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,uVar1,1);
          }
          uVar6 = 1;
          puVar8 = local_108;
          goto LAB_1008afa9c;
        }
        local_68 = local_110;
        puStack_60 = local_108;
        auVar9 = std::io::copy::stack_buffer_copy(&local_68,&uStack_51);
        puVar4 = puStack_60;
        uVar3 = local_68;
        puVar8 = auVar9._8_8_;
        if ((code *)*puStack_60 != (code *)0x0) {
          (*(code *)*puStack_60)(local_68);
        }
        uVar6 = auVar9._0_8_ & 1;
        lVar7 = puVar4[1];
        if (lVar7 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar3,lVar7,puVar4[2]);
        }
      }
      if (uVar1 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,uVar1,1);
      }
      goto LAB_1008afa9c;
    }
    local_108 = (undefined8 *)std::io::error::Error::new(0,"rollout is missing",0x12);
  }
  uVar6 = 1;
  puVar8 = local_108;
LAB_1008afa9c:
  auVar9._8_8_ = puVar8;
  auVar9._0_8_ = uVar6;
  return auVar9;
}
