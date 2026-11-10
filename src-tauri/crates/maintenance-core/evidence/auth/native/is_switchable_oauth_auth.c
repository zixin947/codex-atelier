/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* is_switchable_oauth_auth @ 1001efc84 requested 1001efc84 */

/* codexmate_lib::core::auth::is_switchable_oauth_auth */

bool codexmate_lib::core::auth::is_switchable_oauth_auth(long param_1)

{
  ulong uVar1;
  long lVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 local_78 [16];
  undefined8 local_68;
  long lStack_60;
  undefined8 local_58;
  undefined1 local_50;
  uint local_48 [6];

  uVar1 = is_aimami_managed_virtual_auth();
  if ((uVar1 & 1) == 0) {
    uVar1 = is_api_key_auth(param_1);
    if ((uVar1 & 1) == 0) {
      if (*(long *)(param_1 + 0x30) != -0x8000000000000000) {
        auVar3 = ::core::str::<impl_str>::trim_matches(*(undefined8 *)(param_1 + 0x38),*(undefined8 *)(param_1 + 0x40));
        auVar4._8_8_ = local_78._8_8_;
        auVar4._0_8_ = local_78._0_8_;
        lVar2 = auVar3._8_8_;
        if ((lVar2 != 0) && ((lVar2 == 1 || (local_78 = auVar4, *auVar3._0_8_ != 0x5f5f)))) {
          local_68 = 0;
          local_58 = 0x2e0000002e;
          local_50 = 1;
          uVar1 = 0xffffffffffffffff;
          lStack_60 = lVar2;
          local_78 = auVar3;
          do {
            <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(local_48,local_78);
            uVar1 = uVar1 + 1;
          } while ((local_48[0] & 1) != 0);
          if (1 < uVar1) {
            return true;
          }
        }
      }
      if (*(long *)(param_1 + 0x48) != -0x8000000000000000) {
        auVar4 = ::core::str::<impl_str>::trim_matches(*(undefined8 *)(param_1 + 0x50),*(undefined8 *)(param_1 + 0x58));
        lStack_60 = auVar4._8_8_;
        if ((lStack_60 != 0) && ((lStack_60 == 1 || (*auVar4._0_8_ != 0x5f5f)))) {
          local_68 = 0;
          local_58 = 0x2e0000002e;
          local_50 = 1;
          uVar1 = 0xffffffffffffffff;
          local_78 = auVar4;
          do {
            <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(local_48,local_78);
            uVar1 = uVar1 + 1;
          } while ((local_48[0] & 1) != 0);
          return 1 < uVar1;
        }
      }
    }
  }
  return false;
}
