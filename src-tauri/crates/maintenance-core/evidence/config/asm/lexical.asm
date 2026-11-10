
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078da1c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE>:
10078da1c: 39400008    	ldrb	w8, [x0]
10078da20: b4000dc2    	cbz	x2, 0x10078dbd8 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x1bc>
10078da24: d280000d    	mov	x13, #0x0               ; =0
10078da28: 52844449    	mov	w9, #0x2222             ; =8738
10078da2c: 5280044a    	mov	w10, #0x22              ; =34
10078da30: 5284e4eb    	mov	w11, #0x2727            ; =10023
10078da34: 528004ec    	mov	w12, #0x27              ; =39
10078da38: aa0803ee    	mov	x14, x8
10078da3c: 14000005    	b	0x10078da50 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x34>
10078da40: 910005ad    	add	x13, x13, #0x1
10078da44: 5280002e    	mov	w14, #0x1               ; =1
10078da48: eb0201bf    	cmp	x13, x2
10078da4c: 54000c62    	b.hs	0x10078dbd8 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x1bc>
10078da50: 12001dcf    	and	w15, w14, #0xff
10078da54: 710009ff    	cmp	w15, #0x2
10078da58: 54000181    	b.ne	0x10078da88 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x6c>
10078da5c: 8b0d002e    	add	x14, x1, x13
10078da60: 394001cf    	ldrb	w15, [x14]
10078da64: 710089ff    	cmp	w15, #0x22
10078da68: 54000580    	b.eq	0x10078db18 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0xfc>
10078da6c: 71009dff    	cmp	w15, #0x27
10078da70: 54000340    	b.eq	0x10078dad8 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0xbc>
10078da74: 71008dff    	cmp	w15, #0x23
10078da78: 54000b00    	b.eq	0x10078dbd8 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x1bc>
10078da7c: 910005ad    	add	x13, x13, #0x1
10078da80: 5280004e    	mov	w14, #0x2               ; =2
10078da84: 17fffff1    	b	0x10078da48 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x2c>
10078da88: 3600016e    	tbz	w14, #0x0, 0x10078dab4 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x98>
10078da8c: 91000daf    	add	x15, x13, #0x3
10078da90: eb0201ff    	cmp	x15, x2
10078da94: 54fffd68    	b.hi	0x10078da40 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x24>
10078da98: 8b0d002e    	add	x14, x1, x13
10078da9c: 794001d0    	ldrh	w16, [x14]
10078daa0: 394009ce    	ldrb	w14, [x14, #0x2]
10078daa4: 6b0b021f    	cmp	w16, w11
10078daa8: 7a4c01c0    	ccmp	w14, w12, #0x0, eq
10078daac: 54fffca1    	b.ne	0x10078da40 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x24>
10078dab0: 14000040    	b	0x10078dbb0 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x194>
10078dab4: 8b0d002e    	add	x14, x1, x13
10078dab8: 394001cf    	ldrb	w15, [x14]
10078dabc: 710171ff    	cmp	w15, #0x5c
10078dac0: 54000621    	b.ne	0x10078db84 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x168>
10078dac4: 5280000e    	mov	w14, #0x0               ; =0
10078dac8: 910009ad    	add	x13, x13, #0x2
10078dacc: eb0d005f    	cmp	x2, x13
10078dad0: 9a8d304d    	csel	x13, x2, x13, lo
10078dad4: 17ffffdd    	b	0x10078da48 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x2c>
10078dad8: 91000daf    	add	x15, x13, #0x3
10078dadc: eb0201ff    	cmp	x15, x2
10078dae0: 540000c8    	b.hi	0x10078daf8 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0xdc>
10078dae4: 794001d0    	ldrh	w16, [x14]
10078dae8: 394009ce    	ldrb	w14, [x14, #0x2]
10078daec: 6b0b021f    	cmp	w16, w11
10078daf0: 7a4c01c0    	ccmp	w14, w12, #0x0, eq
10078daf4: 54000640    	b.eq	0x10078dbbc <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x1a0>
10078daf8: 910005ad    	add	x13, x13, #0x1
10078dafc: eb0201bf    	cmp	x13, x2
10078db00: 540006c2    	b.hs	0x10078dbd8 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x1bc>
10078db04: 386d682e    	ldrb	w14, [x1, x13]
10078db08: 910005ad    	add	x13, x13, #0x1
10078db0c: 71009ddf    	cmp	w14, #0x27
10078db10: 54ffff61    	b.ne	0x10078dafc <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0xe0>
10078db14: 1400001a    	b	0x10078db7c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x160>
10078db18: 91000daf    	add	x15, x13, #0x3
10078db1c: eb0201ff    	cmp	x15, x2
10078db20: 540000c8    	b.hi	0x10078db38 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x11c>
10078db24: 794001d0    	ldrh	w16, [x14]
10078db28: 394009ce    	ldrb	w14, [x14, #0x2]
10078db2c: 6b09021f    	cmp	w16, w9
10078db30: 7a4a01c0    	ccmp	w14, w10, #0x0, eq
10078db34: 540004a0    	b.eq	0x10078dbc8 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x1ac>
10078db38: 910005ad    	add	x13, x13, #0x1
10078db3c: eb0201bf    	cmp	x13, x2
10078db40: 540000e3    	b.lo	0x10078db5c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x140>
10078db44: 1400000e    	b	0x10078db7c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x160>
10078db48: 910009ad    	add	x13, x13, #0x2
10078db4c: eb0d005f    	cmp	x2, x13
10078db50: 9a8d304d    	csel	x13, x2, x13, lo
10078db54: eb0201bf    	cmp	x13, x2
10078db58: 54000122    	b.hs	0x10078db7c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x160>
10078db5c: 386d682e    	ldrb	w14, [x1, x13]
10078db60: 710171df    	cmp	w14, #0x5c
10078db64: 54ffff20    	b.eq	0x10078db48 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x12c>
10078db68: 710089df    	cmp	w14, #0x22
10078db6c: 54fff880    	b.eq	0x10078da7c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x60>
10078db70: 910005ad    	add	x13, x13, #0x1
10078db74: eb0201bf    	cmp	x13, x2
10078db78: 54ffff23    	b.lo	0x10078db5c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x140>
10078db7c: 5280004e    	mov	w14, #0x2               ; =2
10078db80: 17ffffb2    	b	0x10078da48 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x2c>
10078db84: 91000daf    	add	x15, x13, #0x3
10078db88: eb0201ff    	cmp	x15, x2
10078db8c: 540000c8    	b.hi	0x10078dba4 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x188>
10078db90: 794001d0    	ldrh	w16, [x14]
10078db94: 394009ce    	ldrb	w14, [x14, #0x2]
10078db98: 6b09021f    	cmp	w16, w9
10078db9c: 7a4a01c0    	ccmp	w14, w10, #0x0, eq
10078dba0: 54000080    	b.eq	0x10078dbb0 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x194>
10078dba4: 5280000e    	mov	w14, #0x0               ; =0
10078dba8: 910005ad    	add	x13, x13, #0x1
10078dbac: 17ffffa7    	b	0x10078da48 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x2c>
10078dbb0: 5280004e    	mov	w14, #0x2               ; =2
10078dbb4: 3900000e    	strb	w14, [x0]
10078dbb8: 14000006    	b	0x10078dbd0 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x1b4>
10078dbbc: 5280002e    	mov	w14, #0x1               ; =1
10078dbc0: 3900000e    	strb	w14, [x0]
10078dbc4: 14000003    	b	0x10078dbd0 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x1b4>
10078dbc8: 5280000e    	mov	w14, #0x0               ; =0
10078dbcc: 3900001f    	strb	wzr, [x0]
10078dbd0: aa0f03ed    	mov	x13, x15
10078dbd4: 17ffff9d    	b	0x10078da48 <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE+0x2c>
10078dbd8: 7100091f    	cmp	w8, #0x2
10078dbdc: 1a9f17e0    	cset	w0, eq
10078dbe0: d65f03c0    	ret

