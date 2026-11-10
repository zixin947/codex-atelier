
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078d874 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE>:
10078d874: 6db923e9    	stp	d9, d8, [sp, #-0x70]!
10078d878: a9016ffc    	stp	x28, x27, [sp, #0x10]
10078d87c: a90267fa    	stp	x26, x25, [sp, #0x20]
10078d880: a9035ff8    	stp	x24, x23, [sp, #0x30]
10078d884: a90457f6    	stp	x22, x21, [sp, #0x40]
10078d888: a9054ff4    	stp	x20, x19, [sp, #0x50]
10078d88c: a9067bfd    	stp	x29, x30, [sp, #0x60]
10078d890: 910183fd    	add	x29, sp, #0x60
10078d894: aa0103f3    	mov	x19, x1
10078d898: f100303f    	cmp	x1, #0xc
10078d89c: 540001e1    	b.ne	0x10078d8d8 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x64>
10078d8a0: f9400008    	ldr	x8, [x0]
10078d8a4: b9400809    	ldr	w9, [x0, #0x8]
10078d8a8: d28d2c2a    	mov	x10, #0x6961            ; =26977
10078d8ac: f2ac2daa    	movk	x10, #0x616d, lsl #16
10078d8b0: f2cd2daa    	movk	x10, #0x696d, lsl #32
10078d8b4: f2ee4bea    	movk	x10, #0x725f, lsl #48
10078d8b8: eb0a011f    	cmp	x8, x10
10078d8bc: 528d8ca8    	mov	w8, #0x6c65             ; =27749
10078d8c0: 72af2c28    	movk	w8, #0x7961, lsl #16
10078d8c4: 7a480120    	ccmp	w9, w8, #0x0, eq
10078d8c8: 540000c1    	b.ne	0x10078d8e0 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x6c>
10078d8cc: 52800028    	mov	w8, #0x1                ; =1
10078d8d0: 35000382    	cbnz	w2, 0x10078d940 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0xcc>
10078d8d4: 14000004    	b	0x10078d8e4 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x70>
10078d8d8: f100367f    	cmp	x19, #0xd
10078d8dc: 54000162    	b.hs	0x10078d908 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x94>
10078d8e0: 52800008    	mov	w8, #0x0                ; =0
10078d8e4: aa0803e0    	mov	x0, x8
10078d8e8: a9467bfd    	ldp	x29, x30, [sp, #0x60]
10078d8ec: a9454ff4    	ldp	x20, x19, [sp, #0x50]
10078d8f0: a94457f6    	ldp	x22, x21, [sp, #0x40]
10078d8f4: a9435ff8    	ldp	x24, x23, [sp, #0x30]
10078d8f8: a94267fa    	ldp	x26, x25, [sp, #0x20]
10078d8fc: a9416ffc    	ldp	x28, x27, [sp, #0x10]
10078d900: 6cc723e9    	ldp	d9, d8, [sp], #0x70
10078d904: d65f03c0    	ret
10078d908: f9400008    	ldr	x8, [x0]
10078d90c: f8405009    	ldur	x9, [x0, #0x5]
10078d910: d28d2c2a    	mov	x10, #0x6961            ; =26977
10078d914: f2ac2daa    	movk	x10, #0x616d, lsl #16
10078d918: f2cd2daa    	movk	x10, #0x696d, lsl #32
10078d91c: f2ee4bea    	movk	x10, #0x725f, lsl #48
10078d920: eb0a011f    	cmp	x8, x10
10078d924: d28bed28    	mov	x8, #0x5f69             ; =24425
10078d928: f2acae48    	movk	x8, #0x6572, lsl #16
10078d92c: f2cc2d88    	movk	x8, #0x616c, lsl #32
10078d930: f2ebef28    	movk	x8, #0x5f79, lsl #48
10078d934: fa480120    	ccmp	x9, x8, #0x0, eq
10078d938: 1a9f17e8    	cset	w8, eq
10078d93c: 34fffd42    	cbz	w2, 0x10078d8e4 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x70>
10078d940: 3607fd28    	tbz	w8, #0x0, 0x10078d8e4 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x70>
10078d944: f9400c68    	ldr	x8, [x3, #0x18]
10078d948: b4000668    	cbz	x8, 0x10078da14 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x1a0>
10078d94c: aa0003e1    	mov	x1, x0
10078d950: 91008060    	add	x0, x3, #0x20
10078d954: aa0103f4    	mov	x20, x1
10078d958: aa1303e2    	mov	x2, x19
10078d95c: aa0303f7    	mov	x23, x3
10078d960: 97fcdd96    	bl	0x1006c4fb8 <__ZN4core4hash11BuildHasher8hash_one17h61e5c885b8041446E>
10078d964: d2800015    	mov	x21, #0x0               ; =0
10078d968: d379fc08    	lsr	x8, x0, #57
10078d96c: a9405af7    	ldp	x23, x22, [x23]
10078d970: 0e010d08    	dup.8b	v8, w8
10078d974: d10062f8    	sub	x24, x23, #0x18
10078d978: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
10078d97c: 52800319    	mov	w25, #0x18              ; =24
10078d980: 8a16001a    	and	x26, x0, x22
10078d984: fc7a6ae9    	ldr	d9, [x23, x26]
10078d988: 2e288d20    	cmeq.8b	v0, v9, v8
10078d98c: 9e660008    	fmov	x8, d0
10078d990: f201c11b    	ands	x27, x8, #0x8080808080808080
10078d994: 54000260    	b.eq	0x10078d9e0 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x16c>
10078d998: dac00368    	rbit	x8, x27
10078d99c: dac01108    	clz	x8, x8
10078d9a0: 8b480f48    	add	x8, x26, x8, lsr #3
10078d9a4: 8a160108    	and	x8, x8, x22
10078d9a8: 9b19fd08    	mneg	x8, x8, x25
10078d9ac: 8b080308    	add	x8, x24, x8
10078d9b0: f9400909    	ldr	x9, [x8, #0x10]
10078d9b4: eb09027f    	cmp	x19, x9
10078d9b8: 540000c1    	b.ne	0x10078d9d0 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x15c>
10078d9bc: f9400501    	ldr	x1, [x8, #0x8]
10078d9c0: aa1403e0    	mov	x0, x20
10078d9c4: aa1303e2    	mov	x2, x19
10078d9c8: 94290f36    	bl	0x1011d16a0 <_writev+0x1011d16a0>
10078d9cc: 34fff8a0    	cbz	w0, 0x10078d8e0 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x6c>
10078d9d0: d1000768    	sub	x8, x27, #0x1
10078d9d4: ea1b011b    	ands	x27, x8, x27
10078d9d8: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
10078d9dc: 54fffde1    	b.ne	0x10078d998 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x124>
10078d9e0: 2e218d20    	cmeq.8b	v0, v9, v1
10078d9e4: 2e30a800    	umaxv.8b	b0, v0
10078d9e8: 1e260008    	fmov	w8, s0
10078d9ec: 37000148    	tbnz	w8, #0x0, 0x10078da14 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x1a0>
10078d9f0: 910022b5    	add	x21, x21, #0x8
10078d9f4: 8b150340    	add	x0, x26, x21
10078d9f8: 8a16001a    	and	x26, x0, x22
10078d9fc: fc7a6ae9    	ldr	d9, [x23, x26]
10078da00: 2e288d20    	cmeq.8b	v0, v9, v8
10078da04: 9e660008    	fmov	x8, d0
10078da08: f201c11b    	ands	x27, x8, #0x8080808080808080
10078da0c: 54fffc61    	b.ne	0x10078d998 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x124>
10078da10: 17fffff4    	b	0x10078d9e0 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x16c>
10078da14: 52800028    	mov	w8, #0x1                ; =1
10078da18: 17ffffb3    	b	0x10078d8e4 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE+0x70>
