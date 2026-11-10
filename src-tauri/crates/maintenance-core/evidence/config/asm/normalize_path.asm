
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100388f50 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E>:
100388f50: d10343ff    	sub	sp, sp, #0xd0
100388f54: a90867fa    	stp	x26, x25, [sp, #0x80]
100388f58: a9095ff8    	stp	x24, x23, [sp, #0x90]
100388f5c: a90a57f6    	stp	x22, x21, [sp, #0xa0]
100388f60: a90b4ff4    	stp	x20, x19, [sp, #0xb0]
100388f64: a90c7bfd    	stp	x29, x30, [sp, #0xc0]
100388f68: 910303fd    	add	x29, sp, #0xc0
100388f6c: aa0203f6    	mov	x22, x2
100388f70: b6f800a2    	tbz	x2, #0x3f, 0x100388f84 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x34>
100388f74: d2800015    	mov	x21, #0x0               ; =0
100388f78: aa1503e0    	mov	x0, x21
100388f7c: aa1603e1    	mov	x1, x22
100388f80: 94391db9    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100388f84: aa0003f4    	mov	x20, x0
100388f88: b40001d6    	cbz	x22, 0x100388fc0 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x70>
100388f8c: aa0103f7    	mov	x23, x1
100388f90: 97f1e179    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100388f94: 52800035    	mov	w21, #0x1               ; =1
100388f98: aa1603e0    	mov	x0, x22
100388f9c: 52800021    	mov	w1, #0x1                ; =1
100388fa0: 97f1e171    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100388fa4: b4fffea0    	cbz	x0, 0x100388f78 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x28>
100388fa8: aa0003f3    	mov	x19, x0
100388fac: f1001edf    	cmp	x22, #0x7
100388fb0: aa1703e8    	mov	x8, x23
100388fb4: 540000a8    	b.hi	0x100388fc8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x78>
100388fb8: d2800009    	mov	x9, #0x0                ; =0
100388fbc: 1400002c    	b	0x10038906c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x11c>
100388fc0: 52800033    	mov	w19, #0x1               ; =1
100388fc4: 14000032    	b	0x10038908c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x13c>
100388fc8: f10102df    	cmp	x22, #0x40
100388fcc: 54000062    	b.hs	0x100388fd8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x88>
100388fd0: d2800009    	mov	x9, #0x0                ; =0
100388fd4: 14000019    	b	0x100389038 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0xe8>
100388fd8: 927ae2c9    	and	x9, x22, #0x7fffffffffffffc0
100388fdc: 9100826a    	add	x10, x19, #0x20
100388fe0: 9100810b    	add	x11, x8, #0x20
100388fe4: 4f02e780    	movi.16b	v0, #0x5c
100388fe8: 4f01e5e1    	movi.16b	v1, #0x2f
100388fec: aa0903ec    	mov	x12, x9
100388ff0: ad7f0d62    	ldp	q2, q3, [x11, #-0x20]
100388ff4: acc21564    	ldp	q4, q5, [x11], #0x40
100388ff8: 6e208c46    	cmeq.16b	v6, v2, v0
100388ffc: 6e208c67    	cmeq.16b	v7, v3, v0
100389000: 6e208c90    	cmeq.16b	v16, v4, v0
100389004: 6e208cb1    	cmeq.16b	v17, v5, v0
100389008: 6ea61c22    	bit.16b	v2, v1, v6
10038900c: 6ea71c23    	bit.16b	v3, v1, v7
100389010: 6eb01c24    	bit.16b	v4, v1, v16
100389014: 6eb11c25    	bit.16b	v5, v1, v17
100389018: ad3f0d42    	stp	q2, q3, [x10, #-0x20]
10038901c: ac821544    	stp	q4, q5, [x10], #0x40
100389020: f101018c    	subs	x12, x12, #0x40
100389024: 54fffe61    	b.ne	0x100388ff0 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0xa0>
100389028: eb0902df    	cmp	x22, x9
10038902c: 54000300    	b.eq	0x10038908c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x13c>
100389030: f27d0adf    	tst	x22, #0x38
100389034: 540001c0    	b.eq	0x10038906c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x11c>
100389038: aa0903ea    	mov	x10, x9
10038903c: 927deec9    	and	x9, x22, #0x7ffffffffffffff8
100389040: 0f02e780    	movi.8b	v0, #0x5c
100389044: 0f01e5e1    	movi.8b	v1, #0x2f
100389048: fc6a6902    	ldr	d2, [x8, x10]
10038904c: 2e208c43    	cmeq.8b	v3, v2, v0
100389050: 2ea31c22    	bit.8b	v2, v1, v3
100389054: fc2a6a62    	str	d2, [x19, x10]
100389058: 9100214a    	add	x10, x10, #0x8
10038905c: eb0a013f    	cmp	x9, x10
100389060: 54ffff41    	b.ne	0x100389048 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0xf8>
100389064: eb0902df    	cmp	x22, x9
100389068: 54000120    	b.eq	0x10038908c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x13c>
10038906c: 528005ea    	mov	w10, #0x2f              ; =47
100389070: 3869690b    	ldrb	w11, [x8, x9]
100389074: 7101717f    	cmp	w11, #0x5c
100389078: 1a8b014b    	csel	w11, w10, w11, eq
10038907c: 38296a6b    	strb	w11, [x19, x9]
100389080: 91000529    	add	x9, x9, #0x1
100389084: eb0902df    	cmp	x22, x9
100389088: 54ffff41    	b.ne	0x100389070 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x120>
10038908c: b0007c37    	adrp	x23, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389090: 9119e6f7    	add	x23, x23, #0x679
100389094: 5285e5f9    	mov	w25, #0x2f2f            ; =12079
100389098: aa1603f5    	mov	x21, x22
10038909c: b0007c38    	adrp	x24, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
1003890a0: 9119e318    	add	x24, x24, #0x678
1003890a4: 14000003    	b	0x1003890b0 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x160>
1003890a8: a941cff5    	ldp	x21, x19, [sp, #0x18]
1003890ac: f94017f6    	ldr	x22, [sp, #0x28]
1003890b0: f1000edf    	cmp	x22, #0x3
1003890b4: 540001a3    	b.lo	0x1003890e8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x198>
1003890b8: 910063e8    	add	x8, sp, #0x18
1003890bc: aa1303e0    	mov	x0, x19
1003890c0: aa1603e1    	mov	x1, x22
1003890c4: aa1703e2    	mov	x2, x23
1003890c8: 52800043    	mov	w3, #0x2                ; =2
1003890cc: 9437f1fb    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
1003890d0: 910003e0    	mov	x0, sp
1003890d4: 910063e1    	add	x1, sp, #0x18
1003890d8: 9401959f    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
1003890dc: b94003e8    	ldr	w8, [sp]
1003890e0: 370000e8    	tbnz	w8, #0x0, 0x1003890fc <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x1ac>
1003890e4: 14000012    	b	0x10038912c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x1dc>
1003890e8: f1000adf    	cmp	x22, #0x2
1003890ec: 54000201    	b.ne	0x10038912c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x1dc>
1003890f0: 79400268    	ldrh	w8, [x19]
1003890f4: 6b19011f    	cmp	w8, w25
1003890f8: 540001a1    	b.ne	0x10038912c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x1dc>
1003890fc: 910063e0    	add	x0, sp, #0x18
100389100: aa1303e1    	mov	x1, x19
100389104: aa1603e2    	mov	x2, x22
100389108: aa1703e3    	mov	x3, x23
10038910c: aa1803e4    	mov	x4, x24
100389110: 94019050    	bl	0x1003ed250 <__ZN5alloc3str21_$LT$impl$u20$str$GT$7replace17ha6c8e79c65c048aaE>
100389114: b4fffcb5    	cbz	x21, 0x1003890a8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x158>
100389118: aa1303e0    	mov	x0, x19
10038911c: aa1503e1    	mov	x1, x21
100389120: 52800022    	mov	w2, #0x1                ; =1
100389124: 97f1e111    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100389128: 17ffffe0    	b	0x1003890a8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x158>
10038912c: 52800038    	mov	w24, #0x1               ; =1
100389130: 14000007    	b	0x10038914c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x1fc>
100389134: 1200114a    	and	w10, w10, #0x1f
100389138: 12001508    	and	w8, w8, #0x3f
10038913c: 2a0a1908    	orr	w8, w8, w10, lsl #6
100389140: cb130136    	sub	x22, x9, x19
100389144: 7100bd1f    	cmp	w8, #0x2f
100389148: 54000341    	b.ne	0x1003891b0 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x260>
10038914c: aa1603f7    	mov	x23, x22
100389150: b4000496    	cbz	x22, 0x1003891e0 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x290>
100389154: 8b17026b    	add	x11, x19, x23
100389158: aa0b03e9    	mov	x9, x11
10038915c: 38dffd28    	ldrsb	w8, [x9, #-0x1]!
100389160: 36ffff08    	tbz	w8, #0x1f, 0x100389140 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x1f0>
100389164: aa0b03e9    	mov	x9, x11
100389168: 38dfed2a    	ldrsb	w10, [x9, #-0x2]!
10038916c: 3101015f    	cmn	w10, #0x40
100389170: 54fffe2a    	b.ge	0x100389134 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x1e4>
100389174: aa0b03e9    	mov	x9, x11
100389178: 38dfdd2c    	ldrsb	w12, [x9, #-0x3]!
10038917c: 3101019f    	cmn	w12, #0x40
100389180: 540000ea    	b.ge	0x10038919c <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x24c>
100389184: 12001d89    	and	w9, w12, #0xff
100389188: 385fcd6d    	ldrb	w13, [x11, #-0x4]!
10038918c: 1200152c    	and	w12, w9, #0x3f
100389190: 331a09ac    	bfi	w12, w13, #6, #3
100389194: aa0b03e9    	mov	x9, x11
100389198: 14000002    	b	0x1003891a0 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x250>
10038919c: 12000d8c    	and	w12, w12, #0xf
1003891a0: 12001d4a    	and	w10, w10, #0xff
1003891a4: 1200154a    	and	w10, w10, #0x3f
1003891a8: 2a0c194a    	orr	w10, w10, w12, lsl #6
1003891ac: 17ffffe3    	b	0x100389138 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x1e8>
1003891b0: b6f800b7    	tbz	x23, #0x3f, 0x1003891c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x274>
1003891b4: d2800000    	mov	x0, #0x0                ; =0
1003891b8: aa1703e1    	mov	x1, x23
1003891bc: 94391d2a    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1003891c0: d4200020    	brk	#0x1
1003891c4: 97f1e0ec    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003891c8: aa1703e0    	mov	x0, x23
1003891cc: 52800021    	mov	w1, #0x1                ; =1
1003891d0: 97f1e0e5    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003891d4: aa0003f8    	mov	x24, x0
1003891d8: 52800020    	mov	w0, #0x1                ; =1
1003891dc: b4fffef8    	cbz	x24, 0x1003891b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x268>
1003891e0: aa1803e0    	mov	x0, x24
1003891e4: aa1303e1    	mov	x1, x19
1003891e8: aa1703e2    	mov	x2, x23
1003891ec: 94392130    	bl	0x1011d16ac <_writev+0x1011d16ac>
1003891f0: a9006297    	stp	x23, x24, [x20]
1003891f4: f9000a97    	str	x23, [x20, #0x10]
1003891f8: b4000175    	cbz	x21, 0x100389224 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x2d4>
1003891fc: aa1303e0    	mov	x0, x19
100389200: aa1503e1    	mov	x1, x21
100389204: 52800022    	mov	w2, #0x1                ; =1
100389208: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
10038920c: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
100389210: a94a57f6    	ldp	x22, x21, [sp, #0xa0]
100389214: a9495ff8    	ldp	x24, x23, [sp, #0x90]
100389218: a94867fa    	ldp	x26, x25, [sp, #0x80]
10038921c: 910343ff    	add	sp, sp, #0xd0
100389220: 17f1e0d2    	b	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100389224: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
100389228: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
10038922c: a94a57f6    	ldp	x22, x21, [sp, #0xa0]
100389230: a9495ff8    	ldp	x24, x23, [sp, #0x90]
100389234: a94867fa    	ldp	x26, x25, [sp, #0x80]
100389238: 910343ff    	add	sp, sp, #0xd0
10038923c: d65f03c0    	ret
100389240: 14000001    	b	0x100389244 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x2f4>
100389244: b40000f5    	cbz	x21, 0x100389260 <__ZN13codexmate_lib4core5relay16codex_diagnostic26normalize_path_for_compare17hef46c0edd5e7ff20E+0x310>
100389248: aa0003f4    	mov	x20, x0
10038924c: aa1303e0    	mov	x0, x19
100389250: aa1503e1    	mov	x1, x21
100389254: 52800022    	mov	w2, #0x1                ; =1
100389258: 97f1e0c4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038925c: aa1403e0    	mov	x0, x20
100389260: 94391fbd    	bl	0x1011d1154 <_writev+0x1011d1154>
