/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* strip_image_generation_tools @ 100367028 requested 100367028 */

/* codexmate_lib::core::relay::image_compat::strip_image_generation_tools */

long codexmate_lib::core::relay::image_compat::strip_image_generation_tools(undefined8 param_1)

{
  ulong uVar1;
  undefined8 *puVar2;
  char *pcVar3;
  ulong uVar4;
  long lVar5;
  undefined8 *puVar6;
  undefined8 *puVar7;
  ulong uVar8;
  ulong uVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  long local_58;
  
  pcVar3 = (char *)<str_as_serde_json::value::index::Index>::index_into_mut
                             ("toolsinputimage_generationgpt-5.6gpt-5.6-read_outputspawnmodelsaimami_no_account::",5,param_1);
  lVar5 = 0;
  if (pcVar3 == (char *)0x0) goto LAB_1003670bc;
  if (*pcVar3 != '\x04') {
    lVar5 = 0;
    goto LAB_1003670bc;
  }
  uVar8 = *(ulong *)(pcVar3 + 0x18);
  if (uVar8 != 0) {
    puVar10 = *(undefined8 **)(pcVar3 + 0x10);
    puVar6 = puVar10;
    uVar9 = 0;
LAB_100367084:
    uVar4 = alloc::vec::Vec<T,A>::retain::__closure__(puVar6);
    uVar1 = uVar9 + 1;
    if ((uVar4 & 1) != 0) goto code_r0x000100367094;
    ::core::ptr::drop_in_place<serde_json::value::Value>(puVar6);
    for (; uVar1 < uVar8; uVar1 = uVar1 + 1) {
      puVar7 = puVar6 + 4;
      uVar4 = alloc::vec::Vec<T,A>::retain::__closure__(puVar7);
      if ((uVar4 & 1) == 0) {
        ::core::ptr::drop_in_place<serde_json::value::Value>(puVar7);
      }
      else {
        uVar11 = *puVar7;
        uVar13 = puVar6[7];
        uVar12 = puVar6[6];
        puVar2 = puVar10 + uVar9 * 4;
        puVar2[1] = puVar6[5];
        *puVar2 = uVar11;
        puVar2[3] = uVar13;
        puVar2[2] = uVar12;
        uVar9 = uVar9 + 1;
      }
      puVar6 = puVar7;
    }
    *(ulong *)(pcVar3 + 0x18) = uVar9;
    goto LAB_1003670b8;
  }
  uVar9 = 0;
  goto LAB_1003670b8;
code_r0x000100367094:
  puVar6 = puVar6 + 4;
  uVar9 = uVar1;
  if (uVar8 == uVar1) goto code_r0x0001003670a4;
  goto LAB_100367084;
code_r0x0001003670a4:
  uVar9 = *(ulong *)(pcVar3 + 0x18);
LAB_1003670b8:
  lVar5 = uVar8 - uVar9;
LAB_1003670bc:
  local_58 = lVar5;
  pcVar3 = (char *)<str_as_serde_json::value::index::Index>::index_into_mut
                             ("inputimage_generationgpt-5.6gpt-5.6-read_outputspawnmodelsaimami_no_account::",5,param_1);
  if ((pcVar3 != (char *)0x0) && (*pcVar3 == '\x04')) {
    alloc::vec::Vec<T,A>::retain_mut(pcVar3 + 8,&local_58);
    lVar5 = local_58;
  }
  return lVar5;
}

