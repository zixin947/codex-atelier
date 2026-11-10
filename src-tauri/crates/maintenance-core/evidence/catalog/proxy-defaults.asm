
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010047f424 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h3953a62d494ac7feE>:
10047f770: a9453bef    	ldp	x15, x14, [sp, #0x50]
10047f774: f94033e8    	ldr	x8, [sp, #0x60]
10047f778: a915a3ef    	stp	x15, x8, [sp, #0x158]
10047f77c: f900b7f9    	str	x25, [sp, #0x168]
10047f780: a91473ee    	stp	x14, x28, [sp, #0x140]
10047f784: f900abf7    	str	x23, [sp, #0x150]
10047f788: a94427e8    	ldp	x8, x9, [sp, #0x40]
10047f78c: a912a7f8    	stp	x24, x9, [sp, #0x128]
10047f790: f9009fe8    	str	x8, [sp, #0x138]
10047f794: 52800028    	mov	w8, #0x1                ; =1
10047f798: a93023bf    	stp	xzr, x8, [x29, #-0x100]
10047f79c: d10403a9    	sub	x9, x29, #0x100
10047f7a0: 9100612a    	add	x10, x9, #0x18
10047f7a4: a9317fbf    	stp	xzr, xzr, [x29, #-0xf0]
10047f7a8: f81203a8    	stur	x8, [x29, #-0xe0]
10047f7ac: 9100c128    	add	x8, x9, #0x30
10047f7b0: f000740b    	adrp	x11, 0x101302000 <_writev+0x101302000>
10047f7b4: 3dc03560    	ldr	q0, [x11, #0xd0]
10047f7b8: 910343f0    	add	x16, sp, #0xd0
10047f7bc: 3c8c8200    	stur	q0, [x16, #0xc8]
10047f7c0: b9403feb    	ldr	w11, [sp, #0x3c]
10047f7c4: 7200017f    	tst	w11, #0x1
10047f7c8: b94027eb    	ldr	w11, [sp, #0x24]
10047f7cc: 1a9f116b    	csel	w11, w11, wzr, ne
10047f7d0: d2f0000c    	mov	x12, #-0x8000000000000000 ; =-9223372036854775808
10047f7d4: 9104a3ed    	add	x13, sp, #0x128
10047f7d8: eb0c031f    	cmp	x24, x12
10047f7dc: 9a8d0129    	csel	x9, x9, x13, eq
10047f7e0: f940092d    	ldr	x13, [x9, #0x10]
10047f7e4: 3dc00120    	ldr	q0, [x9]
10047f7e8: 3d803e00    	str	q0, [x16, #0xf0]
10047f7ec: 910503e9    	add	x9, sp, #0x140
10047f7f0: eb0c01df    	cmp	x14, x12
10047f7f4: 9a890149    	csel	x9, x10, x9, eq
10047f7f8: 3dc00121    	ldr	q1, [x9]
10047f7fc: d102c3ae    	sub	x14, x29, #0xb0
10047f800: 3c8181c1    	stur	q1, [x14, #0x18]
10047f804: f9400929    	ldr	x9, [x9, #0x10]
10047f808: f81603ad    	stur	x13, [x29, #-0xa0]
10047f80c: f81783a9    	stur	x9, [x29, #-0x88]
10047f810: b24107e9    	mov	x9, #-0x7fffffffffffffff ; =-9223372036854775807
10047f814: 910563ea    	add	x10, sp, #0x158
10047f818: eb0901ff    	cmp	x15, x9
10047f81c: 9a8a0108    	csel	x8, x8, x10, eq
10047f820: ad4089c1    	ldp	q1, q2, [x14, #0x10]
10047f824: ad000680    	stp	q0, q1, [x20]
10047f828: 3dc00100    	ldr	q0, [x8]
10047f82c: 3d800dc0    	str	q0, [x14, #0x30]
10047f830: f9400908    	ldr	x8, [x8, #0x10]
10047f834: f81903a8    	stur	x8, [x29, #-0x70]
10047f838: 120002c9    	and	w9, w22, #0x1
10047f83c: f9002288    	str	x8, [x20, #0x40]
10047f840: ad010282    	stp	q2, q0, [x20, #0x20]
10047f844: 7900928b    	strh	w11, [x20, #0x48]
10047f848: 39012a89    	strb	w9, [x20, #0x4a]
