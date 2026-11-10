
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010050ca18 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE>:
10050ca18: d10243ff    	sub	sp, sp, #0x90
10050ca1c: a9036ffc    	stp	x28, x27, [sp, #0x30]
10050ca20: a90467fa    	stp	x26, x25, [sp, #0x40]
10050ca24: a9055ff8    	stp	x24, x23, [sp, #0x50]
10050ca28: a90657f6    	stp	x22, x21, [sp, #0x60]
10050ca2c: a9074ff4    	stp	x20, x19, [sp, #0x70]
10050ca30: a9087bfd    	stp	x29, x30, [sp, #0x80]
10050ca34: 910203fd    	add	x29, sp, #0x80
10050ca38: aa0003f3    	mov	x19, x0
10050ca3c: f9400c18    	ldr	x24, [x0, #0x18]
10050ca40: f9400417    	ldr	x23, [x0, #0x8]
10050ca44: eb1802ff    	cmp	x23, x24
10050ca48: 540011e0    	b.eq	0x10050cc84 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x26c>
10050ca4c: aa0103f4    	mov	x20, x1
10050ca50: 910063f9    	add	x25, sp, #0x18
10050ca54: d28e4e1a    	mov	x26, #0x7270            ; =29296
10050ca58: f2accdfa    	movk	x26, #0x666f, lsl #16
10050ca5c: f2cd8d3a    	movk	x26, #0x6c69, lsl #32
10050ca60: f2ee6cba    	movk	x26, #0x7365, lsl #48
10050ca64: 528005db    	mov	w27, #0x2e              ; =46
10050ca68: 14000003    	b	0x10050ca74 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x5c>
10050ca6c: eb1802ff    	cmp	x23, x24
10050ca70: 540010a0    	b.eq	0x10050cc84 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x26c>
10050ca74: a9405af5    	ldp	x21, x22, [x23]
10050ca78: f9400ae1    	ldr	x1, [x23, #0x10]
10050ca7c: 910062f7    	add	x23, x23, #0x18
10050ca80: f9000677    	str	x23, [x19, #0x8]
10050ca84: aa1603e0    	mov	x0, x22
10050ca88: 97f489b4    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
10050ca8c: d280000d    	mov	x13, #0x0               ; =0
10050ca90: 8b010008    	add	x8, x0, x1
10050ca94: cb08002a    	sub	x10, x1, x8
10050ca98: 14000007    	b	0x10050cab4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x9c>
10050ca9c: 9100096d    	add	x13, x11, #0x2
10050caa0: 331a118f    	bfi	w15, w12, #6, #5
10050caa4: aa0f03ec    	mov	x12, x15
10050caa8: 8b0d014d    	add	x13, x10, x13
10050caac: 71016d9f    	cmp	w12, #0x5b
10050cab0: 540003c1    	b.ne	0x10050cb28 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x110>
10050cab4: eb0101bf    	cmp	x13, x1
10050cab8: 54000340    	b.eq	0x10050cb20 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x108>
10050cabc: aa0d03e9    	mov	x9, x13
10050cac0: 8b0d000b    	add	x11, x0, x13
10050cac4: aa0b03ed    	mov	x13, x11
10050cac8: 38c015ae    	ldrsb	w14, [x13], #0x1
10050cacc: 12001dcc    	and	w12, w14, #0xff
10050cad0: 36fffece    	tbz	w14, #0x1f, 0x10050caa8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x90>
10050cad4: 3940056d    	ldrb	w13, [x11, #0x1]
10050cad8: 120015af    	and	w15, w13, #0x3f
10050cadc: 7103819f    	cmp	w12, #0xe0
10050cae0: 54fffde3    	b.lo	0x10050ca9c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x84>
10050cae4: 1200118e    	and	w14, w12, #0x1f
10050cae8: 3940096d    	ldrb	w13, [x11, #0x2]
10050caec: 120015ad    	and	w13, w13, #0x3f
10050caf0: 2a0f19af    	orr	w15, w13, w15, lsl #6
10050caf4: 7103c19f    	cmp	w12, #0xf0
10050caf8: 540000e3    	b.lo	0x10050cb14 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0xfc>
10050cafc: 9100116d    	add	x13, x11, #0x4
10050cb00: 39400d6c    	ldrb	w12, [x11, #0x3]
10050cb04: 1200158c    	and	w12, w12, #0x3f
10050cb08: 2a0f198c    	orr	w12, w12, w15, lsl #6
10050cb0c: 330e09cc    	bfi	w12, w14, #18, #3
10050cb10: 17ffffe6    	b	0x10050caa8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x90>
10050cb14: 91000d6d    	add	x13, x11, #0x3
10050cb18: 2a0e31ec    	orr	w12, w15, w14, lsl #12
10050cb1c: 17ffffe3    	b	0x10050caa8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x90>
10050cb20: aa0103e9    	mov	x9, x1
10050cb24: 14000002    	b	0x10050cb2c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x114>
10050cb28: aa0b03e8    	mov	x8, x11
10050cb2c: cb09002c    	sub	x12, x1, x9
10050cb30: 8b090009    	add	x9, x0, x9
10050cb34: 14000007    	b	0x10050cb50 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x138>
10050cb38: 120011ad    	and	w13, w13, #0x1f
10050cb3c: 1200156b    	and	w11, w11, #0x3f
10050cb40: 2a0d196b    	orr	w11, w11, w13, lsl #6
10050cb44: cb08018c    	sub	x12, x12, x8
10050cb48: 7101757f    	cmp	w11, #0x5d
10050cb4c: 54000341    	b.ne	0x10050cbb4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x19c>
10050cb50: b40005ec    	cbz	x12, 0x10050cc0c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x1f4>
10050cb54: aa0c03ea    	mov	x10, x12
10050cb58: 8b0c012e    	add	x14, x9, x12
10050cb5c: aa0e03ec    	mov	x12, x14
10050cb60: 38dffd8b    	ldrsb	w11, [x12, #-0x1]!
10050cb64: 36ffff0b    	tbz	w11, #0x1f, 0x10050cb44 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x12c>
10050cb68: aa0e03ec    	mov	x12, x14
10050cb6c: 38dfed8d    	ldrsb	w13, [x12, #-0x2]!
10050cb70: 310101bf    	cmn	w13, #0x40
10050cb74: 54fffe2a    	b.ge	0x10050cb38 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x120>
10050cb78: aa0e03ec    	mov	x12, x14
10050cb7c: 38dfdd8f    	ldrsb	w15, [x12, #-0x3]!
10050cb80: 310101ff    	cmn	w15, #0x40
10050cb84: 540000ea    	b.ge	0x10050cba0 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x188>
10050cb88: 12001dec    	and	w12, w15, #0xff
10050cb8c: 385fcdd0    	ldrb	w16, [x14, #-0x4]!
10050cb90: 1200158f    	and	w15, w12, #0x3f
10050cb94: 331a0a0f    	bfi	w15, w16, #6, #3
10050cb98: aa0e03ec    	mov	x12, x14
10050cb9c: 14000002    	b	0x10050cba4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x18c>
10050cba0: 12000def    	and	w15, w15, #0xf
10050cba4: 12001dad    	and	w13, w13, #0xff
10050cba8: 120015ad    	and	w13, w13, #0x3f
10050cbac: 2a0f19ad    	orr	w13, w13, w15, lsl #6
10050cbb0: 17ffffe3    	b	0x10050cb3c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x124>
10050cbb4: f1004141    	subs	x1, x10, #0x10
10050cbb8: 54000082    	b.hs	0x10050cbc8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x1b0>
10050cbbc: f100255f    	cmp	x10, #0x9
10050cbc0: 540001c2    	b.hs	0x10050cbf8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x1e0>
10050cbc4: 14000012    	b	0x10050cc0c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x1f4>
10050cbc8: a9402d28    	ldp	x8, x11, [x9]
10050cbcc: d28dedac    	mov	x12, #0x6f6d            ; =28525
10050cbd0: f2acac8c    	movk	x12, #0x6564, lsl #16
10050cbd4: f2cbed8c    	movk	x12, #0x5f6c, lsl #32
10050cbd8: f2ee4e0c    	movk	x12, #0x7270, lsl #48
10050cbdc: eb0c011f    	cmp	x8, x12
10050cbe0: d28ecde8    	mov	x8, #0x766f             ; =30319
10050cbe4: f2ac8d28    	movk	x8, #0x6469, lsl #16
10050cbe8: f2ce4ca8    	movk	x8, #0x7265, lsl #32
10050cbec: f2e5ce68    	movk	x8, #0x2e73, lsl #48
10050cbf0: fa480160    	ccmp	x11, x8, #0x0, eq
10050cbf4: 540001e0    	b.eq	0x10050cc30 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x218>
10050cbf8: f9400128    	ldr	x8, [x9]
10050cbfc: 3940212b    	ldrb	w11, [x9, #0x8]
10050cc00: eb1a011f    	cmp	x8, x26
10050cc04: 7a5b0160    	ccmp	w11, w27, #0x0, eq
10050cc08: 540000e0    	b.eq	0x10050cc24 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x20c>
10050cc0c: b4fff315    	cbz	x21, 0x10050ca6c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x54>
10050cc10: aa1603e0    	mov	x0, x22
10050cc14: aa1503e1    	mov	x1, x21
10050cc18: 52800022    	mov	w2, #0x1                ; =1
10050cc1c: 97ebd253    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050cc20: 17ffff93    	b	0x10050ca6c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x54>
10050cc24: d1002541    	sub	x1, x10, #0x9
10050cc28: 91002520    	add	x0, x9, #0x9
10050cc2c: 14000002    	b	0x10050cc34 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x21c>
10050cc30: 91004120    	add	x0, x9, #0x10
10050cc34: 910063e8    	add	x8, sp, #0x18
10050cc38: 97f99a6b    	bl	0x1003735e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E>
10050cc3c: f9400ffc    	ldr	x28, [sp, #0x18]
10050cc40: 3cc08320    	ldur	q0, [x25, #0x8]
10050cc44: 3d8003e0    	str	q0, [sp]
10050cc48: b40000b5    	cbz	x21, 0x10050cc5c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x244>
10050cc4c: aa1603e0    	mov	x0, x22
10050cc50: aa1503e1    	mov	x1, x21
10050cc54: 52800022    	mov	w2, #0x1                ; =1
10050cc58: 97ebd244    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050cc5c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10050cc60: eb08039f    	cmp	x28, x8
10050cc64: 54fff040    	b.eq	0x10050ca6c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x54>
10050cc68: 3dc003e0    	ldr	q0, [sp]
10050cc6c: 3c808320    	stur	q0, [x25, #0x8]
10050cc70: f9000ffc    	str	x28, [sp, #0x18]
10050cc74: 910063e1    	add	x1, sp, #0x18
10050cc78: aa1403e0    	mov	x0, x20
10050cc7c: 9401a552    	bl	0x1005761c4 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17hdb67a7b69d944eb9E>
10050cc80: 17ffff7b    	b	0x10050ca6c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x54>
10050cc84: eb170308    	subs	x8, x24, x23
10050cc88: 54000200    	b.eq	0x10050ccc8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x2b0>
10050cc8c: b201f3e9    	mov	x9, #-0x5555555555555556 ; =-6148914691236517206
10050cc90: f2955569    	movk	x9, #0xaaab
10050cc94: 9bc97d08    	umulh	x8, x8, x9
10050cc98: d344fd14    	lsr	x20, x8, #4
10050cc9c: 910022f5    	add	x21, x23, #0x8
10050cca0: 14000004    	b	0x10050ccb0 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x298>
10050cca4: 910062b5    	add	x21, x21, #0x18
10050cca8: f1000694    	subs	x20, x20, #0x1
10050ccac: 540000e0    	b.eq	0x10050ccc8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x2b0>
10050ccb0: f85f82a1    	ldur	x1, [x21, #-0x8]
10050ccb4: b4ffff81    	cbz	x1, 0x10050cca4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x28c>
10050ccb8: f94002a0    	ldr	x0, [x21]
10050ccbc: 52800022    	mov	w2, #0x1                ; =1
10050ccc0: 97ebd22a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050ccc4: 17fffff8    	b	0x10050cca4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x28c>
10050ccc8: f9400a68    	ldr	x8, [x19, #0x10]
10050cccc: b40000c8    	cbz	x8, 0x10050cce4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x2cc>
10050ccd0: f9400260    	ldr	x0, [x19]
10050ccd4: 8b080508    	add	x8, x8, x8, lsl #1
10050ccd8: d37df101    	lsl	x1, x8, #3
10050ccdc: 52800102    	mov	w2, #0x8                ; =8
10050cce0: 97ebd222    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050cce4: a9487bfd    	ldp	x29, x30, [sp, #0x80]
10050cce8: a9474ff4    	ldp	x20, x19, [sp, #0x70]
10050ccec: a94657f6    	ldp	x22, x21, [sp, #0x60]
10050ccf0: a9455ff8    	ldp	x24, x23, [sp, #0x50]
10050ccf4: a94467fa    	ldp	x26, x25, [sp, #0x40]
10050ccf8: a9436ffc    	ldp	x28, x27, [sp, #0x30]
10050ccfc: 910243ff    	add	sp, sp, #0x90
10050cd00: d65f03c0    	ret
10050cd04: aa0003f4    	mov	x20, x0
10050cd08: aa1303e0    	mov	x0, x19
10050cd0c: 940163b8    	bl	0x100565bec <__ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h2d6b569100b98914E>
10050cd10: aa1403e0    	mov	x0, x20
10050cd14: 94331110    	bl	0x1011d1154 <_writev+0x1011d1154>
10050cd18: aa0003f4    	mov	x20, x0
10050cd1c: b40000b5    	cbz	x21, 0x10050cd30 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46c24d3160f1cceaE+0x318>
10050cd20: aa1603e0    	mov	x0, x22
10050cd24: aa1503e1    	mov	x1, x21
10050cd28: 52800022    	mov	w2, #0x1                ; =1
10050cd2c: 97ebd20f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050cd30: aa1303e0    	mov	x0, x19
10050cd34: 940163ae    	bl	0x100565bec <__ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h2d6b569100b98914E>
10050cd38: aa1403e0    	mov	x0, x20
10050cd3c: 94331106    	bl	0x1011d1154 <_writev+0x1011d1154>
