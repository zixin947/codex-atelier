===BEGIN force_kill_codex @ 1007b3a94=== (GhidraScript)  

/* codexmate_lib::commands::system::force_kill_codex */

ulong codexmate_lib::commands::system::force_kill_codex(undefined8 *param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  undefined1 uVar4;
  undefined4 uVar5;
  uint uVar6;
  int iVar7;
  undefined2 *puVar8;
  undefined4 *puVar9;
  ulong uVar10;
  undefined4 local_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined4 local_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined1 *local_170;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 local_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  long local_150;
  undefined4 uStack_148;
  undefined4 uStack_144;
  uint uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined4 local_e0;
  long local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined4 local_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 *puStack_58;
  undefined **local_50;
  undefined1 *local_48;
  
  core::relay::codex_mutation::CodexMutationGate::lock
            (&local_150,param_2,
             "force closing the ChatGPT desktop appupdating image compatibilityosVersionupdate image compatibility[features]image_generation = false",0x25);
  uVar6 = uStack_140;
  if (local_150 == 0xb) {
    puVar3 = (undefined8 *)CONCAT44(uStack_144,uStack_148);
    platform::process::CodexProcessLifecycle::force_close_all(&local_150);
    if (local_150 == 0xb) {
      uStack_15c = (undefined4)uStack_130;
      uStack_158 = (undefined4)((ulong)uStack_130 >> 0x20);
      uStack_c8 = CONCAT44(uStack_134,uStack_138);
      local_d0 = CONCAT44(uStack_13c,uStack_140);
      local_c0 = uStack_130;
      local_b8 = CONCAT44(local_b8._4_4_,uStack_148);
      uStack_17c = uStack_15c;
      uStack_178 = uStack_158;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar8 = (undefined2 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(2,1);
      if (puVar8 == (undefined2 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,2);
      }
      *puVar8 = 0x6b6f;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar9 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(7,1);
      if (puVar9 != (undefined4 *)0x0) {
        param_1[10] = uStack_c8;
        param_1[9] = local_d0;
        *(undefined4 *)((long)puVar9 + 3) = 0x73736563;
        *puVar9 = 0x63637553;
        param_1[0xc] = local_b8;
        param_1[0xb] = local_c0;
        *param_1 = 2;
        param_1[1] = puVar8;
        param_1[3] = 7;
        param_1[2] = 2;
        param_1[4] = puVar9;
        param_1[6] = 0;
        param_1[5] = 7;
        param_1[7] = 8;
        param_1[8] = 0;
        *(undefined4 *)(param_1 + 0xd) = 1;
        *(undefined1 *)((long)param_1 + 0x6c) = 1;
        if ((((uVar6 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
           (uVar10 = std::panicking::panic_count::is_zero_slow_path(), (uVar10 & 1) == 0)) {
          *(undefined1 *)(puVar3 + 1) = 1;
        }
        uVar6 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*puVar3);
        return (ulong)uVar6;
      }
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,7);
    }
    uStack_a8 = uStack_128;
    local_b0 = uStack_130;
    uStack_98 = uStack_118;
    uStack_a0 = uStack_120;
    uStack_88 = uStack_108;
    local_90 = local_110;
    uStack_78 = uStack_f8;
    uStack_80 = uStack_100;
    local_d0 = local_150;
    local_70 = 0;
    uStack_6c = 0;
    uStack_68 = 1;
    uStack_64 = 0;
    local_48 = &DAT_60000020;
    puStack_58 = &local_70;
    uStack_60 = 0;
    uStack_5c = 0;
    local_50 = &_anon_6408540cdffbe318d49f52812e99783f_1640;
    iVar7 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_d0,&puStack_58);
    uVar5 = local_70;
    if (iVar7 != 0) {
                    /* WARNING: Subroutine does not return */
      ::core::result::unwrap_failed
                ("a Display implementation returned an error unexpectedlycalled `Result::unwrap()` on an `Err` value",0x37,&local_f0,
                 &_anon_6408540cdffbe318d49f52812e99783f_1652,&_anon_6408540cdffbe318d49f52812e99783f_1643);
    }
    uStack_e8 = CONCAT44(uStack_60,uStack_64);
    local_f0 = CONCAT44(uStack_68,uStack_6c);
    local_e0 = uStack_5c;
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_150);
    uStack_188 = (undefined4)uStack_e8;
    local_184 = (undefined4)((ulong)uStack_e8 >> 0x20);
    local_190 = (undefined4)local_f0;
    uStack_18c = (undefined4)((ulong)local_f0 >> 0x20);
    uStack_180 = local_e0;
    *(undefined8 *)((long)param_1 + 0x14) = uStack_e8;
    *(undefined8 *)((long)param_1 + 0xc) = local_f0;
    *(undefined4 *)((long)param_1 + 0x1c) = local_e0;
    *(undefined4 *)(param_1 + 1) = uVar5;
    *param_1 = 0x8000000000000000;
    if ((((uVar6 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
       (uVar10 = std::panicking::panic_count::is_zero_slow_path(), (uVar10 & 1) == 0)) {
      *(undefined1 *)(puVar3 + 1) = 1;
    }
    uVar6 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*puVar3);
    uVar10 = (ulong)uVar6;
  }
  else {
    uStack_a8 = uStack_128;
    local_b0 = uStack_130;
    uStack_98 = uStack_118;
    uStack_a0 = uStack_120;
    uStack_88 = uStack_108;
    local_90 = local_110;
    uStack_78 = uStack_f8;
    uStack_80 = uStack_100;
    local_d0 = local_150;
    local_190 = 0;
    uStack_18c = 0;
    uStack_188 = 1;
    local_184 = 0;
    uStack_180 = 0;
    uStack_17c = 0;
    local_160 = 0x60000020;
    uStack_15c = 0;
    uStack_168 = 0x182bcf8;
    uStack_164 = 1;
    local_170 = (undefined1 *)&local_190;
    iVar7 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_d0,&local_170);
    if (iVar7 != 0) {
                    /* WARNING: Subroutine does not return */
      ::core::result::unwrap_failed
                ("a Display implementation returned an error unexpectedlycalled `Result::unwrap()` on an `Err` value",0x37,&local_f0,
                 &_anon_6408540cdffbe318d49f52812e99783f_1652,&_anon_6408540cdffbe318d49f52812e99783f_1643);
    }
    uVar1 = CONCAT44(uStack_18c,local_190);
    uVar2 = CONCAT44(local_184,uStack_188);
    uVar4 = (undefined1)uStack_180;
    *(uint *)((long)param_1 + 0x19) = CONCAT13((undefined1)uStack_17c,uStack_180._1_3_);
    *(undefined4 *)((long)param_1 + 0x1c) = uStack_17c;
    uVar10 = ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_150);
    param_1[1] = uVar1;
    param_1[2] = uVar2;
    *(undefined1 *)(param_1 + 3) = uVar4;
    *param_1 = 0x8000000000000000;
  }
  return uVar10;
}

 (GhidraScript)  
===END=== (GhidraScript)  
