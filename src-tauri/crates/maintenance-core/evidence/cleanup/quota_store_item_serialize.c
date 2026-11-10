/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* serialize @ 1001e8350 requested 1001e8350 */

/* codexmate_lib::core::quota_store::_::<impl serde_core::ser::Serialize for codexmate_lib::core::quota_store::QuotaStoreItem>::serialize */

long codexmate_lib::core::quota_store::QuotaStoreItem>::serialize(long *param_1,undefined8 *param_2)

{
  long lVar1;
  undefined8 *puVar2;
  long lVar3;
  long lVar4;
  long *plVar5;
  long lVar6;
  long lVar7;
  byte local_70;
  undefined1 uStack_6f;
  undefined6 uStack_6e;
  undefined8 *puStack_68;
  undefined8 local_60;
  undefined8 *puStack_58;

  lVar7 = param_1[0xc];
  lVar6 = *param_1;
  plVar5 = (long *)*param_2;
  param_2[3] = param_2[3] + 1;
  *(undefined1 *)(param_2 + 4) = 0;
  lVar4 = plVar5[2];
  if (*plVar5 == lVar4) {
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar5,lVar4,1,1,1);
    lVar4 = plVar5[2];
  }
  *(undefined1 *)(plVar5[1] + lVar4) = 0x7b;
  plVar5[2] = lVar4 + 1;
  local_70 = 0;
  uStack_6f = 1;
  puStack_68 = param_2;
  <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeMap>::serialize_key
            (&local_70,
             "accountKeycapturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
             ,10);
  puVar2 = puStack_68;
  if (local_70 == 1) {
                    /* WARNING: Subroutine does not return */
    ::core::panicking::panic
              ("internal error: entered unreachable codeinternal error: entered unreachable code: invalid Once state()a Display implementation returned an error unexpectedlycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` valueoffset_secondoffset_hourErrorUnsupportedKeyEmptyTokenInvalidFormatdropped"
               ,0x28,&_anon_8fa53a009f7cd8c90f3f229cc309c2bf_408);
  }
  lVar4 = param_1[0x13];
  lVar1 = param_1[0x14];
  plVar5 = (long *)*puStack_68;
  lVar3 = plVar5[2];
  if ((ulong)(*plVar5 - lVar3) < 2) {
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar5,lVar3,2,1,1);
    lVar3 = plVar5[2];
  }
  *(undefined2 *)(plVar5[1] + lVar3) = 0x203a;
  plVar5[2] = plVar5[2] + 2;
  serde_json::ser::format_escaped_str(puVar2,lVar3,lVar4,lVar1);
  *(undefined1 *)(puVar2 + 4) = 1;
  lVar4 = serde_core::ser::SerializeMap::serialize_entry
                    (&local_70,
                     "capturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
                     ,10,param_1 + 0x16);
  if (lVar4 == 0) {
    if ((local_70 & 1) != 0) {
LAB_1001e8468:
      lVar4 = serde_json::ser::invalid_raw_value();
      return lVar4;
    }
    lVar4 = serde_core::ser::SerializeMap::serialize_entry
                      (&local_70,
                       "usageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
                       ,0xb,param_1 + 0x17);
    if (lVar4 == 0) {
      if ((local_70 & 1) != 0) goto LAB_1001e8468;
      lVar4 = serde_core::ser::SerializeMap::serialize_entry
                        (&local_70,
                         "primaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
                         ,0xd,param_1 + 2);
      if (lVar4 == 0) {
        if ((local_70 & 1) != 0) goto LAB_1001e8468;
        lVar4 = serde_core::ser::SerializeMap::serialize_entry
                          (&local_70,
                           "secondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
                           ,0xf,param_1 + 7);
        if ((((lVar4 == 0) &&
             (lVar4 = <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::serialize_field(&local_70,param_1 + 0x15), lVar4 == 0)) &&
            ((lVar7 == 2 || (lVar4 = <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::serialize_field(&local_70,param_1 + 0xc), lVar4 == 0))
            )) && ((lVar6 == 0 || (lVar4 = <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::serialize_field(&local_70,param_1), lVar4 == 0))
                  )) {
          local_60 = CONCAT62(uStack_6e,CONCAT11(uStack_6f,local_70));
          puStack_58 = puStack_68;
          <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::end(&local_60);
          lVar4 = 0;
        }
      }
    }
  }
  return lVar4;
}
