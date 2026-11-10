
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010039b494 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E>:
10039b494: d102c3ff    	sub	sp, sp, #0xb0
10039b498: a90857f6    	stp	x22, x21, [sp, #0x80]
10039b49c: a9094ff4    	stp	x20, x19, [sp, #0x90]
10039b4a0: a90a7bfd    	stp	x29, x30, [sp, #0xa0]
10039b4a4: 910283fd    	add	x29, sp, #0xa0
10039b4a8: 97fa4f2c    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
10039b4ac: aa0103f3    	mov	x19, x1
10039b4b0: b6f800a1    	tbz	x1, #0x3f, 0x10039b4c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x30>
10039b4b4: d2800016    	mov	x22, #0x0               ; =0
10039b4b8: aa1603e0    	mov	x0, x22
10039b4bc: aa1303e1    	mov	x1, x19
10039b4c0: 9438d469    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10039b4c4: aa0003f5    	mov	x21, x0
10039b4c8: b40001f3    	cbz	x19, 0x10039b504 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x70>
10039b4cc: 97f1982a    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10039b4d0: 52800036    	mov	w22, #0x1               ; =1
10039b4d4: aa1303e0    	mov	x0, x19
10039b4d8: 52800021    	mov	w1, #0x1                ; =1
10039b4dc: 97f19822    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10039b4e0: b4fffec0    	cbz	x0, 0x10039b4b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x24>
10039b4e4: aa0003f4    	mov	x20, x0
10039b4e8: aa1503e1    	mov	x1, x21
10039b4ec: aa1303e2    	mov	x2, x19
10039b4f0: 9438d86f    	bl	0x1011d16ac <_writev+0x1011d16ac>
10039b4f4: f100227f    	cmp	x19, #0x8
10039b4f8: 54000122    	b.hs	0x10039b51c <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x88>
10039b4fc: d2800008    	mov	x8, #0x0                ; =0
10039b500: 1400003c    	b	0x10039b5f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x15c>
10039b504: 52800020    	mov	w0, #0x1                ; =1
10039b508: aa1503e1    	mov	x1, x21
10039b50c: aa1303e2    	mov	x2, x19
10039b510: 9438d867    	bl	0x1011d16ac <_writev+0x1011d16ac>
10039b514: 52800015    	mov	w21, #0x0               ; =0
10039b518: 140000cf    	b	0x10039b854 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3c0>
10039b51c: f101027f    	cmp	x19, #0x40
10039b520: 54000062    	b.hs	0x10039b52c <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x98>
10039b524: d2800008    	mov	x8, #0x0                ; =0
10039b528: 14000021    	b	0x10039b5ac <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x118>
10039b52c: 927ae268    	and	x8, x19, #0x7fffffffffffffc0
10039b530: 91008289    	add	x9, x20, #0x20
10039b534: 4f05e7e0    	movi.16b	v0, #0xbf
10039b538: 4f00e741    	movi.16b	v1, #0x1a
10039b53c: 4f01e402    	movi.16b	v2, #0x20
10039b540: aa0803ea    	mov	x10, x8
10039b544: ad7f1123    	ldp	q3, q4, [x9, #-0x20]
10039b548: ad401925    	ldp	q5, q6, [x9]
10039b54c: 4e208467    	add.16b	v7, v3, v0
10039b550: 4e208490    	add.16b	v16, v4, v0
10039b554: 4e2084b1    	add.16b	v17, v5, v0
10039b558: 4e2084d2    	add.16b	v18, v6, v0
10039b55c: 6e273427    	cmhi.16b	v7, v1, v7
10039b560: 6e303430    	cmhi.16b	v16, v1, v16
10039b564: 6e313431    	cmhi.16b	v17, v1, v17
10039b568: 6e323432    	cmhi.16b	v18, v1, v18
10039b56c: 4e221ce7    	and.16b	v7, v7, v2
10039b570: 4e221e10    	and.16b	v16, v16, v2
10039b574: 4e221e31    	and.16b	v17, v17, v2
10039b578: 4e221e52    	and.16b	v18, v18, v2
10039b57c: 4ea31ce3    	orr.16b	v3, v7, v3
10039b580: 4ea41e04    	orr.16b	v4, v16, v4
10039b584: 4ea51e25    	orr.16b	v5, v17, v5
10039b588: ad3f1123    	stp	q3, q4, [x9, #-0x20]
10039b58c: 4ea61e43    	orr.16b	v3, v18, v6
10039b590: ac820d25    	stp	q5, q3, [x9], #0x40
10039b594: f101014a    	subs	x10, x10, #0x40
10039b598: 54fffd61    	b.ne	0x10039b544 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0xb0>
10039b59c: eb08027f    	cmp	x19, x8
10039b5a0: 540003e0    	b.eq	0x10039b61c <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x188>
10039b5a4: f27d0a7f    	tst	x19, #0x38
10039b5a8: 54000240    	b.eq	0x10039b5f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x15c>
10039b5ac: aa0803ea    	mov	x10, x8
10039b5b0: 927dee68    	and	x8, x19, #0x7ffffffffffffff8
10039b5b4: cb080149    	sub	x9, x10, x8
10039b5b8: 8b0a028a    	add	x10, x20, x10
10039b5bc: 0f05e7e0    	movi.8b	v0, #0xbf
10039b5c0: 0f00e741    	movi.8b	v1, #0x1a
10039b5c4: 0f01e402    	movi.8b	v2, #0x20
10039b5c8: fd400143    	ldr	d3, [x10]
10039b5cc: 0e208464    	add.8b	v4, v3, v0
10039b5d0: 2e243424    	cmhi.8b	v4, v1, v4
10039b5d4: 0e221c84    	and.8b	v4, v4, v2
10039b5d8: 0ea31c83    	orr.8b	v3, v4, v3
10039b5dc: fc008543    	str	d3, [x10], #0x8
10039b5e0: b1002129    	adds	x9, x9, #0x8
10039b5e4: 54ffff21    	b.ne	0x10039b5c8 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x134>
10039b5e8: eb08027f    	cmp	x19, x8
10039b5ec: 54000180    	b.eq	0x10039b61c <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x188>
10039b5f0: cb080269    	sub	x9, x19, x8
10039b5f4: 8b080288    	add	x8, x20, x8
10039b5f8: 5280040a    	mov	w10, #0x20              ; =32
10039b5fc: 3940010b    	ldrb	w11, [x8]
10039b600: 5101056c    	sub	w12, w11, #0x41
10039b604: 7100699f    	cmp	w12, #0x1a
10039b608: 1a9f314c    	csel	w12, w10, wzr, lo
10039b60c: 2a0b018b    	orr	w11, w12, w11
10039b610: 3800150b    	strb	w11, [x8], #0x1
10039b614: f1000529    	subs	x9, x9, #0x1
10039b618: 54ffff21    	b.ne	0x10039b5fc <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x168>
10039b61c: f1002a7f    	cmp	x19, #0xa
10039b620: 54000343    	b.lo	0x10039b688 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x1f4>
10039b624: f0007b82    	adrp	x2, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
10039b628: 91198042    	add	x2, x2, #0x660
10039b62c: 910063e8    	add	x8, sp, #0x18
10039b630: aa1403e0    	mov	x0, x20
10039b634: aa1303e1    	mov	x1, x19
10039b638: 52800123    	mov	w3, #0x9                ; =9
10039b63c: 9437a89f    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
10039b640: 910003e0    	mov	x0, sp
10039b644: 910063e1    	add	x1, sp, #0x18
10039b648: 94014c43    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
10039b64c: b94003e8    	ldr	w8, [sp]
10039b650: 37000f88    	tbnz	w8, #0x0, 0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b654: f0007b82    	adrp	x2, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
10039b658: 9119a442    	add	x2, x2, #0x669
10039b65c: 910063e8    	add	x8, sp, #0x18
10039b660: aa1403e0    	mov	x0, x20
10039b664: aa1303e1    	mov	x1, x19
10039b668: 52800123    	mov	w3, #0x9                ; =9
10039b66c: 9437a893    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
10039b670: 910003e0    	mov	x0, sp
10039b674: 910063e1    	add	x1, sp, #0x18
10039b678: 94014c37    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
10039b67c: b94003e8    	ldr	w8, [sp]
10039b680: 37000e08    	tbnz	w8, #0x0, 0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b684: 14000015    	b	0x10039b6d8 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x244>
10039b688: f100227f    	cmp	x19, #0x8
10039b68c: 54000740    	b.eq	0x10039b774 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x2e0>
10039b690: f100267f    	cmp	x19, #0x9
10039b694: 540008e1    	b.ne	0x10039b7b0 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x31c>
10039b698: d28c6c68    	mov	x8, #0x6363             ; =25443
10039b69c: f2ae65a8    	movk	x8, #0x732d, lsl #16
10039b6a0: f2cd2ee8    	movk	x8, #0x6977, lsl #32
10039b6a4: f2ec6e88    	movk	x8, #0x6374, lsl #48
10039b6a8: f9400289    	ldr	x9, [x20]
10039b6ac: 3940228a    	ldrb	w10, [x20, #0x8]
10039b6b0: eb08013f    	cmp	x9, x8
10039b6b4: 52800d09    	mov	w9, #0x68               ; =104
10039b6b8: 7a490140    	ccmp	w10, w9, #0x0, eq
10039b6bc: 54000c20    	b.eq	0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b6c0: f940028a    	ldr	x10, [x20]
10039b6c4: 914c8108    	add	x8, x8, #0x320, lsl #12 ; =0x320000
10039b6c8: 3940228b    	ldrb	w11, [x20, #0x8]
10039b6cc: eb0a011f    	cmp	x8, x10
10039b6d0: 7a490160    	ccmp	w11, w9, #0x0, eq
10039b6d4: 54000b60    	b.eq	0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b6d8: f0007ba2    	adrp	x2, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
10039b6dc: 913b4042    	add	x2, x2, #0xed0
10039b6e0: 910063e8    	add	x8, sp, #0x18
10039b6e4: aa1403e0    	mov	x0, x20
10039b6e8: aa1303e1    	mov	x1, x19
10039b6ec: 52800103    	mov	w3, #0x8                ; =8
10039b6f0: 9437a872    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
10039b6f4: 910003e0    	mov	x0, sp
10039b6f8: 910063e1    	add	x1, sp, #0x18
10039b6fc: 94014c16    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
10039b700: b94003e8    	ldr	w8, [sp]
10039b704: 370009e8    	tbnz	w8, #0x0, 0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b708: f100367f    	cmp	x19, #0xd
10039b70c: 54000563    	b.lo	0x10039b7b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x324>
10039b710: f0007b82    	adrp	x2, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
10039b714: 911ce042    	add	x2, x2, #0x738
10039b718: 910063e8    	add	x8, sp, #0x18
10039b71c: aa1403e0    	mov	x0, x20
10039b720: aa1303e1    	mov	x1, x19
10039b724: 52800183    	mov	w3, #0xc                ; =12
10039b728: 9437a864    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
10039b72c: 910003e0    	mov	x0, sp
10039b730: 910063e1    	add	x1, sp, #0x18
10039b734: 94014c08    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
10039b738: b94003e8    	ldr	w8, [sp]
10039b73c: 37000828    	tbnz	w8, #0x0, 0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b740: f0007b82    	adrp	x2, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
10039b744: 91331442    	add	x2, x2, #0xcc5
10039b748: 910063e8    	add	x8, sp, #0x18
10039b74c: aa1403e0    	mov	x0, x20
10039b750: aa1303e1    	mov	x1, x19
10039b754: 52800183    	mov	w3, #0xc                ; =12
10039b758: 9437a858    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
10039b75c: 910003e0    	mov	x0, sp
10039b760: 910063e1    	add	x1, sp, #0x18
10039b764: 94014bfc    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
10039b768: b94003e8    	ldr	w8, [sp]
10039b76c: 370006a8    	tbnz	w8, #0x0, 0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b770: 14000028    	b	0x10039b810 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x37c>
10039b774: f9400288    	ldr	x8, [x20]
10039b778: d28c6c69    	mov	x9, #0x6363             ; =25443
10039b77c: f2aeee69    	movk	x9, #0x7773, lsl #16
10039b780: f2ce8d29    	movk	x9, #0x7469, lsl #32
10039b784: f2ed0c69    	movk	x9, #0x6863, lsl #48
10039b788: eb09011f    	cmp	x8, x9
10039b78c: 540005a0    	b.eq	0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b790: f9400288    	ldr	x8, [x20]
10039b794: d28cac89    	mov	x9, #0x6564             ; =25956
10039b798: f2ac6ca9    	movk	x9, #0x6365, lsl #16
10039b79c: f2cc8de9    	movk	x9, #0x646f, lsl #32
10039b7a0: f2ef0ca9    	movk	x9, #0x7865, lsl #48
10039b7a4: eb09011f    	cmp	x8, x9
10039b7a8: 1a9f17f5    	cset	w21, eq
10039b7ac: 14000026    	b	0x10039b844 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3b0>
10039b7b0: 52800015    	mov	w21, #0x0               ; =0
10039b7b4: 14000024    	b	0x10039b844 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3b0>
10039b7b8: f100327f    	cmp	x19, #0xc
10039b7bc: 540002a1    	b.ne	0x10039b810 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x37c>
10039b7c0: 528e8d28    	mov	w8, #0x7469             ; =29801
10039b7c4: 72ad0c68    	movk	w8, #0x6863, lsl #16
10039b7c8: f9400289    	ldr	x9, [x20]
10039b7cc: b9400a8a    	ldr	w10, [x20, #0x8]
10039b7d0: d28dec6b    	mov	x11, #0x6f63            ; =28515
10039b7d4: f2acac8b    	movk	x11, #0x6564, lsl #16
10039b7d8: f2c5af0b    	movk	x11, #0x2d78, lsl #32
10039b7dc: f2eeee6b    	movk	x11, #0x7773, lsl #48
10039b7e0: eb0b013f    	cmp	x9, x11
10039b7e4: 7a480140    	ccmp	w10, w8, #0x0, eq
10039b7e8: 540002c0    	b.eq	0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b7ec: f9400289    	ldr	x9, [x20]
10039b7f0: b9400a8a    	ldr	w10, [x20, #0x8]
10039b7f4: d28dec6b    	mov	x11, #0x6f63            ; =28515
10039b7f8: f2acac8b    	movk	x11, #0x6564, lsl #16
10039b7fc: f2cbef0b    	movk	x11, #0x5f78, lsl #32
10039b800: f2eeee6b    	movk	x11, #0x7773, lsl #48
10039b804: eb0b013f    	cmp	x9, x11
10039b808: fa4a0100    	ccmp	x8, x10, #0x0, eq
10039b80c: 540001a0    	b.eq	0x10039b840 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3ac>
10039b810: f0007ba2    	adrp	x2, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
10039b814: 913b6042    	add	x2, x2, #0xed8
10039b818: 910063e8    	add	x8, sp, #0x18
10039b81c: aa1403e0    	mov	x0, x20
10039b820: aa1303e1    	mov	x1, x19
10039b824: 52800103    	mov	w3, #0x8                ; =8
10039b828: 9437a824    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
10039b82c: 910003e0    	mov	x0, sp
10039b830: 910063e1    	add	x1, sp, #0x18
10039b834: 94014bc8    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
10039b838: b94003f5    	ldr	w21, [sp]
10039b83c: 14000002    	b	0x10039b844 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E+0x3b0>
10039b840: 52800035    	mov	w21, #0x1               ; =1
10039b844: aa1403e0    	mov	x0, x20
10039b848: aa1303e1    	mov	x1, x19
10039b84c: 52800022    	mov	w2, #0x1                ; =1
10039b850: 97f19746    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10039b854: aa1503e0    	mov	x0, x21
10039b858: a94a7bfd    	ldp	x29, x30, [sp, #0xa0]
10039b85c: a9494ff4    	ldp	x20, x19, [sp, #0x90]
10039b860: a94857f6    	ldp	x22, x21, [sp, #0x80]
10039b864: 9102c3ff    	add	sp, sp, #0xb0
10039b868: d65f03c0    	ret
10039b86c: aa0003f5    	mov	x21, x0
10039b870: aa1403e0    	mov	x0, x20
10039b874: aa1303e1    	mov	x1, x19
10039b878: 52800022    	mov	w2, #0x1                ; =1
10039b87c: 97f1973b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10039b880: aa1503e0    	mov	x0, x21
10039b884: 9438d634    	bl	0x1011d1154 <_writev+0x1011d1154>
