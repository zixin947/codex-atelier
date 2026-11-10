
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001003eca74 <__ZN4core6option15Option$LT$T$GT$11map_or_else17hee759b27a6a279f3E>:
1003eca74: a9be4ff4    	stp	x20, x19, [sp, #-0x20]!
1003eca78: a9017bfd    	stp	x29, x30, [sp, #0x10]
1003eca7c: 910043fd    	add	x29, sp, #0x10
1003eca80: aa0003f3    	mov	x19, x0
1003eca84: 97f052bc    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003eca88: 52800540    	mov	w0, #0x2a               ; =42
1003eca8c: 52800021    	mov	w1, #0x1                ; =1
1003eca90: 97f052b5    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003eca94: b40001a0    	cbz	x0, 0x1003ecac8 <__ZN4core6option15Option$LT$T$GT$11map_or_else17hee759b27a6a279f3E+0x54>
1003eca98: d0007908    	adrp	x8, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
1003eca9c: 911d1108    	add	x8, x8, #0x744
1003ecaa0: ad400500    	ldp	q0, q1, [x8]
1003ecaa4: ad000400    	stp	q0, q1, [x0]
1003ecaa8: 3cc1a100    	ldur	q0, [x8, #0x1a]
1003ecaac: 3c81a000    	stur	q0, [x0, #0x1a]
1003ecab0: 52800548    	mov	w8, #0x2a               ; =42
1003ecab4: a9000268    	stp	x8, x0, [x19]
1003ecab8: f9000a68    	str	x8, [x19, #0x10]
1003ecabc: a9417bfd    	ldp	x29, x30, [sp, #0x10]
1003ecac0: a8c24ff4    	ldp	x20, x19, [sp], #0x20
1003ecac4: d65f03c0    	ret
1003ecac8: 52800020    	mov	w0, #0x1                ; =1
1003ecacc: 52800541    	mov	w1, #0x2a               ; =42
1003ecad0: 94378ee5    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>

00000001003ecad4 <__ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hd599aa5bd8b63259E>:
1003ecad4: aa0103e0    	mov	x0, x1
1003ecad8: d0007921    	adrp	x1, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
1003ecadc: 91115021    	add	x1, x1, #0x454
1003ecae0: 528000a2    	mov	w2, #0x5                ; =5
1003ecae4: 14364d92    	b	0x10118012c <__ZN57_$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$9write_str17h42904a4ad69a17fbE>

00000001003ecae8 <__ZN54_$LT$i64$u20$as$u20$rusqlite..types..to_sql..ToSql$GT$6to_sql17hbf14e752ef44670eE>:
1003ecae8: f9400009    	ldr	x9, [x0]
1003ecaec: f9000909    	str	x9, [x8, #0x10]
1003ecaf0: f00078e9    	adrp	x9, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003ecaf4: 3dc13920    	ldr	q0, [x9, #0x4e0]
1003ecaf8: 3d800100    	str	q0, [x8]
1003ecafc: d65f03c0    	ret

00000001003ecb00 <__ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15is_contained_in17h430773999f818014E>:
1003ecb00: d10243ff    	sub	sp, sp, #0x90
1003ecb04: a9087bfd    	stp	x29, x30, [sp, #0x80]
1003ecb08: 910203fd    	add	x29, sp, #0x80
1003ecb0c: aa0103ea    	mov	x10, x1
1003ecb10: aa0003e9    	mov	x9, x0
1003ecb14: eb03003f    	cmp	x1, x3
1003ecb18: 54000182    	b.hs	0x1003ecb48 <__ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15is_contained_in17h430773999f818014E+0x48>
1003ecb1c: 910063e8    	add	x8, sp, #0x18
1003ecb20: aa0203e0    	mov	x0, x2
1003ecb24: aa0303e1    	mov	x1, x3
1003ecb28: aa0903e2    	mov	x2, x9
1003ecb2c: aa0a03e3    	mov	x3, x10
1003ecb30: 94366362    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
1003ecb34: 910003e0    	mov	x0, sp
1003ecb38: 910063e1    	add	x1, sp, #0x18
1003ecb3c: 94000706    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
1003ecb40: b94003e0    	ldr	w0, [sp]
1003ecb44: 1400000a    	b	0x1003ecb6c <__ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15is_contained_in17h430773999f818014E+0x6c>
1003ecb48: 54000101    	b.ne	0x1003ecb68 <__ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15is_contained_in17h430773999f818014E+0x68>
1003ecb4c: aa0903e0    	mov	x0, x9
1003ecb50: aa0203e1    	mov	x1, x2
1003ecb54: aa0a03e2    	mov	x2, x10
1003ecb58: 943792d2    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1003ecb5c: 7100001f    	cmp	w0, #0x0
1003ecb60: 1a9f17e0    	cset	w0, eq
1003ecb64: 14000002    	b	0x1003ecb6c <__ZN55_$LT$$RF$str$u20$as$u20$core..str..pattern..Pattern$GT$15is_contained_in17h430773999f818014E+0x6c>
1003ecb68: 52800000    	mov	w0, #0x0                ; =0
1003ecb6c: a9487bfd    	ldp	x29, x30, [sp, #0x80]
1003ecb70: 910243ff    	add	sp, sp, #0x90
1003ecb74: d65f03c0    	ret

00000001003ecb78 <__ZN56_$LT$$RF$T$u20$as$u20$rusqlite..types..to_sql..ToSql$GT$6to_sql17h808dde3360f4a7bcE>:
1003ecb78: a9402809    	ldp	x9, x10, [x0]
1003ecb7c: 5280006b    	mov	w11, #0x3               ; =3
1003ecb80: a901250b    	stp	x11, x9, [x8, #0x10]
1003ecb84: f900110a    	str	x10, [x8, #0x20]
1003ecb88: f00078e9    	adrp	x9, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003ecb8c: 3dc13d20    	ldr	q0, [x9, #0x4f0]
1003ecb90: 3d800100    	str	q0, [x8]
1003ecb94: d65f03c0    	ret

00000001003ecb98 <__ZN56_$LT$$RF$T$u20$as$u20$rusqlite..types..to_sql..ToSql$GT$6to_sql17hc0496caa73b48bd7E>:
1003ecb98: f9400009    	ldr	x9, [x0]
1003ecb9c: a940a52a    	ldp	x10, x9, [x9, #0x8]
1003ecba0: 5280006b    	mov	w11, #0x3               ; =3
1003ecba4: a901290b    	stp	x11, x10, [x8, #0x10]
1003ecba8: f9001109    	str	x9, [x8, #0x20]
1003ecbac: f00078e9    	adrp	x9, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003ecbb0: 3dc13d20    	ldr	q0, [x9, #0x4f0]
1003ecbb4: 3d800100    	str	q0, [x8]
1003ecbb8: d65f03c0    	ret

00000001003ecbbc <__ZN56_$LT$uuid..fmt..Simple$u20$as$u20$core..fmt..Display$GT$3fmt17ha84d17fdf5076e8aE>:
1003ecbbc: 1431e3f6    	b	0x101065b94 <__ZN57_$LT$uuid..fmt..Simple$u20$as$u20$core..fmt..LowerHex$GT$3fmt17h6c40ac0e335b361dE>

00000001003ecbc0 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE>:
1003ecbc0: a9bd57f6    	stp	x22, x21, [sp, #-0x30]!
1003ecbc4: a9014ff4    	stp	x20, x19, [sp, #0x10]
1003ecbc8: a9027bfd    	stp	x29, x30, [sp, #0x20]
1003ecbcc: 910083fd    	add	x29, sp, #0x20
1003ecbd0: f9400813    	ldr	x19, [x0, #0x10]
1003ecbd4: 52800028    	mov	w8, #0x1                ; =1
1003ecbd8: 52800049    	mov	w9, #0x2                ; =2
1003ecbdc: 7140403f    	cmp	w1, #0x10, lsl #12      ; =0x10000
1003ecbe0: 5280006a    	mov	w10, #0x3               ; =3
1003ecbe4: 9a8a354a    	cinc	x10, x10, hs
1003ecbe8: 7120003f    	cmp	w1, #0x800
1003ecbec: 9a8a3129    	csel	x9, x9, x10, lo
1003ecbf0: 7102003f    	cmp	w1, #0x80
1003ecbf4: 9a893114    	csel	x20, x8, x9, lo
1003ecbf8: f9400008    	ldr	x8, [x0]
1003ecbfc: cb130108    	sub	x8, x8, x19
1003ecc00: eb08029f    	cmp	x20, x8
1003ecc04: 540000e8    	b.hi	0x1003ecc20 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0x60>
1003ecc08: f9400409    	ldr	x9, [x0, #0x8]
1003ecc0c: 8b130128    	add	x8, x9, x19
1003ecc10: 7102003f    	cmp	w1, #0x80
1003ecc14: 54000222    	b.hs	0x1003ecc58 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0x98>
1003ecc18: 39000101    	strb	w1, [x8]
1003ecc1c: 1400002a    	b	0x1003eccc4 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0x104>
1003ecc20: aa0003f5    	mov	x21, x0
1003ecc24: aa0103f6    	mov	x22, x1
1003ecc28: aa1303e1    	mov	x1, x19
1003ecc2c: aa1403e2    	mov	x2, x20
1003ecc30: 52800023    	mov	w3, #0x1                ; =1
1003ecc34: 52800024    	mov	w4, #0x1                ; =1
1003ecc38: 94368727    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
1003ecc3c: aa1603e1    	mov	x1, x22
1003ecc40: aa1503e0    	mov	x0, x21
1003ecc44: f9400aa8    	ldr	x8, [x21, #0x10]
1003ecc48: f94006a9    	ldr	x9, [x21, #0x8]
1003ecc4c: 8b080128    	add	x8, x9, x8
1003ecc50: 7102003f    	cmp	w1, #0x80
1003ecc54: 54fffe23    	b.lo	0x1003ecc18 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0x58>
1003ecc58: 12800fea    	mov	w10, #-0x80             ; =-128
1003ecc5c: 12800fe9    	mov	w9, #-0x80              ; =-128
1003ecc60: 33001429    	bfxil	w9, w1, #0, #6
1003ecc64: 7120003f    	cmp	w1, #0x800
1003ecc68: 540000c2    	b.hs	0x1003ecc80 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0xc0>
1003ecc6c: 53067c2a    	lsr	w10, w1, #6
1003ecc70: 321a054a    	orr	w10, w10, #0xc0
1003ecc74: 3900010a    	strb	w10, [x8]
1003ecc78: 39000509    	strb	w9, [x8, #0x1]
1003ecc7c: 14000012    	b	0x1003eccc4 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0x104>
1003ecc80: 12800feb    	mov	w11, #-0x80             ; =-128
1003ecc84: 33062c2b    	bfxil	w11, w1, #6, #6
1003ecc88: 53107c2c    	lsr	w12, w1, #16
1003ecc8c: 350000ec    	cbnz	w12, 0x1003ecca8 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0xe8>
1003ecc90: 530c7c2a    	lsr	w10, w1, #12
1003ecc94: 321b094a    	orr	w10, w10, #0xe0
1003ecc98: 3900010a    	strb	w10, [x8]
1003ecc9c: 3900050b    	strb	w11, [x8, #0x1]
1003ecca0: 39000909    	strb	w9, [x8, #0x2]
1003ecca4: 14000008    	b	0x1003eccc4 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0x104>
1003ecca8: 53127c2c    	lsr	w12, w1, #18
1003eccac: 321c6d8c    	orr	w12, w12, #0xfffffff0
1003eccb0: 3900010c    	strb	w12, [x8]
1003eccb4: 330c442a    	bfxil	w10, w1, #12, #6
1003eccb8: 3900050a    	strb	w10, [x8, #0x1]
1003eccbc: 3900090b    	strb	w11, [x8, #0x2]
1003eccc0: 39000d09    	strb	w9, [x8, #0x3]
1003eccc4: 8b130288    	add	x8, x20, x19
1003eccc8: f9000808    	str	x8, [x0, #0x10]
1003ecccc: 52800000    	mov	w0, #0x0                ; =0
1003eccd0: a9427bfd    	ldp	x29, x30, [sp, #0x20]
1003eccd4: a9414ff4    	ldp	x20, x19, [sp, #0x10]
1003eccd8: a8c357f6    	ldp	x22, x21, [sp], #0x30
1003eccdc: d65f03c0    	ret

00000001003ecce0 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17hfaf892e59da79312E>:
1003ecce0: a9bd57f6    	stp	x22, x21, [sp, #-0x30]!
1003ecce4: a9014ff4    	stp	x20, x19, [sp, #0x10]
1003ecce8: a9027bfd    	stp	x29, x30, [sp, #0x20]
1003eccec: 910083fd    	add	x29, sp, #0x20
1003eccf0: aa0203f3    	mov	x19, x2
1003eccf4: aa0003f4    	mov	x20, x0
1003eccf8: f9400815    	ldr	x21, [x0, #0x10]
1003eccfc: f9400008    	ldr	x8, [x0]
