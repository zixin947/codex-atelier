
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100371fa8 <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E>:
100371fa8: d10283ff    	sub	sp, sp, #0xa0
100371fac: a9084ff4    	stp	x20, x19, [sp, #0x80]
100371fb0: a9097bfd    	stp	x29, x30, [sp, #0x90]
100371fb4: 910243fd    	add	x29, sp, #0x90
100371fb8: a90207ff    	stp	xzr, x1, [sp, #0x20]
100371fbc: a90307e0    	stp	x0, x1, [sp, #0x30]
100371fc0: a90407ff    	stp	xzr, x1, [sp, #0x40]
100371fc4: 0f0105c0    	movi.2s	v0, #0x2e
100371fc8: fd002be0    	str	d0, [sp, #0x50]
100371fcc: 52800028    	mov	w8, #0x1                ; =1
100371fd0: 390163e8    	strb	w8, [sp, #0x58]
100371fd4: 7900c3e8    	strh	w8, [sp, #0x60]
100371fd8: 910023e8    	add	x8, sp, #0x8
100371fdc: 910083e0    	add	x0, sp, #0x20
100371fe0: 9406f8a9    	bl	0x100530284 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h22057c1c2b15921aE>
100371fe4: f9400fe8    	ldr	x8, [sp, #0x18]
100371fe8: f1000d1f    	cmp	x8, #0x3
100371fec: 54000301    	b.ne	0x10037204c <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E+0xa4>
100371ff0: f9400bf3    	ldr	x19, [sp, #0x10]
100371ff4: a9400a61    	ldp	x1, x2, [x19]
100371ff8: f0007cc0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
100371ffc: 91293400    	add	x0, x0, #0xa4d
100372000: d100a3a8    	sub	x8, x29, #0x28
100372004: 9401f001    	bl	0x1003ee008 <__ZN6base646engine6Engine6decode5inner17hd61753a1cd818caeE>
100372008: f85d83a1    	ldur	x1, [x29, #-0x28]
10037200c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100372010: eb08003f    	cmp	x1, x8
100372014: 540001c0    	b.eq	0x10037204c <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E+0xa4>
100372018: b4000081    	cbz	x1, 0x100372028 <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E+0x80>
10037201c: f85e03a0    	ldur	x0, [x29, #-0x20]
100372020: 52800022    	mov	w2, #0x1                ; =1
100372024: 97f23d51    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100372028: a9410a61    	ldp	x1, x2, [x19, #0x10]
10037202c: d0007cc0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
100372030: 91293400    	add	x0, x0, #0xa4d
100372034: d100a3a8    	sub	x8, x29, #0x28
100372038: 9401eff4    	bl	0x1003ee008 <__ZN6base646engine6Engine6decode5inner17hd61753a1cd818caeE>
10037203c: f85d83a1    	ldur	x1, [x29, #-0x28]
100372040: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100372044: eb08003f    	cmp	x1, x8
100372048: 540001a1    	b.ne	0x10037207c <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E+0xd4>
10037204c: 52800013    	mov	w19, #0x0               ; =0
100372050: f94007e8    	ldr	x8, [sp, #0x8]
100372054: b40000a8    	cbz	x8, 0x100372068 <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E+0xc0>
100372058: f9400be0    	ldr	x0, [sp, #0x10]
10037205c: d37ced01    	lsl	x1, x8, #4
100372060: 52800102    	mov	w2, #0x8                ; =8
100372064: 97f23d41    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100372068: aa1303e0    	mov	x0, x19
10037206c: a9497bfd    	ldp	x29, x30, [sp, #0x90]
100372070: a9484ff4    	ldp	x20, x19, [sp, #0x80]
100372074: 910283ff    	add	sp, sp, #0xa0
100372078: d65f03c0    	ret
10037207c: b40000c1    	cbz	x1, 0x100372094 <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E+0xec>
100372080: f85e03a0    	ldur	x0, [x29, #-0x20]
100372084: 52800033    	mov	w19, #0x1               ; =1
100372088: 52800022    	mov	w2, #0x1                ; =1
10037208c: 97f23d37    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100372090: 17fffff0    	b	0x100372050 <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E+0xa8>
100372094: 52800033    	mov	w19, #0x1               ; =1
100372098: 17ffffee    	b	0x100372050 <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E+0xa8>
10037209c: aa0003f4    	mov	x20, x0
1003720a0: f94007e8    	ldr	x8, [sp, #0x8]
1003720a4: b40000a8    	cbz	x8, 0x1003720b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic16is_plausible_jwt17hbf189bfab992e990E+0x110>
1003720a8: d37ced01    	lsl	x1, x8, #4
1003720ac: aa1303e0    	mov	x0, x19
1003720b0: 52800102    	mov	w2, #0x8                ; =8
1003720b4: 97f23d2d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003720b8: aa1403e0    	mov	x0, x20
1003720bc: 94397c26    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001003720c0 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E>:
1003720c0: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
1003720c4: a90167fa    	stp	x26, x25, [sp, #0x10]
1003720c8: a9025ff8    	stp	x24, x23, [sp, #0x20]
1003720cc: a90357f6    	stp	x22, x21, [sp, #0x30]
1003720d0: a9044ff4    	stp	x20, x19, [sp, #0x40]
1003720d4: a9057bfd    	stp	x29, x30, [sp, #0x50]
1003720d8: 910143fd    	add	x29, sp, #0x50
1003720dc: d14007ff    	sub	sp, sp, #0x1, lsl #12   ; =0x1000
1003720e0: f81107ff    	str	xzr, [sp], #-0xf0
1003720e4: aa0003f3    	mov	x19, x0
1003720e8: 52800108    	mov	w8, #0x8                ; =8
1003720ec: a90123ff    	stp	xzr, x8, [sp, #0x10]
1003720f0: f90013ff    	str	xzr, [sp, #0x20]
1003720f4: 9100a3e8    	add	x8, sp, #0x28
1003720f8: aa0103e0    	mov	x0, x1
1003720fc: aa0203e1    	mov	x1, x2
100372100: 94375988    	bl	0x101148720 <__ZN3std3sys2fs8read_dir17ha353ee9b9b706035E>
100372104: 3940c3e8    	ldrb	w8, [sp, #0x30]
100372108: f94017e9    	ldr	x9, [sp, #0x28]
10037210c: f9065be9    	str	x9, [sp, #0xcb0]
100372110: 3932e3e8    	strb	w8, [sp, #0xcb8]
100372114: 7100091f    	cmp	w8, #0x2
100372118: 54000081    	b.ne	0x100372128 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x68>
10037211c: 9132c3e0    	add	x0, sp, #0xcb0
100372120: 9401a1f8    	bl	0x1003da900 <__ZN4core3ptr89drop_in_place$LT$core..result..Result$LT$std..fs..ReadDir$C$std..io..error..Error$GT$$GT$17h5247929482014814E>
100372124: 1400009a    	b	0x10037238c <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x2cc>
100372128: 9132c3f8    	add	x24, sp, #0xcb0
10037212c: 9100a3f9    	add	x25, sp, #0x28
100372130: f90017ff    	str	xzr, [sp, #0x28]
100372134: f9022bff    	str	xzr, [sp, #0x450]
100372138: 9132c3f7    	add	x23, sp, #0xcb0
10037213c: 5280005a    	mov	w26, #0x2               ; =2
100372140: 9280001b    	mov	x27, #-0x1              ; =-1
100372144: f9043fe9    	str	x9, [sp, #0x878]
100372148: 392203e8    	strb	w8, [sp, #0x880]
10037214c: 528becbc    	mov	w28, #0x5f65            ; =24421
100372150: 14000008    	b	0x100372170 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0xb0>
100372154: aa1403e0    	mov	x0, x20
100372158: 52800301    	mov	w1, #0x18               ; =24
10037215c: 52800102    	mov	w2, #0x8                ; =8
100372160: 97f23d02    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100372164: 396203e8    	ldrb	w8, [sp, #0x880]
100372168: 7100091f    	cmp	w8, #0x2
10037216c: 540010c0    	b.eq	0x100372384 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x2c4>
100372170: 9132c3e8    	add	x8, sp, #0xcb0
100372174: 91214320    	add	x0, x25, #0x850
100372178: 9437e3c7    	bl	0x10116b094 <__ZN75_$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hcbd8a6bdaabc6208E>
10037217c: f9465be8    	ldr	x8, [sp, #0xcb0]
100372180: b4000108    	cbz	x8, 0x1003721a0 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0xe0>
100372184: 3cc08300    	ldur	q0, [x24, #0x8]
100372188: 3d8003e0    	str	q0, [sp]
10037218c: 912283e0    	add	x0, sp, #0x8a0
100372190: 910062e1    	add	x1, x23, #0x18
100372194: 52808202    	mov	w2, #0x410              ; =1040
100372198: 94397d45    	bl	0x1011d16ac <_writev+0x1011d16ac>
10037219c: 14000014    	b	0x1003721ec <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x12c>
1003721a0: 396203e8    	ldrb	w8, [sp, #0x880]
1003721a4: 7100091f    	cmp	w8, #0x2
1003721a8: 54000100    	b.eq	0x1003721c8 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x108>
1003721ac: f9443fe8    	ldr	x8, [sp, #0x878]
1003721b0: f87b0108    	ldaddl	x27, x8, [x8]
1003721b4: f100051f    	cmp	x8, #0x1
1003721b8: 54000081    	b.ne	0x1003721c8 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x108>
1003721bc: d50339bf    	dmb	ishld
1003721c0: 91214320    	add	x0, x25, #0x850
1003721c4: 9437c12c    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1003721c8: 392203fa    	strb	w26, [sp, #0x880]
1003721cc: b94cb3f4    	ldr	w20, [sp, #0xcb0]
1003721d0: 3cc08300    	ldur	q0, [x24, #0x8]
1003721d4: 3d8003e0    	str	q0, [sp]
1003721d8: 912283e0    	add	x0, sp, #0x8a0
1003721dc: 910062e1    	add	x1, x23, #0x18
1003721e0: 52808202    	mov	w2, #0x410              ; =1040
1003721e4: 94397d32    	bl	0x1011d16ac <_writev+0x1011d16ac>
1003721e8: 36000cf4    	tbz	w20, #0x0, 0x100372384 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x2c4>
1003721ec: 3dc003e0    	ldr	q0, [sp]
1003721f0: 9e660008    	fmov	x8, d0
1003721f4: b5000228    	cbnz	x8, 0x100372238 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x178>
1003721f8: 4e183c14    	mov.d	x20, v0[1]
1003721fc: 92400688    	and	x8, x20, #0x3
100372200: f100051f    	cmp	x8, #0x1
100372204: 54fffb01    	b.ne	0x100372164 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0xa4>
100372208: f85ffe95    	ldr	x21, [x20, #-0x1]!
10037220c: f9400696    	ldr	x22, [x20, #0x8]
100372210: f94002c8    	ldr	x8, [x22]
100372214: b4000068    	cbz	x8, 0x100372220 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x160>
100372218: aa1503e0    	mov	x0, x21
10037221c: d63f0100    	blr	x8
100372220: f94006c1    	ldr	x1, [x22, #0x8]
100372224: b4fff981    	cbz	x1, 0x100372154 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x94>
100372228: f9400ac2    	ldr	x2, [x22, #0x10]
10037222c: aa1503e0    	mov	x0, x21
100372230: 97f23cce    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100372234: 17ffffc8    	b	0x100372154 <__ZN13codexmate_lib4core5relay16codex_diagnostic17list_state_dbs_in17h16fafb554f4201e4E+0x94>
100372238: 910042e0    	add	x0, x23, #0x10
10037223c: 912283e1    	add	x1, sp, #0x8a0
100372240: 52808202    	mov	w2, #0x410              ; =1040
100372244: 3d8003e0    	str	q0, [sp]
100372248: 94397d19    	bl	0x1011d16ac <_writev+0x1011d16ac>
