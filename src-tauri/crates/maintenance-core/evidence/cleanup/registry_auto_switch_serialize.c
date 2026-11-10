/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* serialize @ 10082a91c requested 10082a91c */

/* codexmate_lib::core::repository::_::<impl serde_core::ser::Serialize for codexmate_lib::core::repository::AutoSwitchConfig>::serialize */

long codexmate_lib::core::repository::AutoSwitchConfig>::serialize(long param_1,undefined8 *param_2)

{
  long lVar1;
  long *plVar2;
  ushort local_40 [4];
  undefined8 *local_38;

  plVar2 = (long *)*param_2;
  param_2[3] = param_2[3] + 1;
  *(undefined1 *)(param_2 + 4) = 0;
  lVar1 = plVar2[2];
  if (*plVar2 == lVar1) {
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar2,lVar1,1,1,1);
    lVar1 = plVar2[2];
  }
  *(undefined1 *)(plVar2[1] + lVar1) = 0x7b;
  plVar2[2] = lVar1 + 1;
  local_40[0] = 0x100;
  local_38 = param_2;
  lVar1 = serde_core::ser::SerializeMap::serialize_entry
                    (local_40,
                     "revisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                     ,8,param_1);
  if (lVar1 == 0) {
    if ((local_40[0] & 1) != 0) {
LAB_10082a9b0:
      lVar1 = serde_json::ser::invalid_raw_value();
      return lVar1;
    }
    lVar1 = serde_core::ser::SerializeMap::serialize_entry
                      (local_40,
                       "enabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                       ,7,param_1 + 0x10);
    if (lVar1 == 0) {
      if ((local_40[0] & 1) != 0) goto LAB_10082a9b0;
      lVar1 = serde_core::ser::SerializeMap::serialize_entry
                        (local_40,
                         "threshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                         ,0x12,param_1 + 8);
      if (lVar1 == 0) {
        if ((local_40[0] & 1) != 0) goto LAB_10082a9b0;
        lVar1 = serde_core::ser::SerializeMap::serialize_entry
                          (local_40,
                           "thresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                           ,0x16,param_1 + 0xc);
        if (lVar1 == 0) {
          <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::end(local_40);
          lVar1 = 0;
        }
      }
    }
  }
  return lVar1;
}
