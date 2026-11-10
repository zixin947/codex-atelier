/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* cleanup_unlocked @ 1008a8e88 requested 1008a8e88 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::router_unlock_auth::cleanup_unlocked */

void codexmate_lib::core::relay::router_unlock_auth::cleanup_unlocked(undefined8 *param_1,long param_2)

{
  ulong ****ppppuVar1;
  ulong ****ppppuVar2;
  undefined8 ****ppppuVar3;
  undefined8 uVar4;
  ulong uVar5;
  undefined8 ****ppppuVar6;
  undefined8 ****ppppuVar7;
  undefined8 ****ppppuVar8;
  undefined8 uVar9;
  char cVar10;
  byte bVar11;
  uint uVar12;
  undefined8 **ppuVar13;
  undefined8 ****ppppuVar14;
  undefined8 uVar15;
  undefined1 uVar16;
  undefined8 uVar17;
  char *pcVar18;
  undefined8 *puVar19;
  undefined8 ****ppppuVar20;
  undefined8 *puVar21;
  undefined1 auVar22 [16];
  long local_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  long local_1e8;
  undefined8 local_1e0;
  undefined8 uStack_1d8;
  undefined8 ****local_1d0;
  undefined8 ****ppppuStack_1c8;
  undefined8 ****local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  undefined8 ****ppppuStack_1a8;
  undefined8 ****ppppuStack_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  ulong uStack_188;
  ulong **local_178;
  undefined8 local_170;
  undefined8 uStack_168;
  ulong ****local_160;
  undefined8 ****ppppuStack_158;
  undefined8 ****local_150;
  undefined8 ****local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 ****ppppuStack_130;
  undefined8 ****local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  ulong uStack_110;
  undefined8 ****local_108;
  ulong ****local_100;
  code *pcStack_f8;
  undefined8 ****local_f0;
  undefined8 ****ppppuStack_e8;
  undefined8 ****local_e0;
  undefined8 uStack_d8;
  undefined8 ****local_d0;
  undefined8 ****ppppuStack_c8;
  undefined8 uStack_c0;
  char *local_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  char *local_a0;
  undefined8 *****pppppuStack_98;
  undefined1 uStack_51;

  uVar15 = *(undefined8 *)(param_2 + 0x248);
  uVar17 = *(undefined8 *)(param_2 + 0x250);
  std::path::Path::_join
            (&local_200,uVar15,uVar17,
             "router-unlock-auth-marker.jsoncodexmate_lib::core::relay::router_unlock_authaimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
             ,0x1e);
  std::path::Path::_join
            (&local_1e8,uVar15,uVar17,
             "router-unlock-auth-backup.jsonrouter-unlock-auth-marker.jsoncodexmate_lib::core::relay::router_unlock_authaimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
             ,0x1e);
  std::path::Path::_join
            (&local_178,uVar15,uVar17,
             "router-unlock-auth-marker.jsoncodexmate_lib::core::relay::router_unlock_authaimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
             ,0x1e);
  std::fs::read_to_string::inner(&local_160,local_170,uStack_168);
  ppppuVar14 = ppppuStack_158;
  ppppuVar1 = local_160;
  if (local_160 != (undefined8 ****)0x8000000000000000) {
    local_f0 = ppppuStack_158;
    ppppuStack_e8 = local_150;
    local_e0 = (undefined8 ****)0x0;
    uStack_d8 = 0;
    local_d0 = ppppuStack_158;
    ppppuStack_c8 = local_150;
    serde_json::de::from_trait(&local_160,&local_f0);
    ppppuVar2 = local_160;
    ppppuVar20 = local_148;
    uVar15 = uStack_140;
    uVar17 = local_138;
    ppppuVar3 = ppppuStack_130;
    uVar4 = local_118;
    uVar5 = uStack_110;
    ppppuVar6 = ppppuStack_158;
    ppppuVar7 = local_150;
    ppppuVar8 = local_128;
    uVar9 = uStack_120;
    if (local_160 == (undefined8 ****)0x8000000000000001) {
      local_108 = ppppuStack_158;
      if (1 < log::MAX_LOG_LEVEL_FILTER) {
        local_100 = (ulong ****)&local_108;
        pcStack_f8 = <serde_json::error::Error_as_core::fmt::Display>::fmt;
        local_b8 =
        "codexmate_lib::core::relay::router_unlock_authaimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
        ;
        local_b0 = 0x2e;
        pppppuStack_98 = &local_100;
        local_a0 = s_2_AiMaMi__router_unlock_auth__ma_10152f680;
        local_f0 = (undefined8 ****)0x0;
        ppppuStack_e8 = (undefined8 ****)0x1013b4b0a;
        uStack_d8 = 0;
        local_e0 = (undefined8 ****)0x2e;
        local_d0 = (undefined8 ****)0x10152f6b5;
        uStack_c0 = 2;
        ppppuStack_c8 = (undefined8 ****)0x24;
        uStack_a8 = 0x26f00000001;
        <log::__private_api::GlobalLogger_as_log::Log>::log(&uStack_51,&local_f0);
      }
      ::core::ptr::drop_in_place<serde_json::error::Error>(&local_108);
      ppppuVar20 = local_1c0;
      uVar15 = uStack_1b8;
      uVar17 = local_1b0;
      ppppuVar3 = ppppuStack_1a8;
      uVar4 = local_190;
      uVar5 = uStack_188;
      ppppuVar6 = local_1d0;
      ppppuVar7 = ppppuStack_1c8;
      ppppuVar8 = ppppuStack_1a0;
      uVar9 = uStack_198;
    }
    uStack_198 = uVar9;
    ppppuStack_1a0 = ppppuVar8;
    ppppuStack_1c8 = ppppuVar7;
    local_1d0 = ppppuVar6;
    uStack_188 = uVar5;
    local_190 = uVar4;
    ppppuStack_1a8 = ppppuVar3;
    local_1b0 = uVar17;
    uStack_1b8 = uVar15;
    local_1c0 = ppppuVar20;
    if (ppppuVar1 != (undefined8 ****)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar14,ppppuVar1,1);
    }
    if (local_178 != (undefined8 **)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_170,local_178,1);
    }
    if (ppppuVar2 == (undefined8 ****)0x8000000000000001) goto LAB_1008a92a8;
    uStack_140 = uStack_1b8;
    local_148 = local_1c0;
    ppppuStack_130 = ppppuStack_1a8;
    local_138 = local_1b0;
    uStack_120 = uStack_198;
    local_128 = ppppuStack_1a0;
    uStack_110 = uStack_188;
    local_118 = local_190;
    local_150 = ppppuStack_1c8;
    ppppuStack_158 = local_1d0;
    local_160 = ppppuVar2;
    uVar12 = live_auth_state(param_2);
    uVar17 = uStack_1d8;
    uVar15 = local_1e0;
    uVar12 = uVar12 & 0xff;
    if (uVar12 < 3) {
      if (uVar12 - 1 < 2) {
        if ((uStack_110 & 1) == 0) {
LAB_1008a9154:
          uVar15 = *(undefined8 *)(param_2 + 0x20);
          uVar17 = *(undefined8 *)(param_2 + 0x28);
          std::sys::fs::metadata(&local_f0,uVar15,uVar17);
          ppppuVar14 = local_f0;
          ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_f0,ppppuStack_e8);
          if ((((ulong)ppppuVar14 & 1) == 0) && (ppuVar13 = (undefined8 **)std::sys::fs::remove_file(uVar15,uVar17), ppuVar13 != (undefined8 **)0x0)) {
            local_1d0 = (undefined8 ****)&local_178;
            ppppuStack_1c8 = (undefined8 ****)<std::io::error::Error_as_core::fmt::Display>::fmt;
            local_178 = ppuVar13;
            alloc::fmt::format::format_inner(&local_f0,s__remove_router_unlock_auth_json_f_10152f70e,&local_1d0);
            ppppuStack_1c8 = ppppuStack_e8;
            local_1d0 = local_f0;
            local_1c0 = local_e0;
            if (((ulong)local_178 & 3) == 1) {
              puVar21 = (undefined8 *)((long)local_178 - 1);
              uVar15 = *puVar21;
              puVar19 = *(undefined8 **)((long)local_178 + 7);
              if ((code *)*puVar19 != (code *)0x0) {
                (*(code *)*puVar19)(uVar15);
              }
              if (puVar19[1] != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar15,puVar19[1],puVar19[2]);
              }
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar21,0x18,8);
            }
            param_1[2] = ppppuStack_1c8;
            param_1[1] = local_1d0;
            param_1[3] = local_1c0;
            uVar15 = 10;
            goto LAB_1008a95e0;
          }
LAB_1008a9728:
          uVar16 = 1;
          goto LAB_1008a972c;
        }
        std::sys::fs::metadata(&local_f0,local_1e0,uStack_1d8);
        ppppuVar14 = local_f0;
        ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_f0,ppppuStack_e8);
        if (((ulong)ppppuVar14 & 1) != 0) goto LAB_1008a9154;
        read_cleanup_backup(&local_f0,uVar15,uVar17,
                            "router unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
                            ,0xd);
        ppppuVar20 = local_e0;
        ppppuVar14 = ppppuStack_e8;
        if (local_f0 != (undefined8 ****)0xb) goto LAB_1008a991c;
        if (ppppuStack_e8 != (undefined8 ****)0x8000000000000000) {
          auVar22 = atomic_write::write_atomic_private_report(*(undefined8 *)(param_2 + 0x20),*(undefined8 *)(param_2 + 0x28),local_e0);
          if ((auVar22._0_8_ & 1) != 0) {
            cleanup_unlocked::__closure__(&local_f0);
            goto LAB_1008a9974;
          }
          atomic_write::Publication::acknowledge(auVar22._8_8_);
          uVar16 = 1;
          if (ppppuVar14 != (undefined8 ****)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar20,ppppuVar14,1);
          }
          goto LAB_1008a972c;
        }
        remove_file_if_exists
                  (&local_f0,*(undefined8 *)(param_2 + 0x20),*(undefined8 *)(param_2 + 0x28),
                   "router unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
                   ,0x17);
        if (local_f0 == (undefined8 ****)0xb) goto LAB_1008a9728;
      }
      else {
        if ((uStack_110 & 1) != 0) {
          std::sys::fs::metadata(&local_f0,local_1e0,uStack_1d8);
          ppppuVar14 = local_f0;
          ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_f0,ppppuStack_e8);
          if (((ulong)ppppuVar14 & 1) == 0) {
            read_cleanup_backup(&local_f0,uVar15,uVar17,
                                "router unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
                                ,0x1a);
            ppppuVar20 = local_e0;
            ppppuVar14 = ppppuStack_e8;
            if (local_f0 == (undefined8 ****)0xb) {
              if (ppppuStack_e8 == (undefined8 ****)0x8000000000000000) goto LAB_1008a9684;
              auVar22 = atomic_write::write_atomic_private_report(*(undefined8 *)(param_2 + 0x20),*(undefined8 *)(param_2 + 0x28),local_e0);
              if ((auVar22._0_8_ & 1) == 0) {
                atomic_write::Publication::acknowledge(auVar22._8_8_);
                uVar16 = 1;
                if (ppppuVar14 != (undefined8 ****)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar20,ppppuVar14,1);
                }
                goto LAB_1008a972c;
              }
              cleanup_unlocked::__closure__(&local_f0);
LAB_1008a9974:
              param_1[5] = ppppuStack_c8;
              param_1[4] = local_d0;
              param_1[7] = local_b8;
              param_1[6] = uStack_c0;
              param_1[9] = uStack_a8;
              param_1[8] = local_b0;
              param_1[0xb] = pppppuStack_98;
              param_1[10] = local_a0;
              param_1[1] = ppppuStack_e8;
              *param_1 = local_f0;
              param_1[3] = uStack_d8;
              param_1[2] = local_e0;
              if (ppppuVar14 != (undefined8 ****)0x0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar20,ppppuVar14,1);
              }
            }
            else {
LAB_1008a991c:
              param_1[5] = ppppuStack_c8;
              param_1[4] = local_d0;
              param_1[7] = local_b8;
              param_1[6] = uStack_c0;
              param_1[9] = uStack_a8;
              param_1[8] = local_b0;
              param_1[0xb] = pppppuStack_98;
              param_1[10] = local_a0;
              param_1[2] = local_e0;
              param_1[3] = uStack_d8;
              *param_1 = local_f0;
              param_1[1] = ppppuStack_e8;
            }
            goto LAB_1008a9804;
          }
        }
LAB_1008a9684:
        uVar16 = 0;
LAB_1008a972c:
        uVar15 = local_1e0;
        remove_file_if_exists
                  (&local_f0,local_1e0,uStack_1d8,
                   "router unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
                   ,0x19);
        if ((local_f0 == (undefined8 ****)0xb) &&
           (remove_file_if_exists
                      (&local_f0,local_1f8,uStack_1f0,
                       "router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
                       ,0x19), local_f0 == (undefined8 ****)0xb)) {
          *(undefined1 *)(param_1 + 1) = uVar16;
          *param_1 = 0xb;
          if ((ppppuVar2 != (undefined8 ****)0x8000000000000000) && (ppppuVar2 != (undefined8 ****)0x0)) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuStack_158,ppppuVar2,1);
          }
          if ((local_148 != (undefined8 ****)0x8000000000000000) && (local_148 != (undefined8 ****)0x0)) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_140,local_148,1);
          }
          local_1e0 = uVar15;
          if ((ppppuStack_130 != (undefined8 ****)0x8000000000000000) && (ppppuStack_130 != (undefined8 ****)0x0)) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_128,ppppuStack_130,1);
          }
          goto joined_r0x0001008a97e0;
        }
      }
      param_1[5] = ppppuStack_c8;
      param_1[4] = local_d0;
      param_1[7] = local_b8;
      param_1[6] = uStack_c0;
      param_1[9] = uStack_a8;
      param_1[8] = local_b0;
      param_1[0xb] = pppppuStack_98;
      param_1[10] = local_a0;
      param_1[1] = ppppuStack_e8;
      *param_1 = local_f0;
      param_1[3] = uStack_d8;
      param_1[2] = local_e0;
    }
    else {
      if (uVar12 == 3) goto LAB_1008a9684;
      if (1 < log::MAX_LOG_LEVEL_FILTER) {
        local_b0 = 0x2e;
        local_a0 =
        "[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
        ;
        pppppuStack_98 = (undefined8 *****)&DAT_000000c5;
        local_f0 = (undefined8 ****)0x0;
        ppppuStack_e8 = (undefined8 ****)0x1013b4b0a;
        uStack_d8 = 0;
        local_e0 = (undefined8 ****)0x2e;
        local_b8 =
        "codexmate_lib::core::relay::router_unlock_authaimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
        ;
        local_d0 = (undefined8 ****)0x10152f6b5;
        uStack_c0 = 2;
        ppppuStack_c8 = (undefined8 ****)0x24;
        uStack_a8 = 0x10f00000001;
        <log::__private_api::GlobalLogger_as_log::Log>::log(&uStack_51,&local_f0);
      }
      *(undefined1 *)(param_1 + 1) = 0;
      uVar15 = 0xb;
LAB_1008a95e0:
      *param_1 = uVar15;
    }
LAB_1008a9804:
    if ((ppppuVar2 != (undefined8 ****)0x8000000000000000) && (ppppuVar2 != (undefined8 ****)0x0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuStack_158,ppppuVar2,1);
    }
    if ((local_148 != (undefined8 ****)0x8000000000000000) && (local_148 != (undefined8 ****)0x0)) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_140,local_148,1);
    }
    ppppuVar14 = ppppuStack_130;
    ppppuVar20 = local_128;
    if (ppppuStack_130 == (undefined8 ****)0x8000000000000000) goto joined_r0x0001008a97e0;
joined_r0x0001008a9850:
    if (ppppuVar14 != (undefined8 ****)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar20,ppppuVar14,1);
    }
    goto joined_r0x0001008a97e0;
  }
  cVar10 = std::io::error::Error::kind(ppppuStack_158);
  if (cVar10 == '\0') {
    if (((ulong)ppppuVar14 & 3) == 1) {
      pcVar18 = (char *)((long)ppppuVar14 + -1);
      uVar15 = *(undefined8 *)pcVar18;
      puVar21 = *(undefined8 **)((long)ppppuVar14 + 7);
      if ((code *)*puVar21 != (code *)0x0) {
        (*(code *)*puVar21)(uVar15);
      }
      goto LAB_1008a9270;
    }
  }
  else {
    local_108 = ppppuVar14;
    if (1 < log::MAX_LOG_LEVEL_FILTER) {
      local_100 = (ulong ****)&local_108;
      pcStack_f8 = <std::io::error::Error_as_core::fmt::Display>::fmt;
      local_b8 =
      "codexmate_lib::core::relay::router_unlock_authaimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
      ;
      local_b0 = 0x2e;
      pppppuStack_98 = &local_100;
      local_a0 = s_1_AiMaMi__router_unlock_auth__ma_10152f6da;
      local_f0 = (undefined8 ****)0x0;
      ppppuStack_e8 = (undefined8 ****)0x1013b4b0a;
      uStack_d8 = 0;
      local_e0 = (undefined8 ****)0x2e;
      local_d0 = (undefined8 ****)0x10152f6b5;
      uStack_c0 = 2;
      ppppuStack_c8 = (undefined8 ****)0x24;
      uStack_a8 = 0x26800000001;
      <log::__private_api::GlobalLogger_as_log::Log>::log(&uStack_51,&local_f0);
    }
    if (((ulong)local_108 & 3) == 1) {
      pcVar18 = (char *)((long)local_108 + -1);
      uVar15 = *(undefined8 *)pcVar18;
      puVar21 = *(undefined8 **)((long)local_108 + 7);
      if ((code *)*puVar21 != (code *)0x0) {
        (*(code *)*puVar21)(uVar15);
      }
LAB_1008a9270:
      if (puVar21[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar15,puVar21[1],puVar21[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar18,0x18,8);
    }
  }
  if (local_178 != (undefined8 **)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_170,local_178,1);
  }
LAB_1008a92a8:
  bVar11 = live_auth_state(param_2);
  uVar15 = local_1e0;
  if (bVar11 < 2) {
    if (bVar11 == 0) {
      std::sys::fs::metadata(&local_f0,local_1e0,uStack_1d8);
      ppppuVar14 = local_f0;
      ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_f0,ppppuStack_e8);
      if (((ulong)ppppuVar14 & 1) == 0) {
        read_cleanup_backup(&local_f0,uVar15,uStack_1d8,
                            "orphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
                            ,0x15);
        ppppuVar20 = local_e0;
        ppppuVar14 = ppppuStack_e8;
        if (local_f0 != (undefined8 ****)0xb) goto LAB_1008a963c;
        if (ppppuStack_e8 != (undefined8 ****)0x8000000000000000) {
          auVar22 = atomic_write::write_atomic_private_report(*(undefined8 *)(param_2 + 0x20),*(undefined8 *)(param_2 + 0x28),local_e0);
          if ((auVar22._0_8_ & 1) == 0) {
            atomic_write::Publication::acknowledge(auVar22._8_8_);
            uVar16 = 1;
            if (ppppuVar14 != (undefined8 ****)0x0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar20,ppppuVar14,1);
            }
            goto LAB_1008a94e4;
          }
          cleanup_unlocked::__closure__(&local_f0);
LAB_1008a96a4:
          param_1[5] = ppppuStack_c8;
          param_1[4] = local_d0;
          param_1[7] = local_b8;
          param_1[6] = uStack_c0;
          param_1[9] = uStack_a8;
          param_1[8] = local_b0;
          param_1[0xb] = pppppuStack_98;
          param_1[10] = local_a0;
          param_1[1] = ppppuStack_e8;
          *param_1 = local_f0;
          param_1[3] = uStack_d8;
          param_1[2] = local_e0;
          goto joined_r0x0001008a9850;
        }
      }
      uVar16 = 0;
    }
    else {
      std::sys::fs::metadata(&local_f0,local_1e0,uStack_1d8);
      ppppuVar14 = local_f0;
      ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_f0,ppppuStack_e8);
      if (ppppuVar14 == (undefined8 ****)0x0) goto LAB_1008a9484;
LAB_1008a93a8:
      ppppuVar14 = (undefined8 ****)std::sys::fs::remove_file(*(undefined8 *)(param_2 + 0x20),*(undefined8 *)(param_2 + 0x28));
      if (ppppuVar14 != (undefined8 ****)0x0) {
        local_160 = (ulong ****)&local_1d0;
        ppppuStack_158 = (undefined8 ****)<std::io::error::Error_as_core::fmt::Display>::fmt;
        local_1d0 = ppppuVar14;
        alloc::fmt::format::format_inner(&local_f0,s_0remove_orphaned_router_unlock_a_10152f738,&local_160);
        ppppuStack_158 = ppppuStack_e8;
        local_160 = local_f0;
        local_150 = local_e0;
        if (((ulong)local_1d0 & 3) == 1) {
          pcVar18 = (char *)((long)local_1d0 + -1);
          uVar15 = *(undefined8 *)pcVar18;
          puVar21 = *(undefined8 **)((long)local_1d0 + 7);
          if ((code *)*puVar21 != (code *)0x0) {
            (*(code *)*puVar21)(uVar15);
          }
          if (puVar21[1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar15,puVar21[1],puVar21[2]);
          }
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcVar18,0x18,8);
        }
        param_1[2] = ppppuStack_158;
        param_1[1] = local_160;
        param_1[3] = local_150;
        *param_1 = 10;
        goto joined_r0x0001008a97e0;
      }
LAB_1008a94e0:
      uVar16 = 1;
    }
LAB_1008a94e4:
    remove_file_if_exists
              (&local_f0,uVar15,uStack_1d8,
               "orphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
               ,0x22);
    if ((local_f0 == (undefined8 ****)0xb) &&
       (remove_file_if_exists
                  (&local_f0,local_1f8,uStack_1f0,
                   "orphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup",0x22),
       local_f0 == (undefined8 ****)0xb)) {
      *(undefined1 *)(param_1 + 1) = uVar16;
      *param_1 = 0xb;
      goto joined_r0x0001008a97e0;
    }
  }
  else {
    if (bVar11 != 2) {
      if (bVar11 == 4) {
        if (1 < log::MAX_LOG_LEVEL_FILTER) {
          local_b0 = 0x2e;
          local_a0 = "[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup";
          pppppuStack_98 = (undefined8 *****)&DAT_000000af;
          local_f0 = (undefined8 ****)0x0;
          ppppuStack_e8 = (undefined8 ****)0x1013b4b0a;
          uStack_d8 = 0;
          local_e0 = (undefined8 ****)0x2e;
          local_b8 =
          "codexmate_lib::core::relay::router_unlock_authaimami_router_unlock_authrouter unlock missing-authrouter unlockrouter unlock auth.jsonrouter unlock auth backuprouter unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving marker + backup for later cleanuporphaned missing-authorphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
          ;
          local_d0 = (undefined8 ****)0x10152f6b5;
          uStack_c0 = 2;
          ppppuStack_c8 = (undefined8 ****)0x24;
          uStack_a8 = 0xda00000001;
          <log::__private_api::GlobalLogger_as_log::Log>::log(&uStack_51,&local_f0);
        }
        *(undefined1 *)(param_1 + 1) = 0;
        *param_1 = 0xb;
        goto joined_r0x0001008a97e0;
      }
      uVar16 = 0;
      goto LAB_1008a94e4;
    }
    std::sys::fs::metadata(&local_f0,local_1e0,uStack_1d8);
    ppppuVar14 = local_f0;
    ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_f0,ppppuStack_e8);
    if (((ulong)ppppuVar14 & 1) != 0) goto LAB_1008a93a8;
LAB_1008a9484:
    read_cleanup_backup(&local_f0,uVar15,uStack_1d8,&DAT_1013b3030,8);
    ppppuVar20 = local_e0;
    ppppuVar14 = ppppuStack_e8;
    if (local_f0 != (undefined8 ****)0xb) {
LAB_1008a963c:
      param_1[5] = ppppuStack_c8;
      param_1[4] = local_d0;
      param_1[7] = local_b8;
      param_1[6] = uStack_c0;
      param_1[9] = uStack_a8;
      param_1[8] = local_b0;
      param_1[0xb] = pppppuStack_98;
      param_1[10] = local_a0;
      param_1[2] = local_e0;
      param_1[3] = uStack_d8;
      *param_1 = local_f0;
      param_1[1] = ppppuStack_e8;
      goto joined_r0x0001008a97e0;
    }
    if (ppppuStack_e8 != (undefined8 ****)0x8000000000000000) {
      auVar22 = atomic_write::write_atomic_private_report(*(undefined8 *)(param_2 + 0x20),*(undefined8 *)(param_2 + 0x28),local_e0);
      if ((auVar22._0_8_ & 1) != 0) {
        cleanup_unlocked::__closure__(&local_f0);
        goto LAB_1008a96a4;
      }
      atomic_write::Publication::acknowledge(auVar22._8_8_);
      uVar16 = 1;
      if (ppppuVar14 != (undefined8 ****)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppuVar20,ppppuVar14,1);
      }
      goto LAB_1008a94e4;
    }
    remove_file_if_exists
              (&local_f0,*(undefined8 *)(param_2 + 0x20),*(undefined8 *)(param_2 + 0x28),
               "orphaned router unlock auth.jsonorphaned router unlock auth backuporphaned router unlock auth marker[AiMaMi][router-unlock-auth] auth.json is unreadable; preserving orphan marker + backup"
               ,0x20);
    if (local_f0 == (undefined8 ****)0xb) goto LAB_1008a94e0;
  }
  param_1[5] = ppppuStack_c8;
  param_1[4] = local_d0;
  param_1[7] = local_b8;
  param_1[6] = uStack_c0;
  param_1[9] = uStack_a8;
  param_1[8] = local_b0;
  param_1[0xb] = pppppuStack_98;
  param_1[10] = local_a0;
  param_1[1] = ppppuStack_e8;
  *param_1 = local_f0;
  param_1[3] = uStack_d8;
  param_1[2] = local_e0;
joined_r0x0001008a97e0:
  if (local_1e8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1e0,local_1e8,1);
  }
  if (local_200 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_1f8,local_200,1);
  }
  return;
}
