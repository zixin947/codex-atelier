/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* serialize @ 10082afe0 requested 10082afe0 */

/* codexmate_lib::core::repository::_::<impl serde_core::ser::Serialize for codexmate_lib::core::repository::ApiConfig>::serialize */

long codexmate_lib::core::repository::ApiConfig>::serialize(undefined8 param_1,undefined8 *param_2)

{
  void *pvVar1;
  ulong uVar2;
  undefined8 *puVar3;
  long lVar4;
  long lVar5;
  long *plVar6;
  long lVar7;
  undefined2 local_50;
  undefined8 *local_48;

  plVar6 = (long *)*param_2;
  param_2[3] = param_2[3] + 1;
  *(undefined1 *)(param_2 + 4) = 0;
  lVar5 = plVar6[2];
  if (*plVar6 == lVar5) {
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar6,lVar5,1,1,1);
    lVar5 = plVar6[2];
  }
  *(undefined1 *)(plVar6[1] + lVar5) = 0x7b;
  plVar6[2] = lVar5 + 1;
  local_50 = 0x100;
  local_48 = param_2;
  lVar5 = serde_core::ser::SerializeMap::serialize_entry(&local_50,"usageRefreshEnabled",0x13,param_1);
  puVar3 = local_48;
  if (((lVar5 == 0) && ((local_50 & 1) == 0)) && (local_50._1_1_ != '\0')) {
    plVar6 = (long *)*local_48;
    lVar7 = local_48[3] + -1;
    local_48[3] = lVar7;
    if ((*(byte *)(local_48 + 4) & 1) == 0) {
      lVar4 = plVar6[2];
    }
    else {
      lVar4 = plVar6[2];
      if (*plVar6 == lVar4) {
        alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar6,lVar4,1,1,1);
        lVar4 = plVar6[2];
      }
      *(undefined1 *)(plVar6[1] + lVar4) = 10;
      lVar4 = lVar4 + 1;
      plVar6[2] = lVar4;
      if (lVar7 != 0) {
        pvVar1 = (void *)puVar3[1];
        uVar2 = puVar3[2];
        do {
          if ((ulong)(*plVar6 - lVar4) < uVar2) {
            alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar6,lVar4,uVar2,1,1);
            lVar4 = plVar6[2];
          }
          _memcpy((void *)(plVar6[1] + lVar4),pvVar1,uVar2);
          lVar4 = lVar4 + uVar2;
          plVar6[2] = lVar4;
          lVar7 = lVar7 + -1;
        } while (lVar7 != 0);
      }
    }
    if (*plVar6 == lVar4) {
      alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar6,lVar4,1,1,1);
      lVar4 = plVar6[2];
    }
    *(undefined1 *)(plVar6[1] + lVar4) = 0x7d;
    plVar6[2] = lVar4 + 1;
  }
  return lVar5;
}
