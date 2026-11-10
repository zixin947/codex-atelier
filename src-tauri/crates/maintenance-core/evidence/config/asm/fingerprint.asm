
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010037c1ac <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE>:
10037c1ac: d102c3ff    	sub	sp, sp, #0xb0
10037c1b0: a9075ff8    	stp	x24, x23, [sp, #0x70]
10037c1b4: a90857f6    	stp	x22, x21, [sp, #0x80]
10037c1b8: a9094ff4    	stp	x20, x19, [sp, #0x90]
10037c1bc: a90a7bfd    	stp	x29, x30, [sp, #0xa0]
10037c1c0: 910283fd    	add	x29, sp, #0xa0
10037c1c4: 52800048    	mov	w8, #0x2                ; =2
10037c1c8: 39003fe8    	strb	w8, [sp, #0xf]
10037c1cc: a90107ff    	stp	xzr, x1, [sp, #0x10]
10037c1d0: 910043f7    	add	x23, sp, #0x10
10037c1d4: a90207e0    	stp	x0, x1, [sp, #0x20]
10037c1d8: a90307ff    	stp	xzr, x1, [sp, #0x30]
10037c1dc: 0f000540    	movi.2s	v0, #0xa
10037c1e0: fd0023e0    	str	d0, [sp, #0x40]
10037c1e4: 52800038    	mov	w24, #0x1               ; =1
10037c1e8: 390123f8    	strb	w24, [sp, #0x48]
10037c1ec: 7900a3ff    	strh	wzr, [sp, #0x50]
10037c1f0: 14000008    	b	0x10037c210 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x64>
10037c1f4: aa1503e0    	mov	x0, x21
10037c1f8: aa1603e1    	mov	x1, x22
10037c1fc: 94005d51    	bl	0x100393740 <__ZN13codexmate_lib4core5relay16codex_diagnostic30is_explicit_third_party_marker17h8ca48711281fdd83E>
10037c200: 37000840    	tbnz	w0, #0x0, 0x10037c308 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x15c>
10037c204: 394147e8    	ldrb	w8, [sp, #0x51]
10037c208: 7100051f    	cmp	w8, #0x1
10037c20c: 54000700    	b.eq	0x10037c2ec <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x140>
10037c210: f94013f3    	ldr	x19, [sp, #0x20]
10037c214: d10123a0    	sub	x0, x29, #0x48
10037c218: 910042e1    	add	x1, x23, #0x10
10037c21c: 9401c9c0    	bl	0x1003ee91c <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
10037c220: b85b83a8    	ldur	w8, [x29, #-0x48]
10037c224: 7100051f    	cmp	w8, #0x1
10037c228: 54000101    	b.ne	0x10037c248 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x9c>
10037c22c: f85c83a8    	ldur	x8, [x29, #-0x38]
10037c230: f9400be9    	ldr	x9, [sp, #0x10]
10037c234: cb090114    	sub	x20, x8, x9
10037c238: 8b090273    	add	x19, x19, x9
10037c23c: f9000be8    	str	x8, [sp, #0x10]
10037c240: b50001d4    	cbnz	x20, 0x10037c278 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0xcc>
10037c244: 14000018    	b	0x10037c2a4 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0xf8>
10037c248: 394147e8    	ldrb	w8, [sp, #0x51]
10037c24c: 37000508    	tbnz	w8, #0x0, 0x10037c2ec <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x140>
10037c250: 390147f8    	strb	w24, [sp, #0x51]
10037c254: 394143ea    	ldrb	w10, [sp, #0x50]
10037c258: a94127e8    	ldp	x8, x9, [sp, #0x10]
10037c25c: 3700006a    	tbnz	w10, #0x0, 0x10037c268 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0xbc>
10037c260: eb08013f    	cmp	x9, x8
10037c264: 54000440    	b.eq	0x10037c2ec <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x140>
10037c268: f94013ea    	ldr	x10, [sp, #0x20]
10037c26c: cb080134    	sub	x20, x9, x8
10037c270: 8b080153    	add	x19, x10, x8
10037c274: b4000194    	cbz	x20, 0x10037c2a4 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0xf8>
10037c278: 8b140269    	add	x9, x19, x20
10037c27c: 385ff128    	ldurb	w8, [x9, #-0x1]
10037c280: 7100291f    	cmp	w8, #0xa
10037c284: 54000101    	b.ne	0x10037c2a4 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0xf8>
10037c288: f1000688    	subs	x8, x20, #0x1
10037c28c: 540002c0    	b.eq	0x10037c2e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x138>
10037c290: 385fe129    	ldurb	w9, [x9, #-0x2]
10037c294: d1000a8a    	sub	x10, x20, #0x2
10037c298: 7100353f    	cmp	w9, #0xd
10037c29c: 9a931273    	csel	x19, x19, x19, ne
10037c2a0: 9a8a1114    	csel	x20, x8, x10, ne
10037c2a4: b4000253    	cbz	x19, 0x10037c2ec <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x140>
10037c2a8: aa1303e0    	mov	x0, x19
10037c2ac: aa1403e1    	mov	x1, x20
10037c2b0: 97facbaa    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
10037c2b4: aa0003f5    	mov	x21, x0
10037c2b8: aa0103f6    	mov	x22, x1
10037c2bc: 91003fe0    	add	x0, sp, #0xf
10037c2c0: aa1303e1    	mov	x1, x19
10037c2c4: aa1403e2    	mov	x2, x20
10037c2c8: 941045d5    	bl	0x10078da1c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE>
10037c2cc: 34fff9c0    	cbz	w0, 0x10037c204 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x58>
10037c2d0: b4fff936    	cbz	x22, 0x10037c1f4 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x48>
10037c2d4: 394002a8    	ldrb	w8, [x21]
10037c2d8: 71008d1f    	cmp	w8, #0x23
10037c2dc: 54fff8c1    	b.ne	0x10037c1f4 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x48>
10037c2e0: 17ffffc9    	b	0x10037c204 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x58>
10037c2e4: aa0803f4    	mov	x20, x8
10037c2e8: b5fffe13    	cbnz	x19, 0x10037c2a8 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0xfc>
10037c2ec: 52800000    	mov	w0, #0x0                ; =0
10037c2f0: a94a7bfd    	ldp	x29, x30, [sp, #0xa0]
10037c2f4: a9494ff4    	ldp	x20, x19, [sp, #0x90]
10037c2f8: a94857f6    	ldp	x22, x21, [sp, #0x80]
10037c2fc: a9475ff8    	ldp	x24, x23, [sp, #0x70]
10037c300: 9102c3ff    	add	sp, sp, #0xb0
10037c304: d65f03c0    	ret
10037c308: 52800020    	mov	w0, #0x1                ; =1
10037c30c: 17fffff9    	b	0x10037c2f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic24is_cc_switch_fingerprint17hd43027b7f244137aE+0x144>
