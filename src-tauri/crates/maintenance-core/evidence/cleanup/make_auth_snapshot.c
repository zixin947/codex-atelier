/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* make_auth_snapshot @ 1001ecaf8 requested 1001ecaf8 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::auth::make_auth_snapshot */

void codexmate_lib::core::auth::make_auth_snapshot(undefined8 *param_1,long *param_2,undefined8 param_3,long param_4)

{
  char cVar1;
  int ****ppppiVar2;
  int *piVar3;
  int *piVar4;
  long lVar5;
  size_t sVar6;
  int ****ppppiVar7;
  int ****ppppiVar8;
  int **ppiVar9;
  bool bVar10;
  int iVar11;
  ulong uVar12;
  char *pcVar13;
  undefined1 (*pauVar14) [16];
  int ****ppppiVar15;
  ulong uVar16;
  int ***pppiVar17;
  int **ppiVar18;
  long lVar19;
  byte bVar20;
  int **ppiVar21;
  ulong uVar22;
  undefined1 uVar23;
  undefined8 uVar24;
  undefined8 *puVar25;
  int ****ppppiVar26;
  int **ppiVar27;
  int ****ppppiVar28;
  int ****ppppiVar29;
  long *plVar30;
  byte bVar31;
  undefined1 auVar32 [16];
  undefined1 auVar33 [12];
  int **local_350;
  undefined1 local_32c;
  int ****local_318;
  void *local_310;
  undefined1 auStack_300 [48];
  undefined1 auStack_2d0 [48];
  int ****local_2a0;
  int ****ppppiStack_298;
  int ****ppppiStack_290;
  int ****local_288;
  undefined1 local_280 [16];
  int ****local_270;
  int ****ppppiStack_268;
  int ****ppppiStack_260;
  int ****ppppiStack_258;
  undefined1 local_250 [16];
  int ****local_240;
  int ****ppppiStack_238;
  int ****local_230;
  int ****local_228;
  int ****ppppiStack_220;
  int *local_218;
  int ****local_210;
  int ****ppppiStack_208;
  int ****local_200;
  int ****local_1f0;
  int ****ppppiStack_1e8;
  int ****local_1e0;
  int ****local_1d8;
  int ****local_1d0;
  int ***local_1c0;
  int local_1b8;
  int ****local_1b0;
  int ****ppppiStack_1a8;
  int ****local_1a0;
  int ****ppppiStack_198;
  int ****local_190;
  int ****ppppiStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 local_128;
  int ****local_118;
  int ****local_110;
  int ****local_108;
  int ****ppppiStack_100;
  int ****local_f8;
  int ****ppppiStack_f0;
  int ****local_e8;
  int ****ppppiStack_e0;
  int ****local_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 local_78;

  uVar12 = is_aimami_managed_virtual_auth();
  if ((uVar12 & 1) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    pcVar13 = (char *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x42,1);
    if (pcVar13 != (char *)0x0) {
      builtin_strncpy(pcVar13,"auth.json contains AiMaMi managed virtual auth; not a real account",0x42);
      param_1[2] = 0x42;
      param_1[3] = pcVar13;
      param_1[4] = 0x42;
      param_1[1] = 10;
      *param_1 = 2;
      return;
    }
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x42);
  }
  lVar19 = 0;
  if (param_2[6] != -0x8000000000000000) {
    lVar19 = param_2[7];
  }
  decode_jwt_claims(auStack_300,lVar19,param_2[8]);
  lVar19 = 0;
  if (param_2[9] != -0x8000000000000000) {
    lVar19 = param_2[10];
  }
  decode_jwt_claims(auStack_2d0,lVar19,param_2[0xb]);
  nested_dict(&local_108,auStack_300,
              "https://api.openai.com/authhttps://api.openai.com/profileemailchatgpt_user_idchatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
              ,0x1b);
  if (local_108 == (int ****)0x0) {
    nested_dict(&local_1b0,auStack_2d0,
                "https://api.openai.com/authhttps://api.openai.com/profileemailchatgpt_user_idchatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
                ,0x1b);
  }
  else {
    ppppiStack_1a8 = ppppiStack_100;
    local_1b0 = local_108;
    ppppiStack_198 = ppppiStack_f0;
    local_1a0 = local_f8;
    ppppiStack_188 = ppppiStack_e0;
    local_190 = local_e8;
  }
  if (local_1b0 == (int ****)0x0) {
    pauVar14 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
    if (pauVar14[1][0] == '\x01') {
      local_280 = *pauVar14;
    }
    else {
      local_280 = std::sys::random::hashmap_random_keys();
      *pauVar14 = local_280;
      pauVar14[1][0] = 1;
    }
    *(long *)*pauVar14 = local_280._0_8_ + 1;
    ppppiStack_298 = (int ****)0x0;
    local_2a0 = (int ****)&DAT_101303e80;
    local_288 = (int ****)0x0;
    ppppiStack_290 = (int ****)0x0;
  }
  else {
    ppppiStack_298 = ppppiStack_1a8;
    local_2a0 = local_1b0;
    local_288 = ppppiStack_198;
    ppppiStack_290 = local_1a0;
    local_280._8_8_ = ppppiStack_188;
    local_280._0_8_ = local_190;
  }
  nested_dict(&local_108,auStack_2d0,
              "https://api.openai.com/profileemailchatgpt_user_idchatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
              ,0x1e);
  if (local_108 == (int ****)0x0) {
    pauVar14 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
    if (pauVar14[1][0] == '\x01') {
      local_250 = *pauVar14;
    }
    else {
      local_250 = std::sys::random::hashmap_random_keys();
      *pauVar14 = local_250;
      pauVar14[1][0] = 1;
    }
    *(long *)*pauVar14 = local_250._0_8_ + 1;
    ppppiStack_268 = (int ****)0x0;
    local_270 = (int ****)&DAT_101303e80;
    ppppiStack_258 = (int ****)0x0;
    ppppiStack_260 = (int ****)0x0;
  }
  else {
    ppppiStack_268 = ppppiStack_100;
    local_270 = local_108;
    ppppiStack_258 = ppppiStack_f0;
    ppppiStack_260 = local_f8;
    local_250._8_8_ = ppppiStack_e0;
    local_250._0_8_ = local_e8;
  }
  string_val(&local_108,auStack_300,
             "emailchatgpt_user_idchatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
             ,5);
  if (local_108 == (int ****)&DAT_8000000000000000) {
    string_val(&local_1b0,&local_270,
               "emailchatgpt_user_idchatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
               ,5);
    if (local_1b0 != (int ****)&DAT_8000000000000000) goto LAB_1001ece60;
LAB_1001ecd24:
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    ppppiVar15 = (int ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xf,1);
    if (ppppiVar15 == (int ****)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0xf);
    }
    *ppppiVar15 = (int ***)0x206e776f6e6b6e55;
    builtin_strncpy((char *)((long)ppppiVar15 + 7)," Account",8);
    ppppiVar26 = (int ****)&DAT_0000000f;
    auVar32 = ::core::str::<impl_str>::trim_matches(ppppiVar15,0xf);
  }
  else {
    ppppiStack_1a8 = ppppiStack_100;
    local_1b0 = local_108;
    local_1a0 = local_f8;
    if (local_108 == (int ****)&DAT_8000000000000000) goto LAB_1001ecd24;
LAB_1001ece60:
    ppppiVar15 = ppppiStack_1a8;
    ppppiVar26 = local_1b0;
    auVar32 = ::core::str::<impl_str>::trim_matches(ppppiStack_1a8,local_1a0);
  }
  sVar6 = auVar32._8_8_;
  if ((long)sVar6 < 0) {
    uVar24 = 0;
LAB_1001ece78:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(uVar24,sVar6);
  }
  if (sVar6 == 0) {
    local_310 = (void *)0x1;
    _memcpy((void *)0x1,auVar32._0_8_,0);
  }
  else {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    uVar24 = 1;
    local_310 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar6,1);
    if (local_310 == (void *)0x0) goto LAB_1001ece78;
    _memcpy(local_310,auVar32._0_8_,sVar6);
  }
  if (ppppiVar26 != (int ****)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppiVar15,ppppiVar26,1);
  }
  if (param_2[0xf] == -0x8000000000000000) {
LAB_1001ecddc:
    string_val(&local_108,&local_2a0,"chatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens",0x12);
    ppppiVar15 = local_108;
    ppppiVar26 = ppppiStack_100;
    ppppiVar28 = local_f8;
    if (local_108 == (int ****)&DAT_8000000000000000) {
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      ppppiVar26 = (int ****)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xf,1);
      if (ppppiVar26 == (int ****)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0xf);
      }
      *ppppiVar26 = (int ***)0x2d6e776f6e6b6e75;
      builtin_strncpy((char *)((long)ppppiVar26 + 7),"-account",8);
      ppppiVar15 = (int ****)&DAT_0000000f;
      ppppiVar28 = (int ****)&DAT_0000000f;
    }
  }
  else {
    <alloc::string::String_as_core::clone::Clone>::clone(&local_108);
    if (local_108 == (int ****)&DAT_8000000000000000) goto LAB_1001ecddc;
    ppppiVar15 = local_108;
    ppppiVar26 = ppppiStack_100;
    ppppiVar28 = local_f8;
    if (local_f8 == (int ****)0x0) {
      if (local_108 != (int ****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppiStack_100,local_108,1);
      }
      goto LAB_1001ecddc;
    }
  }
  string_val(&local_108,&local_2a0,
             "chatgpt_user_idchatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
             ,0xf);
  if (local_108 == (int ****)&DAT_8000000000000000) {
    string_val(&local_1b0,&local_2a0,
               "user_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens",7);
    if (local_1b0 != (int ****)&DAT_8000000000000000) goto LAB_1001ecf44;
LAB_1001ecf10:
    alloc::str::<impl_str>::to_lowercase(&local_240,local_310,sVar6);
  }
  else {
    ppppiStack_1a8 = ppppiStack_100;
    local_1b0 = local_108;
    local_1a0 = local_f8;
    if (local_108 == (int ****)&DAT_8000000000000000) goto LAB_1001ecf10;
LAB_1001ecf44:
    ppppiStack_238 = ppppiStack_1a8;
    local_240 = local_1b0;
    local_230 = local_1a0;
  }
  string_val(&local_1b0,&local_2a0,
             "chatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
             ,0x11);
  ppppiVar29 = ppppiStack_1a8;
  ppppiVar7 = local_1b0;
  if (local_1b0 == (int ****)&DAT_8000000000000000) {
    local_32c = 10;
    local_318 = (int ****)&DAT_8000000000000000;
    ppppiVar7 = ppppiVar15;
  }
  else {
    alloc::str::<impl_str>::to_lowercase(&local_108,ppppiStack_1a8,local_1a0);
    if (ppppiVar7 != (int ****)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppiVar29,ppppiVar7,1);
    }
    ppppiVar7 = ppppiStack_100;
    local_318 = local_108;
    if (local_108 == (int ****)&DAT_8000000000000000) {
      local_32c = 10;
      local_318 = (int ****)&DAT_8000000000000000;
    }
    else {
      local_32c = plan_mapping::parse_chatgpt_plan_label(ppppiStack_100,local_f8);
    }
  }
  if (local_288 != (int ****)0x0) {
    uVar16 = ::core::hash::BuildHasher::hash_one
                       (local_280,
                        "organizationstitlechatgpt_account_user_idlast_refresh is outside the supported timestamp rangelast_refresh must be a timestamp string or number"
                        ,0xd);
    lVar19 = 0;
    bVar20 = (byte)(uVar16 >> 0x39);
    uVar16 = uVar16 & (ulong)ppppiStack_298;
    uVar24 = *(undefined8 *)((long)local_2a0 + uVar16);
    uVar12 = CONCAT17(-((byte)((ulong)uVar24 >> 0x38) == bVar20),
                      CONCAT16(-((byte)((ulong)uVar24 >> 0x30) == bVar20),
                               CONCAT15(-((byte)((ulong)uVar24 >> 0x28) == bVar20),
                                        CONCAT14(-((byte)((ulong)uVar24 >> 0x20) == bVar20),
                                                 CONCAT13(-((byte)((ulong)uVar24 >> 0x18) == bVar20),
                                                          CONCAT12(-((byte)((ulong)uVar24 >> 0x10) == bVar20),
                                                                   CONCAT11(-((byte)((ulong)uVar24 >> 8) == bVar20),-((byte)uVar24 == bVar20)))))))) &
             0x8080808080808080;
    while( true ) {
      while (uVar12 == 0) {
        bVar31 = NEON_umaxv(CONCAT17(-((char)((ulong)uVar24 >> 0x38) == -1),
                                     CONCAT16(-((char)((ulong)uVar24 >> 0x30) == -1),
                                              CONCAT15(-((char)((ulong)uVar24 >> 0x28) == -1),
                                                       CONCAT14(-((char)((ulong)uVar24 >> 0x20) == -1),
                                                                CONCAT13(-((char)((ulong)uVar24 >> 0x18) == -1),
                                                                         CONCAT12(-((char)((ulong)uVar24 >> 0x10) == -1),
                                                                                  CONCAT11(-((char)((ulong)uVar24 >> 8) == -1),-((char)uVar24 == -1)))))))),1);
        if ((bVar31 & 1) != 0) goto LAB_1001ed1f8;
        lVar19 = lVar19 + 8;
        uVar16 = uVar16 + lVar19 & (ulong)ppppiStack_298;
        uVar24 = *(undefined8 *)((long)local_2a0 + uVar16);
        uVar12 = CONCAT17(-((byte)((ulong)uVar24 >> 0x38) == bVar20),
                          CONCAT16(-((byte)((ulong)uVar24 >> 0x30) == bVar20),
                                   CONCAT15(-((byte)((ulong)uVar24 >> 0x28) == bVar20),
                                            CONCAT14(-((byte)((ulong)uVar24 >> 0x20) == bVar20),
                                                     CONCAT13(-((byte)((ulong)uVar24 >> 0x18) == bVar20),
                                                              CONCAT12(-((byte)((ulong)uVar24 >> 0x10) == bVar20),
                                                                       CONCAT11(-((byte)((ulong)uVar24 >> 8) == bVar20),-((byte)uVar24 == bVar20)))))))) &
                 0x8080808080808080;
      }
      uVar22 = (uVar12 >> 7 & 0xff00ff00ff00ff00) >> 8 | (uVar12 >> 7 & 0xff00ff00ff00ff) << 8;
      uVar22 = (uVar22 & 0xffff0000ffff0000) >> 0x10 | (uVar22 & 0xffff0000ffff) << 0x10;
      uVar22 = uVar16 + ((ulong)LZCOUNT(uVar22 >> 0x20 | uVar22 << 0x20) >> 3) & (ulong)ppppiStack_298;
      if ((local_2a0[uVar22 * -7 + -5] == (int ***)0xd) &&
         (*local_2a0[uVar22 * -7 + -6] == (int **)0x617a696e6167726f && *(long *)((long)local_2a0[uVar22 * -7 + -6] + 5) == 0x736e6f6974617a69)) break;
      uVar12 = uVar12 - 1 & uVar12;
    }
    if (*(char *)(local_2a0 + uVar22 * -7 + -4) == '\x04') {
      if (((local_2a0[uVar22 * -7 + -1] == (int ***)0x0) || (pppiVar17 = local_2a0[uVar22 * -7 + -2], *(char *)pppiVar17 != '\x05')) ||
         (ppiVar21 = pppiVar17[1], ppiVar21 == (int **)0x0)) goto LAB_1001ed75c;
      local_350 = pppiVar17[2];
      goto LAB_1001ed15c;
    }
  }
LAB_1001ed1f8:
  local_228 = (int ****)&DAT_8000000000000000;
  bVar10 = true;
  goto LAB_1001ed204;
LAB_1001ed15c:
  uVar16 = (ulong)*(ushort *)((long)ppiVar21 + 0x272);
  lVar19 = uVar16 * 0x18;
  uVar12 = 0xffffffffffffffff;
  ppiVar18 = ppiVar21 + 0x2d;
  ppiVar9 = ppiVar21 + 0xfffffffffffffffe;
  do {
    ppiVar27 = ppiVar9;
    if (lVar19 == 0) goto LAB_1001ed1dc;
    piVar3 = ppiVar18[2];
    lVar5 = 5 - (long)piVar3;
    if ((int *)0x5 < piVar3 || lVar5 == 0) {
      piVar3 = (int *)0x5;
    }
    iVar11 = _memcmp("titlechatgpt_account_user_idlast_refresh is outside the supported timestamp rangelast_refresh must be a timestamp string or number",
                     ppiVar18[1],(size_t)piVar3);
    if (iVar11 != 0) {
      lVar5 = (long)iVar11;
    }
    cVar1 = 0 < lVar5;
    if (lVar5 < 0) {
      cVar1 = -1;
    }
    uVar12 = uVar12 + 1;
    lVar19 = lVar19 + -0x18;
    ppiVar18 = ppiVar18 + 3;
    ppiVar9 = ppiVar27 + 4;
  } while (cVar1 == '\x01');
  uVar16 = uVar12;
  if (cVar1 != '\0') {
LAB_1001ed1dc:
    if (local_350 == (int **)0x0) goto LAB_1001ed75c;
    ppiVar21 = (int **)ppiVar21[uVar16 + 0x4f];
    local_350 = (int **)((long)local_350 + -1);
    goto LAB_1001ed15c;
  }
  if (*(char *)(ppiVar27 + 2) == '\x03') {
    piVar3 = ppiVar27[4];
    piVar4 = ppiVar27[5];
    alloc::raw_vec::RawVecInner<A>::try_allocate_in(&local_108,piVar4,0,1,1);
    ppppiVar2 = local_f8;
    ppppiVar29 = ppppiStack_100;
    if ((int)local_108 == 1) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(ppppiStack_100,local_f8);
    }
    _memcpy(local_f8,piVar3,(size_t)piVar4);
    local_228 = ppppiVar29;
    ppppiStack_220 = ppppiVar2;
    bVar10 = ppppiVar29 == (int ****)&DAT_8000000000000000;
    local_218 = piVar4;
    goto LAB_1001ed204;
  }
LAB_1001ed75c:
  local_228 = (int ****)&DAT_8000000000000000;
  bVar10 = true;
LAB_1001ed204:
  string_val(&local_108,auStack_300,
             "namechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
             ,4);
  if (local_108 == (int ****)&DAT_8000000000000000) {
    string_val(&local_210,&local_270,
               "namechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
               ,4);
    if (bVar10) goto LAB_1001ed27c;
LAB_1001ed260:
    <alloc::string::String_as_core::clone::Clone>::clone(&local_108,&local_228);
    if (local_108 == (int ****)&DAT_8000000000000000) goto LAB_1001ed27c;
    local_1f0 = local_108;
    ppppiStack_1e8 = ppppiStack_100;
    local_1e0 = local_f8;
  }
  else {
    ppppiStack_208 = ppppiStack_100;
    local_210 = local_108;
    local_200 = local_f8;
    if (!bVar10) goto LAB_1001ed260;
LAB_1001ed27c:
    if (local_210 == (int ****)&DAT_8000000000000000) {
      local_1f0 = (int ****)&DAT_8000000000000000;
    }
    else {
      <alloc::string::String_as_core::clone::Clone>::clone(&local_1f0,&local_210);
    }
  }
  lVar19 = 0;
  if (param_2[0x12] != -0x8000000000000000) {
    lVar19 = param_2[0x13];
  }
  auVar32 = parse_iso_timestamp(lVar19,param_2[0x14]);
  ppppiVar29 = auVar32._8_8_;
  if ((auVar32._0_8_ & 1) == 0) {
    auVar32 = int_val(auStack_300);
    ppppiVar29 = auVar32._8_8_;
    if ((auVar32._0_8_ & 1) != 0) goto LAB_1001ed2e8;
    auVar32 = int_val(auStack_2d0);
    ppppiVar29 = auVar32._8_8_;
    if ((auVar32._0_8_ & 1) != 0) goto LAB_1001ed2e8;
    std::sys::fs::metadata(&local_108,param_3,param_4);
    if (((ulong)local_108 & 1) == 0) {
      uStack_140 = uStack_90;
      local_148 = local_98;
      uStack_130 = uStack_80;
      uStack_138 = uStack_88;
      uStack_180 = uStack_d0;
      ppppiStack_188 = local_d8;
      uStack_170 = uStack_c0;
      uStack_178 = uStack_c8;
      uStack_160 = uStack_b0;
      local_168 = local_b8;
      uStack_150 = uStack_a0;
      uStack_158 = uStack_a8;
      local_1a0 = ppppiStack_f0;
      ppppiStack_1a8 = local_f8;
      local_190 = ppppiStack_e0;
      ppppiStack_198 = local_e8;
      local_128 = local_78;
      local_1b0 = ppppiStack_100;
      std::fs::Metadata::modified(&local_118,&local_1b0);
      if ((int)local_110 == 1000000000) {
        if (((ulong)local_118 & 3) == 1) {
          plVar30 = (long *)((long)local_118 - 1);
          param_4 = *plVar30;
          puVar25 = *(undefined8 **)((long)local_118 + 7);
          if ((code *)*puVar25 != (code *)0x0) {
            (*(code *)*puVar25)(param_4);
          }
          goto LAB_1001ed3cc;
        }
        goto LAB_1001ed420;
      }
      local_1c0 = (int ***)local_118;
      local_1b8 = (int)local_110;
      std::time::SystemTime::duration_since(&local_1d8,&local_1c0,0,0);
      if ((int)local_1d8 == 1) goto LAB_1001ed420;
      lVar19 = *param_2;
      ppppiVar29 = local_1d0;
    }
    else {
      if (((ulong)ppppiStack_100 & 3) == 1) {
        plVar30 = (long *)((long)ppppiStack_100 + -1);
        param_4 = *plVar30;
        puVar25 = *(undefined8 **)((long)ppppiStack_100 + 7);
        if ((code *)*puVar25 != (code *)0x0) {
          (*(code *)*puVar25)(param_4);
        }
LAB_1001ed3cc:
        if (puVar25[1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(param_4,puVar25[1],puVar25[2]);
        }
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar30,0x18,8);
      }
LAB_1001ed420:
      auVar33 = std::time::SystemTime::now();
      local_1b0 = auVar33._0_8_;
      ppppiStack_1a8 = (int ****)CONCAT44(ppppiStack_1a8._4_4_,auVar33._8_4_);
      std::time::SystemTime::duration_since(&local_108,&local_1b0,0,0);
      ppppiVar29 = (int ****)0x0;
      if ((int)local_108 == 0) {
        ppppiVar29 = ppppiStack_100;
      }
      lVar19 = *param_2;
    }
  }
  else {
LAB_1001ed2e8:
    lVar19 = *param_2;
  }
  if (lVar19 != -0x8000000000000000) {
    alloc::str::<impl_str>::to_lowercase(&local_108,param_2[1],param_2[2]);
    if (local_108 != (int ****)&DAT_8000000000000000) {
      if (local_f8 == (int ****)&SUB_00000006) {
        bVar10 = *(int *)ppppiStack_100 == 0x6b697061 && *(short *)((long)ppppiStack_100 + 4) == 0x7965;
      }
      else {
        bVar10 = false;
      }
      if (local_108 != (int ****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppiStack_100,local_108,1);
      }
      goto LAB_1001ed480;
    }
  }
  bVar10 = false;
LAB_1001ed480:
  string_val(&local_108,&local_2a0,
             "chatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
             ,0x21);
  ppppiVar8 = ppppiStack_100;
  ppppiVar2 = local_108;
  if (local_108 == (int ****)&DAT_8000000000000000) {
    uVar24 = 0;
    uVar23 = 2;
  }
  else {
    auVar32 = parse_iso_timestamp(ppppiStack_100,local_f8);
    param_4 = auVar32._8_8_;
    if (ppppiVar2 != (int ****)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppiVar8,ppppiVar2,1);
    }
    if ((auVar32._0_8_ & 1) == 0) {
      uVar24 = 0;
      uVar23 = 2;
    }
    else {
      auVar33 = std::time::SystemTime::now();
      local_1b0 = auVar33._0_8_;
      ppppiStack_1a8 = (int ****)CONCAT44(ppppiStack_1a8._4_4_,auVar33._8_4_);
      std::time::SystemTime::duration_since(&local_108,&local_1b0,0,0);
      ppppiVar2 = (int ****)0x0;
      if ((int)local_108 == 0) {
        ppppiVar2 = ppppiStack_100;
      }
      uVar23 = (long)ppppiVar2 < param_4;
      uVar24 = 1;
    }
  }
  ppppiVar2 = ppppiStack_238;
  local_118 = ppppiStack_238;
  local_110 = local_230;
  local_108 = (int ****)&local_118;
  ppppiStack_100 = (int ****)&<&T_as_core::fmt::Display>::fmt;
  local_f8 = (int ****)&local_1d8;
  ppppiStack_f0 = (int ****)&<&T_as_core::fmt::Display>::fmt;
  local_1d8 = ppppiVar26;
  local_1d0 = ppppiVar28;
  alloc::fmt::format::format_inner(&local_1b0,s____10151cfd8,&local_108);
  credential_generation(&local_108,param_2);
  param_1[6] = ppppiStack_1a8;
  param_1[5] = local_1b0;
  param_1[7] = local_1a0;
  param_1[8] = sVar6;
  param_1[9] = local_310;
  param_1[10] = sVar6;
  param_1[0xc] = ppppiStack_1e8;
  param_1[0xb] = local_1f0;
  param_1[0xd] = local_1e0;
  param_1[0xf] = ppppiStack_220;
  param_1[0xe] = local_228;
  param_1[0x10] = local_218;
  param_1[0x12] = ppppiStack_208;
  param_1[0x11] = local_210;
  param_1[4] = local_f8;
  param_1[3] = ppppiStack_100;
  param_1[2] = local_108;
  *param_1 = uVar24;
  param_1[1] = param_4;
  param_1[0x13] = local_200;
  param_1[0x14] = ppppiVar29;
  *(bool *)(param_1 + 0x15) = bVar10;
  *(undefined1 *)((long)param_1 + 0xa9) = uVar23;
  *(undefined1 *)((long)param_1 + 0xaa) = local_32c;
  if (((ulong)local_318 & 0x7fffffffffffffff) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppiVar7,local_318,1);
  }
  if (local_240 != (int ****)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppiVar2,local_240,1);
  }
  if (ppppiVar15 != (int ****)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppiVar26,ppppiVar15,1);
  }
  <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(&local_270);
  <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(&local_2a0);
  <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(auStack_2d0);
  <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(auStack_300);
  return;
}
