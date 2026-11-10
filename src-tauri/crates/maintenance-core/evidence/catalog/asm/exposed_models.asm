
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001007992d8 <__ZN13codexmate_lib4core5relay6models13RelayProvider14exposed_models17h78c65a493568f3aaE>:
1007992d8: d10143ff    	sub	sp, sp, #0x50
1007992dc: a90257f6    	stp	x22, x21, [sp, #0x20]
1007992e0: a9034ff4    	stp	x20, x19, [sp, #0x30]
1007992e4: a9047bfd    	stp	x29, x30, [sp, #0x40]
1007992e8: 910103fd    	add	x29, sp, #0x40
1007992ec: aa0003f5    	mov	x21, x0
1007992f0: aa0803f3    	mov	x19, x8
1007992f4: f9404408    	ldr	x8, [x0, #0x88]
1007992f8: b4000108    	cbz	x8, 0x100799318 <__ZN13codexmate_lib4core5relay6models13RelayProvider14exposed_models17h78c65a493568f3aaE+0x40>
1007992fc: 9101e2a0    	add	x0, x21, #0x78
100799300: aa1303e8    	mov	x8, x19
100799304: a9447bfd    	ldp	x29, x30, [sp, #0x40]
100799308: a9434ff4    	ldp	x20, x19, [sp, #0x30]
10079930c: a94257f6    	ldp	x22, x21, [sp, #0x20]
100799310: 910143ff    	add	sp, sp, #0x50
100799314: 17f71d63    	b	0x1005608a0 <__ZN67_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..clone..Clone$GT$5clone17h323f129bfcac6b13E>
100799318: 97e1a097    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10079931c: 52800c00    	mov	w0, #0x60               ; =96
100799320: 52800101    	mov	w1, #0x8                ; =8
100799324: 97e1a090    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100799328: b4000300    	cbz	x0, 0x100799388 <__ZN13codexmate_lib4core5relay6models13RelayProvider14exposed_models17h78c65a493568f3aaE+0xb0>
10079932c: aa0003f4    	mov	x20, x0
100799330: 910023e8    	add	x8, sp, #0x8
100799334: 910182a0    	add	x0, x21, #0x60
100799338: 94278071    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
10079933c: 3943faa8    	ldrb	w8, [x21, #0xfe]
100799340: 3cc083e0    	ldur	q0, [sp, #0x8]
100799344: 3d800680    	str	q0, [x20, #0x10]
100799348: f9400fe9    	ldr	x9, [sp, #0x18]
10079934c: f900029f    	str	xzr, [x20]
100799350: a9027e89    	stp	x9, xzr, [x20, #0x20]
100799354: 52800029    	mov	w9, #0x1                ; =1
100799358: f9001a89    	str	x9, [x20, #0x30]
10079935c: b0005b4a    	adrp	x10, 0x101302000 <_writev+0x101302000>
100799360: 3dc03540    	ldr	q0, [x10, #0xd0]
100799364: 3c838280    	stur	q0, [x20, #0x38]
100799368: 39016288    	strb	w8, [x20, #0x58]
10079936c: a9005269    	stp	x9, x20, [x19]
100799370: f9000a69    	str	x9, [x19, #0x10]
100799374: a9447bfd    	ldp	x29, x30, [sp, #0x40]
100799378: a9434ff4    	ldp	x20, x19, [sp, #0x30]
10079937c: a94257f6    	ldp	x22, x21, [sp, #0x20]
100799380: 910143ff    	add	sp, sp, #0x50
100799384: d65f03c0    	ret
100799388: 52800100    	mov	w0, #0x8                ; =8
10079938c: 52800c01    	mov	w1, #0x60               ; =96
100799390: 9428dcaf    	bl	0x1011d064c <__ZN5alloc5alloc18handle_alloc_error17h06754e1196e3be78E>
100799394: aa0003f3    	mov	x19, x0
100799398: aa1403e0    	mov	x0, x20
10079939c: 52800c01    	mov	w1, #0x60               ; =96
1007993a0: 52800102    	mov	w2, #0x8                ; =8
1007993a4: 97e1a071    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007993a8: aa1303e0    	mov	x0, x19
1007993ac: 9428df6a    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001007993b0 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E>:
1007993b0: d10203ff    	sub	sp, sp, #0x80
1007993b4: a90557f6    	stp	x22, x21, [sp, #0x50]
1007993b8: a9064ff4    	stp	x20, x19, [sp, #0x60]
1007993bc: a9077bfd    	stp	x29, x30, [sp, #0x70]
1007993c0: 9101c3fd    	add	x29, sp, #0x70
1007993c4: 97ea5765    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1007993c8: aa0103f3    	mov	x19, x1
1007993cc: b6f800a1    	tbz	x1, #0x3f, 0x1007993e0 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x30>
1007993d0: d2800016    	mov	x22, #0x0               ; =0
1007993d4: aa1603e0    	mov	x0, x22
1007993d8: aa1303e1    	mov	x1, x19
1007993dc: 9428dca2    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1007993e0: aa0003f5    	mov	x21, x0
1007993e4: b40001f3    	cbz	x19, 0x100799420 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x70>
1007993e8: 97e1a063    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1007993ec: 52800036    	mov	w22, #0x1               ; =1
1007993f0: aa1303e0    	mov	x0, x19
1007993f4: 52800021    	mov	w1, #0x1                ; =1
1007993f8: 97e1a05b    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1007993fc: b4fffec0    	cbz	x0, 0x1007993d4 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x24>
100799400: aa0003f4    	mov	x20, x0
100799404: aa1503e1    	mov	x1, x21
100799408: aa1303e2    	mov	x2, x19
10079940c: 9428e0a8    	bl	0x1011d16ac <_writev+0x1011d16ac>
100799410: f100227f    	cmp	x19, #0x8
100799414: 54000122    	b.hs	0x100799438 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x88>
100799418: d2800008    	mov	x8, #0x0                ; =0
10079941c: 1400003b    	b	0x100799508 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x158>
100799420: 52800034    	mov	w20, #0x1               ; =1
100799424: 52800020    	mov	w0, #0x1                ; =1
100799428: aa1503e1    	mov	x1, x21
10079942c: aa1303e2    	mov	x2, x19
100799430: 9428e09f    	bl	0x1011d16ac <_writev+0x1011d16ac>
100799434: 1400003f    	b	0x100799530 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x180>
100799438: f101027f    	cmp	x19, #0x40
10079943c: 54000062    	b.hs	0x100799448 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x98>
100799440: d2800008    	mov	x8, #0x0                ; =0
100799444: 14000021    	b	0x1007994c8 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x118>
100799448: 927ae268    	and	x8, x19, #0x7fffffffffffffc0
10079944c: 91008289    	add	x9, x20, #0x20
100799450: 4f05e7e0    	movi.16b	v0, #0xbf
100799454: 4f00e741    	movi.16b	v1, #0x1a
100799458: 4f01e402    	movi.16b	v2, #0x20
10079945c: aa0803ea    	mov	x10, x8
100799460: ad7f1123    	ldp	q3, q4, [x9, #-0x20]
100799464: ad401925    	ldp	q5, q6, [x9]
100799468: 4e208467    	add.16b	v7, v3, v0
10079946c: 4e208490    	add.16b	v16, v4, v0
100799470: 4e2084b1    	add.16b	v17, v5, v0
100799474: 4e2084d2    	add.16b	v18, v6, v0
100799478: 6e273427    	cmhi.16b	v7, v1, v7
10079947c: 6e303430    	cmhi.16b	v16, v1, v16
100799480: 6e313431    	cmhi.16b	v17, v1, v17
100799484: 6e323432    	cmhi.16b	v18, v1, v18
100799488: 4e221ce7    	and.16b	v7, v7, v2
10079948c: 4e221e10    	and.16b	v16, v16, v2
100799490: 4e221e31    	and.16b	v17, v17, v2
100799494: 4e221e52    	and.16b	v18, v18, v2
100799498: 4ea31ce3    	orr.16b	v3, v7, v3
10079949c: 4ea41e04    	orr.16b	v4, v16, v4
1007994a0: 4ea51e25    	orr.16b	v5, v17, v5
1007994a4: ad3f1123    	stp	q3, q4, [x9, #-0x20]
1007994a8: 4ea61e43    	orr.16b	v3, v18, v6
1007994ac: ac820d25    	stp	q5, q3, [x9], #0x40
1007994b0: f101014a    	subs	x10, x10, #0x40
1007994b4: 54fffd61    	b.ne	0x100799460 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0xb0>
1007994b8: eb08027f    	cmp	x19, x8
1007994bc: 540003a0    	b.eq	0x100799530 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x180>
1007994c0: f27d0a7f    	tst	x19, #0x38
1007994c4: 54000220    	b.eq	0x100799508 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x158>
1007994c8: aa0803e9    	mov	x9, x8
1007994cc: 927dee68    	and	x8, x19, #0x7ffffffffffffff8
1007994d0: 0f05e7e0    	movi.8b	v0, #0xbf
1007994d4: 0f00e741    	movi.8b	v1, #0x1a
1007994d8: 0f01e402    	movi.8b	v2, #0x20
1007994dc: fc696a83    	ldr	d3, [x20, x9]
1007994e0: 0e208464    	add.8b	v4, v3, v0
1007994e4: 2e243424    	cmhi.8b	v4, v1, v4
1007994e8: 0e221c84    	and.8b	v4, v4, v2
1007994ec: 0ea31c83    	orr.8b	v3, v4, v3
1007994f0: fc296a83    	str	d3, [x20, x9]
1007994f4: 91002129    	add	x9, x9, #0x8
1007994f8: eb09011f    	cmp	x8, x9
1007994fc: 54ffff01    	b.ne	0x1007994dc <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x12c>
100799500: eb08027f    	cmp	x19, x8
100799504: 54000160    	b.eq	0x100799530 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x180>
100799508: 52800409    	mov	w9, #0x20               ; =32
10079950c: 38686a8a    	ldrb	w10, [x20, x8]
100799510: 5101054b    	sub	w11, w10, #0x41
100799514: 7100697f    	cmp	w11, #0x1a
100799518: 1a9f312b    	csel	w11, w9, wzr, lo
10079951c: 2a0a016a    	orr	w10, w11, w10
100799520: 38286a8a    	strb	w10, [x20, x8]
100799524: 91000508    	add	x8, x8, #0x1
100799528: eb08027f    	cmp	x19, x8
10079952c: 54ffff01    	b.ne	0x10079950c <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x15c>
100799530: a900cfff    	stp	xzr, x19, [sp, #0x8]
100799534: a901cff4    	stp	x20, x19, [sp, #0x18]
100799538: a902cfff    	stp	xzr, x19, [sp, #0x28]
10079953c: 0f0105e0    	movi.2s	v0, #0x2f
100799540: fd001fe0    	str	d0, [sp, #0x38]
100799544: 52800028    	mov	w8, #0x1                ; =1
100799548: 390103e8    	strb	w8, [sp, #0x40]
10079954c: 790093e8    	strh	w8, [sp, #0x48]
100799550: 910023e0    	add	x0, sp, #0x8
100799554: 94017324    	bl	0x1007f61e4 <__ZN4core3str4iter22SplitInternal$LT$P$GT$9next_back17h919969fa09ebfb78E>
100799558: f100001f    	cmp	x0, #0x0
10079955c: 9a800288    	csel	x8, x20, x0, eq
100799560: 9a810269    	csel	x9, x19, x1, eq
100799564: f100213f    	cmp	x9, #0x8
100799568: 54000102    	b.hs	0x100799588 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x1d8>
10079956c: f100113f    	cmp	x9, #0x4
100799570: 54000342    	b.hs	0x1007995d8 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x228>
100799574: f1000d3f    	cmp	x9, #0x3
100799578: 54000640    	b.eq	0x100799640 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x290>
10079957c: 52800015    	mov	w21, #0x0               ; =0
100799580: b5000813    	cbnz	x19, 0x100799680 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x2d0>
100799584: 14000043    	b	0x100799690 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x2e0>
100799588: f9400109    	ldr	x9, [x8]
10079958c: d28cac8a    	mov	x10, #0x6564            ; =25956
100799590: f2ae0caa    	movk	x10, #0x7065, lsl #16
100799594: f2ccae6a    	movk	x10, #0x6573, lsl #32
100799598: f2ed6caa    	movk	x10, #0x6b65, lsl #48
10079959c: eb0a013f    	cmp	x9, x10
1007995a0: 54000840    	b.eq	0x1007996a8 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x2f8>
1007995a4: b9400109    	ldr	w9, [x8]
1007995a8: 528d2d6a    	mov	w10, #0x696b            ; =26987
1007995ac: 72ad2daa    	movk	w10, #0x696d, lsl #16
1007995b0: 6b0a013f    	cmp	w9, w10
1007995b4: 54000620    	b.eq	0x100799678 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E+0x2c8>
1007995b8: f9400109    	ldr	x9, [x8]
1007995bc: d28dedaa    	mov	x10, #0x6f6d            ; =28525
