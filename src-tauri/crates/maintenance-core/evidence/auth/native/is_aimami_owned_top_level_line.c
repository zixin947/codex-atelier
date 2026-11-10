/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* is_aimami_owned_top_level_line @ 1001fcb88 requested 1001fcb88 */

/* WARNING: Removing unreachable block (ram,0x0001001fcd8c) */
/* codexmate_lib::core::relay::config_takeover::is_aimami_owned_top_level_line */

ulong codexmate_lib::core::relay::config_takeover::is_aimami_owned_top_level_line(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  long lVar1;
  long lVar2;
  void *pvVar3;
  void *pvVar4;
  void *pvVar5;
  long *plVar6;
  long *plVar7;
  char cVar8;
  void *pvVar9;
  short sVar10;
  char *pcVar11;
  ulong uVar12;
  undefined8 *puVar13;
  long *plVar14;
  void *pvVar15;
  void *pvVar16;
  void *pvVar17;
  long *local_2e0;
  long *plStack_2d8;
  undefined1 local_2c0 [96];
  long *local_260;
  char local_258;
  undefined7 local_257;
  char cStack_250;
  undefined7 local_24f;
  undefined1 uStack_248;
  undefined7 uStack_247;
  long local_240;
  long local_238;
  void *local_230;
  void *local_228;
  void *local_220;
  void *local_218;
  void *local_210;
  void *local_208;
  undefined8 local_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  long *local_168;
  long *plStack_160;
  char local_158;
  undefined7 uStack_157;
  long lStack_148;
  long local_140;
  void *pvStack_138;
  void *local_130;
  void *pvStack_128;
  void *local_120;
  void *pvStack_118;
  void *local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;

  ::core::str::<impl_str>::trim_matches(param_2,param_3);
  top_level_assignment_key(&local_168);
  plVar7 = plStack_160;
  plVar6 = local_168;
  if (local_168 == (long *)&DAT_8000000000000000) {
    return 0;
  }
  lVar1 = CONCAT71(uStack_157,local_158);
  ::core::str::<impl_str>::trim_matches(param_2,param_3);
  top_level_assignment_key(&local_168);
  if (local_168 == (long *)&DAT_8000000000000000) goto LAB_1001fce30;
  plStack_2d8 = plStack_160;
  local_2e0 = local_168;
  <toml_edit::de::Deserializer_as_core::str::traits::FromStr>::from_str(&local_260,param_2,param_3);
  pvVar15 = local_210;
  plVar14 = (long *)CONCAT71(local_257,local_258);
  lVar2 = local_240;
  cVar8 = cStack_250;
  pvVar3 = local_228;
  pvVar4 = local_220;
  pvVar17 = local_218;
  pvVar9 = local_210;
  pvVar5 = local_208;
  if (local_260 == (long *)0x2) {
joined_r0x0001001fcda0:
    pvVar15 = pvVar9;
    local_258 = cVar8;
    pvVar16 = pvVar5;
    if (plVar14 == (long *)0x2) goto LAB_1001fcc7c;
    if (lVar2 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_238,lVar2,1);
    }
    if (((ulong)pvVar15 & 0x7fffffffffffffff) != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar5,pvVar15,1);
    }
    if (pvVar17 != (void *)0x0) {
      puVar13 = (undefined8 *)((long)pvVar4 + 8);
      do {
        if (puVar13[-1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar13,puVar13[-1],1);
        }
        puVar13 = puVar13 + 3;
        pvVar17 = (void *)((long)pvVar17 + -1);
      } while (pvVar17 != (void *)0x0);
    }
    if (pvVar3 != (void *)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar4,(long)pvVar3 * 0x18,8);
    }
  }
  else {
    uStack_a8 = uStack_1a0;
    local_b0 = local_1a8;
    uStack_98 = uStack_190;
    local_a0 = local_198;
    uStack_88 = uStack_180;
    local_90 = local_188;
    uStack_78 = uStack_170;
    local_80 = local_178;
    uStack_e8 = uStack_1e0;
    local_f0 = local_1e8;
    uStack_d8 = uStack_1d0;
    local_e0 = local_1d8;
    uStack_c8 = uStack_1c0;
    local_d0 = local_1c8;
    uStack_b8 = uStack_1b0;
    local_c0 = local_1b8;
    uStack_f8 = uStack_1f0;
    local_100 = local_1f8;
    local_168 = local_260;
    local_158 = cStack_250;
    uStack_157 = local_24f;
    lStack_148 = local_240;
    local_140 = local_238;
    pvStack_138 = local_230;
    local_130 = local_228;
    pvStack_128 = local_220;
    local_120 = local_218;
    pvStack_118 = local_210;
    local_110 = local_208;
    uStack_108 = local_200;
    plStack_160 = plVar14;
    <toml_edit::de::Deserializer_as_serde_core::de::Deserializer>::deserialize_any(&local_260,&local_168);
    pvVar16 = local_208;
    lVar2 = CONCAT71(uStack_247,uStack_248);
    cVar8 = local_258;
    local_238 = local_240;
    pvVar3 = local_230;
    pvVar4 = local_228;
    pvVar17 = local_220;
    pvVar9 = local_218;
    pvVar5 = local_210;
    plVar14 = local_260;
    if (local_260 != (long *)0x2) goto joined_r0x0001001fcda0;
LAB_1001fcc7c:
    if (local_258 != '\a') {
      pcVar11 = (char *)<alloc::string::String_as_toml::value::Index>::index(&local_2e0,local_2c0);
      if (pcVar11 != (char *)0x0) {
        if (*pcVar11 == '\0') {
          pvVar17 = *(void **)(pcVar11 + 0x10);
          pvVar16 = *(void **)(pcVar11 + 0x18);
          alloc::raw_vec::RawVecInner<A>::try_allocate_in(&local_168,pvVar16,0,1,1);
          if ((int)local_168 == 1) {
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(plStack_160,CONCAT71(uStack_157,local_158));
          }
          pvVar15 = (void *)CONCAT71(uStack_157,local_158);
          _memcpy(pvVar15,pvVar17,(size_t)pvVar16);
          plVar14 = plStack_160;
        }
        else {
          plVar14 = (long *)&DAT_8000000000000000;
        }
        ::core::ptr::drop_in_place<toml::value::Value>(local_2c0);
        if (local_2e0 != (long *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_2d8,local_2e0,1);
        }
        if (plVar14 == (long *)&DAT_8000000000000000) goto LAB_1001fce30;
        uVar12 = 0;
        if (lVar1 < 0xe) {
          if (lVar1 == 5) {
            if ((int)*plVar7 == 0x65646f6d && *(char *)((long)plVar7 + 4) == 'l') {
              uVar12 = codex_writer::is_legacy_relay_config_id(pvVar15,pvVar16);
              goto LAB_1001fd034;
            }
          }
          else {
            if (lVar1 != 7) goto LAB_1001fd034;
            if ((int)*plVar7 == 0x666f7270 && *(int *)((long)plVar7 + 3) == 0x656c6966) goto LAB_1001fcfb0;
          }
        }
        else if (lVar1 == 0xe) {
          if (*plVar7 == 0x72705f6c65646f6d && *(long *)((long)plVar7 + 6) == 0x72656469766f7270) {
LAB_1001fcfb0:
            uVar12 = codex_writer::is_aimami_owned_provider_name(pvVar15,pvVar16);
            goto LAB_1001fd034;
          }
        }
        else if (lVar1 == 0xf) {
          if (*plVar7 == 0x625f69616e65706f && *(long *)((long)plVar7 + 7) == 0x6c72755f65736162) {
            sVar10 = codex_writer::aimami_router_base_url_port(pvVar15,pvVar16);
            uVar12 = (ulong)(sVar10 == 1);
            goto LAB_1001fd034;
          }
        }
        else {
          if (lVar1 != 0x12) goto LAB_1001fd034;
          if ((*plVar7 == 0x61635f6c65646f6d && plVar7[1] == 0x736a5f676f6c6174) && (short)plVar7[2] == 0x6e6f) {
            uVar12 = is_aimami_catalog_path(param_1,pvVar15,pvVar16);
            goto LAB_1001fd034;
          }
        }
        uVar12 = 0;
LAB_1001fd034:
        if (plVar14 != (long *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pvVar15,plVar14,1);
        }
        if (plVar6 != (long *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar7,plVar6,1);
          return uVar12;
        }
        return uVar12;
      }
      ::core::ptr::drop_in_place<toml::value::Value>(local_2c0);
    }
  }
  if (local_2e0 != (long *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plStack_2d8,local_2e0,1);
  }
LAB_1001fce30:
  if (plVar6 != (long *)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar7,plVar6,1);
  }
  return 0;
}
