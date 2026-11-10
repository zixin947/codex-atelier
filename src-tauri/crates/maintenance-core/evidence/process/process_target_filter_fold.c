/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* try_fold @ 100510efc requested 100510efc */

/* <alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::try_fold */

undefined1  [16] <alloc::vec::into_iter::IntoIter<T,A>as_core::iter::traits::iterator::Iterator>::try_fold(long param_1,undefined8 param_2,undefined4 *param_3)

{
  long lVar1;
  undefined8 uVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined4 *puVar7;
  undefined1 auVar8 [16];
  undefined4 uStack_94;
  undefined4 local_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  
  puVar7 = *(undefined4 **)(param_1 + 0x18);
  puVar4 = *(undefined4 **)(param_1 + 8);
  while (puVar4 != puVar7) {
    uVar3 = *puVar4;
    *(undefined4 **)(param_1 + 8) = puVar4 + 8;
    lVar1 = *(long *)(puVar4 + 2);
    uVar2 = *(undefined8 *)(puVar4 + 4);
    codexmate_lib::platform::process::extract_process_name(&local_78,uVar2,*(undefined8 *)(puVar4 + 6));
    uVar6 = local_68;
    uVar5 = uStack_70;
    uStack_88 = (undefined4)uStack_70;
    local_90 = (undefined4)local_78;
    uStack_8c = (undefined4)((ulong)local_78 >> 0x20);
    if (lVar1 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar2,lVar1,1);
    }
    *param_3 = uVar3;
    *(ulong *)(param_3 + 3) = CONCAT44(uStack_88,uStack_8c);
    *(ulong *)(param_3 + 1) = CONCAT44(local_90,uStack_94);
    *(undefined8 *)(param_3 + 6) = uVar6;
    *(undefined8 *)(param_3 + 4) = uVar5;
    param_3 = param_3 + 8;
    puVar4 = puVar4 + 8;
  }
  auVar8._8_8_ = param_3;
  auVar8._0_8_ = param_2;
  return auVar8;
}

