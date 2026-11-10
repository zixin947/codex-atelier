
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001007c4644 <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E>:
1007c4644: d10403ff    	sub	sp, sp, #0x100
1007c4648: a90b67fa    	stp	x26, x25, [sp, #0xb0]
1007c464c: a90c5ff8    	stp	x24, x23, [sp, #0xc0]
1007c4650: a90d57f6    	stp	x22, x21, [sp, #0xd0]
1007c4654: a90e4ff4    	stp	x20, x19, [sp, #0xe0]
1007c4658: a90f7bfd    	stp	x29, x30, [sp, #0xf0]
1007c465c: 9103c3fd    	add	x29, sp, #0xf0
1007c4660: aa0403f5    	mov	x21, x4
1007c4664: aa0303f6    	mov	x22, x3
1007c4668: aa0203f4    	mov	x20, x2
1007c466c: aa0103e9    	mov	x9, x1
1007c4670: aa0003f3    	mov	x19, x0
1007c4674: 8b021421    	add	x1, x1, x2, lsl #5
1007c4678: 910083e8    	add	x8, sp, #0x20
1007c467c: aa0903e0    	mov	x0, x9
1007c4680: 97f5acb7    	bl	0x10052f95c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17haff5edd42b8c568cE>
1007c4684: a942e3f7    	ldp	x23, x24, [sp, #0x28]
1007c4688: b0005f42    	adrp	x2, 0x1013ad000 <_anon.6408540cdffbe318d49f52812e99783f.1143+0x1>
1007c468c: 911ff842    	add	x2, x2, #0x7fe
1007c4690: d101c3a8    	sub	x8, x29, #0x70
1007c4694: aa1703e0    	mov	x0, x23
1007c4698: aa1803e1    	mov	x1, x24
1007c469c: 52800063    	mov	w3, #0x3                ; =3
1007c46a0: 97e9b529    	bl	0x100231b44 <__ZN5alloc3str17join_generic_copy17hc3fffb246d6f0b82E>
1007c46a4: 3cd903a0    	ldur	q0, [x29, #-0x70]
1007c46a8: 3d8003e0    	str	q0, [sp]
1007c46ac: f85a03a8    	ldur	x8, [x29, #-0x60]
1007c46b0: f9000be8    	str	x8, [sp, #0x10]
1007c46b4: b4000198    	cbz	x24, 0x1007c46e4 <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0xa0>
1007c46b8: 910022f9    	add	x25, x23, #0x8
1007c46bc: 14000004    	b	0x1007c46cc <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x88>
1007c46c0: 91006339    	add	x25, x25, #0x18
1007c46c4: f1000718    	subs	x24, x24, #0x1
1007c46c8: 540000e0    	b.eq	0x1007c46e4 <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0xa0>
1007c46cc: f85f8321    	ldur	x1, [x25, #-0x8]
1007c46d0: b4ffff81    	cbz	x1, 0x1007c46c0 <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x7c>
1007c46d4: f9400320    	ldr	x0, [x25]
1007c46d8: 52800022    	mov	w2, #0x1                ; =1
1007c46dc: 97e0f3a3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007c46e0: 17fffff8    	b	0x1007c46c0 <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x7c>
1007c46e4: f94013e8    	ldr	x8, [sp, #0x20]
1007c46e8: b40000c8    	cbz	x8, 0x1007c4700 <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0xbc>
1007c46ec: 8b080508    	add	x8, x8, x8, lsl #1
1007c46f0: d37df101    	lsl	x1, x8, #3
1007c46f4: aa1703e0    	mov	x0, x23
1007c46f8: 52800102    	mov	w2, #0x8                ; =8
1007c46fc: 97e0f39b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007c4700: b40003f6    	cbz	x22, 0x1007c477c <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x138>
1007c4704: a93b57b6    	stp	x22, x21, [x29, #-0x50]
1007c4708: d10143a8    	sub	x8, x29, #0x50
1007c470c: b00000c9    	adrp	x9, 0x1007dd000 <__ZN3zip9zipcrypto24ZipCryptoReader$LT$R$GT$8validate17h6abfe2f5c5f69c24E+0xf0>
1007c4710: 911b0129    	add	x9, x9, #0x6c0
1007c4714: a90227e8    	stp	x8, x9, [sp, #0x20]
1007c4718: 90006b40    	adrp	x0, 0x10152c000 <_anon.6408540cdffbe318d49f52812e99783f.624+0x973>
1007c471c: 9101b400    	add	x0, x0, #0x6d
1007c4720: d101c3a8    	sub	x8, x29, #0x70
1007c4724: 910083e1    	add	x1, sp, #0x20
1007c4728: 9426cc5b    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1007c472c: 3cd903a0    	ldur	q0, [x29, #-0x70]
1007c4730: 3d800be0    	str	q0, [sp, #0x20]
1007c4734: f85a03a8    	ldur	x8, [x29, #-0x60]
1007c4738: f9001be8    	str	x8, [sp, #0x30]
1007c473c: b4000294    	cbz	x20, 0x1007c478c <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x148>
1007c4740: 97ffe6a9    	bl	0x1007be1e4 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE>
1007c4744: f00001a8    	adrp	x8, 0x1007fb000 <__ZN5rayon4iter7collect21collect_with_consumer17hf967c36702cf3c01E+0x30>
1007c4748: 91095108    	add	x8, x8, #0x254
1007c474c: 910003e9    	mov	x9, sp
1007c4750: 36000340    	tbz	w0, #0x0, 0x1007c47b8 <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x174>
1007c4754: a93923a9    	stp	x9, x8, [x29, #-0x70]
1007c4758: 910083e9    	add	x9, sp, #0x20
1007c475c: a93a23a9    	stp	x9, x8, [x29, #-0x60]
1007c4760: b0005f40    	adrp	x0, 0x1013ad000 <_anon.6408540cdffbe318d49f52812e99783f.1143+0x1>
1007c4764: 91359c00    	add	x0, x0, #0xd67
1007c4768: 910143e8    	add	x8, sp, #0x50
1007c476c: d101c3a1    	sub	x1, x29, #0x70
1007c4770: 9426cc49    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1007c4774: 910143e8    	add	x8, sp, #0x50
1007c4778: 14000019    	b	0x1007c47dc <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x198>
1007c477c: 52800028    	mov	w8, #0x1                ; =1
1007c4780: a90223ff    	stp	xzr, x8, [sp, #0x20]
1007c4784: f9001bff    	str	xzr, [sp, #0x30]
1007c4788: b5fffdd4    	cbnz	x20, 0x1007c4740 <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0xfc>
1007c478c: 910083e8    	add	x8, sp, #0x20
1007c4790: f00001a9    	adrp	x9, 0x1007fb000 <__ZN5rayon4iter7collect21collect_with_consumer17hf967c36702cf3c01E+0x30>
1007c4794: 91095129    	add	x9, x9, #0x254
1007c4798: a93927a8    	stp	x8, x9, [x29, #-0x70]
1007c479c: f0006b20    	adrp	x0, 0x10152b000 <_anon.6408540cdffbe318d49f52812e99783f.200+0x186>
1007c47a0: 913ea800    	add	x0, x0, #0xfaa
1007c47a4: 9100e3e8    	add	x8, sp, #0x38
1007c47a8: d101c3a1    	sub	x1, x29, #0x70
1007c47ac: 9426cc3a    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1007c47b0: 9100e3e8    	add	x8, sp, #0x38
1007c47b4: 1400000a    	b	0x1007c47dc <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x198>
1007c47b8: a93923a9    	stp	x9, x8, [x29, #-0x70]
1007c47bc: 910083e9    	add	x9, sp, #0x20
1007c47c0: a93a23a9    	stp	x9, x8, [x29, #-0x60]
1007c47c4: f0006b20    	adrp	x0, 0x10152b000 <_anon.6408540cdffbe318d49f52812e99783f.200+0x186>
1007c47c8: 913ff800    	add	x0, x0, #0xffe
1007c47cc: 9101a3e8    	add	x8, sp, #0x68
1007c47d0: d101c3a1    	sub	x1, x29, #0x70
1007c47d4: 9426cc30    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1007c47d8: 9101a3e8    	add	x8, sp, #0x68
1007c47dc: 3dc00100    	ldr	q0, [x8]
1007c47e0: 3c808260    	stur	q0, [x19, #0x8]
1007c47e4: f9400908    	ldr	x8, [x8, #0x10]
1007c47e8: f9000e68    	str	x8, [x19, #0x18]
1007c47ec: 52800148    	mov	w8, #0xa                ; =10
1007c47f0: f9000268    	str	x8, [x19]
1007c47f4: f94013e1    	ldr	x1, [sp, #0x20]
1007c47f8: b4000081    	cbz	x1, 0x1007c4808 <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x1c4>
1007c47fc: f94017e0    	ldr	x0, [sp, #0x28]
1007c4800: 52800022    	mov	w2, #0x1                ; =1
1007c4804: 97e0f359    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007c4808: f94003e1    	ldr	x1, [sp]
1007c480c: b4000081    	cbz	x1, 0x1007c481c <__ZN13codexmate_lib8platform7process30codex_transition_quiet_timeout17hc0c2bb3a81580a36E+0x1d8>
1007c4810: f94007e0    	ldr	x0, [sp, #0x8]
1007c4814: 52800022    	mov	w2, #0x1                ; =1
1007c4818: 97e0f354    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007c481c: a94f7bfd    	ldp	x29, x30, [sp, #0xf0]
1007c4820: a94e4ff4    	ldp	x20, x19, [sp, #0xe0]
1007c4824: a94d57f6    	ldp	x22, x21, [sp, #0xd0]
1007c4828: a94c5ff8    	ldp	x24, x23, [sp, #0xc0]
1007c482c: a94b67fa    	ldp	x26, x25, [sp, #0xb0]
1007c4830: 910403ff    	add	sp, sp, #0x100
1007c4834: d65f03c0    	ret
