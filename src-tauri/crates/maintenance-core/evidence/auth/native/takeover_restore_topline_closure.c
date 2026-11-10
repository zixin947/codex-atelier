/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* {{closure}} @ 1001ff460 requested 1001ff460 */

/* codexmate_lib::core::relay::config_takeover::restore::{{closure}} */

void codexmate_lib::core::relay::config_takeover::restore::__closure__(long param_1,long param_2,void *param_3,size_t param_4,long *param_5,long *param_6)

{
  long lVar1;
  long lVar2;
  void *pvVar3;
  size_t sVar4;
  int iVar5;
  void *pvVar6;
  long *plVar7;
  size_t *psVar8;
  long lVar9;
  long lVar10;
  undefined8 *puVar11;
  undefined8 uVar12;
  long local_98;
  long local_78;
  void *local_70;
  size_t local_68;

  ::core::str::<impl_str>::trim_matches(param_3,param_4);
  top_level_assignment_key(&local_78);
  sVar4 = local_68;
  pvVar6 = local_70;
  lVar1 = local_78;
  if (local_78 == -0x8000000000000000) {
    return;
  }
  if (param_2 != 0) {
    param_2 = param_2 * 0x18;
    psVar8 = (size_t *)(param_1 + 0x10);
    do {
      if ((*psVar8 == sVar4) && (iVar5 = _memcmp((void *)psVar8[-1],pvVar6,sVar4), iVar5 == 0)) goto LAB_1001ff508;
      psVar8 = psVar8 + 3;
      param_2 = param_2 + -0x18;
    } while (param_2 != 0);
  }
  local_98 = param_5[1];
  lVar9 = param_5[2];
  if (lVar9 != 0) {
    lVar10 = lVar9 * 0x18;
    puVar11 = (undefined8 *)(local_98 + 0x10);
    do {
      ::core::str::<impl_str>::trim_matches(puVar11[-1],*puVar11);
      top_level_assignment_key(&local_78);
      pvVar3 = local_70;
      lVar2 = local_78;
      if (local_78 != -0x8000000000000000) {
        if (local_68 == sVar4) {
          iVar5 = _memcmp(local_70,pvVar6,sVar4);
          if (lVar2 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar3,lVar2,1);
          }
          if (iVar5 == 0) {
LAB_1001ff508:
            lVar9 = param_6[8];
            if (lVar9 == param_6[6]) {
              alloc::raw_vec::RawVec<T,A>::grow_one();
            }
            plVar7 = (long *)(param_6[7] + lVar9 * 0x18);
            *plVar7 = lVar1;
            plVar7[1] = (long)pvVar6;
            plVar7[2] = sVar4;
            param_6[8] = lVar9 + 1;
            return;
          }
        }
        else if (local_78 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_70,local_78,1);
        }
      }
      puVar11 = puVar11 + 3;
      lVar10 = lVar10 + -0x18;
    } while (lVar10 != 0);
  }
  lVar10 = param_6[2];
  if (lVar10 == *param_6) {
    alloc::raw_vec::RawVec<T,A>::grow_one(param_6);
  }
  plVar7 = (long *)(param_6[1] + lVar10 * 0x18);
  *plVar7 = lVar1;
  plVar7[1] = (long)pvVar6;
  plVar7[2] = sVar4;
  param_6[2] = lVar10 + 1;
  if (-1 < (long)param_4) {
    if (param_4 == 0) {
      pvVar6 = (void *)0x1;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar12 = 1;
      pvVar6 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_4,1);
      if (pvVar6 == (void *)0x0) goto LAB_1001ff630;
    }
    _memcpy(pvVar6,param_3,param_4);
    if (lVar9 == *param_5) {
      alloc::raw_vec::RawVec<T,A>::grow_one(param_5);
      local_98 = param_5[1];
    }
    psVar8 = (size_t *)(local_98 + lVar9 * 0x18);
    *psVar8 = param_4;
    psVar8[1] = (size_t)pvVar6;
    psVar8[2] = param_4;
    param_5[2] = lVar9 + 1;
    return;
  }
  uVar12 = 0;
LAB_1001ff630:
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error(uVar12,param_4);
}
