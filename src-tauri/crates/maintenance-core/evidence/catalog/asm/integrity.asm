
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010037795c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E>:
10037795c: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
100377960: a90167fa    	stp	x26, x25, [sp, #0x10]
100377964: a9025ff8    	stp	x24, x23, [sp, #0x20]
100377968: a90357f6    	stp	x22, x21, [sp, #0x30]
10037796c: a9044ff4    	stp	x20, x19, [sp, #0x40]
100377970: a9057bfd    	stp	x29, x30, [sp, #0x50]
100377974: 910143fd    	add	x29, sp, #0x50
100377978: d10903ff    	sub	sp, sp, #0x240
10037797c: aa0003f6    	mov	x22, x0
100377980: 3945b068    	ldrb	w8, [x3, #0x16c]
100377984: 36000ac8    	tbz	w8, #0x0, 0x100377adc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x180>
100377988: aa0303f7    	mov	x23, x3
10037798c: 90007ca9    	adrp	x9, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100377990: 912ca929    	add	x9, x9, #0xb2a
100377994: 910003e8    	mov	x8, sp
100377998: aa0103e0    	mov	x0, x1
10037799c: aa0203e1    	mov	x1, x2
1003779a0: aa0903e2    	mov	x2, x9
1003779a4: 52800323    	mov	w3, #0x19               ; =25
1003779a8: 94375e2e    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
1003779ac: a940d3f3    	ldp	x19, x20, [sp, #0x8]
1003779b0: d103a3a8    	sub	x8, x29, #0xe8
1003779b4: aa1303e0    	mov	x0, x19
1003779b8: aa1403e1    	mov	x1, x20
1003779bc: 9437430f    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
1003779c0: a97187b5    	ldp	x21, x1, [x29, #-0xe8]
1003779c4: aa1503e0    	mov	x0, x21
1003779c8: 94018c76    	bl	0x1003daba0 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
1003779cc: b4000a75    	cbz	x21, 0x100377b18 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x1bc>
1003779d0: 97f226e9    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003779d4: 52800220    	mov	w0, #0x11               ; =17
1003779d8: 52800021    	mov	w1, #0x1                ; =1
1003779dc: 97f226e2    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003779e0: b4004fe0    	cbz	x0, 0x1003783dc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xa80>
1003779e4: aa0003f4    	mov	x20, x0
1003779e8: 52800f28    	mov	w8, #0x79               ; =121
1003779ec: 39004008    	strb	w8, [x0, #0x10]
1003779f0: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
1003779f4: 910a6d08    	add	x8, x8, #0x29b
1003779f8: 3dc00100    	ldr	q0, [x8]
1003779fc: 3d800000    	str	q0, [x0]
100377a00: 97f226dd    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377a04: 528000a0    	mov	w0, #0x5                ; =5
100377a08: 52800021    	mov	w1, #0x1                ; =1
100377a0c: 97f226d6    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377a10: b4004ee0    	cbz	x0, 0x1003783ec <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xa90>
100377a14: aa0003f5    	mov	x21, x0
100377a18: 52800e48    	mov	w8, #0x72               ; =114
100377a1c: 39001008    	strb	w8, [x0, #0x4]
100377a20: 528e4ca8    	mov	w8, #0x7265             ; =29285
100377a24: 72adee48    	movk	w8, #0x6f72, lsl #16
100377a28: b9000008    	str	w8, [x0]
100377a2c: 97f226d2    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377a30: 52800280    	mov	w0, #0x14               ; =20
100377a34: 52800021    	mov	w1, #0x1                ; =1
100377a38: 97f226cb    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377a3c: b4004e00    	cbz	x0, 0x1003783fc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xaa0>
100377a40: aa0003f7    	mov	x23, x0
100377a44: 529cb748    	mov	w8, #0xe5ba             ; =58810
100377a48: 72b63488    	movk	w8, #0xb1a4, lsl #16
100377a4c: b9001008    	str	w8, [x0, #0x10]
100377a50: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377a54: 911c9d08    	add	x8, x8, #0x727
100377a58: 3dc00100    	ldr	q0, [x8]
100377a5c: 3d800000    	str	q0, [x0]
100377a60: 97f226c5    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377a64: 52800038    	mov	w24, #0x1               ; =1
100377a68: 52800960    	mov	w0, #0x4b               ; =75
100377a6c: 52800021    	mov	w1, #0x1                ; =1
100377a70: 97f226bd    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377a74: b4004cc0    	cbz	x0, 0x10037840c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xab0>
100377a78: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377a7c: 911ced08    	add	x8, x8, #0x73b
100377a80: ad410500    	ldp	q0, q1, [x8, #0x20]
100377a84: ad010400    	stp	q0, q1, [x0, #0x20]
100377a88: 3cc3b100    	ldur	q0, [x8, #0x3b]
100377a8c: 3c83b000    	stur	q0, [x0, #0x3b]
100377a90: ad400101    	ldp	q1, q0, [x8]
100377a94: ad000001    	stp	q1, q0, [x0]
100377a98: 52800228    	mov	w8, #0x11               ; =17
100377a9c: a90052c8    	stp	x8, x20, [x22]
100377aa0: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377aa4: 3dc34500    	ldr	q0, [x8, #0xd10]
100377aa8: 3d8006c0    	str	q0, [x22, #0x10]
100377aac: f90012d5    	str	x21, [x22, #0x20]
100377ab0: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377ab4: 3dc35900    	ldr	q0, [x8, #0xd60]
100377ab8: 3c8282c0    	stur	q0, [x22, #0x28]
100377abc: f9001ed7    	str	x23, [x22, #0x38]
100377ac0: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377ac4: 3dc36500    	ldr	q0, [x8, #0xd90]
100377ac8: 3d8012c0    	str	q0, [x22, #0x40]
100377acc: 52800968    	mov	w8, #0x4b               ; =75
100377ad0: a90522c0    	stp	x0, x8, [x22, #0x50]
100377ad4: 390182d8    	strb	w24, [x22, #0x60]
100377ad8: 14000223    	b	0x100378364 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xa08>
100377adc: d0007ca1    	adrp	x1, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377ae0: 910a6c21    	add	x1, x1, #0x29b
100377ae4: d0007ca3    	adrp	x3, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377ae8: 911be063    	add	x3, x3, #0x6f8
100377aec: aa1603e0    	mov	x0, x22
100377af0: 52800222    	mov	w2, #0x11               ; =17
100377af4: 528005e4    	mov	w4, #0x2f               ; =47
100377af8: 910903ff    	add	sp, sp, #0x240
100377afc: a9457bfd    	ldp	x29, x30, [sp, #0x50]
100377b00: a9444ff4    	ldp	x20, x19, [sp, #0x40]
100377b04: a94357f6    	ldp	x22, x21, [sp, #0x30]
100377b08: a9425ff8    	ldp	x24, x23, [sp, #0x20]
100377b0c: a94167fa    	ldp	x26, x25, [sp, #0x10]
100377b10: a8c66ffc    	ldp	x28, x27, [sp], #0x60
100377b14: 1400cad4    	b	0x1003aa664 <__ZN13codexmate_lib4core5relay16codex_diagnostic7ok_item17h09598b1cce494186E>
100377b18: d103a3a8    	sub	x8, x29, #0xe8
100377b1c: aa1303e0    	mov	x0, x19
100377b20: aa1403e1    	mov	x1, x20
100377b24: 9437296e    	bl	0x1011420dc <__ZN3std2fs14read_to_string5inner17h0f85889ccb0352e2E>
100377b28: f85183b4    	ldur	x20, [x29, #-0xe8]
100377b2c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100377b30: eb08029f    	cmp	x20, x8
100377b34: 54000a61    	b.ne	0x100377c80 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x324>
100377b38: f85203b5    	ldur	x21, [x29, #-0xe0]
100377b3c: f9001bf5    	str	x21, [sp, #0x30]
100377b40: 97f2268d    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377b44: 52800220    	mov	w0, #0x11               ; =17
100377b48: 52800021    	mov	w1, #0x1                ; =1
100377b4c: 97f22686    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377b50: b4004660    	cbz	x0, 0x10037841c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xac0>
100377b54: aa0003f4    	mov	x20, x0
100377b58: 52800f28    	mov	w8, #0x79               ; =121
100377b5c: 39004008    	strb	w8, [x0, #0x10]
100377b60: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377b64: 910a6d08    	add	x8, x8, #0x29b
100377b68: 3dc00100    	ldr	q0, [x8]
100377b6c: 3d800000    	str	q0, [x0]
100377b70: 97f22681    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377b74: 528000a0    	mov	w0, #0x5                ; =5
100377b78: 52800021    	mov	w1, #0x1                ; =1
100377b7c: 97f2267a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377b80: b4004560    	cbz	x0, 0x10037842c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xad0>
100377b84: aa0003f5    	mov	x21, x0
100377b88: 52800e48    	mov	w8, #0x72               ; =114
100377b8c: 39001008    	strb	w8, [x0, #0x4]
100377b90: 528e4ca8    	mov	w8, #0x7265             ; =29285
100377b94: 72adee48    	movk	w8, #0x6f72, lsl #16
100377b98: b9000008    	str	w8, [x0]
100377b9c: 97f22676    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377ba0: 52800340    	mov	w0, #0x1a               ; =26
100377ba4: 52800021    	mov	w1, #0x1                ; =1
100377ba8: 97f2266f    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377bac: b4004480    	cbz	x0, 0x10037843c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xae0>
100377bb0: aa0003f7    	mov	x23, x0
100377bb4: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377bb8: 9120b908    	add	x8, x8, #0x82e
100377bbc: 3dc00100    	ldr	q0, [x8]
100377bc0: 3d800000    	str	q0, [x0]
100377bc4: 3cc0a100    	ldur	q0, [x8, #0xa]
100377bc8: 3c80a000    	stur	q0, [x0, #0xa]
100377bcc: 9100c3e8    	add	x8, sp, #0x30
100377bd0: d0006f89    	adrp	x9, 0x101169000 <__ZN5gimli4read8rnglists20RngListIter$LT$R$GT$4next17ha0add38c23e78896E+0x510>
100377bd4: 91211129    	add	x9, x9, #0x844
100377bd8: a90527e8    	stp	x8, x9, [sp, #0x50]
100377bdc: b0008d20    	adrp	x0, 0x10151c000 <_anon.b03982025400859c41411cbf9a3d7a99.1+0x4c6>
100377be0: 913f9800    	add	x0, x0, #0xfe6
100377be4: 910063e8    	add	x8, sp, #0x18
100377be8: 910143e1    	add	x1, sp, #0x50
100377bec: 9437ff2a    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100377bf0: 3cc183e0    	ldur	q0, [sp, #0x18]
100377bf4: 3c8482c0    	stur	q0, [x22, #0x48]
100377bf8: f94017e8    	ldr	x8, [sp, #0x28]
100377bfc: f9002ec8    	str	x8, [x22, #0x58]
100377c00: 52800228    	mov	w8, #0x11               ; =17
100377c04: a90052c8    	stp	x8, x20, [x22]
100377c08: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377c0c: 3dc34500    	ldr	q0, [x8, #0xd10]
100377c10: 3d8006c0    	str	q0, [x22, #0x10]
100377c14: f90012d5    	str	x21, [x22, #0x20]
100377c18: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377c1c: 3dc36100    	ldr	q0, [x8, #0xd80]
100377c20: 3c8282c0    	stur	q0, [x22, #0x28]
100377c24: 52800348    	mov	w8, #0x1a               ; =26
100377c28: a903a2d7    	stp	x23, x8, [x22, #0x38]
100377c2c: 52800028    	mov	w8, #0x1                ; =1
100377c30: f9401bf4    	ldr	x20, [sp, #0x30]
100377c34: 92400689    	and	x9, x20, #0x3
100377c38: 390182c8    	strb	w8, [x22, #0x60]
100377c3c: f100053f    	cmp	x9, #0x1
100377c40: 54003921    	b.ne	0x100378364 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xa08>
100377c44: f85ffe95    	ldr	x21, [x20, #-0x1]!
100377c48: f9400697    	ldr	x23, [x20, #0x8]
100377c4c: f94002e8    	ldr	x8, [x23]
100377c50: b4000068    	cbz	x8, 0x100377c5c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x300>
100377c54: aa1503e0    	mov	x0, x21
100377c58: d63f0100    	blr	x8
100377c5c: f94006e1    	ldr	x1, [x23, #0x8]
100377c60: b4000081    	cbz	x1, 0x100377c70 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x314>
100377c64: f9400ae2    	ldr	x2, [x23, #0x10]
100377c68: aa1503e0    	mov	x0, x21
100377c6c: 97f2263f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100377c70: aa1403e0    	mov	x0, x20
100377c74: 52800301    	mov	w1, #0x18               ; =24
100377c78: 52800102    	mov	w2, #0x8                ; =8
100377c7c: 140001b9    	b	0x100378360 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xa04>
100377c80: a97263b5    	ldp	x21, x24, [x29, #-0xe0]
100377c84: aa1503e0    	mov	x0, x21
100377c88: aa1803e1    	mov	x1, x24
100377c8c: 97fadd33    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100377c90: b4000941    	cbz	x1, 0x100377db8 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x45c>
100377c94: a931e3b5    	stp	x21, x24, [x29, #-0xe8]
100377c98: a932ffbf    	stp	xzr, xzr, [x29, #-0xd8]
100377c9c: a933e3b5    	stp	x21, x24, [x29, #-0xc8]
100377ca0: 910143e8    	add	x8, sp, #0x50
100377ca4: d103a3a0    	sub	x0, x29, #0xe8
100377ca8: 97f98c5b    	bl	0x1001dae14 <__ZN10serde_json2de10from_trait17h472fe9ac40d19421E>
100377cac: 394143e8    	ldrb	w8, [sp, #0x50]
100377cb0: 7100191f    	cmp	w8, #0x6
100377cb4: 54001041    	b.ne	0x100377ebc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x560>
100377cb8: 97f2262f    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377cbc: 52800220    	mov	w0, #0x11               ; =17
100377cc0: 52800021    	mov	w1, #0x1                ; =1
100377cc4: 97f22628    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377cc8: b4003e20    	cbz	x0, 0x10037848c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb30>
100377ccc: aa0003f7    	mov	x23, x0
100377cd0: 52800f28    	mov	w8, #0x79               ; =121
100377cd4: 39004008    	strb	w8, [x0, #0x10]
100377cd8: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377cdc: 910a6d08    	add	x8, x8, #0x29b
100377ce0: 3dc00100    	ldr	q0, [x8]
100377ce4: 3d800000    	str	q0, [x0]
100377ce8: 97f22623    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377cec: 528000a0    	mov	w0, #0x5                ; =5
100377cf0: 52800021    	mov	w1, #0x1                ; =1
100377cf4: 97f2261c    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377cf8: b4003d20    	cbz	x0, 0x10037849c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb40>
100377cfc: aa0003f8    	mov	x24, x0
100377d00: 52800e48    	mov	w8, #0x72               ; =114
100377d04: 39001008    	strb	w8, [x0, #0x4]
100377d08: 528e4ca8    	mov	w8, #0x7265             ; =29285
100377d0c: 72adee48    	movk	w8, #0x6f72, lsl #16
100377d10: b9000008    	str	w8, [x0]
100377d14: 97f22618    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377d18: 52800400    	mov	w0, #0x20               ; =32
100377d1c: 52800021    	mov	w1, #0x1                ; =1
100377d20: 97f22611    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377d24: b4003c40    	cbz	x0, 0x1003784ac <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb50>
100377d28: aa0003f9    	mov	x25, x0
100377d2c: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377d30: 91203908    	add	x8, x8, #0x80e
100377d34: ad400500    	ldp	q0, q1, [x8]
100377d38: ad000400    	stp	q0, q1, [x0]
100377d3c: 97f2260e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377d40: 5280003a    	mov	w26, #0x1               ; =1
100377d44: 52800240    	mov	w0, #0x12               ; =18
100377d48: 52800021    	mov	w1, #0x1                ; =1
100377d4c: 97f22606    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377d50: b4003b60    	cbz	x0, 0x1003784bc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb60>
100377d54: 52921108    	mov	w8, #0x9088             ; =37000
100377d58: 79002008    	strh	w8, [x0, #0x10]
100377d5c: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377d60: 911e6908    	add	x8, x8, #0x79a
100377d64: 3dc00100    	ldr	q0, [x8]
100377d68: 3d800000    	str	q0, [x0]
100377d6c: 52800228    	mov	w8, #0x11               ; =17
100377d70: a9005ec8    	stp	x8, x23, [x22]
100377d74: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377d78: 3dc34500    	ldr	q0, [x8, #0xd10]
100377d7c: 3d8006c0    	str	q0, [x22, #0x10]
100377d80: f90012d8    	str	x24, [x22, #0x20]
100377d84: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377d88: 3dc35100    	ldr	q0, [x8, #0xd40]
100377d8c: 3c8282c0    	stur	q0, [x22, #0x28]
100377d90: f9001ed9    	str	x25, [x22, #0x38]
100377d94: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377d98: 3dc35500    	ldr	q0, [x8, #0xd50]
100377d9c: 3d8012c0    	str	q0, [x22, #0x40]
100377da0: 52800248    	mov	w8, #0x12               ; =18
100377da4: a90522c0    	stp	x0, x8, [x22, #0x50]
100377da8: 390182da    	strb	w26, [x22, #0x60]
100377dac: 910143e0    	add	x0, sp, #0x50
100377db0: 9401761a    	bl	0x1003d5618 <__ZN4core3ptr100drop_in_place$LT$core..result..Result$LT$serde_json..value..Value$C$serde_json..error..Error$GT$$GT$17he016dcde0b535e91E>
100377db4: 14000167    	b	0x100378350 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x9f4>
100377db8: 97f225ef    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377dbc: 52800220    	mov	w0, #0x11               ; =17
100377dc0: 52800021    	mov	w1, #0x1                ; =1
100377dc4: 97f225e8    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377dc8: b4003420    	cbz	x0, 0x10037844c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xaf0>
100377dcc: aa0003f7    	mov	x23, x0
100377dd0: 52800f28    	mov	w8, #0x79               ; =121
100377dd4: 39004008    	strb	w8, [x0, #0x10]
100377dd8: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377ddc: 910a6d08    	add	x8, x8, #0x29b
100377de0: 3dc00100    	ldr	q0, [x8]
100377de4: 3d800000    	str	q0, [x0]
100377de8: 97f225e3    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377dec: 528000a0    	mov	w0, #0x5                ; =5
100377df0: 52800021    	mov	w1, #0x1                ; =1
100377df4: 97f225dc    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377df8: b4003320    	cbz	x0, 0x10037845c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb00>
100377dfc: aa0003f8    	mov	x24, x0
100377e00: 52800e48    	mov	w8, #0x72               ; =114
100377e04: 39001008    	strb	w8, [x0, #0x4]
100377e08: 528e4ca8    	mov	w8, #0x7265             ; =29285
100377e0c: 72adee48    	movk	w8, #0x6f72, lsl #16
100377e10: b9000008    	str	w8, [x0]
100377e14: 97f225d8    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377e18: 52800280    	mov	w0, #0x14               ; =20
100377e1c: 52800021    	mov	w1, #0x1                ; =1
100377e20: 97f225d1    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377e24: b4003240    	cbz	x0, 0x10037846c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb10>
100377e28: aa0003f9    	mov	x25, x0
100377e2c: 529cf748    	mov	w8, #0xe7ba             ; =59322
100377e30: 72b75528    	movk	w8, #0xbaa9, lsl #16
100377e34: b9001008    	str	w8, [x0, #0x10]
100377e38: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377e3c: 911e1908    	add	x8, x8, #0x786
100377e40: 3dc00100    	ldr	q0, [x8]
100377e44: 3d800000    	str	q0, [x0]
100377e48: 97f225cb    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377e4c: 5280003a    	mov	w26, #0x1               ; =1
100377e50: 52800240    	mov	w0, #0x12               ; =18
100377e54: 52800021    	mov	w1, #0x1                ; =1
100377e58: 97f225c3    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377e5c: b4003100    	cbz	x0, 0x10037847c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb20>
100377e60: 52921108    	mov	w8, #0x9088             ; =37000
100377e64: 79002008    	strh	w8, [x0, #0x10]
100377e68: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377e6c: 911e6908    	add	x8, x8, #0x79a
100377e70: 3dc00100    	ldr	q0, [x8]
100377e74: 3d800000    	str	q0, [x0]
100377e78: 52800228    	mov	w8, #0x11               ; =17
100377e7c: a9005ec8    	stp	x8, x23, [x22]
100377e80: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377e84: 3dc34500    	ldr	q0, [x8, #0xd10]
100377e88: 3d8006c0    	str	q0, [x22, #0x10]
100377e8c: f90012d8    	str	x24, [x22, #0x20]
100377e90: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377e94: 3dc35900    	ldr	q0, [x8, #0xd60]
100377e98: 3c8282c0    	stur	q0, [x22, #0x28]
100377e9c: f9001ed9    	str	x25, [x22, #0x38]
100377ea0: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377ea4: 3dc35d00    	ldr	q0, [x8, #0xd70]
100377ea8: 3d8012c0    	str	q0, [x22, #0x40]
100377eac: 52800248    	mov	w8, #0x12               ; =18
100377eb0: a90522c0    	stp	x0, x8, [x22, #0x50]
100377eb4: 390182da    	strb	w26, [x22, #0x60]
100377eb8: 14000126    	b	0x100378350 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x9f4>
100377ebc: ad4287e0    	ldp	q0, q1, [sp, #0x50]
100377ec0: ad0187e0    	stp	q0, q1, [sp, #0x30]
100377ec4: b0007ca0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
100377ec8: 9102ac00    	add	x0, x0, #0xab
100377ecc: 9100c3e2    	add	x2, sp, #0x30
100377ed0: 528000c1    	mov	w1, #0x6                ; =6
100377ed4: 94365b2f    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
100377ed8: b4000200    	cbz	x0, 0x100377f18 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x5bc>
100377edc: 39400008    	ldrb	w8, [x0]
100377ee0: 7100111f    	cmp	w8, #0x4
100377ee4: 540001a1    	b.ne	0x100377f18 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x5bc>
100377ee8: a9412001    	ldp	x1, x8, [x0, #0x10]
100377eec: 8b081422    	add	x2, x1, x8, lsl #5
100377ef0: 910143e0    	add	x0, sp, #0x50
100377ef4: 97ff9b75    	bl	0x10035ecc8 <__ZN120_$LT$std..collections..hash..set..HashSet$LT$T$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h0c4bdb1accd6688eE>
100377ef8: 3945faf8    	ldrb	w24, [x23, #0x17e]
100377efc: a94086e0    	ldp	x0, x1, [x23, #0x8]
100377f00: a9420ee2    	ldp	x2, x3, [x23, #0x20]
100377f04: 360008f8    	tbz	w24, #0x0, 0x100378020 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x6c4>
100377f08: a94896e4    	ldp	x4, x5, [x23, #0x88]
100377f0c: d103a3a8    	sub	x8, x29, #0xe8
100377f10: 97ffdec7    	bl	0x10036fa2c <__ZN13codexmate_lib4core5relay13codex_catalog24no_account_catalog_slugs17ha19591f82502f1f1E>
100377f14: 14000045    	b	0x100378028 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x6cc>
100377f18: 97f22597    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377f1c: 52800220    	mov	w0, #0x11               ; =17
100377f20: 52800021    	mov	w1, #0x1                ; =1
100377f24: 97f22590    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377f28: b4002d20    	cbz	x0, 0x1003784cc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb70>
100377f2c: aa0003f7    	mov	x23, x0
100377f30: 52800f28    	mov	w8, #0x79               ; =121
100377f34: 39004008    	strb	w8, [x0, #0x10]
100377f38: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377f3c: 910a6d08    	add	x8, x8, #0x29b
100377f40: 3dc00100    	ldr	q0, [x8]
100377f44: 3d800000    	str	q0, [x0]
100377f48: 97f2258b    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377f4c: 528000a0    	mov	w0, #0x5                ; =5
100377f50: 52800021    	mov	w1, #0x1                ; =1
100377f54: 97f22584    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377f58: b4002c20    	cbz	x0, 0x1003784dc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb80>
100377f5c: aa0003f8    	mov	x24, x0
100377f60: 52800e48    	mov	w8, #0x72               ; =114
100377f64: 39001008    	strb	w8, [x0, #0x4]
100377f68: 528e4ca8    	mov	w8, #0x7265             ; =29285
100377f6c: 72adee48    	movk	w8, #0x6f72, lsl #16
100377f70: b9000008    	str	w8, [x0]
100377f74: 97f22580    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377f78: 52800380    	mov	w0, #0x1c               ; =28
100377f7c: 52800021    	mov	w1, #0x1                ; =1
100377f80: 97f22579    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377f84: b4002b40    	cbz	x0, 0x1003784ec <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xb90>
100377f88: aa0003f9    	mov	x25, x0
100377f8c: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377f90: 911fc908    	add	x8, x8, #0x7f2
100377f94: 3dc00100    	ldr	q0, [x8]
100377f98: 3d800000    	str	q0, [x0]
100377f9c: 3cc0c100    	ldur	q0, [x8, #0xc]
100377fa0: 3c80c000    	stur	q0, [x0, #0xc]
100377fa4: 97f22574    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100377fa8: 5280003a    	mov	w26, #0x1               ; =1
100377fac: 52800240    	mov	w0, #0x12               ; =18
100377fb0: 52800021    	mov	w1, #0x1                ; =1
100377fb4: 97f2256c    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100377fb8: b4002a20    	cbz	x0, 0x1003784fc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xba0>
100377fbc: 52921108    	mov	w8, #0x9088             ; =37000
100377fc0: 79002008    	strh	w8, [x0, #0x10]
100377fc4: d0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100377fc8: 911e6908    	add	x8, x8, #0x79a
100377fcc: 3dc00100    	ldr	q0, [x8]
100377fd0: 3d800000    	str	q0, [x0]
100377fd4: 52800228    	mov	w8, #0x11               ; =17
100377fd8: a9005ec8    	stp	x8, x23, [x22]
100377fdc: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377fe0: 3dc34500    	ldr	q0, [x8, #0xd10]
100377fe4: 3d8006c0    	str	q0, [x22, #0x10]
100377fe8: f90012d8    	str	x24, [x22, #0x20]
100377fec: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100377ff0: 3dc34900    	ldr	q0, [x8, #0xd20]
100377ff4: 3c8282c0    	stur	q0, [x22, #0x28]
100377ff8: f9001ed9    	str	x25, [x22, #0x38]
100377ffc: f0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100378000: 3dc34d00    	ldr	q0, [x8, #0xd30]
100378004: 3d8012c0    	str	q0, [x22, #0x40]
100378008: 52800248    	mov	w8, #0x12               ; =18
10037800c: a90522c0    	stp	x0, x8, [x22, #0x50]
100378010: 390182da    	strb	w26, [x22, #0x60]
100378014: 9100c3e0    	add	x0, sp, #0x30
100378018: 94017d6a    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10037801c: 140000cd    	b	0x100378350 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x9f4>
100378020: d103a3a8    	sub	x8, x29, #0xe8
100378024: 9410a5ea    	bl	0x1007a17cc <__ZN13codexmate_lib4core5relay6models30active_codex_relay_model_slugs17h72c809e2cf5f3b42E>
100378028: a971a7a8    	ldp	x8, x9, [x29, #-0xe8]
10037802c: 8b090109    	add	x9, x8, x9
100378030: aa0803ea    	mov	x10, x8
100378034: fc408540    	ldr	d0, [x10], #0x8
100378038: 91000529    	add	x9, x9, #0x1
10037803c: 2e208800    	cmge.8b	v0, v0, #0
100378040: 9e66000b    	fmov	x11, d0
100378044: 9201c16b    	and	x11, x11, #0x8080808080808080
100378048: f85303ac    	ldur	x12, [x29, #-0xd0]
10037804c: a90a2beb    	stp	x11, x10, [sp, #0xa0]
100378050: f9004fe8    	str	x8, [sp, #0x98]
100378054: a90b33e9    	stp	x9, x12, [sp, #0xb0]
100378058: 910143e8    	add	x8, sp, #0x50
10037805c: f90063e8    	str	x8, [sp, #0xc0]
100378060: 910203e8    	add	x8, sp, #0x80
100378064: 910263e0    	add	x0, sp, #0x98
100378068: 9406f826    	bl	0x100536100 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h81b934ad20c5b077E>
10037806c: a948ebf9    	ldp	x25, x26, [sp, #0x88]
100378070: aa1903e0    	mov	x0, x25
100378074: aa1a03e1    	mov	x1, x26
100378078: 9401d52a    	bl	0x1003ed520 <__ZN5alloc5slice11stable_sort17h520a0224053cec60E>
10037807c: 36000298    	tbz	w24, #0x0, 0x1003780cc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x770>
100378080: a94527e8    	ldp	x8, x9, [sp, #0x50]
100378084: 8b090109    	add	x9, x8, x9
100378088: aa0803ea    	mov	x10, x8
10037808c: fc408540    	ldr	d0, [x10], #0x8
100378090: 91000529    	add	x9, x9, #0x1
100378094: 2e208800    	cmge.8b	v0, v0, #0
100378098: 9e66000b    	fmov	x11, d0
10037809c: 9201c16b    	and	x11, x11, #0x8080808080808080
1003780a0: f94037ec    	ldr	x12, [sp, #0x68]
1003780a4: a90eabeb    	stp	x11, x10, [sp, #0xe8]
1003780a8: f90073e8    	str	x8, [sp, #0xe0]
1003780ac: a90fb3e9    	stp	x9, x12, [sp, #0xf8]
1003780b0: d103a3a8    	sub	x8, x29, #0xe8
1003780b4: f90087e8    	str	x8, [sp, #0x108]
1003780b8: 910323e8    	add	x8, sp, #0xc8
1003780bc: 910383e0    	add	x0, sp, #0xe0
1003780c0: 9406f810    	bl	0x100536100 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h81b934ad20c5b077E>
1003780c4: a94d5ff8    	ldp	x24, x23, [sp, #0xd0]
1003780c8: 14000005    	b	0x1003780dc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x780>
1003780cc: d2800017    	mov	x23, #0x0               ; =0
1003780d0: 52800118    	mov	w24, #0x8               ; =8
1003780d4: a90ce3ff    	stp	xzr, x24, [sp, #0xc8]
1003780d8: f9006fff    	str	xzr, [sp, #0xd8]
1003780dc: aa1803e0    	mov	x0, x24
1003780e0: aa1703e1    	mov	x1, x23
1003780e4: 9401d50f    	bl	0x1003ed520 <__ZN5alloc5slice11stable_sort17h520a0224053cec60E>
1003780e8: b00003bb    	adrp	x27, 0x1003ed000 <__ZN5alloc11collections5btree3set21BTreeSet$LT$T$C$A$GT$10difference17h151923f22deefe03E+0x15c>
1003780ec: 9133e37b    	add	x27, x27, #0xcf8
1003780f0: b400041a    	cbz	x26, 0x100378170 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x814>
1003780f4: 52800108    	mov	w8, #0x8                ; =8
1003780f8: a91123ff    	stp	xzr, x8, [sp, #0x110]
1003780fc: f90093ff    	str	xzr, [sp, #0x120]
100378100: b0007ca2    	adrp	x2, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100378104: 911ee842    	add	x2, x2, #0x7ba
100378108: d10403a8    	sub	x8, x29, #0x100
10037810c: aa1903e0    	mov	x0, x25
100378110: aa1a03e1    	mov	x1, x26
100378114: 52800063    	mov	w3, #0x3                ; =3
100378118: 97fae68b    	bl	0x100231b44 <__ZN5alloc3str17join_generic_copy17hc3fffb246d6f0b82E>
10037811c: 3cd003a0    	ldur	q0, [x29, #-0x100]
100378120: 3d805fe0    	str	q0, [sp, #0x170]
100378124: f85103a8    	ldur	x8, [x29, #-0xf0]
100378128: f900c3e8    	str	x8, [sp, #0x180]
10037812c: 9105c3e8    	add	x8, sp, #0x170
100378130: a9306fa8    	stp	x8, x27, [x29, #-0x100]
100378134: 90008d40    	adrp	x0, 0x101520000 <_anon.164d03a2dfc98f969363eaf1c50504fd.175+0x1a1>
100378138: 91136c00    	add	x0, x0, #0x4db
10037813c: 9104a3e8    	add	x8, sp, #0x128
100378140: d10403a1    	sub	x1, x29, #0x100
100378144: 9437fdd4    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100378148: f940bbe1    	ldr	x1, [sp, #0x170]
10037814c: b4000081    	cbz	x1, 0x10037815c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x800>
100378150: f940bfe0    	ldr	x0, [sp, #0x178]
100378154: 52800022    	mov	w2, #0x1                ; =1
100378158: 97f22504    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10037815c: 910443e0    	add	x0, sp, #0x110
100378160: 9104a3e1    	add	x1, sp, #0x128
100378164: 9401d4ab    	bl	0x1003ed410 <__ZN5alloc3vec16Vec$LT$T$C$A$GT$8push_mut17h72def3fadbfe9639E>
100378168: b50000d7    	cbnz	x23, 0x100378180 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x824>
10037816c: 1400001f    	b	0x1003781e8 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x88c>
100378170: b4001157    	cbz	x23, 0x100378398 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xa3c>
100378174: 52800108    	mov	w8, #0x8                ; =8
100378178: a91123ff    	stp	xzr, x8, [sp, #0x110]
10037817c: f90093ff    	str	xzr, [sp, #0x120]
100378180: b0007ca2    	adrp	x2, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100378184: 911ee842    	add	x2, x2, #0x7ba
100378188: d10403a8    	sub	x8, x29, #0x100
10037818c: aa1803e0    	mov	x0, x24
100378190: aa1703e1    	mov	x1, x23
100378194: 52800063    	mov	w3, #0x3                ; =3
100378198: 97fae66b    	bl	0x100231b44 <__ZN5alloc3str17join_generic_copy17hc3fffb246d6f0b82E>
10037819c: 3cd003a0    	ldur	q0, [x29, #-0x100]
1003781a0: 3d805fe0    	str	q0, [sp, #0x170]
1003781a4: f85103a8    	ldur	x8, [x29, #-0xf0]
1003781a8: f900c3e8    	str	x8, [sp, #0x180]
1003781ac: 9105c3e8    	add	x8, sp, #0x170
1003781b0: a9306fa8    	stp	x8, x27, [x29, #-0x100]
1003781b4: 90008d40    	adrp	x0, 0x101520000 <_anon.164d03a2dfc98f969363eaf1c50504fd.175+0x1a1>
1003781b8: 91139c00    	add	x0, x0, #0x4e7
1003781bc: 910503e8    	add	x8, sp, #0x140
1003781c0: d10403a1    	sub	x1, x29, #0x100
1003781c4: 9437fdb4    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1003781c8: f940bbe1    	ldr	x1, [sp, #0x170]
1003781cc: b4000081    	cbz	x1, 0x1003781dc <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x880>
1003781d0: f940bfe0    	ldr	x0, [sp, #0x178]
1003781d4: 52800022    	mov	w2, #0x1                ; =1
1003781d8: 97f224e4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003781dc: 910443e0    	add	x0, sp, #0x110
1003781e0: 910503e1    	add	x1, sp, #0x140
1003781e4: 9401d48b    	bl	0x1003ed410 <__ZN5alloc3vec16Vec$LT$T$C$A$GT$8push_mut17h72def3fadbfe9639E>
1003781e8: 97f224e3    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003781ec: 52800220    	mov	w0, #0x11               ; =17
1003781f0: 52800021    	mov	w1, #0x1                ; =1
1003781f4: 97f224dc    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003781f8: b40018a0    	cbz	x0, 0x10037850c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xbb0>
1003781fc: aa0003f7    	mov	x23, x0
100378200: 52800f28    	mov	w8, #0x79               ; =121
100378204: 39004008    	strb	w8, [x0, #0x10]
100378208: b0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
10037820c: 910a6d08    	add	x8, x8, #0x29b
100378210: 3dc00100    	ldr	q0, [x8]
100378214: 3d800000    	str	q0, [x0]
100378218: 97f224d7    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10037821c: 528000e0    	mov	w0, #0x7                ; =7
100378220: 52800021    	mov	w1, #0x1                ; =1
100378224: 97f224d0    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100378228: b40017a0    	cbz	x0, 0x10037851c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xbc0>
10037822c: aa0003f8    	mov	x24, x0
100378230: 528d2dc8    	mov	w8, #0x696e             ; =26990
100378234: 72acedc8    	movk	w8, #0x676e, lsl #16
100378238: b8003008    	stur	w8, [x0, #0x3]
10037823c: 528c2ee8    	mov	w8, #0x6177             ; =24951
100378240: 72adce48    	movk	w8, #0x6e72, lsl #16
100378244: b9000008    	str	w8, [x0]
100378248: 97f224cb    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10037824c: 528006a0    	mov	w0, #0x35               ; =53
100378250: 52800021    	mov	w1, #0x1                ; =1
100378254: 97f224c4    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100378258: b40016a0    	cbz	x0, 0x10037852c <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xbd0>
10037825c: aa0003f9    	mov	x25, x0
100378260: b0007ca8    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100378264: 911ef508    	add	x8, x8, #0x7bd
100378268: ad400500    	ldp	q0, q1, [x8]
10037826c: ad000400    	stp	q0, q1, [x0]
100378270: 3dc00900    	ldr	q0, [x8, #0x20]
100378274: 3d800800    	str	q0, [x0, #0x20]
100378278: f842d108    	ldur	x8, [x8, #0x2d]
10037827c: f802d008    	stur	x8, [x0, #0x2d]
100378280: a95187e0    	ldp	x0, x1, [sp, #0x118]
100378284: f0007c82    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100378288: 9133f442    	add	x2, x2, #0xcfd
10037828c: d10403a8    	sub	x8, x29, #0x100
100378290: 52800063    	mov	w3, #0x3                ; =3
100378294: 97fae62c    	bl	0x100231b44 <__ZN5alloc3str17join_generic_copy17hc3fffb246d6f0b82E>
100378298: 3cd003a0    	ldur	q0, [x29, #-0x100]
10037829c: 3d805fe0    	str	q0, [sp, #0x170]
1003782a0: f85103a8    	ldur	x8, [x29, #-0xf0]
1003782a4: f900c3e8    	str	x8, [sp, #0x180]
1003782a8: 9105c3e8    	add	x8, sp, #0x170
1003782ac: a9306fa8    	stp	x8, x27, [x29, #-0x100]
1003782b0: 90008d40    	adrp	x0, 0x101520000 <_anon.164d03a2dfc98f969363eaf1c50504fd.175+0x1a1>
1003782b4: 91144400    	add	x0, x0, #0x511
1003782b8: 910563e8    	add	x8, sp, #0x158
1003782bc: d10403a1    	sub	x1, x29, #0x100
1003782c0: 9437fd75    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1003782c4: f940bbe1    	ldr	x1, [sp, #0x170]
1003782c8: b4000081    	cbz	x1, 0x1003782d8 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0x97c>
1003782cc: f940bfe0    	ldr	x0, [sp, #0x178]
1003782d0: 52800022    	mov	w2, #0x1                ; =1
1003782d4: 97f224a5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003782d8: 910167e8    	add	x8, sp, #0x59
1003782dc: 3ccff100    	ldur	q0, [x8, #0xff]
1003782e0: 3c8482c0    	stur	q0, [x22, #0x48]
1003782e4: f940b7e8    	ldr	x8, [sp, #0x168]
1003782e8: f9002ec8    	str	x8, [x22, #0x58]
1003782ec: 52800228    	mov	w8, #0x11               ; =17
1003782f0: a9005ec8    	stp	x8, x23, [x22]
1003782f4: d0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
1003782f8: 3dc33d00    	ldr	q0, [x8, #0xcf0]
1003782fc: 3d8006c0    	str	q0, [x22, #0x10]
100378300: f90012d8    	str	x24, [x22, #0x20]
100378304: d0007c88    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100378308: 3dc34100    	ldr	q0, [x8, #0xd00]
10037830c: 3c8282c0    	stur	q0, [x22, #0x28]
100378310: 528006a8    	mov	w8, #0x35               ; =53
100378314: a903a2d9    	stp	x25, x8, [x22, #0x38]
100378318: 52800028    	mov	w8, #0x1                ; =1
10037831c: 390182c8    	strb	w8, [x22, #0x60]
100378320: 910443e0    	add	x0, sp, #0x110
100378324: 9401800a    	bl	0x1003d834c <__ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h265e755c5c638973E>
100378328: 910323e0    	add	x0, sp, #0xc8
10037832c: 94018008    	bl	0x1003d834c <__ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h265e755c5c638973E>
100378330: 910203e0    	add	x0, sp, #0x80
100378334: 94018006    	bl	0x1003d834c <__ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h265e755c5c638973E>
100378338: d103a3a0    	sub	x0, x29, #0xe8
10037833c: 9413dcb3    	bl	0x10086f608 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5856a37aee37aca3E>
100378340: 910143e0    	add	x0, sp, #0x50
100378344: 9413dcb1    	bl	0x10086f608 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5856a37aee37aca3E>
100378348: 9100c3e0    	add	x0, sp, #0x30
10037834c: 94017c9d    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100378350: b40000b4    	cbz	x20, 0x100378364 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xa08>
100378354: aa1503e0    	mov	x0, x21
100378358: aa1403e1    	mov	x1, x20
10037835c: 52800022    	mov	w2, #0x1                ; =1
100378360: 97f22482    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100378364: f94003e1    	ldr	x1, [sp]
100378368: b4000081    	cbz	x1, 0x100378378 <__ZN13codexmate_lib4core5relay16codex_diagnostic23check_catalog_integrity17h8a486f7833341386E+0xa1c>
10037836c: aa1303e0    	mov	x0, x19
100378370: 52800022    	mov	w2, #0x1                ; =1
100378374: 97f2247d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100378378: 910903ff    	add	sp, sp, #0x240
10037837c: a9457bfd    	ldp	x29, x30, [sp, #0x50]
100378380: a9444ff4    	ldp	x20, x19, [sp, #0x40]
100378384: a94357f6    	ldp	x22, x21, [sp, #0x30]
100378388: a9425ff8    	ldp	x24, x23, [sp, #0x20]
