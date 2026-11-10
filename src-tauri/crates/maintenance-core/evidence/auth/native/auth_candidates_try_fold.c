/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* try_fold @ 10080b34c requested 10080b34c */

/* <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold */

void <core::iter::adapters::map::Map<I,F>as_core::iter::traits::iterator::Iterator>::try_fold(long *param_1,undefined8 param_2)

{
  long *plVar1;
  ulong uVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  int iVar6;
  undefined8 *puVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  undefined1 auVar10 [16];
  uint local_8e8 [2];
  long *local_8e0;
  ulong uStack_8d8;
  undefined1 auStack_8d0 [1064];
  long local_4a8;
  long local_4a0;
  long lStack_498;
  long *local_490;
  ulong local_488;
  undefined1 auStack_480 [1056];

  <std::fs::ReadDir_as_core::iter::traits::iterator::Iterator>::next(local_8e8);
  if (local_8e8[0] == 1) {
    do {
      uVar2 = uStack_8d8;
      plVar1 = local_8e0;
      if (local_8e0 == (long *)0x0) {
        if ((uStack_8d8 & 3) == 1) {
          puVar7 = (undefined8 *)(uStack_8d8 - 1);
          uVar8 = *puVar7;
          puVar9 = *(undefined8 **)(uStack_8d8 + 7);
          if ((code *)*puVar9 != (code *)0x0) {
            (*(code *)*puVar9)(uVar8);
          }
          if (puVar9[1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar8,puVar9[1],puVar9[2]);
          }
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,0x18,8);
        }
      }
      else {
        _memcpy(auStack_480,auStack_8d0,0x410);
        local_490 = plVar1;
        local_488 = uVar2;
        std::fs::DirEntry::path(&local_4a8,&local_490);
        lVar5 = *plVar1;
        *plVar1 = lVar5 + -1;
        LORelease();
        if (lVar5 == 1) {
          DataMemoryBarrier(2,1);
          alloc::sync::Arc<T,A>::drop_slow(&local_490);
        }
        lVar4 = lStack_498;
        lVar3 = local_4a0;
        lVar5 = local_4a8;
        iVar6 = std::path::Path::is_file(local_4a0,lStack_498);
        if (iVar6 != 0) {
          auVar10 = std::path::Path::extension(lVar3,lVar4);
          if (((auVar10._0_8_ != (int *)0x0) && (auVar10._8_8_ == 4)) && (*auVar10._0_8_ == 0x6e6f736a)) {
            if (lVar5 != -0x8000000000000000) {
              param_1[2] = lStack_498;
              param_1[1] = local_4a0;
              *param_1 = lVar5;
              return;
            }
            goto LAB_10080b3b4;
          }
        }
        if (lVar5 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar3,lVar5,1);
        }
      }
LAB_10080b3b4:
      <std::fs::ReadDir_as_core::iter::traits::iterator::Iterator>::next(local_8e8,param_2);
    } while ((local_8e8[0] & 1) != 0);
  }
  *param_1 = -0x8000000000000000;
  return;
}
