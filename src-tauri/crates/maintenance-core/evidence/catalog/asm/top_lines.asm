
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078c01c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E>:
10078c01c: d10403ff    	sub	sp, sp, #0x100
10078c020: a90a6ffc    	stp	x28, x27, [sp, #0xa0]
10078c024: a90b67fa    	stp	x26, x25, [sp, #0xb0]
10078c028: a90c5ff8    	stp	x24, x23, [sp, #0xc0]
10078c02c: a90d57f6    	stp	x22, x21, [sp, #0xd0]
10078c030: a90e4ff4    	stp	x20, x19, [sp, #0xe0]
10078c034: a90f7bfd    	stp	x29, x30, [sp, #0xf0]
10078c038: 9103c3fd    	add	x29, sp, #0xf0
10078c03c: aa0003f3    	mov	x19, x0
10078c040: 9e6703e0    	fmov	d0, xzr
10078c044: 52800008    	mov	w8, #0x0                ; =0
10078c048: d28c2dbc    	mov	x28, #0x616d            ; =24941
10078c04c: f2ad2dbc    	movk	x28, #0x696d, lsl #16
10078c050: f2ce45bc    	movk	x28, #0x722d, lsl #32
10078c054: f2ed8cbc    	movk	x28, #0x6c65, lsl #48
10078c058: 52800109    	mov	w9, #0x8                ; =8
10078c05c: a90027ff    	stp	xzr, x9, [sp]
10078c060: f9000bff    	str	xzr, [sp, #0x10]
10078c064: 52800049    	mov	w9, #0x2                ; =2
10078c068: 39007fe9    	strb	w9, [sp, #0x1f]
10078c06c: 6e180400    	mov.d	v0[1], v0[0]
10078c070: 3d800fe0    	str	q0, [sp, #0x30]
10078c074: d28f2c3a    	mov	x26, #0x7961            ; =31073
10078c078: f2ada41a    	movk	x26, #0x6d20, lsl #16
10078c07c: f2cdcc3a    	movk	x26, #0x6e61, lsl #32
10078c080: f2ecec3a    	movk	x26, #0x6761, lsl #48
10078c084: a9020be1    	stp	x1, x2, [sp, #0x20]
10078c088: a90407e2    	stp	x2, x1, [sp, #0x40]
10078c08c: 52800034    	mov	w20, #0x1               ; =1
10078c090: a9057fe2    	stp	x2, xzr, [sp, #0x50]
10078c094: f90033e2    	str	x2, [sp, #0x60]
10078c098: 0f000540    	movi.2s	v0, #0xa
10078c09c: fd0037e0    	str	d0, [sp, #0x68]
10078c0a0: 3901c3f4    	strb	w20, [sp, #0x70]
10078c0a4: 7900f3ff    	strh	wzr, [sp, #0x78]
10078c0a8: 14000002    	b	0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c0ac: 52800008    	mov	w8, #0x0                ; =0
10078c0b0: aa0803fb    	mov	x27, x8
10078c0b4: 910083e0    	add	x0, sp, #0x20
10078c0b8: 940006cb    	bl	0x10078dbe4 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E>
10078c0bc: aa0003f7    	mov	x23, x0
10078c0c0: b40021e0    	cbz	x0, 0x10078c4fc <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x4e0>
10078c0c4: aa0103f6    	mov	x22, x1
10078c0c8: aa1703e0    	mov	x0, x23
10078c0cc: 97ea8c23    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
10078c0d0: aa0003f9    	mov	x25, x0
10078c0d4: aa0103f8    	mov	x24, x1
10078c0d8: 91007fe0    	add	x0, sp, #0x1f
10078c0dc: aa1703e1    	mov	x1, x23
10078c0e0: aa1603e2    	mov	x2, x22
10078c0e4: 9400064e    	bl	0x10078da1c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE>
10078c0e8: aa1b03e8    	mov	x8, x27
10078c0ec: 34fffe20    	cbz	w0, 0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c0f0: f1009b1f    	cmp	x24, #0x26
10078c0f4: 5400052d    	b.le	0x10078c198 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x17c>
10078c0f8: f100ef1f    	cmp	x24, #0x3b
10078c0fc: 5400082c    	b.gt	0x10078c200 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x1e4>
10078c100: f1009f1f    	cmp	x24, #0x27
10078c104: 54000dc0    	b.eq	0x10078c2bc <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x2a0>
10078c108: f100df1f    	cmp	x24, #0x37
10078c10c: 540017c1    	b.ne	0x10078c404 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x3e8>
10078c110: a9402728    	ldp	x8, x9, [x25]
10078c114: a9412f2a    	ldp	x10, x11, [x25, #0x10]
10078c118: a942372c    	ldp	x12, x13, [x25, #0x20]
10078c11c: f842f32e    	ldur	x14, [x25, #0x2f]
10078c120: d284046f    	mov	x15, #0x2023            ; =8227
10078c124: f2a7c7cf    	movk	x15, #0x3e3e, lsl #16
10078c128: f2c407cf    	movk	x15, #0x203e, lsl #32
10078c12c: f2ed2c2f    	movk	x15, #0x6961, lsl #48
10078c130: eb0f011f    	cmp	x8, x15
10078c134: fa5c0120    	ccmp	x9, x28, #0x0, eq
10078c138: fa5a0140    	ccmp	x10, x26, #0x0, eq
10078c13c: d28c8ca8    	mov	x8, #0x6465             ; =25701
10078c140: f2ae6408    	movk	x8, #0x7320, lsl #16
10078c144: f2cc2e88    	movk	x8, #0x6174, lsl #32
10078c148: f2ee8e48    	movk	x8, #0x7472, lsl #48
10078c14c: fa480160    	ccmp	x11, x8, #0x0, eq
10078c150: d2850408    	mov	x8, #0x2820             ; =10272
10078c154: f2a9e888    	movk	x8, #0x4f44, lsl #16
10078c158: f2c9c408    	movk	x8, #0x4e20, lsl #32
10078c15c: f2ea89e8    	movk	x8, #0x544f, lsl #48
10078c160: fa480180    	ccmp	x12, x8, #0x0, eq
10078c164: d288a408    	mov	x8, #0x4520             ; =17696
10078c168: f2a92888    	movk	x8, #0x4944, lsl #16
10078c16c: f2c40a88    	movk	x8, #0x2054, lsl #32
10078c170: f2e829a8    	movk	x8, #0x414d, lsl #48
10078c174: fa4801a0    	ccmp	x13, x8, #0x0, eq
10078c178: d289c828    	mov	x8, #0x4e41             ; =20033
10078c17c: f2a82aa8    	movk	x8, #0x4155, lsl #16
10078c180: f2c98988    	movk	x8, #0x4c4c, lsl #32
10078c184: f2e52b28    	movk	x8, #0x2959, lsl #48
10078c188: fa4801c0    	ccmp	x14, x8, #0x0, eq
10078c18c: aa1b03e8    	mov	x8, x27
10078c190: 540013a1    	b.ne	0x10078c404 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x3e8>
10078c194: 17ffffc7    	b	0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c198: b4001538    	cbz	x24, 0x10078c43c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x420>
10078c19c: f1007b1f    	cmp	x24, #0x1e
10078c1a0: 54001100    	b.eq	0x10078c3c0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x3a4>
10078c1a4: f100931f    	cmp	x24, #0x24
10078c1a8: 540012e1    	b.ne	0x10078c404 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x3e8>
10078c1ac: a9402728    	ldp	x8, x9, [x25]
10078c1b0: a9412f2a    	ldp	x10, x11, [x25, #0x10]
10078c1b4: b940232c    	ldr	w12, [x25, #0x20]
10078c1b8: d284046d    	mov	x13, #0x2023            ; =8227
10078c1bc: f2a7878d    	movk	x13, #0x3c3c, lsl #16
10078c1c0: f2c4078d    	movk	x13, #0x203c, lsl #32
10078c1c4: f2ed2c2d    	movk	x13, #0x6961, lsl #48
10078c1c8: eb0d011f    	cmp	x8, x13
10078c1cc: fa5c0120    	ccmp	x9, x28, #0x0, eq
10078c1d0: fa5a0140    	ccmp	x10, x26, #0x0, eq
10078c1d4: d28c8ca8    	mov	x8, #0x6465             ; =25701
10078c1d8: f2aca408    	movk	x8, #0x6520, lsl #16
10078c1dc: f2cc8dc8    	movk	x8, #0x646e, lsl #32
10078c1e0: f2e50408    	movk	x8, #0x2820, lsl #48
10078c1e4: fa480160    	ccmp	x11, x8, #0x0, eq
10078c1e8: 528dee88    	mov	w8, #0x6f74             ; =28532
10078c1ec: 72a52e08    	movk	w8, #0x2970, lsl #16
10078c1f0: 7a480180    	ccmp	w12, w8, #0x0, eq
10078c1f4: aa1b03e8    	mov	x8, x27
10078c1f8: 54001061    	b.ne	0x10078c404 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x3e8>
10078c1fc: 17ffffad    	b	0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c200: f100f31f    	cmp	x24, #0x3c
10078c204: 54000920    	b.eq	0x10078c328 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x30c>
10078c208: f101031f    	cmp	x24, #0x40
10078c20c: 54000fc1    	b.ne	0x10078c404 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x3e8>
10078c210: a9402728    	ldp	x8, x9, [x25]
10078c214: a9412f2a    	ldp	x10, x11, [x25, #0x10]
10078c218: a942372c    	ldp	x12, x13, [x25, #0x20]
10078c21c: a9433f2e    	ldp	x14, x15, [x25, #0x30]
10078c220: d2840470    	mov	x16, #0x2023            ; =8227
10078c224: f2a7c7d0    	movk	x16, #0x3e3e, lsl #16
10078c228: f2c407d0    	movk	x16, #0x203e, lsl #32
10078c22c: f2ed2c30    	movk	x16, #0x6961, lsl #48
10078c230: eb10011f    	cmp	x8, x16
10078c234: fa5c0120    	ccmp	x9, x28, #0x0, eq
10078c238: d28f2c28    	mov	x8, #0x7961             ; =31073
10078c23c: f2ac6408    	movk	x8, #0x6320, lsl #16
10078c240: f2cc8de8    	movk	x8, #0x646f, lsl #32
10078c244: f2ef0ca8    	movk	x8, #0x7865, lsl #48
10078c248: fa480140    	ccmp	x10, x8, #0x0, eq
10078c24c: d28e45a8    	mov	x8, #0x722d             ; =29229
10078c250: f2aeade8    	movk	x8, #0x756f, lsl #16
10078c254: f2ccae88    	movk	x8, #0x6574, lsl #32
10078c258: f2e40e48    	movk	x8, #0x2072, lsl #48
10078c25c: fa480160    	ccmp	x11, x8, #0x0, eq
10078c260: d28dee88    	mov	x8, #0x6f74             ; =28532
10078c264: f2a40e08    	movk	x8, #0x2070, lsl #16
10078c268: f2ce8e68    	movk	x8, #0x7473, lsl #32
10078c26c: f2ee4c28    	movk	x8, #0x7261, lsl #48
10078c270: fa480180    	ccmp	x12, x8, #0x0, eq
10078c274: d2840e88    	mov	x8, #0x2074             ; =8308
10078c278: f2a88508    	movk	x8, #0x4428, lsl #16
10078c27c: f2c409e8    	movk	x8, #0x204f, lsl #32
10078c280: f2e9e9c8    	movk	x8, #0x4f4e, lsl #48
10078c284: fa4801a0    	ccmp	x13, x8, #0x0, eq
10078c288: d2840a88    	mov	x8, #0x2054             ; =8276
10078c28c: f2a888a8    	movk	x8, #0x4445, lsl #16
10078c290: f2ca8928    	movk	x8, #0x5449, lsl #32
10078c294: f2e9a408    	movk	x8, #0x4d20, lsl #48
10078c298: fa4801c0    	ccmp	x14, x8, #0x0, eq
10078c29c: d289c828    	mov	x8, #0x4e41             ; =20033
10078c2a0: f2a82aa8    	movk	x8, #0x4155, lsl #16
10078c2a4: f2c98988    	movk	x8, #0x4c4c, lsl #32
10078c2a8: f2e52b28    	movk	x8, #0x2959, lsl #48
10078c2ac: fa4801e0    	ccmp	x15, x8, #0x0, eq
10078c2b0: aa1b03e8    	mov	x8, x27
10078c2b4: 54000a81    	b.ne	0x10078c404 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x3e8>
10078c2b8: 17ffff7e    	b	0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c2bc: a9402728    	ldp	x8, x9, [x25]
10078c2c0: a9412f2a    	ldp	x10, x11, [x25, #0x10]
10078c2c4: f841f32c    	ldur	x12, [x25, #0x1f]
10078c2c8: d284046d    	mov	x13, #0x2023            ; =8227
10078c2cc: f2a7878d    	movk	x13, #0x3c3c, lsl #16
10078c2d0: f2c4078d    	movk	x13, #0x203c, lsl #32
10078c2d4: f2ed2c2d    	movk	x13, #0x6961, lsl #48
10078c2d8: eb0d011f    	cmp	x8, x13
10078c2dc: fa5c0120    	ccmp	x9, x28, #0x0, eq
10078c2e0: d28f2c28    	mov	x8, #0x7961             ; =31073
10078c2e4: f2ac6408    	movk	x8, #0x6320, lsl #16
10078c2e8: f2cc8de8    	movk	x8, #0x646f, lsl #32
10078c2ec: f2ef0ca8    	movk	x8, #0x7865, lsl #48
10078c2f0: fa480140    	ccmp	x10, x8, #0x0, eq
10078c2f4: d28e45a8    	mov	x8, #0x722d             ; =29229
10078c2f8: f2aeade8    	movk	x8, #0x756f, lsl #16
10078c2fc: f2ccae88    	movk	x8, #0x6574, lsl #32
10078c300: f2e40e48    	movk	x8, #0x2072, lsl #48
10078c304: fa480160    	ccmp	x11, x8, #0x0, eq
10078c308: d28e8408    	mov	x8, #0x7420             ; =29728
10078c30c: f2ae0de8    	movk	x8, #0x706f, lsl #16
10078c310: f2cca408    	movk	x8, #0x6520, lsl #32
10078c314: f2ec8dc8    	movk	x8, #0x646e, lsl #48
10078c318: fa480180    	ccmp	x12, x8, #0x0, eq
10078c31c: aa1b03e8    	mov	x8, x27
10078c320: 54000721    	b.ne	0x10078c404 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x3e8>
10078c324: 17ffff63    	b	0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c328: a9402728    	ldp	x8, x9, [x25]
10078c32c: a9412f2a    	ldp	x10, x11, [x25, #0x10]
10078c330: a942372c    	ldp	x12, x13, [x25, #0x20]
10078c334: f9401b2e    	ldr	x14, [x25, #0x30]
10078c338: b9403b2f    	ldr	w15, [x25, #0x38]
10078c33c: d2840470    	mov	x16, #0x2023            ; =8227
10078c340: f2a7c7d0    	movk	x16, #0x3e3e, lsl #16
10078c344: f2c407d0    	movk	x16, #0x203e, lsl #32
10078c348: f2ed2c30    	movk	x16, #0x6961, lsl #48
10078c34c: eb10011f    	cmp	x8, x16
10078c350: fa5c0120    	ccmp	x9, x28, #0x0, eq
10078c354: fa5a0140    	ccmp	x10, x26, #0x0, eq
10078c358: d28c8ca8    	mov	x8, #0x6465             ; =25701
10078c35c: f2ae6408    	movk	x8, #0x7320, lsl #16
10078c360: f2cc2e88    	movk	x8, #0x6174, lsl #32
10078c364: f2ee8e48    	movk	x8, #0x7472, lsl #48
10078c368: fa480160    	ccmp	x11, x8, #0x0, eq
10078c36c: d2850408    	mov	x8, #0x2820             ; =10272
10078c370: f2adee88    	movk	x8, #0x6f74, lsl #16
10078c374: f2c58e08    	movk	x8, #0x2c70, lsl #32
10078c378: f2e88408    	movk	x8, #0x4420, lsl #48
10078c37c: fa480180    	ccmp	x12, x8, #0x0, eq
10078c380: d28409e8    	mov	x8, #0x204f             ; =8271
10078c384: f2a9e9c8    	movk	x8, #0x4f4e, lsl #16
10078c388: f2c40a88    	movk	x8, #0x2054, lsl #32
10078c38c: f2e888a8    	movk	x8, #0x4445, lsl #48
10078c390: fa4801a0    	ccmp	x13, x8, #0x0, eq
10078c394: d28a8928    	mov	x8, #0x5449             ; =21577
10078c398: f2a9a408    	movk	x8, #0x4d20, lsl #16
10078c39c: f2c9c828    	movk	x8, #0x4e41, lsl #32
10078c3a0: f2e82aa8    	movk	x8, #0x4155, lsl #48
10078c3a4: fa4801c0    	ccmp	x14, x8, #0x0, eq
10078c3a8: 52898988    	mov	w8, #0x4c4c             ; =19532
10078c3ac: 72a52b28    	movk	w8, #0x2959, lsl #16
10078c3b0: 7a4801e0    	ccmp	w15, w8, #0x0, eq
10078c3b4: aa1b03e8    	mov	x8, x27
10078c3b8: 54000261    	b.ne	0x10078c404 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x3e8>
10078c3bc: 17ffff3d    	b	0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c3c0: a9402728    	ldp	x8, x9, [x25]
10078c3c4: f9400b2a    	ldr	x10, [x25, #0x10]
10078c3c8: f841632b    	ldur	x11, [x25, #0x16]
10078c3cc: d284046c    	mov	x12, #0x2023            ; =8227
10078c3d0: f2a7878c    	movk	x12, #0x3c3c, lsl #16
10078c3d4: f2c4078c    	movk	x12, #0x203c, lsl #32
10078c3d8: f2ed2c2c    	movk	x12, #0x6961, lsl #48
10078c3dc: eb0c011f    	cmp	x8, x12
10078c3e0: fa5c0120    	ccmp	x9, x28, #0x0, eq
10078c3e4: fa5a0140    	ccmp	x10, x26, #0x0, eq
10078c3e8: d28cec28    	mov	x8, #0x6761             ; =26465
10078c3ec: f2ac8ca8    	movk	x8, #0x6465, lsl #16
10078c3f0: f2cca408    	movk	x8, #0x6520, lsl #32
10078c3f4: f2ec8dc8    	movk	x8, #0x646e, lsl #48
10078c3f8: fa480160    	ccmp	x11, x8, #0x0, eq
10078c3fc: aa1b03e8    	mov	x8, x27

/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078c01c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E>:
10078c400: 54ffe580    	b.eq	0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c404: 39400328    	ldrb	w8, [x25]
10078c408: 71016d1f    	cmp	w8, #0x5b
10078c40c: 54000181    	b.ne	0x10078c43c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x420>
10078c410: 8b180328    	add	x8, x25, x24
10078c414: 385ff108    	ldurb	w8, [x8, #-0x1]
10078c418: 7101751f    	cmp	w8, #0x5d
10078c41c: 54000101    	b.ne	0x10078c43c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x420>
10078c420: f1000b1f    	cmp	x24, #0x2
10078c424: 54000843    	b.lo	0x10078c52c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x510>
10078c428: 39c00729    	ldrsb	w9, [x25, #0x1]
10078c42c: 52800028    	mov	w8, #0x1                ; =1
10078c430: 3101053f    	cmn	w9, #0x41
10078c434: 54ffe3ec    	b.gt	0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c438: 1400003d    	b	0x10078c52c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x510>
10078c43c: 52800028    	mov	w8, #0x1                ; =1
10078c440: 3707e39b    	tbnz	w27, #0x0, 0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c444: d101c3a0    	sub	x0, x29, #0x70
10078c448: aa1903e1    	mov	x1, x25
10078c44c: aa1803e2    	mov	x2, x24
10078c450: 97fffa84    	bl	0x10078ae60 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E>
10078c454: f85903a1    	ldur	x1, [x29, #-0x70]
10078c458: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078c45c: eb08003f    	cmp	x1, x8
10078c460: 54ffe260    	b.eq	0x10078c0ac <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x90>
10078c464: b4000081    	cbz	x1, 0x10078c474 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x458>
10078c468: f85983a0    	ldur	x0, [x29, #-0x68]
10078c46c: 52800022    	mov	w2, #0x1                ; =1
10078c470: 97e1d43e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078c474: b7f803b6    	tbnz	x22, #0x3f, 0x10078c4e8 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x4cc>
10078c478: b4000116    	cbz	x22, 0x10078c498 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x47c>
10078c47c: 97e1d43e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10078c480: aa1603e0    	mov	x0, x22
10078c484: 52800021    	mov	w1, #0x1                ; =1
10078c488: 97e1d437    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10078c48c: b4000600    	cbz	x0, 0x10078c54c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x530>
10078c490: aa0003f5    	mov	x21, x0
10078c494: 14000002    	b	0x10078c49c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x480>
10078c498: 52800035    	mov	w21, #0x1               ; =1
10078c49c: aa1503e0    	mov	x0, x21
10078c4a0: aa1703e1    	mov	x1, x23
10078c4a4: aa1603e2    	mov	x2, x22
10078c4a8: 94291481    	bl	0x1011d16ac <_writev+0x1011d16ac>
10078c4ac: f9400bf7    	ldr	x23, [sp, #0x10]
10078c4b0: f94003e8    	ldr	x8, [sp]
10078c4b4: eb0802ff    	cmp	x23, x8
10078c4b8: 54000061    	b.ne	0x10078c4c4 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x4a8>
10078c4bc: 910003e0    	mov	x0, sp
10078c4c0: 94290e50    	bl	0x1011cfe00 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hca26f846e3ace0fdE>
10078c4c4: 52800008    	mov	w8, #0x0                ; =0
10078c4c8: f94007e9    	ldr	x9, [sp, #0x8]
10078c4cc: 5280030a    	mov	w10, #0x18              ; =24
10078c4d0: 9b0a26e9    	madd	x9, x23, x10, x9
10078c4d4: a9005536    	stp	x22, x21, [x9]
10078c4d8: f9000936    	str	x22, [x9, #0x10]
10078c4dc: 910006e9    	add	x9, x23, #0x1
10078c4e0: f9000be9    	str	x9, [sp, #0x10]
10078c4e4: 17fffef3    	b	0x10078c0b0 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x94>
10078c4e8: d2800014    	mov	x20, #0x0               ; =0
10078c4ec: aa1403e0    	mov	x0, x20
10078c4f0: aa1503e1    	mov	x1, x21
10078c4f4: 9429105c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10078c4f8: 14000014    	b	0x10078c548 <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E+0x52c>
10078c4fc: 3dc003e0    	ldr	q0, [sp]
10078c500: 3d800260    	str	q0, [x19]
10078c504: f9400be8    	ldr	x8, [sp, #0x10]
10078c508: f9000a68    	str	x8, [x19, #0x10]
10078c50c: a94f7bfd    	ldp	x29, x30, [sp, #0xf0]
10078c510: a94e4ff4    	ldp	x20, x19, [sp, #0xe0]
10078c514: a94d57f6    	ldp	x22, x21, [sp, #0xd0]
10078c518: a94c5ff8    	ldp	x24, x23, [sp, #0xc0]
10078c51c: a94b67fa    	ldp	x26, x25, [sp, #0xb0]
10078c520: a94a6ffc    	ldp	x28, x27, [sp, #0xa0]
10078c524: 910403ff    	add	sp, sp, #0x100
10078c528: d65f03c0    	ret
10078c52c: d1000703    	sub	x3, x24, #0x1
10078c530: d00084e4    	adrp	x4, 0x10182a000 <_anon.cdb272158a04aae91ce02c6e47f106c7.1+0x6d0>
10078c534: 9133a084    	add	x4, x4, #0xce8
10078c538: aa1903e0    	mov	x0, x25
10078c53c: aa1803e1    	mov	x1, x24
10078c540: 52800022    	mov	w2, #0x1                ; =1
10078c544: 942910b5    	bl	0x1011d0818 <__ZN4core3str16slice_error_fail17hecde84adb67345c5E>
10078c548: d4200020    	brk	#0x1
10078c54c: aa1603f5    	mov	x21, x22
