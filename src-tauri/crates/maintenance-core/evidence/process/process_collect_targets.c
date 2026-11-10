/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* collect_codex_kill_targets @ 1007c1318 requested 1007c1318 */

/* codexmate_lib::platform::process::collect_codex_kill_targets */

void codexmate_lib::platform::process::collect_codex_kill_targets(long *param_1)

{
  undefined4 *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  long lVar6;
  long lVar7;
  long lVar8;
  undefined4 *puVar9;
  int iVar10;
  undefined1 (*pauVar11) [16];
  ulong uVar12;
  undefined4 *puVar13;
  undefined8 *puVar14;
  undefined4 *puVar15;
  long lVar16;
  undefined4 *puVar17;
  undefined4 *puVar18;
  undefined4 *local_190;
  undefined4 *puStack_188;
  undefined4 *local_180;
  undefined4 *local_170;
  undefined4 *puStack_168;
  undefined4 *puStack_160;
  undefined4 *local_158;
  undefined1 local_150 [16];
  undefined4 *local_110;
  undefined4 *puStack_108;
  undefined8 local_100;
  long local_f0;
  long lStack_e8;
  long local_e0;
  long lStack_d8;
  undefined4 *local_d0;
  undefined4 *puStack_c8;
  undefined4 *local_c0;
  undefined4 *puStack_b8;
  undefined1 local_b0 [16];
  long lStack_a0;
  long lStack_98;
  long local_90;
  long lStack_88;
  long lStack_80;
  long lStack_78;
  
  list_codex_processes_matching(&local_d0);
  if (local_d0 == (undefined4 *)0xb) {
    local_f0 = (long)local_c0;
    lStack_e8 = (long)local_c0;
    lStack_d8 = (long)local_c0 + (long)puStack_b8 * 0x20;
    local_e0 = (long)puStack_c8;
    alloc::vec::in_place_collect::from_iter_in_place(&puStack_168,&local_f0);
    puStack_108 = puStack_160;
    local_110 = puStack_168;
    puStack_188 = puStack_160;
    local_190 = puStack_168;
    local_180 = local_158;
    codex_app_bundle_path(&local_d0);
    if (local_d0 == (undefined4 *)0x8000000000000000) {
      local_110 = (undefined4 *)0x8000000000000000;
    }
    else {
      codex_macos_bundle_main_executable(&local_110,puStack_c8,local_c0);
      if (local_d0 != (undefined4 *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_c8,local_d0,1);
      }
    }
    puVar15 = local_180;
    puVar13 = puStack_188;
    pauVar11 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
    if (pauVar11[1][0] == '\x01') {
      local_b0 = *pauVar11;
    }
    else {
      local_b0 = std::sys::random::hashmap_random_keys();
      *pauVar11 = local_b0;
      pauVar11[1][0] = 1;
    }
    *(long *)*pauVar11 = local_b0._0_8_ + 1;
    puStack_c8 = (undefined4 *)0x0;
    local_d0 = (undefined4 *)&_anon_6408540cdffbe318d49f52812e99783f_28;
    puStack_b8 = (undefined4 *)0x0;
    local_c0 = (undefined4 *)0x0;
    if (puVar15 != (undefined4 *)0x0) {
      hashbrown::raw::RawTable<T,A>::reserve_rehash(&local_d0,puVar15,local_b0,1);
      do {
        hashbrown::map::HashMap<K,V,S,A>::insert(&local_d0,*puVar13);
        puVar15 = (undefined4 *)((long)puVar15 + -1);
        puVar13 = puVar13 + 8;
      } while (puVar15 != (undefined4 *)0x0);
    }
    puStack_168 = puStack_c8;
    local_170 = local_d0;
    local_158 = puStack_b8;
    puStack_160 = local_c0;
    local_150 = local_b0;
    list_codex_db_holder_processes(&local_d0);
    puVar1 = local_c0;
    puVar9 = puStack_c8;
    puVar13 = local_d0;
    puVar15 = puStack_c8 + (long)local_c0 * 8;
    local_d0 = puStack_c8;
    local_c0 = puVar13;
    puVar18 = puStack_c8;
    puStack_b8 = puVar15;
    if (puVar1 != (undefined4 *)0x0) {
      puVar5 = puStack_c8;
      puVar1 = (undefined4 *)0x0;
      if (local_110 != (undefined4 *)0x8000000000000000) {
        puVar1 = puStack_108;
      }
      do {
        puVar17 = puVar5 + 8;
        uVar4 = *puVar5;
        lVar16 = *(long *)(puVar5 + 2);
        puVar18 = puVar17;
        if (lVar16 == -0x8000000000000000) break;
        uVar2 = *(undefined8 *)(puVar5 + 4);
        uVar3 = *(undefined8 *)(puVar5 + 6);
        iVar10 = is_codex_related_command_with_main(uVar2,uVar3,puVar1,local_100);
        if (iVar10 != 0) {
          iVar10 = hashbrown::map::HashMap<K,V,S,A>::insert(&local_170,uVar4);
          if (iVar10 == 0) {
            extract_process_name(&local_f0,uVar2,uVar3);
            lVar8 = local_e0;
            lVar7 = lStack_e8;
            lVar6 = local_f0;
            puVar18 = local_180;
            if (local_180 == local_190) {
              alloc::raw_vec::RawVec<T,A>::grow_one(&local_190);
            }
            puVar5 = puStack_188 + (long)puVar18 * 8;
            *puVar5 = uVar4;
            *(long *)(puVar5 + 2) = lVar6;
            *(long *)(puVar5 + 4) = lVar7;
            *(long *)(puVar5 + 6) = lVar8;
            local_180 = (undefined4 *)((long)puVar18 + 1);
          }
        }
        if (lVar16 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar2,lVar16,1);
        }
        puVar18 = puVar15;
        puVar5 = puVar17;
      } while (puVar17 != puVar15);
    }
    puStack_c8 = puVar18;
    if ((long)puVar15 - (long)puVar18 != 0) {
      uVar12 = (ulong)((long)puVar15 - (long)puVar18) >> 5;
      puVar14 = (undefined8 *)(puVar18 + 4);
      do {
        if (puVar14[-1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar14,puVar14[-1],1);
        }
        puVar14 = puVar14 + 4;
        uVar12 = uVar12 - 1;
      } while (uVar12 != 0);
    }
    if (puVar13 != (undefined4 *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar9,(long)puVar13 << 5,8);
    }
    if (puStack_168 != (undefined4 *)0x0) {
      uVar12 = (long)puStack_168 * 4 + 0xbU & 0xfffffffffffffff8;
      lVar16 = (long)puStack_168 + uVar12 + 9;
      if (lVar16 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((long)local_170 - uVar12,lVar16,8);
      }
    }
    if ((local_110 != (undefined4 *)0x8000000000000000) && (local_110 != (undefined4 *)0x0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_108,local_110,1);
    }
    param_1[2] = (long)puStack_188;
    param_1[1] = (long)local_190;
    param_1[3] = (long)local_180;
    *param_1 = 0xb;
  }
  else {
    param_1[5] = local_b0._8_8_;
    param_1[4] = local_b0._0_8_;
    param_1[7] = lStack_98;
    param_1[6] = lStack_a0;
    param_1[9] = lStack_88;
    param_1[8] = local_90;
    param_1[0xb] = lStack_78;
    param_1[10] = lStack_80;
    param_1[2] = (long)local_c0;
    param_1[1] = (long)puStack_c8;
    param_1[3] = (long)puStack_b8;
    *param_1 = (long)local_d0;
  }
  return;
}

