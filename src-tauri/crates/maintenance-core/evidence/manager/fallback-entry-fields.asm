
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008f194c <__ZN288_$LT$codexmate_lib..core..relay..codex_thread_visibility..model_fallback.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..codex_thread_visibility..model_fallback..ModelRestoreEntry$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd9ec1183567537caE>:
1008f194c: f100243f    	cmp	x1, #0x9
1008f1950: 540003c0    	b.eq	0x1008f19c8 <__ZN288_$LT$codexmate_lib..core..relay..codex_thread_visibility..model_fallback.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..codex_thread_visibility..model_fallback..ModelRestoreEntry$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd9ec1183567537caE+0x7c>
1008f1954: f100383f    	cmp	x1, #0xe
1008f1958: 54000501    	b.ne	0x1008f19f8 <__ZN288_$LT$codexmate_lib..core..relay..codex_thread_visibility..model_fallback.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..codex_thread_visibility..model_fallback..ModelRestoreEntry$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd9ec1183567537caE+0xac>
1008f195c: d28d6c69    	mov	x9, #0x6b63             ; =27491
1008f1960: f2adabe9    	movk	x9, #0x6d5f, lsl #16
1008f1964: f2cc8de9    	movk	x9, #0x646f, lsl #32
1008f1968: f2ed8ca9    	movk	x9, #0x6c65, lsl #48
1008f196c: f940000a    	ldr	x10, [x0]
1008f1970: f840600b    	ldur	x11, [x0, #0x6]
1008f1974: 9103f92c    	add	x12, x9, #0xfe
1008f1978: d28e4ded    	mov	x13, #0x726f            ; =29295
1008f197c: f2aced2d    	movk	x13, #0x6769, lsl #16
1008f1980: f2cdcd2d    	movk	x13, #0x6e69, lsl #32
1008f1984: f2ed8c2d    	movk	x13, #0x6c61, lsl #48
1008f1988: eb0d015f    	cmp	x10, x13
1008f198c: fa4c0160    	ccmp	x11, x12, #0x0, eq
1008f1990: 540004e0    	b.eq	0x1008f1a2c <__ZN288_$LT$codexmate_lib..core..relay..codex_thread_visibility..model_fallback.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..codex_thread_visibility..model_fallback..ModelRestoreEntry$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd9ec1183567537caE+0xe0>
1008f1994: f940000a    	ldr	x10, [x0]
1008f1998: f840600b    	ldur	x11, [x0, #0x6]
1008f199c: d28c2ccc    	mov	x12, #0x6166            ; =24934
1008f19a0: f2ad8d8c    	movk	x12, #0x6c6c, lsl #16
1008f19a4: f2cc2c4c    	movk	x12, #0x6162, lsl #32
1008f19a8: f2ed6c6c    	movk	x12, #0x6b63, lsl #48
1008f19ac: eb0c015f    	cmp	x10, x12
1008f19b0: fa490160    	ccmp	x11, x9, #0x0, eq
1008f19b4: 54000441    	b.ne	0x1008f1a3c <__ZN288_$LT$codexmate_lib..core..relay..codex_thread_visibility..model_fallback.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..codex_thread_visibility..model_fallback..ModelRestoreEntry$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd9ec1183567537caE+0xf0>
1008f19b8: 5280004a    	mov	w10, #0x2               ; =2
1008f19bc: 3900050a    	strb	w10, [x8, #0x1]
1008f19c0: 3900011f    	strb	wzr, [x8]
1008f19c4: d65f03c0    	ret
1008f19c8: f9400009    	ldr	x9, [x0]
1008f19cc: 3940200a    	ldrb	w10, [x0, #0x8]
1008f19d0: d28d0e8b    	mov	x11, #0x6874            ; =26740
1008f19d4: f2acae4b    	movk	x11, #0x6572, lsl #16
1008f19d8: f2cc8c2b    	movk	x11, #0x6461, lsl #32
1008f19dc: f2ed2beb    	movk	x11, #0x695f, lsl #48
1008f19e0: eb0b013f    	cmp	x9, x11
1008f19e4: 52800c89    	mov	w9, #0x64               ; =100
1008f19e8: 7a490140    	ccmp	w10, w9, #0x0, eq
1008f19ec: 54000281    	b.ne	0x1008f1a3c <__ZN288_$LT$codexmate_lib..core..relay..codex_thread_visibility..model_fallback.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..codex_thread_visibility..model_fallback..ModelRestoreEntry$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd9ec1183567537caE+0xf0>
1008f19f0: 7900011f    	strh	wzr, [x8]
1008f19f4: d65f03c0    	ret
1008f19f8: f100203f    	cmp	x1, #0x8
1008f19fc: 54000201    	b.ne	0x1008f1a3c <__ZN288_$LT$codexmate_lib..core..relay..codex_thread_visibility..model_fallback.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..codex_thread_visibility..model_fallback..ModelRestoreEntry$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd9ec1183567537caE+0xf0>
1008f1a00: f9400009    	ldr	x9, [x0]
1008f1a04: d28e8c2a    	mov	x10, #0x7461            ; =29793
1008f1a08: f2acae8a    	movk	x10, #0x6574, lsl #16
1008f1a0c: f2ce0daa    	movk	x10, #0x706d, lsl #32
1008f1a10: f2ee6e8a    	movk	x10, #0x7374, lsl #48
1008f1a14: eb0a013f    	cmp	x9, x10
1008f1a18: 54000121    	b.ne	0x1008f1a3c <__ZN288_$LT$codexmate_lib..core..relay..codex_thread_visibility..model_fallback.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..codex_thread_visibility..model_fallback..ModelRestoreEntry$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd9ec1183567537caE+0xf0>
1008f1a1c: 5280006a    	mov	w10, #0x3               ; =3
1008f1a20: 3900050a    	strb	w10, [x8, #0x1]
1008f1a24: 3900011f    	strb	wzr, [x8]
1008f1a28: d65f03c0    	ret
1008f1a2c: 5280002a    	mov	w10, #0x1               ; =1
1008f1a30: 3900050a    	strb	w10, [x8, #0x1]
1008f1a34: 3900011f    	strb	wzr, [x8]
1008f1a38: d65f03c0    	ret
1008f1a3c: a9be4ff4    	stp	x20, x19, [sp, #-0x20]!
1008f1a40: a9017bfd    	stp	x29, x30, [sp, #0x10]
1008f1a44: 910043fd    	add	x29, sp, #0x10
1008f1a48: 900079e2    	adrp	x2, 0x10182d000 <_anon.973b090bee24c06fdb949486dbea4242.549+0x1e8>
1008f1a4c: 912d0042    	add	x2, x2, #0xb40
1008f1a50: 52800083    	mov	w3, #0x4                ; =4
1008f1a54: aa0803f3    	mov	x19, x8
1008f1a58: 94226aeb    	bl	0x10118c604 <__ZN10serde_core2de5Error13unknown_field17hadbce53ee1f5a987E>
1008f1a5c: aa1303e8    	mov	x8, x19
1008f1a60: f9000660    	str	x0, [x19, #0x8]
1008f1a64: 52800029    	mov	w9, #0x1                ; =1
1008f1a68: a9417bfd    	ldp	x29, x30, [sp, #0x10]
1008f1a6c: a8c24ff4    	ldp	x20, x19, [sp], #0x20
1008f1a70: 39000109    	strb	w9, [x8]
1008f1a74: d65f03c0    	ret
