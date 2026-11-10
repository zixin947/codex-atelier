
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100325c80 <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE>:
100325c80: d10343ff    	sub	sp, sp, #0xd0
100325c84: a90a57f6    	stp	x22, x21, [sp, #0xa0]
100325c88: a90b4ff4    	stp	x20, x19, [sp, #0xb0]
100325c8c: a90c7bfd    	stp	x29, x30, [sp, #0xc0]
100325c90: 910303fd    	add	x29, sp, #0xc0
100325c94: aa0203f6    	mov	x22, x2
100325c98: aa0103f4    	mov	x20, x1
100325c9c: aa0003f5    	mov	x21, x0
100325ca0: 910023e8    	add	x8, sp, #0x8
100325ca4: 94388592    	bl	0x1011472ec <__ZN3std3sys2fs16symlink_metadata17hd0e44a06c5789f14E>
100325ca8: b9400be8    	ldr	w8, [sp, #0x8]
100325cac: f9400bf3    	ldr	x19, [sp, #0x10]
100325cb0: 37000328    	tbnz	w8, #0x0, 0x100325d14 <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE+0x94>
100325cb4: d36cfe68    	lsr	x8, x19, #44
100325cb8: f100611f    	cmp	x8, #0x18
100325cbc: 54000201    	b.ne	0x100325cfc <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE+0x7c>
100325cc0: b94002c8    	ldr	w8, [x22]
100325cc4: 350001c8    	cbnz	w8, 0x100325cfc <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE+0x7c>
100325cc8: f9400fe8    	ldr	x8, [sp, #0x18]
100325ccc: 93407e69    	sxtw	x9, w19
100325cd0: a940aeca    	ldp	x10, x11, [x22, #0x8]
100325cd4: eb0a013f    	cmp	x9, x10
100325cd8: fa4b0100    	ccmp	x8, x11, #0x0, eq
100325cdc: 54000101    	b.ne	0x100325cfc <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE+0x7c>
100325ce0: aa1503e0    	mov	x0, x21
100325ce4: aa1403e1    	mov	x1, x20
100325ce8: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
100325cec: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
100325cf0: a94a57f6    	ldp	x22, x21, [sp, #0xa0]
100325cf4: 910343ff    	add	sp, sp, #0xd0
100325cf8: 1438848f    	b	0x101146f34 <__ZN3std3sys2fs11remove_file17h1a307295b20e7371E>
100325cfc: 90007f21    	adrp	x1, 0x101309000 <_anon.e143397d1bdb1baba839fd20a45cc9a3.246+0x390>
100325d00: 911cb821    	add	x1, x1, #0x72e
100325d04: 528002a0    	mov	w0, #0x15               ; =21
100325d08: 528006a2    	mov	w2, #0x35               ; =53
100325d0c: 9438776b    	bl	0x101143ab8 <__ZN3std2io5error5Error3new17h311cf880a0f7d574E>
100325d10: aa0003f3    	mov	x19, x0
100325d14: aa1303e0    	mov	x0, x19
100325d18: 94007647    	bl	0x100343634 <__ZN3std2io5error5Error4kind17h247456c019dc3d6bE>
100325d1c: 72001c1f    	tst	w0, #0xff
100325d20: 54000281    	b.ne	0x100325d70 <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE+0xf0>
100325d24: 92400668    	and	x8, x19, #0x3
100325d28: f100051f    	cmp	x8, #0x1
100325d2c: 54000201    	b.ne	0x100325d6c <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE+0xec>
100325d30: f85ffe74    	ldr	x20, [x19, #-0x1]!
100325d34: f9400676    	ldr	x22, [x19, #0x8]
100325d38: f94002c8    	ldr	x8, [x22]
100325d3c: b4000068    	cbz	x8, 0x100325d48 <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE+0xc8>
100325d40: aa1403e0    	mov	x0, x20
100325d44: d63f0100    	blr	x8
100325d48: f94006c1    	ldr	x1, [x22, #0x8]
100325d4c: b4000081    	cbz	x1, 0x100325d5c <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE+0xdc>
100325d50: f9400ac2    	ldr	x2, [x22, #0x10]
100325d54: aa1403e0    	mov	x0, x20
100325d58: 97f36e04    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100325d5c: aa1303e0    	mov	x0, x19
100325d60: 52800301    	mov	w1, #0x18               ; =24
100325d64: 52800102    	mov	w2, #0x8                ; =8
100325d68: 97f36e00    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100325d6c: d2800013    	mov	x19, #0x0               ; =0
100325d70: aa1303e0    	mov	x0, x19
100325d74: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
100325d78: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
100325d7c: a94a57f6    	ldp	x22, x21, [sp, #0xa0]
100325d80: 910343ff    	add	sp, sp, #0xd0
100325d84: d65f03c0    	ret
100325d88: aa0003f5    	mov	x21, x0
100325d8c: f94006c1    	ldr	x1, [x22, #0x8]
100325d90: b4000081    	cbz	x1, 0x100325da0 <__ZN13codexmate_lib4core5relay12atomic_write7staging12remove_owned17h65342aaba603b26aE+0x120>
100325d94: f9400ac2    	ldr	x2, [x22, #0x10]
100325d98: aa1403e0    	mov	x0, x20
100325d9c: 97f36df3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100325da0: aa1303e0    	mov	x0, x19
100325da4: 52800301    	mov	w1, #0x18               ; =24
100325da8: 52800102    	mov	w2, #0x8                ; =8
100325dac: 97f36def    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100325db0: aa1503e0    	mov	x0, x21
100325db4: 943aace8    	bl	0x1011d1154 <_writev+0x1011d1154>
