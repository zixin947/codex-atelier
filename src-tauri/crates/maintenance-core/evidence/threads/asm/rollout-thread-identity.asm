
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008ba190 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E>:
1008ba190: d102c3ff    	sub	sp, sp, #0xb0
1008ba194: a90857f6    	stp	x22, x21, [sp, #0x80]
1008ba198: a9094ff4    	stp	x20, x19, [sp, #0x90]
1008ba19c: a90a7bfd    	stp	x29, x30, [sp, #0xa0]
1008ba1a0: 910283fd    	add	x29, sp, #0xa0
1008ba1a4: aa0303f5    	mov	x21, x3
1008ba1a8: aa0203f6    	mov	x22, x2
1008ba1ac: 910003e8    	mov	x8, sp
1008ba1b0: 97fff4e4    	bl	0x1008b7540 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28find_first_session_meta_line17h773ce37e3d50082cE>
1008ba1b4: f94003e8    	ldr	x8, [sp]
1008ba1b8: f1002d1f    	cmp	x8, #0xb
1008ba1bc: 54000301    	b.ne	0x1008ba21c <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x8c>
1008ba1c0: f94007f3    	ldr	x19, [sp, #0x8]
1008ba1c4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008ba1c8: eb08027f    	cmp	x19, x8
1008ba1cc: 540002c0    	b.eq	0x1008ba224 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x94>
1008ba1d0: a94123f4    	ldp	x20, x8, [sp, #0x10]
1008ba1d4: a90023f4    	stp	x20, x8, [sp]
1008ba1d8: a9017fff    	stp	xzr, xzr, [sp, #0x10]
1008ba1dc: a90223f4    	stp	x20, x8, [sp, #0x20]
1008ba1e0: d10103a8    	sub	x8, x29, #0x40
1008ba1e4: 910003e0    	mov	x0, sp
1008ba1e8: 97e4830b    	bl	0x1001dae14 <__ZN10serde_json2de10from_trait17h472fe9ac40d19421E>
1008ba1ec: 385c03a8    	ldurb	w8, [x29, #-0x40]
1008ba1f0: 7100191f    	cmp	w8, #0x6
1008ba1f4: 54000261    	b.ne	0x1008ba240 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0xb0>
1008ba1f8: d10103a8    	sub	x8, x29, #0x40
1008ba1fc: 91002100    	add	x0, x8, #0x8
1008ba200: 9400f21c    	bl	0x1008f6a70 <__ZN4core3ptr45drop_in_place$LT$serde_json..error..Error$GT$17hc34208687d70e728E>
1008ba204: b4000113    	cbz	x19, 0x1008ba224 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x94>
1008ba208: aa1403e0    	mov	x0, x20
1008ba20c: aa1303e1    	mov	x1, x19
1008ba210: 52800022    	mov	w2, #0x1                ; =1
1008ba214: 97dd1cd5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008ba218: 14000003    	b	0x1008ba224 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x94>
1008ba21c: 910003e0    	mov	x0, sp
1008ba220: 9400f57e    	bl	0x1008f7818 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
1008ba224: 52800015    	mov	w21, #0x0               ; =0
1008ba228: aa1503e0    	mov	x0, x21
1008ba22c: a94a7bfd    	ldp	x29, x30, [sp, #0xa0]
1008ba230: a9494ff4    	ldp	x20, x19, [sp, #0x90]
1008ba234: a94857f6    	ldp	x22, x21, [sp, #0x80]
1008ba238: 9102c3ff    	add	sp, sp, #0xb0
1008ba23c: d65f03c0    	ret
1008ba240: ad7e07a0    	ldp	q0, q1, [x29, #-0x40]
1008ba244: ad0007e0    	stp	q0, q1, [sp]
1008ba248: f00057c1    	adrp	x1, 0x1013b5000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.665+0x10b>
1008ba24c: 91115821    	add	x1, x1, #0x456
1008ba250: 910003e0    	mov	x0, sp
1008ba254: 52800162    	mov	w2, #0xb                ; =11
1008ba258: 94212aa5    	bl	0x101104cec <__ZN10serde_json5value5Value7pointer17h84916003af215a14E>
1008ba25c: b40001c0    	cbz	x0, 0x1008ba294 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x104>
1008ba260: 39400008    	ldrb	w8, [x0]
1008ba264: 71000d1f    	cmp	w8, #0x3
1008ba268: 54000161    	b.ne	0x1008ba294 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x104>
1008ba26c: f9400c08    	ldr	x8, [x0, #0x18]
1008ba270: eb15011f    	cmp	x8, x21
1008ba274: 54000101    	b.ne	0x1008ba294 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x104>
1008ba278: f9400800    	ldr	x0, [x0, #0x10]
1008ba27c: aa1603e1    	mov	x1, x22
1008ba280: aa1503e2    	mov	x2, x21
1008ba284: 94245d07    	bl	0x1011d16a0 <_writev+0x1011d16a0>
1008ba288: 7100001f    	cmp	w0, #0x0
1008ba28c: 1a9f17f5    	cset	w21, eq
1008ba290: 14000002    	b	0x1008ba298 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x108>
1008ba294: 52800015    	mov	w21, #0x0               ; =0
1008ba298: 910003e0    	mov	x0, sp
1008ba29c: 9400f22e    	bl	0x1008f6b54 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
1008ba2a0: b4fffc53    	cbz	x19, 0x1008ba228 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x98>
1008ba2a4: aa1403e0    	mov	x0, x20
1008ba2a8: aa1303e1    	mov	x1, x19
1008ba2ac: 52800022    	mov	w2, #0x1                ; =1
1008ba2b0: 97dd1cae    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008ba2b4: 17ffffdd    	b	0x1008ba228 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x98>
1008ba2b8: aa0003f5    	mov	x21, x0
1008ba2bc: 910003e0    	mov	x0, sp
1008ba2c0: 9400f225    	bl	0x1008f6b54 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
1008ba2c4: 14000003    	b	0x1008ba2d0 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x140>
1008ba2c8: 94245a20    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
1008ba2cc: aa0003f5    	mov	x21, x0
1008ba2d0: b40000b3    	cbz	x19, 0x1008ba2e4 <__ZN13codexmate_lib4core5relay23codex_thread_visibility30rollout_file_belongs_to_thread17hbb54e9e5523830c4E+0x154>
1008ba2d4: aa1403e0    	mov	x0, x20
1008ba2d8: aa1303e1    	mov	x1, x19
1008ba2dc: 52800022    	mov	w2, #0x1                ; =1
1008ba2e0: 97dd1ca2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008ba2e4: aa1503e0    	mov	x0, x21
1008ba2e8: 94245b9b    	bl	0x1011d1154 <_writev+0x1011d1154>
