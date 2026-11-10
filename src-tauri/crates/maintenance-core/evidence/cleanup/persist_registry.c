/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* persist_registry @ 100810df0 requested 100810df0 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::repository::Repository::persist_registry */

void codexmate_lib::core::repository::Repository::persist_registry(undefined8 *param_1,long param_2,long param_3,ulong param_4)

{
  undefined8 uVar1;
  int iVar2;
  long lVar3;
  undefined8 *******pppppppuVar4;
  undefined8 ******ppppppuVar5;
  undefined8 ******ppppppuVar6;
  undefined8 uVar7;
  code *pcVar8;
  undefined8 *puVar9;
  undefined1 auVar10 [16];
  undefined1 auVar11 [12];
  long local_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 *******local_180;
  undefined8 ******ppppppuStack_178;
  undefined *local_170;
  undefined8 ******local_168;
  code *pcStack_160;
  undefined8 local_158;
  undefined8 ******local_150;
  undefined8 uStack_148;
  char local_140;
  undefined4 local_13f;
  undefined3 uStack_13b;
  undefined8 local_138;
  int local_130;
  char local_12c;
  undefined8 ******local_120;
  code *pcStack_118;
  undefined8 local_110;
  undefined4 local_108;
  undefined3 uStack_104;
  undefined8 ******local_100;
  undefined8 uStack_f8;
  undefined8 *******local_e8;
  undefined8 ******ppppppuStack_e0;
  undefined *local_d8;
  undefined8 uStack_d0;
  undefined **local_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined *local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  char *local_98;
  undefined8 ******ppppppuStack_90;

  lVar3 = platform::paths::CodexPaths::ensure_directories(param_2);
  if (lVar3 != 0) {
    *param_1 = 2;
    param_1[1] = lVar3;
    return;
  }
  if ((param_4 & 1) != 0) {
    uVar7 = *(undefined8 *)(param_2 + 0xe0);
    uVar1 = *(undefined8 *)(param_2 + 0xe8);
    std::sys::fs::metadata(&local_e8,uVar7,uVar1);
    if ((int)local_e8 == 1) {
      if (((ulong)ppppppuStack_e0 & 3) != 1) goto LAB_100811000;
      ppppppuVar6 = (undefined8 ******)((long)ppppppuStack_e0 + -1);
      uVar7 = *ppppppuVar6;
      puVar9 = *(undefined8 **)((long)ppppppuStack_e0 + 7);
      if ((code *)*puVar9 != (code *)0x0) {
        (*(code *)*puVar9)(uVar7);
      }
      if (puVar9[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar7,puVar9[1],puVar9[2]);
      }
      uVar7 = 8;
      pppppppuVar4 = (undefined8 *******)(undefined8 ******)0x18;
    }
    else {
      auVar11 = std::time::SystemTime::now();
      local_180 = auVar11._0_8_;
      ppppppuStack_178 = (undefined8 ******)CONCAT44(ppppppuStack_178._4_4_,auVar11._8_4_);
      std::time::SystemTime::duration_since(&local_e8,&local_180,0,0);
      local_180 = (undefined8 *******)(undefined8 ******)0x0;
      if ((int)local_e8 == 0) {
        local_180 = (undefined8 *******)ppppppuStack_e0;
      }
      ppppppuStack_e0 = (undefined8 ******)isize>::fmt;
      local_e8 = &local_180;
      alloc::fmt::format::format_inner(&local_198,s_registry___json_10152cb01,&local_e8);
      std::path::Path::_join(&local_180,*(undefined8 *)(param_2 + 0x128),*(undefined8 *)(param_2 + 0x130),local_190,uStack_188);
      if (local_198 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_190,local_198,1);
      }
      ppppppuVar6 = ppppppuStack_178;
      auVar10 = std::sys::fs::copy(uVar7,uVar1,ppppppuStack_178,local_170);
      if ((auVar10._0_8_ & 1) != 0) {
        local_100 = auVar10._8_8_;
        if (1 < log::MAX_LOG_LEVEL_FILTER) {
          local_120 = &local_100;
          pcStack_118 = <std::io::error::Error_as_core::fmt::Display>::fmt;
          local_e8 = (undefined8 *******)&DAT_1013af968;
          ppppppuStack_e0 = (undefined8 ******)0x1f;
          local_d8 = &DAT_1013af968;
          uStack_d0 = 0x1f;
          local_c8 = &PTR_s_src_core_repository_rs_10182c898;
          log::__private_api::log(s___AiMaMi__accounts__registry_bac_10152cb13,&local_120,2,&local_e8);
        }
        if (((ulong)local_100 & 3) == 1) {
          pcVar8 = (code *)((long)local_100 + -1);
          uVar7 = *(undefined8 *)pcVar8;
          puVar9 = *(undefined8 **)((long)local_100 + 7);
          if ((code *)*puVar9 != (code *)0x0) {
            (*(code *)*puVar9)(uVar7);
          }
          if (puVar9[1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar7,puVar9[1],puVar9[2]);
          }
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar8,0x18,8);
        }
      }
      if (local_180 == (undefined8 *******)0x0) goto LAB_100811000;
      uVar7 = 1;
      pppppppuVar4 = local_180;
    }
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppuVar6,pppppppuVar4,uVar7);
  }
LAB_100811000:
  iVar2 = *(int *)(param_3 + 0x50);
  uVar7 = *(undefined8 *)(param_3 + 0x48);
  if (*(long *)(param_3 + 0x18) == -0x8000000000000000) {
    local_120 = (undefined8 ******)&DAT_8000000000000000;
  }
  else {
    <alloc::string::String_as_core::clone::Clone>::clone(&local_120);
  }
  <alloc::vec::Vec<T,A>as_core::clone::Clone>::clone(&local_e8,param_3);
  local_140 = *(char *)(param_3 + 0x40);
  if (local_140 != '\x02') {
    uStack_f8 = *(undefined8 *)(param_3 + 0x38);
    local_100 = *(undefined8 *******)(param_3 + 0x30);
    local_108._0_3_ = (undefined3)*(undefined4 *)(param_3 + 0x41);
    local_108._3_1_ = (undefined1)*(undefined4 *)(param_3 + 0x44);
    uStack_104 = (undefined3)((uint)*(undefined4 *)(param_3 + 0x44) >> 8);
  }
  local_12c = *(char *)(param_3 + 0x54);
  pcStack_160 = pcStack_118;
  local_168 = local_120;
  local_158 = local_110;
  ppppppuStack_178 = ppppppuStack_e0;
  local_180 = local_e8;
  local_170 = local_d8;
  uStack_148 = uStack_f8;
  local_150 = local_100;
  local_13f = local_108;
  uStack_13b = uStack_104;
  if (iVar2 < 5) {
    iVar2 = 4;
  }
  if (local_140 == '\x02') {
    local_150 = (undefined8 ******)0x0;
    uStack_148 = 0xa0000000f;
    local_140 = '\0';
  }
  if (local_12c == '\x02') {
    local_12c = '\x01';
  }
  local_138 = uVar7;
  local_130 = iVar2;
  serde_json::ser::to_vec_pretty(&local_e8,&local_180);
  ppppppuVar6 = ppppppuStack_e0;
  pppppppuVar4 = local_e8;
  if (local_e8 == (undefined8 *******)&DAT_8000000000000000) {
    local_100 = ppppppuStack_e0;
    if (1 < log::MAX_LOG_LEVEL_FILTER) {
      local_120 = &local_100;
      pcStack_118 = <serde_json::error::Error_as_core::fmt::Display>::fmt;
      local_b0 = &DAT_1013af968;
      uStack_a8 = 0x1f;
      ppppppuStack_90 = &local_120;
      local_98 = s___AiMaMi__accounts__registry_enc_10152cb6e;
      local_e8 = (undefined8 *******)0x0;
      ppppppuStack_e0 = (undefined8 ******)&DAT_1013af968;
      uStack_d0 = 0;
      local_d8 = (undefined *)0x1f;
      local_c8 = (undefined **)0x101524922;
      uStack_b8 = 2;
      local_c0 = 0x16;
      local_a0 = 0x9e600000001;
      <log::__private_api::GlobalLogger_as_log::Log>::log(&local_108,&local_e8);
    }
    *param_1 = 3;
    param_1[1] = local_100;
  }
  else {
    auVar10 = relay::atomic_write::write_atomic_private_report(*(undefined8 *)(param_2 + 0xe0),*(undefined8 *)(param_2 + 0xe8),ppppppuStack_e0,local_d8);
    ppppppuVar5 = auVar10._8_8_;
    if ((auVar10._0_8_ & 1) == 0) {
      relay::atomic_write::Publication::acknowledge(ppppppuVar5);
      *param_1 = 0xb;
    }
    else {
      local_100 = ppppppuVar5;
      if (1 < log::MAX_LOG_LEVEL_FILTER) {
        local_120 = &local_100;
        pcStack_118 = <std::io::error::Error_as_core::fmt::Display>::fmt;
        local_b0 = &DAT_1013af968;
        uStack_a8 = 0x1f;
        ppppppuStack_90 = &local_120;
        local_98 = s___AiMaMi__accounts__registry_wri_10152cb41;
        local_e8 = (undefined8 *******)0x0;
        ppppppuStack_e0 = (undefined8 ******)&DAT_1013af968;
        uStack_d0 = 0;
        local_d8 = (undefined *)0x1f;
        local_c8 = (undefined **)0x101524922;
        uStack_b8 = 2;
        local_c0 = 0x16;
        local_a0 = 0x9eb00000001;
        <log::__private_api::GlobalLogger_as_log::Log>::log(&local_108,&local_e8);
      }
      *param_1 = 2;
      param_1[1] = local_100;
    }
    if (pppppppuVar4 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppuVar6,pppppppuVar4,1);
    }
  }
  ::core::ptr::drop_in_place<codexmate_lib::core::repository::RegistryFile>(&local_180);
  return;
}
