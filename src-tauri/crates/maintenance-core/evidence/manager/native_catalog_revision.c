/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* revision @ 100678ae0 requested 100678ae0 */

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* codexmate_lib::core::relay::native_catalog::revision */

void codexmate_lib::core::relay::native_catalog::revision(undefined8 param_1)

{
  void **ppvVar1;
  undefined8 *puVar2;
  void **ppvVar3;
  long lVar4;
  ulong uVar5;
  ulong uVar6;
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  long local_188;
  long local_180;
  long lStack_178;
  undefined8 **local_170;
  code *pcStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  ulong local_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  ulong uStack_110;
  char local_108;
  undefined7 uStack_107;
  void **local_100;
  undefined8 *puStack_f8;
  ulong local_f0;
  undefined8 uStack_e8;
  ulong local_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  ulong local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 local_60;
  ulong uStack_58;

  codex_catalog::build_relay_models(&local_188);
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  puStack_f8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x80,1);
  if ((void **)puStack_f8 == (void **)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x80);
  }
  local_170 = &local_100;
  local_100 = (void **)0x80;
  local_f0 = 0;
  ppvVar3 = (void **)serde_core::ser::Serializer::collect_seq(&local_170,&local_188);
  uVar6 = local_f0;
  puVar2 = puStack_f8;
  ppvVar1 = local_100;
  if (ppvVar3 == (void **)0x0) {
    ppvVar3 = (void **)puStack_f8;
    if (local_100 != (void **)0x8000000000000000) {
      local_108 = 0;
      uStack_110 = 0;
      local_118 = 0;
      uStack_120 = 0;
      local_128 = 0;
      uStack_130 = 0;
      local_138 = 0;
      uStack_140 = 0;
      uStack_148 = 0;
      pcStack_168 = DAT_1013a78c0._8_8_;
      local_170 = (undefined8 **)DAT_1013a78c0;
      uStack_158 = DAT_1013a78d0._8_8_;
      uStack_160 = (undefined8)DAT_1013a78d0;
      local_150 = 0;
      if (0x3f < local_f0) {
        local_150 = local_f0 >> 6;
        sha2::sha256::compress256(&local_170,puStack_f8);
        ppvVar3 = (void **)((long)puVar2 + (uVar6 & 0x7fffffffffffffc0));
        uVar6 = uVar6 & 0x3f;
      }
      _memcpy(&uStack_148,ppvVar3,uVar6);
      local_108 = (char)uVar6;
      local_98 = CONCAT71(uStack_107,local_108);
      uStack_d8 = uStack_148;
      local_e0 = local_150;
      uStack_c8 = local_138;
      uStack_d0 = uStack_140;
      uStack_b8 = local_128;
      local_c0 = uStack_130;
      uStack_a8 = local_118;
      local_b0 = uStack_120;
      local_a0 = uStack_110;
      puStack_f8 = (undefined8 *)pcStack_168;
      local_100 = local_170;
      uStack_e8 = uStack_158;
      local_f0 = uStack_160;
      uVar6 = uVar6 & 0xff;
      uVar5 = local_150 << 9 | uVar6 << 3;
      uVar5 = (uVar5 & 0xff00ff00ff00ff00) >> 8 | (uVar5 & 0xff00ff00ff00ff) << 8;
      uVar5 = (uVar5 & 0xffff0000ffff0000) >> 0x10 | (uVar5 & 0xffff0000ffff) << 0x10;
      uVar5 = uVar5 >> 0x20 | uVar5 << 0x20;
      *(undefined1 *)((long)&uStack_d8 + uVar6) = 0x80;
      if ((local_108 == '?') || (_bzero((void *)((long)&uStack_d8 + uVar6 + 1),uVar6 ^ 0x3f), (uVar6 ^ 0x38) < 8)) {
        sha2::sha256::compress256(&local_100,&uStack_d8,1);
        uStack_78 = 0;
        local_80 = 0;
        uStack_68 = 0;
        uStack_70 = 0;
        uStack_88 = 0;
        local_90 = 0;
        local_60 = 0;
        uStack_58 = uVar5;
        sha2::sha256::compress256(&local_100,&local_90,1);
      }
      else {
        local_a0 = uVar5;
        sha2::sha256::compress256(&local_100,&uStack_d8,1);
      }
      auVar7._8_8_ = puStack_f8;
      auVar7._0_8_ = local_100;
      auVar8._8_8_ = uStack_e8;
      auVar8._0_8_ = local_f0;
      auVar7 = NEON_rev32(auVar7,1);
      auVar8 = NEON_rev32(auVar8,1);
      puStack_f8 = auVar7._8_8_;
      local_100 = auVar7._0_8_;
      uStack_e8 = auVar8._8_8_;
      local_f0 = auVar8._0_8_;
      if (ppvVar1 != (void **)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar2,ppvVar1,1);
      }
      local_170 = &local_100;
      pcStack_168 = generic_array::GenericArray<u8,T>>::fmt;
      alloc::fmt::format::format_inner(param_1,s__10151cfe6,&local_170);
      lStack_178 = lStack_178 + 1;
      lVar4 = local_180;
      while (lStack_178 = lStack_178 + -1, lStack_178 != 0) {
        ::core::ptr::drop_in_place<serde_json::value::Value>(lVar4);
        lVar4 = lVar4 + 0x20;
      }
      if (local_188 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_180,local_188 << 5,8);
      }
      return;
    }
  }
  else if (local_100 != (void **)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_f8,local_100,1);
  }
  local_100 = ppvVar3;
                    /* WARNING: Subroutine does not return */
  ::core::result::unwrap_failed
            (&DAT_1013a2e66,0x15,&local_100,&PTR_drop_in_place<serde_json::error::Error>_101827be0,&PTR_s_src_core_relay_native_catalog_rs_101826d60);
}
