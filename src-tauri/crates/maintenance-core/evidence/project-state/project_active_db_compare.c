/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* call_once @ 1008f349c requested 1008f349c */

/* WARNING: Type propagation algorithm not settling */
/* core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once */

uint &mut_F>::call_once(undefined8 param_1,long param_2,long param_3)

{
  uint uVar1;
  uint uVar2;
  ulong local_80;
  ulong uStack_78;
  long local_70;
  long local_68;
  uint local_60;
  byte local_5c;
  byte local_5b;
  byte local_5a;
  ulong local_50;
  ulong uStack_48;
  long local_40;
  long local_38;
  uint local_30;
  byte local_2c;
  byte local_2b;
  byte local_2a;

  codexmate_lib::platform::paths::CodexPaths::state_db_score
            (&local_80,*(undefined8 *)(param_2 + 8),*(undefined8 *)(param_2 + 0x10),*(undefined1 *)(param_2 + 0x18));
  codexmate_lib::platform::paths::CodexPaths::state_db_score
            (&local_50,*(undefined8 *)(param_3 + 8),*(undefined8 *)(param_3 + 0x10),*(undefined1 *)(param_3 + 0x18));
  if (local_5b == 0) {
    if (local_2b != 0) {
      return 0xff;
    }
    uVar1 = (uint)(!CARRY8(uStack_48,~uStack_78) && !CARRY8(uStack_48 + ~uStack_78,(ulong)(local_80 <= local_50)));
    if (!CARRY8(uStack_78,~uStack_48) && !CARRY8(uStack_78 + ~uStack_48,(ulong)(local_50 <= local_80))) {
      uVar1 = 0xffffffff;
    }
    if (uStack_78 == uStack_48 && local_80 == local_50) {
LAB_1008f35f8:
      uVar1 = (uint)(local_30 < local_60);
      if (local_60 < local_30) {
        uVar1 = 0xffffffff;
      }
    }
LAB_1008f3648:
    if ((uVar1 & 0xff) == 0) {
      uVar1 = (uint)local_5a - (uint)local_2a;
    }
  }
  else {
    if (local_5b == 1) {
      if (local_2b != 0) {
LAB_1008f3578:
        uVar2 = (uint)(!CARRY8(uStack_48,~uStack_78) && !CARRY8(uStack_48 + ~uStack_78,(ulong)(local_80 <= local_50)));
        if (!CARRY8(uStack_78,~uStack_48) && !CARRY8(uStack_78 + ~uStack_48,(ulong)(local_50 <= local_80))) {
          uVar2 = 0xffffffff;
        }
        uVar1 = (uint)(local_2b < local_5b);
        if (local_5b < local_2b) {
          uVar1 = 0xffffffff;
        }
        if (local_80 != local_50 || uStack_78 != uStack_48) {
          uVar1 = uVar2;
        }
        if ((uVar1 & 0xff) == 0) goto LAB_1008f35f8;
        goto LAB_1008f3648;
      }
    }
    else if (local_2b != 0) {
      if (local_2b == 2) {
        uVar1 = (uint)(local_30 < local_60);
        if (local_60 < local_30) {
          uVar1 = 0xffffffff;
        }
        if ((local_60 == local_30) && (uVar1 = (uint)(local_40 < local_70), local_70 < local_40)) {
          uVar1 = 0xffffffff;
        }
        if (((uVar1 & 0xff) == 0) && (uVar1 = (uint)(local_2c < local_5c), local_5c < local_2c)) {
          uVar1 = 0xffffffff;
        }
        if (((uVar1 & 0xff) == 0) && (uVar1 = (uint)(local_38 < local_68), local_68 < local_38)) {
          uVar1 = 0xffffffff;
        }
        if (((uVar1 & 0xff) == 0) &&
           (uVar1 = (uint)(!CARRY8(uStack_48,~uStack_78) && !CARRY8(uStack_48 + ~uStack_78,(ulong)(local_80 <= local_50))),
           !CARRY8(uStack_78,~uStack_48) && !CARRY8(uStack_78 + ~uStack_48,(ulong)(local_50 <= local_80)))) {
          uVar1 = 0xffffffff;
        }
        goto LAB_1008f3648;
      }
      goto LAB_1008f3578;
    }
    uVar1 = 1;
  }
  return uVar1;
}
