
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100208530 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE>:
100208530: 6db923e9    	stp	d9, d8, [sp, #-0x70]!
100208534: a9016ffc    	stp	x28, x27, [sp, #0x10]
100208538: a90267fa    	stp	x26, x25, [sp, #0x20]
10020853c: a9035ff8    	stp	x24, x23, [sp, #0x30]
100208540: a90457f6    	stp	x22, x21, [sp, #0x40]
100208544: a9054ff4    	stp	x20, x19, [sp, #0x50]
100208548: a9067bfd    	stp	x29, x30, [sp, #0x60]
10020854c: 910183fd    	add	x29, sp, #0x60
100208550: d106c3ff    	sub	sp, sp, #0x1b0
100208554: aa0203f5    	mov	x21, x2
100208558: aa0103f4    	mov	x20, x1
10020855c: aa0003f9    	mov	x25, x0
100208560: b000b740    	adrp	x0, 0x1018f1000 <__ZN3std12backtrace_rs9symbolize5gimli5Cache11with_global14MAPPINGS_CACHE17hb3a3380e67302d28E>
100208564: 912f6000    	add	x0, x0, #0xbd8
100208568: f9400008    	ldr	x8, [x0]
10020856c: d63f0100    	blr	x8
100208570: aa0003f6    	mov	x22, x0
100208574: 39404008    	ldrb	w8, [x0, #0x10]
100208578: 7100051f    	cmp	w8, #0x1
10020857c: 540030a1    	b.ne	0x100208b90 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x660>
100208580: a94062d7    	ldp	x23, x24, [x22]
100208584: 910006e8    	add	x8, x23, #0x1
100208588: f90002c8    	str	x8, [x22]
10020858c: 910303e0    	add	x0, sp, #0xc0
100208590: 52800201    	mov	w1, #0x10               ; =16
100208594: aa1503e2    	mov	x2, x21
100208598: 52800023    	mov	w3, #0x1                ; =1
10020859c: 9419bc35    	bl	0x100877670 <__ZN9hashbrown3raw13RawTableInner22fallible_with_capacity17hdb9e36244737cba4E>
1002085a0: ad4607e0    	ldp	q0, q1, [sp, #0xc0]
1002085a4: ad0107e0    	stp	q0, q1, [sp, #0x20]
1002085a8: a90463f7    	stp	x23, x24, [sp, #0x40]
1002085ac: d378deb3    	lsl	x19, x21, #8
1002085b0: 8b130298    	add	x24, x20, x19
1002085b4: d1040297    	sub	x23, x20, #0x100
1002085b8: b40002d3    	cbz	x19, 0x100208610 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0xe0>
1002085bc: a9508ae1    	ldp	x1, x2, [x23, #0x108]
1002085c0: 910083e0    	add	x0, sp, #0x20
1002085c4: 940d9f0d    	bl	0x1005701f8 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h0875cbc66dae6dc0E>
1002085c8: 910402f7    	add	x23, x23, #0x100
1002085cc: d1040273    	sub	x19, x19, #0x100
1002085d0: 34ffff40    	cbz	w0, 0x1002085b8 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x88>
1002085d4: b0000148    	adrp	x8, 0x100231000 <__ZN4core5slice4sort6stable5merge5merge17he20bffacf3d2822eE+0x208>
1002085d8: 91189108    	add	x8, x8, #0x624
1002085dc: a90c23f7    	stp	x23, x8, [sp, #0xc0]
1002085e0: b00098a0    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
1002085e4: 9121a800    	add	x0, x0, #0x86a
1002085e8: 910163e8    	add	x8, sp, #0x58
1002085ec: 910303e1    	add	x1, sp, #0xc0
1002085f0: 943dbca9    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1002085f4: 3cc583e0    	ldur	q0, [sp, #0x58]
1002085f8: 3c808320    	stur	q0, [x25, #0x8]
1002085fc: f94037e8    	ldr	x8, [sp, #0x68]
100208600: f9000f28    	str	x8, [x25, #0x18]
100208604: 52800128    	mov	w8, #0x9                ; =9
100208608: f9000328    	str	x8, [x25]
10020860c: 1400014d    	b	0x100208b40 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x610>
100208610: 394042c8    	ldrb	w8, [x22, #0x10]
100208614: 7100051f    	cmp	w8, #0x1
100208618: 54002ca1    	b.ne	0x100208bac <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x67c>
10020861c: a94006c0    	ldp	x0, x1, [x22]
100208620: f90007f9    	str	x25, [sp, #0x8]
100208624: 91000408    	add	x8, x0, #0x1
100208628: f90002c8    	str	x8, [x22]
10020862c: d000b068    	adrp	x8, 0x101816000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1650+0x18>
100208630: 913b6108    	add	x8, x8, #0xed8
100208634: ad400500    	ldp	q0, q1, [x8]
100208638: ad0607e0    	stp	q0, q1, [sp, #0xc0]
10020863c: a90e07e0    	stp	x0, x1, [sp, #0xe0]
100208640: b40020d5    	cbz	x21, 0x100208a58 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x528>
100208644: d2f00013    	mov	x19, #-0x8000000000000000 ; =-9223372036854775808
100208648: aa1403f6    	mov	x22, x20
10020864c: 14000004    	b	0x10020865c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x12c>
100208650: 910402d6    	add	x22, x22, #0x100
100208654: f10006b5    	subs	x21, x21, #0x1
100208658: 54000260    	b.eq	0x1002086a4 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x174>
10020865c: a94086c0    	ldp	x0, x1, [x22, #0x8]
100208660: 9101c3e8    	add	x8, sp, #0x70
100208664: 941662e3    	bl	0x1007a11f0 <__ZN13codexmate_lib4core5relay6models26validate_relay_provider_id17h52e062fe96fe3cd7E>
100208668: f9403be1    	ldr	x1, [sp, #0x70]
10020866c: eb13003f    	cmp	x1, x19
100208670: 54000101    	b.ne	0x100208690 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x160>
100208674: 9101c3e8    	add	x8, sp, #0x70
100208678: aa1603e0    	mov	x0, x22
10020867c: 943dc3a0    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
100208680: 910303e0    	add	x0, sp, #0xc0
100208684: 9101c3e1    	add	x1, sp, #0x70
100208688: 940db6cf    	bl	0x1005761c4 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17hdb67a7b69d944eb9E>
10020868c: 17fffff1    	b	0x100208650 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x120>
100208690: b4fffe01    	cbz	x1, 0x100208650 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x120>
100208694: f9403fe0    	ldr	x0, [sp, #0x78]
100208698: 52800022    	mov	w2, #0x1                ; =1
10020869c: 97f7e3b3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1002086a0: 17ffffec    	b	0x100208650 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x120>
1002086a4: d280001b    	mov	x27, #0x0               ; =0
1002086a8: ad4607e0    	ldp	q0, q1, [sp, #0xc0]
1002086ac: ad0387e0    	stp	q0, q1, [sp, #0x70]
1002086b0: 3dc03be0    	ldr	q0, [sp, #0xe0]
1002086b4: 3d8027e0    	str	q0, [sp, #0x90]
1002086b8: 52800108    	mov	w8, #0x8                ; =8
1002086bc: a90aa3ff    	stp	xzr, x8, [sp, #0xa8]
1002086c0: f9005fff    	str	xzr, [sp, #0xb8]
1002086c4: d2f00013    	mov	x19, #-0x8000000000000000 ; =-9223372036854775808
1002086c8: f9000ff8    	str	x24, [sp, #0x18]
1002086cc: 14000012    	b	0x100208714 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x1e4>
1002086d0: f9405be8    	ldr	x8, [sp, #0xb0]
1002086d4: 8b131908    	add	x8, x8, x19, lsl #6
1002086d8: 3dc033e0    	ldr	q0, [sp, #0xc0]
1002086dc: 3d800100    	str	q0, [x8]
1002086e0: 3dc037e0    	ldr	q0, [sp, #0xd0]
1002086e4: 3d800500    	str	q0, [x8, #0x10]
1002086e8: 3dc03be0    	ldr	q0, [sp, #0xe0]
1002086ec: 3d800900    	str	q0, [x8, #0x20]
1002086f0: 3dc03fe0    	ldr	q0, [sp, #0xf0]
1002086f4: 3d800d00    	str	q0, [x8, #0x30]
1002086f8: 91000668    	add	x8, x19, #0x1
1002086fc: f9005fe8    	str	x8, [sp, #0xb8]
100208700: d2f00013    	mov	x19, #-0x8000000000000000 ; =-9223372036854775808
100208704: 91040294    	add	x20, x20, #0x100
100208708: 9100077b    	add	x27, x27, #0x1
10020870c: eb18029f    	cmp	x20, x24
100208710: 54001b20    	b.eq	0x100208a74 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x544>
100208714: a9408680    	ldp	x0, x1, [x20, #0x8]
100208718: 910303e8    	add	x8, sp, #0xc0
10020871c: 941662b5    	bl	0x1007a11f0 <__ZN13codexmate_lib4core5relay6models26validate_relay_provider_id17h52e062fe96fe3cd7E>
100208720: f94063e1    	ldr	x1, [sp, #0xc0]
100208724: eb13003f    	cmp	x1, x19
100208728: 54fffee0    	b.eq	0x100208704 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x1d4>
10020872c: b4000081    	cbz	x1, 0x10020873c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x20c>
100208730: f94067e0    	ldr	x0, [sp, #0xc8]
100208734: 52800022    	mov	w2, #0x1                ; =1
100208738: 97f7e38c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10020873c: a9408a81    	ldp	x1, x2, [x20, #0x8]
100208740: 910423e0    	add	x0, sp, #0x108
100208744: 940068c1    	bl	0x100222a48 <__ZN44_$LT$D$u20$as$u20$digest..digest..Digest$GT$6digest17h552c414baf0732ceE>
100208748: d103a3a8    	sub	x8, x29, #0xe8
10020874c: 910423e0    	add	x0, sp, #0x108
100208750: 910423e9    	add	x9, sp, #0x108
100208754: 91002121    	add	x1, x9, #0x8
100208758: 94180df3    	bl	0x10080bf24 <__ZN112_$LT$alloc..string..String$u20$as$u20$core..iter..traits..collect..FromIterator$LT$alloc..string..String$GT$$GT$9from_iter17hc2d6ae7d0a7c2bdbE>
10020875c: d103a3a8    	sub	x8, x29, #0xe8
100208760: f81703a8    	stur	x8, [x29, #-0x90]
100208764: f0000148    	adrp	x8, 0x100233000 <__ZN5tauri8protocol5tauri3get28_$u7b$$u7b$closure$u7d$$u7d$17h4c810e8977013762E+0x220>
100208768: 912be108    	add	x8, x8, #0xaf8
10020876c: f81783a8    	stur	x8, [x29, #-0x88]
100208770: 910303e8    	add	x8, sp, #0xc0
100208774: d10243a1    	sub	x1, x29, #0x90
100208778: b00098a0    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
10020877c: 911fdc00    	add	x0, x0, #0x7f7
100208780: 943dbc45    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100208784: 3dc033e0    	ldr	q0, [sp, #0xc0]
100208788: 3c9303a0    	stur	q0, [x29, #-0xd0]
10020878c: f9406be8    	ldr	x8, [sp, #0xd0]
100208790: f81403a8    	stur	x8, [x29, #-0xc0]
100208794: f94047e8    	ldr	x8, [sp, #0x88]
100208798: b4000628    	cbz	x8, 0x10020885c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x32c>
10020879c: 9101c3e8    	add	x8, sp, #0x70
1002087a0: 91008100    	add	x0, x8, #0x20
1002087a4: d10343a1    	sub	x1, x29, #0xd0
1002087a8: 9412ef90    	bl	0x1006c45e8 <__ZN4core4hash11BuildHasher8hash_one17h7a8f16f0d130c57dE>
1002087ac: d2800016    	mov	x22, #0x0               ; =0
1002087b0: d379fc08    	lsr	x8, x0, #57
1002087b4: a9474ff5    	ldp	x21, x19, [sp, #0x70]
1002087b8: 0e010d08    	dup.8b	v8, w8
1002087bc: d10062b9    	sub	x25, x21, #0x18
1002087c0: a973dfb8    	ldp	x24, x23, [x29, #-0xc8]
1002087c4: 8a13001a    	and	x26, x0, x19
1002087c8: fc7a6aa9    	ldr	d9, [x21, x26]
1002087cc: 2e288d20    	cmeq.8b	v0, v9, v8
1002087d0: 9e660008    	fmov	x8, d0
1002087d4: f201c11c    	ands	x28, x8, #0x8080808080808080
1002087d8: 54000260    	b.eq	0x100208824 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x2f4>
1002087dc: dac00388    	rbit	x8, x28
1002087e0: dac01108    	clz	x8, x8
1002087e4: 8b480f48    	add	x8, x26, x8, lsr #3
1002087e8: 8a130108    	and	x8, x8, x19
1002087ec: 52800309    	mov	w9, #0x18               ; =24
1002087f0: 9b09fd08    	mneg	x8, x8, x9
1002087f4: 8b080328    	add	x8, x25, x8
1002087f8: f9400909    	ldr	x9, [x8, #0x10]
1002087fc: eb0902ff    	cmp	x23, x9
100208800: 540000c1    	b.ne	0x100208818 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x2e8>
100208804: f9400501    	ldr	x1, [x8, #0x8]
100208808: aa1803e0    	mov	x0, x24
10020880c: aa1703e2    	mov	x2, x23
100208810: 943f23a4    	bl	0x1011d16a0 <_writev+0x1011d16a0>
100208814: 340002e0    	cbz	w0, 0x100208870 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x340>
100208818: d1000788    	sub	x8, x28, #0x1
10020881c: ea1c011c    	ands	x28, x8, x28
100208820: 54fffde1    	b.ne	0x1002087dc <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x2ac>
100208824: 6f07e7e0    	movi.2d	v0, #0xffffffffffffffff
100208828: 2e208d20    	cmeq.8b	v0, v9, v0
10020882c: 2e30a800    	umaxv.8b	b0, v0
100208830: 1e260008    	fmov	w8, s0
100208834: 37000148    	tbnz	w8, #0x0, 0x10020885c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x32c>
100208838: 910022d6    	add	x22, x22, #0x8
10020883c: 8b160340    	add	x0, x26, x22
100208840: 8a13001a    	and	x26, x0, x19
100208844: fc7a6aa9    	ldr	d9, [x21, x26]
100208848: 2e288d20    	cmeq.8b	v0, v9, v8
10020884c: 9e660008    	fmov	x8, d0
100208850: f201c11c    	ands	x28, x8, #0x8080808080808080
100208854: 54fffc41    	b.ne	0x1002087dc <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x2ac>
100208858: 17fffff3    	b	0x100208824 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x2f4>
10020885c: 3cd303a0    	ldur	q0, [x29, #-0xd0]
100208860: 3c9703a0    	stur	q0, [x29, #-0x90]
100208864: f85403a8    	ldur	x8, [x29, #-0xc0]
100208868: f81803a8    	stur	x8, [x29, #-0x80]
10020886c: 14000059    	b	0x1002089d0 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x4a0>
100208870: 52800036    	mov	w22, #0x1               ; =1
100208874: b814c3b6    	stur	w22, [x29, #-0xb4]
100208878: d10343a8    	sub	x8, x29, #0xd0
10020887c: f90063e8    	str	x8, [sp, #0xc0]
100208880: f0000148    	adrp	x8, 0x100233000 <__ZN5tauri8protocol5tauri3get28_$u7b$$u7b$closure$u7d$$u7d$17h4c810e8977013762E+0x220>
100208884: 912be109    	add	x9, x8, #0xaf8
100208888: d102d3a8    	sub	x8, x29, #0xb4
10020888c: a90ca3e9    	stp	x9, x8, [sp, #0xc8]
100208890: f0007b88    	adrp	x8, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
100208894: 910bb108    	add	x8, x8, #0x2ec
100208898: f9006fe8    	str	x8, [sp, #0xd8]
10020889c: d10243a8    	sub	x8, x29, #0x90
1002088a0: 910303e1    	add	x1, sp, #0xc0
1002088a4: b00098a0    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
1002088a8: 91203800    	add	x0, x0, #0x80e
1002088ac: 943dbbfa    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1002088b0: 3cd703a0    	ldur	q0, [x29, #-0x90]
1002088b4: 3c9503a0    	stur	q0, [x29, #-0xb0]
1002088b8: f85803a8    	ldur	x8, [x29, #-0x80]
1002088bc: f81603a8    	stur	x8, [x29, #-0xa0]
1002088c0: f94047e8    	ldr	x8, [sp, #0x88]
1002088c4: b4000748    	cbz	x8, 0x1002089ac <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x47c>
1002088c8: 9101c3e8    	add	x8, sp, #0x70
1002088cc: 91008100    	add	x0, x8, #0x20
1002088d0: d102c3a1    	sub	x1, x29, #0xb0
1002088d4: 9412ef45    	bl	0x1006c45e8 <__ZN4core4hash11BuildHasher8hash_one17h7a8f16f0d130c57dE>
1002088d8: d2800009    	mov	x9, #0x0                ; =0
1002088dc: 110006d6    	add	w22, w22, #0x1
1002088e0: d379fc08    	lsr	x8, x0, #57
1002088e4: a9474ff5    	ldp	x21, x19, [sp, #0x70]
1002088e8: 0e010d08    	dup.8b	v8, w8
1002088ec: d10062b9    	sub	x25, x21, #0x18
1002088f0: a975e3b7    	ldp	x23, x24, [x29, #-0xa8]
1002088f4: 8a13001a    	and	x26, x0, x19
1002088f8: fc7a6aa9    	ldr	d9, [x21, x26]
1002088fc: 2e288d20    	cmeq.8b	v0, v9, v8
100208900: 9e660008    	fmov	x8, d0
100208904: f201c11c    	ands	x28, x8, #0x8080808080808080
100208908: 540002a0    	b.eq	0x10020895c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x42c>
10020890c: f9000be9    	str	x9, [sp, #0x10]
100208910: dac00388    	rbit	x8, x28
100208914: dac01108    	clz	x8, x8
100208918: 8b480f48    	add	x8, x26, x8, lsr #3
10020891c: 8a130108    	and	x8, x8, x19
100208920: 52800309    	mov	w9, #0x18               ; =24
100208924: 9b09fd08    	mneg	x8, x8, x9
100208928: 8b080328    	add	x8, x25, x8
10020892c: f9400909    	ldr	x9, [x8, #0x10]
100208930: eb09031f    	cmp	x24, x9
100208934: 540000c1    	b.ne	0x10020894c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x41c>
100208938: f9400501    	ldr	x1, [x8, #0x8]
10020893c: aa1703e0    	mov	x0, x23
100208940: aa1803e2    	mov	x2, x24
100208944: 943f2357    	bl	0x1011d16a0 <_writev+0x1011d16a0>
100208948: 34000260    	cbz	w0, 0x100208994 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x464>
10020894c: d1000788    	sub	x8, x28, #0x1
100208950: ea1c011c    	ands	x28, x8, x28
100208954: f9400be9    	ldr	x9, [sp, #0x10]
100208958: 54fffdc1    	b.ne	0x100208910 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x3e0>
10020895c: 6f07e7e0    	movi.2d	v0, #0xffffffffffffffff
100208960: 2e208d20    	cmeq.8b	v0, v9, v0
100208964: 2e30a800    	umaxv.8b	b0, v0
100208968: 1e260008    	fmov	w8, s0
10020896c: 37000208    	tbnz	w8, #0x0, 0x1002089ac <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x47c>
100208970: 91002129    	add	x9, x9, #0x8
100208974: 8b090340    	add	x0, x26, x9
100208978: 8a13001a    	and	x26, x0, x19
10020897c: fc7a6aa9    	ldr	d9, [x21, x26]
100208980: 2e288d20    	cmeq.8b	v0, v9, v8
100208984: 9e660008    	fmov	x8, d0
100208988: f201c11c    	ands	x28, x8, #0x8080808080808080
10020898c: 54fffc01    	b.ne	0x10020890c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x3dc>
100208990: 17fffff3    	b	0x10020895c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x42c>
100208994: f85503a1    	ldur	x1, [x29, #-0xb0]
100208998: b4fff6e1    	cbz	x1, 0x100208874 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x344>
10020899c: aa1703e0    	mov	x0, x23
1002089a0: 52800022    	mov	w2, #0x1                ; =1
1002089a4: 97f7e2f1    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1002089a8: 17ffffb3    	b	0x100208874 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x344>
1002089ac: 3cd503a0    	ldur	q0, [x29, #-0xb0]
1002089b0: 3c9703a0    	stur	q0, [x29, #-0x90]
1002089b4: f85603a8    	ldur	x8, [x29, #-0xa0]
1002089b8: f81803a8    	stur	x8, [x29, #-0x80]
1002089bc: f85303a1    	ldur	x1, [x29, #-0xd0]
1002089c0: b4000081    	cbz	x1, 0x1002089d0 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x4a0>
1002089c4: f85383a0    	ldur	x0, [x29, #-0xc8]
1002089c8: 52800022    	mov	w2, #0x1                ; =1
1002089cc: 97f7e2e7    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1002089d0: f85183a1    	ldur	x1, [x29, #-0xe8]
1002089d4: f9400ff8    	ldr	x24, [sp, #0x18]
1002089d8: b4000081    	cbz	x1, 0x1002089e8 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x4b8>
1002089dc: f85203a0    	ldur	x0, [x29, #-0xe0]
1002089e0: 52800022    	mov	w2, #0x1                ; =1
1002089e4: 97f7e2e1    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1002089e8: 910303e8    	add	x8, sp, #0xc0
1002089ec: d10243a0    	sub	x0, x29, #0x90
1002089f0: 943dc2c3    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
1002089f4: 9101c3e0    	add	x0, sp, #0x70
1002089f8: 910303e1    	add	x1, sp, #0xc0
1002089fc: 940db5f2    	bl	0x1005761c4 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17hdb67a7b69d944eb9E>
100208a00: 910423e8    	add	x8, sp, #0x108
100208a04: aa1403e0    	mov	x0, x20
100208a08: 943dc2bd    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
100208a0c: 3cd703a0    	ldur	q0, [x29, #-0x90]
100208a10: 910303e9    	add	x9, sp, #0xc0
100208a14: 3c818120    	stur	q0, [x9, #0x18]
100208a18: f85803a8    	ldur	x8, [x29, #-0x80]
100208a1c: f8028128    	stur	x8, [x9, #0x28]
100208a20: 9102a3e8    	add	x8, sp, #0xa8
100208a24: 3dc01900    	ldr	q0, [x8, #0x60]
100208a28: 3d8033e0    	str	q0, [sp, #0xc0]
100208a2c: f9408fe8    	ldr	x8, [sp, #0x118]
100208a30: f9007bfb    	str	x27, [sp, #0xf0]
100208a34: f9006be8    	str	x8, [sp, #0xd0]
100208a38: 3903e3ff    	strb	wzr, [sp, #0xf8]
100208a3c: f9405ff3    	ldr	x19, [sp, #0xb8]
100208a40: f94057e8    	ldr	x8, [sp, #0xa8]
100208a44: eb08027f    	cmp	x19, x8
100208a48: 54ffe441    	b.ne	0x1002086d0 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x1a0>
100208a4c: 9102a3e0    	add	x0, sp, #0xa8
100208a50: 943e1686    	bl	0x10118e468 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17he40067c1696386d9E>
100208a54: 17ffff1f    	b	0x1002086d0 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x1a0>
100208a58: ad4607e0    	ldp	q0, q1, [sp, #0xc0]
100208a5c: ad0387e0    	stp	q0, q1, [sp, #0x70]
100208a60: 3dc03be0    	ldr	q0, [sp, #0xe0]
100208a64: 3d8027e0    	str	q0, [sp, #0x90]
100208a68: 52800108    	mov	w8, #0x8                ; =8
100208a6c: a90aa3ff    	stp	xzr, x8, [sp, #0xa8]
100208a70: f9005fff    	str	xzr, [sp, #0xb8]
100208a74: 9102a3e8    	add	x8, sp, #0xa8
100208a78: 3dc00100    	ldr	q0, [x8]
100208a7c: f94007e9    	ldr	x9, [sp, #0x8]
100208a80: 3c808120    	stur	q0, [x9, #0x8]
100208a84: f9405fe8    	ldr	x8, [sp, #0xb8]
100208a88: f9000d28    	str	x8, [x9, #0x18]
100208a8c: 52800168    	mov	w8, #0xb                ; =11
100208a90: f9000128    	str	x8, [x9]
100208a94: f9403ff3    	ldr	x19, [sp, #0x78]
100208a98: b4000553    	cbz	x19, 0x100208b40 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x610>
100208a9c: f94047f4    	ldr	x20, [sp, #0x88]
100208aa0: b40003d4    	cbz	x20, 0x100208b18 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x5e8>
100208aa4: f9403bf5    	ldr	x21, [sp, #0x70]
100208aa8: aa1503f6    	mov	x22, x21
100208aac: fc4086c0    	ldr	d0, [x22], #0x8
100208ab0: 2e208800    	cmge.8b	v0, v0, #0
100208ab4: 9e660008    	fmov	x8, d0
100208ab8: 9201c118    	and	x24, x8, #0x8080808080808080
100208abc: 52800317    	mov	w23, #0x18              ; =24
100208ac0: 14000005    	b	0x100208ad4 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x5a4>
100208ac4: d1000708    	sub	x8, x24, #0x1
100208ac8: 8a180118    	and	x24, x8, x24
100208acc: d1000694    	sub	x20, x20, #0x1
100208ad0: b4000254    	cbz	x20, 0x100208b18 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x5e8>
100208ad4: b50000f8    	cbnz	x24, 0x100208af0 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x5c0>
100208ad8: fc4086c0    	ldr	d0, [x22], #0x8
100208adc: 2e208800    	cmge.8b	v0, v0, #0
100208ae0: 9e660008    	fmov	x8, d0
100208ae4: d10302b5    	sub	x21, x21, #0xc0
100208ae8: 9201c118    	and	x24, x8, #0x8080808080808080
100208aec: b4ffff78    	cbz	x24, 0x100208ad8 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x5a8>
100208af0: dac00308    	rbit	x8, x24
100208af4: dac01108    	clz	x8, x8
100208af8: cb480fe8    	neg	x8, x8, lsr #3
100208afc: 9b375508    	smaddl	x8, w8, w23, x21
100208b00: f85e8101    	ldur	x1, [x8, #-0x18]
100208b04: b4fffe01    	cbz	x1, 0x100208ac4 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x594>
100208b08: f85f0100    	ldur	x0, [x8, #-0x10]
100208b0c: 52800022    	mov	w2, #0x1                ; =1
100208b10: 97f7e296    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100208b14: 17ffffec    	b	0x100208ac4 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x594>
100208b18: 8b130668    	add	x8, x19, x19, lsl #1
100208b1c: d37df108    	lsl	x8, x8, #3
100208b20: 8b130109    	add	x9, x8, x19
100208b24: b1008521    	adds	x1, x9, #0x21
100208b28: 540000c0    	b.eq	0x100208b40 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x610>
100208b2c: f9403be9    	ldr	x9, [sp, #0x70]
100208b30: cb080128    	sub	x8, x9, x8
100208b34: d1006100    	sub	x0, x8, #0x18
100208b38: 52800102    	mov	w2, #0x8                ; =8
100208b3c: 97f7e28b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100208b40: f94017e9    	ldr	x9, [sp, #0x28]
100208b44: b4000149    	cbz	x9, 0x100208b6c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x63c>
100208b48: d37ced28    	lsl	x8, x9, #4
100208b4c: 8b090109    	add	x9, x8, x9
100208b50: b1006521    	adds	x1, x9, #0x19
100208b54: 540000c0    	b.eq	0x100208b6c <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x63c>
100208b58: f94013e9    	ldr	x9, [sp, #0x20]
100208b5c: cb080128    	sub	x8, x9, x8
100208b60: d1004100    	sub	x0, x8, #0x10
100208b64: 52800102    	mov	w2, #0x8                ; =8
100208b68: 97f7e280    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100208b6c: 9106c3ff    	add	sp, sp, #0x1b0
100208b70: a9467bfd    	ldp	x29, x30, [sp, #0x60]
100208b74: a9454ff4    	ldp	x20, x19, [sp, #0x50]
100208b78: a94457f6    	ldp	x22, x21, [sp, #0x40]
100208b7c: a9435ff8    	ldp	x24, x23, [sp, #0x30]
100208b80: a94267fa    	ldp	x26, x25, [sp, #0x20]
100208b84: a9416ffc    	ldp	x28, x27, [sp, #0x10]
100208b88: 6cc723e9    	ldp	d9, d8, [sp], #0x70
100208b8c: d65f03c0    	ret
100208b90: 943d0a41    	bl	0x10114b494 <__ZN3std3sys6random19hashmap_random_keys17h8d42fafffd67301dE>
100208b94: aa0003f7    	mov	x23, x0
100208b98: aa0103f8    	mov	x24, x1
100208b9c: a90006c0    	stp	x0, x1, [x22]
100208ba0: 52800028    	mov	w8, #0x1                ; =1
100208ba4: 390042c8    	strb	w8, [x22, #0x10]
100208ba8: 17fffe77    	b	0x100208584 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x54>
100208bac: 943d0a3a    	bl	0x10114b494 <__ZN3std3sys6random19hashmap_random_keys17h8d42fafffd67301dE>
100208bb0: a90006c0    	stp	x0, x1, [x22]
100208bb4: 52800028    	mov	w8, #0x1                ; =1
100208bb8: 390042c8    	strb	w8, [x22, #0x10]
100208bbc: 17fffe99    	b	0x100208620 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0xf0>
100208bc0: aa0003f3    	mov	x19, x0
100208bc4: 910083e0    	add	x0, sp, #0x20
100208bc8: 941994e5    	bl	0x10086df5c <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3e2fcf4b3ccc9d35E>
100208bcc: aa1303e0    	mov	x0, x19
100208bd0: 943f2161    	bl	0x1011d1154 <_writev+0x1011d1154>
100208bd4: aa0003f3    	mov	x19, x0
100208bd8: 910303e0    	add	x0, sp, #0xc0
100208bdc: 94009246    	bl	0x10022d4f4 <__ZN4core3ptr77drop_in_place$LT$codexmate_lib..core..relay..storage..ProviderIdMigration$GT$17heaad4f03d91fdb29E>
100208be0: 14000022    	b	0x100208c68 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x738>
100208be4: 14000008    	b	0x100208c04 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x6d4>
100208be8: aa0003f3    	mov	x19, x0
100208bec: f85503a1    	ldur	x1, [x29, #-0xb0]
100208bf0: b40000c1    	cbz	x1, 0x100208c08 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x6d8>
100208bf4: f85583a0    	ldur	x0, [x29, #-0xa8]
100208bf8: 52800022    	mov	w2, #0x1                ; =1
100208bfc: 97f7e25b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100208c00: 14000002    	b	0x100208c08 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x6d8>
100208c04: aa0003f3    	mov	x19, x0
100208c08: f85303a1    	ldur	x1, [x29, #-0xd0]
100208c0c: b40000c1    	cbz	x1, 0x100208c24 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x6f4>
100208c10: f85383a0    	ldur	x0, [x29, #-0xc8]
100208c14: 52800022    	mov	w2, #0x1                ; =1
100208c18: 97f7e254    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100208c1c: 14000002    	b	0x100208c24 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x6f4>
100208c20: aa0003f3    	mov	x19, x0
100208c24: f85183a1    	ldur	x1, [x29, #-0xe8]
100208c28: b4000201    	cbz	x1, 0x100208c68 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x738>
100208c2c: f85203a0    	ldur	x0, [x29, #-0xe0]
100208c30: 14000005    	b	0x100208c44 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x714>
100208c34: aa0003f3    	mov	x19, x0
100208c38: f85703a1    	ldur	x1, [x29, #-0x90]
100208c3c: b4000161    	cbz	x1, 0x100208c68 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x738>
100208c40: f85783a0    	ldur	x0, [x29, #-0x88]
100208c44: 52800022    	mov	w2, #0x1                ; =1
100208c48: 97f7e248    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100208c4c: 14000007    	b	0x100208c68 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x738>
100208c50: aa0003f3    	mov	x19, x0
100208c54: 910083e0    	add	x0, sp, #0x20
100208c58: 941994c1    	bl	0x10086df5c <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3e2fcf4b3ccc9d35E>
100208c5c: aa1303e0    	mov	x0, x19
100208c60: 943f213d    	bl	0x1011d1154 <_writev+0x1011d1154>
100208c64: aa0003f3    	mov	x19, x0
100208c68: 9102a3e0    	add	x0, sp, #0xa8
100208c6c: 94006f2c    	bl	0x10022491c <__ZN4core3ptr100drop_in_place$LT$alloc..vec..Vec$LT$codexmate_lib..core..relay..storage..ProviderIdMigration$GT$$GT$17h53f606ce2e68ad36E>
100208c70: 9101c3e0    	add	x0, sp, #0x70
100208c74: 94199a65    	bl	0x10086f608 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5856a37aee37aca3E>
100208c78: 14000004    	b	0x100208c88 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE+0x758>
100208c7c: aa0003f3    	mov	x19, x0
100208c80: 910303e0    	add	x0, sp, #0xc0
100208c84: 94199a61    	bl	0x10086f608 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5856a37aee37aca3E>
100208c88: 910083e0    	add	x0, sp, #0x20
100208c8c: 941994b4    	bl	0x10086df5c <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3e2fcf4b3ccc9d35E>
100208c90: aa1303e0    	mov	x0, x19
100208c94: 943f2130    	bl	0x1011d1154 <_writev+0x1011d1154>
100208c98: 943f1fac    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
100208c9c: 943f1fab    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
