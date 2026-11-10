/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* list_codex_db_holder_processes @ 1007c4db0 requested 1007c4db0 */

/* codexmate_lib::platform::process::list_codex_db_holder_processes */

void codexmate_lib::platform::process::list_codex_db_holder_processes(long *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined4 uVar6;
  long lVar7;
  long *plVar8;
  long *plVar9;
  long *plVar10;
  undefined4 *puVar11;
  undefined4 *puVar12;
  int iVar13;
  undefined1 (*pauVar14) [16];
  long *plVar15;
  long *plVar16;
  long *plVar17;
  ulong uVar18;
  undefined4 *puVar19;
  undefined4 *puVar20;
  undefined8 *puVar21;
  long lVar22;
  long lVar23;
  undefined *local_f0;
  long lStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined1 local_d0 [16];
  long local_c0;
  long lStack_b8;
  long local_b0;
  long *local_a8;
  long *local_a0;
  long *local_98;
  long *plStack_90;
  undefined4 *local_88;
  undefined4 *local_80;
  undefined4 *local_78;
  undefined4 *puStack_70;
  
  pauVar14 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
  if (pauVar14[1][0] == '\x01') {
    local_d0 = *pauVar14;
  }
  else {
    local_d0 = std::sys::random::hashmap_random_keys();
    *pauVar14 = local_d0;
    pauVar14[1][0] = 1;
  }
  *(long *)*pauVar14 = local_d0._0_8_ + 1;
  lStack_e8 = 0;
  local_f0 = &_anon_6408540cdffbe318d49f52812e99783f_28;
  uStack_d8 = 0;
  uStack_e0 = 0;
  local_c0 = 0;
  lStack_b8 = 8;
  local_b0 = 0;
  paths::CodexPaths::resolve_codex_home(&local_88);
  puVar1 = local_80;
  paths::CodexPaths::runtime_state_db_candidates_for_home(&local_a8,local_80,local_78);
  if (local_88 != (undefined4 *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar1,local_88,1);
  }
  plVar10 = local_98;
  plVar9 = local_a0;
  plVar8 = local_a8;
  plVar16 = local_a0 + (long)local_98 * 3;
  local_a8 = local_a0;
  local_98 = plVar8;
  plVar15 = local_a0;
  plVar17 = local_a0;
  plStack_90 = plVar16;
  if (plVar10 == (long *)0x0) {
LAB_1007c4fd8:
    local_a0 = plVar17;
    if ((long)plVar16 - (long)plVar17 != 0) {
      uVar18 = (ulong)((long)plVar16 - (long)plVar17) / 0x18;
      plVar17 = plVar17 + 1;
      do {
        if (plVar17[-1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*plVar17,plVar17[-1],1);
        }
        plVar17 = plVar17 + 3;
        uVar18 = uVar18 - 1;
      } while (uVar18 != 0);
    }
  }
  else {
    do {
      plVar17 = plVar15 + 3;
      lVar22 = *plVar15;
      if (lVar22 == -0x8000000000000000) goto LAB_1007c4fd8;
      lVar3 = plVar15[1];
      local_a0 = plVar17;
      list_macos_db_holder_processes_for(&local_88,lVar3,plVar15[2]);
      puVar19 = local_78;
      puVar12 = local_80;
      puVar11 = local_88;
      puVar1 = local_80 + (long)local_78 * 8;
      local_88 = local_80;
      local_78 = puVar11;
      puVar20 = local_80;
      puVar2 = local_80;
      puStack_70 = puVar1;
      if (puVar19 != (undefined4 *)0x0) {
        do {
          puVar19 = puVar2 + 8;
          uVar6 = *puVar2;
          lVar23 = *(long *)(puVar2 + 2);
          puVar20 = puVar19;
          if (lVar23 == -0x8000000000000000) break;
          uVar4 = *(undefined8 *)(puVar2 + 4);
          uVar5 = *(undefined8 *)(puVar2 + 6);
          iVar13 = hashbrown::map::HashMap<K,V,S,A>::insert(&local_f0,uVar6);
          lVar7 = local_b0;
          if (iVar13 == 0) {
            if (local_b0 == local_c0) {
              alloc::raw_vec::RawVec<T,A>::grow_one(&local_c0);
            }
            puVar2 = (undefined4 *)(lStack_b8 + lVar7 * 0x20);
            *puVar2 = uVar6;
            *(long *)(puVar2 + 2) = lVar23;
            *(undefined8 *)(puVar2 + 4) = uVar4;
            *(undefined8 *)(puVar2 + 6) = uVar5;
            local_b0 = lVar7 + 1;
          }
          else if (lVar23 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar4,lVar23,1);
          }
          puVar20 = puVar1;
          puVar2 = puVar19;
        } while (puVar19 != puVar1);
      }
      local_80 = puVar20;
      if ((long)puVar1 - (long)puVar20 != 0) {
        uVar18 = (ulong)((long)puVar1 - (long)puVar20) >> 5;
        puVar21 = (undefined8 *)(puVar20 + 4);
        do {
          if (puVar21[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar21,puVar21[-1],1);
          }
          puVar21 = puVar21 + 4;
          uVar18 = uVar18 - 1;
        } while (uVar18 != 0);
      }
      if (puVar11 != (undefined4 *)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar12,(long)puVar11 << 5,8);
      }
      if (lVar22 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar3,lVar22,1);
      }
      plVar15 = plVar17;
    } while (plVar17 != plVar16);
  }
  if (plVar8 != (long *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar9,(long)plVar8 * 0x18,8);
  }
  param_1[1] = lStack_b8;
  *param_1 = local_c0;
  param_1[2] = local_b0;
  if (lStack_e8 != 0) {
    uVar18 = lStack_e8 * 4 + 0xbU & 0xfffffffffffffff8;
    lVar22 = lStack_e8 + uVar18 + 9;
    if (lVar22 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((long)local_f0 - uVar18,lVar22,8);
    }
  }
  return;
}

