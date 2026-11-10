
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008e2674 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E>:
1008e2674: b4000101    	cbz	x1, 0x1008e2694 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x20>
1008e2678: f100083f    	cmp	x1, #0x2
1008e267c: 54000120    	b.eq	0x1008e26a0 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x2c>
1008e2680: f100043f    	cmp	x1, #0x1
1008e2684: 54000161    	b.ne	0x1008e26b0 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x3c>
1008e2688: 39400008    	ldrb	w8, [x0]
1008e268c: 7100b91f    	cmp	w8, #0x2e
1008e2690: 54000101    	b.ne	0x1008e26b0 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x3c>
1008e2694: 52800008    	mov	w8, #0x0                ; =0
1008e2698: aa0803e0    	mov	x0, x8
1008e269c: d65f03c0    	ret
1008e26a0: 79400008    	ldrh	w8, [x0]
1008e26a4: 5285c5c9    	mov	w9, #0x2e2e             ; =11822
1008e26a8: 6b09011f    	cmp	w8, w9
1008e26ac: 54ffff40    	b.eq	0x1008e2694 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x20>
1008e26b0: 8b010009    	add	x9, x0, x1
1008e26b4: eb09001f    	cmp	x0, x9
1008e26b8: 540006a0    	b.eq	0x1008e278c <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x118>
1008e26bc: aa0003e8    	mov	x8, x0
1008e26c0: 38c0150b    	ldrsb	w11, [x8], #0x1
1008e26c4: 12001d6a    	and	w10, w11, #0xff
1008e26c8: 37f800cb    	tbnz	w11, #0x1f, 0x1008e26e0 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x6c>
1008e26cc: aa0803e0    	mov	x0, x8
1008e26d0: 52800008    	mov	w8, #0x0                ; =0
1008e26d4: 7100815f    	cmp	w10, #0x20
1008e26d8: 54000462    	b.hs	0x1008e2764 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0xf0>
1008e26dc: 1400002a    	b	0x1008e2784 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x110>
1008e26e0: 39400408    	ldrb	w8, [x0, #0x1]
1008e26e4: 1200150b    	and	w11, w8, #0x3f
1008e26e8: 7103815f    	cmp	w10, #0xe0
1008e26ec: 54000243    	b.lo	0x1008e2734 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0xc0>
1008e26f0: 12001148    	and	w8, w10, #0x1f
1008e26f4: 3940080c    	ldrb	w12, [x0, #0x2]
1008e26f8: 1200158c    	and	w12, w12, #0x3f
1008e26fc: 2a0b198b    	orr	w11, w12, w11, lsl #6
1008e2700: 7103c15f    	cmp	w10, #0xf0
1008e2704: 54000263    	b.lo	0x1008e2750 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0xdc>
1008e2708: 39400c0a    	ldrb	w10, [x0, #0x3]
1008e270c: 1200154a    	and	w10, w10, #0x3f
1008e2710: 2a0b194a    	orr	w10, w10, w11, lsl #6
1008e2714: 330e090a    	bfi	w10, w8, #18, #3
1008e2718: 7144415f    	cmp	w10, #0x110, lsl #12    ; =0x110000
1008e271c: 54000380    	b.eq	0x1008e278c <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x118>
1008e2720: 91001000    	add	x0, x0, #0x4
1008e2724: 52800008    	mov	w8, #0x0                ; =0
1008e2728: 7100815f    	cmp	w10, #0x20
1008e272c: 540001c2    	b.hs	0x1008e2764 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0xf0>
1008e2730: 14000015    	b	0x1008e2784 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x110>
1008e2734: 91000800    	add	x0, x0, #0x2
1008e2738: 331a114b    	bfi	w11, w10, #6, #5
1008e273c: aa0b03ea    	mov	x10, x11
1008e2740: 52800008    	mov	w8, #0x0                ; =0
1008e2744: 7100815f    	cmp	w10, #0x20
1008e2748: 540000e2    	b.hs	0x1008e2764 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0xf0>
1008e274c: 1400000e    	b	0x1008e2784 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x110>
1008e2750: 91000c00    	add	x0, x0, #0x3
1008e2754: 2a08316a    	orr	w10, w11, w8, lsl #12
1008e2758: 52800008    	mov	w8, #0x0                ; =0
1008e275c: 7100815f    	cmp	w10, #0x20
1008e2760: 54000123    	b.lo	0x1008e2784 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x110>
1008e2764: 5101fd4b    	sub	w11, w10, #0x7f
1008e2768: 7100857f    	cmp	w11, #0x21
1008e276c: 540000c3    	b.lo	0x1008e2784 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x110>
1008e2770: 52800008    	mov	w8, #0x0                ; =0
1008e2774: 7101715f    	cmp	w10, #0x5c
1008e2778: 54000060    	b.eq	0x1008e2784 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x110>
1008e277c: 7100bd5f    	cmp	w10, #0x2f
1008e2780: 54fff9a1    	b.ne	0x1008e26b4 <__ZN13codexmate_lib4core5relay8keychain28legacy_id_is_safe_identifier17hc80642b61185bda3E+0x40>
1008e2784: aa0803e0    	mov	x0, x8
1008e2788: d65f03c0    	ret
1008e278c: 52800020    	mov	w0, #0x1                ; =1
1008e2790: d65f03c0    	ret
