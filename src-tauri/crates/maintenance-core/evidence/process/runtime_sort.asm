
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008fc818 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E>:
1008fc818: d10383ff    	sub	sp, sp, #0xe0
1008fc81c: a9086ffc    	stp	x28, x27, [sp, #0x80]
1008fc820: a90967fa    	stp	x26, x25, [sp, #0x90]
1008fc824: a90a5ff8    	stp	x24, x23, [sp, #0xa0]
1008fc828: a90b57f6    	stp	x22, x21, [sp, #0xb0]
1008fc82c: a90c4ff4    	stp	x20, x19, [sp, #0xc0]
1008fc830: a90d7bfd    	stp	x29, x30, [sp, #0xd0]
1008fc834: 910343fd    	add	x29, sp, #0xd0
1008fc838: aa0103f6    	mov	x22, x1
1008fc83c: aa0003f3    	mov	x19, x0
1008fc840: a940e037    	ldp	x23, x24, [x1, #0x8]
1008fc844: a97f5434    	ldp	x20, x21, [x1, #-0x10]
1008fc848: aa1403e0    	mov	x0, x20
1008fc84c: aa1503e1    	mov	x1, x21
1008fc850: 97ffc5d2    	bl	0x1008edf98 <__ZN13codexmate_lib8platform5paths10CodexPaths16state_db_version17h4a5c0dcaa205a010E>
1008fc854: aa0003fa    	mov	x26, x0
1008fc858: aa0103f9    	mov	x25, x1
1008fc85c: aa1703e0    	mov	x0, x23
1008fc860: aa1803e1    	mov	x1, x24
1008fc864: 97ffc5cd    	bl	0x1008edf98 <__ZN13codexmate_lib8platform5paths10CodexPaths16state_db_version17h4a5c0dcaa205a010E>
1008fc868: 360000da    	tbz	w26, #0x0, 0x1008fc880 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x68>
1008fc86c: 360008e0    	tbz	w0, #0x0, 0x1008fc988 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x170>
1008fc870: 6b01033f    	cmp	w25, w1
1008fc874: 54000080    	b.eq	0x1008fc884 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x6c>
1008fc878: 54000223    	b.lo	0x1008fc8bc <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0xa4>
1008fc87c: 14000043    	b	0x1008fc988 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x170>
1008fc880: 370001e0    	tbnz	w0, #0x0, 0x1008fc8bc <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0xa4>
1008fc884: 910003e8    	mov	x8, sp
1008fc888: aa1703e0    	mov	x0, x23
1008fc88c: aa1803e1    	mov	x1, x24
1008fc890: 942148a2    	bl	0x10114eb18 <__ZN3std4path4Path10components17hb4e2325466cabf36E>
1008fc894: 910103e8    	add	x8, sp, #0x40
1008fc898: aa1403e0    	mov	x0, x20
1008fc89c: aa1503e1    	mov	x1, x21
1008fc8a0: 9421489e    	bl	0x10114eb18 <__ZN3std4path4Path10components17hb4e2325466cabf36E>
1008fc8a4: 910003e0    	mov	x0, sp
1008fc8a8: 910103e1    	add	x1, sp, #0x40
1008fc8ac: 94214774    	bl	0x10114e67c <__ZN3std4path18compare_components17h05aa5ba9b74df0c4E>
1008fc8b0: 52801fe8    	mov	w8, #0xff               ; =255
1008fc8b4: 6a20011f    	bics	wzr, w8, w0
1008fc8b8: 54000681    	b.ne	0x1008fc988 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x170>
1008fc8bc: d10062db    	sub	x27, x22, #0x18
1008fc8c0: a94056da    	ldp	x26, x21, [x22]
1008fc8c4: f9400ad4    	ldr	x20, [x22, #0x10]
1008fc8c8: 3dc00360    	ldr	q0, [x27]
1008fc8cc: 3d8002c0    	str	q0, [x22]
1008fc8d0: f9400b68    	ldr	x8, [x27, #0x10]
1008fc8d4: f9000ac8    	str	x8, [x22, #0x10]
1008fc8d8: eb13037f    	cmp	x27, x19
1008fc8dc: 54000500    	b.eq	0x1008fc97c <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x164>
1008fc8e0: 52801ffc    	mov	w28, #0xff              ; =255
1008fc8e4: 1400000a    	b	0x1008fc90c <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0xf4>
1008fc8e8: 540004a2    	b.hs	0x1008fc97c <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x164>
1008fc8ec: d1006368    	sub	x8, x27, #0x18
1008fc8f0: 3dc00100    	ldr	q0, [x8]
1008fc8f4: 3d800360    	str	q0, [x27]
1008fc8f8: f9400909    	ldr	x9, [x8, #0x10]
1008fc8fc: f9000b69    	str	x9, [x27, #0x10]
1008fc900: aa0803fb    	mov	x27, x8
1008fc904: eb13011f    	cmp	x8, x19
1008fc908: 540003c0    	b.eq	0x1008fc980 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x168>
1008fc90c: a97f5f76    	ldp	x22, x23, [x27, #-0x10]
1008fc910: aa1603e0    	mov	x0, x22
1008fc914: aa1703e1    	mov	x1, x23
1008fc918: 97ffc5a0    	bl	0x1008edf98 <__ZN13codexmate_lib8platform5paths10CodexPaths16state_db_version17h4a5c0dcaa205a010E>
1008fc91c: aa0003f9    	mov	x25, x0
1008fc920: aa0103f8    	mov	x24, x1
1008fc924: aa1503e0    	mov	x0, x21
1008fc928: aa1403e1    	mov	x1, x20
1008fc92c: 97ffc59b    	bl	0x1008edf98 <__ZN13codexmate_lib8platform5paths10CodexPaths16state_db_version17h4a5c0dcaa205a010E>
1008fc930: 360000b9    	tbz	w25, #0x0, 0x1008fc944 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x12c>
1008fc934: 36000240    	tbz	w0, #0x0, 0x1008fc97c <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x164>
1008fc938: 6b01031f    	cmp	w24, w1
1008fc93c: 54fffd61    	b.ne	0x1008fc8e8 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0xd0>
1008fc940: 14000002    	b	0x1008fc948 <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0x130>
1008fc944: 3707fd40    	tbnz	w0, #0x0, 0x1008fc8ec <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0xd4>
1008fc948: 910003e8    	mov	x8, sp
1008fc94c: aa1503e0    	mov	x0, x21
1008fc950: aa1403e1    	mov	x1, x20
1008fc954: 94214871    	bl	0x10114eb18 <__ZN3std4path4Path10components17hb4e2325466cabf36E>
1008fc958: 910103e8    	add	x8, sp, #0x40
1008fc95c: aa1603e0    	mov	x0, x22
1008fc960: aa1703e1    	mov	x1, x23
1008fc964: 9421486d    	bl	0x10114eb18 <__ZN3std4path4Path10components17hb4e2325466cabf36E>
1008fc968: 910003e0    	mov	x0, sp
1008fc96c: 910103e1    	add	x1, sp, #0x40
1008fc970: 94214743    	bl	0x10114e67c <__ZN3std4path18compare_components17h05aa5ba9b74df0c4E>
1008fc974: 6a20039f    	bics	wzr, w28, w0
1008fc978: 54fffba0    	b.eq	0x1008fc8ec <__ZN4core5slice4sort6shared9smallsort11insert_tail17hfae86d6da86b7907E+0xd4>
1008fc97c: aa1b03e8    	mov	x8, x27
1008fc980: a900551a    	stp	x26, x21, [x8]
1008fc984: f9000914    	str	x20, [x8, #0x10]
1008fc988: a94d7bfd    	ldp	x29, x30, [sp, #0xd0]
1008fc98c: a94c4ff4    	ldp	x20, x19, [sp, #0xc0]
1008fc990: a94b57f6    	ldp	x22, x21, [sp, #0xb0]
1008fc994: a94a5ff8    	ldp	x24, x23, [sp, #0xa0]
