
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008b62dc <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE>:
1008b62dc: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
1008b62e0: a90167fa    	stp	x26, x25, [sp, #0x10]
1008b62e4: a9025ff8    	stp	x24, x23, [sp, #0x20]
1008b62e8: a90357f6    	stp	x22, x21, [sp, #0x30]
1008b62ec: a9044ff4    	stp	x20, x19, [sp, #0x40]
1008b62f0: a9057bfd    	stp	x29, x30, [sp, #0x50]
1008b62f4: 910143fd    	add	x29, sp, #0x50
1008b62f8: d14007ff    	sub	sp, sp, #0x1, lsl #12   ; =0x1000
1008b62fc: f90003ff    	str	xzr, [sp]
1008b6300: d10483ff    	sub	sp, sp, #0x120
1008b6304: aa0403f6    	mov	x22, x4
1008b6308: aa0303f7    	mov	x23, x3
1008b630c: aa0203f4    	mov	x20, x2
1008b6310: aa0103fa    	mov	x26, x1
1008b6314: aa0003f5    	mov	x21, x0
1008b6318: aa0503e0    	mov	x0, x5
1008b631c: aa0603e1    	mov	x1, x6
1008b6320: 94226562    	bl	0x10114f8a8 <__ZN3std4path4Path9file_name17h10722b1b995fceadE>
1008b6324: b40001e0    	cbz	x0, 0x1008b6360 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x84>
1008b6328: aa0003f8    	mov	x24, x0
1008b632c: aa0103f9    	mov	x25, x1
1008b6330: 97dd2c91    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008b6334: 52800300    	mov	w0, #0x18               ; =24
1008b6338: 52800101    	mov	w1, #0x8                ; =8
1008b633c: 97dd2c8a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008b6340: b40022e0    	cbz	x0, 0x1008b679c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x4c0>
1008b6344: aa0003f3    	mov	x19, x0
1008b6348: b6f80134    	tbz	x20, #0x3f, 0x1008b636c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x90>
1008b634c: d280001c    	mov	x28, #0x0               ; =0
1008b6350: aa1c03e0    	mov	x0, x28
1008b6354: aa1403e1    	mov	x1, x20
1008b6358: 942468c3    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008b635c: d4200020    	brk	#0x1
1008b6360: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008b6364: f90002a8    	str	x8, [x21]
1008b6368: 140000ec    	b	0x1008b6718 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x43c>
1008b636c: a901d7f8    	stp	x24, x21, [sp, #0x18]
1008b6370: a900dbf7    	stp	x23, x22, [sp, #0x8]
1008b6374: f90017f9    	str	x25, [sp, #0x28]
1008b6378: b4000134    	cbz	x20, 0x1008b639c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0xc0>
1008b637c: 97dd2c7e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008b6380: 5280003c    	mov	w28, #0x1               ; =1
1008b6384: aa1403e0    	mov	x0, x20
1008b6388: 52800021    	mov	w1, #0x1                ; =1
1008b638c: 97dd2c76    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008b6390: b4fffe00    	cbz	x0, 0x1008b6350 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x74>
1008b6394: aa0003fb    	mov	x27, x0
1008b6398: 14000002    	b	0x1008b63a0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0xc4>
1008b639c: 5280003b    	mov	w27, #0x1               ; =1
1008b63a0: 913383fc    	add	x28, sp, #0xce0
1008b63a4: aa1b03e0    	mov	x0, x27
1008b63a8: aa1a03e1    	mov	x1, x26
1008b63ac: aa1403e2    	mov	x2, x20
1008b63b0: 94246cbf    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008b63b4: a9006e74    	stp	x20, x27, [x19]
1008b63b8: 52800028    	mov	w8, #0x1                ; =1
1008b63bc: f9000a74    	str	x20, [x19, #0x10]
1008b63c0: 910163f7    	add	x23, sp, #0x58
1008b63c4: 913383f6    	add	x22, sp, #0xce0
1008b63c8: 52800318    	mov	w24, #0x18              ; =24
1008b63cc: 52800059    	mov	w25, #0x2               ; =2
1008b63d0: a9044fe8    	stp	x8, x19, [sp, #0x40]
1008b63d4: 14000003    	b	0x1008b63e0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x104>
1008b63d8: f9402be8    	ldr	x8, [sp, #0x50]
1008b63dc: b40018a8    	cbz	x8, 0x1008b66f0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x414>
1008b63e0: d1000508    	sub	x8, x8, #0x1
1008b63e4: f9002be8    	str	x8, [sp, #0x50]
1008b63e8: f94027e9    	ldr	x9, [sp, #0x48]
1008b63ec: 9b182508    	madd	x8, x8, x24, x9
1008b63f0: a9405113    	ldp	x19, x20, [x8]
1008b63f4: f9400901    	ldr	x1, [x8, #0x10]
1008b63f8: 910163e8    	add	x8, sp, #0x58
1008b63fc: aa1403e0    	mov	x0, x20
1008b6400: 942248c8    	bl	0x101148720 <__ZN3std3sys2fs8read_dir17ha353ee9b9b706035E>
1008b6404: 394183e8    	ldrb	w8, [sp, #0x60]
1008b6408: f9402fe9    	ldr	x9, [sp, #0x58]
1008b640c: f90673e9    	str	x9, [sp, #0xce0]
1008b6410: 3933a3e8    	strb	w8, [sp, #0xce8]
1008b6414: 7100091f    	cmp	w8, #0x2
1008b6418: 54000081    	b.ne	0x1008b6428 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x14c>
1008b641c: 913383e0    	add	x0, sp, #0xce0
1008b6420: 94010f18    	bl	0x1008fa080 <__ZN4core3ptr89drop_in_place$LT$core..result..Result$LT$std..fs..ReadDir$C$std..io..error..Error$GT$$GT$17h5247929482014814E>
1008b6424: 140000ad    	b	0x1008b66d8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x3fc>
1008b6428: f9002fff    	str	xzr, [sp, #0x58]
1008b642c: f90243ff    	str	xzr, [sp, #0x480]
1008b6430: f90457e9    	str	x9, [sp, #0x8a8]
1008b6434: 3922c3e8    	strb	w8, [sp, #0x8b0]
1008b6438: 14000008    	b	0x1008b6458 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x17c>
1008b643c: aa1a03e0    	mov	x0, x26
1008b6440: 52800301    	mov	w1, #0x18               ; =24
1008b6444: 52800102    	mov	w2, #0x8                ; =8
1008b6448: 97dd2c48    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b644c: 3962c3e8    	ldrb	w8, [sp, #0x8b0]
1008b6450: 7100091f    	cmp	w8, #0x2
1008b6454: 540013e0    	b.eq	0x1008b66d0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x3f4>
1008b6458: 913383e8    	add	x8, sp, #0xce0
1008b645c: 912142e0    	add	x0, x23, #0x850
1008b6460: 9422d30d    	bl	0x10116b094 <__ZN75_$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hcbd8a6bdaabc6208E>
1008b6464: f94673e8    	ldr	x8, [sp, #0xce0]
1008b6468: b4000108    	cbz	x8, 0x1008b6488 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x1ac>
1008b646c: 3cc08380    	ldur	q0, [x28, #0x8]
1008b6470: 3d800fe0    	str	q0, [sp, #0x30]
1008b6474: 912343e0    	add	x0, sp, #0x8d0
1008b6478: 910062c1    	add	x1, x22, #0x18
1008b647c: 52808202    	mov	w2, #0x410              ; =1040
1008b6480: 94246c8b    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008b6484: 14000015    	b	0x1008b64d8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x1fc>
1008b6488: 3962c3e8    	ldrb	w8, [sp, #0x8b0]
1008b648c: 7100091f    	cmp	w8, #0x2
1008b6490: 54000120    	b.eq	0x1008b64b4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x1d8>
1008b6494: f94457e8    	ldr	x8, [sp, #0x8a8]
1008b6498: 92800009    	mov	x9, #-0x1               ; =-1
1008b649c: f8690108    	ldaddl	x9, x8, [x8]
1008b64a0: f100051f    	cmp	x8, #0x1
1008b64a4: 54000081    	b.ne	0x1008b64b4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x1d8>
1008b64a8: d50339bf    	dmb	ishld
1008b64ac: 912142e0    	add	x0, x23, #0x850
1008b64b0: 9422b071    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008b64b4: 3922c3f9    	strb	w25, [sp, #0x8b0]
1008b64b8: b94ce3f5    	ldr	w21, [sp, #0xce0]
1008b64bc: 3cc08380    	ldur	q0, [x28, #0x8]
1008b64c0: 3d800fe0    	str	q0, [sp, #0x30]
1008b64c4: 912343e0    	add	x0, sp, #0x8d0
1008b64c8: 910062c1    	add	x1, x22, #0x18
1008b64cc: 52808202    	mov	w2, #0x410              ; =1040
1008b64d0: 94246c77    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008b64d4: 36000ff5    	tbz	w21, #0x0, 0x1008b66d0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x3f4>
1008b64d8: 3dc00fe0    	ldr	q0, [sp, #0x30]
1008b64dc: 9e660008    	fmov	x8, d0
1008b64e0: b5000228    	cbnz	x8, 0x1008b6524 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x248>
1008b64e4: 4e183c1a    	mov.d	x26, v0[1]
1008b64e8: 92400748    	and	x8, x26, #0x3
1008b64ec: f100051f    	cmp	x8, #0x1
1008b64f0: 54fffae1    	b.ne	0x1008b644c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x170>
1008b64f4: f85fff5b    	ldr	x27, [x26, #-0x1]!
1008b64f8: f9400755    	ldr	x21, [x26, #0x8]
1008b64fc: f94002a8    	ldr	x8, [x21]
1008b6500: b4000068    	cbz	x8, 0x1008b650c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x230>
1008b6504: aa1b03e0    	mov	x0, x27
1008b6508: d63f0100    	blr	x8
1008b650c: f94006a1    	ldr	x1, [x21, #0x8]
1008b6510: b4fff961    	cbz	x1, 0x1008b643c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x160>
1008b6514: f9400aa2    	ldr	x2, [x21, #0x10]
1008b6518: aa1b03e0    	mov	x0, x27
1008b651c: 97dd2c13    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b6520: 17ffffc7    	b	0x1008b643c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x160>
1008b6524: 910042c0    	add	x0, x22, #0x10
1008b6528: 912343e1    	add	x1, sp, #0x8d0
1008b652c: 52808202    	mov	w2, #0x410              ; =1040
1008b6530: 3d800fe0    	str	q0, [sp, #0x30]
1008b6534: 94246c5e    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008b6538: 3dc00fe0    	ldr	q0, [sp, #0x30]
1008b653c: 3d833be0    	str	q0, [sp, #0xce0]
1008b6540: 912343e8    	add	x8, sp, #0x8d0
1008b6544: 913383e0    	add	x0, sp, #0xce0
1008b6548: 94223187    	bl	0x101142b64 <__ZN3std2fs8DirEntry9file_type17h4b49a3827a468684E>
1008b654c: 7951a3e8    	ldrh	w8, [sp, #0x8d0]
1008b6550: 7100051f    	cmp	w8, #0x1
1008b6554: 54000281    	b.ne	0x1008b65a4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x2c8>
1008b6558: f9446ffa    	ldr	x26, [sp, #0x8d8]
1008b655c: 92400748    	and	x8, x26, #0x3
1008b6560: f100051f    	cmp	x8, #0x1
1008b6564: 54000861    	b.ne	0x1008b6670 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x394>
1008b6568: f85fff5b    	ldr	x27, [x26, #-0x1]!
1008b656c: f9400755    	ldr	x21, [x26, #0x8]
1008b6570: f94002a8    	ldr	x8, [x21]
1008b6574: b4000068    	cbz	x8, 0x1008b6580 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x2a4>
1008b6578: aa1b03e0    	mov	x0, x27
1008b657c: d63f0100    	blr	x8
1008b6580: f94006a1    	ldr	x1, [x21, #0x8]
1008b6584: b4000081    	cbz	x1, 0x1008b6594 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x2b8>
1008b6588: f9400aa2    	ldr	x2, [x21, #0x10]
1008b658c: aa1b03e0    	mov	x0, x27
1008b6590: 97dd2bf6    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b6594: aa1a03e0    	mov	x0, x26
1008b6598: 52800301    	mov	w1, #0x18               ; =24
1008b659c: 52800102    	mov	w2, #0x8                ; =8
1008b65a0: 14000033    	b	0x1008b666c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x390>
1008b65a4: 7951a7f5    	ldrh	w21, [sp, #0x8d2]
1008b65a8: 9122e3e8    	add	x8, sp, #0x8b8
1008b65ac: 913383e0    	add	x0, sp, #0xce0
1008b65b0: 94223119    	bl	0x101142a14 <__ZN3std2fs8DirEntry4path17h69519f87704b953cE>
1008b65b4: 12140ea8    	and	w8, w21, #0xf000
1008b65b8: 7140111f    	cmp	w8, #0x4, lsl #12       ; =0x4000
1008b65bc: 54000261    	b.ne	0x1008b6608 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x32c>
1008b65c0: f9445ffa    	ldr	x26, [sp, #0x8b8]
1008b65c4: f94463fb    	ldr	x27, [sp, #0x8c0]
1008b65c8: f94467f5    	ldr	x21, [sp, #0x8c8]
1008b65cc: f9402bf8    	ldr	x24, [sp, #0x50]
1008b65d0: f94023e8    	ldr	x8, [sp, #0x40]
1008b65d4: eb08031f    	cmp	x24, x8
1008b65d8: 54000061    	b.ne	0x1008b65e4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x308>
1008b65dc: 910103e0    	add	x0, sp, #0x40
1008b65e0: 94241ea8    	bl	0x1011be080 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hc102b861917c98eeE>
1008b65e4: f94027e8    	ldr	x8, [sp, #0x48]
1008b65e8: 52800309    	mov	w9, #0x18               ; =24
1008b65ec: 9b092308    	madd	x8, x24, x9, x8
1008b65f0: a9006d1a    	stp	x26, x27, [x8]
1008b65f4: f9000915    	str	x21, [x8, #0x10]
1008b65f8: 91000708    	add	x8, x24, #0x1
1008b65fc: 52800318    	mov	w24, #0x18              ; =24
1008b6600: f9002be8    	str	x8, [sp, #0x50]
1008b6604: 1400001b    	b	0x1008b6670 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x394>
1008b6608: 13003ea8    	sxth	w8, w21
1008b660c: 31401d1f    	cmn	w8, #0x7, lsl #12       ; =0x7000
1008b6610: 5400026a    	b.ge	0x1008b665c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x380>
1008b6614: 912343e8    	add	x8, sp, #0x8d0
1008b6618: 913383e0    	add	x0, sp, #0xce0
1008b661c: 94223136    	bl	0x101142af4 <__ZN3std2fs8DirEntry9file_name17he0bc65473bb31e7bE>
1008b6620: f94473e8    	ldr	x8, [sp, #0x8e0]
1008b6624: f94017e9    	ldr	x9, [sp, #0x28]
1008b6628: eb09011f    	cmp	x8, x9
1008b662c: 540000e1    	b.ne	0x1008b6648 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x36c>
1008b6630: f9446ffa    	ldr	x26, [sp, #0x8d8]
1008b6634: aa1a03e0    	mov	x0, x26
1008b6638: f9400fe1    	ldr	x1, [sp, #0x18]
1008b663c: f94017e2    	ldr	x2, [sp, #0x28]
1008b6640: 94246c18    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1008b6644: 34000280    	cbz	w0, 0x1008b6694 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x3b8>
1008b6648: f9446be1    	ldr	x1, [sp, #0x8d0]
1008b664c: b4000081    	cbz	x1, 0x1008b665c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x380>
1008b6650: f9446fe0    	ldr	x0, [sp, #0x8d8]
1008b6654: 52800022    	mov	w2, #0x1                ; =1
1008b6658: 97dd2bc4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b665c: f9445fe1    	ldr	x1, [sp, #0x8b8]
1008b6660: b4000081    	cbz	x1, 0x1008b6670 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x394>
1008b6664: f94463e0    	ldr	x0, [sp, #0x8c0]
1008b6668: 52800022    	mov	w2, #0x1                ; =1
1008b666c: 97dd2bbf    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b6670: f94673e8    	ldr	x8, [sp, #0xce0]
1008b6674: 92800009    	mov	x9, #-0x1               ; =-1
1008b6678: f8690108    	ldaddl	x9, x8, [x8]
1008b667c: f100051f    	cmp	x8, #0x1
1008b6680: 54ffee61    	b.ne	0x1008b644c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x170>
1008b6684: d50339bf    	dmb	ishld
1008b6688: 913383e0    	add	x0, sp, #0xce0
1008b668c: 9422affa    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008b6690: 17ffff6f    	b	0x1008b644c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x170>
1008b6694: f9446be1    	ldr	x1, [sp, #0x8d0]
1008b6698: b4000081    	cbz	x1, 0x1008b66a8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x3cc>
1008b669c: aa1a03e0    	mov	x0, x26
1008b66a0: 52800022    	mov	w2, #0x1                ; =1
1008b66a4: 97dd2bb1    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b66a8: f94463fa    	ldr	x26, [sp, #0x8c0]
1008b66ac: f94467e1    	ldr	x1, [sp, #0x8c8]
1008b66b0: aa1a03e0    	mov	x0, x26
1008b66b4: a9408fe2    	ldp	x2, x3, [sp, #0x8]
1008b66b8: 94000eb6    	bl	0x1008ba190 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E>
1008b66bc: 37000400    	tbnz	w0, #0x0, 0x1008b673c <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x460>
1008b66c0: f9445fe1    	ldr	x1, [sp, #0x8b8]
1008b66c4: b4fffd61    	cbz	x1, 0x1008b6670 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x394>
1008b66c8: aa1a03e0    	mov	x0, x26
1008b66cc: 17ffffe7    	b	0x1008b6668 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x38c>
1008b66d0: 910163e0    	add	x0, sp, #0x58
1008b66d4: 94010d38    	bl	0x1008f9bb4 <__ZN4core3ptr83drop_in_place$LT$core..iter..adapters..flatten..Flatten$LT$std..fs..ReadDir$GT$$GT$17h373b37329274da1eE>
1008b66d8: b4ffe813    	cbz	x19, 0x1008b63d8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0xfc>
1008b66dc: aa1403e0    	mov	x0, x20
1008b66e0: aa1303e1    	mov	x1, x19
1008b66e4: 52800022    	mov	w2, #0x1                ; =1
1008b66e8: 97dd2ba0    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b66ec: 17ffff3b    	b	0x1008b63d8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0xfc>
1008b66f0: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008b66f4: f94013e9    	ldr	x9, [sp, #0x20]
1008b66f8: f9000128    	str	x8, [x9]
1008b66fc: f94023e8    	ldr	x8, [sp, #0x40]
1008b6700: b40000c8    	cbz	x8, 0x1008b6718 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x43c>
1008b6704: f94027e0    	ldr	x0, [sp, #0x48]
1008b6708: 8b080508    	add	x8, x8, x8, lsl #1
1008b670c: d37df101    	lsl	x1, x8, #3
1008b6710: 52800102    	mov	w2, #0x8                ; =8
1008b6714: 97dd2b95    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b6718: 914007ff    	add	sp, sp, #0x1, lsl #12   ; =0x1000
1008b671c: 910483ff    	add	sp, sp, #0x120
1008b6720: a9457bfd    	ldp	x29, x30, [sp, #0x50]
1008b6724: a9444ff4    	ldp	x20, x19, [sp, #0x40]
1008b6728: a94357f6    	ldp	x22, x21, [sp, #0x30]
1008b672c: a9425ff8    	ldp	x24, x23, [sp, #0x20]
1008b6730: a94167fa    	ldp	x26, x25, [sp, #0x10]
1008b6734: a8c66ffc    	ldp	x28, x27, [sp], #0x60
1008b6738: d65f03c0    	ret
1008b673c: 911ee7e8    	add	x8, sp, #0x7b9
1008b6740: 3ccff100    	ldur	q0, [x8, #0xff]
1008b6744: f94013e9    	ldr	x9, [sp, #0x20]
1008b6748: 3d800120    	str	q0, [x9]
1008b674c: f94467e8    	ldr	x8, [sp, #0x8c8]
1008b6750: f9000928    	str	x8, [x9, #0x10]
1008b6754: f94673e8    	ldr	x8, [sp, #0xce0]
1008b6758: 92800009    	mov	x9, #-0x1               ; =-1
1008b675c: f8690108    	ldaddl	x9, x8, [x8]
1008b6760: f100051f    	cmp	x8, #0x1
1008b6764: 54000081    	b.ne	0x1008b6774 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x498>
1008b6768: d50339bf    	dmb	ishld
1008b676c: 913383e0    	add	x0, sp, #0xce0
1008b6770: 9422afc1    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008b6774: 910163e0    	add	x0, sp, #0x58
1008b6778: 94010d0f    	bl	0x1008f9bb4 <__ZN4core3ptr83drop_in_place$LT$core..iter..adapters..flatten..Flatten$LT$std..fs..ReadDir$GT$$GT$17h373b37329274da1eE>
1008b677c: b40000b3    	cbz	x19, 0x1008b6790 <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x4b4>
1008b6780: aa1403e0    	mov	x0, x20
1008b6784: aa1303e1    	mov	x1, x19
1008b6788: 52800022    	mov	w2, #0x1                ; =1
1008b678c: 97dd2b77    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b6790: 910103e0    	add	x0, sp, #0x40
1008b6794: 97f2b368    	bl	0x100563534 <__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h039a5bcf17780753E>
1008b6798: 17ffffd9    	b	0x1008b66fc <__ZN13codexmate_lib4core5relay23codex_thread_visibility27find_trusted_rollout_backup17he6c4cb2f4bcd659bE+0x420>
