
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100773820 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E>:
100773820: d102c3ff    	sub	sp, sp, #0xb0
100773824: a90667fa    	stp	x26, x25, [sp, #0x60]
100773828: a9075ff8    	stp	x24, x23, [sp, #0x70]
10077382c: a90857f6    	stp	x22, x21, [sp, #0x80]
100773830: a9094ff4    	stp	x20, x19, [sp, #0x90]
100773834: a90a7bfd    	stp	x29, x30, [sp, #0xa0]
100773838: 910283fd    	add	x29, sp, #0xa0
10077383c: aa0003f4    	mov	x20, x0
100773840: aa0803f5    	mov	x21, x8
100773844: a940d813    	ldp	x19, x22, [x0, #0x8]
100773848: 910063e8    	add	x8, sp, #0x18
10077384c: aa1303e0    	mov	x0, x19
100773850: aa1603e1    	mov	x1, x22
100773854: 9423ca68    	bl	0x1010661f4 <__ZN4uuid6parser28_$LT$impl$u20$uuid..Uuid$GT$9parse_str17haf91ace8864407ffE>
100773858: b9401be8    	ldr	w8, [sp, #0x18]
10077385c: 7100251f    	cmp	w8, #0x9
100773860: 540005a1    	b.ne	0x100773914 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0xf4>
100773864: b841f3e8    	ldur	w8, [sp, #0x1f]
100773868: b803b3e8    	stur	w8, [sp, #0x3b]
10077386c: b9401fe8    	ldr	w8, [sp, #0x1c]
100773870: b9003be8    	str	w8, [sp, #0x38]
100773874: f84233f9    	ldur	x25, [sp, #0x23]
100773878: 3940affa    	ldrb	w26, [sp, #0x2b]
10077387c: b843b3e8    	ldur	w8, [sp, #0x3b]
100773880: b800b3e8    	stur	w8, [sp, #0xb]
100773884: b9403be8    	ldr	w8, [sp, #0x38]
100773888: b9000be8    	str	w8, [sp, #0x8]
10077388c: f800f3f9    	stur	x25, [sp, #0xf]
100773890: 39005ffa    	strb	w26, [sp, #0x17]
100773894: 39403be8    	ldrb	w8, [sp, #0xe]
100773898: 121c0d08    	and	w8, w8, #0xf0
10077389c: 7101011f    	cmp	w8, #0x40
1007738a0: 540003a1    	b.ne	0x100773914 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0xf4>
1007738a4: 52800028    	mov	w8, #0x1                ; =1
1007738a8: a90423ff    	stp	xzr, x8, [sp, #0x40]
1007738ac: f9002bff    	str	xzr, [sp, #0x50]
1007738b0: 52800408    	mov	w8, #0x20               ; =32
1007738b4: 72ac0008    	movk	w8, #0x6000, lsl #16
1007738b8: f90017e8    	str	x8, [sp, #0x28]
1007738bc: 910103e8    	add	x8, sp, #0x40
1007738c0: 900085c9    	adrp	x9, 0x10182b000 <_anon.6408540cdffbe318d49f52812e99783f.784+0x28>
1007738c4: 9133e129    	add	x9, x9, #0xcf8
1007738c8: a901a7e8    	stp	x8, x9, [sp, #0x18]
1007738cc: 910023e0    	add	x0, sp, #0x8
1007738d0: 910063e1    	add	x1, sp, #0x18
1007738d4: 9423c970    	bl	0x101065e94 <__ZN4uuid3fmt60_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$uuid..Uuid$GT$3fmt17he33b5b9ccee1ccdbE>
1007738d8: 35000620    	cbnz	w0, 0x10077399c <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0x17c>
1007738dc: a94463f7    	ldp	x23, x24, [sp, #0x40]
1007738e0: f9402be8    	ldr	x8, [sp, #0x50]
1007738e4: eb16011f    	cmp	x8, x22
1007738e8: 540000c1    	b.ne	0x100773900 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0xe0>
1007738ec: aa1803e0    	mov	x0, x24
1007738f0: aa1303e1    	mov	x1, x19
1007738f4: aa1603e2    	mov	x2, x22
1007738f8: 9429776a    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1007738fc: 34000300    	cbz	w0, 0x10077395c <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0x13c>
100773900: b40000b7    	cbz	x23, 0x100773914 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0xf4>
100773904: aa1803e0    	mov	x0, x24
100773908: aa1703e1    	mov	x1, x23
10077390c: 52800022    	mov	w2, #0x1                ; =1
100773910: 97e23716    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100773914: d00061a8    	adrp	x8, 0x1013a9000 <_anon.cdb272158a04aae91ce02c6e47f106c7.539+0xd4>
100773918: 910b0108    	add	x8, x8, #0x2c0
10077391c: 528005e9    	mov	w9, #0x2f               ; =47
100773920: a900a6a8    	stp	x8, x9, [x21, #0x8]
100773924: 52800028    	mov	w8, #0x1                ; =1
100773928: 390002a8    	strb	w8, [x21]
10077392c: f9400281    	ldr	x1, [x20]
100773930: b4000081    	cbz	x1, 0x100773940 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0x120>
100773934: aa1303e0    	mov	x0, x19
100773938: 52800022    	mov	w2, #0x1                ; =1
10077393c: 97e2370b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100773940: a94a7bfd    	ldp	x29, x30, [sp, #0xa0]
100773944: a9494ff4    	ldp	x20, x19, [sp, #0x90]
100773948: a94857f6    	ldp	x22, x21, [sp, #0x80]
10077394c: a9475ff8    	ldp	x24, x23, [sp, #0x70]
100773950: a94667fa    	ldp	x26, x25, [sp, #0x60]
100773954: 9102c3ff    	add	sp, sp, #0xb0
100773958: d65f03c0    	ret
10077395c: b40000b7    	cbz	x23, 0x100773970 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0x150>
100773960: aa1803e0    	mov	x0, x24
100773964: aa1703e1    	mov	x1, x23
100773968: 52800022    	mov	w2, #0x1                ; =1
10077396c: 97e236ff    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100773970: 52800008    	mov	w8, #0x0                ; =0
100773974: b9403be9    	ldr	w9, [sp, #0x38]
100773978: b80012a9    	stur	w9, [x21, #0x1]
10077397c: b843b3e9    	ldur	w9, [sp, #0x3b]
100773980: b90006a9    	str	w9, [x21, #0x4]
100773984: f90006b9    	str	x25, [x21, #0x8]
100773988: 390042ba    	strb	w26, [x21, #0x10]
10077398c: 390002a8    	strb	w8, [x21]
100773990: f9400281    	ldr	x1, [x20]
100773994: b5fffd01    	cbnz	x1, 0x100773934 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0x114>
100773998: 17ffffea    	b	0x100773940 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0x120>
10077399c: f00061c0    	adrp	x0, 0x1013ae000 <_anon.6408540cdffbe318d49f52812e99783f.1450+0xb0>
1007739a0: 91173400    	add	x0, x0, #0x5cd
1007739a4: 900085c3    	adrp	x3, 0x10182b000 <_anon.6408540cdffbe318d49f52812e99783f.784+0x28>
1007739a8: 91382063    	add	x3, x3, #0xe08
1007739ac: 900085c4    	adrp	x4, 0x10182b000 <_anon.6408540cdffbe318d49f52812e99783f.784+0x28>
1007739b0: 9134a084    	add	x4, x4, #0xd28
1007739b4: d10107a2    	sub	x2, x29, #0x41
1007739b8: 528006e1    	mov	w1, #0x37               ; =55
1007739bc: 9429740f    	bl	0x1011d09f8 <__ZN4core6result13unwrap_failed17hfd1628134417d023E>
1007739c0: d4200020    	brk	#0x1
1007739c4: aa0003f5    	mov	x21, x0
1007739c8: 14000007    	b	0x1007739e4 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0x1c4>
1007739cc: aa0003f5    	mov	x21, x0
1007739d0: f94023e1    	ldr	x1, [sp, #0x40]
1007739d4: b4000081    	cbz	x1, 0x1007739e4 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0x1c4>
1007739d8: f94027e0    	ldr	x0, [sp, #0x48]
1007739dc: 52800022    	mov	w2, #0x1                ; =1
1007739e0: 97e236e2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007739e4: f9400281    	ldr	x1, [x20]
1007739e8: b4000081    	cbz	x1, 0x1007739f8 <__ZN125_$LT$codexmate_lib..core..relay..models..RouterOperationId$u20$as$u20$core..convert..TryFrom$LT$alloc..string..String$GT$$GT$8try_from17h4a5af1ab4281f3a3E+0x1d8>
1007739ec: aa1303e0    	mov	x0, x19
1007739f0: 52800022    	mov	w2, #0x1                ; =1
1007739f4: 97e236dd    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007739f8: aa1503e0    	mov	x0, x21
1007739fc: 942975d6    	bl	0x1011d1154 <_writev+0x1011d1154>
