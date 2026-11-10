/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* repair_if_needed @ 1003b0528 requested 1003b0528 */

/* codexmate_lib::core::relay::codex_project_state::repair_if_needed */

void codexmate_lib::core::relay::codex_project_state::repair_if_needed(undefined8 *param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 *****pppppuVar2;
  undefined8 *****pppppuVar3;
  undefined8 *puVar4;
  long lVar5;
  long local_350;
  undefined8 local_348;
  long local_338;
  long local_330;
  long local_328;
  long local_320;
  undefined8 local_318;
  long local_308;
  undefined8 local_300;
  long local_2f0;
  undefined8 local_2e8;
  undefined8 local_2e0;
  long local_2d8;
  undefined8 local_2d0;
  undefined8 local_2c0;
  undefined8 local_2a0;
  byte local_276;
  undefined8 *****local_270;
  undefined8 *puStack_268;
  undefined8 local_260;
  undefined8 local_250;
  undefined8 uStack_248;
  undefined8 local_240;
  undefined8 *****local_238;
  undefined8 *puStack_230;
  undefined8 uStack_228;
  long local_220;
  undefined8 uStack_218;
  undefined8 local_210;
  long lStack_208;
  undefined8 local_200;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined8 local_1e0;
  undefined8 uStack_1d8;
  undefined8 *local_1d0;
  undefined8 uStack_1c8;
  long local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  long lStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 *****local_170;
  undefined8 *puStack_168;
  undefined8 local_160;
  long local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  long lStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 *local_90;
  undefined8 uStack_88;
  long local_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 **local_60;
  undefined8 *puStack_58;

  inspect(&local_350);
  stability_issue(&local_170,&local_350);
  pppppuVar2 = local_170;
  uVar1 = local_2e8;
  lVar5 = local_2f0;
  if (local_170 == (undefined8 *****)&DAT_8000000000000000) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x27,1);
    if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x27);
    }
    puVar4[1] = 0xe5a2b4e7ae9be7b9;
    *puVar4 = 0xa1e9207865646f43;
    puVar4[3] = 0x9ce9a097e68cbcef;
    puVar4[2] = 0xb8b8e5a3ade695bc;
    *(undefined8 *)((long)puVar4 + 0x1f) = 0x8da4e5aebfe4809c;
    param_1[6] = lVar5;
    param_1[7] = uVar1;
    *param_1 = 0x27;
    param_1[1] = puVar4;
    param_1[3] = 0x8000000000000000;
    param_1[2] = 0x27;
    param_1[8] = local_2e0;
    param_1[9] = local_2c0;
    param_1[10] = local_2a0;
    param_1[0xb] = 0;
    *(undefined1 *)(param_1 + 0xc) = 0;
  }
  else {
    puStack_268 = puStack_168;
    local_270 = local_170;
    local_260 = local_160;
    if ((local_276 & 1) == 0) {
      local_170 = &local_270;
      puStack_168 = &<alloc::string::String_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_250,s__Codex_project_index_cannot_be_r_1015238a7,&local_170);
      param_1[4] = local_240;
      param_1[1] = 10;
      *param_1 = 0x8000000000000000;
      param_1[3] = uStack_248;
      param_1[2] = local_250;
    }
    else {
      repair(&local_170,param_2);
      pppppuVar3 = local_170;
      lStack_1a8 = lStack_140;
      local_1b0 = local_148;
      uStack_198 = uStack_130;
      uStack_1a0 = local_138;
      uStack_188 = uStack_120;
      local_190 = local_128;
      uStack_178 = uStack_110;
      uStack_180 = local_118;
      uStack_1c8 = local_160;
      local_1d0 = puStack_168;
      uStack_1b8 = uStack_150;
      local_1c0 = local_158;
      if (local_170 == (undefined8 *****)&DAT_8000000000000000) {
        param_1[6] = lStack_140;
        param_1[5] = local_148;
        param_1[8] = uStack_130;
        param_1[7] = local_138;
        param_1[10] = uStack_120;
        param_1[9] = local_128;
        param_1[0xc] = uStack_110;
        param_1[0xb] = local_118;
        param_1[2] = local_160;
        param_1[1] = puStack_168;
        param_1[4] = uStack_150;
        param_1[3] = local_158;
        *param_1 = 0x8000000000000000;
      }
      else {
        lStack_208 = lStack_140;
        local_210 = local_148;
        uStack_1f8 = uStack_130;
        local_200 = local_138;
        uStack_1e8 = uStack_120;
        local_1f0 = local_128;
        uStack_1d8 = uStack_110;
        local_1e0 = local_118;
        uStack_228 = local_160;
        puStack_230 = puStack_168;
        uStack_218 = uStack_150;
        local_220 = local_158;
        local_238 = local_170;
        inspect(&local_170,param_2);
        stability_issue(&local_90,&local_170);
        if (local_90 == (undefined8 *)0x8000000000000000) {
          param_1[9] = local_1f0;
          param_1[8] = uStack_1f8;
          param_1[0xb] = local_1e0;
          param_1[10] = uStack_1e8;
          param_1[0xc] = uStack_1d8;
          param_1[1] = puStack_230;
          *param_1 = local_238;
          param_1[3] = local_220;
          param_1[2] = uStack_228;
          param_1[5] = local_210;
          param_1[4] = uStack_218;
          param_1[7] = local_200;
          param_1[6] = lStack_208;
          ::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_project_state::ProjectStateInspection>(&local_170);
          if ((undefined8 ******)local_270 != (undefined8 ******)0x0) {
            __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_268,local_270,1);
          }
          ::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_project_state::ProjectStateInspection>(&local_350);
          return;
        }
        uStack_1c8 = uStack_88;
        local_1d0 = local_90;
        local_1c0 = local_80;
        local_60 = &local_1d0;
        puStack_58 = &<alloc::string::String_as_core::fmt::Display>::fmt;
        alloc::fmt::format::format_inner(&local_78,s_4Codex_project_index_is_still_un_1015238d9,&local_60);
        param_1[4] = local_68;
        param_1[1] = 10;
        *param_1 = 0x8000000000000000;
        param_1[3] = uStack_70;
        param_1[2] = local_78;
        if (local_1d0 != (undefined8 *)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_1c8,local_1d0,1);
        }
        ::core::ptr::drop_in_place<codexmate_lib::core::relay::codex_project_state::ProjectStateInspection>(&local_170);
        if ((local_220 != -0x8000000000000000) && (local_220 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uStack_218,local_220,1);
        }
        if ((lStack_208 != -0x8000000000000000) && (lStack_208 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_200,lStack_208,1);
        }
        if ((undefined8 ******)pppppuVar3 != (undefined8 ******)0x0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_230,pppppuVar3,1);
        }
      }
    }
    if ((undefined8 ******)local_270 != (undefined8 ******)0x0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puStack_268,local_270,1);
    }
  }
  if (local_350 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_348,local_350,1);
  }
  if ((local_320 != -0x8000000000000000) && (local_320 != 0)) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_318,local_320,1);
  }
  if ((local_308 != -0x8000000000000000) && (local_308 != 0)) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_300,local_308,1);
  }
  if (((pppppuVar2 != (undefined8 *****)&DAT_8000000000000000) && (local_2f0 != -0x8000000000000000)) && (local_2f0 != 0)) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2e8,local_2f0,1);
  }
  if (local_328 != 0) {
    puVar4 = (undefined8 *)(local_330 + 8);
    lVar5 = local_328;
    do {
      if (puVar4[-1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*puVar4,puVar4[-1],1);
      }
      puVar4 = puVar4 + 3;
      lVar5 = lVar5 + -1;
    } while (lVar5 != 0);
  }
  if (local_338 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_330,local_338 * 0x18,8);
  }
  if (local_2d8 != -0x8000000000000000 && local_2d8 != 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_2d0,local_2d8,1);
  }
  return;
}
