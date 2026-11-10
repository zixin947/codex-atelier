/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* from_iter @ 10053c31c requested 10053c31c */

/* <alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter */

void <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(long *param_1,char *param_2,char *param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  long lVar3;
  long *plVar4;
  void *pvVar5;
  size_t sVar6;
  size_t *psVar7;
  undefined8 uVar8;
  char *pcVar9;
  char *pcVar10;
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  long local_78;
  long *plStack_70;
  long local_68;

  do {
    while( true ) {
      do {
        pcVar10 = param_2;
        if (pcVar10 == param_3) {
          *param_1 = 0;
          param_1[1] = 8;
          param_1[2] = 0;
          return;
        }
        param_2 = pcVar10 + 0x20;
      } while (*pcVar10 != '\x03');
      codexmate_lib::core::relay::codex_project_state::normalize_codex_path(&local_78,*(undefined8 *)(pcVar10 + 0x10),*(undefined8 *)(pcVar10 + 0x18));
      if (local_68 != 0) break;
      if (local_78 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_70,local_78,1);
      }
    }
  } while (local_78 == -0x8000000000000000);
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  plVar4 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x60,8);
  if (plVar4 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(8,0x60);
  }
  pcVar10 = pcVar10 + 0x20;
  *plVar4 = local_78;
  plVar4[1] = (long)plStack_70;
  plVar4[2] = local_68;
  local_78 = 4;
  local_68 = 1;
  plStack_70 = plVar4;
  if (pcVar10 != param_3) {
    uVar8 = 1;
    lVar3 = local_68;
LAB_10053c43c:
    do {
      pcVar9 = pcVar10 + 0x20;
      if (*pcVar10 == '\x03') {
        uVar1 = *(undefined8 *)(pcVar10 + 0x10);
        uVar2 = *(undefined8 *)(pcVar10 + 0x18);
        auVar11 = core::str::<impl_str>::trim_matches(uVar1,uVar2);
        if ((auVar11._8_8_ < 4) || (*auVar11._0_8_ != 0x5c3f5c5c)) {
          auVar12 = core::str::<impl_str>::trim_matches(uVar1,uVar2);
        }
        else {
          auVar12._8_8_ = auVar11._8_8_ - 4;
          auVar12._0_8_ = auVar11._0_8_ + 1;
        }
        auVar11 = core::str::<impl_str>::trim_end_matches(auVar12._0_8_,auVar12._8_8_,0x2f0000005c);
        sVar6 = auVar11._8_8_;
        if ((long)sVar6 < 0) {
          uVar8 = 0;
LAB_10053c534:
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(uVar8);
        }
        if (sVar6 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          pvVar5 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar6,1);
          if (pvVar5 == (void *)0x0) goto LAB_10053c534;
          _memcpy(pvVar5,auVar11._0_8_,sVar6);
          if (lVar3 == local_78) {
            alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_78,lVar3,1,8,0x18);
            plVar4 = plStack_70;
          }
          pcVar10 = pcVar10 + 0x20;
          psVar7 = (size_t *)(plVar4 + lVar3 * 3);
          *psVar7 = sVar6;
          psVar7[1] = (size_t)pvVar5;
          psVar7[2] = sVar6;
          local_68 = lVar3 + 1;
          lVar3 = local_68;
          if (pcVar10 == param_3) break;
          goto LAB_10053c43c;
        }
      }
      pcVar10 = pcVar9;
    } while (pcVar9 != param_3);
  }
  param_1[1] = (long)plStack_70;
  *param_1 = local_78;
  param_1[2] = local_68;
  return;
}
