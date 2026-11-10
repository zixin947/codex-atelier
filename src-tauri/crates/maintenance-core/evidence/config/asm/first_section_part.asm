
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001003735e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E>:
1003735e4: d10203ff    	sub	sp, sp, #0x80
1003735e8: a9026ffc    	stp	x28, x27, [sp, #0x20]
1003735ec: a90367fa    	stp	x26, x25, [sp, #0x30]
1003735f0: a9045ff8    	stp	x24, x23, [sp, #0x40]
1003735f4: a90557f6    	stp	x22, x21, [sp, #0x50]
1003735f8: a9064ff4    	stp	x20, x19, [sp, #0x60]
1003735fc: a9077bfd    	stp	x29, x30, [sp, #0x70]
100373600: 9101c3fd    	add	x29, sp, #0x70
100373604: aa0803f3    	mov	x19, x8
100373608: 52800035    	mov	w21, #0x1               ; =1
10037360c: a900d7ff    	stp	xzr, x21, [sp, #0x8]
100373610: f9000fff    	str	xzr, [sp, #0x18]
100373614: b4002001    	cbz	x1, 0x100373a14 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x430>
100373618: aa0103e9    	mov	x9, x1
10037361c: aa0003f4    	mov	x20, x0
100373620: d2800001    	mov	x1, #0x0                ; =0
100373624: d280001a    	mov	x26, #0x0               ; =0
100373628: 5280001b    	mov	w27, #0x0               ; =0
10037362c: 5280000a    	mov	w10, #0x0               ; =0
100373630: 8b090016    	add	x22, x0, x9
100373634: 52800037    	mov	w23, #0x1               ; =1
100373638: 52800058    	mov	w24, #0x2               ; =2
10037363c: 52800079    	mov	w25, #0x3               ; =3
100373640: 52800028    	mov	w8, #0x1                ; =1
100373644: 14000008    	b	0x100373664 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x80>
100373648: 3900013c    	strb	w28, [x9]
10037364c: 5280000a    	mov	w10, #0x0               ; =0
100373650: 8b0102a1    	add	x1, x21, x1
100373654: f9000fe1    	str	x1, [sp, #0x18]
100373658: aa0103fa    	mov	x26, x1
10037365c: eb16029f    	cmp	x20, x22
100373660: 54001d80    	b.eq	0x100373a10 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x42c>
100373664: aa1403e9    	mov	x9, x20
100373668: 38c0152b    	ldrsb	w11, [x9], #0x1
10037366c: 12001d7c    	and	w28, w11, #0xff
100373670: 37f8008b    	tbnz	w11, #0x1f, 0x100373680 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x9c>
100373674: aa0903f4    	mov	x20, x9
100373678: 3700036a    	tbnz	w10, #0x0, 0x1003736e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x100>
10037367c: 1400007a    	b	0x100373864 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x280>
100373680: 39400689    	ldrb	w9, [x20, #0x1]
100373684: 1200152b    	and	w11, w9, #0x3f
100373688: 7103839f    	cmp	w28, #0xe0
10037368c: 540001c3    	b.lo	0x1003736c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0xe0>
100373690: 12001389    	and	w9, w28, #0x1f
100373694: 39400a8c    	ldrb	w12, [x20, #0x2]
100373698: 1200158c    	and	w12, w12, #0x3f
10037369c: 2a0b198b    	orr	w11, w12, w11, lsl #6
1003736a0: 7103c39f    	cmp	w28, #0xf0
1003736a4: 540001a3    	b.lo	0x1003736d8 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0xf4>
1003736a8: 39400e8c    	ldrb	w12, [x20, #0x3]
1003736ac: 91001294    	add	x20, x20, #0x4
1003736b0: 1200158c    	and	w12, w12, #0x3f
1003736b4: 2a0b199c    	orr	w28, w12, w11, lsl #6
1003736b8: 330e093c    	bfi	w28, w9, #18, #3
1003736bc: 3700014a    	tbnz	w10, #0x0, 0x1003736e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x100>
1003736c0: 14000069    	b	0x100373864 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x280>
1003736c4: 91000a94    	add	x20, x20, #0x2
1003736c8: 331a138b    	bfi	w11, w28, #6, #5
1003736cc: aa0b03fc    	mov	x28, x11
1003736d0: 370000aa    	tbnz	w10, #0x0, 0x1003736e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x100>
1003736d4: 14000064    	b	0x100373864 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x280>
1003736d8: 91000e94    	add	x20, x20, #0x3
1003736dc: 2a09317c    	orr	w28, w11, w9, lsl #12
1003736e0: 36000c2a    	tbz	w10, #0x0, 0x100373864 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x280>
1003736e4: 7140439f    	cmp	w28, #0x10, lsl #12     ; =0x10000
1003736e8: 9a993729    	cinc	x9, x25, hs
1003736ec: 7120039f    	cmp	w28, #0x800
1003736f0: 9a893309    	csel	x9, x24, x9, lo
1003736f4: 7102039f    	cmp	w28, #0x80
1003736f8: 9a8932f5    	csel	x21, x23, x9, lo
1003736fc: f94007e9    	ldr	x9, [sp, #0x8]
100373700: cb010129    	sub	x9, x9, x1
100373704: eb0902bf    	cmp	x21, x9
100373708: 54000148    	b.hi	0x100373730 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x14c>
10037370c: 8b010109    	add	x9, x8, x1
100373710: 7102039f    	cmp	w28, #0x80
100373714: 54000202    	b.hs	0x100373754 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x170>
100373718: 3900013c    	strb	w28, [x9]
10037371c: 8b1a02a1    	add	x1, x21, x26
100373720: f9000fe1    	str	x1, [sp, #0x18]
100373724: eb16029f    	cmp	x20, x22
100373728: 54000641    	b.ne	0x1003737f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x20c>
10037372c: 140000b9    	b	0x100373a10 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x42c>
100373730: 910023e0    	add	x0, sp, #0x8
100373734: aa1503e2    	mov	x2, x21
100373738: 52800023    	mov	w3, #0x1                ; =1
10037373c: 52800024    	mov	w4, #0x1                ; =1
100373740: 94386c65    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100373744: a94107e8    	ldp	x8, x1, [sp, #0x10]
100373748: 8b010109    	add	x9, x8, x1
10037374c: 7102039f    	cmp	w28, #0x80
100373750: 54fffe43    	b.lo	0x100373718 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x134>
100373754: 12800fea    	mov	w10, #-0x80             ; =-128
100373758: 3300178a    	bfxil	w10, w28, #0, #6
10037375c: 7120039f    	cmp	w28, #0x800
100373760: 54000142    	b.hs	0x100373788 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x1a4>
100373764: 53067f8b    	lsr	w11, w28, #6
100373768: 321a056b    	orr	w11, w11, #0xc0
10037376c: 3900012b    	strb	w11, [x9]
100373770: 3900052a    	strb	w10, [x9, #0x1]
100373774: 8b1a02a1    	add	x1, x21, x26
100373778: f9000fe1    	str	x1, [sp, #0x18]
10037377c: eb16029f    	cmp	x20, x22
100373780: 54000381    	b.ne	0x1003737f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x20c>
100373784: 140000a3    	b	0x100373a10 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x42c>
100373788: 12800feb    	mov	w11, #-0x80             ; =-128
10037378c: 33062f8b    	bfxil	w11, w28, #6, #6
100373790: 7140439f    	cmp	w28, #0x10, lsl #12     ; =0x10000
100373794: 54000162    	b.hs	0x1003737c0 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x1dc>
100373798: 530c7f8c    	lsr	w12, w28, #12
10037379c: 321b098c    	orr	w12, w12, #0xe0
1003737a0: 3900012c    	strb	w12, [x9]
1003737a4: 3900052b    	strb	w11, [x9, #0x1]
1003737a8: 3900092a    	strb	w10, [x9, #0x2]
1003737ac: 8b1a02a1    	add	x1, x21, x26
1003737b0: f9000fe1    	str	x1, [sp, #0x18]
1003737b4: eb16029f    	cmp	x20, x22
1003737b8: 540001c1    	b.ne	0x1003737f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x20c>
1003737bc: 14000095    	b	0x100373a10 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x42c>
1003737c0: 12800fec    	mov	w12, #-0x80             ; =-128
1003737c4: 53127f8d    	lsr	w13, w28, #18
1003737c8: 321c6dad    	orr	w13, w13, #0xfffffff0
1003737cc: 3900012d    	strb	w13, [x9]
1003737d0: 330c478c    	bfxil	w12, w28, #12, #6
1003737d4: 3900052c    	strb	w12, [x9, #0x1]
1003737d8: 3900092b    	strb	w11, [x9, #0x2]
1003737dc: 39000d2a    	strb	w10, [x9, #0x3]
1003737e0: 8b1a02a1    	add	x1, x21, x26
1003737e4: f9000fe1    	str	x1, [sp, #0x18]
1003737e8: eb16029f    	cmp	x20, x22
1003737ec: 54001120    	b.eq	0x100373a10 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x42c>
1003737f0: aa1403e9    	mov	x9, x20
1003737f4: 38c0152a    	ldrsb	w10, [x9], #0x1
1003737f8: 12001d5c    	and	w28, w10, #0xff
1003737fc: 37f800ca    	tbnz	w10, #0x1f, 0x100373814 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x230>
100373800: aa0103fa    	mov	x26, x1
100373804: aa0903f4    	mov	x20, x9
100373808: 71008b9f    	cmp	w28, #0x22
10037380c: 54000301    	b.ne	0x10037386c <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x288>
100373810: 14000074    	b	0x1003739e0 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x3fc>
100373814: 39400689    	ldrb	w9, [x20, #0x1]
100373818: 1200152a    	and	w10, w9, #0x3f
10037381c: 7103839f    	cmp	w28, #0xe0
100373820: 540001a3    	b.lo	0x100373854 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x270>
100373824: 12001389    	and	w9, w28, #0x1f
100373828: 39400a8b    	ldrb	w11, [x20, #0x2]
10037382c: 1200156b    	and	w11, w11, #0x3f
100373830: 2a0a196a    	orr	w10, w11, w10, lsl #6
100373834: 7103c39f    	cmp	w28, #0xf0
100373838: 54000ca3    	b.lo	0x1003739cc <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x3e8>
10037383c: 39400e8b    	ldrb	w11, [x20, #0x3]
100373840: 91001294    	add	x20, x20, #0x4
100373844: 1200156b    	and	w11, w11, #0x3f
100373848: 2a0a197c    	orr	w28, w11, w10, lsl #6
10037384c: 330e093c    	bfi	w28, w9, #18, #3
100373850: 14000061    	b	0x1003739d4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x3f0>
100373854: 91000a94    	add	x20, x20, #0x2
100373858: 331a138a    	bfi	w10, w28, #6, #5
10037385c: aa0103fa    	mov	x26, x1
100373860: aa0a03fc    	mov	x28, x10
100373864: 71008b9f    	cmp	w28, #0x22
100373868: 54000bc0    	b.eq	0x1003739e0 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x3fc>
10037386c: 7100bb9f    	cmp	w28, #0x2e
100373870: 54000220    	b.eq	0x1003738b4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x2d0>
100373874: 7101739f    	cmp	w28, #0x5c
100373878: 54000301    	b.ne	0x1003738d8 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x2f4>
10037387c: 3600073b    	tbz	w27, #0x0, 0x100373960 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x37c>
100373880: f94007e8    	ldr	x8, [sp, #0x8]
100373884: aa0103e9    	mov	x9, x1
100373888: eb01011f    	cmp	x8, x1
10037388c: 54000b00    	b.eq	0x1003739ec <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x408>
100373890: f9400be8    	ldr	x8, [sp, #0x10]
100373894: 52800b8a    	mov	w10, #0x5c              ; =92
100373898: 3829690a    	strb	w10, [x8, x9]
10037389c: 91000421    	add	x1, x1, #0x1
1003738a0: f9000fe1    	str	x1, [sp, #0x18]
1003738a4: 5280002a    	mov	w10, #0x1               ; =1
1003738a8: aa0103fa    	mov	x26, x1
1003738ac: 5280003b    	mov	w27, #0x1               ; =1
1003738b0: 17ffff6b    	b	0x10037365c <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x78>
1003738b4: 36000afb    	tbz	w27, #0x0, 0x100373a10 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x42c>
1003738b8: 52800035    	mov	w21, #0x1               ; =1
1003738bc: aa1503fa    	mov	x26, x21
1003738c0: f94007e8    	ldr	x8, [sp, #0x8]
1003738c4: cb010108    	sub	x8, x8, x1
1003738c8: eb0802bf    	cmp	x21, x8
1003738cc: aa0103e9    	mov	x9, x1
1003738d0: 540001c9    	b.ls	0x100373908 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x324>
1003738d4: 1400002a    	b	0x10037397c <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x398>
1003738d8: 7140439f    	cmp	w28, #0x10, lsl #12     ; =0x10000
1003738dc: 9a993728    	cinc	x8, x25, hs
1003738e0: 7120039f    	cmp	w28, #0x800
1003738e4: 9a883308    	csel	x8, x24, x8, lo
1003738e8: 7102039f    	cmp	w28, #0x80
1003738ec: 1a9f32fa    	csel	w26, w23, wzr, lo
1003738f0: 9a8832f5    	csel	x21, x23, x8, lo
1003738f4: f94007e8    	ldr	x8, [sp, #0x8]
1003738f8: cb010108    	sub	x8, x8, x1
1003738fc: eb0802bf    	cmp	x21, x8
100373900: aa0103e9    	mov	x9, x1
100373904: 540003c8    	b.hi	0x10037397c <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x398>
100373908: f9400be8    	ldr	x8, [sp, #0x10]
10037390c: 8b090109    	add	x9, x8, x9
100373910: 3707e9da    	tbnz	w26, #0x0, 0x100373648 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x64>
100373914: 12800fea    	mov	w10, #-0x80             ; =-128
100373918: 3300178a    	bfxil	w10, w28, #0, #6
10037391c: 7120039f    	cmp	w28, #0x800
100373920: 540000c2    	b.hs	0x100373938 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x354>
100373924: 53067f8b    	lsr	w11, w28, #6
100373928: 321a056b    	orr	w11, w11, #0xc0
10037392c: 3900012b    	strb	w11, [x9]
100373930: 3900052a    	strb	w10, [x9, #0x1]
100373934: 17ffff46    	b	0x10037364c <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x68>
100373938: 12800feb    	mov	w11, #-0x80             ; =-128
10037393c: 33062f8b    	bfxil	w11, w28, #6, #6
100373940: 53107f8c    	lsr	w12, w28, #16
100373944: 3500032c    	cbnz	w12, 0x1003739a8 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x3c4>
100373948: 530c7f8c    	lsr	w12, w28, #12
10037394c: 321b098c    	orr	w12, w12, #0xe0
100373950: 3900012c    	strb	w12, [x9]
100373954: 3900052b    	strb	w11, [x9, #0x1]
100373958: 3900092a    	strb	w10, [x9, #0x2]
10037395c: 17ffff3c    	b	0x10037364c <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x68>
100373960: 5280003a    	mov	w26, #0x1               ; =1
100373964: 52800035    	mov	w21, #0x1               ; =1
100373968: f94007e8    	ldr	x8, [sp, #0x8]
10037396c: cb010108    	sub	x8, x8, x1
100373970: eb0802bf    	cmp	x21, x8
100373974: aa0103e9    	mov	x9, x1
100373978: 54fffc89    	b.ls	0x100373908 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x324>
10037397c: 910023e0    	add	x0, sp, #0x8
100373980: f90003e1    	str	x1, [sp]
100373984: aa1503e2    	mov	x2, x21
100373988: 52800023    	mov	w3, #0x1                ; =1
10037398c: 52800024    	mov	w4, #0x1                ; =1
100373990: 94386bd1    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100373994: a94127e8    	ldp	x8, x9, [sp, #0x10]
100373998: f94003e1    	ldr	x1, [sp]
10037399c: 8b090109    	add	x9, x8, x9
1003739a0: 3707e55a    	tbnz	w26, #0x0, 0x100373648 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x64>
1003739a4: 17ffffdc    	b	0x100373914 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x330>
1003739a8: 12800fec    	mov	w12, #-0x80             ; =-128
1003739ac: 53127f8d    	lsr	w13, w28, #18
1003739b0: 321c6dad    	orr	w13, w13, #0xfffffff0
1003739b4: 3900012d    	strb	w13, [x9]
1003739b8: 330c478c    	bfxil	w12, w28, #12, #6
1003739bc: 3900052c    	strb	w12, [x9, #0x1]
1003739c0: 3900092b    	strb	w11, [x9, #0x2]
1003739c4: 39000d2a    	strb	w10, [x9, #0x3]
1003739c8: 17ffff21    	b	0x10037364c <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x68>
1003739cc: 91000e94    	add	x20, x20, #0x3
1003739d0: 2a09315c    	orr	w28, w10, w9, lsl #12
1003739d4: aa0103fa    	mov	x26, x1
1003739d8: 71008b9f    	cmp	w28, #0x22
1003739dc: 54fff481    	b.ne	0x10037386c <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x288>
1003739e0: 5280000a    	mov	w10, #0x0               ; =0
1003739e4: 5200037b    	eor	w27, w27, #0x1
1003739e8: 17ffff1d    	b	0x10037365c <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x78>
1003739ec: 910023e0    	add	x0, sp, #0x8
1003739f0: aa0103f5    	mov	x21, x1
1003739f4: 52800022    	mov	w2, #0x1                ; =1
1003739f8: 52800023    	mov	w3, #0x1                ; =1
1003739fc: 52800024    	mov	w4, #0x1                ; =1
100373a00: 94386bb5    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
100373a04: f9400fe9    	ldr	x9, [sp, #0x18]
100373a08: aa1503e1    	mov	x1, x21
100373a0c: 17ffffa1    	b	0x100373890 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x2ac>
100373a10: f9400bf5    	ldr	x21, [sp, #0x10]
100373a14: aa1503e0    	mov	x0, x21
100373a18: 97faedd0    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100373a1c: aa0103f4    	mov	x20, x1
100373a20: b6f800c1    	tbz	x1, #0x3f, 0x100373a38 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x454>
100373a24: d2800018    	mov	x24, #0x0               ; =0
100373a28: aa1803e0    	mov	x0, x24
100373a2c: aa1403e1    	mov	x1, x20
100373a30: 9439730d    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100373a34: d4200020    	brk	#0x1
100373a38: aa0003f6    	mov	x22, x0
100373a3c: b4000134    	cbz	x20, 0x100373a60 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x47c>
100373a40: 97f236cd    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100373a44: 52800038    	mov	w24, #0x1               ; =1
100373a48: aa1403e0    	mov	x0, x20
100373a4c: 52800021    	mov	w1, #0x1                ; =1
100373a50: 97f236c5    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100373a54: b4fffea0    	cbz	x0, 0x100373a28 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x444>
100373a58: aa0003f7    	mov	x23, x0
100373a5c: 14000002    	b	0x100373a64 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x480>
100373a60: 52800037    	mov	w23, #0x1               ; =1
100373a64: aa1703e0    	mov	x0, x23
100373a68: aa1603e1    	mov	x1, x22
100373a6c: aa1403e2    	mov	x2, x20
100373a70: 9439770f    	bl	0x1011d16ac <_writev+0x1011d16ac>
100373a74: a9005e74    	stp	x20, x23, [x19]
100373a78: f9000a74    	str	x20, [x19, #0x10]
100373a7c: f94007e1    	ldr	x1, [sp, #0x8]
100373a80: b4000161    	cbz	x1, 0x100373aac <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x4c8>
100373a84: aa1503e0    	mov	x0, x21
100373a88: 52800022    	mov	w2, #0x1                ; =1
100373a8c: a9477bfd    	ldp	x29, x30, [sp, #0x70]
100373a90: a9464ff4    	ldp	x20, x19, [sp, #0x60]
100373a94: a94557f6    	ldp	x22, x21, [sp, #0x50]
100373a98: a9445ff8    	ldp	x24, x23, [sp, #0x40]
100373a9c: a94367fa    	ldp	x26, x25, [sp, #0x30]
100373aa0: a9426ffc    	ldp	x28, x27, [sp, #0x20]
100373aa4: 910203ff    	add	sp, sp, #0x80
100373aa8: 17f236b0    	b	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100373aac: a9477bfd    	ldp	x29, x30, [sp, #0x70]
100373ab0: a9464ff4    	ldp	x20, x19, [sp, #0x60]
100373ab4: a94557f6    	ldp	x22, x21, [sp, #0x50]
100373ab8: a9445ff8    	ldp	x24, x23, [sp, #0x40]
100373abc: a94367fa    	ldp	x26, x25, [sp, #0x30]
100373ac0: a9426ffc    	ldp	x28, x27, [sp, #0x20]
100373ac4: 910203ff    	add	sp, sp, #0x80
100373ac8: d65f03c0    	ret
100373acc: 14000002    	b	0x100373ad4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x4f0>
100373ad0: 14000001    	b	0x100373ad4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x4f0>
100373ad4: f94007e1    	ldr	x1, [sp, #0x8]
100373ad8: b40000e1    	cbz	x1, 0x100373af4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E+0x510>
100373adc: f9400be8    	ldr	x8, [sp, #0x10]
100373ae0: aa0003f3    	mov	x19, x0
100373ae4: aa0803e0    	mov	x0, x8
100373ae8: 52800022    	mov	w2, #0x1                ; =1
100373aec: 97f2369f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100373af0: aa1303e0    	mov	x0, x19
100373af4: 94397598    	bl	0x1011d1154 <_writev+0x1011d1154>
