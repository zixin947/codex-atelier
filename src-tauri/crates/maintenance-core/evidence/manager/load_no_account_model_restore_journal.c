/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* load_no_account_model_restore_journal @ 1008c94c4 requested 1008c94c4 */

/* codexmate_lib::core::relay::codex_thread_visibility::load_no_account_model_restore_journal */

char * codexmate_lib::core::relay::codex_thread_visibility::load_no_account_model_restore_journal(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  uint7 uVar2;
  code *pcVar3;
  uint uVar4;
  int iVar5;
  char *pcVar6;
  ulong uVar7;
  long lVar8;
  long extraout_x1;
  code *pcVar9;
  undefined8 uVar10;
  undefined8 *puVar11;
  long lVar12;
  undefined1 auVar13 [16];
  long local_1d8;
  long local_1d0;
  long lStack_1c8;
  undefined8 local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  long local_1a8;
  code *local_1a0;
  code *local_198;
  long local_190;
  long lStack_188;
  long lStack_180;
  undefined8 local_178;
  code *local_170;
  undefined4 *puStack_168;
  code *pcStack_160;
  undefined4 local_150;
  undefined2 uStack_14c;
  undefined1 uStack_14a;
  undefined1 uStack_149;
  undefined2 uStack_148;
  undefined2 local_146;
  undefined8 local_128;
  long lStack_120;
  long local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  code *local_f0;
  code *pcStack_e8;
  undefined4 *local_e0;
  code *local_d8;
  undefined1 auStack_d0 [16];
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  ulong local_88;

  std::path::Path::_join
            (&local_1d8,param_2,param_3,"router_no_account_model_restore.jsonrollout is no longer owned by the router; history preservedthread-rollout-repairs",
             0x24);
  std::sys::fs::symlink_metadata(&local_f0,local_1d0,lStack_1c8);
  pcVar3 = pcStack_e8;
  if (((ulong)local_f0 & 1) == 0) {
    if (((ulong)pcStack_e8 & 0xf00000000000) == 0x800000000000) {
      if (local_88 < 0x800001) {
        local_1a8 = 0;
        local_1a0 = (code *)0x1;
        local_198 = (code *)0x0;
        local_150 = 0;
        uStack_14c = 0x1b6;
        uStack_149 = 0;
        uStack_148 = 0;
        local_146 = 0;
        uStack_14a = 1;
        pcVar6 = (char *)std::fs::OpenOptions::_open(&local_128,&local_150,local_1d0,lStack_1c8);
        if ((int)local_128 == 1) {
          *param_1 = 2;
          param_1[1] = lStack_120;
          lVar8 = local_1a8;
          pcVar9 = local_1a0;
        }
        else {
          local_e0 = (undefined4 *)CONCAT44(local_e0._4_4_,local_128._4_4_);
          pcStack_e8 = (code *)0x800001;
          local_f0 = (code *)0x800001;
          auVar13 = std::io::default_read_to_end(&local_f0,&local_1a8,0);
          if ((auVar13._0_8_ & 1) == 0) {
            iVar5 = _close((int)local_e0);
            if (local_198 < (code *)0x800001) {
              local_f0 = local_1a0;
              pcStack_e8 = local_198;
              local_e0 = (undefined4 *)0x0;
              local_d8 = (code *)0x0;
              serde_json::de::from_trait(&local_128,&local_f0);
              if (local_128 == -0x8000000000000000) {
                pcVar6 = (char *)load_no_account_model_restore_journal::__closure__(&local_f0,local_1d0,lStack_1c8,lStack_120);
                param_1[6] = uStack_c0;
                param_1[5] = auStack_d0._8_8_;
                param_1[8] = uStack_b0;
                param_1[7] = local_b8;
                param_1[10] = uStack_a0;
                param_1[9] = local_a8;
                param_1[0xb] = local_98;
                param_1[4] = auStack_d0._0_8_;
                param_1[3] = local_d8;
                param_1[2] = local_e0;
                param_1[1] = pcStack_e8;
                *param_1 = local_f0;
                lVar8 = local_1a8;
                pcVar9 = local_1a0;
                local_170 = pcStack_e8;
                puStack_168 = local_e0;
                pcStack_160 = local_d8;
              }
              else {
                lStack_188 = lStack_120;
                local_190 = local_128;
                local_178 = uStack_110;
                uVar10 = local_178;
                lStack_180 = local_118;
                local_178._0_4_ = (int)uStack_110;
                iVar5 = (int)local_178;
                local_178 = uVar10;
                if ((int)local_178 - 1U < 2) {
                  auStack_d0 = std::thread::local::LocalKey<T>::with(&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_36);
                  pcStack_e8 = (code *)0x0;
                  local_f0 = (code *)&_anon_201a6070f17c8d3e3a2dcb35fe0dabc2_22;
                  local_d8 = (code *)0x0;
                  local_e0 = (undefined4 *)0x0;
                  lVar12 = lStack_180 * 0x48;
                  lVar8 = lStack_188;
                  do {
                    if (lVar12 == 0) {
                      param_1[2] = lStack_188;
                      param_1[1] = local_190;
                      param_1[4] = local_178;
                      param_1[3] = lStack_180;
                      *param_1 = 0xb;
                      pcVar6 = (char *)<hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(&local_f0);
                      lVar8 = local_1a8;
                      pcVar9 = local_1a0;
                      goto joined_r0x0001008c99fc;
                    }
                    ::core::str::<impl_str>::trim_matches(*(undefined8 *)(lVar8 + 8),*(undefined8 *)(lVar8 + 0x10));
                    if (extraout_x1 == 0) break;
                    uVar10 = *(undefined8 *)(lVar8 + 0x20);
                    uVar1 = *(undefined8 *)(lVar8 + 0x28);
                    uVar7 = codex_catalog::is_no_account_slot_slug(uVar10,uVar1);
                    if ((uVar7 & 1) == 0) {
                      if (iVar5 != 2) break;
                      uVar7 = codex_catalog::is_relay_model_slug(uVar10,uVar1);
                      if ((uVar7 & 1) == 0) break;
                    }
                    uVar7 = hashbrown::map::HashMap<K,V,S,A>::insert(&local_f0,*(undefined8 *)(lVar8 + 8),*(undefined8 *)(lVar8 + 0x10));
                    lVar8 = lVar8 + 0x48;
                    lVar12 = lVar12 + -0x48;
                  } while ((uVar7 & 1) == 0);
                  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                  pcVar6 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x31,1);
                  if (pcVar6 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
                    alloc::raw_vec::handle_error(1,0x31);
                  }
                  builtin_strncpy(pcVar6,"invalid no-account model restore journal contents",0x31);
                  param_1[1] = 0x31;
                  *param_1 = 10;
                  param_1[2] = pcVar6;
                  param_1[3] = 0x31;
                  <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(&local_f0);
                  pcVar6 = (char *)::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_thread_visibility::NoAccountModelRestoreJournal>(&local_190);
                  lVar8 = local_1a8;
                  pcVar9 = local_1a0;
                }
                else {
                  local_f0 = (code *)&local_178;
                  pcStack_e8 = u32>::fmt;
                  alloc::fmt::format::format_inner(&local_108,s_5unsupported_no_account_model_re_101530f00,&local_f0);
                  param_1[2] = uStack_100;
                  param_1[1] = local_108;
                  param_1[3] = local_f8;
                  *param_1 = 10;
                  pcVar6 = (char *)::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_thread_visibility::NoAccountModelRestoreJournal>(&local_190);
                  lVar8 = local_1a8;
                  pcVar9 = local_1a0;
                }
              }
            }
            else {
              __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2(iVar5);
              pcVar6 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x33,1);
              if (pcVar6 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
                alloc::raw_vec::handle_error(1,0x33);
              }
              builtin_strncpy(pcVar6,"no-account model restore journal exceeds size limit",0x33);
              param_1[1] = 0x33;
              *param_1 = 10;
              param_1[2] = pcVar6;
              param_1[3] = 0x33;
              lVar8 = local_1a8;
              pcVar9 = local_1a0;
            }
          }
          else {
            *param_1 = 2;
            param_1[1] = auVar13._8_8_;
            uVar4 = _close((int)local_e0);
            pcVar6 = (char *)(ulong)uVar4;
            lVar8 = local_1a8;
            pcVar9 = local_1a0;
          }
        }
joined_r0x0001008c99fc:
        local_1a8 = lVar8;
        local_1a0 = pcVar9;
        if (lVar8 == 0) goto LAB_1008c9a0c;
        uVar10 = 1;
        goto LAB_1008c9a08;
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar10 = 0x33;
      pcVar6 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x33,1);
      if (pcVar6 == (char *)0x0) goto LAB_1008c9a3c;
      builtin_strncpy(pcVar6,"no-account model restore journal exceeds size limit",0x33);
      param_1[1] = 0x33;
      *param_1 = 10;
      uVar10 = 0x33;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar10 = 0x32;
      pcVar6 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x32,1);
      if (pcVar6 == (char *)0x0) {
LAB_1008c9a3c:
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,uVar10);
      }
      builtin_strncpy(pcVar6,"invalid no-account model restore journal path type",0x32);
      param_1[1] = 0x32;
      *param_1 = 10;
      uVar10 = 0x32;
    }
    param_1[2] = pcVar6;
    param_1[3] = uVar10;
  }
  else {
    pcVar6 = (char *)std::io::error::Error::kind(pcStack_e8);
    if (((ulong)pcVar6 & 0xff) == 0) {
      param_1[2] = 8;
      param_1[3] = 0;
      *(undefined4 *)(param_1 + 4) = 2;
      param_1[1] = 0;
      *param_1 = 0xb;
      if (((ulong)pcVar3 & 3) != 1) goto LAB_1008c9a0c;
      pcVar9 = pcVar3 + -1;
      uVar10 = *(undefined8 *)pcVar9;
      puVar11 = *(undefined8 **)(pcVar3 + 7);
      if ((code *)*puVar11 != (code *)0x0) {
        (*(code *)*puVar11)(uVar10);
      }
      if (puVar11[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar10,puVar11[1],puVar11[2]);
      }
    }
    else {
      local_150 = SUB84(pcVar3,0);
      uStack_14c = (undefined2)((ulong)pcVar3 >> 0x20);
      uStack_14a = (undefined1)((ulong)pcVar3 >> 0x30);
      uStack_149 = (undefined1)((ulong)pcVar3 >> 0x38);
      local_128 = local_1d0;
      lStack_120 = lStack_1c8;
      local_f0 = (code *)&local_128;
      pcStack_e8 = <std::path::Display_as_core::fmt::Display>::fmt;
      local_e0 = &local_150;
      local_d8 = <std::io::error::Error_as_core::fmt::Display>::fmt;
      pcVar6 = (char *)alloc::fmt::format::format_inner(&local_1c0,s__read_no_account_model_restore_j_101530f38,&local_f0);
      param_1[2] = uStack_1b8;
      param_1[1] = local_1c0;
      param_1[3] = local_1b0;
      *param_1 = 10;
      uVar2 = CONCAT16(uStack_14a,CONCAT24(uStack_14c,local_150));
      lVar8 = CONCAT17(uStack_149,uVar2);
      if (((ulong)uVar2 & 3) != 1) goto LAB_1008c9a0c;
      pcVar9 = (code *)(lVar8 + -1);
      uVar10 = *(undefined8 *)pcVar9;
      puVar11 = *(undefined8 **)(lVar8 + 7);
      if ((code *)*puVar11 != (code *)0x0) {
        (*(code *)*puVar11)(uVar10);
      }
      if (puVar11[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar10,puVar11[1],puVar11[2]);
      }
    }
    lVar8 = 0x18;
    uVar10 = 8;
LAB_1008c9a08:
    pcVar6 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar9,lVar8,uVar10);
  }
LAB_1008c9a0c:
  if (local_1d8 != 0) {
    pcVar6 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1d0,local_1d8,1);
  }
  return pcVar6;
}
