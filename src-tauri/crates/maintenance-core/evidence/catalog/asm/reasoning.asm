
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010036d090 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E>:
10036d090: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
10036d094: a90167fa    	stp	x26, x25, [sp, #0x10]
10036d098: a9025ff8    	stp	x24, x23, [sp, #0x20]
10036d09c: a90357f6    	stp	x22, x21, [sp, #0x30]
10036d0a0: a9044ff4    	stp	x20, x19, [sp, #0x40]
10036d0a4: a9057bfd    	stp	x29, x30, [sp, #0x50]
10036d0a8: 910143fd    	add	x29, sp, #0x50
10036d0ac: d109c3ff    	sub	sp, sp, #0x270
10036d0b0: aa0403f4    	mov	x20, x4
10036d0b4: aa0303f7    	mov	x23, x3
10036d0b8: aa0203f5    	mov	x21, x2
10036d0bc: aa0103f6    	mov	x22, x1
10036d0c0: aa0003f3    	mov	x19, x0
10036d0c4: d10343b8    	sub	x24, x29, #0xd0
10036d0c8: 91028bf9    	add	x25, sp, #0xa2
10036d0cc: 3941605a    	ldrb	w26, [x2, #0x58]
10036d0d0: 7100075f    	cmp	w26, #0x1
10036d0d4: 54000c81    	b.ne	0x10036d264 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1d4>
10036d0d8: a9418aa1    	ldp	x1, x2, [x21, #0x18]
10036d0dc: d10343a0    	sub	x0, x29, #0xd0
10036d0e0: 97ffe8bc    	bl	0x1003673d0 <__ZN13codexmate_lib4core5relay13codex_catalog16model_tail_lower17hdcbb168b8679db13E>
10036d0e4: f85403a8    	ldur	x8, [x29, #-0xc0]
10036d0e8: f100451f    	cmp	x8, #0x11
10036d0ec: 54000261    	b.ne	0x10036d138 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xa8>
10036d0f0: a97323a1    	ldp	x1, x8, [x29, #-0xd0]
10036d0f4: a9402909    	ldp	x9, x10, [x8]
10036d0f8: 39404108    	ldrb	w8, [x8, #0x10]
10036d0fc: d28cac8b    	mov	x11, #0x6564            ; =25956
10036d100: f2ae0cab    	movk	x11, #0x7065, lsl #16
10036d104: f2ccae6b    	movk	x11, #0x6573, lsl #32
10036d108: f2ed6cab    	movk	x11, #0x6b65, lsl #48
10036d10c: eb0b013f    	cmp	x9, x11
10036d110: d28ec5a9    	mov	x9, #0x762d             ; =30253
10036d114: f2a5a689    	movk	x9, #0x2d34, lsl #16
10036d118: f2cd8cc9    	movk	x9, #0x6c66, lsl #32
10036d11c: f2ee6c29    	movk	x9, #0x7361, lsl #48
10036d120: fa490140    	ccmp	x10, x9, #0x0, eq
10036d124: 52800d09    	mov	w9, #0x68               ; =104
10036d128: 7a490100    	ccmp	w8, w9, #0x0, eq
10036d12c: 1a9f17fb    	cset	w27, eq
10036d130: b50000a1    	cbnz	x1, 0x10036d144 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xb4>
10036d134: 14000007    	b	0x10036d150 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xc0>
10036d138: 5280001b    	mov	w27, #0x0               ; =0
10036d13c: f85303a1    	ldur	x1, [x29, #-0xd0]
10036d140: b4000081    	cbz	x1, 0x10036d150 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xc0>
10036d144: f85383a0    	ldur	x0, [x29, #-0xc8]
10036d148: 52800022    	mov	w2, #0x1                ; =1
10036d14c: 97f25107    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036d150: 360008bb    	tbz	w27, #0x0, 0x10036d264 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1d4>
10036d154: 97f25108    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d158: 52800080    	mov	w0, #0x4                ; =4
10036d15c: 52800021    	mov	w1, #0x1                ; =1
10036d160: 97f25101    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d164: b40100e0    	cbz	x0, 0x10036f180 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x20f0>
10036d168: aa0003f4    	mov	x20, x0
10036d16c: 528d2d1b    	mov	w27, #0x6968            ; =26984
10036d170: 72ad0cfb    	movk	w27, #0x6867, lsl #16
10036d174: b900001b    	str	w27, [x0]
10036d178: 97f250ff    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d17c: 52800c00    	mov	w0, #0x60               ; =96
10036d180: 52800101    	mov	w1, #0x8                ; =8
10036d184: 97f250f8    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d188: b400f6e0    	cbz	x0, 0x10036f064 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1fd4>
10036d18c: aa0003f5    	mov	x21, x0
10036d190: f90073ff    	str	xzr, [sp, #0xe0]
10036d194: f9007bff    	str	xzr, [sp, #0xf0]
10036d198: 97f250f7    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d19c: 528000d7    	mov	w23, #0x6               ; =6
10036d1a0: 528000c0    	mov	w0, #0x6                ; =6
10036d1a4: 52800021    	mov	w1, #0x1                ; =1
10036d1a8: 97f250ef    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d1ac: b4010000    	cbz	x0, 0x10036f1ac <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x211c>
10036d1b0: aa0003f6    	mov	x22, x0
10036d1b4: 528e8e48    	mov	w8, #0x7472             ; =29810
10036d1b8: 79000808    	strh	w8, [x0, #0x4]
10036d1bc: 528ccca8    	mov	w8, #0x6665             ; =26213
10036d1c0: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036d1c4: b9000008    	str	w8, [x0]
10036d1c8: 528000c8    	mov	w8, #0x6                ; =6
10036d1cc: a91203e8    	stp	x8, x0, [sp, #0x120]
10036d1d0: f9009be8    	str	x8, [sp, #0x130]
10036d1d4: 97f250e8    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d1d8: 52800077    	mov	w23, #0x3               ; =3
10036d1dc: 52800060    	mov	w0, #0x3                ; =3
10036d1e0: 52800021    	mov	w1, #0x1                ; =1
10036d1e4: 97f250e0    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d1e8: b40101a0    	cbz	x0, 0x10036f21c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x218c>
10036d1ec: 52800ee8    	mov	w8, #0x77               ; =119
10036d1f0: 39000808    	strb	w8, [x0, #0x2]
10036d1f4: 528ded88    	mov	w8, #0x6f6c             ; =28524
10036d1f8: 79000008    	strh	w8, [x0]
10036d1fc: 390643f7    	strb	w23, [sp, #0x190]
10036d200: a91983f7    	stp	x23, x0, [sp, #0x198]
10036d204: f900d7f7    	str	x23, [sp, #0x1a8]
10036d208: 9106c3e8    	add	x8, sp, #0x1b0
10036d20c: 910383e0    	add	x0, sp, #0xe0
10036d210: 910483e1    	add	x1, sp, #0x120
10036d214: 940fa589    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036d218: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036d21c: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036d220: eb09011f    	cmp	x8, x9
10036d224: 540045a1    	b.ne	0x10036dad8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xa48>
10036d228: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036d22c: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036d230: 8b091508    	add	x8, x8, x9, lsl #5
10036d234: 3dc00100    	ldr	q0, [x8]
10036d238: 3c9903a0    	stur	q0, [x29, #-0x70]
10036d23c: 3dc00500    	ldr	q0, [x8, #0x10]
10036d240: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036d244: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036d248: ad000101    	stp	q1, q0, [x8]
10036d24c: 385903a8    	ldurb	w8, [x29, #-0x70]
10036d250: 7100191f    	cmp	w8, #0x6
10036d254: 540045a0    	b.eq	0x10036db08 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xa78>
10036d258: d101c3a0    	sub	x0, x29, #0x70
10036d25c: 9401a8d9    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036d260: 1400022a    	b	0x10036db08 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xa78>
10036d264: 360008b7    	tbz	w23, #0x0, 0x10036d378 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2e8>
10036d268: 97f250c3    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d26c: 52800080    	mov	w0, #0x4                ; =4
10036d270: 52800021    	mov	w1, #0x1                ; =1
10036d274: 97f250bc    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d278: b400f840    	cbz	x0, 0x10036f180 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x20f0>
10036d27c: aa0003f4    	mov	x20, x0
10036d280: 528d2d1a    	mov	w26, #0x6968            ; =26984
10036d284: 72ad0cfa    	movk	w26, #0x6867, lsl #16
10036d288: b900001a    	str	w26, [x0]
10036d28c: 97f250ba    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d290: 52800800    	mov	w0, #0x40               ; =64
10036d294: 52800101    	mov	w1, #0x8                ; =8
10036d298: 97f250b3    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d29c: b400edc0    	cbz	x0, 0x10036f054 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1fc4>
10036d2a0: aa0003f5    	mov	x21, x0
10036d2a4: f90083ff    	str	xzr, [sp, #0x100]
10036d2a8: f9008bff    	str	xzr, [sp, #0x110]
10036d2ac: 97f250b2    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d2b0: 528000d7    	mov	w23, #0x6               ; =6
10036d2b4: 528000c0    	mov	w0, #0x6                ; =6
10036d2b8: 52800021    	mov	w1, #0x1                ; =1
10036d2bc: 97f250aa    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d2c0: b400f660    	cbz	x0, 0x10036f18c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x20fc>
10036d2c4: aa0003f6    	mov	x22, x0
10036d2c8: 528e8e48    	mov	w8, #0x7472             ; =29810
10036d2cc: 79000808    	strh	w8, [x0, #0x4]
10036d2d0: 528ccca8    	mov	w8, #0x6665             ; =26213
10036d2d4: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036d2d8: b9000008    	str	w8, [x0]
10036d2dc: 528000c8    	mov	w8, #0x6                ; =6
10036d2e0: a91203e8    	stp	x8, x0, [sp, #0x120]
10036d2e4: f9009be8    	str	x8, [sp, #0x130]
10036d2e8: 97f250a3    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d2ec: 52800080    	mov	w0, #0x4                ; =4
10036d2f0: 52800021    	mov	w1, #0x1                ; =1
10036d2f4: 97f2509c    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d2f8: b400f720    	cbz	x0, 0x10036f1dc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x214c>
10036d2fc: 528dedc8    	mov	w8, #0x6f6e             ; =28526
10036d300: 72acadc8    	movk	w8, #0x656e, lsl #16
10036d304: b9000008    	str	w8, [x0]
10036d308: 52800068    	mov	w8, #0x3                ; =3
10036d30c: 390643e8    	strb	w8, [sp, #0x190]
10036d310: 52800088    	mov	w8, #0x4                ; =4
10036d314: a91983e8    	stp	x8, x0, [sp, #0x198]
10036d318: f900d7e8    	str	x8, [sp, #0x1a8]
10036d31c: 9106c3e8    	add	x8, sp, #0x1b0
10036d320: 910403e0    	add	x0, sp, #0x100
10036d324: 910483e1    	add	x1, sp, #0x120
10036d328: 940fa544    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036d32c: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036d330: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036d334: eb09011f    	cmp	x8, x9
10036d338: 540011c1    	b.ne	0x10036d570 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x4e0>
10036d33c: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036d340: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036d344: 8b091508    	add	x8, x8, x9, lsl #5
10036d348: 3dc00100    	ldr	q0, [x8]
10036d34c: 3c9903a0    	stur	q0, [x29, #-0x70]
10036d350: 3dc00500    	ldr	q0, [x8, #0x10]
10036d354: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036d358: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036d35c: ad000101    	stp	q1, q0, [x8]
10036d360: 385903a8    	ldurb	w8, [x29, #-0x70]
10036d364: 7100191f    	cmp	w8, #0x6
10036d368: 540011c0    	b.eq	0x10036d5a0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x510>
10036d36c: d101c3a0    	sub	x0, x29, #0x70
10036d370: 9401a894    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036d374: 1400008b    	b	0x10036d5a0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x510>
10036d378: 34002cfa    	cbz	w26, 0x10036d914 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x884>
10036d37c: 7100075f    	cmp	w26, #0x1
10036d380: 54003641    	b.ne	0x10036da48 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x9b8>
10036d384: b40006b4    	cbz	x20, 0x10036d458 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x3c8>
10036d388: f0007ce0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036d38c: 910c8400    	add	x0, x0, #0x321
10036d390: 528002e1    	mov	w1, #0x17               ; =23
10036d394: aa1403e2    	mov	x2, x20
10036d398: 943685fe    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
10036d39c: b40005e0    	cbz	x0, 0x10036d458 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x3c8>
10036d3a0: 39400008    	ldrb	w8, [x0]
10036d3a4: 71000d1f    	cmp	w8, #0x3
10036d3a8: 54000581    	b.ne	0x10036d458 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x3c8>
10036d3ac: a9410408    	ldp	x8, x1, [x0, #0x10]
10036d3b0: aa0803e0    	mov	x0, x8
10036d3b4: 97fb0769    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
10036d3b8: aa0003f6    	mov	x22, x0
10036d3bc: aa0103f5    	mov	x21, x1
10036d3c0: f0007ce0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036d3c4: 910ce000    	add	x0, x0, #0x338
10036d3c8: 52800341    	mov	w1, #0x1a               ; =26
10036d3cc: aa1403e2    	mov	x2, x20
10036d3d0: 943685f0    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
10036d3d4: b4000420    	cbz	x0, 0x10036d458 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x3c8>
10036d3d8: 39400008    	ldrb	w8, [x0]
10036d3dc: 7100111f    	cmp	w8, #0x4
10036d3e0: 540003c1    	b.ne	0x10036d458 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x3c8>
10036d3e4: b40003b5    	cbz	x21, 0x10036d458 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x3c8>
10036d3e8: f9400c08    	ldr	x8, [x0, #0x18]
10036d3ec: b4000368    	cbz	x8, 0x10036d458 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x3c8>
10036d3f0: aa0003fa    	mov	x26, x0
10036d3f4: d10343a8    	sub	x8, x29, #0xd0
10036d3f8: aa1503e0    	mov	x0, x21
10036d3fc: 52800001    	mov	w1, #0x0                ; =0
10036d400: 52800022    	mov	w2, #0x1                ; =1
10036d404: 52800023    	mov	w3, #0x1                ; =1
10036d408: 940a905c    	bl	0x100611578 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17hac6eac5846bb4dbdE>
10036d40c: b85303a8    	ldur	w8, [x29, #-0xd0]
10036d410: f85383b4    	ldur	x20, [x29, #-0xc8]
10036d414: 7100051f    	cmp	w8, #0x1
10036d418: 5400eae0    	b.eq	0x10036f174 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x20e4>
10036d41c: f85403b7    	ldur	x23, [x29, #-0xc0]
10036d420: aa1703e0    	mov	x0, x23
10036d424: aa1603e1    	mov	x1, x22
10036d428: aa1503e2    	mov	x2, x21
10036d42c: 943990a0    	bl	0x1011d16ac <_writev+0x1011d16ac>
10036d430: d10343a8    	sub	x8, x29, #0xd0
10036d434: 91002340    	add	x0, x26, #0x8
10036d438: 9407ca13    	bl	0x10055fc84 <__ZN67_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..clone..Clone$GT$5clone17h0c19ebe553bd5809E>
10036d43c: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036d440: 3c826320    	stur	q0, [x25, #0x26]
10036d444: f85403a8    	ldur	x8, [x29, #-0xc0]
10036d448: f8036328    	stur	x8, [x25, #0x36]
10036d44c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10036d450: eb08029f    	cmp	x20, x8
10036d454: 5400dee1    	b.ne	0x10036f030 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1fa0>
10036d458: 97f25047    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d45c: 528000c0    	mov	w0, #0x6                ; =6
10036d460: 52800021    	mov	w1, #0x1                ; =1
10036d464: 97f25040    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d468: b400f2a0    	cbz	x0, 0x10036f2bc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x222c>
10036d46c: aa0003f4    	mov	x20, x0
10036d470: 528daea8    	mov	w8, #0x6d75             ; =28021
10036d474: 79000808    	strh	w8, [x0, #0x4]
10036d478: 528cada8    	mov	w8, #0x656d             ; =25965
10036d47c: 72ad2c88    	movk	w8, #0x6964, lsl #16
10036d480: b9000008    	str	w8, [x0]
10036d484: 97f2503c    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d488: 52801000    	mov	w0, #0x80               ; =128
10036d48c: 52800101    	mov	w1, #0x8                ; =8
10036d490: 97f25035    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d494: b400df00    	cbz	x0, 0x10036f074 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1fe4>
10036d498: aa0003f5    	mov	x21, x0
10036d49c: f90083ff    	str	xzr, [sp, #0x100]
10036d4a0: f9008bff    	str	xzr, [sp, #0x110]
10036d4a4: 97f25034    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d4a8: 528000d7    	mov	w23, #0x6               ; =6
10036d4ac: 528000c0    	mov	w0, #0x6                ; =6
10036d4b0: 52800021    	mov	w1, #0x1                ; =1
10036d4b4: 97f2502c    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d4b8: b400ee20    	cbz	x0, 0x10036f27c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x21ec>
10036d4bc: aa0003f6    	mov	x22, x0
10036d4c0: 528e8e48    	mov	w8, #0x7472             ; =29810
10036d4c4: 79000808    	strh	w8, [x0, #0x4]
10036d4c8: 528ccca8    	mov	w8, #0x6665             ; =26213
10036d4cc: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036d4d0: b9000008    	str	w8, [x0]
10036d4d4: 528000c8    	mov	w8, #0x6                ; =6
10036d4d8: a91203e8    	stp	x8, x0, [sp, #0x120]
10036d4dc: f9009be8    	str	x8, [sp, #0x130]
10036d4e0: 97f25025    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d4e4: 52800077    	mov	w23, #0x3               ; =3
10036d4e8: 52800060    	mov	w0, #0x3                ; =3
10036d4ec: 52800021    	mov	w1, #0x1                ; =1
10036d4f0: 97f2501d    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d4f4: b400ef20    	cbz	x0, 0x10036f2d8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2248>
10036d4f8: 52800ee8    	mov	w8, #0x77               ; =119
10036d4fc: 39000808    	strb	w8, [x0, #0x2]
10036d500: 528ded88    	mov	w8, #0x6f6c             ; =28524
10036d504: 79000008    	strh	w8, [x0]
10036d508: 390643f7    	strb	w23, [sp, #0x190]
10036d50c: a91983f7    	stp	x23, x0, [sp, #0x198]
10036d510: f900d7f7    	str	x23, [sp, #0x1a8]
10036d514: 9106c3e8    	add	x8, sp, #0x1b0
10036d518: 910403e0    	add	x0, sp, #0x100
10036d51c: 910483e1    	add	x1, sp, #0x120
10036d520: 940fa4c6    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036d524: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036d528: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036d52c: eb09011f    	cmp	x8, x9
10036d530: 54005b81    	b.ne	0x10036e0a0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1010>
10036d534: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036d538: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036d53c: 8b091508    	add	x8, x8, x9, lsl #5
10036d540: 3dc00100    	ldr	q0, [x8]
10036d544: 3c9903a0    	stur	q0, [x29, #-0x70]
10036d548: 3dc00500    	ldr	q0, [x8, #0x10]
10036d54c: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036d550: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036d554: ad000101    	stp	q1, q0, [x8]
10036d558: 385903a8    	ldurb	w8, [x29, #-0x70]
10036d55c: 7100191f    	cmp	w8, #0x6
10036d560: 54005b80    	b.eq	0x10036e0d0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1040>
10036d564: d101c3a0    	sub	x0, x29, #0x70
10036d568: 9401a816    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036d56c: 140002d9    	b	0x10036e0d0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1040>
10036d570: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036d574: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036d578: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036d57c: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036d580: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036d584: f81603a8    	stur	x8, [x29, #-0xa0]
10036d588: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036d58c: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036d590: d101c3a8    	sub	x8, x29, #0x70
10036d594: d10343a0    	sub	x0, x29, #0xd0
10036d598: d10243a1    	sub	x1, x29, #0x90
10036d59c: 940a8259    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036d5a0: 97f24ff5    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d5a4: 52800177    	mov	w23, #0xb               ; =11
10036d5a8: 52800160    	mov	w0, #0xb                ; =11
10036d5ac: 52800021    	mov	w1, #0x1                ; =1
10036d5b0: 97f24fed    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d5b4: b400dec0    	cbz	x0, 0x10036f18c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x20fc>
10036d5b8: aa0003f6    	mov	x22, x0
10036d5bc: 528d2e88    	mov	w8, #0x6974             ; =26996
10036d5c0: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036d5c4: b8007008    	stur	w8, [x0, #0x7]
10036d5c8: f0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036d5cc: 91092908    	add	x8, x8, #0x24a
10036d5d0: f940011b    	ldr	x27, [x8]
10036d5d4: f900001b    	str	x27, [x0]
10036d5d8: 52800168    	mov	w8, #0xb                ; =11
10036d5dc: a91203e8    	stp	x8, x0, [sp, #0x120]
10036d5e0: f9009be8    	str	x8, [sp, #0x130]
10036d5e4: 97f24fe4    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d5e8: 52800180    	mov	w0, #0xc                ; =12
10036d5ec: 52800021    	mov	w1, #0x1                ; =1
10036d5f0: 97f24fdd    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d5f4: b400dfc0    	cbz	x0, 0x10036f1ec <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x215c>
10036d5f8: 528d2dc8    	mov	w8, #0x696e             ; =26990
10036d5fc: 72acedc8    	movk	w8, #0x676e, lsl #16
10036d600: b9000808    	str	w8, [x0, #0x8]
10036d604: f0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036d608: 9118c508    	add	x8, x8, #0x631
10036d60c: f9400108    	ldr	x8, [x8]
10036d610: f9000008    	str	x8, [x0]
10036d614: 52800068    	mov	w8, #0x3                ; =3
10036d618: 390643e8    	strb	w8, [sp, #0x190]
10036d61c: 52800188    	mov	w8, #0xc                ; =12
10036d620: a91983e8    	stp	x8, x0, [sp, #0x198]
10036d624: f900d7e8    	str	x8, [sp, #0x1a8]
10036d628: 9106c3e8    	add	x8, sp, #0x1b0
10036d62c: 910403e0    	add	x0, sp, #0x100
10036d630: 910483e1    	add	x1, sp, #0x120
10036d634: 940fa481    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036d638: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036d63c: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036d640: eb09011f    	cmp	x8, x9
10036d644: 54000201    	b.ne	0x10036d684 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x5f4>
10036d648: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036d64c: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036d650: 8b091508    	add	x8, x8, x9, lsl #5
10036d654: 3dc00100    	ldr	q0, [x8]
10036d658: 3c9903a0    	stur	q0, [x29, #-0x70]
10036d65c: 3dc00500    	ldr	q0, [x8, #0x10]
10036d660: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036d664: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036d668: ad000101    	stp	q1, q0, [x8]
10036d66c: 385903a8    	ldurb	w8, [x29, #-0x70]
10036d670: 7100191f    	cmp	w8, #0x6
10036d674: 54000200    	b.eq	0x10036d6b4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x624>
10036d678: d101c3a0    	sub	x0, x29, #0x70
10036d67c: 9401a7d1    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036d680: 1400000d    	b	0x10036d6b4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x624>
10036d684: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036d688: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036d68c: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036d690: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036d694: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036d698: f81603a8    	stur	x8, [x29, #-0xa0]
10036d69c: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036d6a0: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036d6a4: d101c3a8    	sub	x8, x29, #0x70
10036d6a8: d10343a0    	sub	x0, x29, #0xd0
10036d6ac: d10243a1    	sub	x1, x29, #0x90
10036d6b0: 940a8214    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036d6b4: 3dc043e0    	ldr	q0, [sp, #0x100]
10036d6b8: 3c886320    	stur	q0, [x25, #0x86]
10036d6bc: f9408be8    	ldr	x8, [sp, #0x110]
10036d6c0: f9009fe8    	str	x8, [sp, #0x138]
10036d6c4: 528000a8    	mov	w8, #0x5                ; =5
10036d6c8: 390483e8    	strb	w8, [sp, #0x120]
10036d6cc: f90073ff    	str	xzr, [sp, #0xe0]
10036d6d0: f9007bff    	str	xzr, [sp, #0xf0]
10036d6d4: 97f24fa8    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d6d8: 528000d7    	mov	w23, #0x6               ; =6
10036d6dc: 528000c0    	mov	w0, #0x6                ; =6
10036d6e0: 52800021    	mov	w1, #0x1                ; =1
10036d6e4: 97f24fa0    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d6e8: b400d5a0    	cbz	x0, 0x10036f19c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x210c>
10036d6ec: aa0003f6    	mov	x22, x0
10036d6f0: 528e8e48    	mov	w8, #0x7472             ; =29810
10036d6f4: 79000808    	strh	w8, [x0, #0x4]
10036d6f8: 528ccca8    	mov	w8, #0x6665             ; =26213
10036d6fc: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036d700: b9000008    	str	w8, [x0]
10036d704: 528000c8    	mov	w8, #0x6                ; =6
10036d708: a91003e8    	stp	x8, x0, [sp, #0x100]
10036d70c: f9008be8    	str	x8, [sp, #0x110]
10036d710: 97f24f99    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d714: 52800080    	mov	w0, #0x4                ; =4
10036d718: 52800021    	mov	w1, #0x1                ; =1
10036d71c: 97f24f92    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d720: b400d6e0    	cbz	x0, 0x10036f1fc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x216c>
10036d724: b900001a    	str	w26, [x0]
10036d728: 52800068    	mov	w8, #0x3                ; =3
10036d72c: 390643e8    	strb	w8, [sp, #0x190]
10036d730: 52800088    	mov	w8, #0x4                ; =4
10036d734: a91983e8    	stp	x8, x0, [sp, #0x198]
10036d738: f900d7e8    	str	x8, [sp, #0x1a8]
10036d73c: 9106c3e8    	add	x8, sp, #0x1b0
10036d740: 910383e0    	add	x0, sp, #0xe0
10036d744: 910403e1    	add	x1, sp, #0x100
10036d748: 940fa43c    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036d74c: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036d750: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036d754: eb09011f    	cmp	x8, x9
10036d758: 54000201    	b.ne	0x10036d798 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x708>
10036d75c: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036d760: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036d764: 8b091508    	add	x8, x8, x9, lsl #5
10036d768: 3dc00100    	ldr	q0, [x8]
10036d76c: 3c9903a0    	stur	q0, [x29, #-0x70]
10036d770: 3dc00500    	ldr	q0, [x8, #0x10]
10036d774: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036d778: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036d77c: ad000101    	stp	q1, q0, [x8]
10036d780: 385903a8    	ldurb	w8, [x29, #-0x70]
10036d784: 7100191f    	cmp	w8, #0x6
10036d788: 54000200    	b.eq	0x10036d7c8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x738>
10036d78c: d101c3a0    	sub	x0, x29, #0x70
10036d790: 9401a78c    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036d794: 1400000d    	b	0x10036d7c8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x738>
10036d798: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036d79c: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036d7a0: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036d7a4: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036d7a8: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036d7ac: f81603a8    	stur	x8, [x29, #-0xa0]
10036d7b0: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036d7b4: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036d7b8: d101c3a8    	sub	x8, x29, #0x70
10036d7bc: d10343a0    	sub	x0, x29, #0xd0
10036d7c0: d10243a1    	sub	x1, x29, #0x90
10036d7c4: 940a81cf    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036d7c8: 97f24f6b    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d7cc: 52800177    	mov	w23, #0xb               ; =11
10036d7d0: 52800160    	mov	w0, #0xb                ; =11
10036d7d4: 52800021    	mov	w1, #0x1                ; =1
10036d7d8: 97f24f63    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d7dc: b400ce00    	cbz	x0, 0x10036f19c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x210c>
10036d7e0: aa0003f6    	mov	x22, x0
10036d7e4: 528d2e88    	mov	w8, #0x6974             ; =26996
10036d7e8: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036d7ec: b8007008    	stur	w8, [x0, #0x7]
10036d7f0: f900001b    	str	x27, [x0]
10036d7f4: 52800168    	mov	w8, #0xb                ; =11
10036d7f8: a91003e8    	stp	x8, x0, [sp, #0x100]
10036d7fc: f9008be8    	str	x8, [sp, #0x110]
10036d800: 97f24f5d    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d804: 528002a0    	mov	w0, #0x15               ; =21
10036d808: 52800021    	mov	w1, #0x1                ; =1
10036d80c: 97f24f56    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d810: b400cfe0    	cbz	x0, 0x10036f20c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x217c>
10036d814: f0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036d818: 9118f508    	add	x8, x8, #0x63d
10036d81c: 3dc00100    	ldr	q0, [x8]
10036d820: 3d800000    	str	q0, [x0]
10036d824: f840d108    	ldur	x8, [x8, #0xd]
10036d828: f800d008    	stur	x8, [x0, #0xd]
10036d82c: 52800068    	mov	w8, #0x3                ; =3
10036d830: 390643e8    	strb	w8, [sp, #0x190]
10036d834: 528002a8    	mov	w8, #0x15               ; =21
10036d838: a91983e8    	stp	x8, x0, [sp, #0x198]
10036d83c: f900d7e8    	str	x8, [sp, #0x1a8]
10036d840: 9106c3e8    	add	x8, sp, #0x1b0
10036d844: 910383e0    	add	x0, sp, #0xe0
10036d848: 910403e1    	add	x1, sp, #0x100
10036d84c: 940fa3fb    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036d850: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036d854: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036d858: eb09011f    	cmp	x8, x9
10036d85c: 54000201    	b.ne	0x10036d89c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x80c>
10036d860: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036d864: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036d868: 8b091508    	add	x8, x8, x9, lsl #5
10036d86c: 3dc00100    	ldr	q0, [x8]
10036d870: 3c9903a0    	stur	q0, [x29, #-0x70]
10036d874: 3dc00500    	ldr	q0, [x8, #0x10]
10036d878: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036d87c: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036d880: ad000101    	stp	q1, q0, [x8]
10036d884: 385903a8    	ldurb	w8, [x29, #-0x70]
10036d888: 7100191f    	cmp	w8, #0x6
10036d88c: 54000200    	b.eq	0x10036d8cc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x83c>
10036d890: d101c3a0    	sub	x0, x29, #0x70
10036d894: 9401a74b    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036d898: 1400000d    	b	0x10036d8cc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x83c>
10036d89c: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036d8a0: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036d8a4: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036d8a8: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036d8ac: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036d8b0: f81603a8    	stur	x8, [x29, #-0xa0]
10036d8b4: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036d8b8: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036d8bc: d101c3a8    	sub	x8, x29, #0x70
10036d8c0: d10343a0    	sub	x0, x29, #0xd0
10036d8c4: d10243a1    	sub	x1, x29, #0x90
10036d8c8: 940a818e    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036d8cc: f9407be8    	ldr	x8, [sp, #0xf0]
10036d8d0: 3dc03be0    	ldr	q0, [sp, #0xe0]
10036d8d4: 3c807300    	stur	q0, [x24, #0x7]
10036d8d8: f8017308    	stur	x8, [x24, #0x17]
10036d8dc: ad4907e0    	ldp	q0, q1, [sp, #0x120]
10036d8e0: ad0006a0    	stp	q0, q1, [x21]
10036d8e4: 528000a8    	mov	w8, #0x5                ; =5
10036d8e8: 390082a8    	strb	w8, [x21, #0x20]
10036d8ec: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036d8f0: 3c8212a0    	stur	q0, [x21, #0x21]
10036d8f4: 3cc0f300    	ldur	q0, [x24, #0xf]
10036d8f8: 3d800ea0    	str	q0, [x21, #0x30]
10036d8fc: 52800088    	mov	w8, #0x4                ; =4
10036d900: a9005268    	stp	x8, x20, [x19]
10036d904: f9000a68    	str	x8, [x19, #0x10]
10036d908: 39006268    	strb	w8, [x19, #0x18]
10036d90c: 52800048    	mov	w8, #0x2                ; =2
10036d910: 140005a3    	b	0x10036ef9c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f0c>
10036d914: 72001edf    	tst	w22, #0xff
10036d918: 54000081    	b.ne	0x10036d928 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x898>
10036d91c: a94186a0    	ldp	x0, x1, [x21, #0x18]
10036d920: 9410aea4    	bl	0x1007993b0 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E>
10036d924: 12001c16    	and	w22, w0, #0xff
10036d928: 12001ec8    	and	w8, w22, #0xff
10036d92c: 7100051f    	cmp	w8, #0x1
10036d930: 54007c0d    	b.le	0x10036e8b0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1820>
10036d934: 51000908    	sub	w8, w8, #0x2
10036d938: 7100111f    	cmp	w8, #0x4
10036d93c: 54007bc2    	b.hs	0x10036e8b4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1824>
10036d940: 97f24f0d    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d944: 52800080    	mov	w0, #0x4                ; =4
10036d948: 52800021    	mov	w1, #0x1                ; =1
10036d94c: 97f24f06    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d950: b400c180    	cbz	x0, 0x10036f180 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x20f0>
10036d954: aa0003f4    	mov	x20, x0
10036d958: 528d2d19    	mov	w25, #0x6968            ; =26984
10036d95c: 72ad0cf9    	movk	w25, #0x6867, lsl #16
10036d960: b9000019    	str	w25, [x0]
10036d964: 97f24f04    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d968: 52800400    	mov	w0, #0x20               ; =32
10036d96c: 52800101    	mov	w1, #0x8                ; =8
10036d970: 97f24efd    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d974: b400b880    	cbz	x0, 0x10036f084 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1ff4>
10036d978: aa0003f5    	mov	x21, x0
10036d97c: f90083ff    	str	xzr, [sp, #0x100]
10036d980: f9008bff    	str	xzr, [sp, #0x110]
10036d984: 97f24efc    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d988: 528000d7    	mov	w23, #0x6               ; =6
10036d98c: 528000c0    	mov	w0, #0x6                ; =6
10036d990: 52800021    	mov	w1, #0x1                ; =1
10036d994: 97f24ef4    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d998: b400c980    	cbz	x0, 0x10036f2c8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2238>
10036d99c: aa0003f6    	mov	x22, x0
10036d9a0: 528e8e48    	mov	w8, #0x7472             ; =29810
10036d9a4: 79000808    	strh	w8, [x0, #0x4]
10036d9a8: 528ccca8    	mov	w8, #0x6665             ; =26213
10036d9ac: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036d9b0: b9000008    	str	w8, [x0]
10036d9b4: 528000c8    	mov	w8, #0x6                ; =6
10036d9b8: a91203e8    	stp	x8, x0, [sp, #0x120]
10036d9bc: f9009be8    	str	x8, [sp, #0x130]
10036d9c0: 97f24eed    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036d9c4: 52800080    	mov	w0, #0x4                ; =4
10036d9c8: 52800021    	mov	w1, #0x1                ; =1
10036d9cc: 97f24ee6    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036d9d0: b400cc40    	cbz	x0, 0x10036f358 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x22c8>
10036d9d4: b9000019    	str	w25, [x0]
10036d9d8: 52800068    	mov	w8, #0x3                ; =3
10036d9dc: 390643e8    	strb	w8, [sp, #0x190]
10036d9e0: 52800088    	mov	w8, #0x4                ; =4
10036d9e4: a91983e8    	stp	x8, x0, [sp, #0x198]
10036d9e8: f900d7e8    	str	x8, [sp, #0x1a8]
10036d9ec: 9106c3e8    	add	x8, sp, #0x1b0
10036d9f0: 910403e0    	add	x0, sp, #0x100
10036d9f4: 910483e1    	add	x1, sp, #0x120
10036d9f8: 940fa390    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036d9fc: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036da00: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036da04: eb09011f    	cmp	x8, x9
10036da08: 54007e61    	b.ne	0x10036e9d4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1944>
10036da0c: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036da10: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036da14: 8b091508    	add	x8, x8, x9, lsl #5
10036da18: 3dc00100    	ldr	q0, [x8]
10036da1c: 3c9903a0    	stur	q0, [x29, #-0x70]
10036da20: 3dc00500    	ldr	q0, [x8, #0x10]
10036da24: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036da28: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036da2c: ad000101    	stp	q1, q0, [x8]
10036da30: 385903a8    	ldurb	w8, [x29, #-0x70]
10036da34: 7100191f    	cmp	w8, #0x6
10036da38: 54007e60    	b.eq	0x10036ea04 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1974>
10036da3c: d101c3a0    	sub	x0, x29, #0x70
10036da40: 9401a6e0    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036da44: 140003f0    	b	0x10036ea04 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1974>
10036da48: a941deb6    	ldp	x22, x23, [x21, #0x18]
10036da4c: d10343a0    	sub	x0, x29, #0xd0
10036da50: aa1603e1    	mov	x1, x22
10036da54: aa1703e2    	mov	x2, x23
10036da58: 97fa4a1d    	bl	0x1002002cc <__ZN13codexmate_lib4core5relay19anthropic_reasoning13model_profile17h934564ed1308bf10E>
10036da5c: 385553a8    	ldurb	w8, [x29, #-0xab]
10036da60: f85383a9    	ldur	x9, [x29, #-0xc8]
10036da64: f100013f    	cmp	x9, #0x0
10036da68: 7a421904    	ccmp	w8, #0x2, #0x4, ne
10036da6c: 54000160    	b.eq	0x10036da98 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xa08>
10036da70: f85403b5    	ldur	x21, [x29, #-0xc0]
10036da74: b4000135    	cbz	x21, 0x10036da98 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xa08>
10036da78: f85483b4    	ldur	x20, [x29, #-0xb8]
10036da7c: d10343a0    	sub	x0, x29, #0xd0
10036da80: aa1603e1    	mov	x1, x22
10036da84: aa1703e2    	mov	x2, x23
10036da88: 97fa4a11    	bl	0x1002002cc <__ZN13codexmate_lib4core5relay19anthropic_reasoning13model_profile17h934564ed1308bf10E>
10036da8c: 385553a8    	ldurb	w8, [x29, #-0xab]
10036da90: 7100091f    	cmp	w8, #0x2
10036da94: 54008601    	b.ne	0x10036eb54 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1ac4>
10036da98: 97f24eb7    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036da9c: 52800094    	mov	w20, #0x4               ; =4
10036daa0: 52800080    	mov	w0, #0x4                ; =4
10036daa4: 52800021    	mov	w1, #0x1                ; =1
10036daa8: 97f24eaf    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036daac: b400b6a0    	cbz	x0, 0x10036f180 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x20f0>
10036dab0: 528dedc8    	mov	w8, #0x6f6e             ; =28526
10036dab4: 72acadc8    	movk	w8, #0x656e, lsl #16
10036dab8: b9000008    	str	w8, [x0]
10036dabc: a9000274    	stp	x20, x0, [x19]
10036dac0: f9000a74    	str	x20, [x19, #0x10]
10036dac4: 39006274    	strb	w20, [x19, #0x18]
10036dac8: 52800108    	mov	w8, #0x8                ; =8
10036dacc: a902227f    	stp	xzr, x8, [x19, #0x20]
10036dad0: f9001a7f    	str	xzr, [x19, #0x30]
10036dad4: 14000534    	b	0x10036efa4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f14>
10036dad8: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036dadc: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036dae0: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036dae4: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036dae8: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036daec: f81603a8    	stur	x8, [x29, #-0xa0]
10036daf0: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036daf4: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036daf8: d101c3a8    	sub	x8, x29, #0x70
10036dafc: d10343a0    	sub	x0, x29, #0xd0
10036db00: d10243a1    	sub	x1, x29, #0x90
10036db04: 940a80ff    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036db08: 97f24e9b    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036db0c: 52800177    	mov	w23, #0xb               ; =11
10036db10: 52800160    	mov	w0, #0xb                ; =11
10036db14: 52800021    	mov	w1, #0x1                ; =1
10036db18: 97f24e93    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036db1c: b400b480    	cbz	x0, 0x10036f1ac <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x211c>
10036db20: aa0003f6    	mov	x22, x0
10036db24: 528d2e88    	mov	w8, #0x6974             ; =26996
10036db28: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036db2c: b8007008    	stur	w8, [x0, #0x7]
10036db30: f0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036db34: 91092908    	add	x8, x8, #0x24a
10036db38: f940011a    	ldr	x26, [x8]
10036db3c: f900001a    	str	x26, [x0]
10036db40: 52800168    	mov	w8, #0xb                ; =11
10036db44: a91203e8    	stp	x8, x0, [sp, #0x120]
10036db48: f9009be8    	str	x8, [sp, #0x130]
10036db4c: 97f24e8a    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036db50: 528004a0    	mov	w0, #0x25               ; =37
10036db54: 52800021    	mov	w1, #0x1                ; =1
10036db58: 97f24e83    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036db5c: b400b680    	cbz	x0, 0x10036f22c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x219c>
10036db60: f0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036db64: 91095508    	add	x8, x8, #0x255
10036db68: ad400500    	ldp	q0, q1, [x8]
10036db6c: ad000400    	stp	q0, q1, [x0]
10036db70: f841d108    	ldur	x8, [x8, #0x1d]
10036db74: f801d008    	stur	x8, [x0, #0x1d]
10036db78: 52800068    	mov	w8, #0x3                ; =3
10036db7c: 390643e8    	strb	w8, [sp, #0x190]
10036db80: 528004a8    	mov	w8, #0x25               ; =37
10036db84: a91983e8    	stp	x8, x0, [sp, #0x198]
10036db88: f900d7e8    	str	x8, [sp, #0x1a8]
10036db8c: 9106c3e8    	add	x8, sp, #0x1b0
10036db90: 910383e0    	add	x0, sp, #0xe0
10036db94: 910483e1    	add	x1, sp, #0x120
10036db98: 940fa328    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036db9c: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036dba0: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036dba4: eb09011f    	cmp	x8, x9
10036dba8: 54000201    	b.ne	0x10036dbe8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xb58>
10036dbac: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036dbb0: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036dbb4: 8b091508    	add	x8, x8, x9, lsl #5
10036dbb8: 3dc00100    	ldr	q0, [x8]
10036dbbc: 3c9903a0    	stur	q0, [x29, #-0x70]
10036dbc0: 3dc00500    	ldr	q0, [x8, #0x10]
10036dbc4: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036dbc8: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036dbcc: ad000101    	stp	q1, q0, [x8]
10036dbd0: 385903a8    	ldurb	w8, [x29, #-0x70]
10036dbd4: 7100191f    	cmp	w8, #0x6
10036dbd8: 54000200    	b.eq	0x10036dc18 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xb88>
10036dbdc: d101c3a0    	sub	x0, x29, #0x70
10036dbe0: 9401a678    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036dbe4: 1400000d    	b	0x10036dc18 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xb88>
10036dbe8: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036dbec: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036dbf0: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036dbf4: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036dbf8: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036dbfc: f81603a8    	stur	x8, [x29, #-0xa0]
10036dc00: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036dc04: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036dc08: d101c3a8    	sub	x8, x29, #0x70
10036dc0c: d10343a0    	sub	x0, x29, #0xd0
10036dc10: d10243a1    	sub	x1, x29, #0x90
10036dc14: 940a80bb    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036dc18: 3dc03be0    	ldr	q0, [sp, #0xe0]
10036dc1c: 3c866320    	stur	q0, [x25, #0x66]
10036dc20: f9407be8    	ldr	x8, [sp, #0xf0]
10036dc24: f9008fe8    	str	x8, [sp, #0x118]
10036dc28: 528000a8    	mov	w8, #0x5                ; =5
10036dc2c: 390403e8    	strb	w8, [sp, #0x100]
10036dc30: f90073ff    	str	xzr, [sp, #0xe0]
10036dc34: f9007bff    	str	xzr, [sp, #0xf0]
10036dc38: 97f24e4f    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036dc3c: 528000d7    	mov	w23, #0x6               ; =6
10036dc40: 528000c0    	mov	w0, #0x6                ; =6
10036dc44: 52800021    	mov	w1, #0x1                ; =1
10036dc48: 97f24e47    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036dc4c: b400ab80    	cbz	x0, 0x10036f1bc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x212c>
10036dc50: aa0003f6    	mov	x22, x0
10036dc54: 528e8e48    	mov	w8, #0x7472             ; =29810
10036dc58: 79000808    	strh	w8, [x0, #0x4]
10036dc5c: 528ccca8    	mov	w8, #0x6665             ; =26213
10036dc60: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036dc64: b9000008    	str	w8, [x0]
10036dc68: 528000c8    	mov	w8, #0x6                ; =6
10036dc6c: a91203e8    	stp	x8, x0, [sp, #0x120]
10036dc70: f9009be8    	str	x8, [sp, #0x130]
10036dc74: 97f24e40    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036dc78: 52800080    	mov	w0, #0x4                ; =4
10036dc7c: 52800021    	mov	w1, #0x1                ; =1
10036dc80: 97f24e39    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036dc84: b400adc0    	cbz	x0, 0x10036f23c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x21ac>
10036dc88: b900001b    	str	w27, [x0]
10036dc8c: 52800068    	mov	w8, #0x3                ; =3
10036dc90: 390643e8    	strb	w8, [sp, #0x190]
10036dc94: 52800088    	mov	w8, #0x4                ; =4
10036dc98: a91983e8    	stp	x8, x0, [sp, #0x198]
10036dc9c: f900d7e8    	str	x8, [sp, #0x1a8]
10036dca0: 9106c3e8    	add	x8, sp, #0x1b0
10036dca4: 910383e0    	add	x0, sp, #0xe0
10036dca8: 910483e1    	add	x1, sp, #0x120
10036dcac: 940fa2e3    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036dcb0: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036dcb4: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036dcb8: eb09011f    	cmp	x8, x9
10036dcbc: 54000201    	b.ne	0x10036dcfc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xc6c>
10036dcc0: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036dcc4: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036dcc8: 8b091508    	add	x8, x8, x9, lsl #5
10036dccc: 3dc00100    	ldr	q0, [x8]
10036dcd0: 3c9903a0    	stur	q0, [x29, #-0x70]
10036dcd4: 3dc00500    	ldr	q0, [x8, #0x10]
10036dcd8: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036dcdc: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036dce0: ad000101    	stp	q1, q0, [x8]
10036dce4: 385903a8    	ldurb	w8, [x29, #-0x70]
10036dce8: 7100191f    	cmp	w8, #0x6
10036dcec: 54000200    	b.eq	0x10036dd2c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xc9c>
10036dcf0: d101c3a0    	sub	x0, x29, #0x70
10036dcf4: 9401a633    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036dcf8: 1400000d    	b	0x10036dd2c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xc9c>
10036dcfc: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036dd00: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036dd04: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036dd08: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036dd0c: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036dd10: f81603a8    	stur	x8, [x29, #-0xa0]
10036dd14: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036dd18: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036dd1c: d101c3a8    	sub	x8, x29, #0x70
10036dd20: d10343a0    	sub	x0, x29, #0xd0
10036dd24: d10243a1    	sub	x1, x29, #0x90
10036dd28: 940a8076    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036dd2c: 97f24e12    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036dd30: 52800177    	mov	w23, #0xb               ; =11
10036dd34: 52800160    	mov	w0, #0xb                ; =11
10036dd38: 52800021    	mov	w1, #0x1                ; =1
10036dd3c: 97f24e0a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036dd40: b400a3e0    	cbz	x0, 0x10036f1bc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x212c>
10036dd44: aa0003f6    	mov	x22, x0
10036dd48: 528d2e88    	mov	w8, #0x6974             ; =26996
10036dd4c: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036dd50: b8007008    	stur	w8, [x0, #0x7]
10036dd54: f900001a    	str	x26, [x0]
10036dd58: 52800168    	mov	w8, #0xb                ; =11
10036dd5c: a91203e8    	stp	x8, x0, [sp, #0x120]
10036dd60: f9009be8    	str	x8, [sp, #0x130]
10036dd64: 97f24e04    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036dd68: 528005e0    	mov	w0, #0x2f               ; =47
10036dd6c: 52800021    	mov	w1, #0x1                ; =1
10036dd70: 97f24dfd    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036dd74: b400a6c0    	cbz	x0, 0x10036f24c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x21bc>
10036dd78: f0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036dd7c: 910b9908    	add	x8, x8, #0x2e6
10036dd80: ad400500    	ldp	q0, q1, [x8]
10036dd84: ad000400    	stp	q0, q1, [x0]
10036dd88: 3cc1f100    	ldur	q0, [x8, #0x1f]
10036dd8c: 3c81f000    	stur	q0, [x0, #0x1f]
10036dd90: 52800068    	mov	w8, #0x3                ; =3
10036dd94: 390643e8    	strb	w8, [sp, #0x190]
10036dd98: 528005e8    	mov	w8, #0x2f               ; =47
10036dd9c: a91983e8    	stp	x8, x0, [sp, #0x198]
10036dda0: f900d7e8    	str	x8, [sp, #0x1a8]
10036dda4: 9106c3e8    	add	x8, sp, #0x1b0
10036dda8: 910383e0    	add	x0, sp, #0xe0
10036ddac: 910483e1    	add	x1, sp, #0x120
10036ddb0: 940fa2a2    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036ddb4: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036ddb8: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036ddbc: eb09011f    	cmp	x8, x9
10036ddc0: 54000201    	b.ne	0x10036de00 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xd70>
10036ddc4: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036ddc8: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036ddcc: 8b091508    	add	x8, x8, x9, lsl #5
10036ddd0: 3dc00100    	ldr	q0, [x8]
10036ddd4: 3c9903a0    	stur	q0, [x29, #-0x70]
10036ddd8: 3dc00500    	ldr	q0, [x8, #0x10]
10036dddc: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036dde0: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036dde4: ad000101    	stp	q1, q0, [x8]
10036dde8: 385903a8    	ldurb	w8, [x29, #-0x70]
10036ddec: 7100191f    	cmp	w8, #0x6
10036ddf0: 54000200    	b.eq	0x10036de30 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xda0>
10036ddf4: d101c3a0    	sub	x0, x29, #0x70
10036ddf8: 9401a5f2    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036ddfc: 1400000d    	b	0x10036de30 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xda0>
10036de00: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036de04: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036de08: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036de0c: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036de10: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036de14: f81603a8    	stur	x8, [x29, #-0xa0]
10036de18: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036de1c: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036de20: d101c3a8    	sub	x8, x29, #0x70
10036de24: d10343a0    	sub	x0, x29, #0xd0
10036de28: d10243a1    	sub	x1, x29, #0x90
10036de2c: 940a8035    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036de30: 3dc03be0    	ldr	q0, [sp, #0xe0]
10036de34: 3c886320    	stur	q0, [x25, #0x86]
10036de38: f9407be8    	ldr	x8, [sp, #0xf0]
10036de3c: f9009fe8    	str	x8, [sp, #0x138]
10036de40: 528000a8    	mov	w8, #0x5                ; =5
10036de44: 390483e8    	strb	w8, [sp, #0x120]
10036de48: f900bfff    	str	xzr, [sp, #0x178]
10036de4c: f900c7ff    	str	xzr, [sp, #0x188]
10036de50: 97f24dc9    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036de54: 528000d7    	mov	w23, #0x6               ; =6
10036de58: 528000c0    	mov	w0, #0x6                ; =6
10036de5c: 52800021    	mov	w1, #0x1                ; =1
10036de60: 97f24dc1    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036de64: b4009b40    	cbz	x0, 0x10036f1cc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x213c>
10036de68: aa0003f6    	mov	x22, x0
10036de6c: 528e8e48    	mov	w8, #0x7472             ; =29810
10036de70: 79000808    	strh	w8, [x0, #0x4]
10036de74: 528ccca8    	mov	w8, #0x6665             ; =26213
10036de78: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036de7c: b9000008    	str	w8, [x0]
10036de80: 528000c8    	mov	w8, #0x6                ; =6
10036de84: a90e03e8    	stp	x8, x0, [sp, #0xe0]
10036de88: f9007be8    	str	x8, [sp, #0xf0]
10036de8c: 97f24dba    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036de90: 52800077    	mov	w23, #0x3               ; =3
10036de94: 52800060    	mov	w0, #0x3                ; =3
10036de98: 52800021    	mov	w1, #0x1                ; =1
10036de9c: 97f24db2    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036dea0: b4009de0    	cbz	x0, 0x10036f25c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x21cc>
10036dea4: 52800f08    	mov	w8, #0x78               ; =120
10036dea8: 39000808    	strb	w8, [x0, #0x2]
10036deac: 528c2da8    	mov	w8, #0x616d             ; =24941
10036deb0: 79000008    	strh	w8, [x0]
10036deb4: 390643f7    	strb	w23, [sp, #0x190]
10036deb8: a91983f7    	stp	x23, x0, [sp, #0x198]
10036debc: f900d7f7    	str	x23, [sp, #0x1a8]
10036dec0: 9106c3e8    	add	x8, sp, #0x1b0
10036dec4: 9105e3e0    	add	x0, sp, #0x178
10036dec8: 910383e1    	add	x1, sp, #0xe0
10036decc: 940fa25b    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036ded0: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036ded4: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036ded8: eb09011f    	cmp	x8, x9
10036dedc: 54000201    	b.ne	0x10036df1c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xe8c>
10036dee0: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036dee4: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036dee8: 8b091508    	add	x8, x8, x9, lsl #5
10036deec: 3dc00100    	ldr	q0, [x8]
10036def0: 3c9903a0    	stur	q0, [x29, #-0x70]
10036def4: 3dc00500    	ldr	q0, [x8, #0x10]
10036def8: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036defc: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036df00: ad000101    	stp	q1, q0, [x8]
10036df04: 385903a8    	ldurb	w8, [x29, #-0x70]
10036df08: 7100191f    	cmp	w8, #0x6
10036df0c: 54000200    	b.eq	0x10036df4c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xebc>
10036df10: d101c3a0    	sub	x0, x29, #0x70
10036df14: 9401a5ab    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036df18: 1400000d    	b	0x10036df4c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xebc>
10036df1c: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036df20: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036df24: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036df28: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036df2c: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036df30: f81603a8    	stur	x8, [x29, #-0xa0]
10036df34: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036df38: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036df3c: d101c3a8    	sub	x8, x29, #0x70
10036df40: d10343a0    	sub	x0, x29, #0xd0
10036df44: d10243a1    	sub	x1, x29, #0x90
10036df48: 940a7fee    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036df4c: 97f24d8a    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036df50: 52800177    	mov	w23, #0xb               ; =11
10036df54: 52800160    	mov	w0, #0xb                ; =11
10036df58: 52800021    	mov	w1, #0x1                ; =1
10036df5c: 97f24d82    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036df60: b4009360    	cbz	x0, 0x10036f1cc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x213c>
10036df64: aa0003f6    	mov	x22, x0
10036df68: 528d2e88    	mov	w8, #0x6974             ; =26996
10036df6c: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036df70: b8007008    	stur	w8, [x0, #0x7]
10036df74: f900001a    	str	x26, [x0]
10036df78: 52800168    	mov	w8, #0xb                ; =11
10036df7c: a90e03e8    	stp	x8, x0, [sp, #0xe0]
10036df80: f9007be8    	str	x8, [sp, #0xf0]
10036df84: 97f24d7c    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036df88: 52800600    	mov	w0, #0x30               ; =48
10036df8c: 52800021    	mov	w1, #0x1                ; =1
10036df90: 97f24d75    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036df94: b40096c0    	cbz	x0, 0x10036f26c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x21dc>
10036df98: f0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036df9c: 91195508    	add	x8, x8, #0x655
10036dfa0: ad400500    	ldp	q0, q1, [x8]
10036dfa4: ad000400    	stp	q0, q1, [x0]
10036dfa8: 3dc00900    	ldr	q0, [x8, #0x20]
10036dfac: 3d800800    	str	q0, [x0, #0x20]
10036dfb0: 52800068    	mov	w8, #0x3                ; =3
10036dfb4: 390643e8    	strb	w8, [sp, #0x190]
10036dfb8: 52800608    	mov	w8, #0x30               ; =48
10036dfbc: a91983e8    	stp	x8, x0, [sp, #0x198]
10036dfc0: f900d7e8    	str	x8, [sp, #0x1a8]
10036dfc4: 9106c3e8    	add	x8, sp, #0x1b0
10036dfc8: 9105e3e0    	add	x0, sp, #0x178
10036dfcc: 910383e1    	add	x1, sp, #0xe0
10036dfd0: 940fa21a    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036dfd4: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036dfd8: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036dfdc: eb09011f    	cmp	x8, x9
10036dfe0: 54000201    	b.ne	0x10036e020 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xf90>
10036dfe4: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036dfe8: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036dfec: 8b091508    	add	x8, x8, x9, lsl #5
10036dff0: 3dc00100    	ldr	q0, [x8]
10036dff4: 3c9903a0    	stur	q0, [x29, #-0x70]
10036dff8: 3dc00500    	ldr	q0, [x8, #0x10]
10036dffc: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036e000: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036e004: ad000101    	stp	q1, q0, [x8]
10036e008: 385903a8    	ldurb	w8, [x29, #-0x70]
10036e00c: 7100191f    	cmp	w8, #0x6
10036e010: 54000200    	b.eq	0x10036e050 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xfc0>
10036e014: d101c3a0    	sub	x0, x29, #0x70
10036e018: 9401a56a    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036e01c: 1400000d    	b	0x10036e050 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0xfc0>
10036e020: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e024: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e028: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e02c: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e030: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e034: f81603a8    	stur	x8, [x29, #-0xa0]
10036e038: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e03c: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e040: d101c3a8    	sub	x8, x29, #0x70
10036e044: d10343a0    	sub	x0, x29, #0xd0
10036e048: d10243a1    	sub	x1, x29, #0x90
10036e04c: 940a7fad    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036e050: f940c7e8    	ldr	x8, [sp, #0x188]
10036e054: 3ccd6320    	ldur	q0, [x25, #0xd6]
10036e058: 3c807300    	stur	q0, [x24, #0x7]
10036e05c: f8017308    	stur	x8, [x24, #0x17]
10036e060: ad4807e0    	ldp	q0, q1, [sp, #0x100]
10036e064: ad0006a0    	stp	q0, q1, [x21]
10036e068: ad4907e0    	ldp	q0, q1, [sp, #0x120]
10036e06c: ad0106a0    	stp	q0, q1, [x21, #0x20]
10036e070: 528000a8    	mov	w8, #0x5                ; =5
10036e074: 390102a8    	strb	w8, [x21, #0x40]
10036e078: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036e07c: 3c8412a0    	stur	q0, [x21, #0x41]
10036e080: 3cc0f300    	ldur	q0, [x24, #0xf]
10036e084: 3d8016a0    	str	q0, [x21, #0x50]
10036e088: 52800088    	mov	w8, #0x4                ; =4
10036e08c: a9005268    	stp	x8, x20, [x19]
10036e090: f9000a68    	str	x8, [x19, #0x10]
10036e094: 39006268    	strb	w8, [x19, #0x18]
10036e098: 52800068    	mov	w8, #0x3                ; =3
10036e09c: 140003c0    	b	0x10036ef9c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f0c>
10036e0a0: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e0a4: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e0a8: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e0ac: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e0b0: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e0b4: f81603a8    	stur	x8, [x29, #-0xa0]
10036e0b8: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e0bc: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e0c0: d101c3a8    	sub	x8, x29, #0x70
10036e0c4: d10343a0    	sub	x0, x29, #0xd0
10036e0c8: d10243a1    	sub	x1, x29, #0x90
10036e0cc: 940a7f8d    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036e0d0: 97f24d29    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e0d4: 52800177    	mov	w23, #0xb               ; =11
10036e0d8: 52800160    	mov	w0, #0xb                ; =11
10036e0dc: 52800021    	mov	w1, #0x1                ; =1
10036e0e0: 97f24d21    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e0e4: b4008cc0    	cbz	x0, 0x10036f27c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x21ec>
10036e0e8: aa0003f6    	mov	x22, x0
10036e0ec: 528d2e88    	mov	w8, #0x6974             ; =26996
10036e0f0: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036e0f4: b8007008    	stur	w8, [x0, #0x7]
10036e0f8: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036e0fc: 91092908    	add	x8, x8, #0x24a
10036e100: f940011a    	ldr	x26, [x8]
10036e104: f900001a    	str	x26, [x0]
10036e108: 52800168    	mov	w8, #0xb                ; =11
10036e10c: a91203e8    	stp	x8, x0, [sp, #0x120]
10036e110: f9009be8    	str	x8, [sp, #0x130]
10036e114: 97f24d18    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e118: 528004a0    	mov	w0, #0x25               ; =37
10036e11c: 52800021    	mov	w1, #0x1                ; =1
10036e120: 97f24d11    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e124: b4008e20    	cbz	x0, 0x10036f2e8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2258>
10036e128: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036e12c: 91095508    	add	x8, x8, #0x255
10036e130: ad400500    	ldp	q0, q1, [x8]
10036e134: ad000400    	stp	q0, q1, [x0]
10036e138: f841d108    	ldur	x8, [x8, #0x1d]
10036e13c: f801d008    	stur	x8, [x0, #0x1d]
10036e140: 52800068    	mov	w8, #0x3                ; =3
10036e144: 390643e8    	strb	w8, [sp, #0x190]
10036e148: 528004a8    	mov	w8, #0x25               ; =37
10036e14c: a91983e8    	stp	x8, x0, [sp, #0x198]
10036e150: f900d7e8    	str	x8, [sp, #0x1a8]
10036e154: 9106c3e8    	add	x8, sp, #0x1b0
10036e158: 910403e0    	add	x0, sp, #0x100
10036e15c: 910483e1    	add	x1, sp, #0x120
10036e160: 940fa1b6    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036e164: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036e168: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036e16c: eb09011f    	cmp	x8, x9
10036e170: 54000201    	b.ne	0x10036e1b0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1120>
10036e174: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036e178: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036e17c: 8b091508    	add	x8, x8, x9, lsl #5
10036e180: 3dc00100    	ldr	q0, [x8]
10036e184: 3c9903a0    	stur	q0, [x29, #-0x70]
10036e188: 3dc00500    	ldr	q0, [x8, #0x10]
10036e18c: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036e190: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036e194: ad000101    	stp	q1, q0, [x8]
10036e198: 385903a8    	ldurb	w8, [x29, #-0x70]
10036e19c: 7100191f    	cmp	w8, #0x6
10036e1a0: 54000200    	b.eq	0x10036e1e0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1150>
10036e1a4: d101c3a0    	sub	x0, x29, #0x70
10036e1a8: 9401a506    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036e1ac: 1400000d    	b	0x10036e1e0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1150>
10036e1b0: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e1b4: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e1b8: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e1bc: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e1c0: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e1c4: f81603a8    	stur	x8, [x29, #-0xa0]
10036e1c8: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e1cc: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e1d0: d101c3a8    	sub	x8, x29, #0x70
10036e1d4: d10343a0    	sub	x0, x29, #0xd0
10036e1d8: d10243a1    	sub	x1, x29, #0x90
10036e1dc: 940a7f49    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036e1e0: 3dc043e0    	ldr	q0, [sp, #0x100]
10036e1e4: 3c846320    	stur	q0, [x25, #0x46]
10036e1e8: f9408be8    	ldr	x8, [sp, #0x110]
10036e1ec: f9007fe8    	str	x8, [sp, #0xf8]
10036e1f0: 528000a8    	mov	w8, #0x5                ; =5
10036e1f4: 390383e8    	strb	w8, [sp, #0xe0]
10036e1f8: f900bfff    	str	xzr, [sp, #0x178]
10036e1fc: f900c7ff    	str	xzr, [sp, #0x188]
10036e200: 97f24cdd    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e204: 528000d7    	mov	w23, #0x6               ; =6
10036e208: 528000c0    	mov	w0, #0x6                ; =6
10036e20c: 52800021    	mov	w1, #0x1                ; =1
10036e210: 97f24cd5    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e214: b40083c0    	cbz	x0, 0x10036f28c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x21fc>
10036e218: aa0003f6    	mov	x22, x0
10036e21c: 528e8e48    	mov	w8, #0x7472             ; =29810
10036e220: 79000808    	strh	w8, [x0, #0x4]
10036e224: 528ccca8    	mov	w8, #0x6665             ; =26213
10036e228: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036e22c: b9000008    	str	w8, [x0]
10036e230: 528000c8    	mov	w8, #0x6                ; =6
10036e234: a91203e8    	stp	x8, x0, [sp, #0x120]
10036e238: f9009be8    	str	x8, [sp, #0x130]
10036e23c: 97f24cce    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e240: 528000c0    	mov	w0, #0x6                ; =6
10036e244: 52800021    	mov	w1, #0x1                ; =1
10036e248: 97f24cc7    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e24c: b4008560    	cbz	x0, 0x10036f2f8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2268>
10036e250: 528daea8    	mov	w8, #0x6d75             ; =28021
10036e254: 79000808    	strh	w8, [x0, #0x4]
10036e258: 528cada8    	mov	w8, #0x656d             ; =25965
10036e25c: 72ad2c88    	movk	w8, #0x6964, lsl #16
10036e260: b9000008    	str	w8, [x0]
10036e264: 52800068    	mov	w8, #0x3                ; =3
10036e268: 390643e8    	strb	w8, [sp, #0x190]
10036e26c: 528000c8    	mov	w8, #0x6                ; =6
10036e270: a91983e8    	stp	x8, x0, [sp, #0x198]
10036e274: f900d7e8    	str	x8, [sp, #0x1a8]
10036e278: 9106c3e8    	add	x8, sp, #0x1b0
10036e27c: 9105e3e0    	add	x0, sp, #0x178
10036e280: 910483e1    	add	x1, sp, #0x120
10036e284: 940fa16d    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036e288: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036e28c: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036e290: eb09011f    	cmp	x8, x9
10036e294: 54000201    	b.ne	0x10036e2d4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1244>
10036e298: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036e29c: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036e2a0: 8b091508    	add	x8, x8, x9, lsl #5
10036e2a4: 3dc00100    	ldr	q0, [x8]
10036e2a8: 3c9903a0    	stur	q0, [x29, #-0x70]
10036e2ac: 3dc00500    	ldr	q0, [x8, #0x10]
10036e2b0: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036e2b4: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036e2b8: ad000101    	stp	q1, q0, [x8]
10036e2bc: 385903a8    	ldurb	w8, [x29, #-0x70]
10036e2c0: 7100191f    	cmp	w8, #0x6
10036e2c4: 54000200    	b.eq	0x10036e304 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1274>
10036e2c8: d101c3a0    	sub	x0, x29, #0x70
10036e2cc: 9401a4bd    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036e2d0: 1400000d    	b	0x10036e304 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1274>
10036e2d4: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e2d8: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e2dc: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e2e0: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e2e4: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e2e8: f81603a8    	stur	x8, [x29, #-0xa0]
10036e2ec: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e2f0: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e2f4: d101c3a8    	sub	x8, x29, #0x70
10036e2f8: d10343a0    	sub	x0, x29, #0xd0
10036e2fc: d10243a1    	sub	x1, x29, #0x90
10036e300: 940a7f00    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036e304: 97f24c9c    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e308: 52800177    	mov	w23, #0xb               ; =11
10036e30c: 52800160    	mov	w0, #0xb                ; =11
10036e310: 52800021    	mov	w1, #0x1                ; =1
10036e314: 97f24c94    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e318: b4007ba0    	cbz	x0, 0x10036f28c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x21fc>
10036e31c: aa0003f6    	mov	x22, x0
10036e320: 528d2e88    	mov	w8, #0x6974             ; =26996
10036e324: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036e328: b8007008    	stur	w8, [x0, #0x7]
10036e32c: f900001a    	str	x26, [x0]
10036e330: 52800168    	mov	w8, #0xb                ; =11
10036e334: a91203e8    	stp	x8, x0, [sp, #0x120]
10036e338: f9009be8    	str	x8, [sp, #0x130]
10036e33c: 97f24c8e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e340: 528006a0    	mov	w0, #0x35               ; =53
10036e344: 52800021    	mov	w1, #0x1                ; =1
10036e348: 97f24c87    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e34c: b4007de0    	cbz	x0, 0x10036f308 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2278>
10036e350: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036e354: 910a0108    	add	x8, x8, #0x280
10036e358: ad400500    	ldp	q0, q1, [x8]
10036e35c: ad000400    	stp	q0, q1, [x0]
10036e360: 3dc00900    	ldr	q0, [x8, #0x20]
10036e364: 3d800800    	str	q0, [x0, #0x20]
10036e368: f842d108    	ldur	x8, [x8, #0x2d]
10036e36c: f802d008    	stur	x8, [x0, #0x2d]
10036e370: 52800068    	mov	w8, #0x3                ; =3
10036e374: 390643e8    	strb	w8, [sp, #0x190]
10036e378: 528006a8    	mov	w8, #0x35               ; =53
10036e37c: a91983e8    	stp	x8, x0, [sp, #0x198]
10036e380: f900d7e8    	str	x8, [sp, #0x1a8]
10036e384: 9106c3e8    	add	x8, sp, #0x1b0
10036e388: 9105e3e0    	add	x0, sp, #0x178
10036e38c: 910483e1    	add	x1, sp, #0x120
10036e390: 940fa12a    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036e394: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036e398: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036e39c: eb09011f    	cmp	x8, x9
10036e3a0: 54000201    	b.ne	0x10036e3e0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1350>
10036e3a4: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036e3a8: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036e3ac: 8b091508    	add	x8, x8, x9, lsl #5
10036e3b0: 3dc00100    	ldr	q0, [x8]
10036e3b4: 3c9903a0    	stur	q0, [x29, #-0x70]
10036e3b8: 3dc00500    	ldr	q0, [x8, #0x10]
10036e3bc: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036e3c0: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036e3c4: ad000101    	stp	q1, q0, [x8]
10036e3c8: 385903a8    	ldurb	w8, [x29, #-0x70]
10036e3cc: 7100191f    	cmp	w8, #0x6
10036e3d0: 54000200    	b.eq	0x10036e410 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1380>
10036e3d4: d101c3a0    	sub	x0, x29, #0x70
10036e3d8: 9401a47a    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036e3dc: 1400000d    	b	0x10036e410 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1380>
10036e3e0: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e3e4: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e3e8: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e3ec: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e3f0: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e3f4: f81603a8    	stur	x8, [x29, #-0xa0]
10036e3f8: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e3fc: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e400: d101c3a8    	sub	x8, x29, #0x70
10036e404: d10343a0    	sub	x0, x29, #0xd0
10036e408: d10243a1    	sub	x1, x29, #0x90
10036e40c: 940a7ebd    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036e410: 3ccd6320    	ldur	q0, [x25, #0xd6]
10036e414: 3c866320    	stur	q0, [x25, #0x66]
10036e418: f940c7e8    	ldr	x8, [sp, #0x188]
10036e41c: f9008fe8    	str	x8, [sp, #0x118]
10036e420: 528000a8    	mov	w8, #0x5                ; =5
10036e424: 390403e8    	strb	w8, [sp, #0x100]
10036e428: f900bfff    	str	xzr, [sp, #0x178]
10036e42c: f900c7ff    	str	xzr, [sp, #0x188]
10036e430: 97f24c51    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e434: 528000d7    	mov	w23, #0x6               ; =6
10036e438: 528000c0    	mov	w0, #0x6                ; =6
10036e43c: 52800021    	mov	w1, #0x1                ; =1
10036e440: 97f24c49    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e444: b40072c0    	cbz	x0, 0x10036f29c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x220c>
10036e448: aa0003f6    	mov	x22, x0
10036e44c: 528e8e48    	mov	w8, #0x7472             ; =29810
10036e450: 79000808    	strh	w8, [x0, #0x4]
10036e454: 528ccca8    	mov	w8, #0x6665             ; =26213
10036e458: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036e45c: b9000008    	str	w8, [x0]
10036e460: 528000c8    	mov	w8, #0x6                ; =6
10036e464: a91203e8    	stp	x8, x0, [sp, #0x120]
10036e468: f9009be8    	str	x8, [sp, #0x130]
10036e46c: 97f24c42    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e470: 52800080    	mov	w0, #0x4                ; =4
10036e474: 52800021    	mov	w1, #0x1                ; =1
10036e478: 97f24c3b    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e47c: b40074e0    	cbz	x0, 0x10036f318 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2288>
10036e480: 528d2d08    	mov	w8, #0x6968             ; =26984
10036e484: 72ad0ce8    	movk	w8, #0x6867, lsl #16
10036e488: b9000008    	str	w8, [x0]
10036e48c: 52800068    	mov	w8, #0x3                ; =3
10036e490: 390643e8    	strb	w8, [sp, #0x190]
10036e494: 52800088    	mov	w8, #0x4                ; =4
10036e498: a91983e8    	stp	x8, x0, [sp, #0x198]
10036e49c: f900d7e8    	str	x8, [sp, #0x1a8]
10036e4a0: 9106c3e8    	add	x8, sp, #0x1b0
10036e4a4: 9105e3e0    	add	x0, sp, #0x178
10036e4a8: 910483e1    	add	x1, sp, #0x120
10036e4ac: 940fa0e3    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036e4b0: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036e4b4: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036e4b8: eb09011f    	cmp	x8, x9
10036e4bc: 54000201    	b.ne	0x10036e4fc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x146c>
10036e4c0: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036e4c4: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036e4c8: 8b091508    	add	x8, x8, x9, lsl #5
10036e4cc: 3dc00100    	ldr	q0, [x8]
10036e4d0: 3c9903a0    	stur	q0, [x29, #-0x70]
10036e4d4: 3dc00500    	ldr	q0, [x8, #0x10]
10036e4d8: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036e4dc: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036e4e0: ad000101    	stp	q1, q0, [x8]
10036e4e4: 385903a8    	ldurb	w8, [x29, #-0x70]
10036e4e8: 7100191f    	cmp	w8, #0x6
10036e4ec: 54000200    	b.eq	0x10036e52c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x149c>
10036e4f0: d101c3a0    	sub	x0, x29, #0x70
10036e4f4: 9401a433    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036e4f8: 1400000d    	b	0x10036e52c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x149c>
10036e4fc: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e500: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e504: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e508: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e50c: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e510: f81603a8    	stur	x8, [x29, #-0xa0]
10036e514: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e518: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e51c: d101c3a8    	sub	x8, x29, #0x70
10036e520: d10343a0    	sub	x0, x29, #0xd0
10036e524: d10243a1    	sub	x1, x29, #0x90
10036e528: 940a7e76    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036e52c: 97f24c12    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e530: 52800177    	mov	w23, #0xb               ; =11
10036e534: 52800160    	mov	w0, #0xb                ; =11
10036e538: 52800021    	mov	w1, #0x1                ; =1
10036e53c: 97f24c0a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e540: b4006ae0    	cbz	x0, 0x10036f29c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x220c>
10036e544: aa0003f6    	mov	x22, x0
10036e548: 528d2e88    	mov	w8, #0x6974             ; =26996
10036e54c: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036e550: b8007008    	stur	w8, [x0, #0x7]
10036e554: f900001a    	str	x26, [x0]
10036e558: 52800168    	mov	w8, #0xb                ; =11
10036e55c: a91203e8    	stp	x8, x0, [sp, #0x120]
10036e560: f9009be8    	str	x8, [sp, #0x130]
10036e564: 97f24c04    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e568: 52800580    	mov	w0, #0x2c               ; =44
10036e56c: 52800021    	mov	w1, #0x1                ; =1
10036e570: 97f24bfd    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e574: b4006da0    	cbz	x0, 0x10036f328 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2298>
10036e578: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036e57c: 910ad508    	add	x8, x8, #0x2b5
10036e580: ad400500    	ldp	q0, q1, [x8]
10036e584: ad000400    	stp	q0, q1, [x0]
10036e588: 3cc1c100    	ldur	q0, [x8, #0x1c]
10036e58c: 3c81c000    	stur	q0, [x0, #0x1c]
10036e590: 52800068    	mov	w8, #0x3                ; =3
10036e594: 390643e8    	strb	w8, [sp, #0x190]
10036e598: 52800588    	mov	w8, #0x2c               ; =44
10036e59c: a91983e8    	stp	x8, x0, [sp, #0x198]
10036e5a0: f900d7e8    	str	x8, [sp, #0x1a8]
10036e5a4: 9106c3e8    	add	x8, sp, #0x1b0
10036e5a8: 9105e3e0    	add	x0, sp, #0x178
10036e5ac: 910483e1    	add	x1, sp, #0x120
10036e5b0: 940fa0a2    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036e5b4: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036e5b8: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036e5bc: eb09011f    	cmp	x8, x9
10036e5c0: 54000201    	b.ne	0x10036e600 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1570>
10036e5c4: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036e5c8: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036e5cc: 8b091508    	add	x8, x8, x9, lsl #5
10036e5d0: 3dc00100    	ldr	q0, [x8]
10036e5d4: 3c9903a0    	stur	q0, [x29, #-0x70]
10036e5d8: 3dc00500    	ldr	q0, [x8, #0x10]
10036e5dc: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036e5e0: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036e5e4: ad000101    	stp	q1, q0, [x8]
10036e5e8: 385903a8    	ldurb	w8, [x29, #-0x70]
10036e5ec: 7100191f    	cmp	w8, #0x6
10036e5f0: 54000200    	b.eq	0x10036e630 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x15a0>
10036e5f4: d101c3a0    	sub	x0, x29, #0x70
10036e5f8: 9401a3f2    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036e5fc: 1400000d    	b	0x10036e630 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x15a0>
10036e600: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e604: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e608: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e60c: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e610: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e614: f81603a8    	stur	x8, [x29, #-0xa0]
10036e618: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e61c: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e620: d101c3a8    	sub	x8, x29, #0x70
10036e624: d10343a0    	sub	x0, x29, #0xd0
10036e628: d10243a1    	sub	x1, x29, #0x90
10036e62c: 940a7e35    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036e630: 3ccd6320    	ldur	q0, [x25, #0xd6]
10036e634: 3c886320    	stur	q0, [x25, #0x86]
10036e638: f940c7e8    	ldr	x8, [sp, #0x188]
10036e63c: f9009fe8    	str	x8, [sp, #0x138]
10036e640: 528000a8    	mov	w8, #0x5                ; =5
10036e644: 390483e8    	strb	w8, [sp, #0x120]
10036e648: f900b3ff    	str	xzr, [sp, #0x160]
10036e64c: f900bbff    	str	xzr, [sp, #0x170]
10036e650: 97f24bc9    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e654: 528000d7    	mov	w23, #0x6               ; =6
10036e658: 528000c0    	mov	w0, #0x6                ; =6
10036e65c: 52800021    	mov	w1, #0x1                ; =1
10036e660: 97f24bc1    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e664: b4006240    	cbz	x0, 0x10036f2ac <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x221c>
10036e668: aa0003f6    	mov	x22, x0
10036e66c: 528e8e48    	mov	w8, #0x7472             ; =29810
10036e670: 79000808    	strh	w8, [x0, #0x4]
10036e674: 528ccca8    	mov	w8, #0x6665             ; =26213
10036e678: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036e67c: b9000008    	str	w8, [x0]
10036e680: 528000c8    	mov	w8, #0x6                ; =6
10036e684: a91783e8    	stp	x8, x0, [sp, #0x178]
10036e688: f900c7e8    	str	x8, [sp, #0x188]
10036e68c: 97f24bba    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e690: 528000a0    	mov	w0, #0x5                ; =5
10036e694: 52800021    	mov	w1, #0x1                ; =1
10036e698: 97f24bb3    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e69c: b40064e0    	cbz	x0, 0x10036f338 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x22a8>
10036e6a0: 52800d08    	mov	w8, #0x68               ; =104
10036e6a4: 39001008    	strb	w8, [x0, #0x4]
10036e6a8: 528d0f08    	mov	w8, #0x6878             ; =26744
10036e6ac: 72aced28    	movk	w8, #0x6769, lsl #16
10036e6b0: b9000008    	str	w8, [x0]
10036e6b4: 52800068    	mov	w8, #0x3                ; =3
10036e6b8: 390643e8    	strb	w8, [sp, #0x190]
10036e6bc: 528000a8    	mov	w8, #0x5                ; =5
10036e6c0: a91983e8    	stp	x8, x0, [sp, #0x198]
10036e6c4: f900d7e8    	str	x8, [sp, #0x1a8]
10036e6c8: 9106c3e8    	add	x8, sp, #0x1b0
10036e6cc: 910583e0    	add	x0, sp, #0x160
10036e6d0: 9105e3e1    	add	x1, sp, #0x178
10036e6d4: 940fa059    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036e6d8: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036e6dc: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036e6e0: eb09011f    	cmp	x8, x9
10036e6e4: 54000201    	b.ne	0x10036e724 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1694>
10036e6e8: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036e6ec: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036e6f0: 8b091508    	add	x8, x8, x9, lsl #5
10036e6f4: 3dc00100    	ldr	q0, [x8]
10036e6f8: 3c9903a0    	stur	q0, [x29, #-0x70]
10036e6fc: 3dc00500    	ldr	q0, [x8, #0x10]
10036e700: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036e704: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036e708: ad000101    	stp	q1, q0, [x8]
10036e70c: 385903a8    	ldurb	w8, [x29, #-0x70]
10036e710: 7100191f    	cmp	w8, #0x6
10036e714: 54000200    	b.eq	0x10036e754 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x16c4>
10036e718: d101c3a0    	sub	x0, x29, #0x70
10036e71c: 9401a3a9    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036e720: 1400000d    	b	0x10036e754 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x16c4>
10036e724: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e728: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e72c: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e730: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e734: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e738: f81603a8    	stur	x8, [x29, #-0xa0]
10036e73c: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e740: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e744: d101c3a8    	sub	x8, x29, #0x70
10036e748: d10343a0    	sub	x0, x29, #0xd0
10036e74c: d10243a1    	sub	x1, x29, #0x90
10036e750: 940a7dec    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036e754: 97f24b88    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e758: 52800177    	mov	w23, #0xb               ; =11
10036e75c: 52800160    	mov	w0, #0xb                ; =11
10036e760: 52800021    	mov	w1, #0x1                ; =1
10036e764: 97f24b80    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e768: b4005a20    	cbz	x0, 0x10036f2ac <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x221c>
10036e76c: aa0003f6    	mov	x22, x0
10036e770: 528d2e88    	mov	w8, #0x6974             ; =26996
10036e774: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036e778: b8007008    	stur	w8, [x0, #0x7]
10036e77c: f900001a    	str	x26, [x0]
10036e780: 52800168    	mov	w8, #0xb                ; =11
10036e784: a91783e8    	stp	x8, x0, [sp, #0x178]
10036e788: f900c7e8    	str	x8, [sp, #0x188]
10036e78c: 97f24b7a    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e790: 528005e0    	mov	w0, #0x2f               ; =47
10036e794: 52800021    	mov	w1, #0x1                ; =1
10036e798: 97f24b73    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e79c: b4005d60    	cbz	x0, 0x10036f348 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x22b8>
10036e7a0: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036e7a4: 910b9908    	add	x8, x8, #0x2e6
10036e7a8: ad400500    	ldp	q0, q1, [x8]
10036e7ac: ad000400    	stp	q0, q1, [x0]
10036e7b0: 3cc1f100    	ldur	q0, [x8, #0x1f]
10036e7b4: 3c81f000    	stur	q0, [x0, #0x1f]
10036e7b8: 52800068    	mov	w8, #0x3                ; =3
10036e7bc: 390643e8    	strb	w8, [sp, #0x190]
10036e7c0: 528005e8    	mov	w8, #0x2f               ; =47
10036e7c4: a91983e8    	stp	x8, x0, [sp, #0x198]
10036e7c8: f900d7e8    	str	x8, [sp, #0x1a8]
10036e7cc: 9106c3e8    	add	x8, sp, #0x1b0
10036e7d0: 910583e0    	add	x0, sp, #0x160
10036e7d4: 9105e3e1    	add	x1, sp, #0x178
10036e7d8: 940fa018    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036e7dc: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036e7e0: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036e7e4: eb09011f    	cmp	x8, x9
10036e7e8: 54000201    	b.ne	0x10036e828 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1798>
10036e7ec: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036e7f0: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036e7f4: 8b091508    	add	x8, x8, x9, lsl #5
10036e7f8: 3dc00100    	ldr	q0, [x8]
10036e7fc: 3c9903a0    	stur	q0, [x29, #-0x70]
10036e800: 3dc00500    	ldr	q0, [x8, #0x10]
10036e804: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036e808: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036e80c: ad000101    	stp	q1, q0, [x8]
10036e810: 385903a8    	ldurb	w8, [x29, #-0x70]
10036e814: 7100191f    	cmp	w8, #0x6
10036e818: 54000200    	b.eq	0x10036e858 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x17c8>
10036e81c: d101c3a0    	sub	x0, x29, #0x70
10036e820: 9401a368    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036e824: 1400000d    	b	0x10036e858 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x17c8>
10036e828: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e82c: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e830: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e834: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e838: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e83c: f81603a8    	stur	x8, [x29, #-0xa0]
10036e840: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e844: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e848: d101c3a8    	sub	x8, x29, #0x70
10036e84c: d10343a0    	sub	x0, x29, #0xd0
10036e850: d10243a1    	sub	x1, x29, #0x90
10036e854: 940a7dab    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036e858: f940bbe8    	ldr	x8, [sp, #0x170]
10036e85c: 3dc05be0    	ldr	q0, [sp, #0x160]
10036e860: 3c807300    	stur	q0, [x24, #0x7]
10036e864: f8017308    	stur	x8, [x24, #0x17]
10036e868: ad4707e0    	ldp	q0, q1, [sp, #0xe0]
10036e86c: ad0006a0    	stp	q0, q1, [x21]
10036e870: ad4807e0    	ldp	q0, q1, [sp, #0x100]
10036e874: ad0106a0    	stp	q0, q1, [x21, #0x20]
10036e878: ad4907e0    	ldp	q0, q1, [sp, #0x120]
10036e87c: ad0206a0    	stp	q0, q1, [x21, #0x40]
10036e880: 528000a8    	mov	w8, #0x5                ; =5
10036e884: 390182a8    	strb	w8, [x21, #0x60]
10036e888: 3cc0f300    	ldur	q0, [x24, #0xf]
10036e88c: 3d801ea0    	str	q0, [x21, #0x70]
10036e890: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036e894: 3c8612a0    	stur	q0, [x21, #0x61]
10036e898: 528000c8    	mov	w8, #0x6                ; =6
10036e89c: a9005268    	stp	x8, x20, [x19]
10036e8a0: f9000a68    	str	x8, [x19, #0x10]
10036e8a4: 52800088    	mov	w8, #0x4                ; =4
10036e8a8: 39006268    	strb	w8, [x19, #0x18]
10036e8ac: 140001bc    	b	0x10036ef9c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f0c>
10036e8b0: 35001668    	cbnz	w8, 0x10036eb7c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1aec>
10036e8b4: 97f24b30    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e8b8: 528000c0    	mov	w0, #0x6                ; =6
10036e8bc: 52800021    	mov	w1, #0x1                ; =1
10036e8c0: 97f24b29    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e8c4: b4004fc0    	cbz	x0, 0x10036f2bc <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x222c>
10036e8c8: aa0003f4    	mov	x20, x0
10036e8cc: 528daea8    	mov	w8, #0x6d75             ; =28021
10036e8d0: 79000808    	strh	w8, [x0, #0x4]
10036e8d4: 528cada8    	mov	w8, #0x656d             ; =25965
10036e8d8: 72ad2c88    	movk	w8, #0x6964, lsl #16
10036e8dc: b9000008    	str	w8, [x0]
10036e8e0: 97f24b25    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e8e4: 52800400    	mov	w0, #0x20               ; =32
10036e8e8: 52800101    	mov	w1, #0x8                ; =8
10036e8ec: 97f24b1e    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e8f0: b4003d20    	cbz	x0, 0x10036f094 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2004>
10036e8f4: aa0003f5    	mov	x21, x0
10036e8f8: f90083ff    	str	xzr, [sp, #0x100]
10036e8fc: f9008bff    	str	xzr, [sp, #0x110]
10036e900: 97f24b1d    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e904: 528000d7    	mov	w23, #0x6               ; =6
10036e908: 528000c0    	mov	w0, #0x6                ; =6
10036e90c: 52800021    	mov	w1, #0x1                ; =1
10036e910: 97f24b15    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e914: b40052a0    	cbz	x0, 0x10036f368 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x22d8>
10036e918: aa0003f6    	mov	x22, x0
10036e91c: 528e8e48    	mov	w8, #0x7472             ; =29810
10036e920: 79000808    	strh	w8, [x0, #0x4]
10036e924: 528ccca8    	mov	w8, #0x6665             ; =26213
10036e928: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036e92c: b9000008    	str	w8, [x0]
10036e930: 528000c8    	mov	w8, #0x6                ; =6
10036e934: a91203e8    	stp	x8, x0, [sp, #0x120]
10036e938: f9009be8    	str	x8, [sp, #0x130]
10036e93c: 97f24b0e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036e940: 528000c0    	mov	w0, #0x6                ; =6
10036e944: 52800021    	mov	w1, #0x1                ; =1
10036e948: 97f24b07    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036e94c: b40051e0    	cbz	x0, 0x10036f388 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x22f8>
10036e950: 528daea8    	mov	w8, #0x6d75             ; =28021
10036e954: 79000808    	strh	w8, [x0, #0x4]
10036e958: 528cada8    	mov	w8, #0x656d             ; =25965
10036e95c: 72ad2c88    	movk	w8, #0x6964, lsl #16
10036e960: b9000008    	str	w8, [x0]
10036e964: 52800068    	mov	w8, #0x3                ; =3
10036e968: 390643e8    	strb	w8, [sp, #0x190]
10036e96c: 528000c8    	mov	w8, #0x6                ; =6
10036e970: a91983e8    	stp	x8, x0, [sp, #0x198]
10036e974: f900d7e8    	str	x8, [sp, #0x1a8]
10036e978: 9106c3e8    	add	x8, sp, #0x1b0
10036e97c: 910403e0    	add	x0, sp, #0x100
10036e980: 910483e1    	add	x1, sp, #0x120
10036e984: 940f9fad    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036e988: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036e98c: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036e990: eb09011f    	cmp	x8, x9
10036e994: 54002441    	b.ne	0x10036ee1c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1d8c>
10036e998: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036e99c: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036e9a0: 8b091508    	add	x8, x8, x9, lsl #5
10036e9a4: 3dc00100    	ldr	q0, [x8]
10036e9a8: 3c9903a0    	stur	q0, [x29, #-0x70]
10036e9ac: 3dc00500    	ldr	q0, [x8, #0x10]
10036e9b0: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036e9b4: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036e9b8: ad000101    	stp	q1, q0, [x8]
10036e9bc: 385903a8    	ldurb	w8, [x29, #-0x70]
10036e9c0: 7100191f    	cmp	w8, #0x6
10036e9c4: 54002440    	b.eq	0x10036ee4c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1dbc>
10036e9c8: d101c3a0    	sub	x0, x29, #0x70
10036e9cc: 9401a2fd    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036e9d0: 1400011f    	b	0x10036ee4c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1dbc>
10036e9d4: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036e9d8: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036e9dc: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036e9e0: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036e9e4: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036e9e8: f81603a8    	stur	x8, [x29, #-0xa0]
10036e9ec: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036e9f0: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036e9f4: d101c3a8    	sub	x8, x29, #0x70
10036e9f8: d10343a0    	sub	x0, x29, #0xd0
10036e9fc: d10243a1    	sub	x1, x29, #0x90
10036ea00: 940a7d40    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036ea04: 97f24adc    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036ea08: 52800177    	mov	w23, #0xb               ; =11
10036ea0c: 52800160    	mov	w0, #0xb                ; =11
10036ea10: 52800021    	mov	w1, #0x1                ; =1
10036ea14: 97f24ad4    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036ea18: b4004580    	cbz	x0, 0x10036f2c8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2238>
10036ea1c: aa0003f6    	mov	x22, x0
10036ea20: 528d2e88    	mov	w8, #0x6974             ; =26996
10036ea24: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036ea28: b8007008    	stur	w8, [x0, #0x7]
10036ea2c: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036ea30: 91092908    	add	x8, x8, #0x24a
10036ea34: f9400108    	ldr	x8, [x8]
10036ea38: f9000008    	str	x8, [x0]
10036ea3c: 52800168    	mov	w8, #0xb                ; =11
10036ea40: a91203e8    	stp	x8, x0, [sp, #0x120]
10036ea44: f9009be8    	str	x8, [sp, #0x130]
10036ea48: 97f24acb    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036ea4c: 52800260    	mov	w0, #0x13               ; =19
10036ea50: 52800021    	mov	w1, #0x1                ; =1
10036ea54: 97f24ac4    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036ea58: b4004900    	cbz	x0, 0x10036f378 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x22e8>
10036ea5c: 528deda8    	mov	w8, #0x6f6d             ; =28525
10036ea60: 72acac88    	movk	w8, #0x6564, lsl #16
10036ea64: b800f008    	stur	w8, [x0, #0xf]
10036ea68: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036ea6c: 91180508    	add	x8, x8, #0x601
10036ea70: 3dc00100    	ldr	q0, [x8]
10036ea74: 3d800000    	str	q0, [x0]
10036ea78: 52800068    	mov	w8, #0x3                ; =3
10036ea7c: 390643e8    	strb	w8, [sp, #0x190]
10036ea80: 52800268    	mov	w8, #0x13               ; =19
10036ea84: a91983e8    	stp	x8, x0, [sp, #0x198]
10036ea88: f900d7e8    	str	x8, [sp, #0x1a8]
10036ea8c: 9106c3e8    	add	x8, sp, #0x1b0
10036ea90: 910403e0    	add	x0, sp, #0x100
10036ea94: 910483e1    	add	x1, sp, #0x120
10036ea98: 940f9f68    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036ea9c: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036eaa0: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036eaa4: eb09011f    	cmp	x8, x9
10036eaa8: 54000201    	b.ne	0x10036eae8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1a58>
10036eaac: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036eab0: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036eab4: 8b091508    	add	x8, x8, x9, lsl #5
10036eab8: 3dc00100    	ldr	q0, [x8]
10036eabc: 3c9903a0    	stur	q0, [x29, #-0x70]
10036eac0: 3dc00500    	ldr	q0, [x8, #0x10]
10036eac4: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036eac8: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036eacc: ad000101    	stp	q1, q0, [x8]
10036ead0: 385903a8    	ldurb	w8, [x29, #-0x70]
10036ead4: 7100191f    	cmp	w8, #0x6
10036ead8: 54000200    	b.eq	0x10036eb18 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1a88>
10036eadc: d101c3a0    	sub	x0, x29, #0x70
10036eae0: 9401a2b8    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036eae4: 1400000d    	b	0x10036eb18 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1a88>
10036eae8: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036eaec: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036eaf0: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036eaf4: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036eaf8: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036eafc: f81603a8    	stur	x8, [x29, #-0xa0]
10036eb00: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036eb04: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036eb08: d101c3a8    	sub	x8, x29, #0x70
10036eb0c: d10343a0    	sub	x0, x29, #0xd0
10036eb10: d10243a1    	sub	x1, x29, #0x90
10036eb14: 940a7cfb    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036eb18: f9408be8    	ldr	x8, [sp, #0x110]
10036eb1c: 3dc043e0    	ldr	q0, [sp, #0x100]
10036eb20: 3c807300    	stur	q0, [x24, #0x7]
10036eb24: f8017308    	stur	x8, [x24, #0x17]
10036eb28: 528000a8    	mov	w8, #0x5                ; =5
10036eb2c: 390002a8    	strb	w8, [x21]
10036eb30: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036eb34: 3c8012a0    	stur	q0, [x21, #0x1]
10036eb38: 3cc0f300    	ldur	q0, [x24, #0xf]
10036eb3c: 3d8006a0    	str	q0, [x21, #0x10]
10036eb40: 52800088    	mov	w8, #0x4                ; =4
10036eb44: a9005268    	stp	x8, x20, [x19]
10036eb48: f9000a68    	str	x8, [x19, #0x10]
10036eb4c: 39006268    	strb	w8, [x19, #0x18]
10036eb50: 14000112    	b	0x10036ef98 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f08>
10036eb54: a97323a0    	ldp	x0, x8, [x29, #-0xd0]
10036eb58: 8b081001    	add	x1, x0, x8, lsl #4
10036eb5c: d10343a8    	sub	x8, x29, #0xd0
10036eb60: 94070765    	bl	0x1005308f4 <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h277eacb57b97a28bE>
10036eb64: b6f82314    	tbz	x20, #0x3f, 0x10036efc4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f34>
10036eb68: d2800017    	mov	x23, #0x0               ; =0
10036eb6c: aa1703e0    	mov	x0, x23
10036eb70: aa1403e1    	mov	x1, x20
10036eb74: 943986bc    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036eb78: 14000213    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036eb7c: 97f24a7e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036eb80: 52800080    	mov	w0, #0x4                ; =4
10036eb84: 52800021    	mov	w1, #0x1                ; =1
10036eb88: 97f24a77    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036eb8c: b4002fa0    	cbz	x0, 0x10036f180 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x20f0>
10036eb90: aa0003f4    	mov	x20, x0
10036eb94: 528d2d08    	mov	w8, #0x6968             ; =26984
10036eb98: 72ad0ce8    	movk	w8, #0x6867, lsl #16
10036eb9c: b9000008    	str	w8, [x0]
10036eba0: 97f24a75    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036eba4: 52800800    	mov	w0, #0x40               ; =64
10036eba8: 52800101    	mov	w1, #0x8                ; =8
10036ebac: 97f24a6e    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036ebb0: b40027a0    	cbz	x0, 0x10036f0a4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2014>
10036ebb4: aa0003f5    	mov	x21, x0
10036ebb8: f81703bf    	stur	xzr, [x29, #-0x90]
10036ebbc: f81803bf    	stur	xzr, [x29, #-0x80]
10036ebc0: 97f24a6d    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036ebc4: 528000d7    	mov	w23, #0x6               ; =6
10036ebc8: 528000c0    	mov	w0, #0x6                ; =6
10036ebcc: 52800021    	mov	w1, #0x1                ; =1
10036ebd0: 97f24a65    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036ebd4: b4003ea0    	cbz	x0, 0x10036f3a8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2318>
10036ebd8: aa0003f6    	mov	x22, x0
10036ebdc: 528e8e48    	mov	w8, #0x7472             ; =29810
10036ebe0: 79000808    	strh	w8, [x0, #0x4]
10036ebe4: 528ccca8    	mov	w8, #0x6665             ; =26213
10036ebe8: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036ebec: b9000008    	str	w8, [x0]
10036ebf0: 528000c8    	mov	w8, #0x6                ; =6
10036ebf4: a93903a8    	stp	x8, x0, [x29, #-0x70]
10036ebf8: f81a03a8    	stur	x8, [x29, #-0x60]
10036ebfc: b000a560    	adrp	x0, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036ec00: 91182000    	add	x0, x0, #0x608
10036ec04: d10343a8    	sub	x8, x29, #0xd0
10036ec08: 9416b536    	bl	0x10091c0e0 <__ZN10serde_json5value8to_value17h916a5a6325909cd5E>
10036ec0c: 385303a8    	ldurb	w8, [x29, #-0xd0]
10036ec10: 7100191f    	cmp	w8, #0x6
10036ec14: 54002500    	b.eq	0x10036f0b4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2024>
10036ec18: ad7987a0    	ldp	q0, q1, [x29, #-0xd0]
10036ec1c: ad0d87e0    	stp	q0, q1, [sp, #0x1b0]
10036ec20: d10343a8    	sub	x8, x29, #0xd0
10036ec24: d10243a0    	sub	x0, x29, #0x90
10036ec28: d101c3a1    	sub	x1, x29, #0x70
10036ec2c: 9106c3e2    	add	x2, sp, #0x1b0
10036ec30: 940fa16c    	bl	0x1007571e0 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$6insert17h36f796a8ea22555aE>
10036ec34: 385303a8    	ldurb	w8, [x29, #-0xd0]
10036ec38: 7100191f    	cmp	w8, #0x6
10036ec3c: 54000060    	b.eq	0x10036ec48 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1bb8>
10036ec40: d10343a0    	sub	x0, x29, #0xd0
10036ec44: 9401a25f    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036ec48: 97f24a4b    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036ec4c: 52800177    	mov	w23, #0xb               ; =11
10036ec50: 52800160    	mov	w0, #0xb                ; =11
10036ec54: 52800021    	mov	w1, #0x1                ; =1
10036ec58: 97f24a43    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036ec5c: b4003a60    	cbz	x0, 0x10036f3a8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2318>
10036ec60: aa0003f6    	mov	x22, x0
10036ec64: 528d2e88    	mov	w8, #0x6974             ; =26996
10036ec68: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036ec6c: b8007008    	stur	w8, [x0, #0x7]
10036ec70: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036ec74: 91092908    	add	x8, x8, #0x24a
10036ec78: f9400119    	ldr	x25, [x8]
10036ec7c: f9000019    	str	x25, [x0]
10036ec80: 52800168    	mov	w8, #0xb                ; =11
10036ec84: a93903a8    	stp	x8, x0, [x29, #-0x70]
10036ec88: f81a03a8    	stur	x8, [x29, #-0x60]
10036ec8c: b000a560    	adrp	x0, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036ec90: 911a4000    	add	x0, x0, #0x690
10036ec94: d10343a8    	sub	x8, x29, #0xd0
10036ec98: 9416b512    	bl	0x10091c0e0 <__ZN10serde_json5value8to_value17h916a5a6325909cd5E>
10036ec9c: 385303a8    	ldurb	w8, [x29, #-0xd0]
10036eca0: 7100191f    	cmp	w8, #0x6
10036eca4: 54002200    	b.eq	0x10036f0e4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2054>
10036eca8: ad7987a0    	ldp	q0, q1, [x29, #-0xd0]
10036ecac: ad0d87e0    	stp	q0, q1, [sp, #0x1b0]
10036ecb0: d10343a8    	sub	x8, x29, #0xd0
10036ecb4: d10243a0    	sub	x0, x29, #0x90
10036ecb8: d101c3a1    	sub	x1, x29, #0x70
10036ecbc: 9106c3e2    	add	x2, sp, #0x1b0
10036ecc0: 940fa148    	bl	0x1007571e0 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$6insert17h36f796a8ea22555aE>
10036ecc4: 385303a8    	ldurb	w8, [x29, #-0xd0]
10036ecc8: 7100191f    	cmp	w8, #0x6
10036eccc: 54000060    	b.eq	0x10036ecd8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1c48>
10036ecd0: d10343a0    	sub	x0, x29, #0xd0
10036ecd4: 9401a23b    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036ecd8: 3cd703a0    	ldur	q0, [x29, #-0x90]
10036ecdc: 3c868300    	stur	q0, [x24, #0x68]
10036ece0: f85803a8    	ldur	x8, [x29, #-0x80]
10036ece4: f81a83a8    	stur	x8, [x29, #-0x58]
10036ece8: 528000a8    	mov	w8, #0x5                ; =5
10036ecec: 381903a8    	sturb	w8, [x29, #-0x70]
10036ecf0: f900cbff    	str	xzr, [sp, #0x190]
10036ecf4: f900d3ff    	str	xzr, [sp, #0x1a0]
10036ecf8: 97f24a1f    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036ecfc: 528000d7    	mov	w23, #0x6               ; =6
10036ed00: 528000c0    	mov	w0, #0x6                ; =6
10036ed04: 52800021    	mov	w1, #0x1                ; =1
10036ed08: 97f24a17    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036ed0c: b4003560    	cbz	x0, 0x10036f3b8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2328>
10036ed10: aa0003f6    	mov	x22, x0
10036ed14: 528e8e48    	mov	w8, #0x7472             ; =29810
10036ed18: 79000808    	strh	w8, [x0, #0x4]
10036ed1c: 528ccca8    	mov	w8, #0x6665             ; =26213
10036ed20: 72adecc8    	movk	w8, #0x6f66, lsl #16
10036ed24: b9000008    	str	w8, [x0]
10036ed28: 528000c8    	mov	w8, #0x6                ; =6
10036ed2c: a93703a8    	stp	x8, x0, [x29, #-0x90]
10036ed30: f81803a8    	stur	x8, [x29, #-0x80]
10036ed34: b000a560    	adrp	x0, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036ed38: 91186000    	add	x0, x0, #0x618
10036ed3c: d10343a8    	sub	x8, x29, #0xd0
10036ed40: 9416b4e8    	bl	0x10091c0e0 <__ZN10serde_json5value8to_value17h916a5a6325909cd5E>
10036ed44: 385303a8    	ldurb	w8, [x29, #-0xd0]
10036ed48: 7100191f    	cmp	w8, #0x6
10036ed4c: 54001e40    	b.eq	0x10036f114 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2084>
10036ed50: ad7987a0    	ldp	q0, q1, [x29, #-0xd0]
10036ed54: ad0d87e0    	stp	q0, q1, [sp, #0x1b0]
10036ed58: d10343a8    	sub	x8, x29, #0xd0
10036ed5c: 910643e0    	add	x0, sp, #0x190
10036ed60: d10243a1    	sub	x1, x29, #0x90
10036ed64: 9106c3e2    	add	x2, sp, #0x1b0
10036ed68: 940fa11e    	bl	0x1007571e0 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$6insert17h36f796a8ea22555aE>
10036ed6c: 385303a8    	ldurb	w8, [x29, #-0xd0]
10036ed70: 7100191f    	cmp	w8, #0x6
10036ed74: 54000060    	b.eq	0x10036ed80 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1cf0>
10036ed78: d10343a0    	sub	x0, x29, #0xd0
10036ed7c: 9401a211    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036ed80: 97f249fd    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036ed84: 52800177    	mov	w23, #0xb               ; =11
10036ed88: 52800160    	mov	w0, #0xb                ; =11
10036ed8c: 52800021    	mov	w1, #0x1                ; =1
10036ed90: 97f249f5    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036ed94: b4003120    	cbz	x0, 0x10036f3b8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2328>
10036ed98: aa0003f6    	mov	x22, x0
10036ed9c: 528d2e88    	mov	w8, #0x6974             ; =26996
10036eda0: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036eda4: b8007008    	stur	w8, [x0, #0x7]
10036eda8: f9000019    	str	x25, [x0]
10036edac: 52800168    	mov	w8, #0xb                ; =11
10036edb0: a93703a8    	stp	x8, x0, [x29, #-0x90]
10036edb4: f81803a8    	stur	x8, [x29, #-0x80]
10036edb8: b000a560    	adrp	x0, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036edbc: 911a8000    	add	x0, x0, #0x6a0
10036edc0: d10343a8    	sub	x8, x29, #0xd0
10036edc4: 9416b4c7    	bl	0x10091c0e0 <__ZN10serde_json5value8to_value17h916a5a6325909cd5E>
10036edc8: 385303a8    	ldurb	w8, [x29, #-0xd0]
10036edcc: 7100191f    	cmp	w8, #0x6
10036edd0: 54001ba0    	b.eq	0x10036f144 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x20b4>
10036edd4: ad7987a0    	ldp	q0, q1, [x29, #-0xd0]
10036edd8: ad0d87e0    	stp	q0, q1, [sp, #0x1b0]
10036eddc: d10343a8    	sub	x8, x29, #0xd0
10036ede0: 910643e0    	add	x0, sp, #0x190
10036ede4: d10243a1    	sub	x1, x29, #0x90
10036ede8: 9106c3e2    	add	x2, sp, #0x1b0
10036edec: 940fa0fd    	bl	0x1007571e0 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$6insert17h36f796a8ea22555aE>
10036edf0: 385303a8    	ldurb	w8, [x29, #-0xd0]
10036edf4: 7100191f    	cmp	w8, #0x6
10036edf8: 54000060    	b.eq	0x10036ee04 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1d74>
10036edfc: d10343a0    	sub	x0, x29, #0xd0
10036ee00: 9401a1f0    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036ee04: f940d3e8    	ldr	x8, [sp, #0x1a0]
10036ee08: 3dc067e0    	ldr	q0, [sp, #0x190]
10036ee0c: 3c807300    	stur	q0, [x24, #0x7]
10036ee10: f8017308    	stur	x8, [x24, #0x17]
10036ee14: ad7c87a0    	ldp	q0, q1, [x29, #-0x70]
10036ee18: 17fffab2    	b	0x10036d8e0 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x850>
10036ee1c: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036ee20: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036ee24: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036ee28: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036ee2c: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036ee30: f81603a8    	stur	x8, [x29, #-0xa0]
10036ee34: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036ee38: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036ee3c: d101c3a8    	sub	x8, x29, #0x70
10036ee40: d10343a0    	sub	x0, x29, #0xd0
10036ee44: d10243a1    	sub	x1, x29, #0x90
10036ee48: 940a7c2e    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036ee4c: 97f249ca    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036ee50: 52800177    	mov	w23, #0xb               ; =11
10036ee54: 52800160    	mov	w0, #0xb                ; =11
10036ee58: 52800021    	mov	w1, #0x1                ; =1
10036ee5c: 97f249c2    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036ee60: b4002840    	cbz	x0, 0x10036f368 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x22d8>
10036ee64: aa0003f6    	mov	x22, x0
10036ee68: 528d2e88    	mov	w8, #0x6974             ; =26996
10036ee6c: 72adcde8    	movk	w8, #0x6e6f, lsl #16
10036ee70: b8007008    	stur	w8, [x0, #0x7]
10036ee74: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036ee78: 91092908    	add	x8, x8, #0x24a
10036ee7c: f9400108    	ldr	x8, [x8]
10036ee80: f9000008    	str	x8, [x0]
10036ee84: 52800168    	mov	w8, #0xb                ; =11
10036ee88: a91203e8    	stp	x8, x0, [sp, #0x120]
10036ee8c: f9009be8    	str	x8, [sp, #0x130]
10036ee90: 97f249b9    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036ee94: 528003a0    	mov	w0, #0x1d               ; =29
10036ee98: 52800021    	mov	w1, #0x1                ; =1
10036ee9c: 97f249b2    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036eea0: b40027c0    	cbz	x0, 0x10036f398 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2308>
10036eea4: d0007ce8    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036eea8: 91185108    	add	x8, x8, #0x614
10036eeac: 3dc00100    	ldr	q0, [x8]
10036eeb0: 3d800000    	str	q0, [x0]
10036eeb4: 3cc0d100    	ldur	q0, [x8, #0xd]
10036eeb8: 3c80d000    	stur	q0, [x0, #0xd]
10036eebc: 52800068    	mov	w8, #0x3                ; =3
10036eec0: 390643e8    	strb	w8, [sp, #0x190]
10036eec4: 528003a8    	mov	w8, #0x1d               ; =29
10036eec8: a91983e8    	stp	x8, x0, [sp, #0x198]
10036eecc: f900d7e8    	str	x8, [sp, #0x1a8]
10036eed0: 9106c3e8    	add	x8, sp, #0x1b0
10036eed4: 910403e0    	add	x0, sp, #0x100
10036eed8: 910483e1    	add	x1, sp, #0x120
10036eedc: 940f9e57    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036eee0: f940dbe8    	ldr	x8, [sp, #0x1b0]
10036eee4: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036eee8: eb09011f    	cmp	x8, x9
10036eeec: 54000201    	b.ne	0x10036ef2c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1e9c>
10036eef0: f940dfe8    	ldr	x8, [sp, #0x1b8]
10036eef4: f940e7e9    	ldr	x9, [sp, #0x1c8]
10036eef8: 8b091508    	add	x8, x8, x9, lsl #5
10036eefc: 3dc00100    	ldr	q0, [x8]
10036ef00: 3c9903a0    	stur	q0, [x29, #-0x70]
10036ef04: 3dc00500    	ldr	q0, [x8, #0x10]
10036ef08: 3c9a03a0    	stur	q0, [x29, #-0x60]
10036ef0c: ad4c83e1    	ldp	q1, q0, [sp, #0x190]
10036ef10: ad000101    	stp	q1, q0, [x8]
10036ef14: 385903a8    	ldurb	w8, [x29, #-0x70]
10036ef18: 7100191f    	cmp	w8, #0x6
10036ef1c: 54000200    	b.eq	0x10036ef5c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1ecc>
10036ef20: d101c3a0    	sub	x0, x29, #0x70
10036ef24: 9401a1a7    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036ef28: 1400000d    	b	0x10036ef5c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1ecc>
10036ef2c: ad4d87e0    	ldp	q0, q1, [sp, #0x1b0]
10036ef30: ad3987a0    	stp	q0, q1, [x29, #-0xd0]
10036ef34: 3dc077e0    	ldr	q0, [sp, #0x1d0]
10036ef38: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036ef3c: f940f3e8    	ldr	x8, [sp, #0x1e0]
10036ef40: f81603a8    	stur	x8, [x29, #-0xa0]
10036ef44: ad4c87e0    	ldp	q0, q1, [sp, #0x190]
10036ef48: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
10036ef4c: d101c3a8    	sub	x8, x29, #0x70
10036ef50: d10343a0    	sub	x0, x29, #0xd0
10036ef54: d10243a1    	sub	x1, x29, #0x90
10036ef58: 940a7bea    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036ef5c: f9408be8    	ldr	x8, [sp, #0x110]
10036ef60: 3dc043e0    	ldr	q0, [sp, #0x100]
10036ef64: 3c807300    	stur	q0, [x24, #0x7]
10036ef68: f8017308    	stur	x8, [x24, #0x17]
10036ef6c: 528000a8    	mov	w8, #0x5                ; =5
10036ef70: 390002a8    	strb	w8, [x21]
10036ef74: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036ef78: 3c8012a0    	stur	q0, [x21, #0x1]
10036ef7c: 3cc0f300    	ldur	q0, [x24, #0xf]
10036ef80: 3d8006a0    	str	q0, [x21, #0x10]
10036ef84: 528000c8    	mov	w8, #0x6                ; =6
10036ef88: a9005268    	stp	x8, x20, [x19]
10036ef8c: f9000a68    	str	x8, [x19, #0x10]
10036ef90: 52800088    	mov	w8, #0x4                ; =4
10036ef94: 39006268    	strb	w8, [x19, #0x18]
10036ef98: 52800028    	mov	w8, #0x1                ; =1
10036ef9c: a9025668    	stp	x8, x21, [x19, #0x20]
10036efa0: f9001a68    	str	x8, [x19, #0x30]
10036efa4: 9109c3ff    	add	sp, sp, #0x270
10036efa8: a9457bfd    	ldp	x29, x30, [sp, #0x50]
10036efac: a9444ff4    	ldp	x20, x19, [sp, #0x40]
10036efb0: a94357f6    	ldp	x22, x21, [sp, #0x30]
10036efb4: a9425ff8    	ldp	x24, x23, [sp, #0x20]
10036efb8: a94167fa    	ldp	x26, x25, [sp, #0x10]
10036efbc: a8c66ffc    	ldp	x28, x27, [sp], #0x60
10036efc0: d65f03c0    	ret
10036efc4: b4000134    	cbz	x20, 0x10036efe8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f58>
10036efc8: 97f2496b    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036efcc: 52800037    	mov	w23, #0x1               ; =1
10036efd0: aa1403e0    	mov	x0, x20
10036efd4: 52800021    	mov	w1, #0x1                ; =1
10036efd8: 97f24963    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036efdc: b4ffdc80    	cbz	x0, 0x10036eb6c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1adc>
10036efe0: aa0003f6    	mov	x22, x0
10036efe4: 14000002    	b	0x10036efec <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f5c>
10036efe8: 52800036    	mov	w22, #0x1               ; =1
10036efec: aa1603e0    	mov	x0, x22
10036eff0: aa1503e1    	mov	x1, x21
10036eff4: aa1403e2    	mov	x2, x20
10036eff8: 943989ad    	bl	0x1011d16ac <_writev+0x1011d16ac>
10036effc: f85403a8    	ldur	x8, [x29, #-0xc0]
10036f000: f8017328    	stur	x8, [x25, #0x17]
10036f004: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036f008: 3c807320    	stur	q0, [x25, #0x7]
10036f00c: a9005a74    	stp	x20, x22, [x19]
10036f010: f9000a74    	str	x20, [x19, #0x10]
10036f014: 52800088    	mov	w8, #0x4                ; =4
10036f018: 39006268    	strb	w8, [x19, #0x18]
10036f01c: 3dc00320    	ldr	q0, [x25]
10036f020: 3c819260    	stur	q0, [x19, #0x19]
10036f024: 3cc0f320    	ldur	q0, [x25, #0xf]
10036f028: 3c828260    	stur	q0, [x19, #0x28]
10036f02c: 17ffffde    	b	0x10036efa4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f14>
10036f030: a9005e74    	stp	x20, x23, [x19]
10036f034: f9000a75    	str	x21, [x19, #0x10]
10036f038: 52800088    	mov	w8, #0x4                ; =4
10036f03c: 39006268    	strb	w8, [x19, #0x18]
10036f040: 3cc1f320    	ldur	q0, [x25, #0x1f]
10036f044: 3c819260    	stur	q0, [x19, #0x19]
10036f048: 3cc2e320    	ldur	q0, [x25, #0x2e]
10036f04c: 3c828260    	stur	q0, [x19, #0x28]
10036f050: 17ffffd5    	b	0x10036efa4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x1f14>
10036f054: 52800100    	mov	w0, #0x8                ; =8
10036f058: 52800801    	mov	w1, #0x40               ; =64
10036f05c: 9439857c    	bl	0x1011d064c <__ZN5alloc5alloc18handle_alloc_error17h06754e1196e3be78E>
10036f060: 140000d9    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f064: 52800100    	mov	w0, #0x8                ; =8
10036f068: 52800c01    	mov	w1, #0x60               ; =96
10036f06c: 94398578    	bl	0x1011d064c <__ZN5alloc5alloc18handle_alloc_error17h06754e1196e3be78E>
10036f070: 140000d5    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f074: 52800100    	mov	w0, #0x8                ; =8
10036f078: 52801001    	mov	w1, #0x80               ; =128
10036f07c: 94398574    	bl	0x1011d064c <__ZN5alloc5alloc18handle_alloc_error17h06754e1196e3be78E>
10036f080: 140000d1    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f084: 52800100    	mov	w0, #0x8                ; =8
10036f088: 52800401    	mov	w1, #0x20               ; =32
10036f08c: 94398570    	bl	0x1011d064c <__ZN5alloc5alloc18handle_alloc_error17h06754e1196e3be78E>
10036f090: 140000cd    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f094: 52800100    	mov	w0, #0x8                ; =8
10036f098: 52800401    	mov	w1, #0x20               ; =32
10036f09c: 9439856c    	bl	0x1011d064c <__ZN5alloc5alloc18handle_alloc_error17h06754e1196e3be78E>
10036f0a0: 140000c9    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f0a4: 52800100    	mov	w0, #0x8                ; =8
10036f0a8: 52800801    	mov	w1, #0x40               ; =64
10036f0ac: 94398568    	bl	0x1011d064c <__ZN5alloc5alloc18handle_alloc_error17h06754e1196e3be78E>
10036f0b0: 140000c5    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f0b4: f85383a8    	ldur	x8, [x29, #-0xc8]
10036f0b8: f900dbe8    	str	x8, [sp, #0x1b0]
10036f0bc: f0007d00    	adrp	x0, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
10036f0c0: 9110a400    	add	x0, x0, #0x429
10036f0c4: b000a563    	adrp	x3, 0x10181c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1500+0x80>
10036f0c8: 91018063    	add	x3, x3, #0x60
10036f0cc: 9000a564    	adrp	x4, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036f0d0: 9119e084    	add	x4, x4, #0x678
10036f0d4: 9106c3e2    	add	x2, sp, #0x1b0
10036f0d8: 52800561    	mov	w1, #0x2b               ; =43
10036f0dc: 94398647    	bl	0x1011d09f8 <__ZN4core6result13unwrap_failed17hfd1628134417d023E>
10036f0e0: 140000b9    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f0e4: f85383a8    	ldur	x8, [x29, #-0xc8]
10036f0e8: f900dbe8    	str	x8, [sp, #0x1b0]
10036f0ec: f0007d00    	adrp	x0, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
10036f0f0: 9110a400    	add	x0, x0, #0x429
10036f0f4: b000a563    	adrp	x3, 0x10181c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1500+0x80>
10036f0f8: 91018063    	add	x3, x3, #0x60
10036f0fc: 9000a564    	adrp	x4, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036f100: 9119e084    	add	x4, x4, #0x678
10036f104: 9106c3e2    	add	x2, sp, #0x1b0
10036f108: 52800561    	mov	w1, #0x2b               ; =43
10036f10c: 9439863b    	bl	0x1011d09f8 <__ZN4core6result13unwrap_failed17hfd1628134417d023E>
10036f110: 140000ad    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f114: f85383a8    	ldur	x8, [x29, #-0xc8]
10036f118: f900dbe8    	str	x8, [sp, #0x1b0]
10036f11c: f0007d00    	adrp	x0, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
10036f120: 9110a400    	add	x0, x0, #0x429
10036f124: b000a563    	adrp	x3, 0x10181c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1500+0x80>
10036f128: 91018063    	add	x3, x3, #0x60
10036f12c: 9000a564    	adrp	x4, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036f130: 9119e084    	add	x4, x4, #0x678
10036f134: 9106c3e2    	add	x2, sp, #0x1b0
10036f138: 52800561    	mov	w1, #0x2b               ; =43
10036f13c: 9439862f    	bl	0x1011d09f8 <__ZN4core6result13unwrap_failed17hfd1628134417d023E>
10036f140: 140000a1    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f144: f85383a8    	ldur	x8, [x29, #-0xc8]
10036f148: f900dbe8    	str	x8, [sp, #0x1b0]
10036f14c: f0007d00    	adrp	x0, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
10036f150: 9110a400    	add	x0, x0, #0x429
10036f154: b000a563    	adrp	x3, 0x10181c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1500+0x80>
10036f158: 91018063    	add	x3, x3, #0x60
10036f15c: 9000a564    	adrp	x4, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036f160: 9119e084    	add	x4, x4, #0x678
10036f164: 9106c3e2    	add	x2, sp, #0x1b0
10036f168: 52800561    	mov	w1, #0x2b               ; =43
10036f16c: 94398623    	bl	0x1011d09f8 <__ZN4core6result13unwrap_failed17hfd1628134417d023E>
10036f170: 14000095    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f174: f85403a1    	ldur	x1, [x29, #-0xc0]
10036f178: aa1403e0    	mov	x0, x20
10036f17c: 9439853a    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f180: 52800020    	mov	w0, #0x1                ; =1
10036f184: 52800081    	mov	w1, #0x4                ; =4
10036f188: 94398537    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f18c: 52800020    	mov	w0, #0x1                ; =1
10036f190: aa1703e1    	mov	x1, x23
10036f194: 94398534    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f198: 1400008b    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f19c: 52800020    	mov	w0, #0x1                ; =1
10036f1a0: aa1703e1    	mov	x1, x23
10036f1a4: 94398530    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f1a8: 14000087    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f1ac: 52800020    	mov	w0, #0x1                ; =1
10036f1b0: aa1703e1    	mov	x1, x23
10036f1b4: 9439852c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f1b8: 14000083    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f1bc: 52800020    	mov	w0, #0x1                ; =1
10036f1c0: aa1703e1    	mov	x1, x23
10036f1c4: 94398528    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f1c8: 1400007f    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f1cc: 52800020    	mov	w0, #0x1                ; =1
10036f1d0: aa1703e1    	mov	x1, x23
10036f1d4: 94398524    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f1d8: 1400007b    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f1dc: 52800020    	mov	w0, #0x1                ; =1
10036f1e0: 52800081    	mov	w1, #0x4                ; =4
10036f1e4: 94398520    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f1e8: 14000077    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f1ec: 52800020    	mov	w0, #0x1                ; =1
10036f1f0: 52800181    	mov	w1, #0xc                ; =12
10036f1f4: 9439851c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f1f8: 14000073    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f1fc: 52800020    	mov	w0, #0x1                ; =1
10036f200: 52800081    	mov	w1, #0x4                ; =4
10036f204: 94398518    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f208: 1400006f    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f20c: 52800020    	mov	w0, #0x1                ; =1
10036f210: 528002a1    	mov	w1, #0x15               ; =21
10036f214: 94398514    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f218: 1400006b    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f21c: 52800020    	mov	w0, #0x1                ; =1
10036f220: 52800061    	mov	w1, #0x3                ; =3
10036f224: 94398510    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f228: 14000067    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f22c: 52800020    	mov	w0, #0x1                ; =1
10036f230: 528004a1    	mov	w1, #0x25               ; =37
10036f234: 9439850c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f238: 14000063    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f23c: 52800020    	mov	w0, #0x1                ; =1
10036f240: 52800081    	mov	w1, #0x4                ; =4
10036f244: 94398508    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f248: 1400005f    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f24c: 52800020    	mov	w0, #0x1                ; =1
10036f250: 528005e1    	mov	w1, #0x2f               ; =47
10036f254: 94398504    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f258: 1400005b    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f25c: 52800020    	mov	w0, #0x1                ; =1
10036f260: 52800061    	mov	w1, #0x3                ; =3
10036f264: 94398500    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f268: 14000057    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f26c: 52800020    	mov	w0, #0x1                ; =1
10036f270: 52800601    	mov	w1, #0x30               ; =48
10036f274: 943984fc    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f278: 14000053    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f27c: 52800020    	mov	w0, #0x1                ; =1
10036f280: aa1703e1    	mov	x1, x23
10036f284: 943984f8    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f288: 1400004f    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f28c: 52800020    	mov	w0, #0x1                ; =1
10036f290: aa1703e1    	mov	x1, x23
10036f294: 943984f4    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f298: 1400004b    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f29c: 52800020    	mov	w0, #0x1                ; =1
10036f2a0: aa1703e1    	mov	x1, x23
10036f2a4: 943984f0    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f2a8: 14000047    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f2ac: 52800020    	mov	w0, #0x1                ; =1
10036f2b0: aa1703e1    	mov	x1, x23
10036f2b4: 943984ec    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f2b8: 14000043    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f2bc: 52800020    	mov	w0, #0x1                ; =1
10036f2c0: 528000c1    	mov	w1, #0x6                ; =6
10036f2c4: 943984e8    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f2c8: 52800020    	mov	w0, #0x1                ; =1
10036f2cc: aa1703e1    	mov	x1, x23
10036f2d0: 943984e5    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f2d4: 1400003c    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f2d8: 52800020    	mov	w0, #0x1                ; =1
10036f2dc: 52800061    	mov	w1, #0x3                ; =3
10036f2e0: 943984e1    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f2e4: 14000038    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f2e8: 52800020    	mov	w0, #0x1                ; =1
10036f2ec: 528004a1    	mov	w1, #0x25               ; =37
10036f2f0: 943984dd    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f2f4: 14000034    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f2f8: 52800020    	mov	w0, #0x1                ; =1
10036f2fc: 528000c1    	mov	w1, #0x6                ; =6
10036f300: 943984d9    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f304: 14000030    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f308: 52800020    	mov	w0, #0x1                ; =1
10036f30c: 528006a1    	mov	w1, #0x35               ; =53
10036f310: 943984d5    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f314: 1400002c    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f318: 52800020    	mov	w0, #0x1                ; =1
10036f31c: 52800081    	mov	w1, #0x4                ; =4
10036f320: 943984d1    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f324: 14000028    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f328: 52800020    	mov	w0, #0x1                ; =1
10036f32c: 52800581    	mov	w1, #0x2c               ; =44
10036f330: 943984cd    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f334: 14000024    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f338: 52800020    	mov	w0, #0x1                ; =1
10036f33c: 528000a1    	mov	w1, #0x5                ; =5
10036f340: 943984c9    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f344: 14000020    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f348: 52800020    	mov	w0, #0x1                ; =1
10036f34c: 528005e1    	mov	w1, #0x2f               ; =47
10036f350: 943984c5    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f354: 1400001c    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f358: 52800020    	mov	w0, #0x1                ; =1
10036f35c: 52800081    	mov	w1, #0x4                ; =4
10036f360: 943984c1    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f364: 14000018    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f368: 52800020    	mov	w0, #0x1                ; =1
10036f36c: aa1703e1    	mov	x1, x23
10036f370: 943984bd    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f374: 14000014    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f378: 52800020    	mov	w0, #0x1                ; =1
10036f37c: 52800261    	mov	w1, #0x13               ; =19
10036f380: 943984b9    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f384: 14000010    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f388: 52800020    	mov	w0, #0x1                ; =1
10036f38c: 528000c1    	mov	w1, #0x6                ; =6
10036f390: 943984b5    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f394: 1400000c    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f398: 52800020    	mov	w0, #0x1                ; =1
10036f39c: 528003a1    	mov	w1, #0x1d               ; =29
10036f3a0: 943984b1    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f3a4: 14000008    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f3a8: 52800020    	mov	w0, #0x1                ; =1
10036f3ac: aa1703e1    	mov	x1, x23
10036f3b0: 943984ad    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f3b4: 14000004    	b	0x10036f3c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2334>
10036f3b8: 52800020    	mov	w0, #0x1                ; =1
10036f3bc: aa1703e1    	mov	x1, x23
10036f3c0: 943984a9    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036f3c4: d4200020    	brk	#0x1
10036f3c8: aa0003f3    	mov	x19, x0
10036f3cc: aa1703e0    	mov	x0, x23
10036f3d0: aa1403e1    	mov	x1, x20
10036f3d4: 52800022    	mov	w2, #0x1                ; =1
10036f3d8: 97f24864    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036f3dc: aa1303e0    	mov	x0, x19
10036f3e0: 9439875d    	bl	0x1011d1154 <_writev+0x1011d1154>
10036f3e4: 14000024    	b	0x10036f474 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x23e4>
10036f3e8: aa0003f3    	mov	x19, x0
10036f3ec: 1400014b    	b	0x10036f918 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2888>
10036f3f0: 14000021    	b	0x10036f474 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x23e4>
10036f3f4: aa0003f3    	mov	x19, x0
10036f3f8: 1400014f    	b	0x10036f934 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x28a4>
10036f3fc: 14000020    	b	0x10036f47c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x23ec>
10036f400: aa0003f3    	mov	x19, x0
10036f404: 14000159    	b	0x10036f968 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x28d8>
10036f408: 1400001d    	b	0x10036f47c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x23ec>
10036f40c: 1400003a    	b	0x10036f4f4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2464>
10036f410: 14000039    	b	0x10036f4f4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2464>
10036f414: aa0003f3    	mov	x19, x0
10036f418: 1400015b    	b	0x10036f984 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x28f4>
10036f41c: 14000092    	b	0x10036f664 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x25d4>
10036f420: aa0003f3    	mov	x19, x0
10036f424: 910643e0    	add	x0, sp, #0x190
10036f428: 940f57d0    	bl	0x100745368 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036f42c: 14000033    	b	0x10036f4f8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2468>
10036f430: 943985c6    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
10036f434: aa0003f3    	mov	x19, x0
10036f438: aa1603e0    	mov	x0, x22
10036f43c: 52800161    	mov	w1, #0xb                ; =11
10036f440: 1400000a    	b	0x10036f468 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x23d8>
10036f444: 14000088    	b	0x10036f664 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x25d4>
10036f448: aa0003f3    	mov	x19, x0
10036f44c: 910643e0    	add	x0, sp, #0x190
10036f450: 940f57c6    	bl	0x100745368 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036f454: 14000029    	b	0x10036f4f8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2468>
10036f458: 943985bc    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
10036f45c: aa0003f3    	mov	x19, x0
10036f460: aa1603e0    	mov	x0, x22
10036f464: 528000c1    	mov	w1, #0x6                ; =6
10036f468: 52800022    	mov	w2, #0x1                ; =1
10036f46c: 97f2483f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036f470: 14000022    	b	0x10036f4f8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2468>
10036f474: aa0003f3    	mov	x19, x0
10036f478: 14000133    	b	0x10036f944 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x28b4>
10036f47c: aa0003f3    	mov	x19, x0
10036f480: 14000145    	b	0x10036f994 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2904>
10036f484: 140000be    	b	0x10036f77c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x26ec>
10036f488: 140000bd    	b	0x10036f77c <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x26ec>
10036f48c: 140000c2    	b	0x10036f794 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2704>
10036f490: 140000c1    	b	0x10036f794 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2704>
10036f494: 140000c6    	b	0x10036f7ac <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x271c>
10036f498: 140000c5    	b	0x10036f7ac <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x271c>
10036f49c: 140000ca    	b	0x10036f7c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2734>
10036f4a0: 140000c9    	b	0x10036f7c4 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2734>
10036f4a4: aa0003f3    	mov	x19, x0
10036f4a8: 910643e0    	add	x0, sp, #0x190
10036f4ac: 940f57af    	bl	0x100745368 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036f4b0: 1400006e    	b	0x10036f668 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x25d8>
10036f4b4: 943985a5    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
10036f4b8: aa0003f3    	mov	x19, x0
10036f4bc: aa1603e0    	mov	x0, x22
10036f4c0: 52800161    	mov	w1, #0xb                ; =11
10036f4c4: 14000009    	b	0x10036f4e8 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x2458>
10036f4c8: aa0003f3    	mov	x19, x0
10036f4cc: 910643e0    	add	x0, sp, #0x190
10036f4d0: 940f57a6    	bl	0x100745368 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036f4d4: 14000065    	b	0x10036f668 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x25d8>
10036f4d8: 9439859c    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
10036f4dc: aa0003f3    	mov	x19, x0
10036f4e0: aa1603e0    	mov	x0, x22
10036f4e4: 528000c1    	mov	w1, #0x6                ; =6
10036f4e8: 52800022    	mov	w2, #0x1                ; =1
10036f4ec: 97f2481f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036f4f0: 1400005e    	b	0x10036f668 <__ZN13codexmate_lib4core5relay13codex_catalog24catalog_reasoning_levels17h32b3935ff6f7a224E+0x25d8>
10036f4f4: aa0003f3    	mov	x19, x0
10036f4f8: 910403e0    	add	x0, sp, #0x100
10036f4fc: 9410035a    	bl	0x100770264 <__ZN99_$LT$alloc..collections..btree..map..BTreeMap$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h25de00eca2f35937E>
