
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008fb60c <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE>:
1008fb60c: a9bb67fa    	stp	x26, x25, [sp, #-0x50]!
1008fb610: a9015ff8    	stp	x24, x23, [sp, #0x10]
1008fb614: a90257f6    	stp	x22, x21, [sp, #0x20]
1008fb618: a9034ff4    	stp	x20, x19, [sp, #0x30]
1008fb61c: a9047bfd    	stp	x29, x30, [sp, #0x40]
1008fb620: 910103fd    	add	x29, sp, #0x40
1008fb624: a9425015    	ldp	x21, x20, [x0, #0x20]
1008fb628: eb1402bf    	cmp	x21, x20
1008fb62c: 54000642    	b.hs	0x1008fb6f4 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0xe8>
1008fb630: aa0003f3    	mov	x19, x0
1008fb634: f9400016    	ldr	x22, [x0]
1008fb638: f9400817    	ldr	x23, [x0, #0x10]
1008fb63c: d378deb8    	lsl	x24, x21, #8
1008fb640: 52800059    	mov	w25, #0x2               ; =2
1008fb644: 14000005    	b	0x1008fb658 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0x4c>
1008fb648: 910402f7    	add	x23, x23, #0x100
1008fb64c: 910402d6    	add	x22, x22, #0x100
1008fb650: eb15029f    	cmp	x20, x21
1008fb654: 540005e0    	b.eq	0x1008fb710 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0x104>
1008fb658: 910006b5    	add	x21, x21, #0x1
1008fb65c: 8b1802c8    	add	x8, x22, x24
1008fb660: f9400902    	ldr	x2, [x8, #0x10]
1008fb664: 8b1802e9    	add	x9, x23, x24
1008fb668: f940092a    	ldr	x10, [x9, #0x10]
1008fb66c: eb0a005f    	cmp	x2, x10
1008fb670: 540003a1    	b.ne	0x1008fb6e4 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0xd8>
1008fb674: f9400521    	ldr	x1, [x9, #0x8]
1008fb678: f9400500    	ldr	x0, [x8, #0x8]
1008fb67c: 94235809    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1008fb680: 35000320    	cbnz	w0, 0x1008fb6e4 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0xd8>
1008fb684: 8b1802c8    	add	x8, x22, x24
1008fb688: f9406108    	ldr	x8, [x8, #0xc0]
1008fb68c: d2410109    	eor	x9, x8, #0x8000000000000000
1008fb690: f100011f    	cmp	x8, #0x0
1008fb694: 9a99b128    	csel	x8, x9, x25, lt
1008fb698: 8b1802e9    	add	x9, x23, x24
1008fb69c: f9406129    	ldr	x9, [x9, #0xc0]
1008fb6a0: d241012a    	eor	x10, x9, #0x8000000000000000
1008fb6a4: f100013f    	cmp	x9, #0x0
1008fb6a8: 9a99b149    	csel	x9, x10, x25, lt
1008fb6ac: eb09011f    	cmp	x8, x9
1008fb6b0: 540001a1    	b.ne	0x1008fb6e4 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0xd8>
1008fb6b4: f100091f    	cmp	x8, #0x2
1008fb6b8: 54fffc81    	b.ne	0x1008fb648 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0x3c>
1008fb6bc: 8b1802c8    	add	x8, x22, x24
1008fb6c0: f9406902    	ldr	x2, [x8, #0xd0]
1008fb6c4: 8b1802e9    	add	x9, x23, x24
1008fb6c8: f940692a    	ldr	x10, [x9, #0xd0]
1008fb6cc: eb0a005f    	cmp	x2, x10
1008fb6d0: 540000a1    	b.ne	0x1008fb6e4 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0xd8>
1008fb6d4: f9406521    	ldr	x1, [x9, #0xc8]
1008fb6d8: f9406500    	ldr	x0, [x8, #0xc8]
1008fb6dc: 942357f1    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1008fb6e0: 34fffb40    	cbz	w0, 0x1008fb648 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0x3c>
1008fb6e4: 52800020    	mov	w0, #0x1                ; =1
1008fb6e8: aa1503f4    	mov	x20, x21
1008fb6ec: f9001274    	str	x20, [x19, #0x20]
1008fb6f0: 14000002    	b	0x1008fb6f8 <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0xec>
1008fb6f4: 52800000    	mov	w0, #0x0                ; =0
1008fb6f8: a9447bfd    	ldp	x29, x30, [sp, #0x40]
1008fb6fc: a9434ff4    	ldp	x20, x19, [sp, #0x30]
1008fb700: a94257f6    	ldp	x22, x21, [sp, #0x20]
1008fb704: a9415ff8    	ldp	x24, x23, [sp, #0x10]
1008fb708: a8c567fa    	ldp	x26, x25, [sp], #0x50
1008fb70c: d65f03c0    	ret
1008fb710: 52800000    	mov	w0, #0x0                ; =0
1008fb714: 17fffff6    	b	0x1008fb6ec <__ZN4core4iter6traits8iterator8Iterator8try_fold17hd150703405e919eaE+0xe0>
