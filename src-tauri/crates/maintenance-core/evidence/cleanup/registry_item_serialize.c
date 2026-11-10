/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* serialize @ 10082a1e8 requested 10082a1e8 */

/* codexmate_lib::core::repository::_::<impl serde_core::ser::Serialize for codexmate_lib::core::repository::RegistryItem>::serialize */

long codexmate_lib::core::repository::RegistryItem>::serialize(long param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  char cVar3;
  char cVar4;
  undefined8 *puVar5;
  long lVar6;
  long *plVar7;
  byte local_70;
  undefined1 uStack_6f;
  undefined6 uStack_6e;
  undefined8 *puStack_68;
  undefined8 local_60;
  undefined8 *puStack_58;

  cVar3 = *(char *)(param_1 + 0x1a2);
  cVar4 = *(char *)(param_1 + 0x1a3);
  plVar7 = (long *)*param_2;
  param_2[3] = param_2[3] + 1;
  *(undefined1 *)(param_2 + 4) = 0;
  lVar6 = plVar7[2];
  if (*plVar7 == lVar6) {
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar7,lVar6,1,1,1);
    lVar6 = plVar7[2];
  }
  *(undefined1 *)(plVar7[1] + lVar6) = 0x7b;
  plVar7[2] = lVar6 + 1;
  local_70 = 0;
  uStack_6f = 1;
  puStack_68 = param_2;
  <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeMap>::serialize_key
            (&local_70,
             "accountKeysnapshotPathemailaliasaccountNameworkspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
             ,10);
  puVar5 = puStack_68;
  if (local_70 != 1) {
    uVar1 = *(undefined8 *)(param_1 + 200);
    uVar2 = *(undefined8 *)(param_1 + 0xd0);
    plVar7 = (long *)*puStack_68;
    lVar6 = plVar7[2];
    if ((ulong)(*plVar7 - lVar6) < 2) {
      alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar7,lVar6,2,1,1);
      lVar6 = plVar7[2];
    }
    *(undefined2 *)(plVar7[1] + lVar6) = 0x203a;
    plVar7[2] = plVar7[2] + 2;
    serde_json::ser::format_escaped_str(puVar5,lVar6,uVar1,uVar2);
    *(undefined1 *)(puVar5 + 4) = 1;
    <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeMap>::serialize_key
              (&local_70,
               "snapshotPathemailaliasaccountNameworkspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
               ,0xc);
    puVar5 = puStack_68;
    if (local_70 != 1) {
      uVar1 = *(undefined8 *)(param_1 + 0xe0);
      uVar2 = *(undefined8 *)(param_1 + 0xe8);
      plVar7 = (long *)*puStack_68;
      lVar6 = plVar7[2];
      if ((ulong)(*plVar7 - lVar6) < 2) {
        alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar7,lVar6,2,1,1);
        lVar6 = plVar7[2];
      }
      *(undefined2 *)(plVar7[1] + lVar6) = 0x203a;
      plVar7[2] = plVar7[2] + 2;
      serde_json::ser::format_escaped_str(puVar5,lVar6,uVar1,uVar2);
      *(undefined1 *)(puVar5 + 4) = 1;
      <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeMap>::serialize_key
                (&local_70,
                 "emailaliasaccountNameworkspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                 ,5);
      puVar5 = puStack_68;
      if (local_70 != 1) {
        uVar1 = *(undefined8 *)(param_1 + 0xf8);
        uVar2 = *(undefined8 *)(param_1 + 0x100);
        plVar7 = (long *)*puStack_68;
        lVar6 = plVar7[2];
        if ((ulong)(*plVar7 - lVar6) < 2) {
          alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar7,lVar6,2,1,1);
          lVar6 = plVar7[2];
        }
        *(undefined2 *)(plVar7[1] + lVar6) = 0x203a;
        plVar7[2] = plVar7[2] + 2;
        serde_json::ser::format_escaped_str(puVar5,lVar6,uVar1,uVar2);
        *(undefined1 *)(puVar5 + 4) = 1;
        lVar6 = serde_core::ser::SerializeMap::serialize_entry
                          (&local_70,
                           "aliasaccountNameworkspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                           ,5,param_1 + 0x108);
        if (lVar6 != 0) {
          return lVar6;
        }
        if ((local_70 & 1) == 0) {
          lVar6 = serde_core::ser::SerializeMap::serialize_entry
                            (&local_70,
                             "accountNameworkspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                             ,0xb,param_1 + 0x150);
          if (lVar6 != 0) {
            return lVar6;
          }
          if ((local_70 & 1) == 0) {
            lVar6 = serde_core::ser::SerializeMap::serialize_entry
                              (&local_70,
                               "workspaceNameprofileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                               ,0xd,param_1 + 0x168);
            if (lVar6 != 0) {
              return lVar6;
            }
            if ((local_70 & 1) == 0) {
              lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                (&local_70,
                                 "profileNameplanplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                 ,0xb,param_1 + 0x180);
              if (lVar6 != 0) {
                return lVar6;
              }
              if ((local_70 & 1) == 0) {
                lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                  (&local_70,
                                   "planplanStateSourceplanStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                   ,4,param_1 + 0x120);
                if (lVar6 != 0) {
                  return lVar6;
                }
                if ((cVar3 != '\x03') &&
                   (lVar6 = <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::serialize_field(&local_70,param_1 + 0x1a2), lVar6 != 0)) {
                  return lVar6;
                }
                if ((local_70 & 1) == 0) {
                  lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                    (&local_70,
                                     "planStateUpdatedAtauthModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                     ,0x12,param_1);
                  if (lVar6 != 0) {
                    return lVar6;
                  }
                  if ((local_70 & 1) == 0) {
                    lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                      (&local_70,
                                       "authModehasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                       ,8,param_1 + 0x138);
                    if (lVar6 != 0) {
                      return lVar6;
                    }
                    if ((local_70 & 1) == 0) {
                      lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                        (&local_70,
                                         "hasActiveSubscriptionsubscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                         ,0x15,param_1 + 0x1a0);
                      if (lVar6 != 0) {
                        return lVar6;
                      }
                      if ((local_70 & 1) == 0) {
                        lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                          (&local_70,
                                           "subscriptionExpiresAtsubscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                           ,0x15,param_1 + 0x10);
                        if (lVar6 != 0) {
                          return lVar6;
                        }
                        if ((local_70 & 1) == 0) {
                          lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                            (&local_70,
                                             "subscriptionWillRenewsubscriptionStateSourcesubscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                             ,0x15,param_1 + 0x1a1);
                          if (lVar6 != 0) {
                            return lVar6;
                          }
                          if ((cVar4 != '\x03') &&
                             (lVar6 = <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::serialize_field(&local_70,param_1 + 0x1a3),
                             lVar6 != 0)) {
                            return lVar6;
                          }
                          if ((local_70 & 1) == 0) {
                            lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                              (&local_70,
                                               "subscriptionLastAttemptAtsubscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                               ,0x19,param_1 + 0x20);
                            if (lVar6 != 0) {
                              return lVar6;
                            }
                            if ((local_70 & 1) == 0) {
                              lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                                (&local_70,
                                                 "subscriptionLastCheckedAtsubscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                                 ,0x19,param_1 + 0x30);
                              if (lVar6 != 0) {
                                return lVar6;
                              }
                              if ((local_70 & 1) == 0) {
                                lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                                  (&local_70,
                                                   "subscriptionStateUpdatedAtcreatedAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                                   ,0x1a,param_1 + 0x40);
                                if (lVar6 != 0) {
                                  return lVar6;
                                }
                                if ((local_70 & 1) == 0) {
                                  lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                                    (&local_70,
                                                     "createdAtlastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                                     ,9,param_1 + 0x198);
                                  if (lVar6 != 0) {
                                    return lVar6;
                                  }
                                  if ((local_70 & 1) == 0) {
                                    lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                                      (&local_70,
                                                       "lastUsedAtlastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                                       ,10,param_1 + 0x50);
                                    if (lVar6 != 0) {
                                      return lVar6;
                                    }
                                    if ((local_70 & 1) == 0) {
                                      lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                                        (&local_70,
                                                         "lastUsageAtcachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                                         ,0xb,param_1 + 0x60);
                                      if (lVar6 != 0) {
                                        return lVar6;
                                      }
                                      if ((local_70 & 1) == 0) {
                                        lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                                          (&local_70,
                                                           "cachedPrimaryWindowcachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                                           ,0x13,param_1 + 0x70);
                                        if (lVar6 != 0) {
                                          return lVar6;
                                        }
                                        if ((local_70 & 1) == 0) {
                                          lVar6 = serde_core::ser::SerializeMap::serialize_entry
                                                            (&local_70,
                                                             "cachedSecondaryWindowHotspotConfigenabledAutoSwitchConfigrevisionthreshold5hPercentthresholdWeeklyPercentCodexMateSettingshotspotusageRefreshIntervalusageRefreshAllAccountsdeviceIdremoteDeviceSecretnotificationsSinceapiProxyautostartWhenRouterOnMysteryRouteGrantrouteexpiresAtMs"
                                                             ,0x15,param_1 + 0x98);
                                          if (lVar6 != 0) {
                                            return lVar6;
                                          }
                                          local_60 = CONCAT62(uStack_6e,CONCAT11(uStack_6f,local_70));
                                          puStack_58 = puStack_68;
                                          <serde_json::ser::Compound<W,F>as_serde_core::ser::SerializeStruct>::end(&local_60);
                                          return 0;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
        lVar6 = serde_json::ser::invalid_raw_value();
        return lVar6;
      }
    }
  }
                    /* WARNING: Subroutine does not return */
  ::core::panicking::panic
            ("internal error: entered unreachable codeinternal error: entered unreachable code: invalid Once state()a Display implementation returned an error unexpectedlycalled `Result::unwrap_err()` on an `Ok` valuecalled `Result::unwrap()` on an `Err` valueoffset_secondoffset_hourErrorUnsupportedKeyEmptyTokenInvalidFormatdropped"
             ,0x28,&_anon_8fa53a009f7cd8c90f3f229cc309c2bf_408);
}
