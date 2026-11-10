
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100363040 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E>:
100363040: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
100363044: a90167fa    	stp	x26, x25, [sp, #0x10]
100363048: a9025ff8    	stp	x24, x23, [sp, #0x20]
10036304c: a90357f6    	stp	x22, x21, [sp, #0x30]
100363050: a9044ff4    	stp	x20, x19, [sp, #0x40]
100363054: a9057bfd    	stp	x29, x30, [sp, #0x50]
100363058: 910143fd    	add	x29, sp, #0x50
10036305c: d10983ff    	sub	sp, sp, #0x260
100363060: b9002fe2    	str	w2, [sp, #0x2c]
100363064: aa0103f8    	mov	x24, x1
100363068: aa0003f6    	mov	x22, x0
10036306c: aa0803f5    	mov	x21, x8
100363070: 52800108    	mov	w8, #0x8                ; =8
100363074: a903a3ff    	stp	xzr, x8, [sp, #0x38]
100363078: f90027ff    	str	xzr, [sp, #0x48]
10036307c: f9401c00    	ldr	x0, [x0, #0x38]
100363080: f94022c1    	ldr	x1, [x22, #0x40]
100363084: d10403a8    	sub	x8, x29, #0x100
100363088: 94377c15    	bl	0x1011420dc <__ZN3std2fs14read_to_string5inner17h0f85889ccb0352e2E>
10036308c: a97053a9    	ldp	x9, x20, [x29, #-0x100]
100363090: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100363094: eb08013f    	cmp	x9, x8
100363098: 54000621    	b.ne	0x10036315c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x11c>
10036309c: aa1403e0    	mov	x0, x20
1003630a0: 9401c03f    	bl	0x1003d319c <__ZN3std2io5error5Error4kind17h247456c019dc3d6bE>
1003630a4: 72001c1f    	tst	w0, #0xff
1003630a8: 540005e0    	b.eq	0x100363164 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x124>
1003630ac: aa1503f9    	mov	x25, x21
1003630b0: f90083f4    	str	x20, [sp, #0x100]
1003630b4: 910403e8    	add	x8, sp, #0x100
1003630b8: d0007029    	adrp	x9, 0x101169000 <__ZN5gimli4read8rnglists20RngListIter$LT$R$GT$4next17ha0add38c23e78896E+0x510>
1003630bc: 91211129    	add	x9, x9, #0x844
1003630c0: a906a7e8    	stp	x8, x9, [sp, #0x68]
1003630c4: 90008de0    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
1003630c8: 91339800    	add	x0, x0, #0xce6
1003630cc: 910143e8    	add	x8, sp, #0x50
1003630d0: 9101a3e1    	add	x1, sp, #0x68
1003630d4: 943851f0    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1003630d8: a94553f3    	ldp	x19, x20, [sp, #0x50]
1003630dc: f94033f7    	ldr	x23, [sp, #0x60]
1003630e0: f94027f5    	ldr	x21, [sp, #0x48]
1003630e4: f9401fe8    	ldr	x8, [sp, #0x38]
1003630e8: eb0802bf    	cmp	x21, x8
1003630ec: 54000061    	b.ne	0x1003630f8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xb8>
1003630f0: 9100e3e0    	add	x0, sp, #0x38
1003630f4: 9438aaef    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
1003630f8: f94023e8    	ldr	x8, [sp, #0x40]
1003630fc: 8b151508    	add	x8, x8, x21, lsl #5
100363100: a9005113    	stp	x19, x20, [x8]
100363104: f9000917    	str	x23, [x8, #0x10]
100363108: 3900611f    	strb	wzr, [x8, #0x18]
10036310c: 910006a8    	add	x8, x21, #0x1
100363110: f90027e8    	str	x8, [sp, #0x48]
100363114: f94083f3    	ldr	x19, [sp, #0x100]
100363118: 92400668    	and	x8, x19, #0x3
10036311c: f100051f    	cmp	x8, #0x1
100363120: aa1903f5    	mov	x21, x25
100363124: 54000481    	b.ne	0x1003631b4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x174>
100363128: f85ffe74    	ldr	x20, [x19, #-0x1]!
10036312c: f9400677    	ldr	x23, [x19, #0x8]
100363130: f94002e8    	ldr	x8, [x23]
100363134: b4000068    	cbz	x8, 0x100363140 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x100>
100363138: aa1403e0    	mov	x0, x20
10036313c: d63f0100    	blr	x8
100363140: f94006e1    	ldr	x1, [x23, #0x8]
100363144: b4000081    	cbz	x1, 0x100363154 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x114>
100363148: f9400ae2    	ldr	x2, [x23, #0x10]
10036314c: aa1403e0    	mov	x0, x20
100363150: 97f27906    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363154: aa1303e0    	mov	x0, x19
100363158: 14000014    	b	0x1003631a8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x168>
10036315c: f85103bb    	ldur	x27, [x29, #-0xf0]
100363160: 14000018    	b	0x1003631c0 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x180>
100363164: 92400688    	and	x8, x20, #0x3
100363168: f100051f    	cmp	x8, #0x1
10036316c: 54000241    	b.ne	0x1003631b4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x174>
100363170: aa1403e8    	mov	x8, x20
100363174: f85ffd14    	ldr	x20, [x8, #-0x1]!
100363178: aa0803f7    	mov	x23, x8
10036317c: f9400513    	ldr	x19, [x8, #0x8]
100363180: f9400268    	ldr	x8, [x19]
100363184: b4000068    	cbz	x8, 0x100363190 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x150>
100363188: aa1403e0    	mov	x0, x20
10036318c: d63f0100    	blr	x8
100363190: f9400661    	ldr	x1, [x19, #0x8]
100363194: b4000081    	cbz	x1, 0x1003631a4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x164>
100363198: f9400a62    	ldr	x2, [x19, #0x10]
10036319c: aa1403e0    	mov	x0, x20
1003631a0: 97f278f2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003631a4: aa1703e0    	mov	x0, x23
1003631a8: 52800301    	mov	w1, #0x18               ; =24
1003631ac: 52800102    	mov	w2, #0x8                ; =8
1003631b0: 97f278ee    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003631b4: d280001b    	mov	x27, #0x0               ; =0
1003631b8: d2800009    	mov	x9, #0x0                ; =0
1003631bc: 52800034    	mov	w20, #0x1               ; =1
1003631c0: f9001be9    	str	x9, [sp, #0x30]
1003631c4: 9101a3e8    	add	x8, sp, #0x68
1003631c8: aa1603e0    	mov	x0, x22
1003631cc: aa1403e1    	mov	x1, x20
1003631d0: aa1b03e2    	mov	x2, x27
1003631d4: 97fffc8c    	bl	0x100362404 <__ZN13codexmate_lib4core5relay10invariants19RouterConfigSurface5parse17hb3cbfe1e08872d0bE>
1003631d8: 910403e8    	add	x8, sp, #0x100
1003631dc: 9101a3e2    	add	x2, sp, #0x68
1003631e0: aa1603e0    	mov	x0, x22
1003631e4: aa1803e1    	mov	x1, x24
1003631e8: 9400055a    	bl	0x100364750 <__ZN13codexmate_lib4core5relay10invariants37config_surface_violations_for_surface17h37d9541dab531f6eE>
1003631ec: a95067f3    	ldp	x19, x25, [sp, #0x100]
1003631f0: f9408bfa    	ldr	x26, [sp, #0x110]
1003631f4: d37beb5c    	lsl	x28, x26, #5
1003631f8: 8b1c0328    	add	x8, x25, x28
1003631fc: a93067b9    	stp	x25, x25, [x29, #-0x100]
100363200: a93123b3    	stp	x19, x8, [x29, #-0xf0]
100363204: f94027f7    	ldr	x23, [sp, #0x48]
100363208: f9401fe8    	ldr	x8, [sp, #0x38]
10036320c: cb170108    	sub	x8, x8, x23
100363210: eb08035f    	cmp	x26, x8
100363214: 540067e8    	b.hi	0x100363f10 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xed0>
100363218: f94023e8    	ldr	x8, [sp, #0x40]
10036321c: 8b171500    	add	x0, x8, x23, lsl #5
100363220: aa1903e1    	mov	x1, x25
100363224: aa1c03e2    	mov	x2, x28
100363228: 9439b921    	bl	0x1011d16ac <_writev+0x1011d16ac>
10036322c: 8b1a02e8    	add	x8, x23, x26
100363230: f90027e8    	str	x8, [sp, #0x48]
100363234: b40000b3    	cbz	x19, 0x100363248 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x208>
100363238: d37bea61    	lsl	x1, x19, #5
10036323c: aa1903e0    	mov	x0, x25
100363240: 52800102    	mov	w2, #0x8                ; =8
100363244: 97f278c9    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363248: 34001718    	cbz	w24, 0x100363528 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x4e8>
10036324c: aa1603e0    	mov	x0, x22
100363250: 97fa4935    	bl	0x1001f5724 <__ZN13codexmate_lib4core5relay15config_takeover13backup_exists17hdb93c4a64b39b1d5E>
100363254: 36000380    	tbz	w0, #0x0, 0x1003632c4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x284>
100363258: 97f278c7    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036325c: 528004f8    	mov	w24, #0x27              ; =39
100363260: 528004e0    	mov	w0, #0x27               ; =39
100363264: 52800021    	mov	w1, #0x1                ; =1
100363268: 97f278bf    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036326c: b4006620    	cbz	x0, 0x100363f30 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xef0>
100363270: aa0003f9    	mov	x25, x0
100363274: 90007d48    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363278: 913ae508    	add	x8, x8, #0xeb9
10036327c: ad400500    	ldp	q0, q1, [x8]
100363280: ad000400    	stp	q0, q1, [x0]
100363284: f841f108    	ldur	x8, [x8, #0x1f]
100363288: f801f008    	stur	x8, [x0, #0x1f]
10036328c: f94027f3    	ldr	x19, [sp, #0x48]
100363290: f9401fe8    	ldr	x8, [sp, #0x38]
100363294: eb08027f    	cmp	x19, x8
100363298: 54000061    	b.ne	0x1003632a4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x264>
10036329c: 9100e3e0    	add	x0, sp, #0x38
1003632a0: 9438aa84    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
1003632a4: f94023e8    	ldr	x8, [sp, #0x40]
1003632a8: 8b131508    	add	x8, x8, x19, lsl #5
1003632ac: 528004e9    	mov	w9, #0x27               ; =39
1003632b0: a9006509    	stp	x9, x25, [x8]
1003632b4: f9000909    	str	x9, [x8, #0x10]
1003632b8: 3900611f    	strb	wzr, [x8, #0x18]
1003632bc: 91000668    	add	x8, x19, #0x1
1003632c0: f90027e8    	str	x8, [sp, #0x48]
1003632c4: aa1603e0    	mov	x0, x22
1003632c8: 97fa7a6e    	bl	0x100201c80 <__ZN13codexmate_lib4core5relay19legacy_virtual_auth13marker_exists17h234ad5508a490929E>
1003632cc: 36000400    	tbz	w0, #0x0, 0x10036334c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x30c>
1003632d0: 97f278a9    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003632d4: 52800698    	mov	w24, #0x34              ; =52
1003632d8: 52800680    	mov	w0, #0x34               ; =52
1003632dc: 52800021    	mov	w1, #0x1                ; =1
1003632e0: 97f278a1    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003632e4: b4006260    	cbz	x0, 0x100363f30 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xef0>
1003632e8: aa0003f9    	mov	x25, x0
1003632ec: 528d6e48    	mov	w8, #0x6b72             ; =27506
1003632f0: 72ae4ca8    	movk	w8, #0x7265, lsl #16
1003632f4: b9003008    	str	w8, [x0, #0x30]
1003632f8: 90007d48    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003632fc: 91309d08    	add	x8, x8, #0xc27
100363300: ad400500    	ldp	q0, q1, [x8]
100363304: ad000400    	stp	q0, q1, [x0]
100363308: 3dc00900    	ldr	q0, [x8, #0x20]
10036330c: 3d800800    	str	q0, [x0, #0x20]
100363310: f94027f3    	ldr	x19, [sp, #0x48]
100363314: f9401fe8    	ldr	x8, [sp, #0x38]
100363318: eb08027f    	cmp	x19, x8
10036331c: 54000061    	b.ne	0x100363328 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x2e8>
100363320: 9100e3e0    	add	x0, sp, #0x38
100363324: 9438aa63    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100363328: f94023e8    	ldr	x8, [sp, #0x40]
10036332c: 8b131508    	add	x8, x8, x19, lsl #5
100363330: 52800689    	mov	w9, #0x34               ; =52
100363334: a9006509    	stp	x9, x25, [x8]
100363338: f9000909    	str	x9, [x8, #0x10]
10036333c: 52800029    	mov	w9, #0x1                ; =1
100363340: 39006109    	strb	w9, [x8, #0x18]
100363344: 91000668    	add	x8, x19, #0x1
100363348: f90027e8    	str	x8, [sp, #0x48]
10036334c: aa1603e0    	mov	x0, x22
100363350: 94151605    	bl	0x1008a8b64 <__ZN13codexmate_lib4core5relay18router_unlock_auth15live_auth_state17hf9c8eec101776f31E>
100363354: 12001c08    	and	w8, w0, #0xff
100363358: 51000508    	sub	w8, w8, #0x1
10036335c: 7100091f    	cmp	w8, #0x2
100363360: 540003e2    	b.hs	0x1003633dc <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x39c>
100363364: 97f27884    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100363368: 52800658    	mov	w24, #0x32              ; =50
10036336c: 52800640    	mov	w0, #0x32               ; =50
100363370: 52800021    	mov	w1, #0x1                ; =1
100363374: 97f2787c    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100363378: b4005dc0    	cbz	x0, 0x100363f30 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xef0>
10036337c: aa0003f9    	mov	x25, x0
100363380: 528caec8    	mov	w8, #0x6576             ; =25974
100363384: 79006008    	strh	w8, [x0, #0x30]
100363388: 90007d48    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
10036338c: 91316d08    	add	x8, x8, #0xc5b
100363390: ad400500    	ldp	q0, q1, [x8]
100363394: ad000400    	stp	q0, q1, [x0]
100363398: 3dc00900    	ldr	q0, [x8, #0x20]
10036339c: 3d800800    	str	q0, [x0, #0x20]
1003633a0: f94027f3    	ldr	x19, [sp, #0x48]
1003633a4: f9401fe8    	ldr	x8, [sp, #0x38]
1003633a8: eb08027f    	cmp	x19, x8
1003633ac: 54000061    	b.ne	0x1003633b8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x378>
1003633b0: 9100e3e0    	add	x0, sp, #0x38
1003633b4: 9438aa3f    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
1003633b8: f94023e8    	ldr	x8, [sp, #0x40]
1003633bc: 8b131508    	add	x8, x8, x19, lsl #5
1003633c0: 52800649    	mov	w9, #0x32               ; =50
1003633c4: a9006509    	stp	x9, x25, [x8]
1003633c8: f9000909    	str	x9, [x8, #0x10]
1003633cc: 52800029    	mov	w9, #0x1                ; =1
1003633d0: 39006109    	strb	w9, [x8, #0x18]
1003633d4: 91000668    	add	x8, x19, #0x1
1003633d8: f90027e8    	str	x8, [sp, #0x48]
1003633dc: aa1603e0    	mov	x0, x22
1003633e0: 941515bc    	bl	0x1008a8ad0 <__ZN13codexmate_lib4core5relay18router_unlock_auth13marker_exists17h50c3f4cccfec5f10E>
1003633e4: 360003e0    	tbz	w0, #0x0, 0x100363460 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x420>
1003633e8: 97f27863    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003633ec: 52800658    	mov	w24, #0x32              ; =50
1003633f0: 52800640    	mov	w0, #0x32               ; =50
1003633f4: 52800021    	mov	w1, #0x1                ; =1
1003633f8: 97f2785b    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003633fc: b40059a0    	cbz	x0, 0x100363f30 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xef0>
100363400: aa0003f9    	mov	x25, x0
100363404: 528e4ca8    	mov	w8, #0x7265             ; =29285
100363408: 79006008    	strh	w8, [x0, #0x30]
10036340c: 90007d48    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363410: 91323508    	add	x8, x8, #0xc8d
100363414: ad400500    	ldp	q0, q1, [x8]
100363418: ad000400    	stp	q0, q1, [x0]
10036341c: 3dc00900    	ldr	q0, [x8, #0x20]
100363420: 3d800800    	str	q0, [x0, #0x20]
100363424: f94027f3    	ldr	x19, [sp, #0x48]
100363428: f9401fe8    	ldr	x8, [sp, #0x38]
10036342c: eb08027f    	cmp	x19, x8
100363430: 54000061    	b.ne	0x10036343c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x3fc>
100363434: 9100e3e0    	add	x0, sp, #0x38
100363438: 9438aa1e    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
10036343c: f94023e8    	ldr	x8, [sp, #0x40]
100363440: 8b131508    	add	x8, x8, x19, lsl #5
100363444: 52800649    	mov	w9, #0x32               ; =50
100363448: a9006509    	stp	x9, x25, [x8]
10036344c: f9000909    	str	x9, [x8, #0x10]
100363450: 52800029    	mov	w9, #0x1                ; =1
100363454: 39006109    	strb	w9, [x8, #0x18]
100363458: 91000668    	add	x8, x19, #0x1
10036345c: f90027e8    	str	x8, [sp, #0x48]
100363460: aa1603e0    	mov	x0, x22
100363464: 94151565    	bl	0x1008a89f8 <__ZN13codexmate_lib4core5relay18router_unlock_auth11has_residue17h150741b3202acaf4E>
100363468: 36000500    	tbz	w0, #0x0, 0x100363508 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x4c8>
10036346c: aa1603e0    	mov	x0, x22
100363470: 94151598    	bl	0x1008a8ad0 <__ZN13codexmate_lib4core5relay18router_unlock_auth13marker_exists17h50c3f4cccfec5f10E>
100363474: 370004a0    	tbnz	w0, #0x0, 0x100363508 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x4c8>
100363478: aa1603e0    	mov	x0, x22
10036347c: 941515ba    	bl	0x1008a8b64 <__ZN13codexmate_lib4core5relay18router_unlock_auth15live_auth_state17hf9c8eec101776f31E>
100363480: 12001c08    	and	w8, w0, #0xff
100363484: 51000508    	sub	w8, w8, #0x1
100363488: 7100091f    	cmp	w8, #0x2
10036348c: 540003e3    	b.lo	0x100363508 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x4c8>
100363490: 97f27839    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100363494: 528007d8    	mov	w24, #0x3e              ; =62
100363498: 528007c0    	mov	w0, #0x3e               ; =62
10036349c: 52800021    	mov	w1, #0x1                ; =1
1003634a0: 97f27831    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003634a4: b4005460    	cbz	x0, 0x100363f30 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xef0>
1003634a8: aa0003f9    	mov	x25, x0
1003634ac: 90007d48    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003634b0: 9132fd08    	add	x8, x8, #0xcbf
1003634b4: ad400500    	ldp	q0, q1, [x8]
1003634b8: ad000400    	stp	q0, q1, [x0]
1003634bc: 3dc00900    	ldr	q0, [x8, #0x20]
1003634c0: 3d800800    	str	q0, [x0, #0x20]
1003634c4: 3cc2e100    	ldur	q0, [x8, #0x2e]
1003634c8: 3c82e000    	stur	q0, [x0, #0x2e]
1003634cc: f94027f3    	ldr	x19, [sp, #0x48]
1003634d0: f9401fe8    	ldr	x8, [sp, #0x38]
1003634d4: eb08027f    	cmp	x19, x8
1003634d8: 54000061    	b.ne	0x1003634e4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x4a4>
1003634dc: 9100e3e0    	add	x0, sp, #0x38
1003634e0: 9438a9f4    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
1003634e4: f94023e8    	ldr	x8, [sp, #0x40]
1003634e8: 8b131508    	add	x8, x8, x19, lsl #5
1003634ec: 528007c9    	mov	w9, #0x3e               ; =62
1003634f0: a9006509    	stp	x9, x25, [x8]
1003634f4: f9000909    	str	x9, [x8, #0x10]
1003634f8: 52800029    	mov	w9, #0x1                ; =1
1003634fc: 39006109    	strb	w9, [x8, #0x18]
100363500: 91000668    	add	x8, x19, #0x1
100363504: f90027e8    	str	x8, [sp, #0x48]
100363508: b9402fe8    	ldr	w8, [sp, #0x2c]
10036350c: 370042c8    	tbnz	w8, #0x0, 0x100363d64 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xd24>
100363510: 90007d41    	adrp	x1, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363514: 913b8021    	add	x1, x1, #0xee0
100363518: 9100e3e2    	add	x2, sp, #0x38
10036351c: aa1603e0    	mov	x0, x22
100363520: 97fffe48    	bl	0x100362e40 <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E>
100363524: 14000210    	b	0x100363d64 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xd24>
100363528: f94126d9    	ldr	x25, [x22, #0x248]
10036352c: f9412ada    	ldr	x26, [x22, #0x250]
100363530: 90007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363534: 912ca842    	add	x2, x2, #0xb2a
100363538: 9103a3e8    	add	x8, sp, #0xe8
10036353c: aa1903e0    	mov	x0, x25
100363540: aa1a03e1    	mov	x1, x26
100363544: 52800323    	mov	w3, #0x19               ; =25
100363548: 9437af46    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
10036354c: a94f0be1    	ldp	x1, x2, [sp, #0xf0]
100363550: d10403a0    	sub	x0, x29, #0x100
100363554: a901d3e1    	stp	x1, x20, [sp, #0x18]
100363558: 94003153    	bl	0x10036faa4 <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E>
10036355c: 910403f8    	add	x24, sp, #0x100
100363560: f85003a8    	ldur	x8, [x29, #-0x100]
100363564: f1002d1f    	cmp	x8, #0xb
100363568: 540009a1    	b.ne	0x10036369c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x65c>
10036356c: 3ccb8300    	ldur	q0, [x24, #0xb8]
100363570: 3ccc8301    	ldur	q1, [x24, #0xc8]
100363574: ad000700    	stp	q0, q1, [x24]
100363578: 3ccd8300    	ldur	q0, [x24, #0xd8]
10036357c: 3d800b00    	str	q0, [x24, #0x20]
100363580: f9405be8    	ldr	x8, [sp, #0xb0]
100363584: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
100363588: eb09011f    	cmp	x8, x9
10036358c: f9000bf5    	str	x21, [sp, #0x10]
100363590: 54000440    	b.eq	0x100363618 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x5d8>
100363594: a94b87e0    	ldp	x0, x1, [sp, #0xb8]
100363598: a91607e0    	stp	x0, x1, [sp, #0x160]
10036359c: 910403e3    	add	x3, sp, #0x100
1003635a0: 52800022    	mov	w2, #0x1                ; =1
1003635a4: 9410a8b4    	bl	0x10078d874 <__ZN13codexmate_lib4core5relay12codex_writer38top_level_relay_model_requires_cleanup17hd2a925a79047a23aE>
1003635a8: 34000380    	cbz	w0, 0x100363618 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x5d8>
1003635ac: 910583e8    	add	x8, sp, #0x160
1003635b0: d00023c9    	adrp	x9, 0x1007dd000 <__ZN3zip9zipcrypto24ZipCryptoReader$LT$R$GT$8validate17h6abfe2f5c5f69c24E+0xf0>
1003635b4: 911b0129    	add	x9, x9, #0x6c0
1003635b8: a91727e8    	stp	x8, x9, [sp, #0x170]
1003635bc: 90008de0    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
1003635c0: 91397c00    	add	x0, x0, #0xe5f
1003635c4: 910643e8    	add	x8, sp, #0x190
1003635c8: 9105c3e1    	add	x1, sp, #0x170
1003635cc: 943850b2    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1003635d0: f940cbfc    	ldr	x28, [sp, #0x190]
1003635d4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1003635d8: eb08039f    	cmp	x28, x8
1003635dc: 540001e0    	b.eq	0x100363618 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x5d8>
1003635e0: a959d3f7    	ldp	x23, x20, [sp, #0x198]
1003635e4: f94027f3    	ldr	x19, [sp, #0x48]
1003635e8: f9401fe8    	ldr	x8, [sp, #0x38]
1003635ec: eb08027f    	cmp	x19, x8
1003635f0: 54000061    	b.ne	0x1003635fc <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x5bc>
1003635f4: 9100e3e0    	add	x0, sp, #0x38
1003635f8: 9438a9ae    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
1003635fc: f94023e8    	ldr	x8, [sp, #0x40]
100363600: 8b131508    	add	x8, x8, x19, lsl #5
100363604: a9005d1c    	stp	x28, x23, [x8]
100363608: f9000914    	str	x20, [x8, #0x10]
10036360c: 3900611f    	strb	wzr, [x8, #0x18]
100363610: 91000668    	add	x8, x19, #0x1
100363614: f90027e8    	str	x8, [sp, #0x48]
100363618: f94087f7    	ldr	x23, [sp, #0x108]
10036361c: b40005d7    	cbz	x23, 0x1003636d4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x694>
100363620: f9408ff5    	ldr	x21, [sp, #0x118]
100363624: b4000435    	cbz	x21, 0x1003636a8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x668>
100363628: f94083f4    	ldr	x20, [sp, #0x100]
10036362c: aa1403fc    	mov	x28, x20
100363630: fc408780    	ldr	d0, [x28], #0x8
100363634: 2e208800    	cmge.8b	v0, v0, #0
100363638: 9e660008    	fmov	x8, d0
10036363c: 9201c118    	and	x24, x8, #0x8080808080808080
100363640: 52800313    	mov	w19, #0x18              ; =24
100363644: 14000005    	b	0x100363658 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x618>
100363648: d1000708    	sub	x8, x24, #0x1
10036364c: 8a180118    	and	x24, x8, x24
100363650: d10006b5    	sub	x21, x21, #0x1
100363654: b40002b5    	cbz	x21, 0x1003636a8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x668>
100363658: b50000f8    	cbnz	x24, 0x100363674 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x634>
10036365c: fc408780    	ldr	d0, [x28], #0x8
100363660: 2e208800    	cmge.8b	v0, v0, #0
100363664: 9e660008    	fmov	x8, d0
100363668: d1030294    	sub	x20, x20, #0xc0
10036366c: 9201c118    	and	x24, x8, #0x8080808080808080
100363670: b4ffff78    	cbz	x24, 0x10036365c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x61c>
100363674: dac00308    	rbit	x8, x24
100363678: dac01108    	clz	x8, x8
10036367c: cb480fe8    	neg	x8, x8, lsr #3
100363680: 9b335108    	smaddl	x8, w8, w19, x20
100363684: f85e8101    	ldur	x1, [x8, #-0x18]
100363688: b4fffe01    	cbz	x1, 0x100363648 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x608>
10036368c: f85f0100    	ldur	x0, [x8, #-0x10]
100363690: 52800022    	mov	w2, #0x1                ; =1
100363694: 97f277b5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363698: 17ffffec    	b	0x100363648 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x608>
10036369c: d10403a0    	sub	x0, x29, #0x100
1003636a0: 9401d1c4    	bl	0x1003d7db0 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
1003636a4: 1400000d    	b	0x1003636d8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x698>
1003636a8: 8b1706e8    	add	x8, x23, x23, lsl #1
1003636ac: d37df108    	lsl	x8, x8, #3
1003636b0: 8b170109    	add	x9, x8, x23
1003636b4: b1008521    	adds	x1, x9, #0x21
1003636b8: 910403f8    	add	x24, sp, #0x100
1003636bc: 540000c0    	b.eq	0x1003636d4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x694>
1003636c0: f94083e9    	ldr	x9, [sp, #0x100]
1003636c4: cb080128    	sub	x8, x9, x8
1003636c8: d1006100    	sub	x0, x8, #0x18
1003636cc: 52800102    	mov	w2, #0x8                ; =8
1003636d0: 97f277a6    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003636d4: f9400bf5    	ldr	x21, [sp, #0x10]
1003636d8: 90007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003636dc: 912ca842    	add	x2, x2, #0xb2a
1003636e0: 910403e8    	add	x8, sp, #0x100
1003636e4: aa1903e0    	mov	x0, x25
1003636e8: aa1a03e1    	mov	x1, x26
1003636ec: 52800323    	mov	w3, #0x19               ; =25
1003636f0: 9437aedc    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
1003636f4: a9508bfc    	ldp	x28, x2, [sp, #0x108]
1003636f8: d10403a0    	sub	x0, x29, #0x100
1003636fc: aa1c03e1    	mov	x1, x28
100363700: 940031c2    	bl	0x10036fe08 <__ZN13codexmate_lib4core5relay13codex_catalog28read_catalog_no_account_mode17h1b4963966184d485E>
100363704: f85003b4    	ldur	x20, [x29, #-0x100]
100363708: 385083b3    	ldurb	w19, [x29, #-0xf8]
10036370c: f1002e9f    	cmp	x20, #0xb
100363710: 54000060    	b.eq	0x10036371c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x6dc>
100363714: d10403a0    	sub	x0, x29, #0x100
100363718: 9401d1a6    	bl	0x1003d7db0 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
10036371c: f94083e1    	ldr	x1, [sp, #0x100]
100363720: b4000081    	cbz	x1, 0x100363730 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x6f0>
100363724: aa1c03e0    	mov	x0, x28
100363728: 52800022    	mov	w2, #0x1                ; =1
10036372c: 97f2778f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363730: f94013e0    	ldr	x0, [sp, #0x20]
100363734: aa1b03e1    	mov	x1, x27
100363738: 9410a7a3    	bl	0x10078d5c4 <__ZN13codexmate_lib4core5relay12codex_writer36router_provider_requires_openai_auth17h079ea57f0dada285E>
10036373c: 12001c08    	and	w8, w0, #0xff
100363740: f1002e9f    	cmp	x20, #0xb
100363744: 54000381    	b.ne	0x1003637b4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x774>
100363748: 7100091f    	cmp	w8, #0x2
10036374c: 54000380    	b.eq	0x1003637bc <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x77c>
100363750: 6b13011f    	cmp	w8, w19
100363754: 540006e1    	b.ne	0x100363830 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x7f0>
100363758: 97f27787    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036375c: 5280073c    	mov	w28, #0x39              ; =57
100363760: 52800720    	mov	w0, #0x39               ; =57
100363764: 52800021    	mov	w1, #0x1                ; =1
100363768: 97f2777f    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036376c: b4003ea0    	cbz	x0, 0x100363f40 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xf00>
100363770: aa0003fb    	mov	x27, x0
100363774: 90007d48    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363778: 912e1108    	add	x8, x8, #0xb84
10036377c: ad400500    	ldp	q0, q1, [x8]
100363780: ad000400    	stp	q0, q1, [x0]
100363784: 3dc00900    	ldr	q0, [x8, #0x20]
100363788: 3d800800    	str	q0, [x0, #0x20]
10036378c: 3cc29100    	ldur	q0, [x8, #0x29]
100363790: 3c829000    	stur	q0, [x0, #0x29]
100363794: f94027f3    	ldr	x19, [sp, #0x48]
100363798: f9401fe8    	ldr	x8, [sp, #0x38]
10036379c: eb08027f    	cmp	x19, x8
1003637a0: 54000061    	b.ne	0x1003637ac <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x76c>
1003637a4: 9100e3e0    	add	x0, sp, #0x38
1003637a8: 9438a942    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
1003637ac: 52800728    	mov	w8, #0x39               ; =57
1003637b0: 14000019    	b	0x100363814 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x7d4>
1003637b4: 7100091f    	cmp	w8, #0x2
1003637b8: 540003c1    	b.ne	0x100363830 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x7f0>
1003637bc: 97f2776e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003637c0: 5280083c    	mov	w28, #0x41              ; =65
1003637c4: 52800820    	mov	w0, #0x41               ; =65
1003637c8: 52800021    	mov	w1, #0x1                ; =1
1003637cc: 97f27766    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003637d0: b4003b80    	cbz	x0, 0x100363f40 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xf00>
1003637d4: aa0003fb    	mov	x27, x0
1003637d8: 52800ce8    	mov	w8, #0x67               ; =103
1003637dc: 90007d49    	adrp	x9, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003637e0: 912d0d29    	add	x9, x9, #0xb43
1003637e4: 39010008    	strb	w8, [x0, #0x40]
1003637e8: ad400520    	ldp	q0, q1, [x9]
1003637ec: ad000400    	stp	q0, q1, [x0]
1003637f0: ad410520    	ldp	q0, q1, [x9, #0x20]
1003637f4: ad010400    	stp	q0, q1, [x0, #0x20]
1003637f8: f94027f3    	ldr	x19, [sp, #0x48]
1003637fc: f9401fe8    	ldr	x8, [sp, #0x38]
100363800: eb08027f    	cmp	x19, x8
100363804: 54000061    	b.ne	0x100363810 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x7d0>
100363808: 9100e3e0    	add	x0, sp, #0x38
10036380c: 9438a929    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100363810: 52800828    	mov	w8, #0x41               ; =65
100363814: f94023e9    	ldr	x9, [sp, #0x40]
100363818: 8b131529    	add	x9, x9, x19, lsl #5
10036381c: a9006d28    	stp	x8, x27, [x9]
100363820: f9000928    	str	x8, [x9, #0x10]
100363824: 3900613f    	strb	wzr, [x9, #0x18]
100363828: 91000668    	add	x8, x19, #0x1
10036382c: f90027e8    	str	x8, [sp, #0x48]
100363830: aa1603e0    	mov	x0, x22
100363834: 97fa7913    	bl	0x100201c80 <__ZN13codexmate_lib4core5relay19legacy_virtual_auth13marker_exists17h234ad5508a490929E>
100363838: 36000400    	tbz	w0, #0x0, 0x1003638b8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x878>
10036383c: 97f2774e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100363840: 5280067c    	mov	w28, #0x33              ; =51
100363844: 52800660    	mov	w0, #0x33               ; =51
100363848: 52800021    	mov	w1, #0x1                ; =1
10036384c: 97f27746    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100363850: b4003780    	cbz	x0, 0x100363f40 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xf00>
100363854: aa0003fb    	mov	x27, x0
100363858: 528d6e48    	mov	w8, #0x6b72             ; =27506
10036385c: 72ae4ca8    	movk	w8, #0x7265, lsl #16
100363860: b802f008    	stur	w8, [x0, #0x2f]
100363864: 90007d48    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363868: 912ef508    	add	x8, x8, #0xbbd
10036386c: ad400500    	ldp	q0, q1, [x8]
100363870: ad000400    	stp	q0, q1, [x0]
100363874: 3dc00900    	ldr	q0, [x8, #0x20]
100363878: 3d800800    	str	q0, [x0, #0x20]
10036387c: f94027f3    	ldr	x19, [sp, #0x48]
100363880: f9401fe8    	ldr	x8, [sp, #0x38]
100363884: eb08027f    	cmp	x19, x8
100363888: 54000061    	b.ne	0x100363894 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x854>
10036388c: 9100e3e0    	add	x0, sp, #0x38
100363890: 9438a908    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100363894: f94023e8    	ldr	x8, [sp, #0x40]
100363898: 8b131508    	add	x8, x8, x19, lsl #5
10036389c: 52800669    	mov	w9, #0x33               ; =51
1003638a0: a9006d09    	stp	x9, x27, [x8]
1003638a4: f9000909    	str	x9, [x8, #0x10]
1003638a8: 52800029    	mov	w9, #0x1                ; =1
1003638ac: 39006109    	strb	w9, [x8, #0x18]
1003638b0: 91000668    	add	x8, x19, #0x1
1003638b4: f90027e8    	str	x8, [sp, #0x48]
1003638b8: aa1603e0    	mov	x0, x22
1003638bc: 9415144f    	bl	0x1008a89f8 <__ZN13codexmate_lib4core5relay18router_unlock_auth11has_residue17h150741b3202acaf4E>
1003638c0: 360003e0    	tbz	w0, #0x0, 0x10036393c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x8fc>
1003638c4: 97f2772c    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003638c8: 528006fc    	mov	w28, #0x37              ; =55
1003638cc: 528006e0    	mov	w0, #0x37               ; =55
1003638d0: 52800021    	mov	w1, #0x1                ; =1
1003638d4: 97f27724    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003638d8: b4003340    	cbz	x0, 0x100363f40 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xf00>
1003638dc: aa0003fb    	mov	x27, x0
1003638e0: 90007d48    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003638e4: 912fc108    	add	x8, x8, #0xbf0
1003638e8: ad400500    	ldp	q0, q1, [x8]
1003638ec: ad000400    	stp	q0, q1, [x0]
1003638f0: 3dc00900    	ldr	q0, [x8, #0x20]
1003638f4: 3d800800    	str	q0, [x0, #0x20]
1003638f8: f842f108    	ldur	x8, [x8, #0x2f]
1003638fc: f802f008    	stur	x8, [x0, #0x2f]
100363900: f94027f3    	ldr	x19, [sp, #0x48]
100363904: f9401fe8    	ldr	x8, [sp, #0x38]
100363908: eb08027f    	cmp	x19, x8
10036390c: 54000061    	b.ne	0x100363918 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x8d8>
100363910: 9100e3e0    	add	x0, sp, #0x38
100363914: 9438a8e7    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100363918: f94023e8    	ldr	x8, [sp, #0x40]
10036391c: 8b131508    	add	x8, x8, x19, lsl #5
100363920: 528006e9    	mov	w9, #0x37               ; =55
100363924: a9006d09    	stp	x9, x27, [x8]
100363928: f9000909    	str	x9, [x8, #0x10]
10036392c: 52800029    	mov	w9, #0x1                ; =1
100363930: 39006109    	strb	w9, [x8, #0x18]
100363934: 91000668    	add	x8, x19, #0x1
100363938: f90027e8    	str	x8, [sp, #0x48]
10036393c: 90007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363940: 912ca842    	add	x2, x2, #0xb2a
100363944: 9105c3e8    	add	x8, sp, #0x170
100363948: aa1903e0    	mov	x0, x25
10036394c: aa1a03e1    	mov	x1, x26
100363950: 52800323    	mov	w3, #0x19               ; =25
100363954: 9437ae43    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
100363958: a957ebf9    	ldp	x25, x26, [sp, #0x178]
10036395c: d10403a8    	sub	x8, x29, #0x100
100363960: aa1903e0    	mov	x0, x25
100363964: aa1a03e1    	mov	x1, x26
100363968: 94379324    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
10036396c: a97007b7    	ldp	x23, x1, [x29, #-0x100]
100363970: aa1703e0    	mov	x0, x23
100363974: 9401dc8b    	bl	0x1003daba0 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
100363978: b4000397    	cbz	x23, 0x1003639e8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x9a8>
10036397c: a9196bf9    	stp	x25, x26, [sp, #0x190]
100363980: 910643e8    	add	x8, sp, #0x190
100363984: f0006fc9    	adrp	x9, 0x10115e000 <__ZN4core5slice4sort8unstable9quicksort9quicksort17h324c95ad922d2016E+0x1fc>
100363988: 91351129    	add	x9, x9, #0xd44
10036398c: a91027e8    	stp	x8, x9, [sp, #0x100]
100363990: 90008de0    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
100363994: 9131d000    	add	x0, x0, #0xc74
100363998: d10403a8    	sub	x8, x29, #0x100
10036399c: 910403e1    	add	x1, sp, #0x100
1003639a0: 94384fbd    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1003639a4: a9706bb7    	ldp	x23, x26, [x29, #-0x100]
1003639a8: f85103b4    	ldur	x20, [x29, #-0xf0]
1003639ac: f94027f3    	ldr	x19, [sp, #0x48]
1003639b0: f9401fe8    	ldr	x8, [sp, #0x38]
1003639b4: eb08027f    	cmp	x19, x8
1003639b8: 54000061    	b.ne	0x1003639c4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0x984>
1003639bc: 9100e3e0    	add	x0, sp, #0x38
1003639c0: 9438a8bc    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
1003639c4: f94023e8    	ldr	x8, [sp, #0x40]
1003639c8: 8b131508    	add	x8, x8, x19, lsl #5
1003639cc: a9006917    	stp	x23, x26, [x8]
1003639d0: f9000914    	str	x20, [x8, #0x10]
1003639d4: 52800049    	mov	w9, #0x2                ; =2
1003639d8: 39006109    	strb	w9, [x8, #0x18]
1003639dc: 91000668    	add	x8, x19, #0x1
1003639e0: f90027e8    	str	x8, [sp, #0x48]
1003639e4: 14000078    	b	0x100363bc4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xb84>
1003639e8: 910403e0    	add	x0, sp, #0x100
1003639ec: aa1903e1    	mov	x1, x25
1003639f0: aa1a03e2    	mov	x2, x26
1003639f4: 94003105    	bl	0x10036fe08 <__ZN13codexmate_lib4core5relay13codex_catalog28read_catalog_no_account_mode17h1b4963966184d485E>
1003639f8: f94083e8    	ldr	x8, [sp, #0x100]
1003639fc: f1002d1f    	cmp	x8, #0xb
100363a00: 54000a01    	b.ne	0x100363b40 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xb00>
100363a04: 394423f3    	ldrb	w19, [sp, #0x108]
100363a08: 910403e0    	add	x0, sp, #0x100
100363a0c: aa1903e1    	mov	x1, x25
100363a10: aa1a03e2    	mov	x2, x26
100363a14: 94003024    	bl	0x10036faa4 <__ZN13codexmate_lib4core5relay13codex_catalog24read_catalog_model_slugs17h8de892afdc329116E>
100363a18: f94083e8    	ldr	x8, [sp, #0x100]
100363a1c: f90007e8    	str	x8, [sp, #0x8]
100363a20: f1002d1f    	cmp	x8, #0xb
100363a24: 54001c61    	b.ne	0x100363db0 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xd70>
100363a28: aa1503f8    	mov	x24, x21
100363a2c: 36002073    	tbz	w19, #0x0, 0x100363e38 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xdf8>
100363a30: f94093fc    	ldr	x28, [sp, #0x120]
100363a34: b400203c    	cbz	x28, 0x100363e38 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xdf8>
100363a38: f94087f5    	ldr	x21, [sp, #0x108]
100363a3c: aa1503fb    	mov	x27, x21
100363a40: fc408760    	ldr	d0, [x27], #0x8
100363a44: 2e208800    	cmge.8b	v0, v0, #0
100363a48: 9e660008    	fmov	x8, d0
100363a4c: 9201c113    	and	x19, x8, #0x8080808080808080
100363a50: 52800314    	mov	w20, #0x18              ; =24
100363a54: 14000005    	b	0x100363a68 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xa28>
100363a58: d1000668    	sub	x8, x19, #0x1
100363a5c: 8a130113    	and	x19, x8, x19
100363a60: d100079c    	sub	x28, x28, #0x1
100363a64: b4001ebc    	cbz	x28, 0x100363e38 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xdf8>
100363a68: b50000f3    	cbnz	x19, 0x100363a84 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xa44>
100363a6c: fc408760    	ldr	d0, [x27], #0x8
100363a70: 2e208800    	cmge.8b	v0, v0, #0
100363a74: 9e660008    	fmov	x8, d0
100363a78: d10302b5    	sub	x21, x21, #0xc0
100363a7c: 9201c113    	and	x19, x8, #0x8080808080808080
100363a80: b4ffff73    	cbz	x19, 0x100363a6c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xa2c>
100363a84: dac00268    	rbit	x8, x19
100363a88: dac01108    	clz	x8, x8
100363a8c: cb480fe8    	neg	x8, x8, lsr #3
100363a90: 9b345508    	smaddl	x8, w8, w20, x21
100363a94: a97f6917    	ldp	x23, x26, [x8, #-0x10]
100363a98: aa1703e0    	mov	x0, x23
100363a9c: aa1a03e1    	mov	x1, x26
100363aa0: 940024b1    	bl	0x10036cd64 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E>
100363aa4: 3707fda0    	tbnz	w0, #0x0, 0x100363a58 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xa18>
100363aa8: aa1703e0    	mov	x0, x23
100363aac: aa1a03e1    	mov	x1, x26
100363ab0: 940010e8    	bl	0x100367e50 <__ZN13codexmate_lib4core5relay13codex_catalog19is_relay_model_slug17h3d07e3b2bec682bbE>
100363ab4: 35fffd20    	cbnz	w0, 0x100363a58 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xa18>
100363ab8: 97f276af    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100363abc: 528009a0    	mov	w0, #0x4d               ; =77
100363ac0: 52800021    	mov	w1, #0x1                ; =1
100363ac4: 97f276a8    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100363ac8: b4002440    	cbz	x0, 0x100363f50 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xf10>
100363acc: aa0003fa    	mov	x26, x0
100363ad0: 90007d48    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363ad4: 9139b108    	add	x8, x8, #0xe6c
100363ad8: ad410500    	ldp	q0, q1, [x8, #0x20]
100363adc: ad010400    	stp	q0, q1, [x0, #0x20]
100363ae0: 3cc3d100    	ldur	q0, [x8, #0x3d]
100363ae4: 3c83d000    	stur	q0, [x0, #0x3d]
100363ae8: ad400101    	ldp	q1, q0, [x8]
100363aec: ad000001    	stp	q1, q0, [x0]
100363af0: f94027f3    	ldr	x19, [sp, #0x48]
100363af4: f9401fe8    	ldr	x8, [sp, #0x38]
100363af8: eb08027f    	cmp	x19, x8
100363afc: aa1803f5    	mov	x21, x24
100363b00: 54000061    	b.ne	0x100363b0c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xacc>
100363b04: 9100e3e0    	add	x0, sp, #0x38
100363b08: 9438a86a    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100363b0c: f94023e8    	ldr	x8, [sp, #0x40]
100363b10: 8b131508    	add	x8, x8, x19, lsl #5
100363b14: 528009a9    	mov	w9, #0x4d               ; =77
100363b18: a9006909    	stp	x9, x26, [x8]
100363b1c: f9000909    	str	x9, [x8, #0x10]
100363b20: 52800049    	mov	w9, #0x2                ; =2
100363b24: 39006109    	strb	w9, [x8, #0x18]
100363b28: 91000668    	add	x8, x19, #0x1
100363b2c: f90027e8    	str	x8, [sp, #0x48]
100363b30: 910403e8    	add	x8, sp, #0x100
100363b34: 91002100    	add	x0, x8, #0x8
100363b38: 94142eb4    	bl	0x10086f608 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5856a37aee37aca3E>
100363b3c: 14000022    	b	0x100363bc4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xb84>
100363b40: ad410700    	ldp	q0, q1, [x24, #0x20]
100363b44: ad068700    	stp	q0, q1, [x24, #0xd0]
100363b48: ad420700    	ldp	q0, q1, [x24, #0x40]
100363b4c: ad078700    	stp	q0, q1, [x24, #0xf0]
100363b50: ad400700    	ldp	q0, q1, [x24]
100363b54: ad058700    	stp	q0, q1, [x24, #0xb0]
100363b58: d10403a8    	sub	x8, x29, #0x100
100363b5c: b00024c9    	adrp	x9, 0x1007fc000 <__ZN6chrono8datetime40DateTime$LT$chrono..offset..utc..Utc$GT$14from_timestamp17h2b34124b50daed04E+0x8>
100363b60: 91343129    	add	x9, x9, #0xd0c
100363b64: a91627e8    	stp	x8, x9, [sp, #0x160]
100363b68: 90008de0    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
100363b6c: 91331800    	add	x0, x0, #0xcc6
100363b70: 910643e8    	add	x8, sp, #0x190
100363b74: 910583e1    	add	x1, sp, #0x160
100363b78: 94384f47    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100363b7c: a9596bf7    	ldp	x23, x26, [sp, #0x190]
100363b80: f940d3f4    	ldr	x20, [sp, #0x1a0]
100363b84: f94027f3    	ldr	x19, [sp, #0x48]
100363b88: f9401fe8    	ldr	x8, [sp, #0x38]
100363b8c: eb08027f    	cmp	x19, x8
100363b90: 54000061    	b.ne	0x100363b9c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xb5c>
100363b94: 9100e3e0    	add	x0, sp, #0x38
100363b98: 9438a846    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100363b9c: f94023e8    	ldr	x8, [sp, #0x40]
100363ba0: 8b131508    	add	x8, x8, x19, lsl #5
100363ba4: a9006917    	stp	x23, x26, [x8]
100363ba8: f9000914    	str	x20, [x8, #0x10]
100363bac: 52800049    	mov	w9, #0x2                ; =2
100363bb0: 39006109    	strb	w9, [x8, #0x18]
100363bb4: 91000668    	add	x8, x19, #0x1
100363bb8: f90027e8    	str	x8, [sp, #0x48]
100363bbc: d10403a0    	sub	x0, x29, #0x100
100363bc0: 9401d07c    	bl	0x1003d7db0 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
100363bc4: f940bbe1    	ldr	x1, [sp, #0x170]
100363bc8: b4000081    	cbz	x1, 0x100363bd8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xb98>
100363bcc: aa1903e0    	mov	x0, x25
100363bd0: 52800022    	mov	w2, #0x1                ; =1
100363bd4: 97f27665    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363bd8: b9402fe8    	ldr	w8, [sp, #0x2c]
100363bdc: 350000c8    	cbnz	w8, 0x100363bf4 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xbb4>
100363be0: 90007d41    	adrp	x1, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363be4: 912abc21    	add	x1, x1, #0xaaf
100363be8: 9100e3e2    	add	x2, sp, #0x38
100363bec: aa1603e0    	mov	x0, x22
100363bf0: 97fffc94    	bl	0x100362e40 <__ZN13codexmate_lib4core5relay10invariants22verify_thread_provider17hc5a7069a0efa7753E>
100363bf4: d10403b7    	sub	x23, x29, #0x100
100363bf8: d10403a8    	sub	x8, x29, #0x100
100363bfc: aa1603e0    	mov	x0, x22
100363c00: 941584d8    	bl	0x1008c4f60 <__ZN13codexmate_lib4core5relay23codex_thread_visibility35inspect_router_thread_model_catalog17h6fbabaa3ed4f6bf2E>
100363c04: f85103b4    	ldur	x20, [x29, #-0xf0]
100363c08: f85403a8    	ldur	x8, [x29, #-0xc0]
100363c0c: aa140108    	orr	x8, x8, x20
100363c10: b40009a8    	cbz	x8, 0x100363d44 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xd04>
100363c14: aa1503f8    	mov	x24, x21
100363c18: f85283a1    	ldur	x1, [x29, #-0xd8]
100363c1c: d0000455    	adrp	x21, 0x1003ed000 <__ZN5alloc11collections5btree3set21BTreeSet$LT$T$C$A$GT$10difference17h151923f22deefe03E+0x15c>
100363c20: 9133e2b5    	add	x21, x21, #0xcf8
100363c24: b40003c1    	cbz	x1, 0x100363c9c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xc5c>
100363c28: f85203a0    	ldur	x0, [x29, #-0xe0]
100363c2c: 90007d42    	adrp	x2, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100363c30: 9133f442    	add	x2, x2, #0xcfd
100363c34: 910403e8    	add	x8, sp, #0x100
100363c38: 52800063    	mov	w3, #0x3                ; =3
100363c3c: 97fb37c2    	bl	0x100231b44 <__ZN5alloc3str17join_generic_copy17hc3fffb246d6f0b82E>
100363c40: 910403e8    	add	x8, sp, #0x100
100363c44: 3dc00100    	ldr	q0, [x8]
100363c48: 3d802500    	str	q0, [x8, #0x90]
100363c4c: f9408be8    	ldr	x8, [sp, #0x110]
100363c50: f900d3e8    	str	x8, [sp, #0x1a0]
100363c54: 910643e8    	add	x8, sp, #0x190
100363c58: a91757e8    	stp	x8, x21, [sp, #0x170]
100363c5c: 90008de0    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
100363c60: 91284400    	add	x0, x0, #0xa11
100363c64: 910403e8    	add	x8, sp, #0x100
100363c68: 9105c3e1    	add	x1, sp, #0x170
100363c6c: 94384f0a    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100363c70: f940cbe1    	ldr	x1, [sp, #0x190]
100363c74: b4000081    	cbz	x1, 0x100363c84 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xc44>
100363c78: f940cfe0    	ldr	x0, [sp, #0x198]
100363c7c: 52800022    	mov	w2, #0x1                ; =1
100363c80: 97f2763a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363c84: 910403e8    	add	x8, sp, #0x100
100363c88: 3dc00100    	ldr	q0, [x8]
100363c8c: 3d801d00    	str	q0, [x8, #0x70]
100363c90: f9408be8    	ldr	x8, [sp, #0x110]
100363c94: f900c3e8    	str	x8, [sp, #0x180]
100363c98: 14000004    	b	0x100363ca8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xc68>
100363c9c: 52800028    	mov	w8, #0x1                ; =1
100363ca0: a91723ff    	stp	xzr, x8, [sp, #0x170]
100363ca4: f900c3ff    	str	xzr, [sp, #0x180]
100363ca8: 910102e8    	add	x8, x23, #0x40
100363cac: f900b3f4    	str	x20, [sp, #0x160]
100363cb0: 900070c9    	adrp	x9, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
100363cb4: 910d3129    	add	x9, x9, #0x34c
100363cb8: d10403aa    	sub	x10, x29, #0x100
100363cbc: 9101214a    	add	x10, x10, #0x48
100363cc0: a91027e8    	stp	x8, x9, [sp, #0x100]
100363cc4: a91127ea    	stp	x10, x9, [sp, #0x110]
100363cc8: 910583e8    	add	x8, sp, #0x160
100363ccc: a91227e8    	stp	x8, x9, [sp, #0x120]
100363cd0: 9105c3e8    	add	x8, sp, #0x170
100363cd4: a91357e8    	stp	x8, x21, [sp, #0x130]
100363cd8: 90008de0    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
100363cdc: 91368c00    	add	x0, x0, #0xda3
100363ce0: 910643e8    	add	x8, sp, #0x190
100363ce4: 910403e1    	add	x1, sp, #0x100
100363ce8: 94384eeb    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100363cec: a9595ff6    	ldp	x22, x23, [sp, #0x190]
100363cf0: f940d3f4    	ldr	x20, [sp, #0x1a0]
100363cf4: f94027f3    	ldr	x19, [sp, #0x48]
100363cf8: f9401fe8    	ldr	x8, [sp, #0x38]
100363cfc: eb08027f    	cmp	x19, x8
100363d00: aa1803f5    	mov	x21, x24
100363d04: 54000061    	b.ne	0x100363d10 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xcd0>
100363d08: 9100e3e0    	add	x0, sp, #0x38
100363d0c: 9438a7e9    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100363d10: f94023e8    	ldr	x8, [sp, #0x40]
100363d14: 8b131508    	add	x8, x8, x19, lsl #5
100363d18: a9005d16    	stp	x22, x23, [x8]
100363d1c: f9000914    	str	x20, [x8, #0x10]
100363d20: 52800069    	mov	w9, #0x3                ; =3
100363d24: 39006109    	strb	w9, [x8, #0x18]
100363d28: 91000668    	add	x8, x19, #0x1
100363d2c: f90027e8    	str	x8, [sp, #0x48]
100363d30: f940bbe1    	ldr	x1, [sp, #0x170]
100363d34: b4000081    	cbz	x1, 0x100363d44 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xd04>
100363d38: f940bfe0    	ldr	x0, [sp, #0x178]
100363d3c: 52800022    	mov	w2, #0x1                ; =1
100363d40: 97f2760a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363d44: d10403a0    	sub	x0, x29, #0x100
100363d48: 9401c63a    	bl	0x1003d5630 <__ZN4core3ptr102drop_in_place$LT$codexmate_lib..core..relay..codex_thread_visibility..ThreadModelCatalogInspection$GT$17hbc6795982355fa85E>
100363d4c: f94077e1    	ldr	x1, [sp, #0xe8]
100363d50: b4000081    	cbz	x1, 0x100363d60 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xd20>
100363d54: f9400fe0    	ldr	x0, [sp, #0x18]
100363d58: 52800022    	mov	w2, #0x1                ; =1
100363d5c: 97f27603    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363d60: f94013f4    	ldr	x20, [sp, #0x20]
100363d64: 3cc383e0    	ldur	q0, [sp, #0x38]
100363d68: 3d8002a0    	str	q0, [x21]
100363d6c: f94027e8    	ldr	x8, [sp, #0x48]
100363d70: f9000aa8    	str	x8, [x21, #0x10]
100363d74: 9101a3e0    	add	x0, sp, #0x68
100363d78: 9401d777    	bl	0x1003d9b54 <__ZN4core3ptr80drop_in_place$LT$codexmate_lib..core..relay..invariants..RouterConfigSurface$GT$17h4895046180c02a2cE>
100363d7c: f9401be1    	ldr	x1, [sp, #0x30]
100363d80: b4000081    	cbz	x1, 0x100363d90 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xd50>
100363d84: aa1403e0    	mov	x0, x20
100363d88: 52800022    	mov	w2, #0x1                ; =1
100363d8c: 97f275f7    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363d90: 910983ff    	add	sp, sp, #0x260
100363d94: a9457bfd    	ldp	x29, x30, [sp, #0x50]
100363d98: a9444ff4    	ldp	x20, x19, [sp, #0x40]
100363d9c: a94357f6    	ldp	x22, x21, [sp, #0x30]
100363da0: a9425ff8    	ldp	x24, x23, [sp, #0x20]
100363da4: a94167fa    	ldp	x26, x25, [sp, #0x10]
100363da8: a8c66ffc    	ldp	x28, x27, [sp], #0x60
100363dac: d65f03c0    	ret
100363db0: ad410700    	ldp	q0, q1, [x24, #0x20]
100363db4: ad068700    	stp	q0, q1, [x24, #0xd0]
100363db8: ad420700    	ldp	q0, q1, [x24, #0x40]
100363dbc: ad078700    	stp	q0, q1, [x24, #0xf0]
100363dc0: ad400700    	ldp	q0, q1, [x24]
100363dc4: ad058700    	stp	q0, q1, [x24, #0xb0]
100363dc8: d10403a8    	sub	x8, x29, #0x100
100363dcc: b00024c9    	adrp	x9, 0x1007fc000 <__ZN6chrono8datetime40DateTime$LT$chrono..offset..utc..Utc$GT$14from_timestamp17h2b34124b50daed04E+0x8>
100363dd0: 91343129    	add	x9, x9, #0xd0c
100363dd4: a91627e8    	stp	x8, x9, [sp, #0x160]
100363dd8: 90008de0    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
100363ddc: 91325c00    	add	x0, x0, #0xc97
100363de0: 910643e8    	add	x8, sp, #0x190
100363de4: 910583e1    	add	x1, sp, #0x160
100363de8: 94384eab    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100363dec: a9596bf7    	ldp	x23, x26, [sp, #0x190]
100363df0: f940d3f4    	ldr	x20, [sp, #0x1a0]
100363df4: f94027f3    	ldr	x19, [sp, #0x48]
100363df8: f9401fe8    	ldr	x8, [sp, #0x38]
100363dfc: eb08027f    	cmp	x19, x8
100363e00: 54000061    	b.ne	0x100363e0c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xdcc>
100363e04: 9100e3e0    	add	x0, sp, #0x38
100363e08: 9438a7aa    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100363e0c: f94023e8    	ldr	x8, [sp, #0x40]
100363e10: 8b131508    	add	x8, x8, x19, lsl #5
100363e14: a9006917    	stp	x23, x26, [x8]
100363e18: f9000914    	str	x20, [x8, #0x10]
100363e1c: 52800049    	mov	w9, #0x2                ; =2
100363e20: 39006109    	strb	w9, [x8, #0x18]
100363e24: 91000668    	add	x8, x19, #0x1
100363e28: f90027e8    	str	x8, [sp, #0x48]
100363e2c: d10403a0    	sub	x0, x29, #0x100
100363e30: 9401cfe0    	bl	0x1003d7db0 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
100363e34: aa1503f8    	mov	x24, x21
100363e38: f940bbe1    	ldr	x1, [sp, #0x170]
100363e3c: b4000081    	cbz	x1, 0x100363e4c <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xe0c>
100363e40: aa1903e0    	mov	x0, x25
100363e44: 52800022    	mov	w2, #0x1                ; =1
100363e48: 97f275c8    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100363e4c: f94007e8    	ldr	x8, [sp, #0x8]
100363e50: f1002d1f    	cmp	x8, #0xb
100363e54: aa1803f5    	mov	x21, x24
100363e58: 54ffec01    	b.ne	0x100363bd8 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E+0xb98>
100363e5c: f9408bf7    	ldr	x23, [sp, #0x110]
