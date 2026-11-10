/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* ensure_pending_commit_matches @ 100684800 requested 100684800 */

/* codexmate_lib::core::relay::manager::RelayManager::ensure_pending_commit_matches */

void codexmate_lib::core::relay::manager::RelayManager::ensure_pending_commit_matches(long *param_1,long param_2,undefined8 param_3)

{
  long lVar1;
  long *plVar2;
  long lVar3;
  long lVar4;
  long local_a0;
  long lStack_98;
  long *local_90;
  long lStack_88;
  long local_80;
  long lStack_78;
  long local_70;
  long lStack_68;
  long local_60;
  long lStack_58;
  long local_50;
  long lStack_48;

  transition_journal::read_checked(&local_a0,param_2 + 0x10);
  lVar4 = lStack_78;
  lVar3 = local_80;
  plVar2 = local_90;
  lVar1 = lStack_98;
  if (local_a0 == 0xb) {
    if (lStack_98 != -0x8000000000000000) {
      if (((lStack_88 == 10) && (*local_90 == 0x697474696d6d6f63 && (short)local_90[1] == 0x676e)) &&
         (storage::verify_router_commit(&local_a0,param_2 + 0x10,param_3), local_a0 != 0xb)) {
        param_1[5] = lStack_78;
        param_1[4] = local_80;
        param_1[7] = lStack_68;
        param_1[6] = local_70;
        param_1[9] = lStack_58;
        param_1[8] = local_60;
        param_1[0xb] = lStack_48;
        param_1[10] = local_50;
        param_1[1] = lStack_98;
        *param_1 = local_a0;
        param_1[3] = lStack_88;
        param_1[2] = (long)local_90;
        if (lVar1 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar2,lVar1,1);
        }
        if (lVar3 == -0x8000000000000000) {
          return;
        }
        if (lVar3 == 0) {
          return;
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar4,lVar3,1);
        return;
      }
      if (lVar1 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar2,lVar1,1);
      }
      if ((lVar3 != -0x8000000000000000) && (lVar3 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar4,lVar3,1);
      }
    }
    *param_1 = 0xb;
  }
  else {
    param_1[3] = lStack_88;
    param_1[2] = (long)local_90;
    param_1[5] = lStack_78;
    param_1[4] = local_80;
    param_1[7] = lStack_68;
    param_1[6] = local_70;
    param_1[9] = lStack_58;
    param_1[8] = local_60;
    param_1[0xb] = lStack_48;
    param_1[10] = local_50;
    *param_1 = local_a0;
    param_1[1] = lStack_98;
  }
  return;
}
