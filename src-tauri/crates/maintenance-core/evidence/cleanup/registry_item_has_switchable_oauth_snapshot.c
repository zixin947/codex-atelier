/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* registry_item_has_switchable_oauth_snapshot @ 100832a08 requested 100832a08 */

/* codexmate_lib::core::repository::registry_item_has_switchable_oauth_snapshot */

undefined1 codexmate_lib::core::repository::registry_item_has_switchable_oauth_snapshot(long param_1,undefined8 param_2,undefined8 param_3)

{
  long *plVar1;
  long lVar2;
  long lVar3;
  void *pvVar4;
  size_t sVar5;
  void *pvVar6;
  undefined8 uVar7;
  void **local_170;
  code *pcStack_168;
  long local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  void **local_110;
  code *pcStack_108;
  long lStack_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  void *local_60;
  size_t sStack_58;

  lVar2 = *(long *)(param_1 + 200);
  lVar3 = *(long *)(param_1 + 0xd0);
  ::core::str::pattern::StrSearcher::new
            (&local_110,lVar2,lVar3,
             "::organizationstitlechatgpt_account_user_idlast_refresh is outside the supported timestamp rangelast_refresh must be a timestamp string or number"
             ,2);
  <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_170,&local_110);
  if (((((int)local_170 != 1) || (lVar3 - local_160 != 0x24)) ||
      (plVar1 = (long *)(lVar2 + local_160),
      (((*plVar1 != 0x3030303030303030 || plVar1[1] != 0x30342d303030302d) || (plVar1[2] != 0x2d303030382d3030 || plVar1[3] != 0x3030303030303030)) ||
      (int)plVar1[4] != 0x31303030) || pcStack_168 < (code *)0x7)) ||
     (*(int *)(pcStack_168 + lVar2 + -7) != 0x6c6e752d || *(int *)(pcStack_168 + lVar2 + -4) != 0x6b636f6c)) {
    sVar5 = *(size_t *)(param_1 + 0xe8);
    if ((long)sVar5 < 0) {
      uVar7 = 0;
LAB_100832b04:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(uVar7,sVar5);
    }
    pvVar6 = *(void **)(param_1 + 0xe0);
    if (sVar5 == 0) {
      pvVar4 = (void *)0x1;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar7 = 1;
      pvVar4 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar5,1);
      if (pvVar4 == (void *)0x0) goto LAB_100832b04;
    }
    _memcpy(pvVar4,pvVar6,sVar5);
    auth::ensure_managed_account_snapshot_location(&local_110,pvVar4,sVar5,param_2,param_3);
    if (local_110 == (void **)&DAT_0000000b) {
      std::sys::fs::symlink_metadata(&local_110,pvVar4,sVar5);
      if ((int)local_110 == 1) {
        local_170 = (void **)0x2;
        pcStack_168 = pcStack_108;
      }
      else {
        if (((ulong)pcStack_108 & 0xf00000000000) == 0x800000000000) goto LAB_100832bd4;
        local_110 = &local_60;
        pcStack_108 = <std::path::Display_as_core::fmt::Display>::fmt;
        local_60 = pvVar4;
        sStack_58 = sVar5;
        alloc::fmt::format::format_inner((ulong)&local_170 | 8,_anon_dc5b8adae996cb58fd4173e320c2d1cf_283,&local_110);
        local_170 = (void **)&DAT_00000009;
      }
    }
    else {
      uStack_148 = uStack_e8;
      local_150 = local_f0;
      uStack_138 = uStack_d8;
      uStack_140 = uStack_e0;
      uStack_128 = uStack_c8;
      local_130 = local_d0;
      uStack_118 = uStack_b8;
      uStack_120 = uStack_c0;
      pcStack_168 = pcStack_108;
      local_170 = local_110;
      uStack_158 = uStack_f8;
      local_160 = lStack_100;
      if (local_110 == (void **)&DAT_0000000b) {
LAB_100832bd4:
        load_switchable_account_snapshot(&local_110,pvVar4,sVar5);
        if (sVar5 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar4,sVar5,1);
        }
        if (local_110 == (void **)0x2) {
          ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&pcStack_108);
          return 0;
        }
        ::core::ptr::drop_in_place<codexmate_lib::core::auth::AuthSnapshot>(&local_110);
        return 1;
      }
    }
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_170);
    if (sVar5 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar4,sVar5,1);
    }
  }
  return 0;
}
