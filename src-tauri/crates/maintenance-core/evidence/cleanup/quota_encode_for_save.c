/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* encode_for_save @ 1001e7fb8 requested 1001e7fb8 */

/* codexmate_lib::core::quota_store::encode_for_save */

void codexmate_lib::core::quota_store::encode_for_save(undefined8 *param_1,long param_2)

{
  long *plVar1;
  long *plVar2;
  int iVar3;
  long **pplVar4;
  undefined1 *puVar5;
  long lVar6;
  undefined8 *puVar7;
  long *plVar8;
  undefined8 uVar9;
  long lVar10;
  long *plVar11;
  long local_b8;
  long local_b0;
  long local_a8;
  undefined8 local_a0;
  int local_98 [2];
  long local_90;
  undefined1 *local_88;
  undefined8 local_80;
  long *local_78;
  byte *local_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined1 local_58;
  undefined2 local_50;
  long **local_48;

  iVar3 = *(int *)(param_2 + 0x20);
  uVar9 = *(undefined8 *)(param_2 + 0x18);
  <alloc::vec::Vec<T,A>as_core::clone::Clone>::clone(&local_b8);
  if (iVar3 < 4) {
    iVar3 = 3;
  }
  local_a0 = uVar9;
  local_98[0] = iVar3;
  __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
  local_88 = (undefined1 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x80,1);
  if (local_88 == (undefined1 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x80);
  }
  local_90 = 0x80;
  local_78 = &local_90;
  local_70 = &BYTE_10130460c;
  uStack_60 = 1;
  local_68 = 2;
  local_58 = 0;
  *local_88 = 0x7b;
  local_80 = 1;
  local_50 = 0x100;
  local_48 = &local_78;
  puVar5 = (undefined1 *)
           serde_core::ser::SerializeMap::serialize_entry
                     (&local_50,
                      "schemaVersionupdatedAtitemsaccountKeycapturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
                      ,0xd,local_98);
  if (puVar5 == (undefined1 *)0x0) {
    if ((local_50 & 1) != 0) {
LAB_1001e8080:
      puVar5 = (undefined1 *)serde_json::ser::invalid_raw_value();
      goto LAB_1001e80ac;
    }
    puVar5 = (undefined1 *)
             serde_core::ser::SerializeMap::serialize_entry
                       (&local_50,
                        "updatedAtitemsaccountKeycapturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
                        ,9,&local_a0);
    if (puVar5 != (undefined1 *)0x0) goto LAB_1001e80ac;
    if ((local_50 & 1) != 0) goto LAB_1001e8080;
    puVar5 = (undefined1 *)
             serde_core::ser::SerializeMap::serialize_entry
                       (&local_50,
                        "itemsaccountKeycapturedAtusageSourceprimaryWindowsecondaryWindowavailableResetCreditstokenStatustokenStatusUpdatedAtcursorreset-credit detailshttps://chatgpt.com/backend-api/wham/rate-limit-reset-creditshttps://chatgpt.com/backend-api/wham/rate-limit-reset-credits/historycreditsstatus"
                        ,5,&local_b8);
    pplVar4 = local_48;
    if (puVar5 != (undefined1 *)0x0) goto LAB_1001e80ac;
    if (((local_50 & 1) == 0) && (local_50._1_1_ != '\0')) {
      plVar8 = *local_48;
      plVar11 = (long *)((long)local_48[3] + -1);
      local_48[3] = plVar11;
      if ((*(char *)(pplVar4 + 4) == '\x01') &&
         (alloc::vec::Vec<T,A>::extend_from_slice(plVar8,&_anon_dc5b8adae996cb58fd4173e320c2d1cf_1151,1), plVar11 != (long *)0x0)) {
        plVar1 = pplVar4[1];
        plVar2 = pplVar4[2];
        lVar10 = plVar8[2];
        do {
          if ((long *)(*plVar8 - lVar10) < plVar2) {
            alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle(plVar8,lVar10,plVar2,1,1);
            lVar10 = plVar8[2];
          }
          _memcpy((void *)(plVar8[1] + lVar10),plVar1,(size_t)plVar2);
          lVar10 = lVar10 + (long)plVar2;
          plVar8[2] = lVar10;
          plVar11 = (long *)((long)plVar11 + -1);
        } while (plVar11 != (long *)0x0);
      }
      alloc::vec::Vec<T,A>::extend_from_slice(plVar8,&_anon_dc5b8adae996cb58fd4173e320c2d1cf_16,1);
    }
    puVar5 = local_88;
    if (local_90 != -0x8000000000000000) {
      param_1[2] = local_88;
      param_1[3] = local_80;
      *param_1 = 0xb;
      param_1[1] = local_90;
      <alloc::vec::Vec<T,A>as_core::ops::drop::Drop>::drop(&local_b8);
      goto LAB_1001e8238;
    }
  }
  else {
LAB_1001e80ac:
    if (local_90 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_88,local_90,1);
    }
  }
  *param_1 = 3;
  param_1[1] = puVar5;
  if (local_a8 != 0) {
    puVar7 = (undefined8 *)(local_b0 + 0x98);
    lVar10 = local_a8;
    do {
      if (puVar7[-1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar7,puVar7[-1],1);
      }
      if ((puVar7[-7] != 2) && (lVar6 = puVar7[-5], lVar6 != -0x8000000000000000 && lVar6 != 0)) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar7[-4],lVar6,1);
      }
      puVar7 = puVar7 + 0x18;
      lVar10 = lVar10 + -1;
    } while (lVar10 != 0);
  }
LAB_1001e8238:
  if (local_b8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_b0,local_b8 * 0xc0,8);
  }
  return;
}
