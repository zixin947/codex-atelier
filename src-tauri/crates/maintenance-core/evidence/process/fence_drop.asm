
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100465a28 <__ZN4core3ptr75drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence$GT$17h54770a26186b2690E>:
100465a28: d10143ff    	sub	sp, sp, #0x50
100465a2c: a90257f6    	stp	x22, x21, [sp, #0x20]
100465a30: a9034ff4    	stp	x20, x19, [sp, #0x30]
100465a34: a9047bfd    	stp	x29, x30, [sp, #0x40]
100465a38: 910103fd    	add	x29, sp, #0x40
100465a3c: aa0003f3    	mov	x19, x0
100465a40: aa0003f4    	mov	x20, x0
100465a44: f8418e88    	ldr	x8, [x20, #0x18]!
100465a48: 52800029    	mov	w9, #0x1                ; =1
100465a4c: 19010109    	stlurb	w9, [x8, #0x10]
100465a50: f9400008    	ldr	x8, [x0]
100465a54: f900001f    	str	xzr, [x0]
100465a58: b40002a8    	cbz	x8, 0x100465aac <__ZN4core3ptr75drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence$GT$17h54770a26186b2690E+0x84>
100465a5c: f90007e8    	str	x8, [sp, #0x8]
100465a60: 3cc08260    	ldur	q0, [x19, #0x8]
100465a64: 3c8103e0    	stur	q0, [sp, #0x10]
100465a68: 910023e0    	add	x0, sp, #0x8
100465a6c: 97ffc053    	bl	0x100455bb8 <__ZN3std6thread9lifecycle18JoinInner$LT$T$GT$4join17hb3431dfd489e897bE>
100465a70: aa0003f6    	mov	x22, x0
100465a74: b40001c0    	cbz	x0, 0x100465aac <__ZN4core3ptr75drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence$GT$17h54770a26186b2690E+0x84>
100465a78: aa0103f5    	mov	x21, x1
100465a7c: f9400028    	ldr	x8, [x1]
100465a80: b4000068    	cbz	x8, 0x100465a8c <__ZN4core3ptr75drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence$GT$17h54770a26186b2690E+0x64>
100465a84: aa1603e0    	mov	x0, x22
100465a88: d63f0100    	blr	x8
100465a8c: f94006a1    	ldr	x1, [x21, #0x8]
100465a90: b4000081    	cbz	x1, 0x100465aa0 <__ZN4core3ptr75drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence$GT$17h54770a26186b2690E+0x78>
100465a94: f9400aa2    	ldr	x2, [x21, #0x10]
100465a98: aa1603e0    	mov	x0, x22
100465a9c: 97ee6eb3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100465aa0: f9401668    	ldr	x8, [x19, #0x28]
100465aa4: 52800029    	mov	w9, #0x1                ; =1
100465aa8: 19010109    	stlurb	w9, [x8, #0x10]
100465aac: f9400288    	ldr	x8, [x20]
100465ab0: 92800015    	mov	x21, #-0x1              ; =-1
100465ab4: f8750108    	ldaddl	x21, x8, [x8]
100465ab8: f100051f    	cmp	x8, #0x1
100465abc: 54000081    	b.ne	0x100465acc <__ZN4core3ptr75drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence$GT$17h54770a26186b2690E+0xa4>
100465ac0: d50339bf    	dmb	ishld
100465ac4: aa1403e0    	mov	x0, x20
100465ac8: 9423f251    	bl	0x100d6240c <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha0032db423d2ba15E>
100465acc: aa1303e0    	mov	x0, x19
100465ad0: f8420c08    	ldr	x8, [x0, #0x20]!
100465ad4: f8750108    	ldaddl	x21, x8, [x8]
100465ad8: f100051f    	cmp	x8, #0x1
100465adc: 54000061    	b.ne	0x100465ae8 <__ZN4core3ptr75drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence$GT$17h54770a26186b2690E+0xc0>
100465ae0: d50339bf    	dmb	ishld
100465ae4: 942c413a    	bl	0x100f75fcc <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hd06e0bc76c515657E>
100465ae8: aa1303e0    	mov	x0, x19
100465aec: f8428c08    	ldr	x8, [x0, #0x28]!
100465af0: 92800009    	mov	x9, #-0x1               ; =-1
100465af4: f8690108    	ldaddl	x9, x8, [x8]
100465af8: f100051f    	cmp	x8, #0x1
100465afc: 54000061    	b.ne	0x100465b08 <__ZN4core3ptr75drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence$GT$17h54770a26186b2690E+0xe0>
100465b00: d50339bf    	dmb	ishld
100465b04: 9423f242    	bl	0x100d6240c <__ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17ha0032db423d2ba15E>
100465b08: aa1303e0    	mov	x0, x19
100465b0c: a9447bfd    	ldp	x29, x30, [sp, #0x40]
100465b10: a9434ff4    	ldp	x20, x19, [sp, #0x30]
100465b14: a94257f6    	ldp	x22, x21, [sp, #0x20]
100465b18: 910143ff    	add	sp, sp, #0x50
100465b1c: 17ffd7f2    	b	0x10045bae4 <__ZN4core3ptr101drop_in_place$LT$core..option..Option$LT$std..thread..join_handle..JoinHandle$LT$$LP$$RP$$GT$$GT$$GT$17h21e7a4280c4e5bf8E>
