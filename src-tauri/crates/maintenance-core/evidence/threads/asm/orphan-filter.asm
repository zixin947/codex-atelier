
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001003d4efc <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E>:
1003d4efc: 6db923e9    	stp	d9, d8, [sp, #-0x70]!
1003d4f00: a9016ffc    	stp	x28, x27, [sp, #0x10]
1003d4f04: a90267fa    	stp	x26, x25, [sp, #0x20]
1003d4f08: a9035ff8    	stp	x24, x23, [sp, #0x30]
1003d4f0c: a90457f6    	stp	x22, x21, [sp, #0x40]
1003d4f10: a9054ff4    	stp	x20, x19, [sp, #0x50]
1003d4f14: a9067bfd    	stp	x29, x30, [sp, #0x60]
1003d4f18: 910183fd    	add	x29, sp, #0x60
1003d4f1c: f9400008    	ldr	x8, [x0]
1003d4f20: f9400115    	ldr	x21, [x8]
1003d4f24: f9400028    	ldr	x8, [x1]
1003d4f28: a943d113    	ldp	x19, x20, [x8, #0x38]
1003d4f2c: f100329f    	cmp	x20, #0xc
1003d4f30: 54000161    	b.ne	0x1003d4f5c <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x60>
1003d4f34: d0007ea8    	adrp	x8, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
1003d4f38: 912de508    	add	x8, x8, #0xb79
1003d4f3c: f9400269    	ldr	x9, [x19]
1003d4f40: f940010a    	ldr	x10, [x8]
1003d4f44: b9400a6b    	ldr	w11, [x19, #0x8]
1003d4f48: b9400908    	ldr	w8, [x8, #0x8]
1003d4f4c: eb0a013f    	cmp	x9, x10
1003d4f50: fa480160    	ccmp	x11, x8, #0x0, eq
1003d4f54: 54000361    	b.ne	0x1003d4fc0 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0xc4>
1003d4f58: 14000025    	b	0x1003d4fec <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0xf0>
1003d4f5c: f100369f    	cmp	x20, #0xd
1003d4f60: 54000143    	b.lo	0x1003d4f88 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x8c>
1003d4f64: d0007ea8    	adrp	x8, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
1003d4f68: 912e1508    	add	x8, x8, #0xb85
1003d4f6c: f9400109    	ldr	x9, [x8]
1003d4f70: f940026a    	ldr	x10, [x19]
1003d4f74: f8405108    	ldur	x8, [x8, #0x5]
1003d4f78: f840526b    	ldur	x11, [x19, #0x5]
1003d4f7c: eb0a013f    	cmp	x9, x10
1003d4f80: fa4b0100    	ccmp	x8, x11, #0x0, eq
1003d4f84: 54000340    	b.eq	0x1003d4fec <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0xf0>
1003d4f88: f1001a9f    	cmp	x20, #0x6
1003d4f8c: 540001e0    	b.eq	0x1003d4fc8 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0xcc>
1003d4f90: f1004e9f    	cmp	x20, #0x13
1003d4f94: 54000161    	b.ne	0x1003d4fc0 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0xc4>
1003d4f98: d0007ea8    	adrp	x8, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
1003d4f9c: 9132e908    	add	x8, x8, #0xcba
1003d4fa0: a9402a69    	ldp	x9, x10, [x19]
1003d4fa4: a940310b    	ldp	x11, x12, [x8]
1003d4fa8: f840b26d    	ldur	x13, [x19, #0xb]
1003d4fac: f840b108    	ldur	x8, [x8, #0xb]
1003d4fb0: eb0b013f    	cmp	x9, x11
1003d4fb4: fa4c0140    	ccmp	x10, x12, #0x0, eq
1003d4fb8: fa4801a0    	ccmp	x13, x8, #0x0, eq
1003d4fbc: 54000180    	b.eq	0x1003d4fec <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0xf0>
1003d4fc0: 52800000    	mov	w0, #0x0                ; =0
1003d4fc4: 1400003d    	b	0x1003d50b8 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x1bc>
1003d4fc8: d0007ea8    	adrp	x8, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
1003d4fcc: 9122c508    	add	x8, x8, #0x8b1
1003d4fd0: b9400269    	ldr	w9, [x19]
1003d4fd4: b940010a    	ldr	w10, [x8]
1003d4fd8: 79400a6b    	ldrh	w11, [x19, #0x4]
1003d4fdc: 79400908    	ldrh	w8, [x8, #0x4]
1003d4fe0: 6b0a013f    	cmp	w9, w10
1003d4fe4: 7a480160    	ccmp	w11, w8, #0x0, eq
1003d4fe8: 54fffec1    	b.ne	0x1003d4fc0 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0xc4>
1003d4fec: f9400ea8    	ldr	x8, [x21, #0x18]
1003d4ff0: b4000628    	cbz	x8, 0x1003d50b4 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x1b8>
1003d4ff4: 910082a0    	add	x0, x21, #0x20
1003d4ff8: aa1303e1    	mov	x1, x19
1003d4ffc: aa1403e2    	mov	x2, x20
1003d5000: 940bbfee    	bl	0x1006c4fb8 <__ZN4core4hash11BuildHasher8hash_one17h61e5c885b8041446E>
1003d5004: d2800016    	mov	x22, #0x0               ; =0
1003d5008: d379fc08    	lsr	x8, x0, #57
1003d500c: a9405eb5    	ldp	x21, x23, [x21]
1003d5010: 0e010d08    	dup.8b	v8, w8
1003d5014: d10062b8    	sub	x24, x21, #0x18
1003d5018: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
1003d501c: 52800319    	mov	w25, #0x18              ; =24
1003d5020: 8a17001a    	and	x26, x0, x23
1003d5024: fc7a6aa9    	ldr	d9, [x21, x26]
1003d5028: 2e288d20    	cmeq.8b	v0, v9, v8
1003d502c: 9e660008    	fmov	x8, d0
1003d5030: f201c11b    	ands	x27, x8, #0x8080808080808080
1003d5034: 54000260    	b.eq	0x1003d5080 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x184>
1003d5038: dac00368    	rbit	x8, x27
1003d503c: dac01108    	clz	x8, x8
1003d5040: 8b480f48    	add	x8, x26, x8, lsr #3
1003d5044: 8a170108    	and	x8, x8, x23
1003d5048: 9b19fd08    	mneg	x8, x8, x25
1003d504c: 8b080308    	add	x8, x24, x8
1003d5050: f9400909    	ldr	x9, [x8, #0x10]
1003d5054: eb09029f    	cmp	x20, x9
1003d5058: 540000c1    	b.ne	0x1003d5070 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x174>
1003d505c: f9400501    	ldr	x1, [x8, #0x8]
1003d5060: aa1303e0    	mov	x0, x19
1003d5064: aa1403e2    	mov	x2, x20
1003d5068: 9437f18e    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1003d506c: 34000260    	cbz	w0, 0x1003d50b8 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x1bc>
1003d5070: d1000768    	sub	x8, x27, #0x1
1003d5074: ea1b011b    	ands	x27, x8, x27
1003d5078: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
1003d507c: 54fffde1    	b.ne	0x1003d5038 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x13c>
1003d5080: 2e218d20    	cmeq.8b	v0, v9, v1
1003d5084: 2e30a800    	umaxv.8b	b0, v0
1003d5088: 1e260008    	fmov	w8, s0
1003d508c: 37000148    	tbnz	w8, #0x0, 0x1003d50b4 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x1b8>
1003d5090: 910022d6    	add	x22, x22, #0x8
1003d5094: 8b160340    	add	x0, x26, x22
1003d5098: 8a17001a    	and	x26, x0, x23
1003d509c: fc7a6aa9    	ldr	d9, [x21, x26]
1003d50a0: 2e288d20    	cmeq.8b	v0, v9, v8
1003d50a4: 9e660008    	fmov	x8, d0
1003d50a8: f201c11b    	ands	x27, x8, #0x8080808080808080
1003d50ac: 54fffc61    	b.ne	0x1003d5038 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x13c>
1003d50b0: 17fffff4    	b	0x1003d5080 <__ZN4core3ops8function5impls79_$LT$impl$u20$core..ops..function..FnMut$LT$A$GT$$u20$for$u20$$RF$mut$u20$F$GT$8call_mut17hbb347c649e2fef68E+0x184>
1003d50b4: 52800020    	mov	w0, #0x1                ; =1
1003d50b8: a9467bfd    	ldp	x29, x30, [sp, #0x60]
1003d50bc: a9454ff4    	ldp	x20, x19, [sp, #0x50]
1003d50c0: a94457f6    	ldp	x22, x21, [sp, #0x40]
1003d50c4: a9435ff8    	ldp	x24, x23, [sp, #0x30]
1003d50c8: a94267fa    	ldp	x26, x25, [sp, #0x20]
1003d50cc: a9416ffc    	ldp	x28, x27, [sp, #0x10]
1003d50d0: 6cc723e9    	ldp	d9, d8, [sp], #0x70
1003d50d4: d65f03c0    	ret
