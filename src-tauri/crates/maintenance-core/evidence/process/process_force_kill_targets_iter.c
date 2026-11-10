/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* from_iter @ 10053adf0 requested 10053adf0 */

/* <alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter */

void <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(long *param_1,long param_2,long param_3)

{
  undefined8 ******ppppppuVar1;
  long *plVar2;
  long *plVar3;
  undefined8 ******local_a8;
  long *plStack_a0;
  undefined8 *****local_98;
  long local_90;
  long local_88;
  long local_80;
  long lStack_78;
  long local_70;
  long *local_68;
  
  local_a8 = &local_a8;
  do {
    if (param_2 == param_3) {
      *param_1 = 0;
      param_1[1] = 8;
      param_1[2] = 0;
      return;
    }
    param_2 = param_2 + 0x20;
    &mut_F>::call_mut(&local_80,&local_a8);
  } while (local_80 == -0x8000000000000000);
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  plStack_a0 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x60,8);
  if (plStack_a0 != (long *)0x0) {
    plStack_a0[1] = lStack_78;
    *plStack_a0 = local_80;
    plStack_a0[2] = local_70;
    local_a8 = (undefined8 ******)0x4;
    local_98 = (undefined8 ******)0x1;
    local_88 = param_3;
    local_68 = &local_80;
    ppppppuVar1 = (undefined8 ******)local_98;
    plVar3 = plStack_a0;
    while (param_2 != param_3) {
      param_2 = param_2 + 0x20;
      local_90 = param_2;
      &mut_F>::call_mut(&local_80,&local_68);
      if (local_80 != -0x8000000000000000) {
        if (ppppppuVar1 == local_a8) {
          alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_a8,ppppppuVar1,1,8,0x18);
          plVar3 = plStack_a0;
          param_3 = local_88;
          param_2 = local_90;
        }
        plVar2 = plVar3 + (long)ppppppuVar1 * 3;
        plVar2[2] = local_70;
        plVar2[1] = lStack_78;
        *plVar2 = local_80;
        local_98 = (undefined8 *****)((long)ppppppuVar1 + 1);
        local_68 = &local_80;
        ppppppuVar1 = (undefined8 ******)local_98;
      }
    }
    param_1[1] = (long)plStack_a0;
    *param_1 = (long)local_a8;
    param_1[2] = (long)local_98;
    return;
  }
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error(8,0x60);
}

