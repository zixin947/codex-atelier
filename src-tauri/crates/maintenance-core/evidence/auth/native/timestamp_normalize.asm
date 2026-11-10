
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001001ed9c0 <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE>:
1001ed9c0: d101c3ff    	sub	sp, sp, #0x70
1001ed9c4: a90457f6    	stp	x22, x21, [sp, #0x40]
1001ed9c8: a9054ff4    	stp	x20, x19, [sp, #0x50]
1001ed9cc: a9067bfd    	stp	x29, x30, [sp, #0x60]
1001ed9d0: 910183fd    	add	x29, sp, #0x60
1001ed9d4: aa0803f3    	mov	x19, x8
1001ed9d8: b40009a0    	cbz	x0, 0x1001edb0c <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE+0x14c>
1001ed9dc: 940105df    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1001ed9e0: b4000961    	cbz	x1, 0x1001edb0c <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE+0x14c>
1001ed9e4: 9100a3e8    	add	x8, sp, #0x28
1001ed9e8: aa0003f4    	mov	x20, x0
1001ed9ec: aa0103f5    	mov	x21, x1
1001ed9f0: 943e5205    	bl	0x101182204 <__ZN4core3num7dec2flt60_$LT$impl$u20$core..str..traits..FromStr$u20$for$u20$f64$GT$8from_str17h04bab746debd651bE>
1001ed9f4: 3940a3e8    	ldrb	w8, [sp, #0x28]
1001ed9f8: 7100051f    	cmp	w8, #0x1
1001ed9fc: 54000341    	b.ne	0x1001eda64 <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE+0xa4>
1001eda00: 910063e8    	add	x8, sp, #0x18
1001eda04: aa1403e0    	mov	x0, x20
1001eda08: aa1503e1    	mov	x1, x21
1001eda0c: 94348600    	bl	0x100f0f20c <__ZN6chrono8datetime50DateTime$LT$chrono..offset..fixed..FixedOffset$GT$18parse_from_rfc333917h54fb46be2621b4e9E>
1001eda10: b9401be8    	ldr	w8, [sp, #0x18]
1001eda14: 340007c8    	cbz	w8, 0x1001edb0c <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE+0x14c>
1001eda18: 9100a3e8    	add	x8, sp, #0x28
1001eda1c: 910063e0    	add	x0, sp, #0x18
1001eda20: 940591c9    	bl	0x100352144 <__ZN6chrono8datetime18DateTime$LT$Tz$GT$10to_rfc333917h2d5d5f4ecddf489aE>
1001eda24: f94017e8    	ldr	x8, [sp, #0x28]
1001eda28: 3940c3e9    	ldrb	w9, [sp, #0x30]
1001eda2c: f84313ea    	ldur	x10, [sp, #0x31]
1001eda30: f90007ea    	str	x10, [sp, #0x8]
1001eda34: f9401fea    	ldr	x10, [sp, #0x38]
1001eda38: f800f3ea    	stur	x10, [sp, #0xf]
1001eda3c: d2f0000a    	mov	x10, #-0x8000000000000000 ; =-9223372036854775808
1001eda40: eb0a011f    	cmp	x8, x10
1001eda44: 54000640    	b.eq	0x1001edb0c <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE+0x14c>
1001eda48: f94007ea    	ldr	x10, [sp, #0x8]
1001eda4c: f800926a    	stur	x10, [x19, #0x9]
1001eda50: f840f3ea    	ldur	x10, [sp, #0xf]
1001eda54: f9000a6a    	str	x10, [x19, #0x10]
1001eda58: f9000268    	str	x8, [x19]
1001eda5c: 39002269    	strb	w9, [x19, #0x8]
1001eda60: 1400002d    	b	0x1001edb14 <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE+0x154>
1001eda64: fd401be0    	ldr	d0, [sp, #0x30]
1001eda68: d2c80008    	mov	x8, #0x400000000000     ; =70368744177664
1001eda6c: f2e811e8    	movk	x8, #0x408f, lsl #48
1001eda70: 9e670101    	fmov	d1, x8
1001eda74: 1e611801    	fdiv	d1, d0, d1
1001eda78: d2bd0008    	mov	x8, #0xe8000000         ; =3892314112
1001eda7c: f2c90ec8    	movk	x8, #0x4876, lsl #32
1001eda80: f2e846e8    	movk	x8, #0x4237, lsl #48
1001eda84: 9e670102    	fmov	d2, x8
1001eda88: 1e622000    	fcmp	d0, d2
1001eda8c: 1e60cc20    	fcsel	d0, d1, d0, gt
1001eda90: 9e780008    	fcvtzs	x8, d0
1001eda94: d2852ae9    	mov	x9, #0x2957             ; =10583
1001eda98: f2b9ca29    	movk	x9, #0xce51, lsl #16
1001eda9c: f2d91409    	movk	x9, #0xc8a0, lsl #32
1001edaa0: f2e308a9    	movk	x9, #0x1845, lsl #48
1001edaa4: 9b497d09    	smulh	x9, x8, x9
1001edaa8: 934dfd2a    	asr	x10, x9, #13
1001edaac: 8b49fd49    	add	x9, x10, x9, lsr #63
1001edab0: 528a300a    	mov	w10, #0x5180            ; =20864
1001edab4: 72a0002a    	movk	w10, #0x1, lsl #16
1001edab8: 9b0aa12b    	msub	x11, x9, x10, x8
1001edabc: 8b8bfd28    	add	x8, x9, x11, asr #63
1001edac0: 8b0a0169    	add	x9, x11, x10
1001edac4: f100017f    	cmp	x11, #0x0
1001edac8: 9a8bb134    	csel	x20, x9, x11, lt
1001edacc: 9280d889    	mov	x9, #-0x6c5             ; =-1733
1001edad0: f2b00149    	movk	x9, #0x800a, lsl #16
1001edad4: b2607fea    	mov	x10, #-0x100000000      ; =-4294967296
1001edad8: 8b090109    	add	x9, x8, x9
1001edadc: eb0a013f    	cmp	x9, x10
1001edae0: 54000163    	b.lo	0x1001edb0c <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE+0x14c>
1001edae4: 1142bd08    	add	w8, w8, #0xaf, lsl #12  ; =0xaf000
1001edae8: 1124ed00    	add	w0, w8, #0x93b
1001edaec: 94348f6b    	bl	0x100f11898 <__ZN6chrono5naive4date9NaiveDate25from_num_days_from_ce_opt17hbb272b87e48ffc28E>
1001edaf0: 340000e0    	cbz	w0, 0x1001edb0c <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE+0x14c>
1001edaf4: 290353e0    	stp	w0, w20, [sp, #0x18]
1001edaf8: b90023ff    	str	wzr, [sp, #0x20]
1001edafc: 910063e0    	add	x0, sp, #0x18
1001edb00: aa1303e8    	mov	x8, x19
1001edb04: 94059190    	bl	0x100352144 <__ZN6chrono8datetime18DateTime$LT$Tz$GT$10to_rfc333917h2d5d5f4ecddf489aE>
1001edb08: 14000003    	b	0x1001edb14 <__ZN13codexmate_lib4core4auth19normalize_timestamp17h272be36e2885c1dcE+0x154>
1001edb0c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1001edb10: f9000268    	str	x8, [x19]
1001edb14: a9467bfd    	ldp	x29, x30, [sp, #0x60]
1001edb18: a9454ff4    	ldp	x20, x19, [sp, #0x50]
1001edb1c: a94457f6    	ldp	x22, x21, [sp, #0x40]
1001edb20: 9101c3ff    	add	sp, sp, #0x70
1001edb24: d65f03c0    	ret
