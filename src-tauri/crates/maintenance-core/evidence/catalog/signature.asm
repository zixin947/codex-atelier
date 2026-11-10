
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010036ff60 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E>:
10036ff60: d10643ff    	sub	sp, sp, #0x190
10036ff64: a9136ffc    	stp	x28, x27, [sp, #0x130]
10036ff68: a91467fa    	stp	x26, x25, [sp, #0x140]
10036ff6c: a9155ff8    	stp	x24, x23, [sp, #0x150]
10036ff70: a91657f6    	stp	x22, x21, [sp, #0x160]
10036ff74: a9174ff4    	stp	x20, x19, [sp, #0x170]
10036ff78: a9187bfd    	stp	x29, x30, [sp, #0x180]
10036ff7c: 910603fd    	add	x29, sp, #0x180
10036ff80: aa0003f5    	mov	x21, x0
10036ff84: 910263e8    	add	x8, sp, #0x98
10036ff88: 9411410a    	bl	0x1007c03b0 <__ZN13codexmate_lib8platform7process22codex_bundled_cli_path17hb3eb11f7418bff9bE>
10036ff8c: f9404fe8    	ldr	x8, [sp, #0x98]
10036ff90: f1002d1f    	cmp	x8, #0xb
10036ff94: 54000281    	b.ne	0x10036ffe4 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x84>
10036ff98: f94053f3    	ldr	x19, [sp, #0xa0]
10036ff9c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10036ffa0: eb08027f    	cmp	x19, x8
10036ffa4: 54000240    	b.eq	0x10036ffec <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x8c>
10036ffa8: a94adbf4    	ldp	x20, x22, [sp, #0xa8]
10036ffac: 910263e8    	add	x8, sp, #0x98
10036ffb0: aa1403e0    	mov	x0, x20
10036ffb4: aa1603e1    	mov	x1, x22
10036ffb8: 94376190    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
10036ffbc: b9409be8    	ldr	w8, [sp, #0x98]
10036ffc0: f94053f7    	ldr	x23, [sp, #0xa0]
10036ffc4: 360001a8    	tbz	w8, #0x0, 0x10036fff8 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x98>
10036ffc8: 52994019    	mov	w25, #0xca00            ; =51712
10036ffcc: 72a77359    	movk	w25, #0x3b9a, lsl #16
10036ffd0: 924006e8    	and	x8, x23, #0x3
10036ffd4: f100051f    	cmp	x8, #0x1
10036ffd8: 54000800    	b.eq	0x1003700d8 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x178>
10036ffdc: d2800008    	mov	x8, #0x0                ; =0
10036ffe0: 14000032    	b	0x1003700a8 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x148>
10036ffe4: 910263e0    	add	x0, sp, #0x98
10036ffe8: 94019f72    	bl	0x1003d7db0 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
10036ffec: 52800048    	mov	w8, #0x2                ; =2
10036fff0: f90002a8    	str	x8, [x21]
10036fff4: 14000031    	b	0x1003700b8 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x158>
10036fff8: 910263e8    	add	x8, sp, #0x98
10036fffc: ad438500    	ldp	q0, q1, [x8, #0x70]
100370000: ad0387e0    	stp	q0, q1, [sp, #0x70]
100370004: f94097e9    	ldr	x9, [sp, #0x128]
100370008: f9004be9    	str	x9, [sp, #0x90]
10037000c: ad418500    	ldp	q0, q1, [x8, #0x30]
100370010: ad0187e0    	stp	q0, q1, [sp, #0x30]
100370014: ad428101    	ldp	q1, q0, [x8, #0x50]
100370018: ad0283e1    	stp	q1, q0, [sp, #0x50]
10037001c: ad408101    	ldp	q1, q0, [x8, #0x10]
100370020: ad0083e1    	stp	q1, q0, [sp, #0x10]
100370024: 910003e9    	mov	x9, sp
100370028: 52800028    	mov	w8, #0x1                ; =1
10037002c: a9005fe8    	stp	x8, x23, [sp]
100370030: 910263e8    	add	x8, sp, #0x98
100370034: 91002120    	add	x0, x9, #0x8
100370038: 94374b16    	bl	0x101142c90 <__ZN3std2fs8Metadata8modified17h94417a265bee4084E>
10037003c: b940a3f9    	ldr	w25, [sp, #0xa0]
100370040: f9404ffa    	ldr	x26, [sp, #0x98]
100370044: 52994008    	mov	w8, #0xca00             ; =51712
100370048: 72a77348    	movk	w8, #0x3b9a, lsl #16
10037004c: 6b08033f    	cmp	w25, w8
100370050: 54000281    	b.ne	0x1003700a0 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x140>
100370054: 92400748    	and	x8, x26, #0x3
100370058: f100051f    	cmp	x8, #0x1
10037005c: 54000221    	b.ne	0x1003700a0 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x140>
100370060: aa1a03f7    	mov	x23, x26
100370064: f85ffef8    	ldr	x24, [x23, #-0x1]!
100370068: f94006fb    	ldr	x27, [x23, #0x8]
10037006c: f9400368    	ldr	x8, [x27]
100370070: b4000068    	cbz	x8, 0x10037007c <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x11c>
100370074: aa1803e0    	mov	x0, x24
100370078: d63f0100    	blr	x8
10037007c: f9400761    	ldr	x1, [x27, #0x8]
100370080: b4000081    	cbz	x1, 0x100370090 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x130>
100370084: f9400b62    	ldr	x2, [x27, #0x10]
100370088: aa1803e0    	mov	x0, x24
10037008c: 97f24537    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100370090: aa1703e0    	mov	x0, x23
100370094: 52800301    	mov	w1, #0x18               ; =24
100370098: 52800102    	mov	w2, #0x8                ; =8
10037009c: 97f24533    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003700a0: f94037e9    	ldr	x9, [sp, #0x68]
1003700a4: 52800028    	mov	w8, #0x1                ; =1
1003700a8: a90152b3    	stp	x19, x20, [x21, #0x10]
1003700ac: a90026a8    	stp	x8, x9, [x21]
1003700b0: a9026ab6    	stp	x22, x26, [x21, #0x20]
1003700b4: b90032b9    	str	w25, [x21, #0x30]
1003700b8: a9587bfd    	ldp	x29, x30, [sp, #0x180]
1003700bc: a9574ff4    	ldp	x20, x19, [sp, #0x170]
1003700c0: a95657f6    	ldp	x22, x21, [sp, #0x160]
1003700c4: a9555ff8    	ldp	x24, x23, [sp, #0x150]
1003700c8: a95467fa    	ldp	x26, x25, [sp, #0x140]
1003700cc: a9536ffc    	ldp	x28, x27, [sp, #0x130]
1003700d0: 910643ff    	add	sp, sp, #0x190
1003700d4: d65f03c0    	ret
1003700d8: f85ffef8    	ldr	x24, [x23, #-0x1]!
1003700dc: f94006fa    	ldr	x26, [x23, #0x8]
1003700e0: f9400348    	ldr	x8, [x26]
1003700e4: b4000068    	cbz	x8, 0x1003700f0 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x190>
1003700e8: aa1803e0    	mov	x0, x24
1003700ec: d63f0100    	blr	x8
1003700f0: f9400741    	ldr	x1, [x26, #0x8]
1003700f4: b4000081    	cbz	x1, 0x100370104 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x1a4>
1003700f8: f9400b42    	ldr	x2, [x26, #0x10]
1003700fc: aa1803e0    	mov	x0, x24
100370100: 97f2451a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100370104: aa1703e0    	mov	x0, x23
100370108: 52800301    	mov	w1, #0x18               ; =24
10037010c: 52800102    	mov	w2, #0x8                ; =8
100370110: 97f24516    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100370114: d2800008    	mov	x8, #0x0                ; =0
100370118: 17ffffe4    	b	0x1003700a8 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x148>
10037011c: aa0003f5    	mov	x21, x0
100370120: f9400761    	ldr	x1, [x27, #0x8]
100370124: b4000121    	cbz	x1, 0x100370148 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x1e8>
100370128: f9400b62    	ldr	x2, [x27, #0x10]
10037012c: 14000005    	b	0x100370140 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x1e0>
100370130: aa0003f5    	mov	x21, x0
100370134: f9400741    	ldr	x1, [x26, #0x8]
100370138: b4000081    	cbz	x1, 0x100370148 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x1e8>
10037013c: f9400b42    	ldr	x2, [x26, #0x10]
100370140: aa1803e0    	mov	x0, x24
100370144: 97f24509    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100370148: aa1703e0    	mov	x0, x23
10037014c: 52800301    	mov	w1, #0x18               ; =24
100370150: 52800102    	mov	w2, #0x8                ; =8
100370154: 97f24505    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100370158: b50000b3    	cbnz	x19, 0x10037016c <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x20c>
10037015c: 14000008    	b	0x10037017c <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x21c>
100370160: 14000001    	b	0x100370164 <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x204>
100370164: aa0003f5    	mov	x21, x0
100370168: b40000b3    	cbz	x19, 0x10037017c <__ZN13codexmate_lib4core5relay13codex_catalog29current_bundled_cli_signature17hfa1444b69c8bfd22E+0x21c>
10037016c: aa1403e0    	mov	x0, x20
100370170: aa1303e1    	mov	x1, x19
100370174: 52800022    	mov	w2, #0x1                ; =1
100370178: 97f244fc    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10037017c: aa1503e0    	mov	x0, x21
100370180: 943983f5    	bl	0x1011d1154 <_writev+0x1011d1154>
