
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100374940 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE>:
100374940: 6db923e9    	stp	d9, d8, [sp, #-0x70]!
100374944: a9016ffc    	stp	x28, x27, [sp, #0x10]
100374948: a90267fa    	stp	x26, x25, [sp, #0x20]
10037494c: a9035ff8    	stp	x24, x23, [sp, #0x30]
100374950: a90457f6    	stp	x22, x21, [sp, #0x40]
100374954: a9054ff4    	stp	x20, x19, [sp, #0x50]
100374958: a9067bfd    	stp	x29, x30, [sp, #0x60]
10037495c: 910183fd    	add	x29, sp, #0x60
100374960: aa0103f4    	mov	x20, x1
100374964: aa0003f3    	mov	x19, x0
100374968: f100583f    	cmp	x1, #0x16
10037496c: 54001100    	b.eq	0x100374b8c <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x24c>
100374970: f1003a9f    	cmp	x20, #0xe
100374974: 54000720    	b.eq	0x100374a58 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x118>
100374978: f1001a9f    	cmp	x20, #0x6
10037497c: 54000881    	b.ne	0x100374a8c <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x14c>
100374980: 39400269    	ldrb	w9, [x19]
100374984: 5101052a    	sub	w10, w9, #0x41
100374988: 52800408    	mov	w8, #0x20               ; =32
10037498c: 7100695f    	cmp	w10, #0x1a
100374990: 1a9f310a    	csel	w10, w8, wzr, lo
100374994: 2a090149    	orr	w9, w10, w9
100374998: 7101bd3f    	cmp	w9, #0x6f
10037499c: 540004c1    	b.ne	0x100374a34 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0xf4>
1003749a0: 39400669    	ldrb	w9, [x19, #0x1]
1003749a4: 5101052a    	sub	w10, w9, #0x41
1003749a8: 7100695f    	cmp	w10, #0x1a
1003749ac: 1a9f3108    	csel	w8, w8, wzr, lo
1003749b0: 2a090108    	orr	w8, w8, w9
1003749b4: 7101c11f    	cmp	w8, #0x70
1003749b8: 540003e1    	b.ne	0x100374a34 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0xf4>
1003749bc: 39400a69    	ldrb	w9, [x19, #0x2]
1003749c0: 5101052a    	sub	w10, w9, #0x41
1003749c4: 52800408    	mov	w8, #0x20               ; =32
1003749c8: 7100695f    	cmp	w10, #0x1a
1003749cc: 1a9f310a    	csel	w10, w8, wzr, lo
1003749d0: 2a090149    	orr	w9, w10, w9
1003749d4: 7101953f    	cmp	w9, #0x65
1003749d8: 540002e1    	b.ne	0x100374a34 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0xf4>
1003749dc: 39400e69    	ldrb	w9, [x19, #0x3]
1003749e0: 5101052a    	sub	w10, w9, #0x41
1003749e4: 7100695f    	cmp	w10, #0x1a
1003749e8: 1a9f3108    	csel	w8, w8, wzr, lo
1003749ec: 2a090108    	orr	w8, w8, w9
1003749f0: 7101b91f    	cmp	w8, #0x6e
1003749f4: 54000201    	b.ne	0x100374a34 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0xf4>
1003749f8: 39401269    	ldrb	w9, [x19, #0x4]
1003749fc: 5101052a    	sub	w10, w9, #0x41
100374a00: 52800408    	mov	w8, #0x20               ; =32
100374a04: 7100695f    	cmp	w10, #0x1a
100374a08: 1a9f310a    	csel	w10, w8, wzr, lo
100374a0c: 2a090149    	orr	w9, w10, w9
100374a10: 7101853f    	cmp	w9, #0x61
100374a14: 54000101    	b.ne	0x100374a34 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0xf4>
100374a18: 39401669    	ldrb	w9, [x19, #0x5]
100374a1c: 5101052a    	sub	w10, w9, #0x41
100374a20: 7100695f    	cmp	w10, #0x1a
100374a24: 1a9f3108    	csel	w8, w8, wzr, lo
100374a28: 2a090108    	orr	w8, w8, w9
100374a2c: 7101a51f    	cmp	w8, #0x69
100374a30: 54000d20    	b.eq	0x100374bd4 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x294>
100374a34: b9400268    	ldr	w8, [x19]
100374a38: 79400a69    	ldrh	w9, [x19, #0x4]
100374a3c: 528d2c2a    	mov	w10, #0x6961            ; =26977
100374a40: 72ac2daa    	movk	w10, #0x616d, lsl #16
100374a44: 6b0a011f    	cmp	w8, w10
100374a48: 52862d28    	mov	w8, #0x3169             ; =12649
100374a4c: 7a480120    	ccmp	w9, w8, #0x0, eq
100374a50: 540001e1    	b.ne	0x100374a8c <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x14c>
100374a54: 14000060    	b	0x100374bd4 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x294>
100374a58: f9400268    	ldr	x8, [x19]
100374a5c: f8406269    	ldur	x9, [x19, #0x6]
100374a60: d28e0dea    	mov	x10, #0x706f            ; =28783
100374a64: f2adccaa    	movk	x10, #0x6e65, lsl #16
100374a68: f2cd2c2a    	movk	x10, #0x6961, lsl #32
100374a6c: f2ec45aa    	movk	x10, #0x622d, lsl #48
100374a70: eb0a011f    	cmp	x8, x10
100374a74: d28c45a8    	mov	x8, #0x622d             ; =25133
100374a78: f2adcea8    	movk	x8, #0x6e75, lsl #16
100374a7c: f2cd8c88    	movk	x8, #0x6c64, lsl #32
100374a80: f2ec8ca8    	movk	x8, #0x6465, lsl #48
100374a84: fa480120    	ccmp	x9, x8, #0x0, eq
100374a88: 54000a60    	b.eq	0x100374bd4 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x294>
100374a8c: f9400c48    	ldr	x8, [x2, #0x18]
100374a90: b4000648    	cbz	x8, 0x100374b58 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x218>
100374a94: 91008040    	add	x0, x2, #0x20
100374a98: aa1303e1    	mov	x1, x19
100374a9c: aa0203f7    	mov	x23, x2
100374aa0: aa1403e2    	mov	x2, x20
100374aa4: 940d4145    	bl	0x1006c4fb8 <__ZN4core4hash11BuildHasher8hash_one17h61e5c885b8041446E>
100374aa8: d2800015    	mov	x21, #0x0               ; =0
100374aac: d379fc08    	lsr	x8, x0, #57
100374ab0: a9405af7    	ldp	x23, x22, [x23]
100374ab4: 0e010d08    	dup.8b	v8, w8
100374ab8: d10062f8    	sub	x24, x23, #0x18
100374abc: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
100374ac0: 52800319    	mov	w25, #0x18              ; =24
100374ac4: 8a16001a    	and	x26, x0, x22
100374ac8: fc7a6ae9    	ldr	d9, [x23, x26]
100374acc: 2e288d20    	cmeq.8b	v0, v9, v8
100374ad0: 9e660008    	fmov	x8, d0
100374ad4: f201c11b    	ands	x27, x8, #0x8080808080808080
100374ad8: 54000260    	b.eq	0x100374b24 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x1e4>
100374adc: dac00368    	rbit	x8, x27
100374ae0: dac01108    	clz	x8, x8
100374ae4: 8b480f48    	add	x8, x26, x8, lsr #3
100374ae8: 8a160108    	and	x8, x8, x22
100374aec: 9b19fd08    	mneg	x8, x8, x25
100374af0: 8b080308    	add	x8, x24, x8
100374af4: f9400909    	ldr	x9, [x8, #0x10]
100374af8: eb09029f    	cmp	x20, x9
100374afc: 540000c1    	b.ne	0x100374b14 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x1d4>
100374b00: f9400501    	ldr	x1, [x8, #0x8]
100374b04: aa1303e0    	mov	x0, x19
100374b08: aa1403e2    	mov	x2, x20
100374b0c: 943972e5    	bl	0x1011d16a0 <_writev+0x1011d16a0>
100374b10: 34000280    	cbz	w0, 0x100374b60 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x220>
100374b14: d1000768    	sub	x8, x27, #0x1
100374b18: ea1b011b    	ands	x27, x8, x27
100374b1c: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
100374b20: 54fffde1    	b.ne	0x100374adc <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x19c>
100374b24: 2e218d20    	cmeq.8b	v0, v9, v1
100374b28: 2e30a800    	umaxv.8b	b0, v0
100374b2c: 1e260008    	fmov	w8, s0
100374b30: 37000148    	tbnz	w8, #0x0, 0x100374b58 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x218>
100374b34: 910022b5    	add	x21, x21, #0x8
100374b38: 8b150340    	add	x0, x26, x21
100374b3c: 8a16001a    	and	x26, x0, x22
100374b40: fc7a6ae9    	ldr	d9, [x23, x26]
100374b44: 2e288d20    	cmeq.8b	v0, v9, v8
100374b48: 9e660008    	fmov	x8, d0
100374b4c: f201c11b    	ands	x27, x8, #0x8080808080808080
100374b50: 54fffc61    	b.ne	0x100374adc <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x19c>
100374b54: 17fffff4    	b	0x100374b24 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x1e4>
100374b58: 52800000    	mov	w0, #0x0                ; =0
100374b5c: 1400002e    	b	0x100374c14 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x2d4>
100374b60: f100329f    	cmp	x20, #0xc
100374b64: 540003c1    	b.ne	0x100374bdc <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x29c>
100374b68: f9400268    	ldr	x8, [x19]
100374b6c: d00081a9    	adrp	x9, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100374b70: 912de529    	add	x9, x9, #0xb79
100374b74: f940012a    	ldr	x10, [x9]
100374b78: b9400a6b    	ldr	w11, [x19, #0x8]
100374b7c: b9400929    	ldr	w9, [x9, #0x8]
100374b80: eb0a011f    	cmp	x8, x10
100374b84: fa490160    	ccmp	x11, x9, #0x0, eq
100374b88: 14000021    	b	0x100374c0c <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x2cc>
100374b8c: a9402668    	ldp	x8, x9, [x19]
100374b90: f840e26a    	ldur	x10, [x19, #0xe]
100374b94: d28e0deb    	mov	x11, #0x706f            ; =28783
100374b98: f2adccab    	movk	x11, #0x6e65, lsl #16
100374b9c: f2cd2c2b    	movk	x11, #0x6961, lsl #32
100374ba0: f2ee05ab    	movk	x11, #0x702d, lsl #48
100374ba4: eb0b011f    	cmp	x8, x11
100374ba8: d28d2e48    	mov	x8, #0x6972             ; =26994
100374bac: f2ac2da8    	movk	x8, #0x616d, lsl #16
100374bb0: f2cf2e48    	movk	x8, #0x7972, lsl #32
100374bb4: f2ee45a8    	movk	x8, #0x722d, lsl #48
100374bb8: fa480120    	ccmp	x9, x8, #0x0, eq
100374bbc: d28e45a8    	mov	x8, #0x722d             ; =29229
100374bc0: f2adcea8    	movk	x8, #0x6e75, lsl #16
100374bc4: f2cd2e88    	movk	x8, #0x6974, lsl #32
100374bc8: f2ecada8    	movk	x8, #0x656d, lsl #48
100374bcc: fa480140    	ccmp	x10, x8, #0x0, eq
100374bd0: 54fff5e1    	b.ne	0x100374a8c <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x14c>
100374bd4: 52800020    	mov	w0, #0x1                ; =1
100374bd8: 1400000f    	b	0x100374c14 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x2d4>
100374bdc: f100369f    	cmp	x20, #0xd
100374be0: 54000062    	b.hs	0x100374bec <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x2ac>
100374be4: 52800008    	mov	w8, #0x0                ; =0
100374be8: 1400000a    	b	0x100374c10 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE+0x2d0>
100374bec: d00081a8    	adrp	x8, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100374bf0: 912e1508    	add	x8, x8, #0xb85
100374bf4: f9400109    	ldr	x9, [x8]
100374bf8: f940026a    	ldr	x10, [x19]
100374bfc: f8405108    	ldur	x8, [x8, #0x5]
100374c00: f840526b    	ldur	x11, [x19, #0x5]
100374c04: eb0a013f    	cmp	x9, x10
100374c08: fa4b0100    	ccmp	x8, x11, #0x0, eq
100374c0c: 1a9f17e8    	cset	w8, eq
100374c10: 52000100    	eor	w0, w8, #0x1
100374c14: a9467bfd    	ldp	x29, x30, [sp, #0x60]
100374c18: a9454ff4    	ldp	x20, x19, [sp, #0x50]
100374c1c: a94457f6    	ldp	x22, x21, [sp, #0x40]
100374c20: a9435ff8    	ldp	x24, x23, [sp, #0x30]
100374c24: a94267fa    	ldp	x26, x25, [sp, #0x20]
100374c28: a9416ffc    	ldp	x28, x27, [sp, #0x10]
100374c2c: 6cc723e9    	ldp	d9, d8, [sp], #0x70
100374c30: d65f03c0    	ret
