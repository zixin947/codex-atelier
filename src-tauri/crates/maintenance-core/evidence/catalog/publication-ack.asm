
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008a4e78 <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E>:
1008a4e78: d102c3ff    	sub	sp, sp, #0xb0
1008a4e7c: a90857f6    	stp	x22, x21, [sp, #0x80]
1008a4e80: a9094ff4    	stp	x20, x19, [sp, #0x90]
1008a4e84: a90a7bfd    	stp	x29, x30, [sp, #0xa0]
1008a4e88: 910283fd    	add	x29, sp, #0xa0
1008a4e8c: b4000720    	cbz	x0, 0x1008a4f70 <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E+0xf8>
1008a4e90: aa0003f3    	mov	x19, x0
1008a4e94: d0008268    	adrp	x8, 0x1018f2000 <__ZN13codexmate_lib13EXIT_APPROVED17h1f9ee2eac97195fdE.0>
1008a4e98: 910fe108    	add	x8, x8, #0x3f8
1008a4e9c: f9400108    	ldr	x8, [x8]
1008a4ea0: f100051f    	cmp	x8, #0x1
1008a4ea4: 54000429    	b.ls	0x1008a4f28 <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E+0xb0>
1008a4ea8: aa1303e0    	mov	x0, x19
1008a4eac: 94013346    	bl	0x1008f1bc4 <__ZN3std2io5error5Error4kind17h247456c019dc3d6bE>
1008a4eb0: 12001c08    	and	w8, w0, #0xff
1008a4eb4: 39001fe8    	strb	w8, [sp, #0x7]
1008a4eb8: 91001fe8    	add	x8, sp, #0x7
1008a4ebc: b0000369    	adrp	x9, 0x100911000 <__ZN5rayon4iter8plumbing6Folder12consume_iter17h4212d054bfcf564eE+0xc0>
1008a4ec0: 910f0129    	add	x9, x9, #0x3c0
1008a4ec4: a900a7e8    	stp	x8, x9, [sp, #0x8]
1008a4ec8: 90005888    	adrp	x8, 0x1013b4000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.331+0x4>
1008a4ecc: 911ccd08    	add	x8, x8, #0x733
1008a4ed0: 52800509    	mov	w9, #0x28               ; =40
1008a4ed4: f000644a    	adrp	x10, 0x10152f000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.17+0x520>
1008a4ed8: 9116794a    	add	x10, x10, #0x59e
1008a4edc: a90527e8    	stp	x8, x9, [sp, #0x50]
1008a4ee0: 910023e9    	add	x9, sp, #0x8
1008a4ee4: a906a7ea    	stp	x10, x9, [sp, #0x68]
1008a4ee8: a901a3ff    	stp	xzr, x8, [sp, #0x18]
1008a4eec: d00052e8    	adrp	x8, 0x101302000 <_writev+0x101302000>
1008a4ef0: 3dc02d00    	ldr	q0, [x8, #0xb0]
1008a4ef4: 3c8283e0    	stur	q0, [sp, #0x28]
1008a4ef8: f0006448    	adrp	x8, 0x10152f000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.17+0x520>
1008a4efc: 9117b508    	add	x8, x8, #0x5ed
1008a4f00: f9001fe8    	str	x8, [sp, #0x38]
1008a4f04: f0005808    	adrp	x8, 0x1013a7000 <_anon.1f4a05fd1dc96b9982818105670d4dce.1216+0x287f>
1008a4f08: 3dc3bd00    	ldr	q0, [x8, #0xef0]
1008a4f0c: 3c8403e0    	stur	q0, [sp, #0x40]
1008a4f10: d0005868    	adrp	x8, 0x1013b2000 <_anon.973b090bee24c06fdb949486dbea4242.731+0x14a>
1008a4f14: fd472900    	ldr	d0, [x8, #0xe50]
1008a4f18: fd0033e0    	str	d0, [sp, #0x60]
1008a4f1c: d10087a0    	sub	x0, x29, #0x21
1008a4f20: 910063e1    	add	x1, sp, #0x18
1008a4f24: 941cbf5f    	bl	0x100fd4ca0 <__ZN61_$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$3log17he2f3dc859ac0f3c8E>
1008a4f28: 92400668    	and	x8, x19, #0x3
1008a4f2c: f100051f    	cmp	x8, #0x1
1008a4f30: 54000201    	b.ne	0x1008a4f70 <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E+0xf8>
1008a4f34: f85ffe75    	ldr	x21, [x19, #-0x1]!
1008a4f38: f9400674    	ldr	x20, [x19, #0x8]
1008a4f3c: f9400288    	ldr	x8, [x20]
1008a4f40: b4000068    	cbz	x8, 0x1008a4f4c <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E+0xd4>
1008a4f44: aa1503e0    	mov	x0, x21
1008a4f48: d63f0100    	blr	x8
1008a4f4c: f9400681    	ldr	x1, [x20, #0x8]
1008a4f50: b4000081    	cbz	x1, 0x1008a4f60 <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E+0xe8>
1008a4f54: f9400a82    	ldr	x2, [x20, #0x10]
1008a4f58: aa1503e0    	mov	x0, x21
1008a4f5c: 97dd7183    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a4f60: aa1303e0    	mov	x0, x19
1008a4f64: 52800301    	mov	w1, #0x18               ; =24
1008a4f68: 52800102    	mov	w2, #0x8                ; =8
1008a4f6c: 97dd717f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a4f70: a94a7bfd    	ldp	x29, x30, [sp, #0xa0]
1008a4f74: a9494ff4    	ldp	x20, x19, [sp, #0x90]
1008a4f78: a94857f6    	ldp	x22, x21, [sp, #0x80]
1008a4f7c: 9102c3ff    	add	sp, sp, #0xb0
1008a4f80: d65f03c0    	ret
1008a4f84: aa1403e9    	mov	x9, x20
1008a4f88: aa0003f4    	mov	x20, x0
1008a4f8c: f9400521    	ldr	x1, [x9, #0x8]
1008a4f90: b4000081    	cbz	x1, 0x1008a4fa0 <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E+0x128>
1008a4f94: f9400922    	ldr	x2, [x9, #0x10]
1008a4f98: aa1503e0    	mov	x0, x21
1008a4f9c: 97dd7173    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a4fa0: aa1303e0    	mov	x0, x19
1008a4fa4: 52800301    	mov	w1, #0x18               ; =24
1008a4fa8: 52800102    	mov	w2, #0x8                ; =8
1008a4fac: 97dd716f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a4fb0: aa1403e0    	mov	x0, x20
1008a4fb4: 9424b068    	bl	0x1011d1154 <_writev+0x1011d1154>
1008a4fb8: aa0003f4    	mov	x20, x0
1008a4fbc: aa1303e0    	mov	x0, x19
1008a4fc0: 94014589    	bl	0x1008f65e4 <__ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h8fc6649f423239fcE>
1008a4fc4: aa1403e0    	mov	x0, x20
1008a4fc8: 9424b063    	bl	0x1011d1154 <_writev+0x1011d1154>
1008a4fcc: 9424aedf    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

00000001008a4fd0 <__ZN13codexmate_lib4core5relay12atomic_write11Publication12require_sync17h0a26a66998b207caE>:
1008a4fd0: a9bd57f6    	stp	x22, x21, [sp, #-0x30]!
1008a4fd4: a9014ff4    	stp	x20, x19, [sp, #0x10]
1008a4fd8: a9027bfd    	stp	x29, x30, [sp, #0x20]
1008a4fdc: 910083fd    	add	x29, sp, #0x20
1008a4fe0: b4000380    	cbz	x0, 0x1008a5050 <__ZN13codexmate_lib4core5relay12atomic_write11Publication12require_sync17h0a26a66998b207caE+0x80>
1008a4fe4: aa0003f3    	mov	x19, x0
1008a4fe8: 940132f7    	bl	0x1008f1bc4 <__ZN3std2io5error5Error4kind17h247456c019dc3d6bE>
1008a4fec: 90005881    	adrp	x1, 0x1013b4000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.331+0x4>
1008a4ff0: 911d6c21    	add	x1, x1, #0x75b
1008a4ff4: 12001c00    	and	w0, w0, #0xff
1008a4ff8: 52800a62    	mov	w2, #0x53               ; =83
1008a4ffc: 94227aaf    	bl	0x101143ab8 <__ZN3std2io5error5Error3new17h311cf880a0f7d574E>
1008a5000: aa0003f4    	mov	x20, x0
1008a5004: 92400668    	and	x8, x19, #0x3
1008a5008: f100051f    	cmp	x8, #0x1
1008a500c: 54000241    	b.ne	0x1008a5054 <__ZN13codexmate_lib4core5relay12atomic_write11Publication12require_sync17h0a26a66998b207caE+0x84>
1008a5010: f85ffe75    	ldr	x21, [x19, #-0x1]!
1008a5014: f9400676    	ldr	x22, [x19, #0x8]
1008a5018: f94002c8    	ldr	x8, [x22]
1008a501c: b4000068    	cbz	x8, 0x1008a5028 <__ZN13codexmate_lib4core5relay12atomic_write11Publication12require_sync17h0a26a66998b207caE+0x58>
1008a5020: aa1503e0    	mov	x0, x21
1008a5024: d63f0100    	blr	x8
1008a5028: f94006c1    	ldr	x1, [x22, #0x8]
1008a502c: b4000081    	cbz	x1, 0x1008a503c <__ZN13codexmate_lib4core5relay12atomic_write11Publication12require_sync17h0a26a66998b207caE+0x6c>
1008a5030: f9400ac2    	ldr	x2, [x22, #0x10]
1008a5034: aa1503e0    	mov	x0, x21
1008a5038: 97dd714c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a503c: aa1303e0    	mov	x0, x19
1008a5040: 52800301    	mov	w1, #0x18               ; =24
1008a5044: 52800102    	mov	w2, #0x8                ; =8
1008a5048: 97dd7148    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a504c: 14000002    	b	0x1008a5054 <__ZN13codexmate_lib4core5relay12atomic_write11Publication12require_sync17h0a26a66998b207caE+0x84>
1008a5050: d2800014    	mov	x20, #0x0               ; =0
1008a5054: aa1403e0    	mov	x0, x20
1008a5058: a9427bfd    	ldp	x29, x30, [sp, #0x20]
1008a505c: a9414ff4    	ldp	x20, x19, [sp, #0x10]
1008a5060: a8c357f6    	ldp	x22, x21, [sp], #0x30
1008a5064: d65f03c0    	ret
1008a5068: aa0003f4    	mov	x20, x0
1008a506c: f94006c1    	ldr	x1, [x22, #0x8]
1008a5070: b4000081    	cbz	x1, 0x1008a5080 <__ZN13codexmate_lib4core5relay12atomic_write11Publication12require_sync17h0a26a66998b207caE+0xb0>
1008a5074: f9400ac2    	ldr	x2, [x22, #0x10]
1008a5078: aa1503e0    	mov	x0, x21
1008a507c: 97dd713b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a5080: aa1303e0    	mov	x0, x19
1008a5084: 52800301    	mov	w1, #0x18               ; =24
1008a5088: 52800102    	mov	w2, #0x8                ; =8
1008a508c: 97dd7137    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a5090: aa1403e0    	mov	x0, x20
1008a5094: 9424b030    	bl	0x1011d1154 <_writev+0x1011d1154>
1008a5098: aa0003f4    	mov	x20, x0
1008a509c: aa1303e0    	mov	x0, x19
1008a50a0: 94014551    	bl	0x1008f65e4 <__ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h8fc6649f423239fcE>
1008a50a4: aa1403e0    	mov	x0, x20
1008a50a8: 9424b02b    	bl	0x1011d1154 <_writev+0x1011d1154>
1008a50ac: 9424aea7    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

00000001008a50b0 <__ZN13codexmate_lib4core5relay12atomic_write14temp_file_stem17hf7fae2a0e5e52bdeE>:
1008a50b0: d10143ff    	sub	sp, sp, #0x50
1008a50b4: a9034ff4    	stp	x20, x19, [sp, #0x30]
1008a50b8: a9047bfd    	stp	x29, x30, [sp, #0x40]
1008a50bc: 910103fd    	add	x29, sp, #0x40
1008a50c0: aa0803f3    	mov	x19, x8
1008a50c4: 9422a9f9    	bl	0x10114f8a8 <__ZN3std4path4Path9file_name17h10722b1b995fceadE>
1008a50c8: f0005874    	adrp	x20, 0x1013b4000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.331+0x4>
1008a50cc: 911eba94    	add	x20, x20, #0x7ae
1008a50d0: b4000140    	cbz	x0, 0x1008a50f8 <__ZN13codexmate_lib4core5relay12atomic_write14temp_file_stem17hf7fae2a0e5e52bdeE+0x48>
1008a50d4: 910063e8    	add	x8, sp, #0x18
1008a50d8: 9423841e    	bl	0x101186150 <__ZN4core3str8converts9from_utf817h3d3f04c61e7c2c97E>
1008a50dc: b9401be8    	ldr	w8, [sp, #0x18]
1008a50e0: a9422be9    	ldp	x9, x10, [sp, #0x20]
1008a50e4: 7100011f    	cmp	w8, #0x0
1008a50e8: 528000c8    	mov	w8, #0x6                ; =6
1008a50ec: 9a8a1108    	csel	x8, x8, x10, ne
1008a50f0: 9a891294    	csel	x20, x20, x9, ne
1008a50f4: 14000002    	b	0x1008a50fc <__ZN13codexmate_lib4core5relay12atomic_write14temp_file_stem17hf7fae2a0e5e52bdeE+0x4c>
1008a50f8: 528000c8    	mov	w8, #0x6                ; =6
1008a50fc: 8b080288    	add	x8, x20, x8
1008a5100: a901a3f4    	stp	x20, x8, [sp, #0x18]
1008a5104: 52800608    	mov	w8, #0x30               ; =48
1008a5108: f90017e8    	str	x8, [sp, #0x28]
1008a510c: 910003e8    	mov	x8, sp
1008a5110: 910063e0    	add	x0, sp, #0x18
1008a5114: 97ff45df    	bl	0x100876890 <__ZN95_$LT$alloc..string..String$u20$as$u20$core..iter..traits..collect..FromIterator$LT$char$GT$$GT$9from_iter17h7a4dd759d508edb1E>
1008a5118: f9400be8    	ldr	x8, [sp, #0x10]
1008a511c: b40000c8    	cbz	x8, 0x1008a5134 <__ZN13codexmate_lib4core5relay12atomic_write14temp_file_stem17hf7fae2a0e5e52bdeE+0x84>
1008a5120: 3dc003e0    	ldr	q0, [sp]
1008a5124: 3d800260    	str	q0, [x19]
1008a5128: f9400be8    	ldr	x8, [sp, #0x10]
1008a512c: f9000a68    	str	x8, [x19, #0x10]
1008a5130: 14000013    	b	0x1008a517c <__ZN13codexmate_lib4core5relay12atomic_write14temp_file_stem17hf7fae2a0e5e52bdeE+0xcc>
1008a5134: 97dd7110    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008a5138: 528000c0    	mov	w0, #0x6                ; =6
1008a513c: 52800021    	mov	w1, #0x1                ; =1
1008a5140: 97dd7109    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008a5144: b4000240    	cbz	x0, 0x1008a518c <__ZN13codexmate_lib4core5relay12atomic_write14temp_file_stem17hf7fae2a0e5e52bdeE+0xdc>
1008a5148: 528d2da8    	mov	w8, #0x696d             ; =26989
1008a514c: 79000808    	strh	w8, [x0, #0x4]
1008a5150: 528d2c28    	mov	w8, #0x6961             ; =26977
1008a5154: 72ac2da8    	movk	w8, #0x616d, lsl #16
1008a5158: b9000008    	str	w8, [x0]
1008a515c: 528000c8    	mov	w8, #0x6                ; =6
1008a5160: a9000268    	stp	x8, x0, [x19]
1008a5164: f9000a68    	str	x8, [x19, #0x10]
1008a5168: f94003e1    	ldr	x1, [sp]
1008a516c: b4000081    	cbz	x1, 0x1008a517c <__ZN13codexmate_lib4core5relay12atomic_write14temp_file_stem17hf7fae2a0e5e52bdeE+0xcc>
1008a5170: f94007e0    	ldr	x0, [sp, #0x8]
1008a5174: 52800022    	mov	w2, #0x1                ; =1
1008a5178: 97dd70fc    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a517c: a9447bfd    	ldp	x29, x30, [sp, #0x40]
1008a5180: a9434ff4    	ldp	x20, x19, [sp, #0x30]
1008a5184: 910143ff    	add	sp, sp, #0x50
1008a5188: d65f03c0    	ret
1008a518c: 52800020    	mov	w0, #0x1                ; =1
1008a5190: 528000c1    	mov	w1, #0x6                ; =6
1008a5194: 9424ad34    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1008a5198: d4200020    	brk	#0x1
1008a519c: aa0003f3    	mov	x19, x0
1008a51a0: f94003e1    	ldr	x1, [sp]
1008a51a4: b4000081    	cbz	x1, 0x1008a51b4 <__ZN13codexmate_lib4core5relay12atomic_write14temp_file_stem17hf7fae2a0e5e52bdeE+0x104>
1008a51a8: f94007e0    	ldr	x0, [sp, #0x8]
1008a51ac: 52800022    	mov	w2, #0x1                ; =1
1008a51b0: 97dd70ee    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a51b4: aa1303e0    	mov	x0, x19
1008a51b8: 9424afe7    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001008a51bc <__ZN13codexmate_lib4core5relay12atomic_write15sync_parent_dir17h238af3fb419bbc9dE>:
1008a51bc: d10103ff    	sub	sp, sp, #0x40
1008a51c0: a9024ff4    	stp	x20, x19, [sp, #0x20]
1008a51c4: a9037bfd    	stp	x29, x30, [sp, #0x30]
1008a51c8: 9100c3fd    	add	x29, sp, #0x30
1008a51cc: aa0103e2    	mov	x2, x1
1008a51d0: aa0003e1    	mov	x1, x0
1008a51d4: b90017ff    	str	wzr, [sp, #0x14]
1008a51d8: 528036c8    	mov	w8, #0x1b6              ; =438
1008a51dc: 790033e8    	strh	w8, [sp, #0x18]
1008a51e0: b801a3ff    	stur	wzr, [sp, #0x1a]
1008a51e4: 79003fff    	strh	wzr, [sp, #0x1e]
1008a51e8: 52800028    	mov	w8, #0x1                ; =1
1008a51ec: 39006be8    	strb	w8, [sp, #0x1a]
1008a51f0: 910003e8    	mov	x8, sp
1008a51f4: 910053e0    	add	x0, sp, #0x14
1008a51f8: 9422737a    	bl	0x101141fe0 <__ZN3std2fs11OpenOptions5_open17hfd14fc0134faefc7E>
1008a51fc: b94003e8    	ldr	w8, [sp]
1008a5200: 7100051f    	cmp	w8, #0x1
1008a5204: 54000061    	b.ne	0x1008a5210 <__ZN13codexmate_lib4core5relay12atomic_write15sync_parent_dir17h238af3fb419bbc9dE+0x54>
1008a5208: f94007e0    	ldr	x0, [sp, #0x8]
1008a520c: 14000009    	b	0x1008a5230 <__ZN13codexmate_lib4core5relay12atomic_write15sync_parent_dir17h238af3fb419bbc9dE+0x74>
1008a5210: b94007f3    	ldr	w19, [sp, #0x4]
1008a5214: b90003f3    	str	w19, [sp]
1008a5218: 910003e0    	mov	x0, sp
1008a521c: 942274ab    	bl	0x1011424c8 <__ZN3std2fs4File9sync_data17h3f6e029ce9c67cbfE>
1008a5220: aa0003f4    	mov	x20, x0
1008a5224: aa1303e0    	mov	x0, x19
1008a5228: 9424b031    	bl	0x1011d12ec <_writev+0x1011d12ec>
1008a522c: aa1403e0    	mov	x0, x20
1008a5230: a9437bfd    	ldp	x29, x30, [sp, #0x30]
1008a5234: a9424ff4    	ldp	x20, x19, [sp, #0x20]
1008a5238: 910103ff    	add	sp, sp, #0x40
1008a523c: d65f03c0    	ret
1008a5240: aa0003f4    	mov	x20, x0
1008a5244: aa1303e0    	mov	x0, x19
1008a5248: 9424b029    	bl	0x1011d12ec <_writev+0x1011d12ec>
1008a524c: aa1403e0    	mov	x0, x20
1008a5250: 9424afc1    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001008a5254 <__ZN13codexmate_lib4core5relay12atomic_write19copy_atomic_private17hfd768187944c8944E>:
1008a5254: d10143ff    	sub	sp, sp, #0x50
1008a5258: a90257f6    	stp	x22, x21, [sp, #0x20]
1008a525c: a9034ff4    	stp	x20, x19, [sp, #0x30]
1008a5260: a9047bfd    	stp	x29, x30, [sp, #0x40]
1008a5264: 910103fd    	add	x29, sp, #0x40
1008a5268: aa0303f5    	mov	x21, x3
1008a526c: aa0203f6    	mov	x22, x2
1008a5270: 910023e8    	add	x8, sp, #0x8
1008a5274: 942274f6    	bl	0x10114264c <__ZN3std2fs4read5inner17hebd695181b114834E>
1008a5278: a940cff4    	ldp	x20, x19, [sp, #0x8]
1008a527c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008a5280: eb08029f    	cmp	x20, x8
1008a5284: 54000220    	b.eq	0x1008a52c8 <__ZN13codexmate_lib4core5relay12atomic_write19copy_atomic_private17hfd768187944c8944E+0x74>
1008a5288: f9400fe3    	ldr	x3, [sp, #0x18]
1008a528c: aa1603e0    	mov	x0, x22
1008a5290: aa1503e1    	mov	x1, x21
1008a5294: aa1303e2    	mov	x2, x19
1008a5298: 940006aa    	bl	0x1008a6d40 <__ZN13codexmate_lib4core5relay12atomic_write27write_atomic_private_report17h5c130d66b7fb3334E>
1008a529c: aa0103f5    	mov	x21, x1
1008a52a0: 37000080    	tbnz	w0, #0x0, 0x1008a52b0 <__ZN13codexmate_lib4core5relay12atomic_write19copy_atomic_private17hfd768187944c8944E+0x5c>
1008a52a4: aa1503e0    	mov	x0, x21
1008a52a8: 97fffef4    	bl	0x1008a4e78 <__ZN13codexmate_lib4core5relay12atomic_write11Publication11acknowledge17h8ea1d18ecbbcb2e1E>
1008a52ac: d2800015    	mov	x21, #0x0               ; =0
1008a52b0: b40000b4    	cbz	x20, 0x1008a52c4 <__ZN13codexmate_lib4core5relay12atomic_write19copy_atomic_private17hfd768187944c8944E+0x70>
1008a52b4: aa1303e0    	mov	x0, x19
1008a52b8: aa1403e1    	mov	x1, x20
1008a52bc: 52800022    	mov	w2, #0x1                ; =1
1008a52c0: 97dd70aa    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a52c4: aa1503f3    	mov	x19, x21
1008a52c8: aa1303e0    	mov	x0, x19
1008a52cc: a9447bfd    	ldp	x29, x30, [sp, #0x40]
1008a52d0: a9434ff4    	ldp	x20, x19, [sp, #0x30]
1008a52d4: a94257f6    	ldp	x22, x21, [sp, #0x20]
1008a52d8: 910143ff    	add	sp, sp, #0x50
1008a52dc: d65f03c0    	ret
1008a52e0: aa0003f5    	mov	x21, x0
1008a52e4: b40000b4    	cbz	x20, 0x1008a52f8 <__ZN13codexmate_lib4core5relay12atomic_write19copy_atomic_private17hfd768187944c8944E+0xa4>
1008a52e8: aa1303e0    	mov	x0, x19
1008a52ec: aa1403e1    	mov	x1, x20
1008a52f0: 52800022    	mov	w2, #0x1                ; =1
1008a52f4: 97dd709d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a52f8: aa1503e0    	mov	x0, x21
1008a52fc: 9424af96    	bl	0x1011d1154 <_writev+0x1011d1154>
