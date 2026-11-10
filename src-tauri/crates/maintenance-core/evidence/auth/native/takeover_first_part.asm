
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001001f7db0 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE>:
1001f7db0: d10203ff    	sub	sp, sp, #0x80
1001f7db4: a9026ffc    	stp	x28, x27, [sp, #0x20]
1001f7db8: a90367fa    	stp	x26, x25, [sp, #0x30]
1001f7dbc: a9045ff8    	stp	x24, x23, [sp, #0x40]
1001f7dc0: a90557f6    	stp	x22, x21, [sp, #0x50]
1001f7dc4: a9064ff4    	stp	x20, x19, [sp, #0x60]
1001f7dc8: a9077bfd    	stp	x29, x30, [sp, #0x70]
1001f7dcc: 9101c3fd    	add	x29, sp, #0x70
1001f7dd0: aa0003f3    	mov	x19, x0
1001f7dd4: 52800035    	mov	w21, #0x1               ; =1
1001f7dd8: a900d7ff    	stp	xzr, x21, [sp, #0x8]
1001f7ddc: f9000fff    	str	xzr, [sp, #0x18]
1001f7de0: b40010c2    	cbz	x2, 0x1001f7ff8 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x248>
1001f7de4: aa0103f4    	mov	x20, x1
1001f7de8: d2800001    	mov	x1, #0x0                ; =0
1001f7dec: 52800016    	mov	w22, #0x0               ; =0
1001f7df0: 8b020297    	add	x23, x20, x2
1001f7df4: 52800038    	mov	w24, #0x1               ; =1
1001f7df8: 52800059    	mov	w25, #0x2               ; =2
1001f7dfc: 5280007a    	mov	w26, #0x3               ; =3
1001f7e00: 52800028    	mov	w8, #0x1                ; =1
1001f7e04: 14000004    	b	0x1001f7e14 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x64>
1001f7e08: 520002d6    	eor	w22, w22, #0x1
1001f7e0c: eb17029f    	cmp	x20, x23
1001f7e10: 54000e00    	b.eq	0x1001f7fd0 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x220>
1001f7e14: aa1403e9    	mov	x9, x20
1001f7e18: 38c0152a    	ldrsb	w10, [x9], #0x1
1001f7e1c: 12001d5b    	and	w27, w10, #0xff
1001f7e20: 37f800aa    	tbnz	w10, #0x1f, 0x1001f7e34 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x84>
1001f7e24: aa0903f4    	mov	x20, x9
1001f7e28: 71008b7f    	cmp	w27, #0x22
1001f7e2c: 540003c1    	b.ne	0x1001f7ea4 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0xf4>
1001f7e30: 17fffff6    	b	0x1001f7e08 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x58>
1001f7e34: 39400689    	ldrb	w9, [x20, #0x1]
1001f7e38: 1200152a    	and	w10, w9, #0x3f
1001f7e3c: 7103837f    	cmp	w27, #0xe0
1001f7e40: 540001e3    	b.lo	0x1001f7e7c <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0xcc>
1001f7e44: 12001369    	and	w9, w27, #0x1f
1001f7e48: 39400a8b    	ldrb	w11, [x20, #0x2]
1001f7e4c: 1200156b    	and	w11, w11, #0x3f
1001f7e50: 2a0a196a    	orr	w10, w11, w10, lsl #6
1001f7e54: 7103c37f    	cmp	w27, #0xf0
1001f7e58: 540001e3    	b.lo	0x1001f7e94 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0xe4>
1001f7e5c: 39400e8b    	ldrb	w11, [x20, #0x3]
1001f7e60: 91001294    	add	x20, x20, #0x4
1001f7e64: 1200156b    	and	w11, w11, #0x3f
1001f7e68: 2a0a197b    	orr	w27, w11, w10, lsl #6
1001f7e6c: 330e093b    	bfi	w27, w9, #18, #3
1001f7e70: 71008b7f    	cmp	w27, #0x22
1001f7e74: 54000181    	b.ne	0x1001f7ea4 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0xf4>
1001f7e78: 17ffffe4    	b	0x1001f7e08 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x58>
1001f7e7c: 91000a94    	add	x20, x20, #0x2
1001f7e80: 331a136a    	bfi	w10, w27, #6, #5
1001f7e84: aa0a03fb    	mov	x27, x10
1001f7e88: 71008b7f    	cmp	w27, #0x22
1001f7e8c: 540000c1    	b.ne	0x1001f7ea4 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0xf4>
1001f7e90: 17ffffde    	b	0x1001f7e08 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x58>
1001f7e94: 91000e94    	add	x20, x20, #0x3
1001f7e98: 2a09315b    	orr	w27, w10, w9, lsl #12
1001f7e9c: 71008b7f    	cmp	w27, #0x22
1001f7ea0: 54fffb40    	b.eq	0x1001f7e08 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x58>
1001f7ea4: 7100bb7f    	cmp	w27, #0x2e
1001f7ea8: 540000a1    	b.ne	0x1001f7ebc <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x10c>
1001f7eac: 36000936    	tbz	w22, #0x0, 0x1001f7fd0 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x220>
1001f7eb0: 5280003c    	mov	w28, #0x1               ; =1
1001f7eb4: 52800035    	mov	w21, #0x1               ; =1
1001f7eb8: 14000008    	b	0x1001f7ed8 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x128>
1001f7ebc: 7140437f    	cmp	w27, #0x10, lsl #12     ; =0x10000
1001f7ec0: 9a9a3749    	cinc	x9, x26, hs
1001f7ec4: 7120037f    	cmp	w27, #0x800
1001f7ec8: 9a893329    	csel	x9, x25, x9, lo
1001f7ecc: 7102037f    	cmp	w27, #0x80
1001f7ed0: 1a9f331c    	csel	w28, w24, wzr, lo
1001f7ed4: 9a893315    	csel	x21, x24, x9, lo
1001f7ed8: f94007e9    	ldr	x9, [sp, #0x8]
1001f7edc: cb010129    	sub	x9, x9, x1
1001f7ee0: eb0902bf    	cmp	x21, x9
1001f7ee4: aa0103e9    	mov	x9, x1
1001f7ee8: 54000128    	b.hi	0x1001f7f0c <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x15c>
1001f7eec: 8b090109    	add	x9, x8, x9
1001f7ef0: 3600023c    	tbz	w28, #0x0, 0x1001f7f34 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x184>
1001f7ef4: 3900013b    	strb	w27, [x9]
1001f7ef8: 8b0102a1    	add	x1, x21, x1
1001f7efc: f9000fe1    	str	x1, [sp, #0x18]
1001f7f00: eb17029f    	cmp	x20, x23
1001f7f04: 54fff881    	b.ne	0x1001f7e14 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x64>
1001f7f08: 14000032    	b	0x1001f7fd0 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x220>
1001f7f0c: 910023e0    	add	x0, sp, #0x8
1001f7f10: f90003e1    	str	x1, [sp]
1001f7f14: aa1503e2    	mov	x2, x21
1001f7f18: 52800023    	mov	w3, #0x1                ; =1
1001f7f1c: 52800024    	mov	w4, #0x1                ; =1
1001f7f20: 943e5a6d    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
1001f7f24: a94127e8    	ldp	x8, x9, [sp, #0x10]
1001f7f28: f94003e1    	ldr	x1, [sp]
1001f7f2c: 8b090109    	add	x9, x8, x9
1001f7f30: 3707fe3c    	tbnz	w28, #0x0, 0x1001f7ef4 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x144>
1001f7f34: 12800fea    	mov	w10, #-0x80             ; =-128
1001f7f38: 3300176a    	bfxil	w10, w27, #0, #6
1001f7f3c: 7120037f    	cmp	w27, #0x800
1001f7f40: 54000142    	b.hs	0x1001f7f68 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x1b8>
1001f7f44: 53067f6b    	lsr	w11, w27, #6
1001f7f48: 321a056b    	orr	w11, w11, #0xc0
1001f7f4c: 3900012b    	strb	w11, [x9]
1001f7f50: 3900052a    	strb	w10, [x9, #0x1]
1001f7f54: 8b0102a1    	add	x1, x21, x1
1001f7f58: f9000fe1    	str	x1, [sp, #0x18]
1001f7f5c: eb17029f    	cmp	x20, x23
1001f7f60: 54fff5a1    	b.ne	0x1001f7e14 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x64>
1001f7f64: 1400001b    	b	0x1001f7fd0 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x220>
1001f7f68: 12800feb    	mov	w11, #-0x80             ; =-128
1001f7f6c: 33062f6b    	bfxil	w11, w27, #6, #6
1001f7f70: 53107f6c    	lsr	w12, w27, #16
1001f7f74: 3500016c    	cbnz	w12, 0x1001f7fa0 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x1f0>
1001f7f78: 530c7f6c    	lsr	w12, w27, #12
1001f7f7c: 321b098c    	orr	w12, w12, #0xe0
1001f7f80: 3900012c    	strb	w12, [x9]
1001f7f84: 3900052b    	strb	w11, [x9, #0x1]
1001f7f88: 3900092a    	strb	w10, [x9, #0x2]
1001f7f8c: 8b0102a1    	add	x1, x21, x1
1001f7f90: f9000fe1    	str	x1, [sp, #0x18]
1001f7f94: eb17029f    	cmp	x20, x23
1001f7f98: 54fff3e1    	b.ne	0x1001f7e14 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x64>
1001f7f9c: 1400000d    	b	0x1001f7fd0 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x220>
1001f7fa0: 12800fec    	mov	w12, #-0x80             ; =-128
1001f7fa4: 53127f6d    	lsr	w13, w27, #18
1001f7fa8: 321c6dad    	orr	w13, w13, #0xfffffff0
1001f7fac: 3900012d    	strb	w13, [x9]
1001f7fb0: 330c476c    	bfxil	w12, w27, #12, #6
1001f7fb4: 3900052c    	strb	w12, [x9, #0x1]
1001f7fb8: 3900092b    	strb	w11, [x9, #0x2]
1001f7fbc: 39000d2a    	strb	w10, [x9, #0x3]
1001f7fc0: 8b0102a1    	add	x1, x21, x1
1001f7fc4: f9000fe1    	str	x1, [sp, #0x18]
1001f7fc8: eb17029f    	cmp	x20, x23
1001f7fcc: 54fff241    	b.ne	0x1001f7e14 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x64>
1001f7fd0: f9400bf5    	ldr	x21, [sp, #0x10]
1001f7fd4: aa1503e0    	mov	x0, x21
1001f7fd8: 9400dc60    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1001f7fdc: aa0103f4    	mov	x20, x1
1001f7fe0: b6f80161    	tbz	x1, #0x3f, 0x1001f800c <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x25c>
1001f7fe4: d2800018    	mov	x24, #0x0               ; =0
1001f7fe8: aa1803e0    	mov	x0, x24
1001f7fec: aa1403e1    	mov	x1, x20
1001f7ff0: 943f619d    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1001f7ff4: d4200020    	brk	#0x1
1001f7ff8: d2800001    	mov	x1, #0x0                ; =0
1001f7ffc: aa1503e0    	mov	x0, x21
1001f8000: 9400dc56    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1001f8004: aa0103f4    	mov	x20, x1
1001f8008: b7fffee1    	tbnz	x1, #0x3f, 0x1001f7fe4 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x234>
1001f800c: aa0003f6    	mov	x22, x0
1001f8010: b4000134    	cbz	x20, 0x1001f8034 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x284>
1001f8014: 97f82558    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1001f8018: 52800038    	mov	w24, #0x1               ; =1
1001f801c: aa1403e0    	mov	x0, x20
1001f8020: 52800021    	mov	w1, #0x1                ; =1
1001f8024: 97f82550    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1001f8028: b4fffe00    	cbz	x0, 0x1001f7fe8 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x238>
1001f802c: aa0003f7    	mov	x23, x0
1001f8030: 14000002    	b	0x1001f8038 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x288>
1001f8034: 52800037    	mov	w23, #0x1               ; =1
1001f8038: aa1703e0    	mov	x0, x23
1001f803c: aa1603e1    	mov	x1, x22
1001f8040: aa1403e2    	mov	x2, x20
1001f8044: 943f659a    	bl	0x1011d16ac <_writev+0x1011d16ac>
1001f8048: a9005e74    	stp	x20, x23, [x19]
1001f804c: f9000a74    	str	x20, [x19, #0x10]
1001f8050: f94007e1    	ldr	x1, [sp, #0x8]
1001f8054: b4000161    	cbz	x1, 0x1001f8080 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x2d0>
1001f8058: aa1503e0    	mov	x0, x21
1001f805c: 52800022    	mov	w2, #0x1                ; =1
1001f8060: a9477bfd    	ldp	x29, x30, [sp, #0x70]
1001f8064: a9464ff4    	ldp	x20, x19, [sp, #0x60]
1001f8068: a94557f6    	ldp	x22, x21, [sp, #0x50]
1001f806c: a9445ff8    	ldp	x24, x23, [sp, #0x40]
1001f8070: a94367fa    	ldp	x26, x25, [sp, #0x30]
1001f8074: a9426ffc    	ldp	x28, x27, [sp, #0x20]
1001f8078: 910203ff    	add	sp, sp, #0x80
1001f807c: 17f8253b    	b	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f8080: a9477bfd    	ldp	x29, x30, [sp, #0x70]
1001f8084: a9464ff4    	ldp	x20, x19, [sp, #0x60]
1001f8088: a94557f6    	ldp	x22, x21, [sp, #0x50]
1001f808c: a9445ff8    	ldp	x24, x23, [sp, #0x40]
1001f8090: a94367fa    	ldp	x26, x25, [sp, #0x30]
1001f8094: a9426ffc    	ldp	x28, x27, [sp, #0x20]
1001f8098: 910203ff    	add	sp, sp, #0x80
1001f809c: d65f03c0    	ret
1001f80a0: 14000001    	b	0x1001f80a4 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x2f4>
1001f80a4: f94007e1    	ldr	x1, [sp, #0x8]
1001f80a8: b40000e1    	cbz	x1, 0x1001f80c4 <__ZN13codexmate_lib4core5relay15config_takeover18first_section_part17hfae292403a2692adE+0x314>
1001f80ac: f9400be8    	ldr	x8, [sp, #0x10]
1001f80b0: aa0003f3    	mov	x19, x0
1001f80b4: aa0803e0    	mov	x0, x8
1001f80b8: 52800022    	mov	w2, #0x1                ; =1
1001f80bc: 97f8252b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1001f80c0: aa1303e0    	mov	x0, x19
1001f80c4: 943f6424    	bl	0x1011d1154 <_writev+0x1011d1154>
