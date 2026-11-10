
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100370184 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E>:
100370184: f1002c3f    	cmp	x1, #0xb
100370188: 5400030c    	b.gt	0x1003701e8 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x64>
10037018c: f1001c3f    	cmp	x1, #0x7
100370190: 540005a0    	b.eq	0x100370244 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0xc0>
100370194: f1002c3f    	cmp	x1, #0xb
100370198: 54000f41    	b.ne	0x100370380 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1fc>
10037019c: d286c5a8    	mov	x8, #0x362d             ; =13869
1003701a0: f2a6a5c8    	movk	x8, #0x352e, lsl #16
1003701a4: f2ce85a8    	movk	x8, #0x742d, lsl #32
1003701a8: f2ecee08    	movk	x8, #0x6770, lsl #48
1003701ac: f9400009    	ldr	x9, [x0]
1003701b0: dac00d29    	rev	x9, x9
1003701b4: eb08013f    	cmp	x9, x8
1003701b8: 54000901    	b.ne	0x1003702d8 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x154>
1003701bc: d28ded88    	mov	x8, #0x6f6c             ; =28524
1003701c0: f2a5ae68    	movk	x8, #0x2d73, lsl #16
1003701c4: f2c5c6c8    	movk	x8, #0x2e36, lsl #32
1003701c8: f2e5a6a8    	movk	x8, #0x2d35, lsl #48
1003701cc: f8403009    	ldur	x9, [x0, #0x3]
1003701d0: dac00d29    	rev	x9, x9
1003701d4: eb08013f    	cmp	x9, x8
1003701d8: 54000801    	b.ne	0x1003702d8 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x154>
1003701dc: 52800008    	mov	w8, #0x0                ; =0
1003701e0: 35000d08    	cbnz	w8, 0x100370380 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1fc>
1003701e4: 14000041    	b	0x1003702e8 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x164>
1003701e8: f100303f    	cmp	x1, #0xc
1003701ec: 54000540    	b.eq	0x100370294 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x110>
1003701f0: f100343f    	cmp	x1, #0xd
1003701f4: 54000c61    	b.ne	0x100370380 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1fc>
1003701f8: d286c5a8    	mov	x8, #0x362d             ; =13869
1003701fc: f2a6a5c8    	movk	x8, #0x352e, lsl #16
100370200: f2ce85a8    	movk	x8, #0x742d, lsl #32
100370204: f2ecee08    	movk	x8, #0x6770, lsl #48
100370208: f9400009    	ldr	x9, [x0]
10037020c: dac00d29    	rev	x9, x9
100370210: eb08013f    	cmp	x9, x8
100370214: 54000701    	b.ne	0x1003702f4 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x170>
100370218: d28e4c28    	mov	x8, #0x7261             ; =29281
10037021c: f2acae48    	movk	x8, #0x6572, lsl #16
100370220: f2c5ae88    	movk	x8, #0x2d74, lsl #32
100370224: f2e5c6c8    	movk	x8, #0x2e36, lsl #48
100370228: f8405009    	ldur	x9, [x0, #0x5]
10037022c: dac00d29    	rev	x9, x9
100370230: eb08013f    	cmp	x9, x8
100370234: 54000601    	b.ne	0x1003702f4 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x170>
100370238: 52800008    	mov	w8, #0x0                ; =0
10037023c: 35000a28    	cbnz	w8, 0x100370380 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1fc>
100370240: 14000031    	b	0x100370304 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x180>
100370244: 528e85a8    	mov	w8, #0x742d             ; =29741
100370248: 72acee08    	movk	w8, #0x6770, lsl #16
10037024c: b9400009    	ldr	w9, [x0]
100370250: 5ac00929    	rev	w9, w9
100370254: 528e85aa    	mov	w10, #0x742d            ; =29741
100370258: 72acee0a    	movk	w10, #0x6770, lsl #16
10037025c: 6b08013f    	cmp	w9, w8
100370260: 54000581    	b.ne	0x100370310 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x18c>
100370264: b8403009    	ldur	w9, [x0, #0x3]
100370268: 5ac00929    	rev	w9, w9
10037026c: 5285c68a    	mov	w10, #0x2e34            ; =11828
100370270: 72a5a6aa    	movk	w10, #0x2d35, lsl #16
100370274: 3200014a    	orr	w10, w10, #0x1
100370278: 6b09015f    	cmp	w10, w9
10037027c: 540004a1    	b.ne	0x100370310 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x18c>
100370280: 52800009    	mov	w9, #0x0                ; =0
100370284: 350004e9    	cbnz	w9, 0x100370320 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x19c>
100370288: 90007ce0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10037028c: 9116e000    	add	x0, x0, #0x5b8
100370290: d65f03c0    	ret
100370294: d286c5a8    	mov	x8, #0x362d             ; =13869
100370298: f2a6a5c8    	movk	x8, #0x352e, lsl #16
10037029c: f2ce85a8    	movk	x8, #0x742d, lsl #32
1003702a0: f2ecee08    	movk	x8, #0x6770, lsl #48
1003702a4: f9400009    	ldr	x9, [x0]
1003702a8: dac00d29    	rev	x9, x9
1003702ac: eb08013f    	cmp	x9, x8
1003702b0: 54000521    	b.ne	0x100370354 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1d0>
1003702b4: 528dcc28    	mov	w8, #0x6e61             ; =28257
1003702b8: 72ad8ea8    	movk	w8, #0x6c75, lsl #16
1003702bc: b9400809    	ldr	w9, [x0, #0x8]
1003702c0: 5ac00929    	rev	w9, w9
1003702c4: 6b08013f    	cmp	w9, w8
1003702c8: 54000461    	b.ne	0x100370354 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1d0>
1003702cc: 52800008    	mov	w8, #0x0                ; =0
1003702d0: 35000588    	cbnz	w8, 0x100370380 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1fc>
1003702d4: 14000024    	b	0x100370364 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1e0>
1003702d8: eb09011f    	cmp	x8, x9
1003702dc: 12800008    	mov	w8, #-0x1               ; =-1
1003702e0: 5a883508    	cneg	w8, w8, hs
1003702e4: 350004e8    	cbnz	w8, 0x100370380 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1fc>
1003702e8: 90007ce0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
1003702ec: 91165000    	add	x0, x0, #0x594
1003702f0: d65f03c0    	ret
1003702f4: eb09011f    	cmp	x8, x9
1003702f8: 12800008    	mov	w8, #-0x1               ; =-1
1003702fc: 5a883508    	cneg	w8, w8, hs
100370300: 35000408    	cbnz	w8, 0x100370380 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1fc>
100370304: 90007ce0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
100370308: 91167c00    	add	x0, x0, #0x59f
10037030c: d65f03c0    	ret
100370310: 6b09015f    	cmp	w10, w9
100370314: 12800009    	mov	w9, #-0x1               ; =-1
100370318: 5a893529    	cneg	w9, w9, hs
10037031c: 34fffb69    	cbz	w9, 0x100370288 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x104>
100370320: b9400009    	ldr	w9, [x0]
100370324: 5ac00929    	rev	w9, w9
100370328: 6b09011f    	cmp	w8, w9
10037032c: 54000221    	b.ne	0x100370370 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1ec>
100370330: 5285c688    	mov	w8, #0x2e34             ; =11828
100370334: 72a5a6a8    	movk	w8, #0x2d35, lsl #16
100370338: b8403009    	ldur	w9, [x0, #0x3]
10037033c: 5ac00929    	rev	w9, w9
100370340: 6b08013f    	cmp	w9, w8
100370344: 54000161    	b.ne	0x100370370 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1ec>
100370348: 52800008    	mov	w8, #0x0                ; =0
10037034c: 350001a8    	cbnz	w8, 0x100370380 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1fc>
100370350: 1400000e    	b	0x100370388 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x204>
100370354: eb09011f    	cmp	x8, x9
100370358: 12800008    	mov	w8, #-0x1               ; =-1
10037035c: 5a883508    	cneg	w8, w8, hs
100370360: 35000108    	cbnz	w8, 0x100370380 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x1fc>
100370364: 90007ce0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
100370368: 9116b000    	add	x0, x0, #0x5ac
10037036c: d65f03c0    	ret
100370370: 6b09011f    	cmp	w8, w9
100370374: 12800008    	mov	w8, #-0x1               ; =-1
100370378: 5a883508    	cneg	w8, w8, hs
10037037c: 34000068    	cbz	w8, 0x100370388 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E+0x204>
100370380: d2800000    	mov	x0, #0x0                ; =0
100370384: d65f03c0    	ret
100370388: 90007ce0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10037038c: 9116fc00    	add	x0, x0, #0x5bf
100370390: d65f03c0    	ret

0000000100370394 <__ZN13codexmate_lib4core5relay13codex_catalog31migrate_legacy_no_account_slots17h39b1f1f35ca98b9eE>:
100370394: 6db923e9    	stp	d9, d8, [sp, #-0x70]!
100370398: a9016ffc    	stp	x28, x27, [sp, #0x10]
10037039c: a90267fa    	stp	x26, x25, [sp, #0x20]
1003703a0: a9035ff8    	stp	x24, x23, [sp, #0x30]
1003703a4: a90457f6    	stp	x22, x21, [sp, #0x40]
1003703a8: a9054ff4    	stp	x20, x19, [sp, #0x50]
1003703ac: a9067bfd    	stp	x29, x30, [sp, #0x60]
1003703b0: 910183fd    	add	x29, sp, #0x60
1003703b4: d10c03ff    	sub	sp, sp, #0x300
1003703b8: b90027e6    	str	w6, [sp, #0x24]
1003703bc: aa0503fa    	mov	x26, x5
1003703c0: aa0403f9    	mov	x25, x4
1003703c4: aa0203f8    	mov	x24, x2
1003703c8: f9002be1    	str	x1, [sp, #0x50]
1003703cc: f9003be0    	str	x0, [sp, #0x70]
1003703d0: b000ac00    	adrp	x0, 0x1018f1000 <__ZN3std12backtrace_rs9symbolize5gimli5Cache11with_global14MAPPINGS_CACHE17hb3a3380e67302d28E>
1003703d4: 912f6000    	add	x0, x0, #0xbd8
1003703d8: f9400009    	ldr	x9, [x0]
1003703dc: d63f0120    	blr	x9
1003703e0: aa0003f6    	mov	x22, x0
1003703e4: 39404009    	ldrb	w9, [x0, #0x10]
1003703e8: 7100053f    	cmp	w9, #0x1
1003703ec: f90017e8    	str	x8, [sp, #0x28]
1003703f0: 54007d41    	b.ne	0x100371398 <__ZN13codexmate_lib4core5relay13codex_catalog31migrate_legacy_no_account_slots17h39b1f1f35ca98b9eE+0x1004>
1003703f4: a94006c0    	ldp	x0, x1, [x22]
1003703f8: 91000408    	add	x8, x0, #0x1
1003703fc: f90002c8    	str	x8, [x22]
