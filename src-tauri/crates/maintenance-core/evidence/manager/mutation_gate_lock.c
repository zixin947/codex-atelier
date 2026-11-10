/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* lock @ 1005c2754 requested 1005c2754 */

/* codexmate_lib::core::relay::codex_mutation::CodexMutationGate::lock */

ulong codexmate_lib::core::relay::codex_mutation::CodexMutationGate::lock(undefined8 *param_1,long *param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 *puVar1;
  char cVar2;
  uint uVar3;
  ulong uVar4;
  undefined1 uVar5;
  long lVar6;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined1 *local_70;
  undefined8 *puStack_68;
  undefined8 local_60;
  undefined1 *local_58;
  undefined8 uStack_50;
  undefined8 local_48;

  lVar6 = *param_2;
  puVar1 = (undefined8 *)(lVar6 + 0x10);
  local_80 = param_3;
  uStack_78 = param_4;
  if (*(long *)(lVar6 + 0x10) == 0) {
    std::sys::sync::once_box::OnceBox<T>::initialize(puVar1);
    uVar4 = std::sys::pal::unix::sync::mutex::Mutex::lock();
  }
  else {
    uVar4 = std::sys::pal::unix::sync::mutex::Mutex::lock();
  }
  if ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) == 0) {
    uVar3 = 0;
    uVar5 = 0;
    cVar2 = *(char *)(lVar6 + 0x18);
  }
  else {
    uVar4 = std::panicking::panic_count::is_zero_slow_path();
    uVar3 = (uint)uVar4 ^ 1;
    uVar5 = (undefined1)uVar3;
    cVar2 = *(char *)(lVar6 + 0x18);
  }
  if (cVar2 == '\0') {
    *(undefined1 *)(param_1 + 2) = uVar5;
    *param_1 = 0xb;
    param_1[1] = puVar1;
  }
  else {
    puStack_68 = &<&T_as_core::fmt::Display>::fmt;
    local_70 = (undefined1 *)&local_80;
    alloc::fmt::format::format_inner(&local_58,_anon_62728874ff9674d7d8f5de25cb1e85f1_536,&local_70);
    puStack_68 = (undefined8 *)uStack_50;
    local_70 = local_58;
    local_60 = local_48;
    if ((((uVar3 & 1) == 0) && ((std::panicking::panic_count::GLOBAL_PANIC_COUNT & 0x7fffffffffffffff) != 0)) &&
       (uVar4 = std::panicking::panic_count::is_zero_slow_path(), (uVar4 & 1) == 0)) {
      *(undefined1 *)(lVar6 + 0x18) = 1;
    }
    uVar3 = std::sys::pal::unix::sync::mutex::Mutex::unlock((pthread_mutex_t *)*puVar1);
    uVar4 = (ulong)uVar3;
    *param_1 = 10;
    param_1[2] = puStack_68;
    param_1[1] = local_70;
    param_1[3] = local_60;
  }
  return uVar4;
}
