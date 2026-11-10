/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* save @ 1001e92e8 requested 1001e92e8 */

/* codexmate_lib::core::quota_store::save */

void codexmate_lib::core::quota_store::save(long *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  long lVar1;
  undefined1 auVar2 [16];
  undefined2 local_a0;
  undefined1 uStack_9e;
  undefined5 uStack_9d;
  long lStack_98;
  long local_90;
  long lStack_88;
  long local_80;
  long lStack_78;
  long lStack_70;
  long lStack_68;
  long local_60;
  long lStack_58;
  long lStack_50;
  long lStack_48;

  auVar2 = std::path::Path::parent();
  if (auVar2._0_8_ != 0) {
    local_a0 = 0x1ff;
    uStack_9e = 1;
    lVar1 = std::fs::DirBuilder::_create(&local_a0,auVar2._0_8_,auVar2._8_8_);
    if (lVar1 != 0) {
      *param_1 = 2;
      param_1[1] = lVar1;
      return;
    }
  }
  encode_for_save(&local_a0,param_4);
  lVar1 = CONCAT53(uStack_9d,CONCAT12(uStack_9e,local_a0));
  if (lVar1 == 0xb) {
    auVar2 = relay::atomic_write::write_atomic_private_report(param_2,param_3,local_90);
    if ((auVar2._0_8_ & 1) == 0) {
      relay::atomic_write::Publication::acknowledge(auVar2._8_8_);
      *param_1 = 0xb;
    }
    else {
      *param_1 = 2;
      param_1[1] = auVar2._8_8_;
    }
    if (lStack_98 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(local_90,lStack_98,1);
    }
  }
  else {
    param_1[5] = lStack_78;
    param_1[4] = local_80;
    param_1[7] = lStack_68;
    param_1[6] = lStack_70;
    param_1[9] = lStack_58;
    param_1[8] = local_60;
    param_1[0xb] = lStack_48;
    param_1[10] = lStack_50;
    param_1[2] = local_90;
    param_1[3] = lStack_88;
    *param_1 = lVar1;
    param_1[1] = lStack_98;
  }
  return;
}
