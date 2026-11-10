
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010050b1d0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE>:
10050b1d0: d10183ff    	sub	sp, sp, #0x60
10050b1d4: a9025ff8    	stp	x24, x23, [sp, #0x20]
10050b1d8: a90357f6    	stp	x22, x21, [sp, #0x30]
10050b1dc: a9044ff4    	stp	x20, x19, [sp, #0x40]
10050b1e0: a9057bfd    	stp	x29, x30, [sp, #0x50]
10050b1e4: 910143fd    	add	x29, sp, #0x50
10050b1e8: aa0003f3    	mov	x19, x0
10050b1ec: 39410428    	ldrb	w8, [x1, #0x41]
10050b1f0: 36000128    	tbz	w8, #0x0, 0x10050b214 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0x44>
10050b1f4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10050b1f8: f9000268    	str	x8, [x19]
10050b1fc: a9457bfd    	ldp	x29, x30, [sp, #0x50]
10050b200: a9444ff4    	ldp	x20, x19, [sp, #0x40]
10050b204: a94357f6    	ldp	x22, x21, [sp, #0x30]
10050b208: a9425ff8    	ldp	x24, x23, [sp, #0x20]
10050b20c: 910183ff    	add	sp, sp, #0x60
10050b210: d65f03c0    	ret
10050b214: aa0103f6    	mov	x22, x1
10050b218: aa0103f4    	mov	x20, x1
10050b21c: f8410e95    	ldr	x21, [x20, #0x10]!
10050b220: 910023e0    	add	x0, sp, #0x8
10050b224: aa1403e1    	mov	x1, x20
10050b228: 94016890    	bl	0x100565468 <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
10050b22c: b9400be8    	ldr	w8, [sp, #0x8]
10050b230: 7100051f    	cmp	w8, #0x1
10050b234: 54000261    	b.ne	0x10050b280 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0xb0>
10050b238: f9400fe8    	ldr	x8, [sp, #0x18]
10050b23c: f94002c9    	ldr	x9, [x22]
10050b240: cb090114    	sub	x20, x8, x9
10050b244: 8b0902b5    	add	x21, x21, x9
10050b248: f90002c8    	str	x8, [x22]
10050b24c: b4000394    	cbz	x20, 0x10050b2bc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0xec>
10050b250: 8b1402a9    	add	x9, x21, x20
10050b254: 385ff128    	ldurb	w8, [x9, #-0x1]
10050b258: 7100291f    	cmp	w8, #0xa
10050b25c: 54000301    	b.ne	0x10050b2bc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0xec>
10050b260: f1000688    	subs	x8, x20, #0x1
10050b264: 540002a0    	b.eq	0x10050b2b8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0xe8>
10050b268: 385fe129    	ldurb	w9, [x9, #-0x2]
10050b26c: d1000a8a    	sub	x10, x20, #0x2
10050b270: 7100353f    	cmp	w9, #0xd
10050b274: 9a9512b5    	csel	x21, x21, x21, ne
10050b278: 9a8a1114    	csel	x20, x8, x10, ne
10050b27c: 14000010    	b	0x10050b2bc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0xec>
10050b280: 394106c8    	ldrb	w8, [x22, #0x41]
10050b284: 3707fb88    	tbnz	w8, #0x0, 0x10050b1f4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0x24>
10050b288: 52800028    	mov	w8, #0x1                ; =1
10050b28c: 390106c8    	strb	w8, [x22, #0x41]
10050b290: 394102ca    	ldrb	w10, [x22, #0x40]
10050b294: a94026c8    	ldp	x8, x9, [x22]
10050b298: 3700006a    	tbnz	w10, #0x0, 0x10050b2a4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0xd4>
10050b29c: eb08013f    	cmp	x9, x8
10050b2a0: 54fffaa0    	b.eq	0x10050b1f4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0x24>
10050b2a4: f940028a    	ldr	x10, [x20]
10050b2a8: cb080134    	sub	x20, x9, x8
10050b2ac: 8b080155    	add	x21, x10, x8
10050b2b0: b5fffd14    	cbnz	x20, 0x10050b250 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0x80>
10050b2b4: 14000002    	b	0x10050b2bc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0xec>
10050b2b8: aa0803f4    	mov	x20, x8
10050b2bc: b4fff9d5    	cbz	x21, 0x10050b1f4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0x24>
10050b2c0: b6f800b4    	tbz	x20, #0x3f, 0x10050b2d4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0x104>
10050b2c4: d2800017    	mov	x23, #0x0               ; =0
10050b2c8: aa1703e0    	mov	x0, x23
10050b2cc: aa1403e1    	mov	x1, x20
10050b2d0: 943314e5    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10050b2d4: b4000134    	cbz	x20, 0x10050b2f8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0x128>
10050b2d8: 97ebd8a7    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10050b2dc: 52800037    	mov	w23, #0x1               ; =1
10050b2e0: aa1403e0    	mov	x0, x20
10050b2e4: 52800021    	mov	w1, #0x1                ; =1
10050b2e8: 97ebd89f    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10050b2ec: b4fffee0    	cbz	x0, 0x10050b2c8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0xf8>
10050b2f0: aa0003f6    	mov	x22, x0
10050b2f4: 14000002    	b	0x10050b2fc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0x12c>
10050b2f8: 52800036    	mov	w22, #0x1               ; =1
10050b2fc: aa1603e0    	mov	x0, x22
10050b300: aa1503e1    	mov	x1, x21
10050b304: aa1403e2    	mov	x2, x20
10050b308: 943318e9    	bl	0x1011d16ac <_writev+0x1011d16ac>
10050b30c: a9005a74    	stp	x20, x22, [x19]
10050b310: f9000a74    	str	x20, [x19, #0x10]
10050b314: 17ffffba    	b	0x10050b1fc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6df205d2190cad0fE+0x2c>
