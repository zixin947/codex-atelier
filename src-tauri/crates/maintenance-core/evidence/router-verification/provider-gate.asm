
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100362e40 <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E>:
100362e40: d10583ff    	sub	sp, sp, #0x160
100362e44: a9125ff8    	stp	x24, x23, [sp, #0x120]
100362e48: a91357f6    	stp	x22, x21, [sp, #0x130]
100362e4c: a9144ff4    	stp	x20, x19, [sp, #0x140]
100362e50: a9157bfd    	stp	x29, x30, [sp, #0x150]
100362e54: 910543fd    	add	x29, sp, #0x150
100362e58: aa0203f3    	mov	x19, x2
100362e5c: 910003f4    	mov	x20, sp
100362e60: 910003e8    	mov	x8, sp
100362e64: 528000c2    	mov	w2, #0x6                ; =6
100362e68: 94158b38    	bl	0x1008c5b48 <__ZN13codexmate_lib4core5relay23codex_thread_visibility35inspect_thread_provider_convergence17h176adfa9157132f2E>
100362e6c: a94527e8    	ldp	x8, x9, [sp, #0x50]
100362e70: f94033ea    	ldr	x10, [sp, #0x60]
100362e74: f100011f    	cmp	x8, #0x0
100362e78: fa400920    	ccmp	x9, #0x0, #0x0, eq
100362e7c: fa400940    	ccmp	x10, #0x0, #0x0, eq
100362e80: 54000061    	b.ne	0x100362e8c <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E+0x4c>
100362e84: f94017e8    	ldr	x8, [sp, #0x28]
100362e88: b40009e8    	cbz	x8, 0x100362fc4 <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E+0x184>
100362e8c: f94023e1    	ldr	x1, [sp, #0x40]
100362e90: f0000455    	adrp	x21, 0x1003ed000 <__ZN5alloc11collections5btree3set21BTreeSet$LT$T$C$A$GT$10difference17h151923f22deefe03E+0x15c>
100362e94: 9133e2b5    	add	x21, x21, #0xcf8
100362e98: b4000381    	cbz	x1, 0x100362f08 <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E+0xc8>
100362e9c: f9401fe0    	ldr	x0, [sp, #0x38]
100362ea0: b0007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362ea4: 9133f442    	add	x2, x2, #0xcfd
100362ea8: d10283a8    	sub	x8, x29, #0xa0
100362eac: 52800063    	mov	w3, #0x3                ; =3
100362eb0: 97fb3b25    	bl	0x100231b44 <__ZN5alloc3str17join_generic_copy17hc3fffb246d6f0b82E>
100362eb4: 3cd603a0    	ldur	q0, [x29, #-0xa0]
100362eb8: 3d801fe0    	str	q0, [sp, #0x70]
100362ebc: f85703a9    	ldur	x9, [x29, #-0x90]
100362ec0: 9101c3e8    	add	x8, sp, #0x70
100362ec4: a90823e9    	stp	x9, x8, [sp, #0x80]
100362ec8: f9004bf5    	str	x21, [sp, #0x90]
100362ecc: b0008de0    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
100362ed0: 91284400    	add	x0, x0, #0xa11
100362ed4: d10283a8    	sub	x8, x29, #0xa0
100362ed8: 910223e1    	add	x1, sp, #0x88
100362edc: 9438526e    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100362ee0: f9403be1    	ldr	x1, [sp, #0x70]
100362ee4: b4000081    	cbz	x1, 0x100362ef4 <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E+0xb4>
100362ee8: f9403fe0    	ldr	x0, [sp, #0x78]
100362eec: 52800022    	mov	w2, #0x1                ; =1
100362ef0: 97f2799e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100362ef4: 3cd603a0    	ldur	q0, [x29, #-0xa0]
100362ef8: 3d801fe0    	str	q0, [sp, #0x70]
100362efc: f85703a8    	ldur	x8, [x29, #-0x90]
100362f00: f90043e8    	str	x8, [sp, #0x80]
100362f04: 14000004    	b	0x100362f14 <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E+0xd4>
100362f08: 52800028    	mov	w8, #0x1                ; =1
100362f0c: a90723ff    	stp	xzr, x8, [sp, #0x70]
100362f10: f90043ff    	str	xzr, [sp, #0x80]
100362f14: 91014288    	add	x8, x20, #0x50
100362f18: f94017e9    	ldr	x9, [sp, #0x28]
100362f1c: 9101628a    	add	x10, x20, #0x58
100362f20: 9101828b    	add	x11, x20, #0x60
100362f24: f90047e9    	str	x9, [sp, #0x88]
100362f28: 910003e9    	mov	x9, sp
100362f2c: 9101a12c    	add	x12, x9, #0x68
100362f30: a93657a9    	stp	x9, x21, [x29, #-0xa0]
100362f34: b00070c9    	adrp	x9, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
100362f38: 910d3129    	add	x9, x9, #0x34c
100362f3c: a93727a8    	stp	x8, x9, [x29, #-0x90]
100362f40: a93827aa    	stp	x10, x9, [x29, #-0x80]
100362f44: a93927ab    	stp	x11, x9, [x29, #-0x70]
100362f48: a93a27ac    	stp	x12, x9, [x29, #-0x60]
100362f4c: 910223e8    	add	x8, sp, #0x88
100362f50: a93b27a8    	stp	x8, x9, [x29, #-0x50]
100362f54: 9101c3e8    	add	x8, sp, #0x70
100362f58: a93c57a8    	stp	x8, x21, [x29, #-0x40]
100362f5c: b0008de0    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
100362f60: 91287800    	add	x0, x0, #0xa1e
100362f64: 910263e8    	add	x8, sp, #0x98
100362f68: d10283a1    	sub	x1, x29, #0xa0
100362f6c: 9438524a    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100362f70: a949d7f4    	ldp	x20, x21, [sp, #0x98]
100362f74: f94057f7    	ldr	x23, [sp, #0xa8]
100362f78: f9400a76    	ldr	x22, [x19, #0x10]
100362f7c: f9400268    	ldr	x8, [x19]
100362f80: eb0802df    	cmp	x22, x8
100362f84: 54000061    	b.ne	0x100362f90 <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E+0x150>
100362f88: aa1303e0    	mov	x0, x19
100362f8c: 9438ab49    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100362f90: f9400668    	ldr	x8, [x19, #0x8]
100362f94: 8b161508    	add	x8, x8, x22, lsl #5
100362f98: a9005514    	stp	x20, x21, [x8]
100362f9c: f9000917    	str	x23, [x8, #0x10]
100362fa0: 52800069    	mov	w9, #0x3                ; =3
100362fa4: 39006109    	strb	w9, [x8, #0x18]
100362fa8: 910006c8    	add	x8, x22, #0x1
100362fac: f9000a68    	str	x8, [x19, #0x10]
100362fb0: f9403be1    	ldr	x1, [sp, #0x70]
100362fb4: b4000081    	cbz	x1, 0x100362fc4 <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E+0x184>
100362fb8: f9403fe0    	ldr	x0, [sp, #0x78]
100362fbc: 52800022    	mov	w2, #0x1                ; =1
100362fc0: 97f2796a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
