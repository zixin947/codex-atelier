
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010033b7fc <__ZN13codexmate_lib4core5relay17router_transition26TransitionOutcome$LT$T$GT$19into_verified_parts17h9766906545078af3E>:
10033b8c8: aa0803f8    	mov	x24, x8
10033b8cc: a940d276    	ldp	x22, x20, [x19, #0x8]
10033b8d0: f9400e75    	ldr	x21, [x19, #0x18]
10033b8d4: d0007e62    	adrp	x2, 0x101309000 <_anon.e143397d1bdb1baba839fd20a45cc9a3.246+0x390>
10033b8d8: 9123d042    	add	x2, x2, #0x8f4
10033b8dc: 910003e8    	mov	x8, sp
10033b8e0: aa1403e0    	mov	x0, x20
10033b8e4: aa1503e1    	mov	x1, x21
10033b8e8: 52800063    	mov	w3, #0x3                ; =3
10033b8ec: 97fbd896    	bl	0x100231b44 <__ZN5alloc3str17join_generic_copy17hc3fffb246d6f0b82E>
10033b8f0: 3dc003e0    	ldr	q0, [sp]
10033b8f4: 3c9a03a0    	stur	q0, [x29, #-0x60]
10033b8f8: f9400be8    	ldr	x8, [sp, #0x10]
10033b8fc: f81b03a8    	stur	x8, [x29, #-0x50]
10033b900: d10183a8    	sub	x8, x29, #0x60
10033b904: d00000a9    	adrp	x9, 0x100351000 <__ZN5tokio7runtime4park16CachedParkThread8block_on17hcd3d4f5ed312ff0aE+0x1c0>
10033b908: 91092129    	add	x9, x9, #0x248
10033b90c: a90027e8    	stp	x8, x9, [sp]
10033b910: 90008f20    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
10033b914: 910a5000    	add	x0, x0, #0x294
10033b918: d101e3a8    	sub	x8, x29, #0x78
10033b91c: 910003e1    	mov	x1, sp
10033b920: 9438efdd    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10033b924: f85a03a1    	ldur	x1, [x29, #-0x60]
10033b928: b4000081    	cbz	x1, 0x10033b938 <__ZN13codexmate_lib4core5relay17router_transition26TransitionOutcome$LT$T$GT$19into_verified_parts17h9766906545078af3E+0x13c>
10033b92c: f85a83a0    	ldur	x0, [x29, #-0x58]
10033b930: 52800022    	mov	w2, #0x1                ; =1
10033b934: 97f3170d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10033b938: 3cca82e0    	ldur	q0, [x23, #0xa8]
10033b93c: f85983a8    	ldur	x8, [x29, #-0x68]
10033b940: f9001308    	str	x8, [x24, #0x20]
10033b944: d0007e68    	adrp	x8, 0x101309000 <_anon.e143397d1bdb1baba839fd20a45cc9a3.246+0x390>
10033b948: 3dc0ad01    	ldr	q1, [x8, #0x2b0]
10033b94c: ad000301    	stp	q1, q0, [x24]
