/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* cleanup_for_native_off @ 1008aaeb8 requested 1008aaeb8 */

/* codexmate_lib::core::relay::router_unlock_auth::cleanup_for_native_off */

ulong codexmate_lib::core::relay::router_unlock_auth::cleanup_for_native_off(long *param_1,long param_2)

{
  int iVar1;
  uint uVar2;
  ulong uVar3;
  long lVar4;
  ulong uVar5;
  long local_180;
  long lStack_178;
  long lStack_170;
  long lStack_168;
  long local_160;
  long lStack_158;
  long lStack_150;
  long lStack_148;
  long local_140;
  long lStack_138;
  uint local_124;
  long local_120;
  long lStack_118;
  long lStack_110;
  long lStack_108;
  long local_100;
  long lStack_f8;
  long lStack_f0;
  long lStack_e8;
  long local_e0;
  long lStack_d8;
  long local_c8;
  uint local_c0;
  uint uStack_bc;
  undefined1 uStack_b8;
  undefined7 uStack_b7;
  undefined1 uStack_b0;
  undefined7 uStack_af;
  undefined1 uStack_a8;
  undefined7 uStack_a7;
  undefined1 uStack_a0;
  undefined7 uStack_9f;
  undefined1 local_98;
  undefined7 uStack_97;
  undefined1 uStack_90;
  undefined7 uStack_8f;
  undefined1 uStack_88;
  undefined7 uStack_87;
  undefined1 uStack_80;
  undefined7 uStack_7f;
  undefined1 local_78;
  undefined7 uStack_77;
  undefined1 uStack_70;
  undefined7 uStack_6f;
  long local_68;
  undefined8 local_60;
  undefined8 uStack_58;

  uVar3 = has_residue();
  if (((uVar3 & 1) == 0) && (uVar3 = legacy_virtual_auth::marker_exists(param_2), (int)uVar3 == 0)) {
    *(undefined1 *)(param_1 + 1) = 0;
LAB_1008aaf64:
    *param_1 = 0xb;
  }
  else {
    uVar3 = platform::paths::CodexPaths::ensure_directories(param_2);
    if (uVar3 == 0) {
      std::path::Path::_join
                (&local_68,*(undefined8 *)(param_2 + 200),*(undefined8 *)(param_2 + 0xd0),
                 ".live-auth.locklive auth.account-domain.lockaccount state.oauth-refresh.lockaccount.jsonaccount credential.account-transition.lockaccount transition"
                 ,0xf);
      uVar3 = account_coordination::FileLock::acquire
                        (&local_c8,local_60,uStack_58,
                         "live auth.account-domain.lockaccount state.oauth-refresh.lockaccount.jsonaccount credential.account-transition.lockaccount transition"
                         ,9);
      local_124 = local_c0;
      lVar4 = local_c8;
      if (local_c8 == 0xb) {
        uVar5 = 0;
      }
      else {
        lStack_f8 = CONCAT71(uStack_8f,uStack_90);
        local_100 = CONCAT71(uStack_97,local_98);
        lStack_e8 = CONCAT71(uStack_7f,uStack_80);
        lStack_f0 = CONCAT71(uStack_87,uStack_88);
        lStack_d8 = CONCAT71(uStack_6f,uStack_70);
        local_e0 = CONCAT71(uStack_77,local_78);
        lStack_118 = CONCAT71(uStack_af,uStack_b0);
        local_120 = CONCAT71(uStack_b7,uStack_b8);
        lStack_108 = CONCAT71(uStack_9f,uStack_a0);
        lStack_110 = CONCAT71(uStack_a7,uStack_a8);
        uVar5 = (ulong)uStack_bc << 0x20;
      }
      if (local_68 != 0) {
        uVar3 = __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_60,local_68,1);
      }
      if (lVar4 == 0xb) {
        iVar1 = legacy_virtual_auth::marker_exists(param_2);
        if (iVar1 == 0) {
          uVar5 = 0;
          lVar4 = 0xb;
        }
        else {
          uVar3 = has_residue(param_2);
          if ((uVar3 & 1) == 0) {
            uVar5 = 0;
          }
          else {
            cleanup_unlocked(&local_c8,param_2);
            uVar5 = (ulong)(byte)local_c0;
            if (local_c8 != 0xb) {
              lStack_158 = CONCAT71(uStack_8f,uStack_90);
              local_160 = CONCAT71(uStack_97,local_98);
              lStack_148 = CONCAT71(uStack_7f,uStack_80);
              lStack_150 = CONCAT71(uStack_87,uStack_88);
              lStack_138 = CONCAT71(uStack_6f,uStack_70);
              local_140 = CONCAT71(uStack_77,local_78);
              lStack_178 = CONCAT71(uStack_af,uStack_b0);
              local_180 = CONCAT71(uStack_b7,uStack_b8);
              lStack_168 = CONCAT71(uStack_9f,uStack_a0);
              lStack_170 = CONCAT71(uStack_a7,uStack_a8);
              uVar5 = (ulong)CONCAT13((undefined1)uStack_bc,local_c0._1_3_) << 8 | (ulong)uStack_bc._1_3_ << 0x28 | uVar5;
              lVar4 = local_c8;
              goto LAB_1008ab0a8;
            }
          }
          legacy_virtual_auth::cleanup(&local_c8,param_2);
          lVar4 = local_c8;
          if (local_c8 == 0xb) {
            uVar5 = (ulong)((uint)uVar5 | local_c0 & 0xff) & 1;
          }
          else {
            lStack_158 = CONCAT71(uStack_8f,uStack_90);
            local_160 = CONCAT71(uStack_97,local_98);
            lStack_148 = CONCAT71(uStack_7f,uStack_80);
            lStack_150 = CONCAT71(uStack_87,uStack_88);
            lStack_138 = CONCAT71(uStack_6f,uStack_70);
            local_140 = CONCAT71(uStack_77,local_78);
            lStack_178 = CONCAT71(uStack_af,uStack_b0);
            local_180 = CONCAT71(uStack_b7,uStack_b8);
            lStack_168 = CONCAT71(uStack_9f,uStack_a0);
            lStack_170 = CONCAT71(uStack_a7,uStack_a8);
            uVar5 = (ulong)CONCAT13((undefined1)uStack_bc,local_c0._1_3_) << 8 | (ulong)uStack_bc._1_3_ << 0x28 | (ulong)(byte)local_c0;
          }
        }
LAB_1008ab0a8:
        <codexmate_lib::core::account_coordination::FileLock_as_core::ops::drop::Drop>::drop(&local_124);
        uVar2 = _close(local_124);
        uVar3 = (ulong)uVar2;
        if (lVar4 == 0xb) {
          uVar3 = has_residue(param_2);
          if ((uVar3 & 1) != 0) {
            uVar3 = cleanup(&local_c8,param_2);
            if (local_c8 != 0xb) {
              *(ulong *)((long)param_1 + 0x31) = CONCAT17(uStack_90,uStack_97);
              *(ulong *)((long)param_1 + 0x29) = CONCAT17(local_98,uStack_9f);
              *(ulong *)((long)param_1 + 0x41) = CONCAT17(uStack_80,uStack_87);
              *(ulong *)((long)param_1 + 0x39) = CONCAT17(uStack_88,uStack_8f);
              *(ulong *)((long)param_1 + 0x51) = CONCAT17(uStack_70,uStack_77);
              *(ulong *)((long)param_1 + 0x49) = CONCAT17(local_78,uStack_7f);
              param_1[0xb] = CONCAT71(uStack_6f,uStack_70);
              *(ulong *)((long)param_1 + 0x11) = CONCAT17(uStack_b0,uStack_b7);
              *(ulong *)((long)param_1 + 9) = CONCAT17(uStack_b8,CONCAT43(uStack_bc,local_c0._1_3_));
              *(ulong *)((long)param_1 + 0x21) = CONCAT17(uStack_a0,uStack_a7);
              *(ulong *)((long)param_1 + 0x19) = CONCAT17(uStack_a8,uStack_af);
              *param_1 = local_c8;
              *(byte *)(param_1 + 1) = (byte)local_c0;
              return uVar3;
            }
            uVar5 = (ulong)(local_c0 & 0xff | (uint)uVar5);
          }
          *(byte *)(param_1 + 1) = (byte)uVar5 & 1;
          goto LAB_1008aaf64;
        }
      }
      else {
        lStack_158 = lStack_f8;
        local_160 = local_100;
        lStack_148 = lStack_e8;
        lStack_150 = lStack_f0;
        lStack_138 = lStack_d8;
        local_140 = local_e0;
        lStack_178 = lStack_118;
        local_180 = local_120;
        lStack_168 = lStack_108;
        lStack_170 = lStack_110;
        uVar5 = uVar5 | local_124;
      }
    }
    else {
      lVar4 = 2;
      uVar5 = uVar3;
    }
    param_1[7] = lStack_158;
    param_1[6] = local_160;
    param_1[9] = lStack_148;
    param_1[8] = lStack_150;
    param_1[0xb] = lStack_138;
    param_1[10] = local_140;
    param_1[3] = lStack_178;
    param_1[2] = local_180;
    param_1[5] = lStack_168;
    param_1[4] = lStack_170;
    *param_1 = lVar4;
    *(char *)(param_1 + 1) = (char)uVar5;
    *(char *)((long)param_1 + 0xf) = (char)(uVar5 >> 0x38);
    *(short *)((long)param_1 + 0xd) = (short)(uVar5 >> 0x28);
    *(int *)((long)param_1 + 9) = (int)(uVar5 >> 8);
  }
  return uVar3;
}
