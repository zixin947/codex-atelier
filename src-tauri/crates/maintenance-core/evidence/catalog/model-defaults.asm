
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100489798 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E>:
100489b18: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100489b1c: f94027e9    	ldr	x9, [sp, #0x48]
100489b20: eb08013f    	cmp	x9, x8
100489b24: 540011c1    	b.ne	0x100489d5c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x5c4>
100489b28: 90007488    	adrp	x8, 0x101319000 <_anon.b5f536351c4bf1e454cf90283dd9e625.167+0x8>
100489b2c: 9126c908    	add	x8, x8, #0x9b2
100489b30: 528000a9    	mov	w9, #0x5                ; =5
100489b34: a93527a8    	stp	x8, x9, [x29, #-0xb0]
100489b38: d102c3a8    	sub	x8, x29, #0xb0
100489b3c: 90001aa9    	adrp	x9, 0x1007dd000 <__ZN3zip9zipcrypto24ZipCryptoReader$LT$R$GT$8validate17h6abfe2f5c5f69c24E+0xf0>
100489b40: 911b0129    	add	x9, x9, #0x6c0
100489b44: a93727a8    	stp	x8, x9, [x29, #-0x90]
100489b48: d2f0001b    	mov	x27, #-0x8000000000000000 ; =-9223372036854775808
100489b4c: 900084e0    	adrp	x0, 0x101525000 <_anon.a9e2253c03df711f51070ce6102367ea.957+0x57>
100489b50: 91157800    	add	x0, x0, #0x55e
100489b54: d10243a1    	sub	x1, x29, #0x90
100489b58: f94023f5    	ldr	x21, [sp, #0x40]
100489b5c: 94340c0f    	bl	0x10118cb98 <__ZN66_$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$6custom17h984fbb9098575a39E>
100489b60: 52800048    	mov	w8, #0x2                ; =2
100489b64: f94017e9    	ldr	x9, [sp, #0x28]
100489b68: a9000128    	stp	x8, x0, [x9]
100489b6c: 52800033    	mov	w19, #0x1               ; =1
100489b70: aa1503e1    	mov	x1, x21
100489b74: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100489b78: eb0802bf    	cmp	x21, x8
100489b7c: 54000a81    	b.ne	0x100489ccc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x534>
100489b80: 14000057    	b	0x100489cdc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x544>
100489b84: b0007400    	adrp	x0, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100489b88: 9127bc00    	add	x0, x0, #0x9ef
100489b8c: 52800201    	mov	w1, #0x10               ; =16
100489b90: 94340c29    	bl	0x10118cc34 <__ZN67_$LT$serde_json..error..Error$u20$as$u20$serde_core..ser..Error$GT$6custom17hd308ce5bb9ca103cE>
100489b94: 14000046    	b	0x100489cac <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x514>
100489b98: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100489b9c: f90023e8    	str	x8, [sp, #0x40]
100489ba0: b0007400    	adrp	x0, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100489ba4: 9127bc00    	add	x0, x0, #0x9ef
100489ba8: 52800201    	mov	w1, #0x10               ; =16
100489bac: 94340c22    	bl	0x10118cc34 <__ZN67_$LT$serde_json..error..Error$u20$as$u20$serde_core..ser..Error$GT$6custom17hd308ce5bb9ca103cE>
100489bb0: f9001be0    	str	x0, [sp, #0x30]
100489bb4: 52800048    	mov	w8, #0x2                ; =2
100489bb8: a942a7ea    	ldp	x10, x9, [sp, #0x28]
100489bbc: a9002548    	stp	x8, x9, [x10]
100489bc0: 52800033    	mov	w19, #0x1               ; =1
100489bc4: f94027fb    	ldr	x27, [sp, #0x48]
100489bc8: 14000045    	b	0x100489cdc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x544>
100489bcc: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100489bd0: f90027e8    	str	x8, [sp, #0x48]
100489bd4: b0007400    	adrp	x0, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100489bd8: 9127bc00    	add	x0, x0, #0x9ef
100489bdc: 52800201    	mov	w1, #0x10               ; =16
100489be0: 94340c15    	bl	0x10118cc34 <__ZN67_$LT$serde_json..error..Error$u20$as$u20$serde_core..ser..Error$GT$6custom17hd308ce5bb9ca103cE>
100489be4: aa0003f3    	mov	x19, x0
100489be8: 52800048    	mov	w8, #0x2                ; =2
100489bec: f94017e9    	ldr	x9, [sp, #0x28]
100489bf0: a9004d28    	stp	x8, x19, [x9]
100489bf4: 52800033    	mov	w19, #0x1               ; =1
100489bf8: d2f0001b    	mov	x27, #-0x8000000000000000 ; =-9223372036854775808
100489bfc: f94023e1    	ldr	x1, [sp, #0x40]
100489c00: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100489c04: eb08003f    	cmp	x1, x8
100489c08: 54000621    	b.ne	0x100489ccc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x534>
100489c0c: 14000034    	b	0x100489cdc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x544>
100489c10: b0007400    	adrp	x0, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100489c14: 9127bc00    	add	x0, x0, #0x9ef
100489c18: 52800201    	mov	w1, #0x10               ; =16
100489c1c: 94340c06    	bl	0x10118cc34 <__ZN67_$LT$serde_json..error..Error$u20$as$u20$serde_core..ser..Error$GT$6custom17hd308ce5bb9ca103cE>
100489c20: 14000023    	b	0x100489cac <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x514>
100489c24: b0007400    	adrp	x0, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100489c28: 9127bc00    	add	x0, x0, #0x9ef
100489c2c: 52800201    	mov	w1, #0x10               ; =16
100489c30: 94340c01    	bl	0x10118cc34 <__ZN67_$LT$serde_json..error..Error$u20$as$u20$serde_core..ser..Error$GT$6custom17hd308ce5bb9ca103cE>
100489c34: f81583a0    	stur	x0, [x29, #-0xa8]
100489c38: f85583a8    	ldur	x8, [x29, #-0xa8]
100489c3c: 52800049    	mov	w9, #0x2                ; =2
100489c40: f94017ea    	ldr	x10, [sp, #0x28]
100489c44: a9002149    	stp	x9, x8, [x10]
100489c48: 52800033    	mov	w19, #0x1               ; =1
100489c4c: a9446fe1    	ldp	x1, x27, [sp, #0x40]
100489c50: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100489c54: eb08003f    	cmp	x1, x8
100489c58: 540003a1    	b.ne	0x100489ccc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x534>
100489c5c: 14000020    	b	0x100489cdc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x544>
100489c60: b24107e8    	mov	x8, #-0x7fffffffffffffff ; =-9223372036854775807
100489c64: f9002be8    	str	x8, [sp, #0x50]
100489c68: b0007400    	adrp	x0, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100489c6c: 9127bc00    	add	x0, x0, #0x9ef
100489c70: 52800201    	mov	w1, #0x10               ; =16
100489c74: 94340bf0    	bl	0x10118cc34 <__ZN67_$LT$serde_json..error..Error$u20$as$u20$serde_core..ser..Error$GT$6custom17hd308ce5bb9ca103cE>
100489c78: f90013e0    	str	x0, [sp, #0x20]
100489c7c: 52800048    	mov	w8, #0x2                ; =2
100489c80: a94227ea    	ldp	x10, x9, [sp, #0x20]
100489c84: a9002928    	stp	x8, x10, [x9]
100489c88: 52800033    	mov	w19, #0x1               ; =1
100489c8c: b24107e8    	mov	x8, #-0x7fffffffffffffff ; =-9223372036854775807
100489c90: f9002be8    	str	x8, [sp, #0x50]
100489c94: a9446fe1    	ldp	x1, x27, [sp, #0x40]
100489c98: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100489c9c: eb08003f    	cmp	x1, x8
100489ca0: 54000161    	b.ne	0x100489ccc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x534>
100489ca4: 1400000e    	b	0x100489cdc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x544>
100489ca8: f9400fe0    	ldr	x0, [sp, #0x18]
100489cac: 52800048    	mov	w8, #0x2                ; =2
100489cb0: f94017e9    	ldr	x9, [sp, #0x28]
100489cb4: a9000128    	stp	x8, x0, [x9]
100489cb8: 52800033    	mov	w19, #0x1               ; =1
100489cbc: a9446fe1    	ldp	x1, x27, [sp, #0x40]
100489cc0: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100489cc4: eb08003f    	cmp	x1, x8
100489cc8: 540000a0    	b.eq	0x100489cdc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x544>
100489ccc: b4000081    	cbz	x1, 0x100489cdc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x544>
100489cd0: f9401be0    	ldr	x0, [sp, #0x30]
100489cd4: 52800022    	mov	w2, #0x1                ; =1
100489cd8: 97edde24    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100489cdc: d2800048    	mov	x8, #0x2                ; =2
100489ce0: f2f00008    	movk	x8, #0x8000, lsl #48
100489ce4: f9402be1    	ldr	x1, [sp, #0x50]
100489ce8: eb08003f    	cmp	x1, x8
100489cec: 540000ab    	b.lt	0x100489d00 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x568>
100489cf0: b4000081    	cbz	x1, 0x100489d00 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x568>
100489cf4: aa1403e0    	mov	x0, x20
100489cf8: 52800022    	mov	w2, #0x1                ; =1
100489cfc: 97edde1b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100489d00: f240fb7f    	tst	x27, #0x7fffffffffffffff
100489d04: 52000268    	eor	w8, w19, #0x1
100489d08: 1a9f1508    	csinc	w8, w8, wzr, ne
100489d0c: 370000a8    	tbnz	w8, #0x0, 0x100489d20 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x588>
100489d10: aa1903e0    	mov	x0, x25
100489d14: aa1b03e1    	mov	x1, x27
100489d18: 52800022    	mov	w2, #0x1                ; =1
100489d1c: 97edde13    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100489d20: 910163e0    	add	x0, sp, #0x58
100489d24: 940b9e29    	bl	0x1007715c8 <__ZN99_$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5ef754463675f4dcE>
100489d28: 394283e8    	ldrb	w8, [sp, #0xa0]
100489d2c: 7100191f    	cmp	w8, #0x6
100489d30: 54000060    	b.eq	0x100489d3c <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x5a4>
100489d34: 91012380    	add	x0, x28, #0x48
100489d38: 94010c86    	bl	0x1004ccf50 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100489d3c: a95d7bfd    	ldp	x29, x30, [sp, #0x1d0]
100489d40: a95c4ff4    	ldp	x20, x19, [sp, #0x1c0]
100489d44: a95b57f6    	ldp	x22, x21, [sp, #0x1b0]
100489d48: a95a5ff8    	ldp	x24, x23, [sp, #0x1a0]
100489d4c: a95967fa    	ldp	x26, x25, [sp, #0x190]
100489d50: a9586ffc    	ldp	x28, x27, [sp, #0x180]
100489d54: 910783ff    	add	sp, sp, #0x1e0
100489d58: d65f03c0    	ret
100489d5c: b9403fe8    	ldr	w8, [sp, #0x3c]
100489d60: 71000d1f    	cmp	w8, #0x3
100489d64: f94017f3    	ldr	x19, [sp, #0x28]
100489d68: 54000381    	b.ne	0x100489dd8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x640>
100489d6c: 90007488    	adrp	x8, 0x101319000 <_anon.b5f536351c4bf1e454cf90283dd9e625.167+0x8>
100489d70: 9126dd08    	add	x8, x8, #0x9b7
100489d74: 528000e9    	mov	w9, #0x7                ; =7
100489d78: a93527a8    	stp	x8, x9, [x29, #-0xb0]
100489d7c: d102c3a8    	sub	x8, x29, #0xb0
100489d80: 90001aa9    	adrp	x9, 0x1007dd000 <__ZN3zip9zipcrypto24ZipCryptoReader$LT$R$GT$8validate17h6abfe2f5c5f69c24E+0xf0>
100489d84: 911b0129    	add	x9, x9, #0x6c0
100489d88: a93727a8    	stp	x8, x9, [x29, #-0x90]
100489d8c: 900084e0    	adrp	x0, 0x101525000 <_anon.a9e2253c03df711f51070ce6102367ea.957+0x57>
100489d90: 91157800    	add	x0, x0, #0x55e
100489d94: d10243a1    	sub	x1, x29, #0x90
100489d98: 94340b80    	bl	0x10118cb98 <__ZN66_$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$6custom17h984fbb9098575a39E>
100489d9c: 52800048    	mov	w8, #0x2                ; =2
100489da0: a9000268    	stp	x8, x0, [x19]
100489da4: a9446fe1    	ldp	x1, x27, [sp, #0x40]
100489da8: b40000fb    	cbz	x27, 0x100489dc4 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x62c>
100489dac: aa1903e0    	mov	x0, x25
100489db0: aa0103f3    	mov	x19, x1
100489db4: aa1b03e1    	mov	x1, x27
100489db8: 52800022    	mov	w2, #0x1                ; =1
100489dbc: 97edddeb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100489dc0: aa1303e1    	mov	x1, x19
100489dc4: 52800013    	mov	w19, #0x0               ; =0
100489dc8: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100489dcc: eb08003f    	cmp	x1, x8
100489dd0: 54fff7e1    	b.ne	0x100489ccc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x534>
100489dd4: 17ffffc2    	b	0x100489cdc <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x544>
100489dd8: f1000adf    	cmp	x22, #0x2
100489ddc: aa0803ec    	mov	x12, x8
100489de0: 9a9603e8    	csel	x8, xzr, x22, eq
100489de4: b24107e9    	mov	x9, #-0x7fffffffffffffff ; =-9223372036854775807
100489de8: a944afea    	ldp	x10, x11, [sp, #0x48]
100489dec: a901666a    	stp	x10, x25, [x19, #0x10]
100489df0: d2f0000a    	mov	x10, #-0x8000000000000000 ; =-9223372036854775808
100489df4: eb09017f    	cmp	x11, x9
100489df8: 9a8b0149    	csel	x9, x10, x11, eq
100489dfc: f9000268    	str	x8, [x19]
100489e00: f9400fe8    	ldr	x8, [sp, #0x18]
100489e04: f9000668    	str	x8, [x19, #0x8]
100489e08: f94023e8    	ldr	x8, [sp, #0x40]
100489e0c: eb0a011f    	cmp	x8, x10
100489e10: 9a8803e8    	csel	x8, xzr, x8, eq
100489e14: f9401bea    	ldr	x10, [sp, #0x30]
100489e18: 9a9f154a    	csinc	x10, x10, xzr, ne
100489e1c: 9a9803eb    	csel	x11, xzr, x24, eq
100489e20: a902227b    	stp	x27, x8, [x19, #0x20]
100489e24: a9032e6a    	stp	x10, x11, [x19, #0x30]
100489e28: a9045269    	stp	x9, x20, [x19, #0x40]
100489e2c: f9400be8    	ldr	x8, [sp, #0x10]
100489e30: f9002a68    	str	x8, [x19, #0x50]
100489e34: 3901626c    	strb	w12, [x19, #0x58]
100489e38: ad410660    	ldp	q0, q1, [x19, #0x20]
100489e3c: 910303e8    	add	x8, sp, #0xc0
100489e40: ad010500    	stp	q0, q1, [x8, #0x20]
100489e44: ad420261    	ldp	q1, q0, [x19, #0x40]
100489e48: ad020101    	stp	q1, q0, [x8, #0x40]
100489e4c: ad400261    	ldp	q1, q0, [x19]
100489e50: ad0603e1    	stp	q1, q0, [sp, #0xc0]
100489e54: f9404fe8    	ldr	x8, [sp, #0x98]
100489e58: b5000e68    	cbnz	x8, 0x10048a024 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x88c>
100489e5c: 910163e0    	add	x0, sp, #0x58
100489e60: 940b9dda    	bl	0x1007715c8 <__ZN99_$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5ef754463675f4dcE>
100489e64: 17ffffb1    	b	0x100489d28 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hb793c78c3ad60688E+0x590>
