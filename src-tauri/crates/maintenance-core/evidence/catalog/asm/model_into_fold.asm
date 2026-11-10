
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010050d55c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE>:
10050d55c: d10403ff    	sub	sp, sp, #0x100
10050d560: a90a6ffc    	stp	x28, x27, [sp, #0xa0]
10050d564: a90b67fa    	stp	x26, x25, [sp, #0xb0]
10050d568: a90c5ff8    	stp	x24, x23, [sp, #0xc0]
10050d56c: a90d57f6    	stp	x22, x21, [sp, #0xd0]
10050d570: a90e4ff4    	stp	x20, x19, [sp, #0xe0]
10050d574: a90f7bfd    	stp	x29, x30, [sp, #0xf0]
10050d578: 9103c3fd    	add	x29, sp, #0xf0
10050d57c: aa0003f3    	mov	x19, x0
10050d580: f9400c18    	ldr	x24, [x0, #0x18]
10050d584: f9400417    	ldr	x23, [x0, #0x8]
10050d588: eb1802ff    	cmp	x23, x24
10050d58c: 54000640    	b.eq	0x10050d654 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0xf8>
10050d590: aa0203f4    	mov	x20, x2
10050d594: 910083e8    	add	x8, sp, #0x20
10050d598: 91004119    	add	x25, x8, #0x10
10050d59c: f9400035    	ldr	x21, [x1]
10050d5a0: d000029a    	adrp	x26, 0x10055f000 <__ZN5alloc3vec16in_place_collect108_$LT$impl$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$u20$for$u20$alloc..vec..Vec$LT$T$GT$$GT$9from_iter17h47a329132187b06bE+0xc>
10050d5a4: 911bd35a    	add	x26, x26, #0x6f4
10050d5a8: f0008076    	adrp	x22, 0x10151c000 <_anon.b03982025400859c41411cbf9a3d7a99.1+0x4c6>
10050d5ac: 913f62d6    	add	x22, x22, #0xfd8
10050d5b0: d2f0001b    	mov	x27, #-0x8000000000000000 ; =-9223372036854775808
10050d5b4: ad4006e0    	ldp	q0, q1, [x23]
10050d5b8: ad0107e0    	stp	q0, q1, [sp, #0x20]
10050d5bc: ad4106e0    	ldp	q0, q1, [x23, #0x20]
10050d5c0: ad420ee2    	ldp	q2, q3, [x23, #0x40]
10050d5c4: ad030fe2    	stp	q2, q3, [sp, #0x60]
10050d5c8: ad0207e0    	stp	q0, q1, [sp, #0x40]
10050d5cc: 910182f7    	add	x23, x23, #0x60
10050d5d0: f9000677    	str	x23, [x19, #0x8]
10050d5d4: a9396bb4    	stp	x20, x26, [x29, #-0x70]
10050d5d8: a93a6bb9    	stp	x25, x26, [x29, #-0x60]
10050d5dc: 910023e8    	add	x8, sp, #0x8
10050d5e0: d101c3a1    	sub	x1, x29, #0x70
10050d5e4: aa1603e0    	mov	x0, x22
10050d5e8: 9431a8ab    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10050d5ec: f9401be1    	ldr	x1, [sp, #0x30]
10050d5f0: b4000081    	cbz	x1, 0x10050d600 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0xa4>
10050d5f4: f9401fe0    	ldr	x0, [sp, #0x38]
10050d5f8: 52800022    	mov	w2, #0x1                ; =1
10050d5fc: 97ebcfdb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050d600: f94033e1    	ldr	x1, [sp, #0x60]
10050d604: eb1b003f    	cmp	x1, x27
10050d608: 540000a0    	b.eq	0x10050d61c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0xc0>
10050d60c: b4000081    	cbz	x1, 0x10050d61c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0xc0>
10050d610: f94037e0    	ldr	x0, [sp, #0x68]
10050d614: 52800022    	mov	w2, #0x1                ; =1
10050d618: 97ebcfd4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050d61c: f94027e1    	ldr	x1, [sp, #0x48]
10050d620: b4000081    	cbz	x1, 0x10050d630 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0xd4>
10050d624: f9402be0    	ldr	x0, [sp, #0x50]
10050d628: 52800022    	mov	w2, #0x1                ; =1
10050d62c: 97ebcfcf    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050d630: 3cc083e0    	ldur	q0, [sp, #0x8]
10050d634: 3d800be0    	str	q0, [sp, #0x20]
10050d638: f9400fe8    	ldr	x8, [sp, #0x18]
10050d63c: f9001be8    	str	x8, [sp, #0x30]
10050d640: 910083e1    	add	x1, sp, #0x20
10050d644: aa1503e0    	mov	x0, x21
10050d648: 9401a2df    	bl	0x1005761c4 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17hdb67a7b69d944eb9E>
10050d64c: eb1802ff    	cmp	x23, x24
10050d650: 54fffb21    	b.ne	0x10050d5b4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0x58>
10050d654: eb170308    	subs	x8, x24, x23
10050d658: 54000400    	b.eq	0x10050d6d8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0x17c>
10050d65c: b201f3e9    	mov	x9, #-0x5555555555555556 ; =-6148914691236517206
10050d660: f2955569    	movk	x9, #0xaaab
10050d664: 9bc97d08    	umulh	x8, x8, x9
10050d668: d346fd14    	lsr	x20, x8, #6
10050d66c: 9100a2f5    	add	x21, x23, #0x28
10050d670: d2f00016    	mov	x22, #-0x8000000000000000 ; =-9223372036854775808
10050d674: 14000004    	b	0x10050d684 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0x128>
10050d678: 910182b5    	add	x21, x21, #0x60
10050d67c: f1000694    	subs	x20, x20, #0x1
10050d680: 540002c0    	b.eq	0x10050d6d8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0x17c>
10050d684: f85e82a1    	ldur	x1, [x21, #-0x18]
10050d688: b4000081    	cbz	x1, 0x10050d698 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17hd8f18d9630f4080aE+0x13c>
10050d68c: f85f02a0    	ldur	x0, [x21, #-0x10]
