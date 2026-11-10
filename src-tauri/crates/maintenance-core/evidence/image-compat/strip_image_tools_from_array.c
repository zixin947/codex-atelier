/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* strip_image_tools_from_array @ 1003671a0 requested 1003671a0 */

/* codexmate_lib::core::relay::image_compat::strip_image_tools_from_array */

long codexmate_lib::core::relay::image_compat::strip_image_tools_from_array(long param_1)

{
  ulong uVar1;
  undefined8 *puVar2;
  ulong uVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  ulong uVar6;
  ulong uVar7;
  undefined8 *puVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  
  uVar6 = *(ulong *)(param_1 + 0x10);
  if (uVar6 == 0) {
    uVar3 = 0;
  }
  else {
    puVar8 = *(undefined8 **)(param_1 + 8);
    puVar4 = puVar8;
    uVar7 = 0;
    do {
      uVar3 = alloc::vec::Vec<T,A>::retain::__closure__(puVar4);
      uVar1 = uVar7 + 1;
      if ((uVar3 & 1) == 0) {
        ::core::ptr::drop_in_place<serde_json::value::Value>(puVar4);
        for (; uVar1 < uVar6; uVar1 = uVar1 + 1) {
          puVar5 = puVar4 + 4;
          uVar3 = alloc::vec::Vec<T,A>::retain::__closure__(puVar5);
          if ((uVar3 & 1) == 0) {
            ::core::ptr::drop_in_place<serde_json::value::Value>(puVar5);
          }
          else {
            uVar9 = *puVar5;
            uVar11 = puVar4[7];
            uVar10 = puVar4[6];
            puVar2 = puVar8 + uVar7 * 4;
            puVar2[1] = puVar4[5];
            *puVar2 = uVar9;
            puVar2[3] = uVar11;
            puVar2[2] = uVar10;
            uVar7 = uVar7 + 1;
          }
          puVar4 = puVar5;
        }
        *(ulong *)(param_1 + 0x10) = uVar7;
        uVar3 = uVar7;
        break;
      }
      puVar4 = puVar4 + 4;
      uVar3 = uVar6;
      uVar7 = uVar1;
    } while (uVar6 != uVar1);
  }
  return uVar6 - uVar3;
}

