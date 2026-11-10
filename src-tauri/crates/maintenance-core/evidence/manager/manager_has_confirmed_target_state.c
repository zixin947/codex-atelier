/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* has_confirmed_target_state @ 10068273c requested 10068273c */

/* codexmate_lib::core::relay::manager::RelayManager::has_confirmed_target_state */

bool codexmate_lib::core::relay::manager::RelayManager::has_confirmed_target_state(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  bool bVar1;
  long lVar2;
  undefined8 *puVar3;
  long local_90;
  byte local_88;
  undefined7 uStack_87;
  long lStack_80;

  transition_journal::commit_pending(&local_90,param_2);
  if ((local_90 == 0xb) && ((local_88 & 1) == 0)) {
    verify_target_state_at_with_depth(&local_90,param_1,param_2,param_3,0);
    bVar1 = lStack_80 == 0;
    if (lStack_80 != 0) {
      puVar3 = (undefined8 *)(CONCAT71(uStack_87,local_88) + 8);
      lVar2 = lStack_80;
      do {
        if (puVar3[-1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar3,puVar3[-1],1);
        }
        puVar3 = puVar3 + 4;
        lVar2 = lVar2 + -1;
      } while (lVar2 != 0);
    }
    if (local_90 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(CONCAT71(uStack_87,local_88),local_90 << 5,8);
    }
  }
  else {
    if (local_90 != 0xb) {
      ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_90);
    }
    bVar1 = false;
  }
  return bVar1;
}
