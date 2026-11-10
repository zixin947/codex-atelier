
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008b799c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE>:
1008b799c: f1004c3f    	cmp	x1, #0x13
1008b79a0: 540008e0    	b.eq	0x1008b7abc <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x120>
1008b79a4: f100303f    	cmp	x1, #0xc
1008b79a8: 54000720    	b.eq	0x1008b7a8c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0xf0>
1008b79ac: f100183f    	cmp	x1, #0x6
1008b79b0: 54000ae1    	b.ne	0x1008b7b0c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x170>
1008b79b4: b9400008    	ldr	w8, [x0]
1008b79b8: 79400809    	ldrh	w9, [x0, #0x4]
1008b79bc: 528d2c2a    	mov	w10, #0x6961            ; =26977
1008b79c0: 72ac2daa    	movk	w10, #0x616d, lsl #16
1008b79c4: 6b0a011f    	cmp	w8, w10
1008b79c8: 52862d28    	mov	w8, #0x3169             ; =12649
1008b79cc: 7a480120    	ccmp	w9, w8, #0x0, eq
1008b79d0: 540009a0    	b.eq	0x1008b7b04 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x168>
1008b79d4: 39400009    	ldrb	w9, [x0]
1008b79d8: 5101052a    	sub	w10, w9, #0x41
1008b79dc: 52800408    	mov	w8, #0x20               ; =32
1008b79e0: 7100695f    	cmp	w10, #0x1a
1008b79e4: 1a9f310a    	csel	w10, w8, wzr, lo
1008b79e8: 2a090149    	orr	w9, w10, w9
1008b79ec: 7101bd3f    	cmp	w9, #0x6f
1008b79f0: 54000921    	b.ne	0x1008b7b14 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x178>
1008b79f4: 39400409    	ldrb	w9, [x0, #0x1]
1008b79f8: 5101052a    	sub	w10, w9, #0x41
1008b79fc: 7100695f    	cmp	w10, #0x1a
1008b7a00: 1a9f3108    	csel	w8, w8, wzr, lo
1008b7a04: 2a090108    	orr	w8, w8, w9
1008b7a08: 7101c11f    	cmp	w8, #0x70
1008b7a0c: 54000841    	b.ne	0x1008b7b14 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x178>
1008b7a10: 39400809    	ldrb	w9, [x0, #0x2]
1008b7a14: 5101052a    	sub	w10, w9, #0x41
1008b7a18: 52800408    	mov	w8, #0x20               ; =32
1008b7a1c: 7100695f    	cmp	w10, #0x1a
1008b7a20: 1a9f310a    	csel	w10, w8, wzr, lo
1008b7a24: 2a090149    	orr	w9, w10, w9
1008b7a28: 7101953f    	cmp	w9, #0x65
1008b7a2c: 54000741    	b.ne	0x1008b7b14 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x178>
1008b7a30: 39400c09    	ldrb	w9, [x0, #0x3]
1008b7a34: 5101052a    	sub	w10, w9, #0x41
1008b7a38: 7100695f    	cmp	w10, #0x1a
1008b7a3c: 1a9f3108    	csel	w8, w8, wzr, lo
1008b7a40: 2a090108    	orr	w8, w8, w9
1008b7a44: 7101b91f    	cmp	w8, #0x6e
1008b7a48: 54000661    	b.ne	0x1008b7b14 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x178>
1008b7a4c: 39401009    	ldrb	w9, [x0, #0x4]
1008b7a50: 5101052a    	sub	w10, w9, #0x41
1008b7a54: 52800408    	mov	w8, #0x20               ; =32
1008b7a58: 7100695f    	cmp	w10, #0x1a
1008b7a5c: 1a9f310a    	csel	w10, w8, wzr, lo
1008b7a60: 2a090149    	orr	w9, w10, w9
1008b7a64: 7101853f    	cmp	w9, #0x61
1008b7a68: 54000561    	b.ne	0x1008b7b14 <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x178>
1008b7a6c: 39401409    	ldrb	w9, [x0, #0x5]
1008b7a70: 5101052a    	sub	w10, w9, #0x41
1008b7a74: 7100695f    	cmp	w10, #0x1a
1008b7a78: 1a9f3108    	csel	w8, w8, wzr, lo
1008b7a7c: 2a090108    	orr	w8, w8, w9
1008b7a80: 7101a51f    	cmp	w8, #0x69
1008b7a84: 1a9f17e0    	cset	w0, eq
1008b7a88: d65f03c0    	ret
1008b7a8c: f9400008    	ldr	x8, [x0]
1008b7a90: b9400809    	ldr	w9, [x0, #0x8]
1008b7a94: d28d2c2a    	mov	x10, #0x6961            ; =26977
1008b7a98: f2ac2daa    	movk	x10, #0x616d, lsl #16
1008b7a9c: f2cd2daa    	movk	x10, #0x696d, lsl #32
1008b7aa0: f2ee4bea    	movk	x10, #0x725f, lsl #48
1008b7aa4: eb0a011f    	cmp	x8, x10
1008b7aa8: 528d8ca8    	mov	w8, #0x6c65             ; =27749
1008b7aac: 72af2c28    	movk	w8, #0x7961, lsl #16
1008b7ab0: 7a480120    	ccmp	w9, w8, #0x0, eq
1008b7ab4: 1a9f17e0    	cset	w0, eq
1008b7ab8: d65f03c0    	ret
1008b7abc: a9402408    	ldp	x8, x9, [x0]
1008b7ac0: f840b00a    	ldur	x10, [x0, #0xb]
1008b7ac4: d28d2c2b    	mov	x11, #0x6961            ; =26977
1008b7ac8: f2ac2dab    	movk	x11, #0x616d, lsl #16
1008b7acc: f2cd2dab    	movk	x11, #0x696d, lsl #32
1008b7ad0: f2ec6beb    	movk	x11, #0x635f, lsl #48
1008b7ad4: eb0b011f    	cmp	x8, x11
1008b7ad8: d28c8de8    	mov	x8, #0x646f             ; =25711
1008b7adc: f2af0ca8    	movk	x8, #0x7865, lsl #16
1008b7ae0: f2ce4be8    	movk	x8, #0x725f, lsl #32
1008b7ae4: f2eeade8    	movk	x8, #0x756f, lsl #48
1008b7ae8: fa480120    	ccmp	x9, x8, #0x0, eq
1008b7aec: d28bef08    	mov	x8, #0x5f78             ; =24440
1008b7af0: f2adee48    	movk	x8, #0x6f72, lsl #16
1008b7af4: f2ce8ea8    	movk	x8, #0x7475, lsl #32
1008b7af8: f2ee4ca8    	movk	x8, #0x7265, lsl #48
1008b7afc: fa480140    	ccmp	x10, x8, #0x0, eq
1008b7b00: 540000e1    	b.ne	0x1008b7b1c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x180>
1008b7b04: 52800020    	mov	w0, #0x1                ; =1
1008b7b08: d65f03c0    	ret
1008b7b0c: f100343f    	cmp	x1, #0xd
1008b7b10: 54000062    	b.hs	0x1008b7b1c <__ZN13codexmate_lib4core5relay23codex_thread_visibility28is_router_or_openai_provider17hf6006662328dfd8cE+0x180>
1008b7b14: 52800000    	mov	w0, #0x0                ; =0
1008b7b18: d65f03c0    	ret
1008b7b1c: f9400008    	ldr	x8, [x0]
1008b7b20: f8405009    	ldur	x9, [x0, #0x5]
1008b7b24: d28d2c2a    	mov	x10, #0x6961            ; =26977
1008b7b28: f2ac2daa    	movk	x10, #0x616d, lsl #16
1008b7b2c: f2cd2daa    	movk	x10, #0x696d, lsl #32
1008b7b30: f2ee4bea    	movk	x10, #0x725f, lsl #48
1008b7b34: eb0a011f    	cmp	x8, x10
1008b7b38: d28bed28    	mov	x8, #0x5f69             ; =24425
1008b7b3c: f2acae48    	movk	x8, #0x6572, lsl #16
1008b7b40: f2cc2d88    	movk	x8, #0x616c, lsl #32
1008b7b44: f2ebef28    	movk	x8, #0x5f79, lsl #48
1008b7b48: fa480120    	ccmp	x9, x8, #0x0, eq
1008b7b4c: 1a9f17e0    	cset	w0, eq
1008b7b50: d65f03c0    	ret
