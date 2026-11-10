
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001001fad28 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E>:
1001fad28: d102c3ff    	sub	sp, sp, #0xb0
1001fad2c: a9075ff8    	stp	x24, x23, [sp, #0x70]
1001fad30: a90857f6    	stp	x22, x21, [sp, #0x80]
1001fad34: a9094ff4    	stp	x20, x19, [sp, #0x90]
1001fad38: a90a7bfd    	stp	x29, x30, [sp, #0xa0]
1001fad3c: 910283fd    	add	x29, sp, #0xa0
1001fad40: b40000a1    	cbz	x1, 0x1001fad54 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x2c>
1001fad44: aa0003f3    	mov	x19, x0
1001fad48: 39400009    	ldrb	w9, [x0]
1001fad4c: 71008d3f    	cmp	w9, #0x23
1001fad50: 54000121    	b.ne	0x1001fad74 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x4c>
1001fad54: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
1001fad58: f9000109    	str	x9, [x8]
1001fad5c: a94a7bfd    	ldp	x29, x30, [sp, #0xa0]
1001fad60: a9494ff4    	ldp	x20, x19, [sp, #0x90]
1001fad64: a94857f6    	ldp	x22, x21, [sp, #0x80]
1001fad68: a9475ff8    	ldp	x24, x23, [sp, #0x70]
1001fad6c: 9102c3ff    	add	sp, sp, #0xb0
1001fad70: d65f03c0    	ret
1001fad74: f1003c3f    	cmp	x1, #0xf
1001fad78: 54000128    	b.hi	0x1001fad9c <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x74>
1001fad7c: aa1303e9    	mov	x9, x19
1001fad80: aa0103ea    	mov	x10, x1
1001fad84: 3840152b    	ldrb	w11, [x9], #0x1
1001fad88: 7100f57f    	cmp	w11, #0x3d
1001fad8c: 540001c0    	b.eq	0x1001fadc4 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x9c>
1001fad90: f100054a    	subs	x10, x10, #0x1
1001fad94: 54ffff81    	b.ne	0x1001fad84 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x5c>
1001fad98: 17ffffef    	b	0x1001fad54 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x2c>
1001fad9c: 528007a0    	mov	w0, #0x3d               ; =61
1001fada0: aa0103f4    	mov	x20, x1
1001fada4: aa1303e1    	mov	x1, x19
1001fada8: aa1403e2    	mov	x2, x20
1001fadac: aa0803f5    	mov	x21, x8
1001fadb0: 943e2e4e    	bl	0x1011866e8 <__ZN4core5slice6memchr14memchr_aligned17h58876bc074a374c5E>
1001fadb4: aa1403e1    	mov	x1, x20
1001fadb8: aa1503e8    	mov	x8, x21
1001fadbc: f100041f    	cmp	x0, #0x1
1001fadc0: 54fffca1    	b.ne	0x1001fad54 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x2c>
1001fadc4: aa0803f7    	mov	x23, x8
1001fadc8: 910003e8    	mov	x8, sp
1001fadcc: a9014fe1    	stp	x1, x19, [sp, #0x10]
1001fadd0: a9027fe1    	stp	x1, xzr, [sp, #0x20]
1001fadd4: f9001be1    	str	x1, [sp, #0x30]
1001fadd8: 0f0107a0    	movi.2s	v0, #0x3d
1001faddc: fd001fe0    	str	d0, [sp, #0x38]
1001fade0: 52800029    	mov	w9, #0x1                ; =1
1001fade4: 390103e9    	strb	w9, [sp, #0x40]
1001fade8: 790093e9    	strh	w9, [sp, #0x48]
1001fadec: d0008849    	adrp	x9, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
1001fadf0: 3dc04920    	ldr	q0, [x9, #0x120]
1001fadf4: 3d8003e0    	str	q0, [sp]
1001fadf8: d10123a0    	sub	x0, x29, #0x48
1001fadfc: 91006101    	add	x1, x8, #0x18
1001fae00: 940101b3    	bl	0x10023b4cc <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
1001fae04: b85b83a8    	ldur	w8, [x29, #-0x48]
1001fae08: 7100051f    	cmp	w8, #0x1
1001fae0c: 54000761    	b.ne	0x1001faef8 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x1d0>
1001fae10: f85c03a8    	ldur	x8, [x29, #-0x40]
1001fae14: f94007e9    	ldr	x9, [sp, #0x8]
1001fae18: cb090101    	sub	x1, x8, x9
1001fae1c: 8b090260    	add	x0, x19, x9
1001fae20: 9400d0ce    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1001fae24: aa1703e8    	mov	x8, x23
1001fae28: b4fff961    	cbz	x1, 0x1001fad54 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x2c>
1001fae2c: aa0103f3    	mov	x19, x1
1001fae30: 8b010009    	add	x9, x0, x1
1001fae34: aa0003ea    	mov	x10, x0
1001fae38: eb09015f    	cmp	x10, x9
1001fae3c: 54000720    	b.eq	0x1001faf20 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x1f8>
1001fae40: aa0a03ec    	mov	x12, x10
1001fae44: 38c0158d    	ldrsb	w13, [x12], #0x1
1001fae48: 12001dab    	and	w11, w13, #0xff
1001fae4c: 37f800ad    	tbnz	w13, #0x1f, 0x1001fae60 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x138>
1001fae50: aa0c03ea    	mov	x10, x12
1001fae54: 7100b57f    	cmp	w11, #0x2d
1001fae58: 54000401    	b.ne	0x1001faed8 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x1b0>
1001fae5c: 17fffff7    	b	0x1001fae38 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x110>
1001fae60: 3940054c    	ldrb	w12, [x10, #0x1]
1001fae64: 1200158d    	and	w13, w12, #0x3f
1001fae68: 7103817f    	cmp	w11, #0xe0
1001fae6c: 54000223    	b.lo	0x1001faeb0 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x188>
1001fae70: 1200116c    	and	w12, w11, #0x1f
1001fae74: 3940094e    	ldrb	w14, [x10, #0x2]
1001fae78: 120015ce    	and	w14, w14, #0x3f
1001fae7c: 2a0d19cd    	orr	w13, w14, w13, lsl #6
1001fae80: 7103c17f    	cmp	w11, #0xf0
1001fae84: 54000223    	b.lo	0x1001faec8 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x1a0>
1001fae88: 39400d4b    	ldrb	w11, [x10, #0x3]
1001fae8c: 1200156b    	and	w11, w11, #0x3f
1001fae90: 2a0d196b    	orr	w11, w11, w13, lsl #6
1001fae94: 330e098b    	bfi	w11, w12, #18, #3
1001fae98: 7144417f    	cmp	w11, #0x110, lsl #12    ; =0x110000
1001fae9c: 54000420    	b.eq	0x1001faf20 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x1f8>
1001faea0: 9100114a    	add	x10, x10, #0x4
1001faea4: 7100b57f    	cmp	w11, #0x2d
1001faea8: 54000181    	b.ne	0x1001faed8 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x1b0>
1001faeac: 17ffffe3    	b	0x1001fae38 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x110>
1001faeb0: 9100094a    	add	x10, x10, #0x2
1001faeb4: 331a116d    	bfi	w13, w11, #6, #5
1001faeb8: aa0d03eb    	mov	x11, x13
1001faebc: 7100b57f    	cmp	w11, #0x2d
1001faec0: 540000c1    	b.ne	0x1001faed8 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x1b0>
1001faec4: 17ffffdd    	b	0x1001fae38 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x110>
1001faec8: 91000d4a    	add	x10, x10, #0x3
1001faecc: 2a0c31ab    	orr	w11, w13, w12, lsl #12
1001faed0: 7100b57f    	cmp	w11, #0x2d
1001faed4: 54fffb20    	b.eq	0x1001fae38 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x110>
1001faed8: 5100e96c    	sub	w12, w11, #0x3a
1001faedc: 121a796d    	and	w13, w11, #0xffffffdf
1001faee0: 51016dad    	sub	w13, w13, #0x5b
1001faee4: 71017d7f    	cmp	w11, #0x5f
1001faee8: 3a4b1982    	ccmn	w12, #0xb, #0x2, ne
1001faeec: 3a5a99a2    	ccmn	w13, #0x1a, #0x2, ls
1001faef0: 54fffa42    	b.hs	0x1001fae38 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x110>
1001faef4: 17ffff98    	b	0x1001fad54 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x2c>
1001faef8: 394127e8    	ldrb	w8, [sp, #0x49]
1001faefc: 370000c8    	tbnz	w8, #0x0, 0x1001faf14 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x1ec>
1001faf00: 394123ea    	ldrb	w10, [sp, #0x48]
1001faf04: a940a7e8    	ldp	x8, x9, [sp, #0x8]
1001faf08: 3700016a    	tbnz	w10, #0x0, 0x1001faf34 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x20c>
1001faf0c: eb08013f    	cmp	x9, x8
1001faf10: 54000121    	b.ne	0x1001faf34 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x20c>
1001faf14: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1001faf18: f90002e8    	str	x8, [x23]
1001faf1c: 17ffff90    	b	0x1001fad5c <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x34>
1001faf20: b6f80193    	tbz	x19, #0x3f, 0x1001faf50 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x228>
1001faf24: d2800015    	mov	x21, #0x0               ; =0
1001faf28: aa1503e0    	mov	x0, x21
1001faf2c: aa1303e1    	mov	x1, x19
1001faf30: 943f55cd    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1001faf34: f9400fea    	ldr	x10, [sp, #0x18]
1001faf38: cb080121    	sub	x1, x9, x8
1001faf3c: 8b080140    	add	x0, x10, x8
1001faf40: 9400d086    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1001faf44: aa1703e8    	mov	x8, x23
1001faf48: b5fff721    	cbnz	x1, 0x1001fae2c <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x104>
1001faf4c: 17ffff82    	b	0x1001fad54 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x2c>
1001faf50: aa0003f4    	mov	x20, x0
1001faf54: 97f81988    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1001faf58: 52800035    	mov	w21, #0x1               ; =1
1001faf5c: aa1303e0    	mov	x0, x19
1001faf60: 52800021    	mov	w1, #0x1                ; =1
1001faf64: 97f81980    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1001faf68: b4fffe00    	cbz	x0, 0x1001faf28 <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x200>
1001faf6c: aa0003f6    	mov	x22, x0
1001faf70: aa1403e1    	mov	x1, x20
1001faf74: aa1303e2    	mov	x2, x19
1001faf78: 943f59cd    	bl	0x1011d16ac <_writev+0x1011d16ac>
1001faf7c: a9005af3    	stp	x19, x22, [x23]
1001faf80: f9000af3    	str	x19, [x23, #0x10]
1001faf84: 17ffff76    	b	0x1001fad5c <__ZN13codexmate_lib4core5relay15config_takeover24top_level_assignment_key17h4bdc006ee7ee0bd2E+0x34>
