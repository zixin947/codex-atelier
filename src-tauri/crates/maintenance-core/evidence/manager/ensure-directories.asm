
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008ee11c <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E>:
1008ee11c: d10103ff    	sub	sp, sp, #0x40
1008ee120: a90157f6    	stp	x22, x21, [sp, #0x10]
1008ee124: a9024ff4    	stp	x20, x19, [sp, #0x20]
1008ee128: a9037bfd    	stp	x29, x30, [sp, #0x30]
1008ee12c: 9100c3fd    	add	x29, sp, #0x30
1008ee130: aa0003f3    	mov	x19, x0
1008ee134: 52803ff4    	mov	w20, #0x1ff             ; =511
1008ee138: 79001bf4    	strh	w20, [sp, #0xc]
1008ee13c: 52800035    	mov	w21, #0x1               ; =1
1008ee140: 39003bf5    	strb	w21, [sp, #0xe]
1008ee144: a94c8801    	ldp	x1, x2, [x0, #0xc8]
1008ee148: 910033e0    	add	x0, sp, #0xc
1008ee14c: 94214dab    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008ee150: b50006a0    	cbnz	x0, 0x1008ee224 <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E+0x108>
1008ee154: 79001bf4    	strh	w20, [sp, #0xc]
1008ee158: 39003bf5    	strb	w21, [sp, #0xe]
1008ee15c: a94f8a61    	ldp	x1, x2, [x19, #0xf8]
1008ee160: 910033e0    	add	x0, sp, #0xc
1008ee164: 94214da5    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008ee168: b50005e0    	cbnz	x0, 0x1008ee224 <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E+0x108>
1008ee16c: 79001bf4    	strh	w20, [sp, #0xc]
1008ee170: 39003bf5    	strb	w21, [sp, #0xe]
1008ee174: a9510a61    	ldp	x1, x2, [x19, #0x110]
1008ee178: 910033e0    	add	x0, sp, #0xc
1008ee17c: 94214d9f    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008ee180: b5000520    	cbnz	x0, 0x1008ee224 <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E+0x108>
1008ee184: 79001bf4    	strh	w20, [sp, #0xc]
1008ee188: 39003bf5    	strb	w21, [sp, #0xe]
1008ee18c: a9528a61    	ldp	x1, x2, [x19, #0x128]
1008ee190: 910033e0    	add	x0, sp, #0xc
1008ee194: 94214d99    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008ee198: b5000460    	cbnz	x0, 0x1008ee224 <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E+0x108>
1008ee19c: 79001bf4    	strh	w20, [sp, #0xc]
1008ee1a0: 39003bf5    	strb	w21, [sp, #0xe]
1008ee1a4: a9558a61    	ldp	x1, x2, [x19, #0x158]
1008ee1a8: 910033e0    	add	x0, sp, #0xc
1008ee1ac: 94214d93    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008ee1b0: b50003a0    	cbnz	x0, 0x1008ee224 <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E+0x108>
1008ee1b4: 79001bf4    	strh	w20, [sp, #0xc]
1008ee1b8: 39003bf5    	strb	w21, [sp, #0xe]
1008ee1bc: a9570a61    	ldp	x1, x2, [x19, #0x170]
1008ee1c0: 910033e0    	add	x0, sp, #0xc
1008ee1c4: 94214d8d    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008ee1c8: b50002e0    	cbnz	x0, 0x1008ee224 <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E+0x108>
1008ee1cc: 79001bf4    	strh	w20, [sp, #0xc]
1008ee1d0: 39003bf5    	strb	w21, [sp, #0xe]
1008ee1d4: f9415661    	ldr	x1, [x19, #0x2a8]
1008ee1d8: f9415a62    	ldr	x2, [x19, #0x2b0]
1008ee1dc: 910033e0    	add	x0, sp, #0xc
1008ee1e0: 94214d86    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008ee1e4: b5000200    	cbnz	x0, 0x1008ee224 <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E+0x108>
1008ee1e8: 79001bf4    	strh	w20, [sp, #0xc]
1008ee1ec: 39003bf5    	strb	w21, [sp, #0xe]
1008ee1f0: f9416261    	ldr	x1, [x19, #0x2c0]
1008ee1f4: f9416662    	ldr	x2, [x19, #0x2c8]
1008ee1f8: 910033e0    	add	x0, sp, #0xc
1008ee1fc: 94214d7f    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008ee200: b5000120    	cbnz	x0, 0x1008ee224 <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E+0x108>
1008ee204: 52803fe8    	mov	w8, #0x1ff              ; =511
1008ee208: 79001be8    	strh	w8, [sp, #0xc]
1008ee20c: 52800028    	mov	w8, #0x1                ; =1
1008ee210: 39003be8    	strb	w8, [sp, #0xe]
1008ee214: f9412661    	ldr	x1, [x19, #0x248]
1008ee218: f9412a62    	ldr	x2, [x19, #0x250]
1008ee21c: 910033e0    	add	x0, sp, #0xc
1008ee220: 94214d76    	bl	0x1011417f8 <__ZN3std2fs10DirBuilder7_create17h8aa79a64ba94c04eE>
1008ee224: a9437bfd    	ldp	x29, x30, [sp, #0x30]
1008ee228: a9424ff4    	ldp	x20, x19, [sp, #0x20]
1008ee22c: a94157f6    	ldp	x22, x21, [sp, #0x10]
1008ee230: 910103ff    	add	sp, sp, #0x40
1008ee234: d65f03c0    	ret
