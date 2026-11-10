/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* is_aimami_managed_virtual_auth @ 1001f0f9c requested 1001f0f9c */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::auth::is_aimami_managed_virtual_auth */

bool codexmate_lib::core::auth::is_aimami_managed_virtual_auth(long param_1)

{
  undefined8 uVar1;
  undefined8 *******pppppppuVar2;
  bool bVar3;
  bool bVar4;
  undefined1 (*pauVar5) [16];
  undefined8 *******pppppppuVar6;
  undefined8 *******pppppppuVar7;
  long lVar8;
  long *plVar9;
  long lVar10;
  undefined1 auStack_1e0 [48];
  undefined1 auStack_1b0 [48];
  undefined8 *******local_180;
  undefined8 *******pppppppuStack_178;
  undefined8 *******pppppppuStack_170;
  undefined8 *puStack_168;
  undefined1 local_160 [16];
  undefined8 *******local_150;
  undefined8 *******pppppppuStack_148;
  undefined8 *******local_140;
  undefined8 *puStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 *******local_118;
  undefined8 *******pppppppuStack_110;
  undefined8 *******local_108;
  undefined8 *******local_100;
  undefined8 *******pppppppuStack_f8;
  undefined8 *******local_f0;
  long local_e0;
  long local_d8;
  long lStack_d0;
  undefined8 *******local_c8;
  undefined8 *******pppppppuStack_c0;
  undefined8 *******local_b8;
  undefined8 *******pppppppuStack_b0;
  undefined8 *******local_a8;
  undefined8 *puStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;

  if ((*(byte *)(param_1 + 0xa8) & 1) != 0) {
    return true;
  }
  if ((*(long *)(param_1 + 0x18) == -0x8000000000000000) || (*(ulong *)(param_1 + 0x28) < 0x17)) {
    lVar8 = *(long *)(param_1 + 0x30);
    lVar10 = *(long *)(param_1 + 0x48);
  }
  else {
    plVar9 = *(long **)(param_1 + 0x20);
    lVar8 = *(long *)(param_1 + 0x30);
    lVar10 = *(long *)(param_1 + 0x48);
    if ((((*plVar9 == 0x6d616d69612d6b73 && plVar9[1] == 0x2d79616c65722d69) && *(long *)((long)plVar9 + 0xf) == 0x6c6175747269762d) &&
        lVar8 == -0x8000000000000000) && lVar10 == -0x8000000000000000) {
      return true;
    }
  }
  uVar1 = 0;
  if (lVar8 != -0x8000000000000000) {
    uVar1 = *(undefined8 *)(param_1 + 0x38);
  }
  decode_jwt_claims(auStack_1e0,uVar1,*(undefined8 *)(param_1 + 0x40));
  uVar1 = 0;
  if (lVar10 != -0x8000000000000000) {
    uVar1 = *(undefined8 *)(param_1 + 0x50);
  }
  decode_jwt_claims(auStack_1b0,uVar1,*(undefined8 *)(param_1 + 0x58));
  nested_dict(&local_b8,auStack_1e0,
              "https://api.openai.com/authhttps://api.openai.com/profileemailchatgpt_user_idchatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
              ,0x1b);
  if (local_b8 == (undefined8 *******)0x0) {
    nested_dict(&local_150,auStack_1b0,
                "https://api.openai.com/authhttps://api.openai.com/profileemailchatgpt_user_idchatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
                ,0x1b);
  }
  else {
    pppppppuStack_148 = pppppppuStack_b0;
    local_150 = local_b8;
    puStack_138 = puStack_a0;
    local_140 = local_a8;
    uStack_128 = uStack_90;
    local_130 = local_98;
  }
  if (local_150 == (undefined8 *******)0x0) {
    pauVar5 = (undefined1 (*) [16])(*(code *)std::hash::random::RandomState::new::KEYS::{{constant}}::{{closure}}::__RUST_STD_INTERNAL_VAL)();
    if (pauVar5[1][0] == '\x01') {
      local_160 = *pauVar5;
    }
    else {
      local_160 = std::sys::random::hashmap_random_keys();
      *pauVar5 = local_160;
      pauVar5[1][0] = 1;
    }
    *(long *)*pauVar5 = local_160._0_8_ + 1;
    pppppppuStack_178 = (undefined8 *******)0x0;
    local_180 = (undefined8 *******)&DAT_101303e80;
    puStack_168 = (undefined8 *)0x0;
    pppppppuStack_170 = (undefined8 *******)0x0;
    if (*(long *)(param_1 + 0x78) != -0x8000000000000000) goto LAB_1001f10e8;
LAB_1001f1104:
    string_val(&local_118,&local_180,"chatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens",0x12);
    pppppppuVar7 = local_118;
    pppppppuVar6 = pppppppuStack_110;
    pppppppuVar2 = local_108;
  }
  else {
    pppppppuStack_178 = pppppppuStack_148;
    local_180 = local_150;
    puStack_168 = puStack_138;
    pppppppuStack_170 = local_140;
    local_160._8_8_ = uStack_128;
    local_160._0_8_ = local_130;
    if (*(long *)(param_1 + 0x78) == -0x8000000000000000) goto LAB_1001f1104;
LAB_1001f10e8:
    <alloc::string::String_as_core::clone::Clone>::clone(&local_b8,param_1 + 0x78);
    if (local_b8 == (undefined8 *******)&DAT_8000000000000000) goto LAB_1001f1104;
    pppppppuVar7 = local_b8;
    pppppppuVar6 = pppppppuStack_b0;
    pppppppuVar2 = local_a8;
    if (local_a8 == (undefined8 *******)0x0) {
      if (local_b8 != (undefined8 *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuStack_b0,local_b8,1);
      }
      goto LAB_1001f1104;
    }
  }
  local_108 = pppppppuVar2;
  pppppppuStack_110 = pppppppuVar6;
  local_118 = pppppppuVar7;
  string_val(&local_b8,&local_180,
             "chatgpt_user_idchatgpt_plan_typenamechatgpt_subscription_active_untilauth.json contains AiMaMi managed virtual auth; not a real accountunknown-accountuser_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens"
             ,0xf);
  if (local_b8 == (undefined8 *******)&DAT_8000000000000000) {
    string_val(&local_150,&local_180,
               "chatgpt_account_user_idlast_refresh is outside the supported timestamp rangelast_refresh must be a timestamp string or number",0x17);
    if (local_150 == (undefined8 *******)&DAT_8000000000000000) {
LAB_1001f116c:
      string_val(&local_100,&local_180,
                 "user_idUnknown Accountiatchatgpt_account_id%Y-%m-%dT%H:%M:%S%.fauth_modeOPENAI_API_KEYaimami_router_unlock_authtokenslast_refreshAuthTokens",7
                );
      goto LAB_1001f11b8;
    }
  }
  else {
    pppppppuStack_148 = pppppppuStack_b0;
    local_150 = local_b8;
    local_140 = local_a8;
    if (local_b8 == (undefined8 *******)&DAT_8000000000000000) goto LAB_1001f116c;
  }
  pppppppuStack_f8 = pppppppuStack_148;
  local_100 = local_150;
  local_f0 = local_140;
LAB_1001f11b8:
  pppppppuVar6 = pppppppuStack_f8;
  pppppppuVar7 = local_100;
  bVar4 = local_100 != (undefined8 *******)&DAT_8000000000000000;
  if (bVar4 && local_118 != (undefined8 *******)&DAT_8000000000000000) {
    local_c8 = pppppppuStack_f8;
    pppppppuStack_c0 = local_f0;
    local_150 = pppppppuStack_110;
    pppppppuStack_148 = local_108;
    local_b8 = &local_c8;
    pppppppuStack_b0 = (undefined8 *******)&<&T_as_core::fmt::Display>::fmt;
    local_a8 = &local_150;
    puStack_a0 = &<&T_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_e0,s____10151cfd8,&local_b8);
    ::core::str::pattern::StrSearcher::new
              (&local_b8,local_d8,lStack_d0,
               "::organizationstitlechatgpt_account_user_idlast_refresh is outside the supported timestamp rangelast_refresh must be a timestamp string or number"
               ,2);
    <core::str::pattern::StrSearcher_as_core::str::pattern::Searcher>::next_match(&local_150,&local_b8);
    if (((int)local_150 == 1) && (lStack_d0 - (long)local_140 == 0x24)) {
      bVar3 = false;
      plVar9 = (long *)(local_d8 + (long)local_140);
      if ((((*plVar9 == 0x3030303030303030 && plVar9[1] == 0x30342d303030302d) && (plVar9[2] == 0x2d303030382d3030 && plVar9[3] == 0x3030303030303030)) &&
           (int)plVar9[4] == 0x31303030) && (&SUB_00000006 < pppppppuStack_148)) {
        bVar3 = *(int *)((long)pppppppuStack_148 + local_d8 + -7) == 0x6c6e752d && *(int *)((long)pppppppuStack_148 + local_d8 + -4) == 0x6b636f6c;
      }
    }
    else {
      bVar3 = false;
    }
    if (local_e0 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_d8,local_e0,1);
    }
    if (local_118 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuStack_110,local_118,1);
    }
    if (pppppppuVar7 != (undefined8 *******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar6,pppppppuVar7,1);
    }
  }
  else {
    bVar3 = false;
  }
  <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(&local_180);
  <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(auStack_1b0);
  <hashbrown::raw::RawTable<T,A>as_core::ops::drop::Drop>::drop(auStack_1e0);
  if (((((ulong)pppppppuVar7 & 0x7fffffffffffffff) != 0) && (local_118 == (undefined8 *******)&DAT_8000000000000000)) ||
     (pppppppuVar6 = pppppppuStack_110, pppppppuVar7 = local_118,
     ((ulong)local_118 & 0x7fffffffffffffff) != 0 && (!bVar4 || local_118 == (undefined8 *******)&DAT_8000000000000000))) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar6,pppppppuVar7,1);
  }
  return bVar3;
}
