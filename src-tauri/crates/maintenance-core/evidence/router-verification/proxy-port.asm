
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010068ef50 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE>:
10068ef50: d10183ff    	sub	sp, sp, #0x60
10068ef54: a90357f6    	stp	x22, x21, [sp, #0x30]
10068ef58: a9044ff4    	stp	x20, x19, [sp, #0x40]
10068ef5c: a9057bfd    	stp	x29, x30, [sp, #0x50]
10068ef60: 910143fd    	add	x29, sp, #0x50
10068ef64: f9400408    	ldr	x8, [x0, #0x8]
10068ef68: 19410108    	ldapurb	w8, [x8, #0x10]
10068ef6c: 34000848    	cbz	w8, 0x10068f074 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x124>
10068ef70: 79402008    	ldrh	w8, [x0, #0x10]
10068ef74: 52800fe9    	mov	w9, #0x7f               ; =127
10068ef78: 72a02009    	movk	w9, #0x100, lsl #16
10068ef7c: b80023e9    	stur	w9, [sp, #0x2]
10068ef80: 79000fe8    	strh	w8, [sp, #0x6]
10068ef84: 790003ff    	strh	wzr, [sp]
10068ef88: 910083e8    	add	x8, sp, #0x20
10068ef8c: 910003e0    	mov	x0, sp
10068ef90: d2800001    	mov	x1, #0x0                ; =0
10068ef94: 52946002    	mov	w2, #0xa300             ; =41728
10068ef98: 72a23c22    	movk	w2, #0x11e1, lsl #16
10068ef9c: 942add04    	bl	0x1011463ac <__ZN3std3net3tcp9TcpStream15connect_timeout17hfbd9ef097b11a6a5E>
10068efa0: b94023e8    	ldr	w8, [sp, #0x20]
10068efa4: 340006c8    	cbz	w8, 0x10068f07c <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x12c>
10068efa8: f94017f3    	ldr	x19, [sp, #0x28]
10068efac: 92400668    	and	x8, x19, #0x3
10068efb0: f100051f    	cmp	x8, #0x1
10068efb4: 54000201    	b.ne	0x10068eff4 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0xa4>
10068efb8: f85ffe75    	ldr	x21, [x19, #-0x1]!
10068efbc: f9400674    	ldr	x20, [x19, #0x8]
10068efc0: f9400288    	ldr	x8, [x20]
10068efc4: b4000068    	cbz	x8, 0x10068efd0 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x80>
10068efc8: aa1503e0    	mov	x0, x21
10068efcc: d63f0100    	blr	x8
10068efd0: f9400681    	ldr	x1, [x20, #0x8]
10068efd4: b4000081    	cbz	x1, 0x10068efe4 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x94>
10068efd8: f9400a82    	ldr	x2, [x20, #0x10]
10068efdc: aa1503e0    	mov	x0, x21
10068efe0: 97e5c962    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068efe4: aa1303e0    	mov	x0, x19
10068efe8: 52800301    	mov	w1, #0x18               ; =24
10068efec: 52800102    	mov	w2, #0x8                ; =8
10068eff0: 97e5c95e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068eff4: d2800000    	mov	x0, #0x0                ; =0
10068eff8: 529e1001    	mov	w1, #0xf080             ; =61568
10068effc: 72a05f41    	movk	w1, #0x2fa, lsl #16
10068f000: 942b097e    	bl	0x1011515f8 <__ZN3std6thread9functions5sleep17hea2e257bc341ef60E>
10068f004: 910083e8    	add	x8, sp, #0x20
10068f008: 910003e0    	mov	x0, sp
10068f00c: d2800001    	mov	x1, #0x0                ; =0
10068f010: 52946002    	mov	w2, #0xa300             ; =41728
10068f014: 72a23c22    	movk	w2, #0x11e1, lsl #16
10068f018: 942adce5    	bl	0x1011463ac <__ZN3std3net3tcp9TcpStream15connect_timeout17hfbd9ef097b11a6a5E>
10068f01c: b94023f4    	ldr	w20, [sp, #0x20]
10068f020: 34000374    	cbz	w20, 0x10068f08c <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x13c>
10068f024: f94017f3    	ldr	x19, [sp, #0x28]
10068f028: 92400668    	and	x8, x19, #0x3
10068f02c: f100051f    	cmp	x8, #0x1
10068f030: 54000321    	b.ne	0x10068f094 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x144>
10068f034: f85ffe75    	ldr	x21, [x19, #-0x1]!
10068f038: f9400676    	ldr	x22, [x19, #0x8]
10068f03c: f94002c8    	ldr	x8, [x22]
10068f040: b4000068    	cbz	x8, 0x10068f04c <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0xfc>
10068f044: aa1503e0    	mov	x0, x21
10068f048: d63f0100    	blr	x8
10068f04c: f94006c1    	ldr	x1, [x22, #0x8]
10068f050: b4000081    	cbz	x1, 0x10068f060 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x110>
10068f054: f9400ac2    	ldr	x2, [x22, #0x10]
10068f058: aa1503e0    	mov	x0, x21
10068f05c: 97e5c943    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068f060: aa1303e0    	mov	x0, x19
10068f064: 52800301    	mov	w1, #0x18               ; =24
10068f068: 52800102    	mov	w2, #0x8                ; =8
10068f06c: 97e5c93f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068f070: 14000009    	b	0x10068f094 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x144>
10068f074: 52800000    	mov	w0, #0x0                ; =0
10068f078: 14000008    	b	0x10068f098 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x148>
10068f07c: b94027e0    	ldr	w0, [sp, #0x24]
10068f080: 942d089b    	bl	0x1011d12ec <_writev+0x1011d12ec>
10068f084: 52800020    	mov	w0, #0x1                ; =1
10068f088: 14000004    	b	0x10068f098 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x148>
10068f08c: b94027e0    	ldr	w0, [sp, #0x24]
10068f090: 942d0897    	bl	0x1011d12ec <_writev+0x1011d12ec>
10068f094: 52000280    	eor	w0, w20, #0x1
10068f098: a9457bfd    	ldp	x29, x30, [sp, #0x50]
10068f09c: a9444ff4    	ldp	x20, x19, [sp, #0x40]
10068f0a0: a94357f6    	ldp	x22, x21, [sp, #0x30]
10068f0a4: 910183ff    	add	sp, sp, #0x60
10068f0a8: d65f03c0    	ret
10068f0ac: aa0003f4    	mov	x20, x0
10068f0b0: f94006c1    	ldr	x1, [x22, #0x8]
10068f0b4: b4000141    	cbz	x1, 0x10068f0dc <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x18c>
10068f0b8: f9400ac2    	ldr	x2, [x22, #0x10]
10068f0bc: 14000006    	b	0x10068f0d4 <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x184>
10068f0c0: aa1403e9    	mov	x9, x20
10068f0c4: aa0003f4    	mov	x20, x0
10068f0c8: f9400521    	ldr	x1, [x9, #0x8]
10068f0cc: b4000081    	cbz	x1, 0x10068f0dc <__ZN13codexmate_lib4core5relay7manager25proxy_handle_port_accepts17h492f49dd210849fcE+0x18c>
10068f0d0: f9400922    	ldr	x2, [x9, #0x10]
10068f0d4: aa1503e0    	mov	x0, x21
10068f0d8: 97e5c924    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068f0dc: aa1303e0    	mov	x0, x19
10068f0e0: 52800301    	mov	w1, #0x18               ; =24
10068f0e4: 52800102    	mov	w2, #0x8                ; =8
10068f0e8: 97e5c920    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068f0ec: aa1403e0    	mov	x0, x20
10068f0f0: 942d0819    	bl	0x1011d1154 <_writev+0x1011d1154>
