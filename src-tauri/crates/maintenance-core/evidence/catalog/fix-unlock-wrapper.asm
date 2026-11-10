
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010049fee4 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit11RepairAudit3fix17h804d2ec3928990d7E>:
10049fee4: d10503ff    	sub	sp, sp, #0x140
10049fee8: a91157f6    	stp	x22, x21, [sp, #0x110]
10049feec: a9124ff4    	stp	x20, x19, [sp, #0x120]
10049fef0: a9137bfd    	stp	x29, x30, [sp, #0x130]
10049fef4: 9104c3fd    	add	x29, sp, #0x130
10049fef8: aa0403f4    	mov	x20, x4
10049fefc: aa0303f5    	mov	x21, x3
10049ff00: aa0203e5    	mov	x5, x2
10049ff04: aa0103e4    	mov	x4, x1
10049ff08: aa0003e1    	mov	x1, x0
10049ff0c: aa0803f3    	mov	x19, x8
10049ff10: b00073c2    	adrp	x2, 0x101318000 <_anon.a9e2253c03df711f51070ce6102367ea.930+0x2c>
10049ff14: 913ef042    	add	x2, x2, #0xfbc
10049ff18: b00073c6    	adrp	x6, 0x101318000 <_anon.a9e2253c03df711f51070ce6102367ea.930+0x2c>
10049ff1c: 913f00c6    	add	x6, x6, #0xfc0
10049ff20: 910023e0    	add	x0, sp, #0x8
10049ff24: 52800083    	mov	w3, #0x4                ; =4
10049ff28: 528000c7    	mov	w7, #0x6                ; =6
10049ff2c: 9400057b    	bl	0x1004a1518 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit9AuditSpan5start17h419151aa56b6a0b2E>
10049ff30: 910103e8    	add	x8, sp, #0x40
10049ff34: aa1503e0    	mov	x0, x21
10049ff38: aa1403e1    	mov	x1, x20
10049ff3c: 97fbcc76    	bl	0x100393114 <__ZN13codexmate_lib4core5relay16codex_diagnostic30fix_router_unlock_auth_residue17ha30f89795617cd6fE>
10049ff40: 910103f5    	add	x21, sp, #0x40
10049ff44: f94023e8    	ldr	x8, [sp, #0x40]
10049ff48: f1002d1f    	cmp	x8, #0xb
10049ff4c: 540000c1    	b.ne	0x10049ff64 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit11RepairAudit3fix17h804d2ec3928990d7E+0x80>
10049ff50: 910103e8    	add	x8, sp, #0x40
10049ff54: d10243a0    	sub	x0, x29, #0x90
10049ff58: 91002101    	add	x1, x8, #0x8
10049ff5c: 94002785    	bl	0x1004a9d70 <__ZN4core3ops8function6FnOnce9call_once17h6026a69110e4f762E>
10049ff60: 14000018    	b	0x10049ffc0 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit11RepairAudit3fix17h804d2ec3928990d7E+0xdc>
10049ff64: f1000908    	subs	x8, x8, #0x2
10049ff68: 52800049    	mov	w9, #0x2                ; =2
10049ff6c: 9a883128    	csel	x8, x9, x8, lo
10049ff70: f0009be9    	adrp	x9, 0x10181e000 <_anon.b5f536351c4bf1e454cf90283dd9e625.131+0x220>
10049ff74: 913d0129    	add	x9, x9, #0xf40
10049ff78: f8687929    	ldr	x9, [x9, x8, lsl #3]
10049ff7c: d00073ca    	adrp	x10, 0x101319000 <_anon.b5f536351c4bf1e454cf90283dd9e625.167+0x8>
10049ff80: 913c214a    	add	x10, x10, #0xf08
10049ff84: f8687948    	ldr	x8, [x10, x8, lsl #3]
10049ff88: a93923a9    	stp	x9, x8, [x29, #-0x70]
10049ff8c: d101c3a8    	sub	x8, x29, #0x70
10049ff90: d00019e9    	adrp	x9, 0x1007dd000 <__ZN3zip9zipcrypto24ZipCryptoReader$LT$R$GT$8validate17h6abfe2f5c5f69c24E+0xf0>
10049ff94: 911b0129    	add	x9, x9, #0x6c0
10049ff98: a93727a8    	stp	x8, x9, [x29, #-0x90]
10049ff9c: d0008420    	adrp	x0, 0x101525000 <_anon.a9e2253c03df711f51070ce6102367ea.957+0x57>
