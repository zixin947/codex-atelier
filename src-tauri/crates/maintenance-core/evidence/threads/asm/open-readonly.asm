
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008b3938 <__ZN13codexmate_lib4core5relay23codex_thread_visibility22open_codex_db_readonly17hcde6d9c093f2ce44E>:
1008b3938: a9bc6ffc    	stp	x28, x27, [sp, #-0x40]!
1008b393c: a90157f6    	stp	x22, x21, [sp, #0x10]
1008b3940: a9024ff4    	stp	x20, x19, [sp, #0x20]
1008b3944: a9037bfd    	stp	x29, x30, [sp, #0x30]
1008b3948: 9100c3fd    	add	x29, sp, #0x30
1008b394c: d10703ff    	sub	sp, sp, #0x1c0
1008b3950: aa0003f3    	mov	x19, x0
1008b3954: d10283b4    	sub	x20, x29, #0xa0
1008b3958: 910283f6    	add	x22, sp, #0xa0
1008b395c: d10283a0    	sub	x0, x29, #0xa0
1008b3960: 52900023    	mov	w3, #0x8001             ; =32769
1008b3964: 94017f87    	bl	0x100913780 <__ZN8rusqlite10Connection15open_with_flags17h6c8f17fb55b23d77E>
1008b3968: 385c83a8    	ldurb	w8, [x29, #-0x38]
1008b396c: 3ccb82c2    	ldur	q2, [x22, #0xb8]
1008b3970: f85603b5    	ldur	x21, [x29, #-0xa0]
1008b3974: f85783a9    	ldur	x9, [x29, #-0x88]
1008b3978: f81503a9    	stur	x9, [x29, #-0xb0]
1008b397c: ad7c07a0    	ldp	q0, q1, [x29, #-0x80]
1008b3980: 3c9203a0    	stur	q0, [x29, #-0xe0]
1008b3984: ad398ba1    	stp	q1, q2, [x29, #-0xd0]
1008b3988: 71000d1f    	cmp	w8, #0x3
1008b398c: 54000101    	b.ne	0x1008b39ac <__ZN13codexmate_lib4core5relay23codex_thread_visibility22open_codex_db_readonly17hcde6d9c093f2ce44E+0x74>
1008b3990: ad7983a1    	ldp	q1, q0, [x29, #-0xd0]
1008b3994: 3d8023e0    	str	q0, [sp, #0x80]
1008b3998: f85503a8    	ldur	x8, [x29, #-0xb0]
1008b399c: f9004be8    	str	x8, [sp, #0x90]
1008b39a0: 3cd203a0    	ldur	q0, [x29, #-0xe0]
1008b39a4: ad0207e0    	stp	q0, q1, [sp, #0x40]
1008b39a8: 1400004b    	b	0x1008b3ad4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility22open_codex_db_readonly17hcde6d9c093f2ce44E+0x19c>
1008b39ac: ad7d07a0    	ldp	q0, q1, [x29, #-0x60]
1008b39b0: ad0707e0    	stp	q0, q1, [sp, #0xe0]
1008b39b4: f85c03a9    	ldur	x9, [x29, #-0x40]
1008b39b8: f90083e9    	str	x9, [sp, #0x100]
1008b39bc: b8469289    	ldur	w9, [x20, #0x69]
1008b39c0: b80692c9    	stur	w9, [x22, #0x69]
1008b39c4: b85cc3a9    	ldur	w9, [x29, #-0x34]
1008b39c8: b9010fe9    	str	w9, [sp, #0x10c]
1008b39cc: ad7983a1    	ldp	q1, q0, [x29, #-0xd0]
1008b39d0: 3c8082c0    	stur	q0, [x22, #0x8]
1008b39d4: f85503a9    	ldur	x9, [x29, #-0xb0]
1008b39d8: f90053f5    	str	x21, [sp, #0xa0]
1008b39dc: f9005fe9    	str	x9, [sp, #0xb8]
1008b39e0: 3cd203a0    	ldur	q0, [x29, #-0xe0]
1008b39e4: ad0607e0    	stp	q0, q1, [sp, #0xc0]
1008b39e8: 390423e8    	strb	w8, [sp, #0x108]
1008b39ec: d10283a8    	sub	x8, x29, #0xa0
1008b39f0: 910283e0    	add	x0, sp, #0xa0
1008b39f4: 528000a1    	mov	w1, #0x5                ; =5
1008b39f8: 52800002    	mov	w2, #0x0                ; =0
1008b39fc: 941339fe    	bl	0x100d821f4 <__ZN8rusqlite4busy38_$LT$impl$u20$rusqlite..Connection$GT$12busy_timeout17h5dd4f2f9ca09a7d4E>
1008b3a00: f85603b5    	ldur	x21, [x29, #-0xa0]
1008b3a04: d28002c8    	mov	x8, #0x16               ; =22
1008b3a08: f2f00008    	movk	x8, #0x8000, lsl #48
1008b3a0c: eb0802bf    	cmp	x21, x8
1008b3a10: 54000521    	b.ne	0x1008b3ab4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility22open_codex_db_readonly17hcde6d9c093f2ce44E+0x17c>
1008b3a14: 3cc082c2    	ldur	q2, [x22, #0x8]
1008b3a18: f84182c8    	ldur	x8, [x22, #0x18]
1008b3a1c: f9004be8    	str	x8, [sp, #0x90]
1008b3a20: ad4106c0    	ldp	q0, q1, [x22, #0x20]
1008b3a24: ad0207e0    	stp	q0, q1, [sp, #0x40]
1008b3a28: ad4206c0    	ldp	q0, q1, [x22, #0x40]
1008b3a2c: 3d801be0    	str	q0, [sp, #0x60]
1008b3a30: ad038be1    	stp	q1, q2, [sp, #0x70]
1008b3a34: f94053f5    	ldr	x21, [sp, #0xa0]
1008b3a38: f94083e8    	ldr	x8, [sp, #0x100]
1008b3a3c: 394423e9    	ldrb	w9, [sp, #0x108]
1008b3a40: b84692ca    	ldur	w10, [x22, #0x69]
1008b3a44: b846c2cb    	ldur	w11, [x22, #0x6c]
1008b3a48: b803b3eb    	stur	w11, [sp, #0x3b]
1008b3a4c: b9003bea    	str	w10, [sp, #0x38]
1008b3a50: 71000d3f    	cmp	w9, #0x3
1008b3a54: 54000400    	b.eq	0x1008b3ad4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility22open_codex_db_readonly17hcde6d9c093f2ce44E+0x19c>
1008b3a58: 3dc023e0    	ldr	q0, [sp, #0x80]
1008b3a5c: 3d800be0    	str	q0, [sp, #0x20]
1008b3a60: f9404bea    	ldr	x10, [sp, #0x90]
1008b3a64: f9001bea    	str	x10, [sp, #0x30]
1008b3a68: ad4207e0    	ldp	q0, q1, [sp, #0x40]
1008b3a6c: ad010660    	stp	q0, q1, [x19, #0x20]
1008b3a70: ad4307e0    	ldp	q0, q1, [sp, #0x60]
1008b3a74: ad020660    	stp	q0, q1, [x19, #0x40]
1008b3a78: b9403bea    	ldr	w10, [sp, #0x38]
1008b3a7c: b806926a    	stur	w10, [x19, #0x69]
1008b3a80: b843b3ea    	ldur	w10, [sp, #0x3b]
1008b3a84: b9006e6a    	str	w10, [x19, #0x6c]
1008b3a88: 3dc00be0    	ldr	q0, [sp, #0x20]
1008b3a8c: f9401bea    	ldr	x10, [sp, #0x30]
1008b3a90: f9000bea    	str	x10, [sp, #0x10]
1008b3a94: 3d8003e0    	str	q0, [sp]
1008b3a98: 3dc003e0    	ldr	q0, [sp]
1008b3a9c: 3c808260    	stur	q0, [x19, #0x8]
1008b3aa0: f9000e6a    	str	x10, [x19, #0x18]
1008b3aa4: f9000275    	str	x21, [x19]
1008b3aa8: f9003268    	str	x8, [x19, #0x60]
1008b3aac: 3901a269    	strb	w9, [x19, #0x68]
1008b3ab0: 14000029    	b	0x1008b3b54 <__ZN13codexmate_lib4core5relay23codex_thread_visibility22open_codex_db_readonly17hcde6d9c093f2ce44E+0x21c>
1008b3ab4: 3cc08280    	ldur	q0, [x20, #0x8]
1008b3ab8: 3d8023e0    	str	q0, [sp, #0x80]
1008b3abc: f85783a8    	ldur	x8, [x29, #-0x88]
1008b3ac0: f9004be8    	str	x8, [sp, #0x90]
1008b3ac4: ad7c07a0    	ldp	q0, q1, [x29, #-0x80]
1008b3ac8: ad0207e0    	stp	q0, q1, [sp, #0x40]
1008b3acc: 910283e0    	add	x0, sp, #0xa0
1008b3ad0: 94010a23    	bl	0x1008f635c <__ZN4core3ptr41drop_in_place$LT$rusqlite..Connection$GT$17h7d7dc73024fb556fE>
1008b3ad4: 3dc023e0    	ldr	q0, [sp, #0x80]
1008b3ad8: 3c808280    	stur	q0, [x20, #0x8]
1008b3adc: f9404be8    	ldr	x8, [sp, #0x90]
1008b3ae0: ad4207e0    	ldp	q0, q1, [sp, #0x40]
1008b3ae4: ad3c07a0    	stp	q0, q1, [x29, #-0x80]
1008b3ae8: f81783a8    	stur	x8, [x29, #-0x88]
1008b3aec: f81603b5    	stur	x21, [x29, #-0xa0]
1008b3af0: d10283a8    	sub	x8, x29, #0xa0
1008b3af4: b0002669    	adrp	x9, 0x100d80000 <__ZN63_$LT$httpdate..date..HttpDate$u20$as$u20$core..fmt..Display$GT$3fmt17h5c6b37a265892b57E+0x1b4>
1008b3af8: 9119b129    	add	x9, x9, #0x66c
1008b3afc: a93227a8    	stp	x8, x9, [x29, #-0xe0]
1008b3b00: 900063e0    	adrp	x0, 0x10152f000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.17+0x520>
1008b3b04: 913d4c00    	add	x0, x0, #0xf53
1008b3b08: 910283e8    	add	x8, sp, #0xa0
1008b3b0c: d10383a1    	sub	x1, x29, #0xe0
1008b3b10: 94230f61    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1008b3b14: 3dc02be0    	ldr	q0, [sp, #0xa0]
1008b3b18: 3d800be0    	str	q0, [sp, #0x20]
1008b3b1c: f9405be8    	ldr	x8, [sp, #0xb0]
1008b3b20: f9001be8    	str	x8, [sp, #0x30]
1008b3b24: d10283a0    	sub	x0, x29, #0xa0
1008b3b28: 94010b13    	bl	0x1008f6774 <__ZN4core3ptr43drop_in_place$LT$rusqlite..error..Error$GT$17h09391278bf407148E>
1008b3b2c: f9401be8    	ldr	x8, [sp, #0x30]
1008b3b30: f9000be8    	str	x8, [sp, #0x10]
1008b3b34: 3dc00be0    	ldr	q0, [sp, #0x20]
1008b3b38: 3d8003e0    	str	q0, [sp]
1008b3b3c: 3c808260    	stur	q0, [x19, #0x8]
1008b3b40: f9000e68    	str	x8, [x19, #0x18]
1008b3b44: 52800148    	mov	w8, #0xa                ; =10
1008b3b48: f9000268    	str	x8, [x19]
1008b3b4c: 52800068    	mov	w8, #0x3                ; =3
1008b3b50: 3901a268    	strb	w8, [x19, #0x68]
