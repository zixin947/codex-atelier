
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100538670 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE>:
100538670: d10343ff    	sub	sp, sp, #0xd0
100538674: a90867fa    	stp	x26, x25, [sp, #0x80]
100538678: a9095ff8    	stp	x24, x23, [sp, #0x90]
10053867c: a90a57f6    	stp	x22, x21, [sp, #0xa0]
100538680: a90b4ff4    	stp	x20, x19, [sp, #0xb0]
100538684: a90c7bfd    	stp	x29, x30, [sp, #0xc0]
100538688: 910303fd    	add	x29, sp, #0xc0
10053868c: aa0803f3    	mov	x19, x8
100538690: 39410408    	ldrb	w8, [x0, #0x41]
100538694: 36000168    	tbz	w8, #0x0, 0x1005386c0 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x50>
100538698: 52800108    	mov	w8, #0x8                ; =8
10053869c: a900227f    	stp	xzr, x8, [x19]
1005386a0: f9000a7f    	str	xzr, [x19, #0x10]
1005386a4: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
1005386a8: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
1005386ac: a94a57f6    	ldp	x22, x21, [sp, #0xa0]
1005386b0: a9495ff8    	ldp	x24, x23, [sp, #0x90]
1005386b4: a94867fa    	ldp	x26, x25, [sp, #0x80]
1005386b8: 910343ff    	add	sp, sp, #0xd0
1005386bc: d65f03c0    	ret
1005386c0: aa0003f5    	mov	x21, x0
1005386c4: aa0003f4    	mov	x20, x0
1005386c8: f8410e96    	ldr	x22, [x20, #0x10]!
1005386cc: 910083e0    	add	x0, sp, #0x20
1005386d0: aa1403e1    	mov	x1, x20
1005386d4: 9400b365    	bl	0x100565468 <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
1005386d8: b94023e8    	ldr	w8, [sp, #0x20]
1005386dc: 7100051f    	cmp	w8, #0x1
1005386e0: 54000481    	b.ne	0x100538770 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x100>
1005386e4: a942a7e8    	ldp	x8, x9, [sp, #0x28]
1005386e8: f94002aa    	ldr	x10, [x21]
1005386ec: cb0a0101    	sub	x1, x8, x10
1005386f0: 8b0a02c0    	add	x0, x22, x10
1005386f4: f90002a9    	str	x9, [x21]
1005386f8: 97f3da98    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1005386fc: 52800442    	mov	w2, #0x22               ; =34
100538700: 97f3db27    	bl	0x10022f39c <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h3c87e74f9c47ae93E>
100538704: b4fffca0    	cbz	x0, 0x100538698 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x28>
100538708: aa0003f6    	mov	x22, x0
10053870c: aa0103f7    	mov	x23, x1
100538710: 97eb2399    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100538714: 52800800    	mov	w0, #0x40               ; =64
100538718: 52800101    	mov	w1, #0x8                ; =8
10053871c: 97eb2392    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100538720: b4000b20    	cbz	x0, 0x100538884 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x214>
100538724: aa0003f4    	mov	x20, x0
100538728: a9005c16    	stp	x22, x23, [x0]
10053872c: 52800088    	mov	w8, #0x4                ; =4
100538730: a90083e8    	stp	x8, x0, [sp, #0x8]
100538734: 52800038    	mov	w24, #0x1               ; =1
100538738: f9000ff8    	str	x24, [sp, #0x18]
10053873c: f94022a8    	ldr	x8, [x21, #0x40]
100538740: f90033e8    	str	x8, [sp, #0x60]
100538744: ad4106a0    	ldp	q0, q1, [x21, #0x20]
100538748: ad0207e0    	stp	q0, q1, [sp, #0x40]
10053874c: ad4002a1    	ldp	q1, q0, [x21]
100538750: ad0103e1    	stp	q1, q0, [sp, #0x20]
100538754: d3483d08    	ubfx	x8, x8, #8, #8
100538758: 360002e8    	tbz	w8, #0x0, 0x1005387b4 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x144>
10053875c: 3cc083e0    	ldur	q0, [sp, #0x8]
100538760: 3d800260    	str	q0, [x19]
100538764: f9400fe8    	ldr	x8, [sp, #0x18]
100538768: f9000a68    	str	x8, [x19, #0x10]
10053876c: 17ffffce    	b	0x1005386a4 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x34>
100538770: 394106a8    	ldrb	w8, [x21, #0x41]
100538774: 3707f928    	tbnz	w8, #0x0, 0x100538698 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x28>
100538778: 52800028    	mov	w8, #0x1                ; =1
10053877c: 390106a8    	strb	w8, [x21, #0x41]
100538780: 394102aa    	ldrb	w10, [x21, #0x40]
100538784: a94026a8    	ldp	x8, x9, [x21]
100538788: 3700006a    	tbnz	w10, #0x0, 0x100538794 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x124>
10053878c: eb08013f    	cmp	x9, x8
100538790: 54fff840    	b.eq	0x100538698 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x28>
100538794: f940028a    	ldr	x10, [x20]
100538798: cb080121    	sub	x1, x9, x8
10053879c: 8b080140    	add	x0, x10, x8
1005387a0: 97f3da6e    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1005387a4: 52800442    	mov	w2, #0x22               ; =34
1005387a8: 97f3dafd    	bl	0x10022f39c <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h3c87e74f9c47ae93E>
1005387ac: b5fffae0    	cbnz	x0, 0x100538708 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x98>
1005387b0: 17ffffba    	b	0x100538698 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x28>
1005387b4: d2800019    	mov	x25, #0x0               ; =0
1005387b8: 910083fa    	add	x26, sp, #0x20
1005387bc: 52800035    	mov	w21, #0x1               ; =1
1005387c0: 14000008    	b	0x1005387e0 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x170>
1005387c4: 8b190288    	add	x8, x20, x25
1005387c8: a9015d16    	stp	x22, x23, [x8, #0x10]
1005387cc: 910006b5    	add	x21, x21, #0x1
1005387d0: f9000ff5    	str	x21, [sp, #0x18]
1005387d4: 394187e8    	ldrb	w8, [sp, #0x61]
1005387d8: 91004339    	add	x25, x25, #0x10
1005387dc: 3707fc08    	tbnz	w8, #0x0, 0x10053875c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0xec>
1005387e0: f9401bf6    	ldr	x22, [sp, #0x30]
1005387e4: d10163a0    	sub	x0, x29, #0x58
1005387e8: 91004341    	add	x1, x26, #0x10
1005387ec: 9400b31f    	bl	0x100565468 <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
1005387f0: b85a83a8    	ldur	w8, [x29, #-0x58]
1005387f4: 7100051f    	cmp	w8, #0x1
1005387f8: 540000e1    	b.ne	0x100538814 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x1a4>
1005387fc: a97b27a8    	ldp	x8, x9, [x29, #-0x50]
100538800: f94013ea    	ldr	x10, [sp, #0x20]
100538804: cb0a0101    	sub	x1, x8, x10
100538808: 8b0a02c0    	add	x0, x22, x10
10053880c: f90013e9    	str	x9, [sp, #0x20]
100538810: 1400000c    	b	0x100538840 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x1d0>
100538814: 394187e8    	ldrb	w8, [sp, #0x61]
100538818: 3707fa28    	tbnz	w8, #0x0, 0x10053875c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0xec>
10053881c: 390187f8    	strb	w24, [sp, #0x61]
100538820: 394183ea    	ldrb	w10, [sp, #0x60]
100538824: a94227e8    	ldp	x8, x9, [sp, #0x20]
100538828: 3700006a    	tbnz	w10, #0x0, 0x100538834 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x1c4>
10053882c: eb08013f    	cmp	x9, x8
100538830: 54fff960    	b.eq	0x10053875c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0xec>
100538834: f9401bea    	ldr	x10, [sp, #0x30]
100538838: cb080121    	sub	x1, x9, x8
10053883c: 8b080140    	add	x0, x10, x8
100538840: 97f3da46    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100538844: 52800442    	mov	w2, #0x22               ; =34
100538848: 97f3dad5    	bl	0x10022f39c <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h3c87e74f9c47ae93E>
10053884c: aa0003f6    	mov	x22, x0
100538850: b4fff860    	cbz	x0, 0x10053875c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0xec>
100538854: aa0103f7    	mov	x23, x1
100538858: f94007e8    	ldr	x8, [sp, #0x8]
10053885c: eb0802bf    	cmp	x21, x8
100538860: 54fffb21    	b.ne	0x1005387c4 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x154>
100538864: 910023e0    	add	x0, sp, #0x8
100538868: aa1503e1    	mov	x1, x21
10053886c: 52800022    	mov	w2, #0x1                ; =1
100538870: 52800103    	mov	w3, #0x8                ; =8
100538874: 52800204    	mov	w4, #0x10               ; =16
100538878: 94315817    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
10053887c: f9400bf4    	ldr	x20, [sp, #0x10]
100538880: 17ffffd1    	b	0x1005387c4 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x154>
100538884: 52800100    	mov	w0, #0x8                ; =8
100538888: 52800801    	mov	w1, #0x40               ; =64
10053888c: 94325f76    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100538890: aa0003f3    	mov	x19, x0
100538894: f94007e8    	ldr	x8, [sp, #0x8]
100538898: b40000a8    	cbz	x8, 0x1005388ac <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d15990672b98a7aE+0x23c>
10053889c: f9400be0    	ldr	x0, [sp, #0x10]
1005388a0: d37ced01    	lsl	x1, x8, #4
1005388a4: 52800102    	mov	w2, #0x8                ; =8
1005388a8: 97eb2330    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005388ac: aa1303e0    	mov	x0, x19
1005388b0: 94326229    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001005388b4 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E>:
1005388b4: d10343ff    	sub	sp, sp, #0xd0
1005388b8: a90867fa    	stp	x26, x25, [sp, #0x80]
1005388bc: a9095ff8    	stp	x24, x23, [sp, #0x90]
1005388c0: a90a57f6    	stp	x22, x21, [sp, #0xa0]
1005388c4: a90b4ff4    	stp	x20, x19, [sp, #0xb0]
1005388c8: a90c7bfd    	stp	x29, x30, [sp, #0xc0]
1005388cc: 910303fd    	add	x29, sp, #0xc0
1005388d0: aa0803f3    	mov	x19, x8
1005388d4: 39410408    	ldrb	w8, [x0, #0x41]
1005388d8: 36000168    	tbz	w8, #0x0, 0x100538904 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0x50>
1005388dc: 52800108    	mov	w8, #0x8                ; =8
1005388e0: a900227f    	stp	xzr, x8, [x19]
1005388e4: f9000a7f    	str	xzr, [x19, #0x10]
1005388e8: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
1005388ec: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
1005388f0: a94a57f6    	ldp	x22, x21, [sp, #0xa0]
1005388f4: a9495ff8    	ldp	x24, x23, [sp, #0x90]
1005388f8: a94867fa    	ldp	x26, x25, [sp, #0x80]
1005388fc: 910343ff    	add	sp, sp, #0xd0
100538900: d65f03c0    	ret
100538904: aa0003f5    	mov	x21, x0
100538908: aa0003f4    	mov	x20, x0
10053890c: f8410e96    	ldr	x22, [x20, #0x10]!
100538910: 910083e0    	add	x0, sp, #0x20
100538914: aa1403e1    	mov	x1, x20
100538918: 9400b2d4    	bl	0x100565468 <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
10053891c: b94023e8    	ldr	w8, [sp, #0x20]
100538920: 7100051f    	cmp	w8, #0x1
100538924: 54000261    	b.ne	0x100538970 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0xbc>
100538928: f9401be8    	ldr	x8, [sp, #0x30]
10053892c: f94002a9    	ldr	x9, [x21]
100538930: cb090117    	sub	x23, x8, x9
100538934: 8b0902d6    	add	x22, x22, x9
100538938: f90002a8    	str	x8, [x21]
10053893c: b4000397    	cbz	x23, 0x1005389ac <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0xf8>
100538940: 8b1702c9    	add	x9, x22, x23
100538944: 385ff128    	ldurb	w8, [x9, #-0x1]
100538948: 7100291f    	cmp	w8, #0xa
10053894c: 54000301    	b.ne	0x1005389ac <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0xf8>
100538950: f10006e8    	subs	x8, x23, #0x1
100538954: 540002a0    	b.eq	0x1005389a8 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0xf4>
100538958: 385fe129    	ldurb	w9, [x9, #-0x2]
10053895c: d1000aea    	sub	x10, x23, #0x2
100538960: 7100353f    	cmp	w9, #0xd
100538964: 9a9612d6    	csel	x22, x22, x22, ne
100538968: 9a8a1117    	csel	x23, x8, x10, ne
10053896c: 14000010    	b	0x1005389ac <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0xf8>
100538970: 394106a8    	ldrb	w8, [x21, #0x41]
100538974: 3707fb48    	tbnz	w8, #0x0, 0x1005388dc <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0x28>
100538978: 52800028    	mov	w8, #0x1                ; =1
10053897c: 390106a8    	strb	w8, [x21, #0x41]
100538980: 394102aa    	ldrb	w10, [x21, #0x40]
100538984: a94026a8    	ldp	x8, x9, [x21]
100538988: 3700006a    	tbnz	w10, #0x0, 0x100538994 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0xe0>
10053898c: eb08013f    	cmp	x9, x8
100538990: 54fffa60    	b.eq	0x1005388dc <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0x28>
100538994: f940028a    	ldr	x10, [x20]
100538998: cb080137    	sub	x23, x9, x8
10053899c: 8b080156    	add	x22, x10, x8
1005389a0: b5fffd17    	cbnz	x23, 0x100538940 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0x8c>
1005389a4: 14000002    	b	0x1005389ac <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0xf8>
1005389a8: aa0803f7    	mov	x23, x8
1005389ac: b4fff996    	cbz	x22, 0x1005388dc <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0x28>
1005389b0: 97eb22f1    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1005389b4: 52800800    	mov	w0, #0x40               ; =64
1005389b8: 52800101    	mov	w1, #0x8                ; =8
1005389bc: 97eb22ea    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1005389c0: b4000a60    	cbz	x0, 0x100538b0c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0x258>
1005389c4: aa0003f4    	mov	x20, x0
1005389c8: a9005c16    	stp	x22, x23, [x0]
1005389cc: 52800088    	mov	w8, #0x4                ; =4
1005389d0: a90083e8    	stp	x8, x0, [sp, #0x8]
1005389d4: 52800036    	mov	w22, #0x1               ; =1
1005389d8: f9000ff6    	str	x22, [sp, #0x18]
1005389dc: f94022a8    	ldr	x8, [x21, #0x40]
1005389e0: f90033e8    	str	x8, [sp, #0x60]
1005389e4: ad4106a0    	ldp	q0, q1, [x21, #0x20]
1005389e8: ad0207e0    	stp	q0, q1, [sp, #0x40]
1005389ec: ad4002a1    	ldp	q1, q0, [x21]
1005389f0: ad0103e1    	stp	q1, q0, [sp, #0x20]
1005389f4: d3483d08    	ubfx	x8, x8, #8, #8
1005389f8: 360000c8    	tbz	w8, #0x0, 0x100538a10 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0x15c>
1005389fc: 3cc083e0    	ldur	q0, [sp, #0x8]
100538a00: 3d800260    	str	q0, [x19]
100538a04: f9400fe8    	ldr	x8, [sp, #0x18]
100538a08: f9000a68    	str	x8, [x19, #0x10]
100538a0c: 17ffffb7    	b	0x1005388e8 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0x34>
100538a10: d2800017    	mov	x23, #0x0               ; =0
100538a14: 910083f8    	add	x24, sp, #0x20
100538a18: 52800035    	mov	w21, #0x1               ; =1
100538a1c: 14000009    	b	0x100538a40 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h9d36c31263ffcb25E+0x18c>
