/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* from_trait @ 1001dac90 requested 1001dac90 */

/* serde_json::de::from_trait */

void serde_json::de::from_trait(long *param_1,long *param_2)

{
  long lVar1;
  long lVar2;
  long lVar3;
  undefined8 *puVar4;
  long lVar5;
  long local_88;
  undefined8 local_80;
  undefined8 local_78;
  long local_70;
  ulong uStack_68;
  ulong local_60;
  long lStack_58;
  undefined1 local_50;
  undefined8 local_48 [3];

  uStack_68 = param_2[1];
  local_70 = *param_2;
  lStack_58 = param_2[3];
  local_60 = param_2[2];
  local_88 = 0;
  local_80 = 1;
  local_78 = 0;
  local_50 = 0x80;
  <&mut_serde_json::de::Deserializer<R>as_serde_core::de::Deserializer>::deserialize_struct(param_1,&local_88);
  if ((*param_1 != -0x8000000000000000) && (local_60 < uStack_68)) {
    lVar1 = *param_1;
    lVar2 = param_1[1];
    lVar5 = param_1[2];
    do {
      if (0x20 < *(byte *)(local_70 + local_60) || (1L << ((ulong)*(byte *)(local_70 + local_60) & 0x3f) & 0x100002600U) == 0) {
        local_48[0] = 0x16;
        lVar3 = Deserializer<R>::peek_error(&local_88,local_48);
        *param_1 = -0x8000000000000000;
        param_1[1] = lVar3;
        if (lVar5 != 0) {
          puVar4 = (undefined8 *)(lVar2 + 0x20);
          do {
            if (puVar4[-4] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar4[-3],puVar4[-4],1);
            }
            if (puVar4[-1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar4,puVar4[-1],1);
            }
            if (puVar4[2] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar4[3],puVar4[2],1);
            }
            puVar4 = puVar4 + 10;
            lVar5 = lVar5 + -1;
          } while (lVar5 != 0);
        }
        if (lVar1 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar2,lVar1 * 0x50,8);
        }
        break;
      }
      local_60 = local_60 + 1;
    } while (uStack_68 != local_60);
  }
  if (local_88 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_80,local_88,1);
  }
  return;
}
