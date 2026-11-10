
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010036cd64 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E>:
10036cd64: f1002c3f    	cmp	x1, #0xb
10036cd68: 5400028c    	b.gt	0x10036cdb8 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0x54>
10036cd6c: f1001c3f    	cmp	x1, #0x7
10036cd70: 540004a0    	b.eq	0x10036ce04 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0xa0>
10036cd74: f1002c3f    	cmp	x1, #0xb
10036cd78: 54000801    	b.ne	0x10036ce78 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0x114>
10036cd7c: f9400008    	ldr	x8, [x0]
10036cd80: f8403009    	ldur	x9, [x0, #0x3]
10036cd84: d28e0cea    	mov	x10, #0x7067            ; =28775
10036cd88: f2a5ae8a    	movk	x10, #0x2d74, lsl #16
10036cd8c: f2c5c6aa    	movk	x10, #0x2e35, lsl #32
10036cd90: f2e5a6ca    	movk	x10, #0x2d36, lsl #48
10036cd94: eb0a011f    	cmp	x8, x10
10036cd98: d286a5a8    	mov	x8, #0x352d             ; =13613
10036cd9c: f2a6c5c8    	movk	x8, #0x362e, lsl #16
10036cda0: f2ce65a8    	movk	x8, #0x732d, lsl #32
10036cda4: f2ed8de8    	movk	x8, #0x6c6f, lsl #48
10036cda8: fa480120    	ccmp	x9, x8, #0x0, eq
10036cdac: 54000661    	b.ne	0x10036ce78 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0x114>
10036cdb0: 52800020    	mov	w0, #0x1                ; =1
10036cdb4: d65f03c0    	ret
10036cdb8: f100303f    	cmp	x1, #0xc
10036cdbc: 54000480    	b.eq	0x10036ce4c <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0xe8>
10036cdc0: f100343f    	cmp	x1, #0xd
10036cdc4: 540005a1    	b.ne	0x10036ce78 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0x114>
10036cdc8: f9400008    	ldr	x8, [x0]
10036cdcc: f8405009    	ldur	x9, [x0, #0x5]
10036cdd0: d28e0cea    	mov	x10, #0x7067            ; =28775
10036cdd4: f2a5ae8a    	movk	x10, #0x2d74, lsl #16
10036cdd8: f2c5c6aa    	movk	x10, #0x2e35, lsl #32
10036cddc: f2e5a6ca    	movk	x10, #0x2d36, lsl #48
10036cde0: eb0a011f    	cmp	x8, x10
10036cde4: d286c5c8    	mov	x8, #0x362e             ; =13870
10036cde8: f2ae85a8    	movk	x8, #0x742d, lsl #16
10036cdec: f2ce4ca8    	movk	x8, #0x7265, lsl #32
10036cdf0: f2ec2e48    	movk	x8, #0x6172, lsl #48
10036cdf4: fa480120    	ccmp	x9, x8, #0x0, eq
10036cdf8: 54000401    	b.ne	0x10036ce78 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0x114>
10036cdfc: 52800020    	mov	w0, #0x1                ; =1
10036ce00: d65f03c0    	ret
10036ce04: 528e0ce8    	mov	w8, #0x7067             ; =28775
10036ce08: 72a5ae88    	movk	w8, #0x2d74, lsl #16
10036ce0c: b9400009    	ldr	w9, [x0]
10036ce10: b840300a    	ldur	w10, [x0, #0x3]
10036ce14: 6b08013f    	cmp	w9, w8
10036ce18: 5286a5a9    	mov	w9, #0x352d             ; =13613
10036ce1c: 72a6a5c9    	movk	w9, #0x352e, lsl #16
10036ce20: 7a490140    	ccmp	w10, w9, #0x0, eq
10036ce24: 54000100    	b.eq	0x10036ce44 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0xe0>
10036ce28: b9400009    	ldr	w9, [x0]
10036ce2c: b840300a    	ldur	w10, [x0, #0x3]
10036ce30: 6b09011f    	cmp	w8, w9
10036ce34: 5286a5a8    	mov	w8, #0x352d             ; =13613
10036ce38: 72a685c8    	movk	w8, #0x342e, lsl #16
10036ce3c: 7a480140    	ccmp	w10, w8, #0x0, eq
10036ce40: 540001c1    	b.ne	0x10036ce78 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0x114>
10036ce44: 52800020    	mov	w0, #0x1                ; =1
10036ce48: d65f03c0    	ret
10036ce4c: f9400008    	ldr	x8, [x0]
10036ce50: b9400809    	ldr	w9, [x0, #0x8]
10036ce54: d28e0cea    	mov	x10, #0x7067            ; =28775
10036ce58: f2a5ae8a    	movk	x10, #0x2d74, lsl #16
10036ce5c: f2c5c6aa    	movk	x10, #0x2e35, lsl #32
10036ce60: f2e5a6ca    	movk	x10, #0x2d36, lsl #48
10036ce64: eb0a011f    	cmp	x8, x10
10036ce68: 528ead88    	mov	w8, #0x756c             ; =30060
10036ce6c: 72ac2dc8    	movk	w8, #0x616e, lsl #16
10036ce70: 7a480120    	ccmp	w9, w8, #0x0, eq
10036ce74: 54fffe80    	b.eq	0x10036ce44 <__ZN13codexmate_lib4core5relay13codex_catalog23is_no_account_slot_slug17hef5aba0642992d20E+0xe0>
10036ce78: 52800000    	mov	w0, #0x0                ; =0
10036ce7c: d65f03c0    	ret

000000010036ce80 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E>:
10036ce80: f9404808    	ldr	x8, [x0, #0x90]
10036ce84: b4000ee8    	cbz	x8, 0x10036d060 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x1e0>
10036ce88: d10203ff    	sub	sp, sp, #0x80
10036ce8c: 6d0123e9    	stp	d9, d8, [sp, #0x10]
10036ce90: a9026ffc    	stp	x28, x27, [sp, #0x20]
10036ce94: a90367fa    	stp	x26, x25, [sp, #0x30]
10036ce98: a9045ff8    	stp	x24, x23, [sp, #0x40]
10036ce9c: a90557f6    	stp	x22, x21, [sp, #0x50]
10036cea0: a9064ff4    	stp	x20, x19, [sp, #0x60]
10036cea4: a9077bfd    	stp	x29, x30, [sp, #0x70]
10036cea8: 9101c3fd    	add	x29, sp, #0x70
10036ceac: aa0403f3    	mov	x19, x4
10036ceb0: aa0303f4    	mov	x20, x3
10036ceb4: aa0203f5    	mov	x21, x2
10036ceb8: aa0103f6    	mov	x22, x1
10036cebc: aa0003f7    	mov	x23, x0
10036cec0: 91026000    	add	x0, x0, #0x98
10036cec4: 940d603d    	bl	0x1006c4fb8 <__ZN4core4hash11BuildHasher8hash_one17h61e5c885b8041446E>
10036cec8: d2800009    	mov	x9, #0x0                ; =0
10036cecc: d379fc08    	lsr	x8, x0, #57
10036ced0: a947e6f8    	ldp	x24, x25, [x23, #0x78]
10036ced4: 0e010d08    	dup.8b	v8, w8
10036ced8: d101231a    	sub	x26, x24, #0x48
10036cedc: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
10036cee0: 5280090a    	mov	w10, #0x48              ; =72
10036cee4: 8a19001c    	and	x28, x0, x25
10036cee8: fc7c6b09    	ldr	d9, [x24, x28]
10036ceec: 2e288d20    	cmeq.8b	v0, v9, v8
10036cef0: 9e660008    	fmov	x8, d0
10036cef4: f201c11b    	ands	x27, x8, #0x8080808080808080
10036cef8: 540002c0    	b.eq	0x10036cf50 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0xd0>
10036cefc: f90007e9    	str	x9, [sp, #0x8]
10036cf00: dac00368    	rbit	x8, x27
10036cf04: dac01108    	clz	x8, x8
10036cf08: 8b480f88    	add	x8, x28, x8, lsr #3
10036cf0c: 8a190117    	and	x23, x8, x25
10036cf10: 9b0afee8    	mneg	x8, x23, x10
10036cf14: 8b080348    	add	x8, x26, x8
10036cf18: f9400909    	ldr	x9, [x8, #0x10]
10036cf1c: eb0902bf    	cmp	x21, x9
10036cf20: 540000c1    	b.ne	0x10036cf38 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0xb8>
10036cf24: f9400501    	ldr	x1, [x8, #0x8]
10036cf28: aa1603e0    	mov	x0, x22
10036cf2c: aa1503e2    	mov	x2, x21
10036cf30: 943991dc    	bl	0x1011d16a0 <_writev+0x1011d16a0>
10036cf34: 34000280    	cbz	w0, 0x10036cf84 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x104>
10036cf38: d1000768    	sub	x8, x27, #0x1
10036cf3c: ea1b011b    	ands	x27, x8, x27
10036cf40: f94007e9    	ldr	x9, [sp, #0x8]
10036cf44: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
10036cf48: 5280090a    	mov	w10, #0x48              ; =72
10036cf4c: 54fffda1    	b.ne	0x10036cf00 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x80>
10036cf50: 2e218d20    	cmeq.8b	v0, v9, v1
10036cf54: 2e30a800    	umaxv.8b	b0, v0
10036cf58: 1e260008    	fmov	w8, s0
10036cf5c: 370007e8    	tbnz	w8, #0x0, 0x10036d058 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x1d8>
10036cf60: 91002129    	add	x9, x9, #0x8
10036cf64: 8b090380    	add	x0, x28, x9
10036cf68: 8a19001c    	and	x28, x0, x25
10036cf6c: fc7c6b09    	ldr	d9, [x24, x28]
10036cf70: 2e288d20    	cmeq.8b	v0, v9, v8
10036cf74: 9e660008    	fmov	x8, d0
10036cf78: f201c11b    	ands	x27, x8, #0x8080808080808080
10036cf7c: 54fffc01    	b.ne	0x10036cefc <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x7c>
10036cf80: 17fffff4    	b	0x10036cf50 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0xd0>
10036cf84: cb1703e8    	neg	x8, x23
10036cf88: 52800909    	mov	w9, #0x48               ; =72
10036cf8c: 9b096117    	madd	x23, x8, x9, x24
10036cf90: f85e82e8    	ldur	x8, [x23, #-0x18]
10036cf94: b4000628    	cbz	x8, 0x10036d058 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x1d8>
10036cf98: d10042e0    	sub	x0, x23, #0x10
10036cf9c: aa1403e1    	mov	x1, x20
10036cfa0: aa1303e2    	mov	x2, x19
10036cfa4: 940d6005    	bl	0x1006c4fb8 <__ZN4core4hash11BuildHasher8hash_one17h61e5c885b8041446E>
10036cfa8: d2800015    	mov	x21, #0x0               ; =0
10036cfac: d379fc08    	lsr	x8, x0, #57
10036cfb0: a97d5af7    	ldp	x23, x22, [x23, #-0x30]
10036cfb4: 0e010d08    	dup.8b	v8, w8
10036cfb8: d10062f8    	sub	x24, x23, #0x18
10036cfbc: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
10036cfc0: 52800319    	mov	w25, #0x18              ; =24
10036cfc4: 8a16001a    	and	x26, x0, x22
10036cfc8: fc7a6ae9    	ldr	d9, [x23, x26]
10036cfcc: 2e288d20    	cmeq.8b	v0, v9, v8
10036cfd0: 9e660008    	fmov	x8, d0
10036cfd4: f201c11b    	ands	x27, x8, #0x8080808080808080
10036cfd8: 54000260    	b.eq	0x10036d024 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x1a4>
10036cfdc: dac00368    	rbit	x8, x27
10036cfe0: dac01108    	clz	x8, x8
10036cfe4: 8b480f48    	add	x8, x26, x8, lsr #3
10036cfe8: 8a160108    	and	x8, x8, x22
10036cfec: 9b19fd08    	mneg	x8, x8, x25
10036cff0: 8b080308    	add	x8, x24, x8
10036cff4: f9400909    	ldr	x9, [x8, #0x10]
10036cff8: eb09027f    	cmp	x19, x9
10036cffc: 540000c1    	b.ne	0x10036d014 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x194>
10036d000: f9400501    	ldr	x1, [x8, #0x8]
10036d004: aa1403e0    	mov	x0, x20
10036d008: aa1303e2    	mov	x2, x19
10036d00c: 943991a5    	bl	0x1011d16a0 <_writev+0x1011d16a0>
10036d010: 340002c0    	cbz	w0, 0x10036d068 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x1e8>
10036d014: d1000768    	sub	x8, x27, #0x1
10036d018: ea1b011b    	ands	x27, x8, x27
10036d01c: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
10036d020: 54fffde1    	b.ne	0x10036cfdc <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x15c>
10036d024: 2e218d20    	cmeq.8b	v0, v9, v1
10036d028: 2e30a800    	umaxv.8b	b0, v0
10036d02c: 1e260008    	fmov	w8, s0
10036d030: 37000148    	tbnz	w8, #0x0, 0x10036d058 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x1d8>
10036d034: 910022b5    	add	x21, x21, #0x8
10036d038: 8b150340    	add	x0, x26, x21
10036d03c: 8a16001a    	and	x26, x0, x22
10036d040: fc7a6ae9    	ldr	d9, [x23, x26]
10036d044: 2e288d20    	cmeq.8b	v0, v9, v8
10036d048: 9e660008    	fmov	x8, d0
10036d04c: f201c11b    	ands	x27, x8, #0x8080808080808080
10036d050: 54fffc61    	b.ne	0x10036cfdc <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x15c>
10036d054: 17fffff4    	b	0x10036d024 <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x1a4>
10036d058: 52800000    	mov	w0, #0x0                ; =0
10036d05c: 14000004    	b	0x10036d06c <__ZN13codexmate_lib4core5relay13codex_catalog24NativeThreadModelCatalog18supports_reasoning17h6af12c78aa2d7c57E+0x1ec>
10036d060: 52800000    	mov	w0, #0x0                ; =0
10036d064: d65f03c0    	ret
10036d068: 52800020    	mov	w0, #0x1                ; =1
10036d06c: a9477bfd    	ldp	x29, x30, [sp, #0x70]
10036d070: a9464ff4    	ldp	x20, x19, [sp, #0x60]
10036d074: a94557f6    	ldp	x22, x21, [sp, #0x50]
10036d078: a9445ff8    	ldp	x24, x23, [sp, #0x40]
10036d07c: a94367fa    	ldp	x26, x25, [sp, #0x30]
10036d080: a9426ffc    	ldp	x28, x27, [sp, #0x20]
10036d084: 6d4123e9    	ldp	d9, d8, [sp, #0x10]
10036d088: 910203ff    	add	sp, sp, #0x80
10036d08c: d65f03c0    	ret
