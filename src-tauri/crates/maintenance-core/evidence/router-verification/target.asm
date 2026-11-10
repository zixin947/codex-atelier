
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010068704c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E>:
10068704c: d103c3ff    	sub	sp, sp, #0xf0
100687050: a9096ffc    	stp	x28, x27, [sp, #0x90]
100687054: a90a67fa    	stp	x26, x25, [sp, #0xa0]
100687058: a90b5ff8    	stp	x24, x23, [sp, #0xb0]
10068705c: a90c57f6    	stp	x22, x21, [sp, #0xc0]
100687060: a90d4ff4    	stp	x20, x19, [sp, #0xd0]
100687064: a90e7bfd    	stp	x29, x30, [sp, #0xe0]
100687068: 910383fd    	add	x29, sp, #0xe0
10068706c: aa0303f3    	mov	x19, x3
100687070: aa0203f8    	mov	x24, x2
100687074: aa0103f5    	mov	x21, x1
100687078: aa0003f6    	mov	x22, x0
10068707c: 910063e8    	add	x8, sp, #0x18
100687080: aa0203e0    	mov	x0, x2
100687084: aa0303e1    	mov	x1, x3
100687088: aa0403e2    	mov	x2, x4
10068708c: 97f36fed    	bl	0x100363040 <__ZN13codexmate_lib4core5relay10invariants28verify_invariants_with_depth17h517a2cae31c9c842E>
100687090: 360000d3    	tbz	w19, #0x0, 0x1006870a8 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x5c>
100687094: 3cc183e0    	ldur	q0, [sp, #0x18]
100687098: 3d8002c0    	str	q0, [x22]
10068709c: f94017e8    	ldr	x8, [sp, #0x28]
1006870a0: f9000ac8    	str	x8, [x22, #0x10]
1006870a4: 140000c0    	b	0x1006873a4 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x358>
1006870a8: 910042b3    	add	x19, x21, #0x10
1006870ac: d94102a0    	ldapur	x0, [x21, #0x10]
1006870b0: b4000600    	cbz	x0, 0x100687170 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x124>
1006870b4: 942b0eae    	bl	0x10114ab6c <__ZN3std3sys3pal4unix4sync5mutex5Mutex4lock17h8bb1e91d7d7c25e1E>
1006870b8: b000937b    	adrp	x27, 0x1018f4000 <__ZN15crossbeam_utils6atomic11atomic_cell4lock5LOCKS17h71d6ab0bc78afdcbE+0x1b00>
1006870bc: 9126637b    	add	x27, x27, #0x998
1006870c0: f9400368    	ldr	x8, [x27]
1006870c4: f240f91f    	tst	x8, #0x7fffffffffffffff
1006870c8: 540005a1    	b.ne	0x10068717c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x130>
1006870cc: 52800014    	mov	w20, #0x0               ; =0
1006870d0: 394062a8    	ldrb	w8, [x21, #0x18]
1006870d4: 7100011f    	cmp	w8, #0x0
1006870d8: 1a9f07e9    	cset	w9, ne
1006870dc: 390203f4    	strb	w20, [sp, #0x80]
1006870e0: a9074fe9    	stp	x9, x19, [sp, #0x70]
1006870e4: 340005c8    	cbz	w8, 0x10068719c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x150>
1006870e8: 97e5e923    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1006870ec: 52800940    	mov	w0, #0x4a               ; =74
1006870f0: 52800021    	mov	w1, #0x1                ; =1
1006870f4: 97e5e91c    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1006870f8: b4001800    	cbz	x0, 0x1006873f8 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x3ac>
1006870fc: aa0003f7    	mov	x23, x0
100687100: 900068e8    	adrp	x8, 0x1013a3000 <_anon.1f4a05fd1dc96b9982818105670d4dce.533+0x1cd>
100687104: 9119f108    	add	x8, x8, #0x67c
100687108: ad410500    	ldp	q0, q1, [x8, #0x20]
10068710c: ad010400    	stp	q0, q1, [x0, #0x20]
100687110: 3cc3a100    	ldur	q0, [x8, #0x3a]
100687114: 3c83a000    	stur	q0, [x0, #0x3a]
100687118: ad400101    	ldp	q1, q0, [x8]
10068711c: ad000001    	stp	q1, q0, [x0]
100687120: f94017f8    	ldr	x24, [sp, #0x28]
100687124: f9400fe8    	ldr	x8, [sp, #0x18]
100687128: eb08031f    	cmp	x24, x8
10068712c: 54000061    	b.ne	0x100687138 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0xec>
100687130: 910063e0    	add	x0, sp, #0x18
100687134: 942c1adf    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100687138: f94013e8    	ldr	x8, [sp, #0x20]
10068713c: 8b181508    	add	x8, x8, x24, lsl #5
100687140: 52800949    	mov	w9, #0x4a               ; =74
100687144: a9005d09    	stp	x9, x23, [x8]
100687148: f9000909    	str	x9, [x8, #0x10]
10068714c: 52800089    	mov	w9, #0x4                ; =4
100687150: 39006109    	strb	w9, [x8, #0x18]
100687154: 91000708    	add	x8, x24, #0x1
100687158: f90017e8    	str	x8, [sp, #0x28]
10068715c: f9000ac8    	str	x8, [x22, #0x10]
100687160: 3cc183e0    	ldur	q0, [sp, #0x18]
100687164: 3d8002c0    	str	q0, [x22]
100687168: 36001154    	tbz	w20, #0x0, 0x100687390 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x344>
10068716c: 1400008c    	b	0x10068739c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x350>
100687170: aa1303e0    	mov	x0, x19
100687174: 942c64e9    	bl	0x1011a0518 <__ZN3std3sys4sync8once_box16OnceBox$LT$T$GT$10initialize17h95d0a095904ca1eaE>
100687178: 17ffffcf    	b	0x1006870b4 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x68>
10068717c: 942d1754    	bl	0x1011ccecc <__ZN3std9panicking11panic_count17is_zero_slow_path17hbe4f1645e48c7385E>
100687180: 52000014    	eor	w20, w0, #0x1
100687184: 394062a8    	ldrb	w8, [x21, #0x18]
100687188: 7100011f    	cmp	w8, #0x0
10068718c: 1a9f07e9    	cset	w9, ne
100687190: 390203f4    	strb	w20, [sp, #0x80]
100687194: a9074fe9    	stp	x9, x19, [sp, #0x70]
100687198: 35fffa88    	cbnz	w8, 0x1006870e8 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x9c>
10068719c: aa1503f7    	mov	x23, x21
1006871a0: f8420ee8    	ldr	x8, [x23, #0x20]!
1006871a4: b4000128    	cbz	x8, 0x1006871c8 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x17c>
1006871a8: a94223e9    	ldp	x9, x8, [sp, #0x20]
1006871ac: d37be908    	lsl	x8, x8, #5
1006871b0: b4000388    	cbz	x8, 0x100687220 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x1d4>
1006871b4: 3940612a    	ldrb	w10, [x9, #0x18]
1006871b8: 91008129    	add	x9, x9, #0x20
1006871bc: d1008108    	sub	x8, x8, #0x20
1006871c0: 35ffff8a    	cbnz	w10, 0x1006871b0 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x164>
1006871c4: 14000052    	b	0x10068730c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x2c0>
1006871c8: 97e5e8eb    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1006871cc: 52800560    	mov	w0, #0x2b               ; =43
1006871d0: 52800021    	mov	w1, #0x1                ; =1
1006871d4: 97e5e8e4    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1006871d8: b4001180    	cbz	x0, 0x100687408 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x3bc>
1006871dc: aa0003f7    	mov	x23, x0
1006871e0: 900068e8    	adrp	x8, 0x1013a3000 <_anon.1f4a05fd1dc96b9982818105670d4dce.533+0x1cd>
1006871e4: 91194508    	add	x8, x8, #0x651
1006871e8: ad400500    	ldp	q0, q1, [x8]
1006871ec: ad000400    	stp	q0, q1, [x0]
1006871f0: 3cc1b100    	ldur	q0, [x8, #0x1b]
1006871f4: 3c81b000    	stur	q0, [x0, #0x1b]
1006871f8: f94017f8    	ldr	x24, [sp, #0x28]
1006871fc: f9400fe8    	ldr	x8, [sp, #0x18]
100687200: eb08031f    	cmp	x24, x8
100687204: 54000061    	b.ne	0x100687210 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x1c4>
100687208: 910063e0    	add	x0, sp, #0x18
10068720c: 942c1aa9    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
100687210: f94013e8    	ldr	x8, [sp, #0x20]
100687214: 8b181508    	add	x8, x8, x24, lsl #5
100687218: 52800569    	mov	w9, #0x2b               ; =43
10068721c: 17ffffca    	b	0x100687144 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0xf8>
100687220: 900057a8    	adrp	x8, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
100687224: 91079108    	add	x8, x8, #0x1e4
100687228: 9100c2a9    	add	x9, x21, #0x30
10068722c: a90423e9    	stp	x9, x8, [sp, #0x40]
100687230: 90007500    	adrp	x0, 0x101527000 <_anon.62728874ff9674d7d8f5de25cb1e85f1.1160+0x10>
100687234: 912fb000    	add	x0, x0, #0xbec
100687238: 9101c3f9    	add	x25, sp, #0x70
10068723c: 9101c3e8    	add	x8, sp, #0x70
100687240: 910103e1    	add	x1, sp, #0x40
100687244: 942bc194    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100687248: 90000388    	adrp	x8, 0x1006f7000 <__ZN5tokio7runtime7context7runtime13enter_runtime17hbd5c52b5b8dc183fE+0x104>
10068724c: 911a7108    	add	x8, x8, #0x69c
100687250: a90323f9    	stp	x25, x8, [sp, #0x30]
100687254: 900074e0    	adrp	x0, 0x101523000 <_anon.164d03a2dfc98f969363eaf1c50504fd.794+0x1931>
100687258: 910a1000    	add	x0, x0, #0x284
10068725c: 910103e8    	add	x8, sp, #0x40
100687260: 9100c3e1    	add	x1, sp, #0x30
100687264: 942bc18c    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100687268: f9403be1    	ldr	x1, [sp, #0x70]
10068726c: b4000081    	cbz	x1, 0x10068727c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x230>
100687270: f9403fe0    	ldr	x0, [sp, #0x78]
100687274: 52800022    	mov	w2, #0x1                ; =1
100687278: 97e5e8bc    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068727c: a94407e8    	ldp	x8, x1, [sp, #0x40]
100687280: a900a3e1    	stp	x1, x8, [sp, #0x8]
100687284: f9402be2    	ldr	x2, [sp, #0x50]
100687288: 910103e8    	add	x8, sp, #0x40
10068728c: aa1803e0    	mov	x0, x24
100687290: 97f3740a    	bl	0x1003642b8 <__ZN13codexmate_lib4core5relay10invariants30verify_router_on_http_contract17hb03248688cf468f3E>
100687294: a94463fa    	ldp	x26, x24, [sp, #0x40]
100687298: f9402bfb    	ldr	x27, [sp, #0x50]
10068729c: d37beb7c    	lsl	x28, x27, #5
1006872a0: 8b1c0308    	add	x8, x24, x28
1006872a4: a90763f8    	stp	x24, x24, [sp, #0x70]
1006872a8: a90823fa    	stp	x26, x8, [sp, #0x80]
1006872ac: f94017f9    	ldr	x25, [sp, #0x28]
1006872b0: f9400fe8    	ldr	x8, [sp, #0x18]
1006872b4: cb190108    	sub	x8, x8, x25
1006872b8: eb08037f    	cmp	x27, x8
1006872bc: 540008e8    	b.hi	0x1006873d8 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x38c>
1006872c0: f94013e8    	ldr	x8, [sp, #0x20]
1006872c4: 8b191500    	add	x0, x8, x25, lsl #5
1006872c8: aa1803e1    	mov	x1, x24
1006872cc: aa1c03e2    	mov	x2, x28
1006872d0: 942d28f7    	bl	0x1011d16ac <_writev+0x1011d16ac>
1006872d4: 8b1b0328    	add	x8, x25, x27
1006872d8: f90017e8    	str	x8, [sp, #0x28]
1006872dc: b40000ba    	cbz	x26, 0x1006872f0 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x2a4>
1006872e0: d37beb41    	lsl	x1, x26, #5
1006872e4: aa1803e0    	mov	x0, x24
1006872e8: 52800102    	mov	w2, #0x8                ; =8
1006872ec: 97e5e89f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1006872f0: b000937b    	adrp	x27, 0x1018f4000 <__ZN15crossbeam_utils6atomic11atomic_cell4lock5LOCKS17h71d6ab0bc78afdcbE+0x1b00>
1006872f4: 9126637b    	add	x27, x27, #0x998
1006872f8: f9400be1    	ldr	x1, [sp, #0x10]
1006872fc: b4000081    	cbz	x1, 0x10068730c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x2c0>
100687300: f94007e0    	ldr	x0, [sp, #0x8]
100687304: 52800022    	mov	w2, #0x1                ; =1
100687308: 97e5e898    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068730c: aa1703e0    	mov	x0, x23
100687310: 94001f10    	bl	0x10068ef50 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE>
100687314: 37000340    	tbnz	w0, #0x0, 0x10068737c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x330>
100687318: 900057a8    	adrp	x8, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
10068731c: 91079108    	add	x8, x8, #0x1e4
100687320: 9100c2a9    	add	x9, x21, #0x30
100687324: a90723e9    	stp	x9, x8, [sp, #0x70]
100687328: b0007500    	adrp	x0, 0x101528000 <_anon.1f4a05fd1dc96b9982818105670d4dce.206+0x414>
10068732c: 913a5800    	add	x0, x0, #0xe96
100687330: 910163e8    	add	x8, sp, #0x58
100687334: 9101c3e1    	add	x1, sp, #0x70
100687338: 942bc157    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10068733c: a945e3f7    	ldp	x23, x24, [sp, #0x58]
100687340: f94037fa    	ldr	x26, [sp, #0x68]
100687344: f94017f9    	ldr	x25, [sp, #0x28]
100687348: f9400fe8    	ldr	x8, [sp, #0x18]
10068734c: eb08033f    	cmp	x25, x8
100687350: 54000061    	b.ne	0x10068735c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x310>
100687354: 910063e0    	add	x0, sp, #0x18
100687358: 942c1a56    	bl	0x10118dcb0 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hf959252a2abc9249E>
10068735c: f94013e8    	ldr	x8, [sp, #0x20]
100687360: 8b191508    	add	x8, x8, x25, lsl #5
100687364: a9006117    	stp	x23, x24, [x8]
100687368: f900091a    	str	x26, [x8, #0x10]
10068736c: 52800089    	mov	w9, #0x4                ; =4
100687370: 39006109    	strb	w9, [x8, #0x18]
100687374: 91000728    	add	x8, x25, #0x1
100687378: f90017e8    	str	x8, [sp, #0x28]
10068737c: 3cc183e0    	ldur	q0, [sp, #0x18]
100687380: 3d8002c0    	str	q0, [x22]
100687384: f94017e8    	ldr	x8, [sp, #0x28]
100687388: f9000ac8    	str	x8, [x22, #0x10]
10068738c: 37000094    	tbnz	w20, #0x0, 0x10068739c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x350>
100687390: f9400368    	ldr	x8, [x27]
100687394: f240f91f    	tst	x8, #0x7fffffffffffffff
100687398: 54000161    	b.ne	0x1006873c4 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x378>
10068739c: f9400260    	ldr	x0, [x19]
1006873a0: 942b0dfa    	bl	0x10114ab88 <__ZN3std3sys3pal4unix4sync5mutex5Mutex6unlock17hb12cc63852d40a98E>
1006873a4: a94e7bfd    	ldp	x29, x30, [sp, #0xe0]
1006873a8: a94d4ff4    	ldp	x20, x19, [sp, #0xd0]
1006873ac: a94c57f6    	ldp	x22, x21, [sp, #0xc0]
1006873b0: a94b5ff8    	ldp	x24, x23, [sp, #0xb0]
1006873b4: a94a67fa    	ldp	x26, x25, [sp, #0xa0]
1006873b8: a9496ffc    	ldp	x28, x27, [sp, #0x90]
1006873bc: 9103c3ff    	add	sp, sp, #0xf0
1006873c0: d65f03c0    	ret
1006873c4: 942d16c2    	bl	0x1011ccecc <__ZN3std9panicking11panic_count17is_zero_slow_path17hbe4f1645e48c7385E>
1006873c8: 3707fea0    	tbnz	w0, #0x0, 0x10068739c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x350>
1006873cc: 52800028    	mov	w8, #0x1                ; =1
1006873d0: 390062a8    	strb	w8, [x21, #0x18]
1006873d4: 17fffff2    	b	0x10068739c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x350>
1006873d8: 910063e0    	add	x0, sp, #0x18
1006873dc: aa1903e1    	mov	x1, x25
1006873e0: aa1b03e2    	mov	x2, x27
1006873e4: 52800103    	mov	w3, #0x8                ; =8
1006873e8: 52800404    	mov	w4, #0x20               ; =32
1006873ec: 942c1d3a    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
1006873f0: f94017f9    	ldr	x25, [sp, #0x28]
1006873f4: 17ffffb3    	b	0x1006872c0 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x274>
1006873f8: 52800020    	mov	w0, #0x1                ; =1
1006873fc: 52800941    	mov	w1, #0x4a               ; =74
100687400: 942d2499    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100687404: 14000004    	b	0x100687414 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x3c8>
100687408: 52800020    	mov	w0, #0x1                ; =1
10068740c: 52800561    	mov	w1, #0x2b               ; =43
100687410: 942d2495    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100687414: d4200020    	brk	#0x1
100687418: aa0003f5    	mov	x21, x0
10068741c: 9101c3e0    	add	x0, sp, #0x70
100687420: 97fb7937    	bl	0x1005658fc <__ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5ab1c611742a3f5cE>
100687424: 1400000c    	b	0x100687454 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x408>
100687428: 942d25c8    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
10068742c: aa0003f5    	mov	x21, x0
100687430: b4000377    	cbz	x23, 0x10068749c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x450>
100687434: aa1803e0    	mov	x0, x24
100687438: aa1703e1    	mov	x1, x23
10068743c: 1400000e    	b	0x100687474 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x428>
100687440: aa0003f5    	mov	x21, x0
100687444: aa1703e0    	mov	x0, x23
100687448: 52800561    	mov	w1, #0x2b               ; =43
10068744c: 1400000a    	b	0x100687474 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x428>
100687450: aa0003f5    	mov	x21, x0
100687454: f9400be8    	ldr	x8, [sp, #0x10]
100687458: b4000228    	cbz	x8, 0x10068749c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x450>
10068745c: a94087e0    	ldp	x0, x1, [sp, #0x8]
100687460: 14000005    	b	0x100687474 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x428>
100687464: aa0003f5    	mov	x21, x0
100687468: f9403be1    	ldr	x1, [sp, #0x70]
10068746c: b4000181    	cbz	x1, 0x10068749c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x450>
100687470: f9403fe0    	ldr	x0, [sp, #0x78]
100687474: 52800022    	mov	w2, #0x1                ; =1
100687478: 97e5e83c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068747c: 14000008    	b	0x10068749c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x450>
100687480: aa0003f5    	mov	x21, x0
100687484: aa1703e0    	mov	x0, x23
100687488: 52800941    	mov	w1, #0x4a               ; =74
10068748c: 52800022    	mov	w2, #0x1                ; =1
100687490: 97e5e836    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100687494: 14000007    	b	0x1006874b0 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x464>
100687498: aa0003f5    	mov	x21, x0
10068749c: aa1303e0    	mov	x0, x19
1006874a0: aa1403e1    	mov	x1, x20
1006874a4: 9400543a    	bl	0x10069c58c <__ZN4core3ptr86drop_in_place$LT$std..sync..poison..mutex..MutexGuard$LT$tauri..window..Window$GT$$GT$17h1aa41336d17a5038E>
1006874a8: 14000007    	b	0x1006874c4 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x478>
1006874ac: aa0003f5    	mov	x21, x0
1006874b0: 9101c3e0    	add	x0, sp, #0x70
1006874b4: 94008f3a    	bl	0x1006ab19c <__ZN4core3ptr194drop_in_place$LT$core..result..Result$LT$std..sync..poison..mutex..MutexGuard$LT$$LP$$RP$$GT$$C$std..sync..poison..PoisonError$LT$std..sync..poison..mutex..MutexGuard$LT$$LP$$RP$$GT$$GT$$GT$$GT$17h3c57ec25660609c5E>
1006874b8: 14000003    	b	0x1006874c4 <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E+0x478>
1006874bc: 942d25a3    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
1006874c0: aa0003f5    	mov	x21, x0
1006874c4: 910063e0    	add	x0, sp, #0x18
1006874c8: 9400ef5c    	bl	0x1006c3238 <__ZN4core3ptr88drop_in_place$LT$alloc..vec..Vec$LT$tauri_runtime..webview..InitializationScript$GT$$GT$17h5cfd3345e6aa0213E>
1006874cc: aa1503e0    	mov	x0, x21
1006874d0: 942d2721    	bl	0x1011d1154 <_writev+0x1011d1154>
