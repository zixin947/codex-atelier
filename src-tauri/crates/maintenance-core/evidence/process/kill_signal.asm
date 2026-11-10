
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001007e6ec0 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E>:
1007e6ec0: d10503ff    	sub	sp, sp, #0x140
1007e6ec4: a90f6ffc    	stp	x28, x27, [sp, #0xf0]
1007e6ec8: a9105ff8    	stp	x24, x23, [sp, #0x100]
1007e6ecc: a91157f6    	stp	x22, x21, [sp, #0x110]
1007e6ed0: a9124ff4    	stp	x20, x19, [sp, #0x120]
1007e6ed4: a9137bfd    	stp	x29, x30, [sp, #0x130]
1007e6ed8: 9104c3fd    	add	x29, sp, #0x130
1007e6edc: aa0803f3    	mov	x19, x8
1007e6ee0: f90007e1    	str	x1, [sp, #0x8]
1007e6ee4: b9400020    	ldr	w0, [x1]
1007e6ee8: 52800121    	mov	w1, #0x9                ; =9
1007e6eec: 9427a9b4    	bl	0x1011d15bc <_writev+0x1011d15bc>
1007e6ef0: 340001c0    	cbz	w0, 0x1007e6f28 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E+0x68>
1007e6ef4: 9427a8a7    	bl	0x1011d1190 <_writev+0x1011d1190>
1007e6ef8: b9400016    	ldr	w22, [x0]
1007e6efc: 52800048    	mov	w8, #0x2                ; =2
1007e6f00: aa168114    	orr	x20, x8, x22, lsl #32
1007e6f04: f81b83b4    	stur	x20, [x29, #-0x48]
1007e6f08: 71000edf    	cmp	w22, #0x3
1007e6f0c: 54000141    	b.ne	0x1007e6f34 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E+0x74>
1007e6f10: 52800177    	mov	w23, #0xb               ; =11
1007e6f14: 92400688    	and	x8, x20, #0x3
1007e6f18: f100051f    	cmp	x8, #0x1
1007e6f1c: 54000300    	b.eq	0x1007e6f7c <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E+0xbc>
1007e6f20: 71000edf    	cmp	w22, #0x3
1007e6f24: 540004e1    	b.ne	0x1007e6fc0 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E+0x100>
1007e6f28: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1007e6f2c: f9000268    	str	x8, [x19]
1007e6f30: 14000043    	b	0x1007e703c <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E+0x17c>
1007e6f34: f0004c08    	adrp	x8, 0x101169000 <__ZN5gimli4read8rnglists20RngListIter$LT$R$GT$4next17ha0add38c23e78896E+0x510>
1007e6f38: 91211108    	add	x8, x8, #0x844
1007e6f3c: d10123a9    	sub	x9, x29, #0x48
1007e6f40: a90723e9    	stp	x9, x8, [sp, #0x70]
1007e6f44: b0006a20    	adrp	x0, 0x10152b000 <_anon.6408540cdffbe318d49f52812e99783f.200+0x186>
1007e6f48: 912d9800    	add	x0, x0, #0xb66
1007e6f4c: 910223e8    	add	x8, sp, #0x88
1007e6f50: 9101c3e1    	add	x1, sp, #0x70
1007e6f54: 94264250    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1007e6f58: 3cc883e0    	ldur	q0, [sp, #0x88]
1007e6f5c: 3d8007e0    	str	q0, [sp, #0x10]
1007e6f60: f9404fe8    	ldr	x8, [sp, #0x98]
1007e6f64: f90013e8    	str	x8, [sp, #0x20]
1007e6f68: f85b83b4    	ldur	x20, [x29, #-0x48]
1007e6f6c: 52800157    	mov	w23, #0xa               ; =10
1007e6f70: 92400688    	and	x8, x20, #0x3
1007e6f74: f100051f    	cmp	x8, #0x1
1007e6f78: 54fffd41    	b.ne	0x1007e6f20 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E+0x60>
1007e6f7c: f85ffe95    	ldr	x21, [x20, #-0x1]!
1007e6f80: f9400698    	ldr	x24, [x20, #0x8]
1007e6f84: f9400308    	ldr	x8, [x24]
1007e6f88: b4000068    	cbz	x8, 0x1007e6f94 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E+0xd4>
1007e6f8c: aa1503e0    	mov	x0, x21
1007e6f90: d63f0100    	blr	x8
1007e6f94: f9400701    	ldr	x1, [x24, #0x8]
1007e6f98: b4000081    	cbz	x1, 0x1007e6fa8 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E+0xe8>
1007e6f9c: f9400b02    	ldr	x2, [x24, #0x10]
1007e6fa0: aa1503e0    	mov	x0, x21
1007e6fa4: 97e06971    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007e6fa8: aa1403e0    	mov	x0, x20
1007e6fac: 52800301    	mov	w1, #0x18               ; =24
1007e6fb0: 52800102    	mov	w2, #0x8                ; =8
1007e6fb4: 97e0696d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007e6fb8: 71000edf    	cmp	w22, #0x3
1007e6fbc: 54fffb60    	b.eq	0x1007e6f28 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17h83677e25ce091d37E+0x68>
1007e6fc0: 910223e8    	add	x8, sp, #0x88
1007e6fc4: ad4187e0    	ldp	q0, q1, [sp, #0x30]
1007e6fc8: 3c828100    	stur	q0, [x8, #0x28]
1007e6fcc: 3c838101    	stur	q1, [x8, #0x38]
1007e6fd0: 3dc017e0    	ldr	q0, [sp, #0x50]
1007e6fd4: 3c848100    	stur	q0, [x8, #0x48]
1007e6fd8: ad4087e0    	ldp	q0, q1, [sp, #0x10]
1007e6fdc: 3c808100    	stur	q0, [x8, #0x8]
1007e6fe0: f94033e9    	ldr	x9, [sp, #0x60]
1007e6fe4: 3c818101    	stur	q1, [x8, #0x18]
1007e6fe8: f90073e9    	str	x9, [sp, #0xe0]
1007e6fec: f90047f7    	str	x23, [sp, #0x88]
1007e6ff0: 910023e8    	add	x8, sp, #0x8
1007e6ff4: f0ffffa9    	adrp	x9, 0x1007dd000 <__ZN3zip9zipcrypto24ZipCryptoReader$LT$R$GT$8validate17h6abfe2f5c5f69c24E+0xf0>
1007e6ff8: 911bc129    	add	x9, x9, #0x6f0
1007e6ffc: a90127e8    	stp	x8, x9, [sp, #0x10]
1007e7000: 910223e8    	add	x8, sp, #0x88
