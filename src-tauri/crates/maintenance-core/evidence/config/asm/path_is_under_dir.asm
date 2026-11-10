
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001003724b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E>:
1003724b8: d10243ff    	sub	sp, sp, #0x90
1003724bc: a90657f6    	stp	x22, x21, [sp, #0x60]
1003724c0: a9074ff4    	stp	x20, x19, [sp, #0x70]
1003724c4: a9087bfd    	stp	x29, x30, [sp, #0x80]
1003724c8: 910203fd    	add	x29, sp, #0x80
1003724cc: aa0303f3    	mov	x19, x3
1003724d0: aa0203f4    	mov	x20, x2
1003724d4: aa0103e2    	mov	x2, x1
1003724d8: aa0003e1    	mov	x1, x0
1003724dc: 910023e0    	add	x0, sp, #0x8
1003724e0: 94005a9c    	bl	0x100388f50 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E>
1003724e4: 9100e3e8    	add	x8, sp, #0x38
1003724e8: aa1403e0    	mov	x0, x20
1003724ec: aa1303e1    	mov	x1, x19
1003724f0: 94381b3c    	bl	0x1011791e0 <__ZN5alloc6string6String15from_utf8_lossy17h03e7be16830322ceE>
1003724f4: a9440bf4    	ldp	x20, x2, [sp, #0x40]
1003724f8: 910083e0    	add	x0, sp, #0x20
1003724fc: aa1403e1    	mov	x1, x20
100372500: 94005a94    	bl	0x100388f50 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E>
100372504: f9401fe1    	ldr	x1, [sp, #0x38]
100372508: f240f83f    	tst	x1, #0x7fffffffffffffff
10037250c: 540000c1    	b.ne	0x100372524 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x6c>
100372510: a94153f3    	ldp	x19, x20, [sp, #0x10]
100372514: f9401be8    	ldr	x8, [sp, #0x30]
100372518: eb08029f    	cmp	x20, x8
10037251c: 54000120    	b.eq	0x100372540 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x88>
100372520: 1400000d    	b	0x100372554 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x9c>
100372524: aa1403e0    	mov	x0, x20
100372528: 52800022    	mov	w2, #0x1                ; =1
10037252c: 97f23c0f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100372530: a94153f3    	ldp	x19, x20, [sp, #0x10]
100372534: f9401be8    	ldr	x8, [sp, #0x30]
100372538: eb08029f    	cmp	x20, x8
10037253c: 540000c1    	b.ne	0x100372554 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x9c>
100372540: f94017e1    	ldr	x1, [sp, #0x28]
100372544: aa1303e0    	mov	x0, x19
100372548: aa1403e2    	mov	x2, x20
10037254c: 94397c55    	bl	0x1011d16a0 <_writev+0x1011d16a0>
100372550: 34000560    	cbz	w0, 0x1003725fc <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x144>
100372554: 910083e8    	add	x8, sp, #0x20
100372558: f00003c9    	adrp	x9, 0x1003ed000 <__ZN5alloc11collections5btree3set21BTreeSet$LT$T$C$A$GT$10difference17h151923f22deefe03E+0x15c>
10037255c: 9133e129    	add	x9, x9, #0xcf8
100372560: a93d27a8    	stp	x8, x9, [x29, #-0x30]
100372564: d0008d60    	adrp	x0, 0x101520000 <_anon.164d03a2dfc98f969363eaf1c50504fd.175+0x1a1>
100372568: 9104b800    	add	x0, x0, #0x12e
10037256c: 9100e3e8    	add	x8, sp, #0x38
100372570: d100c3a1    	sub	x1, x29, #0x30
100372574: 943814c8    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100372578: a943dbf5    	ldp	x21, x22, [sp, #0x38]
10037257c: f94027e2    	ldr	x2, [sp, #0x48]
100372580: eb02029f    	cmp	x20, x2
100372584: 54000082    	b.hs	0x100372594 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0xdc>
100372588: 52800014    	mov	w20, #0x0               ; =0
10037258c: b5000115    	cbnz	x21, 0x1003725ac <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0xf4>
100372590: 1400000b    	b	0x1003725bc <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x104>
100372594: aa1603e0    	mov	x0, x22
100372598: aa1303e1    	mov	x1, x19
10037259c: 94397c41    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1003725a0: 7100001f    	cmp	w0, #0x0
1003725a4: 1a9f17f4    	cset	w20, eq
1003725a8: b40000b5    	cbz	x21, 0x1003725bc <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x104>
1003725ac: aa1603e0    	mov	x0, x22
1003725b0: aa1503e1    	mov	x1, x21
1003725b4: 52800022    	mov	w2, #0x1                ; =1
1003725b8: 97f23bec    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003725bc: f94013e1    	ldr	x1, [sp, #0x20]
1003725c0: b4000081    	cbz	x1, 0x1003725d0 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x118>
1003725c4: f94017e0    	ldr	x0, [sp, #0x28]
1003725c8: 52800022    	mov	w2, #0x1                ; =1
1003725cc: 97f23be7    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003725d0: f94007e1    	ldr	x1, [sp, #0x8]
1003725d4: b4000081    	cbz	x1, 0x1003725e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x12c>
1003725d8: aa1303e0    	mov	x0, x19
1003725dc: 52800022    	mov	w2, #0x1                ; =1
1003725e0: 97f23be2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003725e4: aa1403e0    	mov	x0, x20
1003725e8: a9487bfd    	ldp	x29, x30, [sp, #0x80]
1003725ec: a9474ff4    	ldp	x20, x19, [sp, #0x70]
1003725f0: a94657f6    	ldp	x22, x21, [sp, #0x60]
1003725f4: 910243ff    	add	sp, sp, #0x90
1003725f8: d65f03c0    	ret
1003725fc: 52800034    	mov	w20, #0x1               ; =1
100372600: f94013e1    	ldr	x1, [sp, #0x20]
100372604: b5fffe01    	cbnz	x1, 0x1003725c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x10c>
100372608: 17fffff2    	b	0x1003725d0 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x118>
10037260c: aa0003f3    	mov	x19, x0
100372610: f94013e1    	ldr	x1, [sp, #0x20]
100372614: b4000181    	cbz	x1, 0x100372644 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x18c>
100372618: f94017e0    	ldr	x0, [sp, #0x28]
10037261c: 14000006    	b	0x100372634 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x17c>
100372620: aa0003f3    	mov	x19, x0
100372624: f9401fe1    	ldr	x1, [sp, #0x38]
100372628: f240f83f    	tst	x1, #0x7fffffffffffffff
10037262c: 540000c0    	b.eq	0x100372644 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x18c>
100372630: aa1403e0    	mov	x0, x20
100372634: 52800022    	mov	w2, #0x1                ; =1
100372638: 97f23bcc    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10037263c: 14000002    	b	0x100372644 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x18c>
100372640: aa0003f3    	mov	x19, x0
100372644: f94007e1    	ldr	x1, [sp, #0x8]
100372648: b4000081    	cbz	x1, 0x100372658 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E+0x1a0>
10037264c: f9400be0    	ldr	x0, [sp, #0x10]
100372650: 52800022    	mov	w2, #0x1                ; =1
100372654: 97f23bc5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100372658: aa1303e0    	mov	x0, x19
10037265c: 94397abe    	bl	0x1011d1154 <_writev+0x1011d1154>
