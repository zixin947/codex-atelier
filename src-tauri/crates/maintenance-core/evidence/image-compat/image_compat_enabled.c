/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* image_compat_enabled @ 100366934 requested 100366934 */

/* codexmate_lib::core::relay::image_compat::image_compat_enabled */

byte codexmate_lib::core::relay::image_compat::image_compat_enabled(undefined8 param_1,undefined8 param_2)

{
  uint uVar1;
  byte bVar2;
  int iVar3;
  uint uVar4;
  ulong uVar5;
  undefined8 *puVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  uint uVar9;
  long unaff_x26;
  long lVar10;
  long local_1c0;
  undefined1 auStack_1b0 [8];
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  long lStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  long local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  uint local_108;
  undefined4 uStack_104;
  uint uStack_100;
  undefined4 uStack_fc;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  long lStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  
  std::sys::fs::metadata(&local_108);
  uVar1 = local_108;
  if ((local_108 & 1) == 0) {
    uStack_140 = uStack_90;
    local_148 = local_98;
    uStack_130 = uStack_80;
    local_138 = local_88;
    local_128 = local_78;
    uStack_180 = uStack_d0;
    local_188 = local_d8;
    uStack_170 = uStack_c0;
    local_178 = local_c8;
    lStack_150 = lStack_a0;
    local_158 = local_a8;
    uStack_160 = uStack_b0;
    local_168 = local_b8;
    uStack_190 = uStack_e0;
    local_198 = local_e8;
    uStack_1a0 = uStack_f0;
    local_1a8 = local_f8;
    std::fs::Metadata::modified(&local_108,auStack_1b0);
    uVar9 = uStack_100;
    local_1c0 = CONCAT44(uStack_104,local_108);
    if ((uStack_100 == 1000000000) && ((local_108 & 3) == 1)) {
      uVar7 = *(undefined8 *)(local_1c0 + -1);
      puVar6 = *(undefined8 **)(local_1c0 + 7);
      if ((code *)*puVar6 != (code *)0x0) {
        (*(code *)*puVar6)(uVar7);
      }
      if (puVar6[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar7,puVar6[1],puVar6[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)(local_1c0 + -1),0x18,8);
    }
    lVar10 = 1;
    unaff_x26 = lStack_150;
joined_r0x000100366aa8:
    if (DAT_1018ec0e0 == 0) goto LAB_100366aac;
LAB_100366bb8:
    std::sync::once_lock::OnceLock<T>::initialize(&IMAGE_COMPAT_CACHE);
    if (IMAGE_COMPAT_CACHE != (pthread_mutex_t *)0x0) goto LAB_100366ab4;
LAB_100366bcc:
    std::sys::sync::once_box::OnceBox<T>::initialize(&IMAGE_COMPAT_CACHE);
    std::sys::pal::unix::sync::mutex::Mutex::lock();
    if ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) == 0) goto LAB_100366acc;
LAB_100366bf0:
    uVar4 = std::panicking::panic_count::is_zero_slow_path();
    if (DAT_1018ec098 == '\0') {
      uVar4 = uVar4 ^ 1;
      goto joined_r0x000100366c88;
    }
    if (uVar4 != 0) goto LAB_100366ad8;
  }
  else {
    if ((uStack_100 & 3) == 1) {
      puVar6 = (undefined8 *)(CONCAT44(uStack_fc,uStack_100) + -1);
      uVar7 = *puVar6;
      puVar8 = *(undefined8 **)(CONCAT44(uStack_fc,uStack_100) + 7);
      if ((code *)*puVar8 != (code *)0x0) {
        (*(code *)*puVar8)(uVar7);
      }
      if (puVar8[1] != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar7,puVar8[1],puVar8[2]);
      }
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(puVar6,0x18,8);
      lVar10 = 0;
      uVar9 = 1000000000;
      goto joined_r0x000100366aa8;
    }
    lVar10 = 0;
    uVar9 = 1000000000;
    if (DAT_1018ec0e0 != 0) goto LAB_100366bb8;
LAB_100366aac:
    if (IMAGE_COMPAT_CACHE == (pthread_mutex_t *)0x0) goto LAB_100366bcc;
LAB_100366ab4:
    std::sys::pal::unix::sync::mutex::Mutex::lock();
    if ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0) goto LAB_100366bf0;
LAB_100366acc:
    if (DAT_1018ec098 == '\0') {
      uVar4 = 0;
joined_r0x000100366c88:
      if ((DAT_1018ec0a0 == 2) ||
         (iVar3 = <std::path::PathBuf_as_core::cmp::PartialEq<&std::path::Path>>::eq(DAT_1018ec0b8,DAT_1018ec0c0,param_1,param_2), iVar3 == 0)) {
LAB_100366c8c:
        bVar2 = read_image_compat_enabled(param_1,param_2);
        std::path::Path::to_path_buf(&local_108,param_1,param_2);
        uStack_118 = CONCAT44(uStack_fc,uStack_100);
        local_120 = CONCAT44(uStack_104,local_108);
        local_110 = local_f8;
        if ((DAT_1018ec0a0 != 2) && (DAT_1018ec0b0 != 0)) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(DAT_1018ec0b8,DAT_1018ec0b0,1);
        }
        DAT_1018ec0b8 = uStack_118;
        DAT_1018ec0b0 = local_120;
        DAT_1018ec0c0 = local_110;
        DAT_1018ec0a0 = lVar10;
        DAT_1018ec0a8 = unaff_x26;
        DAT_1018ec0c8 = local_1c0;
        DAT_1018ec0d0 = uVar9;
        DAT_1018ec0d8 = bVar2;
      }
      else {
        if (DAT_1018ec0d0 != 1000000000) {
          if (((uVar9 != 1000000000) && (DAT_1018ec0d0 == uVar9)) && (DAT_1018ec0c8 == local_1c0)) goto LAB_100366c2c;
          goto LAB_100366c8c;
        }
        if (uVar9 != 1000000000) goto LAB_100366c8c;
LAB_100366c2c:
        if ((int)DAT_1018ec0a0 == 1) {
          if ((lVar10 == 0) || (DAT_1018ec0a8 != unaff_x26)) goto LAB_100366c8c;
        }
        else if (uVar1 == 0) goto LAB_100366c8c;
      }
      bVar2 = DAT_1018ec0d8;
      if ((((uVar4 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
         (uVar5 = std::panicking::panic_count::is_zero_slow_path(), (uVar5 & 1) == 0)) {
        DAT_1018ec098 = '\x01';
      }
      std::sys::pal::unix::sync::mutex::Mutex::unlock(IMAGE_COMPAT_CACHE);
      goto LAB_100366d2c;
    }
LAB_100366ad8:
    if (((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0) &&
       (uVar5 = std::panicking::panic_count::is_zero_slow_path(), (uVar5 & 1) == 0)) {
      DAT_1018ec098 = '\x01';
    }
  }
  std::sys::pal::unix::sync::mutex::Mutex::unlock(IMAGE_COMPAT_CACHE);
  bVar2 = read_image_compat_enabled(param_1,param_2);
LAB_100366d2c:
  return bVar2 & 1;
}

