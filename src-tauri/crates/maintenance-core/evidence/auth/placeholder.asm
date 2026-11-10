
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100388ce8 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE>:
100388ce8: d10103ff    	sub	sp, sp, #0x40
100388cec: a9024ff4    	stp	x20, x19, [sp, #0x20]
100388cf0: a9037bfd    	stp	x29, x30, [sp, #0x30]
100388cf4: 9100c3fd    	add	x29, sp, #0x30
100388cf8: aa0103f3    	mov	x19, x1
100388cfc: 910023e8    	add	x8, sp, #0x8
100388d00: 9437bb52    	bl	0x101177a48 <__ZN5alloc3str21_$LT$impl$u20$str$GT$12to_lowercase17h5fb7aebff6ad1647E>
100388d04: a94123e0    	ldp	x0, x8, [sp, #0x10]
100388d08: f100211f    	cmp	x8, #0x8
100388d0c: 540001ec    	b.gt	0x100388d48 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x60>
100388d10: f100111f    	cmp	x8, #0x4
100388d14: 540004c0    	b.eq	0x100388dac <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0xc4>
100388d18: f1001d1f    	cmp	x8, #0x7
100388d1c: 540003a1    	b.ne	0x100388d90 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0xa8>
100388d20: b9400008    	ldr	w8, [x0]
100388d24: b8403009    	ldur	w9, [x0, #0x3]
100388d28: 528dcd2a    	mov	w10, #0x6e69            ; =28265
100388d2c: 72ac2eca    	movk	w10, #0x6176, lsl #16
100388d30: 6b0a011f    	cmp	w8, w10
100388d34: 528d8c28    	mov	w8, #0x6c61             ; =27745
100388d38: 72ac8d28    	movk	w8, #0x6469, lsl #16
100388d3c: 7a480120    	ccmp	w9, w8, #0x0, eq
100388d40: 54000c61    	b.ne	0x100388ecc <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x1e4>
100388d44: 14000079    	b	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388d48: f100251f    	cmp	x8, #0x9
100388d4c: 54000640    	b.eq	0x100388e14 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x12c>
100388d50: f1002d1f    	cmp	x8, #0xb
100388d54: 540001e1    	b.ne	0x100388d90 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0xa8>
100388d58: f9400008    	ldr	x8, [x0]
100388d5c: f8403009    	ldur	x9, [x0, #0x3]
100388d60: d28d8e0a    	mov	x10, #0x6c70            ; =27760
100388d64: f2ac6c2a    	movk	x10, #0x6361, lsl #16
100388d68: f2cd0caa    	movk	x10, #0x6865, lsl #32
100388d6c: f2ed8dea    	movk	x10, #0x6c6f, lsl #48
100388d70: eb0a011f    	cmp	x8, x10
100388d74: d28cac68    	mov	x8, #0x6563             ; =25955
100388d78: f2aded08    	movk	x8, #0x6f68, lsl #16
100388d7c: f2cc8d88    	movk	x8, #0x646c, lsl #32
100388d80: f2ee4ca8    	movk	x8, #0x7265, lsl #48
100388d84: fa480120    	ccmp	x9, x8, #0x0, eq
100388d88: 540005a1    	b.ne	0x100388e3c <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x154>
100388d8c: 14000067    	b	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388d90: f100251f    	cmp	x8, #0x9
100388d94: 54000542    	b.hs	0x100388e3c <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x154>
100388d98: f1001d1f    	cmp	x8, #0x7
100388d9c: 54000980    	b.eq	0x100388ecc <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x1e4>
100388da0: f100211f    	cmp	x8, #0x8
100388da4: 540006c0    	b.eq	0x100388e7c <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x194>
100388da8: 14000043    	b	0x100388eb4 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x1cc>
100388dac: b9400008    	ldr	w8, [x0]
100388db0: 528c2cc9    	mov	w9, #0x6166             ; =24934
100388db4: 72acad69    	movk	w9, #0x656b, lsl #16
100388db8: 6b09011f    	cmp	w8, w9
100388dbc: 54000b60    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388dc0: b9400008    	ldr	w8, [x0]
100388dc4: 528cae89    	mov	w9, #0x6574             ; =25972
100388dc8: 72ae8e69    	movk	w9, #0x7473, lsl #16
100388dcc: 6b09011f    	cmp	w8, w9
100388dd0: 54000ac0    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388dd4: b9400008    	ldr	w8, [x0]
100388dd8: 528deda9    	mov	w9, #0x6f6d             ; =28525
100388ddc: 72ad6c69    	movk	w9, #0x6b63, lsl #16
100388de0: 6b09011f    	cmp	w8, w9
100388de4: 54000a20    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388de8: b9400008    	ldr	w8, [x0]
100388dec: 528eadc9    	mov	w9, #0x756e             ; =30062
100388df0: 72ad8d89    	movk	w9, #0x6c6c, lsl #16
100388df4: 6b09011f    	cmp	w8, w9
100388df8: 54000980    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388dfc: b9400008    	ldr	w8, [x0]
100388e00: 528dedc9    	mov	w9, #0x6f6e             ; =28526
100388e04: 72acadc9    	movk	w9, #0x656e, lsl #16
100388e08: 6b09011f    	cmp	w8, w9
100388e0c: 54000541    	b.ne	0x100388eb4 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x1cc>
100388e10: 14000046    	b	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388e14: f9400008    	ldr	x8, [x0]
100388e18: 39402009    	ldrb	w9, [x0, #0x8]
100388e1c: d28dceaa    	mov	x10, #0x6e75            ; =28277
100388e20: f2acac8a    	movk	x10, #0x6564, lsl #16
100388e24: f2cd2cca    	movk	x10, #0x6966, lsl #32
100388e28: f2ecadca    	movk	x10, #0x656e, lsl #48
100388e2c: eb0a011f    	cmp	x8, x10
100388e30: 52800c88    	mov	w8, #0x64               ; =100
100388e34: 7a480120    	ccmp	w9, w8, #0x0, eq
100388e38: 54000780    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388e3c: d28c6c68    	mov	x8, #0x6363             ; =25443
100388e40: f2ae65a8    	movk	x8, #0x732d, lsl #16
100388e44: f2cd2ee8    	movk	x8, #0x6977, lsl #32
100388e48: f2ec6e88    	movk	x8, #0x6374, lsl #48
100388e4c: f9400009    	ldr	x9, [x0]
100388e50: 3940200a    	ldrb	w10, [x0, #0x8]
100388e54: eb08013f    	cmp	x9, x8
100388e58: 52800d09    	mov	w9, #0x68               ; =104
100388e5c: 7a490140    	ccmp	w10, w9, #0x0, eq
100388e60: 54000640    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388e64: f940000a    	ldr	x10, [x0]
100388e68: 914c8108    	add	x8, x8, #0x320, lsl #12 ; =0x320000
100388e6c: 3940200b    	ldrb	w11, [x0, #0x8]
100388e70: eb0a011f    	cmp	x8, x10
100388e74: 7a490160    	ccmp	w11, w9, #0x0, eq
100388e78: 54000580    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388e7c: f9400008    	ldr	x8, [x0]
100388e80: d28c6c69    	mov	x9, #0x6363             ; =25443
100388e84: f2aeee69    	movk	x9, #0x7773, lsl #16
100388e88: f2ce8d29    	movk	x9, #0x7469, lsl #32
100388e8c: f2ed0c69    	movk	x9, #0x6863, lsl #48
100388e90: eb09011f    	cmp	x8, x9
100388e94: 540004a0    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388e98: f9400008    	ldr	x8, [x0]
100388e9c: d28cac89    	mov	x9, #0x6564             ; =25956
100388ea0: f2ac6ca9    	movk	x9, #0x6365, lsl #16
100388ea4: f2cc8de9    	movk	x9, #0x646f, lsl #32
100388ea8: f2ef0ca9    	movk	x9, #0x7865, lsl #48
100388eac: eb09011f    	cmp	x8, x9
100388eb0: 540003c0    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388eb4: 52800008    	mov	w8, #0x0                ; =0
100388eb8: f1002a7f    	cmp	x19, #0xa
100388ebc: 1a9f2513    	csinc	w19, w8, wzr, hs
100388ec0: f94007e1    	ldr	x1, [sp, #0x8]
100388ec4: b5000381    	cbnz	x1, 0x100388f34 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x24c>
100388ec8: 1400001d    	b	0x100388f3c <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x254>
100388ecc: b9400008    	ldr	w8, [x0]
100388ed0: b8403009    	ldur	w9, [x0, #0x3]
100388ed4: 528d6e6a    	mov	w10, #0x6b73            ; =27507
100388ed8: 72acc5aa    	movk	w10, #0x662d, lsl #16
100388edc: 6b0a011f    	cmp	w8, w10
100388ee0: 528c2cc8    	mov	w8, #0x6166             ; =24934
100388ee4: 72acad68    	movk	w8, #0x656b, lsl #16
100388ee8: 7a480120    	ccmp	w9, w8, #0x0, eq
100388eec: 540001e0    	b.eq	0x100388f28 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x240>
100388ef0: b9400008    	ldr	w8, [x0]
100388ef4: b8403009    	ldur	w9, [x0, #0x3]
100388ef8: 528d6e6a    	mov	w10, #0x6b73            ; =27507
100388efc: 72ae85aa    	movk	w10, #0x742d, lsl #16
100388f00: 6b0a011f    	cmp	w8, w10
100388f04: 528cae88    	mov	w8, #0x6574             ; =25972
100388f08: 72ae8e68    	movk	w8, #0x7473, lsl #16
100388f0c: 7a480120    	ccmp	w9, w8, #0x0, eq
100388f10: 1a9f17e8    	cset	w8, eq
100388f14: f1002a7f    	cmp	x19, #0xa
100388f18: 1a9f2513    	csinc	w19, w8, wzr, hs
100388f1c: f94007e1    	ldr	x1, [sp, #0x8]
100388f20: b50000a1    	cbnz	x1, 0x100388f34 <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x24c>
100388f24: 14000006    	b	0x100388f3c <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x254>
100388f28: 52800033    	mov	w19, #0x1               ; =1
100388f2c: f94007e1    	ldr	x1, [sp, #0x8]
100388f30: b4000061    	cbz	x1, 0x100388f3c <__ZN13codexmate_lib4core5relay16codex_diagnostic26is_known_placeholder_token17h5c5279a0d959e28aE+0x254>
100388f34: 52800022    	mov	w2, #0x1                ; =1
100388f38: 97f1e18c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100388f3c: aa1303e0    	mov	x0, x19
100388f40: a9437bfd    	ldp	x29, x30, [sp, #0x30]
100388f44: a9424ff4    	ldp	x20, x19, [sp, #0x20]
100388f48: 910103ff    	add	sp, sp, #0x40
100388f4c: d65f03c0    	ret
