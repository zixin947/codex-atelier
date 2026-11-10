
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008b01e8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E>:
1008b01e8: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
1008b01ec: a90167fa    	stp	x26, x25, [sp, #0x10]
1008b01f0: a9025ff8    	stp	x24, x23, [sp, #0x20]
1008b01f4: a90357f6    	stp	x22, x21, [sp, #0x30]
1008b01f8: a9044ff4    	stp	x20, x19, [sp, #0x40]
1008b01fc: a9057bfd    	stp	x29, x30, [sp, #0x50]
1008b0200: 910143fd    	add	x29, sp, #0x50
1008b0204: d10683ff    	sub	sp, sp, #0x1a0
1008b0208: aa0203f4    	mov	x20, x2
1008b020c: aa0103f5    	mov	x21, x1
1008b0210: aa0003f9    	mov	x25, x0
1008b0214: 9103e3e8    	add	x8, sp, #0xf8
1008b0218: aa0103e0    	mov	x0, x1
1008b021c: aa0203e1    	mov	x1, x2
1008b0220: 942323f0    	bl	0x1011791e0 <__ZN5alloc6string6String15from_utf8_lossy17h03e7be16830322ceE>
1008b0224: a95023e0    	ldp	x0, x8, [sp, #0x100]
1008b0228: f100251f    	cmp	x8, #0x9
1008b022c: 54000068    	b.hi	0x1008b0238 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x50>
1008b0230: f9407fe1    	ldr	x1, [sp, #0xf8]
1008b0234: 1400000d    	b	0x1008b0268 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x80>
1008b0238: 8b080008    	add	x8, x0, x8
1008b023c: f85f6109    	ldur	x9, [x8, #-0xa]
1008b0240: 785fe108    	ldurh	w8, [x8, #-0x2]
1008b0244: d28d45ca    	mov	x10, #0x6a2e            ; =27182
1008b0248: f2adee6a    	movk	x10, #0x6f73, lsl #16
1008b024c: f2cd8dca    	movk	x10, #0x6c6e, lsl #32
1008b0250: f2ef45ca    	movk	x10, #0x7a2e, lsl #48
1008b0254: eb0a013f    	cmp	x9, x10
1008b0258: 528e8e69    	mov	w9, #0x7473             ; =29811
1008b025c: 7a490100    	ccmp	w8, w9, #0x0, eq
1008b0260: f9407fe1    	ldr	x1, [sp, #0xf8]
1008b0264: 54000440    	b.eq	0x1008b02ec <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x104>
1008b0268: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008b026c: eb08003f    	cmp	x1, x8
1008b0270: 54000080    	b.eq	0x1008b0280 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x98>
1008b0274: b4000061    	cbz	x1, 0x1008b0280 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x98>
1008b0278: 52800022    	mov	w2, #0x1                ; =1
1008b027c: 97dd44bb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b0280: 910083e8    	add	x8, sp, #0x20
1008b0284: aa1503e0    	mov	x0, x21
1008b0288: aa1403e1    	mov	x1, x20
1008b028c: 94227a37    	bl	0x10114eb68 <__ZN3std4path4Path11to_path_buf17h98dc9231b300fab0E>
1008b0290: 97dd44b9    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008b0294: 52800300    	mov	w0, #0x18               ; =24
1008b0298: 52800101    	mov	w1, #0x8                ; =8
1008b029c: 97dd44b2    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008b02a0: b4000440    	cbz	x0, 0x1008b0328 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x140>
1008b02a4: aa0003f6    	mov	x22, x0
1008b02a8: f9401bf5    	ldr	x21, [sp, #0x30]
1008b02ac: b6f800d5    	tbz	x21, #0x3f, 0x1008b02c4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0xdc>
1008b02b0: d2800018    	mov	x24, #0x0               ; =0
1008b02b4: aa1803e0    	mov	x0, x24
1008b02b8: aa1503e1    	mov	x1, x21
1008b02bc: 942480ea    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008b02c0: 1400001d    	b	0x1008b0334 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x14c>
1008b02c4: f94017f4    	ldr	x20, [sp, #0x28]
1008b02c8: b4000395    	cbz	x21, 0x1008b0338 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x150>
1008b02cc: 97dd44aa    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008b02d0: 52800038    	mov	w24, #0x1               ; =1
1008b02d4: aa1503e0    	mov	x0, x21
1008b02d8: 52800021    	mov	w1, #0x1                ; =1
1008b02dc: 97dd44a2    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008b02e0: b4fffea0    	cbz	x0, 0x1008b02b4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0xcc>
1008b02e4: aa0003f7    	mov	x23, x0
1008b02e8: 14000015    	b	0x1008b033c <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x154>
1008b02ec: f240f83f    	tst	x1, #0x7fffffffffffffff
1008b02f0: 54000060    	b.eq	0x1008b02fc <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x114>
1008b02f4: 52800022    	mov	w2, #0x1                ; =1
1008b02f8: 97dd449c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b02fc: 910083e8    	add	x8, sp, #0x20
1008b0300: aa1503e0    	mov	x0, x21
1008b0304: aa1403e1    	mov	x1, x20
1008b0308: 52800022    	mov	w2, #0x1                ; =1
1008b030c: d2800003    	mov	x3, #0x0                ; =0
1008b0310: 94227b1f    	bl	0x10114ef8c <__ZN3std4path4Path15_with_extension17hb31df1dcfb2620b2E>
1008b0314: 97dd4498    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008b0318: 52800300    	mov	w0, #0x18               ; =24
1008b031c: 52800101    	mov	w1, #0x8                ; =8
1008b0320: 97dd4491    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008b0324: b5fffc00    	cbnz	x0, 0x1008b02a4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0xbc>
1008b0328: 52800100    	mov	w0, #0x8                ; =8
1008b032c: 52800301    	mov	w1, #0x18               ; =24
1008b0330: 942480c7    	bl	0x1011d064c <__ZN5alloc5alloc18handle_alloc_error17h06754e1196e3be78E>
1008b0334: d4200020    	brk	#0x1
1008b0338: 52800037    	mov	w23, #0x1               ; =1
1008b033c: aa1703e0    	mov	x0, x23
1008b0340: aa1403e1    	mov	x1, x20
1008b0344: aa1503e2    	mov	x2, x21
1008b0348: 942484d9    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008b034c: a9005ed5    	stp	x21, x23, [x22]
1008b0350: f9000ad5    	str	x21, [x22, #0x10]
1008b0354: 52800028    	mov	w8, #0x1                ; =1
1008b0358: a903dbe8    	stp	x8, x22, [sp, #0x38]
1008b035c: f90027e8    	str	x8, [sp, #0x48]
1008b0360: aa1403e0    	mov	x0, x20
1008b0364: aa1503e1    	mov	x1, x21
1008b0368: 94227d20    	bl	0x10114f7e8 <__ZN3std4path4Path9extension17hba82a71fe92867a6E>
1008b036c: 9103e3f6    	add	x22, sp, #0xf8
1008b0370: a90167f4    	stp	x20, x25, [sp, #0x10]
1008b0374: b4000160    	cbz	x0, 0x1008b03a0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x1b8>
1008b0378: f100143f    	cmp	x1, #0x5
1008b037c: 54000121    	b.ne	0x1008b03a0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x1b8>
1008b0380: b9400008    	ldr	w8, [x0]
1008b0384: 39401009    	ldrb	w9, [x0, #0x4]
1008b0388: 528e6d4a    	mov	w10, #0x736a            ; =29546
1008b038c: 72adcdea    	movk	w10, #0x6e6f, lsl #16
1008b0390: 6b0a011f    	cmp	w8, w10
1008b0394: 52800d88    	mov	w8, #0x6c               ; =108
1008b0398: 7a480120    	ccmp	w9, w8, #0x0, eq
1008b039c: 54000760    	b.eq	0x1008b0488 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x2a0>
1008b03a0: f94027e8    	ldr	x8, [sp, #0x48]
1008b03a4: a943d7ea    	ldp	x10, x21, [sp, #0x38]
1008b03a8: 8b080509    	add	x9, x8, x8, lsl #1
1008b03ac: d37df13b    	lsl	x27, x9, #3
1008b03b0: 8b1b02b4    	add	x20, x21, x27
1008b03b4: a90557f5    	stp	x21, x21, [sp, #0x50]
1008b03b8: f90007ea    	str	x10, [sp, #0x8]
1008b03bc: a90653ea    	stp	x10, x20, [sp, #0x60]
1008b03c0: aa1503e9    	mov	x9, x21
1008b03c4: b4000da8    	cbz	x8, 0x1008b0578 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x390>
1008b03c8: d280001c    	mov	x28, #0x0               ; =0
1008b03cc: d2f00013    	mov	x19, #-0x8000000000000000 ; =-9223372036854775808
1008b03d0: 14000004    	b	0x1008b03e0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x1f8>
1008b03d4: 9100639c    	add	x28, x28, #0x18
1008b03d8: eb1c037f    	cmp	x27, x28
1008b03dc: 54000ca0    	b.eq	0x1008b0570 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x388>
1008b03e0: 8b1c02a8    	add	x8, x21, x28
1008b03e4: f9400116    	ldr	x22, [x8]
1008b03e8: eb1302df    	cmp	x22, x19
1008b03ec: 540007c0    	b.eq	0x1008b04e4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x2fc>
1008b03f0: a940e117    	ldp	x23, x24, [x8, #0x8]
1008b03f4: 9103e3e8    	add	x8, sp, #0xf8
1008b03f8: aa1703e0    	mov	x0, x23
1008b03fc: aa1803e1    	mov	x1, x24
1008b0400: 9422607e    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
1008b0404: b940fbe8    	ldr	w8, [sp, #0xf8]
1008b0408: 36000728    	tbz	w8, #0x0, 0x1008b04ec <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x304>
1008b040c: f94083f8    	ldr	x24, [sp, #0x100]
1008b0410: aa1803e0    	mov	x0, x24
1008b0414: 940105ec    	bl	0x1008f1bc4 <__ZN3std2io5error5Error4kind17h247456c019dc3d6bE>
1008b0418: 72001c1f    	tst	w0, #0xff
1008b041c: 54000f21    	b.ne	0x1008b0600 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x418>
1008b0420: 92400708    	and	x8, x24, #0x3
1008b0424: f100051f    	cmp	x8, #0x1
1008b0428: 540000e0    	b.eq	0x1008b0444 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x25c>
1008b042c: b4fffd56    	cbz	x22, 0x1008b03d4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x1ec>
1008b0430: aa1703e0    	mov	x0, x23
1008b0434: aa1603e1    	mov	x1, x22
1008b0438: 52800022    	mov	w2, #0x1                ; =1
1008b043c: 97dd444b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b0440: 17ffffe5    	b	0x1008b03d4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x1ec>
1008b0444: f85fff19    	ldr	x25, [x24, #-0x1]!
1008b0448: f940071a    	ldr	x26, [x24, #0x8]
1008b044c: f9400348    	ldr	x8, [x26]
1008b0450: b4000068    	cbz	x8, 0x1008b045c <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x274>
1008b0454: aa1903e0    	mov	x0, x25
1008b0458: d63f0100    	blr	x8
1008b045c: f9400741    	ldr	x1, [x26, #0x8]
1008b0460: b4000081    	cbz	x1, 0x1008b0470 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x288>
1008b0464: f9400b42    	ldr	x2, [x26, #0x10]
1008b0468: aa1903e0    	mov	x0, x25
1008b046c: 97dd443f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b0470: aa1803e0    	mov	x0, x24
1008b0474: 52800301    	mov	w1, #0x18               ; =24
1008b0478: 52800102    	mov	w2, #0x8                ; =8
1008b047c: 97dd443b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b0480: b5fffd96    	cbnz	x22, 0x1008b0430 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x248>
1008b0484: 17ffffd4    	b	0x1008b03d4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x1ec>
1008b0488: b0005822    	adrp	x2, 0x1013b5000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.665+0x10b>
1008b048c: 91090c42    	add	x2, x2, #0x243
1008b0490: 9103e3e8    	add	x8, sp, #0xf8
1008b0494: aa1403e0    	mov	x0, x20
1008b0498: aa1503e1    	mov	x1, x21
1008b049c: 52800123    	mov	w3, #0x9                ; =9
1008b04a0: 94227abb    	bl	0x10114ef8c <__ZN3std4path4Path15_with_extension17hb31df1dcfb2620b2E>
1008b04a4: f94027f3    	ldr	x19, [sp, #0x48]
1008b04a8: f9401fe8    	ldr	x8, [sp, #0x38]
1008b04ac: eb08027f    	cmp	x19, x8
1008b04b0: 54000061    	b.ne	0x1008b04bc <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x2d4>
1008b04b4: 9100e3e0    	add	x0, sp, #0x38
1008b04b8: 942436f2    	bl	0x1011be080 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hc102b861917c98eeE>
1008b04bc: f94023e8    	ldr	x8, [sp, #0x40]
1008b04c0: 52800309    	mov	w9, #0x18               ; =24
1008b04c4: 9b092268    	madd	x8, x19, x9, x8
1008b04c8: 3dc002c0    	ldr	q0, [x22]
1008b04cc: 3d800100    	str	q0, [x8]
1008b04d0: f94087e9    	ldr	x9, [sp, #0x108]
1008b04d4: f9000909    	str	x9, [x8, #0x10]
1008b04d8: 91000668    	add	x8, x19, #0x1
1008b04dc: f90027e8    	str	x8, [sp, #0x48]
1008b04e0: 17ffffb1    	b	0x1008b03a4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x1bc>
1008b04e4: 91006109    	add	x9, x8, #0x18
1008b04e8: 14000023    	b	0x1008b0574 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x38c>
1008b04ec: 8b1c02a8    	add	x8, x21, x28
1008b04f0: 91006108    	add	x8, x8, #0x18
1008b04f4: f9002fe8    	str	x8, [sp, #0x58]
1008b04f8: 9103e3ea    	add	x10, sp, #0xf8
1008b04fc: ad438540    	ldp	q0, q1, [x10, #0x70]
1008b0500: ad0687e0    	stp	q0, q1, [sp, #0xd0]
1008b0504: f94083e8    	ldr	x8, [sp, #0x100]
1008b0508: f940c7e9    	ldr	x9, [sp, #0x188]
1008b050c: f9007be9    	str	x9, [sp, #0xf0]
1008b0510: ad418540    	ldp	q0, q1, [x10, #0x30]
1008b0514: ad0487e0    	stp	q0, q1, [sp, #0x90]
1008b0518: ad428141    	ldp	q1, q0, [x10, #0x50]
1008b051c: ad0583e1    	stp	q1, q0, [sp, #0xb0]
1008b0520: ad408141    	ldp	q1, q0, [x10, #0x10]
1008b0524: 92540d09    	and	x9, x8, #0xf00000000000
1008b0528: d2d0000a    	mov	x10, #0x800000000000    ; =140737488355328
1008b052c: ad0383e1    	stp	q1, q0, [sp, #0x70]
1008b0530: eb0a013f    	cmp	x9, x10
1008b0534: 540005a1    	b.ne	0x1008b05e8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x400>
1008b0538: ad4687e0    	ldp	q0, q1, [sp, #0xd0]
1008b053c: f9400fea    	ldr	x10, [sp, #0x18]
1008b0540: ad040540    	stp	q0, q1, [x10, #0x80]
1008b0544: f9407be9    	ldr	x9, [sp, #0xf0]
1008b0548: f9005149    	str	x9, [x10, #0xa0]
1008b054c: ad4487e0    	ldp	q0, q1, [sp, #0x90]
1008b0550: ad020540    	stp	q0, q1, [x10, #0x40]
1008b0554: ad4583e1    	ldp	q1, q0, [sp, #0xb0]
1008b0558: ad030141    	stp	q1, q0, [x10, #0x60]
1008b055c: ad4383e1    	ldp	q1, q0, [sp, #0x70]
1008b0560: ad010141    	stp	q1, q0, [x10, #0x20]
1008b0564: a9005d56    	stp	x22, x23, [x10]
1008b0568: a9012158    	stp	x24, x8, [x10, #0x10]
1008b056c: 1400002d    	b	0x1008b0620 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x438>
1008b0570: aa1403e9    	mov	x9, x20
1008b0574: f9002fe9    	str	x9, [sp, #0x58]
1008b0578: eb090288    	subs	x8, x20, x9
1008b057c: f9400ff6    	ldr	x22, [sp, #0x18]
1008b0580: 54000200    	b.eq	0x1008b05c0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x3d8>
1008b0584: b201f3ea    	mov	x10, #-0x5555555555555556 ; =-6148914691236517206
1008b0588: f295556a    	movk	x10, #0xaaab
1008b058c: 9bca7d08    	umulh	x8, x8, x10
1008b0590: d344fd13    	lsr	x19, x8, #4
1008b0594: 91002134    	add	x20, x9, #0x8
1008b0598: 14000004    	b	0x1008b05a8 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x3c0>
1008b059c: 91006294    	add	x20, x20, #0x18
1008b05a0: f1000673    	subs	x19, x19, #0x1
1008b05a4: 540000e0    	b.eq	0x1008b05c0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x3d8>
1008b05a8: f85f8281    	ldur	x1, [x20, #-0x8]
1008b05ac: b4ffff81    	cbz	x1, 0x1008b059c <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x3b4>
1008b05b0: f9400280    	ldr	x0, [x20]
1008b05b4: 52800022    	mov	w2, #0x1                ; =1
1008b05b8: 97dd43ec    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b05bc: 17fffff8    	b	0x1008b059c <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x3b4>
1008b05c0: f94007e8    	ldr	x8, [sp, #0x8]
1008b05c4: b40000c8    	cbz	x8, 0x1008b05dc <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x3f4>
1008b05c8: 8b080508    	add	x8, x8, x8, lsl #1
1008b05cc: d37df101    	lsl	x1, x8, #3
1008b05d0: aa1503e0    	mov	x0, x21
1008b05d4: 52800102    	mov	w2, #0x8                ; =8
1008b05d8: 97dd43e4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b05dc: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008b05e0: f90002c8    	str	x8, [x22]
1008b05e4: 1400002b    	b	0x1008b0690 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x4a8>
1008b05e8: b0005821    	adrp	x1, 0x1013b5000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.665+0x10b>
1008b05ec: 91093021    	add	x1, x1, #0x24c
1008b05f0: 52800280    	mov	w0, #0x14               ; =20
1008b05f4: 528002a2    	mov	w2, #0x15               ; =21
1008b05f8: 94224d30    	bl	0x101143ab8 <__ZN3std2io5error5Error3new17h311cf880a0f7d574E>
1008b05fc: aa0003f8    	mov	x24, x0
1008b0600: b24107e8    	mov	x8, #-0x7fffffffffffffff ; =-9223372036854775807
1008b0604: f9400fe9    	ldr	x9, [sp, #0x18]
1008b0608: a9006128    	stp	x8, x24, [x9]
1008b060c: b40000b6    	cbz	x22, 0x1008b0620 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x438>
1008b0610: aa1703e0    	mov	x0, x23
1008b0614: aa1603e1    	mov	x1, x22
1008b0618: 52800022    	mov	w2, #0x1                ; =1
1008b061c: 97dd43d3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b0620: d1006368    	sub	x8, x27, #0x18
1008b0624: eb1c011f    	cmp	x8, x28
1008b0628: 54000260    	b.eq	0x1008b0674 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x48c>
1008b062c: 8b1c02a8    	add	x8, x21, x28
1008b0630: 91008113    	add	x19, x8, #0x20
1008b0634: cb1c0368    	sub	x8, x27, x28
1008b0638: d1006108    	sub	x8, x8, #0x18
1008b063c: b201f3e9    	mov	x9, #-0x5555555555555556 ; =-6148914691236517206
1008b0640: f2955569    	movk	x9, #0xaaab
1008b0644: 9bc97d08    	umulh	x8, x8, x9
1008b0648: d344fd14    	lsr	x20, x8, #4
1008b064c: 14000004    	b	0x1008b065c <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x474>
1008b0650: 91006273    	add	x19, x19, #0x18
1008b0654: f1000694    	subs	x20, x20, #0x1
1008b0658: 540000e0    	b.eq	0x1008b0674 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x48c>
1008b065c: f85f8261    	ldur	x1, [x19, #-0x8]
1008b0660: b4ffff81    	cbz	x1, 0x1008b0650 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x468>
1008b0664: f9400260    	ldr	x0, [x19]
1008b0668: 52800022    	mov	w2, #0x1                ; =1
1008b066c: 97dd43bf    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b0670: 17fffff8    	b	0x1008b0650 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x468>
1008b0674: f94007e8    	ldr	x8, [sp, #0x8]
1008b0678: b40000c8    	cbz	x8, 0x1008b0690 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x4a8>
1008b067c: 8b080508    	add	x8, x8, x8, lsl #1
1008b0680: d37df101    	lsl	x1, x8, #3
1008b0684: aa1503e0    	mov	x0, x21
1008b0688: 52800102    	mov	w2, #0x8                ; =8
1008b068c: 97dd43b7    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b0690: f9400be0    	ldr	x0, [sp, #0x10]
1008b0694: f94013e1    	ldr	x1, [sp, #0x20]
1008b0698: b4000061    	cbz	x1, 0x1008b06a4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility15rollout_history7resolve17h5085bee2fd8a9da6E+0x4bc>
1008b069c: 52800022    	mov	w2, #0x1                ; =1
1008b06a0: 97dd43b2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008b06a4: 910683ff    	add	sp, sp, #0x1a0
1008b06a8: a9457bfd    	ldp	x29, x30, [sp, #0x50]
1008b06ac: a9444ff4    	ldp	x20, x19, [sp, #0x40]
1008b06b0: a94357f6    	ldp	x22, x21, [sp, #0x30]
1008b06b4: a9425ff8    	ldp	x24, x23, [sp, #0x20]
1008b06b8: a94167fa    	ldp	x26, x25, [sp, #0x10]
1008b06bc: a8c66ffc    	ldp	x28, x27, [sp], #0x60
1008b06c0: d65f03c0    	ret
