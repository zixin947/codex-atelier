
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

0000000100372660 <__ZN13codexmate_lib4core5relay16codex_diagnostic18check_config_stale17h8327367d50e67bb2E>:
100372660: d10683ff    	sub	sp, sp, #0x1a0
100372664: a9146ffc    	stp	x28, x27, [sp, #0x140]
100372668: a91567fa    	stp	x26, x25, [sp, #0x150]
10037266c: a9165ff8    	stp	x24, x23, [sp, #0x160]
100372670: a91757f6    	stp	x22, x21, [sp, #0x170]
100372674: a9184ff4    	stp	x20, x19, [sp, #0x180]
100372678: a9197bfd    	stp	x29, x30, [sp, #0x190]
10037267c: 910643fd    	add	x29, sp, #0x190
100372680: aa0003f3    	mov	x19, x0
100372684: b4000342    	cbz	x2, 0x1003726ec <__ZN13codexmate_lib4core5relay16codex_diagnostic18check_config_stale17h8327367d50e67bb2E+0x8c>
100372688: aa0403f6    	mov	x22, x4
10037268c: aa0103f5    	mov	x21, x1
100372690: 3945b094    	ldrb	w20, [x4, #0x16c]
100372694: 910003e8    	mov	x8, sp
100372698: aa0103e0    	mov	x0, x1
10037269c: aa0203e1    	mov	x1, x2
1003726a0: aa0303e2    	mov	x2, x3
1003726a4: 97ffbf58    	bl	0x100362404 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE>
1003726a8: 3945fac8    	ldrb	w8, [x22, #0x17e]
1003726ac: a94086c0    	ldp	x0, x1, [x22, #0x8]
1003726b0: a9420ec2    	ldp	x2, x3, [x22, #0x20]
1003726b4: 360003a8    	tbz	w8, #0x0, 0x100372728 <__ZN13codexmate_lib4core5relay16codex_diagnostic18check_config_stale17h8327367d50e67bb2E+0xc8>
1003726b8: a948dad7    	ldp	x23, x22, [x22, #0x88]
1003726bc: d10283a8    	sub	x8, x29, #0xa0
1003726c0: 9410bc43    	bl	0x1007a17cc <__ZN13codexmate_lib4core5relay6models30active_codex_relay_model_slugs17h72c809e2cf5f3b42E>
1003726c4: 52800908    	mov	w8, #0x48               ; =72
1003726c8: 9b085ec1    	madd	x1, x22, x8, x23
1003726cc: d10283a2    	sub	x2, x29, #0xa0
1003726d0: aa1703e0    	mov	x0, x23
1003726d4: 9412464b    	bl	0x100804000 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE>
1003726d8: ad7b07a0    	ldp	q0, q1, [x29, #-0xa0]
1003726dc: ad0407e0    	stp	q0, q1, [sp, #0x80]
1003726e0: 3cd803a0    	ldur	q0, [x29, #-0x80]
1003726e4: 3d802be0    	str	q0, [sp, #0xa0]
1003726e8: 14000012    	b	0x100372730 <__ZN13codexmate_lib4core5relay16codex_diagnostic18check_config_stale17h8327367d50e67bb2E+0xd0>
1003726ec: d0007cc1    	adrp	x1, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
1003726f0: 91287421    	add	x1, x1, #0xa1d
1003726f4: d0007cc3    	adrp	x3, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
1003726f8: 91310863    	add	x3, x3, #0xc42
1003726fc: aa1303e0    	mov	x0, x19
