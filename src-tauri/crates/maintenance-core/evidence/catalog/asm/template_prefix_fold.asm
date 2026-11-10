
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008029a4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE>:
1008029a4: d10483ff    	sub	sp, sp, #0x120
1008029a8: 6d0a2beb    	stp	d11, d10, [sp, #0xa0]
1008029ac: 6d0b23e9    	stp	d9, d8, [sp, #0xb0]
1008029b0: a90c6ffc    	stp	x28, x27, [sp, #0xc0]
1008029b4: a90d67fa    	stp	x26, x25, [sp, #0xd0]
1008029b8: a90e5ff8    	stp	x24, x23, [sp, #0xe0]
1008029bc: a90f57f6    	stp	x22, x21, [sp, #0xf0]
1008029c0: a9104ff4    	stp	x20, x19, [sp, #0x100]
1008029c4: a9117bfd    	stp	x29, x30, [sp, #0x110]
1008029c8: 910443fd    	add	x29, sp, #0x110
1008029cc: a9402c1c    	ldp	x28, x11, [x0]
1008029d0: eb0b039f    	cmp	x28, x11
1008029d4: 54001960    	b.eq	0x100802d00 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x35c>
1008029d8: f90007e8    	str	x8, [sp, #0x8]
1008029dc: d2800014    	mov	x20, #0x0               ; =0
1008029e0: f9400809    	ldr	x9, [x0, #0x10]
1008029e4: 3dc00020    	ldr	q0, [x1]
1008029e8: 3d800be0    	str	q0, [sp, #0x20]
1008029ec: f940082a    	ldr	x10, [x1, #0x10]
1008029f0: f9001bea    	str	x10, [sp, #0x30]
1008029f4: cb1c0168    	sub	x8, x11, x28
1008029f8: d345fd15    	lsr	x21, x8, #5
1008029fc: a940a12a    	ldp	x10, x8, [x9, #0x8]
100802a00: a90123ea    	stp	x10, x8, [sp, #0x10]
100802a04: 910183e8    	add	x8, sp, #0x60
100802a08: 91006108    	add	x8, x8, #0x18
100802a0c: f90003e8    	str	x8, [sp]
100802a10: d0005d76    	adrp	x22, 0x1013b0000 <_anon.973b090bee24c06fdb949486dbea4242.108+0x3d8>
100802a14: 912bcad6    	add	x22, x22, #0xaf2
100802a18: 5280041a    	mov	w26, #0x20              ; =32
100802a1c: 0f05e7e8    	movi.8b	v8, #0xbf
100802a20: 0f00e749    	movi.8b	v9, #0x1a
100802a24: 0f01e40a    	movi.8b	v10, #0x20
100802a28: 8b141797    	add	x23, x28, x20, lsl #5
100802a2c: aa1603e0    	mov	x0, x22
100802a30: 52800081    	mov	w1, #0x4                ; =4
100802a34: aa1703e2    	mov	x2, x23
100802a38: 94243056    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
100802a3c: b40010e0    	cbz	x0, 0x100802c58 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x2b4>
100802a40: 39400008    	ldrb	w8, [x0]
100802a44: 71000d1f    	cmp	w8, #0x3
100802a48: 54001081    	b.ne	0x100802c58 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x2b4>
100802a4c: a9410408    	ldp	x8, x1, [x0, #0x10]
100802a50: aa0803e0    	mov	x0, x8
100802a54: 97e8b1c1    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100802a58: b7f814e1    	tbnz	x1, #0x3f, 0x100802cf4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x350>
100802a5c: aa0003fb    	mov	x27, x0
100802a60: aa0103f8    	mov	x24, x1
100802a64: b40001c1    	cbz	x1, 0x100802a9c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0xf8>
100802a68: 97dffac3    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100802a6c: aa1803e0    	mov	x0, x24
100802a70: 52800021    	mov	w1, #0x1                ; =1
100802a74: 97dffabc    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100802a78: b40016c0    	cbz	x0, 0x100802d50 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x3ac>
100802a7c: aa0003f9    	mov	x25, x0
100802a80: aa1b03e1    	mov	x1, x27
100802a84: aa1803e2    	mov	x2, x24
100802a88: 94273b09    	bl	0x1011d16ac <_writev+0x1011d16ac>
100802a8c: f100231f    	cmp	x24, #0x8
100802a90: 54000142    	b.hs	0x100802ab8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x114>
100802a94: d2800008    	mov	x8, #0x0                ; =0
100802a98: 14000038    	b	0x100802b78 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x1d4>
100802a9c: 52800020    	mov	w0, #0x1                ; =1
100802aa0: aa1b03e1    	mov	x1, x27
100802aa4: aa1803e2    	mov	x2, x24
100802aa8: 94273b01    	bl	0x1011d16ac <_writev+0x1011d16ac>
100802aac: 52800039    	mov	w25, #0x1               ; =1
100802ab0: 52800033    	mov	w19, #0x1               ; =1
100802ab4: 1400003b    	b	0x100802ba0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x1fc>
100802ab8: f101031f    	cmp	x24, #0x40
100802abc: 54000062    	b.hs	0x100802ac8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x124>
100802ac0: d2800008    	mov	x8, #0x0                ; =0
100802ac4: 14000021    	b	0x100802b48 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x1a4>
100802ac8: 927ae308    	and	x8, x24, #0x7fffffffffffffc0
100802acc: 91008329    	add	x9, x25, #0x20
100802ad0: aa0803ea    	mov	x10, x8
100802ad4: 4f05e7f0    	movi.16b	v16, #0xbf
100802ad8: 4f00e751    	movi.16b	v17, #0x1a
100802adc: 4f01e412    	movi.16b	v18, #0x20
100802ae0: ad7f0520    	ldp	q0, q1, [x9, #-0x20]
100802ae4: ad400d22    	ldp	q2, q3, [x9]
100802ae8: 4e308404    	add.16b	v4, v0, v16
100802aec: 4e308425    	add.16b	v5, v1, v16
100802af0: 4e308446    	add.16b	v6, v2, v16
100802af4: 4e308467    	add.16b	v7, v3, v16
100802af8: 6e243624    	cmhi.16b	v4, v17, v4
100802afc: 6e253625    	cmhi.16b	v5, v17, v5
100802b00: 6e263626    	cmhi.16b	v6, v17, v6
100802b04: 6e273627    	cmhi.16b	v7, v17, v7
100802b08: 4e321c84    	and.16b	v4, v4, v18
100802b0c: 4e321ca5    	and.16b	v5, v5, v18
100802b10: 4e321cc6    	and.16b	v6, v6, v18
100802b14: 4e321ce7    	and.16b	v7, v7, v18
100802b18: 4ea01c80    	orr.16b	v0, v4, v0
100802b1c: 4ea11ca1    	orr.16b	v1, v5, v1
100802b20: 4ea21cc2    	orr.16b	v2, v6, v2
100802b24: ad3f0520    	stp	q0, q1, [x9, #-0x20]
100802b28: 4ea31ce0    	orr.16b	v0, v7, v3
100802b2c: ac820122    	stp	q2, q0, [x9], #0x40
100802b30: f101014a    	subs	x10, x10, #0x40
100802b34: 54fffd61    	b.ne	0x100802ae0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x13c>
100802b38: eb08031f    	cmp	x24, x8
100802b3c: 54000300    	b.eq	0x100802b9c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x1f8>
100802b40: f27d0b1f    	tst	x24, #0x38
100802b44: 540001a0    	b.eq	0x100802b78 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x1d4>
100802b48: aa0803e9    	mov	x9, x8
100802b4c: 927def08    	and	x8, x24, #0x7ffffffffffffff8
100802b50: fc696b20    	ldr	d0, [x25, x9]
100802b54: 0e288401    	add.8b	v1, v0, v8
100802b58: 2e213521    	cmhi.8b	v1, v9, v1
100802b5c: 0e2a1c21    	and.8b	v1, v1, v10
100802b60: 0ea01c20    	orr.8b	v0, v1, v0
100802b64: fc296b20    	str	d0, [x25, x9]
100802b68: 91002129    	add	x9, x9, #0x8
100802b6c: eb09011f    	cmp	x8, x9
100802b70: 54ffff01    	b.ne	0x100802b50 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x1ac>
100802b74: 14000008    	b	0x100802b94 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x1f0>
100802b78: 38686b29    	ldrb	w9, [x25, x8]
100802b7c: 5101052a    	sub	w10, w9, #0x41
100802b80: 7100695f    	cmp	w10, #0x1a
100802b84: 1a9f334a    	csel	w10, w26, wzr, lo
100802b88: 2a090149    	orr	w9, w10, w9
100802b8c: 38286b29    	strb	w9, [x25, x8]
100802b90: 91000508    	add	x8, x8, #0x1
100802b94: eb08031f    	cmp	x24, x8
100802b98: 54ffff01    	b.ne	0x100802b78 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x1d4>
100802b9c: aa1903f3    	mov	x19, x25
100802ba0: 910183e0    	add	x0, sp, #0x60
100802ba4: aa1903e1    	mov	x1, x25
100802ba8: aa1803e2    	mov	x2, x24
100802bac: 97ed9209    	bl	0x1003673d0 <__ZN13codexmate_lib4core5relay13codex_catalog16model_tail_lower17hdcbb168b8679db13E>
100802bb0: a946a3e0    	ldp	x0, x8, [sp, #0x68]
100802bb4: f1001d1f    	cmp	x8, #0x7
100802bb8: 54000141    	b.ne	0x100802be0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x23c>
100802bbc: b9400008    	ldr	w8, [x0]
100802bc0: d000584b    	adrp	x11, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
100802bc4: 9102316b    	add	x11, x11, #0x8c
100802bc8: b9400169    	ldr	w9, [x11]
100802bcc: b840300a    	ldur	w10, [x0, #0x3]
100802bd0: b840316b    	ldur	w11, [x11, #0x3]
100802bd4: 6b09011f    	cmp	w8, w9
100802bd8: 7a4b0140    	ccmp	w10, w11, #0x0, eq
100802bdc: 1400000c    	b	0x100802c0c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x268>
100802be0: f100211f    	cmp	x8, #0x8
100802be4: 540000a2    	b.hs	0x100802bf8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x254>
100802be8: 5280001b    	mov	w27, #0x0               ; =0
100802bec: f94033e1    	ldr	x1, [sp, #0x60]
100802bf0: b5000141    	cbnz	x1, 0x100802c18 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x274>
100802bf4: 1400000b    	b	0x100802c20 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x27c>
100802bf8: d0005848    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
100802bfc: 91024d08    	add	x8, x8, #0x93
100802c00: f9400108    	ldr	x8, [x8]
100802c04: f9400009    	ldr	x9, [x0]
100802c08: eb09011f    	cmp	x8, x9
100802c0c: 1a9f17fb    	cset	w27, eq
100802c10: f94033e1    	ldr	x1, [sp, #0x60]
100802c14: b4000061    	cbz	x1, 0x100802c20 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x27c>
100802c18: 52800022    	mov	w2, #0x1                ; =1
100802c1c: 97dffa53    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100802c20: f9400fe8    	ldr	x8, [sp, #0x18]
100802c24: eb18011f    	cmp	x8, x24
100802c28: 1a9b33e8    	csel	w8, wzr, w27, lo
100802c2c: 360000c8    	tbz	w8, #0x0, 0x100802c44 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x2a0>
100802c30: aa1903e0    	mov	x0, x25
100802c34: f9400be1    	ldr	x1, [sp, #0x10]
100802c38: aa1803e2    	mov	x2, x24
100802c3c: 94273a99    	bl	0x1011d16a0 <_writev+0x1011d16a0>
100802c40: 34000220    	cbz	w0, 0x100802c84 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x2e0>
100802c44: b40000b8    	cbz	x24, 0x100802c58 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x2b4>
100802c48: aa1903e0    	mov	x0, x25
100802c4c: aa1803e1    	mov	x1, x24
100802c50: 52800022    	mov	w2, #0x1                ; =1
100802c54: 97dffa45    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100802c58: 3dc00be0    	ldr	q0, [sp, #0x20]
100802c5c: 3d8013e0    	str	q0, [sp, #0x40]
100802c60: f9401be8    	ldr	x8, [sp, #0x30]
100802c64: f9002be8    	str	x8, [sp, #0x50]
100802c68: 3dc013e0    	ldr	q0, [sp, #0x40]
100802c6c: 3d800be0    	str	q0, [sp, #0x20]
100802c70: f9001be8    	str	x8, [sp, #0x30]
100802c74: 91000694    	add	x20, x20, #0x1
100802c78: eb15029f    	cmp	x20, x21
100802c7c: 54ffed61    	b.ne	0x100802a28 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x84>
100802c80: 1400002e    	b	0x100802d38 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x394>
100802c84: f9400fe8    	ldr	x8, [sp, #0x18]
100802c88: eb18011f    	cmp	x8, x24
100802c8c: 540000c0    	b.eq	0x100802ca4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x300>
100802c90: f9400be8    	ldr	x8, [sp, #0x10]
100802c94: 38786908    	ldrb	w8, [x8, x24]
100802c98: 12001d08    	and	w8, w8, #0xff
100802c9c: 7100b51f    	cmp	w8, #0x2d
100802ca0: 9a9f02f7    	csel	x23, x23, xzr, eq
100802ca4: b40000b8    	cbz	x24, 0x100802cb8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x314>
100802ca8: aa1903e0    	mov	x0, x25
100802cac: aa1803e1    	mov	x1, x24
100802cb0: 52800022    	mov	w2, #0x1                ; =1
100802cb4: 97dffa2d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100802cb8: b4fffd17    	cbz	x23, 0x100802c58 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x2b4>
100802cbc: 3dc00be0    	ldr	q0, [sp, #0x20]
100802cc0: 3d801be0    	str	q0, [sp, #0x60]
100802cc4: f9401be8    	ldr	x8, [sp, #0x30]
100802cc8: a90763e8    	stp	x8, x24, [sp, #0x70]
100802ccc: a9085ff8    	stp	x24, x23, [sp, #0x80]
100802cd0: f94033e8    	ldr	x8, [sp, #0x60]
100802cd4: eb18011f    	cmp	x8, x24
100802cd8: 910183e8    	add	x8, sp, #0x60
100802cdc: f94003e9    	ldr	x9, [sp]
100802ce0: 9a898108    	csel	x8, x8, x9, hi
100802ce4: 3dc00100    	ldr	q0, [x8]
100802ce8: 3d8013e0    	str	q0, [sp, #0x40]
100802cec: f9400908    	ldr	x8, [x8, #0x10]
100802cf0: 17ffffdd    	b	0x100802c64 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x2c0>
100802cf4: d2800000    	mov	x0, #0x0                ; =0
100802cf8: aa1303e1    	mov	x1, x19
100802cfc: 9427365a    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100802d00: 3dc00020    	ldr	q0, [x1]
100802d04: 3d800100    	str	q0, [x8]
100802d08: f9400829    	ldr	x9, [x1, #0x10]
100802d0c: f9000909    	str	x9, [x8, #0x10]
100802d10: a9517bfd    	ldp	x29, x30, [sp, #0x110]
100802d14: a9504ff4    	ldp	x20, x19, [sp, #0x100]
100802d18: a94f57f6    	ldp	x22, x21, [sp, #0xf0]
100802d1c: a94e5ff8    	ldp	x24, x23, [sp, #0xe0]
100802d20: a94d67fa    	ldp	x26, x25, [sp, #0xd0]
100802d24: a94c6ffc    	ldp	x28, x27, [sp, #0xc0]
100802d28: 6d4b23e9    	ldp	d9, d8, [sp, #0xb0]
100802d2c: 6d4a2beb    	ldp	d11, d10, [sp, #0xa0]
100802d30: 910483ff    	add	sp, sp, #0x120
100802d34: d65f03c0    	ret
100802d38: 3dc00be0    	ldr	q0, [sp, #0x20]
100802d3c: f94007e9    	ldr	x9, [sp, #0x8]
100802d40: 3d800120    	str	q0, [x9]
100802d44: f9401be8    	ldr	x8, [sp, #0x30]
100802d48: f9000928    	str	x8, [x9, #0x10]
100802d4c: 17fffff1    	b	0x100802d10 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x36c>
100802d50: 52800020    	mov	w0, #0x1                ; =1
100802d54: aa1803e1    	mov	x1, x24
100802d58: 94273643    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100802d5c: aa0003f3    	mov	x19, x0
100802d60: b40000b8    	cbz	x24, 0x100802d74 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE+0x3d0>
100802d64: aa1903e0    	mov	x0, x25
100802d68: aa1803e1    	mov	x1, x24
100802d6c: 52800022    	mov	w2, #0x1                ; =1
100802d70: 97dff9fe    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100802d74: aa1303e0    	mov	x0, x19
100802d78: 942738f7    	bl	0x1011d1154 <_writev+0x1011d1154>

0000000100802d7c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE>:
100802d7c: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
100802d80: a90167fa    	stp	x26, x25, [sp, #0x10]
100802d84: a9025ff8    	stp	x24, x23, [sp, #0x20]
100802d88: a90357f6    	stp	x22, x21, [sp, #0x30]
100802d8c: a9044ff4    	stp	x20, x19, [sp, #0x40]
100802d90: a9057bfd    	stp	x29, x30, [sp, #0x50]
100802d94: 910143fd    	add	x29, sp, #0x50
100802d98: d11d43ff    	sub	sp, sp, #0x750
100802d9c: f90003ff    	str	xzr, [sp]
100802da0: f90017e1    	str	x1, [sp, #0x28]
100802da4: f9402008    	ldr	x8, [x0, #0x40]
100802da8: ad400400    	ldp	q0, q1, [x0]
100802dac: ad0187e0    	stp	q0, q1, [sp, #0x30]
100802db0: ad410001    	ldp	q1, q0, [x0, #0x20]
100802db4: ad0283e1    	stp	q1, q0, [sp, #0x50]
100802db8: f94037f5    	ldr	x21, [sp, #0x68]
100802dbc: b4001175    	cbz	x21, 0x100802fe8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x26c>
100802dc0: a940250a    	ldp	x10, x9, [x8]
100802dc4: a901abe9    	stp	x9, x10, [sp, #0x18]
100802dc8: a941250a    	ldp	x10, x9, [x8, #0x10]
100802dcc: a900abe9    	stp	x9, x10, [sp, #0x8]
100802dd0: a9426913    	ldp	x19, x26, [x8, #0x20]
100802dd4: 910f63fb    	add	x27, sp, #0x3d8
100802dd8: a944e3f6    	ldp	x22, x24, [sp, #0x48]
100802ddc: 5280003c    	mov	w28, #0x1               ; =1
100802de0: f9402ff7    	ldr	x23, [sp, #0x58]
100802de4: 14000002    	b	0x100802dec <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x70>
100802de8: b4001015    	cbz	x21, 0x100802fe8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x26c>
100802dec: b50000f8    	cbnz	x24, 0x100802e08 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x8c>
100802df0: fc4086e0    	ldr	d0, [x23], #0x8
100802df4: 2e208800    	cmge.8b	v0, v0, #0
100802df8: 9e660008    	fmov	x8, d0
100802dfc: d13802d6    	sub	x22, x22, #0xe00
100802e00: 9201c118    	and	x24, x8, #0x8080808080808080
100802e04: b4ffff78    	cbz	x24, 0x100802df0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x74>
100802e08: d1000708    	sub	x8, x24, #0x1
100802e0c: dac00309    	rbit	x9, x24
100802e10: dac01129    	clz	x9, x9
100802e14: 8a180118    	and	x24, x8, x24
100802e18: cb490fe8    	neg	x8, x9, lsr #3
100802e1c: 52803809    	mov	w9, #0x1c0              ; =448
100802e20: 9b295908    	smaddl	x8, w8, w9, x22
100802e24: d10006b5    	sub	x21, x21, #0x1
100802e28: d106e109    	sub	x9, x8, #0x1b8
100802e2c: f9400139    	ldr	x25, [x9]
100802e30: f100133f    	cmp	x25, #0x4
100802e34: 540009a0    	b.eq	0x100802f68 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x1ec>
100802e38: d106c101    	sub	x1, x8, #0x1b0
100802e3c: d1070108    	sub	x8, x8, #0x1c0
100802e40: b9400114    	ldr	w20, [x8]
100802e44: 9101c3e0    	add	x0, sp, #0x70
100802e48: 52803602    	mov	w2, #0x1b0              ; =432
100802e4c: 94273a1b    	bl	0x1011d16b8 <_writev+0x1011d16b8>
100802e50: f1000f3f    	cmp	x25, #0x3
100802e54: 54000081    	b.ne	0x100802e64 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0xe8>
100802e58: ad4387e0    	ldp	q0, q1, [sp, #0x70]
100802e5c: ad1d87e0    	stp	q0, q1, [sp, #0x3b0]
100802e60: 1400002f    	b	0x100802f1c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x1a0>
100802e64: ad4387e0    	ldp	q0, q1, [sp, #0x70]
100802e68: 911643e8    	add	x8, sp, #0x590
100802e6c: 3c808100    	stur	q0, [x8, #0x8]
100802e70: 3c818101    	stur	q1, [x8, #0x18]
100802e74: 9100a100    	add	x0, x8, #0x28
100802e78: 9101c3e8    	add	x8, sp, #0x70
100802e7c: 91008101    	add	x1, x8, #0x20
100802e80: 52803202    	mov	w2, #0x190              ; =400
100802e84: 94273a0a    	bl	0x1011d16ac <_writev+0x1011d16ac>
100802e88: f902cbf9    	str	x25, [sp, #0x590]
100802e8c: f94013e8    	ldr	x8, [sp, #0x20]
100802e90: f83c0108    	ldadd	x28, x8, [x8]
100802e94: b7f80c68    	tbnz	x8, #0x3f, 0x100803020 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x2a4>
100802e98: f9400fe8    	ldr	x8, [sp, #0x18]
100802e9c: f83c0108    	ldadd	x28, x8, [x8]
100802ea0: b7f80c08    	tbnz	x8, #0x3f, 0x100803020 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x2a4>
100802ea4: f9400be8    	ldr	x8, [sp, #0x10]
100802ea8: f83c0108    	ldadd	x28, x8, [x8]
100802eac: b7f80ba8    	tbnz	x8, #0x3f, 0x100803020 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x2a4>
100802eb0: f94007e8    	ldr	x8, [sp, #0x8]
100802eb4: f83c0108    	ldadd	x28, x8, [x8]
100802eb8: b7f80b48    	tbnz	x8, #0x3f, 0x100803020 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x2a4>
100802ebc: f83c0268    	ldadd	x28, x8, [x19]
100802ec0: b7f80b08    	tbnz	x8, #0x3f, 0x100803020 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x2a4>
100802ec4: f83c0348    	ldadd	x28, x8, [x26]
100802ec8: b7f80ac8    	tbnz	x8, #0x3f, 0x100803020 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x2a4>
100802ecc: a941a7e8    	ldp	x8, x9, [sp, #0x18]
100802ed0: f90113e9    	str	x9, [sp, #0x220]
100802ed4: f90117e8    	str	x8, [sp, #0x228]
100802ed8: a940a7e8    	ldp	x8, x9, [sp, #0x8]
100802edc: f9011be9    	str	x9, [sp, #0x230]
100802ee0: f9011fe8    	str	x8, [sp, #0x238]
100802ee4: f90123f3    	str	x19, [sp, #0x240]
100802ee8: f90127fa    	str	x26, [sp, #0x248]
100802eec: 910f63e8    	add	x8, sp, #0x3d8
100802ef0: 911643e0    	add	x0, sp, #0x590
100802ef4: 910883e1    	add	x1, sp, #0x220
100802ef8: 97ea2090    	bl	0x10028b138 <__ZN4axum7routing14method_routing25MethodRouter$LT$S$C$E$GT$10with_state17hf00dce4f9a031a21E>
100802efc: f941eff9    	ldr	x25, [sp, #0x3d8]
100802f00: 3cc08360    	ldur	q0, [x27, #0x8]
100802f04: 3cc18361    	ldur	q1, [x27, #0x18]
100802f08: ad1d87e0    	stp	q0, q1, [sp, #0x3b0]
100802f0c: 910883e0    	add	x0, sp, #0x220
100802f10: 9100a361    	add	x1, x27, #0x28
100802f14: 52803202    	mov	w2, #0x190              ; =400
100802f18: 942739e5    	bl	0x1011d16ac <_writev+0x1011d16ac>
100802f1c: f901eff9    	str	x25, [sp, #0x3d8]
100802f20: ad5d87e0    	ldp	q0, q1, [sp, #0x3b0]
100802f24: 3c808360    	stur	q0, [x27, #0x8]
100802f28: 3c818361    	stur	q1, [x27, #0x18]
100802f2c: 9100a360    	add	x0, x27, #0x28
100802f30: 910883e1    	add	x1, sp, #0x220
100802f34: 52803202    	mov	w2, #0x190              ; =400
100802f38: 942739dd    	bl	0x1011d16ac <_writev+0x1011d16ac>
100802f3c: 911643e8    	add	x8, sp, #0x590
100802f40: 910f63e2    	add	x2, sp, #0x3d8
100802f44: f94017e0    	ldr	x0, [sp, #0x28]
100802f48: aa1403e1    	mov	x1, x20
100802f4c: 97f5d0b0    	bl	0x10057720c <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17hf04aeb8bfc20e129E>
100802f50: f942cbe8    	ldr	x8, [sp, #0x590]
100802f54: f100111f    	cmp	x8, #0x4
100802f58: 54fff480    	b.eq	0x100802de8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x6c>
100802f5c: 911643e0    	add	x0, sp, #0x590
100802f60: 940185ab    	bl	0x10086460c <__ZN4core3ptr60drop_in_place$LT$axum..routing..Endpoint$LT$$LP$$RP$$GT$$GT$17h0a03c815fc53c50eE>
100802f64: 17ffffa1    	b	0x100802de8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x6c>
100802f68: a904e3f6    	stp	x22, x24, [sp, #0x48]
100802f6c: f9002ff7    	str	x23, [sp, #0x58]
100802f70: f90037f5    	str	x21, [sp, #0x68]
100802f74: b40003b5    	cbz	x21, 0x100802fe8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x26c>
100802f78: 52803813    	mov	w19, #0x1c0             ; =448
100802f7c: 14000007    	b	0x100802f98 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x21c>
100802f80: d106c100    	sub	x0, x8, #0x1b0
100802f84: 940182fa    	bl	0x100863b6c <__ZN4core3ptr48drop_in_place$LT$axum..routing..route..Route$GT$17hc8d945349ed9eec6E>
100802f88: d1000708    	sub	x8, x24, #0x1
100802f8c: 8a180118    	and	x24, x8, x24
100802f90: d10006b5    	sub	x21, x21, #0x1
100802f94: b40002b5    	cbz	x21, 0x100802fe8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x26c>
100802f98: b50000f8    	cbnz	x24, 0x100802fb4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x238>
100802f9c: fc4086e0    	ldr	d0, [x23], #0x8
100802fa0: 2e208800    	cmge.8b	v0, v0, #0
100802fa4: 9e660008    	fmov	x8, d0
100802fa8: d13802d6    	sub	x22, x22, #0xe00
100802fac: 9201c118    	and	x24, x8, #0x8080808080808080
100802fb0: b4ffff78    	cbz	x24, 0x100802f9c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x220>
100802fb4: dac00308    	rbit	x8, x24
100802fb8: dac01108    	clz	x8, x8
100802fbc: cb480fe8    	neg	x8, x8, lsr #3
100802fc0: 9b335908    	smaddl	x8, w8, w19, x22
100802fc4: d106e100    	sub	x0, x8, #0x1b8
100802fc8: f9400009    	ldr	x9, [x0]
100802fcc: f1000d3f    	cmp	x9, #0x3
100802fd0: 54fffd80    	b.eq	0x100802f80 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x204>
100802fd4: 940176a1    	bl	0x100860a58 <__ZN4core3ptr126drop_in_place$LT$axum..routing..method_routing..MethodRouter$LT$codexmate_lib..core..relay..proxy_server..ProxyContext$GT$$GT$17h73fe5481201871dbE>
100802fd8: d1000708    	sub	x8, x24, #0x1
100802fdc: 8a180118    	and	x24, x8, x24
100802fe0: d10006b5    	sub	x21, x21, #0x1
100802fe4: b5fffdb5    	cbnz	x21, 0x100802f98 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x21c>
100802fe8: f9401be2    	ldr	x2, [sp, #0x30]
100802fec: b40000a2    	cbz	x2, 0x100803000 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x284>
100802ff0: f9401fe1    	ldr	x1, [sp, #0x38]
100802ff4: b4000061    	cbz	x1, 0x100803000 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h76407df6e917256aE+0x284>
100802ff8: f94023e0    	ldr	x0, [sp, #0x40]
100802ffc: 97dff95b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100803000: 911d43ff    	add	sp, sp, #0x750
100803004: a9457bfd    	ldp	x29, x30, [sp, #0x50]
100803008: a9444ff4    	ldp	x20, x19, [sp, #0x40]
10080300c: a94357f6    	ldp	x22, x21, [sp, #0x30]
100803010: a9425ff8    	ldp	x24, x23, [sp, #0x20]
100803014: a94167fa    	ldp	x26, x25, [sp, #0x10]
100803018: a8c66ffc    	ldp	x28, x27, [sp], #0x60
10080301c: d65f03c0    	ret
100803020: d4200020    	brk	#0x1
100803024: aa0003f3    	mov	x19, x0
100803028: a904e3f6    	stp	x22, x24, [sp, #0x48]
10080302c: f9002ff7    	str	x23, [sp, #0x58]
100803030: f90037f5    	str	x21, [sp, #0x68]
100803034: 9100c3e0    	add	x0, sp, #0x30
100803038: 9401bcad    	bl	0x1008722ec <__ZN82_$LT$hashbrown..raw..RawIntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb7a90a1af86f77edE>
10080303c: aa1303e0    	mov	x0, x19
100803040: 94273845    	bl	0x1011d1154 <_writev+0x1011d1154>
100803044: 942736c1    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

0000000100803048 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2037003267bde095E>:
100803048: d10383ff    	sub	sp, sp, #0xe0
10080304c: a9086ffc    	stp	x28, x27, [sp, #0x80]
100803050: a90967fa    	stp	x26, x25, [sp, #0x90]
100803054: a90a5ff8    	stp	x24, x23, [sp, #0xa0]
100803058: a90b57f6    	stp	x22, x21, [sp, #0xb0]
10080305c: a90c4ff4    	stp	x20, x19, [sp, #0xc0]
100803060: a90d7bfd    	stp	x29, x30, [sp, #0xd0]
100803064: 910343fd    	add	x29, sp, #0xd0
100803068: a9402408    	ldp	x8, x9, [x0]
10080306c: a940602a    	ldp	x10, x24, [x1]
100803070: f90007ea    	str	x10, [sp, #0x8]
100803074: eb09011f    	cmp	x8, x9
100803078: 54000600    	b.eq	0x100803138 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2037003267bde095E+0xf0>
10080307c: f940082a    	ldr	x10, [x1, #0x10]
100803080: f9400819    	ldr	x25, [x0, #0x10]
100803084: cb080129    	sub	x9, x9, x8
100803088: d345fd3a    	lsr	x26, x9, #5
10080308c: 91002113    	add	x19, x8, #0x8
100803090: 5280091b    	mov	w27, #0x48              ; =72
100803094: 9b1b2b08    	madd	x8, x24, x27, x10
100803098: 9100c11c    	add	x28, x8, #0x30
10080309c: f85f8260    	ldur	x0, [x19, #-0x8]
1008030a0: f9400b21    	ldr	x1, [x25, #0x10]
1008030a4: eb01001f    	cmp	x0, x1
1008030a8: 540005c2    	b.hs	0x100803160 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2037003267bde095E+0x118>
1008030ac: f9400728    	ldr	x8, [x25, #0x8]
1008030b0: 9b1b2016    	madd	x22, x0, x27, x8
1008030b4: 910143e8    	add	x8, sp, #0x50
1008030b8: aa1603e0    	mov	x0, x22
1008030bc: 9425d910    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
1008030c0: f8430ec8    	ldr	x8, [x22, #0x30]!
1008030c4: d2f00015    	mov	x21, #-0x8000000000000000 ; =-9223372036854775808
1008030c8: eb15011f    	cmp	x8, x21
1008030cc: 540000c0    	b.eq	0x1008030e4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2037003267bde095E+0x9c>
1008030d0: 9101a3e8    	add	x8, sp, #0x68
1008030d4: aa1603e0    	mov	x0, x22
1008030d8: 9425d909    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
1008030dc: a946d3f5    	ldp	x21, x20, [sp, #0x68]
1008030e0: f9403ff7    	ldr	x23, [sp, #0x78]
1008030e4: 9101a3e8    	add	x8, sp, #0x68
1008030e8: aa1303e0    	mov	x0, x19
1008030ec: 9425d904    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
1008030f0: 3dc017e0    	ldr	q0, [sp, #0x50]
1008030f4: 3d800fe0    	str	q0, [sp, #0x30]
1008030f8: f94033e8    	ldr	x8, [sp, #0x60]
1008030fc: f90023e8    	str	x8, [sp, #0x40]
100803100: 3cc683e1    	ldur	q1, [sp, #0x68]
100803104: 3d8007e1    	str	q1, [sp, #0x10]
100803108: f9403fe9    	ldr	x9, [sp, #0x78]
10080310c: 3c9d0380    	stur	q0, [x28, #-0x30]
100803110: a93e5788    	stp	x8, x21, [x28, #-0x20]
100803114: a93f5f94    	stp	x20, x23, [x28, #-0x10]
100803118: 3d800381    	str	q1, [x28]
10080311c: 91000718    	add	x24, x24, #0x1
100803120: f9000b89    	str	x9, [x28, #0x10]
100803124: 91008273    	add	x19, x19, #0x20
100803128: 9101239c    	add	x28, x28, #0x48
10080312c: f90013e9    	str	x9, [sp, #0x20]
100803130: f100075a    	subs	x26, x26, #0x1
100803134: 54fffb41    	b.ne	0x10080309c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2037003267bde095E+0x54>
100803138: f94007e8    	ldr	x8, [sp, #0x8]
10080313c: f9000118    	str	x24, [x8]
100803140: a94d7bfd    	ldp	x29, x30, [sp, #0xd0]
100803144: a94c4ff4    	ldp	x20, x19, [sp, #0xc0]
100803148: a94b57f6    	ldp	x22, x21, [sp, #0xb0]
10080314c: a94a5ff8    	ldp	x24, x23, [sp, #0xa0]
100803150: a94967fa    	ldp	x26, x25, [sp, #0x90]
100803154: a9486ffc    	ldp	x28, x27, [sp, #0x80]
100803158: 910383ff    	add	sp, sp, #0xe0
10080315c: d65f03c0    	ret
100803160: b0008142    	adrp	x2, 0x10182c000 <_anon.6408540cdffbe318d49f52812e99783f.1678+0xb8>
100803164: 91344042    	add	x2, x2, #0xd10
100803168: 9427367e    	bl	0x1011d0b60 <__ZN4core9panicking18panic_bounds_check17h865e5d10cdb12e28E>
10080316c: d4200020    	brk	#0x1
100803170: aa0003f3    	mov	x19, x0
100803174: 1400000c    	b	0x1008031a4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2037003267bde095E+0x15c>
100803178: 14000001    	b	0x10080317c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2037003267bde095E+0x134>
10080317c: f94007e8    	ldr	x8, [sp, #0x8]
100803180: f9000118    	str	x24, [x8]
100803184: 942737f4    	bl	0x1011d1154 <_writev+0x1011d1154>
100803188: aa0003f3    	mov	x19, x0
10080318c: f240fabf    	tst	x21, #0x7fffffffffffffff
100803190: 540000a0    	b.eq	0x1008031a4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2037003267bde095E+0x15c>
100803194: aa1403e0    	mov	x0, x20
100803198: aa1503e1    	mov	x1, x21
10080319c: 52800022    	mov	w2, #0x1                ; =1
1008031a0: 97dff8f2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008031a4: f9402be1    	ldr	x1, [sp, #0x50]
1008031a8: b4000081    	cbz	x1, 0x1008031b8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2037003267bde095E+0x170>
1008031ac: f9402fe0    	ldr	x0, [sp, #0x58]
1008031b0: 52800022    	mov	w2, #0x1                ; =1
1008031b4: 97dff8ed    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008031b8: f94007e8    	ldr	x8, [sp, #0x8]
1008031bc: f9000118    	str	x24, [x8]
1008031c0: aa1303e0    	mov	x0, x19
1008031c4: 942737e4    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001008031c8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2317c90935a7f9edE>:
1008031c8: d10303ff    	sub	sp, sp, #0xc0
1008031cc: a9066ffc    	stp	x28, x27, [sp, #0x60]
1008031d0: a90767fa    	stp	x26, x25, [sp, #0x70]
1008031d4: a9085ff8    	stp	x24, x23, [sp, #0x80]
1008031d8: a90957f6    	stp	x22, x21, [sp, #0x90]
1008031dc: a90a4ff4    	stp	x20, x19, [sp, #0xa0]
1008031e0: a90b7bfd    	stp	x29, x30, [sp, #0xb0]
1008031e4: 9102c3fd    	add	x29, sp, #0xb0
1008031e8: a9406048    	ldp	x8, x24, [x2]
1008031ec: f90007e8    	str	x8, [sp, #0x8]
1008031f0: eb01001f    	cmp	x0, x1
1008031f4: 54000a00    	b.eq	0x100803334 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h2317c90935a7f9edE+0x16c>
1008031f8: f9400848    	ldr	x8, [x2, #0x10]
1008031fc: cb000029    	sub	x9, x1, x0
