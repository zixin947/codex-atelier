
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100389264 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE>:
100389264: d10203ff    	sub	sp, sp, #0x80
100389268: a9026ffc    	stp	x28, x27, [sp, #0x20]
10038926c: a90367fa    	stp	x26, x25, [sp, #0x30]
100389270: a9045ff8    	stp	x24, x23, [sp, #0x40]
100389274: a90557f6    	stp	x22, x21, [sp, #0x50]
100389278: a9064ff4    	stp	x20, x19, [sp, #0x60]
10038927c: a9077bfd    	stp	x29, x30, [sp, #0x70]
100389280: 9101c3fd    	add	x29, sp, #0x70
100389284: aa0203f6    	mov	x22, x2
100389288: b6f800a2    	tbz	x2, #0x3f, 0x10038929c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x38>
10038928c: d2800015    	mov	x21, #0x0               ; =0
100389290: aa1503e0    	mov	x0, x21
100389294: aa1603e1    	mov	x1, x22
100389298: 94391cf3    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038929c: aa0003f3    	mov	x19, x0
1003892a0: b4002ad6    	cbz	x22, 0x1003897f8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x594>
1003892a4: aa0103f4    	mov	x20, x1
1003892a8: 97f1e0b3    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003892ac: 52800035    	mov	w21, #0x1               ; =1
1003892b0: aa1603e0    	mov	x0, x22
1003892b4: 52800021    	mov	w1, #0x1                ; =1
1003892b8: 97f1e0ab    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003892bc: b4fffea0    	cbz	x0, 0x100389290 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x2c>
1003892c0: d2800015    	mov	x21, #0x0               ; =0
1003892c4: 8b160297    	add	x23, x20, x22
1003892c8: 52800038    	mov	w24, #0x1               ; =1
1003892cc: a90083f6    	stp	x22, x0, [sp, #0x8]
1003892d0: 52800b9a    	mov	w26, #0x5c              ; =92
1003892d4: f9000fff    	str	xzr, [sp, #0x18]
1003892d8: 5280005c    	mov	w28, #0x2               ; =2
1003892dc: 52800079    	mov	w25, #0x3               ; =3
1003892e0: 1400001a    	b	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
1003892e4: 3900011b    	strb	w27, [x8]
1003892e8: 8b1502d5    	add	x21, x22, x21
1003892ec: f9000ff5    	str	x21, [sp, #0x18]
1003892f0: eb17029f    	cmp	x20, x23
1003892f4: 540002a1    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
1003892f8: 1400014d    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
1003892fc: aa1503e9    	mov	x9, x21
100389300: eb15011f    	cmp	x8, x21
100389304: 54000120    	b.eq	0x100389328 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xc4>
100389308: f9400be0    	ldr	x0, [sp, #0x10]
10038930c: 52800448    	mov	w8, #0x22               ; =34
100389310: 38296808    	strb	w8, [x0, x9]
100389314: 910006b5    	add	x21, x21, #0x1
100389318: f9000ff5    	str	x21, [sp, #0x18]
10038931c: eb17029f    	cmp	x20, x23
100389320: 54000141    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
100389324: 14000142    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
100389328: 910023e0    	add	x0, sp, #0x8
10038932c: aa1503e1    	mov	x1, x21
100389330: 52800022    	mov	w2, #0x1                ; =1
100389334: 52800023    	mov	w3, #0x1                ; =1
100389338: 52800024    	mov	w4, #0x1                ; =1
10038933c: 94381566    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100389340: f9400fe9    	ldr	x9, [sp, #0x18]
100389344: 17fffff1    	b	0x100389308 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xa4>
100389348: aa1403e8    	mov	x8, x20
10038934c: 38c01509    	ldrsb	w9, [x8], #0x1
100389350: 12001d3b    	and	w27, w9, #0xff
100389354: 37f803e9    	tbnz	w9, #0x1f, 0x1003893d0 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x16c>
100389358: aa0803f4    	mov	x20, x8
10038935c: 7101737f    	cmp	w27, #0x5c
100389360: 54000661    	b.ne	0x10038942c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x1c8>
100389364: eb17029f    	cmp	x20, x23
100389368: 54002500    	b.eq	0x100389808 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5a4>
10038936c: aa1403e8    	mov	x8, x20
100389370: 38c01509    	ldrsb	w9, [x8], #0x1
100389374: 12001d3b    	and	w27, w9, #0xff
100389378: 37f80909    	tbnz	w9, #0x1f, 0x100389498 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x234>
10038937c: aa0803f4    	mov	x20, x8
100389380: f94007e8    	ldr	x8, [sp, #0x8]
100389384: 7101977f    	cmp	w27, #0x65
100389388: 54000fed    	b.le	0x100389584 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x320>
10038938c: 7101c77f    	cmp	w27, #0x71
100389390: 54000c6c    	b.gt	0x10038951c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x2b8>
100389394: 71019b7f    	cmp	w27, #0x66
100389398: 54001560    	b.eq	0x100389644 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x3e0>
10038939c: 7101bb7f    	cmp	w27, #0x6e
1003893a0: 540012e1    	b.ne	0x1003895fc <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x398>
1003893a4: aa1503e9    	mov	x9, x21
1003893a8: eb15011f    	cmp	x8, x21
1003893ac: 54001e60    	b.eq	0x100389778 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x514>
1003893b0: f9400be0    	ldr	x0, [sp, #0x10]
1003893b4: 52800148    	mov	w8, #0xa                ; =10
1003893b8: 38296808    	strb	w8, [x0, x9]
1003893bc: 910006b5    	add	x21, x21, #0x1
1003893c0: f9000ff5    	str	x21, [sp, #0x18]
1003893c4: eb17029f    	cmp	x20, x23
1003893c8: 54fffc01    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
1003893cc: 14000118    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
1003893d0: 39400688    	ldrb	w8, [x20, #0x1]
1003893d4: 12001509    	and	w9, w8, #0x3f
1003893d8: 7103837f    	cmp	w27, #0xe0
1003893dc: 540001e3    	b.lo	0x100389418 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x1b4>
1003893e0: 12001368    	and	w8, w27, #0x1f
1003893e4: 39400a8a    	ldrb	w10, [x20, #0x2]
1003893e8: 1200154a    	and	w10, w10, #0x3f
1003893ec: 2a091949    	orr	w9, w10, w9, lsl #6
1003893f0: 7103c37f    	cmp	w27, #0xf0
1003893f4: 54000b23    	b.lo	0x100389558 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x2f4>
1003893f8: 39400e8a    	ldrb	w10, [x20, #0x3]
1003893fc: 91001294    	add	x20, x20, #0x4
100389400: 1200154a    	and	w10, w10, #0x3f
100389404: 2a09195b    	orr	w27, w10, w9, lsl #6
100389408: 330e091b    	bfi	w27, w8, #18, #3
10038940c: 7101737f    	cmp	w27, #0x5c
100389410: 54fffaa0    	b.eq	0x100389364 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x100>
100389414: 14000006    	b	0x10038942c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x1c8>
100389418: 91000a94    	add	x20, x20, #0x2
10038941c: 331a1369    	bfi	w9, w27, #6, #5
100389420: aa0903fb    	mov	x27, x9
100389424: 7101737f    	cmp	w27, #0x5c
100389428: 54fff9e0    	b.eq	0x100389364 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x100>
10038942c: 7140437f    	cmp	w27, #0x10, lsl #12     ; =0x10000
100389430: 9a993728    	cinc	x8, x25, hs
100389434: 7120037f    	cmp	w27, #0x800
100389438: 9a883388    	csel	x8, x28, x8, lo
10038943c: 7102037f    	cmp	w27, #0x80
100389440: 9a883316    	csel	x22, x24, x8, lo
100389444: f94007e8    	ldr	x8, [sp, #0x8]
100389448: cb150108    	sub	x8, x8, x21
10038944c: eb0802df    	cmp	x22, x8
100389450: aa1503e8    	mov	x8, x21
100389454: 54001448    	b.hi	0x1003896dc <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x478>
100389458: 8b080008    	add	x8, x0, x8
10038945c: 7102037f    	cmp	w27, #0x80
100389460: 54fff423    	b.lo	0x1003892e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x80>
100389464: 12800fe9    	mov	w9, #-0x80              ; =-128
100389468: 33001769    	bfxil	w9, w27, #0, #6
10038946c: 7120037f    	cmp	w27, #0x800
100389470: 540003a2    	b.hs	0x1003894e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x280>
100389474: 53067f6a    	lsr	w10, w27, #6
100389478: 321a054a    	orr	w10, w10, #0xc0
10038947c: 3900010a    	strb	w10, [x8]
100389480: 39000509    	strb	w9, [x8, #0x1]
100389484: 8b1502d5    	add	x21, x22, x21
100389488: f9000ff5    	str	x21, [sp, #0x18]
10038948c: eb17029f    	cmp	x20, x23
100389490: 54fff5c1    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
100389494: 140000e6    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
100389498: 39400688    	ldrb	w8, [x20, #0x1]
10038949c: 12001509    	and	w9, w8, #0x3f
1003894a0: 7103837f    	cmp	w27, #0xe0
1003894a4: 54000643    	b.lo	0x10038956c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x308>
1003894a8: 12001368    	and	w8, w27, #0x1f
1003894ac: 39400a8a    	ldrb	w10, [x20, #0x2]
1003894b0: 1200154a    	and	w10, w10, #0x3f
1003894b4: 2a091949    	orr	w9, w10, w9, lsl #6
1003894b8: 7103c37f    	cmp	w27, #0xf0
1003894bc: 54001043    	b.lo	0x1003896c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x460>
1003894c0: 39400e8a    	ldrb	w10, [x20, #0x3]
1003894c4: 91001294    	add	x20, x20, #0x4
1003894c8: 1200154a    	and	w10, w10, #0x3f
1003894cc: 2a09195b    	orr	w27, w10, w9, lsl #6
1003894d0: 330e091b    	bfi	w27, w8, #18, #3
1003894d4: f94007e8    	ldr	x8, [sp, #0x8]
1003894d8: 7101977f    	cmp	w27, #0x65
1003894dc: 54fff58c    	b.gt	0x10038938c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x128>
1003894e0: 14000029    	b	0x100389584 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x320>
1003894e4: 12800fea    	mov	w10, #-0x80             ; =-128
1003894e8: 33062f6a    	bfxil	w10, w27, #6, #6
1003894ec: 53107f6b    	lsr	w11, w27, #16
1003894f0: 350006cb    	cbnz	w11, 0x1003895c8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x364>
1003894f4: 530c7f6b    	lsr	w11, w27, #12
1003894f8: 321b096b    	orr	w11, w11, #0xe0
1003894fc: 3900010b    	strb	w11, [x8]
100389500: 3900050a    	strb	w10, [x8, #0x1]
100389504: 39000909    	strb	w9, [x8, #0x2]
100389508: 8b1502d5    	add	x21, x22, x21
10038950c: f9000ff5    	str	x21, [sp, #0x18]
100389510: eb17029f    	cmp	x20, x23
100389514: 54fff1a1    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
100389518: 140000c5    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
10038951c: 7101cb7f    	cmp	w27, #0x72
100389520: 54000a80    	b.eq	0x100389670 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x40c>
100389524: 7101d37f    	cmp	w27, #0x74
100389528: 540006a1    	b.ne	0x1003895fc <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x398>
10038952c: aa1503e9    	mov	x9, x21
100389530: eb15011f    	cmp	x8, x21
100389534: 54001320    	b.eq	0x100389798 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x534>
100389538: f9400be0    	ldr	x0, [sp, #0x10]
10038953c: 52800128    	mov	w8, #0x9                ; =9
100389540: 38296808    	strb	w8, [x0, x9]
100389544: 910006b5    	add	x21, x21, #0x1
100389548: f9000ff5    	str	x21, [sp, #0x18]
10038954c: eb17029f    	cmp	x20, x23
100389550: 54ffefc1    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
100389554: 140000b6    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
100389558: 91000e94    	add	x20, x20, #0x3
10038955c: 2a08313b    	orr	w27, w9, w8, lsl #12
100389560: 7101737f    	cmp	w27, #0x5c
100389564: 54fff000    	b.eq	0x100389364 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x100>
100389568: 17ffffb1    	b	0x10038942c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x1c8>
10038956c: 91000a94    	add	x20, x20, #0x2
100389570: 331a1369    	bfi	w9, w27, #6, #5
100389574: aa0903fb    	mov	x27, x9
100389578: f94007e8    	ldr	x8, [sp, #0x8]
10038957c: 7101977f    	cmp	w27, #0x65
100389580: 54fff06c    	b.gt	0x10038938c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x128>
100389584: 71008b7f    	cmp	w27, #0x22
100389588: 54ffeba0    	b.eq	0x1003892fc <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x98>
10038958c: 7101737f    	cmp	w27, #0x5c
100389590: 54000860    	b.eq	0x10038969c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x438>
100389594: 71018b7f    	cmp	w27, #0x62
100389598: 54000321    	b.ne	0x1003895fc <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x398>
10038959c: aa1503e9    	mov	x9, x21
1003895a0: eb15011f    	cmp	x8, x21
1003895a4: 540010a0    	b.eq	0x1003897b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x554>
1003895a8: f9400be0    	ldr	x0, [sp, #0x10]
1003895ac: 52800108    	mov	w8, #0x8                ; =8
1003895b0: 38296808    	strb	w8, [x0, x9]
1003895b4: 910006b5    	add	x21, x21, #0x1
1003895b8: f9000ff5    	str	x21, [sp, #0x18]
1003895bc: eb17029f    	cmp	x20, x23
1003895c0: 54ffec41    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
1003895c4: 1400009a    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
1003895c8: 12800feb    	mov	w11, #-0x80             ; =-128
1003895cc: 53127f6c    	lsr	w12, w27, #18
1003895d0: 321c6d8c    	orr	w12, w12, #0xfffffff0
1003895d4: 3900010c    	strb	w12, [x8]
1003895d8: 330c476b    	bfxil	w11, w27, #12, #6
1003895dc: 3900050b    	strb	w11, [x8, #0x1]
1003895e0: 3900090a    	strb	w10, [x8, #0x2]
1003895e4: 39000d09    	strb	w9, [x8, #0x3]
1003895e8: 8b1502d5    	add	x21, x22, x21
1003895ec: f9000ff5    	str	x21, [sp, #0x18]
1003895f0: eb17029f    	cmp	x20, x23
1003895f4: 54ffeaa1    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
1003895f8: 1400008d    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
1003895fc: aa1503e9    	mov	x9, x21
100389600: eb15011f    	cmp	x8, x21
100389604: 540007c0    	b.eq	0x1003896fc <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x498>
100389608: a94083e8    	ldp	x8, x0, [sp, #0x8]
10038960c: 3829681a    	strb	w26, [x0, x9]
100389610: 910006b5    	add	x21, x21, #0x1
100389614: f9000ff5    	str	x21, [sp, #0x18]
100389618: 7140437f    	cmp	w27, #0x10, lsl #12     ; =0x10000
10038961c: 9a993729    	cinc	x9, x25, hs
100389620: 7120037f    	cmp	w27, #0x800
100389624: 9a893389    	csel	x9, x28, x9, lo
100389628: 7102037f    	cmp	w27, #0x80
10038962c: 9a893316    	csel	x22, x24, x9, lo
100389630: cb150108    	sub	x8, x8, x21
100389634: eb0802df    	cmp	x22, x8
100389638: 54000928    	b.hi	0x10038975c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x4f8>
10038963c: aa1503e8    	mov	x8, x21
100389640: 17ffff86    	b	0x100389458 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x1f4>
100389644: aa1503e9    	mov	x9, x21
100389648: eb15011f    	cmp	x8, x21
10038964c: 54000680    	b.eq	0x10038971c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x4b8>
100389650: f9400be0    	ldr	x0, [sp, #0x10]
100389654: 52800188    	mov	w8, #0xc                ; =12
100389658: 38296808    	strb	w8, [x0, x9]
10038965c: 910006b5    	add	x21, x21, #0x1
100389660: f9000ff5    	str	x21, [sp, #0x18]
100389664: eb17029f    	cmp	x20, x23
100389668: 54ffe701    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
10038966c: 14000070    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
100389670: aa1503e9    	mov	x9, x21
100389674: eb15011f    	cmp	x8, x21
100389678: 54000620    	b.eq	0x10038973c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x4d8>
10038967c: f9400be0    	ldr	x0, [sp, #0x10]
100389680: 528001a8    	mov	w8, #0xd                ; =13
100389684: 38296808    	strb	w8, [x0, x9]
100389688: 910006b5    	add	x21, x21, #0x1
10038968c: f9000ff5    	str	x21, [sp, #0x18]
100389690: eb17029f    	cmp	x20, x23
100389694: 54ffe5a1    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
100389698: 14000065    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
10038969c: aa1503e9    	mov	x9, x21
1003896a0: eb15011f    	cmp	x8, x21
1003896a4: 540009a0    	b.eq	0x1003897d8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x574>
1003896a8: f9400be0    	ldr	x0, [sp, #0x10]
1003896ac: 3829681a    	strb	w26, [x0, x9]
1003896b0: 910006b5    	add	x21, x21, #0x1
1003896b4: f9000ff5    	str	x21, [sp, #0x18]
1003896b8: eb17029f    	cmp	x20, x23
1003896bc: 54ffe461    	b.ne	0x100389348 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0xe4>
1003896c0: 1400005b    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
1003896c4: 91000e94    	add	x20, x20, #0x3
1003896c8: 2a08313b    	orr	w27, w9, w8, lsl #12
1003896cc: f94007e8    	ldr	x8, [sp, #0x8]
1003896d0: 7101977f    	cmp	w27, #0x65
1003896d4: 54ffe5cc    	b.gt	0x10038938c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x128>
1003896d8: 17ffffab    	b	0x100389584 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x320>
1003896dc: 910023e0    	add	x0, sp, #0x8
1003896e0: aa1503e1    	mov	x1, x21
1003896e4: aa1603e2    	mov	x2, x22
1003896e8: 52800023    	mov	w3, #0x1                ; =1
1003896ec: 52800024    	mov	w4, #0x1                ; =1
1003896f0: 94381479    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
1003896f4: a94123e0    	ldp	x0, x8, [sp, #0x10]
1003896f8: 17ffff58    	b	0x100389458 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x1f4>
1003896fc: 910023e0    	add	x0, sp, #0x8
100389700: aa1503e1    	mov	x1, x21
100389704: 52800022    	mov	w2, #0x1                ; =1
100389708: 52800023    	mov	w3, #0x1                ; =1
10038970c: 52800024    	mov	w4, #0x1                ; =1
100389710: 94381471    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100389714: f9400fe9    	ldr	x9, [sp, #0x18]
100389718: 17ffffbc    	b	0x100389608 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x3a4>
10038971c: 910023e0    	add	x0, sp, #0x8
100389720: aa1503e1    	mov	x1, x21
100389724: 52800022    	mov	w2, #0x1                ; =1
100389728: 52800023    	mov	w3, #0x1                ; =1
10038972c: 52800024    	mov	w4, #0x1                ; =1
100389730: 94381469    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100389734: f9400fe9    	ldr	x9, [sp, #0x18]
100389738: 17ffffc6    	b	0x100389650 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x3ec>
10038973c: 910023e0    	add	x0, sp, #0x8
100389740: aa1503e1    	mov	x1, x21
100389744: 52800022    	mov	w2, #0x1                ; =1
100389748: 52800023    	mov	w3, #0x1                ; =1
10038974c: 52800024    	mov	w4, #0x1                ; =1
100389750: 94381461    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100389754: f9400fe9    	ldr	x9, [sp, #0x18]
100389758: 17ffffc9    	b	0x10038967c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x418>
10038975c: 910023e0    	add	x0, sp, #0x8
100389760: aa1503e1    	mov	x1, x21
100389764: aa1603e2    	mov	x2, x22
100389768: 52800023    	mov	w3, #0x1                ; =1
10038976c: 52800024    	mov	w4, #0x1                ; =1
100389770: 94381459    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100389774: 17ffffe0    	b	0x1003896f4 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x490>
100389778: 910023e0    	add	x0, sp, #0x8
10038977c: aa1503e1    	mov	x1, x21
100389780: 52800022    	mov	w2, #0x1                ; =1
100389784: 52800023    	mov	w3, #0x1                ; =1
100389788: 52800024    	mov	w4, #0x1                ; =1
10038978c: 94381452    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100389790: f9400fe9    	ldr	x9, [sp, #0x18]
100389794: 17ffff07    	b	0x1003893b0 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x14c>
100389798: 910023e0    	add	x0, sp, #0x8
10038979c: aa1503e1    	mov	x1, x21
1003897a0: 52800022    	mov	w2, #0x1                ; =1
1003897a4: 52800023    	mov	w3, #0x1                ; =1
1003897a8: 52800024    	mov	w4, #0x1                ; =1
1003897ac: 9438144a    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
1003897b0: f9400fe9    	ldr	x9, [sp, #0x18]
1003897b4: 17ffff61    	b	0x100389538 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x2d4>
1003897b8: 910023e0    	add	x0, sp, #0x8
1003897bc: aa1503e1    	mov	x1, x21
1003897c0: 52800022    	mov	w2, #0x1                ; =1
1003897c4: 52800023    	mov	w3, #0x1                ; =1
1003897c8: 52800024    	mov	w4, #0x1                ; =1
1003897cc: 94381442    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
1003897d0: f9400fe9    	ldr	x9, [sp, #0x18]
1003897d4: 17ffff75    	b	0x1003895a8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x344>
1003897d8: 910023e0    	add	x0, sp, #0x8
1003897dc: aa1503e1    	mov	x1, x21
1003897e0: 52800022    	mov	w2, #0x1                ; =1
1003897e4: 52800023    	mov	w3, #0x1                ; =1
1003897e8: 52800024    	mov	w4, #0x1                ; =1
1003897ec: 9438143a    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
1003897f0: f9400fe9    	ldr	x9, [sp, #0x18]
1003897f4: 17ffffad    	b	0x1003896a8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x444>
1003897f8: 52800028    	mov	w8, #0x1                ; =1
1003897fc: a900a3ff    	stp	xzr, x8, [sp, #0x8]
100389800: f9000fff    	str	xzr, [sp, #0x18]
100389804: 1400000a    	b	0x10038982c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5c8>
100389808: f94007e9    	ldr	x9, [sp, #0x8]
10038980c: aa1503e8    	mov	x8, x21
100389810: eb15013f    	cmp	x9, x21
100389814: 54000240    	b.eq	0x10038985c <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5f8>
100389818: f9400be9    	ldr	x9, [sp, #0x10]
10038981c: 52800b8a    	mov	w10, #0x5c              ; =92
100389820: 3828692a    	strb	w10, [x9, x8]
100389824: 910006a8    	add	x8, x21, #0x1
100389828: f9000fe8    	str	x8, [sp, #0x18]
10038982c: 3cc083e0    	ldur	q0, [sp, #0x8]
100389830: 3d800260    	str	q0, [x19]
100389834: f9400fe8    	ldr	x8, [sp, #0x18]
100389838: f9000a68    	str	x8, [x19, #0x10]
10038983c: a9477bfd    	ldp	x29, x30, [sp, #0x70]
100389840: a9464ff4    	ldp	x20, x19, [sp, #0x60]
100389844: a94557f6    	ldp	x22, x21, [sp, #0x50]
100389848: a9445ff8    	ldp	x24, x23, [sp, #0x40]
10038984c: a94367fa    	ldp	x26, x25, [sp, #0x30]
100389850: a9426ffc    	ldp	x28, x27, [sp, #0x20]
100389854: 910203ff    	add	sp, sp, #0x80
100389858: d65f03c0    	ret
10038985c: 910023e0    	add	x0, sp, #0x8
100389860: aa1503e1    	mov	x1, x21
100389864: 52800022    	mov	w2, #0x1                ; =1
100389868: 52800023    	mov	w3, #0x1                ; =1
10038986c: 52800024    	mov	w4, #0x1                ; =1
100389870: 94381419    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100389874: f9400fe8    	ldr	x8, [sp, #0x18]
100389878: 17ffffe8    	b	0x100389818 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x5b4>
10038987c: 14000001    	b	0x100389880 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x61c>
100389880: aa0003f3    	mov	x19, x0
100389884: f94007e1    	ldr	x1, [sp, #0x8]
100389888: b4000081    	cbz	x1, 0x100389898 <__ZN13codexmate_lib4core5relay16codex_diagnostic26unescape_toml_basic_string17h6b06e8784f0f892bE+0x634>
10038988c: f9400be0    	ldr	x0, [sp, #0x10]
100389890: 52800022    	mov	w2, #0x1                ; =1
100389894: 97f1df35    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100389898: aa1303e0    	mov	x0, x19
10038989c: 94391e2e    	bl	0x1011d1154 <_writev+0x1011d1154>
