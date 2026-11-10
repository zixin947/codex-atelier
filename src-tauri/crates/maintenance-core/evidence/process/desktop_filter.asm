
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010050e63c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E>:
10050e63c: d10503ff    	sub	sp, sp, #0x140
10050e640: 6d0c2beb    	stp	d11, d10, [sp, #0xc0]
10050e644: 6d0d23e9    	stp	d9, d8, [sp, #0xd0]
10050e648: a90e6ffc    	stp	x28, x27, [sp, #0xe0]
10050e64c: a90f67fa    	stp	x26, x25, [sp, #0xf0]
10050e650: a9105ff8    	stp	x24, x23, [sp, #0x100]
10050e654: a91157f6    	stp	x22, x21, [sp, #0x110]
10050e658: a9124ff4    	stp	x20, x19, [sp, #0x120]
10050e65c: a9137bfd    	stp	x29, x30, [sp, #0x130]
10050e660: 9104c3fd    	add	x29, sp, #0x130
10050e664: aa0203f5    	mov	x21, x2
10050e668: aa0103fc    	mov	x28, x1
10050e66c: f9400c09    	ldr	x9, [x0, #0x18]
10050e670: f9400408    	ldr	x8, [x0, #0x8]
10050e674: eb09011f    	cmp	x8, x9
10050e678: 54001820    	b.eq	0x10050e97c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x340>
10050e67c: aa0003f7    	mov	x23, x0
10050e680: b940010a    	ldr	w10, [x8]
10050e684: a940d113    	ldp	x19, x20, [x8, #0x8]
10050e688: f9400d1b    	ldr	x27, [x8, #0x18]
10050e68c: 91008118    	add	x24, x8, #0x20
10050e690: f9000418    	str	x24, [x0, #0x8]
10050e694: a901d7fc    	stp	x28, x21, [sp, #0x18]
10050e698: b7f818bb    	tbnz	x27, #0x3f, 0x10050e9ac <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x370>
10050e69c: 52800036    	mov	w22, #0x1               ; =1
10050e6a0: 52800419    	mov	w25, #0x20              ; =32
10050e6a4: 0f05e7e8    	movi.8b	v8, #0xbf
10050e6a8: 0f00e749    	movi.8b	v9, #0x1a
10050e6ac: 0f01e40a    	movi.8b	v10, #0x20
10050e6b0: f9000be9    	str	x9, [sp, #0x10]
10050e6b4: b40001fb    	cbz	x27, 0x10050e6f0 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0xb4>
10050e6b8: b9000fea    	str	w10, [sp, #0xc]
10050e6bc: 97ebcbae    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10050e6c0: aa1b03e0    	mov	x0, x27
10050e6c4: 52800021    	mov	w1, #0x1                ; =1
10050e6c8: 97ebcba7    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10050e6cc: b4001740    	cbz	x0, 0x10050e9b4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x378>
10050e6d0: aa0003fa    	mov	x26, x0
10050e6d4: aa1403e1    	mov	x1, x20
10050e6d8: aa1b03e2    	mov	x2, x27
10050e6dc: 94330bf4    	bl	0x1011d16ac <_writev+0x1011d16ac>
10050e6e0: f100237f    	cmp	x27, #0x8
10050e6e4: 54000142    	b.hs	0x10050e70c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0xd0>
10050e6e8: d2800008    	mov	x8, #0x0                ; =0
10050e6ec: 1400003a    	b	0x10050e7d4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x198>
10050e6f0: 52800020    	mov	w0, #0x1                ; =1
10050e6f4: aa1403e1    	mov	x1, x20
10050e6f8: aa1b03e2    	mov	x2, x27
10050e6fc: 94330bec    	bl	0x1011d16ac <_writev+0x1011d16ac>
10050e700: 5280003a    	mov	w26, #0x1               ; =1
10050e704: b5001093    	cbnz	x19, 0x10050e914 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x2d8>
10050e708: 14000087    	b	0x10050e924 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x2e8>
10050e70c: f101037f    	cmp	x27, #0x40
10050e710: 54000062    	b.hs	0x10050e71c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0xe0>
10050e714: d2800008    	mov	x8, #0x0                ; =0
10050e718: 14000021    	b	0x10050e79c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x160>
10050e71c: 927ae368    	and	x8, x27, #0x7fffffffffffffc0
10050e720: 91008349    	add	x9, x26, #0x20
10050e724: aa0803ea    	mov	x10, x8
10050e728: 4f05e7f0    	movi.16b	v16, #0xbf
10050e72c: 4f00e751    	movi.16b	v17, #0x1a
10050e730: 4f01e412    	movi.16b	v18, #0x20
10050e734: ad7f0520    	ldp	q0, q1, [x9, #-0x20]
10050e738: ad400d22    	ldp	q2, q3, [x9]
10050e73c: 4e308404    	add.16b	v4, v0, v16
10050e740: 4e308425    	add.16b	v5, v1, v16
10050e744: 4e308446    	add.16b	v6, v2, v16
10050e748: 4e308467    	add.16b	v7, v3, v16
10050e74c: 6e243624    	cmhi.16b	v4, v17, v4
10050e750: 6e253625    	cmhi.16b	v5, v17, v5
10050e754: 6e263626    	cmhi.16b	v6, v17, v6
10050e758: 6e273627    	cmhi.16b	v7, v17, v7
10050e75c: 4e321c84    	and.16b	v4, v4, v18
10050e760: 4e321ca5    	and.16b	v5, v5, v18
10050e764: 4e321cc6    	and.16b	v6, v6, v18
10050e768: 4e321ce7    	and.16b	v7, v7, v18
10050e76c: 4ea01c80    	orr.16b	v0, v4, v0
10050e770: 4ea11ca1    	orr.16b	v1, v5, v1
10050e774: 4ea21cc2    	orr.16b	v2, v6, v2
10050e778: ad3f0520    	stp	q0, q1, [x9, #-0x20]
10050e77c: 4ea31ce0    	orr.16b	v0, v7, v3
10050e780: ac820122    	stp	q2, q0, [x9], #0x40
10050e784: f101014a    	subs	x10, x10, #0x40
10050e788: 54fffd61    	b.ne	0x10050e734 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0xf8>
10050e78c: eb08037f    	cmp	x27, x8
10050e790: 54000360    	b.eq	0x10050e7fc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x1c0>
10050e794: f27d0b7f    	tst	x27, #0x38
10050e798: 540001e0    	b.eq	0x10050e7d4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x198>
10050e79c: aa0803ea    	mov	x10, x8
10050e7a0: 927def68    	and	x8, x27, #0x7ffffffffffffff8
10050e7a4: cb080149    	sub	x9, x10, x8
10050e7a8: 8b0a034a    	add	x10, x26, x10
10050e7ac: fd400140    	ldr	d0, [x10]
10050e7b0: 0e288401    	add.8b	v1, v0, v8
10050e7b4: 2e213521    	cmhi.8b	v1, v9, v1
10050e7b8: 0e2a1c21    	and.8b	v1, v1, v10
10050e7bc: 0ea01c20    	orr.8b	v0, v1, v0
10050e7c0: fc008540    	str	d0, [x10], #0x8
10050e7c4: b1002129    	adds	x9, x9, #0x8
10050e7c8: 54ffff21    	b.ne	0x10050e7ac <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x170>
10050e7cc: eb08037f    	cmp	x27, x8
10050e7d0: 54000160    	b.eq	0x10050e7fc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x1c0>
10050e7d4: cb080369    	sub	x9, x27, x8
10050e7d8: 8b080348    	add	x8, x26, x8
10050e7dc: 3940010a    	ldrb	w10, [x8]
10050e7e0: 5101054b    	sub	w11, w10, #0x41
10050e7e4: 7100697f    	cmp	w11, #0x1a
10050e7e8: 1a9f332b    	csel	w11, w25, wzr, lo
10050e7ec: 2a0a016a    	orr	w10, w11, w10
10050e7f0: 3800150a    	strb	w10, [x8], #0x1
10050e7f4: f1000529    	subs	x9, x9, #0x1
10050e7f8: 54ffff21    	b.ne	0x10050e7dc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x1a0>
10050e7fc: f1005b7f    	cmp	x27, #0x16
10050e800: 54000569    	b.ls	0x10050e8ac <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x270>
10050e804: 9100a3e8    	add	x8, sp, #0x28
10050e808: aa1a03e0    	mov	x0, x26
10050e80c: aa1b03e1    	mov	x1, x27
10050e810: 90007062    	adrp	x2, 0x10131a000 <_anon.b5f536351c4bf1e454cf90283dd9e625.537+0x24f>
10050e814: 911db442    	add	x2, x2, #0x76d
10050e818: 528002c3    	mov	w3, #0x16               ; =22
10050e81c: 9431dc27    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
10050e820: d10243a0    	sub	x0, x29, #0x90
10050e824: 9100a3e1    	add	x1, sp, #0x28
10050e828: 94015aa3    	bl	0x1005652b4 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
10050e82c: b85703b6    	ldur	w22, [x29, #-0x90]
10050e830: aa1a03e0    	mov	x0, x26
10050e834: aa1b03e1    	mov	x1, x27
10050e838: 52800022    	mov	w2, #0x1                ; =1
10050e83c: 97ebcb4b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050e840: 36000676    	tbz	w22, #0x0, 0x10050e90c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x2d0>
10050e844: a90957fc    	stp	x28, x21, [sp, #0x90]
10050e848: d10243a8    	sub	x8, x29, #0x90
10050e84c: aa1403e0    	mov	x0, x20
10050e850: aa1b03e1    	mov	x1, x27
10050e854: 940abdff    	bl	0x1007be050 <__ZN13codexmate_lib8platform7process20extract_process_name17h3ae4d3431dc159c8E>
10050e858: 3cd703a0    	ldur	q0, [x29, #-0x90]
10050e85c: 9100a3e9    	add	x9, sp, #0x28
10050e860: 3c804120    	stur	q0, [x9, #0x4]
10050e864: f85803a8    	ldur	x8, [x29, #-0x80]
10050e868: f8014128    	stur	x8, [x9, #0x14]
10050e86c: 52800036    	mov	w22, #0x1               ; =1
10050e870: b40000b3    	cbz	x19, 0x10050e884 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x248>
10050e874: aa1403e0    	mov	x0, x20
10050e878: aa1303e1    	mov	x1, x19
10050e87c: 52800022    	mov	w2, #0x1                ; =1
10050e880: 97ebcb3a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050e884: b9400fe8    	ldr	w8, [sp, #0xc]
10050e888: b90002a8    	str	w8, [x21]
10050e88c: 3cc283e0    	ldur	q0, [sp, #0x28]
10050e890: 3cc343e1    	ldur	q1, [sp, #0x34]
10050e894: 3d8006a1    	str	q1, [x21, #0x10]
10050e898: 3c8042a0    	stur	q0, [x21, #0x4]
10050e89c: 910082b5    	add	x21, x21, #0x20
10050e8a0: f9400ee9    	ldr	x9, [x23, #0x18]
10050e8a4: f94006f8    	ldr	x24, [x23, #0x8]
10050e8a8: 14000020    	b	0x10050e928 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x2ec>
10050e8ac: 54000521    	b.ne	0x10050e950 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x314>
10050e8b0: a9402748    	ldp	x8, x9, [x26]
10050e8b4: f840e34a    	ldur	x10, [x26, #0xe]
10050e8b8: d28c65eb    	mov	x11, #0x632f            ; =25391
10050e8bc: f2ac2d0b    	movk	x11, #0x6168, lsl #16
10050e8c0: f2ccee8b    	movk	x11, #0x6774, lsl #32
10050e8c4: f2ee8e0b    	movk	x11, #0x7470, lsl #48
10050e8c8: eb0b011f    	cmp	x8, x11
10050e8cc: d28c25c8    	mov	x8, #0x612e             ; =24878
10050e8d0: f2ae0e08    	movk	x8, #0x7070, lsl #16
10050e8d4: f2cc65e8    	movk	x8, #0x632f, lsl #32
10050e8d8: f2edcde8    	movk	x8, #0x6e6f, lsl #48
10050e8dc: fa480120    	ccmp	x9, x8, #0x0, eq
10050e8e0: d28dcde8    	mov	x8, #0x6e6f             ; =28271
10050e8e4: f2acae88    	movk	x8, #0x6574, lsl #16
10050e8e8: f2ce8dc8    	movk	x8, #0x746e, lsl #32
10050e8ec: f2e5ee68    	movk	x8, #0x2f73, lsl #48
10050e8f0: fa480140    	ccmp	x10, x8, #0x0, eq
10050e8f4: 1a9f07f6    	cset	w22, ne
10050e8f8: aa1a03e0    	mov	x0, x26
10050e8fc: aa1b03e1    	mov	x1, x27
10050e900: 52800022    	mov	w2, #0x1                ; =1
10050e904: 97ebcb19    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050e908: 34fff9f6    	cbz	w22, 0x10050e844 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x208>
10050e90c: 52800036    	mov	w22, #0x1               ; =1
10050e910: b40000b3    	cbz	x19, 0x10050e924 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x2e8>
10050e914: aa1403e0    	mov	x0, x20
10050e918: aa1303e1    	mov	x1, x19
10050e91c: 52800022    	mov	w2, #0x1                ; =1
10050e920: 97ebcb12    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050e924: f9400be9    	ldr	x9, [sp, #0x10]
10050e928: eb09031f    	cmp	x24, x9
10050e92c: 54000280    	b.eq	0x10050e97c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x340>
10050e930: b940030a    	ldr	w10, [x24]
10050e934: a940d313    	ldp	x19, x20, [x24, #0x8]
10050e938: f9400f1b    	ldr	x27, [x24, #0x18]
10050e93c: 91008318    	add	x24, x24, #0x20
10050e940: f90006f8    	str	x24, [x23, #0x8]
10050e944: a901d7fc    	stp	x28, x21, [sp, #0x18]
10050e948: b7f8011b    	tbnz	x27, #0x3f, 0x10050e968 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x32c>
10050e94c: 17ffff59    	b	0x10050e6b0 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x74>
10050e950: aa1a03e0    	mov	x0, x26
10050e954: aa1b03e1    	mov	x1, x27
10050e958: 52800022    	mov	w2, #0x1                ; =1
10050e95c: 97ebcb03    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050e960: b5fffdb3    	cbnz	x19, 0x10050e914 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x2d8>
10050e964: 17fffff0    	b	0x10050e924 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x2e8>
10050e968: d2800016    	mov	x22, #0x0               ; =0
10050e96c: aa1603e0    	mov	x0, x22
10050e970: aa1a03e1    	mov	x1, x26
10050e974: 9433073c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10050e978: d4200020    	brk	#0x1
10050e97c: aa1c03e0    	mov	x0, x28
10050e980: aa1503e1    	mov	x1, x21
10050e984: a9537bfd    	ldp	x29, x30, [sp, #0x130]
10050e988: a9524ff4    	ldp	x20, x19, [sp, #0x120]
10050e98c: a95157f6    	ldp	x22, x21, [sp, #0x110]
10050e990: a9505ff8    	ldp	x24, x23, [sp, #0x100]
10050e994: a94f67fa    	ldp	x26, x25, [sp, #0xf0]
10050e998: a94e6ffc    	ldp	x28, x27, [sp, #0xe0]
10050e99c: 6d4d23e9    	ldp	d9, d8, [sp, #0xd0]
10050e9a0: 6d4c2beb    	ldp	d11, d10, [sp, #0xc0]
10050e9a4: 910503ff    	add	sp, sp, #0x140
10050e9a8: d65f03c0    	ret
10050e9ac: d2800016    	mov	x22, #0x0               ; =0
10050e9b0: 17ffffef    	b	0x10050e96c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x330>
10050e9b4: aa1b03fa    	mov	x26, x27
10050e9b8: 17ffffed    	b	0x10050e96c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x330>
10050e9bc: aa0003f5    	mov	x21, x0
10050e9c0: b50000b3    	cbnz	x19, 0x10050e9d4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h2bc4ea84069647a8E+0x398>
10050e9c4: 910243e0    	add	x0, sp, #0x90
10050e9c8: 9400f7e4    	bl	0x10054c958 <__ZN4core3ptr102drop_in_place$LT$alloc..vec..in_place_drop..InPlaceDrop$LT$$LP$u32$C$alloc..string..String$RP$$GT$$GT$17h2a59e34e7bcffca4E>
10050e9cc: aa1503e0    	mov	x0, x21
10050e9d0: 943309e1    	bl	0x1011d1154 <_writev+0x1011d1154>
10050e9d4: aa1403e0    	mov	x0, x20
10050e9d8: aa1303e1    	mov	x1, x19
