
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001007851a4 <__ZN13codexmate_lib4core20account_coordination8FileLock4open17h5c3917effdd1e43bE>:
1007851a4: d103c3ff    	sub	sp, sp, #0xf0
1007851a8: a90b5ff8    	stp	x24, x23, [sp, #0xb0]
1007851ac: a90c57f6    	stp	x22, x21, [sp, #0xc0]
1007851b0: a90d4ff4    	stp	x20, x19, [sp, #0xd0]
1007851b4: a90e7bfd    	stp	x29, x30, [sp, #0xe0]
1007851b8: 910383fd    	add	x29, sp, #0xe0
1007851bc: aa0203f4    	mov	x20, x2
1007851c0: aa0103f5    	mov	x21, x1
1007851c4: aa0003f3    	mov	x19, x0
1007851c8: a90013e3    	stp	x3, x4, [sp]
1007851cc: aa0103e0    	mov	x0, x1
1007851d0: aa0203e1    	mov	x1, x2
1007851d4: 942728f6    	bl	0x10114f5ac <__ZN3std4path4Path6parent17h2f7f79d216a1f739E>
1007851d8: b40001c0    	cbz	x0, 0x100785210 <__ZN13codexmate_lib4core20account_coordination8FileLock4open17h5c3917effdd1e43bE+0x6c>
1007851dc: aa0003e8    	mov	x8, x0
1007851e0: aa0103e2    	mov	x2, x1
1007851e4: 52803fe9    	mov	w9, #0x1ff              ; =511
1007851e8: 790063e9    	strh	w9, [sp, #0x30]
1007851ec: 52800029    	mov	w9, #0x1                ; =1
1007851f0: 3900cbe9    	strb	w9, [sp, #0x32]
1007851f4: 9100c3e0    	add	x0, sp, #0x30
1007851f8: aa0803e1    	mov	x1, x8
1007851fc: 9426f17f    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
100785200: b4000080    	cbz	x0, 0x100785210 <__ZN13codexmate_lib4core20account_coordination8FileLock4open17h5c3917effdd1e43bE+0x6c>
100785204: 52800048    	mov	w8, #0x2                ; =2
100785208: a9000268    	stp	x8, x0, [x19]
10078520c: 14000046    	b	0x100785324 <__ZN13codexmate_lib4core20account_coordination8FileLock4open17h5c3917effdd1e43bE+0x180>
100785210: d2c03008    	mov	x8, #0x18000000000      ; =1649267441664
100785214: f2e02028    	movk	x8, #0x101, lsl #48
100785218: f80143e8    	stur	x8, [sp, #0x14]
10078521c: 52a00028    	mov	w8, #0x10000            ; =65536
100785220: b9001fe8    	str	w8, [sp, #0x1c]
100785224: 910083e8    	add	x8, sp, #0x20
100785228: 910053e0    	add	x0, sp, #0x14
10078522c: aa1503e1    	mov	x1, x21
100785230: aa1403e2    	mov	x2, x20
100785234: 9426f36b    	bl	0x101141fe0 <__ZN3std2fs11OpenOptions5_open17hfd14fc0134faefc7E>
100785238: b94023e8    	ldr	w8, [sp, #0x20]
10078523c: 360006c8    	tbz	w8, #0x0, 0x100785314 <__ZN13codexmate_lib4core20account_coordination8FileLock4open17h5c3917effdd1e43bE+0x170>
100785240: f94017e8    	ldr	x8, [sp, #0x28]
100785244: f81b03a8    	stur	x8, [x29, #-0x50]
100785248: 910003e8    	mov	x8, sp
10078524c: 900002c9    	adrp	x9, 0x1007dd000 <__ZN3zip9zipcrypto24ZipCryptoReader$LT$R$GT$8validate17h6abfe2f5c5f69c24E+0xf0>
100785250: 911b0129    	add	x9, x9, #0x6c0
100785254: a90327e8    	stp	x8, x9, [sp, #0x30]
100785258: d10143a8    	sub	x8, x29, #0x50
10078525c: 90004f29    	adrp	x9, 0x101169000 <__ZN5gimli4read8rnglists20RngListIter$LT$R$GT$4next17ha0add38c23e78896E+0x510>
100785260: 91211129    	add	x9, x9, #0x844
100785264: a90427e8    	stp	x8, x9, [sp, #0x40]
100785268: d0006d20    	adrp	x0, 0x10152b000 <_anon.6408540cdffbe318d49f52812e99783f.200+0x186>
10078526c: 91017c00    	add	x0, x0, #0x5f
100785270: d10123a8    	sub	x8, x29, #0x48
100785274: 9100c3e1    	add	x1, sp, #0x30
100785278: 9427c987    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10078527c: b85b83b6    	ldur	w22, [x29, #-0x48]
100785280: 3cdbc3a0    	ldur	q0, [x29, #-0x44]
100785284: 3d800fe0    	str	q0, [sp, #0x30]
100785288: b85cc3a8    	ldur	w8, [x29, #-0x34]
10078528c: b90043e8    	str	w8, [sp, #0x40]
100785290: f85b03b4    	ldur	x20, [x29, #-0x50]
100785294: 92400688    	and	x8, x20, #0x3
100785298: f100051f    	cmp	x8, #0x1
10078529c: 54000201    	b.ne	0x1007852dc <__ZN13codexmate_lib4core20account_coordination8FileLock4open17h5c3917effdd1e43bE+0x138>
1007852a0: f85ffe95    	ldr	x21, [x20, #-0x1]!
1007852a4: f9400697    	ldr	x23, [x20, #0x8]
1007852a8: f94002e8    	ldr	x8, [x23]
1007852ac: b4000068    	cbz	x8, 0x1007852b8 <__ZN13codexmate_lib4core20account_coordination8FileLock4open17h5c3917effdd1e43bE+0x114>
1007852b0: aa1503e0    	mov	x0, x21
1007852b4: d63f0100    	blr	x8
1007852b8: f94006e1    	ldr	x1, [x23, #0x8]
1007852bc: b4000081    	cbz	x1, 0x1007852cc <__ZN13codexmate_lib4core20account_coordination8FileLock4open17h5c3917effdd1e43bE+0x128>
1007852c0: f9400ae2    	ldr	x2, [x23, #0x10]
1007852c4: aa1503e0    	mov	x0, x21
1007852c8: 97e1f0a8    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007852cc: aa1403e0    	mov	x0, x20
1007852d0: 52800301    	mov	w1, #0x18               ; =24
1007852d4: 52800102    	mov	w2, #0x8                ; =8
1007852d8: 97e1f0a4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007852dc: ad4287e0    	ldp	q0, q1, [sp, #0x50]
1007852e0: 3c82c260    	stur	q0, [x19, #0x2c]
1007852e4: 3c83c261    	stur	q1, [x19, #0x3c]
1007852e8: 3dc01fe0    	ldr	q0, [sp, #0x70]
1007852ec: 3c84c260    	stur	q0, [x19, #0x4c]
1007852f0: ad4187e0    	ldp	q0, q1, [sp, #0x30]
1007852f4: 3c80c260    	stur	q0, [x19, #0xc]
1007852f8: b94083e8    	ldr	w8, [sp, #0x80]
1007852fc: b9005e68    	str	w8, [x19, #0x5c]
100785300: 3c81c261    	stur	q1, [x19, #0x1c]
100785304: 52800148    	mov	w8, #0xa                ; =10
100785308: f9000268    	str	x8, [x19]
10078530c: b9000a76    	str	w22, [x19, #0x8]
100785310: 14000005    	b	0x100785324 <__ZN13codexmate_lib4core20account_coordination8FileLock4open17h5c3917effdd1e43bE+0x180>
100785314: b94027e8    	ldr	w8, [sp, #0x24]
100785318: b9000a68    	str	w8, [x19, #0x8]
10078531c: 52800168    	mov	w8, #0xb                ; =11
100785320: f9000268    	str	x8, [x19]
100785324: a94e7bfd    	ldp	x29, x30, [sp, #0xe0]
