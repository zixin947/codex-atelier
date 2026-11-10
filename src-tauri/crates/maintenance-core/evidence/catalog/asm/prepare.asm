
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010036bed0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E>:
10036bed0: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
10036bed4: a90167fa    	stp	x26, x25, [sp, #0x10]
10036bed8: a9025ff8    	stp	x24, x23, [sp, #0x20]
10036bedc: a90357f6    	stp	x22, x21, [sp, #0x30]
10036bee0: a9044ff4    	stp	x20, x19, [sp, #0x40]
10036bee4: a9057bfd    	stp	x29, x30, [sp, #0x50]
10036bee8: 910143fd    	add	x29, sp, #0x50
10036beec: d10c03ff    	sub	sp, sp, #0x300
10036bef0: aa0703f4    	mov	x20, x7
10036bef4: aa0603f7    	mov	x23, x6
10036bef8: aa0503fb    	mov	x27, x5
10036befc: aa0403f8    	mov	x24, x4
10036bf00: aa0303f9    	mov	x25, x3
10036bf04: aa0203f5    	mov	x21, x2
10036bf08: f9002fe1    	str	x1, [sp, #0x58]
10036bf0c: aa0003f3    	mov	x19, x0
10036bf10: 910443e8    	add	x8, sp, #0x110
10036bf14: aa0603e0    	mov	x0, x6
10036bf18: aa0703e1    	mov	x1, x7
10036bf1c: 940015b8    	bl	0x1003715fc <__ZN13codexmate_lib4core5relay13codex_catalog34validate_legacy_no_account_aliases17h35bdd5996afac233E>
10036bf20: f9408be8    	ldr	x8, [sp, #0x110]
10036bf24: f1002d1f    	cmp	x8, #0xb
10036bf28: 54000161    	b.ne	0x10036bf54 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x84>
10036bf2c: 3600023b    	tbz	w27, #0x0, 0x10036bf70 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0xa0>
10036bf30: 910283e8    	add	x8, sp, #0xa0
10036bf34: f9402fe0    	ldr	x0, [sp, #0x58]
10036bf38: aa1503e1    	mov	x1, x21
10036bf3c: aa1903e2    	mov	x2, x25
10036bf40: aa1803e3    	mov	x3, x24
10036bf44: 52800104    	mov	w4, #0x8                ; =8
10036bf48: d2800005    	mov	x5, #0x0                ; =0
10036bf4c: 97ffedc3    	bl	0x100367658 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE>
10036bf50: 1400014a    	b	0x10036c478 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x5a8>
10036bf54: a951d7f6    	ldp	x22, x21, [sp, #0x118]
10036bf58: f94097f7    	ldr	x23, [sp, #0x128]
10036bf5c: ad4987e0    	ldp	q0, q1, [sp, #0x130]
10036bf60: ad0307e0    	stp	q0, q1, [sp, #0x60]
10036bf64: ad4a87e0    	ldp	q0, q1, [sp, #0x150]
10036bf68: ad0407e0    	stp	q0, q1, [sp, #0x80]
10036bf6c: 140001e9    	b	0x10036c710 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x840>
10036bf70: 9103e3e8    	add	x8, sp, #0xf8
10036bf74: f9402fe0    	ldr	x0, [sp, #0x58]
10036bf78: f9002bf5    	str	x21, [sp, #0x50]
10036bf7c: aa1503e1    	mov	x1, x21
10036bf80: aa1903e2    	mov	x2, x25
10036bf84: aa1803e3    	mov	x3, x24
10036bf88: 52800104    	mov	w4, #0x8                ; =8
10036bf8c: d2800005    	mov	x5, #0x0                ; =0
10036bf90: 97ffedb2    	bl	0x100367658 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE>
10036bf94: b4002694    	cbz	x20, 0x10036c464 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x594>
10036bf98: 52800908    	mov	w8, #0x48               ; =72
10036bf9c: 9b085e88    	madd	x8, x20, x8, x23
10036bfa0: f9000fe8    	str	x8, [sp, #0x18]
10036bfa4: 910443f9    	add	x25, sp, #0x110
10036bfa8: f9402be8    	ldr	x8, [sp, #0x50]
10036bfac: d378dd08    	lsl	x8, x8, #8
10036bfb0: f90007e8    	str	x8, [sp, #0x8]
10036bfb4: 52800038    	mov	w24, #0x1               ; =1
10036bfb8: d2f00016    	mov	x22, #-0x8000000000000000 ; =-9223372036854775808
10036bfbc: f0007ca8    	adrp	x8, 0x101302000 <_writev+0x101302000>
10036bfc0: 3dc03500    	ldr	q0, [x8, #0xd0]
10036bfc4: 3d800fe0    	str	q0, [sp, #0x30]
10036bfc8: 52800088    	mov	w8, #0x4                ; =4
10036bfcc: f90003e8    	str	x8, [sp]
10036bfd0: f0007ce8    	adrp	x8, 0x10130a000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.765+0x10f>
10036bfd4: 3dc30100    	ldr	q0, [x8, #0xc00]
10036bfd8: 3d800be0    	str	q0, [sp, #0x20]
10036bfdc: b9004ffb    	str	w27, [sp, #0x4c]
10036bfe0: d10203a8    	sub	x8, x29, #0x80
10036bfe4: aa1703e0    	mov	x0, x23
10036bfe8: 94383545    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
10036bfec: 910843e8    	add	x8, sp, #0x210
10036bff0: aa1703e0    	mov	x0, x23
10036bff4: 94383542    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
10036bff8: d10383a8    	sub	x8, x29, #0xe0
10036bffc: 910062e0    	add	x0, x23, #0x18
10036c000: 9438353f    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
10036c004: 3cd803a0    	ldur	q0, [x29, #-0x80]
10036c008: 3d8047e0    	str	q0, [sp, #0x110]
10036c00c: f94113e8    	ldr	x8, [sp, #0x220]
10036c010: f8028328    	stur	x8, [x25, #0x28]
10036c014: f85903a8    	ldur	x8, [x29, #-0x70]
10036c018: 3dc087e0    	ldr	q0, [sp, #0x210]
10036c01c: 3c818320    	stur	q0, [x25, #0x18]
10036c020: f90093e8    	str	x8, [sp, #0x120]
10036c024: a91463ff    	stp	xzr, x24, [sp, #0x140]
10036c028: a9047f3f    	stp	xzr, xzr, [x25, #0x40]
10036c02c: a9167ff8    	stp	x24, xzr, [sp, #0x160]
10036c030: 3cd203a0    	ldur	q0, [x29, #-0xe0]
10036c034: f85303a8    	ldur	x8, [x29, #-0xd0]
10036c038: f8070328    	stur	x8, [x25, #0x70]
10036c03c: 3c860320    	stur	q0, [x25, #0x60]
10036c040: a908ff3f    	stp	xzr, xzr, [x25, #0x88]
10036c044: 52800108    	mov	w8, #0x8                ; =8
10036c048: a918a3ff    	stp	xzr, x8, [sp, #0x188]
10036c04c: f900d7f8    	str	x24, [sp, #0x1a8]
10036c050: f90107ff    	str	xzr, [sp, #0x208]
10036c054: f900ebf6    	str	x22, [sp, #0x1d0]
10036c058: 92800008    	mov	x8, #-0x1               ; =-1
10036c05c: f900f7e8    	str	x8, [sp, #0x1e8]
10036c060: a90eff3f    	stp	xzr, xzr, [x25, #0xe8]
10036c064: f80e033f    	stur	xzr, [x25, #0xe0]
10036c068: 3dc00fe0    	ldr	q0, [sp, #0x30]
10036c06c: 3d806fe0    	str	q0, [sp, #0x1b0]
10036c070: f9402be8    	ldr	x8, [sp, #0x50]
10036c074: b4000288    	cbz	x8, 0x10036c0c4 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x1f4>
10036c078: a940d2fa    	ldp	x26, x20, [x23, #0x8]
10036c07c: f94007fb    	ldr	x27, [sp, #0x8]
10036c080: f9402ff5    	ldr	x21, [sp, #0x58]
10036c084: 14000004    	b	0x10036c094 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x1c4>
10036c088: 910402b5    	add	x21, x21, #0x100
10036c08c: f104037b    	subs	x27, x27, #0x100
10036c090: 54000140    	b.eq	0x10036c0b8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x1e8>
10036c094: f9400aa8    	ldr	x8, [x21, #0x10]
10036c098: eb14011f    	cmp	x8, x20
10036c09c: 54ffff61    	b.ne	0x10036c088 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x1b8>
10036c0a0: f94006a0    	ldr	x0, [x21, #0x8]
10036c0a4: aa1a03e1    	mov	x1, x26
10036c0a8: aa1403e2    	mov	x2, x20
10036c0ac: 9439957d    	bl	0x1011d16a0 <_writev+0x1011d16a0>
10036c0b0: 35fffec0    	cbnz	w0, 0x10036c088 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x1b8>
10036c0b4: 14000002    	b	0x10036c0bc <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x1ec>
10036c0b8: d2800015    	mov	x21, #0x0               ; =0
10036c0bc: b9404ffb    	ldr	w27, [sp, #0x4c]
10036c0c0: 14000002    	b	0x10036c0c8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x1f8>
10036c0c4: d2800015    	mov	x21, #0x0               ; =0
10036c0c8: f10002bf    	cmp	x21, #0x0
10036c0cc: 9a95033a    	csel	x26, x25, x21, eq
10036c0d0: d10383a8    	sub	x8, x29, #0xe0
10036c0d4: aa1a03e0    	mov	x0, x26
10036c0d8: 9410b480    	bl	0x1007992d8 <__ZN13codexmate_lib4core5relay6models13RelayProvider14exposed_models17h78c65a493568f3aaE>
10036c0dc: a97223a9    	ldp	x9, x8, [x29, #-0xe0]
10036c0e0: f85303aa    	ldur	x10, [x29, #-0xd0]
10036c0e4: 52800c0b    	mov	w11, #0x60              ; =96
10036c0e8: 9b0b214a    	madd	x10, x10, x11, x8
10036c0ec: a93823a8    	stp	x8, x8, [x29, #-0x80]
10036c0f0: a9392ba9    	stp	x9, x10, [x29, #-0x70]
10036c0f4: d10383a8    	sub	x8, x29, #0xe0
10036c0f8: d10203a0    	sub	x0, x29, #0x80
10036c0fc: aa1703e1    	mov	x1, x23
10036c100: 94068e3f    	bl	0x10050f9fc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E>
10036c104: f85203a8    	ldur	x8, [x29, #-0xe0]
10036c108: f100091f    	cmp	x8, #0x2
10036c10c: 54000881    	b.ne	0x10036c21c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x34c>
10036c110: 910343e8    	add	x8, sp, #0xd0
10036c114: 910443e0    	add	x0, sp, #0x110
10036c118: 9410b470    	bl	0x1007992d8 <__ZN13codexmate_lib4core5relay6models13RelayProvider14exposed_models17h78c65a493568f3aaE>
10036c11c: f94073e8    	ldr	x8, [sp, #0xe0]
10036c120: b40035c8    	cbz	x8, 0x10036c7d8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x908>
10036c124: f9406ffb    	ldr	x27, [sp, #0xd8]
10036c128: f9400375    	ldr	x21, [x27]
10036c12c: 3cc28360    	ldur	q0, [x27, #0x28]
10036c130: 3c9403a0    	stur	q0, [x29, #-0xc0]
10036c134: 3cc38360    	ldur	q0, [x27, #0x38]
10036c138: 3c9503a0    	stur	q0, [x29, #-0xb0]
10036c13c: 3cc48360    	ldur	q0, [x27, #0x48]
10036c140: 3c9603a0    	stur	q0, [x29, #-0xa0]
10036c144: f9402f69    	ldr	x9, [x27, #0x58]
10036c148: f81703a9    	stur	x9, [x29, #-0x90]
10036c14c: 3cc08360    	ldur	q0, [x27, #0x8]
10036c150: 3c9203a0    	stur	q0, [x29, #-0xe0]
10036c154: 3cc18360    	ldur	q0, [x27, #0x18]
10036c158: 3c9303a0    	stur	q0, [x29, #-0xd0]
10036c15c: d1000514    	sub	x20, x8, #0x1
10036c160: 8b140688    	add	x8, x20, x20, lsl #1
10036c164: d37be902    	lsl	x2, x8, #5
10036c168: 91018361    	add	x1, x27, #0x60
10036c16c: aa1b03e0    	mov	x0, x27
10036c170: 94399552    	bl	0x1011d16b8 <_writev+0x1011d16b8>
10036c174: f90073f4    	str	x20, [sp, #0xe0]
10036c178: f1000abf    	cmp	x21, #0x2
10036c17c: 54003300    	b.eq	0x10036c7dc <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x90c>
10036c180: f9010bf5    	str	x21, [sp, #0x210]
10036c184: ad7a07a0    	ldp	q0, q1, [x29, #-0xc0]
10036c188: 910843e9    	add	x9, sp, #0x210
10036c18c: 3c828120    	stur	q0, [x9, #0x28]
10036c190: 3c838121    	stur	q1, [x9, #0x38]
10036c194: 3cd603a0    	ldur	q0, [x29, #-0xa0]
10036c198: 3c848120    	stur	q0, [x9, #0x48]
10036c19c: f85703a8    	ldur	x8, [x29, #-0x90]
10036c1a0: f8058128    	stur	x8, [x9, #0x58]
10036c1a4: ad7907a0    	ldp	q0, q1, [x29, #-0xe0]
10036c1a8: 3c808120    	stur	q0, [x9, #0x8]
10036c1ac: 3c818121    	stur	q1, [x9, #0x18]
10036c1b0: b4000574    	cbz	x20, 0x10036c25c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x38c>
10036c1b4: 9100a375    	add	x21, x27, #0x28
10036c1b8: 14000004    	b	0x10036c1c8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x2f8>
10036c1bc: 910182b5    	add	x21, x21, #0x60
10036c1c0: f1000694    	subs	x20, x20, #0x1
10036c1c4: 540004c0    	b.eq	0x10036c25c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x38c>
10036c1c8: f85e82a1    	ldur	x1, [x21, #-0x18]
10036c1cc: b4000081    	cbz	x1, 0x10036c1dc <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x30c>
10036c1d0: f85f02a0    	ldur	x0, [x21, #-0x10]
10036c1d4: 52800022    	mov	w2, #0x1                ; =1
10036c1d8: 97f254e4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036c1dc: f9400ea1    	ldr	x1, [x21, #0x18]
10036c1e0: eb16003f    	cmp	x1, x22
10036c1e4: fa401824    	ccmp	x1, #0x0, #0x4, ne
10036c1e8: 54000081    	b.ne	0x10036c1f8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x328>
10036c1ec: f94002a1    	ldr	x1, [x21]
10036c1f0: b50000e1    	cbnz	x1, 0x10036c20c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x33c>
10036c1f4: 17fffff2    	b	0x10036c1bc <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x2ec>
10036c1f8: f94012a0    	ldr	x0, [x21, #0x20]
10036c1fc: 52800022    	mov	w2, #0x1                ; =1
10036c200: 97f254da    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036c204: f94002a1    	ldr	x1, [x21]
10036c208: b4fffda1    	cbz	x1, 0x10036c1bc <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x2ec>
10036c20c: f94006a0    	ldr	x0, [x21, #0x8]
10036c210: 52800022    	mov	w2, #0x1                ; =1
10036c214: 97f254d5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036c218: 17ffffe9    	b	0x10036c1bc <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x2ec>
10036c21c: d10383aa    	sub	x10, x29, #0xe0
10036c220: 3cc28140    	ldur	q0, [x10, #0x28]
10036c224: 910843eb    	add	x11, sp, #0x210
10036c228: 3c828160    	stur	q0, [x11, #0x28]
10036c22c: 3cc38140    	ldur	q0, [x10, #0x38]
10036c230: 3c838160    	stur	q0, [x11, #0x38]
10036c234: 3cc48140    	ldur	q0, [x10, #0x48]
10036c238: 3c848160    	stur	q0, [x11, #0x48]
10036c23c: f8458149    	ldur	x9, [x10, #0x58]
10036c240: f8058169    	stur	x9, [x11, #0x58]
10036c244: 3cc08140    	ldur	q0, [x10, #0x8]
10036c248: 3c808160    	stur	q0, [x11, #0x8]
10036c24c: 3cc18140    	ldur	q0, [x10, #0x18]
10036c250: 3c818160    	stur	q0, [x11, #0x18]
10036c254: f9010be8    	str	x8, [sp, #0x210]
10036c258: 14000009    	b	0x10036c27c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x3ac>
10036c25c: f9406be8    	ldr	x8, [sp, #0xd0]
10036c260: b40000c8    	cbz	x8, 0x10036c278 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x3a8>
10036c264: 8b080508    	add	x8, x8, x8, lsl #1
10036c268: d37be901    	lsl	x1, x8, #5
10036c26c: aa1b03e0    	mov	x0, x27
10036c270: 52800102    	mov	w2, #0x8                ; =8
10036c274: 97f254bd    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036c278: b9404ffb    	ldr	w27, [sp, #0x4c]
10036c27c: d10203a0    	sub	x0, x29, #0x80
10036c280: 9407eaa2    	bl	0x100566d08 <__ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8b7cbd6ddb489c57E>
10036c284: d10203a0    	sub	x0, x29, #0x80
10036c288: 910843e2    	add	x2, sp, #0x210
10036c28c: aa1a03e1    	mov	x1, x26
10036c290: d2800003    	mov	x3, #0x0                ; =0
10036c294: d2800004    	mov	x4, #0x0                ; =0
10036c298: 97ffef28    	bl	0x100367f38 <__ZN13codexmate_lib4core5relay13codex_catalog21make_relay_model_info17h303a38e566039289E>
10036c29c: f9401ae8    	ldr	x8, [x23, #0x30]
10036c2a0: eb16011f    	cmp	x8, x22
10036c2a4: 54000061    	b.ne	0x10036c2b0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x3e0>
10036c2a8: 52800008    	mov	w8, #0x0                ; =0
10036c2ac: 14000011    	b	0x10036c2f0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x420>
10036c2b0: f94022fc    	ldr	x28, [x23, #0x40]
10036c2b4: b7f8267c    	tbnz	x28, #0x3f, 0x10036c780 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x8b0>
10036c2b8: f9401ef4    	ldr	x20, [x23, #0x38]
10036c2bc: b40000fc    	cbz	x28, 0x10036c2d8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x408>
10036c2c0: 97f254ad    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036c2c4: aa1c03e0    	mov	x0, x28
10036c2c8: 52800021    	mov	w1, #0x1                ; =1
10036c2cc: 97f254a6    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036c2d0: b5000060    	cbnz	x0, 0x10036c2dc <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x40c>
10036c2d4: 1400012c    	b	0x10036c784 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x8b4>
10036c2d8: 52800020    	mov	w0, #0x1                ; =1
10036c2dc: f9000be0    	str	x0, [sp, #0x10]
10036c2e0: aa1403e1    	mov	x1, x20
10036c2e4: aa1c03e2    	mov	x2, x28
10036c2e8: 943994f1    	bl	0x1011d16ac <_writev+0x1011d16ac>
10036c2ec: 52800068    	mov	w8, #0x3                ; =3
10036c2f0: 381203a8    	sturb	w8, [x29, #-0xe0]
10036c2f4: f9400be8    	ldr	x8, [sp, #0x10]
10036c2f8: a932a3bc    	stp	x28, x8, [x29, #-0xd8]
10036c2fc: f81383bc    	stur	x28, [x29, #-0xc8]
10036c300: d10203a2    	sub	x2, x29, #0x80
10036c304: d0007d20    	adrp	x0, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
10036c308: 913c5000    	add	x0, x0, #0xf14
10036c30c: 52800081    	mov	w1, #0x4                ; =4
10036c310: 94368a94    	bl	0x10110ed60 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$15index_or_insert17h0050fffd7d438de5E>
10036c314: aa0003f4    	mov	x20, x0
10036c318: 9401acaa    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036c31c: 3cd203a0    	ldur	q0, [x29, #-0xe0]
10036c320: 3d800280    	str	q0, [x20]
10036c324: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036c328: 3d800680    	str	q0, [x20, #0x10]
10036c32c: 97f25492    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036c330: 52800080    	mov	w0, #0x4                ; =4
10036c334: 52800021    	mov	w1, #0x1                ; =1
10036c338: 97f2548b    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036c33c: b4002260    	cbz	x0, 0x10036c788 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x8b8>
10036c340: 528d2d08    	mov	w8, #0x6968             ; =26984
10036c344: 72acac88    	movk	w8, #0x6564, lsl #16
10036c348: b9000008    	str	w8, [x0]
10036c34c: 52800068    	mov	w8, #0x3                ; =3
10036c350: 381203a8    	sturb	w8, [x29, #-0xe0]
10036c354: 52800089    	mov	w9, #0x4                ; =4
10036c358: a93283a9    	stp	x9, x0, [x29, #-0xd8]
10036c35c: f81383a9    	stur	x9, [x29, #-0xc8]
10036c360: d10203a2    	sub	x2, x29, #0x80
10036c364: 90007d00    	adrp	x0, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036c368: 910da400    	add	x0, x0, #0x369
10036c36c: 52800141    	mov	w1, #0xa                ; =10
10036c370: 94368a7c    	bl	0x10110ed60 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$15index_or_insert17h0050fffd7d438de5E>
10036c374: aa0003f4    	mov	x20, x0
10036c378: 9401ac92    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036c37c: 3cd203a0    	ldur	q0, [x29, #-0xe0]
10036c380: 3d800280    	str	q0, [x20]
10036c384: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036c388: 3d800680    	str	q0, [x20, #0x10]
10036c38c: 52800048    	mov	w8, #0x2                ; =2
10036c390: 381203a8    	sturb	w8, [x29, #-0xe0]
10036c394: 3dc00be0    	ldr	q0, [sp, #0x20]
10036c398: 3c9283a0    	stur	q0, [x29, #-0xd8]
10036c39c: d10203a2    	sub	x2, x29, #0x80
10036c3a0: d0007d20    	adrp	x0, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
10036c3a4: 913a8000    	add	x0, x0, #0xea0
10036c3a8: 52800101    	mov	w1, #0x8                ; =8
10036c3ac: 94368a6d    	bl	0x10110ed60 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$15index_or_insert17h0050fffd7d438de5E>
10036c3b0: aa0003f4    	mov	x20, x0
10036c3b4: 9401ac83    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036c3b8: 3cd203a0    	ldur	q0, [x29, #-0xe0]
10036c3bc: 3d800280    	str	q0, [x20]
10036c3c0: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036c3c4: 3d800680    	str	q0, [x20, #0x10]
10036c3c8: ad7c07a0    	ldp	q0, q1, [x29, #-0x80]
10036c3cc: ad3907a0    	stp	q0, q1, [x29, #-0xe0]
10036c3d0: f94087f4    	ldr	x20, [sp, #0x108]
10036c3d4: f9407fe8    	ldr	x8, [sp, #0xf8]
10036c3d8: eb08029f    	cmp	x20, x8
10036c3dc: 54000061    	b.ne	0x10036c3e8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x518>
10036c3e0: 9103e3e0    	add	x0, sp, #0xf8
10036c3e4: 943978ed    	bl	0x1011ca798 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h7488d3d97b01b200E>
10036c3e8: f94083e8    	ldr	x8, [sp, #0x100]
10036c3ec: 8b141508    	add	x8, x8, x20, lsl #5
10036c3f0: 3cd203a0    	ldur	q0, [x29, #-0xe0]
10036c3f4: 3d800100    	str	q0, [x8]
10036c3f8: 3cd303a0    	ldur	q0, [x29, #-0xd0]
10036c3fc: 3d800500    	str	q0, [x8, #0x10]
10036c400: 91000688    	add	x8, x20, #0x1
10036c404: f90087e8    	str	x8, [sp, #0x108]
10036c408: f94113e1    	ldr	x1, [sp, #0x220]
10036c40c: b4000081    	cbz	x1, 0x10036c41c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x54c>
10036c410: f94117e0    	ldr	x0, [sp, #0x228]
10036c414: 52800022    	mov	w2, #0x1                ; =1
10036c418: 97f25454    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036c41c: f9412be1    	ldr	x1, [sp, #0x250]
10036c420: eb16003f    	cmp	x1, x22
10036c424: 540000a0    	b.eq	0x10036c438 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x568>
10036c428: b4000081    	cbz	x1, 0x10036c438 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x568>
10036c42c: f9412fe0    	ldr	x0, [sp, #0x258]
10036c430: 52800022    	mov	w2, #0x1                ; =1
10036c434: 97f2544d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036c438: f9411fe1    	ldr	x1, [sp, #0x238]
10036c43c: b4000081    	cbz	x1, 0x10036c44c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x57c>
10036c440: f94123e0    	ldr	x0, [sp, #0x240]
10036c444: 52800022    	mov	w2, #0x1                ; =1
10036c448: 97f25448    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036c44c: 910122f7    	add	x23, x23, #0x48
10036c450: 910443e0    	add	x0, sp, #0x110
10036c454: 9401b0af    	bl	0x1003d8710 <__ZN4core3ptr70drop_in_place$LT$codexmate_lib..core..relay..models..RelayProvider$GT$17h0c9f2423823e18abE>
10036c458: f9400fe8    	ldr	x8, [sp, #0x18]
10036c45c: eb0802ff    	cmp	x23, x8
10036c460: 54ffdc01    	b.ne	0x10036bfe0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x110>
10036c464: 9103e3e8    	add	x8, sp, #0xf8
10036c468: 3dc00100    	ldr	q0, [x8]
10036c46c: 3d802be0    	str	q0, [sp, #0xa0]
10036c470: f94087e8    	ldr	x8, [sp, #0x108]
10036c474: f9005be8    	str	x8, [sp, #0xb0]
10036c478: f9005fff    	str	xzr, [sp, #0xb8]
10036c47c: f90067ff    	str	xzr, [sp, #0xc8]
10036c480: 97f2543d    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036c484: 528000d5    	mov	w21, #0x6               ; =6
10036c488: 528000c0    	mov	w0, #0x6                ; =6
10036c48c: 52800021    	mov	w1, #0x1                ; =1
10036c490: 97f25435    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036c494: b40019a0    	cbz	x0, 0x10036c7c8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x8f8>
10036c498: aa0003f4    	mov	x20, x0
10036c49c: 528e6d88    	mov	w8, #0x736c             ; =29548
10036c4a0: 79000808    	strh	w8, [x0, #0x4]
10036c4a4: 528deda8    	mov	w8, #0x6f6d             ; =28525
10036c4a8: 72acac88    	movk	w8, #0x6564, lsl #16
10036c4ac: b9000008    	str	w8, [x0]
10036c4b0: 528000c8    	mov	w8, #0x6                ; =6
10036c4b4: a90f83e8    	stp	x8, x0, [sp, #0xf8]
10036c4b8: f90087e8    	str	x8, [sp, #0x108]
10036c4bc: 910443e8    	add	x8, sp, #0x110
10036c4c0: 910283e0    	add	x0, sp, #0xa0
10036c4c4: 97f9a86a    	bl	0x1001d666c <__ZN10serde_core3ser10Serializer11collect_seq17hb0315d44daa2db91E>
10036c4c8: 394443e8    	ldrb	w8, [sp, #0x110]
10036c4cc: 7100191f    	cmp	w8, #0x6
10036c4d0: 54001640    	b.eq	0x10036c798 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x8c8>
10036c4d4: ad4887e0    	ldp	q0, q1, [sp, #0x110]
10036c4d8: ad0687e0    	stp	q0, q1, [sp, #0xd0]
10036c4dc: d10383a8    	sub	x8, x29, #0xe0
10036c4e0: 9102e3e0    	add	x0, sp, #0xb8
10036c4e4: 9103e3e1    	add	x1, sp, #0xf8
10036c4e8: 940fa8d4    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036c4ec: f85203a8    	ldur	x8, [x29, #-0xe0]
10036c4f0: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036c4f4: eb09011f    	cmp	x8, x9
10036c4f8: 54000201    	b.ne	0x10036c538 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x668>
10036c4fc: f85283a8    	ldur	x8, [x29, #-0xd8]
10036c500: f85383a9    	ldur	x9, [x29, #-0xc8]
10036c504: 8b091508    	add	x8, x8, x9, lsl #5
10036c508: 3dc00100    	ldr	q0, [x8]
10036c50c: 3d8087e0    	str	q0, [sp, #0x210]
10036c510: 3dc00500    	ldr	q0, [x8, #0x10]
10036c514: 3d808be0    	str	q0, [sp, #0x220]
10036c518: ad4683e1    	ldp	q1, q0, [sp, #0xd0]
10036c51c: ad000101    	stp	q1, q0, [x8]
10036c520: 394843e8    	ldrb	w8, [sp, #0x210]
10036c524: 7100191f    	cmp	w8, #0x6
10036c528: 54000200    	b.eq	0x10036c568 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x698>
10036c52c: 910843e0    	add	x0, sp, #0x210
10036c530: 9401ac24    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036c534: 1400000d    	b	0x10036c568 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x698>
10036c538: ad7907a0    	ldp	q0, q1, [x29, #-0xe0]
10036c53c: ad0887e0    	stp	q0, q1, [sp, #0x110]
10036c540: 3cd403a0    	ldur	q0, [x29, #-0xc0]
10036c544: 3d804fe0    	str	q0, [sp, #0x130]
10036c548: f85503a8    	ldur	x8, [x29, #-0xb0]
10036c54c: f900a3e8    	str	x8, [sp, #0x140]
10036c550: ad4687e0    	ldp	q0, q1, [sp, #0xd0]
10036c554: ad3c07a0    	stp	q0, q1, [x29, #-0x80]
10036c558: 910843e8    	add	x8, sp, #0x210
10036c55c: 910443e0    	add	x0, sp, #0x110
10036c560: d10203a1    	sub	x1, x29, #0x80
10036c564: 940a8667    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036c568: 97f25403    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10036c56c: 52800235    	mov	w21, #0x11              ; =17
10036c570: 52800034    	mov	w20, #0x1               ; =1
10036c574: 52800220    	mov	w0, #0x11               ; =17
10036c578: 52800021    	mov	w1, #0x1                ; =1
10036c57c: 97f253fa    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10036c580: b4001240    	cbz	x0, 0x10036c7c8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x8f8>
10036c584: 52800e88    	mov	w8, #0x74               ; =116
10036c588: 39004008    	strb	w8, [x0, #0x10]
10036c58c: 90007d08    	adrp	x8, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
10036c590: 9102c508    	add	x8, x8, #0xb1
10036c594: 3dc00100    	ldr	q0, [x8]
10036c598: 3d800000    	str	q0, [x0]
10036c59c: 52800228    	mov	w8, #0x11               ; =17
10036c5a0: a90f83e8    	stp	x8, x0, [sp, #0xf8]
10036c5a4: f90087e8    	str	x8, [sp, #0x108]
10036c5a8: 52000368    	eor	w8, w27, #0x1
10036c5ac: 390343f4    	strb	w20, [sp, #0xd0]
10036c5b0: 390347e8    	strb	w8, [sp, #0xd1]
10036c5b4: d10383a8    	sub	x8, x29, #0xe0
10036c5b8: 9102e3e0    	add	x0, sp, #0xb8
10036c5bc: 9103e3e1    	add	x1, sp, #0xf8
10036c5c0: 940fa89e    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
10036c5c4: f85203a8    	ldur	x8, [x29, #-0xe0]
10036c5c8: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10036c5cc: eb09011f    	cmp	x8, x9
10036c5d0: 54000201    	b.ne	0x10036c610 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x740>
10036c5d4: f85283a8    	ldur	x8, [x29, #-0xd8]
10036c5d8: f85383a9    	ldur	x9, [x29, #-0xc8]
10036c5dc: 8b091508    	add	x8, x8, x9, lsl #5
10036c5e0: 3dc00100    	ldr	q0, [x8]
10036c5e4: 3d8087e0    	str	q0, [sp, #0x210]
10036c5e8: 3dc00500    	ldr	q0, [x8, #0x10]
10036c5ec: 3d808be0    	str	q0, [sp, #0x220]
10036c5f0: ad4683e1    	ldp	q1, q0, [sp, #0xd0]
10036c5f4: ad000101    	stp	q1, q0, [x8]
10036c5f8: 394843e8    	ldrb	w8, [sp, #0x210]
10036c5fc: 7100191f    	cmp	w8, #0x6
10036c600: 54000200    	b.eq	0x10036c640 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x770>
10036c604: 910843e0    	add	x0, sp, #0x210
10036c608: 9401abee    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036c60c: 1400000d    	b	0x10036c640 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x770>
10036c610: ad7907a0    	ldp	q0, q1, [x29, #-0xe0]
10036c614: ad0887e0    	stp	q0, q1, [sp, #0x110]
10036c618: 3cd403a0    	ldur	q0, [x29, #-0xc0]
10036c61c: 3d804fe0    	str	q0, [sp, #0x130]
10036c620: f85503a8    	ldur	x8, [x29, #-0xb0]
10036c624: f900a3e8    	str	x8, [sp, #0x140]
10036c628: ad4687e0    	ldp	q0, q1, [sp, #0xd0]
10036c62c: ad3c07a0    	stp	q0, q1, [x29, #-0x80]
10036c630: 910843e8    	add	x8, sp, #0x210
10036c634: 910443e0    	add	x0, sp, #0x110
10036c638: d10203a1    	sub	x1, x29, #0x80
10036c63c: 940a8631    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
10036c640: 3ccb83e0    	ldur	q0, [sp, #0xb8]
10036c644: 9103e3e8    	add	x8, sp, #0xf8
10036c648: 3c820100    	stur	q0, [x8, #0x20]
10036c64c: f94067e8    	ldr	x8, [sp, #0xc8]
10036c650: f90097e8    	str	x8, [sp, #0x128]
10036c654: 528000a8    	mov	w8, #0x5                ; =5
10036c658: 390443e8    	strb	w8, [sp, #0x110]
10036c65c: 910843e0    	add	x0, sp, #0x210
10036c660: 910443e1    	add	x1, sp, #0x110
10036c664: 97ffc5c5    	bl	0x10035dd78 <__ZN10serde_json3ser13to_vec_pretty17h26f635c90744d57eE>
10036c668: f9410bf8    	ldr	x24, [sp, #0x210]
10036c66c: f9410ff5    	ldr	x21, [sp, #0x218]
10036c670: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10036c674: eb08031f    	cmp	x24, x8
10036c678: 54000201    	b.ne	0x10036c6b8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x7e8>
10036c67c: f9006bf5    	str	x21, [sp, #0xd0]
10036c680: 910343e8    	add	x8, sp, #0xd0
10036c684: f0006ce9    	adrp	x9, 0x10110b000 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0x50>
10036c688: 910ab129    	add	x9, x9, #0x2ac
10036c68c: a93827a8    	stp	x8, x9, [x29, #-0x80]
10036c690: f0008d80    	adrp	x0, 0x10151f000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.151+0x178>
10036c694: 913db000    	add	x0, x0, #0xf6c
10036c698: d10383a8    	sub	x8, x29, #0xe0
10036c69c: d10203a1    	sub	x1, x29, #0x80
10036c6a0: 94382c7d    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10036c6a4: a97257b6    	ldp	x22, x21, [x29, #-0xe0]
10036c6a8: f85303b7    	ldur	x23, [x29, #-0xd0]
10036c6ac: 910343e0    	add	x0, sp, #0xd0
10036c6b0: 9401ab8b    	bl	0x1003d74dc <__ZN4core3ptr45drop_in_place$LT$serde_json..error..Error$GT$17hc34208687d70e728E>
10036c6b4: 14000003    	b	0x10036c6c0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x7f0>
10036c6b8: f94113f7    	ldr	x23, [sp, #0x220]
10036c6bc: aa1803f6    	mov	x22, x24
10036c6c0: 910443e0    	add	x0, sp, #0x110
10036c6c4: 9401abbf    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036c6c8: a94aa3e0    	ldp	x0, x8, [sp, #0xa8]
10036c6cc: 91000519    	add	x25, x8, #0x1
10036c6d0: f1000739    	subs	x25, x25, #0x1
10036c6d4: 540000a0    	b.eq	0x10036c6e8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x818>
10036c6d8: 91008014    	add	x20, x0, #0x20
10036c6dc: 9407955c    	bl	0x100551c4c <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036c6e0: aa1403e0    	mov	x0, x20
10036c6e4: 17fffffb    	b	0x10036c6d0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x800>
10036c6e8: f94053e8    	ldr	x8, [sp, #0xa0]
10036c6ec: b40000a8    	cbz	x8, 0x10036c700 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x830>
10036c6f0: f94057e0    	ldr	x0, [sp, #0xa8]
10036c6f4: d37be901    	lsl	x1, x8, #5
10036c6f8: 52800102    	mov	w2, #0x8                ; =8
10036c6fc: 97f2539b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10036c700: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10036c704: eb08031f    	cmp	x24, x8
10036c708: 54000121    	b.ne	0x10036c72c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x85c>
10036c70c: 52800148    	mov	w8, #0xa                ; =10
10036c710: a9015e75    	stp	x21, x23, [x19, #0x10]
10036c714: a9005a68    	stp	x8, x22, [x19]
10036c718: ad4307e0    	ldp	q0, q1, [sp, #0x60]
10036c71c: ad010660    	stp	q0, q1, [x19, #0x20]
10036c720: ad4407e0    	ldp	q0, q1, [sp, #0x80]
10036c724: ad020660    	stp	q0, q1, [x19, #0x40]
10036c728: 1400000e    	b	0x10036c760 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x890>
10036c72c: 910443e0    	add	x0, sp, #0x110
10036c730: 94000e0c    	bl	0x10036ff60 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E>
10036c734: ad4887e0    	ldp	q0, q1, [sp, #0x110]
10036c738: 3c808260    	stur	q0, [x19, #0x8]
10036c73c: 3c818261    	stur	q1, [x19, #0x18]
10036c740: 3dc04fe0    	ldr	q0, [sp, #0x130]
10036c744: 3c828260    	stur	q0, [x19, #0x28]
10036c748: f940a3e8    	ldr	x8, [sp, #0x140]
10036c74c: a903da68    	stp	x8, x22, [x19, #0x38]
10036c750: a904de75    	stp	x21, x23, [x19, #0x48]
10036c754: 3901627b    	strb	w27, [x19, #0x58]
10036c758: 52800168    	mov	w8, #0xb                ; =11
10036c75c: f9000268    	str	x8, [x19]
10036c760: 910c03ff    	add	sp, sp, #0x300
10036c764: a9457bfd    	ldp	x29, x30, [sp, #0x50]
10036c768: a9444ff4    	ldp	x20, x19, [sp, #0x40]
10036c76c: a94357f6    	ldp	x22, x21, [sp, #0x30]
10036c770: a9425ff8    	ldp	x24, x23, [sp, #0x20]
10036c774: a94167fa    	ldp	x26, x25, [sp, #0x10]
10036c778: a8c66ffc    	ldp	x28, x27, [sp], #0x60
10036c77c: d65f03c0    	ret
10036c780: d2800018    	mov	x24, #0x0               ; =0
10036c784: f90003fc    	str	x28, [sp]
10036c788: aa1803e0    	mov	x0, x24
10036c78c: f94003e1    	ldr	x1, [sp]
10036c790: 94398fb5    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036c794: 14000017    	b	0x10036c7f0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x920>
10036c798: f9408fe8    	ldr	x8, [sp, #0x118]
10036c79c: f81203a8    	stur	x8, [x29, #-0xe0]
10036c7a0: d0007d20    	adrp	x0, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
10036c7a4: 9110a400    	add	x0, x0, #0x429
10036c7a8: 9000a583    	adrp	x3, 0x10181c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1500+0x80>
10036c7ac: 91018063    	add	x3, x3, #0x60
10036c7b0: f000a564    	adrp	x4, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036c7b4: 9117c084    	add	x4, x4, #0x5f0
10036c7b8: d10383a2    	sub	x2, x29, #0xe0
10036c7bc: 52800561    	mov	w1, #0x2b               ; =43
10036c7c0: 9439908e    	bl	0x1011d09f8 <__ZN4core6result13unwrap_failed17hfd1628134417d023E>
10036c7c4: 1400000b    	b	0x10036c7f0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x920>
10036c7c8: 52800020    	mov	w0, #0x1                ; =1
10036c7cc: aa1503e1    	mov	x1, x21
10036c7d0: 94398fa5    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10036c7d4: 14000007    	b	0x10036c7f0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0x920>
10036c7d8: d2800014    	mov	x20, #0x0               ; =0
10036c7dc: f000a562    	adrp	x2, 0x10181b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1127+0x140>
10036c7e0: 911ac042    	add	x2, x2, #0x6b0
10036c7e4: d2800000    	mov	x0, #0x0                ; =0
10036c7e8: aa1403e1    	mov	x1, x20
10036c7ec: 94398f86    	bl	0x1011d0604 <__ZN5alloc3vec16Vec$LT$T$C$A$GT$6remove13assert_failed17h35c5139e34985a77E>
10036c7f0: d4200020    	brk	#0x1
10036c7f4: aa0003f3    	mov	x19, x0
10036c7f8: b4000ff6    	cbz	x22, 0x10036c9f4 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0xb24>
10036c7fc: aa1503e0    	mov	x0, x21
10036c800: aa1603e1    	mov	x1, x22
10036c804: 52800022    	mov	w2, #0x1                ; =1
10036c808: 1400007a    	b	0x10036c9f0 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0xb20>
10036c80c: aa0003f3    	mov	x19, x0
10036c810: 910343e0    	add	x0, sp, #0xd0
10036c814: 9401ab32    	bl	0x1003d74dc <__ZN4core3ptr45drop_in_place$LT$serde_json..error..Error$GT$17hc34208687d70e728E>
10036c818: 14000030    	b	0x10036c8d8 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0xa08>
10036c81c: 943990cb    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
10036c820: 14000047    	b	0x10036c93c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0xa6c>
10036c824: 14000046    	b	0x10036c93c <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0xa6c>
10036c828: aa0003f3    	mov	x19, x0
10036c82c: d10383a0    	sub	x0, x29, #0xe0
10036c830: 9401ab64    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10036c834: 14000064    	b	0x10036c9c4 <__ZN13codexmate_lib4core5relay13codex_catalog22prepare_router_catalog17hdf450e31a10c8067E+0xaf4>
