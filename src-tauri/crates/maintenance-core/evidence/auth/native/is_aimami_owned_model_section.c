/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* is_aimami_owned_model_section @ 1001fc9ec requested 1001fc9ec */

/* codexmate_lib::core::relay::config_takeover::is_aimami_owned_model_section */

bool codexmate_lib::core::relay::config_takeover::is_aimami_owned_model_section(long *param_1,ulong param_2)

{
  bool bVar1;
  long lVar2;
  long local_38;
  long *local_30;
  ulong uStack_28;

  lVar2 = param_2 - 0x10;
  if (param_2 < 0x10) {
    if (param_2 < 9) {
      return false;
    }
  }
  else if (*param_1 == 0x72705f6c65646f6d && param_1[1] == 0x2e7372656469766f) {
    param_1 = param_1 + 2;
    goto LAB_1001fca70;
  }
  if (*param_1 != 0x73656c69666f7270 || (char)param_1[1] != '.') {
    return false;
  }
  lVar2 = param_2 - 9;
  param_1 = (long *)((long)param_1 + 9);
LAB_1001fca70:
  first_section_part(&local_38,param_1,lVar2);
  if (local_38 == -0x8000000000000000) {
    bVar1 = false;
  }
  else {
    if (uStack_28 == 0xc) {
      bVar1 = *local_30 == 0x725f696d616d6961 && (int)local_30[1] == 0x79616c65;
    }
    else if ((uStack_28 < 0xd) || (*local_30 != 0x725f696d616d6961 || *(long *)((long)local_30 + 5) != 0x5f79616c65725f69)) {
      if (uStack_28 == 6) {
        bVar1 = false;
        if ((int)*local_30 == 0x616d6961) {
          bVar1 = *(short *)((long)local_30 + 4) == 0x3169;
        }
      }
      else if (uStack_28 == 0x13) {
        bVar1 = (*local_30 == 0x635f696d616d6961 && local_30[1] == 0x756f725f7865646f) && *(long *)((long)local_30 + 0xb) == 0x726574756f725f78;
      }
      else {
        bVar1 = false;
      }
    }
    else {
      bVar1 = true;
    }
    if (local_38 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_30,local_38,1);
    }
  }
  return bVar1;
}
