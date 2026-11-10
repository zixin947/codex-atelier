
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100799e28 <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E>:
100799e28: a9bf7bfd    	stp	x29, x30, [sp, #-0x10]!
100799e2c: 910003fd    	mov	x29, sp
100799e30: aa0103e8    	mov	x8, x1
100799e34: 39400009    	ldrb	w9, [x0]
100799e38: 7100093f    	cmp	w9, #0x2
100799e3c: 5400010d    	b.le	0x100799e5c <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0x34>
100799e40: 7100113f    	cmp	w9, #0x4
100799e44: 540001ec    	b.gt	0x100799e80 <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0x58>
100799e48: 71000d3f    	cmp	w9, #0x3
100799e4c: 54000421    	b.ne	0x100799ed0 <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0xa8>
100799e50: f0006082    	adrp	x2, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
100799e54: 91089842    	add	x2, x2, #0x226
100799e58: 14000014    	b	0x100799ea8 <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0x80>
100799e5c: 34000229    	cbz	w9, 0x100799ea0 <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0x78>
100799e60: 7100053f    	cmp	w9, #0x1
100799e64: 540002a1    	b.ne	0x100799eb8 <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0x90>
100799e68: f0006082    	adrp	x2, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
100799e6c: 91086842    	add	x2, x2, #0x21a
100799e70: 91002101    	add	x1, x8, #0x8
100799e74: aa0803e0    	mov	x0, x8
100799e78: 52800103    	mov	w3, #0x8                ; =8
100799e7c: 14000020    	b	0x100799efc <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0xd4>
100799e80: 7100153f    	cmp	w9, #0x5
100799e84: 54000321    	b.ne	0x100799ee8 <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0xc0>
100799e88: f0006082    	adrp	x2, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
100799e8c: 910a7042    	add	x2, x2, #0x29c
100799e90: 91002101    	add	x1, x8, #0x8
100799e94: aa0803e0    	mov	x0, x8
100799e98: 528000a3    	mov	w3, #0x5                ; =5
100799e9c: 14000018    	b	0x100799efc <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0xd4>
100799ea0: f0006082    	adrp	x2, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
100799ea4: 910a3c42    	add	x2, x2, #0x28f
100799ea8: 91002101    	add	x1, x8, #0x8
100799eac: aa0803e0    	mov	x0, x8
100799eb0: 528000e3    	mov	w3, #0x7                ; =7
100799eb4: 14000012    	b	0x100799efc <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0xd4>
100799eb8: f0006082    	adrp	x2, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
100799ebc: 91088842    	add	x2, x2, #0x222
100799ec0: 91002101    	add	x1, x8, #0x8
100799ec4: aa0803e0    	mov	x0, x8
100799ec8: 52800083    	mov	w3, #0x4                ; =4
100799ecc: 1400000c    	b	0x100799efc <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0xd4>
100799ed0: f0006082    	adrp	x2, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
100799ed4: 910a5842    	add	x2, x2, #0x296
100799ed8: 91002101    	add	x1, x8, #0x8
100799edc: aa0803e0    	mov	x0, x8
100799ee0: 528000c3    	mov	w3, #0x6                ; =6
100799ee4: 14000006    	b	0x100799efc <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0xd4>
100799ee8: f0006082    	adrp	x2, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
100799eec: 910a8442    	add	x2, x2, #0x2a1
100799ef0: 91002101    	add	x1, x8, #0x8
100799ef4: aa0803e0    	mov	x0, x8
100799ef8: 52800143    	mov	w3, #0xa                ; =10
100799efc: 97f64b63    	bl	0x10052cc88 <__ZN10serde_json3ser18format_escaped_str17he8601f72f97bccadE>
100799f00: b5000080    	cbnz	x0, 0x100799f10 <__ZN13codexmate_lib4core5relay6models1_103_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayBrand$GT$9serialize17h237a14a9829309a3E+0xe8>
100799f04: d2800000    	mov	x0, #0x0                ; =0
100799f08: a8c17bfd    	ldp	x29, x30, [sp], #0x10
100799f0c: d65f03c0    	ret
100799f10: a8c17bfd    	ldp	x29, x30, [sp], #0x10
100799f14: 1428c089    	b	0x1011ca138 <__ZN10serde_json5error5Error2io17h3ac6a1bc26e932ccE>
