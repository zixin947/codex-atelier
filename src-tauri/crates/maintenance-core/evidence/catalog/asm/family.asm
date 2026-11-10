
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001002007bc <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E>:
1002007bc: d10343ff    	sub	sp, sp, #0xd0
1002007c0: a90867fa    	stp	x26, x25, [sp, #0x80]
1002007c4: a9095ff8    	stp	x24, x23, [sp, #0x90]
1002007c8: a90a57f6    	stp	x22, x21, [sp, #0xa0]
1002007cc: a90b4ff4    	stp	x20, x19, [sp, #0xb0]
1002007d0: a90c7bfd    	stp	x29, x30, [sp, #0xc0]
1002007d4: 910303fd    	add	x29, sp, #0xc0
1002007d8: aa0303f5    	mov	x21, x3
1002007dc: aa0103f3    	mov	x19, x1
1002007e0: aa0003f4    	mov	x20, x0
1002007e4: 910003e8    	mov	x8, sp
1002007e8: 943e1434    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
1002007ec: d10163a0    	sub	x0, x29, #0x58
1002007f0: 910003e1    	mov	x1, sp
1002007f4: 9400d63d    	bl	0x1002360e8 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
1002007f8: b85a83a8    	ldur	w8, [x29, #-0x58]
1002007fc: 7100051f    	cmp	w8, #0x1
100200800: 54001641    	b.ne	0x100200ac8 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x30c>
100200804: 8b130296    	add	x22, x20, x19
100200808: 529fffb7    	mov	w23, #0xfffd            ; =65533
10020080c: 72a003f7    	movk	w23, #0x1f, lsl #16
100200810: 52800038    	mov	w24, #0x1               ; =1
100200814: 528400b9    	mov	w25, #0x2005            ; =8197
100200818: 72a00119    	movk	w25, #0x8, lsl #16
10020081c: 14000006    	b	0x100200834 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x78>
100200820: d10163a0    	sub	x0, x29, #0x58
100200824: 910003e1    	mov	x1, sp
100200828: 9400d630    	bl	0x1002360e8 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
10020082c: b85a83a8    	ldur	w8, [x29, #-0x58]
100200830: 360014c8    	tbz	w8, #0x0, 0x100200ac8 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x30c>
100200834: f85b03a8    	ldur	x8, [x29, #-0x50]
100200838: b40001a8    	cbz	x8, 0x10020086c <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0xb0>
10020083c: d1000500    	sub	x0, x8, #0x1
100200840: eb13001f    	cmp	x0, x19
100200844: 54001582    	b.hs	0x100200af4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x338>
100200848: 38606a89    	ldrb	w9, [x20, x0]
10020084c: 5100b529    	sub	w9, w9, #0x2d
100200850: 1ac9272a    	lsr	w10, w25, w9
100200854: 7100513f    	cmp	w9, #0x14
100200858: 1a9f3149    	csel	w9, w10, wzr, lo
10020085c: 8b150108    	add	x8, x8, x21
100200860: eb13011f    	cmp	x8, x19
100200864: 540000c1    	b.ne	0x10020087c <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0xc0>
100200868: 1400000d    	b	0x10020089c <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0xe0>
10020086c: 52800029    	mov	w9, #0x1                ; =1
100200870: 8b150108    	add	x8, x8, x21
100200874: eb13011f    	cmp	x8, x19
100200878: 54000120    	b.eq	0x10020089c <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0xe0>
10020087c: 54001442    	b.hs	0x100200b04 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x348>
100200880: 38686a8a    	ldrb	w10, [x20, x8]
100200884: 5100b54a    	sub	w10, w10, #0x2d
100200888: 71004d5f    	cmp	w10, #0x13
10020088c: 1aca230a    	lsl	w10, w24, w10
100200890: 0a19014a    	and	w10, w10, w25
100200894: 7a409944    	ccmp	w10, #0x0, #0x4, ls
100200898: 54fffc40    	b.eq	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
10020089c: 3607fc29    	tbz	w9, #0x0, 0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
1002008a0: b40000c8    	cbz	x8, 0x1002008b8 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0xfc>
1002008a4: eb13011f    	cmp	x8, x19
1002008a8: 54001142    	b.hs	0x100200ad0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x314>
1002008ac: 38e86a89    	ldrsb	w9, [x20, x8]
1002008b0: 3101013f    	cmn	w9, #0x40
1002008b4: 5400132b    	b.lt	0x100200b18 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x35c>
1002008b8: eb13011f    	cmp	x8, x19
1002008bc: 540010c0    	b.eq	0x100200ad4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x318>
1002008c0: 8b080289    	add	x9, x20, x8
1002008c4: 3940012a    	ldrb	w10, [x9]
1002008c8: 7100b55f    	cmp	w10, #0x2d
1002008cc: 54001041    	b.ne	0x100200ad4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x318>
1002008d0: d280000f    	mov	x15, #0x0               ; =0
1002008d4: aa2803e8    	mvn	x8, x8
1002008d8: 8b08026b    	add	x11, x19, x8
1002008dc: 9100052a    	add	x10, x9, #0x1
1002008e0: aa0a03ec    	mov	x12, x10
1002008e4: eb16019f    	cmp	x12, x22
1002008e8: 54000620    	b.eq	0x1002009ac <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x1f0>
1002008ec: aa0f03e8    	mov	x8, x15
1002008f0: aa0c03ee    	mov	x14, x12
1002008f4: 38c015cf    	ldrsb	w15, [x14], #0x1
1002008f8: 12001ded    	and	w13, w15, #0xff
1002008fc: 37f8018f    	tbnz	w15, #0x1f, 0x10020092c <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x170>
100200900: 0a1701af    	and	w15, w13, w23
100200904: 7100b5ff    	cmp	w15, #0x2d
100200908: 54000540    	b.eq	0x1002009b0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x1f4>
10020090c: 710101bf    	cmp	w13, #0x40
100200910: 54000500    	b.eq	0x1002009b0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x1f4>
100200914: cb0c010c    	sub	x12, x8, x12
100200918: 8b0e018f    	add	x15, x12, x14
10020091c: aa0e03ec    	mov	x12, x14
100200920: 7100e9bf    	cmp	w13, #0x3a
100200924: 54fffe01    	b.ne	0x1002008e4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x128>
100200928: 14000022    	b	0x1002009b0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x1f4>
10020092c: 3940058e    	ldrb	w14, [x12, #0x1]
100200930: 120015d0    	and	w16, w14, #0x3f
100200934: 710381bf    	cmp	w13, #0xe0
100200938: 54000203    	b.lo	0x100200978 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x1bc>
10020093c: 120011af    	and	w15, w13, #0x1f
100200940: 3940098e    	ldrb	w14, [x12, #0x2]
100200944: 120015ce    	and	w14, w14, #0x3f
100200948: 2a1019d0    	orr	w16, w14, w16, lsl #6
10020094c: 7103c1bf    	cmp	w13, #0xf0
100200950: 54000223    	b.lo	0x100200994 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x1d8>
100200954: 9100118e    	add	x14, x12, #0x4
100200958: 39400d8d    	ldrb	w13, [x12, #0x3]
10020095c: 120015ad    	and	w13, w13, #0x3f
100200960: 2a1019ad    	orr	w13, w13, w16, lsl #6
100200964: 330e09ed    	bfi	w13, w15, #18, #3
100200968: 0a1701af    	and	w15, w13, w23
10020096c: 7100b5ff    	cmp	w15, #0x2d
100200970: 54fffce1    	b.ne	0x10020090c <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x150>
100200974: 1400000f    	b	0x1002009b0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x1f4>
100200978: 9100098e    	add	x14, x12, #0x2
10020097c: 331a11b0    	bfi	w16, w13, #6, #5
100200980: aa1003ed    	mov	x13, x16
100200984: 0a1701af    	and	w15, w13, w23
100200988: 7100b5ff    	cmp	w15, #0x2d
10020098c: 54fffc01    	b.ne	0x10020090c <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x150>
100200990: 14000008    	b	0x1002009b0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x1f4>
100200994: 91000d8e    	add	x14, x12, #0x3
100200998: 2a0f320d    	orr	w13, w16, w15, lsl #12
10020099c: 0a1701af    	and	w15, w13, w23
1002009a0: 7100b5ff    	cmp	w15, #0x2d
1002009a4: 54fffb41    	b.ne	0x10020090c <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x150>
1002009a8: 14000002    	b	0x1002009b0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x1f4>
1002009ac: aa0b03e8    	mov	x8, x11

/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001002007bc <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E>:
1002009ac: aa0b03e8    	mov	x8, x11
1002009b0: b4000928    	cbz	x8, 0x100200ad4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x318>
1002009b4: 39c0014c    	ldrsb	w12, [x10]
1002009b8: 12001d8a    	and	w10, w12, #0xff
1002009bc: aa0a03eb    	mov	x11, x10
1002009c0: 37f804cc    	tbnz	w12, #0x1f, 0x100200a58 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x29c>
1002009c4: 5100c16b    	sub	w11, w11, #0x30
1002009c8: 7100257f    	cmp	w11, #0x9
1002009cc: 54000848    	b.hi	0x100200ad4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x318>
1002009d0: f100211f    	cmp	x8, #0x8
1002009d4: 54fff261    	b.ne	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
1002009d8: 5100e948    	sub	w8, w10, #0x3a
1002009dc: 3100291f    	cmn	w8, #0xa
1002009e0: 54fff203    	b.lo	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
1002009e4: 39400928    	ldrb	w8, [x9, #0x2]
1002009e8: 5100e908    	sub	w8, w8, #0x3a
1002009ec: 3100291f    	cmn	w8, #0xa
1002009f0: 54fff183    	b.lo	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
1002009f4: 39400d28    	ldrb	w8, [x9, #0x3]
1002009f8: 5100e908    	sub	w8, w8, #0x3a
1002009fc: 3100291f    	cmn	w8, #0xa
100200a00: 54fff103    	b.lo	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
100200a04: 39401128    	ldrb	w8, [x9, #0x4]
100200a08: 5100e908    	sub	w8, w8, #0x3a
100200a0c: 3100291f    	cmn	w8, #0xa
100200a10: 54fff083    	b.lo	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
100200a14: 39401528    	ldrb	w8, [x9, #0x5]
100200a18: 5100e908    	sub	w8, w8, #0x3a
100200a1c: 3100291f    	cmn	w8, #0xa
100200a20: 54fff003    	b.lo	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
100200a24: 39401928    	ldrb	w8, [x9, #0x6]
100200a28: 5100e908    	sub	w8, w8, #0x3a
100200a2c: 3100291f    	cmn	w8, #0xa
100200a30: 54ffef83    	b.lo	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
100200a34: 39401d28    	ldrb	w8, [x9, #0x7]
100200a38: 5100e908    	sub	w8, w8, #0x3a
100200a3c: 3100291f    	cmn	w8, #0xa
100200a40: 54ffef03    	b.lo	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
100200a44: 39402128    	ldrb	w8, [x9, #0x8]
100200a48: 5100c108    	sub	w8, w8, #0x30
100200a4c: 7100291f    	cmp	w8, #0xa
100200a50: 54ffee82    	b.hs	0x100200820 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x64>
100200a54: 14000020    	b	0x100200ad4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x318>
100200a58: 3940092b    	ldrb	w11, [x9, #0x2]
100200a5c: 1200156b    	and	w11, w11, #0x3f
100200a60: 7103815f    	cmp	w10, #0xe0
100200a64: 540001e3    	b.lo	0x100200aa0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x2e4>
100200a68: 1200114c    	and	w12, w10, #0x1f
100200a6c: 39400d2d    	ldrb	w13, [x9, #0x3]
100200a70: 120015ad    	and	w13, w13, #0x3f
100200a74: 2a0b19ab    	orr	w11, w13, w11, lsl #6
100200a78: 7103c15f    	cmp	w10, #0xf0
100200a7c: 540001c3    	b.lo	0x100200ab4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x2f8>
100200a80: 3940112d    	ldrb	w13, [x9, #0x4]
100200a84: 120015ad    	and	w13, w13, #0x3f
100200a88: 2a0b19ab    	orr	w11, w13, w11, lsl #6
100200a8c: 330e098b    	bfi	w11, w12, #18, #3
100200a90: 5100c16b    	sub	w11, w11, #0x30
100200a94: 7100257f    	cmp	w11, #0x9
100200a98: 54fff9c9    	b.ls	0x1002009d0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x214>
100200a9c: 1400000e    	b	0x100200ad4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x318>
100200aa0: 331a114b    	bfi	w11, w10, #6, #5
100200aa4: 5100c16b    	sub	w11, w11, #0x30
100200aa8: 7100257f    	cmp	w11, #0x9
100200aac: 54fff929    	b.ls	0x1002009d0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x214>
100200ab0: 14000009    	b	0x100200ad4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x318>
100200ab4: 2a0c316b    	orr	w11, w11, w12, lsl #12
100200ab8: 5100c16b    	sub	w11, w11, #0x30
100200abc: 7100257f    	cmp	w11, #0x9
100200ac0: 54fff889    	b.ls	0x1002009d0 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x214>
100200ac4: 14000004    	b	0x100200ad4 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x318>
100200ac8: 52800000    	mov	w0, #0x0                ; =0
100200acc: 14000003    	b	0x100200ad8 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x31c>
100200ad0: 54000241    	b.ne	0x100200b18 <__ZN13codexmate_lib4core5relay19anthropic_reasoning20model_matches_family17h68ca3e0bda3fd0c8E+0x35c>
100200ad4: 52800020    	mov	w0, #0x1                ; =1
100200ad8: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
100200adc: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
100200ae0: a94a57f6    	ldp	x22, x21, [sp, #0xa0]
100200ae4: a9495ff8    	ldp	x24, x23, [sp, #0x90]
100200ae8: a94867fa    	ldp	x26, x25, [sp, #0x80]
100200aec: 910343ff    	add	sp, sp, #0xd0
100200af0: d65f03c0    	ret
100200af4: f000b0a2    	adrp	x2, 0x101817000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.56+0x2d8>
100200af8: 91106042    	add	x2, x2, #0x418
100200afc: aa1303e1    	mov	x1, x19
100200b00: 943f4018    	bl	0x1011d0b60 <__ZN4core9panicking18panic_bounds_check17h865e5d10cdb12e28E>
100200b04: f000b0a2    	adrp	x2, 0x101817000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.56+0x2d8>
100200b08: 9110c042    	add	x2, x2, #0x430
100200b0c: aa0803e0    	mov	x0, x8
100200b10: aa1303e1    	mov	x1, x19
100200b14: 943f4013    	bl	0x1011d0b60 <__ZN4core9panicking18panic_bounds_check17h865e5d10cdb12e28E>
100200b18: f000b0a4    	adrp	x4, 0x101817000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.56+0x2d8>
100200b1c: 91112084    	add	x4, x4, #0x448
100200b20: aa1403e0    	mov	x0, x20
100200b24: aa1303e1    	mov	x1, x19
100200b28: aa0803e2    	mov	x2, x8
100200b2c: aa1303e3    	mov	x3, x19
100200b30: 943f3f3a    	bl	0x1011d0818 <__ZN4core3str16slice_error_fail17hecde84adb67345c5E>
