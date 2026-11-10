
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078c79c <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE>:
10078c79c: d10303ff    	sub	sp, sp, #0xc0
10078c7a0: a9066ffc    	stp	x28, x27, [sp, #0x60]
10078c7a4: a90767fa    	stp	x26, x25, [sp, #0x70]
10078c7a8: a9085ff8    	stp	x24, x23, [sp, #0x80]
10078c7ac: a90957f6    	stp	x22, x21, [sp, #0x90]
10078c7b0: a90a4ff4    	stp	x20, x19, [sp, #0xa0]
10078c7b4: a90b7bfd    	stp	x29, x30, [sp, #0xb0]
10078c7b8: 9102c3fd    	add	x29, sp, #0xb0
10078c7bc: aa0303f3    	mov	x19, x3
10078c7c0: f9000fe2    	str	x2, [sp, #0x18]
10078c7c4: aa0103e2    	mov	x2, x1
10078c7c8: aa0003e1    	mov	x1, x0
10078c7cc: aa0803f6    	mov	x22, x8
10078c7d0: 9100a3e0    	add	x0, sp, #0x28
10078c7d4: 97fffe12    	bl	0x10078c01c <__ZN13codexmate_lib4core5relay12codex_writer26top_level_assignment_lines17hffee39061cda3404E>
10078c7d8: a942dff5    	ldp	x21, x23, [sp, #0x28]
10078c7dc: f9401fe9    	ldr	x9, [sp, #0x38]
10078c7e0: 8b090528    	add	x8, x9, x9, lsl #1
10078c7e4: d37df114    	lsl	x20, x8, #3
10078c7e8: 8b1402ea    	add	x10, x23, x20
10078c7ec: a902dff7    	stp	x23, x23, [sp, #0x28]
10078c7f0: a903abf5    	stp	x21, x10, [sp, #0x38]
10078c7f4: aa1703e8    	mov	x8, x23
10078c7f8: b40006a9    	cbz	x9, 0x10078c8cc <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x130>
10078c7fc: a9005bea    	stp	x10, x22, [sp]
10078c800: f90013f3    	str	x19, [sp, #0x20]
10078c804: f9000bf5    	str	x21, [sp, #0x10]
10078c808: d2800018    	mov	x24, #0x0               ; =0
10078c80c: d2f00013    	mov	x19, #-0x8000000000000000 ; =-9223372036854775808
10078c810: 14000004    	b	0x10078c820 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x84>
10078c814: 91006318    	add	x24, x24, #0x18
10078c818: eb18029f    	cmp	x20, x24
10078c81c: 54000500    	b.eq	0x10078c8bc <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x120>
10078c820: 8b1802e8    	add	x8, x23, x24
10078c824: f9400115    	ldr	x21, [x8]
10078c828: eb1302bf    	cmp	x21, x19
10078c82c: 54000420    	b.eq	0x10078c8b0 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x114>
10078c830: a9408516    	ldp	x22, x1, [x8, #0x8]
10078c834: aa1603e0    	mov	x0, x22
10078c838: 97ea8a48    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
10078c83c: aa0003f9    	mov	x25, x0
10078c840: aa0103fa    	mov	x26, x1
10078c844: 910123e0    	add	x0, sp, #0x48
10078c848: aa1903e1    	mov	x1, x25
10078c84c: aa1a03e2    	mov	x2, x26
10078c850: 97fff984    	bl	0x10078ae60 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E>
10078c854: f94027fb    	ldr	x27, [sp, #0x48]
10078c858: eb13037f    	cmp	x27, x19
10078c85c: 540001e0    	b.eq	0x10078c898 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0xfc>
10078c860: f9402fe8    	ldr	x8, [sp, #0x58]
10078c864: f94013e2    	ldr	x2, [sp, #0x20]
10078c868: eb02011f    	cmp	x8, x2
10078c86c: 540000c1    	b.ne	0x10078c884 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0xe8>
10078c870: f9402bfc    	ldr	x28, [sp, #0x50]
10078c874: aa1c03e0    	mov	x0, x28
10078c878: f9400fe1    	ldr	x1, [sp, #0x18]
10078c87c: 94291389    	bl	0x1011d16a0 <_writev+0x1011d16a0>
10078c880: 34000680    	cbz	w0, 0x10078c950 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x1b4>
10078c884: b40000bb    	cbz	x27, 0x10078c898 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0xfc>
10078c888: f9402be0    	ldr	x0, [sp, #0x50]
10078c88c: aa1b03e1    	mov	x1, x27
10078c890: 52800022    	mov	w2, #0x1                ; =1
10078c894: 97e1d335    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078c898: b4fffbf5    	cbz	x21, 0x10078c814 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x78>
10078c89c: aa1603e0    	mov	x0, x22
10078c8a0: aa1503e1    	mov	x1, x21
10078c8a4: 52800022    	mov	w2, #0x1                ; =1
10078c8a8: 97e1d330    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078c8ac: 17ffffda    	b	0x10078c814 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x78>
10078c8b0: 91006108    	add	x8, x8, #0x18
10078c8b4: f94003ea    	ldr	x10, [sp]
10078c8b8: 14000003    	b	0x10078c8c4 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x128>
10078c8bc: f94003ea    	ldr	x10, [sp]
10078c8c0: aa0a03e8    	mov	x8, x10
10078c8c4: f9001be8    	str	x8, [sp, #0x30]
10078c8c8: a940d7f6    	ldp	x22, x21, [sp, #0x8]
10078c8cc: eb080149    	subs	x9, x10, x8
10078c8d0: 54000200    	b.eq	0x10078c910 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x174>
10078c8d4: b201f3ea    	mov	x10, #-0x5555555555555556 ; =-6148914691236517206
10078c8d8: f295556a    	movk	x10, #0xaaab
10078c8dc: 9bca7d29    	umulh	x9, x9, x10
10078c8e0: d344fd33    	lsr	x19, x9, #4
10078c8e4: 91002114    	add	x20, x8, #0x8
10078c8e8: 14000004    	b	0x10078c8f8 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x15c>
10078c8ec: 91006294    	add	x20, x20, #0x18
10078c8f0: f1000673    	subs	x19, x19, #0x1
10078c8f4: 540000e0    	b.eq	0x10078c910 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x174>
10078c8f8: f85f8281    	ldur	x1, [x20, #-0x8]
10078c8fc: b4ffff81    	cbz	x1, 0x10078c8ec <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x150>
10078c900: f9400280    	ldr	x0, [x20]
10078c904: 52800022    	mov	w2, #0x1                ; =1
10078c908: 97e1d318    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078c90c: 17fffff8    	b	0x10078c8ec <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x150>
10078c910: b40000d5    	cbz	x21, 0x10078c928 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x18c>
10078c914: 8b1506a8    	add	x8, x21, x21, lsl #1
10078c918: d37df101    	lsl	x1, x8, #3
10078c91c: aa1703e0    	mov	x0, x23
10078c920: 52800102    	mov	w2, #0x8                ; =8
10078c924: 97e1d311    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078c928: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078c92c: f90002c8    	str	x8, [x22]
10078c930: a94b7bfd    	ldp	x29, x30, [sp, #0xb0]
10078c934: a94a4ff4    	ldp	x20, x19, [sp, #0xa0]
10078c938: a94957f6    	ldp	x22, x21, [sp, #0x90]
10078c93c: a9485ff8    	ldp	x24, x23, [sp, #0x80]
10078c940: a94767fa    	ldp	x26, x25, [sp, #0x70]
10078c944: a9466ffc    	ldp	x28, x27, [sp, #0x60]
10078c948: 910303ff    	add	sp, sp, #0xc0
10078c94c: d65f03c0    	ret
10078c950: 8b1802f3    	add	x19, x23, x24
10078c954: 91006268    	add	x8, x19, #0x18
10078c958: f9001be8    	str	x8, [sp, #0x30]
10078c95c: b40000bb    	cbz	x27, 0x10078c970 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x1d4>
10078c960: aa1c03e0    	mov	x0, x28
10078c964: aa1b03e1    	mov	x1, x27
10078c968: 52800022    	mov	w2, #0x1                ; =1
10078c96c: 97e1d2ff    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078c970: f94007e0    	ldr	x0, [sp, #0x8]
10078c974: aa1903e1    	mov	x1, x25
10078c978: aa1a03e2    	mov	x2, x26
10078c97c: 97fff9d4    	bl	0x10078b0cc <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE>
10078c980: d100629a    	sub	x26, x20, #0x18
10078c984: b40000b5    	cbz	x21, 0x10078c998 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x1fc>
10078c988: aa1603e0    	mov	x0, x22
10078c98c: aa1503e1    	mov	x1, x21
10078c990: 52800022    	mov	w2, #0x1                ; =1
10078c994: 97e1d2f5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078c998: eb18035f    	cmp	x26, x24
10078c99c: f9400bf5    	ldr	x21, [sp, #0x10]
10078c9a0: 54000240    	b.eq	0x10078c9e8 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x24c>
10078c9a4: 91008273    	add	x19, x19, #0x20
10078c9a8: cb180288    	sub	x8, x20, x24
10078c9ac: d1006108    	sub	x8, x8, #0x18
10078c9b0: b201f3e9    	mov	x9, #-0x5555555555555556 ; =-6148914691236517206
10078c9b4: f2955569    	movk	x9, #0xaaab
10078c9b8: 9bc97d08    	umulh	x8, x8, x9
10078c9bc: d344fd14    	lsr	x20, x8, #4
10078c9c0: 14000004    	b	0x10078c9d0 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x234>
10078c9c4: 91006273    	add	x19, x19, #0x18
10078c9c8: f1000694    	subs	x20, x20, #0x1
10078c9cc: 540000e0    	b.eq	0x10078c9e8 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x24c>
10078c9d0: f85f8261    	ldur	x1, [x19, #-0x8]
10078c9d4: b4ffff81    	cbz	x1, 0x10078c9c4 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x228>
10078c9d8: f9400260    	ldr	x0, [x19]
10078c9dc: 52800022    	mov	w2, #0x1                ; =1
10078c9e0: 97e1d2e2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078c9e4: 17fffff8    	b	0x10078c9c4 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x228>
10078c9e8: b4fffa55    	cbz	x21, 0x10078c930 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x194>
10078c9ec: 8b1506a8    	add	x8, x21, x21, lsl #1
10078c9f0: d37df101    	lsl	x1, x8, #3
10078c9f4: aa1703e0    	mov	x0, x23
10078c9f8: 52800102    	mov	w2, #0x8                ; =8
10078c9fc: 97e1d2db    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078ca00: 17ffffcc    	b	0x10078c930 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x194>
10078ca04: aa0003f4    	mov	x20, x0
10078ca08: 14000005    	b	0x10078ca1c <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x280>
10078ca0c: aa0003f4    	mov	x20, x0
10078ca10: 8b1802e8    	add	x8, x23, x24
10078ca14: 91006108    	add	x8, x8, #0x18
10078ca18: f9001be8    	str	x8, [sp, #0x30]
10078ca1c: b40000b5    	cbz	x21, 0x10078ca30 <__ZN13codexmate_lib4core5relay12codex_writer27read_top_level_string_value17hb7b0b7867ffdda3cE+0x294>
10078ca20: aa1603e0    	mov	x0, x22
10078ca24: aa1503e1    	mov	x1, x21
10078ca28: 52800022    	mov	w2, #0x1                ; =1
10078ca2c: 97e1d2cf    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
