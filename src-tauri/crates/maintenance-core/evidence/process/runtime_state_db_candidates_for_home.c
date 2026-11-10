/* Static Ghidra export; input AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de */
/* runtime_state_db_candidates_for_home @ 1008ef428 requested 1008ef428 */

/* codexmate_lib::platform::paths::CodexPaths::runtime_state_db_candidates_for_home */

void codexmate_lib::platform::paths::CodexPaths::runtime_state_db_candidates_for_home(ulong *param_1)

{
  long *plVar1;
  long lVar2;
  byte bVar3;
  uint uVar4;
  long *plVar5;
  long ***ppplVar6;
  long lVar7;
  void *pvVar8;
  int *piVar9;
  void *pvVar10;
  int *piVar11;
  long **pplVar12;
  int iVar13;
  ulong uVar14;
  long lVar15;
  char cVar16;
  byte *pbVar17;
  long *plVar18;
  uint uVar19;
  long *plVar20;
  long lVar21;
  ulong uVar22;
  undefined8 uVar23;
  undefined8 *puVar24;
  long **local_1290;
  long **pplStack_1288;
  ulong local_1280;
  ulong uStack_1278;
  ulong local_1270;
  long *local_1260;
  long **local_1258;
  long *plStack_1250;
  long ***local_1248;
  char local_1240;
  undefined7 uStack_123f;
  long local_1238;
  undefined8 local_1060;
  undefined8 local_e20;
  long **local_9f8;
  char local_9f0;
  long local_9e8;
  long local_9e0;
  long local_9d8;
  void *local_9d0;
  int *local_9c8;
  char local_9c0;
  undefined7 uStack_9bf;
  undefined1 uStack_9b8;
  undefined8 uStack_9b7;
  undefined8 local_9af;
  undefined2 local_998;
  undefined1 local_996;
  long **local_5c0;
  long **pplStack_5b8;
  long **pplStack_5b0;
  undefined1 auStack_5a8 [1048];
  void *local_190;
  int *local_188;
  char local_180;
  undefined8 local_17f;
  undefined8 uStack_177;
  undefined8 local_16f;
  undefined2 local_158;
  undefined1 local_156;
  void *local_150;
  int *local_148;
  char local_140;
  undefined8 local_13f;
  undefined8 uStack_137;
  undefined8 local_12f;
  undefined2 local_118;
  undefined1 local_116;
  void *local_110;
  int *piStack_108;
  char local_100;
  undefined8 local_ff;
  undefined8 uStack_f7;
  undefined8 local_ef;
  undefined2 local_d8;
  undefined1 local_d6;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  
  local_1060 = 0;
  local_1280 = 0;
  uStack_1278 = 8;
  local_1270 = 0;
  state_db_search_dirs(&local_1248);
  ppplVar6 = local_1248;
  plVar5 = (long *)CONCAT71(uStack_123f,local_1240);
  plVar1 = plVar5 + local_1238 * 4;
  local_1258 = (long **)local_1248;
  plVar20 = plVar5;
  plVar18 = plVar5;
  plStack_1250 = plVar1;
  if (local_1238 == 0) {
LAB_1008ef97c:
    local_1260 = plVar20;
    if ((long)plVar1 - (long)plVar20 != 0) {
      uVar14 = (ulong)((long)plVar1 - (long)plVar20) >> 5;
      plVar20 = plVar20 + 1;
      do {
        if (plVar20[-1] != 0) {
          __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(*plVar20,plVar20[-1],1);
        }
        plVar20 = plVar20 + 4;
        uVar14 = uVar14 - 1;
      } while (uVar14 != 0);
    }
  }
  else {
    do {
      plVar20 = plVar18 + 4;
      lVar21 = *plVar18;
      if (lVar21 == -0x8000000000000000) goto LAB_1008ef97c;
      lVar2 = plVar18[1];
      local_1260 = plVar20;
      std::sys::fs::read_dir(&local_1248,lVar2,plVar18[2]);
      local_5c0 = (long **)local_1248;
      pplStack_5b8 = (long **)CONCAT71(pplStack_5b8._1_7_,local_1240);
      cVar16 = local_1240;
      if (lVar21 != 0) {
        __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar2,lVar21,1);
        cVar16 = (char)pplStack_5b8;
      }
      if (cVar16 == '\x02') {
        ::core::ptr::drop_in_place<core::result::Result<std::fs::ReadDir,std::io::error::Error>>(&local_5c0);
      }
      else {
        local_1248 = (long ***)0x0;
        local_e20 = 0;
        local_9f8 = local_5c0;
        local_9f0 = cVar16;
        do {
          <std::fs::ReadDir_as_core::iter::traits::iterator::Iterator>::next(&local_5c0,&local_9f8);
          if ((long ***)local_5c0 == (long ***)0x0) {
            if (local_9f0 != '\x02') {
              pplVar12 = (long **)*local_9f8;
              *local_9f8 = (long *)((long)pplVar12 + -1);
              LORelease();
              if (pplVar12 == (long **)0x1) {
                DataMemoryBarrier(2,1);
                alloc::sync::Arc<T,A>::drop_slow(&local_9f8);
              }
            }
            pplVar12 = local_5c0;
            local_9f0 = '\x02';
            pplStack_1288 = pplStack_5b0;
            local_1290 = pplStack_5b8;
            _memcpy(&local_9d0,auStack_5a8,0x410);
            if (((ulong)pplVar12 & 1) == 0) break;
          }
          else {
            pplStack_1288 = pplStack_5b0;
            local_1290 = pplStack_5b8;
            _memcpy(&local_9d0,auStack_5a8,0x410);
          }
          if ((long ***)local_1290 == (long ***)0x0) {
            if (((ulong)pplStack_1288 & 3) == 1) {
              uVar23 = *(undefined8 *)((long)pplStack_1288 + -1);
              puVar24 = *(undefined8 **)((long)pplStack_1288 + 7);
              if ((code *)*puVar24 != (code *)0x0) {
                (*(code *)*puVar24)(uVar23);
              }
              if (puVar24[1] != 0) {
                __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(uVar23,puVar24[1],puVar24[2]);
              }
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc((undefined8 *)((long)pplStack_1288 + -1),0x18,8);
            }
          }
          else {
            _memcpy(&pplStack_5b0,&local_9d0,0x410);
            pplStack_5b8 = pplStack_1288;
            local_5c0 = local_1290;
            std::fs::DirEntry::path(&local_9e8,&local_5c0);
            lVar2 = local_9d8;
            lVar21 = local_9e0;
            uVar14 = std::path::Path::is_file(local_9e0,local_9d8);
            if (((uVar14 & 1) != 0) && (lVar15 = std::path::Path::file_name(lVar21,lVar2), lVar15 != 0)) {
              ::core::str::converts::from_utf8(&local_9d0);
              if ((int)local_9d0 != 1) {
                uVar14 = CONCAT71(uStack_9bf,local_9c0);
                if ((((5 < uVar14) && (*local_9c8 == 0x74617473 && (short)local_9c8[1] == 0x5f65)) && (6 < uVar14 - 6)) &&
                   ((pbVar17 = (byte *)((long)local_9c8 + 6),
                    *(int *)(pbVar17 + (uVar14 - 0xd)) == 0x6c71732e && *(int *)(pbVar17 + (uVar14 - 10)) == 0x6574696c && (uVar14 - 0xd != 0)))) {
                  if (uVar14 == 0xe) {
                    bVar3 = *pbVar17;
                    if ((bVar3 == 0x2b) || (bVar3 == 0x2d)) goto LAB_1008ef6d4;
                  }
                  else {
                    bVar3 = *pbVar17;
                  }
                  if (bVar3 == 0x2b) {
                    pbVar17 = (byte *)((long)local_9c8 + 7);
                  }
                  uVar14 = (uVar14 - 0xd) - (ulong)(bVar3 == 0x2b);
                  if (uVar14 < 9) {
                    for (; uVar14 != 0; uVar14 = uVar14 - 1) {
                      if (9 < *pbVar17 - 0x30) goto LAB_1008ef6d4;
                      pbVar17 = pbVar17 + 1;
                    }
LAB_1008ef768:
                    lVar15 = local_1270 * 0x18;
                    uVar14 = uStack_1278;
                    do {
                      lVar7 = local_9e8;
                      uVar22 = local_1270;
                      if (lVar15 == 0) {
                        if (local_1270 == local_1280) {
                          alloc::raw_vec::RawVec<T,A>::grow_one(&local_1280);
                        }
                        plVar18 = (long *)(uStack_1278 + uVar22 * 0x18);
                        *plVar18 = lVar7;
                        plVar18[1] = lVar21;
                        plVar18[2] = lVar2;
                        local_1270 = uVar22 + 1;
                        pplVar12 = (long **)*local_5c0;
                        *local_5c0 = (long *)((long)pplVar12 + -1);
                        LORelease();
                        if (pplVar12 != (long **)0x1) goto LAB_1008ef538;
                        goto LAB_1008ef6f8;
                      }
                      std::path::Path::components(&local_190,*(undefined8 *)(uVar14 + 8),*(undefined8 *)(uVar14 + 0x10));
                      std::path::Path::components(&local_150,lVar21,lVar2);
                      piVar11 = local_148;
                      pvVar10 = local_150;
                      piVar9 = local_188;
                      pvVar8 = local_190;
                      if (local_188 == local_148) {
                        if (((((char)local_158 == (char)local_118) && (local_158._1_1_ == '\x02')) && (local_118._1_1_ == '\x02')) &&
                           (iVar13 = _memcmp(local_190,local_150,(size_t)local_188), iVar13 == 0)) break;
                      }
                      if (local_180 != '\x06') {
                        uStack_c8 = uStack_177;
                        local_d0 = local_17f;
                        uStack_c0 = local_16f;
                      }
                      uStack_f7 = uStack_c8;
                      local_ff = local_d0;
                      local_110 = pvVar8;
                      piStack_108 = piVar9;
                      local_100 = local_180;
                      local_ef = uStack_c0;
                      local_d8 = local_158;
                      local_d6 = local_156;
                      if (local_140 != '\x06') {
                        uStack_98 = uStack_137;
                        local_a0 = local_13f;
                        uStack_90 = local_12f;
                      }
                      local_9d0 = pvVar10;
                      local_9c8 = piVar11;
                      local_9c0 = local_140;
                      uStack_9b7 = uStack_98;
                      uStack_9bf = (undefined7)local_a0;
                      uStack_9b8 = (undefined1)((ulong)local_a0 >> 0x38);
                      local_9af = uStack_90;
                      local_998 = local_118;
                      local_996 = local_116;
                      uVar22 = ::core::iter::traits::iterator::Iterator::eq_by(&local_110,&local_9d0);
                      uVar14 = uVar14 + 0x18;
                      lVar15 = lVar15 + -0x18;
                    } while ((uVar22 & 1) == 0);
                  }
                  else {
                    uVar22 = 0;
                    while( true ) {
                      if (uVar14 == 0) goto LAB_1008ef768;
                      if ((uVar22 * 10 & 0xffffffff00000000) != 0) break;
                      uVar4 = *pbVar17 - 0x30;
                      uVar19 = (uint)(uVar22 * 10);
                      uVar22 = (ulong)(uVar4 + uVar19);
                      if ((9 < uVar4) || (uVar14 = uVar14 - 1, pbVar17 = pbVar17 + 1, CARRY4(uVar4,uVar19))) break;
                    }
                  }
                }
              }
            }
LAB_1008ef6d4:
            if (local_9e8 != 0) {
              __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(lVar21,local_9e8,1);
            }
            pplVar12 = (long **)*local_5c0;
            *local_5c0 = (long *)((long)pplVar12 + -1);
            LORelease();
            if (pplVar12 == (long **)0x1) {
LAB_1008ef6f8:
              DataMemoryBarrier(2,1);
              alloc::sync::Arc<T,A>::drop_slow(&local_5c0);
            }
          }
LAB_1008ef538:
        } while (local_9f0 != '\x02');
        ::core::ptr::drop_in_place<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>>(&local_1248);
      }
      plVar18 = plVar20;
    } while (plVar20 != plVar1);
  }
  if (ppplVar6 != (long ***)0x0) {
    __RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc(plVar5,(long)ppplVar6 << 5,8);
  }
  uVar14 = uStack_1278;
  local_1248 = &local_5c0;
  if (1 < local_1270) {
    if (local_1270 < 0x15) {
      lVar21 = local_1270 * 0x18 + -0x18;
      uVar22 = uStack_1278;
      do {
        uVar22 = uVar22 + 0x18;
        ::core::slice::sort::shared::smallsort::insert_tail(uVar14,uVar22);
        lVar21 = lVar21 + -0x18;
      } while (lVar21 != 0);
    }
    else {
      ::core::slice::sort::stable::driftsort_main(uStack_1278,local_1270,&local_1248);
    }
  }
  param_1[1] = uStack_1278;
  *param_1 = local_1280;
  param_1[2] = local_1270;
  return;
}

