
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008b7540 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE>:
1008b7540: d105c3ff    	sub	sp, sp, #0x170
1008b7544: 6d1023e9    	stp	d9, d8, [sp, #0x100]
1008b7548: a9116ffc    	stp	x28, x27, [sp, #0x110]
1008b754c: a91267fa    	stp	x26, x25, [sp, #0x120]
1008b7550: a9135ff8    	stp	x24, x23, [sp, #0x130]
1008b7554: a91457f6    	stp	x22, x21, [sp, #0x140]
1008b7558: a9154ff4    	stp	x20, x19, [sp, #0x150]
1008b755c: a9167bfd    	stp	x29, x30, [sp, #0x160]
1008b7560: 910583fd    	add	x29, sp, #0x160
1008b7564: aa0103fc    	mov	x28, x1
1008b7568: aa0003f7    	mov	x23, x0
1008b756c: aa0803f4    	mov	x20, x8
1008b7570: 910243e0    	add	x0, sp, #0x90
1008b7574: aa1703e1    	mov	x1, x23
1008b7578: aa1c03e2    	mov	x2, x28
1008b757c: 97ffe245    	bl	0x1008afe90 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history6reader17ha1ea64990302ac8aE>
1008b7580: a9496be8    	ldp	x8, x26, [sp, #0x90]
1008b7584: f90023e8    	str	x8, [sp, #0x40]
1008b7588: b4001468    	cbz	x8, 0x1008b7814 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x2d4>
1008b758c: f90003f4    	str	x20, [sp]
1008b7590: f9001fff    	str	xzr, [sp, #0x38]
1008b7594: 52800028    	mov	w8, #0x1                ; =1
1008b7598: a904a3ff    	stp	xzr, x8, [sp, #0x48]
1008b759c: f9403b55    	ldr	x21, [x26, #0x70]
1008b75a0: f00081d4    	adrp	x20, 0x1018f2000 <__ZN13codexmate_lib13EXIT_APPROVED17h1f9ee2eac97195fdE.0>
1008b75a4: 910fe294    	add	x20, x20, #0x3f8
1008b75a8: b0005308    	adrp	x8, 0x101318000 <_anon.a9e2253c03df711f51070ce6102367ea.930+0x2c>
1008b75ac: 3dc18d00    	ldr	q0, [x8, #0x630]
1008b75b0: 3d800be0    	str	q0, [sp, #0x20]
1008b75b4: b0005308    	adrp	x8, 0x101318000 <_anon.a9e2253c03df711f51070ce6102367ea.930+0x2c>
1008b75b8: 3dc19100    	ldr	q0, [x8, #0x640]
1008b75bc: 3d8007e0    	str	q0, [sp, #0x10]
1008b75c0: f00057c8    	adrp	x8, 0x1013b2000 <_anon.973b090bee24c06fdb949486dbea4242.731+0x14a>
1008b75c4: fd475d08    	ldr	d8, [x8, #0xeb8]
1008b75c8: 14000007    	b	0x1008b75e4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0xa4>
1008b75cc: aa1703fc    	mov	x28, x23
1008b75d0: aa1503f7    	mov	x23, x21
1008b75d4: aa1303f5    	mov	x21, x19
1008b75d8: f9401fe8    	ldr	x8, [sp, #0x38]
1008b75dc: 8b1b0108    	add	x8, x8, x27
1008b75e0: f9001fe8    	str	x8, [sp, #0x38]
1008b75e4: f9002fff    	str	xzr, [sp, #0x58]
1008b75e8: 910123e2    	add	x2, sp, #0x48
1008b75ec: f94023e0    	ldr	x0, [sp, #0x40]
1008b75f0: 52800141    	mov	w1, #0xa                ; =10
1008b75f4: d63f02a0    	blr	x21
1008b75f8: aa0103fb    	mov	x27, x1
1008b75fc: 37001220    	tbnz	w0, #0x0, 0x1008b7840 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x300>
1008b7600: b400129b    	cbz	x27, 0x1008b7850 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x310>
1008b7604: a9450be0    	ldp	x0, x2, [sp, #0x50]
1008b7608: f100005f    	cmp	x2, #0x0
1008b760c: 1a9f07f8    	cset	w24, ne
1008b7610: b4000122    	cbz	x2, 0x1008b7634 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0xf4>
1008b7614: d100044a    	sub	x10, x2, #0x1
1008b7618: 386a6808    	ldrb	w8, [x0, x10]
1008b761c: 7100291f    	cmp	w8, #0xa
1008b7620: 1a9f17e9    	cset	w9, eq
1008b7624: 540000e1    	b.ne	0x1008b7640 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x100>
1008b7628: b400006a    	cbz	x10, 0x1008b7634 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0xf4>
1008b762c: d1000848    	sub	x8, x2, #0x2
1008b7630: 14000006    	b	0x1008b7648 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x108>
1008b7634: d2800001    	mov	x1, #0x0                ; =0
1008b7638: 52800016    	mov	w22, #0x0               ; =0
1008b763c: 1400000c    	b	0x1008b766c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x12c>
1008b7640: aa0a03e8    	mov	x8, x10
1008b7644: aa0203ea    	mov	x10, x2
1008b7648: eb02011f    	cmp	x8, x2
1008b764c: 54001482    	b.hs	0x1008b78dc <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x39c>
1008b7650: 3868680b    	ldrb	w11, [x0, x8]
1008b7654: 7100357f    	cmp	w11, #0xd
1008b7658: 1a9f17f6    	cset	w22, eq
1008b765c: 9a8a0101    	csel	x1, x8, x10, eq
1008b7660: eb02003f    	cmp	x1, x2
1008b7664: 540011c8    	b.hi	0x1008b789c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x35c>
1008b7668: aa0903f8    	mov	x24, x9
1008b766c: 910243e8    	add	x8, sp, #0x90
1008b7670: 94233ab8    	bl	0x101186150 <__ZN4core3str8converts9from_utf817h3d3f04c61e7c2c97E>
1008b7674: b94093e8    	ldr	w8, [sp, #0x90]
1008b7678: 360003c8    	tbz	w8, #0x0, 0x1008b76f0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x1b0>
1008b767c: f9400288    	ldr	x8, [x20]
1008b7680: f100091f    	cmp	x8, #0x2
1008b7684: 54fffaa3    	b.lo	0x1008b75d8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x98>
1008b7688: a90673f7    	stp	x23, x28, [sp, #0x60]
1008b768c: 910183e8    	add	x8, sp, #0x60
1008b7690: f9003be8    	str	x8, [sp, #0x70]
1008b7694: f0004528    	adrp	x8, 0x10115e000 <__ZN4core5slice4sort8unstable9quicksort9quicksort17h324c95ad922d2016E+0x1fc>
1008b7698: 91351108    	add	x8, x8, #0xd44
1008b769c: f9003fe8    	str	x8, [sp, #0x78]
1008b76a0: d00057e9    	adrp	x9, 0x1013b5000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.665+0x10b>
1008b76a4: 910de929    	add	x9, x9, #0x37a
1008b76a8: 52800668    	mov	w8, #0x33               ; =51
1008b76ac: a90ca3e9    	stp	x9, x8, [sp, #0xc8]
1008b76b0: b00063c8    	adrp	x8, 0x101530000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.748+0x277>
1008b76b4: 9105c10a    	add	x10, x8, #0x170
1008b76b8: 9101c3e8    	add	x8, sp, #0x70
1008b76bc: a90e23ea    	stp	x10, x8, [sp, #0xe0]
1008b76c0: a90927ff    	stp	xzr, x9, [sp, #0x90]
1008b76c4: ad4087e0    	ldp	q0, q1, [sp, #0x10]
1008b76c8: 3d802be1    	str	q1, [sp, #0xa0]
1008b76cc: 900063c8    	adrp	x8, 0x10152f000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.17+0x520>
1008b76d0: 91362508    	add	x8, x8, #0xd89
1008b76d4: f9005be8    	str	x8, [sp, #0xb0]
1008b76d8: 3c8b83e0    	stur	q0, [sp, #0xb8]
1008b76dc: fd006fe8    	str	d8, [sp, #0xd8]
1008b76e0: d101a7a0    	sub	x0, x29, #0x69
1008b76e4: 910243e1    	add	x1, sp, #0x90
1008b76e8: 941c756e    	bl	0x100fd4ca0 <__ZN61_$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$3log17he2f3dc859ac0f3c8E>
1008b76ec: 17ffffbb    	b	0x1008b75d8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x98>
1008b76f0: aa1503f3    	mov	x19, x21
1008b76f4: aa1703f5    	mov	x21, x23
1008b76f8: aa1c03f7    	mov	x23, x28
1008b76fc: f94053fc    	ldr	x28, [sp, #0xa0]
1008b7700: b7f8083c    	tbnz	x28, #0x3f, 0x1008b7804 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x2c4>
1008b7704: f90007fa    	str	x26, [sp, #0x8]
1008b7708: f9404ffa    	ldr	x26, [sp, #0x98]
1008b770c: b400011c    	cbz	x28, 0x1008b772c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x1ec>
1008b7710: 97dd2799    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008b7714: aa1c03e0    	mov	x0, x28
1008b7718: 52800021    	mov	w1, #0x1                ; =1
1008b771c: 97dd2792    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008b7720: b4000ec0    	cbz	x0, 0x1008b78f8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x3b8>
1008b7724: aa0003f9    	mov	x25, x0
1008b7728: 14000002    	b	0x1008b7730 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x1f0>
1008b772c: 52800039    	mov	w25, #0x1               ; =1
1008b7730: aa1903e0    	mov	x0, x25
1008b7734: aa1a03e1    	mov	x1, x26
1008b7738: aa1c03e2    	mov	x2, x28
1008b773c: 942467dc    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008b7740: a90973f9    	stp	x25, x28, [sp, #0x90]
1008b7744: 910243e8    	add	x8, sp, #0x90
1008b7748: a9017d1f    	stp	xzr, xzr, [x8, #0x10]
1008b774c: a90b73f9    	stp	x25, x28, [sp, #0xb0]
1008b7750: 9101c3e8    	add	x8, sp, #0x70
1008b7754: 910243e0    	add	x0, sp, #0x90
1008b7758: f94007fa    	ldr	x26, [sp, #0x8]
1008b775c: 97e48dae    	bl	0x1001dae14 <__ZN10serde_json2de10from_trait17h472fe9ac40d19421E>
1008b7760: 3941c3e8    	ldrb	w8, [sp, #0x70]
1008b7764: 7100191f    	cmp	w8, #0x6
1008b7768: 540000a1    	b.ne	0x1008b777c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x23c>
1008b776c: 9101c3e8    	add	x8, sp, #0x70
1008b7770: 91002100    	add	x0, x8, #0x8
1008b7774: 9400fcbf    	bl	0x1008f6a70 <__ZN4core3ptr45drop_in_place$LT$serde_json..error..Error$GT$17hc34208687d70e728E>
1008b7778: 1400001d    	b	0x1008b77ec <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x2ac>
1008b777c: ad4387e0    	ldp	q0, q1, [sp, #0x70]
1008b7780: ad0487e0    	stp	q0, q1, [sp, #0x90]
1008b7784: 910243e2    	add	x2, sp, #0x90
1008b7788: f0005280    	adrp	x0, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
1008b778c: 912ec000    	add	x0, x0, #0xbb0
1008b7790: 52800081    	mov	w1, #0x4                ; =4
1008b7794: 94215cff    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
1008b7798: b4000260    	cbz	x0, 0x1008b77e4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x2a4>
1008b779c: 39400008    	ldrb	w8, [x0]
1008b77a0: 71000d1f    	cmp	w8, #0x3
1008b77a4: 54000201    	b.ne	0x1008b77e4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x2a4>
1008b77a8: f9400c08    	ldr	x8, [x0, #0x18]
1008b77ac: f100311f    	cmp	x8, #0xc
1008b77b0: 540001a1    	b.ne	0x1008b77e4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x2a4>
1008b77b4: f9400808    	ldr	x8, [x0, #0x10]
1008b77b8: f9400109    	ldr	x9, [x8]
1008b77bc: b9400908    	ldr	w8, [x8, #0x8]
1008b77c0: d28cae6a    	mov	x10, #0x6573            ; =25971
1008b77c4: f2ae6e6a    	movk	x10, #0x7373, lsl #16
1008b77c8: f2cded2a    	movk	x10, #0x6f69, lsl #32
1008b77cc: f2ebedca    	movk	x10, #0x5f6e, lsl #48
1008b77d0: eb0a013f    	cmp	x9, x10
1008b77d4: 528cada9    	mov	w9, #0x656d             ; =25965
1008b77d8: 72ac2e89    	movk	w9, #0x6174, lsl #16
1008b77dc: 7a490100    	ccmp	w8, w9, #0x0, eq
1008b77e0: 54000680    	b.eq	0x1008b78b0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x370>
1008b77e4: 9101c3e0    	add	x0, sp, #0x70
1008b77e8: 9400fcdb    	bl	0x1008f6b54 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
1008b77ec: b4ffef1c    	cbz	x28, 0x1008b75cc <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x8c>
1008b77f0: aa1903e0    	mov	x0, x25
1008b77f4: aa1c03e1    	mov	x1, x28
1008b77f8: 52800022    	mov	w2, #0x1                ; =1
1008b77fc: 97dd275b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b7800: 17ffff73    	b	0x1008b75cc <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x8c>
1008b7804: d2800000    	mov	x0, #0x0                ; =0
1008b7808: aa1903e1    	mov	x1, x25
1008b780c: 94246396    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008b7810: 14000039    	b	0x1008b78f4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x3b4>
1008b7814: 52800048    	mov	w8, #0x2                ; =2
1008b7818: a9006a88    	stp	x8, x26, [x20]
1008b781c: a9567bfd    	ldp	x29, x30, [sp, #0x160]
1008b7820: a9554ff4    	ldp	x20, x19, [sp, #0x150]
1008b7824: a95457f6    	ldp	x22, x21, [sp, #0x140]
1008b7828: a9535ff8    	ldp	x24, x23, [sp, #0x130]
1008b782c: a95267fa    	ldp	x26, x25, [sp, #0x120]
1008b7830: a9516ffc    	ldp	x28, x27, [sp, #0x110]
1008b7834: 6d5023e9    	ldp	d9, d8, [sp, #0x100]
1008b7838: 9105c3ff    	add	sp, sp, #0x170
1008b783c: d65f03c0    	ret
1008b7840: 52800048    	mov	w8, #0x2                ; =2
1008b7844: f94003e9    	ldr	x9, [sp]
1008b7848: a9006d28    	stp	x8, x27, [x9]
1008b784c: 14000005    	b	0x1008b7860 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x320>
1008b7850: 90005268    	adrp	x8, 0x101303000 <_anon.b03982025400859c41411cbf9a3d7a99.0+0xfa1>
1008b7854: 3dc3fd00    	ldr	q0, [x8, #0xff0]
1008b7858: f94003e8    	ldr	x8, [sp]
1008b785c: 3d800100    	str	q0, [x8]
1008b7860: f94027e1    	ldr	x1, [sp, #0x48]
1008b7864: b4000081    	cbz	x1, 0x1008b7874 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x334>
1008b7868: f9402be0    	ldr	x0, [sp, #0x50]
1008b786c: 52800022    	mov	w2, #0x1                ; =1
1008b7870: 97dd273e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b7874: f9400348    	ldr	x8, [x26]
1008b7878: b4000068    	cbz	x8, 0x1008b7884 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x344>
1008b787c: f94023e0    	ldr	x0, [sp, #0x40]
1008b7880: d63f0100    	blr	x8
1008b7884: f9400741    	ldr	x1, [x26, #0x8]
1008b7888: b4fffca1    	cbz	x1, 0x1008b781c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x2dc>
1008b788c: f9400b42    	ldr	x2, [x26, #0x10]
1008b7890: f94023e0    	ldr	x0, [sp, #0x40]
1008b7894: 97dd2735    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b7898: 17ffffe1    	b	0x1008b781c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x2dc>
1008b789c: d0007ba3    	adrp	x3, 0x10182d000 <_anon.973b090bee24c06fdb949486dbea4242.549+0x1e8>
1008b78a0: 913e0063    	add	x3, x3, #0xf80
1008b78a4: d2800000    	mov	x0, #0x0                ; =0
1008b78a8: 94246414    	bl	0x1011d08f8 <__ZN4core5slice5index16slice_index_fail17hadaab6fd6d0d9c6dE>
1008b78ac: 14000012    	b	0x1008b78f4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x3b4>
1008b78b0: f94003e9    	ldr	x9, [sp]
1008b78b4: a9017139    	stp	x25, x28, [x9, #0x10]
1008b78b8: f9401fe8    	ldr	x8, [sp, #0x38]
1008b78bc: a9026d28    	stp	x8, x27, [x9, #0x20]
1008b78c0: 3900c138    	strb	w24, [x9, #0x30]
1008b78c4: 3900c536    	strb	w22, [x9, #0x31]
1008b78c8: 52800168    	mov	w8, #0xb                ; =11
1008b78cc: a9007128    	stp	x8, x28, [x9]
1008b78d0: 9101c3e0    	add	x0, sp, #0x70
1008b78d4: 9400fca0    	bl	0x1008f6b54 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
1008b78d8: 17ffffe2    	b	0x1008b7860 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x320>
1008b78dc: d0007ba9    	adrp	x9, 0x10182d000 <_anon.973b090bee24c06fdb949486dbea4242.549+0x1e8>
1008b78e0: 913da129    	add	x9, x9, #0xf68
1008b78e4: aa0803e0    	mov	x0, x8
1008b78e8: aa0203e1    	mov	x1, x2
1008b78ec: aa0903e2    	mov	x2, x9
1008b78f0: 9424649c    	bl	0x1011d0b60 <__ZN4core9panicking18panic_bounds_check17h865e5d10cdb12e28E>
1008b78f4: d4200020    	brk	#0x1
1008b78f8: aa1c03f9    	mov	x25, x28
1008b78fc: f94007fa    	ldr	x26, [sp, #0x8]
1008b7900: 52800020    	mov	w0, #0x1                ; =1
1008b7904: 17ffffc1    	b	0x1008b7808 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x2c8>
1008b7908: 14000019    	b	0x1008b796c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x42c>
1008b790c: aa0003f5    	mov	x21, x0
1008b7910: f9400741    	ldr	x1, [x26, #0x8]
1008b7914: b40003e1    	cbz	x1, 0x1008b7990 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x450>
1008b7918: f9400b42    	ldr	x2, [x26, #0x10]
1008b791c: f94023e0    	ldr	x0, [sp, #0x40]
1008b7920: 97dd2712    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b7924: aa1503e0    	mov	x0, x21
1008b7928: 9424660b    	bl	0x1011d1154 <_writev+0x1011d1154>
1008b792c: 14000005    	b	0x1008b7940 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x400>
1008b7930: aa0003f5    	mov	x21, x0
1008b7934: 9101c3e0    	add	x0, sp, #0x70
1008b7938: 9400fc87    	bl	0x1008f6b54 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
1008b793c: 14000002    	b	0x1008b7944 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x404>
1008b7940: aa0003f5    	mov	x21, x0
1008b7944: b500007c    	cbnz	x28, 0x1008b7950 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x410>
1008b7948: f94007fa    	ldr	x26, [sp, #0x8]
1008b794c: 14000009    	b	0x1008b7970 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x430>
1008b7950: aa1903e0    	mov	x0, x25
1008b7954: aa1c03e1    	mov	x1, x28
1008b7958: 52800022    	mov	w2, #0x1                ; =1
1008b795c: 97dd2703    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b7960: f94007fa    	ldr	x26, [sp, #0x8]
1008b7964: 14000003    	b	0x1008b7970 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x430>
1008b7968: 14000001    	b	0x1008b796c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x42c>
1008b796c: aa0003f5    	mov	x21, x0
1008b7970: f94027e1    	ldr	x1, [sp, #0x48]
1008b7974: b4000081    	cbz	x1, 0x1008b7984 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE+0x444>
1008b7978: f9402be0    	ldr	x0, [sp, #0x50]
1008b797c: 52800022    	mov	w2, #0x1                ; =1
1008b7980: 97dd26fa    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b7984: f94023e0    	ldr	x0, [sp, #0x40]
1008b7988: aa1a03e1    	mov	x1, x26
1008b798c: 9400f838    	bl	0x1008f5a6c <__ZN4core3ptr132drop_in_place$LT$core..iter..adapters..take..Take$LT$std..io..Lines$LT$alloc..boxed..Box$LT$dyn$u20$std..io..BufRead$GT$$GT$$GT$$GT$17h1b9b3eb4629d3349E>
1008b7990: aa1503e0    	mov	x0, x21
1008b7994: 942465f0    	bl	0x1011d1154 <_writev+0x1011d1154>
1008b7998: 9424646c    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
