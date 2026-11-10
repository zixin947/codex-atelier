/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* tidy_blank_lines @ 1001f77e8 requested 1001f77e8 */

/* codexmate_lib::core::relay::config_takeover::tidy_blank_lines */

void codexmate_lib::core::relay::config_takeover::tidy_blank_lines(long *param_1,long param_2,long param_3)

{
  ushort uVar1;
  long lVar2;
  long extraout_x1;
  long lVar3;
  long extraout_x1_00;
  undefined8 uVar4;
  long lVar5;
  void *pvVar6;
  ulong uVar7;
  ulong uVar8;
  long lVar9;
  long lVar10;
  long local_d8;
  long lStack_d0;
  long local_c8;
  long local_c0;
  long local_b8;
  long local_b0;
  long local_a8;
  undefined8 uStack_a0;
  long local_98;
  undefined8 local_90;
  undefined1 local_88;
  undefined2 local_80;
  int local_78 [4];
  long local_68;

  if (-1 < param_3) {
    if (param_3 == 0) {
      lVar2 = 1;
    }
    else {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      uVar4 = 1;
      lVar2 = __RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(param_3,1);
      if (lVar2 == 0) goto LAB_1001f7814;
    }
    lVar10 = 0;
    local_c8 = 0;
    local_c0 = 0;
    uStack_a0 = 0;
    local_90 = 0xa0000000a;
    local_88 = 1;
    local_80 = 0;
    local_d8 = param_3;
    lStack_d0 = lVar2;
    local_b8 = param_3;
    local_b0 = param_2;
    local_a8 = param_3;
    local_98 = param_3;
LAB_1001f788c:
    lVar5 = local_c8;
    lVar9 = 0;
    do {
      lVar3 = local_b0;
      <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(local_78,&local_b0);
      uVar1 = local_80;
      if (local_78[0] == 1) {
        uVar7 = local_68 - local_c0;
        pvVar6 = (void *)(lVar3 + local_c0);
        local_c0 = local_68;
        uVar8 = uVar7;
        if (uVar7 == 0) goto LAB_1001f7924;
LAB_1001f78f8:
        uVar8 = uVar7;
        if (*(char *)((long)pvVar6 + (uVar7 - 1)) != '\n') goto LAB_1001f7924;
        if (uVar7 - 1 != 0) {
          uVar8 = uVar7 - 1;
          if (*(char *)((long)pvVar6 + (uVar7 - 2)) == '\r') {
            uVar8 = uVar7 - 2;
          }
          goto LAB_1001f7924;
        }
        uVar8 = 0;
        ::core::str::<impl_str>::trim_matches(pvVar6,0);
        lVar3 = extraout_x1_00;
      }
      else {
        if ((local_80 & 0x100) != 0) goto LAB_1001f7a5c;
        local_80 = CONCAT11(1,(undefined1)local_80);
        if (((uVar1 & 1) == 0) && (local_b8 == local_c0)) goto LAB_1001f7a5c;
        uVar7 = local_b8 - local_c0;
        pvVar6 = (void *)(local_b0 + local_c0);
        uVar8 = uVar7;
        if (uVar7 != 0) goto LAB_1001f78f8;
LAB_1001f7924:
        ::core::str::<impl_str>::trim_matches(pvVar6,uVar8);
        lVar3 = extraout_x1;
      }
      if (lVar3 != 0) goto joined_r0x0001001f7958;
      lVar9 = lVar9 + lVar10;
      if ((local_80 & 0x100) != 0) goto LAB_1001f7a5c;
    } while( true );
  }
  uVar4 = 0;
LAB_1001f7814:
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error(uVar4,param_3);
joined_r0x0001001f7958:
  for (; lVar9 != 0; lVar9 = lVar9 + -1) {
    while (lVar5 != local_d8) {
      *(undefined1 *)(lVar2 + lVar5) = 10;
      lVar5 = lVar5 + 1;
      lVar9 = lVar9 + -1;
      local_c8 = lVar5;
      if (lVar9 == 0) goto LAB_1001f79b4;
    }
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_d8,lVar5,1,1,1);
    *(undefined1 *)(lStack_d0 + local_c8) = 10;
    lVar5 = lVar5 + 1;
    lVar2 = lStack_d0;
    local_c8 = lVar5;
  }
LAB_1001f79b4:
  if ((ulong)(local_d8 - lVar5) < uVar8) {
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_d8,lVar5,uVar8,1,1);
    lVar5 = local_c8;
  }
  lVar2 = lStack_d0;
  lVar10 = local_d8;
  _memcpy((void *)(lStack_d0 + lVar5),pvVar6,uVar8);
  lVar5 = lVar5 + uVar8;
  if (lVar10 == lVar5) {
    local_c8 = lVar5;
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(&local_d8,lVar10,1,1,1);
    *(undefined1 *)(lStack_d0 + local_c8) = 10;
    lVar2 = lStack_d0;
  }
  else {
    *(undefined1 *)(lVar2 + lVar5) = 10;
  }
  lVar10 = 1;
  local_c8 = lVar5 + 1;
  if ((local_80 & 0x100) != 0) {
LAB_1001f7a5c:
    param_1[1] = lStack_d0;
    *param_1 = local_d8;
    param_1[2] = local_c8;
    return;
  }
  goto LAB_1001f788c;
}
