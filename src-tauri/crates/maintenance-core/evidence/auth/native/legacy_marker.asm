
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100201c80 <__ZN13codexmate_lib4core5relay19legacy_virtual_auth13marker_exists17h234ad5508a490929E>:
100201c80: d10343ff    	sub	sp, sp, #0xd0
100201c84: a90b4ff4    	stp	x20, x19, [sp, #0xb0]
100201c88: a90c7bfd    	stp	x29, x30, [sp, #0xc0]
100201c8c: 910303fd    	add	x29, sp, #0xc0
100201c90: f9412409    	ldr	x9, [x0, #0x248]
100201c94: f9412801    	ldr	x1, [x0, #0x250]
100201c98: b0008822    	adrp	x2, 0x101306000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.305+0x2bd>
100201c9c: 910b0042    	add	x2, x2, #0x2c0
100201ca0: 910003e8    	mov	x8, sp
100201ca4: aa0903e0    	mov	x0, x9
100201ca8: 52800303    	mov	w3, #0x18               ; =24
100201cac: 943d356d    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
100201cb0: a94087f3    	ldp	x19, x1, [sp, #0x8]
100201cb4: 910063e8    	add	x8, sp, #0x18
100201cb8: aa1303e0    	mov	x0, x19
100201cbc: 943d1a4f    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
100201cc0: a94187f4    	ldp	x20, x1, [sp, #0x18]
100201cc4: aa1403e0    	mov	x0, x20
100201cc8: 9400b1da    	bl	0x10022e430 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
100201ccc: f94003e1    	ldr	x1, [sp]
100201cd0: b4000081    	cbz	x1, 0x100201ce0 <__ZN13codexmate_lib4core5relay19legacy_virtual_auth13marker_exists17h234ad5508a490929E+0x60>
100201cd4: aa1303e0    	mov	x0, x19
100201cd8: 52800022    	mov	w2, #0x1                ; =1
100201cdc: 97f7fe23    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100201ce0: 52000280    	eor	w0, w20, #0x1
100201ce4: a94c7bfd    	ldp	x29, x30, [sp, #0xc0]
100201ce8: a94b4ff4    	ldp	x20, x19, [sp, #0xb0]
100201cec: 910343ff    	add	sp, sp, #0xd0
100201cf0: d65f03c0    	ret
100201cf4: aa0003f4    	mov	x20, x0
100201cf8: f94003e1    	ldr	x1, [sp]
100201cfc: b4000081    	cbz	x1, 0x100201d0c <__ZN13codexmate_lib4core5relay19legacy_virtual_auth13marker_exists17h234ad5508a490929E+0x8c>
100201d00: aa1303e0    	mov	x0, x19
100201d04: 52800022    	mov	w2, #0x1                ; =1
100201d08: 97f7fe18    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100201d0c: aa1403e0    	mov	x0, x20
100201d10: 943f3d11    	bl	0x1011d1154 <_writev+0x1011d1154>
