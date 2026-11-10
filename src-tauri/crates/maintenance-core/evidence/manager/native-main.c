===BEGIN fix_diagnostic_issue_audited @ 100682818=== (GhidraScript)

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::manager::RelayManager::fix_diagnostic_issue_audited */

ulong codexmate_lib::core::relay::manager::RelayManager::fix_diagnostic_issue_audited
                (long *param_1,long *******param_2,long ****param_3,long ******param_4,long *******param_5)

{
  long ******pppppplVar1;
  long ******pppppplVar2;
  long ******pppppplVar3;
  long *******ppppppplVar4;
  byte bVar5;
  long *******ppppppplVar6;
  undefined7 uVar7;
  undefined1 uVar8;
  undefined7 uVar9;
  undefined1 uVar10;
  undefined7 uVar11;
  undefined1 uVar12;
  undefined7 uVar13;
  undefined1 uVar14;
  undefined7 uVar15;
  undefined1 uVar16;
  undefined7 uVar17;
  undefined1 uVar18;
  undefined7 uVar19;
  undefined1 uVar20;
  undefined7 uVar21;
  undefined1 uVar22;
  undefined7 uVar23;
  bool bVar24;
  char cVar25;
  byte bVar26;
  byte bVar27;
  uint uVar28;
  int iVar29;
  uint uVar30;
  ulong uVar31;
  undefined8 *puVar32;
  long *******ppppppplVar33;
  undefined8 uVar34;
  long *****ppppplVar35;
  long *****ppppplVar36;
  long lVar37;
  long ******pppppplVar38;
  long *****local_10f8;
  long ******local_10f0;
  undefined1 auStack_10e8 [136];
  undefined8 local_1060;
  byte local_f7c;
  long *******local_f68;
  long *******local_f60;
  long *******ppppppplStack_f58;
  long *******local_f50;
  long *******ppppppplStack_f48;
  long *******local_f40;
  long *******ppppppplStack_f38;
  long *******local_f30;
  long *******ppppppplStack_f28;
  long *******local_f20;
  long *******ppppppplStack_f18;
  long *******local_f10;
  long *******ppppppplStack_f08;
  undefined1 auStack_f00 [640];
  long local_c80;
  long *******local_c78;
  long local_c70;
  long *******local_c60;
  long *******ppppppplStack_c58;
  long *******local_c50;
  long *******ppppppplStack_c48;
  long *******local_c40;
  long *******ppppppplStack_c38;
  long *******local_c30;
  long *******ppppppplStack_c28;
  long *******local_c20;
  long *******ppppppplStack_c18;
  long *******local_c10;
  long *******ppppppplStack_c08;
  long *******local_c00;
  long *******ppppppplStack_bf8;
  long *******local_bf0;
  long *******ppppppplStack_be8;
  long *******local_be0;
  long *******ppppppplStack_bd8;
  long *******local_bd0;
  long *******ppppppplStack_bc8;
  long *******local_bc0;
  long *******ppppppplStack_bb8;
  long *******local_bb0;
  long *******ppppppplStack_ba8;
  long *******local_ba0;
  undefined8 uStack_b98;
  undefined8 local_b90;
  undefined8 uStack_b88;
  long *******local_b80;
  long *******ppppppplStack_b78;
  long *******ppppppplStack_b70;
  long *******ppppppplStack_b68;
  long *******local_b60;
  long *******ppppppplStack_b58;
  long *******ppppppplStack_b50;
  long *******ppppppplStack_b48;
  long *******local_b40;
  long *******ppppppplStack_b38;
  long *******ppppppplStack_b30;
  long *******ppppppplStack_b28;
  long *******local_b20;
  long *******ppppppplStack_b18;
  long *******ppppppplStack_b10;
  long *******ppppppplStack_b08;
  long *******local_b00;
  long *******ppppppplStack_af8;
  long *******ppppppplStack_af0;
  long *******ppppppplStack_ae8;
  long *******local_ae0;
  long *******ppppppplStack_ad8;
  long *******ppppppplStack_ad0;
  long *******ppppppplStack_ac8;
  long *******local_ac0;
  undefined8 uStack_ab8;
  undefined8 uStack_ab0;
  undefined8 uStack_aa8;
  long *******local_a98;
  long *******local_a90;
  long *******ppppppplStack_a88;
  long *******local_a80;
  long *******ppppppplStack_a78;
  long *******local_a70;
  long *******ppppppplStack_a68;
  long *******local_a60;
  long *******ppppppplStack_a58;
  long *******local_a50;
  long *******ppppppplStack_a48;
  long *******local_a40;
  long *******ppppppplStack_a38;
  long *******local_a30;
  long *******ppppppplStack_a28;
  long *******local_a20;
  long *******ppppppplStack_a18;
  long *******local_a10;
  long *******ppppppplStack_a08;
  long *******local_a00;
  long *******ppppppplStack_9f8;
  long *******local_9f0;
  long *******ppppppplStack_9e8;
  long *******local_9e0;
  long *******ppppppplStack_9d8;
  long *******local_9d0;
  long *******ppppppplStack_9c8;
  long *******local_9c0;
  undefined8 uStack_9b8;
  undefined8 local_9b0;
  undefined8 uStack_9a8;
  long *******local_9a0;
  long *******ppppppplStack_998;
  byte local_981;
  long *******local_980;
  long *******ppppppplStack_978;
  long *******local_970;
  long *******local_968;
  long *******ppppppplStack_960;
  long *******ppppppplStack_958;
  long *******ppppppplStack_950;
  long *******ppppppplStack_948;
  long *******local_940;
  long *******ppppppplStack_938;
  long *******local_930;
  long *******ppppppplStack_928;
  long *******local_918;
  long *******ppppppplStack_910;
  undefined8 local_908;
  undefined8 uStack_900;
  undefined8 local_8f8;
  undefined8 uStack_8f0;
  undefined8 local_8e8;
  undefined8 uStack_8e0;
  undefined8 local_8d8;
  undefined8 uStack_8d0;
  undefined8 local_8c8;
  long *******ppppppplStack_8c0;
  undefined1 auStack_8b8 [24];
  long local_8a0;
  long lStack_898;
  long local_890;
  long *******local_888;
  long *****local_880;
  long *******local_878;
  undefined1 *local_870;
  long *******local_868;
  long *******local_860;
  long *******local_858;
  long *******local_850;
  long *******ppppppplStack_848;
  long *******local_840;
  long *******ppppppplStack_838;
  long *******local_830;
  long *******ppppppplStack_828;
  long *******ppppppplStack_820;
  long *******ppppppplStack_818;
  long *******local_810;
  long *******ppppppplStack_808;
  long *******local_800;
  long *******ppppppplStack_7f8;
  undefined8 local_7f0;
  undefined8 uStack_7e8;
  undefined8 local_7e0;
  byte local_6e4;
  undefined1 auStack_6d2 [2];
  long *******local_6d0;
  long *******ppppppplStack_6c8;
  byte bStack_6c0;
  undefined7 uStack_6bf;
  undefined1 uStack_6b8;
  undefined7 uStack_6b7;
  undefined1 uStack_6b0;
  undefined7 uStack_6af;
  undefined1 local_6a8;
  undefined7 uStack_6a7;
  undefined1 uStack_6a0;
  undefined7 uStack_69f;
  undefined1 local_698;
  undefined7 uStack_697;
  undefined1 uStack_690;
  undefined7 uStack_68f;
  undefined1 local_688;
  undefined7 uStack_687;
  undefined1 uStack_680;
  undefined7 uStack_67f;
  long *******local_678;
  long *******ppppppplStack_670;
  long *******local_668;
  long *******ppppppplStack_660;
  long *******local_658;
  long *******ppppppplStack_650;
  long *******local_648;
  long *******ppppppplStack_640;
  long *******local_638;
  long *******ppppppplStack_630;
  long *******local_628;
  long *******ppppppplStack_620;
  long *******local_618;
  long *******ppppppplStack_610;
  undefined8 local_608;
  undefined8 uStack_600;
  undefined8 local_5f8;
  byte local_3e1;
  long *******local_3e0;
  long *******ppppppplStack_3d8;
  long *******ppppppplStack_3d0;
  long *******local_3c8;
  long *******ppppppplStack_3c0;
  long *******local_3b8;
  long *******ppppppplStack_3b0;
  long *******local_3a8;
  long *******ppppppplStack_3a0;
  long *******local_398;
  long *******ppppppplStack_390;
  long *******local_388;
  long *******ppppppplStack_380;
  byte local_378;
  byte bStack_377;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 local_360;
  undefined8 uStack_358;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined8 local_340;
  long *******local_330;
  long *******ppppppplStack_328;
  long *******local_320;
  long *******local_318;
  long *******local_310;
  long *******ppppppplStack_308;
  long *******ppppppplStack_300;
  long *******ppppppplStack_2f8;
  long *******local_2f0;
  long *******ppppppplStack_2e8;
  long *******ppppppplStack_2e0;
  long *******ppppppplStack_2d8;
  long *******local_2d0;
  undefined8 local_2c0;
  undefined8 uStack_2b8;
  undefined8 local_2b0;
  undefined8 uStack_2a8;
  undefined8 local_2a0;
  undefined8 uStack_298;
  undefined8 local_290;
  long *******local_280;
  long *******ppppppplStack_278;
  long *******local_270;
  long *******local_268;
  long *******ppppppplStack_260;
  long *******local_258;
  long *******local_250;
  long *******ppppppplStack_248;
  long *******local_240;
  long *******ppppppplStack_238;
  long *******ppppppplStack_230;
  long *******ppppppplStack_228;
  long *******local_220;
  long *******ppppppplStack_218;
  long *******local_210;
  long *******ppppppplStack_208;
  long *******ppppppplStack_200;
  long *******local_1f8;
  long *******ppppppplStack_1f0;
  long *******local_1e8;
  long *******ppppppplStack_1e0;
  long *******local_1d8;
  long *******ppppppplStack_1d0;
  long *******local_1c0;
  long *******ppppppplStack_1b8;
  long *******local_1b0;
  long *******local_1a8;
  long *******local_1a0;
  long *******ppppppplStack_198;
  long *******ppppppplStack_190;
  long *******ppppppplStack_188;
  long *******local_180;
  long *******ppppppplStack_178;
  long *******ppppppplStack_170;
  long *******ppppppplStack_168;
  long *******local_160;
  long *******ppppppplStack_158;
  long *******local_150;
  long *******local_148;
  long *******ppppppplStack_140;
  long *******ppppppplStack_138;
  long *******local_130;
  long *******local_128;
  long *******local_120;
  long *******ppppppplStack_118;
  long *******ppppppplStack_110;
  long *******local_108;
  long *******ppppppplStack_100;
  long *******local_f8;
  long *******ppppppplStack_f0;
  long *******local_e8;
  long *******ppppppplStack_e0;
  long *******local_d8;
  long *******local_d0;
  long local_c8;
  long *******local_c0;
  long *******local_a8;
  long *******local_a0;
  long *******ppppppplStack_98;
  long *******local_90;
  long *******ppppppplStack_88;
  long *******local_80;

  local_1060 = 0;
  local_10f8 = (long *****)param_3;
  local_10f0 = param_4;
  if (param_2[2][2] != (long *****)0x8000000000000000) {
    uVar31 = <alloc::string::String_as_core::clone::Clone>::clone(&ppppppplStack_6c8);
    param_1[5] = CONCAT71(uStack_6a7,local_6a8);
    param_1[4] = CONCAT71(uStack_6af,uStack_6b0);
    param_1[7] = CONCAT71(uStack_697,local_698);
    param_1[6] = CONCAT71(uStack_69f,uStack_6a0);
    param_1[9] = CONCAT71(uStack_687,local_688);
    param_1[8] = CONCAT71(uStack_68f,uStack_690);
    param_1[0xb] = (long)local_678;
    param_1[10] = CONCAT71(uStack_67f,uStack_680);
    param_1[1] = (long)ppppppplStack_6c8;
    *param_1 = 9;
    param_1[3] = CONCAT71(uStack_6b7,uStack_6b8);
    param_1[2] = CONCAT71(uStack_6bf,bStack_6c0);
    return uVar31;
  }
  if (((param_4 != (long ******)0x3) || (*(short *)param_3 != 0x6c61 || *(char *)((long)param_3 + 2) != 'l')) &&
     (cVar25 = codex_diagnostic::fix_requires_quiet_window(param_3,param_4), cVar25 == '\x02')) {
    local_6d0 = (long *******)&local_10f8;
    ppppppplStack_6c8 = (long *******)&<&T_as_core::fmt::Display>::fmt;
    uVar31 = alloc::fmt::format::format_inner(param_1 + 1,s__unknown_or_non_fixable_diagnost_101528a4f,&local_6d0);
    *param_1 = 10;
    return uVar31;
  }
  pppppplVar38 = param_2[4];
  if (pppppplVar38[2] == (long *****)0x0) {
    std::sys::sync::once_box::OnceBox<T>::initialize(pppppplVar38 + 2);
    std::sys::pal::unix::sync::mutex::Mutex::lock();
    if ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) == 0) goto LAB_100682934;
LAB_1006829d8:
    uVar28 = std::panicking::panic_count::is_zero_slow_path();
    uVar28 = uVar28 ^ 1;
    cVar25 = *(char *)(pppppplVar38 + 3);
  }
  else {
    std::sys::pal::unix::sync::mutex::Mutex::lock();
    if ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0) goto LAB_1006829d8;
LAB_100682934:
    uVar28 = 0;
    cVar25 = *(char *)(pppppplVar38 + 3);
  }
  if (cVar25 != '\0') {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar32 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x28,1);
    if (puVar32 != (undefined8 *)0x0) {
      puVar32[1] = 0x6172742072657475;
      *puVar32 = 0x6f72207865646f43;
      puVar32[3] = 0x207369206b636f6c;
      puVar32[2] = 0x206e6f697469736e;
      puVar32[4] = 0x64656e6f73696f70;
      if ((((uVar28 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
         (uVar31 = std::panicking::panic_count::is_zero_slow_path(), (uVar31 & 1) == 0)) {
        *(undefined1 *)(pppppplVar38 + 3) = 1;
      }
      uVar28 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)pppppplVar38[2]);
      *param_1 = 10;
      param_1[1] = 0x28;
      *(char *)(param_1 + 2) = (char)puVar32;
      *(char *)((long)param_1 + 0x17) = (char)((ulong)puVar32 >> 0x38);
      *(short *)((long)param_1 + 0x15) = (short)((ulong)puVar32 >> 0x28);
      *(int *)((long)param_1 + 0x11) = (int)((ulong)puVar32 >> 8);
      param_1[3] = 0x28;
      return (ulong)uVar28;
    }
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x28);
  }
  uVar31 = codex_mutation::CodexMutationGate::lock(&local_6d0,param_2 + 5,&DAT_1013a3553,0x1b);
  bVar5 = bStack_6c0;
  ppppppplVar4 = ppppppplStack_6c8;
  if (local_6d0 != (long *******)&DAT_0000000b) {
    *(ulong *)((long)param_1 + 0x29) = CONCAT17(uStack_6a0,uStack_6a7);
    *(ulong *)((long)param_1 + 0x21) = CONCAT17(local_6a8,uStack_6af);
    *(ulong *)((long)param_1 + 0x39) = CONCAT17(uStack_690,uStack_697);
    *(ulong *)((long)param_1 + 0x31) = CONCAT17(local_698,uStack_69f);
    *(ulong *)((long)param_1 + 0x49) = CONCAT17(uStack_680,uStack_687);
    *(ulong *)((long)param_1 + 0x41) = CONCAT17(local_688,uStack_68f);
    param_1[0xb] = (long)local_678;
    param_1[10] = CONCAT71(uStack_67f,uStack_680);
    *(ulong *)((long)param_1 + 0x19) = CONCAT17(uStack_6b0,uStack_6b7);
    *(ulong *)((long)param_1 + 0x11) = CONCAT17(uStack_6b8,uStack_6bf);
    *param_1 = (long)local_6d0;
    param_1[1] = (long)ppppppplStack_6c8;
    *(byte *)(param_1 + 2) = bStack_6c0;
    goto joined_r0x000100683cd8;
  }
  snapshot(auStack_10e8,param_2);
  transition_paths(&local_6d0,param_2);
  ppppppplVar33 = local_6d0;
  ppppppplStack_828 = (long *******)CONCAT71(uStack_69f,uStack_6a0);
  local_830 = (long *******)CONCAT71(uStack_6a7,local_6a8);
  ppppppplStack_818 = (long *******)CONCAT71(uStack_68f,uStack_690);
  ppppppplStack_820 = (long *******)CONCAT71(uStack_697,local_698);
  ppppppplStack_808 = (long *******)CONCAT71(uStack_67f,uStack_680);
  local_810 = (long *******)CONCAT71(uStack_687,local_688);
  ppppppplStack_7f8 = ppppppplStack_670;
  local_800 = local_678;
  ppppppplStack_848 = (long *******)CONCAT71(uStack_6bf,bStack_6c0);
  ppppppplStack_838 = (long *******)CONCAT71(uStack_6af,uStack_6b0);
  local_840 = (long *******)CONCAT71(uStack_6b7,uStack_6b8);
  local_850 = ppppppplStack_6c8;
  if (local_6d0 != (long *******)&DAT_8000000000000000) {
    _memcpy(auStack_f00,&local_668,0x280);
    ppppppplStack_f38 = ppppppplStack_828;
    local_f40 = local_830;
    ppppppplStack_f28 = ppppppplStack_818;
    local_f30 = ppppppplStack_820;
    ppppppplStack_f18 = ppppppplStack_808;
    local_f20 = local_810;
    ppppppplStack_f08 = ppppppplStack_7f8;
    local_f10 = local_800;
    ppppppplStack_f58 = ppppppplStack_848;
    local_f60 = local_850;
    ppppppplStack_f48 = ppppppplStack_838;
    local_f50 = local_840;
    local_f68 = ppppppplVar33;
    codex_diagnostic::run_diagnostics_with_mode(&local_c80,&local_f68,auStack_10e8,1);
    pppppplVar1 = local_10f0;
    ppppplVar36 = local_10f8;
    if (local_c70 != 0) {
      lVar37 = local_c70 * 0x68;
      if (local_10f0 == (long ******)0x3) {
        ppppppplVar33 = local_c78 + 0xc;
        do {
          pppppplVar1 = ppppppplVar33[-0xb];
          if (ppppppplVar33[-10] == (long ******)0x13) {
            ppppplVar36 = *(long ******)((long)pppppplVar1 + 0xb);
            bVar24 = false;
            if (*pppppplVar1 == (long *****)0x616870726f5f6264) {
              bVar24 = pppppplVar1[1] == (long *****)0x6469766f72705f6e;
            }
LAB_100682c14:
            if ((((bVar24 && ppppplVar36 == (long *****)0x7372656469766f72) && (((ulong)*ppppppplVar33 & 1) != 0)) &&
                ((ppppppplVar33[-7] != (long ******)0x2 || (*(short *)ppppppplVar33[-8] != 0x6b6f)))) &&
               (*(short *)local_10f8 == 0x6c61 && *(char *)((long)local_10f8 + 2) == 'l')) goto LAB_100683244;
          }
          else if (ppppppplVar33[-10] == (long ******)0x18) {
            ppppplVar36 = pppppplVar1[2];
            bVar24 = *pppppplVar1 == (long *****)0x5f74756f6c6c6f72 && pppppplVar1[1] == (long *****)0x705f6e616870726f;
            goto LAB_100682c14;
          }
          ppppppplVar33 = ppppppplVar33 + 0xd;
          lVar37 = lVar37 + -0x68;
        } while (lVar37 != 0);
      }
      else {
        ppppppplVar33 = local_c78 + 0xc;
        do {
          pppppplVar2 = ppppppplVar33[-0xb];
          pppppplVar3 = ppppppplVar33[-10];
          if (pppppplVar3 == (long ******)0x18) {
            ppppplVar35 = pppppplVar2[2];
            bVar24 = false;
            if (*pppppplVar2 == (long *****)0x5f74756f6c6c6f72) {
              bVar24 = pppppplVar2[1] == (long *****)0x705f6e616870726f;
            }
LAB_100682cc0:
            if ((bVar24 && ppppplVar35 == (long *****)0x7372656469766f72) && (((ulong)*ppppppplVar33 & 1) != 0)) {
              if (ppppppplVar33[-7] == (long ******)0x2) {
                if ((pppppplVar1 == pppppplVar3) && (*(short *)ppppppplVar33[-8] != 0x6b6f)) {
LAB_100682c58:
                  iVar29 = _memcmp(ppppplVar36,pppppplVar2,(size_t)pppppplVar3);
                  if (iVar29 == 0) goto LAB_100683244;
                }
              }
              else if (pppppplVar1 == pppppplVar3) goto LAB_100682c58;
            }
          }
          else if (pppppplVar3 == (long ******)0x13) {
            ppppplVar35 = *(long ******)((long)pppppplVar2 + 0xb);
            bVar24 = *pppppplVar2 == (long *****)0x616870726f5f6264 && pppppplVar2[1] == (long *****)0x6469766f72705f6e;
            goto LAB_100682cc0;
          }
          ppppppplVar33 = ppppppplVar33 + 0xd;
          lVar37 = lVar37 + -0x68;
        } while (lVar37 != 0);
      }
    }
    local_c60 = (long *******)&DAT_8000000000000001;
    goto LAB_100682d1c;
  }
  param_1[5] = (long)ppppppplStack_828;
  param_1[4] = (long)local_830;
  param_1[7] = (long)ppppppplStack_818;
  param_1[6] = (long)ppppppplStack_820;
  param_1[9] = (long)ppppppplStack_808;
  param_1[8] = (long)local_810;
  param_1[0xb] = (long)ppppppplStack_670;
  param_1[10] = (long)local_678;
  param_1[1] = (long)ppppppplStack_848;
  *param_1 = (long)ppppppplStack_6c8;
  param_1[3] = (long)ppppppplStack_838;
  param_1[2] = (long)local_840;
LAB_100683774:
  ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayState>(auStack_10e8);
  if ((((bVar5 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
     (uVar31 = std::panicking::panic_count::is_zero_slow_path(), (uVar31 & 1) == 0)) {
    *(undefined1 *)(ppppppplVar4 + 1) = 1;
  }
  uVar30 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*ppppppplVar4);
  uVar31 = (ulong)uVar30;
  goto joined_r0x000100683cd8;
LAB_100683244:
  native_repair_catalog::PreparedRepairCatalog::prepare(&local_6d0,&local_f68,auStack_10e8);
  local_c30 = (long *******)CONCAT71(uStack_69f,uStack_6a0);
  ppppppplStack_c38 = (long *******)CONCAT71(uStack_6a7,local_6a8);
  local_c20 = (long *******)CONCAT71(uStack_68f,uStack_690);
  ppppppplStack_c28 = (long *******)CONCAT71(uStack_697,local_698);
  local_c10 = (long *******)CONCAT71(uStack_67f,uStack_680);
  ppppppplStack_c18 = (long *******)CONCAT71(uStack_687,local_688);
  ppppppplStack_b28 = ppppppplStack_670;
  ppppppplStack_b30 = local_678;
  local_c50 = (long *******)CONCAT71(uStack_6bf,bStack_6c0);
  local_c40 = (long *******)CONCAT71(uStack_6af,uStack_6b0);
  ppppppplStack_c48 = (long *******)CONCAT71(uStack_6b7,uStack_6b8);
  local_b80 = ppppppplStack_6c8;
  ppppppplStack_848 = ppppppplStack_660;
  local_850 = local_668;
  ppppppplStack_838 = ppppppplStack_650;
  local_840 = local_658;
  ppppppplStack_828 = ppppppplStack_640;
  local_830 = local_648;
  ppppppplStack_818 = ppppppplStack_630;
  ppppppplStack_820 = local_638;
  ppppppplStack_808 = ppppppplStack_620;
  local_810 = local_628;
  ppppppplStack_7f8 = ppppppplStack_610;
  local_800 = local_618;
  uStack_7e8 = uStack_600;
  local_7f0 = local_608;
  local_c60 = local_6d0;
  local_7e0 = local_5f8;
  ppppppplStack_c58 = ppppppplStack_6c8;
  local_c00 = ppppppplStack_670;
  ppppppplStack_c08 = local_678;
  if (local_6d0 == (long *******)&DAT_8000000000000001) {
    local_c60 = (long *******)&DAT_8000000000000002;
LAB_100683710:
    ppppppplStack_b78 = local_c50;
    ppppppplStack_b70 = ppppppplStack_c48;
    if ((param_4 != (long ******)0x3) || (*(short *)param_3 != 0x6c61 || *(char *)((long)param_3 + 2) != 'l')) {
      param_1[5] = (long)local_c30;
      param_1[4] = (long)ppppppplStack_c38;
      param_1[7] = (long)local_c20;
      param_1[6] = (long)ppppppplStack_c28;
      param_1[9] = (long)local_c10;
      param_1[8] = (long)ppppppplStack_c18;
      param_1[0xb] = (long)ppppppplStack_670;
      param_1[10] = (long)local_678;
      param_1[1] = (long)local_c50;
      *param_1 = (long)ppppppplStack_6c8;
      param_1[3] = (long)local_c40;
      param_1[2] = (long)ppppppplStack_c48;
      ppppppplStack_b68 = local_c40;
      local_b60 = ppppppplStack_c38;
      ppppppplStack_b58 = local_c30;
      ppppppplStack_b50 = ppppppplStack_c28;
      ppppppplStack_b48 = local_c20;
      local_b40 = ppppppplStack_c18;
      ppppppplStack_b38 = local_c10;
LAB_10068375c:
      ::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_diagnostic::DiagnosticReport>(&local_c80);
      ::core::ptr::drop_in_place<codexmate_lib::platform::paths::CodexPaths>(&local_f68);
      goto LAB_100683774;
    }
    ppppppplStack_7f8 = ppppppplStack_670;
    local_800 = local_678;
    local_850 = ppppppplStack_6c8;
    local_6d0 = (long *******)&DAT_8000000000000001;
    ppppppplStack_848 = local_c50;
    local_840 = ppppppplStack_c48;
    ppppppplStack_838 = local_c40;
    local_830 = ppppppplStack_c38;
    ppppppplStack_828 = local_c30;
    ppppppplStack_820 = ppppppplStack_c28;
    ppppppplStack_818 = local_c20;
    local_810 = ppppppplStack_c18;
    ppppppplStack_808 = local_c10;
    <T_as_alloc::string::SpecToString>::spec_to_string(&local_b80,&local_850);
    ppppppplStack_998 = ppppppplStack_b70;
    local_9a0 = ppppppplStack_b78;
    ppppppplStack_9d8 = local_628;
    local_9e0 = ppppppplStack_630;
    ppppppplStack_9c8 = local_618;
    local_9d0 = ppppppplStack_620;
    uStack_9b8 = local_608;
    local_9c0 = ppppppplStack_610;
    uStack_9a8 = local_5f8;
    local_9b0 = uStack_600;
    ppppppplStack_a18 = local_668;
    local_a20 = ppppppplStack_670;
    ppppppplStack_a08 = local_658;
    local_a10 = ppppppplStack_660;
    ppppppplStack_9f8 = local_648;
    local_a00 = ppppppplStack_650;
    ppppppplStack_9e8 = local_638;
    local_9f0 = ppppppplStack_640;
    ppppppplStack_a58 = (long *******)CONCAT71(uStack_6a7,local_6a8);
    local_a60 = (long *******)CONCAT71(uStack_6af,uStack_6b0);
    ppppppplStack_a48 = (long *******)CONCAT71(uStack_697,local_698);
    local_a50 = (long *******)CONCAT71(uStack_69f,uStack_6a0);
    ppppppplStack_a38 = (long *******)CONCAT71(uStack_687,local_688);
    local_a40 = (long *******)CONCAT71(uStack_68f,uStack_690);
    local_a30 = (long *******)CONCAT71(uStack_67f,uStack_680);
    ppppppplStack_a28 = local_678;
    ppppppplStack_a68 = (long *******)CONCAT71(uStack_6b7,uStack_6b8);
    local_a70 = (long *******)CONCAT71(uStack_6bf,bStack_6c0);
    ppppppplStack_a78 = ppppppplStack_6c8;
    local_a80 = local_6d0;
    ppppppplVar33 = local_b80;
    ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&ppppppplStack_c58);
    local_a98 = ppppppplVar33;
LAB_100682e2c:
    local_b80 = local_a80;
    ppppppplStack_b78 = ppppppplStack_a78;
    ppppppplStack_b70 = local_a70;
    ppppppplStack_b68 = ppppppplStack_a68;
    local_b60 = local_a60;
    ppppppplStack_b58 = ppppppplStack_a58;
    ppppppplStack_b50 = local_a50;
    ppppppplStack_b48 = ppppppplStack_a48;
    local_b40 = local_a40;
    ppppppplStack_b38 = ppppppplStack_a38;
    ppppppplStack_b30 = local_a30;
    ppppppplStack_b28 = ppppppplStack_a28;
    local_b20 = local_a20;
    ppppppplStack_b18 = ppppppplStack_a18;
    ppppppplStack_b10 = local_a10;
    ppppppplStack_b08 = ppppppplStack_a08;
    local_b00 = local_a00;
    ppppppplStack_af8 = ppppppplStack_9f8;
    ppppppplStack_af0 = local_9f0;
    ppppppplStack_ae8 = ppppppplStack_9e8;
    local_ae0 = local_9e0;
    ppppppplStack_ad8 = ppppppplStack_9d8;
    ppppppplStack_ad0 = local_9d0;
    ppppppplStack_ac8 = ppppppplStack_9c8;
    local_ac0 = local_9c0;
    uStack_ab8 = uStack_9b8;
    uStack_ab0 = local_9b0;
    uStack_aa8 = uStack_9a8;
    local_a90 = local_9a0;
    ppppppplStack_a88 = ppppppplStack_998;
    iVar29 = _memcmp(param_3,&DAT_1013a3550,(size_t)param_4);
    if (iVar29 != 0) {
      local_981 = 0;
      bVar24 = false;
      goto LAB_100682e70;
    }
    local_6d0 = local_c78;
    ppppppplStack_6c8 = local_c78 + local_c70 * 0xd;
    local_981 = <core::slice::iter::Iter<T>as_core::iter::traits::iterator::Iterator>::any(&local_6d0);
    local_6d0 = local_c78;
    ppppppplStack_6c8 = local_c78 + local_c70 * 0xd;
    uVar31 = <core::slice::iter::Iter<T>as_core::iter::traits::iterator::Iterator>::any(&local_6d0);
    if ((uVar31 & 1) != 0) goto LAB_100682eac;
    bVar24 = false;
  }
  else {
    local_bb0 = ppppppplStack_620;
    ppppppplStack_bb8 = local_628;
    local_ba0 = ppppppplStack_610;
    ppppppplStack_ba8 = local_618;
    local_b90 = uStack_600;
    uStack_b98 = local_608;
    local_bf0 = ppppppplStack_660;
    ppppppplStack_bf8 = local_668;
    local_be0 = ppppppplStack_650;
    ppppppplStack_be8 = local_658;
    local_bd0 = ppppppplStack_640;
    ppppppplStack_bd8 = local_648;
    local_bc0 = ppppppplStack_630;
    ppppppplStack_bc8 = local_638;
    uStack_b88 = local_5f8;
    if (local_6d0 == (long *******)&DAT_8000000000000002) goto LAB_100683710;
LAB_100682d1c:
    ppppppplStack_9d8 = ppppppplStack_bb8;
    local_9e0 = local_bc0;
    ppppppplStack_9c8 = ppppppplStack_ba8;
    local_9d0 = local_bb0;
    uStack_9b8 = uStack_b98;
    local_9c0 = local_ba0;
    uStack_9a8 = uStack_b88;
    local_9b0 = local_b90;
    ppppppplStack_a18 = ppppppplStack_bf8;
    local_a20 = local_c00;
    ppppppplStack_a08 = ppppppplStack_be8;
    local_a10 = local_bf0;
    ppppppplStack_9f8 = ppppppplStack_bd8;
    local_a00 = local_be0;
    ppppppplStack_9e8 = ppppppplStack_bc8;
    local_9f0 = local_bd0;
    ppppppplStack_a58 = ppppppplStack_c38;
    local_a60 = local_c40;
    ppppppplStack_a48 = ppppppplStack_c28;
    local_a50 = local_c30;
    ppppppplStack_a38 = ppppppplStack_c18;
    local_a40 = local_c20;
    ppppppplStack_a28 = ppppppplStack_c08;
    local_a30 = local_c10;
    ppppppplStack_a78 = ppppppplStack_c58;
    local_a80 = local_c60;
    ppppppplStack_a68 = ppppppplStack_c48;
    local_a70 = local_c50;
    ppppppplStack_ad8 = ppppppplStack_bb8;
    local_ae0 = local_bc0;
    ppppppplStack_ac8 = ppppppplStack_ba8;
    ppppppplStack_ad0 = local_bb0;
    uStack_ab8 = uStack_b98;
    local_ac0 = local_ba0;
    uStack_aa8 = uStack_b88;
    uStack_ab0 = local_b90;
    ppppppplStack_b18 = ppppppplStack_bf8;
    local_b20 = local_c00;
    ppppppplStack_b08 = ppppppplStack_be8;
    ppppppplStack_b10 = local_bf0;
    ppppppplStack_af8 = ppppppplStack_bd8;
    local_b00 = local_be0;
    ppppppplStack_ae8 = ppppppplStack_bc8;
    ppppppplStack_af0 = local_bd0;
    ppppppplStack_b58 = ppppppplStack_c38;
    local_b60 = local_c40;
    ppppppplStack_b48 = ppppppplStack_c28;
    ppppppplStack_b50 = local_c30;
    ppppppplStack_b38 = ppppppplStack_c18;
    local_b40 = local_c20;
    ppppppplStack_b28 = ppppppplStack_c08;
    ppppppplStack_b30 = local_c10;
    ppppppplStack_b78 = ppppppplStack_c58;
    local_b80 = local_c60;
    ppppppplStack_b68 = ppppppplStack_c48;
    ppppppplStack_b70 = local_c50;
    local_a98 = (long *******)&DAT_8000000000000000;
    ppppppplStack_a88 = ppppppplStack_998;
    local_a90 = local_9a0;
    bVar24 = param_4 == (long ******)0x14;
    if (!bVar24) {
      if (param_4 != (long ******)0x3) goto LAB_100682e5c;
      goto LAB_100682e2c;
    }
    if ((*param_3 == (long ***)0x705f726574756f72 && param_3[1] == (long ***)0x69646e6f6374736f) && *(int *)(param_3 + 2) == 0x6e6f6974) {
      local_981 = 1;
      bVar24 = true;
    }
    else {
LAB_100682e5c:
      local_981 = 0;
    }
LAB_100682e70:
    uVar31 = codex_diagnostic::fix_requires_quiet_window(local_10f8,local_10f0);
    if ((uVar31 & 1) == 0) {
      bVar24 = false;
    }
    else if ((bVar24) && (iVar29 = _memcmp(param_3,&DAT_1013a356e,(size_t)param_4), iVar29 == 0)) {
      bVar24 = true;
    }
    else {
LAB_100682eac:
      preflight_repair_state_write(&local_6d0,*param_2,auStack_10e8);
      if (local_6d0 != (long *******)&DAT_0000000b) {
        param_1[5] = CONCAT71(uStack_6a7,local_6a8);
        param_1[4] = CONCAT71(uStack_6af,uStack_6b0);
        param_1[7] = CONCAT71(uStack_697,local_698);
        param_1[6] = CONCAT71(uStack_69f,uStack_6a0);
        param_1[9] = CONCAT71(uStack_687,local_688);
        param_1[8] = CONCAT71(uStack_68f,uStack_690);
        param_1[0xb] = (long)local_678;
        param_1[10] = CONCAT71(uStack_67f,uStack_680);
        param_1[1] = (long)ppppppplStack_6c8;
        *param_1 = (long)local_6d0;
        param_1[3] = CONCAT71(uStack_6b7,uStack_6b8);
        param_1[2] = CONCAT71(uStack_6bf,bStack_6c0);
LAB_1006831fc:
        if ((local_a98 != (long *******)&DAT_8000000000000000) && (local_a98 != (long *******)0x0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_a90,local_a98,1);
        }
        ::core::ptr::drop_in_place<core::option::Option<codexmate_lib::core::relay::native_repair_catalog::PreparedRepairCatalog>>(&local_b80);
        goto LAB_10068375c;
      }
      if (local_f7c == 1) {
        router_reconciler::RouterOnPlan::from_state(&local_6d0,auStack_10e8);
        router_reconciler::RouterOnPlan::ensure_ready_with_client_check(&local_850,&local_6d0);
        if (local_850 != (long *******)&DAT_0000000b) {
          param_1[5] = (long)ppppppplStack_828;
          param_1[4] = (long)local_830;
          param_1[7] = (long)ppppppplStack_818;
          param_1[6] = (long)ppppppplStack_820;
          param_1[9] = (long)ppppppplStack_808;
          param_1[8] = (long)local_810;
          param_1[0xb] = (long)ppppppplStack_7f8;
          param_1[10] = (long)local_800;
          param_1[1] = (long)ppppppplStack_848;
          *param_1 = (long)local_850;
          param_1[3] = (long)ppppppplStack_838;
          param_1[2] = (long)local_840;
          ::core::ptr::drop_in_place<codexmate_lib::core::relay::router_reconciler::RouterOnPlan>(&local_6d0);
          goto LAB_1006831fc;
        }
        ::core::ptr::drop_in_place<codexmate_lib::core::relay::router_reconciler::RouterOnPlan>(&local_6d0);
      }
      bVar24 = true;
    }
  }
  bVar26 = platform::process::is_codex_app_running();
  if ((local_10f0 == (long ******)0x14) &&
     ((*local_10f8 == (long ****)0x705f726574756f72 && local_10f8[1] == (long ****)0x69646e6f6374736f) && *(int *)(local_10f8 + 2) == 0x6e6f6974)) {
    codex_diagnostic::audit::RepairAudit::fix(&local_980,param_5,local_10f8,0x14,param_2);
LAB_100683a2c:
    bVar27 = 1;
LAB_100683ac0:
    if (local_980 == (long *******)&DAT_0000000b) goto LAB_100683acc;
    if ((bVar26 & bVar27) != 0) {
      uVar31 = has_confirmed_target_state(param_2[3],&local_f68,(local_f7c ^ 0xff) & 1);
      if ((uVar31 & 1) == 0) {
        bVar27 = 1;
        goto LAB_100683b88;
      }
      goto LAB_100683afc;
    }
LAB_100683b88:
    bVar26 = bVar26 & bVar27;
LAB_100683b8c:
    local_918 = (long *******)&DAT_0000000b;
    param_1[5] = (long)ppppppplStack_958;
    param_1[4] = (long)ppppppplStack_960;
    param_1[7] = (long)ppppppplStack_948;
    param_1[6] = (long)ppppppplStack_950;
    param_1[9] = (long)ppppppplStack_938;
    param_1[8] = (long)local_940;
    param_1[0xb] = (long)ppppppplStack_928;
    param_1[10] = (long)local_930;
    param_1[1] = (long)ppppppplStack_978;
    *param_1 = (long)local_980;
    param_1[3] = (long)local_968;
    param_1[2] = (long)local_970;
    bVar27 = bStack_6c0;
    uVar7 = uStack_6bf;
    uVar8 = uStack_6b8;
    uVar9 = uStack_6b7;
    uVar10 = uStack_6b0;
    uVar11 = uStack_6af;
    uVar12 = local_6a8;
    uVar13 = uStack_6a7;
    uVar14 = uStack_6a0;
    uVar15 = uStack_69f;
    uVar16 = local_698;
    uVar17 = uStack_697;
    uVar18 = uStack_690;
    uVar19 = uStack_68f;
    uVar20 = local_688;
    uVar21 = uStack_687;
    uVar22 = uStack_680;
    uVar23 = uStack_67f;
    if (*param_1 == 0xb) {
      local_6a8 = SUB81(ppppppplStack_950,0);
      uStack_6a7 = (undefined7)((ulong)ppppppplStack_950 >> 8);
      uStack_6b0 = SUB81(ppppppplStack_958,0);
      uStack_6af = (undefined7)((ulong)ppppppplStack_958 >> 8);
      local_698 = SUB81(local_940,0);
      uStack_697 = (undefined7)((ulong)local_940 >> 8);
      uStack_6a0 = SUB81(ppppppplStack_948,0);
      uStack_69f = (undefined7)((ulong)ppppppplStack_948 >> 8);
      uStack_690 = SUB81(ppppppplStack_938,0);
      uStack_68f = (undefined7)((ulong)ppppppplStack_938 >> 8);
      ppppppplStack_6c8 = local_970;
      local_6d0 = ppppppplStack_978;
      uStack_6b8 = SUB81(ppppppplStack_960,0);
      uStack_6b7 = (undefined7)((ulong)ppppppplStack_960 >> 8);
      bStack_6c0 = (byte)local_968;
      uStack_6bf = (undefined7)((ulong)local_968 >> 8);
      if ((bVar26 & 1) != 0) {
LAB_100683be4:
        __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
        ppppppplVar33 = (long *******)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x2d,1);
        if (ppppppplVar33 == (long *******)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error(1,0x2d);
        }
        ppppppplVar33[1] = (long ******)0xe6b096e68d87e9a8;
        *ppppppplVar33 = (long ******)0x8ae5aa87e8b2b7e5;
        ppppppplVar33[3] = (long ******)0xbde4205450477461;
        ppppppplVar33[2] = (long ******)0x68432080bce59389;
        *(undefined8 *)((long)ppppppplVar33 + 0x25) = 0x8895e69f94e78da4;
        *(undefined8 *)((long)ppppppplVar33 + 0x1d) = 0xe5aebfe4bfbde420;
        local_850 = (long *******)&DAT_0000002d;
        local_840 = (long *******)&DAT_0000002d;
        ppppppplStack_848 = ppppppplVar33;
        alloc::vec::Vec<T,A>::push_mut(&uStack_6a0,&local_850);
      }
      param_1[9] = CONCAT71(uStack_68f,uStack_690);
      param_1[6] = CONCAT71(uStack_6a7,local_6a8);
      param_1[5] = CONCAT71(uStack_6af,uStack_6b0);
      param_1[8] = CONCAT71(uStack_697,local_698);
      param_1[7] = CONCAT71(uStack_69f,uStack_6a0);
      param_1[2] = (long)ppppppplStack_6c8;
      param_1[1] = (long)local_6d0;
      param_1[4] = CONCAT71(uStack_6b7,uStack_6b8);
      param_1[3] = CONCAT71(uStack_6bf,bStack_6c0);
      *param_1 = 0xb;
      bVar27 = bStack_6c0;
      uVar7 = uStack_6bf;
      uVar8 = uStack_6b8;
      uVar9 = uStack_6b7;
      uVar10 = uStack_6b0;
      uVar11 = uStack_6af;
      uVar12 = local_6a8;
      uVar13 = uStack_6a7;
      uVar14 = uStack_6a0;
      uVar15 = uStack_69f;
      uVar16 = local_698;
      uVar17 = uStack_697;
      uVar18 = uStack_690;
      uVar19 = uStack_68f;
      uVar20 = local_688;
      uVar21 = uStack_687;
      uVar22 = uStack_680;
      uVar23 = uStack_67f;
    }
  }
  else {
    local_880 = (long *****)&local_10f8;
    local_878 = (long *******)&local_f68;
    local_870 = auStack_10e8;
    local_868 = (long *******)&local_b80;
    local_858 = (long *******)&local_a98;
    local_888 = param_2;
    local_860 = param_5;
    if (bVar24) {
      snapshot(&local_850,param_2);
      <codexmate_lib::platform::paths::CodexPaths_as_core::clone::Clone>::clone(&local_6d0,&local_f68);
      local_3e1 = local_6e4 ^ 1;
      if (local_10f0 == (long ******)0x17) {
        if ((*local_10f8 == (long ****)0x6d5f646165726874 && local_10f8[1] == (long ****)0x725f676e69737369) &&
            *(long *)((long)local_10f8 + 0xf) == 0x7374756f6c6c6f72) goto LAB_100683350;
LAB_100683004:
        uVar34 = 0;
      }
      else {
        if ((local_10f0 != (long ******)0x3) || (*(short *)local_10f8 != 0x6c61 || *(char *)((long)local_10f8 + 2) != 'l')) goto LAB_100683004;
LAB_100683350:
        uVar34 = 1;
      }
      local_160 = (long *******)&local_888;
      ppppppplStack_248 = (long *******)&local_6d0;
      local_240 = (long *******)&local_3e1;
      ppppppplStack_140 = (long *******)&local_981;
      local_130 = (long *******)&local_850;
      local_980 = (long *******)((ulong)local_980 & 0xffffffffffffff00);
      local_1c0 = (long *******)((ulong)local_1c0 & 0xffffffffffffff00);
      local_250 = param_2;
      ppppppplStack_158 = param_2;
      local_150 = ppppppplStack_248;
      local_148 = local_240;
      ppppppplStack_138 = param_5;
      router_transition::run_with_environment
                (&local_3e0,&local_6d0,local_3e1 & 1,0,&DAT_1013a3582,0x11,&local_6d0,uVar34,&local_980,&local_1c0,&local_160,&local_250,param_2,auStack_6d2);
      if (local_3e0 == (long *******)0x3) {
        if ((bStack_377 < 2) && (local_378 < 2)) {
          ppppppplStack_958 = ppppppplStack_3b0;
          ppppppplStack_960 = local_3b8;
          ppppppplStack_948 = ppppppplStack_3a0;
          ppppppplStack_950 = local_3a8;
          ppppppplStack_938 = ppppppplStack_390;
          local_940 = local_398;
          ppppppplStack_928 = ppppppplStack_380;
          local_930 = local_388;
          ppppppplStack_978 = ppppppplStack_3d0;
          local_980 = ppppppplStack_3d8;
          local_968 = ppppppplStack_3c0;
          local_970 = local_3c8;
        }
        else {
          ppppppplStack_118 = ppppppplStack_390;
          local_120 = local_398;
          local_108 = ppppppplStack_380;
          ppppppplStack_110 = local_388;
          ppppppplStack_158 = ppppppplStack_3d0;
          local_160 = ppppppplStack_3d8;
          local_148 = ppppppplStack_3c0;
          local_150 = local_3c8;
          ppppppplStack_138 = ppppppplStack_3b0;
          ppppppplStack_140 = local_3b8;
          local_128 = ppppppplStack_3a0;
          local_130 = local_3a8;
          local_1c0 = (long *******)&local_160;
          ppppppplStack_1b8 = (long *******)<codexmate_lib::core::relay::router_transition::TransitionFailure_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_250,s__diagnostic_repair_failed_after_m_101528a7a,&local_1c0);
          ppppppplStack_278 = ppppppplStack_248;
          local_280 = local_250;
          local_270 = local_240;
          ppppppplStack_248 = (long *******)&local_6d0;
          local_240 = (long *******)&local_850;
          ppppppplStack_238 = (long *******)&local_280;
          local_250 = param_2;
          codex_diagnostic::audit::RepairAudit::step(&local_1c0,param_5,local_10f8,local_10f0,"compensate",10,&local_250);
          if (local_1c0 == (long *******)&DAT_0000000b) {
            ppppppplStack_958 = ppppppplStack_138;
            ppppppplStack_960 = ppppppplStack_140;
            ppppppplStack_948 = local_128;
            ppppppplStack_950 = local_130;
            ppppppplStack_938 = ppppppplStack_118;
            local_940 = local_120;
            ppppppplStack_928 = local_108;
            local_930 = ppppppplStack_110;
            ppppppplStack_978 = ppppppplStack_158;
            local_980 = local_160;
            local_968 = local_148;
            local_970 = local_150;
            if (local_280 != (long *******)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_278,local_280,1);
            }
          }
          else {
            ppppppplStack_228 = ppppppplStack_198;
            ppppppplStack_230 = local_1a0;
            ppppppplStack_218 = ppppppplStack_188;
            local_220 = ppppppplStack_190;
            ppppppplStack_208 = ppppppplStack_178;
            local_210 = local_180;
            local_1f8 = ppppppplStack_168;
            ppppppplStack_200 = ppppppplStack_170;
            ppppppplStack_248 = ppppppplStack_1b8;
            local_250 = local_1c0;
            ppppppplStack_238 = local_1a8;
            local_240 = local_1b0;
            local_980 = (long *******)&local_160;
            ppppppplStack_978 = (long *******)<codexmate_lib::core::relay::router_transition::TransitionFailure_as_core::fmt::Display>::fmt;
            local_970 = (long *******)&local_250;
            local_968 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&local_268,s_5__restoring_the_pre_repair_rout_101528aac,&local_980);
            local_970 = ppppppplStack_260;
            ppppppplStack_978 = local_268;
            local_968 = local_258;
            local_980 = (long *******)&DAT_0000000a;
            ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_250);
            if (local_280 != (long *******)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppplStack_278,local_280,1);
            }
            ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_160);
          }
        }
      }
      else {
        uStack_2b8 = uStack_368;
        local_2c0 = uStack_370;
        uStack_2a8 = uStack_358;
        local_2b0 = local_360;
        uStack_298 = uStack_348;
        local_2a0 = uStack_350;
        local_290 = local_340;
        ppppppplStack_2e8 = local_398;
        local_2f0 = ppppppplStack_3a0;
        ppppppplStack_2d8 = local_388;
        ppppppplStack_2e0 = ppppppplStack_390;
        local_2d0 = ppppppplStack_380;
        ppppppplStack_328 = ppppppplStack_3d8;
        local_330 = local_3e0;
        local_318 = local_3c8;
        local_320 = ppppppplStack_3d0;
        ppppppplStack_308 = local_3b8;
        local_310 = ppppppplStack_3c0;
        ppppppplStack_2f8 = local_3a8;
        ppppppplStack_300 = ppppppplStack_3b0;
        router_transition::TransitionOutcome<T>::into_verified_parts(&local_160,&local_330);
        ppppppplVar6 = local_120;
        ppppppplVar33 = local_160;
        ppppppplStack_198 = local_130;
        local_1a0 = ppppppplStack_138;
        ppppppplStack_188 = local_120;
        ppppppplStack_190 = local_128;
        ppppppplStack_178 = ppppppplStack_110;
        local_180 = ppppppplStack_118;
        ppppppplStack_168 = ppppppplStack_100;
        ppppppplStack_170 = local_108;
        ppppppplStack_1b8 = local_150;
        local_1c0 = ppppppplStack_158;
        local_1a8 = ppppppplStack_140;
        local_1b0 = local_148;
        if (local_160 == (long *******)0x3) {
          ppppppplStack_958 = local_130;
          ppppppplStack_960 = ppppppplStack_138;
          ppppppplStack_948 = local_120;
          ppppppplStack_950 = local_128;
          ppppppplStack_938 = ppppppplStack_110;
          local_940 = ppppppplStack_118;
          ppppppplStack_928 = ppppppplStack_100;
          local_930 = local_108;
          ppppppplStack_978 = local_150;
          local_980 = ppppppplStack_158;
          local_968 = ppppppplStack_140;
          local_970 = local_148;
        }
        else {
          ppppppplStack_1e0 = ppppppplStack_f0;
          local_1e8 = local_f8;
          ppppppplStack_1d0 = ppppppplStack_e0;
          local_1d8 = local_e8;
          local_220 = local_130;
          ppppppplStack_228 = ppppppplStack_138;
          local_210 = local_120;
          ppppppplStack_218 = local_128;
          ppppppplStack_200 = ppppppplStack_110;
          ppppppplStack_208 = ppppppplStack_118;
          ppppppplStack_1f0 = ppppppplStack_100;
          local_1f8 = local_108;
          local_240 = local_150;
          ppppppplStack_248 = ppppppplStack_158;
          ppppppplStack_230 = ppppppplStack_140;
          ppppppplStack_238 = local_148;
          local_250 = local_160;
          ppppppplStack_138 = local_f8;
          ppppppplStack_140 = ppppppplStack_100;
          local_128 = local_e8;
          local_130 = ppppppplStack_f0;
          local_120 = ppppppplStack_e0;
          ppppppplStack_158 = ppppppplStack_118;
          local_160 = ppppppplVar6;
          local_148 = local_108;
          local_150 = ppppppplStack_110;
          local_1a8 = local_d0 + local_c8 * 3;
          local_1c0 = local_d0;
          ppppppplStack_1b8 = local_d0;
          local_1b0 = local_d8;
          <alloc::vec::Vec<T,A1>as_alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend(&local_130,&local_1c0);
          ppppppplStack_960 = local_148;
          local_968 = local_150;
          ppppppplStack_950 = ppppppplStack_138;
          ppppppplStack_958 = ppppppplStack_140;
          local_940 = local_128;
          ppppppplStack_948 = local_130;
          local_970 = ppppppplStack_158;
          ppppppplStack_978 = local_160;
          ppppppplStack_938 = local_120;
          local_980 = (long *******)&DAT_0000000b;
          if (ppppppplVar33 != (long *******)0x2) {
            ::core::ptr::drop_in_place<codexmate_lib::core::relay::router_reconciler::RouterOnResult>(&ppppppplStack_248);
          }
        }
      }
      ::core::ptr::drop_in_place<codexmate_lib::platform::paths::CodexPaths>(&local_6d0);
      ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayState>(&local_850);
      goto LAB_100683a2c;
    }
    local_280 = param_2;
    if ((local_10f0 == (long ******)0x3) && (*(short *)local_10f8 == 0x6c61 && *(char *)((long)local_10f8 + 2) == 'l')) {
      ppppppplVar33 = (long *******)0x0;
      if (local_b80 != (long *******)&DAT_8000000000000001) {
        ppppppplVar33 = (long *******)&local_b80;
      }
      codex_diagnostic::fix_all(&local_6d0,&local_f68,auStack_10e8,&local_280,&DAT_101826ee0,0,ppppppplVar33,param_5);
      local_830 = (long *******)CONCAT71(uStack_6af,uStack_6b0);
      ppppppplStack_838 = (long *******)CONCAT71(uStack_6b7,uStack_6b8);
      ppppppplStack_820 = (long *******)CONCAT71(uStack_69f,uStack_6a0);
      ppppppplStack_828 = (long *******)CONCAT71(uStack_6a7,local_6a8);
      local_810 = (long *******)CONCAT71(uStack_68f,uStack_690);
      ppppppplStack_818 = (long *******)CONCAT71(uStack_697,local_698);
      ppppppplStack_808 = (long *******)CONCAT71(uStack_687,local_688);
      local_840 = (long *******)CONCAT71(uStack_6bf,bStack_6c0);
      local_3e0 = ppppppplStack_6c8;
      if (local_6d0 == (long *******)&DAT_0000000b) {
        local_160 = ppppppplStack_6c8;
        ppppppplStack_158 = local_840;
        local_150 = ppppppplStack_838;
        local_148 = local_830;
        ppppppplStack_140 = ppppppplStack_828;
        ppppppplStack_138 = ppppppplStack_820;
        local_130 = ppppppplStack_818;
        local_128 = local_810;
        local_120 = ppppppplStack_808;
        if (local_a98 != (long *******)&DAT_8000000000000000) {
          local_3e0 = (long *******)&local_a98;
          local_6d0 = (long *******)&local_3e0;
          ppppppplStack_6c8 = (long *******)<&T_as_core::fmt::Display>::fmt;
          alloc::fmt::format::format_inner(&local_980,s___10151f356,&local_6d0);
          if (local_120 == local_130) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_130);
          }
          ppppppplVar33 = local_128 + (long)local_120 * 3;
          ppppppplVar33[1] = (long ******)ppppppplStack_978;
          *ppppppplVar33 = (long ******)local_980;
          ppppppplVar33[2] = (long ******)local_970;
          local_120 = (long *******)((long)local_120 + 1);
        }
        local_840 = ppppppplStack_158;
        ppppppplStack_848 = local_160;
        local_830 = local_148;
        ppppppplStack_838 = local_150;
        ppppppplStack_820 = ppppppplStack_138;
        ppppppplStack_828 = ppppppplStack_140;
        local_810 = local_128;
        ppppppplStack_818 = local_130;
        ppppppplStack_808 = local_120;
        goto LAB_100683080;
      }
      local_800 = (long *******)CONCAT71(uStack_67f,uStack_680);
      ppppppplStack_7f8 = local_678;
      ppppppplStack_848 = ppppppplStack_6c8;
      local_980 = local_6d0;
LAB_100683a68:
      bVar27 = 0;
      ppppppplStack_238 = local_830;
      local_240 = ppppppplStack_838;
      ppppppplStack_228 = ppppppplStack_820;
      ppppppplStack_230 = ppppppplStack_828;
      ppppppplStack_218 = local_810;
      local_220 = ppppppplStack_818;
      local_210 = ppppppplStack_808;
      ppppppplStack_248 = local_840;
      local_250 = ppppppplStack_848;
      ppppppplStack_928 = ppppppplStack_7f8;
      local_930 = local_800;
      ppppppplStack_950 = ppppppplStack_820;
      ppppppplStack_958 = ppppppplStack_828;
      local_940 = local_810;
      ppppppplStack_948 = ppppppplStack_818;
      ppppppplStack_960 = local_830;
      local_968 = ppppppplStack_838;
      local_970 = local_840;
      ppppppplStack_978 = ppppppplStack_848;
      ppppppplStack_938 = ppppppplStack_808;
      goto LAB_100683ac0;
    }
    ppppppplVar33 = (long *******)0x0;
    if (local_b80 != (long *******)&DAT_8000000000000001) {
      ppppppplVar33 = (long *******)&local_b80;
    }
    codex_diagnostic::fix_issue(&local_850,&local_f68,auStack_10e8,local_10f8,local_10f0,&local_280,&DAT_101826ee0,0,ppppppplVar33,param_5);
    local_980 = local_850;
    if (local_850 != (long *******)&DAT_0000000b) goto LAB_100683a68;
LAB_100683080:
    bVar27 = local_981;
    ppppppplStack_218 = local_810;
    local_220 = ppppppplStack_818;
    ppppppplStack_198 = ppppppplStack_820;
    local_1a0 = ppppppplStack_828;
    ppppppplStack_188 = local_810;
    ppppppplStack_190 = ppppppplStack_818;
    local_180 = ppppppplStack_808;
    ppppppplStack_1b8 = local_840;
    local_1c0 = ppppppplStack_848;
    local_1a8 = local_830;
    local_1b0 = ppppppplStack_838;
    if ((local_981 & 1) != 0) {
      codex_diagnostic::audit::RepairAudit::fix(&local_6d0,param_5,&DAT_1013a356e,0x14,param_2);
      ppppppplStack_960 = (long *******)CONCAT71(uStack_6af,uStack_6b0);
      local_968 = (long *******)CONCAT71(uStack_6b7,uStack_6b8);
      ppppppplStack_950 = (long *******)CONCAT71(uStack_69f,uStack_6a0);
      ppppppplStack_958 = (long *******)CONCAT71(uStack_6a7,local_6a8);
      local_940 = (long *******)CONCAT71(uStack_68f,uStack_690);
      ppppppplStack_948 = (long *******)CONCAT71(uStack_697,local_698);
      ppppppplStack_938 = (long *******)CONCAT71(uStack_687,local_688);
      local_970 = (long *******)CONCAT71(uStack_6bf,bStack_6c0);
      local_850 = ppppppplStack_6c8;
      ppppppplStack_848 = local_970;
      local_840 = local_968;
      ppppppplStack_838 = ppppppplStack_960;
      local_830 = ppppppplStack_958;
      ppppppplStack_828 = ppppppplStack_950;
      ppppppplStack_820 = ppppppplStack_948;
      ppppppplStack_818 = local_940;
      local_810 = ppppppplStack_938;
      if (local_6d0 != (long *******)&DAT_0000000b) {
        local_930 = (long *******)CONCAT71(uStack_67f,uStack_680);
        ppppppplStack_928 = local_678;
        ppppppplStack_978 = ppppppplStack_6c8;
        local_980 = local_6d0;
        ::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_diagnostic::FixResult>(&local_1c0);
        goto LAB_100683a2c;
      }
      local_c0 = ppppppplStack_6c8;
      bStack_6c0 = (byte)ppppppplStack_6c8;
      uStack_6bf = (undefined7)((ulong)ppppppplStack_6c8 >> 8);
      uStack_6b8 = SUB81(local_970 + (long)local_968 * 3,0);
      uStack_6b7 = (undefined7)((ulong)(local_970 + (long)local_968 * 3) >> 8);
      local_a8 = ppppppplStack_960;
      local_a0 = ppppppplStack_958;
      ppppppplStack_98 = ppppppplStack_950;
      local_90 = ppppppplStack_948;
      ppppppplStack_88 = local_940;
      local_80 = ppppppplStack_938;
      <alloc::vec::Vec<T,A1>as_alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend(&local_1c0,&local_6d0);
      local_6d0 = local_a0;
      ppppppplStack_6c8 = local_a0;
      bStack_6c0 = (byte)local_a8;
      uStack_6bf = (undefined7)((ulong)local_a8 >> 8);
      uStack_6b8 = SUB81(local_a0 + (long)ppppppplStack_98 * 3,0);
      uStack_6b7 = (undefined7)((ulong)(local_a0 + (long)ppppppplStack_98 * 3) >> 8);
      <alloc::vec::Vec<T,A1>as_alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend(&local_1a8,&local_6d0);
      local_6d0 = ppppppplStack_88;
      ppppppplStack_6c8 = ppppppplStack_88;
      bStack_6c0 = (byte)local_90;
      uStack_6bf = (undefined7)((ulong)local_90 >> 8);
      uStack_6b8 = SUB81(ppppppplStack_88 + (long)local_80 * 3,0);
      uStack_6b7 = (undefined7)((ulong)(ppppppplStack_88 + (long)local_80 * 3) >> 8);
      <alloc::vec::Vec<T,A1>as_alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend(&ppppppplStack_190,&local_6d0);
    }
    ppppppplStack_960 = local_1a8;
    local_968 = local_1b0;
    ppppppplStack_950 = ppppppplStack_198;
    ppppppplStack_958 = local_1a0;
    local_940 = ppppppplStack_188;
    ppppppplStack_948 = ppppppplStack_190;
    local_970 = ppppppplStack_1b8;
    ppppppplStack_978 = local_1c0;
    ppppppplStack_938 = local_180;
    local_980 = (long *******)&DAT_0000000b;
LAB_100683acc:
    if ((bVar26 & bVar27 & 1) == 0) goto LAB_100683b88;
LAB_100683afc:
    bVar27 = platform::process::is_codex_app_running();
    if ((bVar26 & (bVar27 ^ 1) & 1) == 0) goto LAB_100683b8c;
    codex_diagnostic::audit::RepairAudit::step
              (&local_918,param_5,local_10f8,local_10f0,
               "relaunchfilePathcategoryuserNotecurrencymutationdownloadbytesRiddeepLinkprotocolregisterunlistenset_sizeminimizeset_iconmaximizereparent",8);
    uVar23 = uStack_67f;
    uVar22 = uStack_680;
    uVar21 = uStack_687;
    uVar20 = local_688;
    uVar19 = uStack_68f;
    uVar18 = uStack_690;
    uVar17 = uStack_697;
    uVar16 = local_698;
    uVar15 = uStack_69f;
    uVar14 = uStack_6a0;
    uVar13 = uStack_6a7;
    uVar12 = local_6a8;
    uVar11 = uStack_6af;
    uVar10 = uStack_6b0;
    uVar9 = uStack_6b7;
    uVar8 = uStack_6b8;
    uVar7 = uStack_6bf;
    bVar27 = bStack_6c0;
    param_1[5] = (long)ppppppplStack_958;
    param_1[4] = (long)ppppppplStack_960;
    param_1[7] = (long)ppppppplStack_948;
    param_1[6] = (long)ppppppplStack_950;
    param_1[9] = (long)ppppppplStack_938;
    param_1[8] = (long)local_940;
    param_1[0xb] = (long)ppppppplStack_928;
    param_1[10] = (long)local_930;
    param_1[1] = (long)ppppppplStack_978;
    *param_1 = (long)local_980;
    param_1[3] = (long)local_968;
    param_1[2] = (long)local_970;
    uStack_6b0 = (undefined1)local_8f8;
    uStack_6af = (undefined7)((ulong)local_8f8 >> 8);
    local_6a8 = (undefined1)uStack_8f0;
    uStack_6a7 = (undefined7)((ulong)uStack_8f0 >> 8);
    uStack_6a0 = (undefined1)local_8e8;
    uStack_69f = (undefined7)((ulong)local_8e8 >> 8);
    local_698 = (undefined1)uStack_8e0;
    uStack_697 = (undefined7)((ulong)uStack_8e0 >> 8);
    uStack_690 = (undefined1)local_8d8;
    uStack_68f = (undefined7)((ulong)local_8d8 >> 8);
    local_688 = (undefined1)uStack_8d0;
    uStack_687 = (undefined7)((ulong)uStack_8d0 >> 8);
    uStack_680 = (undefined1)local_8c8;
    uStack_67f = (undefined7)((ulong)local_8c8 >> 8);
    bStack_6c0 = (byte)local_908;
    uStack_6bf = (undefined7)((ulong)local_908 >> 8);
    uStack_6b8 = (undefined1)uStack_900;
    uStack_6b7 = (undefined7)((ulong)uStack_900 >> 8);
    if (*param_1 == 0xb) {
      puVar32 = (undefined8 *)((ulong)&local_980 | 8);
      if (local_918 != (long *******)&DAT_0000000b) {
        ppppppplStack_828 = (long *******)puVar32[5];
        local_830 = (long *******)puVar32[4];
        ppppppplStack_818 = (long *******)puVar32[7];
        ppppppplStack_820 = (long *******)puVar32[6];
        local_810 = (long *******)puVar32[8];
        ppppppplStack_848 = (long *******)puVar32[1];
        local_850 = (long *******)*puVar32;
        ppppppplStack_838 = (long *******)puVar32[3];
        local_840 = (long *******)puVar32[2];
        local_678 = ppppppplStack_8c0;
        ppppppplStack_6c8 = ppppppplStack_910;
        local_6d0 = local_918;
        local_330 = (long *******)&local_6d0;
        ppppppplStack_328 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(auStack_8b8,s_6_ChatGPT_ChatGPT_1015289bc,&local_330);
        alloc::vec::Vec<T,A>::push_mut(&ppppppplStack_820,auStack_8b8);
        param_1[6] = (long)ppppppplStack_828;
        param_1[5] = (long)local_830;
        param_1[8] = (long)ppppppplStack_818;
        param_1[7] = (long)ppppppplStack_820;
        param_1[9] = (long)local_810;
        param_1[2] = (long)ppppppplStack_848;
        param_1[1] = (long)local_850;
        param_1[4] = (long)ppppppplStack_838;
        param_1[3] = (long)local_840;
        *param_1 = 0xb;
        ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_6d0);
        bVar27 = bStack_6c0;
        uVar7 = uStack_6bf;
        uVar8 = uStack_6b8;
        uVar9 = uStack_6b7;
        uVar10 = uStack_6b0;
        uVar11 = uStack_6af;
        uVar12 = local_6a8;
        uVar13 = uStack_6a7;
        uVar14 = uStack_6a0;
        uVar15 = uStack_69f;
        uVar16 = local_698;
        uVar17 = uStack_697;
        uVar18 = uStack_690;
        uVar19 = uStack_68f;
        uVar20 = local_688;
        uVar21 = uStack_687;
        uVar22 = uStack_680;
        uVar23 = uStack_67f;
        goto LAB_100683c50;
      }
      local_6a8 = (undefined1)puVar32[5];
      uStack_6a7 = (undefined7)((ulong)puVar32[5] >> 8);
      uStack_6b0 = (undefined1)puVar32[4];
      uStack_6af = (undefined7)((ulong)puVar32[4] >> 8);
      local_698 = (undefined1)puVar32[7];
      uStack_697 = (undefined7)((ulong)puVar32[7] >> 8);
      uStack_6a0 = (undefined1)puVar32[6];
      uStack_69f = (undefined7)((ulong)puVar32[6] >> 8);
      uStack_690 = (undefined1)puVar32[8];
      uStack_68f = (undefined7)((ulong)puVar32[8] >> 8);
      ppppppplStack_6c8 = (long *******)puVar32[1];
      local_6d0 = (long *******)*puVar32;
      uStack_6b8 = (undefined1)puVar32[3];
      uStack_6b7 = (undefined7)((ulong)puVar32[3] >> 8);
      bStack_6c0 = (byte)puVar32[2];
      uStack_6bf = (undefined7)((ulong)puVar32[2] >> 8);
      local_688 = uVar20;
      uStack_687 = uVar21;
      uStack_680 = uVar22;
      uStack_67f = uVar23;
      goto LAB_100683be4;
    }
    if (local_918 != (long *******)&DAT_0000000b) {
      ppppppplStack_828 = ppppppplStack_958;
      local_830 = ppppppplStack_960;
      ppppppplStack_818 = ppppppplStack_948;
      ppppppplStack_820 = ppppppplStack_950;
      ppppppplStack_808 = ppppppplStack_938;
      local_810 = local_940;
      ppppppplStack_7f8 = ppppppplStack_928;
      local_800 = local_930;
      ppppppplStack_848 = ppppppplStack_978;
      local_850 = local_980;
      ppppppplStack_838 = local_968;
      local_840 = local_970;
      local_678 = ppppppplStack_8c0;
      ppppppplStack_6c8 = ppppppplStack_910;
      local_6d0 = local_918;
      local_330 = (long *******)&local_850;
      ppppppplStack_328 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
      local_320 = (long *******)&local_6d0;
      local_318 = (long *******)<codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_8a0,s_diagnostic_repair_failed____Chat_101528a10,&local_330);
      param_1[2] = lStack_898;
      param_1[1] = local_8a0;
      param_1[3] = local_890;
      *param_1 = 10;
      ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_6d0);
      ::core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_850);
      bVar27 = bStack_6c0;
      uVar7 = uStack_6bf;
      uVar8 = uStack_6b8;
      uVar9 = uStack_6b7;
      uVar10 = uStack_6b0;
      uVar11 = uStack_6af;
      uVar12 = local_6a8;
      uVar13 = uStack_6a7;
      uVar14 = uStack_6a0;
      uVar15 = uStack_69f;
      uVar16 = local_698;
      uVar17 = uStack_697;
      uVar18 = uStack_690;
      uVar19 = uStack_68f;
      uVar20 = local_688;
      uVar21 = uStack_687;
      uVar22 = uStack_680;
      uVar23 = uStack_67f;
    }
  }
LAB_100683c50:
  uStack_67f = uVar23;
  uStack_680 = uVar22;
  uStack_687 = uVar21;
  local_688 = uVar20;
  uStack_68f = uVar19;
  uStack_690 = uVar18;
  uStack_697 = uVar17;
  local_698 = uVar16;
  uStack_69f = uVar15;
  uStack_6a0 = uVar14;
  uStack_6a7 = uVar13;
  local_6a8 = uVar12;
  uStack_6af = uVar11;
  uStack_6b0 = uVar10;
  uStack_6b7 = uVar9;
  uStack_6b8 = uVar8;
  uStack_6bf = uVar7;
  bStack_6c0 = bVar27;
  if ((local_a98 != (long *******)&DAT_8000000000000000) && (local_a98 != (long *******)0x0)) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_a90,local_a98,1);
  }
  ::core::ptr::drop_in_place<core::option::Option<codexmate_lib::core::relay::native_repair_catalog::PreparedRepairCatalog>>(&local_b80);
  <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_c80);
  if (local_c80 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_c78,local_c80 * 0x68,8);
  }
  ::core::ptr::drop_in_place<codexmate_lib::platform::paths::CodexPaths>(&local_f68);
  ::core::ptr::drop_in_place<codexmate_lib::core::relay::models::RelayState>(auStack_10e8);
  if ((((bVar5 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
     (uVar31 = std::panicking::panic_count::is_zero_slow_path(), (uVar31 & 1) == 0)) {
    *(undefined1 *)(ppppppplVar4 + 1) = 1;
  }
  uVar30 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*ppppppplVar4);
  uVar31 = (ulong)uVar30;
joined_r0x000100683cd8:
  if ((((uVar28 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
     (uVar31 = std::panicking::panic_count::is_zero_slow_path(uVar31), (uVar31 & 1) == 0)) {
    *(undefined1 *)(pppppplVar38 + 3) = 1;
  }
  uVar28 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)pppppplVar38[2]);
  return (ulong)uVar28;
}

 (GhidraScript)
===END=== (GhidraScript)
