/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* quiesce_codex_transition_targets @ 1007c5fe0 requested 1007c5fe0 */

/* codexmate_lib::platform::process::quiesce_codex_transition_targets */

void codexmate_lib::platform::process::quiesce_codex_transition_targets(long *param_1,undefined8 param_2,long param_3,int param_4)

{
  long lVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  undefined8 *puVar5;
  long lVar6;
  long lVar7;
  long local_130;
  long lStack_128;
  long local_120;
  long local_118;
  long local_110;
  long lStack_108;
  long lStack_100;
  long lStack_f8;
  long local_f0;
  long lStack_e8;
  long lStack_e0;
  long lStack_d8;
  long local_d0;
  long lStack_c8;
  long local_c0;
  long lStack_b8;
  long local_b0;
  long lStack_a8;
  long lStack_a0;
  long local_98;
  long local_90;
  long lStack_88;
  long lStack_80;
  long lStack_78;
  long local_70;
  long lStack_68;
  long lStack_60;
  long lStack_58;
  
  if (param_3 == 0) {
    *param_1 = 0xb;
  }
  else {
    if (param_4 == 0) {
      force_kill_targets(param_1,param_2,param_3);
      return;
    }
    collect_external_codex_writers(&local_b0);
    lVar7 = local_98;
    lVar4 = lStack_a0;
    lVar3 = lStack_a8;
    lStack_128 = lStack_a0;
    local_130 = lStack_a8;
    local_120 = local_98;
    if (local_b0 == 0xb) {
      <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_130,lStack_a0,lStack_a0 + local_98 * 0x20);
      reject_external_codex_writers(&local_b0,&local_130);
      if (local_b0 == 0xb) {
        list_codex_processes_matching(&local_b0);
        lVar2 = lStack_a0;
        lVar1 = lStack_a8;
        lVar6 = local_b0;
        if (local_b0 == 0xb) {
          local_d0 = lStack_a0;
          lStack_c8 = lStack_a0;
          lStack_b8 = lStack_a0 + local_98 * 0x20;
          local_c0 = lStack_a8;
          alloc::vec::in_place_collect::from_iter_in_place(&lStack_128,&local_d0);
          lVar6 = local_118;
          lVar2 = local_120;
          lVar1 = lStack_128;
          lStack_a8 = local_120;
          local_b0 = lStack_128;
          lStack_a0 = local_118;
          force_kill_targets(param_1,local_120,local_118);
          if (lVar6 != 0) {
            puVar5 = (undefined8 *)(lVar2 + 0x10);
            do {
              if (puVar5[-1] != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar5,puVar5[-1],1);
              }
              puVar5 = puVar5 + 4;
              lVar6 = lVar6 + -1;
            } while (lVar6 != 0);
          }
          if (lVar1 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar2,lVar1 << 5,8);
          }
        }
        else {
          lStack_108 = lStack_88;
          local_110 = local_90;
          lStack_f8 = lStack_78;
          lStack_100 = lStack_80;
          lStack_e8 = lStack_68;
          local_f0 = local_70;
          lStack_d8 = lStack_58;
          lStack_e0 = lStack_60;
          lStack_128 = lStack_a8;
          local_120 = lStack_a0;
          local_118 = local_98;
          lStack_a0 = local_98;
          lStack_a8 = lVar2;
          local_b0 = lVar1;
          param_1[5] = lStack_88;
          param_1[4] = local_90;
          param_1[7] = lStack_78;
          param_1[6] = lStack_80;
          param_1[9] = lStack_68;
          param_1[8] = local_70;
          param_1[0xb] = lStack_58;
          param_1[10] = lStack_60;
          param_1[2] = lVar2;
          param_1[1] = lVar1;
          param_1[3] = local_98;
          *param_1 = lVar6;
        }
      }
      else {
        param_1[5] = lStack_88;
        param_1[4] = local_90;
        param_1[7] = lStack_78;
        param_1[6] = lStack_80;
        param_1[9] = lStack_68;
        param_1[8] = local_70;
        param_1[0xb] = lStack_58;
        param_1[10] = lStack_60;
        param_1[1] = lStack_a8;
        *param_1 = local_b0;
        param_1[3] = local_98;
        param_1[2] = lStack_a0;
      }
      if (lVar7 != 0) {
        puVar5 = (undefined8 *)(lVar4 + 8);
        do {
          if (puVar5[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar5,puVar5[-1],1);
          }
          puVar5 = puVar5 + 4;
          lVar7 = lVar7 + -1;
        } while (lVar7 != 0);
      }
      if (lVar3 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar4,lVar3 << 5,8);
      }
    }
    else {
      param_1[5] = lStack_88;
      param_1[4] = local_90;
      param_1[7] = lStack_78;
      param_1[6] = lStack_80;
      param_1[9] = lStack_68;
      param_1[8] = local_70;
      param_1[0xb] = lStack_58;
      param_1[10] = lStack_60;
      param_1[2] = lStack_a0;
      param_1[1] = lStack_a8;
      param_1[3] = local_98;
      *param_1 = local_b0;
    }
  }
  return;
}

