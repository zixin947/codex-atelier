
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100376d60 <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E>:
100376d60: f1001c3f    	cmp	x1, #0x7
100376d64: 54000143    	b.lo	0x100376d8c <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0x2c>
100376d68: b9400008    	ldr	w8, [x0]
100376d6c: b8403009    	ldur	w9, [x0, #0x3]
100376d70: 528e4e0a    	mov	w10, #0x7270            ; =29296
100376d74: 72accdea    	movk	w10, #0x666f, lsl #16
100376d78: 6b0a011f    	cmp	w8, w10
100376d7c: 528d2cc8    	mov	w8, #0x6966             ; =26982
100376d80: 72acad88    	movk	w8, #0x656c, lsl #16
100376d84: 7a480120    	ccmp	w9, w8, #0x0, eq
100376d88: 54000060    	b.eq	0x100376d94 <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0x34>
100376d8c: 120003e0    	and	w0, wzr, #0x1
100376d90: d65f03c0    	ret
100376d94: f1001c3f    	cmp	x1, #0x7
100376d98: 54000081    	b.ne	0x100376da8 <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0x48>
100376d9c: 52800029    	mov	w9, #0x1                ; =1
100376da0: 12000120    	and	w0, w9, #0x1
100376da4: d65f03c0    	ret
100376da8: 39c01c09    	ldrsb	w9, [x0, #0x7]
100376dac: 12001d28    	and	w8, w9, #0xff
100376db0: 36f80289    	tbz	w9, #0x1f, 0x100376e00 <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0xa0>
100376db4: 39402009    	ldrb	w9, [x0, #0x8]
100376db8: 1200152a    	and	w10, w9, #0x3f
100376dbc: 7103811f    	cmp	w8, #0xe0
100376dc0: 54000183    	b.lo	0x100376df0 <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0x90>
100376dc4: 12001109    	and	w9, w8, #0x1f
100376dc8: 3940240b    	ldrb	w11, [x0, #0x9]
100376dcc: 1200156b    	and	w11, w11, #0x3f
100376dd0: 2a0a196a    	orr	w10, w11, w10, lsl #6
100376dd4: 7103c11f    	cmp	w8, #0xf0
100376dd8: 54000123    	b.lo	0x100376dfc <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0x9c>
100376ddc: 39402808    	ldrb	w8, [x0, #0xa]
100376de0: 12001508    	and	w8, w8, #0x3f
100376de4: 2a0a1908    	orr	w8, w8, w10, lsl #6
100376de8: 330e0928    	bfi	w8, w9, #18, #3
100376dec: 14000005    	b	0x100376e00 <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0xa0>
100376df0: 331a110a    	bfi	w10, w8, #6, #5
100376df4: aa0a03e8    	mov	x8, x10
100376df8: 14000002    	b	0x100376e00 <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0xa0>
100376dfc: 2a093148    	orr	w8, w10, w9, lsl #12
100376e00: 52800029    	mov	w9, #0x1                ; =1
100376e04: 7100811f    	cmp	w8, #0x20
100376e08: 54000060    	b.eq	0x100376e14 <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0xb4>
100376e0c: 7100f51f    	cmp	w8, #0x3d
100376e10: 54000061    	b.ne	0x100376e1c <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0xbc>
100376e14: 12000120    	and	w0, w9, #0x1
100376e18: d65f03c0    	ret
100376e1c: 51002509    	sub	w9, w8, #0x9
100376e20: 7100153f    	cmp	w9, #0x5
100376e24: 54fffbc3    	b.lo	0x100376d9c <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0x3c>
100376e28: 7102011f    	cmp	w8, #0x80
100376e2c: 54fffb03    	b.lo	0x100376d8c <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0x2c>
100376e30: 53087d09    	lsr	w9, w8, #8
100376e34: 71007d3f    	cmp	w9, #0x1f
100376e38: 5400012c    	b.gt	0x100376e5c <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0xfc>
100376e3c: 34000209    	cbz	w9, 0x100376e7c <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0x11c>
100376e40: 5282d00a    	mov	w10, #0x1680            ; =5760
100376e44: 6b0a011f    	cmp	w8, w10
100376e48: 1a9f17e8    	cset	w8, eq
100376e4c: 7100593f    	cmp	w9, #0x16
100376e50: 1a9f0109    	csel	w9, w8, wzr, eq
100376e54: 12000120    	and	w0, w9, #0x1
100376e58: d65f03c0    	ret
100376e5c: 7100813f    	cmp	w9, #0x20
100376e60: 540001a0    	b.eq	0x100376e94 <__ZN13codexmate_lib4core5relay16codex_diagnostic21is_profile_assignment17he3203a9aa7a72039E+0x134>
100376e64: 71400d1f    	cmp	w8, #0x3, lsl #12       ; =0x3000
100376e68: 1a9f17e8    	cset	w8, eq
100376e6c: 7100c13f    	cmp	w9, #0x30
100376e70: 1a9f0109    	csel	w9, w8, wzr, eq
100376e74: 12000120    	and	w0, w9, #0x1
100376e78: d65f03c0    	ret
100376e7c: 92401d08    	and	x8, x8, #0xff
100376e80: d0008cc9    	adrp	x9, 0x101510000 <__ZN4core7unicode12unicode_data10alphabetic7OFFSETS17h22124eb1d7f80c9fE+0xb0>
100376e84: 9114fd29    	add	x9, x9, #0x53f
100376e88: 38686929    	ldrb	w9, [x9, x8]
100376e8c: 12000120    	and	w0, w9, #0x1
100376e90: d65f03c0    	ret
100376e94: 92401d08    	and	x8, x8, #0xff
100376e98: d0008cc9    	adrp	x9, 0x101510000 <__ZN4core7unicode12unicode_data10alphabetic7OFFSETS17h22124eb1d7f80c9fE+0xb0>
100376e9c: 9114fd29    	add	x9, x9, #0x53f
100376ea0: 38686928    	ldrb	w8, [x9, x8]
100376ea4: 53010509    	ubfx	w9, w8, #1, #1
100376ea8: 12000120    	and	w0, w9, #0x1
100376eac: d65f03c0    	ret
