
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001004831f8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE>:
1004831f8: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
1004831fc: a90167fa    	stp	x26, x25, [sp, #0x10]
100483200: a9025ff8    	stp	x24, x23, [sp, #0x20]
100483204: a90357f6    	stp	x22, x21, [sp, #0x30]
100483208: a9044ff4    	stp	x20, x19, [sp, #0x40]
10048320c: a9057bfd    	stp	x29, x30, [sp, #0x50]
100483210: 910143fd    	add	x29, sp, #0x50
100483214: d11583ff    	sub	sp, sp, #0x560
100483218: f90003ff    	str	xzr, [sp]
10048321c: f90023e8    	str	x8, [sp, #0x40]
100483220: b9009fff    	str	wzr, [sp, #0x9c]
100483224: d10403ac    	sub	x12, x29, #0x100
100483228: 910cc3ed    	add	x13, sp, #0x330
10048322c: a940a809    	ldp	x9, x10, [x0, #0x8]
100483230: f9400008    	ldr	x8, [x0]
100483234: f100011f    	cmp	x8, #0x0
100483238: f90017ea    	str	x10, [sp, #0x28]
10048323c: 9a9f114a    	csel	x10, x10, xzr, ne
100483240: 1a9f07eb    	cset	w11, ne
100483244: a90affeb    	stp	x11, xzr, [sp, #0xa8]
100483248: 9102a3fc    	add	x28, sp, #0xa8
10048324c: 528000ce    	mov	w14, #0x6               ; =6
100483250: d2f0001b    	mov	x27, #-0x8000000000000000 ; =-9223372036854775808
100483254: a90ba7e8    	stp	x8, x9, [sp, #0xb8]
100483258: b24001b6    	orr	x22, x13, #0x1
10048325c: b240018c    	orr	x12, x12, #0x1
100483260: a907b3ff    	stp	xzr, x12, [sp, #0x78]
100483264: d101e3ac    	sub	x12, x29, #0x78
100483268: b240018c    	orr	x12, x12, #0x1
10048326c: f9001fec    	str	x12, [sp, #0x38]
100483270: a90cffeb    	stp	x11, xzr, [sp, #0xc8]
100483274: 5280004b    	mov	w11, #0x2               ; =2
100483278: b00074ba    	adrp	x26, 0x101318000 <_anon.a9e2253c03df711f51070ce6102367ea.930+0x2c>
10048327c: 911cbb5a    	add	x26, x26, #0x72e
100483280: a90da7e8    	stp	x8, x9, [sp, #0xd8]
100483284: f90077ea    	str	x10, [sp, #0xe8]
100483288: 3903c3ee    	strb	w14, [sp, #0xf0]
10048328c: f9014bfb    	str	x27, [sp, #0x290]
100483290: d2f00019    	mov	x25, #-0x8000000000000000 ; =-9223372036854775808
100483294: f90157fb    	str	x27, [sp, #0x2a8]
100483298: d2f00018    	mov	x24, #-0x8000000000000000 ; =-9223372036854775808
10048329c: f90163fb    	str	x27, [sp, #0x2c0]
1004832a0: d2f00014    	mov	x20, #-0x8000000000000000 ; =-9223372036854775808
1004832a4: d2f00015    	mov	x21, #-0x8000000000000000 ; =-9223372036854775808
1004832a8: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1004832ac: f90053e8    	str	x8, [sp, #0xa0]
1004832b0: 52800048    	mov	w8, #0x2                ; =2
1004832b4: 2912a3e8    	stp	w8, w8, [sp, #0x94]
1004832b8: 2911a3eb    	stp	w11, w8, [sp, #0x8c]
1004832bc: 910cc3e8    	add	x8, sp, #0x330
1004832c0: 9102a3e0    	add	x0, sp, #0xa8
1004832c4: 97fb4b1a    	bl	0x100355f2c <__ZN84_$LT$serde_json..value..de..MapDeserializer$u20$as$u20$serde_core..de..MapAccess$GT$13next_key_seed17h946d32fbdac4fb5cE>
1004832c8: 394cc3e8    	ldrb	w8, [sp, #0x330]
1004832cc: 7100051f    	cmp	w8, #0x1
1004832d0: 540021e0    	b.eq	0x10048370c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x514>
1004832d4: 394cc7e8    	ldrb	w8, [sp, #0x331]
1004832d8: 10000089    	adr	x9, 0x1004832e8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xf0>
1004832dc: 78687b4a    	ldrh	w10, [x26, x8, lsl #1]
1004832e0: 8b0a0929    	add	x9, x9, x10, lsl #2
1004832e4: d61f0120    	br	x9
1004832e8: b9409fe8    	ldr	w8, [sp, #0x9c]
1004832ec: 350063a8    	cbnz	w8, 0x100483f60 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xd68>
1004832f0: 910cc3e8    	add	x8, sp, #0x330
1004832f4: 9102a3e0    	add	x0, sp, #0xa8
1004832f8: 97fb4f11    	bl	0x100356f3c <__ZN84_$LT$serde_json..value..de..MapDeserializer$u20$as$u20$serde_core..de..MapAccess$GT$15next_value_seed17h1c236884872ad08eE>
1004832fc: b94333e8    	ldr	w8, [sp, #0x330]
100483300: 7100051f    	cmp	w8, #0x1
100483304: 54002040    	b.eq	0x10048370c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x514>
100483308: b94337e8    	ldr	w8, [sp, #0x334]
10048330c: b90037e8    	str	w8, [sp, #0x34]
100483310: 52800028    	mov	w8, #0x1                ; =1
100483314: b9009fe8    	str	w8, [sp, #0x9c]
100483318: 17ffffe9    	b	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
10048331c: b94097e8    	ldr	w8, [sp, #0x94]
100483320: 7100091f    	cmp	w8, #0x2
100483324: 54006921    	b.ne	0x100484048 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xe50>
100483328: 3943c3f3    	ldrb	w19, [sp, #0xf0]
10048332c: 528000c8    	mov	w8, #0x6                ; =6
100483330: 3903c3e8    	strb	w8, [sp, #0xf0]
100483334: 71001a7f    	cmp	w19, #0x6
100483338: 54002d80    	b.eq	0x1004838e8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x6f0>
10048333c: 390cc3f3    	strb	w19, [sp, #0x330]
100483340: 9102a3e8    	add	x8, sp, #0xa8
100483344: 3cc49100    	ldur	q0, [x8, #0x49]
100483348: 3d8002c0    	str	q0, [x22]
10048334c: 9102a3fc    	add	x28, sp, #0xa8
100483350: 3cc58100    	ldur	q0, [x8, #0x58]
100483354: 3c80f2c0    	stur	q0, [x22, #0xf]
100483358: 7100067f    	cmp	w19, #0x1
10048335c: 54001ae1    	b.ne	0x1004836b8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x4c0>
100483360: 394cc7e8    	ldrb	w8, [sp, #0x331]
100483364: b9007be8    	str	w8, [sp, #0x78]
100483368: 910cc3e0    	add	x0, sp, #0x330
10048336c: 9413a2cd    	bl	0x10096bea0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100483370: b9407be8    	ldr	w8, [sp, #0x78]
100483374: b90097e8    	str	w8, [sp, #0x94]
100483378: 7100067f    	cmp	w19, #0x1
10048337c: 54fffa00    	b.eq	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
100483380: 14000213    	b	0x100483bcc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x9d4>
100483384: 3943c3e8    	ldrb	w8, [sp, #0xf0]
100483388: 528000c9    	mov	w9, #0x6                ; =6
10048338c: 3903c3e9    	strb	w9, [sp, #0xf0]
100483390: 7100191f    	cmp	w8, #0x6
100483394: 540025a0    	b.eq	0x100483848 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x650>
100483398: 9102a3e9    	add	x9, sp, #0xa8
10048339c: 3cc49120    	ldur	q0, [x9, #0x49]
1004833a0: 3d8002c0    	str	q0, [x22]
1004833a4: 9102a3fc    	add	x28, sp, #0xa8
1004833a8: 3cc58120    	ldur	q0, [x9, #0x58]
1004833ac: 3c80f2c0    	stur	q0, [x22, #0xf]
1004833b0: 390cc3e8    	strb	w8, [sp, #0x330]
1004833b4: 910cc3e0    	add	x0, sp, #0x330
1004833b8: 9413a2ba    	bl	0x10096bea0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
1004833bc: 17ffffc0    	b	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
1004833c0: eb1b031f    	cmp	x24, x27
1004833c4: 54005941    	b.ne	0x100483eec <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xcf4>
1004833c8: 3943c3e8    	ldrb	w8, [sp, #0xf0]
1004833cc: 528000c9    	mov	w9, #0x6                ; =6
1004833d0: 3903c3e9    	strb	w9, [sp, #0xf0]
1004833d4: 7100191f    	cmp	w8, #0x6
1004833d8: 54002b00    	b.eq	0x100483938 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x740>
1004833dc: 390cc3e8    	strb	w8, [sp, #0x330]
1004833e0: 9102a3e8    	add	x8, sp, #0xa8
1004833e4: 3cc49100    	ldur	q0, [x8, #0x49]
1004833e8: 3d8002c0    	str	q0, [x22]
1004833ec: 9102a3fc    	add	x28, sp, #0xa8
1004833f0: 3cc58100    	ldur	q0, [x8, #0x58]
1004833f4: 3c80f2c0    	stur	q0, [x22, #0xf]
1004833f8: d2f00018    	mov	x24, #-0x8000000000000000 ; =-9223372036854775808
1004833fc: d10403a8    	sub	x8, x29, #0x100
100483400: 910cc3e0    	add	x0, sp, #0x330
100483404: b0007941    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
100483408: 91110021    	add	x1, x1, #0x440
10048340c: 52800202    	mov	w2, #0x10               ; =16
100483410: f0009d23    	adrp	x3, 0x10182a000 <_anon.cdb272158a04aae91ce02c6e47f106c7.1+0x6d0>
100483414: 913a2063    	add	x3, x3, #0xe88
100483418: 52800024    	mov	w4, #0x1                ; =1
10048341c: 941256c2    	bl	0x100918f24 <__ZN10serde_json5value2de83_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..value..Value$GT$18deserialize_struct17h7e49d5bf169e5c26E>
100483420: a97003b8    	ldp	x24, x0, [x29, #-0x100]
100483424: eb1b031f    	cmp	x24, x27
100483428: 54003b20    	b.eq	0x100483b8c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x994>
10048342c: f85103a8    	ldur	x8, [x29, #-0xf0]
100483430: a90523e0    	stp	x0, x8, [sp, #0x50]
100483434: 17ffffa2    	b	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
100483438: eb1b029f    	cmp	x20, x27
10048343c: 54006401    	b.ne	0x1004840bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xec4>
100483440: 3943c3e8    	ldrb	w8, [sp, #0xf0]
100483444: 528000c9    	mov	w9, #0x6                ; =6
100483448: 3903c3e9    	strb	w9, [sp, #0xf0]
10048344c: 7100191f    	cmp	w8, #0x6
100483450: 54002ee0    	b.eq	0x100483a2c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x834>
100483454: 381003a8    	sturb	w8, [x29, #-0x100]
100483458: 9102a3e8    	add	x8, sp, #0xa8
10048345c: 3cc49100    	ldur	q0, [x8, #0x49]
100483460: f94043e9    	ldr	x9, [sp, #0x80]
100483464: 3d800120    	str	q0, [x9]
100483468: 9102a3fc    	add	x28, sp, #0xa8
10048346c: 3cc58100    	ldur	q0, [x8, #0x58]
100483470: 3c80f120    	stur	q0, [x9, #0xf]
100483474: d2f00014    	mov	x20, #-0x8000000000000000 ; =-9223372036854775808
100483478: 910cc3e8    	add	x8, sp, #0x330
10048347c: d10403a0    	sub	x0, x29, #0x100
100483480: b0007941    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
100483484: 9111b021    	add	x1, x1, #0x46c
100483488: 52800202    	mov	w2, #0x10               ; =16
10048348c: 90009d43    	adrp	x3, 0x10182b000 <_anon.6408540cdffbe318d49f52812e99783f.784+0x28>
100483490: 9105a063    	add	x3, x3, #0x168
100483494: 528000a4    	mov	w4, #0x5                ; =5
100483498: 941254de    	bl	0x100918810 <__ZN10serde_json5value2de83_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..value..Value$GT$18deserialize_struct17h3353156f05d18bbdE>
10048349c: f9419bf4    	ldr	x20, [sp, #0x330]
1004834a0: f9419fe0    	ldr	x0, [sp, #0x338]
1004834a4: eb1b029f    	cmp	x20, x27
1004834a8: 54003420    	b.eq	0x100483b2c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x934>
1004834ac: 910cc3e8    	add	x8, sp, #0x330
1004834b0: ad408500    	ldp	q0, q1, [x8, #0x10]
1004834b4: 910b03e9    	add	x9, sp, #0x2c0
1004834b8: ad008520    	stp	q0, q1, [x9, #0x10]
1004834bc: ad418500    	ldp	q0, q1, [x8, #0x30]
1004834c0: ad018520    	stp	q0, q1, [x9, #0x30]
1004834c4: f90027e0    	str	x0, [sp, #0x48]
1004834c8: 17ffff7d    	b	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
1004834cc: f94053e8    	ldr	x8, [sp, #0xa0]
1004834d0: eb1b011f    	cmp	x8, x27
1004834d4: 54006681    	b.ne	0x1004841a4 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xfac>
1004834d8: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1004834dc: f90053e8    	str	x8, [sp, #0xa0]
1004834e0: 910cc3e8    	add	x8, sp, #0x330
1004834e4: 9102a3e0    	add	x0, sp, #0xa8
1004834e8: 97fb522f    	bl	0x100357da4 <__ZN84_$LT$serde_json..value..de..MapDeserializer$u20$as$u20$serde_core..de..MapAccess$GT$15next_value_seed17hea734d97f47f3ba4E>
1004834ec: f9419bf5    	ldr	x21, [sp, #0x330]
1004834f0: f9419fe8    	ldr	x8, [sp, #0x338]
1004834f4: f9003be8    	str	x8, [sp, #0x70]
1004834f8: eb1b02bf    	cmp	x21, x27
1004834fc: 54002c20    	b.eq	0x100483a80 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x888>
100483500: f941a3f7    	ldr	x23, [sp, #0x340]
100483504: f90053f5    	str	x21, [sp, #0xa0]
100483508: 17ffff6d    	b	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
10048350c: eb1b033f    	cmp	x25, x27
100483510: 54006101    	b.ne	0x100484130 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xf38>
100483514: d2f00019    	mov	x25, #-0x8000000000000000 ; =-9223372036854775808
100483518: 910cc3e8    	add	x8, sp, #0x330
10048351c: 9102a3e0    	add	x0, sp, #0xa8
100483520: 97fb50e3    	bl	0x1003578ac <__ZN84_$LT$serde_json..value..de..MapDeserializer$u20$as$u20$serde_core..de..MapAccess$GT$15next_value_seed17hb7db488e697945edE>
100483524: f9419bf9    	ldr	x25, [sp, #0x330]
100483528: f9419fe0    	ldr	x0, [sp, #0x338]
10048352c: eb1b033f    	cmp	x25, x27
100483530: 54002c40    	b.eq	0x100483ab8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x8c0>
100483534: f941a3e8    	ldr	x8, [sp, #0x340]
100483538: a90623e0    	stp	x0, x8, [sp, #0x60]
10048353c: 17ffff60    	b	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
100483540: b9408fe8    	ldr	w8, [sp, #0x8c]
100483544: 7100091f    	cmp	w8, #0x2
100483548: 54005461    	b.ne	0x100483fd4 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xddc>
10048354c: 3943c3e8    	ldrb	w8, [sp, #0xf0]
100483550: 528000c9    	mov	w9, #0x6                ; =6
100483554: 3903c3e9    	strb	w9, [sp, #0xf0]
100483558: 7100191f    	cmp	w8, #0x6
10048355c: 54002180    	b.eq	0x10048398c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x794>
100483560: 381003a8    	sturb	w8, [x29, #-0x100]
100483564: 9102a3e9    	add	x9, sp, #0xa8
100483568: 3cc49120    	ldur	q0, [x9, #0x49]
10048356c: f94043ea    	ldr	x10, [sp, #0x80]
100483570: 3d800140    	str	q0, [x10]
100483574: 9102a3fc    	add	x28, sp, #0xa8
100483578: 3cc58120    	ldur	q0, [x9, #0x58]
10048357c: 3c80f140    	stur	q0, [x10, #0xf]
100483580: 34000948    	cbz	w8, 0x1004836a8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x4b0>
100483584: 910cc3e8    	add	x8, sp, #0x330
100483588: ad4c0500    	ldp	q0, q1, [x8, #0x180]
10048358c: ad000500    	stp	q0, q1, [x8]
100483590: d101e3a8    	sub	x8, x29, #0x78
100483594: 910cc3e0    	add	x0, sp, #0x330
100483598: 940c6e8c    	bl	0x10079efc8 <__ZN13codexmate_lib4core5relay6models1_111_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..RouterOperationId$GT$11deserialize17h0ead1bed9d66bab3E>
10048359c: 385883a8    	ldurb	w8, [x29, #-0x78]
1004835a0: 7100051f    	cmp	w8, #0x1
1004835a4: 54003920    	b.eq	0x100483cc8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xad0>
1004835a8: f9401fe9    	ldr	x9, [sp, #0x38]
1004835ac: b9400128    	ldr	w8, [x9]
1004835b0: b9032be8    	str	w8, [sp, #0x328]
1004835b4: 79400928    	ldrh	w8, [x9, #0x4]
1004835b8: 79065be8    	strh	w8, [sp, #0x32c]
1004835bc: d10403a8    	sub	x8, x29, #0x100
1004835c0: f848f109    	ldur	x9, [x8, #0x8f]
1004835c4: f90013e9    	str	x9, [sp, #0x20]
1004835c8: 78497108    	ldurh	w8, [x8, #0x97]
1004835cc: b9001fe8    	str	w8, [sp, #0x1c]
1004835d0: 52800028    	mov	w8, #0x1                ; =1
1004835d4: b9008fe8    	str	w8, [sp, #0x8c]
1004835d8: 17ffff39    	b	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
1004835dc: b9409be8    	ldr	w8, [sp, #0x98]
1004835e0: 7100091f    	cmp	w8, #0x2
1004835e4: 54006cc1    	b.ne	0x10048437c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x1184>
1004835e8: 3943c3f3    	ldrb	w19, [sp, #0xf0]
1004835ec: 528000c8    	mov	w8, #0x6                ; =6
1004835f0: 3903c3e8    	strb	w8, [sp, #0xf0]
1004835f4: 71001a7f    	cmp	w19, #0x6
1004835f8: 54001f20    	b.eq	0x1004839dc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x7e4>
1004835fc: 390cc3f3    	strb	w19, [sp, #0x330]
100483600: 9102a3e8    	add	x8, sp, #0xa8
100483604: 3cc49100    	ldur	q0, [x8, #0x49]
100483608: 3d8002c0    	str	q0, [x22]
10048360c: 9102a3fc    	add	x28, sp, #0xa8
100483610: 3cc58100    	ldur	q0, [x8, #0x58]
100483614: 3c80f2c0    	stur	q0, [x22, #0xf]
100483618: 7100067f    	cmp	w19, #0x1
10048361c: 540005c1    	b.ne	0x1004836d4 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x4dc>
100483620: 394cc7e8    	ldrb	w8, [sp, #0x331]
100483624: b9007fe8    	str	w8, [sp, #0x7c]
100483628: 910cc3e0    	add	x0, sp, #0x330
10048362c: 9413a21d    	bl	0x10096bea0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100483630: b9407fe8    	ldr	w8, [sp, #0x7c]
100483634: b9009be8    	str	w8, [sp, #0x98]
100483638: 7100067f    	cmp	w19, #0x1
10048363c: 54ffe400    	b.eq	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
100483640: 14000172    	b	0x100483c08 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xa10>
100483644: b94093e8    	ldr	w8, [sp, #0x90]
100483648: 12001d08    	and	w8, w8, #0xff
10048364c: 7100091f    	cmp	w8, #0x2
100483650: 54004141    	b.ne	0x100483e78 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc80>
100483654: 3943c3f3    	ldrb	w19, [sp, #0xf0]
100483658: 528000c8    	mov	w8, #0x6                ; =6
10048365c: 3903c3e8    	strb	w8, [sp, #0xf0]
100483660: 71001a7f    	cmp	w19, #0x6
100483664: 540011a0    	b.eq	0x100483898 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x6a0>
100483668: 390cc3f3    	strb	w19, [sp, #0x330]
10048366c: 9102a3e8    	add	x8, sp, #0xa8
100483670: 3cc49100    	ldur	q0, [x8, #0x49]
100483674: 3d8002c0    	str	q0, [x22]
100483678: 9102a3fc    	add	x28, sp, #0xa8
10048367c: 3cc58100    	ldur	q0, [x8, #0x58]
100483680: 3c80f2c0    	stur	q0, [x22, #0xf]
100483684: 7100067f    	cmp	w19, #0x1
100483688: 54000341    	b.ne	0x1004836f0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x4f8>
10048368c: 394cc7e8    	ldrb	w8, [sp, #0x331]
100483690: b90093e8    	str	w8, [sp, #0x90]
100483694: 910cc3e0    	add	x0, sp, #0x330
100483698: 9413a202    	bl	0x10096bea0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10048369c: 7100067f    	cmp	w19, #0x1
1004836a0: 54ffe0e0    	b.eq	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
1004836a4: 14000113    	b	0x100483af0 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x8f8>
1004836a8: d10403a0    	sub	x0, x29, #0x100
1004836ac: 97ff7566    	bl	0x100460c44 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
1004836b0: b9008fff    	str	wzr, [sp, #0x8c]
1004836b4: 17ffff02    	b	0x1004832bc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xc4>
1004836b8: 910cc3e0    	add	x0, sp, #0x330
1004836bc: d10403a1    	sub	x1, x29, #0x100
1004836c0: 90009d62    	adrp	x2, 0x10182f000 <_anon.93008c7a652f50ea5d1586046426690a.9+0x10>
1004836c4: 910a4042    	add	x2, x2, #0x290
1004836c8: 94347357    	bl	0x1011a0424 <__ZN10serde_json5value2de42_$LT$impl$u20$serde_json..value..Value$GT$12invalid_type17hb2f6e8292cb4d8b1E>
1004836cc: f90007e0    	str	x0, [sp, #0x8]
1004836d0: 17ffff26    	b	0x100483368 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x170>
1004836d4: 910cc3e0    	add	x0, sp, #0x330
1004836d8: d10403a1    	sub	x1, x29, #0x100
1004836dc: 90009d62    	adrp	x2, 0x10182f000 <_anon.93008c7a652f50ea5d1586046426690a.9+0x10>
1004836e0: 910a4042    	add	x2, x2, #0x290
1004836e4: 94347350    	bl	0x1011a0424 <__ZN10serde_json5value2de42_$LT$impl$u20$serde_json..value..Value$GT$12invalid_type17hb2f6e8292cb4d8b1E>
1004836e8: f9000be0    	str	x0, [sp, #0x10]
1004836ec: 17ffffcf    	b	0x100483628 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x430>
1004836f0: 910cc3e0    	add	x0, sp, #0x330
1004836f4: d10403a1    	sub	x1, x29, #0x100
1004836f8: 90009d62    	adrp	x2, 0x10182f000 <_anon.93008c7a652f50ea5d1586046426690a.9+0x10>
1004836fc: 910a4042    	add	x2, x2, #0x290
100483700: 94347349    	bl	0x1011a0424 <__ZN10serde_json5value2de42_$LT$impl$u20$serde_json..value..Value$GT$12invalid_type17hb2f6e8292cb4d8b1E>
100483704: f90003e0    	str	x0, [sp]
100483708: 17ffffe3    	b	0x100483694 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x49c>
10048370c: f9018bf5    	str	x21, [sp, #0x310]

/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001004831f8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE>:
100483c78: f10006f7    	subs	x23, x23, #0x1
100483c7c: 54002d60    	b.eq	0x100484228 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0x1030>
100483c80: f85e0261    	ldur	x1, [x19, #-0x20]
100483c84: b4000081    	cbz	x1, 0x100483c94 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xa9c>
100483c88: f85e8260    	ldur	x0, [x19, #-0x18]
100483c8c: 52800022    	mov	w2, #0x1                ; =1
100483c90: 97edf636    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100483c94: f85f8261    	ldur	x1, [x19, #-0x8]
100483c98: b4000081    	cbz	x1, 0x100483ca8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xab0>
100483c9c: f9400260    	ldr	x0, [x19]
100483ca0: 52800022    	mov	w2, #0x1                ; =1
100483ca4: 97edf631    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100483ca8: f9400a61    	ldr	x1, [x19, #0x10]
100483cac: eb15003f    	cmp	x1, x21
100483cb0: fa401824    	ccmp	x1, #0x0, #0x4, ne
100483cb4: 54fffe00    	b.eq	0x100483c74 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xa7c>
100483cb8: f9400e60    	ldr	x0, [x19, #0x18]
100483cbc: 52800022    	mov	w2, #0x1                ; =1
100483cc0: 97edf62a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100483cc4: 17ffffec    	b	0x100483c74 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xa7c>
100483cc8: f9018bf5    	str	x21, [sp, #0x310]
100483ccc: f9403be8    	ldr	x8, [sp, #0x70]
100483cd0: f9018fe8    	str	x8, [sp, #0x318]
100483cd4: f90193f7    	str	x23, [sp, #0x320]
100483cd8: f90163f4    	str	x20, [sp, #0x2c0]
100483cdc: a944a3e9    	ldp	x9, x8, [sp, #0x48]
100483ce0: f90167e9    	str	x9, [sp, #0x2c8]
100483ce4: f90157f8    	str	x24, [sp, #0x2a8]
100483ce8: f9015be8    	str	x8, [sp, #0x2b0]
100483cec: a945a3e9    	ldp	x9, x8, [sp, #0x58]
100483cf0: f9015fe9    	str	x9, [sp, #0x2b8]
100483cf4: f9014bf9    	str	x25, [sp, #0x290]
100483cf8: f9014fe8    	str	x8, [sp, #0x298]
100483cfc: f94037e8    	ldr	x8, [sp, #0x68]
100483d00: f90153e8    	str	x8, [sp, #0x2a0]
100483d04: f85903a0    	ldur	x0, [x29, #-0x70]
100483d08: 17ffffd0    	b	0x100483c48 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE+0xa50>
100483d0c: b9432bec    	ldr	w12, [sp, #0x328]
100483d10: b81803ac    	stur	w12, [x29, #-0x80]
100483d14: 79465bec    	ldrh	w12, [sp, #0x32c]
100483d18: 781843ac    	sturh	w12, [x29, #-0x7c]
100483d1c: f94023e1    	ldr	x1, [sp, #0x40]
100483d20: f94053ec    	ldr	x12, [sp, #0xa0]
100483d24: f94013e4    	ldr	x4, [sp, #0x20]
100483d28: b9401fe5    	ldr	w5, [sp, #0x1c]
100483d2c: 29529be7    	ldp	w7, w6, [sp, #0x94]
100483d30: eb08019f    	cmp	x12, x8
100483d34: 9a8c03ec    	csel	x12, xzr, x12, eq
100483d38: 52800110    	mov	w16, #0x8               ; =8
100483d3c: ad430560    	ldp	q0, q1, [x11, #0x60]
100483d40: f9403bf1    	ldr	x17, [sp, #0x70]
100483d44: 9a910210    	csel	x16, x16, x17, eq
100483d48: 9a9703f1    	csel	x17, xzr, x23, eq
100483d4c: 3c8f8020    	stur	q0, [x1, #0xf8]
100483d50: b94093e2    	ldr	w2, [sp, #0x90]
100483d54: 12001c40    	and	w0, w2, #0xff
100483d58: 7100081f    	cmp	w0, #0x2
100483d5c: 1a9f17e0    	cset	w0, eq
100483d60: 2a020000    	orr	w0, w0, w2
100483d64: ad440960    	ldp	q0, q2, [x11, #0x80]
100483d68: 91026022    	add	x2, x1, #0x98
100483d6c: ad038041    	stp	q1, q0, [x2, #0x70]
100483d70: 3d802442    	str	q2, [x2, #0x90]
100483d74: ad410560    	ldp	q0, q1, [x11, #0x20]
100483d78: 3c8b8020    	stur	q0, [x1, #0xb8]
100483d7c: 3c8c8021    	stur	q1, [x1, #0xc8]
100483d80: ad420560    	ldp	q0, q1, [x11, #0x40]
100483d84: 3c8d8020    	stur	q0, [x1, #0xd8]
100483d88: 3c8e8021    	stur	q1, [x1, #0xe8]
100483d8c: ad400560    	ldp	q0, q1, [x11]
100483d90: 3c898020    	stur	q0, [x1, #0x98]
100483d94: 3c8a8021    	stur	q1, [x1, #0xa8]
100483d98: 9104e02b    	add	x11, x1, #0x138
100483d9c: ad400540    	ldp	q0, q1, [x10]
100483da0: ad000560    	stp	q0, q1, [x11]
100483da4: 3dc00940    	ldr	q0, [x10, #0x20]
100483da8: 3d800960    	str	q0, [x11, #0x20]
100483dac: 3dc001a0    	ldr	q0, [x13]
100483db0: 910cc3ea    	add	x10, sp, #0x330
100483db4: 3d806140    	str	q0, [x10, #0x180]
100483db8: f94009aa    	ldr	x10, [x13, #0x10]
100483dbc: 3dc001e0    	ldr	q0, [x15]
100483dc0: d10403a2    	sub	x2, x29, #0x100
100483dc4: 3c818040    	stur	q0, [x2, #0x18]
100483dc8: f94009eb    	ldr	x11, [x15, #0x10]
100483dcc: f81103aa    	stur	x10, [x29, #-0xf0]
100483dd0: f81283ab    	stur	x11, [x29, #-0xd8]
100483dd4: ad4005c0    	ldp	q0, q1, [x14]
100483dd8: ad410dc2    	ldp	q2, q3, [x14, #0x20]
100483ddc: 3dc011c4    	ldr	q4, [x14, #0x40]
100483de0: ad031043    	stp	q3, q4, [x2, #0x60]
100483de4: ad020841    	stp	q1, q2, [x2, #0x40]
100483de8: 3d800c40    	str	q0, [x2, #0x30]
100483dec: 120000ea    	and	w10, w7, #0x1
100483df0: 9105b82b    	add	x11, x1, #0x16e
100483df4: b85803ad    	ldur	w13, [x29, #-0x80]
100483df8: b900016d    	str	w13, [x11]
100483dfc: 785843ab    	ldurh	w11, [x29, #-0x7c]
100483e00: 7902e42b    	strh	w11, [x1, #0x172]
100483e04: 120000cb    	and	w11, w6, #0x1
100483e08: 1200000d    	and	w13, w0, #0x1
100483e0c: ad031023    	stp	q3, q4, [x1, #0x60]
100483e10: ad020821    	stp	q1, q2, [x1, #0x40]
100483e14: ad400440    	ldp	q0, q1, [x2]
100483e18: ad000420    	stp	q0, q1, [x1]
100483e1c: ad410440    	ldp	q0, q1, [x2, #0x20]
100483e20: ad010420    	stp	q0, q1, [x1, #0x20]
100483e24: a908402c    	stp	x12, x16, [x1, #0x80]
100483e28: f9004831    	str	x17, [x1, #0x90]
100483e2c: b9016829    	str	w9, [x1, #0x168]
100483e30: 3905b02a    	strb	w10, [x1, #0x16c]
