
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001001f9a64 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E>:
1001f9a64: d10243ff    	sub	sp, sp, #0x90
1001f9a68: a90657f6    	stp	x22, x21, [sp, #0x60]
1001f9a6c: a9074ff4    	stp	x20, x19, [sp, #0x70]
1001f9a70: a9087bfd    	stp	x29, x30, [sp, #0x80]
1001f9a74: 910203fd    	add	x29, sp, #0x80
1001f9a78: aa0003f3    	mov	x19, x0
1001f9a7c: 910003e0    	mov	x0, sp
1001f9a80: 94000052    	bl	0x1001f9bc8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE>
1001f9a84: f9412660    	ldr	x0, [x19, #0x248]
1001f9a88: f9412a61    	ldr	x1, [x19, #0x250]
1001f9a8c: d0008882    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1001f9a90: 912ca842    	add	x2, x2, #0xb2a
1001f9a94: d100e3a8    	sub	x8, x29, #0x38
1001f9a98: 52800323    	mov	w3, #0x19               ; =25
1001f9a9c: 943d55f1    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
1001f9aa0: a97d07b3    	ldp	x19, x1, [x29, #-0x30]
1001f9aa4: 9100c3e8    	add	x8, sp, #0x30
1001f9aa8: aa1303e0    	mov	x0, x19
1001f9aac: 943dfdcd    	bl	0x1011791e0 <__ZN5alloc6string6String15from_utf8_lossy17h03e7be16830322ceE>
1001f9ab0: a9438bf5    	ldp	x21, x2, [sp, #0x38]
1001f9ab4: 910063e0    	add	x0, sp, #0x18
1001f9ab8: aa1503e1    	mov	x1, x21
1001f9abc: 94000043    	bl	0x1001f9bc8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE>
1001f9ac0: f9401be1    	ldr	x1, [sp, #0x30]
1001f9ac4: f240f83f    	tst	x1, #0x7fffffffffffffff
1001f9ac8: 54000461    	b.ne	0x1001f9b54 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0xf0>
1001f9acc: f85c83a1    	ldur	x1, [x29, #-0x38]
1001f9ad0: b4000081    	cbz	x1, 0x1001f9ae0 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0x7c>
1001f9ad4: aa1303e0    	mov	x0, x19
1001f9ad8: 52800022    	mov	w2, #0x1                ; =1
1001f9adc: 97f81ea3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f9ae0: f9400be2    	ldr	x2, [sp, #0x10]
1001f9ae4: f94017e8    	ldr	x8, [sp, #0x28]
1001f9ae8: eb08005f    	cmp	x2, x8
1001f9aec: 54000121    	b.ne	0x1001f9b10 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0xac>
1001f9af0: f94013e1    	ldr	x1, [sp, #0x20]
1001f9af4: f94007e0    	ldr	x0, [sp, #0x8]
1001f9af8: 943f5eea    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1001f9afc: 7100001f    	cmp	w0, #0x0
1001f9b00: 1a9f17f3    	cset	w19, eq
1001f9b04: f9400fe1    	ldr	x1, [sp, #0x18]
1001f9b08: b50000a1    	cbnz	x1, 0x1001f9b1c <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0xb8>
1001f9b0c: 14000007    	b	0x1001f9b28 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0xc4>
1001f9b10: 52800013    	mov	w19, #0x0               ; =0
1001f9b14: f9400fe1    	ldr	x1, [sp, #0x18]
1001f9b18: b4000081    	cbz	x1, 0x1001f9b28 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0xc4>
1001f9b1c: f94013e0    	ldr	x0, [sp, #0x20]
1001f9b20: 52800022    	mov	w2, #0x1                ; =1
1001f9b24: 97f81e91    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f9b28: f94003e1    	ldr	x1, [sp]
1001f9b2c: b4000081    	cbz	x1, 0x1001f9b3c <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0xd8>
1001f9b30: f94007e0    	ldr	x0, [sp, #0x8]
1001f9b34: 52800022    	mov	w2, #0x1                ; =1
1001f9b38: 97f81e8c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f9b3c: aa1303e0    	mov	x0, x19
1001f9b40: a9487bfd    	ldp	x29, x30, [sp, #0x80]
1001f9b44: a9474ff4    	ldp	x20, x19, [sp, #0x70]
1001f9b48: a94657f6    	ldp	x22, x21, [sp, #0x60]
1001f9b4c: 910243ff    	add	sp, sp, #0x90
1001f9b50: d65f03c0    	ret
1001f9b54: aa1503e0    	mov	x0, x21
1001f9b58: 52800022    	mov	w2, #0x1                ; =1
1001f9b5c: 97f81e83    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f9b60: f85c83a1    	ldur	x1, [x29, #-0x38]
1001f9b64: b5fffb81    	cbnz	x1, 0x1001f9ad4 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0x70>
1001f9b68: 17ffffde    	b	0x1001f9ae0 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0x7c>
1001f9b6c: aa0003f4    	mov	x20, x0
1001f9b70: f9401be1    	ldr	x1, [sp, #0x30]
1001f9b74: f240f83f    	tst	x1, #0x7fffffffffffffff
1001f9b78: 540000c0    	b.eq	0x1001f9b90 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0x12c>
1001f9b7c: aa1503e0    	mov	x0, x21
1001f9b80: 52800022    	mov	w2, #0x1                ; =1
1001f9b84: 97f81e79    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f9b88: 14000002    	b	0x1001f9b90 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0x12c>
1001f9b8c: aa0003f4    	mov	x20, x0
1001f9b90: f85c83a1    	ldur	x1, [x29, #-0x38]
1001f9b94: b40000c1    	cbz	x1, 0x1001f9bac <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0x148>
1001f9b98: aa1303e0    	mov	x0, x19
1001f9b9c: 52800022    	mov	w2, #0x1                ; =1
1001f9ba0: 97f81e72    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f9ba4: 14000002    	b	0x1001f9bac <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0x148>
1001f9ba8: aa0003f4    	mov	x20, x0
1001f9bac: f94003e1    	ldr	x1, [sp]
1001f9bb0: b4000081    	cbz	x1, 0x1001f9bc0 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E+0x15c>
1001f9bb4: f94007e0    	ldr	x0, [sp, #0x8]
1001f9bb8: 52800022    	mov	w2, #0x1                ; =1
1001f9bbc: 97f81e6b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f9bc0: aa1403e0    	mov	x0, x20
1001f9bc4: 943f5d64    	bl	0x1011d1154 <_writev+0x1011d1154>
