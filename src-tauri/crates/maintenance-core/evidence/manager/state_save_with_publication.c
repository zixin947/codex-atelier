/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* save_with_publication @ 100207780 requested 100207780 */

/* codexmate_lib::core::relay::storage::save_with_publication */

void codexmate_lib::core::relay::storage::save_with_publication(long *param_1,long param_2,long param_3)

{
  long lVar1;
  undefined8 *puVar2;
  long lVar3;
  undefined1 auVar4 [16];
  long local_90;
  long lStack_88;
  long local_80;
  long lStack_78;
  long local_70;
  long lStack_68;
  long lStack_60;
  long lStack_58;
  long local_50;
  long lStack_48;
  long lStack_40;
  long lStack_38;

  validate_credentials(&local_90);
  if (local_90 == 0xb) {
    if ((*(int *)(param_3 + 0x168) < 0x11) && ((0xe < *(int *)(param_3 + 0x168) || ((*(byte *)(param_3 + 0x16d) & 1) == 0)))) {
      lVar3 = platform::paths::CodexPaths::ensure_directories(param_2);
      if (lVar3 == 0) {
        serde_json::ser::to_vec_pretty(&local_90,param_3);
        lVar1 = lStack_88;
        lVar3 = local_90;
        if (local_90 == -0x8000000000000000) {
          *param_1 = 3;
          param_1[1] = lStack_88;
        }
        else {
          auVar4 = atomic_write::write_atomic_report(*(undefined8 *)(param_2 + 0x260),*(undefined8 *)(param_2 + 0x268),lStack_88,local_80);
          if ((auVar4._0_8_ & 1) == 0) {
            *param_1 = 0xb;
            param_1[1] = auVar4._8_8_;
          }
          else {
            save_with_publication::__closure__(&local_90);
            param_1[5] = lStack_68;
            param_1[4] = local_70;
            param_1[7] = lStack_58;
            param_1[6] = lStack_60;
            param_1[9] = lStack_48;
            param_1[8] = local_50;
            param_1[0xb] = lStack_38;
            param_1[10] = lStack_40;
            param_1[1] = lStack_88;
            *param_1 = local_90;
            param_1[3] = lStack_78;
            param_1[2] = local_80;
          }
          if (lVar3 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar1,lVar3,1);
            return;
          }
        }
      }
      else {
        *param_1 = 2;
        param_1[1] = lVar3;
      }
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar2 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x3e,1);
      if (puVar2 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x3e);
      }
      puVar2[1] = 0x6568637320657461;
      *puVar2 = 0x74732079616c6572;
      puVar2[3] = 0x7365727065722074;
      puVar2[2] = 0x6f6e6e616320616d;
      puVar2[5] = 0x20726574756f7220;
      puVar2[4] = 0x7369687420746e65;
      *(undefined8 *)((long)puVar2 + 0x36) = 0x7470696563657220;
      *(undefined8 *)((long)puVar2 + 0x2e) = 0x74696d6d6f632072;
      param_1[1] = 0x3e;
      *param_1 = 9;
      param_1[2] = (long)puVar2;
      param_1[3] = 0x3e;
    }
  }
  else {
    param_1[5] = lStack_68;
    param_1[4] = local_70;
    param_1[7] = lStack_58;
    param_1[6] = lStack_60;
    param_1[9] = lStack_48;
    param_1[8] = local_50;
    param_1[0xb] = lStack_38;
    param_1[10] = lStack_40;
    param_1[1] = lStack_88;
    *param_1 = local_90;
    param_1[3] = lStack_78;
    param_1[2] = local_80;
  }
  return;
}
