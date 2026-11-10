/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* ensure_managed_account_snapshot_location @ 1001f1bb8 requested 1001f1bb8 */

/* codexmate_lib::core::auth::ensure_managed_account_snapshot_location */

void codexmate_lib::core::auth::ensure_managed_account_snapshot_location
               (undefined8 *param_1,void *param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5)

{
  undefined8 uVar1;
  void **ppvVar2;
  void **ppvVar3;
  code *pcVar4;
  code *pcVar5;
  int iVar6;
  long lVar7;
  char *pcVar8;
  undefined8 uVar9;
  void *local_1d0;
  size_t sStack_1c8;
  char local_1c0;
  undefined8 local_1bf;
  undefined8 uStack_1b7;
  undefined8 local_1af;
  undefined2 local_198;
  undefined1 local_196;
  void **local_190;
  size_t sStack_188;
  char local_180;
  undefined8 local_17f;
  undefined8 uStack_177;
  undefined8 local_16f;
  undefined2 local_158;
  undefined1 local_156;
  void *local_150;
  size_t sStack_148;
  char local_140;
  undefined8 local_13f;
  undefined8 uStack_137;
  undefined8 local_12f;
  undefined2 local_118;
  undefined1 local_116;
  void **local_110;
  code *pcStack_108;
  char local_100;
  undefined7 uStack_ff;
  undefined1 uStack_f8;
  undefined8 uStack_f7;
  undefined8 local_ef;
  undefined2 local_d8;
  undefined1 local_d6;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;

  lVar7 = std::path::Path::parent();
  if (lVar7 == 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    pcVar8 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1e,1);
    if (pcVar8 != (char *)0x0) {
      builtin_strncpy(pcVar8,"Account snapshot has no parent",0x1e);
      *param_1 = 9;
      param_1[1] = 0x1e;
      param_1[2] = pcVar8;
      param_1[3] = 0x1e;
      return;
    }
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x1e);
  }
  std::sys::fs::canonicalize(&local_110);
  pcVar4 = pcStack_108;
  ppvVar2 = local_110;
  if (local_110 == (void **)&DAT_8000000000000000) {
    *param_1 = 2;
    param_1[1] = pcStack_108;
    return;
  }
  uVar9 = CONCAT71(uStack_ff,local_100);
  std::sys::fs::canonicalize(&local_110,param_4,param_5);
  pcVar5 = pcStack_108;
  ppvVar3 = local_110;
  if (local_110 == (void **)&DAT_8000000000000000) {
    *param_1 = 2;
    param_1[1] = pcStack_108;
    if (ppvVar2 == (void **)0x0) {
      return;
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar4,ppvVar2,1);
    return;
  }
  uVar1 = CONCAT71(uStack_ff,local_100);
  std::path::Path::components(&local_1d0,pcVar4,uVar9);
  std::path::Path::components(&local_190,pcVar5,uVar1);
  if (sStack_1c8 == sStack_188) {
    if (((((char)local_198 != (char)local_158) || (local_198._1_1_ != '\x02')) || (local_158._1_1_ != '\x02')) ||
       (iVar6 = _memcmp(local_1d0,local_190,sStack_1c8), iVar6 != 0)) goto LAB_1001f1d10;
  }
  else {
LAB_1001f1d10:
    if (local_1c0 != '\x06') {
      uStack_c8 = uStack_1b7;
      local_d0 = local_1bf;
      uStack_c0 = local_1af;
    }
    uStack_137 = uStack_c8;
    local_13f = local_d0;
    local_150 = local_1d0;
    sStack_148 = sStack_1c8;
    local_140 = local_1c0;
    local_12f = uStack_c0;
    local_118 = local_198;
    local_116 = local_196;
    if (local_180 != '\x06') {
      uStack_98 = uStack_177;
      local_a0 = local_17f;
      uStack_90 = local_16f;
    }
    local_110 = local_190;
    pcStack_108 = (code *)sStack_188;
    local_100 = local_180;
    uStack_f7 = uStack_98;
    uStack_ff = (undefined7)local_a0;
    uStack_f8 = (undefined1)((ulong)local_a0 >> 0x38);
    local_ef = uStack_90;
    local_d8 = local_158;
    local_d6 = local_156;
    iVar6 = ::core::iter::traits::iterator::Iterator::eq_by(&local_150,&local_110);
    if (iVar6 == 0) {
      if (ppvVar3 != (void **)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar5,ppvVar3,1);
      }
      if (ppvVar2 != (void **)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar4,ppvVar2,1);
      }
      local_110 = &local_150;
      pcStack_108 = <std::path::Display_as_core::fmt::Display>::fmt;
      local_150 = param_2;
      sStack_148 = param_3;
      alloc::fmt::format::format_inner(param_1 + 1,s_<Account_snapshot_is_outside_the_10151d024,&local_110);
      uVar9 = 9;
      goto LAB_1001f1e44;
    }
  }
  if (ppvVar3 != (void **)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar5,ppvVar3,1);
  }
  if (ppvVar2 != (void **)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar4,ppvVar2,1);
  }
  uVar9 = 0xb;
LAB_1001f1e44:
  *param_1 = uVar9;
  return;
}
