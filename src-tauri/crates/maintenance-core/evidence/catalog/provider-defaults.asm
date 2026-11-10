
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010048e998 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17hf90f52c124681ab0E>:
10048f2ac: f94073e8    	ldr	x8, [sp, #0xe0]
10048f2b0: f90183e8    	str	x8, [sp, #0x300]
10048f2b4: a947a7e8    	ldp	x8, x9, [sp, #0x78]
10048f2b8: f90187e9    	str	x9, [sp, #0x308]
10048f2bc: f9016fe8    	str	x8, [sp, #0x2d8]
10048f2c0: f94037e8    	ldr	x8, [sp, #0x68]
10048f2c4: f90163e8    	str	x8, [sp, #0x2c0]
10048f2c8: f9404fe8    	ldr	x8, [sp, #0x98]
10048f2cc: f90157e8    	str	x8, [sp, #0x2a8]
10048f2d0: f90147fc    	str	x28, [sp, #0x288]
10048f2d4: f9403be8    	ldr	x8, [sp, #0x70]
10048f2d8: f9014be8    	str	x8, [sp, #0x290]
10048f2dc: 52800028    	mov	w8, #0x1                ; =1
10048f2e0: f9019fe8    	str	x8, [sp, #0x338]
10048f2e4: 910cc3ee    	add	x14, sp, #0x330
10048f2e8: 9100c1c9    	add	x9, x14, #0x30
10048f2ec: f901abe8    	str	x8, [sp, #0x350]
10048f2f0: f901b7e8    	str	x8, [sp, #0x368]
10048f2f4: d2f00013    	mov	x19, #-0x8000000000000000 ; =-9223372036854775808
10048f2f8: f901c3e8    	str	x8, [sp, #0x380]
10048f2fc: f901cfe8    	str	x8, [sp, #0x398]
10048f300: f901e7e8    	str	x8, [sp, #0x3c8]
10048f304: 910a23e8    	add	x8, sp, #0x288
10048f308: eb13039f    	cmp	x28, x19
10048f30c: 9a8801d1    	csel	x17, x14, x8, eq
10048f310: 910ae3e8    	add	x8, sp, #0x2b8
10048f314: a94ed3f9    	ldp	x25, x20, [sp, #0xe8]
10048f318: eb13029f    	cmp	x20, x19
10048f31c: 9a88012d    	csel	x13, x9, x8, eq
10048f320: 910301c8    	add	x8, x14, #0xc0
10048f324: 910b43e9    	add	x9, sp, #0x2d0
10048f328: f94083f8    	ldr	x24, [sp, #0x100]
10048f32c: d280004a    	mov	x10, #0x2               ; =2
10048f330: f2f0000a    	movk	x10, #0x8000, lsl #48
10048f334: eb0a031f    	cmp	x24, x10
10048f338: 9a89010f    	csel	x15, x8, x9, eq
10048f33c: 910181c8    	add	x8, x14, #0x60
10048f340: 910ba3e9    	add	x9, sp, #0x2e8
10048f344: eb13033f    	cmp	x25, x19
10048f348: 9a890110    	csel	x16, x8, x9, eq
10048f34c: 5280010b    	mov	w11, #0x8               ; =8
10048f350: f901dbeb    	str	x11, [sp, #0x3b0]
10048f354: 295527e8    	ldp	w8, w9, [sp, #0xa8]
10048f358: 71000d1f    	cmp	w8, #0x3
10048f35c: 1a8803e8    	csel	w8, wzr, w8, eq
10048f360: 71001d3f    	cmp	w9, #0x7
10048f364: 1a8903e9    	csel	w9, wzr, w9, eq
10048f368: a950abe2    	ldp	x2, x10, [sp, #0x108]
10048f36c: eb13015f    	cmp	x10, x19
10048f370: 9a8a03ea    	csel	x10, xzr, x10, eq
10048f374: f94027ec    	ldr	x12, [sp, #0x48]
10048f378: 9a8c016b    	csel	x11, x11, x12, eq
10048f37c: f9402fec    	ldr	x12, [sp, #0x58]
10048f380: f90193ec    	str	x12, [sp, #0x320]
10048f384: f9018fe2    	str	x2, [sp, #0x318]
10048f388: 9a9703ec    	csel	x12, xzr, x23, eq
10048f38c: 910241c0    	add	x0, x14, #0x90
10048f390: 910c63e1    	add	x1, sp, #0x318
10048f394: eb13005f    	cmp	x2, x19
10048f398: f94033e2    	ldr	x2, [sp, #0x60]
10048f39c: f9017be2    	str	x2, [sp, #0x2f0]
10048f3a0: f90177f9    	str	x25, [sp, #0x2e8]
10048f3a4: f9016bf8    	str	x24, [sp, #0x2d0]
10048f3a8: f9015ff4    	str	x20, [sp, #0x2b8]
10048f3ac: f90153f6    	str	x22, [sp, #0x2a0]
10048f3b0: f9019bff    	str	xzr, [sp, #0x330]
10048f3b4: 910061c5    	add	x5, x14, #0x18
10048f3b8: 6f00e400    	movi.2d	v0, #0000000000000000
10048f3bc: 910603f7    	add	x23, sp, #0x180
10048f3c0: 3d8072e0    	str	q0, [x23, #0x1c0]
10048f3c4: 910cc3f5    	add	x21, sp, #0x330
10048f3c8: 3c8282a0    	stur	q0, [x21, #0x28]
10048f3cc: 3d8012a0    	str	q0, [x21, #0x40]
10048f3d0: 3c8582a0    	stur	q0, [x21, #0x58]
10048f3d4: 3d801ea0    	str	q0, [x21, #0x70]
10048f3d8: 3c8882a0    	stur	q0, [x21, #0x88]
10048f3dc: f901fbf3    	str	x19, [sp, #0x3f0]
10048f3e0: f901ebff    	str	xzr, [sp, #0x3d0]
10048f3e4: 9a810006    	csel	x6, x0, x1, eq
10048f3e8: b940dfee    	ldr	w14, [sp, #0xdc]
10048f3ec: 720001df    	tst	w14, #0x1
10048f3f0: b9401fee    	ldr	w14, [sp, #0x1c]
10048f3f4: 1a9f11ce    	csel	w14, w14, wzr, ne
10048f3f8: a94c83e1    	ldp	x1, x0, [sp, #0xc8]
10048f3fc: 7200001f    	tst	w0, #0x1
10048f400: f9401fe0    	ldr	x0, [sp, #0x38]
10048f404: da9f1000    	csinv	x0, x0, xzr, ne
10048f408: 7200003f    	tst	w1, #0x1
10048f40c: f94013e1    	ldr	x1, [sp, #0x20]
10048f410: 9a9f1021    	csel	x1, x1, xzr, ne
10048f414: a94b8be3    	ldp	x3, x2, [sp, #0xb8]
10048f418: 7200005f    	tst	w2, #0x1
10048f41c: f94017e2    	ldr	x2, [sp, #0x28]
10048f420: 9a9f1042    	csel	x2, x2, xzr, ne
10048f424: 7200007f    	tst	w3, #0x1
10048f428: f9401be3    	ldr	x3, [sp, #0x30]
10048f42c: 9a9f1063    	csel	x3, x3, xzr, ne
10048f430: b24107e4    	mov	x4, #-0x7fffffffffffffff ; =-9223372036854775807
10048f434: f9407fe7    	ldr	x7, [sp, #0xf8]
10048f438: eb0400ff    	cmp	x7, x4
10048f43c: 9a870264    	csel	x4, x19, x7, eq
10048f440: 910a83e7    	add	x7, sp, #0x2a0
10048f444: eb1302df    	cmp	x22, x19
10048f448: 9a8700a5    	csel	x5, x5, x7, eq
10048f44c: 3cc482a0    	ldur	q0, [x21, #0x48]
10048f450: d10383b6    	sub	x22, x29, #0xe0
10048f454: 3c8482c0    	stur	q0, [x22, #0x48]
10048f458: f941c7e7    	ldr	x7, [sp, #0x388]
10048f45c: 3dc00220    	ldr	q0, [x17]
10048f460: 3d8002c0    	str	q0, [x22]
10048f464: f9400a31    	ldr	x17, [x17, #0x10]
10048f468: 3dc000a0    	ldr	q0, [x5]
10048f46c: 3c8182c0    	stur	q0, [x22, #0x18]
10048f470: f94008a5    	ldr	x5, [x5, #0x10]
10048f474: f81303b1    	stur	x17, [x29, #-0xd0]
10048f478: f81483a5    	stur	x5, [x29, #-0xb8]
10048f47c: f94009b1    	ldr	x17, [x13, #0x10]
10048f480: f81783a7    	stur	x7, [x29, #-0x88]
10048f484: f81603b1    	stur	x17, [x29, #-0xa0]
10048f488: 3dc001a0    	ldr	q0, [x13]
10048f48c: 3d800ec0    	str	q0, [x22, #0x30]
10048f490: b940b7ed    	ldr	w13, [sp, #0xb4]
10048f494: 120001ad    	and	w13, w13, #0x1
10048f498: 3dc001e0    	ldr	q0, [x15]
10048f49c: f94009ef    	ldr	x15, [x15, #0x10]
10048f4a0: f9404bf5    	ldr	x21, [sp, #0x90]
10048f4a4: 3d8032a0    	str	q0, [x21, #0xc0]
10048f4a8: f9400a11    	ldr	x17, [x16, #0x10]
10048f4ac: f81903b1    	stur	x17, [x29, #-0x70]
10048f4b0: 3dc00203    	ldr	q3, [x16]
10048f4b4: 3d801ac3    	str	q3, [x22, #0x60]
10048f4b8: 3dc000c1    	ldr	q1, [x6]
10048f4bc: 3d8026a1    	str	q1, [x21, #0x90]
10048f4c0: f94008d0    	ldr	x16, [x6, #0x10]
10048f4c4: f9402be6    	ldr	x6, [sp, #0x50]
10048f4c8: f94007e5    	ldr	x5, [sp, #0x8]
10048f4cc: a90b16a6    	stp	x6, x5, [x21, #0xb0]
10048f4d0: b940b3e5    	ldr	w5, [sp, #0xb0]
10048f4d4: 120000a5    	and	w5, w5, #0x1
10048f4d8: ad4106c2    	ldp	q2, q1, [x22, #0x20]
10048f4dc: ad0106a2    	stp	q2, q1, [x21, #0x20]
10048f4e0: ad4006c2    	ldp	q2, q1, [x22]
10048f4e4: ad0006a2    	stp	q2, q1, [x21]
10048f4e8: ad4202c1    	ldp	q1, q0, [x22, #0x40]
10048f4ec: ad028ea0    	stp	q0, q3, [x21, #0x50]
10048f4f0: 3d8012a1    	str	q1, [x21, #0x40]
10048f4f4: a9072ab1    	stp	x17, x10, [x21, #0x70]
10048f4f8: a90832ab    	stp	x11, x12, [x21, #0x80]
10048f4fc: a90a12b0    	stp	x16, x4, [x21, #0xa0]
10048f500: a90d02af    	stp	x15, x0, [x21, #0xd0]
10048f504: a90e0aa1    	stp	x1, x2, [x21, #0xe0]
10048f508: f9007aa3    	str	x3, [x21, #0xf0]
10048f50c: b900faae    	str	w14, [x21, #0xf8]
10048f510: 3903f2ad    	strb	w13, [x21, #0xfc]
10048f514: 3903f6a5    	strb	w5, [x21, #0xfd]
10048f518: 3903faa8    	strb	w8, [x21, #0xfe]
10048f51c: 3903fea9    	strb	w9, [x21, #0xff]
