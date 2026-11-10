
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001001f76a0 <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E>:
1001f76a0: d10443ff    	sub	sp, sp, #0x110
1001f76a4: 6d0c23e9    	stp	d9, d8, [sp, #0xc0]
1001f76a8: a90d5ff8    	stp	x24, x23, [sp, #0xd0]
1001f76ac: a90e57f6    	stp	x22, x21, [sp, #0xe0]
1001f76b0: a90f4ff4    	stp	x20, x19, [sp, #0xf0]
1001f76b4: a9107bfd    	stp	x29, x30, [sp, #0x100]
1001f76b8: 910403fd    	add	x29, sp, #0x100
1001f76bc: aa0303f3    	mov	x19, x3
1001f76c0: aa0203f4    	mov	x20, x2
1001f76c4: a90187ff    	stp	xzr, x1, [sp, #0x18]
1001f76c8: a90287e0    	stp	x0, x1, [sp, #0x28]
1001f76cc: a90387ff    	stp	xzr, x1, [sp, #0x38]
1001f76d0: 0f0105c8    	movi.2s	v8, #0x2e
1001f76d4: fd0027e8    	str	d8, [sp, #0x48]
1001f76d8: 52800035    	mov	w21, #0x1               ; =1
1001f76dc: 390143f5    	strb	w21, [sp, #0x50]
1001f76e0: 7900b3f5    	strh	w21, [sp, #0x58]
1001f76e4: 910003e8    	mov	x8, sp
1001f76e8: 910063e0    	add	x0, sp, #0x18
1001f76ec: 940ce2e6    	bl	0x100530284 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h22057c1c2b15921aE>
1001f76f0: a907cfff    	stp	xzr, x19, [sp, #0x78]
1001f76f4: a908cff4    	stp	x20, x19, [sp, #0x88]
1001f76f8: a909cfff    	stp	xzr, x19, [sp, #0x98]
1001f76fc: fd0057e8    	str	d8, [sp, #0xa8]
1001f7700: 3902c3f5    	strb	w21, [sp, #0xb0]
1001f7704: 790173f5    	strh	w21, [sp, #0xb8]
1001f7708: 910183e8    	add	x8, sp, #0x60
1001f770c: 9101e3e0    	add	x0, sp, #0x78
1001f7710: 940ce2dd    	bl	0x100530284 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h22057c1c2b15921aE>
1001f7714: a940a3f3    	ldp	x19, x8, [sp, #0x8]
1001f7718: a946a7f4    	ldp	x20, x9, [sp, #0x68]
1001f771c: eb08013f    	cmp	x9, x8
1001f7720: 9a883135    	csel	x21, x9, x8, lo
1001f7724: b40001f5    	cbz	x21, 0x1001f7760 <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E+0xc0>
1001f7728: 91002296    	add	x22, x20, #0x8
1001f772c: 91002277    	add	x23, x19, #0x8
1001f7730: f94002e2    	ldr	x2, [x23]
1001f7734: f94002c8    	ldr	x8, [x22]
1001f7738: eb08005f    	cmp	x2, x8
1001f773c: 540003c1    	b.ne	0x1001f77b4 <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E+0x114>
1001f7740: f85f82c1    	ldur	x1, [x22, #-0x8]
1001f7744: f85f82e0    	ldur	x0, [x23, #-0x8]
1001f7748: 943f67d6    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1001f774c: 35000340    	cbnz	w0, 0x1001f77b4 <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E+0x114>
1001f7750: 910042d6    	add	x22, x22, #0x10
1001f7754: 910042f7    	add	x23, x23, #0x10
1001f7758: f10006b5    	subs	x21, x21, #0x1
1001f775c: 54fffea1    	b.ne	0x1001f7730 <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E+0x90>
1001f7760: 52800035    	mov	w21, #0x1               ; =1
1001f7764: f94033e8    	ldr	x8, [sp, #0x60]
1001f7768: b40000a8    	cbz	x8, 0x1001f777c <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E+0xdc>
1001f776c: d37ced01    	lsl	x1, x8, #4
1001f7770: aa1403e0    	mov	x0, x20
1001f7774: 52800102    	mov	w2, #0x8                ; =8
1001f7778: 97f8277c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f777c: f94003e8    	ldr	x8, [sp]
1001f7780: b40000a8    	cbz	x8, 0x1001f7794 <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E+0xf4>
1001f7784: d37ced01    	lsl	x1, x8, #4
1001f7788: aa1303e0    	mov	x0, x19
1001f778c: 52800102    	mov	w2, #0x8                ; =8
1001f7790: 97f82776    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f7794: aa1503e0    	mov	x0, x21
1001f7798: a9507bfd    	ldp	x29, x30, [sp, #0x100]
1001f779c: a94f4ff4    	ldp	x20, x19, [sp, #0xf0]
1001f77a0: a94e57f6    	ldp	x22, x21, [sp, #0xe0]
1001f77a4: a94d5ff8    	ldp	x24, x23, [sp, #0xd0]
1001f77a8: 6d4c23e9    	ldp	d9, d8, [sp, #0xc0]
1001f77ac: 910443ff    	add	sp, sp, #0x110
1001f77b0: d65f03c0    	ret
1001f77b4: 52800015    	mov	w21, #0x0               ; =0
1001f77b8: f94033e8    	ldr	x8, [sp, #0x60]
1001f77bc: b5fffd88    	cbnz	x8, 0x1001f776c <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E+0xcc>
1001f77c0: 17ffffef    	b	0x1001f777c <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E+0xdc>
1001f77c4: aa0003f3    	mov	x19, x0
1001f77c8: f94003e8    	ldr	x8, [sp]
1001f77cc: b40000a8    	cbz	x8, 0x1001f77e0 <__ZN13codexmate_lib4core5relay15config_takeover16section_overlaps17hc7bf871a7f0a8df8E+0x140>
1001f77d0: f94007e0    	ldr	x0, [sp, #0x8]
1001f77d4: d37ced01    	lsl	x1, x8, #4
1001f77d8: 52800102    	mov	w2, #0x8                ; =8
1001f77dc: 97f82763    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f77e0: aa1303e0    	mov	x0, x19
1001f77e4: 943f665c    	bl	0x1011d1154 <_writev+0x1011d1154>
