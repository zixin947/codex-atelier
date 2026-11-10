/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* read_image_compat_enabled @ 100366dbc requested 100366dbc */

/* codexmate_lib::core::relay::image_compat::read_image_compat_enabled */

undefined8 codexmate_lib::core::relay::image_compat::read_image_compat_enabled(void)

{
  long lVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  ushort uVar6;
  bool bVar7;
  long *plVar8;
  int *piVar9;
  long lVar10;
  long lVar11;
  undefined1 auVar13 [16];
  long local_c0;
  long local_b8;
  long local_b0;
  long lStack_a8;
  undefined8 local_a0;
  long lStack_98;
  undefined8 local_90;
  undefined1 local_88;
  undefined2 local_80;
  int local_78 [4];
  long local_68;
  ulong uVar12;
  
  std::fs::read_to_string::inner(&local_c0);
  lStack_a8 = local_b0;
  lVar5 = local_b8;
  lVar4 = local_c0;
  if (local_c0 == -0x8000000000000000) {
    ::core::ptr::drop_in_place<core::result::Result<alloc::string::String,std::io::error::Error>>(&local_c0);
  }
  else {
    bVar7 = false;
    local_c0 = 0;
    local_b8 = local_b0;
    local_b0 = lVar5;
    local_a0 = 0;
    lStack_98 = lStack_a8;
    local_90 = 0xa0000000a;
    local_88 = 1;
    local_80 = 0;
    do {
      lVar2 = local_b0;
      <core::str::pattern::CharSearcher_as_core::str::pattern::Searcher>::next_match(local_78,&local_b0);
      uVar6 = local_80;
      if (local_78[0] == 1) {
        lVar10 = local_68 - local_c0;
        lVar3 = local_68;
      }
      else {
        if ((local_80 & 0x100) != 0) break;
        local_80 = CONCAT11(1,(undefined1)local_80);
        if (((uVar6 & 1) == 0) && (local_b8 == local_c0)) break;
        lVar10 = local_b8 - local_c0;
        lVar3 = local_c0;
        lVar2 = local_b0;
      }
      lVar2 = lVar2 + local_c0;
      lVar11 = lVar10;
      if ((lVar10 != 0) && (lVar1 = lVar2 + lVar10, *(char *)(lVar1 + -1) == '\n')) {
        if (lVar10 + -1 == 0) {
          lVar11 = 0;
        }
        else {
          lVar11 = lVar10 + -1;
          if (*(char *)(lVar1 + -2) == '\r') {
            lVar11 = lVar10 + -2;
          }
        }
      }
      local_c0 = lVar3;
      auVar13 = ::core::str::<impl_str>::trim_matches(lVar2,lVar11);
      uVar12 = auVar13._8_8_;
      plVar8 = auVar13._0_8_;
      if (uVar12 != 0) {
        if ((char)*plVar8 == '[') {
          if (uVar12 == 10) {
            bVar7 = *plVar8 == 0x657275746165665b && (short)plVar8[1] == 0x5d73;
          }
          else {
            bVar7 = false;
          }
        }
        else if (((bool)(bVar7 & 0xf < uVar12)) && (*plVar8 == 0x65675f6567616d69 && plVar8[1] == 0x6e6f69746172656e)) {
          auVar13 = ::core::str::<impl_str>::trim_start_matches(plVar8 + 2,uVar12 - 0x10);
          if ((auVar13._8_8_ != 0) && (*auVar13._0_8_ == '=')) {
            auVar13 = ::core::str::<impl_str>::trim_matches(auVar13._0_8_ + 1,auVar13._8_8_ + -1);
            piVar9 = auVar13._0_8_;
            if (((piVar9 != (int *)0x0) && (auVar13._8_8_ == 5)) && (*piVar9 == 0x736c6166 && (char)piVar9[1] == 'e')) {
              if (lVar4 == 0) {
                return 1;
              }
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar5,lVar4,1);
              return 1;
            }
          }
        }
      }
    } while ((local_80 & 0x100) == 0);
    if (lVar4 != 0) {
      __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar5,lVar4,1);
    }
  }
  return 0;
}

