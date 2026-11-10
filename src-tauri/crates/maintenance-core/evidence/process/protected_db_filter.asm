
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100511130 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E>:
100511130: d103c3ff    	sub	sp, sp, #0xf0
100511134: a9096ffc    	stp	x28, x27, [sp, #0x90]
100511138: a90a67fa    	stp	x26, x25, [sp, #0xa0]
10051113c: a90b5ff8    	stp	x24, x23, [sp, #0xb0]
100511140: a90c57f6    	stp	x22, x21, [sp, #0xc0]
100511144: a90d4ff4    	stp	x20, x19, [sp, #0xd0]
100511148: a90e7bfd    	stp	x29, x30, [sp, #0xe0]
10051114c: 910383fd    	add	x29, sp, #0xe0
100511150: aa0203f3    	mov	x19, x2
100511154: aa0103f4    	mov	x20, x1
100511158: f9400c17    	ldr	x23, [x0, #0x18]
10051115c: f940041b    	ldr	x27, [x0, #0x8]
100511160: eb17037f    	cmp	x27, x23
100511164: 54000860    	b.eq	0x100511270 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0x140>
100511168: aa0003f5    	mov	x21, x0
10051116c: 910003f9    	mov	x25, sp
100511170: f940007a    	ldr	x26, [x3]
100511174: d2f0001c    	mov	x28, #-0x8000000000000000 ; =-9223372036854775808
100511178: 900080b6    	adrp	x22, 0x101525000 <_anon.a9e2253c03df711f51070ce6102367ea.957+0x57>
10051117c: 913c7ed6    	add	x22, x22, #0xf1f
100511180: 14000003    	b	0x10051118c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0x5c>
100511184: eb17037f    	cmp	x27, x23
100511188: 54000740    	b.eq	0x100511270 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0x140>
10051118c: b9400368    	ldr	w8, [x27]
100511190: 3cc04360    	ldur	q0, [x27, #0x4]
100511194: 3dc00761    	ldr	q1, [x27, #0x10]
100511198: 3c80c3e1    	stur	q1, [sp, #0xc]
10051119c: 3d8003e0    	str	q0, [sp]
1005111a0: 9100837b    	add	x27, x27, #0x20
1005111a4: f90006bb    	str	x27, [x21, #0x8]
1005111a8: a902cff4    	stp	x20, x19, [sp, #0x28]
1005111ac: b9003fe8    	str	w8, [sp, #0x3c]
1005111b0: 3cc04320    	ldur	q0, [x25, #0x4]
1005111b4: 3d8013e0    	str	q0, [sp, #0x40]
1005111b8: f8414321    	ldur	x1, [x25, #0x14]
1005111bc: f9002be1    	str	x1, [sp, #0x50]
1005111c0: f9400348    	ldr	x8, [x26]
1005111c4: eb1c011f    	cmp	x8, x28
1005111c8: 54000061    	b.ne	0x1005111d4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0xa4>
1005111cc: d2800002    	mov	x2, #0x0                ; =0
1005111d0: 14000002    	b	0x1005111d8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0xa8>
1005111d4: a9408f42    	ldp	x2, x3, [x26, #0x8]
1005111d8: f94027e0    	ldr	x0, [sp, #0x48]
1005111dc: 940ad4fc    	bl	0x1007c65cc <__ZN13codexmate_lib8platform7process34is_codex_related_command_with_main17hc48da49d9eea1ecdE>
1005111e0: 36000200    	tbz	w0, #0x0, 0x100511220 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0xf0>
1005111e4: d2f00018    	mov	x24, #-0x8000000000000000 ; =-9223372036854775808
1005111e8: f94023e1    	ldr	x1, [sp, #0x40]
1005111ec: b4000081    	cbz	x1, 0x1005111fc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0xcc>
1005111f0: f94027e0    	ldr	x0, [sp, #0x48]
1005111f4: 52800022    	mov	w2, #0x1                ; =1
1005111f8: 97ebc0dc    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005111fc: eb1c031f    	cmp	x24, x28
100511200: 54fffc20    	b.eq	0x100511184 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0x54>
100511204: f9000278    	str	x24, [x19]
100511208: 3dc01fe0    	ldr	q0, [sp, #0x70]
10051120c: 3c808260    	stur	q0, [x19, #0x8]
100511210: 91006273    	add	x19, x19, #0x18
100511214: f9400eb7    	ldr	x23, [x21, #0x18]
100511218: f94006bb    	ldr	x27, [x21, #0x8]
10051121c: 17ffffda    	b	0x100511184 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0x54>
100511220: 9100f3e8    	add	x8, sp, #0x3c
100511224: f9003be8    	str	x8, [sp, #0x70]
100511228: d0006348    	adrp	x8, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
10051122c: 910bb109    	add	x9, x8, #0x2ec
100511230: 910103e8    	add	x8, sp, #0x40
100511234: a907a3e9    	stp	x9, x8, [sp, #0x78]
100511238: d0000268    	adrp	x8, 0x10055f000 <__ZN5alloc3vec16in_place_collect108_$LT$impl$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$u20$for$u20$alloc..vec..Vec$LT$T$GT$$GT$9from_iter17h47a329132187b06bE+0xc>
10051123c: 911bd108    	add	x8, x8, #0x6f4
100511240: f90047e8    	str	x8, [sp, #0x88]
100511244: 910163e8    	add	x8, sp, #0x58
100511248: 9101c3e1    	add	x1, sp, #0x70
10051124c: aa1603e0    	mov	x0, x22
100511250: 94319991    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100511254: f9402ff8    	ldr	x24, [sp, #0x58]
100511258: 910163e8    	add	x8, sp, #0x58
10051125c: 3cc08100    	ldur	q0, [x8, #0x8]
100511260: 3d801fe0    	str	q0, [sp, #0x70]
100511264: f94023e1    	ldr	x1, [sp, #0x40]
100511268: b5fffc41    	cbnz	x1, 0x1005111f0 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0xc0>
10051126c: 17ffffe4    	b	0x1005111fc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0xcc>
100511270: aa1403e0    	mov	x0, x20
100511274: aa1303e1    	mov	x1, x19
100511278: a94e7bfd    	ldp	x29, x30, [sp, #0xe0]
10051127c: a94d4ff4    	ldp	x20, x19, [sp, #0xd0]
100511280: a94c57f6    	ldp	x22, x21, [sp, #0xc0]
100511284: a94b5ff8    	ldp	x24, x23, [sp, #0xb0]
100511288: a94a67fa    	ldp	x26, x25, [sp, #0xa0]
10051128c: a9496ffc    	ldp	x28, x27, [sp, #0x90]
100511290: 9103c3ff    	add	sp, sp, #0xf0
100511294: d65f03c0    	ret
100511298: aa0003f3    	mov	x19, x0
10051129c: f94023e1    	ldr	x1, [sp, #0x40]
1005112a0: b4000081    	cbz	x1, 0x1005112b0 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hf4f8e181ba02a5d3E+0x180>
1005112a4: f94027e0    	ldr	x0, [sp, #0x48]
1005112a8: 52800022    	mov	w2, #0x1                ; =1
1005112ac: 97ebc0af    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005112b0: 9100a3e0    	add	x0, sp, #0x28
1005112b4: 9401191d    	bl	0x100557728 <__ZN4core3ptr88drop_in_place$LT$alloc..vec..in_place_drop..InPlaceDrop$LT$alloc..string..String$GT$$GT$17haa8877873d4cef89E>
