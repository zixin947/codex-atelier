/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* deserialize @ 1008f1a78 requested 1008f1a78 */

/* <<codexmate_lib::core::relay::codex_thread_visibility::_::<impl serde_core::de::Deserialize for
   codexmate_lib::core::relay::codex_thread_visibility::NoAccountModelRestoreEntry>::deserialize::__Visitor as
   serde_core::de::Visitor>::visit_map::__DeserializeWith as serde_core::de::Deserialize>::deserialize */

void codexmate_lib::core::relay::codex_thread_visibility::NoAccountModelRestoreEntry>::deserialize::__Visitor_as_serde_core::de::Visitor>::visit_map::
     __DeserializeWith_as_serde_core::de::Deserialize>::deserialize(long *param_1,long param_2)

{
  ulong uVar1;
  ulong uVar2;
  byte bVar3;
  char cVar4;
  ulong uVar5;
  long lVar6;
  ulong uVar7;
  long local_38;
  long lStack_30;
  long local_28;

  uVar2 = *(ulong *)(param_2 + 0x20);
  if (*(ulong *)(param_2 + 0x28) < uVar2) {
    uVar5 = *(ulong *)(param_2 + 0x28) + 2;
    lVar6 = *(long *)(param_2 + 0x18);
    do {
      bVar3 = *(byte *)(lVar6 + uVar5 + -2);
      if (0x20 < bVar3 || (1L << ((ulong)bVar3 & 0x3f) & 0x100002600U) == 0) {
        if (bVar3 == 0x6e) {
          uVar7 = uVar5 - 1;
          *(ulong *)(param_2 + 0x28) = uVar7;
          uVar1 = uVar7;
          if (uVar7 <= uVar2) {
            uVar1 = uVar2;
          }
          if (uVar7 < uVar2) {
            cVar4 = *(char *)(lVar6 + uVar5 + -1);
            *(ulong *)(param_2 + 0x28) = uVar5;
            if (cVar4 == 'u') {
              if (uVar5 == uVar1) goto LAB_1008f1ba0;
              cVar4 = *(char *)(lVar6 + uVar5);
              *(ulong *)(param_2 + 0x28) = uVar5 + 1;
              if (cVar4 == 'l') {
                if (uVar5 + 1 == uVar1) goto LAB_1008f1ba0;
                cVar4 = *(char *)(lVar6 + uVar5 + 1);
                *(ulong *)(param_2 + 0x28) = uVar5 + 2;
                if (cVar4 == 'l') {
                  local_38 = -0x8000000000000000;
                  local_28 = 0x6c;
                  goto LAB_1008f1b88;
                }
              }
            }
            local_38 = 9;
          }
          else {
LAB_1008f1ba0:
            local_38 = 5;
          }
          lStack_30 = serde_json::de::Deserializer<R>::error(param_2,&local_38);
          goto LAB_1008f1b78;
        }
        break;
      }
      *(ulong *)(param_2 + 0x28) = uVar5 - 1;
      uVar5 = uVar5 + 1;
    } while (uVar5 - uVar2 != 2);
  }
  <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_string(&local_38);
  param_2 = lStack_30;
  if (local_38 == -0x8000000000000000) {
LAB_1008f1b78:
    *param_1 = -0x7fffffffffffffff;
    param_1[1] = lStack_30;
  }
  else {
LAB_1008f1b88:
    *param_1 = local_38;
    param_1[1] = param_2;
    param_1[2] = local_28;
  }
  return;
}
