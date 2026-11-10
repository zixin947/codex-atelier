/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* ensure_directories @ 1008ee11c requested 1008ee11c */

/* codexmate_lib::platform::paths::CodexPaths::ensure_directories */

void codexmate_lib::platform::paths::CodexPaths::ensure_directories(long param_1)

{
  long lVar1;
  undefined2 local_34;
  undefined1 local_32;

  local_34 = 0x1ff;
  local_32 = 1;
  lVar1 = std::fs::DirBuilder::_create(&local_34,*(undefined8 *)(param_1 + 200),*(undefined8 *)(param_1 + 0xd0));
  if (lVar1 == 0) {
    local_34 = 0x1ff;
    local_32 = 1;
    lVar1 = std::fs::DirBuilder::_create(&local_34,*(undefined8 *)(param_1 + 0xf8),*(undefined8 *)(param_1 + 0x100));
    if (lVar1 == 0) {
      local_34 = 0x1ff;
      local_32 = 1;
      lVar1 = std::fs::DirBuilder::_create(&local_34,*(undefined8 *)(param_1 + 0x110),*(undefined8 *)(param_1 + 0x118));
      if (lVar1 == 0) {
        local_34 = 0x1ff;
        local_32 = 1;
        lVar1 = std::fs::DirBuilder::_create(&local_34,*(undefined8 *)(param_1 + 0x128),*(undefined8 *)(param_1 + 0x130));
        if (lVar1 == 0) {
          local_34 = 0x1ff;
          local_32 = 1;
          lVar1 = std::fs::DirBuilder::_create(&local_34,*(undefined8 *)(param_1 + 0x158),*(undefined8 *)(param_1 + 0x160));
          if (lVar1 == 0) {
            local_34 = 0x1ff;
            local_32 = 1;
            lVar1 = std::fs::DirBuilder::_create(&local_34,*(undefined8 *)(param_1 + 0x170),*(undefined8 *)(param_1 + 0x178));
            if (lVar1 == 0) {
              local_34 = 0x1ff;
              local_32 = 1;
              lVar1 = std::fs::DirBuilder::_create(&local_34,*(undefined8 *)(param_1 + 0x2a8),*(undefined8 *)(param_1 + 0x2b0));
              if (lVar1 == 0) {
                local_34 = 0x1ff;
                local_32 = 1;
                lVar1 = std::fs::DirBuilder::_create(&local_34,*(undefined8 *)(param_1 + 0x2c0),*(undefined8 *)(param_1 + 0x2c8));
                if (lVar1 == 0) {
                  local_34 = 0x1ff;
                  local_32 = 1;
                  std::fs::DirBuilder::_create(&local_34,*(undefined8 *)(param_1 + 0x248),*(undefined8 *)(param_1 + 0x250));
                }
              }
            }
          }
        }
      }
    }
  }
  return;
}
