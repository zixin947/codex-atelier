
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001003898a0 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E>:
1003898a0: d10543ff    	sub	sp, sp, #0x150
1003898a4: a9106ffc    	stp	x28, x27, [sp, #0x100]
1003898a8: a9115ff8    	stp	x24, x23, [sp, #0x110]
1003898ac: a91257f6    	stp	x22, x21, [sp, #0x120]
1003898b0: a9134ff4    	stp	x20, x19, [sp, #0x130]
1003898b4: a9147bfd    	stp	x29, x30, [sp, #0x140]
1003898b8: 910503fd    	add	x29, sp, #0x140
1003898bc: 36000b05    	tbz	w5, #0x0, 0x100389a1c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x17c>
1003898c0: b4000bc1    	cbz	x1, 0x100389a38 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x198>
1003898c4: aa0303f5    	mov	x21, x3
1003898c8: aa0403f4    	mov	x20, x4
1003898cc: aa0003f3    	mov	x19, x0
1003898d0: d0007c09    	adrp	x9, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003898d4: 912b3d29    	add	x9, x9, #0xacf
1003898d8: 910003e8    	mov	x8, sp
1003898dc: aa0103e0    	mov	x0, x1
1003898e0: aa0203e1    	mov	x1, x2
1003898e4: aa0903e2    	mov	x2, x9
1003898e8: 52800243    	mov	w3, #0x12               ; =18
1003898ec: aa0603f6    	mov	x22, x6
1003898f0: 94100bab    	bl	0x10078c79c <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE>
1003898f4: f94003e8    	ldr	x8, [sp]
1003898f8: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
1003898fc: eb09011f    	cmp	x8, x9
100389900: 36000b76    	tbz	w22, #0x0, 0x100389a6c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x1cc>
100389904: 54000c81    	b.ne	0x100389a94 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x1f4>
100389908: 97f1df1b    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10038990c: 528002b4    	mov	w20, #0x15              ; =21
100389910: 52800037    	mov	w23, #0x1               ; =1
100389914: 528002a0    	mov	w0, #0x15               ; =21
100389918: 52800021    	mov	w1, #0x1                ; =1
10038991c: 97f1df12    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389920: b4000ce0    	cbz	x0, 0x100389abc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x21c>
100389924: aa0003f5    	mov	x21, x0
100389928: 90007c28    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
10038992c: 913e9108    	add	x8, x8, #0xfa4
100389930: 3dc00100    	ldr	q0, [x8]
100389934: 3d800000    	str	q0, [x0]
100389938: f840d108    	ldur	x8, [x8, #0xd]
10038993c: f800d008    	stur	x8, [x0, #0xd]
100389940: 97f1df0d    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389944: 528000e0    	mov	w0, #0x7                ; =7
100389948: 52800021    	mov	w1, #0x1                ; =1
10038994c: 97f1df06    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389950: b4003560    	cbz	x0, 0x100389ffc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x75c>
100389954: aa0003f4    	mov	x20, x0
100389958: 528d2dc8    	mov	w8, #0x696e             ; =26990
10038995c: 72acedc8    	movk	w8, #0x676e, lsl #16
100389960: b8003008    	stur	w8, [x0, #0x3]
100389964: 528c2ee8    	mov	w8, #0x6177             ; =24951
100389968: 72adce48    	movk	w8, #0x6e72, lsl #16
10038996c: b9000008    	str	w8, [x0]
100389970: 97f1df01    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389974: 52800580    	mov	w0, #0x2c               ; =44
100389978: 52800021    	mov	w1, #0x1                ; =1
10038997c: 97f1defa    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389980: b40034e0    	cbz	x0, 0x10038a01c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x77c>
100389984: aa0003f6    	mov	x22, x0
100389988: b0007c28    	adrp	x8, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
10038998c: 911f8508    	add	x8, x8, #0x7e1
100389990: ad400500    	ldp	q0, q1, [x8]
100389994: ad000400    	stp	q0, q1, [x0]
100389998: 3cc1c100    	ldur	q0, [x8, #0x1c]
10038999c: 3c81c000    	stur	q0, [x0, #0x1c]
1003899a0: 97f1def5    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003899a4: 52800037    	mov	w23, #0x1               ; =1
1003899a8: 52800ae0    	mov	w0, #0x57               ; =87
1003899ac: 52800021    	mov	w1, #0x1                ; =1
1003899b0: 97f1deed    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003899b4: b4003440    	cbz	x0, 0x10038a03c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x79c>
1003899b8: b0007c28    	adrp	x8, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
1003899bc: 91203508    	add	x8, x8, #0x80d
1003899c0: ad410500    	ldp	q0, q1, [x8, #0x20]
1003899c4: ad010400    	stp	q0, q1, [x0, #0x20]
1003899c8: 3dc01100    	ldr	q0, [x8, #0x40]
1003899cc: 3d801000    	str	q0, [x0, #0x40]
1003899d0: f844f109    	ldur	x9, [x8, #0x4f]
1003899d4: f804f009    	stur	x9, [x0, #0x4f]
1003899d8: ad400500    	ldp	q0, q1, [x8]
1003899dc: ad000400    	stp	q0, q1, [x0]
1003899e0: 528002a8    	mov	w8, #0x15               ; =21
1003899e4: a9005668    	stp	x8, x21, [x19]
1003899e8: d0007c08    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003899ec: 3dc03900    	ldr	q0, [x8, #0xe0]
1003899f0: 3d800660    	str	q0, [x19, #0x10]
1003899f4: f9001274    	str	x20, [x19, #0x20]
1003899f8: d0007c08    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003899fc: 3dc03d00    	ldr	q0, [x8, #0xf0]
100389a00: 3c828260    	stur	q0, [x19, #0x28]
100389a04: f9001e76    	str	x22, [x19, #0x38]
100389a08: d0007c08    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100389a0c: 3dc04100    	ldr	q0, [x8, #0x100]
100389a10: 3d801260    	str	q0, [x19, #0x40]
100389a14: 52800ae8    	mov	w8, #0x57               ; =87
100389a18: 1400006f    	b	0x100389bd4 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x334>
100389a1c: 90007c21    	adrp	x1, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100389a20: 913e9021    	add	x1, x1, #0xfa4
100389a24: b0007c23    	adrp	x3, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389a28: 9119f063    	add	x3, x3, #0x67c
100389a2c: 528002a2    	mov	w2, #0x15               ; =21
100389a30: 52800584    	mov	w4, #0x2c               ; =44
100389a34: 14000007    	b	0x100389a50 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x1b0>
100389a38: 90007c21    	adrp	x1, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100389a3c: 913e9021    	add	x1, x1, #0xfa4
100389a40: b0007c23    	adrp	x3, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389a44: 91219063    	add	x3, x3, #0x864
100389a48: 528002a2    	mov	w2, #0x15               ; =21
100389a4c: 528005a4    	mov	w4, #0x2d               ; =45
100389a50: a9547bfd    	ldp	x29, x30, [sp, #0x140]
100389a54: a9534ff4    	ldp	x20, x19, [sp, #0x130]
100389a58: a95257f6    	ldp	x22, x21, [sp, #0x120]
100389a5c: a9515ff8    	ldp	x24, x23, [sp, #0x110]
100389a60: a9506ffc    	ldp	x28, x27, [sp, #0x100]
100389a64: 910543ff    	add	sp, sp, #0x150
100389a68: 140082ff    	b	0x1003aa664 <__ZN13codexmate_lib4core5relay16codex_diagnostic7ok_item17h09598b1cce494186E>
100389a6c: 54000301    	b.ne	0x100389acc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x22c>
100389a70: 90007c21    	adrp	x1, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100389a74: 913e9021    	add	x1, x1, #0xfa4
100389a78: b0007c23    	adrp	x3, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389a7c: 911c0863    	add	x3, x3, #0x702
100389a80: aa1303e0    	mov	x0, x19
100389a84: 528002a2    	mov	w2, #0x15               ; =21
100389a88: 528006c4    	mov	w4, #0x36               ; =54
100389a8c: 940082f6    	bl	0x1003aa664 <__ZN13codexmate_lib4core5relay16codex_diagnostic7ok_item17h09598b1cce494186E>
100389a90: 14000053    	b	0x100389bdc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x33c>
100389a94: 910003e8    	mov	x8, sp
100389a98: f9000fe8    	str	x8, [sp, #0x18]
100389a9c: a94087e0    	ldp	x0, x1, [sp, #0x8]
100389aa0: aa1503e2    	mov	x2, x21
100389aa4: aa1403e3    	mov	x3, x20
100389aa8: 97ffa284    	bl	0x1003724b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E>
100389aac: a940d3f6    	ldp	x22, x20, [sp, #0x8]
100389ab0: 36000b40    	tbz	w0, #0x0, 0x100389c18 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x378>
100389ab4: b6f810b4    	tbz	x20, #0x3f, 0x100389cc8 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x428>
100389ab8: d2800017    	mov	x23, #0x0               ; =0
100389abc: aa1703e0    	mov	x0, x23
100389ac0: aa1403e1    	mov	x1, x20
100389ac4: 94391ae8    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100389ac8: 14000174    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
100389acc: 97f1deaa    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389ad0: 528002b4    	mov	w20, #0x15              ; =21
100389ad4: 52800037    	mov	w23, #0x1               ; =1
100389ad8: 528002a0    	mov	w0, #0x15               ; =21
100389adc: 52800021    	mov	w1, #0x1                ; =1
100389ae0: 97f1dea1    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389ae4: b4fffec0    	cbz	x0, 0x100389abc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x21c>
100389ae8: aa0003f5    	mov	x21, x0
100389aec: 90007c28    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100389af0: 913e9108    	add	x8, x8, #0xfa4
100389af4: 3dc00100    	ldr	q0, [x8]
100389af8: 3d800000    	str	q0, [x0]
100389afc: f840d108    	ldur	x8, [x8, #0xd]
100389b00: f800d008    	stur	x8, [x0, #0xd]
100389b04: 97f1de9c    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389b08: 528000a0    	mov	w0, #0x5                ; =5
100389b0c: 52800021    	mov	w1, #0x1                ; =1
100389b10: 97f1de95    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389b14: b40027c0    	cbz	x0, 0x10038a00c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x76c>
100389b18: aa0003f4    	mov	x20, x0
100389b1c: 52800e48    	mov	w8, #0x72               ; =114
100389b20: 39001008    	strb	w8, [x0, #0x4]
100389b24: 528e4ca8    	mov	w8, #0x7265             ; =29285
100389b28: 72adee48    	movk	w8, #0x6f72, lsl #16
100389b2c: b9000008    	str	w8, [x0]
100389b30: 97f1de91    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389b34: 52800480    	mov	w0, #0x24               ; =36
100389b38: 52800021    	mov	w1, #0x1                ; =1
100389b3c: 97f1de8a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389b40: b4002760    	cbz	x0, 0x10038a02c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x78c>
100389b44: aa0003f6    	mov	x22, x0
100389b48: 529cf0c8    	mov	w8, #0xe786             ; =59270
100389b4c: 72b2d368    	movk	w8, #0x969b, lsl #16
100389b50: b9002008    	str	w8, [x0, #0x20]
100389b54: b0007c28    	adrp	x8, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389b58: 911aa108    	add	x8, x8, #0x6a8
100389b5c: ad400500    	ldp	q0, q1, [x8]
100389b60: ad000400    	stp	q0, q1, [x0]
100389b64: 97f1de84    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389b68: 52800037    	mov	w23, #0x1               ; =1
100389b6c: 528006c0    	mov	w0, #0x36               ; =54
100389b70: 52800021    	mov	w1, #0x1                ; =1
100389b74: 97f1de7c    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389b78: b40026a0    	cbz	x0, 0x10038a04c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7ac>
100389b7c: b0007c28    	adrp	x8, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389b80: 911b3108    	add	x8, x8, #0x6cc
100389b84: ad400500    	ldp	q0, q1, [x8]
100389b88: ad000400    	stp	q0, q1, [x0]
100389b8c: 3dc00900    	ldr	q0, [x8, #0x20]
100389b90: 3d800800    	str	q0, [x0, #0x20]
100389b94: f842e108    	ldur	x8, [x8, #0x2e]
100389b98: f802e008    	stur	x8, [x0, #0x2e]
100389b9c: 528002a8    	mov	w8, #0x15               ; =21
100389ba0: a9005668    	stp	x8, x21, [x19]
100389ba4: d0007c08    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100389ba8: 3dc02d00    	ldr	q0, [x8, #0xb0]
100389bac: 3d800660    	str	q0, [x19, #0x10]
100389bb0: f9001274    	str	x20, [x19, #0x20]
100389bb4: b0007c08    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
100389bb8: 3dc3a500    	ldr	q0, [x8, #0xe90]
100389bbc: 3c828260    	stur	q0, [x19, #0x28]
100389bc0: f9001e76    	str	x22, [x19, #0x38]
100389bc4: d0007c08    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100389bc8: 3dc03100    	ldr	q0, [x8, #0xc0]
100389bcc: 3d801260    	str	q0, [x19, #0x40]
100389bd0: 528006c8    	mov	w8, #0x36               ; =54
100389bd4: a9052260    	stp	x0, x8, [x19, #0x50]
100389bd8: 39018277    	strb	w23, [x19, #0x60]
100389bdc: f94003e1    	ldr	x1, [sp]
100389be0: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100389be4: eb08003f    	cmp	x1, x8
100389be8: 540000a0    	b.eq	0x100389bfc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x35c>
100389bec: b4000081    	cbz	x1, 0x100389bfc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x35c>
100389bf0: f94007e0    	ldr	x0, [sp, #0x8]
100389bf4: 52800022    	mov	w2, #0x1                ; =1
100389bf8: 97f1de5c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100389bfc: a9547bfd    	ldp	x29, x30, [sp, #0x140]
100389c00: a9534ff4    	ldp	x20, x19, [sp, #0x130]
100389c04: a95257f6    	ldp	x22, x21, [sp, #0x120]
100389c08: a9515ff8    	ldp	x24, x23, [sp, #0x110]
100389c0c: a9506ffc    	ldp	x28, x27, [sp, #0x100]
100389c10: 910543ff    	add	sp, sp, #0x150
100389c14: d65f03c0    	ret
100389c18: f1002a9f    	cmp	x20, #0xa
100389c1c: 540003c3    	b.lo	0x100389c94 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x3f4>
100389c20: b0007c22    	adrp	x2, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389c24: 91198042    	add	x2, x2, #0x660
100389c28: 9101a3e8    	add	x8, sp, #0x68
100389c2c: aa1603e0    	mov	x0, x22
100389c30: aa1403e1    	mov	x1, x20
100389c34: 52800123    	mov	w3, #0x9                ; =9
100389c38: 9437ef20    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
100389c3c: 910143e0    	add	x0, sp, #0x50
100389c40: 9101a3e1    	add	x1, sp, #0x68
100389c44: 940192c4    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
100389c48: b94053e8    	ldr	w8, [sp, #0x50]
100389c4c: 37001388    	tbnz	w8, #0x0, 0x100389ebc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x61c>
100389c50: a940d3f6    	ldp	x22, x20, [sp, #0x8]
100389c54: f100269f    	cmp	x20, #0x9
100389c58: 54000fe9    	b.ls	0x100389e54 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x5b4>
100389c5c: b0007c22    	adrp	x2, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389c60: 9119a442    	add	x2, x2, #0x669
100389c64: 9101a3e8    	add	x8, sp, #0x68
100389c68: aa1603e0    	mov	x0, x22
100389c6c: aa1403e1    	mov	x1, x20
100389c70: 52800123    	mov	w3, #0x9                ; =9
100389c74: 9437ef11    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
100389c78: 910143e0    	add	x0, sp, #0x50
100389c7c: 9101a3e1    	add	x1, sp, #0x68
100389c80: 940192b5    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
100389c84: b94053e8    	ldr	w8, [sp, #0x50]
100389c88: 370011a8    	tbnz	w8, #0x0, 0x100389ebc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x61c>
100389c8c: a940d3f6    	ldp	x22, x20, [sp, #0x8]
100389c90: 1400007e    	b	0x100389e88 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x5e8>
100389c94: f100269f    	cmp	x20, #0x9
100389c98: 54000f81    	b.ne	0x100389e88 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x5e8>
100389c9c: f94002c8    	ldr	x8, [x22]
100389ca0: 394022c9    	ldrb	w9, [x22, #0x8]
100389ca4: d28c6c6a    	mov	x10, #0x6363            ; =25443
100389ca8: f2ae65aa    	movk	x10, #0x732d, lsl #16
100389cac: f2cd2eea    	movk	x10, #0x6977, lsl #32
100389cb0: f2ec6e8a    	movk	x10, #0x6374, lsl #48
100389cb4: eb0a011f    	cmp	x8, x10
100389cb8: 52800d08    	mov	w8, #0x68               ; =104
100389cbc: 7a480120    	ccmp	w9, w8, #0x0, eq
100389cc0: 54000cc1    	b.ne	0x100389e58 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x5b8>
100389cc4: 1400007e    	b	0x100389ebc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x61c>
100389cc8: b4000134    	cbz	x20, 0x100389cec <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x44c>
100389ccc: 97f1de2a    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389cd0: 52800037    	mov	w23, #0x1               ; =1
100389cd4: aa1403e0    	mov	x0, x20
100389cd8: 52800021    	mov	w1, #0x1                ; =1
100389cdc: 97f1de22    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389ce0: b4ffeee0    	cbz	x0, 0x100389abc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x21c>
100389ce4: aa0003f5    	mov	x21, x0
100389ce8: 14000002    	b	0x100389cf0 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x450>
100389cec: 52800035    	mov	w21, #0x1               ; =1
100389cf0: aa1503e0    	mov	x0, x21
100389cf4: aa1603e1    	mov	x1, x22
100389cf8: aa1403e2    	mov	x2, x20
100389cfc: 94391e6c    	bl	0x1011d16ac <_writev+0x1011d16ac>
100389d00: 9101a3e8    	add	x8, sp, #0x68
100389d04: aa1503e0    	mov	x0, x21
100389d08: aa1403e1    	mov	x1, x20
100389d0c: 9436fa3b    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
100389d10: a94687f6    	ldp	x22, x1, [sp, #0x68]
100389d14: aa1603e0    	mov	x0, x22
100389d18: 940143a2    	bl	0x1003daba0 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
100389d1c: b4000816    	cbz	x22, 0x100389e1c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x57c>
100389d20: 97f1de15    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389d24: 528002a0    	mov	w0, #0x15               ; =21
100389d28: 52800021    	mov	w1, #0x1                ; =1
100389d2c: 97f1de0e    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389d30: b40019e0    	cbz	x0, 0x10038a06c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7cc>
100389d34: aa0003f6    	mov	x22, x0
100389d38: 90007c28    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100389d3c: 913e9108    	add	x8, x8, #0xfa4
100389d40: 3dc00100    	ldr	q0, [x8]
100389d44: 3d800000    	str	q0, [x0]
100389d48: f840d108    	ldur	x8, [x8, #0xd]
100389d4c: f800d008    	stur	x8, [x0, #0xd]
100389d50: 97f1de09    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389d54: 528000a0    	mov	w0, #0x5                ; =5
100389d58: 52800021    	mov	w1, #0x1                ; =1
100389d5c: 97f1de02    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389d60: b40018e0    	cbz	x0, 0x10038a07c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7dc>
100389d64: aa0003f7    	mov	x23, x0
100389d68: 52800e48    	mov	w8, #0x72               ; =114
100389d6c: 39001008    	strb	w8, [x0, #0x4]
100389d70: 528e4ca8    	mov	w8, #0x7265             ; =29285
100389d74: 72adee48    	movk	w8, #0x6f72, lsl #16
100389d78: b9000008    	str	w8, [x0]
100389d7c: 97f1ddfe    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389d80: 52800560    	mov	w0, #0x2b               ; =43
100389d84: 52800021    	mov	w1, #0x1                ; =1
100389d88: 97f1ddf7    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389d8c: b4001800    	cbz	x0, 0x10038a08c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7ec>
100389d90: aa0003f8    	mov	x24, x0
100389d94: b0007c28    	adrp	x8, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389d98: 911e8908    	add	x8, x8, #0x7a2
100389d9c: ad400500    	ldp	q0, q1, [x8]
100389da0: ad000400    	stp	q0, q1, [x0]
100389da4: 3cc1b100    	ldur	q0, [x8, #0x1b]
100389da8: 3c81b000    	stur	q0, [x0, #0x1b]
100389dac: 910063e8    	add	x8, sp, #0x18
100389db0: 900026a9    	adrp	x9, 0x10085d000 <__ZN216_$LT$codexmate_lib..core..repository.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..repository..CodexMateSettings$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h9394a6c6fb5730f4E+0x128>
100389db4: 912d6129    	add	x9, x9, #0xb58
100389db8: a906a7e8    	stp	x8, x9, [sp, #0x68]
100389dbc: f0008ca0    	adrp	x0, 0x101520000 <_anon.164d03a2dfc98f969363eaf1c50504fd.175+0x1a1>
100389dc0: 913bf400    	add	x0, x0, #0xefd
100389dc4: 9100e3e8    	add	x8, sp, #0x38
100389dc8: 9101a3e1    	add	x1, sp, #0x68
100389dcc: 9437b6b2    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100389dd0: 3cc383e0    	ldur	q0, [sp, #0x38]
100389dd4: 3c848260    	stur	q0, [x19, #0x48]
100389dd8: f94027e8    	ldr	x8, [sp, #0x48]
100389ddc: f9002e68    	str	x8, [x19, #0x58]
100389de0: 528002a8    	mov	w8, #0x15               ; =21
100389de4: a9005a68    	stp	x8, x22, [x19]
100389de8: d0007c08    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100389dec: 3dc02d00    	ldr	q0, [x8, #0xb0]
100389df0: 3d800660    	str	q0, [x19, #0x10]
100389df4: f9001277    	str	x23, [x19, #0x20]
100389df8: d0007c08    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100389dfc: 3dc03500    	ldr	q0, [x8, #0xd0]
100389e00: 3c828260    	stur	q0, [x19, #0x28]
100389e04: 52800568    	mov	w8, #0x2b               ; =43
100389e08: a903a278    	stp	x24, x8, [x19, #0x38]
100389e0c: 52800028    	mov	w8, #0x1                ; =1
100389e10: 39018268    	strb	w8, [x19, #0x60]
100389e14: b5000174    	cbnz	x20, 0x100389e40 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x5a0>
100389e18: 17ffff71    	b	0x100389bdc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x33c>
100389e1c: 90007c21    	adrp	x1, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100389e20: 913e9021    	add	x1, x1, #0xfa4
100389e24: b0007c23    	adrp	x3, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389e28: 911f3463    	add	x3, x3, #0x7cd
100389e2c: aa1303e0    	mov	x0, x19
100389e30: 528002a2    	mov	w2, #0x15               ; =21
100389e34: 52800284    	mov	w4, #0x14               ; =20
100389e38: 9400820b    	bl	0x1003aa664 <__ZN13codexmate_lib4core5relay16codex_diagnostic7ok_item17h09598b1cce494186E>
100389e3c: b4ffed14    	cbz	x20, 0x100389bdc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x33c>
100389e40: aa1503e0    	mov	x0, x21
100389e44: aa1403e1    	mov	x1, x20
100389e48: 52800022    	mov	w2, #0x1                ; =1
100389e4c: 97f1ddc7    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100389e50: 17ffff63    	b	0x100389bdc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x33c>
100389e54: 540001a1    	b.ne	0x100389e88 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x5e8>
100389e58: f94002c8    	ldr	x8, [x22]
100389e5c: d28c6c69    	mov	x9, #0x6363             ; =25443
100389e60: f2ae65a9    	movk	x9, #0x732d, lsl #16
100389e64: f2cd2ee9    	movk	x9, #0x6977, lsl #32
100389e68: f2ec6e89    	movk	x9, #0x6374, lsl #48
100389e6c: 914c8129    	add	x9, x9, #0x320, lsl #12 ; =0x320000
100389e70: 394022ca    	ldrb	w10, [x22, #0x8]
100389e74: eb08013f    	cmp	x9, x8
100389e78: 52800d08    	mov	w8, #0x68               ; =104
100389e7c: 7a480140    	ccmp	w10, w8, #0x0, eq
100389e80: 540001e0    	b.eq	0x100389ebc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x61c>
100389e84: 52800134    	mov	w20, #0x9               ; =9
100389e88: b0007c40    	adrp	x0, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
100389e8c: 913b4000    	add	x0, x0, #0xed0
100389e90: 52800101    	mov	w1, #0x8                ; =8
100389e94: aa1603e2    	mov	x2, x22
100389e98: aa1403e3    	mov	x3, x20
100389e9c: 94018b19    	bl	0x1003ecb00 <__ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15is_contained_in17h430773999f818014E>
100389ea0: 370000e0    	tbnz	w0, #0x0, 0x100389ebc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x61c>
100389ea4: a9408fe2    	ldp	x2, x3, [sp, #0x8]
100389ea8: b0007c20    	adrp	x0, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389eac: 911ce000    	add	x0, x0, #0x738
100389eb0: 52800181    	mov	w1, #0xc                ; =12
100389eb4: 94018b13    	bl	0x1003ecb00 <__ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15is_contained_in17h430773999f818014E>
100389eb8: 36000880    	tbz	w0, #0x0, 0x100389fc8 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x728>
100389ebc: 97f1ddae    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389ec0: 52800694    	mov	w20, #0x34              ; =52
100389ec4: 52800037    	mov	w23, #0x1               ; =1
100389ec8: 52800680    	mov	w0, #0x34               ; =52
100389ecc: 52800021    	mov	w1, #0x1                ; =1
100389ed0: 97f1dda5    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389ed4: b4ffdf40    	cbz	x0, 0x100389abc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x21c>
100389ed8: aa0003f5    	mov	x21, x0
100389edc: 529cb5e8    	mov	w8, #0xe5af             ; =58799
100389ee0: 72b097c8    	movk	w8, #0x84be, lsl #16
100389ee4: b9003008    	str	w8, [x0, #0x30]
100389ee8: b0007c28    	adrp	x8, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
100389eec: 911db908    	add	x8, x8, #0x76e
100389ef0: ad400500    	ldp	q0, q1, [x8]
100389ef4: ad000400    	stp	q0, q1, [x0]
100389ef8: 3dc00900    	ldr	q0, [x8, #0x20]
100389efc: 3d800800    	str	q0, [x0, #0x20]
100389f00: 52800698    	mov	w24, #0x34              ; =52
100389f04: 52800694    	mov	w20, #0x34              ; =52
100389f08: 97f1dd9b    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389f0c: 528002a0    	mov	w0, #0x15               ; =21
100389f10: 52800021    	mov	w1, #0x1                ; =1
100389f14: 97f1dd94    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389f18: b40006a0    	cbz	x0, 0x100389fec <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x74c>
100389f1c: aa0003f6    	mov	x22, x0
100389f20: 90007c28    	adrp	x8, 0x10130d000 <_anon.164d03a2dfc98f969363eaf1c50504fd.450+0x126>
100389f24: 913e9108    	add	x8, x8, #0xfa4
100389f28: 3dc00100    	ldr	q0, [x8]
100389f2c: 3d800000    	str	q0, [x0]
100389f30: f840d108    	ldur	x8, [x8, #0xd]
100389f34: f800d008    	stur	x8, [x0, #0xd]
100389f38: 97f1dd8f    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389f3c: 528000a0    	mov	w0, #0x5                ; =5
100389f40: 52800021    	mov	w1, #0x1                ; =1
100389f44: 97f1dd88    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389f48: b40008a0    	cbz	x0, 0x10038a05c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7bc>
100389f4c: aa0003f7    	mov	x23, x0
100389f50: 52800e48    	mov	w8, #0x72               ; =114
100389f54: 39001008    	strb	w8, [x0, #0x4]
100389f58: 528e4ca8    	mov	w8, #0x7265             ; =29285
100389f5c: 72adee48    	movk	w8, #0x6f72, lsl #16
100389f60: b9000008    	str	w8, [x0]
100389f64: 910063e8    	add	x8, sp, #0x18
100389f68: 900026a9    	adrp	x9, 0x10085d000 <__ZN216_$LT$codexmate_lib..core..repository.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..repository..CodexMateSettings$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h9394a6c6fb5730f4E+0x128>
100389f6c: 912d6129    	add	x9, x9, #0xb58
100389f70: a906a7e8    	stp	x8, x9, [sp, #0x68]
100389f74: f0008ca0    	adrp	x0, 0x101520000 <_anon.164d03a2dfc98f969363eaf1c50504fd.175+0x1a1>
100389f78: 913bac00    	add	x0, x0, #0xeeb
100389f7c: 910083e8    	add	x8, sp, #0x20
100389f80: 9101a3e1    	add	x1, sp, #0x68
100389f84: 9437b644    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100389f88: 3dc00be0    	ldr	q0, [sp, #0x20]
100389f8c: 3c848260    	stur	q0, [x19, #0x48]
100389f90: f9401be8    	ldr	x8, [sp, #0x30]
100389f94: f9002e68    	str	x8, [x19, #0x58]
100389f98: 528002a8    	mov	w8, #0x15               ; =21
100389f9c: a9005a68    	stp	x8, x22, [x19]
100389fa0: d0007c08    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100389fa4: 3dc02d00    	ldr	q0, [x8, #0xb0]
100389fa8: 3d800660    	str	q0, [x19, #0x10]
100389fac: 528000a8    	mov	w8, #0x5                ; =5
100389fb0: a9022277    	stp	x23, x8, [x19, #0x20]
100389fb4: a9035674    	stp	x20, x21, [x19, #0x30]
100389fb8: f9002278    	str	x24, [x19, #0x40]
100389fbc: 52800028    	mov	w8, #0x1                ; =1
100389fc0: 39018268    	strb	w8, [x19, #0x60]
100389fc4: 17ffff06    	b	0x100389bdc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x33c>
100389fc8: 9101a3e0    	add	x0, sp, #0x68
100389fcc: 94018aaa    	bl	0x1003eca74 <__ZN4core6option15Option$LT$T$GT$11map_or_else17hee759b27a6a279f3E>
100389fd0: a946d7f4    	ldp	x20, x21, [sp, #0x68]
100389fd4: f9403ff8    	ldr	x24, [sp, #0x78]
100389fd8: 97f1dd67    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100389fdc: 528002a0    	mov	w0, #0x15               ; =21
100389fe0: 52800021    	mov	w1, #0x1                ; =1
100389fe4: 97f1dd60    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100389fe8: b5fff9a0    	cbnz	x0, 0x100389f1c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x67c>
100389fec: 52800020    	mov	w0, #0x1                ; =1
100389ff0: 528002a1    	mov	w1, #0x15               ; =21
100389ff4: 9439199c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100389ff8: 14000028    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
100389ffc: 52800020    	mov	w0, #0x1                ; =1
10038a000: 528000e1    	mov	w1, #0x7                ; =7
10038a004: 94391998    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a008: 14000024    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
10038a00c: 52800020    	mov	w0, #0x1                ; =1
10038a010: 528000a1    	mov	w1, #0x5                ; =5
10038a014: 94391994    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a018: 14000020    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
10038a01c: 52800020    	mov	w0, #0x1                ; =1
10038a020: 52800581    	mov	w1, #0x2c               ; =44
10038a024: 94391990    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a028: 1400001c    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
10038a02c: 52800020    	mov	w0, #0x1                ; =1
10038a030: 52800481    	mov	w1, #0x24               ; =36
10038a034: 9439198c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a038: 14000018    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
10038a03c: 52800020    	mov	w0, #0x1                ; =1
10038a040: 52800ae1    	mov	w1, #0x57               ; =87
10038a044: 94391988    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a048: 14000014    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
10038a04c: 52800020    	mov	w0, #0x1                ; =1
10038a050: 528006c1    	mov	w1, #0x36               ; =54
10038a054: 94391984    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a058: 14000010    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
10038a05c: 52800020    	mov	w0, #0x1                ; =1
10038a060: 528000a1    	mov	w1, #0x5                ; =5
10038a064: 94391980    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a068: 1400000c    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
10038a06c: 52800020    	mov	w0, #0x1                ; =1
10038a070: 528002a1    	mov	w1, #0x15               ; =21
10038a074: 9439197c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a078: 14000008    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
10038a07c: 52800020    	mov	w0, #0x1                ; =1
10038a080: 528000a1    	mov	w1, #0x5                ; =5
10038a084: 94391978    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a088: 14000004    	b	0x10038a098 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x7f8>
10038a08c: 52800020    	mov	w0, #0x1                ; =1
10038a090: 52800561    	mov	w1, #0x2b               ; =43
10038a094: 94391974    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10038a098: d4200020    	brk	#0x1
10038a09c: aa0003f3    	mov	x19, x0
10038a0a0: aa1803e0    	mov	x0, x24
10038a0a4: 52800561    	mov	w1, #0x2b               ; =43
10038a0a8: 52800022    	mov	w2, #0x1                ; =1
10038a0ac: 97f1dd2f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a0b0: 14000002    	b	0x10038a0b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x818>
10038a0b4: aa0003f3    	mov	x19, x0
10038a0b8: aa1703e0    	mov	x0, x23
10038a0bc: 528000a1    	mov	w1, #0x5                ; =5
10038a0c0: 52800022    	mov	w2, #0x1                ; =1
10038a0c4: 97f1dd29    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a0c8: 14000002    	b	0x10038a0d0 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x830>
10038a0cc: aa0003f3    	mov	x19, x0
10038a0d0: aa1603e0    	mov	x0, x22
10038a0d4: 528002a1    	mov	w1, #0x15               ; =21
10038a0d8: 52800022    	mov	w2, #0x1                ; =1
10038a0dc: 97f1dd23    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a0e0: 14000019    	b	0x10038a144 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x8a4>
10038a0e4: aa0003f3    	mov	x19, x0
10038a0e8: b40000b4    	cbz	x20, 0x10038a0fc <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x85c>
10038a0ec: aa1503e0    	mov	x0, x21
10038a0f0: aa1403e1    	mov	x1, x20
10038a0f4: 52800022    	mov	w2, #0x1                ; =1
10038a0f8: 97f1dd1c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a0fc: aa1703e0    	mov	x0, x23
10038a100: 528000a1    	mov	w1, #0x5                ; =5
10038a104: 52800022    	mov	w2, #0x1                ; =1
10038a108: 97f1dd18    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a10c: aa1603e0    	mov	x0, x22
10038a110: 14000029    	b	0x10038a1b4 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x914>
10038a114: aa0003f3    	mov	x19, x0
10038a118: aa1603e0    	mov	x0, x22
10038a11c: 528002a1    	mov	w1, #0x15               ; =21
10038a120: 52800022    	mov	w2, #0x1                ; =1
10038a124: 97f1dd11    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a128: 14000002    	b	0x10038a130 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x890>
10038a12c: aa0003f3    	mov	x19, x0
10038a130: b40000d4    	cbz	x20, 0x10038a148 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x8a8>
10038a134: aa1503e0    	mov	x0, x21
10038a138: aa1403e1    	mov	x1, x20
10038a13c: 1400001f    	b	0x10038a1b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x918>
10038a140: aa0003f3    	mov	x19, x0
10038a144: b5ffff94    	cbnz	x20, 0x10038a134 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x894>
10038a148: aa1303e0    	mov	x0, x19
10038a14c: 1400001f    	b	0x10038a1c8 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x928>
10038a150: aa0003f3    	mov	x19, x0
10038a154: aa1603e0    	mov	x0, x22
10038a158: 52800481    	mov	w1, #0x24               ; =36
10038a15c: 52800022    	mov	w2, #0x1                ; =1
10038a160: 97f1dd02    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a164: 14000008    	b	0x10038a184 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x8e4>
10038a168: aa0003f3    	mov	x19, x0
10038a16c: aa1603e0    	mov	x0, x22
10038a170: 52800581    	mov	w1, #0x2c               ; =44
10038a174: 52800022    	mov	w2, #0x1                ; =1
10038a178: 97f1dcfc    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a17c: 14000006    	b	0x10038a194 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x8f4>
10038a180: aa0003f3    	mov	x19, x0
10038a184: aa1403e0    	mov	x0, x20
10038a188: 528000a1    	mov	w1, #0x5                ; =5
10038a18c: 14000004    	b	0x10038a19c <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x8fc>
10038a190: aa0003f3    	mov	x19, x0
10038a194: aa1403e0    	mov	x0, x20
10038a198: 528000e1    	mov	w1, #0x7                ; =7
10038a19c: 52800022    	mov	w2, #0x1                ; =1
10038a1a0: 97f1dcf2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a1a4: 14000003    	b	0x10038a1b0 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x910>
10038a1a8: 14000001    	b	0x10038a1ac <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x90c>
10038a1ac: aa0003f3    	mov	x19, x0
10038a1b0: aa1503e0    	mov	x0, x21
10038a1b4: 528002a1    	mov	w1, #0x15               ; =21
10038a1b8: 52800022    	mov	w2, #0x1                ; =1
10038a1bc: 97f1dceb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a1c0: aa1303e0    	mov	x0, x19
10038a1c4: 14000001    	b	0x10038a1c8 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x928>
10038a1c8: f94003e1    	ldr	x1, [sp]
10038a1cc: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10038a1d0: eb08003f    	cmp	x1, x8
10038a1d4: 54000100    	b.eq	0x10038a1f4 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x954>
10038a1d8: b40000e1    	cbz	x1, 0x10038a1f4 <__ZN13codexmate_lib4core5relay16codex_diagnostic27check_catalog_path_validity17h292ed833e9f527a8E+0x954>
10038a1dc: f94007e8    	ldr	x8, [sp, #0x8]
10038a1e0: aa0003f3    	mov	x19, x0
10038a1e4: aa0803e0    	mov	x0, x8
10038a1e8: 52800022    	mov	w2, #0x1                ; =1
10038a1ec: 97f1dcdf    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10038a1f0: aa1303e0    	mov	x0, x19
10038a1f4: 94391bd8    	bl	0x1011d1154 <_writev+0x1011d1154>

000000010038a1f8 <__ZN13codexmate_lib4core5relay16codex_diagnostic27fix_config_profile_conflict17he5eaca380d4b6c3bE>:
10038a1f8: d10543ff    	sub	sp, sp, #0x150
10038a1fc: a90f6ffc    	stp	x28, x27, [sp, #0xf0]
10038a200: a91067fa    	stp	x26, x25, [sp, #0x100]
10038a204: a9115ff8    	stp	x24, x23, [sp, #0x110]
10038a208: a91257f6    	stp	x22, x21, [sp, #0x120]
10038a20c: a9134ff4    	stp	x20, x19, [sp, #0x130]
10038a210: a9147bfd    	stp	x29, x30, [sp, #0x140]
10038a214: 910503fd    	add	x29, sp, #0x140
10038a218: aa0303f7    	mov	x23, x3
10038a21c: aa0203f5    	mov	x21, x2
10038a220: aa0103f6    	mov	x22, x1
10038a224: aa0003f4    	mov	x20, x0
10038a228: aa0803f3    	mov	x19, x8
10038a22c: 910203e8    	add	x8, sp, #0x80
10038a230: 94003541    	bl	0x100397734 <__ZN13codexmate_lib4core5relay16codex_diagnostic32fix_config_profile_conflict_text17hf79ba0f8bae92502E>
10038a234: a94807e8    	ldp	x8, x1, [sp, #0x80]
10038a238: f9404be0    	ldr	x0, [sp, #0x90]
10038a23c: f1002d1f    	cmp	x8, #0xb
10038a240: 54000561    	b.ne	0x10038a2ec <__ZN13codexmate_lib4core5relay16codex_diagnostic27fix_config_profile_conflict17he5eaca380d4b6c3bE+0xf4>
10038a244: b4000061    	cbz	x1, 0x10038a250 <__ZN13codexmate_lib4core5relay16codex_diagnostic27fix_config_profile_conflict17he5eaca380d4b6c3bE+0x58>
10038a248: 52800022    	mov	w2, #0x1                ; =1
10038a24c: 97f1dcc7    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
