
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100362404 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE>:
100362404: d10683ff    	sub	sp, sp, #0x1a0
100362408: a9146ffc    	stp	x28, x27, [sp, #0x140]
10036240c: a91567fa    	stp	x26, x25, [sp, #0x150]
100362410: a9165ff8    	stp	x24, x23, [sp, #0x160]
100362414: a91757f6    	stp	x22, x21, [sp, #0x170]
100362418: a9184ff4    	stp	x20, x19, [sp, #0x180]
10036241c: a9197bfd    	stp	x29, x30, [sp, #0x190]
100362420: 910643fd    	add	x29, sp, #0x190
100362424: aa0203f6    	mov	x22, x2
100362428: aa0103f7    	mov	x23, x1
10036242c: aa0003f5    	mov	x21, x0
100362430: aa0803f4    	mov	x20, x8
100362434: b0007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362438: 912ad442    	add	x2, x2, #0xab5
10036243c: 910143e8    	add	x8, sp, #0x50
100362440: aa0103e0    	mov	x0, x1
100362444: aa1603e1    	mov	x1, x22
100362448: 528001c3    	mov	w3, #0xe                ; =14
10036244c: 9410a8d4    	bl	0x10078c79c <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE>
100362450: b0007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362454: 912b0c42    	add	x2, x2, #0xac3
100362458: 9101a3e8    	add	x8, sp, #0x68
10036245c: aa1703e0    	mov	x0, x23
100362460: aa1603e1    	mov	x1, x22
100362464: 528000e3    	mov	w3, #0x7                ; =7
100362468: 9410a8cd    	bl	0x10078c79c <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE>
10036246c: b0007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362470: 912b2842    	add	x2, x2, #0xaca
100362474: d102e3a8    	sub	x8, x29, #0xb8
100362478: aa1703e0    	mov	x0, x23
10036247c: aa1603e1    	mov	x1, x22
100362480: 528000a3    	mov	w3, #0x5                ; =5
100362484: 9410a8c6    	bl	0x10078c79c <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE>
100362488: f85483a8    	ldur	x8, [x29, #-0xb8]
10036248c: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
100362490: eb09011f    	cmp	x8, x9
100362494: 540000c1    	b.ne	0x1003624ac <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0xa8>
100362498: 910203e8    	add	x8, sp, #0x80
10036249c: aa1703e0    	mov	x0, x23
1003624a0: aa1603e1    	mov	x1, x22
1003624a4: 9410a3fd    	bl	0x10078b498 <__ZN13codexmate_lib4core5relay12codex_writer25read_top_level_model_line17hb785f0da46cd98feE>
1003624a8: 14000005    	b	0x1003624bc <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0xb8>
1003624ac: 3cd483a0    	ldur	q0, [x29, #-0xb8]
1003624b0: 3d8023e0    	str	q0, [sp, #0x80]
1003624b4: f85583a8    	ldur	x8, [x29, #-0xa8]
1003624b8: f9004be8    	str	x8, [sp, #0x90]
1003624bc: b0007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003624c0: 912b3c42    	add	x2, x2, #0xacf
1003624c4: 910283e8    	add	x8, sp, #0xa0
1003624c8: aa1703e0    	mov	x0, x23
1003624cc: aa1603e1    	mov	x1, x22
1003624d0: 52800243    	mov	w3, #0x12               ; =18
1003624d4: 9410a8b2    	bl	0x10078c79c <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE>
1003624d8: 52800019    	mov	w25, #0x0               ; =0
1003624dc: f94053f3    	ldr	x19, [sp, #0xa0]
1003624e0: b4000115    	cbz	x21, 0x100362500 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0xfc>
1003624e4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1003624e8: eb08027f    	cmp	x19, x8
1003624ec: 540000a0    	b.eq	0x100362500 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0xfc>
1003624f0: a94a8be1    	ldp	x1, x2, [sp, #0xa8]
1003624f4: aa1503e0    	mov	x0, x21
1003624f8: 97fa5d5b    	bl	0x1001f9a64 <__ZN13codexmate_lib4core5relay15config_takeover22is_aimami_catalog_path17h2919b19fe61e8337E>
1003624fc: aa0003f9    	mov	x25, x0
100362500: b0007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362504: 912b8442    	add	x2, x2, #0xae1
100362508: d102e3a8    	sub	x8, x29, #0xb8
10036250c: aa1703e0    	mov	x0, x23
100362510: aa1603e1    	mov	x1, x22
100362514: 528001e3    	mov	w3, #0xf                ; =15
100362518: 9410a8a1    	bl	0x10078c79c <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE>
10036251c: f85483b5    	ldur	x21, [x29, #-0xb8]
100362520: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100362524: eb0802bf    	cmp	x21, x8
100362528: 54000061    	b.ne	0x100362534 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x130>
10036252c: b9004fff    	str	wzr, [sp, #0x4c]
100362530: 1400000d    	b	0x100362564 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x160>
100362534: a97507b8    	ldp	x24, x1, [x29, #-0xb0]
100362538: aa1803e0    	mov	x0, x24
10036253c: 9410a816    	bl	0x10078c594 <__ZN13codexmate_lib4core5relay12codex_writer27aimami_router_base_url_port17h03bf6b2df6bbc630E>
100362540: 12003c08    	and	w8, w0, #0xffff
100362544: 7100051f    	cmp	w8, #0x1
100362548: 1a9f17e8    	cset	w8, eq
10036254c: b9004fe8    	str	w8, [sp, #0x4c]
100362550: b40000b5    	cbz	x21, 0x100362564 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x160>
100362554: aa1803e0    	mov	x0, x24
100362558: aa1503e1    	mov	x1, x21
10036255c: 52800022    	mov	w2, #0x1                ; =1
100362560: 97f27c02    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100362564: a9452be9    	ldp	x9, x10, [sp, #0x50]
100362568: a903cfe9    	stp	x9, x19, [sp, #0x38]
10036256c: a946a3e9    	ldp	x9, x8, [sp, #0x68]
100362570: a901abe8    	stp	x8, x10, [sp, #0x18]
100362574: a9482fea    	ldp	x10, x11, [sp, #0x80]
100362578: a902a7ea    	stp	x10, x9, [sp, #0x28]
10036257c: f94057e8    	ldr	x8, [sp, #0xa8]
100362580: a900afe8    	stp	x8, x11, [sp, #0x8]
100362584: aa1703e0    	mov	x0, x23
100362588: aa1603e1    	mov	x1, x22
10036258c: 9410ac0e    	bl	0x10078d5c4 <__ZN13codexmate_lib4core5relay12codex_writer36router_provider_requires_openai_auth17h079ea57f0dada285E>
100362590: aa0003f8    	mov	x24, x0
100362594: aa1703e0    	mov	x0, x23
100362598: aa1603e1    	mov	x1, x22
10036259c: 94109703    	bl	0x1007881a8 <__ZN13codexmate_lib4core5relay12codex_writer21router_entry_presence17h1123fdd6371f062bE>
1003625a0: aa0003f5    	mov	x21, x0
1003625a4: 12001f1c    	and	w28, w24, #0xff
1003625a8: 52800108    	mov	w8, #0x8                ; =8
1003625ac: a90ba3ff    	stp	xzr, x8, [sp, #0xb8]
1003625b0: f90067ff    	str	xzr, [sp, #0xc8]
1003625b4: 52800048    	mov	w8, #0x2                ; =2
1003625b8: 381473a8    	sturb	w8, [x29, #-0xb9]
1003625bc: a934dbbf    	stp	xzr, x22, [x29, #-0xb8]
1003625c0: d102e3b3    	sub	x19, x29, #0xb8
1003625c4: 0f000540    	movi.2s	v0, #0xa
1003625c8: 5280003b    	mov	w27, #0x1               ; =1
1003625cc: a935dbb7    	stp	x23, x22, [x29, #-0xa8]
1003625d0: a936dbbf    	stp	xzr, x22, [x29, #-0x98]
1003625d4: fc1783a0    	stur	d0, [x29, #-0x88]
1003625d8: 381803bb    	sturb	w27, [x29, #-0x80]
1003625dc: 781883bf    	sturh	wzr, [x29, #-0x78]
1003625e0: 1400000a    	b	0x100362608 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x204>
1003625e4: f94063e8    	ldr	x8, [sp, #0xc0]
1003625e8: 52800309    	mov	w9, #0x18               ; =24
1003625ec: 9b0922e8    	madd	x8, x23, x9, x8
1003625f0: a9006116    	stp	x22, x24, [x8]
1003625f4: f9000916    	str	x22, [x8, #0x10]
1003625f8: 910006e8    	add	x8, x23, #0x1
1003625fc: f90067e8    	str	x8, [sp, #0xc8]
100362600: 385893a8    	ldurb	w8, [x29, #-0x77]
100362604: 37001488    	tbnz	w8, #0x0, 0x100362894 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x490>
100362608: f85583b6    	ldur	x22, [x29, #-0xa8]
10036260c: d101c3a0    	sub	x0, x29, #0x70
100362610: 91004261    	add	x1, x19, #0x10
100362614: 940230c2    	bl	0x1003ee91c <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
100362618: b85903a8    	ldur	w8, [x29, #-0x70]
10036261c: 7100051f    	cmp	w8, #0x1
100362620: 54000101    	b.ne	0x100362640 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x23c>
100362624: f85a03a8    	ldur	x8, [x29, #-0x60]
100362628: f85483a9    	ldur	x9, [x29, #-0xb8]
10036262c: cb09011a    	sub	x26, x8, x9
100362630: 8b0902d8    	add	x24, x22, x9
100362634: f81483a8    	stur	x8, [x29, #-0xb8]
100362638: b50001da    	cbnz	x26, 0x100362670 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x26c>
10036263c: 1400001a    	b	0x1003626a4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x2a0>
100362640: 385893a8    	ldurb	w8, [x29, #-0x77]
100362644: 37001288    	tbnz	w8, #0x0, 0x100362894 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x490>
100362648: 381893bb    	sturb	w27, [x29, #-0x77]
10036264c: 385883aa    	ldurb	w10, [x29, #-0x78]
100362650: a974a7a8    	ldp	x8, x9, [x29, #-0xb8]
100362654: 3700006a    	tbnz	w10, #0x0, 0x100362660 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x25c>
100362658: eb08013f    	cmp	x9, x8
10036265c: 540011c0    	b.eq	0x100362894 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x490>
100362660: f85583aa    	ldur	x10, [x29, #-0xa8]
100362664: cb08013a    	sub	x26, x9, x8
100362668: 8b080158    	add	x24, x10, x8
10036266c: b40001da    	cbz	x26, 0x1003626a4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x2a0>
100362670: 8b1a0308    	add	x8, x24, x26
100362674: 385ff109    	ldurb	w9, [x8, #-0x1]
100362678: 7100293f    	cmp	w9, #0xa
10036267c: 54000141    	b.ne	0x1003626a4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x2a0>
100362680: f1000749    	subs	x9, x26, #0x1
100362684: 540000e0    	b.eq	0x1003626a0 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x29c>
100362688: 385fe108    	ldurb	w8, [x8, #-0x2]
10036268c: d1000b4a    	sub	x10, x26, #0x2
100362690: 7100351f    	cmp	w8, #0xd
100362694: 9a981318    	csel	x24, x24, x24, ne
100362698: 9a8a113a    	csel	x26, x9, x10, ne
10036269c: 14000002    	b	0x1003626a4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x2a0>
1003626a0: d280001a    	mov	x26, #0x0               ; =0
1003626a4: aa1803e0    	mov	x0, x24
1003626a8: aa1a03e1    	mov	x1, x26
1003626ac: 97fb32ab    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1003626b0: aa0003f6    	mov	x22, x0
1003626b4: aa0103f7    	mov	x23, x1
1003626b8: d102e7a0    	sub	x0, x29, #0xb9
1003626bc: aa1803e1    	mov	x1, x24
1003626c0: aa1a03e2    	mov	x2, x26
1003626c4: 9410acd6    	bl	0x10078da1c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE>
1003626c8: 34fff9c0    	cbz	w0, 0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
1003626cc: b4fff9b7    	cbz	x23, 0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
1003626d0: 394002c8    	ldrb	w8, [x22]
1003626d4: 71016d1f    	cmp	w8, #0x5b
1003626d8: 54fff941    	b.ne	0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
1003626dc: 8b1702c8    	add	x8, x22, x23
1003626e0: 385ff108    	ldurb	w8, [x8, #-0x1]
1003626e4: 7101751f    	cmp	w8, #0x5d
1003626e8: 54fff8c1    	b.ne	0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
1003626ec: f10006e3    	subs	x3, x23, #0x1
1003626f0: 54001180    	b.eq	0x100362920 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x51c>
1003626f4: 39c006c8    	ldrsb	w8, [x22, #0x1]
1003626f8: 3101051f    	cmn	w8, #0x41
1003626fc: 5400112d    	b.le	0x100362920 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x51c>
100362700: d1000ae8    	sub	x8, x23, #0x2
100362704: f100411f    	cmp	x8, #0x10
100362708: 54000082    	b.hs	0x100362718 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x314>
10036270c: f100251f    	cmp	x8, #0x9
100362710: 54fff783    	b.lo	0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
100362714: 1400000e    	b	0x10036274c <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x348>
100362718: f84012c8    	ldur	x8, [x22, #0x1]
10036271c: f84092c9    	ldur	x9, [x22, #0x9]
100362720: d28dedaa    	mov	x10, #0x6f6d            ; =28525
100362724: f2acac8a    	movk	x10, #0x6564, lsl #16
100362728: f2cbed8a    	movk	x10, #0x5f6c, lsl #32
10036272c: f2ee4e0a    	movk	x10, #0x7270, lsl #48
100362730: eb0a011f    	cmp	x8, x10
100362734: d28ecde8    	mov	x8, #0x766f             ; =30319
100362738: f2ac8d28    	movk	x8, #0x6469, lsl #16
10036273c: f2ce4ca8    	movk	x8, #0x7265, lsl #32
100362740: f2e5ce68    	movk	x8, #0x2e73, lsl #48
100362744: fa480120    	ccmp	x9, x8, #0x0, eq
100362748: 540001c0    	b.eq	0x100362780 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x37c>
10036274c: f84012c8    	ldur	x8, [x22, #0x1]
100362750: 394026c9    	ldrb	w9, [x22, #0x9]
100362754: d28e4e0a    	mov	x10, #0x7270            ; =29296
100362758: f2accdea    	movk	x10, #0x666f, lsl #16
10036275c: f2cd8d2a    	movk	x10, #0x6c69, lsl #32
100362760: f2ee6caa    	movk	x10, #0x7365, lsl #48
100362764: eb0a011f    	cmp	x8, x10
100362768: 528005c8    	mov	w8, #0x2e               ; =46
10036276c: 7a480120    	ccmp	w9, w8, #0x0, eq
100362770: 54fff481    	b.ne	0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
100362774: d1002ee1    	sub	x1, x23, #0xb
100362778: 91002ac0    	add	x0, x22, #0xa
10036277c: 14000003    	b	0x100362788 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x384>
100362780: d1004ae1    	sub	x1, x23, #0x12
100362784: 910046c0    	add	x0, x22, #0x11
100362788: 52800442    	mov	w2, #0x22               ; =34
10036278c: 97fb3304    	bl	0x10022f39c <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h3c87e74f9c47ae93E>
100362790: aa0003f7    	mov	x23, x0
100362794: aa0103f6    	mov	x22, x1
100362798: f100303f    	cmp	x1, #0xc
10036279c: 54000161    	b.ne	0x1003627c8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x3c4>
1003627a0: f94002e8    	ldr	x8, [x23]
1003627a4: 9000824b    	adrp	x11, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
1003627a8: 912de56b    	add	x11, x11, #0xb79
1003627ac: f9400169    	ldr	x9, [x11]
1003627b0: b9400aea    	ldr	w10, [x23, #0x8]
1003627b4: b940096b    	ldr	w11, [x11, #0x8]
1003627b8: eb09011f    	cmp	x8, x9
1003627bc: fa4b0140    	ccmp	x10, x11, #0x0, eq
1003627c0: 54fff201    	b.ne	0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
1003627c4: 14000024    	b	0x100362854 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x450>
1003627c8: f10036df    	cmp	x22, #0xd
1003627cc: 54000143    	b.lo	0x1003627f4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x3f0>
1003627d0: 9000824a    	adrp	x10, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
1003627d4: 912e154a    	add	x10, x10, #0xb85
1003627d8: f9400148    	ldr	x8, [x10]
1003627dc: f94002e9    	ldr	x9, [x23]
1003627e0: f840514a    	ldur	x10, [x10, #0x5]
1003627e4: f84052eb    	ldur	x11, [x23, #0x5]
1003627e8: eb09011f    	cmp	x8, x9
1003627ec: fa4b0140    	ccmp	x10, x11, #0x0, eq
1003627f0: 54000320    	b.eq	0x100362854 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x450>
1003627f4: f1001adf    	cmp	x22, #0x6
1003627f8: 540001c0    	b.eq	0x100362830 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x42c>
1003627fc: f1004edf    	cmp	x22, #0x13
100362800: 54fff001    	b.ne	0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
100362804: a94026e8    	ldp	x8, x9, [x23]
100362808: 9000824d    	adrp	x13, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10036280c: 9132e9ad    	add	x13, x13, #0xcba
100362810: a9402daa    	ldp	x10, x11, [x13]
100362814: f840b2ec    	ldur	x12, [x23, #0xb]
100362818: f840b1ad    	ldur	x13, [x13, #0xb]
10036281c: eb0a011f    	cmp	x8, x10
100362820: fa4b0120    	ccmp	x9, x11, #0x0, eq
100362824: fa4d0180    	ccmp	x12, x13, #0x0, eq
100362828: 54ffeec1    	b.ne	0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
10036282c: 1400000a    	b	0x100362854 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x450>
100362830: b94002e8    	ldr	w8, [x23]
100362834: 9000824b    	adrp	x11, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100362838: 9122c56b    	add	x11, x11, #0x8b1
10036283c: b9400169    	ldr	w9, [x11]
100362840: 79400aea    	ldrh	w10, [x23, #0x4]
100362844: 7940096b    	ldrh	w11, [x11, #0x4]
100362848: 6b09011f    	cmp	w8, w9
10036284c: 7a4b0140    	ccmp	w10, w11, #0x0, eq
100362850: 54ffed81    	b.ne	0x100362600 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1fc>
100362854: 97f27b48    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100362858: aa1603e0    	mov	x0, x22
10036285c: 52800021    	mov	w1, #0x1                ; =1
100362860: 97f27b41    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100362864: b40006c0    	cbz	x0, 0x10036293c <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x538>
100362868: aa0003f8    	mov	x24, x0
10036286c: aa1703e1    	mov	x1, x23
100362870: aa1603e2    	mov	x2, x22
100362874: 9439bb8e    	bl	0x1011d16ac <_writev+0x1011d16ac>
100362878: f94067f7    	ldr	x23, [sp, #0xc8]
10036287c: f9405fe8    	ldr	x8, [sp, #0xb8]
100362880: eb0802ff    	cmp	x23, x8
100362884: 54ffeb01    	b.ne	0x1003625e4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1e0>
100362888: 9102e3e0    	add	x0, sp, #0xb8
10036288c: 9439b55d    	bl	0x1011cfe00 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hca26f846e3ace0fdE>
100362890: 17ffff55    	b	0x1003625e4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x1e0>
100362894: 3ccb83e0    	ldur	q0, [sp, #0xb8]
100362898: 3d800280    	str	q0, [x20]
10036289c: f94067e8    	ldr	x8, [sp, #0xc8]
1003628a0: f9000a88    	str	x8, [x20, #0x10]
1003628a4: 3dc017e0    	ldr	q0, [sp, #0x50]
1003628a8: 3c818280    	stur	q0, [x20, #0x18]
1003628ac: f94033e8    	ldr	x8, [sp, #0x60]
1003628b0: f9001688    	str	x8, [x20, #0x28]
1003628b4: 3cc683e0    	ldur	q0, [sp, #0x68]
1003628b8: 3d800e80    	str	q0, [x20, #0x30]
1003628bc: f9403fe8    	ldr	x8, [sp, #0x78]
1003628c0: f9002288    	str	x8, [x20, #0x40]
1003628c4: 3dc023e0    	ldr	q0, [sp, #0x80]
1003628c8: 3c848280    	stur	q0, [x20, #0x48]
1003628cc: f9404be8    	ldr	x8, [sp, #0x90]
1003628d0: f9002e88    	str	x8, [x20, #0x58]
1003628d4: 3dc02be0    	ldr	q0, [sp, #0xa0]
1003628d8: 3d801a80    	str	q0, [x20, #0x60]
1003628dc: f9405be8    	ldr	x8, [sp, #0xb0]
1003628e0: f9003a88    	str	x8, [x20, #0x70]
1003628e4: 3901f69c    	strb	w28, [x20, #0x7d]
1003628e8: 3901e299    	strb	w25, [x20, #0x78]
1003628ec: b9404fe8    	ldr	w8, [sp, #0x4c]
1003628f0: 3901e688    	strb	w8, [x20, #0x79]
1003628f4: 53107ea8    	lsr	w8, w21, #16
1003628f8: 3901f288    	strb	w8, [x20, #0x7c]
1003628fc: 7900f695    	strh	w21, [x20, #0x7a]
100362900: a9597bfd    	ldp	x29, x30, [sp, #0x190]
100362904: a9584ff4    	ldp	x20, x19, [sp, #0x180]
100362908: a95757f6    	ldp	x22, x21, [sp, #0x170]
10036290c: a9565ff8    	ldp	x24, x23, [sp, #0x160]
100362910: a95567fa    	ldp	x26, x25, [sp, #0x150]
100362914: a9546ffc    	ldp	x28, x27, [sp, #0x140]
100362918: 910683ff    	add	sp, sp, #0x1a0
10036291c: d65f03c0    	ret
100362920: 9000a644    	adrp	x4, 0x10182a000 <_anon.cdb272158a04aae91ce02c6e47f106c7.1+0x6d0>
100362924: 9133a084    	add	x4, x4, #0xce8
100362928: aa1603e0    	mov	x0, x22
10036292c: aa1703e1    	mov	x1, x23
100362930: 52800022    	mov	w2, #0x1                ; =1
100362934: 9439b7b9    	bl	0x1011d0818 <__ZN4core3str16slice_error_fail17hecde84adb67345c5E>
100362938: 14000004    	b	0x100362948 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x544>
10036293c: 52800020    	mov	w0, #0x1                ; =1
100362940: aa1603e1    	mov	x1, x22
100362944: 9439b748    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100362948: d4200020    	brk	#0x1
10036294c: aa0003f4    	mov	x20, x0
100362950: aa1803e0    	mov	x0, x24
100362954: aa1603e1    	mov	x1, x22
100362958: 52800022    	mov	w2, #0x1                ; =1
10036295c: 97f27b03    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100362960: 14000035    	b	0x100362a34 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x630>
100362964: 14000033    	b	0x100362a30 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x62c>
100362968: aa0003f4    	mov	x20, x0
10036296c: b40001f5    	cbz	x21, 0x1003629a8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x5a4>
100362970: aa1803e0    	mov	x0, x24
100362974: aa1503e1    	mov	x1, x21
100362978: 52800022    	mov	w2, #0x1                ; =1
10036297c: 97f27afb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100362980: 1400000a    	b	0x1003629a8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x5a4>
100362984: aa0003f4    	mov	x20, x0
100362988: 14000018    	b	0x1003629e8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x5e4>
10036298c: aa0003f4    	mov	x20, x0
100362990: 1400000e    	b	0x1003629c8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x5c4>
100362994: aa0003f4    	mov	x20, x0
100362998: 14000014    	b	0x1003629e8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x5e4>
10036299c: aa0003f4    	mov	x20, x0
1003629a0: 1400001a    	b	0x100362a08 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x604>
1003629a4: aa0003f4    	mov	x20, x0
1003629a8: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1003629ac: eb08027f    	cmp	x19, x8
1003629b0: 540000c0    	b.eq	0x1003629c8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x5c4>
1003629b4: b40000b3    	cbz	x19, 0x1003629c8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x5c4>
1003629b8: f94057e0    	ldr	x0, [sp, #0xa8]
1003629bc: aa1303e1    	mov	x1, x19
1003629c0: 52800022    	mov	w2, #0x1                ; =1
1003629c4: 97f27ae9    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003629c8: f94043e1    	ldr	x1, [sp, #0x80]
1003629cc: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1003629d0: eb08003f    	cmp	x1, x8
1003629d4: 540000a0    	b.eq	0x1003629e8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x5e4>
1003629d8: b4000081    	cbz	x1, 0x1003629e8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x5e4>
1003629dc: f94047e0    	ldr	x0, [sp, #0x88]
1003629e0: 52800022    	mov	w2, #0x1                ; =1
1003629e4: 97f27ae1    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003629e8: f94037e1    	ldr	x1, [sp, #0x68]
1003629ec: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1003629f0: eb08003f    	cmp	x1, x8
1003629f4: 540000a0    	b.eq	0x100362a08 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x604>
1003629f8: b4000081    	cbz	x1, 0x100362a08 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x604>
1003629fc: f9403be0    	ldr	x0, [sp, #0x70]
100362a00: 52800022    	mov	w2, #0x1                ; =1
100362a04: 97f27ad9    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100362a08: f9402be1    	ldr	x1, [sp, #0x50]
100362a0c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100362a10: eb08003f    	cmp	x1, x8
100362a14: fa401824    	ccmp	x1, #0x0, #0x4, ne
100362a18: 540005e0    	b.eq	0x100362ad4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x6d0>
100362a1c: f9402fe0    	ldr	x0, [sp, #0x58]
100362a20: 1400002b    	b	0x100362acc <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x6c8>
100362a24: aa0003f4    	mov	x20, x0
100362a28: 14000005    	b	0x100362a3c <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x638>
100362a2c: 14000001    	b	0x100362a30 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x62c>
100362a30: aa0003f4    	mov	x20, x0
100362a34: 9102e3e0    	add	x0, sp, #0xb8
100362a38: 9401d645    	bl	0x1003d834c <__ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h265e755c5c638973E>
100362a3c: f94023e8    	ldr	x8, [sp, #0x40]
100362a40: f240f91f    	tst	x8, #0x7fffffffffffffff
100362a44: 54000161    	b.ne	0x100362a70 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x66c>
100362a48: f94017e8    	ldr	x8, [sp, #0x28]
100362a4c: f240f91f    	tst	x8, #0x7fffffffffffffff
100362a50: 540001e1    	b.ne	0x100362a8c <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x688>
100362a54: f9401be8    	ldr	x8, [sp, #0x30]
100362a58: f240f91f    	tst	x8, #0x7fffffffffffffff
100362a5c: 54000261    	b.ne	0x100362aa8 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x6a4>
100362a60: f9401fe8    	ldr	x8, [sp, #0x38]
100362a64: f240f91f    	tst	x8, #0x7fffffffffffffff
100362a68: 54000360    	b.eq	0x100362ad4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x6d0>
100362a6c: 14000016    	b	0x100362ac4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x6c0>
100362a70: f94007e0    	ldr	x0, [sp, #0x8]
100362a74: f94023e1    	ldr	x1, [sp, #0x40]
100362a78: 52800022    	mov	w2, #0x1                ; =1
100362a7c: 97f27abb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100362a80: f94017e8    	ldr	x8, [sp, #0x28]
100362a84: f240f91f    	tst	x8, #0x7fffffffffffffff
100362a88: 54fffe60    	b.eq	0x100362a54 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x650>
100362a8c: f9400be0    	ldr	x0, [sp, #0x10]
100362a90: f94017e1    	ldr	x1, [sp, #0x28]
100362a94: 52800022    	mov	w2, #0x1                ; =1
100362a98: 97f27ab4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100362a9c: f9401be8    	ldr	x8, [sp, #0x30]
100362aa0: f240f91f    	tst	x8, #0x7fffffffffffffff
100362aa4: 54fffde0    	b.eq	0x100362a60 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x65c>
100362aa8: f9400fe0    	ldr	x0, [sp, #0x18]
100362aac: f9401be1    	ldr	x1, [sp, #0x30]
100362ab0: 52800022    	mov	w2, #0x1                ; =1
100362ab4: 97f27aad    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100362ab8: f9401fe8    	ldr	x8, [sp, #0x38]
100362abc: f240f91f    	tst	x8, #0x7fffffffffffffff
100362ac0: 540000a0    	b.eq	0x100362ad4 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE+0x6d0>
100362ac4: f94013e0    	ldr	x0, [sp, #0x20]
100362ac8: f9401fe1    	ldr	x1, [sp, #0x38]
100362acc: 52800022    	mov	w2, #0x1                ; =1
100362ad0: 97f27aa6    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100362ad4: aa1403e0    	mov	x0, x20
100362ad8: 9439b99f    	bl	0x1011d1154 <_writev+0x1011d1154>

