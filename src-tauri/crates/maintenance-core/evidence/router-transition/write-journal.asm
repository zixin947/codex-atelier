
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100325f24 <__ZN13codexmate_lib4core5relay17router_transition13write_journal17h224a0b13baabb3b9E>:
100325f24: d10443ff    	sub	sp, sp, #0x110
100325f28: a90f4ff4    	stp	x20, x19, [sp, #0xf0]
100325f2c: a9107bfd    	stp	x29, x30, [sp, #0x100]
100325f30: 910403fd    	add	x29, sp, #0x100
100325f34: aa0603e8    	mov	x8, x6
100325f38: aa0503e6    	mov	x6, x5
100325f3c: aa0403e5    	mov	x5, x4
100325f40: aa0303e4    	mov	x4, x3
100325f44: aa0203e3    	mov	x3, x2
100325f48: aa0103e2    	mov	x2, x1
100325f4c: aa0003e1    	mov	x1, x0
100325f50: 390023ff    	strb	wzr, [sp, #0x8]
100325f54: f90003e7    	str	x7, [sp]
100325f58: d101e3b3    	sub	x19, x29, #0x78
100325f5c: d101e3a0    	sub	x0, x29, #0x78
100325f60: aa0803e7    	mov	x7, x8
100325f64: 9411a0bc    	bl	0x10078e254 <__ZN13codexmate_lib4core5relay18transition_journal11write_inner17h9acef3d2661da72dE>
100325f68: f85883a8    	ldur	x8, [x29, #-0x78]
100325f6c: f1002d1f    	cmp	x8, #0xb
100325f70: 54000081    	b.ne	0x100325f80 <__ZN13codexmate_lib4core5relay17router_transition13write_journal17h224a0b13baabb3b9E+0x5c>
100325f74: f85903a0    	ldur	x0, [x29, #-0x70]
100325f78: 9415fbc0    	bl	0x1008a4e78 <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E>
100325f7c: 1400002f    	b	0x100326038 <__ZN13codexmate_lib4core5relay17router_transition13write_journal17h224a0b13baabb3b9E+0x114>
100325f80: 3cc28261    	ldur	q1, [x19, #0x28]
100325f84: 3cc38260    	ldur	q0, [x19, #0x38]
100325f88: ad0203e1    	stp	q1, q0, [sp, #0x40]
100325f8c: 3cc48260    	ldur	q0, [x19, #0x48]
100325f90: 3c8603e0    	stur	q0, [sp, #0x60]
100325f94: f8458269    	ldur	x9, [x19, #0x58]
100325f98: f9003be9    	str	x9, [sp, #0x70]
100325f9c: 3cc08261    	ldur	q1, [x19, #0x8]
100325fa0: 3cc18260    	ldur	q0, [x19, #0x18]
100325fa4: ad0103e1    	stp	q1, q0, [sp, #0x20]
100325fa8: f9000fe8    	str	x8, [sp, #0x18]
100325fac: b000ae68    	adrp	x8, 0x1018f2000 <__ZN13codexmate_lib13EXIT_APPROVED17h1f9ee2eac97195fdE.0>
100325fb0: 910fe108    	add	x8, x8, #0x3f8
100325fb4: f9400108    	ldr	x8, [x8]
100325fb8: f100051f    	cmp	x8, #0x1
100325fbc: 540003a9    	b.ls	0x100326030 <__ZN13codexmate_lib4core5relay17router_transition13write_journal17h224a0b13baabb3b9E+0x10c>
100325fc0: f00026a8    	adrp	x8, 0x1007fc000 <__ZN6chrono8datetime40DateTime$LT$chrono..offset..utc..Utc$GT$14from_timestamp17h2b34124b50daed04E+0x8>
100325fc4: 91343108    	add	x8, x8, #0xd0c
100325fc8: 910063e9    	add	x9, sp, #0x18
100325fcc: a907a3e9    	stp	x9, x8, [sp, #0x78]
100325fd0: 90007f28    	adrp	x8, 0x101309000 <_anon.e143397d1bdb1baba839fd20a45cc9a3.246+0x390>
100325fd4: 9122a908    	add	x8, x8, #0x8aa
100325fd8: 528005a9    	mov	w9, #0x2d               ; =45
100325fdc: b0008fca    	adrp	x10, 0x10151e000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.1109+0x11f>
100325fe0: 913b494a    	add	x10, x10, #0xed2
100325fe4: a93c27a8    	stp	x8, x9, [x29, #-0x40]
100325fe8: 9101e3e9    	add	x9, sp, #0x78
100325fec: a93da7aa    	stp	x10, x9, [x29, #-0x28]
100325ff0: a938a3bf    	stp	xzr, x8, [x29, #-0x78]
100325ff4: 90007f28    	adrp	x8, 0x101309000 <_anon.e143397d1bdb1baba839fd20a45cc9a3.246+0x390>
100325ff8: 3dc09900    	ldr	q0, [x8, #0x260]
100325ffc: 3c9983a0    	stur	q0, [x29, #-0x68]
100326000: 90008fc8    	adrp	x8, 0x10151e000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.1109+0x11f>
100326004: 913c1d08    	add	x8, x8, #0xf07
100326008: f81a83a8    	stur	x8, [x29, #-0x58]
10032600c: f0007f08    	adrp	x8, 0x101309000 <_anon.e143397d1bdb1baba839fd20a45cc9a3.246+0x390>
100326010: 3dc09d00    	ldr	q0, [x8, #0x270]
100326014: 3c9b03a0    	stur	q0, [x29, #-0x50]
100326018: f0007f08    	adrp	x8, 0x101309000 <_anon.e143397d1bdb1baba839fd20a45cc9a3.246+0x390>
10032601c: fd41b900    	ldr	d0, [x8, #0x370]
100326020: fc1d03a0    	stur	d0, [x29, #-0x30]
100326024: d10047a0    	sub	x0, x29, #0x11
100326028: d101e3a1    	sub	x1, x29, #0x78
10032602c: 9432bb1d    	bl	0x100fd4ca0 <__ZN61_$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$3log17he2f3dc859ac0f3c8E>
100326030: 910063e0    	add	x0, sp, #0x18
100326034: 94008e0d    	bl	0x100349868 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
100326038: a9507bfd    	ldp	x29, x30, [sp, #0x100]
10032603c: a94f4ff4    	ldp	x20, x19, [sp, #0xf0]
100326040: 910443ff    	add	sp, sp, #0x110
100326044: d65f03c0    	ret
100326048: aa0003f3    	mov	x19, x0
10032604c: 910063e0    	add	x0, sp, #0x18
