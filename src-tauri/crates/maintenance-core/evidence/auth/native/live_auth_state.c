/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* live_auth_state @ 1008a8b64 requested 1008a8b64 */

/* codexmate_lib::core::relay::router_unlock_auth::live_auth_state */

byte codexmate_lib::core::relay::router_unlock_auth::live_auth_state(long param_1)

{
  uint uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long lVar4;
  long lVar5;
  char *pcVar6;
  byte *pbVar7;
  long *plVar8;
  uint uVar9;
  byte bVar10;
  undefined1 auVar11 [16];
  char local_f0 [8];
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  long local_d0;
  long lStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  long local_b0;
  undefined8 uStack_a8;

  uVar2 = *(undefined8 *)(param_1 + 0x20);
  uVar3 = *(undefined8 *)(param_1 + 0x28);
  std::sys::fs::metadata(&local_d0,uVar2,uVar3);
  if ((int)local_d0 == 1) {
    ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(1,lStack_c8);
    return 0;
  }
  ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(0,lStack_c8);
  std::fs::read_to_string::inner(&local_d0,uVar2,uVar3);
  uStack_a8 = local_c0;
  lVar5 = lStack_c8;
  lVar4 = local_d0;
  if (local_d0 == -0x8000000000000000) {
    ::core::ptr::drop_in_place<core::result::Result<alloc::string::String,std::io::error::Error>>(0x8000000000000000,lStack_c8);
    return 4;
  }
  local_d0 = lStack_c8;
  lStack_c8 = local_c0;
  local_c0 = 0;
  uStack_b8 = 0;
  local_b0 = lVar5;
  serde_json::de::from_trait(local_f0,&local_d0);
  if (local_f0[0] == '\x06') {
    ::core::ptr::drop_in_place<serde_json::error::Error>(&uStack_e8);
    if (lVar4 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar5,lVar4,1);
      return 4;
    }
    return 4;
  }
  lStack_c8 = uStack_e8;
  uStack_b8 = uStack_d8;
  local_c0 = uStack_e0;
  pcVar6 = (char *)<str_as_serde_json::value::index::Index>::index_into("auth_modeOPENAI_API_KEY",9,&local_d0);
  if ((pcVar6 != (char *)0x0) && (*pcVar6 == '\x03')) {
    auVar11 = ::core::str::<impl_str>::trim_matches(*(undefined8 *)(pcVar6 + 0x10),*(undefined8 *)(pcVar6 + 0x18));
    pbVar7 = auVar11._0_8_;
    if (auVar11._8_8_ == 6) {
      uVar9 = 0x20;
      uVar1 = uVar9;
      if (0x19 < *pbVar7 - 0x41) {
        uVar1 = 0;
      }
      if ((uVar1 | *pbVar7) == 0x61) {
        if (0x19 < pbVar7[1] - 0x41) {
          uVar9 = 0;
        }
        if ((uVar9 | pbVar7[1]) == 0x70) {
          uVar9 = 0x20;
          uVar1 = uVar9;
          if (0x19 < pbVar7[2] - 0x41) {
            uVar1 = 0;
          }
          if ((uVar1 | pbVar7[2]) == 0x69) {
            if (0x19 < pbVar7[3] - 0x41) {
              uVar9 = 0;
            }
            if ((uVar9 | pbVar7[3]) == 0x6b) {
              uVar9 = 0x20;
              uVar1 = uVar9;
              if (0x19 < pbVar7[4] - 0x41) {
                uVar1 = 0;
              }
              if ((uVar1 | pbVar7[4]) == 0x65) {
                if (0x19 < pbVar7[5] - 0x41) {
                  uVar9 = 0;
                }
                if ((((uVar9 | pbVar7[5]) == 0x79) &&
                    (pcVar6 = (char *)<str_as_serde_json::value::index::Index>::index_into("OPENAI_API_KEY",0xe,&local_d0), pcVar6 != (char *)0x0)) &&
                   (*pcVar6 == '\x03')) {
                  auVar11 = ::core::str::<impl_str>::trim_matches(*(undefined8 *)(pcVar6 + 0x10),*(undefined8 *)(pcVar6 + 0x18));
                  plVar8 = auVar11._0_8_;
                  if ((((plVar8 != (long *)0x0) && (0x16 < auVar11._8_8_)) &&
                      ((*plVar8 == 0x6d616d69612d6b73 && plVar8[1] == 0x2d79616c65722d69) && *(long *)((long)plVar8 + 0xf) == 0x6c6175747269762d)) &&
                     (((pcVar6 = (char *)<str_as_serde_json::value::index::Index>::index_into(&DAT_1013b375d,6,&local_d0), pcVar6 == (char *)0x0 ||
                       (*pcVar6 != '\x05')) || (*(long *)(pcVar6 + 0x18) == 0)))) {
                    bVar10 = 1;
                    goto LAB_1008a8de4;
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  pcVar6 = (char *)<str_as_serde_json::value::index::Index>::index_into
                             ("aimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
                              ,0x19,&local_d0);
  if ((pcVar6 == (char *)0x0) || (*pcVar6 != '\x01')) {
    bVar10 = 3;
  }
  else {
    bVar10 = pcVar6[1] ^ 3;
  }
LAB_1008a8de4:
  ::core::ptr::drop_in_place<serde_json::value::Value>(&local_d0);
  if (lVar4 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar5,lVar4,1);
  }
  return bVar10;
}
