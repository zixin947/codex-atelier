
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008e2f40 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE>:
1008e2f40: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
1008e2f44: a90167fa    	stp	x26, x25, [sp, #0x10]
1008e2f48: a9025ff8    	stp	x24, x23, [sp, #0x20]
1008e2f4c: a90357f6    	stp	x22, x21, [sp, #0x30]
1008e2f50: a9044ff4    	stp	x20, x19, [sp, #0x40]
1008e2f54: a9057bfd    	stp	x29, x30, [sp, #0x50]
1008e2f58: 910143fd    	add	x29, sp, #0x50
1008e2f5c: d12ec3ff    	sub	sp, sp, #0xbb0
1008e2f60: f90003ff    	str	xzr, [sp]
1008e2f64: aa0103f4    	mov	x20, x1
1008e2f68: aa0003fa    	mov	x26, x0
1008e2f6c: aa0803f5    	mov	x21, x8
1008e2f70: f9412416    	ldr	x22, [x0, #0x248]
1008e2f74: f9412817    	ldr	x23, [x0, #0x250]
1008e2f78: 900056a2    	adrp	x2, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e2f7c: 912ae442    	add	x2, x2, #0xab9
1008e2f80: 910123e8    	add	x8, sp, #0x48
1008e2f84: aa1603e0    	mov	x0, x22
1008e2f88: aa1703e1    	mov	x1, x23
1008e2f8c: 528001a3    	mov	w3, #0xd                ; =13
1008e2f90: 9421b0b4    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
1008e2f94: a9454ffc    	ldp	x28, x19, [sp, #0x50]
1008e2f98: 911de3e8    	add	x8, sp, #0x778
1008e2f9c: aa1c03e0    	mov	x0, x28
1008e2fa0: aa1303e1    	mov	x1, x19
1008e2fa4: 942190d2    	bl	0x1011472ec <__ZN3std3sys2fs16symlink_metadata17hd0e44a06c5789f14E>
1008e2fa8: b9477be8    	ldr	w8, [sp, #0x778]
1008e2fac: 360003c8    	tbz	w8, #0x0, 0x1008e3024 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xe4>
1008e2fb0: f943c3f3    	ldr	x19, [sp, #0x780]
1008e2fb4: aa1303e0    	mov	x0, x19
1008e2fb8: 94003b03    	bl	0x1008f1bc4 <__ZN3std2io5error5Error4kind17h247456c019dc3d6bE>
1008e2fbc: 72001c1f    	tst	w0, #0xff
1008e2fc0: 54000a20    	b.eq	0x1008e3104 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x1c4>
1008e2fc4: 97dc796c    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e2fc8: 52800520    	mov	w0, #0x29               ; =41
1008e2fcc: 52800021    	mov	w1, #0x1                ; =1
1008e2fd0: 97dc7965    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e2fd4: b4005000    	cbz	x0, 0x1008e39d4 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xa94>
1008e2fd8: 900056a8    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e2fdc: 91200d08    	add	x8, x8, #0x803
1008e2fe0: ad400500    	ldp	q0, q1, [x8]
1008e2fe4: ad000400    	stp	q0, q1, [x0]
1008e2fe8: 3cc19100    	ldur	q0, [x8, #0x19]
1008e2fec: 3c819000    	stur	q0, [x0, #0x19]
1008e2ff0: 52800528    	mov	w8, #0x29               ; =41
1008e2ff4: a90002a8    	stp	x8, x0, [x21]
1008e2ff8: f9000aa8    	str	x8, [x21, #0x10]
1008e2ffc: 92400668    	and	x8, x19, #0x3
1008e3000: f100051f    	cmp	x8, #0x1
1008e3004: 54001ba1    	b.ne	0x1008e3378 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x438>
1008e3008: f85ffe74    	ldr	x20, [x19, #-0x1]!
1008e300c: f9400676    	ldr	x22, [x19, #0x8]
1008e3010: f94002c8    	ldr	x8, [x22]
1008e3014: b40008e8    	cbz	x8, 0x1008e3130 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x1f0>
1008e3018: aa1403e0    	mov	x0, x20
1008e301c: d63f0100    	blr	x8
1008e3020: 14000044    	b	0x1008e3130 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x1f0>
1008e3024: 911de3e8    	add	x8, sp, #0x778
1008e3028: aa1c03e0    	mov	x0, x28
1008e302c: aa1303e1    	mov	x1, x19
1008e3030: 942190af    	bl	0x1011472ec <__ZN3std3sys2fs16symlink_metadata17hd0e44a06c5789f14E>
1008e3034: b9477be8    	ldr	w8, [sp, #0x778]
1008e3038: f943c3fb    	ldr	x27, [sp, #0x780]
1008e303c: 7100051f    	cmp	w8, #0x1
1008e3040: 54000960    	b.eq	0x1008e316c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x22c>
1008e3044: 92540f68    	and	x8, x27, #0xf00000000000
1008e3048: d2c80009    	mov	x9, #0x400000000000     ; =70368744177664
1008e304c: eb09011f    	cmp	x8, x9
1008e3050: 54000821    	b.ne	0x1008e3154 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x214>
1008e3054: f943c7f9    	ldr	x25, [sp, #0x788]
1008e3058: f9413340    	ldr	x0, [x26, #0x260]
1008e305c: f9413741    	ldr	x1, [x26, #0x268]
1008e3060: 911de3e8    	add	x8, sp, #0x778
1008e3064: a90303e1    	stp	x1, x0, [sp, #0x30]
1008e3068: 94217d79    	bl	0x10114264c <__ZN3std2fs4read5inner17hebd695181b114834E>
1008e306c: f943bfe9    	ldr	x9, [sp, #0x778]
1008e3070: f943c3f8    	ldr	x24, [sp, #0x780]
1008e3074: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008e3078: f90023e9    	str	x9, [sp, #0x40]
1008e307c: eb08013f    	cmp	x9, x8
1008e3080: 54000ce1    	b.ne	0x1008e321c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x2dc>
1008e3084: 92400708    	and	x8, x24, #0x3
1008e3088: f100051f    	cmp	x8, #0x1
1008e308c: 54000201    	b.ne	0x1008e30cc <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x18c>
1008e3090: f85fff13    	ldr	x19, [x24, #-0x1]!
1008e3094: f9400714    	ldr	x20, [x24, #0x8]
1008e3098: f9400288    	ldr	x8, [x20]
1008e309c: b4000068    	cbz	x8, 0x1008e30a8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x168>
1008e30a0: aa1303e0    	mov	x0, x19
1008e30a4: d63f0100    	blr	x8
1008e30a8: f9400681    	ldr	x1, [x20, #0x8]
1008e30ac: b4000081    	cbz	x1, 0x1008e30bc <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x17c>
1008e30b0: f9400a82    	ldr	x2, [x20, #0x10]
1008e30b4: aa1303e0    	mov	x0, x19
1008e30b8: 97dc792c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e30bc: aa1803e0    	mov	x0, x24
1008e30c0: 52800301    	mov	w1, #0x18               ; =24
1008e30c4: 52800102    	mov	w2, #0x8                ; =8
1008e30c8: 97dc7928    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e30cc: 97dc792a    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e30d0: 528005b3    	mov	w19, #0x2d              ; =45
1008e30d4: 528005a0    	mov	w0, #0x2d               ; =45
1008e30d8: 52800021    	mov	w1, #0x1                ; =1
1008e30dc: 97dc7922    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e30e0: b4000960    	cbz	x0, 0x1008e320c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x2cc>
1008e30e4: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e30e8: 9120b108    	add	x8, x8, #0x82c
1008e30ec: ad400500    	ldp	q0, q1, [x8]
1008e30f0: ad000400    	stp	q0, q1, [x0]
1008e30f4: 3cc1d100    	ldur	q0, [x8, #0x1d]
1008e30f8: 3c81d000    	stur	q0, [x0, #0x1d]
1008e30fc: 528005a8    	mov	w8, #0x2d               ; =45
1008e3100: 1400002b    	b	0x1008e31ac <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x26c>
1008e3104: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008e3108: f90002a8    	str	x8, [x21]
1008e310c: 92400668    	and	x8, x19, #0x3
1008e3110: f100051f    	cmp	x8, #0x1
1008e3114: 54001321    	b.ne	0x1008e3378 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x438>
1008e3118: f85ffe74    	ldr	x20, [x19, #-0x1]!
1008e311c: f9400676    	ldr	x22, [x19, #0x8]
1008e3120: f94002c8    	ldr	x8, [x22]
1008e3124: b4000068    	cbz	x8, 0x1008e3130 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x1f0>
1008e3128: aa1403e0    	mov	x0, x20
1008e312c: d63f0100    	blr	x8
1008e3130: f94006c1    	ldr	x1, [x22, #0x8]
1008e3134: b4000081    	cbz	x1, 0x1008e3144 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x204>
1008e3138: f9400ac2    	ldr	x2, [x22, #0x10]
1008e313c: aa1403e0    	mov	x0, x20
1008e3140: 97dc790a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3144: aa1303e0    	mov	x0, x19
1008e3148: 52800301    	mov	w1, #0x18               ; =24
1008e314c: 52800102    	mov	w2, #0x8                ; =8
1008e3150: 14000089    	b	0x1008e3374 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x434>
1008e3154: d0005121    	adrp	x1, 0x101309000 <_anon.e143397d1bdb1baba839fd20a45cc9a3.246+0x390>
1008e3158: 911cb821    	add	x1, x1, #0x72e
1008e315c: 528002a0    	mov	w0, #0x15               ; =21
1008e3160: 528006a2    	mov	w2, #0x35               ; =53
1008e3164: 94218255    	bl	0x101143ab8 <__ZN3std2io5error5Error3new17h311cf880a0f7d574E>
1008e3168: aa0003fb    	mov	x27, x0
1008e316c: 92400768    	and	x8, x27, #0x3
1008e3170: f100051f    	cmp	x8, #0x1
1008e3174: 54000220    	b.eq	0x1008e31b8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x278>
1008e3178: 97dc78ff    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e317c: 52800453    	mov	w19, #0x22              ; =34
1008e3180: 52800440    	mov	w0, #0x22               ; =34
1008e3184: 52800021    	mov	w1, #0x1                ; =1
1008e3188: 97dc78f7    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e318c: b4000400    	cbz	x0, 0x1008e320c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x2cc>
1008e3190: 528f2e48    	mov	w8, #0x7972             ; =31090
1008e3194: 79004008    	strh	w8, [x0, #0x20]
1008e3198: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e319c: 9122c108    	add	x8, x8, #0x8b0
1008e31a0: ad400500    	ldp	q0, q1, [x8]
1008e31a4: ad000400    	stp	q0, q1, [x0]
1008e31a8: 52800448    	mov	w8, #0x22               ; =34
1008e31ac: a90002a8    	stp	x8, x0, [x21]
1008e31b0: f9000aa8    	str	x8, [x21, #0x10]
1008e31b4: 14000071    	b	0x1008e3378 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x438>
1008e31b8: f85fff73    	ldr	x19, [x27, #-0x1]!
1008e31bc: f9400774    	ldr	x20, [x27, #0x8]
1008e31c0: f9400288    	ldr	x8, [x20]
1008e31c4: b4000068    	cbz	x8, 0x1008e31d0 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x290>
1008e31c8: aa1303e0    	mov	x0, x19
1008e31cc: d63f0100    	blr	x8
1008e31d0: f9400681    	ldr	x1, [x20, #0x8]
1008e31d4: b4000081    	cbz	x1, 0x1008e31e4 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x2a4>
1008e31d8: f9400a82    	ldr	x2, [x20, #0x10]
1008e31dc: aa1303e0    	mov	x0, x19
1008e31e0: 97dc78e2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e31e4: aa1b03e0    	mov	x0, x27
1008e31e8: 52800301    	mov	w1, #0x18               ; =24
1008e31ec: 52800102    	mov	w2, #0x8                ; =8
1008e31f0: 97dc78de    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e31f4: 97dc78e0    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e31f8: 52800453    	mov	w19, #0x22              ; =34
1008e31fc: 52800440    	mov	w0, #0x22               ; =34
1008e3200: 52800021    	mov	w1, #0x1                ; =1
1008e3204: 97dc78d8    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e3208: b5fffc40    	cbnz	x0, 0x1008e3190 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x250>
1008e320c: 52800020    	mov	w0, #0x1                ; =1
1008e3210: aa1303e1    	mov	x1, x19
1008e3214: 9423b514    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008e3218: 1400020a    	b	0x1008e3a40 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb00>
1008e321c: f943c7e8    	ldr	x8, [sp, #0x788]
1008e3220: f90123f8    	str	x24, [sp, #0x240]
1008e3224: a901a3f9    	stp	x25, x8, [sp, #0x18]
1008e3228: f90127e8    	str	x8, [sp, #0x248]
1008e322c: f9012fff    	str	xzr, [sp, #0x258]
1008e3230: f9012bff    	str	xzr, [sp, #0x250]
1008e3234: 911de3e8    	add	x8, sp, #0x778
1008e3238: 910903e0    	add	x0, sp, #0x240
1008e323c: f90017f8    	str	x24, [sp, #0x28]
1008e3240: 97e3df3e    	bl	0x1001daf38 <__ZN10serde_json2de10from_trait17h4c732b8933312aeaE>
1008e3244: f943bff9    	ldr	x25, [sp, #0x778]
1008e3248: f943c3f8    	ldr	x24, [sp, #0x780]
1008e324c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008e3250: eb08033f    	cmp	x25, x8
1008e3254: 54000281    	b.ne	0x1008e32a4 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x364>
1008e3258: f90123f8    	str	x24, [sp, #0x240]
1008e325c: 910903e0    	add	x0, sp, #0x240
1008e3260: f94017f9    	ldr	x25, [sp, #0x28]
1008e3264: 94004e03    	bl	0x1008f6a70 <__ZN4core3ptr45drop_in_place$LT$serde_json..error..Error$GT$17hc34208687d70e728E>
1008e3268: 97dc78c3    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e326c: 528004e0    	mov	w0, #0x27               ; =39
1008e3270: 52800021    	mov	w1, #0x1                ; =1
1008e3274: 97dc78bc    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e3278: b4003b60    	cbz	x0, 0x1008e39e4 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xaa4>
1008e327c: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e3280: 91249d08    	add	x8, x8, #0x927
1008e3284: ad400500    	ldp	q0, q1, [x8]
1008e3288: ad000400    	stp	q0, q1, [x0]
1008e328c: f841f108    	ldur	x8, [x8, #0x1f]
1008e3290: f801f008    	stur	x8, [x0, #0x1f]
1008e3294: 528004e8    	mov	w8, #0x27               ; =39
1008e3298: a90002a8    	stp	x8, x0, [x21]
1008e329c: f9000aa8    	str	x8, [x21, #0x10]
1008e32a0: 14000031    	b	0x1008e3364 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x424>
1008e32a4: f943c7e8    	ldr	x8, [sp, #0x788]
1008e32a8: f9000be8    	str	x8, [sp, #0x10]
1008e32ac: 911de3e8    	add	x8, sp, #0x778
1008e32b0: 910183e9    	add	x9, sp, #0x60
1008e32b4: 91006120    	add	x0, x9, #0x18
1008e32b8: 91006101    	add	x1, x8, #0x18
1008e32bc: 52802d02    	mov	w2, #0x168              ; =360
1008e32c0: 9423b8fb    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008e32c4: f9400bea    	ldr	x10, [sp, #0x10]
1008e32c8: a90663f9    	stp	x25, x24, [sp, #0x60]
1008e32cc: f9003bea    	str	x10, [sp, #0x70]
1008e32d0: b941cbe8    	ldr	w8, [sp, #0x1c8]
1008e32d4: b9416a89    	ldr	w9, [x20, #0x168]
1008e32d8: 6b09011f    	cmp	w8, w9
1008e32dc: 54000221    	b.ne	0x1008e3320 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x3e0>
1008e32e0: f9400a88    	ldr	x8, [x20, #0x10]
1008e32e4: eb08015f    	cmp	x10, x8
1008e32e8: 540001c1    	b.ne	0x1008e3320 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x3e0>
1008e32ec: f9400694    	ldr	x20, [x20, #0x8]
1008e32f0: d378dd48    	lsl	x8, x10, #8
1008e32f4: 8b080309    	add	x9, x24, x8
1008e32f8: 8b080299    	add	x25, x20, x8
1008e32fc: f903bff8    	str	x24, [sp, #0x778]
1008e3300: f903c3e9    	str	x9, [sp, #0x780]
1008e3304: f903c7f4    	str	x20, [sp, #0x788]
1008e3308: f903cbf9    	str	x25, [sp, #0x790]
1008e330c: f903cfff    	str	xzr, [sp, #0x798]
1008e3310: f903d3ea    	str	x10, [sp, #0x7a0]
1008e3314: 911de3e0    	add	x0, sp, #0x778
1008e3318: 940060bd    	bl	0x1008fb60c <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE>
1008e331c: 36000480    	tbz	w0, #0x0, 0x1008e33ac <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x46c>
1008e3320: 97dc7895    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e3324: 528005a0    	mov	w0, #0x2d               ; =45
1008e3328: 52800021    	mov	w1, #0x1                ; =1
1008e332c: 97dc788e    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e3330: b4003620    	cbz	x0, 0x1008e39f4 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xab4>
1008e3334: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e3338: 911f5908    	add	x8, x8, #0x7d6
1008e333c: ad400500    	ldp	q0, q1, [x8]
1008e3340: ad000400    	stp	q0, q1, [x0]
1008e3344: 3cc1d100    	ldur	q0, [x8, #0x1d]
1008e3348: 3c81d000    	stur	q0, [x0, #0x1d]
1008e334c: 528005a8    	mov	w8, #0x2d               ; =45
1008e3350: a90002a8    	stp	x8, x0, [x21]
1008e3354: f9000aa8    	str	x8, [x21, #0x10]
1008e3358: f94017f9    	ldr	x25, [sp, #0x28]
1008e335c: 910183e0    	add	x0, sp, #0x60
1008e3360: 940053ba    	bl	0x1008f8248 <__ZN4core3ptr67drop_in_place$LT$codexmate_lib..core..relay..models..RelayState$GT$17he3bbc346fbe4bfa7E>
1008e3364: f94023e1    	ldr	x1, [sp, #0x40]
1008e3368: b4000081    	cbz	x1, 0x1008e3378 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x438>
1008e336c: aa1903e0    	mov	x0, x25
1008e3370: 52800022    	mov	w2, #0x1                ; =1
1008e3374: 97dc787d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3378: f94027e1    	ldr	x1, [sp, #0x48]
1008e337c: b4000081    	cbz	x1, 0x1008e338c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x44c>
1008e3380: f9402be0    	ldr	x0, [sp, #0x50]
1008e3384: 52800022    	mov	w2, #0x1                ; =1
1008e3388: 97dc7878    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e338c: 912ec3ff    	add	sp, sp, #0xbb0
1008e3390: a9457bfd    	ldp	x29, x30, [sp, #0x50]
1008e3394: a9444ff4    	ldp	x20, x19, [sp, #0x40]
1008e3398: a94357f6    	ldp	x22, x21, [sp, #0x30]
1008e339c: a9425ff8    	ldp	x24, x23, [sp, #0x20]
1008e33a0: a94167fa    	ldp	x26, x25, [sp, #0x10]
1008e33a4: a8c66ffc    	ldp	x28, x27, [sp], #0x60
1008e33a8: d65f03c0    	ret
1008e33ac: f9010bf4    	str	x20, [sp, #0x210]
1008e33b0: f9010ff9    	str	x25, [sp, #0x218]
1008e33b4: f90113fa    	str	x26, [sp, #0x220]
1008e33b8: 911de3e0    	add	x0, sp, #0x778
1008e33bc: 910843e1    	add	x1, sp, #0x210
1008e33c0: f94017f9    	ldr	x25, [sp, #0x28]
1008e33c4: 940061e8    	bl	0x1008fbb64 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E>
1008e33c8: 910783fa    	add	x26, sp, #0x1e0
1008e33cc: f943bfe8    	ldr	x8, [sp, #0x778]
1008e33d0: 911a07e9    	add	x9, sp, #0x681
1008e33d4: 3ccff120    	ldur	q0, [x9, #0xff]
1008e33d8: 3d801b40    	str	q0, [x26, #0x60]
1008e33dc: f943cbe9    	ldr	x9, [sp, #0x790]
1008e33e0: f9012be9    	str	x9, [sp, #0x250]
1008e33e4: b4000448    	cbz	x8, 0x1008e346c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x52c>
1008e33e8: 911a67e9    	add	x9, sp, #0x699
1008e33ec: 3ccff120    	ldur	q0, [x9, #0xff]
1008e33f0: 3d800b40    	str	q0, [x26, #0x20]
1008e33f4: 3dc01b40    	ldr	q0, [x26, #0x60]
1008e33f8: 3c808340    	stur	q0, [x26, #0x8]
1008e33fc: f9412be9    	ldr	x9, [sp, #0x250]
1008e3400: f900ffe9    	str	x9, [sp, #0x1f8]
1008e3404: f900f3e8    	str	x8, [sp, #0x1e0]
1008e3408: 911de3e8    	add	x8, sp, #0x778
1008e340c: aa1c03e0    	mov	x0, x28
1008e3410: aa1303e1    	mov	x1, x19
1008e3414: 942194c3    	bl	0x101148720 <__ZN3std3sys2fs8read_dir17ha353ee9b9b706035E>
1008e3418: 395e03e8    	ldrb	w8, [sp, #0x780]
1008e341c: f943bfe0    	ldr	x0, [sp, #0x778]
1008e3420: 7100091f    	cmp	w8, #0x2
1008e3424: 540002e1    	b.ne	0x1008e3480 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x540>
1008e3428: 940001f0    	bl	0x1008e3be8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced28_$u7b$$u7b$closure$u7d$$u7d$17h1137491cfa7f6eb5E>
1008e342c: 97dc7852    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e3430: 52800573    	mov	w19, #0x2b              ; =43
1008e3434: 52800560    	mov	w0, #0x2b               ; =43
1008e3438: 52800021    	mov	w1, #0x1                ; =1
1008e343c: 97dc784a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e3440: b4002e20    	cbz	x0, 0x1008e3a04 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xac4>
1008e3444: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e3448: 91221508    	add	x8, x8, #0x885
1008e344c: ad400500    	ldp	q0, q1, [x8]
1008e3450: ad000400    	stp	q0, q1, [x0]
1008e3454: 3cc1b100    	ldur	q0, [x8, #0x1b]
1008e3458: 3c81b000    	stur	q0, [x0, #0x1b]
1008e345c: 52800568    	mov	w8, #0x2b               ; =43
1008e3460: a90002a8    	stp	x8, x0, [x21]
1008e3464: f9000aa8    	str	x8, [x21, #0x10]
1008e3468: 1400014d    	b	0x1008e399c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xa5c>
1008e346c: 3dc01b40    	ldr	q0, [x26, #0x60]
1008e3470: 3d8002a0    	str	q0, [x21]
1008e3474: f9412be8    	ldr	x8, [sp, #0x250]
1008e3478: f9000aa8    	str	x8, [x21, #0x10]
1008e347c: 17ffffb8    	b	0x1008e335c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x41c>
1008e3480: b9000fff    	str	wzr, [sp, #0xc]
1008e3484: 93407f69    	sxtw	x9, w27
1008e3488: f9000be9    	str	x9, [sp, #0x10]
1008e348c: 911de3fb    	add	x27, sp, #0x778
1008e3490: f90117ff    	str	xzr, [sp, #0x228]
1008e3494: f9011be0    	str	x0, [sp, #0x230]
1008e3498: 3908e3e8    	strb	w8, [sp, #0x238]
1008e349c: 911c03fc    	add	x28, sp, #0x700
1008e34a0: 911de3e8    	add	x8, sp, #0x778
1008e34a4: 9108a3e9    	add	x9, sp, #0x228
1008e34a8: 91002120    	add	x0, x9, #0x8
1008e34ac: 94221efa    	bl	0x10116b094 <__ZN75_$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hcbd8a6bdaabc6208E>
1008e34b0: b9477be8    	ldr	w8, [sp, #0x778]
1008e34b4: 36001608    	tbz	w8, #0x0, 0x1008e3774 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x834>
1008e34b8: 910903e0    	add	x0, sp, #0x240
1008e34bc: 91002361    	add	x1, x27, #0x8
1008e34c0: 52808402    	mov	w2, #0x420              ; =1056
1008e34c4: 9423b87a    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008e34c8: f94117e8    	ldr	x8, [sp, #0x228]
1008e34cc: 91000509    	add	x9, x8, #0x1
1008e34d0: f90117e9    	str	x9, [sp, #0x228]
1008e34d4: f13ffd1f    	cmp	x8, #0xfff
1008e34d8: 54001808    	b.hi	0x1008e37d8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x898>
1008e34dc: f94123f4    	ldr	x20, [sp, #0x240]
1008e34e0: f94127f3    	ldr	x19, [sp, #0x248]
1008e34e4: b4001a14    	cbz	x20, 0x1008e3824 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x8e4>
1008e34e8: f9412bf8    	ldr	x24, [sp, #0x250]
1008e34ec: 91006360    	add	x0, x27, #0x18
1008e34f0: 910903e8    	add	x8, sp, #0x240
1008e34f4: 91006101    	add	x1, x8, #0x18
1008e34f8: 52808102    	mov	w2, #0x408              ; =1032
1008e34fc: 9423b86c    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008e3500: f903bff4    	str	x20, [sp, #0x778]
1008e3504: f903c3f3    	str	x19, [sp, #0x780]
1008e3508: f903c7f8    	str	x24, [sp, #0x788]
1008e350c: 9119a3e8    	add	x8, sp, #0x668
1008e3510: 911de3e0    	add	x0, sp, #0x778
1008e3514: 94217d40    	bl	0x101142a14 <__ZN3std2fs8DirEntry4path17h69519f87704b953cE>
1008e3518: 910783e0    	add	x0, sp, #0x1e0
1008e351c: 9119a3e1    	add	x1, sp, #0x668
1008e3520: 97f23105    	bl	0x10056f934 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$12contains_key17hfb418cd54bd68858E>
1008e3524: f94013f8    	ldr	x24, [sp, #0x20]
1008e3528: 37000ec0    	tbnz	w0, #0x0, 0x1008e3700 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x7c0>
1008e352c: f9433be1    	ldr	x1, [sp, #0x670]
1008e3530: f9433fe2    	ldr	x2, [sp, #0x678]
1008e3534: 911c03e0    	add	x0, sp, #0x700
1008e3538: 97fffc97    	bl	0x1008e2794 <__ZN13codexmate_lib4core5relay8keychain9revisions11read_stored17h9ed1fa5c0405c861E>
1008e353c: f94383e8    	ldr	x8, [sp, #0x700]
1008e3540: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
1008e3544: eb09011f    	cmp	x8, x9
1008e3548: 54000c40    	b.eq	0x1008e36d0 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x790>
1008e354c: 3dc15340    	ldr	q0, [x26, #0x540]
1008e3550: 3d813340    	str	q0, [x26, #0x4c0]
1008e3554: 3dc15740    	ldr	q0, [x26, #0x550]
1008e3558: 3d813740    	str	q0, [x26, #0x4d0]
1008e355c: 3dc15b40    	ldr	q0, [x26, #0x560]
1008e3560: 3d813b40    	str	q0, [x26, #0x4e0]
1008e3564: 3dc15f40    	ldr	q0, [x26, #0x570]
1008e3568: 3d813f40    	str	q0, [x26, #0x4f0]
1008e356c: 3dc14b40    	ldr	q0, [x26, #0x520]
1008e3570: 3d812b40    	str	q0, [x26, #0x4a0]
1008e3574: 3dc14f40    	ldr	q0, [x26, #0x530]
1008e3578: 3d812f40    	str	q0, [x26, #0x4b0]
1008e357c: 3cc60380    	ldur	q0, [x28, #0x60]
1008e3580: 3d814340    	str	q0, [x26, #0x500]
1008e3584: f8470388    	ldur	x8, [x28, #0x70]
1008e3588: f9037be8    	str	x8, [sp, #0x6f0]
1008e358c: f94353e3    	ldr	x3, [sp, #0x6a0]
1008e3590: f94357e4    	ldr	x4, [sp, #0x6a8]
1008e3594: f9435fe5    	ldr	x5, [sp, #0x6b8]
1008e3598: f94363e6    	ldr	x6, [sp, #0x6c0]
1008e359c: 911c03e0    	add	x0, sp, #0x700
1008e35a0: aa1603e1    	mov	x1, x22
1008e35a4: aa1703e2    	mov	x2, x23
1008e35a8: 940006c7    	bl	0x1008e50c4 <__ZN13codexmate_lib4core5relay8keychain9revisions8key_path17hd5fc4e16eadecf41E>
1008e35ac: b94703e8    	ldr	w8, [sp, #0x700]
1008e35b0: f94387f4    	ldr	x20, [sp, #0x708]
1008e35b4: f9438bf3    	ldr	x19, [sp, #0x710]
1008e35b8: f9438fe1    	ldr	x1, [sp, #0x718]
1008e35bc: 37001bc8    	tbnz	w8, #0x0, 0x1008e3934 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x9f4>
1008e35c0: f9433be2    	ldr	x2, [sp, #0x670]
1008e35c4: f9433fe3    	ldr	x3, [sp, #0x678]
1008e35c8: aa1303e0    	mov	x0, x19
1008e35cc: 9400b2d3    	bl	0x100910118 <__ZN59_$LT$std..path..PathBuf$u20$as$u20$core..cmp..PartialEq$GT$2eq17h429a74dcfb9e7ed1E>
1008e35d0: 34000860    	cbz	w0, 0x1008e36dc <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x79c>
1008e35d4: b40000b4    	cbz	x20, 0x1008e35e8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x6a8>
1008e35d8: aa1303e0    	mov	x0, x19
1008e35dc: aa1403e1    	mov	x1, x20
1008e35e0: 52800022    	mov	w2, #0x1                ; =1
1008e35e4: 97dc77e1    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e35e8: a94507e0    	ldp	x0, x1, [sp, #0x50]
1008e35ec: 911c03e8    	add	x8, sp, #0x700
1008e35f0: 97e9095e    	bl	0x100325b68 <__ZN13codexmate_lib4core5relay12atomic_write7staging12FileIdentity14read_directory17h6e444a1c818d5233E>
1008e35f4: b94703e8    	ldr	w8, [sp, #0x700]
1008e35f8: f94387e0    	ldr	x0, [sp, #0x708]
1008e35fc: 7100091f    	cmp	w8, #0x2
1008e3600: 54001340    	b.eq	0x1008e3868 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x928>
1008e3604: 35001348    	cbnz	w8, 0x1008e386c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x92c>
1008e3608: f9400be8    	ldr	x8, [sp, #0x10]
1008e360c: eb08001f    	cmp	x0, x8
1008e3610: 540012e1    	b.ne	0x1008e386c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x92c>
1008e3614: f9438be8    	ldr	x8, [sp, #0x710]
1008e3618: f9400fe9    	ldr	x9, [sp, #0x18]
1008e361c: eb09011f    	cmp	x8, x9
1008e3620: 54001261    	b.ne	0x1008e386c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x92c>
1008e3624: 911c03e8    	add	x8, sp, #0x700
1008e3628: a94303e1    	ldp	x1, x0, [sp, #0x30]
1008e362c: 94217c08    	bl	0x10114264c <__ZN3std2fs4read5inner17hebd695181b114834E>
1008e3630: f94383f4    	ldr	x20, [sp, #0x700]
1008e3634: f94387f3    	ldr	x19, [sp, #0x708]
1008e3638: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008e363c: eb08029f    	cmp	x20, x8
1008e3640: 54001360    	b.eq	0x1008e38ac <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x96c>
1008e3644: f9438be8    	ldr	x8, [sp, #0x710]
1008e3648: eb18011f    	cmp	x8, x24
1008e364c: 54000761    	b.ne	0x1008e3738 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x7f8>
1008e3650: aa1303e0    	mov	x0, x19
1008e3654: aa1903e1    	mov	x1, x25
1008e3658: aa1803e2    	mov	x2, x24
1008e365c: 9423b811    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1008e3660: 350006c0    	cbnz	w0, 0x1008e3738 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x7f8>
1008e3664: b40000b4    	cbz	x20, 0x1008e3678 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x738>
1008e3668: aa1303e0    	mov	x0, x19
1008e366c: aa1403e1    	mov	x1, x20
1008e3670: 52800022    	mov	w2, #0x1                ; =1
1008e3674: 97dc77bd    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3678: f9433bf3    	ldr	x19, [sp, #0x670]
1008e367c: f9433fe1    	ldr	x1, [sp, #0x678]
1008e3680: 911b83e2    	add	x2, sp, #0x6e0
1008e3684: aa1303e0    	mov	x0, x19
1008e3688: 97e9097e    	bl	0x100325c80 <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE>
1008e368c: b5001340    	cbnz	x0, 0x1008e38f4 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x9b4>
1008e3690: 911a03e0    	add	x0, sp, #0x680
1008e3694: 9400586d    	bl	0x1008f9848 <__ZN4core3ptr79drop_in_place$LT$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$17h06beddeda09cc401E>
1008e3698: f94337e1    	ldr	x1, [sp, #0x668]
1008e369c: b4000081    	cbz	x1, 0x1008e36ac <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x76c>
1008e36a0: aa1303e0    	mov	x0, x19
1008e36a4: 52800022    	mov	w2, #0x1                ; =1
1008e36a8: 97dc77b0    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e36ac: f943bfe8    	ldr	x8, [sp, #0x778]
1008e36b0: 92800009    	mov	x9, #-0x1               ; =-1
1008e36b4: f8690108    	ldaddl	x9, x8, [x8]
1008e36b8: f100051f    	cmp	x8, #0x1
1008e36bc: 54ffef21    	b.ne	0x1008e34a0 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x560>
1008e36c0: d50339bf    	dmb	ishld
1008e36c4: 911de3e0    	add	x0, sp, #0x778
1008e36c8: 9421fbeb    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008e36cc: 17ffff75    	b	0x1008e34a0 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x560>
1008e36d0: 911c03e0    	add	x0, sp, #0x700
1008e36d4: 94004a37    	bl	0x1008f5fb0 <__ZN4core3ptr205drop_in_place$LT$core..result..Result$LT$$LP$codexmate_lib..core..relay..keychain..revisions..StoredKey$C$codexmate_lib..core..relay..atomic_write..staging..FileIdentity$RP$$C$alloc..string..String$GT$$GT$17hb9a78152ac8c5c9fE>
1008e36d8: 14000008    	b	0x1008e36f8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x7b8>
1008e36dc: b40000b4    	cbz	x20, 0x1008e36f0 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x7b0>
1008e36e0: aa1303e0    	mov	x0, x19
1008e36e4: aa1403e1    	mov	x1, x20
1008e36e8: 52800022    	mov	w2, #0x1                ; =1
1008e36ec: 97dc779f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e36f0: 911a03e0    	add	x0, sp, #0x680
1008e36f4: 94005855    	bl	0x1008f9848 <__ZN4core3ptr79drop_in_place$LT$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$17h06beddeda09cc401E>
1008e36f8: 52800028    	mov	w8, #0x1                ; =1
1008e36fc: b9000fe8    	str	w8, [sp, #0xc]
1008e3700: f94337e1    	ldr	x1, [sp, #0x668]
1008e3704: b4000081    	cbz	x1, 0x1008e3714 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x7d4>
1008e3708: f9433be0    	ldr	x0, [sp, #0x670]
1008e370c: 52800022    	mov	w2, #0x1                ; =1
1008e3710: 97dc7796    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3714: f943bfe8    	ldr	x8, [sp, #0x778]
1008e3718: 92800009    	mov	x9, #-0x1               ; =-1
1008e371c: f8690108    	ldaddl	x9, x8, [x8]
1008e3720: f100051f    	cmp	x8, #0x1
1008e3724: 54ffebe1    	b.ne	0x1008e34a0 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x560>
1008e3728: d50339bf    	dmb	ishld
1008e372c: 911de3e0    	add	x0, sp, #0x778
1008e3730: 9421fbd1    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008e3734: 17ffff5b    	b	0x1008e34a0 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x560>
1008e3738: b40000b4    	cbz	x20, 0x1008e374c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x80c>
1008e373c: aa1303e0    	mov	x0, x19
1008e3740: aa1403e1    	mov	x1, x20
1008e3744: 52800022    	mov	w2, #0x1                ; =1
1008e3748: 97dc7788    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e374c: 97dc778a    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e3750: 528005b4    	mov	w20, #0x2d              ; =45
1008e3754: 528005a0    	mov	w0, #0x2d               ; =45
1008e3758: 52800021    	mov	w1, #0x1                ; =1
1008e375c: 97dc7782    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e3760: b40015a0    	cbz	x0, 0x1008e3a14 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xad4>
1008e3764: aa0003f3    	mov	x19, x0
1008e3768: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e376c: 911d1508    	add	x8, x8, #0x745
1008e3770: 1400005a    	b	0x1008e38d8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x998>
1008e3774: f9411be8    	ldr	x8, [sp, #0x230]
1008e3778: 92800009    	mov	x9, #-0x1               ; =-1
1008e377c: f8690108    	ldaddl	x9, x8, [x8]
1008e3780: f100051f    	cmp	x8, #0x1
1008e3784: 540000a1    	b.ne	0x1008e3798 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x858>
1008e3788: d50339bf    	dmb	ishld
1008e378c: 9108a3e8    	add	x8, sp, #0x228
1008e3790: 91002100    	add	x0, x8, #0x8
1008e3794: 9421fbb8    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008e3798: b9400fe8    	ldr	w8, [sp, #0xc]
1008e379c: 36001068    	tbz	w8, #0x0, 0x1008e39a8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xa68>
1008e37a0: 97dc7775    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e37a4: 52800593    	mov	w19, #0x2c              ; =44
1008e37a8: 52800580    	mov	w0, #0x2c               ; =44
1008e37ac: 52800021    	mov	w1, #0x1                ; =1
1008e37b0: 97dc776d    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e37b4: b4001280    	cbz	x0, 0x1008e3a04 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xac4>
1008e37b8: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e37bc: 911c6508    	add	x8, x8, #0x719
1008e37c0: ad400500    	ldp	q0, q1, [x8]
1008e37c4: ad000400    	stp	q0, q1, [x0]
1008e37c8: 3cc1c100    	ldur	q0, [x8, #0x1c]
1008e37cc: 3c81c000    	stur	q0, [x0, #0x1c]
1008e37d0: 52800588    	mov	w8, #0x2c               ; =44
1008e37d4: 17ffff23    	b	0x1008e3460 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x520>
1008e37d8: 97dc7767    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e37dc: 52800840    	mov	w0, #0x42               ; =66
1008e37e0: 52800021    	mov	w1, #0x1                ; =1
1008e37e4: 97dc7760    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e37e8: b40011e0    	cbz	x0, 0x1008e3a24 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xae4>
1008e37ec: 528c8ca8    	mov	w8, #0x6465             ; =25701
1008e37f0: 79008008    	strh	w8, [x0, #0x40]
1008e37f4: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e37f8: 911e5108    	add	x8, x8, #0x794
1008e37fc: ad400500    	ldp	q0, q1, [x8]
1008e3800: ad000400    	stp	q0, q1, [x0]
1008e3804: ad410500    	ldp	q0, q1, [x8, #0x20]
1008e3808: ad010400    	stp	q0, q1, [x0, #0x20]
1008e380c: 52800848    	mov	w8, #0x42               ; =66
1008e3810: a90002a8    	stp	x8, x0, [x21]
1008e3814: f9000aa8    	str	x8, [x21, #0x10]
1008e3818: 910903e0    	add	x0, sp, #0x240
1008e381c: 94005a71    	bl	0x1008fa1e0 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..DirEntry$C$std..io..error..Error$GT$$GT$17h7dc6b9c47bdabf5eE>
1008e3820: 14000056    	b	0x1008e3978 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xa38>
1008e3824: aa1303e0    	mov	x0, x19
1008e3828: 940000f0    	bl	0x1008e3be8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced28_$u7b$$u7b$closure$u7d$$u7d$17h1137491cfa7f6eb5E>
1008e382c: 97dc7752    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e3830: 52800500    	mov	w0, #0x28               ; =40
1008e3834: 52800021    	mov	w1, #0x1                ; =1
1008e3838: 97dc774b    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e383c: b4000fc0    	cbz	x0, 0x1008e3a34 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xaf4>
1008e3840: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e3844: 9123fd08    	add	x8, x8, #0x8ff
1008e3848: ad400500    	ldp	q0, q1, [x8]
1008e384c: ad000400    	stp	q0, q1, [x0]
1008e3850: f9401108    	ldr	x8, [x8, #0x20]
1008e3854: f9001008    	str	x8, [x0, #0x20]
1008e3858: 52800508    	mov	w8, #0x28               ; =40
1008e385c: a90002a8    	stp	x8, x0, [x21]
1008e3860: f9000aa8    	str	x8, [x21, #0x10]
1008e3864: 14000045    	b	0x1008e3978 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xa38>
1008e3868: 940000e0    	bl	0x1008e3be8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced28_$u7b$$u7b$closure$u7d$$u7d$17h1137491cfa7f6eb5E>
1008e386c: 97dc7742    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e3870: 52800454    	mov	w20, #0x22              ; =34
1008e3874: 52800440    	mov	w0, #0x22               ; =34
1008e3878: 52800021    	mov	w1, #0x1                ; =1
1008e387c: 97dc773a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e3880: b4000ca0    	cbz	x0, 0x1008e3a14 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xad4>
1008e3884: aa0003f3    	mov	x19, x0
1008e3888: 528c8ca8    	mov	w8, #0x6465             ; =25701
1008e388c: 79004008    	strh	w8, [x0, #0x20]
1008e3890: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e3894: 911dc908    	add	x8, x8, #0x772
1008e3898: ad400500    	ldp	q0, q1, [x8]
1008e389c: ad000400    	stp	q0, q1, [x0]
1008e38a0: 52800454    	mov	w20, #0x22              ; =34
1008e38a4: 52800441    	mov	w1, #0x22               ; =34
1008e38a8: 14000023    	b	0x1008e3934 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x9f4>
1008e38ac: aa1303e0    	mov	x0, x19
1008e38b0: 940000ce    	bl	0x1008e3be8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced28_$u7b$$u7b$closure$u7d$$u7d$17h1137491cfa7f6eb5E>
1008e38b4: 97dc7730    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e38b8: 528005b4    	mov	w20, #0x2d              ; =45
1008e38bc: 528005a0    	mov	w0, #0x2d               ; =45
1008e38c0: 52800021    	mov	w1, #0x1                ; =1
1008e38c4: 97dc7728    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e38c8: b4000a60    	cbz	x0, 0x1008e3a14 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xad4>
1008e38cc: aa0003f3    	mov	x19, x0
1008e38d0: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e38d4: 91234908    	add	x8, x8, #0x8d2
1008e38d8: ad400500    	ldp	q0, q1, [x8]
1008e38dc: ad000660    	stp	q0, q1, [x19]
1008e38e0: 3cc1d100    	ldur	q0, [x8, #0x1d]
1008e38e4: 3c81d260    	stur	q0, [x19, #0x1d]
1008e38e8: 528005b4    	mov	w20, #0x2d              ; =45
1008e38ec: 528005a1    	mov	w1, #0x2d               ; =45
1008e38f0: 14000011    	b	0x1008e3934 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x9f4>
1008e38f4: 940000bd    	bl	0x1008e3be8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced28_$u7b$$u7b$closure$u7d$$u7d$17h1137491cfa7f6eb5E>
1008e38f8: 97dc771f    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008e38fc: 52800594    	mov	w20, #0x2c              ; =44
1008e3900: 52800580    	mov	w0, #0x2c               ; =44
1008e3904: 52800021    	mov	w1, #0x1                ; =1
1008e3908: 97dc7717    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008e390c: b4000840    	cbz	x0, 0x1008e3a14 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xad4>
1008e3910: aa0003f3    	mov	x19, x0
1008e3914: f0005688    	adrp	x8, 0x1013b6000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.831+0x8a8>
1008e3918: 91216508    	add	x8, x8, #0x859
1008e391c: ad400500    	ldp	q0, q1, [x8]
1008e3920: ad000400    	stp	q0, q1, [x0]
1008e3924: 3cc1c100    	ldur	q0, [x8, #0x1c]
1008e3928: 3c81c000    	stur	q0, [x0, #0x1c]
1008e392c: 52800594    	mov	w20, #0x2c              ; =44
1008e3930: 52800581    	mov	w1, #0x2c               ; =44
1008e3934: a9004eb4    	stp	x20, x19, [x21]
1008e3938: f9000aa1    	str	x1, [x21, #0x10]
1008e393c: 911a03e0    	add	x0, sp, #0x680
1008e3940: 940057c2    	bl	0x1008f9848 <__ZN4core3ptr79drop_in_place$LT$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$17h06beddeda09cc401E>
1008e3944: f94337e1    	ldr	x1, [sp, #0x668]
1008e3948: b4000081    	cbz	x1, 0x1008e3958 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xa18>
1008e394c: f9433be0    	ldr	x0, [sp, #0x670]
1008e3950: 52800022    	mov	w2, #0x1                ; =1
1008e3954: 97dc7705    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3958: f943bfe8    	ldr	x8, [sp, #0x778]
1008e395c: 92800009    	mov	x9, #-0x1               ; =-1
1008e3960: f8690108    	ldaddl	x9, x8, [x8]
1008e3964: f100051f    	cmp	x8, #0x1
1008e3968: 54000081    	b.ne	0x1008e3978 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xa38>
1008e396c: d50339bf    	dmb	ishld
1008e3970: 911de3e0    	add	x0, sp, #0x778
1008e3974: 9421fb40    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008e3978: f9411be8    	ldr	x8, [sp, #0x230]
1008e397c: 92800009    	mov	x9, #-0x1               ; =-1
1008e3980: f8690108    	ldaddl	x9, x8, [x8]
1008e3984: f100051f    	cmp	x8, #0x1
1008e3988: 540000a1    	b.ne	0x1008e399c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xa5c>
1008e398c: d50339bf    	dmb	ishld
1008e3990: 9108a3e8    	add	x8, sp, #0x228
1008e3994: 91002100    	add	x0, x8, #0x8
1008e3998: 9421fb37    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008e399c: 910783e0    	add	x0, sp, #0x1e0
1008e39a0: 97fe31dd    	bl	0x100870114 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9aa9677d07c4e8b5E>
1008e39a4: 17fffe6e    	b	0x1008e335c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x41c>
1008e39a8: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008e39ac: f90002a8    	str	x8, [x21]
1008e39b0: 910783e0    	add	x0, sp, #0x1e0
1008e39b4: 97fe31d8    	bl	0x100870114 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9aa9677d07c4e8b5E>
1008e39b8: 910183e0    	add	x0, sp, #0x60
1008e39bc: 94005223    	bl	0x1008f8248 <__ZN4core3ptr67drop_in_place$LT$codexmate_lib..core..relay..models..RelayState$GT$17he3bbc346fbe4bfa7E>
1008e39c0: f94023e8    	ldr	x8, [sp, #0x40]
1008e39c4: b4ffcda8    	cbz	x8, 0x1008e3378 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x438>
1008e39c8: f94017e0    	ldr	x0, [sp, #0x28]
1008e39cc: f94023e1    	ldr	x1, [sp, #0x40]
1008e39d0: 17fffe68    	b	0x1008e3370 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0x430>
1008e39d4: 52800020    	mov	w0, #0x1                ; =1
1008e39d8: 52800521    	mov	w1, #0x29               ; =41
1008e39dc: 9423b322    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008e39e0: 14000018    	b	0x1008e3a40 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb00>
1008e39e4: 52800020    	mov	w0, #0x1                ; =1
1008e39e8: 528004e1    	mov	w1, #0x27               ; =39
1008e39ec: 9423b31e    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008e39f0: 14000014    	b	0x1008e3a40 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb00>
1008e39f4: 52800020    	mov	w0, #0x1                ; =1
1008e39f8: 528005a1    	mov	w1, #0x2d               ; =45
1008e39fc: 9423b31a    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008e3a00: 14000010    	b	0x1008e3a40 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb00>
1008e3a04: 52800020    	mov	w0, #0x1                ; =1
1008e3a08: aa1303e1    	mov	x1, x19
1008e3a0c: 9423b316    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008e3a10: 1400000c    	b	0x1008e3a40 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb00>
1008e3a14: 52800020    	mov	w0, #0x1                ; =1
1008e3a18: aa1403e1    	mov	x1, x20
1008e3a1c: 9423b312    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008e3a20: 14000008    	b	0x1008e3a40 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb00>
1008e3a24: 52800020    	mov	w0, #0x1                ; =1
1008e3a28: 52800841    	mov	w1, #0x42               ; =66
1008e3a2c: 9423b30e    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008e3a30: 14000004    	b	0x1008e3a40 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb00>
1008e3a34: 52800020    	mov	w0, #0x1                ; =1
1008e3a38: 52800501    	mov	w1, #0x28               ; =40
1008e3a3c: 9423b30a    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008e3a40: d4200020    	brk	#0x1
1008e3a44: 14000012    	b	0x1008e3a8c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb4c>
1008e3a48: aa0003f5    	mov	x21, x0
1008e3a4c: 910903e0    	add	x0, sp, #0x240
1008e3a50: 940059e4    	bl	0x1008fa1e0 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..DirEntry$C$std..io..error..Error$GT$$GT$17h7dc6b9c47bdabf5eE>
1008e3a54: 1400002c    	b	0x1008e3b04 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xbc4>
1008e3a58: 1400000d    	b	0x1008e3a8c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb4c>
1008e3a5c: 1400000c    	b	0x1008e3a8c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb4c>
1008e3a60: 14000019    	b	0x1008e3ac4 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb84>
1008e3a64: 1400000a    	b	0x1008e3a8c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb4c>
1008e3a68: aa0003f5    	mov	x21, x0
1008e3a6c: b40002f4    	cbz	x20, 0x1008e3ac8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb88>
1008e3a70: aa1303e0    	mov	x0, x19
1008e3a74: aa1403e1    	mov	x1, x20
1008e3a78: 52800022    	mov	w2, #0x1                ; =1
1008e3a7c: 97dc76bb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3a80: 14000012    	b	0x1008e3ac8 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb88>
1008e3a84: aa0003f5    	mov	x21, x0
1008e3a88: 14000017    	b	0x1008e3ae4 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xba4>
1008e3a8c: aa0003f5    	mov	x21, x0
1008e3a90: 1400001d    	b	0x1008e3b04 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xbc4>
1008e3a94: aa0003f5    	mov	x21, x0
1008e3a98: 14000024    	b	0x1008e3b28 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xbe8>
1008e3a9c: aa0003f5    	mov	x21, x0
1008e3aa0: 1400000c    	b	0x1008e3ad0 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb90>
1008e3aa4: aa0003f5    	mov	x21, x0
1008e3aa8: f9400681    	ldr	x1, [x20, #0x8]
1008e3aac: b4000081    	cbz	x1, 0x1008e3abc <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xb7c>
1008e3ab0: f9400a82    	ldr	x2, [x20, #0x10]
1008e3ab4: aa1303e0    	mov	x0, x19
1008e3ab8: 97dc76ac    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3abc: aa1803e0    	mov	x0, x24
1008e3ac0: 14000031    	b	0x1008e3b84 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc44>
1008e3ac4: aa0003f5    	mov	x21, x0
1008e3ac8: 911a03e0    	add	x0, sp, #0x680
1008e3acc: 9400575f    	bl	0x1008f9848 <__ZN4core3ptr79drop_in_place$LT$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$17h06beddeda09cc401E>
1008e3ad0: f94337e1    	ldr	x1, [sp, #0x668]
1008e3ad4: b4000081    	cbz	x1, 0x1008e3ae4 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xba4>
1008e3ad8: f9433be0    	ldr	x0, [sp, #0x670]
1008e3adc: 52800022    	mov	w2, #0x1                ; =1
1008e3ae0: 97dc76a2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3ae4: f943bfe8    	ldr	x8, [sp, #0x778]
1008e3ae8: 92800009    	mov	x9, #-0x1               ; =-1
1008e3aec: f8690108    	ldaddl	x9, x8, [x8]
1008e3af0: f100051f    	cmp	x8, #0x1
1008e3af4: 54000081    	b.ne	0x1008e3b04 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xbc4>
1008e3af8: d50339bf    	dmb	ishld
1008e3afc: 911de3e0    	add	x0, sp, #0x778
1008e3b00: 9421fadd    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008e3b04: f9411be8    	ldr	x8, [sp, #0x230]
1008e3b08: 92800009    	mov	x9, #-0x1               ; =-1
1008e3b0c: f8690108    	ldaddl	x9, x8, [x8]
1008e3b10: f100051f    	cmp	x8, #0x1
1008e3b14: 540000a1    	b.ne	0x1008e3b28 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xbe8>
1008e3b18: d50339bf    	dmb	ishld
1008e3b1c: 9108a3e8    	add	x8, sp, #0x228
1008e3b20: 91002100    	add	x0, x8, #0x8
1008e3b24: 9421fad4    	bl	0x101162674 <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hc70d573bd026517aE>
1008e3b28: 910783e0    	add	x0, sp, #0x1e0
1008e3b2c: 97fe317a    	bl	0x100870114 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9aa9677d07c4e8b5E>
1008e3b30: 14000003    	b	0x1008e3b3c <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xbfc>
1008e3b34: 14000005    	b	0x1008e3b48 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc08>
1008e3b38: aa0003f5    	mov	x21, x0
1008e3b3c: 910183e0    	add	x0, sp, #0x60
1008e3b40: 940051c2    	bl	0x1008f8248 <__ZN4core3ptr67drop_in_place$LT$codexmate_lib..core..relay..models..RelayState$GT$17he3bbc346fbe4bfa7E>
1008e3b44: 14000015    	b	0x1008e3b98 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc58>
1008e3b48: aa0003f5    	mov	x21, x0
1008e3b4c: f94006c1    	ldr	x1, [x22, #0x8]
1008e3b50: b4000081    	cbz	x1, 0x1008e3b60 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc20>
1008e3b54: f9400ac2    	ldr	x2, [x22, #0x10]
1008e3b58: aa1403e0    	mov	x0, x20
1008e3b5c: 97dc7683    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3b60: aa1303e0    	mov	x0, x19
1008e3b64: 14000008    	b	0x1008e3b84 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc44>
1008e3b68: aa0003f5    	mov	x21, x0
1008e3b6c: f9400681    	ldr	x1, [x20, #0x8]
1008e3b70: b4000081    	cbz	x1, 0x1008e3b80 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc40>
1008e3b74: f9400a82    	ldr	x2, [x20, #0x10]
1008e3b78: aa1303e0    	mov	x0, x19
1008e3b7c: 97dc767b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3b80: aa1b03e0    	mov	x0, x27
1008e3b84: 52800301    	mov	w1, #0x18               ; =24
1008e3b88: 52800102    	mov	w2, #0x8                ; =8
1008e3b8c: 97dc7677    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3b90: 1400000f    	b	0x1008e3bcc <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc8c>
1008e3b94: aa0003f5    	mov	x21, x0
1008e3b98: f94023e8    	ldr	x8, [sp, #0x40]
1008e3b9c: b4000188    	cbz	x8, 0x1008e3bcc <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc8c>
1008e3ba0: f94017e0    	ldr	x0, [sp, #0x28]
1008e3ba4: f94023e1    	ldr	x1, [sp, #0x40]
1008e3ba8: 52800022    	mov	w2, #0x1                ; =1
1008e3bac: 97dc766f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3bb0: 14000007    	b	0x1008e3bcc <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc8c>
1008e3bb4: aa0003f5    	mov	x21, x0
1008e3bb8: aa1303e0    	mov	x0, x19
1008e3bbc: 94004a8a    	bl	0x1008f65e4 <__ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h8fc6649f423239fcE>
1008e3bc0: 14000003    	b	0x1008e3bcc <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xc8c>
1008e3bc4: 9423b3e1    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
1008e3bc8: aa0003f5    	mov	x21, x0
1008e3bcc: f94027e1    	ldr	x1, [sp, #0x48]
1008e3bd0: b4000081    	cbz	x1, 0x1008e3be0 <__ZN13codexmate_lib4core5relay8keychain9revisions20cleanup_unreferenced17h9ab0525d1076f87cE+0xca0>
1008e3bd4: f9402be0    	ldr	x0, [sp, #0x50]
1008e3bd8: 52800022    	mov	w2, #0x1                ; =1
1008e3bdc: 97dc7663    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008e3be0: aa1503e0    	mov	x0, x21
1008e3be4: 9423b55c    	bl	0x1011d1154 <_writev+0x1011d1154>
