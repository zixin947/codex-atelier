/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* serialize @ 1007a506c requested 1007a506c */

/* codexmate_lib::core::models::_::<impl serde_core::ser::Serialize for codexmate_lib::core::models::AccountTokenStatus>::serialize */

long codexmate_lib::core::models::AccountTokenStatus>::serialize(long *param_1,undefined8 *param_2)

{
  long lVar1;
  long *plVar2;
  long lVar3;
  long lVar4;
  undefined2 local_50;
  undefined8 *local_48;

  lVar4 = param_1[2];
  lVar3 = *param_1;
  plVar2 = (long *)*param_2;
  lVar1 = plVar2[2];
  if (*plVar2 == lVar1) {
    alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar2,lVar1,1,1,1);
    lVar1 = plVar2[2];
  }
  *(undefined1 *)(plVar2[1] + lVar1) = 0x7b;
  plVar2[2] = lVar1 + 1;
  local_50 = 0x100;
  local_48 = param_2;
  lVar1 = serde_core::ser::SerializeMap::serialize_entry
                    (&local_50,
                     "codetypedata did not match any variant of untagged enum TokenEndpointErrorValue%Y-%m-%dT%H:%M:%S%.6fZiat[AiMaMi][oauth-refresh] active auth ownership changed before refreshcodexmate_lib::core::oauth_refreshactive account refresh lost the live auth pathtoken endpoint returned an empty access_token[AiMaMi][oauth-refresh] skipped auth.json sync because live auth changed[AiMaMi][oauth-refresh] skipped auth.json sync because Codex became activetoken endpoint returned credentials for a different accountactive account token refresh deferred while Codex owns auth.json"
                     ,4,param_1 + 5);
  if (lVar1 == 0) {
    if (lVar4 != -0x8000000000000000) {
      if ((local_50 & 1) != 0) goto LAB_1007a5180;
      lVar1 = serde_core::ser::SerializeMap::serialize_entry
                        (&local_50,
                         "credentialGenerationaccessTokenExpiresAtskillIDtitlerelativePathbackupPathskillreplacedExistingremovedSkillIDremainingInstalledCount",
                         0x14,param_1 + 2);
      if (lVar1 != 0) {
        return lVar1;
      }
    }
    if (lVar3 != 0) {
      if ((local_50 & 1) != 0) {
LAB_1007a5180:
        lVar1 = serde_json::ser::invalid_raw_value();
        return lVar1;
      }
      lVar1 = serde_core::ser::SerializeMap::serialize_entry
                        (&local_50,"accessTokenExpiresAtskillIDtitlerelativePathbackupPathskillreplacedExistingremovedSkillIDremainingInstalledCount",0x14,
                         param_1);
      if (lVar1 != 0) {
        return lVar1;
      }
    }
    lVar1 = 0;
    if (((local_50 & 1) == 0) && (local_50._1_1_ != '\0')) {
      plVar2 = (long *)*local_48;
      lVar3 = plVar2[2];
      if (*plVar2 == lVar3) {
        alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar2,lVar3,1,1,1);
        lVar3 = plVar2[2];
      }
      lVar1 = 0;
      *(undefined1 *)(plVar2[1] + lVar3) = 0x7d;
      plVar2[2] = lVar3 + 1;
    }
  }
  return lVar1;
}
