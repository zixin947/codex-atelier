/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* deserialize @ 1007aae04 requested 1007aae04 */

/* codexmate_lib::core::models::_::<impl serde_core::de::Deserialize for codexmate_lib::core::models::UsageSource>::deserialize */

void codexmate_lib::core::models::UsageSource>::deserialize(undefined2 *param_1,long param_2)

{
  byte bVar1;
  char cVar2;
  undefined8 uVar3;
  ulong uVar4;
  undefined1 uVar5;
  undefined8 local_48;
  long local_40;

  uVar4 = *(ulong *)(param_2 + 0x28);
  if (uVar4 < *(ulong *)(param_2 + 0x20)) {
    do {
      bVar1 = *(byte *)(*(long *)(param_2 + 0x18) + uVar4);
      if (0x22 < bVar1) {
LAB_1007aaeb4:
        if (bVar1 != 0x7b) {
          local_48 = 10;
          goto LAB_1007aae68;
        }
        cVar2 = *(char *)(param_2 + 0x48) + -1;
        *(char *)(param_2 + 0x48) = cVar2;
        if (cVar2 == '\0') {
          local_48 = 0x18;
          goto LAB_1007aae68;
        }
        *(ulong *)(param_2 + 0x28) = uVar4 + 1;
        <core::marker::PhantomData<T>as_serde_core::de::DeserializeSeed>::deserialize(&local_48,param_2);
        if ((char)local_48 == '\x01') goto LAB_1007aaf8c;
        uVar4 = *(ulong *)(param_2 + 0x28);
        if (*(ulong *)(param_2 + 0x20) <= uVar4) goto LAB_1007aaf58;
        goto LAB_1007aaf30;
      }
      if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) {
        if ((ulong)bVar1 != 0x22) goto LAB_1007aaeb4;
        <core::marker::PhantomData<T>as_serde_core::de::DeserializeSeed>::deserialize(&local_48,param_2);
        if ((char)local_48 != '\x01') {
          if (local_48._1_1_ != '\0') {
            *(undefined1 *)((long)param_1 + 1) = 1;
            *(undefined1 *)param_1 = 0;
            return;
          }
          *param_1 = 0;
          return;
        }
        *(long *)(param_1 + 4) = local_40;
        goto LAB_1007aaea8;
      }
      uVar4 = uVar4 + 1;
      *(ulong *)(param_2 + 0x28) = uVar4;
    } while (*(ulong *)(param_2 + 0x20) != uVar4);
  }
  local_48 = 5;
LAB_1007aae68:
  uVar3 = serde_json::de::Deserializer<R>::peek_error(param_2,&local_48);
  *(undefined8 *)(param_1 + 4) = uVar3;
LAB_1007aaea8:
  *(undefined1 *)param_1 = 1;
  return;
LAB_1007aaf30:
  bVar1 = *(byte *)(*(long *)(param_2 + 0x18) + uVar4);
  if (0x3a < bVar1) {
LAB_1007ab060:
    local_48 = 6;
    goto LAB_1007aaf5c;
  }
  if ((1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) {
    if ((ulong)bVar1 != 0x3a) goto LAB_1007ab060;
    *(ulong *)(param_2 + 0x28) = uVar4 + 1;
    local_40 = <serde_json::de::VariantAccess<R>as_serde_core::de::VariantAccess>::unit_variant(param_2);
    if (local_40 != 0) goto LAB_1007aaf8c;
    *(char *)((long)param_1 + 1) = local_48._1_1_;
    *(char *)(param_2 + 0x48) = *(char *)(param_2 + 0x48) + '\x01';
    uVar4 = *(ulong *)(param_2 + 0x28);
    if (*(ulong *)(param_2 + 0x20) <= uVar4) goto LAB_1007ab004;
    goto LAB_1007aafdc;
  }
  uVar4 = uVar4 + 1;
  *(ulong *)(param_2 + 0x28) = uVar4;
  if (*(ulong *)(param_2 + 0x20) == uVar4) {
LAB_1007aaf58:
    local_48 = 3;
LAB_1007aaf5c:
    local_40 = serde_json::de::Deserializer<R>::peek_error(param_2,&local_48);
LAB_1007aaf8c:
    *(long *)(param_1 + 4) = local_40;
    *(undefined1 *)param_1 = 1;
    *(char *)(param_2 + 0x48) = *(char *)(param_2 + 0x48) + '\x01';
    return;
  }
  goto LAB_1007aaf30;
  while( true ) {
    uVar4 = uVar4 + 1;
    *(ulong *)(param_2 + 0x28) = uVar4;
    if (*(ulong *)(param_2 + 0x20) == uVar4) break;
LAB_1007aafdc:
    bVar1 = *(byte *)(*(long *)(param_2 + 0x18) + uVar4);
    if (0x20 < bVar1 || (1L << ((ulong)bVar1 & 0x3f) & 0x100002600U) == 0) {
      if (bVar1 != 0x7d) {
        local_48 = 10;
        goto LAB_1007ab008;
      }
      uVar5 = 0;
      *(ulong *)(param_2 + 0x28) = uVar4 + 1;
      goto LAB_1007ab038;
    }
  }
LAB_1007ab004:
  local_48 = 3;
LAB_1007ab008:
  uVar3 = serde_json::de::Deserializer<R>::error(param_2,&local_48);
  *(undefined8 *)(param_1 + 4) = uVar3;
  uVar5 = 1;
LAB_1007ab038:
  *(undefined1 *)param_1 = uVar5;
  return;
}
