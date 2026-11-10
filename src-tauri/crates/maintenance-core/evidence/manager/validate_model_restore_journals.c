/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* validate_model_restore_journals @ 1008bb588 requested 1008bb588 */

/* codexmate_lib::core::relay::codex_thread_visibility::validate_model_restore_journals */

void codexmate_lib::core::relay::codex_thread_visibility::validate_model_restore_journals(long *param_1,long param_2)

{
  long lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long lVar4;
  undefined8 *puVar5;
  long lVar6;
  long local_b0;
  long lStack_a8;
  long lStack_a0;
  long local_98;
  long lStack_90;
  long lStack_88;
  long lStack_80;
  long lStack_78;
  long local_70;
  long lStack_68;
  long lStack_60;
  long local_58;

  uVar2 = *(undefined8 *)(param_2 + 0x248);
  uVar3 = *(undefined8 *)(param_2 + 0x250);
  model_fallback::load_model_restore_journal(&local_b0,uVar2,uVar3);
  lVar1 = lStack_a0;
  lVar4 = lStack_a8;
  if (local_b0 == 0xb) {
    if (local_98 != 0) {
      puVar5 = (undefined8 *)(lStack_a0 + 0x20);
      lVar6 = local_98;
      do {
        if (puVar5[-4] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar5[-3],puVar5[-4],1);
        }
        if (puVar5[-1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar5,puVar5[-1],1);
        }
        if (puVar5[2] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar5[3],puVar5[2],1);
        }
        puVar5 = puVar5 + 10;
        lVar6 = lVar6 + -1;
      } while (lVar6 != 0);
    }
    if (lVar4 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar1,lVar4 * 0x50,8);
    }
    load_no_account_model_restore_journal(&local_b0,uVar2,uVar3);
    if (local_b0 != 0xb) {
      param_1[5] = lStack_88;
      param_1[4] = lStack_90;
      param_1[7] = lStack_78;
      param_1[6] = lStack_80;
      param_1[9] = lStack_68;
      param_1[8] = local_70;
      param_1[0xb] = local_58;
      param_1[10] = lStack_60;
      param_1[1] = lStack_a8;
      *param_1 = local_b0;
      param_1[3] = local_98;
      param_1[2] = lStack_a0;
      return;
    }
    if (local_98 != 0) {
      puVar5 = (undefined8 *)(lStack_a0 + 0x20);
      lVar4 = local_98;
      do {
        if (puVar5[-4] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar5[-3],puVar5[-4],1);
        }
        lVar1 = puVar5[2];
        if (lVar1 == -0x8000000000000000 || lVar1 == 0) {
          lVar1 = puVar5[-1];
        }
        else {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar5[3],lVar1,1);
          lVar1 = puVar5[-1];
        }
        if (lVar1 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar5,lVar1,1);
        }
        puVar5 = puVar5 + 9;
        lVar4 = lVar4 + -1;
      } while (lVar4 != 0);
    }
    if (lStack_a8 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_a0,lStack_a8 * 0x48,8);
    }
    local_b0 = 0xb;
  }
  else {
    param_1[6] = lStack_80;
    param_1[5] = lStack_88;
    param_1[8] = local_70;
    param_1[7] = lStack_78;
    param_1[10] = lStack_60;
    param_1[9] = lStack_68;
    param_1[0xb] = local_58;
    param_1[2] = lStack_a0;
    param_1[1] = lStack_a8;
    param_1[4] = lStack_90;
    param_1[3] = local_98;
  }
  *param_1 = local_b0;
  return;
}
