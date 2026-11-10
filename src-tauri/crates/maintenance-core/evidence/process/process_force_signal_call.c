/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* call_mut @ 1007e6ec0 requested 1007e6ec0 */

/* core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut */

void &mut_F>::call_mut(undefined8 *param_1,undefined8 param_2,pid_t *param_3)

{
  uint uVar1;
  pid_t **pppVar2;
  pid_t **pppVar3;
  pid_t **pppVar4;
  int iVar5;
  uint *puVar6;
  undefined8 *puVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  pid_t *local_138;
  pid_t **local_130;
  code *pcStack_128;
  pid_t ***local_120;
  code *pcStack_118;
  ulong *local_d0;
  code *pcStack_c8;
  undefined8 local_c0;
  pid_t **local_b8;
  pid_t **pppStack_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  ulong local_58;
  
  local_138 = param_3;
  iVar5 = _kill(*param_3,9);
  if (iVar5 != 0) {
    puVar6 = (uint *)___error();
    uVar1 = *puVar6;
    local_58 = (ulong)uVar1 << 0x20 | 2;
    if (uVar1 == 3) {
      pppVar2 = (pid_t **)0xb;
      pppVar3 = local_130;
      pppVar4 = (pid_t **)pcStack_128;
      local_a0 = local_120;
    }
    else {
      local_d0 = &local_58;
      pcStack_c8 = <std::io::error::Error_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_b8,s_SIGKILL_failed__10152bb66,&local_d0);
      pcStack_128 = (code *)pppStack_b0;
      local_130 = local_b8;
      local_120 = (pid_t ***)uStack_a8;
      pppVar2 = (pid_t **)0xa;
      pppVar3 = local_b8;
      pppVar4 = pppStack_b0;
      local_a0 = (pid_t ***)uStack_a8;
      if ((local_58 & 3) == 1) {
        puVar7 = (undefined8 *)(local_58 - 1);
        uVar8 = *puVar7;
        puVar9 = *(undefined8 **)(local_58 + 7);
        if ((code *)*puVar9 != (code *)0x0) {
          (*(code *)*puVar9)(uVar8);
        }
        if (puVar9[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar8,puVar9[1],puVar9[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7,0x18,8);
        pppVar2 = (pid_t **)0xa;
        pppVar3 = local_130;
        pppVar4 = (pid_t **)pcStack_128;
        local_a0 = local_120;
      }
    }
    uStack_a8 = pppVar4;
    pppStack_b0 = pppVar3;
    local_b8 = pppVar2;
    if (uVar1 != 3) {
      local_130 = &local_138;
      pcStack_128 = <&T_as_core::fmt::Display>::fmt;
      local_120 = &local_b8;
      pcStack_118 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_d0,s_pid____10152bb79,&local_130);
      core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_b8);
      param_1[1] = pcStack_c8;
      *param_1 = local_d0;
      param_1[2] = local_c0;
      return;
    }
  }
  *param_1 = 0x8000000000000000;
  return;
}

