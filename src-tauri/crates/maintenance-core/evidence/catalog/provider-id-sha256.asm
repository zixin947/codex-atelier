
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100222a48 <__ZN44_$LT$D$u20$as$u20$digest..digest..Digest$GT$6digest17h552c414baf0732ceE>:
100222a48: d10583ff    	sub	sp, sp, #0x160
100222a4c: a9125ff8    	stp	x24, x23, [sp, #0x120]
100222a50: a91357f6    	stp	x22, x21, [sp, #0x130]
100222a54: a9144ff4    	stp	x20, x19, [sp, #0x140]
100222a58: a9157bfd    	stp	x29, x30, [sp, #0x150]
100222a5c: 910543fd    	add	x29, sp, #0x150
100222a60: aa0203f4    	mov	x20, x2
100222a64: aa0003f3    	mov	x19, x0
100222a68: 910003f6    	mov	x22, sp
100222a6c: 3901a3ff    	strb	wzr, [sp, #0x68]
100222a70: 6f00e400    	movi.2d	v0, #0000000000000000
100222a74: 3c8583e0    	stur	q0, [sp, #0x58]
100222a78: 3c8483e0    	stur	q0, [sp, #0x48]
100222a7c: 3c8383e0    	stur	q0, [sp, #0x38]
100222a80: 3c8283e0    	stur	q0, [sp, #0x28]
100222a84: d0008728    	adrp	x8, 0x101308000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.1087+0x3d>
100222a88: 91074108    	add	x8, x8, #0x1d0
100222a8c: ad400500    	ldp	q0, q1, [x8]
100222a90: ad0007e0    	stp	q0, q1, [sp]
100222a94: f90013ff    	str	xzr, [sp, #0x20]
100222a98: f101005f    	cmp	x2, #0x40
100222a9c: 540000a2    	b.hs	0x100222ab0 <__ZN44_$LT$D$u20$as$u20$digest..digest..Digest$GT$6digest17h552c414baf0732ceE+0x68>
100222aa0: 9100a2c0    	add	x0, x22, #0x28
100222aa4: aa1403e2    	mov	x2, x20
100222aa8: 943ebb01    	bl	0x1011d16ac <_writev+0x1011d16ac>
100222aac: 1400000d    	b	0x100222ae0 <__ZN44_$LT$D$u20$as$u20$digest..digest..Digest$GT$6digest17h552c414baf0732ceE+0x98>
100222ab0: 927ae297    	and	x23, x20, #0x7fffffffffffffc0
100222ab4: 92401695    	and	x21, x20, #0x3f
100222ab8: d346fe82    	lsr	x2, x20, #6
100222abc: f90013e2    	str	x2, [sp, #0x20]
100222ac0: 910003e0    	mov	x0, sp
100222ac4: aa0103f4    	mov	x20, x1
100222ac8: 9433bd33    	bl	0x100f11f94 <__ZN4sha26sha25611compress25617h3787580ae4296a35E>
100222acc: 9100a2c0    	add	x0, x22, #0x28
100222ad0: 8b170281    	add	x1, x20, x23
100222ad4: aa1503e2    	mov	x2, x21
100222ad8: 943ebaf5    	bl	0x1011d16ac <_writev+0x1011d16ac>
100222adc: aa1503f4    	mov	x20, x21
100222ae0: 3901a3f4    	strb	w20, [sp, #0x68]
100222ae4: 3dc01be2    	ldr	q2, [sp, #0x60]
100222ae8: ad4107e0    	ldp	q0, q1, [sp, #0x20]
100222aec: ad0487e0    	stp	q0, q1, [sp, #0x90]
100222af0: ad4207e0    	ldp	q0, q1, [sp, #0x40]
100222af4: 3d802fe0    	str	q0, [sp, #0xb0]
100222af8: ad060be1    	stp	q1, q2, [sp, #0xc0]
100222afc: ad4007e0    	ldp	q0, q1, [sp]
100222b00: ad0387e0    	stp	q0, q1, [sp, #0x70]
100222b04: 9101c3e8    	add	x8, sp, #0x70
100222b08: 9100a114    	add	x20, x8, #0x28
100222b0c: 394363f6    	ldrb	w22, [sp, #0xd8]
100222b10: f9404be8    	ldr	x8, [sp, #0x90]
100222b14: d377d908    	lsl	x8, x8, #9
100222b18: aa160d08    	orr	x8, x8, x22, lsl #3
100222b1c: dac00d15    	rev	x21, x8
100222b20: 8b160288    	add	x8, x20, x22
100222b24: 52801009    	mov	w9, #0x80               ; =128
100222b28: 39000109    	strb	w9, [x8]
100222b2c: 7100fedf    	cmp	w22, #0x3f
100222b30: 540000e0    	b.eq	0x100222b4c <__ZN44_$LT$D$u20$as$u20$digest..digest..Digest$GT$6digest17h552c414baf0732ceE+0x104>
100222b34: 91000500    	add	x0, x8, #0x1
100222b38: d24016c1    	eor	x1, x22, #0x3f
100222b3c: 943eb9c8    	bl	0x1011d125c <_writev+0x1011d125c>
100222b40: d27d0ac8    	eor	x8, x22, #0x38
100222b44: f1001d1f    	cmp	x8, #0x7
100222b48: 54000188    	b.hi	0x100222b78 <__ZN44_$LT$D$u20$as$u20$digest..digest..Digest$GT$6digest17h552c414baf0732ceE+0x130>
100222b4c: 9101c3e0    	add	x0, sp, #0x70
100222b50: aa1403e1    	mov	x1, x20
100222b54: 52800022    	mov	w2, #0x1                ; =1
100222b58: 9433bd0f    	bl	0x100f11f94 <__ZN4sha26sha25611compress25617h3787580ae4296a35E>
100222b5c: 6f00e400    	movi.2d	v0, #0000000000000000
100222b60: ad3d03a0    	stp	q0, q0, [x29, #-0x60]
100222b64: 3c9903a0    	stur	q0, [x29, #-0x70]
100222b68: a93c57bf    	stp	xzr, x21, [x29, #-0x40]
100222b6c: 9101c3e0    	add	x0, sp, #0x70
100222b70: d101c3a1    	sub	x1, x29, #0x70
100222b74: 14000004    	b	0x100222b84 <__ZN44_$LT$D$u20$as$u20$digest..digest..Digest$GT$6digest17h552c414baf0732ceE+0x13c>
100222b78: f9006bf5    	str	x21, [sp, #0xd0]
100222b7c: 9101c3e0    	add	x0, sp, #0x70
100222b80: aa1403e1    	mov	x1, x20
100222b84: 52800022    	mov	w2, #0x1                ; =1
100222b88: 9433bd03    	bl	0x100f11f94 <__ZN4sha26sha25611compress25617h3787580ae4296a35E>
100222b8c: ad4387e0    	ldp	q0, q1, [sp, #0x70]
100222b90: 6e200800    	rev32.16b	v0, v0
100222b94: 6e200821    	rev32.16b	v1, v1
100222b98: ad000660    	stp	q0, q1, [x19]
100222b9c: a9557bfd    	ldp	x29, x30, [sp, #0x150]
100222ba0: a9544ff4    	ldp	x20, x19, [sp, #0x140]
100222ba4: a95357f6    	ldp	x22, x21, [sp, #0x130]
100222ba8: a9525ff8    	ldp	x24, x23, [sp, #0x120]
100222bac: 910583ff    	add	sp, sp, #0x160
100222bb0: d65f03c0    	ret

0000000100222bb4 <__ZN44_$LT$D$u20$as$u20$digest..digest..Digest$GT$6digest17h98333f15241ef626E>:
100222bb4: d105c3ff    	sub	sp, sp, #0x170
100222bb8: a9126ffc    	stp	x28, x27, [sp, #0x120]
100222bbc: a9135ff8    	stp	x24, x23, [sp, #0x130]
100222bc0: a91457f6    	stp	x22, x21, [sp, #0x140]
100222bc4: a9154ff4    	stp	x20, x19, [sp, #0x150]
100222bc8: a9167bfd    	stp	x29, x30, [sp, #0x160]
100222bcc: 910583fd    	add	x29, sp, #0x160
