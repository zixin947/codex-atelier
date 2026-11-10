
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100791a40 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE>:
100791a40: d10403ff    	sub	sp, sp, #0x100
100791a44: a90a6ffc    	stp	x28, x27, [sp, #0xa0]
100791a48: a90b67fa    	stp	x26, x25, [sp, #0xb0]
100791a4c: a90c5ff8    	stp	x24, x23, [sp, #0xc0]
100791a50: a90d57f6    	stp	x22, x21, [sp, #0xd0]
100791a54: a90e4ff4    	stp	x20, x19, [sp, #0xe0]
100791a58: a90f7bfd    	stp	x29, x30, [sp, #0xf0]
100791a5c: 9103c3fd    	add	x29, sp, #0xf0
100791a60: aa0503f9    	mov	x25, x5
100791a64: aa0403f4    	mov	x20, x4
100791a68: aa0303f5    	mov	x21, x3
100791a6c: aa0203f6    	mov	x22, x2
100791a70: aa0103f7    	mov	x23, x1
100791a74: aa0003f8    	mov	x24, x0
100791a78: aa0803f3    	mov	x19, x8
100791a7c: 9100e3e8    	add	x8, sp, #0x38
100791a80: 97fffc3a    	bl	0x100790b68 <__ZN13codexmate_lib4core5relay18transition_journal12read_checked17hd86b1646f346cffbE>
100791a84: a94387e8    	ldp	x8, x1, [sp, #0x38]
100791a88: f94027e0    	ldr	x0, [sp, #0x48]
100791a8c: f1002d1f    	cmp	x8, #0xb
100791a90: 54000441    	b.ne	0x100791b18 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0xd8>
100791a94: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100791a98: eb08003f    	cmp	x1, x8
100791a9c: 54000140    	b.eq	0x100791ac4 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x84>
100791aa0: a945effa    	ldp	x26, x27, [sp, #0x58]
100791aa4: 394203fc    	ldrb	w28, [sp, #0x80]
100791aa8: b4000061    	cbz	x1, 0x100791ab4 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x74>
100791aac: 52800022    	mov	w2, #0x1                ; =1
100791ab0: 97e1beae    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100791ab4: f240fb5f    	tst	x26, #0x7fffffffffffffff
100791ab8: 54000741    	b.ne	0x100791ba0 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x160>
100791abc: 6b17039f    	cmp	w28, w23
100791ac0: 540007c1    	b.ne	0x100791bb8 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x178>
100791ac4: 52800028    	mov	w8, #0x1                ; =1
100791ac8: 390023e8    	strb	w8, [sp, #0x8]
100791acc: f90003f9    	str	x25, [sp]
100791ad0: b00060c3    	adrp	x3, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100791ad4: 913b2063    	add	x3, x3, #0xec8
100791ad8: 9100e3e0    	add	x0, sp, #0x38
100791adc: aa1803e1    	mov	x1, x24
100791ae0: aa1703e2    	mov	x2, x23
100791ae4: 52800104    	mov	w4, #0x8                ; =8
100791ae8: aa1603e5    	mov	x5, x22
100791aec: aa1503e6    	mov	x6, x21
100791af0: aa1403e7    	mov	x7, x20
100791af4: 97fff1d8    	bl	0x10078e254 <__ZN13codexmate_lib4core5relay18transition_journal11write_inner17h9acef3d2661da72dE>
100791af8: f9401fe8    	ldr	x8, [sp, #0x38]
100791afc: f1002d1f    	cmp	x8, #0xb
100791b00: 540003c1    	b.ne	0x100791b78 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x138>
100791b04: f94023e0    	ldr	x0, [sp, #0x40]
100791b08: 94044cdc    	bl	0x1008a4e78 <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E>
100791b0c: 52800168    	mov	w8, #0xb                ; =11
100791b10: f9000268    	str	x8, [x19]
100791b14: 14000041    	b	0x100791c18 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x1d8>
100791b18: f94033e9    	ldr	x9, [sp, #0x60]
100791b1c: 3cc683e0    	ldur	q0, [sp, #0x68]
100791b20: 3d800be0    	str	q0, [sp, #0x20]
100791b24: f9403fea    	ldr	x10, [sp, #0x78]
100791b28: f9001bea    	str	x10, [sp, #0x30]
100791b2c: 394203eb    	ldrb	w11, [sp, #0x80]
100791b30: b84813ec    	ldur	w12, [sp, #0x81]
100791b34: b9001bec    	str	w12, [sp, #0x18]
100791b38: b94087ec    	ldr	w12, [sp, #0x84]
100791b3c: b801b3ec    	stur	w12, [sp, #0x1b]
100791b40: 3cc883e1    	ldur	q1, [sp, #0x88]
100791b44: 3d801661    	str	q1, [x19, #0x50]
100791b48: 3cc503e1    	ldur	q1, [sp, #0x50]
100791b4c: a9008261    	stp	x1, x0, [x19, #0x8]
100791b50: 3c818261    	stur	q1, [x19, #0x18]
100791b54: f9001669    	str	x9, [x19, #0x28]
100791b58: 3d800e60    	str	q0, [x19, #0x30]
100791b5c: f900226a    	str	x10, [x19, #0x40]
100791b60: 3901226b    	strb	w11, [x19, #0x48]
100791b64: b9401be9    	ldr	w9, [sp, #0x18]
100791b68: b8049269    	stur	w9, [x19, #0x49]
100791b6c: b9004e6c    	str	w12, [x19, #0x4c]
100791b70: f9000268    	str	x8, [x19]
100791b74: 14000029    	b	0x100791c18 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x1d8>
100791b78: 3cc583e0    	ldur	q0, [sp, #0x58]
100791b7c: 3cc683e1    	ldur	q1, [sp, #0x68]
100791b80: ad010660    	stp	q0, q1, [x19, #0x20]
100791b84: 3cc783e0    	ldur	q0, [sp, #0x78]
100791b88: 3cc883e1    	ldur	q1, [sp, #0x88]
100791b8c: ad020660    	stp	q0, q1, [x19, #0x40]
100791b90: 3cc383e0    	ldur	q0, [sp, #0x38]
100791b94: 3cc483e1    	ldur	q1, [sp, #0x48]
100791b98: ad000660    	stp	q0, q1, [x19]
100791b9c: 1400001f    	b	0x100791c18 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x1d8>
100791ba0: aa1b03e0    	mov	x0, x27
100791ba4: aa1a03e1    	mov	x1, x26
100791ba8: 52800022    	mov	w2, #0x1                ; =1
100791bac: 97e1be6f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100791bb0: 6b17039f    	cmp	w28, w23
100791bb4: 54fff880    	b.eq	0x100791ac4 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x84>
100791bb8: 97e1be6f    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100791bbc: 52801540    	mov	w0, #0xaa               ; =170
100791bc0: 52800021    	mov	w1, #0x1                ; =1
100791bc4: 97e1be68    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100791bc8: b4000380    	cbz	x0, 0x100791c38 <__ZN13codexmate_lib4core5relay18transition_journal17warn_after_commit17haf3f068f036d297fE+0x1f8>
100791bcc: b00060c8    	adrp	x8, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100791bd0: 913d9508    	add	x8, x8, #0xf65
100791bd4: ad440500    	ldp	q0, q1, [x8, #0x80]
100791bd8: ad040400    	stp	q0, q1, [x0, #0x80]
100791bdc: 3cc9a100    	ldur	q0, [x8, #0x9a]
100791be0: 3c89a000    	stur	q0, [x0, #0x9a]
100791be4: ad420500    	ldp	q0, q1, [x8, #0x40]
100791be8: ad020400    	stp	q0, q1, [x0, #0x40]
100791bec: ad430101    	ldp	q1, q0, [x8, #0x60]
100791bf0: ad030001    	stp	q1, q0, [x0, #0x60]
100791bf4: ad400500    	ldp	q0, q1, [x8]
100791bf8: ad000400    	stp	q0, q1, [x0]
100791bfc: ad410101    	ldp	q1, q0, [x8, #0x20]
100791c00: ad010001    	stp	q1, q0, [x0, #0x20]
100791c04: 90005bc8    	adrp	x8, 0x101309000 <_anon.e143397d1bdb1baba839fd20a45cc9a3.246+0x390>
100791c08: 3dc0a100    	ldr	q0, [x8, #0x280]
100791c0c: 3d800260    	str	q0, [x19]
100791c10: 52801548    	mov	w8, #0xaa               ; =170
100791c14: a9012260    	stp	x0, x8, [x19, #0x10]
100791c18: a94f7bfd    	ldp	x29, x30, [sp, #0xf0]
100791c1c: a94e4ff4    	ldp	x20, x19, [sp, #0xe0]
100791c20: a94d57f6    	ldp	x22, x21, [sp, #0xd0]
100791c24: a94c5ff8    	ldp	x24, x23, [sp, #0xc0]
100791c28: a94b67fa    	ldp	x26, x25, [sp, #0xb0]
100791c2c: a94a6ffc    	ldp	x28, x27, [sp, #0xa0]
100791c30: 910403ff    	add	sp, sp, #0x100
100791c34: d65f03c0    	ret
