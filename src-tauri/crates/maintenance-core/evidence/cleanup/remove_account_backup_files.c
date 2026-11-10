/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* remove_account_backup_files @ 10082fb00 requested 10082fb00 */

/* codexmate_lib::core::repository::remove_account_backup_files */

void codexmate_lib::core::repository::remove_account_backup_files
               (int *param_1,undefined8 param_2,undefined8 param_3,void *param_4,size_t param_5,long param_6,undefined8 param_7)

{
  code *pcVar1;
  long *plVar2;
  undefined *puVar3;
  long lVar4;
  size_t *psVar5;
  void *pvVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  int local_9d4;
  long local_9a8;
  undefined8 uStack_9a0;
  size_t local_998;
  size_t sStack_990;
  size_t local_988;
  int local_97c;
  long *local_978;
  char local_970;
  long *local_968;
  code *local_960;
  long *local_958;
  undefined8 *puStack_950;
  char *local_948;
  undefined8 local_940;
  undefined8 uStack_938;
  undefined *local_930;
  undefined8 uStack_928;
  undefined8 local_920;
  char *local_918;
  long **pplStack_910;
  long *local_540;
  undefined8 *local_538;
  long **local_530;
  code *local_528;
  undefined8 *local_120;
  long *local_118;
  undefined8 *local_110;
  ulong *local_108;
  code *local_100;
  size_t local_f8;
  size_t sStack_f0;
  size_t local_e8;
  long *local_e0;
  undefined *local_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  char *local_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined *local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  char *local_90;
  undefined8 **ppuStack_88;
  undefined1 auStack_79 [9];

  local_9a8 = param_6;
  uStack_9a0 = param_7;
  std::sys::fs::metadata(&local_968,param_2,param_3);
  if ((int)local_968 == 1) {
    if (((ulong)local_960 & 3) == 1) {
      uVar7 = *(undefined8 *)((long)local_960 + -1);
      puVar8 = *(undefined8 **)((long)local_960 + 7);
      if ((code *)*puVar8 != (code *)0x0) {
        (*(code *)*puVar8)(uVar7);
      }
      if (puVar8[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar7,puVar8[1],puVar8[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)local_960 + -1),0x18,8);
    }
    *param_1 = 0;
    param_1[2] = 0;
    param_1[3] = 0;
    param_1[4] = 8;
    param_1[5] = 0;
    param_1[6] = 0;
    param_1[7] = 0;
  }
  else {
    std::sys::fs::read_dir(&local_968,param_2,param_3);
    if ((char)local_960 == '\x02') {
      local_e0 = local_968;
      if (1 < log::MAX_LOG_LEVEL_FILTER) {
        local_540 = &local_9a8;
        local_538 = &<&T_as_core::fmt::Display>::fmt;
        local_530 = &local_e0;
        local_528 = <std::io::error::Error_as_core::fmt::Display>::fmt;
        local_930 = &DAT_1013af968;
        uStack_928 = 0x1f;
        pplStack_910 = &local_540;
        local_918 = s___AiMaMi__accounts__failed_to_sc_10152d92d;
        local_968 = (long *)0x0;
        local_960 = (code *)&DAT_1013af968;
        puStack_950 = (undefined8 *)0x0;
        local_958 = (undefined8 *)0x1f;
        local_948 = "src/core/repository.rs";
        uStack_938 = 2;
        local_940 = 0x16;
        local_920 = 0x1cf00000001;
        <log::__private_api::GlobalLogger_as_log::Log>::log(auStack_79,&local_968);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      psVar5 = (size_t *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x30,8);
      if (psVar5 == (size_t *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error(8,0x30);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      pvVar6 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_5,1);
      if (pvVar6 == (void *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,param_5);
      }
      _memcpy(pvVar6,param_4,param_5);
      local_968 = &local_9a8;
      local_960 = (code *)&<&T_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_998,s_Some_backups_could_not_be_inspec_10152d95e,&local_968);
      psVar5[4] = sStack_990;
      psVar5[3] = local_998;
      psVar5[5] = local_988;
      *psVar5 = param_5;
      psVar5[1] = (size_t)pvVar6;
      psVar5[2] = param_5;
      *param_1 = 0;
      param_1[2] = 1;
      param_1[3] = 0;
      *(size_t **)(param_1 + 4) = psVar5;
      param_1[6] = 1;
      param_1[7] = 0;
      if (((ulong)local_e0 & 3) == 1) {
        puVar8 = (undefined8 *)((long)local_e0 + -1);
        uVar7 = *puVar8;
        puVar9 = *(undefined8 **)((long)local_e0 + 7);
        if ((code *)*puVar9 != (code *)0x0) {
          (*(code *)*puVar9)(uVar7);
        }
        if (puVar9[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar7,puVar9[1],puVar9[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar8,0x18,8);
      }
    }
    else {
      local_9d4 = 0;
      local_97c = 0;
      local_978 = local_968;
      local_970 = (char)local_960;
      while (<std::fs::ReadDir_as_core::iter::traits::iterator::Iterator>::next(&local_968,&local_978), plVar2 = local_958, pcVar1 = local_960,
            (int)local_968 == 1) {
        if (local_960 == (code *)0x0) {
          local_120 = local_958;
          local_97c = local_97c + 1;
          if (1 < log::MAX_LOG_LEVEL_FILTER) {
            local_118 = &local_9a8;
            local_110 = &<&T_as_core::fmt::Display>::fmt;
            local_108 = (ulong *)&local_120;
            local_100 = <std::io::error::Error_as_core::fmt::Display>::fmt;
            local_a8 = &DAT_1013af968;
            uStack_a0 = 0x1f;
            local_90 = s___AiMaMi__accounts__failed_to_in_10152d8f8;
            local_e0 = (long *)0x0;
            local_d8 = &DAT_1013af968;
            uStack_c8 = 0;
            local_d0 = 0x1f;
            local_c0 = "src/core/repository.rs";
            uStack_b0 = 2;
            local_b8 = 0x16;
            local_98 = 0x1e100000001;
            ppuStack_88 = &local_118;
            <log::__private_api::GlobalLogger_as_log::Log>::log(auStack_79,&local_e0);
          }
          if (((ulong)local_120 & 3) == 1) {
            puVar9 = (undefined8 *)((long)local_120 + -1);
            uVar7 = *puVar9;
            puVar8 = *(undefined8 **)((long)local_120 + 7);
            if ((code *)*puVar8 != (code *)0x0) {
              (*(code *)*puVar8)(uVar7);
            }
            if (puVar8[1] != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar7,puVar8[1],puVar8[2]);
            }
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar9,0x18,8);
          }
        }
        else {
          _memcpy(&local_530,&puStack_950,0x410);
          local_540 = (long *)pcVar1;
          local_538 = plVar2;
          std::fs::DirEntry::path(&local_e0,&local_540);
          puVar3 = local_d8;
          puVar8 = (undefined8 *)std::sys::fs::remove_file(local_d8,local_d0);
          if (local_e0 != (long *)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar3,local_e0,1);
          }
          if (puVar8 == (undefined8 *)0x0) {
            local_9d4 = local_9d4 + 1;
          }
          else {
            local_97c = local_97c + 1;
            local_120 = puVar8;
            if (1 < log::MAX_LOG_LEVEL_FILTER) {
              local_118 = &local_9a8;
              local_110 = &<&T_as_core::fmt::Display>::fmt;
              local_108 = (ulong *)&local_120;
              local_100 = <std::io::error::Error_as_core::fmt::Display>::fmt;
              local_a8 = &DAT_1013af968;
              uStack_a0 = 0x1f;
              local_90 = s___AiMaMi__accounts__failed_to_re_10152d8c4;
              local_e0 = (long *)0x0;
              local_d8 = &DAT_1013af968;
              uStack_c8 = 0;
              local_d0 = 0x1f;
              local_c0 = "src/core/repository.rs";
              uStack_b0 = 2;
              local_b8 = 0x16;
              local_98 = 0x1e900000001;
              ppuStack_88 = &local_118;
              <log::__private_api::GlobalLogger_as_log::Log>::log(auStack_79,&local_e0);
            }
            if (((ulong)local_120 & 3) == 1) {
              puVar8 = (undefined8 *)((long)local_120 + -1);
              uVar7 = *puVar8;
              puVar9 = *(undefined8 **)((long)local_120 + 7);
              if ((code *)*puVar9 != (code *)0x0) {
                (*(code *)*puVar9)(uVar7);
              }
              if (puVar9[1] != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar7,puVar9[1],puVar9[2]);
              }
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar8,0x18,8);
            }
          }
          lVar4 = *local_540;
          *local_540 = lVar4 + -1;
          LORelease();
          if (lVar4 == 1) {
            DataMemoryBarrier(2,1);
            alloc::sync::Arc<T,A>::drop_slow(&local_540);
          }
        }
      }
      lVar4 = *local_978;
      *local_978 = lVar4 + -1;
      LORelease();
      if (lVar4 == 1) {
        DataMemoryBarrier(2,1);
        alloc::sync::Arc<T,A>::drop_slow(&local_978);
      }
      if (local_97c == 0) {
        uVar7 = 0;
        psVar5 = (size_t *)&SUB_00000008;
      }
      else {
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        psVar5 = (size_t *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x30,8);
        if (psVar5 == (size_t *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error(8,0x30);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        pvVar6 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_5,1);
        if (pvVar6 == (void *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,param_5);
        }
        _memcpy(pvVar6,param_4,param_5);
        local_968 = (long *)&local_97c;
        local_960 = i32>::fmt;
        local_958 = &local_9a8;
        puStack_950 = &<&T_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_f8,s_backup_s__could_not_be_removed__10152d89e,&local_968);
        psVar5[4] = sStack_f0;
        psVar5[3] = local_f8;
        psVar5[5] = local_e8;
        *psVar5 = param_5;
        psVar5[1] = (size_t)pvVar6;
        psVar5[2] = param_5;
        uVar7 = 1;
      }
      *param_1 = local_9d4;
      *(undefined8 *)(param_1 + 2) = uVar7;
      *(size_t **)(param_1 + 4) = psVar5;
      *(undefined8 *)(param_1 + 6) = uVar7;
    }
  }
  return;
}
