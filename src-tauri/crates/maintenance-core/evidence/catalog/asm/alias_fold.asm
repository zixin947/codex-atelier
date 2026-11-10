
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100804000 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE>:
100804000: d101c3ff    	sub	sp, sp, #0x70
100804004: a90267fa    	stp	x26, x25, [sp, #0x20]
100804008: a9035ff8    	stp	x24, x23, [sp, #0x30]
10080400c: a90457f6    	stp	x22, x21, [sp, #0x40]
100804010: a9054ff4    	stp	x20, x19, [sp, #0x50]
100804014: a9067bfd    	stp	x29, x30, [sp, #0x60]
100804018: 910183fd    	add	x29, sp, #0x60
10080401c: eb01001f    	cmp	x0, x1
100804020: 540005e0    	b.eq	0x1008040dc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE+0xdc>
100804024: aa0203f3    	mov	x19, x2
100804028: cb000028    	sub	x8, x1, x0
10080402c: d29c71e9    	mov	x9, #0xe38f             ; =58255
100804030: f2b1c709    	movk	x9, #0x8e38, lsl #16
100804034: f2c71c69    	movk	x9, #0x38e3, lsl #32
100804038: f2fc71c9    	movk	x9, #0xe38e, lsl #48
10080403c: 9bc97d08    	umulh	x8, x8, x9
100804040: d346fd18    	lsr	x24, x8, #6
100804044: 91010019    	add	x25, x0, #0x40
100804048: d2f0001a    	mov	x26, #-0x8000000000000000 ; =-9223372036854775808
10080404c: 52800034    	mov	w20, #0x1               ; =1
100804050: 1400000e    	b	0x100804088 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE+0x88>
100804054: 52800035    	mov	w21, #0x1               ; =1
100804058: aa1503e0    	mov	x0, x21
10080405c: aa1703e1    	mov	x1, x23
100804060: aa1603e2    	mov	x2, x22
100804064: 94273592    	bl	0x1011d16ac <_writev+0x1011d16ac>
100804068: a900d7f6    	stp	x22, x21, [sp, #0x8]
10080406c: f9000ff6    	str	x22, [sp, #0x18]
100804070: 910023e1    	add	x1, sp, #0x8
100804074: aa1303e0    	mov	x0, x19
100804078: 97f5c853    	bl	0x1005761c4 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17hdb67a7b69d944eb9E>
10080407c: 91012339    	add	x25, x25, #0x48
100804080: f1000718    	subs	x24, x24, #0x1
100804084: 540002c0    	b.eq	0x1008040dc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE+0xdc>
100804088: f85f0328    	ldur	x8, [x25, #-0x10]
10080408c: eb1a011f    	cmp	x8, x26
100804090: 54ffff60    	b.eq	0x10080407c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE+0x7c>
100804094: a97f8720    	ldp	x0, x1, [x25, #-0x8]
100804098: 97edb03b    	bl	0x100370184 <__ZN13codexmate_lib4core5relay13codex_catalog30canonical_no_account_slot_slug17h123ace5e8633ba78E>
10080409c: b4ffff00    	cbz	x0, 0x10080407c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE+0x7c>
1008040a0: aa0103f6    	mov	x22, x1
1008040a4: b7f80141    	tbnz	x1, #0x3f, 0x1008040cc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE+0xcc>
1008040a8: aa0003f7    	mov	x23, x0
1008040ac: b4fffd56    	cbz	x22, 0x100804054 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE+0x54>
1008040b0: 97dff531    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008040b4: aa1603e0    	mov	x0, x22
1008040b8: 52800021    	mov	w1, #0x1                ; =1
1008040bc: 97dff52a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008040c0: b40001c0    	cbz	x0, 0x1008040f8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE+0xf8>
1008040c4: aa0003f5    	mov	x21, x0
1008040c8: 17ffffe4    	b	0x100804058 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h46b2d0e332cd9eaeE+0x58>
1008040cc: d2800014    	mov	x20, #0x0               ; =0
1008040d0: aa1403e0    	mov	x0, x20
1008040d4: aa1503e1    	mov	x1, x21
1008040d8: 94273163    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008040dc: a9467bfd    	ldp	x29, x30, [sp, #0x60]
1008040e0: a9454ff4    	ldp	x20, x19, [sp, #0x50]
1008040e4: a94457f6    	ldp	x22, x21, [sp, #0x40]
1008040e8: a9435ff8    	ldp	x24, x23, [sp, #0x30]
1008040ec: a94267fa    	ldp	x26, x25, [sp, #0x20]
1008040f0: 9101c3ff    	add	sp, sp, #0x70
1008040f4: d65f03c0    	ret
1008040f8: aa1603f5    	mov	x21, x22
1008040fc: aa1403e0    	mov	x0, x20
100804100: aa1503e1    	mov	x1, x21
100804104: 94273158    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>

0000000100804108 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4cc864d32037ae6dE>:
100804108: d10303ff    	sub	sp, sp, #0xc0
10080410c: a9085ff8    	stp	x24, x23, [sp, #0x80]
100804110: a90957f6    	stp	x22, x21, [sp, #0x90]
100804114: a90a4ff4    	stp	x20, x19, [sp, #0xa0]
100804118: a90b7bfd    	stp	x29, x30, [sp, #0xb0]
10080411c: 9102c3fd    	add	x29, sp, #0xb0
100804120: ad410400    	ldp	q0, q1, [x0, #0x20]
100804124: ad0107e0    	stp	q0, q1, [sp, #0x20]
100804128: a9444c08    	ldp	x8, x19, [x0, #0x40]
10080412c: f90023e8    	str	x8, [sp, #0x40]
100804130: ad400001    	ldp	q1, q0, [x0]
100804134: a9405434    	ldp	x20, x21, [x1]
100804138: f9400828    	ldr	x8, [x1, #0x10]
10080413c: 8b151516    	add	x22, x8, x21, lsl #5
100804140: 52800317    	mov	w23, #0x18              ; =24
100804144: d2f00018    	mov	x24, #-0x8000000000000000 ; =-9223372036854775808
100804148: ad0003e1    	stp	q1, q0, [sp]
10080414c: d10123a0    	sub	x0, x29, #0x48
100804150: 910003e1    	mov	x1, sp
100804154: 97fd5651    	bl	0x100759a98 <__ZN5alloc11collections5btree3map25IntoIter$LT$K$C$V$C$A$GT$10dying_next17he5a5ebea673209b1E>
100804158: f85b83a8    	ldur	x8, [x29, #-0x48]
10080415c: b4000288    	cbz	x8, 0x1008041ac <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4cc864d32037ae6dE+0xa4>
100804160: f85c83aa    	ldur	x10, [x29, #-0x38]
100804164: 9b17214b    	madd	x11, x10, x23, x8
100804168: f9400569    	ldr	x9, [x11, #0x8]
10080416c: 3dc00560    	ldr	q0, [x11, #0x10]
100804170: 3d8017e0    	str	q0, [sp, #0x50]
100804174: eb18013f    	cmp	x9, x24
100804178: 540001a0    	b.eq	0x1008041ac <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4cc864d32037ae6dE+0xa4>
10080417c: 8b0a0d08    	add	x8, x8, x10, lsl #3
100804180: f9408908    	ldr	x8, [x8, #0x110]
100804184: f940026a    	ldr	x10, [x19]
100804188: 8b080148    	add	x8, x10, x8
10080418c: f9000268    	str	x8, [x19]
100804190: f90002c9    	str	x9, [x22]
100804194: 3dc017e0    	ldr	q0, [sp, #0x50]
100804198: 3c8082c0    	stur	q0, [x22, #0x8]
10080419c: f9000ec8    	str	x8, [x22, #0x18]
1008041a0: 910006b5    	add	x21, x21, #0x1
1008041a4: 910082d6    	add	x22, x22, #0x20
1008041a8: 17ffffe9    	b	0x10080414c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4cc864d32037ae6dE+0x44>
1008041ac: f9000295    	str	x21, [x20]
1008041b0: d10123a0    	sub	x0, x29, #0x48
1008041b4: 910003e1    	mov	x1, sp
1008041b8: 97fd5638    	bl	0x100759a98 <__ZN5alloc11collections5btree3map25IntoIter$LT$K$C$V$C$A$GT$10dying_next17he5a5ebea673209b1E>
1008041bc: f85b83a8    	ldur	x8, [x29, #-0x48]
1008041c0: b4000208    	cbz	x8, 0x100804200 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4cc864d32037ae6dE+0xf8>
1008041c4: 52800313    	mov	w19, #0x18              ; =24
1008041c8: 14000006    	b	0x1008041e0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4cc864d32037ae6dE+0xd8>
1008041cc: d10123a0    	sub	x0, x29, #0x48
1008041d0: 910003e1    	mov	x1, sp
1008041d4: 97fd5631    	bl	0x100759a98 <__ZN5alloc11collections5btree3map25IntoIter$LT$K$C$V$C$A$GT$10dying_next17he5a5ebea673209b1E>
1008041d8: f85b83a8    	ldur	x8, [x29, #-0x48]
1008041dc: b4000128    	cbz	x8, 0x100804200 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4cc864d32037ae6dE+0xf8>
1008041e0: f85c83a9    	ldur	x9, [x29, #-0x38]
1008041e4: 9b132128    	madd	x8, x9, x19, x8
1008041e8: f8408d01    	ldr	x1, [x8, #0x8]!
1008041ec: b4ffff01    	cbz	x1, 0x1008041cc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4cc864d32037ae6dE+0xc4>
1008041f0: f9400500    	ldr	x0, [x8, #0x8]
1008041f4: 52800022    	mov	w2, #0x1                ; =1
1008041f8: 97dff4dc    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008041fc: 17fffff4    	b	0x1008041cc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4cc864d32037ae6dE+0xc4>
100804200: a94b7bfd    	ldp	x29, x30, [sp, #0xb0]
100804204: a94a4ff4    	ldp	x20, x19, [sp, #0xa0]
100804208: a94957f6    	ldp	x22, x21, [sp, #0x90]
10080420c: a9485ff8    	ldp	x24, x23, [sp, #0x80]
100804210: 910303ff    	add	sp, sp, #0xc0
100804214: d65f03c0    	ret
100804218: aa0003f3    	mov	x19, x0
10080421c: f9000295    	str	x21, [x20]
100804220: 910003e0    	mov	x0, sp
100804224: 97fdb533    	bl	0x1007716f0 <__ZN99_$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hf0477a0d0c90776fE>
100804228: aa1303e0    	mov	x0, x19
10080422c: 942733ca    	bl	0x1011d1154 <_writev+0x1011d1154>
100804230: 94273246    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
