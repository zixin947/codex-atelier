/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* compose_proxy_status @ 10067dd5c requested 10067dd5c */

/* codexmate_lib::core::relay::manager::RelayManager::compose_proxy_status */

int codexmate_lib::core::relay::manager::RelayManager::compose_proxy_status(undefined8 *param_1,long param_2)

{
  char cVar1;
  undefined2 uVar2;
  int iVar3;
  uint uVar4;
  ulong uVar5;
  undefined2 *puVar6;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined2 *local_78;
  code *pcStack_70;
  undefined8 local_68;
  undefined2 *local_60;
  code *pcStack_58;

  if (*(long *)(param_2 + 0x10) == 0) {
    std::sys::sync::once_box::OnceBox<T>::initialize(param_2 + 0x10);
    std::sys::pal::unix::sync::mutex::Mutex::lock();
  }
  else {
    std::sys::pal::unix::sync::mutex::Mutex::lock();
  }
  if ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) == 0) {
    uVar4 = 0;
    cVar1 = *(char *)(param_2 + 0x18);
  }
  else {
    uVar4 = std::panicking::panic_count::is_zero_slow_path();
    uVar4 = uVar4 ^ 1;
    cVar1 = *(char *)(param_2 + 0x18);
  }
  if (cVar1 != '\0') {
    *(undefined1 *)((long)param_1 + 0x4a) = 0;
    *(undefined2 *)(param_1 + 9) = 0;
    *param_1 = 0;
    param_1[1] = 1;
    param_1[2] = 0;
    param_1[3] = 0;
    param_1[4] = 1;
    param_1[6] = 0x8000000000000000;
    param_1[5] = 0;
    if ((((uVar4 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
       (uVar5 = std::panicking::panic_count::is_zero_slow_path(), (uVar5 & 1) == 0)) {
      *(undefined1 *)(param_2 + 0x18) = 1;
    }
    iVar3 = std::sys::pal::unix::sync::mutex::Mutex::unlock(*(pthread_mutex_t **)(param_2 + 0x10));
    return iVar3;
  }
  if (*(long *)(param_2 + 0x20) == 0) {
    *(undefined1 *)((long)param_1 + 0x4a) = 0;
    *(undefined2 *)(param_1 + 9) = 0;
    *param_1 = 0;
    param_1[1] = 1;
    param_1[2] = 0;
    param_1[3] = 0;
    param_1[4] = 1;
    param_1[6] = 0x8000000000000000;
    param_1[5] = 0;
  }
  else {
    puVar6 = (undefined2 *)(param_2 + 0x30);
    uVar2 = *puVar6;
    pcStack_70 = u16>::fmt;
    local_78 = puVar6;
    alloc::fmt::format::format_inner(&local_90,_anon_1f4a05fd1dc96b9982818105670d4dce_206,&local_78);
    pcStack_58 = u16>::fmt;
    local_60 = puVar6;
    alloc::fmt::format::format_inner(&local_78,s_http___127_0_0_1___codex_v1_101527bce,&local_60);
    *(undefined1 *)((long)param_1 + 0x4a) = 1;
    *(undefined2 *)(param_1 + 9) = uVar2;
    param_1[1] = uStack_88;
    *param_1 = local_90;
    param_1[2] = local_80;
    param_1[4] = pcStack_70;
    param_1[3] = local_78;
    param_1[5] = local_68;
    param_1[6] = 0x8000000000000000;
  }
  if ((((uVar4 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
     (uVar5 = std::panicking::panic_count::is_zero_slow_path(), (uVar5 & 1) == 0)) {
    *(undefined1 *)(param_2 + 0x18) = 1;
  }
  iVar3 = std::sys::pal::unix::sync::mutex::Mutex::unlock(*(pthread_mutex_t **)(param_2 + 0x10));
  return iVar3;
}
