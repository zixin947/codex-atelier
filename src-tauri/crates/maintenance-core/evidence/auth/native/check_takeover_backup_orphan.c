===BEGIN check_takeover_backup_orphan @ 10038fafc=== (GhidraScript)

/* codexmate_lib::core::relay::codex_diagnostic::check_takeover_backup_orphan */

void codexmate_lib::core::relay::codex_diagnostic::check_takeover_backup_orphan(undefined8 *param_1,undefined8 param_2,ulong param_3)

{
  ulong uVar1;
  undefined8 *puVar2;
  undefined4 *puVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  undefined *puVar6;
  undefined8 uVar7;

  uVar1 = config_takeover::backup_exists(param_2);
  if ((uVar1 & 1) == 0) {
    puVar6 = &DAT_10130eba1;
    uVar7 = 0x19;
  }
  else {
    if ((param_3 & 1) == 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar2 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x16,1);
      if (puVar2 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x16);
      }
      puVar2[1] = 0x5f70756b6361625f;
      *puVar2 = 0x7265766f656b6174;
      *(undefined8 *)((long)puVar2 + 0xe) = 0x6e616870726f5f70;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar3 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(7,1);
      if (puVar3 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,7);
      }
      *(undefined4 *)((long)puVar3 + 3) = 0x676e696e;
      *puVar3 = 0x6e726177;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x2b,1);
      if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x2b);
      }
      puVar4[1] = 0xe4ad97e9b385e5b2;
      *puVar4 = 0xb7e5b194e7afb7e8;
      puVar4[3] = 0xe487a4e520726576;
      puVar4[2] = 0x6f656b61742086bd;
      *(undefined8 *)((long)puVar4 + 0x23) = 0xa89ce598ade58dbb;
      *(undefined8 *)((long)puVar4 + 0x1b) = 0xe4bdbbe487a4e520;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x60,1);
      if (puVar5 != (undefined8 *)0x0) {
        puVar5[5] = 0x949ae920694d614d;
        puVar5[4] = 0x6941208ebae484a4;
        puVar5[7] = 0xe58cbcefadb8e4bd;
        puVar5[6] = 0xbbe487a4e5bba6e7;
        puVar5[9] = 0x99e9a088e58e90e5;
        puVar5[8] = 0x9f8ee598bfe894ba;
        puVar5[0xb] = 0x8280e3b6bbe48796;
        puVar5[10] = 0xe6bdbbe487a4e5a4;
        puVar5[1] = 0xb788e6a894e72fb9;
        *puVar5 = 0x96e689b8e4acace7;
        puVar5[3] = 0xe58dbbe4aebde78d;
        puVar5[2] = 0x85e9b194e7afb7e8;
        *param_1 = 0x16;
        param_1[1] = puVar2;
        param_1[3] = 7;
        param_1[2] = 0x16;
        param_1[4] = puVar3;
        param_1[6] = 0x2b;
        param_1[5] = 7;
        param_1[7] = puVar4;
        param_1[9] = 0x60;
        param_1[8] = 0x2b;
        param_1[10] = puVar5;
        param_1[0xb] = 0x60;
        *(undefined1 *)(param_1 + 0xc) = 1;
        return;
      }
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x60);
    }
    puVar6 = &DAT_10130ec45;
    uVar7 = 0x33;
  }
  ok_item(param_1,&_anon_164d03a2dfc98f969363eaf1c50504fd_631,0x16,puVar6,uVar7);
  return;
}

 (GhidraScript)
===END=== (GhidraScript)
