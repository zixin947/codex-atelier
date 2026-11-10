
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010049f188 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit11RepairAudit3fix17h09ed44c66da3de57E>:
10049f188: d10503ff    	sub	sp, sp, #0x140
10049f18c: a91157f6    	stp	x22, x21, [sp, #0x110]
10049f190: a9124ff4    	stp	x20, x19, [sp, #0x120]
10049f194: a9137bfd    	stp	x29, x30, [sp, #0x130]
10049f198: 9104c3fd    	add	x29, sp, #0x130
10049f19c: aa0403f4    	mov	x20, x4
10049f1a0: aa0303f5    	mov	x21, x3
10049f1a4: aa0203e5    	mov	x5, x2
10049f1a8: aa0103e4    	mov	x4, x1
10049f1ac: aa0003e1    	mov	x1, x0
10049f1b0: aa0803f3    	mov	x19, x8
10049f1b4: b00073c2    	adrp	x2, 0x101318000 <_anon.a9e2253c03df711f51070ce6102367ea.930+0x2c>
10049f1b8: 913ef042    	add	x2, x2, #0xfbc
10049f1bc: b00073c6    	adrp	x6, 0x101318000 <_anon.a9e2253c03df711f51070ce6102367ea.930+0x2c>
10049f1c0: 913f00c6    	add	x6, x6, #0xfc0
10049f1c4: 910023e0    	add	x0, sp, #0x8
10049f1c8: 52800083    	mov	w3, #0x4                ; =4
10049f1cc: 528000c7    	mov	w7, #0x6                ; =6
10049f1d0: 940008d2    	bl	0x1004a1518 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit9AuditSpan5start17h419151aa56b6a0b2E>
10049f1d4: 910103e8    	add	x8, sp, #0x40
10049f1d8: aa1503e0    	mov	x0, x21
10049f1dc: aa1403e1    	mov	x1, x20
10049f1e0: 97fb5246    	bl	0x100373af8 <__ZN13codexmate_lib4core5relay16codex_diagnostic18fix_auth_integrity17h63dc6ed23ad5e96eE>
10049f1e4: 910103f5    	add	x21, sp, #0x40
10049f1e8: f94023e8    	ldr	x8, [sp, #0x40]
10049f1ec: f1002d1f    	cmp	x8, #0xb
10049f1f0: 540000c1    	b.ne	0x10049f208 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit11RepairAudit3fix17h09ed44c66da3de57E+0x80>
10049f1f4: 910103e8    	add	x8, sp, #0x40
10049f1f8: d10243a0    	sub	x0, x29, #0x90
10049f1fc: 91002101    	add	x1, x8, #0x8
10049f200: 94002adc    	bl	0x1004a9d70 <__ZN4core3ops8function6FnOnce9call_once17h6026a69110e4f762E>
10049f204: 14000018    	b	0x10049f264 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit11RepairAudit3fix17h09ed44c66da3de57E+0xdc>
10049f208: f1000908    	subs	x8, x8, #0x2
10049f20c: 52800049    	mov	w9, #0x2                ; =2
10049f210: 9a883128    	csel	x8, x9, x8, lo
10049f214: f0009be9    	adrp	x9, 0x10181e000 <_anon.b5f536351c4bf1e454cf90283dd9e625.131+0x220>
10049f218: 913d0129    	add	x9, x9, #0xf40
10049f21c: f8687929    	ldr	x9, [x9, x8, lsl #3]
10049f220: d00073ca    	adrp	x10, 0x101319000 <_anon.b5f536351c4bf1e454cf90283dd9e625.167+0x8>
10049f224: 913c214a    	add	x10, x10, #0xf08
10049f228: f8687948    	ldr	x8, [x10, x8, lsl #3]
10049f22c: a93923a9    	stp	x9, x8, [x29, #-0x70]
10049f230: d101c3a8    	sub	x8, x29, #0x70
10049f234: d00019e9    	adrp	x9, 0x1007dd000 <__ZN3zip9zipcrypto24ZipCryptoReader$LT$R$GT$8validate17h6abfe2f5c5f69c24E+0xf0>
10049f238: 911b0129    	add	x9, x9, #0x6c0
10049f23c: a93727a8    	stp	x8, x9, [x29, #-0x90]
