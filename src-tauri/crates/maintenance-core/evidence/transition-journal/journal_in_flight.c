/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* in_flight @ 100792720 requested 100792720 */

/* codexmate_lib::core::relay::transition_journal::in_flight */

void codexmate_lib::core::relay::transition_journal::in_flight(long *param_1)

{
  long local_78;
  long *plStack_70;
  long local_68;
  ulong uStack_60;
  long local_58;
  long lStack_50;
  long local_48;
  long lStack_40;
  long local_38;
  
  read(&local_78);
  if (local_78 != -0x8000000000000000) {
    if (local_68 < 0xb) {
      if (local_68 == 8) {
        if (*plStack_70 == 0x676e697972746572) goto LAB_10079283c;
      }
      else if ((local_68 == 10) && (*plStack_70 == 0x697474696d6d6f63 && (short)plStack_70[1] == 0x676e)) {
LAB_10079283c:
        param_1[5] = lStack_50;
        param_1[4] = local_58;
        param_1[7] = lStack_40;
        param_1[6] = local_48;
        param_1[8] = local_38;
        param_1[1] = (long)plStack_70;
        *param_1 = local_78;
        param_1[3] = uStack_60;
        param_1[2] = local_68;
        return;
      }
    }
    else if (local_68 == 0xb) {
      if (*plStack_70 == 0x6c69636e6f636572 && *(long *)((long)plStack_70 + 3) == 0x676e696c69636e6f) goto LAB_10079283c;
    }
    else if ((local_68 == 0xe) && (*plStack_70 == 0x676e6970706f7473 && *(long *)((long)plStack_70 + 6) == 0x7865646f635f676e)) goto LAB_10079283c;
    if (local_78 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_70,local_78,1);
    }
    if ((uStack_60 & 0x7fffffffffffffff) != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_58,uStack_60,1);
    }
  }
  *param_1 = -0x8000000000000000;
  return;
}

