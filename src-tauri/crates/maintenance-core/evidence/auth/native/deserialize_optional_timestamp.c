/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* deserialize_optional_timestamp @ 1001f0b8c requested 1001f0b8c */

/* codexmate_lib::core::auth::deserialize_optional_timestamp */

void codexmate_lib::core::auth::deserialize_optional_timestamp(size_t *param_1,long param_2)

{
  ulong uVar1;
  ulong uVar2;
  byte bVar3;
  char cVar4;
  undefined **ppuVar5;
  undefined1 *puVar6;
  int iVar7;
  size_t sVar8;
  void *pvVar9;
  long lVar10;
  long lVar11;
  ulong uVar12;
  undefined8 uVar13;
  undefined1 auVar14 [16];
  byte local_f8;
  undefined4 local_f7;
  undefined3 uStack_f3;
  undefined **local_f0;
  undefined1 *puStack_e8;
  undefined8 uStack_e0;
  size_t local_d8;
  size_t sStack_d0;
  size_t local_c8;
  undefined **local_c0;
  undefined1 *puStack_b8;
  size_t local_a8;
  size_t sStack_a0;
  size_t local_98;
  undefined8 local_90;
  undefined **local_88;
  undefined1 *local_80;
  undefined8 uStack_78;
  undefined **local_70;
  undefined1 *puStack_68;
  undefined8 local_60;
  undefined1 uStack_51;

  uVar2 = *(ulong *)(param_2 + 0x20);
  if (*(ulong *)(param_2 + 0x28) < uVar2) {
    lVar10 = *(ulong *)(param_2 + 0x28) + 4;
    lVar11 = *(long *)(param_2 + 0x18);
    do {
      bVar3 = *(byte *)(lVar11 + lVar10 + -4);
      if (0x20 < bVar3 || (1L << ((ulong)bVar3 & 0x3f) & 0x100002600U) == 0) {
        if (bVar3 == 0x6e) {
          uVar12 = lVar10 - 3;
          *(ulong *)(param_2 + 0x28) = uVar12;
          uVar1 = uVar12;
          if (uVar12 <= uVar2) {
            uVar1 = uVar2;
          }
          if (uVar12 < uVar2) {
            cVar4 = *(char *)(lVar11 + lVar10 + -3);
            *(ulong *)(param_2 + 0x28) = lVar10 - 2U;
            if (cVar4 == 'u') {
              if (lVar10 - 2U == uVar1) goto LAB_1001f0eb0;
              cVar4 = *(char *)(lVar11 + lVar10 + -2);
              *(ulong *)(param_2 + 0x28) = lVar10 - 1U;
              if (cVar4 == 'l') {
                if (lVar10 - 1U == uVar1) goto LAB_1001f0eb0;
                cVar4 = *(char *)(lVar11 + lVar10 + -1);
                *(long *)(param_2 + 0x28) = lVar10;
                if (cVar4 == 'l') {
                  *param_1 = 0x8000000000000000;
                  return;
                }
              }
            }
            local_90 = (undefined ***)0x9;
          }
          else {
LAB_1001f0eb0:
            local_90 = (undefined ***)0x5;
          }
          local_88 = (undefined **)serde_json::de::Deserializer<R>::error(param_2,&local_90);
          goto LAB_1001f0ca0;
        }
        break;
      }
      *(long *)(param_2 + 0x28) = lVar10 + -3;
      lVar10 = lVar10 + 1;
    } while (lVar10 - uVar2 != 4);
  }
  serde_json::value::Value>::deserialize(&local_90);
  puVar6 = local_80;
  ppuVar5 = local_88;
  if ((byte)local_90 == 6) {
LAB_1001f0ca0:
    *param_1 = 0x8000000000000001;
    param_1[1] = (size_t)local_88;
  }
  else {
    uStack_f3 = (undefined3)((ulong)local_90 >> 0x28);
    uStack_e0 = uStack_78;
    puStack_e8 = local_80;
    local_f8 = (byte)local_90;
    local_f0 = local_88;
    local_f7 = local_90._1_4_;
    if ((byte)local_90 == 3) {
      auVar14 = ::core::str::<impl_str>::trim_matches(local_80,uStack_78);
      sVar8 = auVar14._8_8_;
      if (sVar8 == 0) {
        *param_1 = 0x8000000000000000;
      }
      else {
        f64>::from_str(&local_90);
        if (((ulong)local_90 & 1) == 0) {
          normalize_timestamp(&local_d8,auVar14._0_8_,sVar8);
          if (local_d8 == 0x8000000000000000) {
            sVar8 = <serde_json::error::Error_as_serde_core::de::Error>::custom
                              ("last_refresh is outside the supported timestamp rangelast_refresh must be a timestamp string or number",0x35);
            *param_1 = 0x8000000000000001;
            param_1[1] = sVar8;
          }
          else {
            param_1[1] = sStack_d0;
            *param_1 = local_d8;
            param_1[2] = local_c8;
          }
        }
        else {
          if ((long)sVar8 < 0) {
            uVar13 = 0;
LAB_1001f0da8:
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(uVar13,sVar8);
          }
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          uVar13 = 1;
          pvVar9 = (void *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(sVar8,1);
          if (pvVar9 == (void *)0x0) goto LAB_1001f0da8;
          _memcpy(pvVar9,auVar14._0_8_,sVar8);
          *param_1 = sVar8;
          param_1[1] = (size_t)pvVar9;
          param_1[2] = sVar8;
        }
      }
    }
    else {
      if ((byte)local_90 != 2) {
        if ((byte)local_90 == 0) {
          *param_1 = 0x8000000000000000;
        }
        else {
          sVar8 = <serde_json::error::Error_as_serde_core::de::Error>::custom("last_refresh must be a timestamp string or number",0x31);
          *param_1 = 0x8000000000000001;
          param_1[1] = sVar8;
          if (((byte)local_90 & 6) == 2) {
            return;
          }
        }
        ::core::ptr::drop_in_place<serde_json::value::Value>(&local_f8);
        return;
      }
      puStack_b8 = local_80;
      local_c0 = local_88;
      local_70 = (undefined **)0x0;
      puStack_68 = (undefined1 *)0x1;
      local_60 = 0;
      local_80 = &DAT_60000020;
      local_88 = &_anon_dc5b8adae996cb58fd4173e320c2d1cf_1086;
      local_90 = &local_70;
      iVar7 = <serde_json::number::Number_as_core::fmt::Display>::fmt(&local_c0,&local_90);
      puVar6 = puStack_68;
      ppuVar5 = local_70;
      if (iVar7 != 0) {
                    /* WARNING: Subroutine does not return */
        ::core::result::unwrap_failed
                  ("a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaymid > lencalled `Result::unwrap()` on an `Err` valueErrorattempt to join into collection with len > usize::MAX"
                   ,0x37,&uStack_51,&_anon_dc5b8adae996cb58fd4173e320c2d1cf_1103,&_anon_dc5b8adae996cb58fd4173e320c2d1cf_1089);
      }
      normalize_timestamp(&local_a8,puStack_68,local_60);
      if (local_a8 == 0x8000000000000000) {
        sVar8 = <serde_json::error::Error_as_serde_core::de::Error>::custom
                          ("last_refresh is outside the supported timestamp rangelast_refresh must be a timestamp string or number",0x35);
        *param_1 = 0x8000000000000001;
        param_1[1] = sVar8;
      }
      else {
        param_1[1] = sStack_a0;
        *param_1 = local_a8;
        param_1[2] = local_98;
      }
    }
    if (ppuVar5 != (undefined **)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar6,ppuVar5,1);
    }
  }
  return;
}
