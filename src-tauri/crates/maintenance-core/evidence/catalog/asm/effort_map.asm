
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001005308f4 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h277eacb57b97a28bE>:
1005308f4: d101c3ff    	sub	sp, sp, #0x70
1005308f8: a9035ff8    	stp	x24, x23, [sp, #0x30]
1005308fc: a90457f6    	stp	x22, x21, [sp, #0x40]
100530900: a9054ff4    	stp	x20, x19, [sp, #0x50]
100530904: a9067bfd    	stp	x29, x30, [sp, #0x60]
100530908: 910183fd    	add	x29, sp, #0x60
10053090c: aa0803f3    	mov	x19, x8
100530910: d2800014    	mov	x20, #0x0               ; =0
100530914: b27cebe8    	mov	x8, #0x7ffffffffffffff0 ; =9223372036854775792
100530918: cb000036    	sub	x22, x1, x0
10053091c: eb0802df    	cmp	x22, x8
100530920: d37ffad5    	lsl	x21, x22, #1
100530924: 54000248    	b.hi	0x10053096c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h277eacb57b97a28bE+0x78>
100530928: b27defe8    	mov	x8, #0x7ffffffffffffff8 ; =9223372036854775800
10053092c: eb0802bf    	cmp	x21, x8
100530930: 540001e8    	b.hi	0x10053096c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h277eacb57b97a28bE+0x78>
100530934: b4000235    	cbz	x21, 0x100530978 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h277eacb57b97a28bE+0x84>
100530938: aa0103f8    	mov	x24, x1
10053093c: aa0003f7    	mov	x23, x0
100530940: 97eb430d    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100530944: 52800114    	mov	w20, #0x8               ; =8
100530948: aa1503e0    	mov	x0, x21
10053094c: 52800101    	mov	w1, #0x8                ; =8
100530950: 97eb4305    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100530954: aa0003e8    	mov	x8, x0
100530958: b40000a0    	cbz	x0, 0x10053096c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h277eacb57b97a28bE+0x78>
10053095c: aa1803e1    	mov	x1, x24
100530960: aa1703e0    	mov	x0, x23
100530964: d344fec9    	lsr	x9, x22, #4
100530968: 14000006    	b	0x100530980 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h277eacb57b97a28bE+0x8c>
10053096c: aa1403e0    	mov	x0, x20
100530970: aa1503e1    	mov	x1, x21
100530974: 94327f3c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100530978: d2800009    	mov	x9, #0x0                ; =0
10053097c: 52800108    	mov	w8, #0x8                ; =8
100530980: a90023e9    	stp	x9, x8, [sp]
100530984: 910003e9    	mov	x9, sp
100530988: 91004129    	add	x9, x9, #0x10
10053098c: a90127ff    	stp	xzr, x9, [sp, #0x10]
100530990: a90223ff    	stp	xzr, x8, [sp, #0x20]
100530994: 910063e2    	add	x2, sp, #0x18
100530998: 940b531c    	bl	0x100805608 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE>
10053099c: 3dc003e0    	ldr	q0, [sp]
1005309a0: 3d800260    	str	q0, [x19]
1005309a4: f9400be8    	ldr	x8, [sp, #0x10]
1005309a8: f9000a68    	str	x8, [x19, #0x10]
1005309ac: a9467bfd    	ldp	x29, x30, [sp, #0x60]
1005309b0: a9454ff4    	ldp	x20, x19, [sp, #0x50]
1005309b4: a94457f6    	ldp	x22, x21, [sp, #0x40]
1005309b8: a9435ff8    	ldp	x24, x23, [sp, #0x30]
1005309bc: 9101c3ff    	add	sp, sp, #0x70
1005309c0: d65f03c0    	ret
1005309c4: aa0003f3    	mov	x19, x0
1005309c8: 910003e0    	mov	x0, sp
1005309cc: 940091a3    	bl	0x100555058 <__ZN4core3ptr68drop_in_place$LT$alloc..vec..Vec$LT$serde_json..value..Value$GT$$GT$17h50bef9f86c8df221E>
1005309d0: aa1303e0    	mov	x0, x19
1005309d4: 943281e0    	bl	0x1011d1154 <_writev+0x1011d1154>
1005309d8: 9432805c    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

00000001005309dc <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E>:
1005309dc: d10343ff    	sub	sp, sp, #0xd0
1005309e0: a9076ffc    	stp	x28, x27, [sp, #0x70]
1005309e4: a90867fa    	stp	x26, x25, [sp, #0x80]
1005309e8: a9095ff8    	stp	x24, x23, [sp, #0x90]
1005309ec: a90a57f6    	stp	x22, x21, [sp, #0xa0]
1005309f0: a90b4ff4    	stp	x20, x19, [sp, #0xb0]
1005309f4: a90c7bfd    	stp	x29, x30, [sp, #0xc0]
1005309f8: 910303fd    	add	x29, sp, #0xc0
1005309fc: aa0803f3    	mov	x19, x8
100530a00: 39403008    	ldrb	w8, [x0, #0xc]
100530a04: 37000368    	tbnz	w8, #0x0, 0x100530a70 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0x94>
100530a08: aa0003f5    	mov	x21, x0
100530a0c: 91002000    	add	x0, x0, #0x8
100530a10: 9427826a    	bl	0x100f113b8 <__ZN101_$LT$chrono..naive..date..NaiveDateDaysIterator$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hc90aa866b7b1b069E>
100530a14: 340002e0    	cbz	w0, 0x100530a70 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0x94>
100530a18: f94002a8    	ldr	x8, [x21]
100530a1c: b9400108    	ldr	w8, [x8]
100530a20: 6b08001f    	cmp	w0, w8
100530a24: 5400026c    	b.gt	0x100530a70 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0x94>
100530a28: 52800418    	mov	w24, #0x20              ; =32
100530a2c: 72ac0018    	movk	w24, #0x6000, lsl #16
100530a30: b9000be0    	str	w0, [sp, #0x8]
100530a34: 52800028    	mov	w8, #0x1                ; =1
100530a38: a903a3ff    	stp	xzr, x8, [sp, #0x38]
100530a3c: 9100e3e8    	add	x8, sp, #0x38
100530a40: f0009779    	adrp	x25, 0x10181f000 <_anon.8fa53a009f7cd8c90f3f229cc309c2bf.22+0x40>
100530a44: 9104e339    	add	x25, x25, #0x138
100530a48: a904a3ff    	stp	xzr, x8, [sp, #0x48]
100530a4c: a905e3f9    	stp	x25, x24, [sp, #0x58]
100530a50: 910023e0    	add	x0, sp, #0x8
100530a54: 910143e1    	add	x1, sp, #0x50
100530a58: 94278317    	bl	0x100f116b4 <__ZN69_$LT$chrono..naive..date..NaiveDate$u20$as$u20$core..fmt..Display$GT$3fmt17h143dae50adbd07faE>
100530a5c: 35000d00    	cbnz	w0, 0x100530bfc <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0x220>
100530a60: f9401ff6    	ldr	x22, [sp, #0x38]
100530a64: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100530a68: eb0802df    	cmp	x22, x8
100530a6c: 54000181    	b.ne	0x100530a9c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0xc0>
100530a70: 52800108    	mov	w8, #0x8                ; =8
100530a74: a900227f    	stp	xzr, x8, [x19]
100530a78: f9000a7f    	str	xzr, [x19, #0x10]
100530a7c: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
100530a80: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
100530a84: a94a57f6    	ldp	x22, x21, [sp, #0xa0]
100530a88: a9495ff8    	ldp	x24, x23, [sp, #0x90]
100530a8c: a94867fa    	ldp	x26, x25, [sp, #0x80]
100530a90: a9476ffc    	ldp	x28, x27, [sp, #0x70]
100530a94: 910343ff    	add	sp, sp, #0xd0
100530a98: d65f03c0    	ret
100530a9c: a9446bf7    	ldp	x23, x26, [sp, #0x40]
100530aa0: 394032a8    	ldrb	w8, [x21, #0xc]
100530aa4: 37000088    	tbnz	w8, #0x0, 0x100530ab4 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0xd8>
100530aa8: 910143e8    	add	x8, sp, #0x50
100530aac: 910022a0    	add	x0, x21, #0x8
100530ab0: 94278267    	bl	0x100f1144c <__ZN101_$LT$chrono..naive..date..NaiveDateDaysIterator$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h049045f08098f09bE>
100530ab4: 97eb42b0    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100530ab8: 52800c00    	mov	w0, #0x60               ; =96
100530abc: 52800101    	mov	w1, #0x8                ; =8
100530ac0: 97eb42a9    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100530ac4: b4000b00    	cbz	x0, 0x100530c24 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0x248>
100530ac8: aa0003f4    	mov	x20, x0
100530acc: a9005c16    	stp	x22, x23, [x0]
100530ad0: f900081a    	str	x26, [x0, #0x10]
100530ad4: 52800088    	mov	w8, #0x4                ; =4
100530ad8: a90083e8    	stp	x8, x0, [sp, #0x8]
100530adc: 52800028    	mov	w8, #0x1                ; =1
100530ae0: f9000fe8    	str	x8, [sp, #0x18]
100530ae4: 3dc002a0    	ldr	q0, [x21]
100530ae8: 3d800be0    	str	q0, [sp, #0x20]
100530aec: 3940b3e8    	ldrb	w8, [sp, #0x2c]
100530af0: 360000c8    	tbz	w8, #0x0, 0x100530b08 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0x12c>
100530af4: 3cc083e0    	ldur	q0, [sp, #0x8]
100530af8: 3d800260    	str	q0, [x19]
100530afc: f9400fe8    	ldr	x8, [sp, #0x18]
100530b00: f9000a68    	str	x8, [x19, #0x10]
100530b04: 17ffffde    	b	0x100530a7c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0xa0>
100530b08: d280001a    	mov	x26, #0x0               ; =0
100530b0c: 910083fb    	add	x27, sp, #0x20
100530b10: 52800035    	mov	w21, #0x1               ; =1
100530b14: 14000009    	b	0x100530b38 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h291c256ab8bb9089E+0x15c>
100530b18: 8b1a0288    	add	x8, x20, x26
100530b1c: a901dd16    	stp	x22, x23, [x8, #0x18]
100530b20: f900151c    	str	x28, [x8, #0x28]
100530b24: 910006b5    	add	x21, x21, #0x1
100530b28: f9000ff5    	str	x21, [sp, #0x18]
100530b2c: 3940b3e8    	ldrb	w8, [sp, #0x2c]
