
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010036fa2c <__ZN13codexmate_lib4core5relay13codex_catalog24no_account_catalog_slugs17ha19591f82502f1f1E>:
10036fa2c: d10183ff    	sub	sp, sp, #0x60
10036fa30: a90357f6    	stp	x22, x21, [sp, #0x30]
10036fa34: a9044ff4    	stp	x20, x19, [sp, #0x40]
10036fa38: a9057bfd    	stp	x29, x30, [sp, #0x50]
10036fa3c: 910143fd    	add	x29, sp, #0x50
10036fa40: aa0503f4    	mov	x20, x5
10036fa44: aa0403f5    	mov	x21, x4
10036fa48: aa0803f3    	mov	x19, x8
10036fa4c: 910003e8    	mov	x8, sp
10036fa50: 9410c75f    	bl	0x1007a17cc <__ZN13codexmate_lib4core5relay6models30active_codex_relay_model_slugs17h72c809e2cf5f3b42E>
10036fa54: 52800908    	mov	w8, #0x48               ; =72
10036fa58: 9b085681    	madd	x1, x20, x8, x21
10036fa5c: 910003e2    	mov	x2, sp
10036fa60: aa1503e0    	mov	x0, x21
10036fa64: 94125167    	bl	0x100804000 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE>
10036fa68: ad4007e0    	ldp	q0, q1, [sp]
10036fa6c: ad000660    	stp	q0, q1, [x19]
10036fa70: 3dc00be0    	ldr	q0, [sp, #0x20]
10036fa74: 3d800a60    	str	q0, [x19, #0x20]
10036fa78: a9457bfd    	ldp	x29, x30, [sp, #0x50]
10036fa7c: a9444ff4    	ldp	x20, x19, [sp, #0x40]
10036fa80: a94357f6    	ldp	x22, x21, [sp, #0x30]
10036fa84: 910183ff    	add	sp, sp, #0x60
10036fa88: d65f03c0    	ret
10036fa8c: aa0003f3    	mov	x19, x0
10036fa90: 910003e0    	mov	x0, sp
10036fa94: 9413fedd    	bl	0x10086f608 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5856a37aee37aca3E>
10036fa98: aa1303e0    	mov	x0, x19
10036fa9c: 943985ae    	bl	0x1011d1154 <_writev+0x1011d1154>
10036faa0: 9439842a    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

000000010036faa4 <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E>:
10036faa4: d10583ff    	sub	sp, sp, #0x160
10036faa8: a91167fa    	stp	x26, x25, [sp, #0x110]
10036faac: a9125ff8    	stp	x24, x23, [sp, #0x120]
10036fab0: a91357f6    	stp	x22, x21, [sp, #0x130]
10036fab4: a9144ff4    	stp	x20, x19, [sp, #0x140]
10036fab8: a9157bfd    	stp	x29, x30, [sp, #0x150]
10036fabc: 910543fd    	add	x29, sp, #0x150
10036fac0: aa0203f6    	mov	x22, x2
10036fac4: aa0103f7    	mov	x23, x1
10036fac8: aa0003f3    	mov	x19, x0
10036facc: 910103e8    	add	x8, sp, #0x40
10036fad0: aa0103e0    	mov	x0, x1
10036fad4: aa0203e1    	mov	x1, x2
10036fad8: 94374add    	bl	0x10114264c <__ZN3std2fs4read5inner17hebd695181b114834E>
10036fadc: a94457f4    	ldp	x20, x21, [sp, #0x40]
10036fae0: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10036fae4: eb08029f    	cmp	x20, x8
10036fae8: 54000521    	b.ne	0x10036fb8c <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0xe8>
10036faec: f90003f5    	str	x21, [sp]
10036faf0: a9025bf7    	stp	x23, x22, [sp, #0x20]
10036faf4: 910083e8    	add	x8, sp, #0x20
10036faf8: f0006f69    	adrp	x9, 0x10115e000 <__ZN4core5slice4sort8unstable9quicksort9quicksort17h324c95ad922d2016E+0x1fc>
10036fafc: 91351129    	add	x9, x9, #0xd44
10036fb00: a90827e8    	stp	x8, x9, [sp, #0x80]
10036fb04: 910003e8    	mov	x8, sp
10036fb08: d0006fc9    	adrp	x9, 0x101169000 <__ZN5gimli4read8rnglists20RngListIter$LT$R$GT$4next17ha0add38c23e78896E+0x510>
10036fb0c: 91211129    	add	x9, x9, #0x844
10036fb10: a90927e8    	stp	x8, x9, [sp, #0x90]
10036fb14: 90008d80    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
10036fb18: 913fec00    	add	x0, x0, #0xffb
10036fb1c: 910183e8    	add	x8, sp, #0x60
10036fb20: 910203e1    	add	x1, sp, #0x80
10036fb24: 94381f5c    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10036fb28: a9465ff6    	ldp	x22, x23, [sp, #0x60]
10036fb2c: f9403bf8    	ldr	x24, [sp, #0x70]
10036fb30: f94003f4    	ldr	x20, [sp]
10036fb34: 92400688    	and	x8, x20, #0x3
10036fb38: f100051f    	cmp	x8, #0x1
10036fb3c: 54000201    	b.ne	0x10036fb7c <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0xd8>
10036fb40: f85ffe95    	ldr	x21, [x20, #-0x1]!
10036fb44: f9400699    	ldr	x25, [x20, #0x8]
10036fb48: f9400328    	ldr	x8, [x25]
10036fb4c: b4000068    	cbz	x8, 0x10036fb58 <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0xb4>
10036fb50: aa1503e0    	mov	x0, x21
10036fb54: d63f0100    	blr	x8
10036fb58: f9400721    	ldr	x1, [x25, #0x8]
10036fb5c: b4000081    	cbz	x1, 0x10036fb6c <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0xc8>
10036fb60: f9400b22    	ldr	x2, [x25, #0x10]
10036fb64: aa1503e0    	mov	x0, x21
10036fb68: 97f24680    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036fb6c: aa1403e0    	mov	x0, x20
10036fb70: 52800301    	mov	w1, #0x18               ; =24
10036fb74: 52800102    	mov	w2, #0x8                ; =8
10036fb78: 97f2467c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036fb7c: a9016277    	stp	x23, x24, [x19, #0x10]
10036fb80: 52800148    	mov	w8, #0xa                ; =10
10036fb84: a9005a68    	stp	x8, x22, [x19]
10036fb88: 1400002b    	b	0x10036fc34 <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0x190>
10036fb8c: f9402be8    	ldr	x8, [sp, #0x50]
10036fb90: a90823f5    	stp	x21, x8, [sp, #0x80]
10036fb94: a9097fff    	stp	xzr, xzr, [sp, #0x90]
10036fb98: 910183e8    	add	x8, sp, #0x60
10036fb9c: 910203e0    	add	x0, sp, #0x80
10036fba0: 97f9aba8    	bl	0x1001daa40 <__ZN10serde_json2de10from_trait17h3ae205149e9a07e1E>
10036fba4: 394183e8    	ldrb	w8, [sp, #0x60]
10036fba8: 7100191f    	cmp	w8, #0x6
10036fbac: 54000521    	b.ne	0x10036fc50 <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0x1ac>
10036fbb0: f94037e8    	ldr	x8, [sp, #0x68]
10036fbb4: f81903a8    	stur	x8, [x29, #-0x70]
10036fbb8: a93b5bb7    	stp	x23, x22, [x29, #-0x50]
10036fbbc: d10143a8    	sub	x8, x29, #0x50
10036fbc0: f0006f69    	adrp	x9, 0x10115e000 <__ZN4core5slice4sort8unstable9quicksort9quicksort17h324c95ad922d2016E+0x1fc>
10036fbc4: 91351129    	add	x9, x9, #0xd44
10036fbc8: a90827e8    	stp	x8, x9, [sp, #0x80]
10036fbcc: d101c3a8    	sub	x8, x29, #0x70
10036fbd0: 90006ce9    	adrp	x9, 0x10110b000 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0x50>
10036fbd4: 910ab129    	add	x9, x9, #0x2ac
10036fbd8: a90927e8    	stp	x8, x9, [sp, #0x90]
10036fbdc: b0008d80    	adrp	x0, 0x101520000 <_anon.164d03a2dfc98f969363eaf1c50504fd.175+0x1a1>
10036fbe0: 91014000    	add	x0, x0, #0x50
10036fbe4: d101a3a8    	sub	x8, x29, #0x68
10036fbe8: 910203e1    	add	x1, sp, #0x80
10036fbec: 94381f2a    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10036fbf0: 3cd983a0    	ldur	q0, [x29, #-0x68]
10036fbf4: 3d8023e0    	str	q0, [sp, #0x80]
10036fbf8: f85a83a8    	ldur	x8, [x29, #-0x58]
10036fbfc: f9004be8    	str	x8, [sp, #0x90]
10036fc00: d101c3a0    	sub	x0, x29, #0x70
10036fc04: 94019e36    	bl	0x1003d74dc <__ZN4core3ptr45drop_in_place$LT$serde_json..error..Error$GT$17hc34208687d70e728E>
10036fc08: ad4403e1    	ldp	q1, q0, [sp, #0x80]
10036fc0c: ad0103e1    	stp	q1, q0, [sp, #0x20]
10036fc10: 3c808261    	stur	q1, [x19, #0x8]
10036fc14: 3c818260    	stur	q0, [x19, #0x18]
10036fc18: 52800148    	mov	w8, #0xa                ; =10
10036fc1c: f9000268    	str	x8, [x19]
10036fc20: b40000b4    	cbz	x20, 0x10036fc34 <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0x190>
10036fc24: aa1503e0    	mov	x0, x21
10036fc28: aa1403e1    	mov	x1, x20
10036fc2c: 52800022    	mov	w2, #0x1                ; =1
10036fc30: 97f2464e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036fc34: a9557bfd    	ldp	x29, x30, [sp, #0x150]
10036fc38: a9544ff4    	ldp	x20, x19, [sp, #0x140]
10036fc3c: a95357f6    	ldp	x22, x21, [sp, #0x130]
10036fc40: a9525ff8    	ldp	x24, x23, [sp, #0x120]
10036fc44: a95167fa    	ldp	x26, x25, [sp, #0x110]
10036fc48: 910583ff    	add	sp, sp, #0x160
10036fc4c: d65f03c0    	ret
10036fc50: ad4303e1    	ldp	q1, q0, [sp, #0x60]
10036fc54: ad0003e1    	stp	q1, q0, [sp]
10036fc58: b0007ce0    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036fc5c: 9102ac00    	add	x0, x0, #0xab
10036fc60: 910003e2    	mov	x2, sp
10036fc64: 528000c1    	mov	w1, #0x6                ; =6
10036fc68: 94367bca    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
10036fc6c: b4000480    	cbz	x0, 0x10036fcfc <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0x258>
10036fc70: 39400008    	ldrb	w8, [x0]
10036fc74: 7100111f    	cmp	w8, #0x4
10036fc78: 54000421    	b.ne	0x10036fcfc <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0x258>
10036fc7c: a9416016    	ldp	x22, x24, [x0, #0x10]
10036fc80: d000ac00    	adrp	x0, 0x1018f1000 <__ZN3std12backtrace_rs9symbolize5gimli5Cache11with_global14MAPPINGS_CACHE17hb3a3380e67302d28E>
10036fc84: 912f6000    	add	x0, x0, #0xbd8
10036fc88: f9400008    	ldr	x8, [x0]
10036fc8c: d63f0100    	blr	x8
10036fc90: aa0003f7    	mov	x23, x0
10036fc94: 39404008    	ldrb	w8, [x0, #0x10]
10036fc98: 7100051f    	cmp	w8, #0x1
10036fc9c: 54000601    	b.ne	0x10036fd5c <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0x2b8>
10036fca0: a94006e0    	ldp	x0, x1, [x23]
10036fca4: 91000408    	add	x8, x0, #0x1
10036fca8: f90002e8    	str	x8, [x23]
10036fcac: 8b1816c8    	add	x8, x22, x24, lsl #5
10036fcb0: 9000a569    	adrp	x9, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036fcb4: 91154129    	add	x9, x9, #0x550
10036fcb8: ad400520    	ldp	q0, q1, [x9]
10036fcbc: ad0407e0    	stp	q0, q1, [sp, #0x80]
10036fcc0: a90a07e0    	stp	x0, x1, [sp, #0xa0]
10036fcc4: 910203e2    	add	x2, sp, #0x80
10036fcc8: aa1603e0    	mov	x0, x22
10036fccc: aa0803e1    	mov	x1, x8
10036fcd0: 94125f29    	bl	0x100807974 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hb19bd27610d72d29E>
10036fcd4: ad4407e0    	ldp	q0, q1, [sp, #0x80]
10036fcd8: 3c808260    	stur	q0, [x19, #0x8]
10036fcdc: 3c818261    	stur	q1, [x19, #0x18]
10036fce0: 3dc02be0    	ldr	q0, [sp, #0xa0]
10036fce4: 3c828260    	stur	q0, [x19, #0x28]
10036fce8: 52800168    	mov	w8, #0xb                ; =11
10036fcec: f9000268    	str	x8, [x19]
10036fcf0: 910003e0    	mov	x0, sp
10036fcf4: 94019e33    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036fcf8: 14000017    	b	0x10036fd54 <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E+0x2b0>
10036fcfc: a9045bf7    	stp	x23, x22, [sp, #0x40]
