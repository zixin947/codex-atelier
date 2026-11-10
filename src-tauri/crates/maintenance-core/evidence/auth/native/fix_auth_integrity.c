===BEGIN fix_auth_integrity @ 100373af8=== (GhidraScript)

/* WARNING: Type propagation algorithm not settling */
/* codexmate_lib::core::relay::codex_diagnostic::fix_auth_integrity */

void codexmate_lib::core::relay::codex_diagnostic::fix_auth_integrity(long *param_1,long param_2,long param_3)

{
  undefined8 uVar1;
  bool bVar2;
  long lVar3;
  undefined8 *******pppppppuVar4;
  undefined8 *puVar5;
  char *pcVar6;
  ulong uVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  long lVar10;
  short *psVar11;
  long lVar12;
  undefined8 uVar13;
  undefined8 *******local_240;
  byte local_238;
  long local_230;
  long local_228;
  ulong uStack_220;
  long local_218;
  undefined8 *******local_210;
  long lStack_208;
  char local_200 [8];
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  long local_1e0;
  long lStack_1d8;
  long local_1d0;
  undefined8 uStack_1c8;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 *******local_1a0;
  code *pcStack_198;
  long local_190;
  long lStack_188;
  undefined8 *puStack_180;
  long local_178;
  undefined8 ******local_168;
  undefined8 *puStack_160;
  long local_158;
  long local_150;
  undefined8 *puStack_148;
  long local_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 *******local_120;
  undefined8 *puStack_118;
  undefined8 *******local_110;
  long lStack_108;
  long local_100;
  long lStack_f8;
  undefined8 *******pppppppuStack_f0;
  long lStack_e8;
  undefined8 ******local_e0;
  long lStack_d8;
  long local_d0;
  undefined8 ******ppppppuStack_c8;
  long local_c0;
  long lStack_b8;

  if ((*(byte *)(param_3 + 0x16c) & 1) != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x50,1);
    if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x50);
    }
    puVar5[5] = 0xe88cbcef81afe8a4;
    puVar5[4] = 0xaee82049416e6570;
    puVar5[7] = 0xb8e4205450477461;
    puVar5[6] = 0x684320a89ce5b7af;
    puVar5[9] = 0x8280e395bde5bb99;
    puVar5[8] = 0xe78690e7a1aee7ad;
    puVar5[1] = 0xe68db8e48fbce5a1;
    *puVar5 = 0xa8e6b194e7afb7e8;
    puVar5[3] = 0x4f20969be786a6e8;
    puVar5[2] = 0x9688e68da4e5a281;
    param_1[1] = 0x50;
    *param_1 = 10;
    param_1[2] = (long)puVar5;
    param_1[3] = 0x50;
    return;
  }
  uVar13 = *(undefined8 *)(param_2 + 0x110);
  uVar1 = *(undefined8 *)(param_2 + 0x118);
  std::sys::fs::metadata(&local_110,uVar13,uVar1);
  if ((int)local_110 == 1) {
    ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(1,lStack_108);
    goto switchD_10035a7e8_caseD_23;
  }
  ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(0,lStack_108);
  std::sys::fs::read_dir(&local_110,uVar13,uVar1);
  local_240 = local_110;
  local_238 = (byte)lStack_108;
  if ((byte)lStack_108 != 2) {
    <alloc::vec::Vec<T>as_alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter(&local_230,local_110,(byte)lStack_108 & 1);
    local_110 = &local_1a0;
    if (uStack_220 < 2) {
      if (uStack_220 != 0) goto LAB_100373bfc;
    }
    else {
      if (uStack_220 < 0x15) {
        ::core::slice::sort::shared::smallsort::insertion_sort_shift_left(local_228,uStack_220,1,&local_110);
      }
      else {
        ::core::slice::sort::stable::driftsort_main(local_228,uStack_220,&local_110);
      }
LAB_100373bfc:
      lVar10 = uStack_220 * 0x18;
      puVar5 = (undefined8 *)(local_228 + 0x10);
      do {
        std::fs::read_to_string::inner(&local_218,puVar5[-1],*puVar5);
        pppppppuVar4 = local_210;
        lVar3 = local_218;
        if (local_218 == -0x8000000000000000) {
          ::core::ptr::drop_in_place<core::result::Result<alloc::string::String,std::io::error::Error>>(&local_218);
        }
        else {
          local_110 = local_210;
          lStack_108 = lStack_208;
          local_100 = 0;
          lStack_f8 = 0;
          pppppppuStack_f0 = local_210;
          lStack_e8 = lStack_208;
          serde_json::de::from_trait(local_200,&local_110);
          if (local_200[0] == '\x06') {
            ::core::ptr::drop_in_place<serde_json::error::Error>(&uStack_1f8);
          }
          else {
            lStack_1d8 = uStack_1f8;
            uStack_1c8 = uStack_1e8;
            local_1d0 = uStack_1f0;
            pcVar6 = (char *)serde_json::value::Value::pointer(&local_1e0,"/tokens/access_tokenOPENAI_API_KEY",0x14);
            if (pcVar6 == (char *)0x0) {
              lVar12 = 0;
              uVar13 = 1;
            }
            else {
              uVar13 = *(undefined8 *)(pcVar6 + 0x10);
              lVar12 = *(long *)(pcVar6 + 0x18);
              if (*pcVar6 != '\x03') {
                lVar12 = 0;
                uVar13 = 1;
              }
            }
            pcVar6 = (char *)<str_as_serde_json::value::index::Index>::index_into("OPENAI_API_KEY",0xe,&local_1e0);
            if (pcVar6 == (char *)0x0) {
              bVar2 = true;
              psVar11 = (short *)0x1;
            }
            else {
              psVar11 = *(short **)(pcVar6 + 0x10);
              if (*pcVar6 != '\x03') {
                psVar11 = (short *)0x1;
              }
              bVar2 = *(ulong *)(pcVar6 + 0x18) < 3 || *pcVar6 != '\x03';
            }
            if (((lVar12 != 0) && (uVar7 = is_known_placeholder_token(uVar13,lVar12), (uVar7 & 1) == 0)) ||
               ((!bVar2 && (*psVar11 == 0x6b73 && (char)psVar11[1] == '-')))) {
              uVar13 = *(undefined8 *)(param_2 + 0x20);
              uVar1 = *(undefined8 *)(param_2 + 0x28);
              lVar10 = atomic_write::copy_atomic_private(puVar5[-1],*puVar5,uVar13,uVar1);
              if (lVar10 == 0) {
                check_auth_integrity(&local_110,uVar13,uVar1);
                if ((lStack_e8 == 2) && (*(short *)pppppppuStack_f0 == 0x6b6f)) {
                  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                  puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
                  if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                    alloc::alloc::handle_alloc_error(8,0x18);
                  }
                  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                  puVar9 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
                  if (puVar9 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                    alloc::raw_vec::handle_error(1,0xe);
                  }
                  *puVar9 = 0x746e695f68747561;
                  *(undefined8 *)((long)puVar9 + 6) = 0x797469726765746e;
                  *puVar8 = 0xe;
                  puVar8[1] = puVar9;
                  puVar8[2] = 0xe;
                  local_168 = (undefined8 ******)0x1;
                  local_158 = 1;
                  local_150 = 0;
                  puStack_148 = (undefined8 *)&SUB_00000008;
                  local_140 = 0;
                  puStack_160 = puVar8;
                  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                  puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
                  if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                    alloc::alloc::handle_alloc_error(8,0x18);
                  }
                  local_120 = (undefined8 *******)puVar5[-1];
                  puStack_118 = (undefined8 *)*puVar5;
                  local_1a0 = &local_120;
                  pcStack_198 = <std::path::Display_as_core::fmt::Display>::fmt;
                  alloc::fmt::format::format_inner(&local_138,s___auth_json_101520153,&local_1a0);
                  puVar8[1] = uStack_130;
                  *puVar8 = local_138;
                  puVar8[2] = local_128;
                  param_1[2] = (long)puStack_160;
                  param_1[1] = (long)local_168;
                  param_1[4] = local_150;
                  param_1[3] = local_158;
                  param_1[6] = local_140;
                  param_1[5] = (long)puStack_148;
                  bVar2 = true;
                  param_1[7] = 1;
                  param_1[8] = (long)puVar8;
                  param_1[9] = 1;
                  *param_1 = 0xb;
                  local_1a0 = (undefined8 *******)local_168;
                  pcStack_198 = (code *)puStack_160;
                  local_190 = local_158;
                  lStack_188 = local_150;
                  puStack_180 = puStack_148;
                  local_178 = local_140;
                }
                else {
                  local_168 = (undefined8 ******)0x0;
                  puStack_160 = (undefined8 *)&SUB_00000008;
                  local_158 = 0;
                  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                  puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
                  if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                    alloc::alloc::handle_alloc_error(8,0x18);
                  }
                  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                  puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
                  if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                    alloc::raw_vec::handle_error(1,0xe);
                  }
                  *puVar8 = 0x746e695f68747561;
                  *(undefined8 *)((long)puVar8 + 6) = 0x797469726765746e;
                  *puVar5 = 0xe;
                  puVar5[1] = puVar8;
                  puVar5[2] = 0xe;
                  local_150 = 1;
                  local_140 = 1;
                  puStack_148 = puVar5;
                  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
                  puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
                  if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
                    alloc::alloc::handle_alloc_error(8,0x18);
                  }
                  if (ppppppuStack_c8 == (undefined8 ******)0x8000000000000000) {
                    pcStack_198 = (code *)lStack_d8;
                    local_1a0 = (undefined8 *******)local_e0;
                    local_190 = local_d0;
                  }
                  else {
                    local_1a0 = (undefined8 *******)ppppppuStack_c8;
                    local_190 = lStack_b8;
                    pcStack_198 = (code *)local_c0;
                    if (local_e0 != (undefined8 ******)0x0) {
                      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_d8,local_e0,1);
                    }
                  }
                  local_120 = &local_1a0;
                  puStack_118 = &<alloc::string::String_as_core::fmt::Display>::fmt;
                  alloc::fmt::format::format_inner(&local_1b8,s___auth_json_101520182,&local_120);
                  if (local_1a0 != (undefined8 *******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pcStack_198,local_1a0,1);
                  }
                  bVar2 = false;
                  puVar5[1] = uStack_1b0;
                  *puVar5 = local_1b8;
                  puVar5[2] = local_1a8;
                  param_1[2] = (long)puStack_160;
                  param_1[1] = (long)local_168;
                  param_1[4] = local_150;
                  param_1[3] = local_158;
                  param_1[6] = local_140;
                  param_1[5] = (long)puStack_148;
                  param_1[7] = 1;
                  param_1[8] = (long)puVar5;
                  param_1[9] = 1;
                  *param_1 = 0xb;
                  local_1a0 = (undefined8 *******)local_168;
                  pcStack_198 = (code *)puStack_160;
                  local_190 = local_158;
                  lStack_188 = local_150;
                  puStack_180 = puStack_148;
                  local_178 = local_140;
                }
                local_168 = local_1a0;
                puStack_160 = (undefined8 *)pcStack_198;
                local_158 = local_190;
                local_150 = lStack_188;
                puStack_148 = puStack_180;
                local_140 = local_178;
                if (local_110 != (undefined8 *******)0x0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_108,local_110,1);
                }
                if (lStack_f8 != 0) {
                  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuStack_f0,lStack_f8,1);
                }
                if (bVar2) {
                  if (local_e0 != (undefined8 ******)0x0) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lStack_d8,local_e0,1);
                  }
                  if ((ppppppuStack_c8 != (undefined8 ******)0x8000000000000000) && (ppppppuStack_c8 != (undefined8 ******)0x0)) {
                    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_c0,ppppppuStack_c8,1);
                  }
                }
              }
              else {
                fix_auth_integrity::__closure__(&local_110);
                param_1[5] = lStack_e8;
                param_1[4] = (long)pppppppuStack_f0;
                param_1[7] = lStack_d8;
                param_1[6] = (long)local_e0;
                param_1[9] = (long)ppppppuStack_c8;
                param_1[8] = local_d0;
                param_1[0xb] = lStack_b8;
                param_1[10] = local_c0;
                param_1[1] = lStack_108;
                *param_1 = (long)local_110;
                param_1[3] = lStack_f8;
                param_1[2] = local_100;
              }
              ::core::ptr::drop_in_place<serde_json::value::Value>(&local_1e0);
              if (lVar3 != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar4,lVar3,1);
              }
              <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_230);
              if (local_230 != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_228,local_230 * 0x18,8);
              }
              if (local_238 == 2) {
                ::core::ptr::drop_in_place<core::result::Result<std::fs::ReadDir,std::io::error::Error>>(&local_240);
                return;
              }
              return;
            }
            ::core::ptr::drop_in_place<serde_json::value::Value>(local_200);
          }
          if (lVar3 != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar4,lVar3,1);
          }
        }
        puVar5 = puVar5 + 3;
        lVar10 = lVar10 + -0x18;
      } while (lVar10 != 0);
      if (uStack_220 != 0) {
        puVar5 = (undefined8 *)(local_228 + 8);
        uVar7 = uStack_220;
        do {
          if (puVar5[-1] != 0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar5,puVar5[-1],1);
          }
          puVar5 = puVar5 + 3;
          uVar7 = uVar7 - 1;
        } while (uVar7 != 0);
      }
    }
    if (local_230 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_228,local_230 * 0x18,8);
    }
    if (local_238 != 2) goto switchD_10035a7e8_caseD_23;
  }
  ::core::ptr::drop_in_place<core::result::Result<std::fs::ReadDir,std::io::error::Error>>(&local_240);
switchD_10035a7e8_caseD_23:
  local_1e0 = 0;
  lStack_1d8 = 8;
  local_1d0 = 0;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
  if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error(8,0x18);
  }
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
  if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0xe);
  }
  *puVar8 = 0x746e695f68747561;
  *(undefined8 *)((long)puVar8 + 6) = 0x797469726765746e;
  *puVar5 = 0xe;
  puVar5[1] = puVar8;
  puVar5[2] = 0xe;
  local_1a0 = (undefined8 *******)0x1;
  local_190 = 1;
  pcStack_198 = (code *)puVar5;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  puVar5 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x18,8);
  if (puVar5 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error(8,0x18);
  }
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  puVar8 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x57,1);
  if (puVar8 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x57);
  }
  puVar8[5] = 0xe9adb8e48690e7a1;
  puVar8[4] = 0xaee7b78fe5a6b4e8;
  puVar8[7] = 0x746168432095bde5;
  puVar8[6] = 0xbb99e7b096e68d87;
  puVar8[9] = 0x7561208da4e5a281;
  puVar8[8] = 0xe6a5bbe420545047;
  *(undefined8 *)((long)puVar8 + 0x4f) = 0x6e6f736a2e687475;
  puVar8[1] = 0xe3bdbbe487a4e5a8;
  *puVar8 = 0x94e7af8fe5a097e6;
  puVar8[3] = 0x20694d614d694120;
  puVar8[2] = 0xa89ce5b7afe88280;
  *puVar5 = 0x57;
  puVar5[1] = puVar8;
  puVar5[2] = 0x57;
  param_1[2] = lStack_1d8;
  param_1[1] = local_1e0;
  param_1[4] = (long)local_1a0;
  param_1[3] = local_1d0;
  param_1[6] = local_190;
  param_1[5] = (long)pcStack_198;
  param_1[7] = 1;
  param_1[8] = (long)puVar5;
  param_1[9] = 1;
  *param_1 = 0xb;
  return;
}

 (GhidraScript)
===END=== (GhidraScript)
