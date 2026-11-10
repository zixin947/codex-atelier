
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008a5300 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E>:
1008a5300: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
1008a5304: a90167fa    	stp	x26, x25, [sp, #0x10]
1008a5308: a9025ff8    	stp	x24, x23, [sp, #0x20]
1008a530c: a90357f6    	stp	x22, x21, [sp, #0x30]
1008a5310: a9044ff4    	stp	x20, x19, [sp, #0x40]
1008a5314: a9057bfd    	stp	x29, x30, [sp, #0x50]
1008a5318: 910143fd    	add	x29, sp, #0x50
1008a531c: d10843ff    	sub	sp, sp, #0x210
1008a5320: aa0303f6    	mov	x22, x3
1008a5324: aa0203f7    	mov	x23, x2
1008a5328: aa0103f4    	mov	x20, x1
1008a532c: aa0003f5    	mov	x21, x0
1008a5330: 9422a89f    	bl	0x10114f5ac <__ZN3std4path4Path6parent17h2f7f79d216a1f739E>
1008a5334: b40001e0    	cbz	x0, 0x1008a5370 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x70>
1008a5338: aa0003f3    	mov	x19, x0
1008a533c: aa0103fa    	mov	x26, x1
1008a5340: f90003f6    	str	x22, [sp]
1008a5344: 52803fe8    	mov	w8, #0x1ff              ; =511
1008a5348: 781083a8    	sturh	w8, [x29, #-0xf8]
1008a534c: 52800038    	mov	w24, #0x1               ; =1
1008a5350: 3810a3b8    	sturb	w24, [x29, #-0xf6]
1008a5354: d103e3a0    	sub	x0, x29, #0xf8
1008a5358: aa1303e1    	mov	x1, x19
1008a535c: aa1a03e2    	mov	x2, x26
1008a5360: 94227126    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008a5364: b4000160    	cbz	x0, 0x1008a5390 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x90>
1008a5368: aa0003f9    	mov	x25, x0
1008a536c: 14000087    	b	0x1008a5588 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x288>
1008a5370: f0005861    	adrp	x1, 0x1013b4000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.331+0x4>
1008a5374: 91208421    	add	x1, x1, #0x821
1008a5378: 52800280    	mov	w0, #0x14               ; =20
1008a537c: 52800382    	mov	w2, #0x1c               ; =28
1008a5380: 942279ce    	bl	0x101143ab8 <__ZN3std2io5error5Error3new17h311cf880a0f7d574E>
1008a5384: aa0003f9    	mov	x25, x0
1008a5388: 52800038    	mov	w24, #0x1               ; =1
1008a538c: 1400007f    	b	0x1008a5588 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x288>
1008a5390: 910023e8    	add	x8, sp, #0x8
1008a5394: aa1503e0    	mov	x0, x21
1008a5398: aa1403e1    	mov	x1, x20
1008a539c: 97ffff45    	bl	0x1008a50b0 <__ZN13codexmate_lib4core5relay12atomic_write14temp_file_stem17hf7fae2a0e5e52bdeE>
1008a53a0: 9422b204    	bl	0x101151bb0 <__ZN3std7process2id17h3613cdd14e59bdb4E>
1008a53a4: b90093e0    	str	w0, [sp, #0x90]
1008a53a8: d103e3a8    	sub	x8, x29, #0xf8
1008a53ac: 941f0288    	bl	0x101065dcc <__ZN4uuid2v428_$LT$impl$u20$uuid..Uuid$GT$6new_v417h6ad5de516c3d59ecE>
1008a53b0: 910243f6    	add	x22, sp, #0x90
1008a53b4: 3cd083a0    	ldur	q0, [x29, #-0xf8]
1008a53b8: 3d8012c0    	str	q0, [x22, #0x40]
1008a53bc: 910023e8    	add	x8, sp, #0x8
1008a53c0: 90000369    	adrp	x9, 0x100911000 <__ZN5rayon4iter8plumbing6Folder12consume_iter17h4212d054bfcf564eE+0xc0>
1008a53c4: 910ec129    	add	x9, x9, #0x3b0
1008a53c8: a930a7a8    	stp	x8, x9, [x29, #-0xf8]
1008a53cc: 910243e8    	add	x8, sp, #0x90
1008a53d0: d00046a9    	adrp	x9, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
1008a53d4: 910bb129    	add	x9, x9, #0x2ec
1008a53d8: a931a7a8    	stp	x8, x9, [x29, #-0xe8]
1008a53dc: 910343e8    	add	x8, sp, #0xd0
1008a53e0: d0000349    	adrp	x9, 0x10090f000 <__ZN4core5slice4sort6shared9smallsort31small_sort_general_with_scratch17hfa0722afa63362d1E+0x5f4>
1008a53e4: 913a0129    	add	x9, x9, #0xe80
1008a53e8: a932a7a8    	stp	x8, x9, [x29, #-0xd8]
1008a53ec: d0006440    	adrp	x0, 0x10152f000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.17+0x520>
1008a53f0: 91183000    	add	x0, x0, #0x60c
1008a53f4: 9100e3e8    	add	x8, sp, #0x38
1008a53f8: d103e3a1    	sub	x1, x29, #0xf8
1008a53fc: 94234926    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1008a5400: a9440ff8    	ldp	x24, x3, [sp, #0x40]
1008a5404: 910083e8    	add	x8, sp, #0x20
1008a5408: aa1303e0    	mov	x0, x19
1008a540c: aa1a03e1    	mov	x1, x26
1008a5410: aa1803e2    	mov	x2, x24
1008a5414: 9422a793    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
1008a5418: f9401fe1    	ldr	x1, [sp, #0x38]
1008a541c: b4000081    	cbz	x1, 0x1008a542c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x12c>
1008a5420: aa1803e0    	mov	x0, x24
1008a5424: 52800022    	mov	w2, #0x1                ; =1
1008a5428: 97dd7050    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a542c: a942e3f3    	ldp	x19, x24, [sp, #0x28]
1008a5430: 910143e8    	add	x8, sp, #0x50
1008a5434: aa1503e0    	mov	x0, x21
1008a5438: aa1403e1    	mov	x1, x20
1008a543c: 9422a5b7    	bl	0x10114eb18 <__ZN3std4path4Path10components17hb4e2325466cabf36E>
1008a5440: 910243e8    	add	x8, sp, #0x90
1008a5444: aa1303e0    	mov	x0, x19
1008a5448: aa1803e1    	mov	x1, x24
1008a544c: 9422a5b3    	bl	0x10114eb18 <__ZN3std4path4Path10components17hb4e2325466cabf36E>
1008a5450: a9456bfb    	ldp	x27, x26, [sp, #0x50]
1008a5454: a94973f9    	ldp	x25, x28, [sp, #0x90]
1008a5458: eb1c035f    	cmp	x26, x28
1008a545c: 54000201    	b.ne	0x1008a549c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x19c>
1008a5460: 394223e8    	ldrb	w8, [sp, #0x88]
1008a5464: 394323e9    	ldrb	w9, [sp, #0xc8]
1008a5468: 6b09011f    	cmp	w8, w9
1008a546c: 54000181    	b.ne	0x1008a549c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x19c>
1008a5470: 394227e8    	ldrb	w8, [sp, #0x89]
1008a5474: 7100091f    	cmp	w8, #0x2
1008a5478: 54000121    	b.ne	0x1008a549c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x19c>
1008a547c: 394327e8    	ldrb	w8, [sp, #0xc9]
1008a5480: 7100091f    	cmp	w8, #0x2
1008a5484: 540000c1    	b.ne	0x1008a549c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x19c>
1008a5488: aa1b03e0    	mov	x0, x27
1008a548c: aa1903e1    	mov	x1, x25
1008a5490: aa1a03e2    	mov	x2, x26
1008a5494: 9424b083    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1008a5498: 34000560    	cbz	w0, 0x1008a5544 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x244>
1008a549c: 394183e8    	ldrb	w8, [sp, #0x60]
1008a54a0: 7100191f    	cmp	w8, #0x6
1008a54a4: 540000c0    	b.eq	0x1008a54bc <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x1bc>
1008a54a8: 3cc613e0    	ldur	q0, [sp, #0x61]
1008a54ac: 3cc713e1    	ldur	q1, [sp, #0x71]
1008a54b0: ad0406c0    	stp	q0, q1, [x22, #0x80]
1008a54b4: f94043e9    	ldr	x9, [sp, #0x80]
1008a54b8: f809f2c9    	stur	x9, [x22, #0x9f]
1008a54bc: ad4406c0    	ldp	q0, q1, [x22, #0x80]
1008a54c0: 3c8512c0    	stur	q0, [x22, #0x51]
1008a54c4: 39422be9    	ldrb	w9, [sp, #0x8a]
1008a54c8: 794113ea    	ldrh	w10, [sp, #0x88]
1008a54cc: a90d6bfb    	stp	x27, x26, [sp, #0xd0]
1008a54d0: 390383e8    	strb	w8, [sp, #0xe0]
1008a54d4: 3c8612c1    	stur	q1, [x22, #0x61]
1008a54d8: f849f2c8    	ldur	x8, [x22, #0x9f]
1008a54dc: f90083e8    	str	x8, [sp, #0x100]
1008a54e0: 790213ea    	strh	w10, [sp, #0x108]
1008a54e4: 39042be9    	strb	w9, [sp, #0x10a]
1008a54e8: 394283e8    	ldrb	w8, [sp, #0xa0]
1008a54ec: 7100191f    	cmp	w8, #0x6
1008a54f0: 540000c0    	b.eq	0x1008a5508 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x208>
1008a54f4: 3cc112c0    	ldur	q0, [x22, #0x11]
1008a54f8: 3cc212c1    	ldur	q1, [x22, #0x21]
1008a54fc: ad0586c0    	stp	q0, q1, [x22, #0xb0]
1008a5500: f94063e9    	ldr	x9, [sp, #0xc0]
1008a5504: f80cf2c9    	stur	x9, [x22, #0xcf]
1008a5508: 39432be9    	ldrb	w9, [sp, #0xca]
1008a550c: 794193ea    	ldrh	w10, [sp, #0xc8]
1008a5510: a930f3b9    	stp	x25, x28, [x29, #-0xf8]
1008a5514: 381183a8    	sturb	w8, [x29, #-0xe8]
1008a5518: ad4586c0    	ldp	q0, q1, [x22, #0xb0]
1008a551c: 3c8e92c0    	stur	q0, [x22, #0xe9]
1008a5520: 3c8f92c1    	stur	q1, [x22, #0xf9]
1008a5524: f84cf2c8    	ldur	x8, [x22, #0xcf]
1008a5528: f81383a8    	stur	x8, [x29, #-0xc8]
1008a552c: 781403aa    	sturh	w10, [x29, #-0xc0]
1008a5530: 381423a9    	sturb	w9, [x29, #-0xbe]
1008a5534: 910343e0    	add	x0, sp, #0xd0
1008a5538: d103e3a1    	sub	x1, x29, #0xf8
1008a553c: 97fd43ba    	bl	0x1007f6424 <__ZN4core4iter6traits8iterator8Iterator5eq_by17hd65221847cff6de6E>
1008a5540: 36000380    	tbz	w0, #0x0, 0x1008a55b0 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x2b0>
1008a5544: f0005861    	adrp	x1, 0x1013b4000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.331+0x4>
1008a5548: 911ed021    	add	x1, x1, #0x7b4
1008a554c: 52800280    	mov	w0, #0x14               ; =20
1008a5550: 52800442    	mov	w2, #0x22               ; =34
1008a5554: 94227959    	bl	0x101143ab8 <__ZN3std2io5error5Error3new17h311cf880a0f7d574E>
1008a5558: aa0003f9    	mov	x25, x0
1008a555c: 52800038    	mov	w24, #0x1               ; =1
1008a5560: f94013e1    	ldr	x1, [sp, #0x20]
1008a5564: b4000081    	cbz	x1, 0x1008a5574 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x274>
1008a5568: aa1303e0    	mov	x0, x19
1008a556c: 52800022    	mov	w2, #0x1                ; =1
1008a5570: 97dd6ffe    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a5574: f94007e1    	ldr	x1, [sp, #0x8]
1008a5578: b4000081    	cbz	x1, 0x1008a5588 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x288>
1008a557c: f9400be0    	ldr	x0, [sp, #0x10]
1008a5580: 52800022    	mov	w2, #0x1                ; =1
1008a5584: 97dd6ff9    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a5588: aa1803e0    	mov	x0, x24
1008a558c: aa1903e1    	mov	x1, x25
1008a5590: 910843ff    	add	sp, sp, #0x210
1008a5594: a9457bfd    	ldp	x29, x30, [sp, #0x50]
1008a5598: a9444ff4    	ldp	x20, x19, [sp, #0x40]
1008a559c: a94357f6    	ldp	x22, x21, [sp, #0x30]
1008a55a0: a9425ff8    	ldp	x24, x23, [sp, #0x20]
1008a55a4: a94167fa    	ldp	x26, x25, [sp, #0x10]
1008a55a8: a8c66ffc    	ldp	x28, x27, [sp], #0x60
1008a55ac: d65f03c0    	ret
1008a55b0: aa1503e0    	mov	x0, x21
1008a55b4: aa1403e1    	mov	x1, x20
1008a55b8: 9422a7fd    	bl	0x10114f5ac <__ZN3std4path4Path6parent17h2f7f79d216a1f739E>
1008a55bc: aa0003fb    	mov	x27, x0
1008a55c0: aa0103fc    	mov	x28, x1
1008a55c4: aa1303e0    	mov	x0, x19
1008a55c8: aa1803e1    	mov	x1, x24
1008a55cc: 9422a7f8    	bl	0x10114f5ac <__ZN3std4path4Path6parent17h2f7f79d216a1f739E>
1008a55d0: aa0003f9    	mov	x25, x0
1008a55d4: b400093b    	cbz	x27, 0x1008a56f8 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x3f8>
1008a55d8: b4fffb79    	cbz	x25, 0x1008a5544 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x244>
1008a55dc: aa0103fa    	mov	x26, x1
1008a55e0: 910143e8    	add	x8, sp, #0x50
1008a55e4: aa1b03e0    	mov	x0, x27
1008a55e8: aa1c03e1    	mov	x1, x28
1008a55ec: 9422a54b    	bl	0x10114eb18 <__ZN3std4path4Path10components17hb4e2325466cabf36E>
1008a55f0: 910243e8    	add	x8, sp, #0x90
1008a55f4: aa1903e0    	mov	x0, x25
1008a55f8: aa1a03e1    	mov	x1, x26
1008a55fc: 9422a547    	bl	0x10114eb18 <__ZN3std4path4Path10components17hb4e2325466cabf36E>
1008a5600: a9456bfb    	ldp	x27, x26, [sp, #0x50]
1008a5604: a94973f9    	ldp	x25, x28, [sp, #0x90]
1008a5608: eb1c035f    	cmp	x26, x28
1008a560c: 54000201    	b.ne	0x1008a564c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x34c>
1008a5610: 394223e8    	ldrb	w8, [sp, #0x88]
1008a5614: 394323e9    	ldrb	w9, [sp, #0xc8]
1008a5618: 6b09011f    	cmp	w8, w9
1008a561c: 54000181    	b.ne	0x1008a564c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x34c>
1008a5620: 394227e8    	ldrb	w8, [sp, #0x89]
1008a5624: 7100091f    	cmp	w8, #0x2
1008a5628: 54000121    	b.ne	0x1008a564c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x34c>
1008a562c: 394327e8    	ldrb	w8, [sp, #0xc9]
1008a5630: 7100091f    	cmp	w8, #0x2
1008a5634: 540000c1    	b.ne	0x1008a564c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x34c>
1008a5638: aa1b03e0    	mov	x0, x27
1008a563c: aa1903e1    	mov	x1, x25
1008a5640: aa1a03e2    	mov	x2, x26
1008a5644: 9424b017    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1008a5648: 340005a0    	cbz	w0, 0x1008a56fc <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x3fc>
1008a564c: 394183e8    	ldrb	w8, [sp, #0x60]
1008a5650: 7100191f    	cmp	w8, #0x6
1008a5654: 540000c0    	b.eq	0x1008a566c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x36c>
1008a5658: 3cc613e0    	ldur	q0, [sp, #0x61]
1008a565c: 3cc713e1    	ldur	q1, [sp, #0x71]
1008a5660: ad0406c0    	stp	q0, q1, [x22, #0x80]
1008a5664: f94043e9    	ldr	x9, [sp, #0x80]
1008a5668: f809f2c9    	stur	x9, [x22, #0x9f]
1008a566c: ad4406c0    	ldp	q0, q1, [x22, #0x80]
1008a5670: 3c8512c0    	stur	q0, [x22, #0x51]
1008a5674: 39422be9    	ldrb	w9, [sp, #0x8a]
1008a5678: 794113ea    	ldrh	w10, [sp, #0x88]
1008a567c: a90d6bfb    	stp	x27, x26, [sp, #0xd0]
1008a5680: 390383e8    	strb	w8, [sp, #0xe0]
1008a5684: 3c8612c1    	stur	q1, [x22, #0x61]
1008a5688: f849f2c8    	ldur	x8, [x22, #0x9f]
1008a568c: f90083e8    	str	x8, [sp, #0x100]
1008a5690: 790213ea    	strh	w10, [sp, #0x108]
1008a5694: 39042be9    	strb	w9, [sp, #0x10a]
1008a5698: 394283e8    	ldrb	w8, [sp, #0xa0]
1008a569c: 7100191f    	cmp	w8, #0x6
1008a56a0: 540000c0    	b.eq	0x1008a56b8 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x3b8>
1008a56a4: 3cc112c0    	ldur	q0, [x22, #0x11]
1008a56a8: 3cc212c1    	ldur	q1, [x22, #0x21]
1008a56ac: ad0586c0    	stp	q0, q1, [x22, #0xb0]
1008a56b0: f94063e9    	ldr	x9, [sp, #0xc0]
1008a56b4: f80cf2c9    	stur	x9, [x22, #0xcf]
1008a56b8: 39432be9    	ldrb	w9, [sp, #0xca]
1008a56bc: 794193ea    	ldrh	w10, [sp, #0xc8]
1008a56c0: a930f3b9    	stp	x25, x28, [x29, #-0xf8]
1008a56c4: 381183a8    	sturb	w8, [x29, #-0xe8]
1008a56c8: ad4586c0    	ldp	q0, q1, [x22, #0xb0]
1008a56cc: 3c8e92c0    	stur	q0, [x22, #0xe9]
1008a56d0: 3c8f92c1    	stur	q1, [x22, #0xf9]
1008a56d4: f84cf2c8    	ldur	x8, [x22, #0xcf]
1008a56d8: f81383a8    	stur	x8, [x29, #-0xc8]
1008a56dc: 781403aa    	sturh	w10, [x29, #-0xc0]
1008a56e0: 381423a9    	sturb	w9, [x29, #-0xbe]
1008a56e4: 910343e0    	add	x0, sp, #0xd0
1008a56e8: d103e3a1    	sub	x1, x29, #0xf8
1008a56ec: 97fd434e    	bl	0x1007f6424 <__ZN4core4iter6traits8iterator8Iterator5eq_by17hd65221847cff6de6E>
1008a56f0: 34fff2a0    	cbz	w0, 0x1008a5544 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x244>
1008a56f4: 14000002    	b	0x1008a56fc <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x3fc>
1008a56f8: b5fff279    	cbnz	x25, 0x1008a5544 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x244>
1008a56fc: 910343e8    	add	x8, sp, #0xd0
1008a5700: aa1303e0    	mov	x0, x19
1008a5704: aa1803e1    	mov	x1, x24
1008a5708: 9422a518    	bl	0x10114eb68 <__ZN3std4path4Path11to_path_buf17h98dc9231b300fab0E>
1008a570c: 52800048    	mov	w8, #0x2                ; =2
1008a5710: b900ebe8    	str	w8, [sp, #0xe8]
1008a5714: b90053ff    	str	wzr, [sp, #0x50]
1008a5718: 528036c8    	mov	w8, #0x1b6              ; =438
1008a571c: 7900abe8    	strh	w8, [sp, #0x54]
1008a5720: b80563ff    	stur	wzr, [sp, #0x56]
1008a5724: 7900b7ff    	strh	wzr, [sp, #0x5a]
1008a5728: 52800028    	mov	w8, #0x1                ; =1
1008a572c: 39015fe8    	strb	w8, [sp, #0x57]
1008a5730: 39016fe8    	strb	w8, [sp, #0x5b]
1008a5734: d103e3a8    	sub	x8, x29, #0xf8
1008a5738: aa1503e0    	mov	x0, x21
1008a573c: aa1403e1    	mov	x1, x20
1008a5740: 94228bae    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
1008a5744: b85083a8    	ldur	w8, [x29, #-0xf8]
1008a5748: f85103a1    	ldur	x1, [x29, #-0xf0]
1008a574c: 360000a8    	tbz	w8, #0x0, 0x1008a5760 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x460>
1008a5750: 52800020    	mov	w0, #0x1                ; =1
1008a5754: 940152d9    	bl	0x1008fa2b8 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
1008a5758: 528036c8    	mov	w8, #0x1b6              ; =438
1008a575c: 14000002    	b	0x1008a5764 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x464>
1008a5760: d360a028    	ubfx	x8, x1, #32, #9
1008a5764: 7900abe8    	strh	w8, [sp, #0x54]
1008a5768: d103e3a8    	sub	x8, x29, #0xf8
1008a576c: 910143e0    	add	x0, sp, #0x50
1008a5770: aa1303e1    	mov	x1, x19
1008a5774: aa1803e2    	mov	x2, x24
1008a5778: 9422721a    	bl	0x101141fe0 <__ZN3std2fs11OpenOptions5_open17hfd14fc0134faefc7E>
1008a577c: b85083a8    	ldur	w8, [x29, #-0xf8]
1008a5780: 7100051f    	cmp	w8, #0x1
1008a5784: 54000061    	b.ne	0x1008a5790 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x490>
1008a5788: f85103b9    	ldur	x25, [x29, #-0xf0]
1008a578c: 14000020    	b	0x1008a580c <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x50c>
1008a5790: b850c3ba    	ldur	w26, [x29, #-0xf4]
1008a5794: b90143fa    	str	w26, [sp, #0x140]
1008a5798: d103e3a8    	sub	x8, x29, #0xf8
1008a579c: 910503e0    	add	x0, sp, #0x140
1008a57a0: 97ea0112    	bl	0x100325be8 <__ZN13codexmate_lib4core5relay12atomic_write7staging12FileIdentity4read17hd0f3c53370637c95E>
1008a57a4: b85083a8    	ldur	w8, [x29, #-0xf8]
1008a57a8: 7100091f    	cmp	w8, #0x2
1008a57ac: 54000061    	b.ne	0x1008a57b8 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x4b8>
1008a57b0: f85103b9    	ldur	x25, [x29, #-0xf0]
1008a57b4: 14000014    	b	0x1008a5804 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x504>
1008a57b8: b850c3a9    	ldur	w9, [x29, #-0xf4]
1008a57bc: a9712faa    	ldp	x10, x11, [x29, #-0xf0]
1008a57c0: 291227e8    	stp	w8, w9, [sp, #0x90]
1008a57c4: a909afea    	stp	x10, x11, [sp, #0x98]
1008a57c8: 291d27e8    	stp	w8, w9, [sp, #0xe8]
1008a57cc: a90f2fea    	stp	x10, x11, [sp, #0xf0]
1008a57d0: 910503e0    	add	x0, sp, #0x140
1008a57d4: aa1703e1    	mov	x1, x23
1008a57d8: f94003e2    	ldr	x2, [sp]
1008a57dc: 97f4e1ee    	bl	0x1005ddf94 <__ZN3std2io5Write9write_all17h3d18456a7d80afebE>
1008a57e0: aa0003f9    	mov	x25, x0
1008a57e4: b4000060    	cbz	x0, 0x1008a57f0 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x4f0>
1008a57e8: b94143fa    	ldr	w26, [sp, #0x140]
1008a57ec: 14000006    	b	0x1008a5804 <__ZN13codexmate_lib4core5relay12atomic_write19write_atomic_report17hb11aeeab130ff059E+0x504>
1008a57f0: 910503e0    	add	x0, sp, #0x140
1008a57f4: 94227335    	bl	0x1011424c8 <__ZN3std2fs4File9sync_data17h3f6e029ce9c67cbfE>
1008a57f8: aa0003f9    	mov	x25, x0
1008a57fc: b94143fa    	ldr	w26, [sp, #0x140]
