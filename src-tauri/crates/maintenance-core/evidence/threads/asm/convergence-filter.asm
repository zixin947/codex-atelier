
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010050e2f4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E>:
10050e2f4: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
10050e2f8: a90167fa    	stp	x26, x25, [sp, #0x10]
10050e2fc: a9025ff8    	stp	x24, x23, [sp, #0x20]
10050e300: a90357f6    	stp	x22, x21, [sp, #0x30]
10050e304: a9044ff4    	stp	x20, x19, [sp, #0x40]
10050e308: a9057bfd    	stp	x29, x30, [sp, #0x50]
10050e30c: 910143fd    	add	x29, sp, #0x50
10050e310: d10803ff    	sub	sp, sp, #0x200
10050e314: aa0203f3    	mov	x19, x2
10050e318: aa0103fb    	mov	x27, x1
10050e31c: f9400c14    	ldr	x20, [x0, #0x18]
10050e320: f9400416    	ldr	x22, [x0, #0x8]
10050e324: eb1402df    	cmp	x22, x20
10050e328: 540010a0    	b.eq	0x10050e53c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x248>
10050e32c: aa0003fa    	mov	x26, x0
10050e330: 9102a3f5    	add	x21, sp, #0xa8
10050e334: 9104c3f8    	add	x24, sp, #0x130
10050e338: d000753c    	adrp	x28, 0x1013b4000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.331+0x4>
10050e33c: 910e039c    	add	x28, x28, #0x380
10050e340: a9016fe0    	stp	x0, x27, [sp, #0x10]
10050e344: 14000005    	b	0x10050e358 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x64>
10050e348: 910042a0    	add	x0, x21, #0x10
10050e34c: 940105b6    	bl	0x10054fa24 <__ZN4core3ptr202drop_in_place$LT$$LP$alloc..string..String$C$core..option..Option$LT$alloc..string..String$GT$$C$alloc..string..String$C$alloc..string..String$C$core..option..Option$LT$alloc..string..String$GT$$RP$$GT$17hcba51645dcbb0235E>
10050e350: eb1402df    	cmp	x22, x20
10050e354: 54000f40    	b.eq	0x10050e53c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x248>
10050e358: ad4006c0    	ldp	q0, q1, [x22]
10050e35c: ad410ec2    	ldp	q2, q3, [x22, #0x20]
10050e360: ad028fe2    	stp	q2, q3, [sp, #0x50]
10050e364: ad0187e0    	stp	q0, q1, [sp, #0x30]
10050e368: ad4206c0    	ldp	q0, q1, [x22, #0x40]
10050e36c: 3dc01ac2    	ldr	q2, [x22, #0x60]
10050e370: f9403ac8    	ldr	x8, [x22, #0x70]
10050e374: a90a6fe8    	stp	x8, x27, [sp, #0xa0]
10050e378: ad040be1    	stp	q1, q2, [sp, #0x80]
10050e37c: 3d801fe0    	str	q0, [sp, #0x70]
10050e380: 9101e2d6    	add	x22, x22, #0x78
10050e384: f9000756    	str	x22, [x26, #0x8]
10050e388: f9005bf3    	str	x19, [sp, #0xb0]
10050e38c: f80802a8    	stur	x8, [x21, #0x80]
10050e390: ad4407e0    	ldp	q0, q1, [sp, #0x80]
10050e394: ad0306a0    	stp	q0, q1, [x21, #0x60]
10050e398: 3dc01fe2    	ldr	q2, [sp, #0x70]
10050e39c: ad4187e0    	ldp	q0, q1, [sp, #0x30]
10050e3a0: ad0086a0    	stp	q0, q1, [x21, #0x10]
10050e3a4: ad4283e1    	ldp	q1, q0, [sp, #0x50]
10050e3a8: ad020aa0    	stp	q0, q2, [x21, #0x40]
10050e3ac: 3c8302a1    	stur	q1, [x21, #0x30]
10050e3b0: a94f07f7    	ldp	x23, x1, [sp, #0xf0]
10050e3b4: f100283f    	cmp	x1, #0xa
10050e3b8: 54000189    	b.ls	0x10050e3e8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0xf4>
10050e3bc: 9104c3e8    	add	x8, sp, #0x130
10050e3c0: aa1703e0    	mov	x0, x23
10050e3c4: aa1c03e2    	mov	x2, x28
10050e3c8: 52800143    	mov	w3, #0xa                ; =10
10050e3cc: 9431dd3b    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
10050e3d0: d10283a0    	sub	x0, x29, #0xa0
10050e3d4: 9104c3e1    	add	x1, sp, #0x130
10050e3d8: 94100ef2    	bl	0x100911fa0 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
10050e3dc: b85603a8    	ldur	w8, [x29, #-0xa0]
10050e3e0: 3707fb48    	tbnz	w8, #0x0, 0x10050e348 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x54>
10050e3e4: 14000009    	b	0x10050e408 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x114>
10050e3e8: 54000101    	b.ne	0x10050e408 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x114>
10050e3ec: f9400388    	ldr	x8, [x28]
10050e3f0: f94002e9    	ldr	x9, [x23]
10050e3f4: 7940138a    	ldrh	w10, [x28, #0x8]
10050e3f8: 794012eb    	ldrh	w11, [x23, #0x8]
10050e3fc: eb09011f    	cmp	x8, x9
10050e400: fa4b0140    	ccmp	x10, x11, #0x0, eq
10050e404: 54fffa20    	b.eq	0x10050e348 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x54>
10050e408: a94d87f9    	ldp	x25, x1, [sp, #0xd8]
10050e40c: aa1903e0    	mov	x0, x25
10050e410: 940ea563    	bl	0x1008b799c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE>
10050e414: 34fff9a0    	cbz	w0, 0x10050e348 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x54>
10050e418: ad4387e0    	ldp	q0, q1, [sp, #0x70]
10050e41c: ad0b87e0    	stp	q0, q1, [sp, #0x170]
10050e420: 3dc027e0    	ldr	q0, [sp, #0x90]
10050e424: 3d8067e0    	str	q0, [sp, #0x190]
10050e428: f94053e8    	ldr	x8, [sp, #0xa0]
10050e42c: f900d3e8    	str	x8, [sp, #0x1a0]
10050e430: ad4187e0    	ldp	q0, q1, [sp, #0x30]
10050e434: ad0987e0    	stp	q0, q1, [sp, #0x130]
10050e438: ad4287e0    	ldp	q0, q1, [sp, #0x50]
10050e43c: ad0a87e0    	stp	q0, q1, [sp, #0x150]
10050e440: d2f00014    	mov	x20, #-0x8000000000000000 ; =-9223372036854775808
10050e444: f94077e1    	ldr	x1, [sp, #0xe8]
10050e448: f94083fc    	ldr	x28, [sp, #0x100]
10050e44c: eb14039f    	cmp	x28, x20
10050e450: 54000360    	b.eq	0x10050e4bc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x1c8>
10050e454: a90207f7    	stp	x23, x1, [sp, #0x20]
10050e458: a951dbf8    	ldp	x24, x22, [sp, #0x118]
10050e45c: a94bd3f7    	ldp	x23, x20, [sp, #0xb8]
10050e460: f9406bfb    	ldr	x27, [sp, #0xd0]
10050e464: a95087fa    	ldp	x26, x1, [sp, #0x108]
10050e468: aa1a03e0    	mov	x0, x26
10050e46c: 97f4833b    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
10050e470: b4000121    	cbz	x1, 0x10050e494 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x1a0>
10050e474: 9100c3e8    	add	x8, sp, #0x30
10050e478: 3cc50100    	ldur	q0, [x8, #0x50]
10050e47c: 3c9903a0    	stur	q0, [x29, #-0x70]
10050e480: aa1c03f4    	mov	x20, x28
10050e484: a9416ffa    	ldp	x26, x27, [sp, #0x10]
10050e488: 9104c3f8    	add	x24, sp, #0x130
10050e48c: a94207f7    	ldp	x23, x1, [sp, #0x20]
10050e490: 1400000b    	b	0x10050e4bc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x1c8>
10050e494: a941dffb    	ldp	x27, x23, [sp, #0x18]
10050e498: b40000bc    	cbz	x28, 0x10050e4ac <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x1b8>
10050e49c: aa1a03e0    	mov	x0, x26
10050e4a0: aa1c03e1    	mov	x1, x28
10050e4a4: 52800022    	mov	w2, #0x1                ; =1
10050e4a8: 97ebcc30    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050e4ac: d2f00014    	mov	x20, #-0x8000000000000000 ; =-9223372036854775808
10050e4b0: f9400bfa    	ldr	x26, [sp, #0x10]
10050e4b4: 9104c3f8    	add	x24, sp, #0x130
10050e4b8: f94017e1    	ldr	x1, [sp, #0x28]
10050e4bc: 9100c3e8    	add	x8, sp, #0x30
10050e4c0: 3cc18100    	ldur	q0, [x8, #0x18]
10050e4c4: d10283a9    	sub	x9, x29, #0xa0
10050e4c8: 3c818120    	stur	q0, [x9, #0x18]
10050e4cc: f8428108    	ldur	x8, [x8, #0x28]
10050e4d0: f8028128    	stur	x8, [x9, #0x28]
10050e4d4: 3dc00fe0    	ldr	q0, [sp, #0x30]
10050e4d8: 3c9603a0    	stur	q0, [x29, #-0xa0]
10050e4dc: f94023e8    	ldr	x8, [sp, #0x40]
10050e4e0: f81703a8    	stur	x8, [x29, #-0x90]
10050e4e4: b4000081    	cbz	x1, 0x10050e4f4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x200>
10050e4e8: aa1703e0    	mov	x0, x23
10050e4ec: 52800022    	mov	w2, #0x1                ; =1
10050e4f0: 97ebcc1e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050e4f4: ad7b83a1    	ldp	q1, q0, [x29, #-0x90]
10050e4f8: 3cd603a2    	ldur	q2, [x29, #-0xa0]
10050e4fc: ad008261    	stp	q1, q0, [x19, #0x10]
10050e500: 3d800262    	str	q2, [x19]
10050e504: f9001a74    	str	x20, [x19, #0x30]
10050e508: 3cd903a0    	ldur	q0, [x29, #-0x70]
10050e50c: 3c838260    	stur	q0, [x19, #0x38]
10050e510: f8470308    	ldur	x8, [x24, #0x70]
10050e514: 3cc60300    	ldur	q0, [x24, #0x60]
10050e518: 3c848260    	stur	q0, [x19, #0x48]
10050e51c: f9002e68    	str	x8, [x19, #0x58]
10050e520: 91018273    	add	x19, x19, #0x60
10050e524: f9400f54    	ldr	x20, [x26, #0x18]
10050e528: f9400756    	ldr	x22, [x26, #0x8]
10050e52c: d000753c    	adrp	x28, 0x1013b4000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.331+0x4>
10050e530: 910e039c    	add	x28, x28, #0x380
10050e534: eb1402df    	cmp	x22, x20
10050e538: 54fff101    	b.ne	0x10050e358 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17h27524cabd83a5790E+0x64>
10050e53c: aa1b03e0    	mov	x0, x27
10050e540: aa1303e1    	mov	x1, x19
10050e544: 910803ff    	add	sp, sp, #0x200
10050e548: a9457bfd    	ldp	x29, x30, [sp, #0x50]
10050e54c: a9444ff4    	ldp	x20, x19, [sp, #0x40]
10050e550: a94357f6    	ldp	x22, x21, [sp, #0x30]
10050e554: a9425ff8    	ldp	x24, x23, [sp, #0x20]
10050e558: a94167fa    	ldp	x26, x25, [sp, #0x10]
10050e55c: a8c66ffc    	ldp	x28, x27, [sp], #0x60
10050e560: d65f03c0    	ret
