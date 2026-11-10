===BEGIN check_auth_integrity @ 100375360=== (GhidraScript)

/* codexmate_lib::core::relay::codex_diagnostic::check_auth_integrity */

void codexmate_lib::core::relay::codex_diagnostic::check_auth_integrity(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  undefined4 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  char *pcVar5;
  ulong uVar6;
  undefined8 *puVar7;
  undefined8 *******pppppppuVar8;
  long extraout_x1;
  undefined8 uVar9;
  undefined *puVar10;
  undefined1 uVar11;
  long lVar12;
  long lVar13;
  long lVar14;
  long lVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  long lVar18;
  long lVar19;
  undefined8 *******pppppppuVar20;
  undefined8 ******ppppppuVar21;
  long lVar22;
  long lVar23;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 ******local_220;
  undefined8 ******ppppppuStack_218;
  undefined8 ******ppppppuStack_210;
  undefined8 uStack_208;
  undefined8 ******local_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined8 local_1e0;
  undefined8 ******local_1d8;
  undefined8 ******ppppppuStack_1d0;
  undefined8 ******local_1c8;
  undefined8 uStack_1c0;
  undefined8 ******ppppppuStack_1b8;
  undefined8 ******local_1b0;
  undefined8 uStack_1a8;
  undefined8 local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 ******local_110;
  undefined8 ******ppppppuStack_108;
  undefined8 ******local_100;
  undefined8 local_f8;
  undefined8 ******ppppppuStack_f0;
  undefined8 ******local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;

  std::sys::fs::metadata(&local_110,param_2,param_3);
  if ((int)local_110 == 1) {
    ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(1,ppppppuStack_108);
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
    if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0xe);
    }
    *puVar1 = 0x746e695f68747561;
    *(undefined8 *)((long)puVar1 + 6) = 0x797469726765746e;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar2 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(7,1);
    if (puVar2 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,7);
    }
    *(undefined4 *)((long)puVar2 + 3) = 0x676e696e;
    *puVar2 = 0x6e726177;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x13,1);
    if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x13);
    }
    *(undefined4 *)((long)puVar3 + 0xf) = 0xa89ce598;
    puVar3[1] = 0x98ade58db8e4206e;
    *puVar3 = 0x6f736a2e68747561;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x6e,1);
    if (puVar4 != (undefined8 *)0x0) {
      puVar4[9] = 0x9f94e79f8ee5a89c;
      puVar4[8] = 0xe5b7afe88cbcefa8;
      puVar4[0xb] = 0xe7a4aee8aea1e7af;
      puVar4[10] = 0xabe7b788e6a2aee5;
      *(undefined8 *)((long)puVar4 + 0x66) = 0x8280e38180e6b68a;
      *(undefined8 *)((long)puVar4 + 0x5e) = 0xe795bde5bb99e7a4;
      puVar4[1] = 0xe5b6bbe48796e6b0;
      *puVar4 = 0x8ee7918fe5aa9ce6;
      puVar4[3] = 0x47746168439bbcef;
      puVar4[2] = 0x81afe8a4aee88b9e;
      puVar4[5] = 0xe7a894e7bfbde4bd;
      puVar4[4] = 0x83e8af8fe5205450;
      puVar4[7] = 0x82e598ade581afe8;
      puVar4[6] = 0xad87e59fbbe7bbb3;
      *param_1 = 0xe;
      param_1[1] = puVar1;
      param_1[3] = 7;
      param_1[2] = 0xe;
      param_1[4] = puVar2;
      param_1[6] = 0x13;
      param_1[5] = 7;
      param_1[7] = puVar3;
      param_1[9] = 0x6e;
      param_1[8] = 0x13;
      param_1[10] = puVar4;
      param_1[0xb] = 0x6e;
      *(undefined1 *)(param_1 + 0xc) = 1;
      return;
    }
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error(1,0x6e);
  }
  ::core::ptr::drop_in_place<core::result::Result<std::fs::Metadata,std::io::error::Error>>(0,ppppppuStack_108);
  std::fs::read_to_string::inner(&local_110,param_2,param_3);
  ppppppuVar21 = local_100;
  pppppppuVar20 = (undefined8 *******)ppppppuStack_108;
  pppppppuVar8 = (undefined8 *******)local_110;
  if (local_110 == (undefined8 ******)&DAT_8000000000000000) {
    local_220 = ppppppuStack_108;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
    if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0xe);
    }
    *puVar1 = 0x746e695f68747561;
    *(undefined8 *)((long)puVar1 + 6) = 0x797469726765746e;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar2 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
    if (puVar2 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,5);
    }
    *(undefined1 *)(puVar2 + 1) = 0x72;
    *puVar2 = 0x6f727265;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x16,1);
    if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x16);
    }
    puVar3[1] = 0x95b3e6a097e6206e;
    *puVar3 = 0x6f736a2e68747561;
    *(undefined8 *)((long)puVar3 + 0xe) = 0x968fe5bbafe895b3;
    local_1d8 = &local_220;
    ppppppuStack_1d0 = (undefined8 ******)<std::io::error::Error_as_core::fmt::Display>::fmt;
    alloc::fmt::format::format_inner(&local_238,s__10151cfe6,&local_1d8);
    param_1[10] = uStack_230;
    param_1[9] = local_238;
    param_1[0xb] = local_228;
    *param_1 = 0xe;
    param_1[1] = puVar1;
    param_1[3] = 5;
    param_1[2] = 0xe;
    param_1[4] = puVar2;
    param_1[6] = 0x16;
    param_1[5] = 5;
    param_1[7] = puVar3;
    param_1[8] = 0x16;
    *(undefined1 *)(param_1 + 0xc) = 1;
    if (((ulong)local_220 & 3) != 1) {
      return;
    }
    pppppppuVar20 = (undefined8 *******)((long)local_220 + -1);
    ppppppuVar21 = *pppppppuVar20;
    puVar1 = *(undefined8 **)((long)local_220 + 7);
    if ((code *)*puVar1 != (code *)0x0) {
      (*(code *)*puVar1)(ppppppuVar21);
    }
    if (puVar1[1] != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(ppppppuVar21,puVar1[1],puVar1[2]);
    }
    pppppppuVar8 = (undefined8 *******)&DAT_00000018;
    uVar9 = 8;
    goto LAB_100375ccc;
  }
  ::core::str::<impl_str>::trim_matches(ppppppuStack_108,local_100);
  if (extraout_x1 == 0) {
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
    if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0xe);
    }
    *puVar1 = 0x746e695f68747561;
    *(undefined8 *)((long)puVar1 + 6) = 0x797469726765746e;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar2 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
    if (puVar2 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,5);
    }
    *(undefined1 *)(puVar2 + 1) = 0x72;
    *puVar2 = 0x6f727265;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x10,1);
    if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x10);
    }
    puVar3[1] = 0xbaa9e7bab8e4206e;
    *puVar3 = 0x6f736a2e68747561;
    __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
    puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x30,1);
    if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error(1,0x30);
    }
    puVar4[1] = 0xe6aba2e8b9aee585;
    *puVar4 = 0x86e5b6bbe48796e6;
    puVar4[3] = 0x97e6207865646f43;
    puVar4[2] = 0x8cbcefbaa9e785b8;
    puVar4[5] = 0x839de6b489e9b8b8;
    puVar4[4] = 0xe5a3ade695b3e6a0;
    *param_1 = 0xe;
    param_1[1] = puVar1;
    param_1[3] = 5;
    param_1[2] = 0xe;
    param_1[4] = puVar2;
    param_1[6] = 0x10;
    param_1[5] = 5;
    param_1[7] = puVar3;
    param_1[9] = 0x30;
    param_1[8] = 0x10;
    param_1[10] = puVar4;
    param_1[0xb] = 0x30;
    *(undefined1 *)(param_1 + 0xc) = 1;
  }
  else {
    local_110 = pppppppuVar20;
    ppppppuStack_108 = ppppppuVar21;
    local_100 = (undefined8 *******)0x0;
    local_f8 = 0;
    ppppppuStack_f0 = pppppppuVar20;
    local_e8 = ppppppuVar21;
    serde_json::de::from_trait(&local_1d8,&local_110);
    if ((byte)local_1d8 == 6) {
      local_1f8 = ppppppuStack_1d0;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
      if (puVar1 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0xe);
      }
      *puVar1 = 0x746e695f68747561;
      *(undefined8 *)((long)puVar1 + 6) = 0x797469726765746e;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar2 = (undefined4 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
      if (puVar2 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,5);
      }
      *(undefined1 *)(puVar2 + 1) = 0x72;
      *puVar2 = 0x6f727265;
      __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
      puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1b,1);
      if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error(1,0x1b);
      }
      puVar3[1] = 0xe6204e4f534a206e;
      *puVar3 = 0x6f736a2e68747561;
      *(undefined8 *)((long)puVar3 + 0x13) = 0x8f9de59f8de68fbc;
      *(undefined8 *)((long)puVar3 + 0xb) = 0xe5bca0e6204e4f53;
      local_110 = &local_1f8;
      ppppppuStack_108 = (undefined8 ******)<serde_json::error::Error_as_core::fmt::Display>::fmt;
      alloc::fmt::format::format_inner(&local_1f0,s__10152029e,&local_110);
      param_1[10] = uStack_1e8;
      param_1[9] = local_1f0;
      param_1[0xb] = local_1e0;
      *param_1 = 0xe;
      param_1[1] = puVar1;
      param_1[3] = 5;
      param_1[2] = 0xe;
      param_1[4] = puVar2;
      param_1[6] = 0x1b;
      param_1[5] = 5;
      param_1[7] = puVar3;
      param_1[8] = 0x1b;
      *(undefined1 *)(param_1 + 0xc) = 1;
      ::core::ptr::drop_in_place<serde_json::error::Error>(&local_1f8);
    }
    else {
      ppppppuStack_218 = ppppppuStack_1d0;
      local_220 = local_1d8;
      uStack_208 = uStack_1c0;
      ppppppuStack_210 = local_1c8;
      if ((byte)local_1d8 < 3) {
        if ((byte)local_1d8 == 0) {
          local_110 = (undefined8 ******)((ulong)local_110 & 0xffffffffffffff00);
        }
        else {
          ppppppuStack_108 = ppppppuStack_1d0;
          local_110 = local_1d8;
          local_f8 = uStack_1c0;
          local_100 = local_1c8;
        }
      }
      else {
        if ((byte)local_1d8 == 3) {
          <alloc::string::String_as_core::clone::Clone>::clone(&ppppppuStack_108,(ulong)&local_220 | 8);
          uVar11 = 3;
        }
        else if ((byte)local_1d8 == 4) {
          <alloc::vec::Vec<T,A>as_core::clone::Clone>::clone(&ppppppuStack_108,(ulong)&local_220 | 8);
          uVar11 = 4;
        }
        else {
          <alloc::collections::btree::map::BTreeMap<K,V,A>as_core::clone::Clone>::clone(&ppppppuStack_108,(ulong)&local_220 | 8);
          uVar11 = 5;
        }
        local_110 = (undefined8 ******)CONCAT71(local_110._1_7_,uVar11);
      }
      serde_json::value::Value>::deserialize_struct(&local_1d8,&local_110,"AuthFile~/.codexccswitchdeecodexarchivedSKILL.mdmarkdown",8,&PTR_DAT_10181b710,5);
      if (local_1d8 == (undefined8 ******)&DAT_8000000000000001) {
        ::core::ptr::drop_in_place<serde_json::error::Error>(&ppppppuStack_1d0);
LAB_1003759e4:
        pcVar5 = (char *)serde_json::value::Value::pointer(&local_220,"/tokens/access_tokenOPENAI_API_KEY",0x14);
        if (((pcVar5 == (char *)0x0) || (*pcVar5 != '\x03')) || (lVar22 = *(long *)(pcVar5 + 0x18), lVar22 == 0)) {
          pcVar5 = (char *)<str_as_serde_json::value::index::Index>::index_into("OPENAI_API_KEY",0xe,&local_220);
          if (((pcVar5 != (char *)0x0) && (*pcVar5 == '\x03')) &&
             ((2 < *(ulong *)(pcVar5 + 0x18) && (**(short **)(pcVar5 + 0x10) == 0x6b73 && (char)(*(short **)(pcVar5 + 0x10))[1] == '-')))) {
            puVar10 = &DAT_10130d049;
            uVar9 = 0x24;
            goto LAB_1003759cc;
          }
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
          if (puVar1 == (undefined8 *)0x0) goto LAB_100375f24;
          *puVar1 = 0x746e695f68747561;
          *(undefined8 *)((long)puVar1 + 6) = 0x797469726765746e;
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
          if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(1,5);
          }
          *(undefined1 *)((long)puVar3 + 4) = 0x72;
          *(undefined4 *)puVar3 = 0x6f727265;
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x1d,1);
          if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(1,0x1d);
          }
          puVar4[1] = 0x737365636361206e;
          *puVar4 = 0x6f736a2e68747561;
          *(undefined8 *)((long)puVar4 + 0x15) = 0xbaa9e7bab8e4206e;
          *(undefined8 *)((long)puVar4 + 0xd) = 0x656b6f745f737365;
          __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
          puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x61,1);
          if (puVar7 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error(1,0x61);
          }
          *(undefined1 *)(puVar7 + 0xc) = 0xba;
          puVar7[5] = 0xa1a8e6b996e698ae;
          puVar7[4] = 0xe5a0bce48f80e9b1;
          puVar7[7] = 0x313034209e9be594;
          puVar7[6] = 0xbfe886b0e58b9ee5;
          puVar7[9] = 0xe4acace7aba2e8bd;
          puVar7[8] = 0x83e8af8fe58280e3;
          puVar7[0xb] = 0xa9e785b8e6b785e5;
          puVar7[10] = 0xa5b7e5b996e689b8;
          puVar7[1] = 0xe5babce781afe8ad;
          *puVar7 = 0x87e520687475414f;
          puVar7[3] = 0x94e7afb7e8bd83e8;
          puVar7[2] = 0xba99e68cbcefb1a4;
          lVar23 = 0x58;
          lVar12 = 0x50;
          uVar9 = 0x61;
          lVar13 = 0x48;
          lVar14 = 0x40;
          lVar15 = 0x38;
          uVar17 = 0x1d;
          *param_1 = 0xe;
          param_1[1] = puVar1;
          lVar18 = 0x30;
          uVar16 = 5;
          lVar19 = 0x28;
          lVar22 = 0x20;
          param_1[3] = 5;
          param_1[2] = 0xe;
        }
        else {
          lVar23 = *(long *)(pcVar5 + 0x10);
          uVar6 = is_known_placeholder_token(lVar23,lVar22);
          if ((uVar6 & 1) == 0) {
            uVar6 = is_plausible_jwt(lVar23,lVar22);
            if ((uVar6 & 1) != 0) {
              ok_item(param_1,&_anon_164d03a2dfc98f969363eaf1c50504fd_418,0xe,&DAT_10130d0ca,0x16);
              ::core::ptr::drop_in_place<serde_json::value::Value>(&local_220);
              goto LAB_100375cbc;
            }
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            puVar1 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
            if (puVar1 == (undefined8 *)0x0) goto LAB_100375f24;
            *puVar1 = 0x746e695f68747561;
            *(undefined8 *)((long)puVar1 + 6) = 0x797469726765746e;
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(7,1);
            if (puVar3 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error(1,7);
            }
            *(undefined4 *)((long)puVar3 + 3) = 0x676e696e;
            *(undefined4 *)puVar3 = 0x6e726177;
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x23,1);
            if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error(1,0x23);
            }
            *(undefined4 *)((long)puVar4 + 0x1f) = 0xb8b8e582;
            puVar4[1] = 0x737365636361206e;
            *puVar4 = 0x6f736a2e68747561;
            puVar4[3] = 0x82bce58fbce5bca0;
            puVar4[2] = 0xe6206e656b6f745f;
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x3a,1);
            if (puVar7 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error(1,0x3a);
            }
            puVar7[1] = 0xe6af98e68f83e58d;
            *puVar7 = 0xb8e4206e656b6f74;
            puVar7[3] = 0xe58cbcef54574a20;
            puVar7[2] = 0x849ae78895e6899c;
            puVar7[5] = 0x88e68f9de59f8de6;
            puVar7[4] = 0xb2b7e5bd83e8af8f;
            *(undefined8 *)((long)puVar7 + 0x32) = 0xb994e6a1afe7aba2;
            *(undefined8 *)((long)puVar7 + 0x2a) = 0xe89688e68f9de59f;
            *param_1 = 0xe;
            param_1[1] = puVar1;
            lVar23 = 0x58;
            lVar12 = 0x50;
            uVar9 = 0x3a;
            lVar13 = 0x48;
            lVar14 = 0x40;
            lVar15 = 0x38;
            uVar17 = 0x23;
            param_1[3] = 7;
            param_1[2] = 0xe;
            lVar18 = 0x30;
            uVar16 = 7;
            lVar19 = 0x28;
            lVar22 = 0x20;
          }
          else {
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            puVar3 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0xe,1);
            if (puVar3 == (undefined8 *)0x0) {
LAB_100375f24:
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error(1,0xe);
            }
            *puVar3 = 0x746e695f68747561;
            *(undefined8 *)((long)puVar3 + 6) = 0x797469726765746e;
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            puVar4 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(5,1);
            if (puVar4 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error(1,5);
            }
            *(undefined1 *)((long)puVar4 + 4) = 0x72;
            *(undefined4 *)puVar4 = 0x6f727265;
            __RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2();
            puVar7 = (undefined8 *)__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc(0x23,1);
            if (puVar7 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error(1,0x23);
            }
            *(undefined4 *)((long)puVar7 + 0x1f) = 0xa6ace78d;
            puVar7[1] = 0x737365636361206e;
            *puVar7 = 0x6f736a2e68747561;
            puVar7[3] = 0x8dbde4a08de5bab8;
            puVar7[2] = 0xe4206e656b6f745f;
            local_1d8 = (undefined8 ******)<core::str::iter::Chars_as_core::iter::traits::iterator::Iterator>::count(lVar23,lVar23 + lVar22);
            local_110 = &local_1d8;
            ppppppuStack_108 = (undefined8 ******)usize>::fmt;
            alloc::fmt::format::format_inner(&local_128,s___token___101520245,&local_110);
            param_1[10] = uStack_120;
            param_1[9] = local_128;
            uVar16 = 0xe;
            lVar23 = 0x40;
            lVar12 = 0x38;
            uVar9 = 0x23;
            lVar13 = 0x30;
            lVar14 = 0x28;
            lVar15 = 0x20;
            param_1[0xb] = local_118;
            *param_1 = 0xe;
            uVar17 = 5;
            lVar18 = 0x18;
            lVar19 = 0x10;
            lVar22 = 8;
          }
        }
        *(undefined8 **)((long)param_1 + lVar22) = puVar3;
        *(undefined8 *)((long)param_1 + lVar19) = uVar16;
        *(undefined8 *)((long)param_1 + lVar18) = uVar17;
        *(undefined8 **)((long)param_1 + lVar15) = puVar4;
        *(undefined8 *)((long)param_1 + lVar14) = uVar17;
        *(undefined8 *)((long)param_1 + lVar13) = uVar9;
        *(undefined8 **)((long)param_1 + lVar12) = puVar7;
        *(undefined8 *)((long)param_1 + lVar23) = uVar9;
        *(undefined1 *)(param_1 + 0xc) = 1;
      }
      else {
        uStack_90 = uStack_158;
        local_98 = local_160;
        uStack_80 = uStack_148;
        local_88 = local_150;
        uStack_70 = uStack_138;
        local_78 = local_140;
        uStack_d0 = uStack_198;
        local_d8 = local_1a0;
        uStack_c0 = uStack_188;
        local_c8 = local_190;
        uStack_b0 = uStack_178;
        local_b8 = local_180;
        uStack_a0 = uStack_168;
        local_a8 = local_170;
        local_100 = local_1c8;
        ppppppuStack_108 = ppppppuStack_1d0;
        ppppppuStack_f0 = ppppppuStack_1b8;
        local_f8 = uStack_1c0;
        uStack_e0 = uStack_1a8;
        local_e8 = local_1b0;
        local_68 = local_130;
        local_110 = local_1d8;
        uVar6 = auth::is_aimami_managed_virtual_auth(&local_110);
        ::core::ptr::drop_in_place<codexmate_lib::core::auth::AuthFile>(&local_110);
        if ((uVar6 & 1) == 0) goto LAB_1003759e4;
        puVar10 = &DAT_10130d103;
        uVar9 = 0x36;
LAB_1003759cc:
        ok_item(param_1,&_anon_164d03a2dfc98f969363eaf1c50504fd_418,0xe,puVar10,uVar9);
      }
      ::core::ptr::drop_in_place<serde_json::value::Value>(&local_220);
    }
  }
LAB_100375cbc:
  if (pppppppuVar8 == (undefined8 *******)0x0) {
    return;
  }
  uVar9 = 1;
LAB_100375ccc:
  __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(pppppppuVar20,pppppppuVar8,uVar9);
  return;
}

 (GhidraScript)
===END=== (GhidraScript)
