/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* quarantine_damaged_backups @ 1001fb728 requested 1001fb728 */

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::config_takeover::quarantine_damaged_backups */

void codexmate_lib::core::relay::config_takeover::quarantine_damaged_backups(undefined8 *param_1,long param_2)

{
  ulong uVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  char *******pppppppcVar4;
  char *******pppppppcVar5;
  undefined8 uVar6;
  char *******pppppppcVar7;
  char *******pppppppcVar8;
  char *******pppppppcVar9;
  char *******pppppppcVar10;
  char *******pppppppcVar11;
  char *******pppppppcVar12;
  char *******pppppppcVar13;
  char *******pppppppcVar14;
  char *******pppppppcVar15;
  bool bVar16;
  int iVar17;
  undefined8 *puVar18;
  long lVar19;
  char ******ppppppcVar20;
  char ******ppppppcVar21;
  char *******pppppppcVar22;
  undefined8 uVar23;
  char *******pppppppcVar24;
  undefined8 uVar25;
  uint uVar26;
  char *******pppppppcVar27;
  char *******pppppppcVar28;
  char *******pppppppcVar29;
  undefined1 auVar30 [16];
  undefined1 auVar31 [12];
  long local_288;
  long lStack_280;
  long local_278;
  char *******local_270;
  char *******pppppppcStack_268;
  char *******local_260;
  char *******pppppppcStack_258;
  char *******local_250;
  char *******pppppppcStack_248;
  char *******pppppppcStack_240;
  char *******pppppppcStack_238;
  long local_228;
  char ******local_220;
  undefined8 uStack_218;
  char *******local_210;
  char *******pppppppcStack_208;
  char *******local_200;
  char *******pppppppcStack_1f8;
  char *******pppppppcStack_1f0;
  char *******local_1e8;
  char *******pppppppcStack_1e0;
  char *******local_1d8;
  char *******pppppppcStack_1d0;
  char *******local_1c8;
  char *******pppppppcStack_1c0;
  char *******local_1b0;
  char *******pppppppcStack_1a8;
  char *******local_1a0;
  char *******pppppppcStack_198;
  char *******local_190;
  char *******pppppppcStack_188;
  char *******pppppppcStack_180;
  char *******pppppppcStack_178;
  char ******local_170;
  undefined8 uStack_168;
  char ******local_160;
  long lStack_158;
  char *******local_148;
  char *******pppppppcStack_140;
  char *******local_138;
  char *******local_130;
  char *******pppppppcStack_128;
  char *******local_120;
  char ******local_118;
  undefined8 uStack_110;
  char ******local_108;
  char *******pppppppcStack_100;
  char *******local_f8;
  char *******pppppppcStack_f0;
  char *******local_e8;
  char *******pppppppcStack_e0;
  char *******pppppppcStack_d8;
  char *******pppppppcStack_d0;
  char *******pppppppcStack_c8;
  char *******local_c0;
  char *******pppppppcStack_b8;
  char *******local_b0;

  validate_recovery_backups(&local_108,param_2);
  if (local_108 != (char ******)&DAT_0000000b) {
    param_1[5] = pppppppcStack_e0;
    param_1[4] = local_e8;
    param_1[7] = pppppppcStack_d0;
    param_1[6] = pppppppcStack_d8;
    param_1[9] = local_c0;
    param_1[8] = pppppppcStack_c8;
    param_1[0xb] = local_b0;
    param_1[10] = pppppppcStack_b8;
    param_1[1] = pppppppcStack_100;
    *param_1 = local_108;
    param_1[3] = pppppppcStack_f0;
    param_1[2] = local_f8;
    return;
  }
  local_288 = 0;
  lStack_280 = 8;
  local_278 = 0;
  uVar23 = *(undefined8 *)(param_2 + 0x248);
  uVar25 = *(undefined8 *)(param_2 + 0x250);
  std::path::Path::_join(&local_228,uVar23,uVar25,"router-takeover-backup.json",0x1b);
  uVar6 = uStack_218;
  ppppppcVar20 = local_220;
  local_170 = (char ******)0x101306110;
  uStack_168 = 0x1b;
  read_backup_bytes(&local_108,local_220,uStack_218,"router-takeover-backup.json",0x1b);
  pppppppcVar28 = pppppppcStack_f0;
  pppppppcVar7 = local_f8;
  pppppppcVar8 = pppppppcStack_100;
  if (local_108 == (char ******)&DAT_0000000b) {
    if (pppppppcStack_100 != (char *******)&DAT_8000000000000000) {
      decode_backup(&local_108,local_f8,pppppppcStack_f0);
      pppppppcVar24 = local_f8;
      pppppppcStack_1a8 = pppppppcStack_e0;
      local_1b0 = local_e8;
      pppppppcStack_198 = pppppppcStack_d0;
      local_1a0 = pppppppcStack_d8;
      pppppppcStack_188 = local_c0;
      local_190 = pppppppcStack_c8;
      pppppppcStack_178 = local_b0;
      pppppppcStack_180 = pppppppcStack_b8;
      if (local_108 == (char ******)&DAT_0000000b) {
        local_210 = pppppppcStack_100;
        pppppppcStack_208 = local_f8;
        local_200 = pppppppcStack_f0;
        pppppppcStack_1f0 = pppppppcStack_e0;
        pppppppcStack_1f8 = local_e8;
        pppppppcStack_1e0 = pppppppcStack_d0;
        local_1e8 = pppppppcStack_d8;
        pppppppcStack_1d0 = local_c0;
        local_1d8 = pppppppcStack_c8;
        pppppppcStack_1c0 = local_b0;
        local_1c8 = pppppppcStack_b8;
        if (pppppppcStack_100 == (char *******)&DAT_8000000000000000) {
          auVar30 = std::path::Path::parent(ppppppcVar20,uVar6);
          if (auVar30._0_8_ == 0) {
            local_210 = &local_170;
            pppppppcStack_208 = (char *******)&<&T_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&pppppppcStack_100,s_has_no_parent_directory_10151d3d1,&local_210);
          }
          else {
            auVar31 = std::time::SystemTime::now();
            local_210 = auVar31._0_8_;
            pppppppcStack_208 = (char *******)CONCAT44(pppppppcStack_208._4_4_,auVar31._8_4_);
            std::time::SystemTime::duration_since(&local_108,&local_210,0,0);
            uVar26 = 0;
            auVar2._8_8_ = 0;
            auVar2._0_8_ = pppppppcStack_100;
            lVar19 = SUB168(auVar2 * ZEXT816(1000),8);
            if (CARRY8((long)pppppppcStack_100 * 1000,(ulong)(uint)local_f8 / 1000000)) {
              lVar19 = lVar19 + 1;
            }
            lStack_158 = 0;
            if ((int)local_108 == 0) {
              lStack_158 = lVar19;
            }
            local_160 = (char ******)0x0;
            if ((int)local_108 == 0) {
              local_160 = (char ******)((long)pppppppcStack_100 * 1000 + (ulong)(uint)local_f8 / 1000000);
            }
            do {
              local_118 = (char ******)CONCAT71(local_118._1_7_,(char)uVar26);
              local_108 = (char ******)&local_170;
              local_f8 = &local_160;
              pppppppcStack_100 = (char *******)&<&T_as_core::fmt::Display>::fmt;
              local_e8 = &local_118;
              pppppppcStack_f0 = (char *******)u128>::fmt;
              pppppppcStack_e0 = (char *******)u8>::fmt;
              alloc::fmt::format::format_inner(&local_210,s__damaged_____bak_10151d2ef,&local_108);
              pppppppcVar22 = pppppppcStack_208;
              std::path::Path::_join(&local_1b0,auVar30._0_8_,auVar30._8_8_,pppppppcStack_208,local_200);
              if (local_210 != (char *******)0x0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar22,local_210,1);
              }
              pppppppcVar22 = local_1a0;
              pppppppcVar27 = pppppppcStack_1a8;
              pppppppcVar29 = local_1b0;
              std::sys::fs::metadata(&local_108,pppppppcStack_1a8,local_1a0);
              ppppppcVar21 = local_108;
              ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_108,pppppppcStack_100);
              if (((ulong)ppppppcVar21 & 1) == 0) {
                if (pppppppcVar29 != (char *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar27,pppppppcVar29,1);
                }
              }
              else if (pppppppcVar29 != (char *******)&DAT_8000000000000000) {
                auVar30 = atomic_write::write_atomic_private_report(pppppppcVar27,pppppppcVar22,pppppppcVar7,pppppppcVar28);
                if (((auVar30._0_8_ & 1) == 0) && (lVar19 = atomic_write::Publication::require_sync(auVar30._8_8_), lVar19 == 0)) {
                  std::fs::read::inner(&local_210,pppppppcVar27,pppppppcVar22,0);
                  pppppppcVar10 = pppppppcStack_208;
                  pppppppcVar9 = local_210;
                  if (local_210 == (char *******)&DAT_8000000000000000) {
                    quarantine_damaged_backup::__closure__(&local_108,&local_170,pppppppcStack_208);
                    goto LAB_1001fbf38;
                  }
                  if ((local_200 == pppppppcVar28) && (iVar17 = _memcmp(pppppppcStack_208,pppppppcVar7,(size_t)pppppppcVar28), iVar17 == 0)) {
                    std::fs::read::inner(&local_210,ppppppcVar20,uVar6);
                    pppppppcVar12 = pppppppcStack_208;
                    pppppppcVar11 = local_210;
                    if (local_210 == (char *******)&DAT_8000000000000000) {
                      quarantine_damaged_backup::__closure__(&local_108,&local_170,pppppppcStack_208);
                      pppppppcStack_268 = pppppppcStack_e0;
                      local_270 = local_e8;
                      pppppppcStack_258 = pppppppcStack_d0;
                      local_260 = pppppppcStack_d8;
                      pppppppcStack_248 = local_c0;
                      local_250 = pppppppcStack_c8;
                      pppppppcStack_238 = local_b0;
                      pppppppcStack_240 = pppppppcStack_b8;
                      ppppppcVar21 = local_108;
                      pppppppcVar24 = local_f8;
                      pppppppcVar28 = pppppppcStack_f0;
                      pppppppcVar22 = pppppppcStack_100;
                    }
                    else {
                      if ((local_200 == pppppppcVar28) && (iVar17 = _memcmp(pppppppcStack_208,pppppppcVar7,(size_t)pppppppcVar28), iVar17 == 0)) {
                        lVar19 = std::sys::fs::remove_file(ppppppcVar20,uVar6);
                        if (lVar19 == 0) {
                          if (1 < log::MAX_LOG_LEVEL_FILTER) {
                            local_118 = ppppppcVar20;
                            uStack_110 = uVar6;
                            local_1b0 = pppppppcVar27;
                            pppppppcStack_1a8 = pppppppcVar22;
                            local_210 = &local_118;
                            pppppppcStack_208 = (char *******)<std::path::Display_as_core::fmt::Display>::fmt;
                            local_200 = (char *******)&local_1b0;
                            pppppppcStack_1f8 = (char *******)<std::path::Display_as_core::fmt::Display>::fmt;
                            local_108 = (char ******)0x1013061a2;
                            pppppppcStack_100 = (char *******)&DAT_0000002b;
                            local_f8 = (char *******)0x1013061a2;
                            pppppppcStack_f0 = (char *******)&DAT_0000002b;
                            local_e8 = (char *******)&PTR_s_src_core_relay_config_takeover_r_1018172a0;
                            log::__private_api::log(s_6_AiMaMi__takeover__damaged_back_10151d247,&local_210,2,&local_108);
                          }
                          alloc::string::String::from_utf8_lossy(&local_210,pppppppcVar27,pppppppcVar22);
                          pppppppcVar28 = local_200;
                          pppppppcVar13 = pppppppcStack_208;
                          alloc::raw_vec::RawVecInner<A>::try_allocate_in(&local_108,local_200,0,1,1);
                          pppppppcVar24 = local_f8;
                          pppppppcVar22 = pppppppcStack_100;
                          if (((ulong)local_108 & 1) != 0) {
                    /* WARNING: Subroutine does not return */
                            alloc::raw_vec::handle_error(pppppppcStack_100,local_f8);
                          }
                          _memcpy(local_f8,pppppppcVar13,(size_t)pppppppcVar28);
                          if (pppppppcVar11 != (char *******)0x0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar12,pppppppcVar11,1);
                          }
                          if (pppppppcVar9 != (char *******)0x0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar10,pppppppcVar9,1);
                          }
                          if (pppppppcVar29 != (char *******)0x0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar27,pppppppcVar29,1);
                          }
                          if (pppppppcVar8 != (char *******)0x0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar7,pppppppcVar8,1);
                          }
                          if (((ulong)local_210 & 0x7fffffffffffffff) != 0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar13,local_210,1);
                          }
                          if (pppppppcVar22 == (char *******)&DAT_8000000000000000) goto LAB_1001fbb24;
                          goto LAB_1001fbaf0;
                        }
                        quarantine_damaged_backup::__closure__(&local_108,&local_170,lVar19);
                        ppppppcVar21 = local_108;
                        local_270 = local_e8;
                        pppppppcStack_268 = pppppppcStack_e0;
                        local_260 = pppppppcStack_d8;
                        pppppppcStack_258 = pppppppcStack_d0;
                        local_250 = pppppppcStack_c8;
                        pppppppcStack_248 = local_c0;
                        pppppppcStack_240 = pppppppcStack_b8;
                        pppppppcStack_238 = local_b0;
                        pppppppcVar22 = pppppppcStack_100;
                        pppppppcVar24 = local_f8;
                        pppppppcVar28 = pppppppcStack_f0;
                      }
                      else {
                        local_108 = (char ******)&local_170;
                        pppppppcStack_100 = (char *******)&<&T_as_core::fmt::Display>::fmt;
                        alloc::fmt::format::format_inner(&local_130,s_1_changed_during_recovery__origi_10151d28e,&local_108);
                        ppppppcVar21 = (char ******)&DAT_0000000a;
                        pppppppcVar22 = local_130;
                        pppppppcVar24 = pppppppcStack_128;
                        pppppppcVar28 = local_120;
                      }
                      if (pppppppcVar11 != (char *******)0x0) {
                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar12,pppppppcVar11,1);
                      }
                    }
                  }
                  else {
                    local_108 = (char ******)&local_170;
                    pppppppcStack_100 = (char *******)&<&T_as_core::fmt::Display>::fmt;
                    alloc::fmt::format::format_inner(&local_148,s_damaged___quarantine_verificatio_10151d2c2,&local_108);
                    ppppppcVar21 = (char ******)&DAT_0000000a;
                    pppppppcVar24 = pppppppcStack_140;
                    pppppppcVar28 = local_138;
                    pppppppcVar22 = local_148;
                  }
                  if (pppppppcVar9 != (char *******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar10,pppppppcVar9,1);
                  }
                }
                else {
                  quarantine_damaged_backup::__closure__(&local_108,&local_170);
LAB_1001fbf38:
                  pppppppcStack_268 = pppppppcStack_e0;
                  local_270 = local_e8;
                  pppppppcStack_258 = pppppppcStack_d0;
                  local_260 = pppppppcStack_d8;
                  pppppppcStack_248 = local_c0;
                  local_250 = pppppppcStack_c8;
                  pppppppcStack_238 = local_b0;
                  pppppppcStack_240 = pppppppcStack_b8;
                  ppppppcVar21 = local_108;
                  pppppppcVar24 = local_f8;
                  pppppppcVar28 = pppppppcStack_f0;
                  pppppppcVar22 = pppppppcStack_100;
                }
                if (pppppppcVar29 != (char *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar27,pppppppcVar29,1);
                }
                goto LAB_1001fbac8;
              }
              bVar16 = uVar26 < 99;
              uVar26 = uVar26 + 1;
            } while (bVar16);
            local_210 = &local_170;
            pppppppcStack_208 = (char *******)&<&T_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&pppppppcStack_100,s__cannot_allocate_quarantine_path_10151d3a2,&local_210);
          }
          pppppppcStack_268 = pppppppcStack_e0;
          local_270 = local_e8;
          pppppppcStack_258 = pppppppcStack_d0;
          local_260 = pppppppcStack_d8;
          pppppppcStack_248 = local_c0;
          local_250 = pppppppcStack_c8;
          pppppppcStack_238 = local_b0;
          pppppppcStack_240 = pppppppcStack_b8;
          ppppppcVar21 = (char ******)&DAT_0000000a;
          pppppppcVar24 = local_f8;
          pppppppcVar28 = pppppppcStack_f0;
          pppppppcVar22 = pppppppcStack_100;
        }
        else {
          ::core::ptr::drop_in_place<codexmate_lib::core::relay::config_takeover::TakeoverBackup>(&local_210);
          ppppppcVar21 = (char ******)&DAT_0000000b;
          pppppppcVar22 = (char *******)&DAT_8000000000000000;
        }
      }
      else {
        pppppppcStack_268 = pppppppcStack_e0;
        local_270 = local_e8;
        pppppppcStack_258 = pppppppcStack_d0;
        local_260 = pppppppcStack_d8;
        pppppppcStack_248 = local_c0;
        local_250 = pppppppcStack_c8;
        pppppppcStack_238 = local_b0;
        pppppppcStack_240 = pppppppcStack_b8;
        ppppppcVar21 = local_108;
        pppppppcVar28 = pppppppcStack_f0;
        pppppppcVar22 = pppppppcStack_100;
      }
LAB_1001fbac8:
      if (pppppppcVar8 != (char *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar7,pppppppcVar8,1);
      }
      if (ppppppcVar21 != (char ******)&DAT_0000000b) goto LAB_1001fb83c;
      if (pppppppcVar22 != (char *******)&DAT_8000000000000000) {
LAB_1001fbaf0:
        lVar19 = local_278;
        if (local_278 == local_288) {
          alloc::raw_vec::RawVec<T,A>::grow_one(&local_288);
        }
        puVar18 = (undefined8 *)(lStack_280 + lVar19 * 0x18);
        *puVar18 = pppppppcVar22;
        puVar18[1] = pppppppcVar24;
        puVar18[2] = pppppppcVar28;
        local_278 = lVar19 + 1;
      }
    }
LAB_1001fbb24:
    if (local_228 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppcVar20,local_228,1);
    }
    std::path::Path::_join(&local_228,uVar23,uVar25,"codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey",0x22)
    ;
    local_170 = (char ******)0x10130612c;
    uStack_168 = 0x22;
    read_backup_bytes(&local_108,local_220,uStack_218,"codex_router_top_level_backup.jsonversionlinescreatedAtMstopLevelLinessectionBlocksoriginalConfigkey",
                      0x22);
    pppppppcVar7 = pppppppcStack_f0;
    pppppppcVar8 = local_f8;
    pppppppcVar28 = pppppppcStack_100;
    if (local_108 == (char ******)&DAT_0000000b) {
      if (pppppppcStack_100 == (char *******)&DAT_8000000000000000) {
LAB_1001fbe7c:
        if (local_228 != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_220,local_228,1);
        }
        param_1[2] = lStack_280;
        param_1[1] = local_288;
        param_1[3] = local_278;
        *param_1 = 0xb;
        return;
      }
      decode_backup(&local_108,local_f8,pppppppcStack_f0);
      pppppppcVar22 = local_e8;
      pppppppcVar29 = local_f8;
      pppppppcVar24 = pppppppcStack_100;
      ppppppcVar20 = local_108;
      pppppppcVar27 = pppppppcStack_f0;
      pppppppcVar9 = pppppppcStack_e0;
      pppppppcVar10 = pppppppcStack_d8;
      pppppppcVar11 = pppppppcStack_d0;
      pppppppcVar12 = pppppppcStack_c8;
      pppppppcVar13 = local_c0;
      pppppppcVar14 = pppppppcStack_b8;
      pppppppcVar15 = local_b0;
      if (local_108 == (char ******)&DAT_0000000b) {
        local_1b0 = pppppppcStack_100;
        pppppppcStack_1a8 = local_f8;
        local_1a0 = pppppppcStack_f0;
        pppppppcStack_198 = local_e8;
        if (pppppppcStack_100 == (char *******)&DAT_8000000000000000) {
          auVar30 = std::path::Path::parent(local_220,uStack_218);
          if (auVar30._0_8_ == 0) {
            local_1b0 = &local_170;
            pppppppcStack_1a8 = (char *******)&<&T_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&pppppppcStack_100,s_has_no_parent_directory_10151d3d1,&local_1b0);
          }
          else {
            auVar31 = std::time::SystemTime::now();
            local_1b0 = auVar31._0_8_;
            pppppppcStack_1a8 = (char *******)CONCAT44(pppppppcStack_1a8._4_4_,auVar31._8_4_);
            std::time::SystemTime::duration_since(&local_108,&local_1b0,0,0);
            uVar26 = 0;
            auVar3._8_8_ = 0;
            auVar3._0_8_ = pppppppcStack_100;
            lVar19 = SUB168(auVar3 * ZEXT816(1000),8);
            uVar1 = ((ulong)local_f8 & 0xffffffff) / 1000000;
            if (CARRY8((long)pppppppcStack_100 * 1000,uVar1)) {
              lVar19 = lVar19 + 1;
            }
            lStack_158 = 0;
            if ((int)local_108 == 0) {
              lStack_158 = lVar19;
            }
            local_160 = (char ******)0x0;
            if ((int)local_108 == 0) {
              local_160 = (char ******)((long)pppppppcStack_100 * 1000 + uVar1);
            }
            do {
              local_118 = (char ******)CONCAT71(local_118._1_7_,(char)uVar26);
              local_108 = (char ******)&local_170;
              local_f8 = &local_160;
              pppppppcStack_100 = (char *******)&<&T_as_core::fmt::Display>::fmt;
              local_e8 = &local_118;
              pppppppcStack_f0 = (char *******)u128>::fmt;
              pppppppcStack_e0 = (char *******)u8>::fmt;
              alloc::fmt::format::format_inner(&local_1b0,s__damaged_____bak_10151d2ef,&local_108);
              pppppppcVar22 = pppppppcStack_1a8;
              std::path::Path::_join(&local_270,auVar30._0_8_,auVar30._8_8_,pppppppcStack_1a8,local_1a0);
              if (local_1b0 != (char *******)0x0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar22,local_1b0,1);
              }
              pppppppcVar24 = local_260;
              pppppppcVar5 = pppppppcStack_268;
              pppppppcVar4 = local_270;
              std::sys::fs::metadata(&local_108,pppppppcStack_268,local_260);
              ppppppcVar20 = local_108;
              ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(local_108,pppppppcStack_100);
              if (((ulong)ppppppcVar20 & 1) == 0) {
                if (pppppppcVar4 != (char *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar5,pppppppcVar4,1);
                }
              }
              else if (pppppppcVar4 != (char *******)&DAT_8000000000000000) {
                auVar30 = atomic_write::write_atomic_private_report(pppppppcVar5,pppppppcVar24,pppppppcVar8,pppppppcVar7);
                if (((auVar30._0_8_ & 1) == 0) && (lVar19 = atomic_write::Publication::require_sync(auVar30._8_8_), lVar19 == 0)) {
                  std::fs::read::inner(&local_1b0,pppppppcVar5,pppppppcVar24,0);
                  pppppppcVar10 = pppppppcStack_1a8;
                  pppppppcVar9 = local_1b0;
                  if (local_1b0 == (char *******)&DAT_8000000000000000) {
                    quarantine_damaged_backup::__closure__(&local_108,&local_170,pppppppcStack_1a8);
                    goto LAB_1001fc048;
                  }
                  pppppppcVar22 = pppppppcVar10;
                  if ((local_1a0 == pppppppcVar7) && (iVar17 = _memcmp(pppppppcStack_1a8,pppppppcVar8,(size_t)pppppppcVar7), iVar17 == 0)) {
                    std::fs::read::inner(&local_1b0,local_220,uStack_218);
                    pppppppcVar12 = pppppppcStack_1a8;
                    pppppppcVar11 = local_1b0;
                    if (local_1b0 == (char *******)&DAT_8000000000000000) {
                      quarantine_damaged_backup::__closure__(&local_108,&local_170,pppppppcStack_1a8);
                      pppppppcStack_208 = pppppppcStack_d8;
                      local_210 = pppppppcStack_e0;
                      pppppppcStack_1f8 = pppppppcStack_c8;
                      local_200 = pppppppcStack_d0;
                      local_1e8 = pppppppcStack_b8;
                      pppppppcStack_1f0 = local_c0;
                      pppppppcStack_1e0 = local_b0;
                      ppppppcVar20 = local_108;
                      pppppppcVar22 = local_e8;
                      pppppppcVar27 = pppppppcStack_f0;
                      pppppppcVar24 = pppppppcStack_100;
                      pppppppcVar29 = local_f8;
                    }
                    else {
                      if ((local_1a0 == pppppppcVar7) && (iVar17 = _memcmp(pppppppcStack_1a8,pppppppcVar8,(size_t)pppppppcVar7), iVar17 == 0)) {
                        lVar19 = std::sys::fs::remove_file(local_220,uStack_218);
                        if (lVar19 == 0) {
                          if (1 < log::MAX_LOG_LEVEL_FILTER) {
                            local_118 = local_220;
                            uStack_110 = uStack_218;
                            local_270 = pppppppcVar5;
                            pppppppcStack_268 = pppppppcVar24;
                            local_1b0 = &local_118;
                            pppppppcStack_1a8 = (char *******)<std::path::Display_as_core::fmt::Display>::fmt;
                            local_1a0 = (char *******)&local_270;
                            pppppppcStack_198 = (char *******)<std::path::Display_as_core::fmt::Display>::fmt;
                            local_108 = (char ******)0x1013061a2;
                            pppppppcStack_100 = (char *******)&DAT_0000002b;
                            local_f8 = (char *******)0x1013061a2;
                            pppppppcStack_f0 = (char *******)&DAT_0000002b;
                            local_e8 = (char *******)&PTR_s_src_core_relay_config_takeover_r_1018172a0;
                            log::__private_api::log(s_6_AiMaMi__takeover__damaged_back_10151d247,&local_1b0,2,&local_108);
                          }
                          alloc::string::String::from_utf8_lossy(&local_1b0,pppppppcVar5,pppppppcVar24);
                          pppppppcVar27 = local_1a0;
                          pppppppcVar7 = pppppppcStack_1a8;
                          alloc::raw_vec::RawVecInner<A>::try_allocate_in(&local_108,local_1a0,0,1,1);
                          pppppppcVar29 = local_f8;
                          pppppppcVar24 = pppppppcStack_100;
                          if (((ulong)local_108 & 1) != 0) {
                    /* WARNING: Subroutine does not return */
                            alloc::raw_vec::handle_error(pppppppcStack_100,local_f8);
                          }
                          _memcpy(local_f8,pppppppcVar7,(size_t)pppppppcVar27);
                          if (pppppppcVar11 != (char *******)0x0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar12,pppppppcVar11,1);
                          }
                          if (pppppppcVar9 != (char *******)0x0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar10,pppppppcVar9,1);
                          }
                          if (pppppppcVar4 != (char *******)0x0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar5,pppppppcVar4,1);
                          }
                          if (pppppppcVar28 != (char *******)0x0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar8,pppppppcVar28,1);
                          }
                          if (((ulong)local_1b0 & 0x7fffffffffffffff) != 0) {
                            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar7,local_1b0,1);
                          }
                          if (pppppppcVar24 == (char *******)&DAT_8000000000000000) goto LAB_1001fbe7c;
                          goto LAB_1001fbe48;
                        }
                        quarantine_damaged_backup::__closure__(&local_108,&local_170,lVar19);
                        ppppppcVar20 = local_108;
                        local_210 = pppppppcStack_e0;
                        pppppppcStack_208 = pppppppcStack_d8;
                        local_200 = pppppppcStack_d0;
                        pppppppcStack_1f8 = pppppppcStack_c8;
                        pppppppcStack_1f0 = local_c0;
                        local_1e8 = pppppppcStack_b8;
                        pppppppcStack_1e0 = local_b0;
                        pppppppcVar22 = local_e8;
                        pppppppcVar24 = pppppppcStack_100;
                        pppppppcVar29 = local_f8;
                        pppppppcVar27 = pppppppcStack_f0;
                      }
                      else {
                        local_108 = (char ******)&local_170;
                        pppppppcStack_100 = (char *******)&<&T_as_core::fmt::Display>::fmt;
                        alloc::fmt::format::format_inner(&local_130,s_1_changed_during_recovery__origi_10151d28e,&local_108);
                        ppppppcVar20 = (char ******)&DAT_0000000a;
                        pppppppcVar24 = local_130;
                        pppppppcVar29 = pppppppcStack_128;
                        pppppppcVar27 = local_120;
                      }
                      if (pppppppcVar11 != (char *******)0x0) {
                        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar12,pppppppcVar11,1);
                      }
                    }
                  }
                  else {
                    local_108 = (char ******)&local_170;
                    pppppppcStack_100 = (char *******)&<&T_as_core::fmt::Display>::fmt;
                    alloc::fmt::format::format_inner(&local_148,s_damaged___quarantine_verificatio_10151d2c2,&local_108);
                    ppppppcVar20 = (char ******)&DAT_0000000a;
                    pppppppcVar27 = local_138;
                    pppppppcVar24 = local_148;
                    pppppppcVar29 = pppppppcStack_140;
                  }
                  if (pppppppcVar9 != (char *******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar10,pppppppcVar9,1);
                  }
                }
                else {
                  quarantine_damaged_backup::__closure__(&local_108,&local_170);
LAB_1001fc048:
                  pppppppcStack_208 = pppppppcStack_d8;
                  local_210 = pppppppcStack_e0;
                  pppppppcStack_1f8 = pppppppcStack_c8;
                  local_200 = pppppppcStack_d0;
                  local_1e8 = pppppppcStack_b8;
                  pppppppcStack_1f0 = local_c0;
                  pppppppcStack_1e0 = local_b0;
                  ppppppcVar20 = local_108;
                  pppppppcVar22 = local_e8;
                  pppppppcVar27 = pppppppcStack_f0;
                  pppppppcVar24 = pppppppcStack_100;
                  pppppppcVar29 = local_f8;
                }
                pppppppcVar9 = local_210;
                pppppppcVar10 = pppppppcStack_208;
                pppppppcVar11 = local_200;
                pppppppcVar12 = pppppppcStack_1f8;
                pppppppcVar13 = pppppppcStack_1f0;
                pppppppcVar14 = local_1e8;
                pppppppcVar15 = pppppppcStack_1e0;
                if (pppppppcVar4 != (char *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar5,pppppppcVar4,1);
                  pppppppcVar9 = local_210;
                  pppppppcVar10 = pppppppcStack_208;
                  pppppppcVar11 = local_200;
                  pppppppcVar12 = pppppppcStack_1f8;
                  pppppppcVar13 = pppppppcStack_1f0;
                  pppppppcVar14 = local_1e8;
                  pppppppcVar15 = pppppppcStack_1e0;
                }
                goto joined_r0x0001001fc090;
              }
              bVar16 = uVar26 < 99;
              uVar26 = uVar26 + 1;
            } while (bVar16);
            local_1b0 = &local_170;
            pppppppcStack_1a8 = (char *******)&<&T_as_core::fmt::Display>::fmt;
            alloc::fmt::format::format_inner(&pppppppcStack_100,s__cannot_allocate_quarantine_path_10151d3a2,&local_1b0);
          }
          ppppppcVar20 = (char ******)&DAT_0000000a;
          pppppppcVar22 = local_e8;
          pppppppcVar27 = pppppppcStack_f0;
          pppppppcVar24 = pppppppcStack_100;
          pppppppcVar29 = local_f8;
          pppppppcVar9 = pppppppcStack_e0;
          pppppppcVar10 = pppppppcStack_d8;
          pppppppcVar11 = pppppppcStack_d0;
          pppppppcVar12 = pppppppcStack_c8;
          pppppppcVar13 = local_c0;
          pppppppcVar14 = pppppppcStack_b8;
          pppppppcVar15 = local_b0;
        }
        else {
          <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_1b0);
          if (pppppppcVar24 != (char *******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar29,(long)pppppppcVar24 * 0x18,8);
          }
          ppppppcVar20 = (char ******)&DAT_0000000b;
          pppppppcVar27 = pppppppcVar7;
          pppppppcVar24 = (char *******)&DAT_8000000000000000;
          pppppppcVar9 = local_210;
          pppppppcVar10 = pppppppcStack_208;
          pppppppcVar11 = local_200;
          pppppppcVar12 = pppppppcStack_1f8;
          pppppppcVar13 = pppppppcStack_1f0;
          pppppppcVar14 = local_1e8;
          pppppppcVar15 = pppppppcStack_1e0;
        }
      }
joined_r0x0001001fc090:
      pppppppcStack_1e0 = pppppppcVar15;
      local_1e8 = pppppppcVar14;
      pppppppcStack_1f0 = pppppppcVar13;
      pppppppcStack_1f8 = pppppppcVar12;
      local_200 = pppppppcVar11;
      pppppppcStack_208 = pppppppcVar10;
      local_210 = pppppppcVar9;
      if (pppppppcVar28 != (char *******)0x0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppcVar8,pppppppcVar28,1);
      }
      if (ppppppcVar20 == (char ******)&DAT_0000000b) {
        if (pppppppcVar24 != (char *******)&DAT_8000000000000000) {
LAB_1001fbe48:
          lVar19 = local_278;
          if (local_278 == local_288) {
            alloc::raw_vec::RawVec<T,A>::grow_one(&local_288);
          }
          puVar18 = (undefined8 *)(lStack_280 + lVar19 * 0x18);
          *puVar18 = pppppppcVar24;
          puVar18[1] = pppppppcVar29;
          puVar18[2] = pppppppcVar27;
          local_278 = lVar19 + 1;
        }
        goto LAB_1001fbe7c;
      }
    }
    else {
      pppppppcStack_208 = pppppppcStack_d8;
      local_210 = pppppppcStack_e0;
      pppppppcStack_1f8 = pppppppcStack_c8;
      local_200 = pppppppcStack_d0;
      local_1e8 = pppppppcStack_b8;
      pppppppcStack_1f0 = local_c0;
      pppppppcStack_1e0 = local_b0;
      ppppppcVar20 = local_108;
      pppppppcVar22 = local_e8;
      pppppppcVar27 = pppppppcStack_f0;
      pppppppcVar24 = pppppppcStack_100;
      pppppppcVar29 = local_f8;
    }
    param_1[6] = pppppppcStack_208;
    param_1[5] = local_210;
    param_1[8] = pppppppcStack_1f8;
    param_1[7] = local_200;
    param_1[10] = local_1e8;
    param_1[9] = pppppppcStack_1f0;
    param_1[0xb] = pppppppcStack_1e0;
    *param_1 = ppppppcVar20;
    param_1[1] = pppppppcVar24;
    param_1[2] = pppppppcVar29;
    param_1[3] = pppppppcVar27;
    param_1[4] = pppppppcVar22;
  }
  else {
    pppppppcStack_268 = pppppppcStack_e0;
    local_270 = local_e8;
    pppppppcStack_258 = pppppppcStack_d0;
    local_260 = pppppppcStack_d8;
    pppppppcStack_248 = local_c0;
    local_250 = pppppppcStack_c8;
    pppppppcStack_238 = local_b0;
    pppppppcStack_240 = pppppppcStack_b8;
    ppppppcVar21 = local_108;
    pppppppcVar24 = local_f8;
    pppppppcVar22 = pppppppcStack_100;
LAB_1001fb83c:
    param_1[5] = pppppppcStack_268;
    param_1[4] = local_270;
    param_1[7] = pppppppcStack_258;
    param_1[6] = local_260;
    param_1[9] = pppppppcStack_248;
    param_1[8] = local_250;
    param_1[0xb] = pppppppcStack_238;
    param_1[10] = pppppppcStack_240;
    *param_1 = ppppppcVar21;
    param_1[1] = pppppppcVar22;
    param_1[2] = pppppppcVar24;
    param_1[3] = pppppppcVar28;
    local_220 = ppppppcVar20;
  }
  if (local_228 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_220,local_228,1);
  }
  if (local_278 != 0) {
    puVar18 = (undefined8 *)(lStack_280 + 8);
    lVar19 = local_278;
    do {
      if (puVar18[-1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar18,puVar18[-1],1);
      }
      puVar18 = puVar18 + 3;
      lVar19 = lVar19 + -1;
    } while (lVar19 != 0);
  }
  if (local_288 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_280,local_288 * 0x18,8);
  }
  return;
}
