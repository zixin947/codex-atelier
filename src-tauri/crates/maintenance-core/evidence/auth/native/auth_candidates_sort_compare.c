/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* {{closure}} @ 100910780 requested 100910780 */

/* alloc::slice::<impl [T]>::sort_by::{{closure}} */

bool alloc::slice::<impl[T]>::sort_by::__closure__(long param_1,long param_2)

{
  bool bVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  ulong uVar5;
  uint uVar6;
  undefined8 *puVar7;
  byte bVar8;
  undefined1 local_178 [8];
  undefined8 local_170;
  undefined8 uStack_168;
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
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  uint local_e8;
  undefined4 uStack_e4;
  uint local_e0;
  undefined4 uStack_dc;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
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
  undefined8 uStack_60;
  undefined8 local_58;

  std::sys::fs::metadata(&local_e8,*(undefined8 *)(param_1 + 8),*(undefined8 *)(param_1 + 0x10));
  uVar5 = CONCAT44(uStack_dc,local_e0);
  uVar6 = local_e0;
  if (local_e8 == 1) {
joined_r0x00010091084c:
    if ((uVar6 & 3) == 1) {
      uVar4 = *(undefined8 *)(uVar5 - 1);
      puVar7 = *(undefined8 **)(uVar5 + 7);
      if ((code *)*puVar7 != (code *)0x0) {
        (*(code *)*puVar7)(uVar4);
      }
      if (puVar7[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar4,puVar7[1],puVar7[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(uVar5 - 1),0x18,8);
    }
    bVar8 = 1;
    uVar6 = 1000000000;
  }
  else {
    uStack_118 = uStack_80;
    local_120 = local_88;
    uStack_108 = uStack_70;
    uStack_110 = local_78;
    uStack_f8 = uStack_60;
    local_100 = local_68;
    uStack_158 = uStack_c0;
    local_160 = local_c8;
    uStack_148 = uStack_b0;
    uStack_150 = local_b8;
    uStack_138 = uStack_a0;
    local_140 = local_a8;
    uStack_128 = uStack_90;
    uStack_130 = local_98;
    local_f0 = local_58;
    uStack_168 = uStack_d0;
    local_170 = local_d8;
    std::fs::Metadata::modified(&local_e8,local_178);
    uVar5 = CONCAT44(uStack_e4,local_e8);
    uVar6 = local_e8;
    if (local_e0 == 1000000000) goto joined_r0x00010091084c;
    bVar8 = 0;
    uVar6 = local_e0;
  }
  std::sys::fs::metadata(&local_e8,*(undefined8 *)(param_2 + 8),*(undefined8 *)(param_2 + 0x10));
  uVar3 = CONCAT44(uStack_dc,local_e0);
  if (local_e8 != 1) {
    uStack_118 = uStack_80;
    local_120 = local_88;
    uStack_108 = uStack_70;
    uStack_110 = local_78;
    uStack_f8 = uStack_60;
    local_100 = local_68;
    uStack_158 = uStack_c0;
    local_160 = local_c8;
    uStack_148 = uStack_b0;
    uStack_150 = local_b8;
    uStack_138 = uStack_a0;
    local_140 = local_a8;
    uStack_128 = uStack_90;
    uStack_130 = local_98;
    local_f0 = local_58;
    uStack_168 = uStack_d0;
    local_170 = local_d8;
    std::fs::Metadata::modified(&local_e8,local_178);
    uVar3 = CONCAT44(uStack_e4,local_e8);
    if (local_e0 != 1000000000) goto LAB_100910970;
  }
  if ((uVar3 & 3) == 1) {
    uVar4 = *(undefined8 *)(uVar3 - 1);
    puVar7 = *(undefined8 **)(uVar3 + 7);
    if ((code *)*puVar7 != (code *)0x0) {
      (*(code *)*puVar7)(uVar4);
    }
    if (puVar7[1] != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar4,puVar7[1],puVar7[2]);
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(uVar3 - 1),0x18,8);
  }
  uVar3 = 0;
  local_e0 = 0;
LAB_100910970:
  uVar2 = 0;
  if (bVar8 == 0) {
    uVar2 = uVar5;
  }
  bVar1 = (bool)(local_e0 < uVar6 & (bVar8 ^ 0xff));
  if (uVar3 != uVar2) {
    bVar1 = (long)uVar3 < (long)uVar2;
  }
  return bVar1;
}
