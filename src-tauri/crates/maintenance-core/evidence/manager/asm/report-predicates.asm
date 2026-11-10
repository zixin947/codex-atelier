
AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010070b0fc <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h0c787a1a458de8cdE>:
10070b0fc: a9402009    	ldp	x9, x8, [x0]
10070b100: eb08013f    	cmp	x9, x8
10070b104: 54000600    	b.eq	0x10070b1c4 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h0c787a1a458de8cdE+0xc8>
10070b108: 9101812e    	add	x14, x9, #0x60
10070b10c: d28dee4a    	mov	x10, #0x6f72            ; =28530
10070b110: f2ae8eaa    	movk	x10, #0x7475, lsl #16
10070b114: f2ce4caa    	movk	x10, #0x7265, lsl #32
10070b118: f2ee0bea    	movk	x10, #0x705f, lsl #48
10070b11c: d28e6deb    	mov	x11, #0x736f            ; =29551
10070b120: f2ac6e8b    	movk	x11, #0x6374, lsl #16
10070b124: f2cdcdeb    	movk	x11, #0x6e6f, lsl #32
10070b128: f2ed2c8b    	movk	x11, #0x6964, lsl #48
10070b12c: 528d2e8c    	mov	w12, #0x6974            ; =26996
10070b130: 72adcdec    	movk	w12, #0x6e6f, lsl #16
10070b134: 528d6ded    	mov	w13, #0x6b6f            ; =27503
10070b138: 14000005    	b	0x10070b14c <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h0c787a1a458de8cdE+0x50>
10070b13c: 9101a12e    	add	x14, x9, #0x68
10070b140: 9100212f    	add	x15, x9, #0x8
10070b144: eb0801ff    	cmp	x15, x8
10070b148: 54000340    	b.eq	0x10070b1b0 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h0c787a1a458de8cdE+0xb4>
10070b14c: aa0e03e9    	mov	x9, x14
10070b150: f85b01ce    	ldur	x14, [x14, #-0x50]
10070b154: f10051df    	cmp	x14, #0x14
10070b158: 54ffff21    	b.ne	0x10070b13c <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h0c787a1a458de8cdE+0x40>
10070b15c: f85a812e    	ldur	x14, [x9, #-0x58]
10070b160: a94041cf    	ldp	x15, x16, [x14]
10070b164: b94011ce    	ldr	w14, [x14, #0x10]
10070b168: eb0a01ff    	cmp	x15, x10
10070b16c: fa4b0200    	ccmp	x16, x11, #0x0, eq
10070b170: 7a4c01c0    	ccmp	w14, w12, #0x0, eq
10070b174: 54fffe41    	b.ne	0x10070b13c <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h0c787a1a458de8cdE+0x40>
10070b178: 3940012e    	ldrb	w14, [x9]
10070b17c: 3607fe0e    	tbz	w14, #0x0, 0x10070b13c <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h0c787a1a458de8cdE+0x40>
10070b180: f85c812e    	ldur	x14, [x9, #-0x38]
10070b184: f10009df    	cmp	x14, #0x2
10070b188: 540000a1    	b.ne	0x10070b19c <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h0c787a1a458de8cdE+0xa0>
10070b18c: f85c012e    	ldur	x14, [x9, #-0x40]
10070b190: 794001ce    	ldrh	w14, [x14]
10070b194: 6b0d01df    	cmp	w14, w13
10070b198: 54fffd20    	b.eq	0x10070b13c <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h0c787a1a458de8cdE+0x40>
10070b19c: 52800028    	mov	w8, #0x1                ; =1
10070b1a0: 91002129    	add	x9, x9, #0x8
10070b1a4: f9000009    	str	x9, [x0]
10070b1a8: aa0803e0    	mov	x0, x8
10070b1ac: d65f03c0    	ret
10070b1b0: 52800008    	mov	w8, #0x0                ; =0
10070b1b4: 91002129    	add	x9, x9, #0x8
10070b1b8: f9000009    	str	x9, [x0]
10070b1bc: aa0803e0    	mov	x0, x8
10070b1c0: d65f03c0    	ret
10070b1c4: 52800000    	mov	w0, #0x0                ; =0
10070b1c8: d65f03c0    	ret

000000010070b1cc <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E>:
10070b1cc: a9bc5ff8    	stp	x24, x23, [sp, #-0x40]!
10070b1d0: a90157f6    	stp	x22, x21, [sp, #0x10]
10070b1d4: a9024ff4    	stp	x20, x19, [sp, #0x20]
10070b1d8: a9037bfd    	stp	x29, x30, [sp, #0x30]
10070b1dc: 9100c3fd    	add	x29, sp, #0x30
10070b1e0: a9405408    	ldp	x8, x21, [x0]
10070b1e4: eb15011f    	cmp	x8, x21
10070b1e8: 54000380    	b.eq	0x10070b258 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0x8c>
10070b1ec: aa0003f3    	mov	x19, x0
10070b1f0: 91018108    	add	x8, x8, #0x60
10070b1f4: 528d6df6    	mov	w22, #0x6b6f            ; =27503
10070b1f8: 52801fb7    	mov	w23, #0xfd              ; =253
10070b1fc: 14000009    	b	0x10070b220 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0x54>
10070b200: a97a8680    	ldp	x0, x1, [x20, #-0x58]
10070b204: 97f1c8e4    	bl	0x10037d594 <__ZN13codexmate_lib4core5relay16codex_diagnostic25fix_requires_quiet_window17ha04fb1a985f8b99bE>
10070b208: 0a170008    	and	w8, w0, w23
10070b20c: 370002a8    	tbnz	w8, #0x0, 0x10070b260 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0x94>
10070b210: 9101a288    	add	x8, x20, #0x68
10070b214: 91002289    	add	x9, x20, #0x8
10070b218: eb15013f    	cmp	x9, x21
10070b21c: 540001a0    	b.eq	0x10070b250 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0x84>
10070b220: aa0803f4    	mov	x20, x8
10070b224: 39400108    	ldrb	w8, [x8]
10070b228: 7100051f    	cmp	w8, #0x1
10070b22c: 54ffff21    	b.ne	0x10070b210 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0x44>
10070b230: f85c8288    	ldur	x8, [x20, #-0x38]
10070b234: f100091f    	cmp	x8, #0x2
10070b238: 54fffe41    	b.ne	0x10070b200 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0x34>
10070b23c: f85c0288    	ldur	x8, [x20, #-0x40]
10070b240: 79400108    	ldrh	w8, [x8]
10070b244: 6b16011f    	cmp	w8, w22
10070b248: 54fffdc1    	b.ne	0x10070b200 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0x34>
10070b24c: 17fffff1    	b	0x10070b210 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0x44>
10070b250: 52800000    	mov	w0, #0x0                ; =0
10070b254: 14000004    	b	0x10070b264 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0x98>
10070b258: 52800000    	mov	w0, #0x0                ; =0
10070b25c: 14000004    	b	0x10070b26c <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17h8b92dd9b63c49666E+0xa0>
10070b260: 52800020    	mov	w0, #0x1                ; =1
10070b264: 91002288    	add	x8, x20, #0x8
10070b268: f9000268    	str	x8, [x19]
10070b26c: a9437bfd    	ldp	x29, x30, [sp, #0x30]
10070b270: a9424ff4    	ldp	x20, x19, [sp, #0x20]
10070b274: a94157f6    	ldp	x22, x21, [sp, #0x10]
10070b278: a8c45ff8    	ldp	x24, x23, [sp], #0x40
10070b27c: d65f03c0    	ret
