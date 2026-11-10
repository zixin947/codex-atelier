
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010050f9fc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E>:
10050f9fc: d10203ff    	sub	sp, sp, #0x80
10050fa00: a9026ffc    	stp	x28, x27, [sp, #0x20]
10050fa04: a90367fa    	stp	x26, x25, [sp, #0x30]
10050fa08: a9045ff8    	stp	x24, x23, [sp, #0x40]
10050fa0c: a90557f6    	stp	x22, x21, [sp, #0x50]
10050fa10: a9064ff4    	stp	x20, x19, [sp, #0x60]
10050fa14: a9077bfd    	stp	x29, x30, [sp, #0x70]
10050fa18: 9101c3fd    	add	x29, sp, #0x70
10050fa1c: f9400c0a    	ldr	x10, [x0, #0x18]
10050fa20: a90103ea    	stp	x10, x0, [sp, #0x10]
10050fa24: f9400409    	ldr	x9, [x0, #0x8]
10050fa28: eb0a013f    	cmp	x9, x10
10050fa2c: 54000620    	b.eq	0x10050faf0 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0xf4>
10050fa30: f90003e8    	str	x8, [sp]
10050fa34: a9425428    	ldp	x8, x21, [x1, #0x20]
10050fa38: f90007e8    	str	x8, [sp, #0x8]
10050fa3c: 91002133    	add	x19, x9, #0x8
10050fa40: 14000007    	b	0x10050fa5c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0x60>
10050fa44: f1000a9f    	cmp	x20, #0x2
10050fa48: 54000581    	b.ne	0x10050faf8 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0xfc>
10050fa4c: 91018273    	add	x19, x19, #0x60
10050fa50: f9400be8    	ldr	x8, [sp, #0x10]
10050fa54: eb0802df    	cmp	x22, x8
10050fa58: 54000460    	b.eq	0x10050fae4 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0xe8>
10050fa5c: f85f8274    	ldur	x20, [x19, #-0x8]
10050fa60: a940f27b    	ldp	x27, x28, [x19, #0x8]
10050fa64: a941de68    	ldp	x8, x23, [x19, #0x18]
10050fa68: f9401678    	ldr	x24, [x19, #0x28]
10050fa6c: a943ea79    	ldp	x25, x26, [x19, #0x38]
10050fa70: 91016276    	add	x22, x19, #0x58
10050fa74: f9400fe9    	ldr	x9, [sp, #0x18]
10050fa78: f9000536    	str	x22, [x9, #0x8]
10050fa7c: eb15011f    	cmp	x8, x21
10050fa80: 540000c1    	b.ne	0x10050fa98 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0x9c>
10050fa84: aa1c03e0    	mov	x0, x28
10050fa88: f94007e1    	ldr	x1, [sp, #0x8]
10050fa8c: aa1503e2    	mov	x2, x21
10050fa90: 94330704    	bl	0x1011d16a0 <_writev+0x1011d16a0>
10050fa94: 34fffd80    	cbz	w0, 0x10050fa44 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0x48>
10050fa98: b40000bb    	cbz	x27, 0x10050faac <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0xb0>
10050fa9c: aa1c03e0    	mov	x0, x28
10050faa0: aa1b03e1    	mov	x1, x27
10050faa4: 52800022    	mov	w2, #0x1                ; =1
10050faa8: 97ebc6b0    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050faac: f240fb3f    	tst	x25, #0x7fffffffffffffff
10050fab0: 54000061    	b.ne	0x10050fabc <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0xc0>
10050fab4: b50000f7    	cbnz	x23, 0x10050fad0 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0xd4>
10050fab8: 17ffffe5    	b	0x10050fa4c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0x50>
10050fabc: aa1a03e0    	mov	x0, x26
10050fac0: aa1903e1    	mov	x1, x25
10050fac4: 52800022    	mov	w2, #0x1                ; =1
10050fac8: 97ebc6a8    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050facc: b4fffc17    	cbz	x23, 0x10050fa4c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0x50>
10050fad0: aa1803e0    	mov	x0, x24
10050fad4: aa1703e1    	mov	x1, x23
10050fad8: 52800022    	mov	w2, #0x1                ; =1
10050fadc: 97ebc6a3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10050fae0: 17ffffdb    	b	0x10050fa4c <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0x50>
10050fae4: 52800054    	mov	w20, #0x2               ; =2
10050fae8: f94003e8    	ldr	x8, [sp]
10050faec: 1400000e    	b	0x10050fb24 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0x128>
10050faf0: 52800054    	mov	w20, #0x2               ; =2
10050faf4: 1400000c    	b	0x10050fb24 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfce3d2bdfe47ebb6E+0x128>
10050faf8: ad410660    	ldp	q0, q1, [x19, #0x20]
10050fafc: f94003e8    	ldr	x8, [sp]
10050fb00: 3c828100    	stur	q0, [x8, #0x28]
10050fb04: 3c838101    	stur	q1, [x8, #0x38]
10050fb08: 3dc01260    	ldr	q0, [x19, #0x40]
10050fb0c: 3c848100    	stur	q0, [x8, #0x48]
10050fb10: f9402a69    	ldr	x9, [x19, #0x50]
10050fb14: f9002d09    	str	x9, [x8, #0x58]
10050fb18: ad400660    	ldp	q0, q1, [x19]
10050fb1c: 3c808100    	stur	q0, [x8, #0x8]
10050fb20: 3c818101    	stur	q1, [x8, #0x18]
10050fb24: f9000114    	str	x20, [x8]
10050fb28: a9477bfd    	ldp	x29, x30, [sp, #0x70]
10050fb2c: a9464ff4    	ldp	x20, x19, [sp, #0x60]
10050fb30: a94557f6    	ldp	x22, x21, [sp, #0x50]
10050fb34: a9445ff8    	ldp	x24, x23, [sp, #0x40]
10050fb38: a94367fa    	ldp	x26, x25, [sp, #0x30]
10050fb3c: a9426ffc    	ldp	x28, x27, [sp, #0x20]
10050fb40: 910203ff    	add	sp, sp, #0x80
10050fb44: d65f03c0    	ret

000000010050fb48 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfccc097fbfd12d0aE>:
10050fb48: d107c3ff    	sub	sp, sp, #0x1f0
10050fb4c: a9196ffc    	stp	x28, x27, [sp, #0x190]
10050fb50: a91a67fa    	stp	x26, x25, [sp, #0x1a0]
10050fb54: a91b5ff8    	stp	x24, x23, [sp, #0x1b0]
10050fb58: a91c57f6    	stp	x22, x21, [sp, #0x1c0]
10050fb5c: a91d4ff4    	stp	x20, x19, [sp, #0x1d0]
10050fb60: a91e7bfd    	stp	x29, x30, [sp, #0x1e0]
10050fb64: 910783fd    	add	x29, sp, #0x1e0
10050fb68: aa0203f3    	mov	x19, x2
10050fb6c: aa0103f4    	mov	x20, x1
10050fb70: f9400c16    	ldr	x22, [x0, #0x18]
10050fb74: f9400408    	ldr	x8, [x0, #0x8]
10050fb78: eb16011f    	cmp	x8, x22
10050fb7c: 54000660    	b.eq	0x10050fc48 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfccc097fbfd12d0aE+0x100>
10050fb80: aa0003f5    	mov	x21, x0
10050fb84: 910183f7    	add	x23, sp, #0x60
10050fb88: 910023f8    	add	x24, sp, #0x8
10050fb8c: f9400079    	ldr	x25, [x3]
10050fb90: 5280003a    	mov	w26, #0x1               ; =1
10050fb94: 3dc00100    	ldr	q0, [x8]
10050fb98: 3c810300    	stur	q0, [x24, #0x10]
10050fb9c: f9400909    	ldr	x9, [x8, #0x10]
10050fba0: f8020309    	stur	x9, [x24, #0x20]
10050fba4: 9100611b    	add	x27, x8, #0x18
10050fba8: f90006bb    	str	x27, [x21, #0x8]
10050fbac: a900cff4    	stp	x20, x19, [sp, #0x8]
10050fbb0: a9034ff4    	stp	x20, x19, [sp, #0x30]
10050fbb4: 3dc00100    	ldr	q0, [x8]
10050fbb8: f9400908    	ldr	x8, [x8, #0x10]
10050fbbc: f9002be8    	str	x8, [sp, #0x50]
10050fbc0: 3d8013e0    	str	q0, [sp, #0x40]
10050fbc4: f940033c    	ldr	x28, [x25]
10050fbc8: d103a3a0    	sub	x0, x29, #0xe8
10050fbcc: 91004381    	add	x1, x28, #0x10
10050fbd0: 9404e328    	bl	0x100648870 <__ZN74_$LT$tauri_runtime_wry..Context$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17h9a6a0e68fad96574E>
10050fbd4: f9404f88    	ldr	x8, [x28, #0x98]
10050fbd8: f83a0109    	ldadd	x26, x9, [x8]
10050fbdc: b7f804a9    	tbnz	x9, #0x3f, 0x10050fc70 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfccc097fbfd12d0aE+0x128>
10050fbe0: f9405389    	ldr	x9, [x28, #0xa0]
10050fbe4: f83a012a    	ldadd	x26, x10, [x9]
10050fbe8: b7f8044a    	tbnz	x10, #0x3f, 0x10050fc70 <__ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8try_fold17hfccc097fbfd12d0aE+0x128>
10050fbec: 3cd783a0    	ldur	q0, [x29, #-0x88]
10050fbf0: 3cd883a1    	ldur	q1, [x29, #-0x78]
10050fbf4: 3cd683a2    	ldur	q2, [x29, #-0x98]
10050fbf8: 3cd583a3    	ldur	q3, [x29, #-0xa8]
10050fbfc: ad0282e2    	stp	q2, q0, [x23, #0x50]
