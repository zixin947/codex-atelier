
AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100242ae4 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E>:
100242ae4: d10503ff    	sub	sp, sp, #0x140
100242ae8: a90f67fa    	stp	x26, x25, [sp, #0xf0]
100242aec: a9105ff8    	stp	x24, x23, [sp, #0x100]
100242af0: a91157f6    	stp	x22, x21, [sp, #0x110]
100242af4: a9124ff4    	stp	x20, x19, [sp, #0x120]
100242af8: a9137bfd    	stp	x29, x30, [sp, #0x130]
100242afc: 9104c3fd    	add	x29, sp, #0x130
100242b00: aa0203f4    	mov	x20, x2
100242b04: aa0003f3    	mov	x19, x0
100242b08: f9400015    	ldr	x21, [x0]
100242b0c: b24107e8    	mov	x8, #-0x7fffffffffffffff ; =-9223372036854775807
100242b10: eb0802bf    	cmp	x21, x8
100242b14: 54000441    	b.ne	0x100242b9c <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0xb8>
100242b18: f1007a9f    	cmp	x20, #0x1e
100242b1c: 54000321    	b.ne	0x100242b80 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x9c>
100242b20: a9402428    	ldp	x8, x9, [x1]
100242b24: f940082a    	ldr	x10, [x1, #0x10]
100242b28: f841602b    	ldur	x11, [x1, #0x16]
100242b2c: d28e648c    	mov	x12, #0x7324            ; =29476
100242b30: f2ae4cac    	movk	x12, #0x7265, lsl #16
100242b34: f2ccac8c    	movk	x12, #0x6564, lsl #32
100242b38: f2ed4bec    	movk	x12, #0x6a5f, lsl #48
100242b3c: eb0c011f    	cmp	x8, x12
100242b40: d28dee68    	mov	x8, #0x6f73             ; =28531
100242b44: f2a74dc8    	movk	x8, #0x3a6e, lsl #16
100242b48: f2ce0748    	movk	x8, #0x703a, lsl #32
100242b4c: f2ed2e48    	movk	x8, #0x6972, lsl #48
100242b50: fa480120    	ccmp	x9, x8, #0x0, eq
100242b54: d28c2ec8    	mov	x8, #0x6176             ; =24950
100242b58: f2acae88    	movk	x8, #0x6574, lsl #16
100242b5c: f2c74748    	movk	x8, #0x3a3a, lsl #32
100242b60: f2ec2a48    	movk	x8, #0x6152, lsl #48
100242b64: fa480140    	ccmp	x10, x8, #0x0, eq
100242b68: d28c2a48    	mov	x8, #0x6152             ; =24914
100242b6c: f2aacee8    	movk	x8, #0x5677, lsl #16
100242b70: f2cd8c28    	movk	x8, #0x6c61, lsl #32
100242b74: f2ecaea8    	movk	x8, #0x6575, lsl #48
100242b78: fa480160    	ccmp	x11, x8, #0x0, eq
100242b7c: 540003c0    	b.eq	0x100242bf4 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x110>
100242b80: a9537bfd    	ldp	x29, x30, [sp, #0x130]
100242b84: a9524ff4    	ldp	x20, x19, [sp, #0x120]
100242b88: a95157f6    	ldp	x22, x21, [sp, #0x110]
100242b8c: a9505ff8    	ldp	x24, x23, [sp, #0x100]
100242b90: a94f67fa    	ldp	x26, x25, [sp, #0xf0]
100242b94: 910503ff    	add	sp, sp, #0x140
100242b98: 143e1e52    	b	0x1011ca4e0 <__ZN10serde_json5value3ser17invalid_raw_value17h1bd968611ad6424aE>
100242b9c: b6f800b4    	tbz	x20, #0x3f, 0x100242bb0 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0xcc>
100242ba0: d2800017    	mov	x23, #0x0               ; =0
100242ba4: aa1703e0    	mov	x0, x23
100242ba8: aa1403e1    	mov	x1, x20
100242bac: 943e36ae    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100242bb0: aa0303f8    	mov	x24, x3
100242bb4: b4000514    	cbz	x20, 0x100242c54 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x170>
100242bb8: aa0103f9    	mov	x25, x1
100242bbc: 97f6fa6e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100242bc0: 52800037    	mov	w23, #0x1               ; =1
100242bc4: aa1403e0    	mov	x0, x20
100242bc8: 52800021    	mov	w1, #0x1                ; =1
100242bcc: 97f6fa66    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100242bd0: b4fffea0    	cbz	x0, 0x100242ba4 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0xc0>
100242bd4: aa0003f6    	mov	x22, x0
100242bd8: aa1903e1    	mov	x1, x25
100242bdc: aa1403e2    	mov	x2, x20
100242be0: 943e3ab3    	bl	0x1011d16ac <_writev+0x1011d16ac>
100242be4: d2f00017    	mov	x23, #-0x8000000000000000 ; =-9223372036854775808
100242be8: eb1702bf    	cmp	x21, x23
100242bec: 54000421    	b.ne	0x100242c70 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x18c>
100242bf0: 14000025    	b	0x100242c84 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x1a0>
100242bf4: 39400068    	ldrb	w8, [x3]
100242bf8: 90008b40    	adrp	x0, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100242bfc: 913bd000    	add	x0, x0, #0xef4
100242c00: 7100051f    	cmp	w8, #0x1
100242c04: 54000e21    	b.ne	0x100242dc8 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x2e4>
100242c08: 90008b43    	adrp	x3, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100242c0c: 913c3063    	add	x3, x3, #0xf0c
100242c10: 9101c3e8    	add	x8, sp, #0x70
100242c14: 52800201    	mov	w1, #0x10               ; =16
100242c18: 52800022    	mov	w2, #0x1                ; =1
100242c1c: 52800124    	mov	w4, #0x9                ; =9
100242c20: 943e1ec0    	bl	0x1011ca720 <__ZN87_$LT$serde_json..value..ser..RawValueEmitter$u20$as$u20$serde_core..ser..Serializer$GT$22serialize_unit_variant17h39b1cf38b19761c3E>
100242c24: 3941c3e8    	ldrb	w8, [sp, #0x70]
100242c28: 7100191f    	cmp	w8, #0x6
100242c2c: 54000e20    	b.eq	0x100242df0 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x30c>
100242c30: 38408e68    	ldrb	w8, [x19, #0x8]!
100242c34: 7100191f    	cmp	w8, #0x6
100242c38: 54000060    	b.eq	0x100242c44 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x160>
100242c3c: aa1303e0    	mov	x0, x19
100242c40: 97ff9cd8    	bl	0x100229fa0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100242c44: d2800000    	mov	x0, #0x0                ; =0
100242c48: ad4387e0    	ldp	q0, q1, [sp, #0x70]
100242c4c: ad000660    	stp	q0, q1, [x19]
100242c50: 14000057    	b	0x100242dac <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x2c8>
100242c54: 52800036    	mov	w22, #0x1               ; =1
100242c58: aa1603e0    	mov	x0, x22
100242c5c: aa1403e2    	mov	x2, x20
100242c60: 943e3a93    	bl	0x1011d16ac <_writev+0x1011d16ac>
100242c64: d2f00017    	mov	x23, #-0x8000000000000000 ; =-9223372036854775808
100242c68: eb1702bf    	cmp	x21, x23
100242c6c: 540000c0    	b.eq	0x100242c84 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x1a0>
100242c70: b40000b5    	cbz	x21, 0x100242c84 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x1a0>
100242c74: f9400660    	ldr	x0, [x19, #0x8]
100242c78: aa1503e1    	mov	x1, x21
100242c7c: 52800022    	mov	w2, #0x1                ; =1
100242c80: 97f6fa3a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100242c84: a900d276    	stp	x22, x20, [x19, #0x8]
100242c88: f9000277    	str	x23, [x19]
100242c8c: 3cc08260    	ldur	q0, [x19, #0x8]
100242c90: 3c8083e0    	stur	q0, [sp, #0x8]
100242c94: f90003f4    	str	x20, [sp]
100242c98: 39400315    	ldrb	w21, [x24]
100242c9c: 97f6fa36    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100242ca0: 710006bf    	cmp	w21, #0x1
100242ca4: 540001c1    	b.ne	0x100242cdc <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x1f8>
100242ca8: 52800135    	mov	w21, #0x9               ; =9
100242cac: 52800120    	mov	w0, #0x9                ; =9
100242cb0: 52800021    	mov	w1, #0x1                ; =1
100242cb4: 97f6fa2c    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100242cb8: b4000a00    	cbz	x0, 0x100242df8 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x314>
100242cbc: 90008b48    	adrp	x8, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100242cc0: 913c3108    	add	x8, x8, #0xf0c
100242cc4: f9400109    	ldr	x9, [x8]
100242cc8: f9000009    	str	x9, [x0]
100242ccc: 39402108    	ldrb	w8, [x8, #0x8]
100242cd0: 39002008    	strb	w8, [x0, #0x8]
100242cd4: 52800128    	mov	w8, #0x9                ; =9
100242cd8: 1400000c    	b	0x100242d08 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x224>
100242cdc: 52800115    	mov	w21, #0x8               ; =8
100242ce0: 52800100    	mov	w0, #0x8                ; =8
100242ce4: 52800021    	mov	w1, #0x1                ; =1
100242ce8: 97f6fa1f    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100242cec: b4000860    	cbz	x0, 0x100242df8 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x314>
100242cf0: d28dee48    	mov	x8, #0x6f72             ; =28530
100242cf4: f2ae8ea8    	movk	x8, #0x7475, lsl #16
100242cf8: f2ce4ca8    	movk	x8, #0x7265, lsl #32
100242cfc: f2edc9e8    	movk	x8, #0x6e4f, lsl #48
100242d00: f9000008    	str	x8, [x0]
100242d04: 52800108    	mov	w8, #0x8                ; =8
100242d08: 52800069    	mov	w9, #0x3                ; =3
100242d0c: 390063e9    	strb	w9, [sp, #0x18]
100242d10: a90203e8    	stp	x8, x0, [sp, #0x20]
100242d14: f9001be8    	str	x8, [sp, #0x30]
100242d18: 9100e3e8    	add	x8, sp, #0x38
100242d1c: 91006260    	add	x0, x19, #0x18
100242d20: 910003e1    	mov	x1, sp
100242d24: 94144ec5    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
100242d28: f9401fe8    	ldr	x8, [sp, #0x38]
100242d2c: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
100242d30: eb09011f    	cmp	x8, x9
100242d34: 540001e1    	b.ne	0x100242d70 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x28c>
100242d38: f94023e8    	ldr	x8, [sp, #0x40]
100242d3c: f9402be9    	ldr	x9, [sp, #0x50]
100242d40: 8b091508    	add	x8, x8, x9, lsl #5
100242d44: ad400500    	ldp	q0, q1, [x8]
100242d48: ad3d07a0    	stp	q0, q1, [x29, #-0x60]
100242d4c: 3cc283e0    	ldur	q0, [sp, #0x28]
100242d50: 3cc183e1    	ldur	q1, [sp, #0x18]
100242d54: ad000101    	stp	q1, q0, [x8]
100242d58: 385a03a8    	ldurb	w8, [x29, #-0x60]
100242d5c: 7100191f    	cmp	w8, #0x6
100242d60: 54000240    	b.eq	0x100242da8 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x2c4>
100242d64: d10183a0    	sub	x0, x29, #0x60
100242d68: 97ff9c8e    	bl	0x100229fa0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100242d6c: 1400000f    	b	0x100242da8 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x2c4>
100242d70: 3cc383e0    	ldur	q0, [sp, #0x38]
100242d74: 3cc483e1    	ldur	q1, [sp, #0x48]
100242d78: ad0387e0    	stp	q0, q1, [sp, #0x70]
100242d7c: 3cc583e0    	ldur	q0, [sp, #0x58]
100242d80: 3d8027e0    	str	q0, [sp, #0x90]
100242d84: f94037e8    	ldr	x8, [sp, #0x68]
100242d88: f90053e8    	str	x8, [sp, #0xa0]
100242d8c: 3cc183e0    	ldur	q0, [sp, #0x18]
100242d90: 3cc283e1    	ldur	q1, [sp, #0x28]
100242d94: ad3c07a0    	stp	q0, q1, [x29, #-0x80]
100242d98: d10183a8    	sub	x8, x29, #0x60
100242d9c: 9101c3e0    	add	x0, sp, #0x70
100242da0: d10203a1    	sub	x1, x29, #0x80
100242da4: 940f2c57    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
100242da8: d2800000    	mov	x0, #0x0                ; =0
100242dac: a9537bfd    	ldp	x29, x30, [sp, #0x130]
100242db0: a9524ff4    	ldp	x20, x19, [sp, #0x120]
100242db4: a95157f6    	ldp	x22, x21, [sp, #0x110]
100242db8: a9505ff8    	ldp	x24, x23, [sp, #0x100]
100242dbc: a94f67fa    	ldp	x26, x25, [sp, #0xf0]
100242dc0: 910503ff    	add	sp, sp, #0x140
100242dc4: d65f03c0    	ret
100242dc8: 90008b43    	adrp	x3, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100242dcc: 913c1063    	add	x3, x3, #0xf04
100242dd0: 9101c3e8    	add	x8, sp, #0x70
100242dd4: 52800201    	mov	w1, #0x10               ; =16
100242dd8: 52800002    	mov	w2, #0x0                ; =0
100242ddc: 52800104    	mov	w4, #0x8                ; =8
100242de0: 943e1e50    	bl	0x1011ca720 <__ZN87_$LT$serde_json..value..ser..RawValueEmitter$u20$as$u20$serde_core..ser..Serializer$GT$22serialize_unit_variant17h39b1cf38b19761c3E>
100242de4: 3941c3e8    	ldrb	w8, [sp, #0x70]
100242de8: 7100191f    	cmp	w8, #0x6
100242dec: 54fff221    	b.ne	0x100242c30 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x14c>
100242df0: f9403fe0    	ldr	x0, [sp, #0x78]
100242df4: 17ffffee    	b	0x100242dac <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x2c8>
100242df8: 52800020    	mov	w0, #0x1                ; =1
100242dfc: aa1503e1    	mov	x1, x21
100242e00: 943e3619    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100242e04: d4200020    	brk	#0x1
100242e08: ad4387e0    	ldp	q0, q1, [sp, #0x70]
100242e0c: ad000660    	stp	q0, q1, [x19]
100242e10: 943e38d1    	bl	0x1011d1154 <_writev+0x1011d1154>
100242e14: aa0003f5    	mov	x21, x0
100242e18: 910063e0    	add	x0, sp, #0x18
100242e1c: 94140953    	bl	0x100745368 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100242e20: 14000008    	b	0x100242e40 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x35c>
100242e24: 943e3749    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
100242e28: aa0003f5    	mov	x21, x0
100242e2c: b40000b4    	cbz	x20, 0x100242e40 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E+0x35c>
100242e30: f94007e0    	ldr	x0, [sp, #0x8]
100242e34: aa1403e1    	mov	x1, x20
100242e38: 52800022    	mov	w2, #0x1                ; =1
100242e3c: 97f6f9cb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100242e40: aa1503e0    	mov	x0, x21
100242e44: 943e38c4    	bl	0x1011d1154 <_writev+0x1011d1154>
