/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* to_vec_pretty @ 10080bd84 requested 10080bd84 */

/* serde_json::ser::to_vec_pretty */

void serde_json::ser::to_vec_pretty(long *param_1,long param_2)

{
  long lVar1;
  long local_80;
  undefined1 *puStack_78;
  long local_70;
  undefined1 *local_68;
  byte *pbStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined1 local_48;
  byte local_40;
  undefined1 uStack_3f;
  undefined6 uStack_3e;
  undefined1 **ppuStack_38;
  undefined8 local_30;
  undefined1 **ppuStack_28;

  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  puStack_78 = (undefined1 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x80,1);
  if (puStack_78 == (undefined1 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x80);
  }
  local_80 = 0x80;
  pbStack_60 = &BYTE_1013af7fc;
  uStack_50 = 1;
  local_58 = 2;
  local_48 = 0;
  *puStack_78 = 0x7b;
  local_70 = 1;
  local_40 = 0;
  uStack_3f = 1;
  ppuStack_38 = &local_68;
  local_68 = (undefined1 *)&local_80;
  lVar1 = serde_core::ser::SerializeMap::serialize_entry
                    (&local_40,
                     "schemaVersionupdatedAtactiveAccountKeyitemsautoSwitchapiRegistryItemaccountKeysnapshotPathemailaliasaccountNameworkspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                     ,0xd,param_2 + 0x50);
  if (lVar1 != 0) goto LAB_10080be40;
  if ((local_40 & 1) == 0) {
    lVar1 = serde_core::ser::SerializeMap::serialize_entry
                      (&local_40,
                       "updatedAtactiveAccountKeyitemsautoSwitchapiRegistryItemaccountKeysnapshotPathemailaliasaccountNameworkspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                       ,9,param_2 + 0x48);
    if (lVar1 != 0) goto LAB_10080be40;
    if ((local_40 & 1) == 0) {
      lVar1 = serde_core::ser::SerializeMap::serialize_entry
                        (&local_40,
                         "activeAccountKeyitemsautoSwitchapiRegistryItemaccountKeysnapshotPathemailaliasaccountNameworkspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                         ,0x10,param_2 + 0x18);
      if (lVar1 != 0) goto LAB_10080be40;
      if ((local_40 & 1) == 0) {
        lVar1 = serde_core::ser::SerializeMap::serialize_entry
                          (&local_40,
                           "itemsautoSwitchapiRegistryItemaccountKeysnapshotPathemailaliasaccountNameworkspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                           ,5,param_2);
        if (((lVar1 == 0) &&
            (lVar1 = <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::serialize_field(&local_40,param_2 + 0x30), lVar1 == 0)) &&
           (lVar1 = <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::serialize_field(&local_40,param_2 + 0x54), lVar1 == 0)) {
          local_30 = CONCAT62(uStack_3e,CONCAT11(uStack_3f,local_40));
          ppuStack_28 = ppuStack_38;
          <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::end(&local_30);
          param_1[1] = (long)puStack_78;
          *param_1 = local_80;
          param_1[2] = local_70;
          return;
        }
        goto LAB_10080be40;
      }
    }
  }
  lVar1 = invalid_raw_value();
LAB_10080be40:
  *param_1 = -0x8000000000000000;
  param_1[1] = lVar1;
  if (local_80 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_78,local_80,1);
  }
  return;
}
