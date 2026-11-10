
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010079dca8 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE>:
10079dca8: d10243ff    	sub	sp, sp, #0x90
10079dcac: a90657f6    	stp	x22, x21, [sp, #0x60]
10079dcb0: a9074ff4    	stp	x20, x19, [sp, #0x70]
10079dcb4: a9087bfd    	stp	x29, x30, [sp, #0x80]
10079dcb8: 910203fd    	add	x29, sp, #0x80
10079dcbc: aa0003f4    	mov	x20, x0
10079dcc0: aa0803f3    	mov	x19, x8
10079dcc4: f0006060    	adrp	x0, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079dcc8: 91110000    	add	x0, x0, #0x440
10079dccc: 9100c3e8    	add	x8, sp, #0x30
10079dcd0: 52800201    	mov	w1, #0x10               ; =16
10079dcd4: 52800022    	mov	w2, #0x1                ; =1
10079dcd8: 9425ba33    	bl	0x10110c5a4 <__ZN82_$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$16serialize_struct17h9def4837d60a4ab2E>
10079dcdc: a94323e9    	ldp	x9, x8, [sp, #0x30]
10079dce0: d280004a    	mov	x10, #0x2               ; =2
10079dce4: f2f0000a    	movk	x10, #0x8000, lsl #48
10079dce8: eb0a013f    	cmp	x9, x10
10079dcec: 540000a1    	b.ne	0x10079dd00 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0x58>
10079dcf0: f9000668    	str	x8, [x19, #0x8]
10079dcf4: 528000c8    	mov	w8, #0x6                ; =6
10079dcf8: 39000268    	strb	w8, [x19]
10079dcfc: 1400002c    	b	0x10079ddac <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0x104>
10079dd00: ad4207e0    	ldp	q0, q1, [sp, #0x40]
10079dd04: ad0087e0    	stp	q0, q1, [sp, #0x10]
10079dd08: 910003f5    	mov	x21, sp
10079dd0c: a90023e9    	stp	x9, x8, [sp]
10079dd10: f0006061    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079dd14: 910a0021    	add	x1, x1, #0x280
10079dd18: 910003e0    	mov	x0, sp
10079dd1c: 528000a2    	mov	w2, #0x5                ; =5
10079dd20: aa1403e3    	mov	x3, x20
10079dd24: 97ea8ea0    	bl	0x1002417a4 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h23be61ec6d6b58f1E>
10079dd28: b40001c0    	cbz	x0, 0x10079dd60 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0xb8>
10079dd2c: f9000660    	str	x0, [x19, #0x8]
10079dd30: 528000c8    	mov	w8, #0x6                ; =6
10079dd34: 39000268    	strb	w8, [x19]
10079dd38: f94003f3    	ldr	x19, [sp]
10079dd3c: b24107e8    	mov	x8, #-0x7fffffffffffffff ; =-9223372036854775807
10079dd40: eb08027f    	cmp	x19, x8
10079dd44: 540001e1    	b.ne	0x10079dd80 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0xd8>
10079dd48: 394023e8    	ldrb	w8, [sp, #0x8]
10079dd4c: 7100191f    	cmp	w8, #0x6
10079dd50: 540002e0    	b.eq	0x10079ddac <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0x104>
10079dd54: 910022a0    	add	x0, x21, #0x8
10079dd58: 94013f1f    	bl	0x1007ed9d4 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10079dd5c: 14000014    	b	0x10079ddac <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0x104>
10079dd60: ad4007e0    	ldp	q0, q1, [sp]
10079dd64: ad0187e0    	stp	q0, q1, [sp, #0x30]
10079dd68: 3dc00be0    	ldr	q0, [sp, #0x20]
10079dd6c: 3d8017e0    	str	q0, [sp, #0x50]
10079dd70: 9100c3e0    	add	x0, sp, #0x30
10079dd74: aa1303e8    	mov	x8, x19
10079dd78: 9425ba68    	bl	0x10110c718 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$3end17h3354b3c9e0455d21E>
10079dd7c: 1400000c    	b	0x10079ddac <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0x104>
10079dd80: 910003e8    	mov	x8, sp
10079dd84: 91006100    	add	x0, x8, #0x18
10079dd88: 97ff4937    	bl	0x100770264 <__ZN99_$LT$alloc..collections..btree..map..BTreeMap$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h25de00eca2f35937E>
10079dd8c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10079dd90: eb08027f    	cmp	x19, x8
10079dd94: 540000c0    	b.eq	0x10079ddac <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0x104>
10079dd98: b40000b3    	cbz	x19, 0x10079ddac <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0x104>
10079dd9c: f94007e0    	ldr	x0, [sp, #0x8]
10079dda0: aa1303e1    	mov	x1, x19
10079dda4: 52800022    	mov	w2, #0x1                ; =1
10079dda8: 97e18df0    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10079ddac: a9487bfd    	ldp	x29, x30, [sp, #0x80]
10079ddb0: a9474ff4    	ldp	x20, x19, [sp, #0x70]
10079ddb4: a94657f6    	ldp	x22, x21, [sp, #0x60]
10079ddb8: 910243ff    	add	sp, sp, #0x90
10079ddbc: d65f03c0    	ret
10079ddc0: aa0003f4    	mov	x20, x0
10079ddc4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10079ddc8: eb08027f    	cmp	x19, x8
10079ddcc: 54000160    	b.eq	0x10079ddf8 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0x150>
10079ddd0: b4000153    	cbz	x19, 0x10079ddf8 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayActiveByIde$GT$9serialize17h2ccb2f2d35385a1bE+0x150>
10079ddd4: f94007e0    	ldr	x0, [sp, #0x8]
10079ddd8: aa1303e1    	mov	x1, x19
10079dddc: 52800022    	mov	w2, #0x1                ; =1
10079dde0: 97e18de2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10079dde4: aa1403e0    	mov	x0, x20
10079dde8: 9428ccdb    	bl	0x1011d1154 <_writev+0x1011d1154>
10079ddec: aa0003f4    	mov	x20, x0
10079ddf0: 910003e0    	mov	x0, sp
10079ddf4: 940140ab    	bl	0x1007ee0a0 <__ZN4core3ptr57drop_in_place$LT$serde_json..value..ser..SerializeMap$GT$17he7d746eed2e7e905E>
10079ddf8: aa1403e0    	mov	x0, x20
10079ddfc: 9428ccd6    	bl	0x1011d1154 <_writev+0x1011d1154>
10079de00: 9428cb52    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
