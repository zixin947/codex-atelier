
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001001f9bc8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE>:
1001f9bc8: a9bc5ff8    	stp	x24, x23, [sp, #-0x40]!
1001f9bcc: a90157f6    	stp	x22, x21, [sp, #0x10]
1001f9bd0: a9024ff4    	stp	x20, x19, [sp, #0x20]
1001f9bd4: a9037bfd    	stp	x29, x30, [sp, #0x30]
1001f9bd8: 9100c3fd    	add	x29, sp, #0x30
1001f9bdc: aa0003f5    	mov	x21, x0
1001f9be0: aa0103e0    	mov	x0, x1
1001f9be4: aa0203e1    	mov	x1, x2
1001f9be8: 9400d55c    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1001f9bec: aa0103f3    	mov	x19, x1
1001f9bf0: b6f800a1    	tbz	x1, #0x3f, 0x1001f9c04 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x3c>
1001f9bf4: d2800016    	mov	x22, #0x0               ; =0
1001f9bf8: aa1603e0    	mov	x0, x22
1001f9bfc: aa1303e1    	mov	x1, x19
1001f9c00: 943f5a99    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1001f9c04: b4000293    	cbz	x19, 0x1001f9c54 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x8c>
1001f9c08: aa0003f7    	mov	x23, x0
1001f9c0c: 97f81e5a    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1001f9c10: 52800036    	mov	w22, #0x1               ; =1
1001f9c14: aa1303e0    	mov	x0, x19
1001f9c18: 52800021    	mov	w1, #0x1                ; =1
1001f9c1c: 97f81e52    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1001f9c20: b4fffec0    	cbz	x0, 0x1001f9bf8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x30>
1001f9c24: aa0003f4    	mov	x20, x0
1001f9c28: d2800009    	mov	x9, #0x0                ; =0
1001f9c2c: f100227f    	cmp	x19, #0x8
1001f9c30: aa1703e8    	mov	x8, x23
1001f9c34: 54000623    	b.lo	0x1001f9cf8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x130>
1001f9c38: cb08028a    	sub	x10, x20, x8
1001f9c3c: f100fd5f    	cmp	x10, #0x3f
1001f9c40: 540005c9    	b.ls	0x1001f9cf8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x130>
1001f9c44: f101027f    	cmp	x19, #0x40
1001f9c48: 540000a2    	b.hs	0x1001f9c5c <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x94>
1001f9c4c: d2800009    	mov	x9, #0x0                ; =0
1001f9c50: 1400001b    	b	0x1001f9cbc <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0xf4>
1001f9c54: 52800034    	mov	w20, #0x1               ; =1
1001f9c58: 14000032    	b	0x1001f9d20 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x158>
1001f9c5c: 927ae269    	and	x9, x19, #0x7fffffffffffffc0
1001f9c60: 9100810a    	add	x10, x8, #0x20
1001f9c64: 9100828b    	add	x11, x20, #0x20
1001f9c68: 4f02e780    	movi.16b	v0, #0x5c
1001f9c6c: 4f01e5e1    	movi.16b	v1, #0x2f
1001f9c70: aa0903ec    	mov	x12, x9
1001f9c74: ad7f0d42    	ldp	q2, q3, [x10, #-0x20]
1001f9c78: acc21544    	ldp	q4, q5, [x10], #0x40
1001f9c7c: 6e208c46    	cmeq.16b	v6, v2, v0
1001f9c80: 6e208c67    	cmeq.16b	v7, v3, v0
1001f9c84: 6e208c90    	cmeq.16b	v16, v4, v0
1001f9c88: 6e208cb1    	cmeq.16b	v17, v5, v0
1001f9c8c: 6ea61c22    	bit.16b	v2, v1, v6
1001f9c90: 6ea71c23    	bit.16b	v3, v1, v7
1001f9c94: 6eb01c24    	bit.16b	v4, v1, v16
1001f9c98: 6eb11c25    	bit.16b	v5, v1, v17
1001f9c9c: ad3f0d62    	stp	q2, q3, [x11, #-0x20]
1001f9ca0: ac821564    	stp	q4, q5, [x11], #0x40
1001f9ca4: f101018c    	subs	x12, x12, #0x40
1001f9ca8: 54fffe61    	b.ne	0x1001f9c74 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0xac>
1001f9cac: eb09027f    	cmp	x19, x9
1001f9cb0: 54000380    	b.eq	0x1001f9d20 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x158>
1001f9cb4: f27d0a7f    	tst	x19, #0x38
1001f9cb8: 54000200    	b.eq	0x1001f9cf8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x130>
1001f9cbc: aa0903ec    	mov	x12, x9
1001f9cc0: 927dee69    	and	x9, x19, #0x7ffffffffffffff8
1001f9cc4: cb09018a    	sub	x10, x12, x9
1001f9cc8: 8b0c028b    	add	x11, x20, x12
1001f9ccc: 8b0c010c    	add	x12, x8, x12
1001f9cd0: 0f02e780    	movi.8b	v0, #0x5c
1001f9cd4: 0f01e5e1    	movi.8b	v1, #0x2f
1001f9cd8: fc408582    	ldr	d2, [x12], #0x8
1001f9cdc: 2e208c43    	cmeq.8b	v3, v2, v0
1001f9ce0: 2ea31c22    	bit.8b	v2, v1, v3
1001f9ce4: fc008562    	str	d2, [x11], #0x8
1001f9ce8: b100214a    	adds	x10, x10, #0x8
1001f9cec: 54ffff61    	b.ne	0x1001f9cd8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x110>
1001f9cf0: eb09027f    	cmp	x19, x9
1001f9cf4: 54000160    	b.eq	0x1001f9d20 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x158>
1001f9cf8: cb09026a    	sub	x10, x19, x9
1001f9cfc: 8b09028b    	add	x11, x20, x9
1001f9d00: 8b090108    	add	x8, x8, x9
1001f9d04: 528005e9    	mov	w9, #0x2f               ; =47
1001f9d08: 3840150c    	ldrb	w12, [x8], #0x1
1001f9d0c: 7101719f    	cmp	w12, #0x5c
1001f9d10: 1a8c012c    	csel	w12, w9, w12, eq
1001f9d14: 3800156c    	strb	w12, [x11], #0x1
1001f9d18: f100054a    	subs	x10, x10, #0x1
1001f9d1c: 54ffff61    	b.ne	0x1001f9d08 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x140>
1001f9d20: 52800037    	mov	w23, #0x1               ; =1
1001f9d24: aa1303e9    	mov	x9, x19
1001f9d28: 14000007    	b	0x1001f9d44 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x17c>
1001f9d2c: 1200114a    	and	w10, w10, #0x1f
1001f9d30: 12001508    	and	w8, w8, #0x3f
1001f9d34: 2a0a1908    	orr	w8, w8, w10, lsl #6
1001f9d38: cb140129    	sub	x9, x9, x20
1001f9d3c: 7100bd1f    	cmp	w8, #0x2f
1001f9d40: 54000341    	b.ne	0x1001f9da8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x1e0>
1001f9d44: aa0903f6    	mov	x22, x9
1001f9d48: b4000489    	cbz	x9, 0x1001f9dd8 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x210>
1001f9d4c: 8b16028b    	add	x11, x20, x22
1001f9d50: aa0b03e9    	mov	x9, x11
1001f9d54: 38dffd28    	ldrsb	w8, [x9, #-0x1]!
1001f9d58: 36ffff08    	tbz	w8, #0x1f, 0x1001f9d38 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x170>
1001f9d5c: aa0b03e9    	mov	x9, x11
1001f9d60: 38dfed2a    	ldrsb	w10, [x9, #-0x2]!
1001f9d64: 3101015f    	cmn	w10, #0x40
1001f9d68: 54fffe2a    	b.ge	0x1001f9d2c <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x164>
1001f9d6c: aa0b03e9    	mov	x9, x11
1001f9d70: 38dfdd2c    	ldrsb	w12, [x9, #-0x3]!
1001f9d74: 3101019f    	cmn	w12, #0x40
1001f9d78: 540000ea    	b.ge	0x1001f9d94 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x1cc>
1001f9d7c: 12001d89    	and	w9, w12, #0xff
1001f9d80: 385fcd6d    	ldrb	w13, [x11, #-0x4]!
1001f9d84: 1200152c    	and	w12, w9, #0x3f
1001f9d88: 331a09ac    	bfi	w12, w13, #6, #3
1001f9d8c: aa0b03e9    	mov	x9, x11
1001f9d90: 14000002    	b	0x1001f9d98 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x1d0>
1001f9d94: 12000d8c    	and	w12, w12, #0xf
1001f9d98: 12001d4a    	and	w10, w10, #0xff
1001f9d9c: 1200154a    	and	w10, w10, #0x3f
1001f9da0: 2a0c194a    	orr	w10, w10, w12, lsl #6
1001f9da4: 17ffffe3    	b	0x1001f9d30 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x168>
1001f9da8: b6f800b6    	tbz	x22, #0x3f, 0x1001f9dbc <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x1f4>
1001f9dac: d2800000    	mov	x0, #0x0                ; =0
1001f9db0: aa1603e1    	mov	x1, x22
1001f9db4: 943f5a2c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1001f9db8: d4200020    	brk	#0x1
1001f9dbc: 97f81dee    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1001f9dc0: aa1603e0    	mov	x0, x22
1001f9dc4: 52800021    	mov	w1, #0x1                ; =1
1001f9dc8: 97f81de7    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1001f9dcc: aa0003f7    	mov	x23, x0
1001f9dd0: 52800020    	mov	w0, #0x1                ; =1
1001f9dd4: b4fffef7    	cbz	x23, 0x1001f9db0 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x1e8>
1001f9dd8: aa1703e0    	mov	x0, x23
1001f9ddc: aa1403e1    	mov	x1, x20
1001f9de0: aa1603e2    	mov	x2, x22
1001f9de4: 943f5e32    	bl	0x1011d16ac <_writev+0x1011d16ac>
1001f9de8: a9005eb6    	stp	x22, x23, [x21]
1001f9dec: f9000ab6    	str	x22, [x21, #0x10]
1001f9df0: b4000133    	cbz	x19, 0x1001f9e14 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x24c>
1001f9df4: aa1403e0    	mov	x0, x20
1001f9df8: aa1303e1    	mov	x1, x19
1001f9dfc: 52800022    	mov	w2, #0x1                ; =1
1001f9e00: a9437bfd    	ldp	x29, x30, [sp, #0x30]
1001f9e04: a9424ff4    	ldp	x20, x19, [sp, #0x20]
1001f9e08: a94157f6    	ldp	x22, x21, [sp, #0x10]
1001f9e0c: a8c45ff8    	ldp	x24, x23, [sp], #0x40
1001f9e10: 17f81dd6    	b	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f9e14: a9437bfd    	ldp	x29, x30, [sp, #0x30]
1001f9e18: a9424ff4    	ldp	x20, x19, [sp, #0x20]
1001f9e1c: a94157f6    	ldp	x22, x21, [sp, #0x10]
1001f9e20: a8c45ff8    	ldp	x24, x23, [sp], #0x40
1001f9e24: d65f03c0    	ret
1001f9e28: b40000f3    	cbz	x19, 0x1001f9e44 <__ZN13codexmate_lib4core5relay15config_takeover22normalize_catalog_path17h68eacb2edf5cfefdE+0x27c>
1001f9e2c: aa0003f5    	mov	x21, x0
1001f9e30: aa1403e0    	mov	x0, x20
1001f9e34: aa1303e1    	mov	x1, x19
1001f9e38: 52800022    	mov	w2, #0x1                ; =1
1001f9e3c: 97f81dcb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f9e40: aa1503e0    	mov	x0, x21
1001f9e44: 943f5cc4    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001001f9e48 <__ZN13codexmate_lib4core5relay15config_takeover24is_foreign_model_section17hd4a710f2ae2f74d8E>:
1001f9e48: d10103ff    	sub	sp, sp, #0x40
1001f9e4c: a9024ff4    	stp	x20, x19, [sp, #0x20]
1001f9e50: a9037bfd    	stp	x29, x30, [sp, #0x30]
1001f9e54: 9100c3fd    	add	x29, sp, #0x30
1001f9e58: f1004022    	subs	x2, x1, #0x10
1001f9e5c: 54000082    	b.hs	0x1001f9e6c <__ZN13codexmate_lib4core5relay15config_takeover24is_foreign_model_section17hd4a710f2ae2f74d8E+0x24>
1001f9e60: f100243f    	cmp	x1, #0x9
1001f9e64: 540001c2    	b.hs	0x1001f9e9c <__ZN13codexmate_lib4core5relay15config_takeover24is_foreign_model_section17hd4a710f2ae2f74d8E+0x54>
1001f9e68: 1400001f    	b	0x1001f9ee4 <__ZN13codexmate_lib4core5relay15config_takeover24is_foreign_model_section17hd4a710f2ae2f74d8E+0x9c>
1001f9e6c: a9402408    	ldp	x8, x9, [x0]
1001f9e70: d28dedaa    	mov	x10, #0x6f6d            ; =28525
1001f9e74: f2acac8a    	movk	x10, #0x6564, lsl #16
1001f9e78: f2cbed8a    	movk	x10, #0x5f6c, lsl #32
1001f9e7c: f2ee4e0a    	movk	x10, #0x7270, lsl #48
1001f9e80: eb0a011f    	cmp	x8, x10
1001f9e84: d28ecde8    	mov	x8, #0x766f             ; =30319
1001f9e88: f2ac8d28    	movk	x8, #0x6469, lsl #16
1001f9e8c: f2ce4ca8    	movk	x8, #0x7265, lsl #32
1001f9e90: f2e5ce68    	movk	x8, #0x2e73, lsl #48
1001f9e94: fa480120    	ccmp	x9, x8, #0x0, eq
1001f9e98: 54000300    	b.eq	0x1001f9ef8 <__ZN13codexmate_lib4core5relay15config_takeover24is_foreign_model_section17hd4a710f2ae2f74d8E+0xb0>
1001f9e9c: f9400008    	ldr	x8, [x0]
1001f9ea0: 39402009    	ldrb	w9, [x0, #0x8]
1001f9ea4: d28e4e0a    	mov	x10, #0x7270            ; =29296
1001f9ea8: f2accdea    	movk	x10, #0x666f, lsl #16
1001f9eac: f2cd8d2a    	movk	x10, #0x6c69, lsl #32
1001f9eb0: f2ee6caa    	movk	x10, #0x7365, lsl #48
1001f9eb4: eb0a011f    	cmp	x8, x10
1001f9eb8: 528005c8    	mov	w8, #0x2e               ; =46
1001f9ebc: 7a480120    	ccmp	w9, w8, #0x0, eq
1001f9ec0: 54000121    	b.ne	0x1001f9ee4 <__ZN13codexmate_lib4core5relay15config_takeover24is_foreign_model_section17hd4a710f2ae2f74d8E+0x9c>
1001f9ec4: d1002422    	sub	x2, x1, #0x9
1001f9ec8: 91002401    	add	x1, x0, #0x9
1001f9ecc: 910023e0    	add	x0, sp, #0x8
