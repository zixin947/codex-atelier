/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* spawn_unchecked @ 10044f568 requested 10044f568 */

/* std::thread::lifecycle::spawn_unchecked */

void std::thread::lifecycle::spawn_unchecked(undefined8 *param_1,undefined8 param_2,ulong param_3,ulong param_4,ulong param_5,long param_6,long *param_7)

{
  byte bVar1;
  uint uVar2;
  code *pcVar3;
  long lVar4;
  undefined8 uVar5;
  long *plVar6;
  undefined8 *puVar7;
  ulong uVar8;
  ulong uVar9;
  ulong uVar10;
  ulong uVar11;
  undefined1 auVar12 [16];
  long *local_130;
  long *local_128;
  long lStack_120;
  long local_118;
  long lStack_110;
  long *local_108;
  long *local_100;
  long lStack_f8;
  long lStack_f0;
  long lStack_e8;
  long *local_e0;
  long lStack_d8;
  long lStack_d0;
  long local_c8;
  long lStack_c0;
  long *local_b0;
  byte *pbStack_a8;
  undefined **local_a0;
  long lStack_98;
  long *local_90;
  long lStack_88;
  long lStack_80;
  long lStack_78;
  long local_70;
  
  if ((param_3 & 1) != 0) goto LAB_10044f6fc;
  if (spawn_unchecked::{{closure}}::MIN != 0) {
    param_4 = spawn_unchecked::{{closure}}::MIN - 1;
    goto LAB_10044f6fc;
  }
  env::_var_os(&local_100,
               "RUST_MIN_STACKthreads should not terminate unexpectedlyLarge file option has not been setZipWriter was already closedUnsupported compression levelAES encryption is enabled through FileOptions::with_aes_encryptionUnsupported compressionShould have switched to stored and unencrypted beforehandAttempted to update a nonexistent ZIP64 extra fieldsocks4socks4asocks5socks5ha Display implementation returned an error unexpectedlydescription() is deprecated; use Displaymid > lencalled `Result::unwrap()` on an `Err` valueErrorPatternErrorposmsginnerEmptyHostIdnaErrorInvalidPortInvalidIpv4AddressInvalidIpv6AddressInvalidDomainCharacterRelativeUrlWithoutBaseRelativeUrlWithCannotBeABaseBaseSetHostOnCannotBeABaseUrlUtf8Errorvalid_up_toerror_lenCustomNotEofPoisonErrorpoisoned lock: another task failed insidefatal runtime error: thread result panicked on drop, aborting\nnext_value_seed called before next_key_seedull"
               ,0xe);
  lVar4 = lStack_f8;
  plVar6 = local_100;
  if (local_100 == (long *)0x8000000000000000) {
    param_4 = 0x200000;
  }
  else {
    core::str::converts::from_utf8(&local_b0,lStack_f8,lStack_f0);
    if (((int)local_b0 == 1) || (local_a0 == (undefined **)0x0)) {
      param_4 = 0x200000;
    }
    else {
      if (local_a0 == (undefined **)0x1) {
        bVar1 = *pbStack_a8;
        param_4 = 0x200000;
        if ((bVar1 == 0x2b) || (bVar1 == 0x2d)) goto joined_r0x00010044f8ec;
      }
      else {
        bVar1 = *pbStack_a8;
      }
      if (bVar1 == 0x2b) {
        pbStack_a8 = pbStack_a8 + 1;
      }
      uVar8 = (long)local_a0 - (ulong)(bVar1 == 0x2b);
      if (uVar8 < 0x11) {
        param_4 = 0;
        for (; uVar8 != 0; uVar8 = uVar8 - 1) {
          if (9 < *pbStack_a8 - 0x30) {
            param_4 = 0x200000;
            break;
          }
          param_4 = param_4 * 10 + (ulong)(*pbStack_a8 - 0x30);
          pbStack_a8 = pbStack_a8 + 1;
        }
      }
      else {
        uVar9 = 0;
        uVar11 = 0x200000;
        while ((param_4 = uVar9, uVar8 != 0 && (auVar12._8_8_ = 0, auVar12._0_8_ = uVar9, param_4 = uVar11, SUB168(auVar12 * ZEXT816(10),8) == 0))) {
          uVar10 = uVar9 * 10;
          uVar2 = *pbStack_a8 - 0x30;
          uVar9 = uVar10 + uVar2;
          if ((9 < uVar2) || (uVar8 = uVar8 - 1, pbStack_a8 = pbStack_a8 + 1, CARRY8(uVar10,(ulong)uVar2))) break;
        }
      }
    }
joined_r0x00010044f8ec:
    if (plVar6 != (long *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar4,plVar6,1);
    }
  }
  spawn_unchecked::{{closure}}::MIN = param_4 + 1;
LAB_10044f6fc:
  uVar5 = id::ThreadId::new();
  local_130 = (long *)thread::Thread::new(uVar5,param_2);
  if ((param_5 & 1) == 0) {
    spawnhook::run_spawn_hooks(&local_128,&local_130);
  }
  else {
    local_128 = (long *)0x0;
    lStack_120 = 8;
    local_118 = 0;
    lStack_110 = 0;
  }
  pbStack_a8 = (byte *)0x1;
  local_b0 = (long *)0x1;
  lStack_98 = 0;
  local_a0 = (undefined **)param_6;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  local_108 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x30,8);
  if (local_108 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
    ::alloc::alloc::handle_alloc_error(8,0x30);
  }
  local_108[1] = (long)pbStack_a8;
  *local_108 = (long)local_b0;
  local_108[3] = lStack_98;
  local_108[2] = (long)local_a0;
  local_108[5] = lStack_88;
  local_108[4] = (long)local_90;
  lVar4 = *local_108;
  *local_108 = lVar4 + 1;
  if (-1 < lVar4) {
    lStack_d0 = param_7[1];
    lStack_d8 = *param_7;
    lStack_c0 = param_7[3];
    local_c8 = param_7[2];
    lStack_f8 = lStack_120;
    local_100 = local_128;
    lStack_e8 = lStack_110;
    lStack_f0 = local_118;
    local_e0 = local_108;
    if (local_108[2] != 0) {
      scoped::ScopeData::increment_num_running_threads(local_108[2] + 0x10);
    }
    lStack_88 = lStack_d8;
    local_90 = local_e0;
    lStack_78 = local_c8;
    lStack_80 = lStack_d0;
    local_70 = lStack_c0;
    pbStack_a8 = (byte *)lStack_f8;
    local_b0 = local_100;
    lStack_98 = lStack_e8;
    local_a0 = (undefined **)lStack_f0;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    plVar6 = (long *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x48,8);
    if (plVar6 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
      ::alloc::alloc::handle_alloc_error(8,0x48);
    }
    plVar6[5] = lStack_d8;
    plVar6[4] = (long)local_e0;
    plVar6[7] = local_c8;
    plVar6[6] = lStack_d0;
    plVar6[8] = lStack_c0;
    plVar6[1] = lStack_f8;
    *plVar6 = (long)local_100;
    plVar6[3] = lStack_e8;
    plVar6[2] = lStack_f0;
    lVar4 = *local_130;
    *local_130 = lVar4 + 1;
    if (-1 < lVar4) {
      local_b0 = local_130;
      local_a0 = &
                 PTR_drop_in_place<std::thread::lifecycle::spawn_unchecked<codexmate_lib::platform::process::CodexTransitionFence::acquire_with_policy::__closure__,()>::__closure__>_10181cf60
      ;
      pbStack_a8 = (byte *)plVar6;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
      if (puVar7 != (undefined8 *)0x0) {
        puVar7[1] = pbStack_a8;
        *puVar7 = local_b0;
        puVar7[2] = local_a0;
        auVar12 = sys::thread::unix::Thread::new(param_4,puVar7);
        if ((auVar12._0_8_ & 1) == 0) {
          *param_1 = local_130;
          param_1[1] = local_108;
          param_1[2] = auVar12._8_8_;
        }
        else {
          *param_1 = 0;
          param_1[1] = auVar12._8_8_;
          lVar4 = *local_108;
          *local_108 = lVar4 + -1;
          LORelease();
          if (lVar4 == 1) {
            DataMemoryBarrier(2,1);
            ::alloc::sync::Arc<T,A>::drop_slow(&local_108);
          }
          lVar4 = *local_130;
          *local_130 = lVar4 + -1;
          LORelease();
          if (lVar4 == 1) {
            DataMemoryBarrier(2,1);
            ::alloc::sync::Arc<T,A>::drop_slow(&local_130);
          }
        }
        return;
      }
                    /* WARNING: Subroutine does not return */
      ::alloc::alloc::handle_alloc_error(8,0x18);
    }
  }
                    /* WARNING: Does not return */
  pcVar3 = (code *)SoftwareBreakpoint(1,0x10044f8e8);
  (*pcVar3)();
}

