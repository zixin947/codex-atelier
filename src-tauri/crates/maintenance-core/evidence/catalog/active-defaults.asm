
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010047c99c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E>:
10047c99c: d104c3ff    	sub	sp, sp, #0x130
10047c9a0: a90d6ffc    	stp	x28, x27, [sp, #0xd0]
10047c9a4: a90e67fa    	stp	x26, x25, [sp, #0xe0]
10047c9a8: a90f5ff8    	stp	x24, x23, [sp, #0xf0]
10047c9ac: a91057f6    	stp	x22, x21, [sp, #0x100]
10047c9b0: a9114ff4    	stp	x20, x19, [sp, #0x110]
10047c9b4: a9127bfd    	stp	x29, x30, [sp, #0x120]
10047c9b8: 910483fd    	add	x29, sp, #0x120
10047c9bc: aa0803f3    	mov	x19, x8
10047c9c0: a940d409    	ldp	x9, x21, [x0, #0x8]
10047c9c4: f9400008    	ldr	x8, [x0]
10047c9c8: f100011f    	cmp	x8, #0x0
10047c9cc: 9a9f12aa    	csel	x10, x21, xzr, ne
10047c9d0: 1a9f07eb    	cset	w11, ne
10047c9d4: a900ffeb    	stp	x11, xzr, [sp, #0x8]
10047c9d8: a901a7e8    	stp	x8, x9, [sp, #0x18]
10047c9dc: a902ffeb    	stp	x11, xzr, [sp, #0x28]
10047c9e0: a903a7e8    	stp	x8, x9, [sp, #0x38]
10047c9e4: f90027ea    	str	x10, [sp, #0x48]
10047c9e8: 910023f6    	add	x22, sp, #0x8
10047c9ec: 528000d9    	mov	w25, #0x6               ; =6
10047c9f0: 390143f9    	strb	w25, [sp, #0x50]
10047c9f4: d101c3a8    	sub	x8, x29, #0x70
10047c9f8: b240011a    	orr	x26, x8, #0x1
10047c9fc: d2f0001b    	mov	x27, #-0x8000000000000000 ; =-9223372036854775808
10047ca00: d2f00017    	mov	x23, #-0x8000000000000000 ; =-9223372036854775808
10047ca04: d101c3a8    	sub	x8, x29, #0x70
10047ca08: 910023e0    	add	x0, sp, #0x8
10047ca0c: 97fb6697    	bl	0x100356468 <__ZN84_$LT$serde_json..value..de..MapDeserializer$u20$as$u20$serde_core..de..MapAccess$GT$13next_key_seed17hbb5a8af949b9fe2cE>
10047ca10: 385903a8    	ldurb	w8, [x29, #-0x70]
10047ca14: 7100051f    	cmp	w8, #0x1
10047ca18: 54000360    	b.eq	0x10047ca84 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0xe8>
10047ca1c: 385913a8    	ldurb	w8, [x29, #-0x6f]
10047ca20: 7100091f    	cmp	w8, #0x2
10047ca24: 54000380    	b.eq	0x10047ca94 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0xf8>
10047ca28: 360001a8    	tbz	w8, #0x0, 0x10047ca5c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0xc0>
10047ca2c: 394143e8    	ldrb	w8, [sp, #0x50]
10047ca30: 390143f9    	strb	w25, [sp, #0x50]
10047ca34: 7100191f    	cmp	w8, #0x6
10047ca38: 54000400    	b.eq	0x10047cab8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x11c>
10047ca3c: 3cc492c0    	ldur	q0, [x22, #0x49]
10047ca40: 3d800340    	str	q0, [x26]
10047ca44: 3cc582c0    	ldur	q0, [x22, #0x58]
10047ca48: 3c80f340    	stur	q0, [x26, #0xf]
10047ca4c: 381903a8    	sturb	w8, [x29, #-0x70]
10047ca50: d101c3a0    	sub	x0, x29, #0x70
10047ca54: 9413bd13    	bl	0x10096bea0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10047ca58: 17ffffeb    	b	0x10047ca04 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x68>
10047ca5c: eb1b02ff    	cmp	x23, x27
10047ca60: 540007e1    	b.ne	0x10047cb5c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x1c0>
10047ca64: d101c3a8    	sub	x8, x29, #0x70
10047ca68: 910023e0    	add	x0, sp, #0x8
10047ca6c: 97fb6bc5    	bl	0x100357980 <__ZN84_$LT$serde_json..value..de..MapDeserializer$u20$as$u20$serde_core..de..MapAccess$GT$15next_value_seed17hba99c432fe1ce0b5E>
10047ca70: a97953b7    	ldp	x23, x20, [x29, #-0x70]
10047ca74: eb1b02ff    	cmp	x23, x27
10047ca78: 540006c0    	b.eq	0x10047cb50 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x1b4>
10047ca7c: f85a03b8    	ldur	x24, [x29, #-0x60]
10047ca80: 17ffffe1    	b	0x10047ca04 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x68>
10047ca84: a908d3f7    	stp	x23, x20, [sp, #0x88]
10047ca88: f9004ff8    	str	x24, [sp, #0x98]
10047ca8c: f85983a0    	ldur	x0, [x29, #-0x68]
10047ca90: 14000010    	b	0x10047cad0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x134>
10047ca94: a908d3f7    	stp	x23, x20, [sp, #0x88]
10047ca98: f9004ff8    	str	x24, [sp, #0x98]
10047ca9c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10047caa0: eb0802ff    	cmp	x23, x8
10047caa4: 54000361    	b.ne	0x10047cb10 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x174>
10047caa8: 52800108    	mov	w8, #0x8                ; =8
10047caac: a900227f    	stp	xzr, x8, [x19]
10047cab0: f9000a7f    	str	xzr, [x19, #0x10]
10047cab4: 1400001e    	b	0x10047cb2c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x190>
10047cab8: a908d3f7    	stp	x23, x20, [sp, #0x88]
10047cabc: f9004ff8    	str	x24, [sp, #0x98]
10047cac0: d0007460    	adrp	x0, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
10047cac4: 9127bc00    	add	x0, x0, #0x9ef
10047cac8: 52800201    	mov	w1, #0x10               ; =16
10047cacc: 9434405a    	bl	0x10118cc34 <__ZN67_$LT$serde_json..error..Error$u20$as$u20$serde_core..ser..Error$GT$6custom17hd308ce5bb9ca103cE>
10047cad0: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10047cad4: a9000268    	stp	x8, x0, [x19]
10047cad8: eb0802ff    	cmp	x23, x8
10047cadc: 540006e0    	b.eq	0x10047cbb8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x21c>
10047cae0: b4000618    	cbz	x24, 0x10047cba0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x204>
10047cae4: 91002293    	add	x19, x20, #0x8
10047cae8: 14000004    	b	0x10047caf8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x15c>
10047caec: 91006273    	add	x19, x19, #0x18
10047caf0: f1000718    	subs	x24, x24, #0x1
10047caf4: 54000560    	b.eq	0x10047cba0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x204>
10047caf8: f85f8261    	ldur	x1, [x19, #-0x8]
10047cafc: b4ffff81    	cbz	x1, 0x10047caec <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x150>
10047cb00: f9400260    	ldr	x0, [x19]
10047cb04: 52800022    	mov	w2, #0x1                ; =1
10047cb08: 97ee1298    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10047cb0c: 17fffff8    	b	0x10047caec <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x150>
10047cb10: 3cc883e0    	ldur	q0, [sp, #0x88]
10047cb14: 3d800260    	str	q0, [x19]
10047cb18: f9404fe9    	ldr	x9, [sp, #0x98]
10047cb1c: f9000a69    	str	x9, [x19, #0x10]
10047cb20: f9400269    	ldr	x9, [x19]
10047cb24: eb08013f    	cmp	x9, x8
10047cb28: 54000480    	b.eq	0x10047cbb8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x21c>
10047cb2c: 3dc00260    	ldr	q0, [x19]
10047cb30: 3d801fe0    	str	q0, [sp, #0x70]
10047cb34: f9400a68    	ldr	x8, [x19, #0x10]
10047cb38: f90043e8    	str	x8, [sp, #0x80]
10047cb3c: f94027e8    	ldr	x8, [sp, #0x48]
10047cb40: b50005a8    	cbnz	x8, 0x10047cbf4 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x258>
10047cb44: 910023e0    	add	x0, sp, #0x8
10047cb48: 940bd2a0    	bl	0x1007715c8 <__ZN99_$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5ef754463675f4dcE>
10047cb4c: 1400001d    	b	0x10047cbc0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x224>
10047cb50: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10047cb54: a9005268    	stp	x8, x20, [x19]
10047cb58: 14000018    	b	0x10047cbb8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x21c>
10047cb5c: a908d3f7    	stp	x23, x20, [sp, #0x88]
10047cb60: f9004ff8    	str	x24, [sp, #0x98]
10047cb64: b00074e8    	adrp	x8, 0x101319000 <_anon.b5f536351c4bf1e454cf90283dd9e625.167+0x8>
10047cb68: 91292908    	add	x8, x8, #0xa4a
10047cb6c: 528000a9    	mov	w9, #0x5                ; =5
10047cb70: a93827a8    	stp	x8, x9, [x29, #-0x80]
10047cb74: d10203a8    	sub	x8, x29, #0x80
10047cb78: b0001b09    	adrp	x9, 0x1007dd000 <__ZN3zip9zipcrypto24ZipCryptoReader$LT$R$GT$8validate17h6abfe2f5c5f69c24E+0xf0>
10047cb7c: 911b0129    	add	x9, x9, #0x6c0
10047cb80: a93927a8    	stp	x8, x9, [x29, #-0x70]
10047cb84: b0008540    	adrp	x0, 0x101525000 <_anon.a9e2253c03df711f51070ce6102367ea.957+0x57>
10047cb88: 91176800    	add	x0, x0, #0x5da
10047cb8c: d101c3a1    	sub	x1, x29, #0x70
10047cb90: 94344002    	bl	0x10118cb98 <__ZN66_$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$6custom17h984fbb9098575a39E>
10047cb94: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10047cb98: a9000268    	stp	x8, x0, [x19]
10047cb9c: b5fffa58    	cbnz	x24, 0x10047cae4 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x148>
10047cba0: b40000d7    	cbz	x23, 0x10047cbb8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x21c>
10047cba4: 8b1706e8    	add	x8, x23, x23, lsl #1
10047cba8: d37df101    	lsl	x1, x8, #3
10047cbac: aa1403e0    	mov	x0, x20
10047cbb0: 52800102    	mov	w2, #0x8                ; =8
10047cbb4: 97ee126d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10047cbb8: 910023e0    	add	x0, sp, #0x8
10047cbbc: 940bd283    	bl	0x1007715c8 <__ZN99_$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5ef754463675f4dcE>
10047cbc0: 394143e8    	ldrb	w8, [sp, #0x50]
10047cbc4: 7100191f    	cmp	w8, #0x6
10047cbc8: 54000060    	b.eq	0x10047cbd4 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x238>
10047cbcc: 910122c0    	add	x0, x22, #0x48
10047cbd0: 940140e0    	bl	0x1004ccf50 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10047cbd4: a9527bfd    	ldp	x29, x30, [sp, #0x120]
10047cbd8: a9514ff4    	ldp	x20, x19, [sp, #0x110]
10047cbdc: a95057f6    	ldp	x22, x21, [sp, #0x100]
10047cbe0: a94f5ff8    	ldp	x24, x23, [sp, #0xf0]
10047cbe4: a94e67fa    	ldp	x26, x25, [sp, #0xe0]
10047cbe8: a94d6ffc    	ldp	x28, x27, [sp, #0xd0]
10047cbec: 9104c3ff    	add	sp, sp, #0x130
10047cbf0: d65f03c0    	ret
10047cbf4: b0009d01    	adrp	x1, 0x10181d000 <_anon.a9e2253c03df711f51070ce6102367ea.995+0x1f8>
10047cbf8: 91354021    	add	x1, x1, #0xd50
10047cbfc: b0009d02    	adrp	x2, 0x10181d000 <_anon.a9e2253c03df711f51070ce6102367ea.995+0x1f8>
10047cc00: 91358042    	add	x2, x2, #0xd60
10047cc04: aa1503e0    	mov	x0, x21
10047cc08: 94343e9b    	bl	0x10118c674 <__ZN10serde_core2de5Error14invalid_length17h47103574f7b55d58E>
10047cc0c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10047cc10: a9000268    	stp	x8, x0, [x19]
10047cc14: 9101c3e0    	add	x0, sp, #0x70
10047cc18: 94015076    	bl	0x1004d0df0 <__ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h265e755c5c638973E>
10047cc1c: 17ffffca    	b	0x10047cb44 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x1a8>
10047cc20: aa0003f5    	mov	x21, x0
10047cc24: 1400001f    	b	0x10047cca0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x304>
10047cc28: aa0003f5    	mov	x21, x0
10047cc2c: 1400001c    	b	0x10047cc9c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x300>
10047cc30: aa0003f5    	mov	x21, x0
10047cc34: 394143e8    	ldrb	w8, [sp, #0x50]
10047cc38: 7100191f    	cmp	w8, #0x6
10047cc3c: 540003a0    	b.eq	0x10047ccb0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x314>
10047cc40: 910122c0    	add	x0, x22, #0x48
10047cc44: 940140c3    	bl	0x1004ccf50 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10047cc48: 1400001a    	b	0x10047ccb0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x314>
10047cc4c: 94354fbf    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
10047cc50: aa0003f5    	mov	x21, x0
10047cc54: 1400000f    	b	0x10047cc90 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x2f4>
10047cc58: aa0003f5    	mov	x21, x0
10047cc5c: d2f00017    	mov	x23, #-0x8000000000000000 ; =-9223372036854775808
10047cc60: 1400000a    	b	0x10047cc88 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x2ec>
10047cc64: aa0003f5    	mov	x21, x0
10047cc68: 394143e8    	ldrb	w8, [sp, #0x50]
10047cc6c: 7100191f    	cmp	w8, #0x6
10047cc70: 54000200    	b.eq	0x10047ccb0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x314>
10047cc74: 910122c0    	add	x0, x22, #0x48
10047cc78: 940140b6    	bl	0x1004ccf50 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10047cc7c: 1400000d    	b	0x10047ccb0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x314>
10047cc80: 94354fb2    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
10047cc84: aa0003f5    	mov	x21, x0
10047cc88: a908d3f7    	stp	x23, x20, [sp, #0x88]
10047cc8c: f9004ff8    	str	x24, [sp, #0x98]
10047cc90: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10047cc94: eb0802ff    	cmp	x23, x8
10047cc98: 54000080    	b.eq	0x10047cca8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h14c7d0fae247b271E+0x30c>
10047cc9c: 910223f3    	add	x19, sp, #0x88
10047cca0: aa1303e0    	mov	x0, x19
10047cca4: 94015053    	bl	0x1004d0df0 <__ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h265e755c5c638973E>
10047cca8: 910023e0    	add	x0, sp, #0x8
10047ccac: 94014a59    	bl	0x1004cf610 <__ZN4core3ptr59drop_in_place$LT$serde_json..value..de..MapDeserializer$GT$17h07ad5c50c1632d21E>
10047ccb0: aa1503e0    	mov	x0, x21
10047ccb4: 94355128    	bl	0x1011d1154 <_writev+0x1011d1154>
10047ccb8: 94354fa4    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
