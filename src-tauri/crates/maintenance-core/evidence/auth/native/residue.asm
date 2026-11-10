
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008a89f8 <__ZN13codexmate_lib4core5relay18router_unlock_auth11has_residue17h150741b3202acaf4E>:
1008a89f8: d10383ff    	sub	sp, sp, #0xe0
1008a89fc: a90b57f6    	stp	x22, x21, [sp, #0xb0]
1008a8a00: a90c4ff4    	stp	x20, x19, [sp, #0xc0]
1008a8a04: a90d7bfd    	stp	x29, x30, [sp, #0xd0]
1008a8a08: 910343fd    	add	x29, sp, #0xd0
1008a8a0c: aa0003f3    	mov	x19, x0
1008a8a10: 94000030    	bl	0x1008a8ad0 <__ZN13codexmate_lib4core5relay18router_unlock_auth13marker_exists17h50c3f4cccfec5f10E>
1008a8a14: 37000420    	tbnz	w0, #0x0, 0x1008a8a98 <__ZN13codexmate_lib4core5relay18router_unlock_auth11has_residue17h150741b3202acaf4E+0xa0>
1008a8a18: f9412660    	ldr	x0, [x19, #0x248]
1008a8a1c: f9412a61    	ldr	x1, [x19, #0x250]
1008a8a20: 90005862    	adrp	x2, 0x1013b4000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.331+0x4>
1008a8a24: 912b3842    	add	x2, x2, #0xace
1008a8a28: 910003e8    	mov	x8, sp
1008a8a2c: 528003c3    	mov	w3, #0x1e               ; =30
1008a8a30: 94229a0c    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
1008a8a34: a94087f4    	ldp	x20, x1, [sp, #0x8]
1008a8a38: 910063e8    	add	x8, sp, #0x18
1008a8a3c: aa1403e0    	mov	x0, x20
1008a8a40: 94227eee    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
1008a8a44: a94187f5    	ldp	x21, x1, [sp, #0x18]
1008a8a48: aa1503e0    	mov	x0, x21
1008a8a4c: 9401461b    	bl	0x1008fa2b8 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
1008a8a50: 360001b5    	tbz	w21, #0x0, 0x1008a8a84 <__ZN13codexmate_lib4core5relay18router_unlock_auth11has_residue17h150741b3202acaf4E+0x8c>
1008a8a54: f94003e1    	ldr	x1, [sp]
1008a8a58: b4000081    	cbz	x1, 0x1008a8a68 <__ZN13codexmate_lib4core5relay18router_unlock_auth11has_residue17h150741b3202acaf4E+0x70>
1008a8a5c: aa1403e0    	mov	x0, x20
1008a8a60: 52800022    	mov	w2, #0x1                ; =1
1008a8a64: 97dd62c1    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a8a68: aa1303e0    	mov	x0, x19
1008a8a6c: 9400003e    	bl	0x1008a8b64 <__ZN13codexmate_lib4core5relay18router_unlock_auth15live_auth_state17hf9c8eec101776f31E>
1008a8a70: 12001c08    	and	w8, w0, #0xff
1008a8a74: 51000508    	sub	w8, w8, #0x1
1008a8a78: 7100091f    	cmp	w8, #0x2
1008a8a7c: 1a9f27e0    	cset	w0, lo
1008a8a80: 14000007    	b	0x1008a8a9c <__ZN13codexmate_lib4core5relay18router_unlock_auth11has_residue17h150741b3202acaf4E+0xa4>
1008a8a84: f94003e1    	ldr	x1, [sp]
1008a8a88: b4000081    	cbz	x1, 0x1008a8a98 <__ZN13codexmate_lib4core5relay18router_unlock_auth11has_residue17h150741b3202acaf4E+0xa0>
1008a8a8c: aa1403e0    	mov	x0, x20
1008a8a90: 52800022    	mov	w2, #0x1                ; =1
1008a8a94: 97dd62b5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a8a98: 52800020    	mov	w0, #0x1                ; =1
1008a8a9c: a94d7bfd    	ldp	x29, x30, [sp, #0xd0]
1008a8aa0: a94c4ff4    	ldp	x20, x19, [sp, #0xc0]
1008a8aa4: a94b57f6    	ldp	x22, x21, [sp, #0xb0]
1008a8aa8: 910383ff    	add	sp, sp, #0xe0
1008a8aac: d65f03c0    	ret
1008a8ab0: aa0003f3    	mov	x19, x0
1008a8ab4: f94003e1    	ldr	x1, [sp]
1008a8ab8: b4000081    	cbz	x1, 0x1008a8ac8 <__ZN13codexmate_lib4core5relay18router_unlock_auth11has_residue17h150741b3202acaf4E+0xd0>
1008a8abc: aa1403e0    	mov	x0, x20
1008a8ac0: 52800022    	mov	w2, #0x1                ; =1
1008a8ac4: 97dd62a9    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a8ac8: aa1303e0    	mov	x0, x19
1008a8acc: 9424a1a2    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001008a8ad0 <__ZN13codexmate_lib4core5relay18router_unlock_auth13marker_exists17h50c3f4cccfec5f10E>:
1008a8ad0: d10343ff    	sub	sp, sp, #0xd0
1008a8ad4: a90b4ff4    	stp	x20, x19, [sp, #0xb0]
1008a8ad8: a90c7bfd    	stp	x29, x30, [sp, #0xc0]
1008a8adc: 910303fd    	add	x29, sp, #0xc0
1008a8ae0: f9412409    	ldr	x9, [x0, #0x248]
1008a8ae4: f9412801    	ldr	x1, [x0, #0x250]
1008a8ae8: 90005862    	adrp	x2, 0x1013b4000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.331+0x4>
1008a8aec: 912bb042    	add	x2, x2, #0xaec
1008a8af0: 910003e8    	mov	x8, sp
1008a8af4: aa0903e0    	mov	x0, x9
1008a8af8: 528003c3    	mov	w3, #0x1e               ; =30
1008a8afc: 942299d9    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
1008a8b00: a94087f3    	ldp	x19, x1, [sp, #0x8]
1008a8b04: 910063e8    	add	x8, sp, #0x18
1008a8b08: aa1303e0    	mov	x0, x19
1008a8b0c: 94227ebb    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
1008a8b10: a94187f4    	ldp	x20, x1, [sp, #0x18]
1008a8b14: aa1403e0    	mov	x0, x20
1008a8b18: 940145e8    	bl	0x1008fa2b8 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
1008a8b1c: f94003e1    	ldr	x1, [sp]
1008a8b20: b4000081    	cbz	x1, 0x1008a8b30 <__ZN13codexmate_lib4core5relay18router_unlock_auth13marker_exists17h50c3f4cccfec5f10E+0x60>
1008a8b24: aa1303e0    	mov	x0, x19
1008a8b28: 52800022    	mov	w2, #0x1                ; =1
1008a8b2c: 97dd628f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a8b30: 52000280    	eor	w0, w20, #0x1
1008a8b34: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
1008a8b38: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
1008a8b3c: 910343ff    	add	sp, sp, #0xd0
1008a8b40: d65f03c0    	ret
1008a8b44: aa0003f4    	mov	x20, x0
1008a8b48: f94003e1    	ldr	x1, [sp]
1008a8b4c: b4000081    	cbz	x1, 0x1008a8b5c <__ZN13codexmate_lib4core5relay18router_unlock_auth13marker_exists17h50c3f4cccfec5f10E+0x8c>
1008a8b50: aa1303e0    	mov	x0, x19
1008a8b54: 52800022    	mov	w2, #0x1                ; =1
1008a8b58: 97dd6284    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008a8b5c: aa1403e0    	mov	x0, x20
1008a8b60: 9424a17d    	bl	0x1011d1154 <_writev+0x1011d1154>
