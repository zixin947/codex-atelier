
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010020791c <__ZN13codexmate_lib4core5relay7storage21save_with_publication28_$u7b$$u7b$closure$u7d$$u7d$17hc7d3c1631a37c396E>:
10020791c: d10183ff    	sub	sp, sp, #0x60
100207920: a90357f6    	stp	x22, x21, [sp, #0x30]
100207924: a9044ff4    	stp	x20, x19, [sp, #0x40]
100207928: a9057bfd    	stp	x29, x30, [sp, #0x50]
10020792c: 910143fd    	add	x29, sp, #0x50
100207930: aa0003f3    	mov	x19, x0
100207934: f90003e1    	str	x1, [sp]
100207938: d0007b08    	adrp	x8, 0x101169000 <__ZN5gimli4read8rnglists20RngListIter$LT$R$GT$4next17ha0add38c23e78896E+0x510>
10020793c: 91211108    	add	x8, x8, #0x844
100207940: 910003e9    	mov	x9, sp
100207944: a90223e9    	stp	x9, x8, [sp, #0x20]
100207948: d00098a0    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
10020794c: 91204c00    	add	x0, x0, #0x813
100207950: 910023e8    	add	x8, sp, #0x8
100207954: 910083e1    	add	x1, sp, #0x20
100207958: 943dbfcf    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10020795c: 3cc083e0    	ldur	q0, [sp, #0x8]
100207960: 3c808260    	stur	q0, [x19, #0x8]
100207964: f9400fe8    	ldr	x8, [sp, #0x18]
100207968: f9000e68    	str	x8, [x19, #0x18]
10020796c: 52800148    	mov	w8, #0xa                ; =10
100207970: f9000268    	str	x8, [x19]
100207974: f94003f3    	ldr	x19, [sp]
100207978: 92400668    	and	x8, x19, #0x3
10020797c: f100051f    	cmp	x8, #0x1
100207980: 54000201    	b.ne	0x1002079c0 <__ZN13codexmate_lib4core5relay7storage21save_with_publication28_$u7b$$u7b$closure$u7d$$u7d$17hc7d3c1631a37c396E+0xa4>
100207984: f85ffe75    	ldr	x21, [x19, #-0x1]!
100207988: f9400674    	ldr	x20, [x19, #0x8]
10020798c: f9400288    	ldr	x8, [x20]
100207990: b4000068    	cbz	x8, 0x10020799c <__ZN13codexmate_lib4core5relay7storage21save_with_publication28_$u7b$$u7b$closure$u7d$$u7d$17hc7d3c1631a37c396E+0x80>
100207994: aa1503e0    	mov	x0, x21
100207998: d63f0100    	blr	x8
10020799c: f9400681    	ldr	x1, [x20, #0x8]
1002079a0: b4000081    	cbz	x1, 0x1002079b0 <__ZN13codexmate_lib4core5relay7storage21save_with_publication28_$u7b$$u7b$closure$u7d$$u7d$17hc7d3c1631a37c396E+0x94>
1002079a4: f9400a82    	ldr	x2, [x20, #0x10]
1002079a8: aa1503e0    	mov	x0, x21
1002079ac: 97f7e6ef    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1002079b0: aa1303e0    	mov	x0, x19
1002079b4: 52800301    	mov	w1, #0x18               ; =24
1002079b8: 52800102    	mov	w2, #0x8                ; =8
1002079bc: 97f7e6eb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1002079c0: a9457bfd    	ldp	x29, x30, [sp, #0x50]
1002079c4: a9444ff4    	ldp	x20, x19, [sp, #0x40]
1002079c8: a94357f6    	ldp	x22, x21, [sp, #0x30]
1002079cc: 910183ff    	add	sp, sp, #0x60
1002079d0: d65f03c0    	ret
1002079d4: aa1403e9    	mov	x9, x20
