/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* poll @ 100476ed8 requested 100476ed8 */

/* <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll */

void <tokio::runtime::blocking::task::BlockingTask<T>as_core::future::future::Future>::poll(undefined8 *param_1,long *param_2)

{
  undefined8 *puVar1;
  int iVar2;
  ulong uVar3;
  long lVar4;
  undefined8 uVar5;
  undefined1 unaff_w24;
  undefined8 unaff_x25;
  long lVar6;
  long lVar7;
  long lVar8;
  long lVar9;
  long lVar10;
  long lVar11;
  long lVar12;
  long lVar13;
  long lVar14;
  long lVar15;
  long lVar16;
  long lVar17;
  long lVar18;
  long lVar19;
  long lVar20;
  long lVar21;
  long lVar22;
  long lVar23;
  long local_1f0;
  long lStack_1e8;
  long lStack_1e0;
  long lStack_1d8;
  long local_1d0;
  long lStack_1c8;
  long lStack_1c0;
  long lStack_1b8;
  long local_1b0;
  long lStack_1a8;
  long lStack_1a0;
  long lStack_198;
  long local_190;
  long lStack_188;
  long local_180;
  long lStack_178;
  long lStack_170;
  long local_168;
  long local_160;
  char *local_150;
  undefined8 *puStack_148;
  ulong local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined4 local_e8;
  undefined3 uStack_e4;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 *puStack_c8;
  undefined **local_c0;
  undefined1 *local_b8;
  undefined8 local_b0;
  undefined8 *puStack_a8;
  ulong local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 uStack_58;
  
  lVar4 = *param_2;
  *param_2 = 3;
  if (lVar4 == 3) {
                    /* WARNING: Subroutine does not return */
    core::option::expect_failed(0x10131894f,0x2d,&_anon_b5f536351c4bf1e454cf90283dd9e625_7);
  }
  lVar21 = param_2[0xc];
  lVar18 = param_2[0xb];
  lVar12 = param_2[0xe];
  lVar6 = param_2[0xd];
  lVar13 = param_2[0x10];
  lVar7 = param_2[0xf];
  lVar14 = param_2[0x12];
  lVar8 = param_2[0x11];
  lVar22 = param_2[4];
  lVar19 = param_2[3];
  lVar15 = param_2[6];
  lVar9 = param_2[5];
  lVar23 = param_2[8];
  lVar20 = param_2[7];
  lVar16 = param_2[10];
  lVar10 = param_2[9];
  lVar17 = param_2[2];
  lVar11 = param_2[1];
  tokio::task::coop::stop();
  local_1f0 = lVar4;
  lStack_1e8 = lVar11;
  lStack_1e0 = lVar17;
  lStack_1d8 = lVar19;
  local_1d0 = lVar22;
  lStack_1c8 = lVar9;
  lStack_1c0 = lVar15;
  lStack_1b8 = lVar20;
  local_1b0 = lVar23;
  lStack_1a8 = lVar10;
  lStack_1a0 = lVar16;
  lStack_198 = lVar18;
  local_190 = lVar21;
  lStack_188 = lVar6;
  local_180 = lVar12;
  lStack_178 = lVar7;
  lStack_170 = lVar13;
  local_168 = lVar8;
  local_160 = lVar14;
  lVar4 = tauri::state::StateManager::try_get(*(long *)(lVar8 + 0x1308) + 0x10);
  if (lVar4 == 0) {
    local_150 = 
    "codexmate_lib::core::relay::codex_mutation::CodexMutationGatecodexmate_lib::core::relay::manager::RelayManagerstd::sync::poison::mutex::Mutex<codexmate_lib::core::repository::Repository>poisoned menu mutextauri::apppoisoned plugin store2.10.3"
    ;
    puStack_148 = (undefined8 *)0x3d;
    local_b0 = &local_150;
    puStack_a8 = &<&T_as_core::fmt::Display>::fmt;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt(_anon_62728874ff9674d7d8f5de25cb1e85f1_1680,&local_b0,&_anon_62728874ff9674d7d8f5de25cb1e85f1_1681);
  }
  codexmate_lib::core::relay::codex_mutation::CodexMutationGate::lock(&local_150,lVar4,&DAT_1013191da,0x22);
  uVar3 = local_140;
  puVar1 = puStack_148;
  if (local_150 != (char *)0xb) {
    uStack_88 = uStack_128;
    local_90 = local_130;
    uStack_78 = uStack_118;
    local_80 = uStack_120;
    uStack_68 = uStack_108;
    local_70 = local_110;
    uStack_58 = uStack_f8;
    local_60 = uStack_100;
    puStack_a8 = puStack_148;
    local_b0 = (char **)local_150;
    uStack_98 = uStack_138;
    local_a0 = local_140;
    local_e0 = 0;
    uStack_d8 = 1;
    local_b8 = &DAT_60000020;
    puStack_c8 = &local_e0;
    local_d0 = 0;
    local_c0 = &_anon_b5f536351c4bf1e454cf90283dd9e625_474;
    iVar2 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_b0,&puStack_c8);
    unaff_x25 = uStack_d8;
    uVar5 = local_e0;
    if (iVar2 != 0) {
                    /* WARNING: Subroutine does not return */
      core::result::unwrap_failed
                ("a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap()` on an `Err` valueErrorCreateWebviewCreateWindowInvalidWindowLabelFailedToSendMessageFailedToReceiveMessageInvalidIconFailedToGetMonitorFailedToGetCursorPositionInvalidHeaderNameInvalidHeaderValueInvalidStatusCodeInvalidMethodEventLoopClosedInvalidProxyUrlWindowNotFoundFailedToRemoveDataStoreWebviewRuntimeNotInstalledCallbackFnresolver consumed_root_data did not match any variant of untagged enum JsImage"
                 ,0x37,&local_e8,&_anon_b5f536351c4bf1e454cf90283dd9e625_503,&_anon_b5f536351c4bf1e454cf90283dd9e625_477);
    }
    unaff_w24 = (undefined1)local_d0;
    uStack_e4 = (undefined3)((ulong)local_d0 >> 0x28);
    local_e8 = local_d0._1_4_;
    core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_150);
    local_b0._0_3_ = (undefined3)local_e8;
    local_b0._0_7_ = CONCAT43(CONCAT31(uStack_e4,local_e8._3_1_),(undefined3)local_b0);
    goto LAB_100477184;
  }
  codexmate_lib::platform::process::CodexProcessLifecycle::prepare_transition(&local_b0,5,0,0);
  if (local_b0 == (char **)0xb) {
    codexmate_lib::platform::process::CodexProcessLifecycle::launch(&local_150);
    if (local_150 == (char *)0xb) goto LAB_10047700c;
LAB_1004770d8:
    uStack_88 = uStack_128;
    local_90 = local_130;
    uStack_78 = uStack_118;
    local_80 = uStack_120;
    uStack_68 = uStack_108;
    local_70 = local_110;
    uStack_58 = uStack_f8;
    local_60 = uStack_100;
    puStack_a8 = puStack_148;
    local_b0 = (char **)local_150;
    uStack_98 = uStack_138;
    local_a0 = local_140;
    local_e0 = 0;
    uStack_d8 = 1;
    local_b8 = &DAT_60000020;
    puStack_c8 = &local_e0;
    local_d0 = 0;
    local_c0 = &_anon_b5f536351c4bf1e454cf90283dd9e625_474;
    iVar2 = <codexmate_lib::core::models::CoreError_as_core::fmt::Display>::fmt(&local_b0,&puStack_c8);
    unaff_x25 = uStack_d8;
    uVar5 = local_e0;
    if (iVar2 != 0) {
                    /* WARNING: Subroutine does not return */
      core::result::unwrap_failed
                ("a Display implementation returned an error unexpectedlydescription() is deprecated; use Displaycalled `Result::unwrap()` on an `Err` valueErrorCreateWebviewCreateWindowInvalidWindowLabelFailedToSendMessageFailedToReceiveMessageInvalidIconFailedToGetMonitorFailedToGetCursorPositionInvalidHeaderNameInvalidHeaderValueInvalidStatusCodeInvalidMethodEventLoopClosedInvalidProxyUrlWindowNotFoundFailedToRemoveDataStoreWebviewRuntimeNotInstalledCallbackFnresolver consumed_root_data did not match any variant of untagged enum JsImage"
                 ,0x37,&local_e8,&_anon_b5f536351c4bf1e454cf90283dd9e625_503,&_anon_b5f536351c4bf1e454cf90283dd9e625_477);
    }
    unaff_w24 = (undefined1)local_d0;
    uStack_e4 = (undefined3)((ulong)local_d0 >> 0x28);
    local_e8 = local_d0._1_4_;
    core::ptr::drop_in_place<codexmate_lib::core::models::CoreError>(&local_150);
    local_b0._0_3_ = (undefined3)local_e8;
    local_b0._0_7_ = CONCAT43(CONCAT31(uStack_e4,local_e8._3_1_),(undefined3)local_b0);
  }
  else {
    uStack_128 = uStack_88;
    local_130 = local_90;
    uStack_118 = uStack_78;
    uStack_120 = local_80;
    uStack_108 = uStack_68;
    local_110 = local_70;
    uStack_f8 = uStack_58;
    uStack_100 = local_60;
    puStack_148 = puStack_a8;
    local_150 = (char *)local_b0;
    uStack_138 = uStack_98;
    local_140 = local_a0;
    if (local_b0 != (char **)0xb) goto LAB_1004770d8;
LAB_10047700c:
    uVar5 = 0x8000000000000000;
  }
  if ((((uVar3 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
     (uVar3 = std::panicking::panic_count::is_zero_slow_path(), (uVar3 & 1) == 0)) {
    *(undefined1 *)(puVar1 + 1) = 1;
  }
  std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*puVar1);
LAB_100477184:
  core::ptr::drop_in_place<tauri::app::AppHandle>(&local_1f0);
  *param_1 = uVar5;
  param_1[1] = unaff_x25;
  *(undefined1 *)(param_1 + 2) = unaff_w24;
  *(undefined4 *)((long)param_1 + 0x11) = (undefined4)local_b0;
  *(undefined4 *)((long)param_1 + 0x14) = local_b0._3_4_;
  return;
}

