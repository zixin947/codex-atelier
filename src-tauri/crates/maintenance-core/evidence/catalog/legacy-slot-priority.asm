
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100370394 <__ZN13codexmate_lib4core5relay13codex_catalog31migrate_legacy_no_account_slots17h39b1f1f35ca98b9eE>:
100370918: f940a3e8    	ldr	x8, [sp, #0x140]
10037091c: f9400bf8    	ldr	x24, [sp, #0x10]
100370920: b40028e8    	cbz	x8, 0x100370e3c <__ZN13codexmate_lib4core5relay13codex_catalog31migrate_legacy_no_account_slots17h39b1f1f35ca98b9eE+0xaa8>
100370924: d28e0cfa    	mov	x26, #0x7067            ; =28775
100370928: f2a5ae9a    	movk	x26, #0x2d74, lsl #16
10037092c: f2c5c6ba    	movk	x26, #0x2e35, lsl #32
100370930: f2e5a6da    	movk	x26, #0x2d36, lsl #48
100370934: 52800909    	mov	w9, #0x48               ; =72
100370938: f9409ff8    	ldr	x24, [sp, #0x138]
10037093c: 9b096117    	madd	x23, x8, x9, x24
100370940: 90007cf4    	adrp	x20, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
100370944: 91165294    	add	x20, x20, #0x594
100370948: d2f00016    	mov	x22, #-0x8000000000000000 ; =-9223372036854775808
10037094c: 5280003c    	mov	w28, #0x1               ; =1
100370950: 52800315    	mov	w21, #0x18              ; =24
100370954: 14000006    	b	0x10037096c <__ZN13codexmate_lib4core5relay13codex_catalog31migrate_legacy_no_account_slots17h39b1f1f35ca98b9eE+0x5d8>
100370958: a903671b    	stp	x27, x25, [x24, #0x30]
10037095c: f900231b    	str	x27, [x24, #0x40]
100370960: 91012318    	add	x24, x24, #0x48
100370964: eb17031f    	cmp	x24, x23
100370968: 54002a20    	b.eq	0x100370eac <__ZN13codexmate_lib4core5relay13codex_catalog31migrate_legacy_no_account_slots17h39b1f1f35ca98b9eE+0xb18>
10037096c: f9401b08    	ldr	x8, [x24, #0x30]
100370970: eb16011f    	cmp	x8, x22
100370974: 54ffff61    	b.ne	0x100370960 <__ZN13codexmate_lib4core5relay13codex_catalog31migrate_legacy_no_account_slots17h39b1f1f35ca98b9eE+0x5cc>
100370978: f9408fe8    	ldr	x8, [sp, #0x118]
10037097c: b40006a8    	cbz	x8, 0x100370a50 <__ZN13codexmate_lib4core5relay13codex_catalog31migrate_legacy_no_account_slots17h39b1f1f35ca98b9eE+0x6bc>
