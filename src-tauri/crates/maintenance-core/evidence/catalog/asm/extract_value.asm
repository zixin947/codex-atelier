
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078b0cc <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE>:
10078b0cc: a9bb67fa    	stp	x26, x25, [sp, #-0x50]!
10078b0d0: a9015ff8    	stp	x24, x23, [sp, #0x10]
10078b0d4: a90257f6    	stp	x22, x21, [sp, #0x20]
10078b0d8: a9034ff4    	stp	x20, x19, [sp, #0x30]
10078b0dc: a9047bfd    	stp	x29, x30, [sp, #0x40]
10078b0e0: 910103fd    	add	x29, sp, #0x40
10078b0e4: d109c3ff    	sub	sp, sp, #0x270
10078b0e8: aa0203f4    	mov	x20, x2
10078b0ec: aa0103f5    	mov	x21, x1
10078b0f0: aa0003f3    	mov	x19, x0
10078b0f4: 9105e3e0    	add	x0, sp, #0x178
10078b0f8: 97ffff5a    	bl	0x10078ae60 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E>
10078b0fc: f940bfe9    	ldr	x9, [sp, #0x178]
10078b100: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078b104: eb08013f    	cmp	x9, x8
10078b108: 54000061    	b.ne	0x10078b114 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x48>
10078b10c: f9000268    	str	x8, [x19]
10078b110: 1400008f    	b	0x10078b34c <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x280>
10078b114: 910203f9    	add	x25, sp, #0x80
10078b118: 3ccf8320    	ldur	q0, [x25, #0xf8]
10078b11c: 3d8003e0    	str	q0, [sp]
10078b120: f940c7e8    	ldr	x8, [sp, #0x188]
10078b124: f9000be8    	str	x8, [sp, #0x10]
10078b128: 910203e8    	add	x8, sp, #0x80
10078b12c: aa1503e0    	mov	x0, x21
10078b130: aa1403e1    	mov	x1, x20
10078b134: 940fa418    	bl	0x100b74194 <__ZN74_$LT$toml_edit..de..Deserializer$u20$as$u20$core..str..traits..FromStr$GT$8from_str17h13cc4747bad7bb97E>
10078b138: a94827ea    	ldp	x10, x9, [sp, #0x80]
10078b13c: 394243e8    	ldrb	w8, [sp, #0x90]
10078b140: f841132b    	ldur	x11, [x25, #0x11]
10078b144: f9003beb    	str	x11, [sp, #0x70]
10078b148: a94987eb    	ldp	x11, x1, [sp, #0x98]
10078b14c: f80773eb    	stur	x11, [sp, #0x77]
10078b150: f94057e0    	ldr	x0, [sp, #0xa8]
10078b154: a94bd3f7    	ldp	x23, x20, [sp, #0xb8]
10078b158: a94cd7f8    	ldp	x24, x21, [sp, #0xc8]
10078b15c: f9406ff6    	ldr	x22, [sp, #0xd8]
10078b160: f100095f    	cmp	x10, #0x2
10078b164: 54000181    	b.ne	0x10078b194 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0xc8>
10078b168: f9403bea    	ldr	x10, [sp, #0x70]
10078b16c: f90033ea    	str	x10, [sp, #0x60]
10078b170: f84773ea    	ldur	x10, [sp, #0x77]
10078b174: f80673ea    	stur	x10, [sp, #0x67]
10078b178: f94033ea    	ldr	x10, [sp, #0x60]
10078b17c: f9002bea    	str	x10, [sp, #0x50]
10078b180: f84673ea    	ldur	x10, [sp, #0x67]
10078b184: f80573ea    	stur	x10, [sp, #0x57]
10078b188: f100093f    	cmp	x9, #0x2
10078b18c: 54000641    	b.ne	0x10078b254 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x188>
10078b190: 1400003d    	b	0x10078b284 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x1b8>
10078b194: 9105e3eb    	add	x11, sp, #0x178
10078b198: f94073ec    	ldr	x12, [sp, #0xe0]
10078b19c: f9405bed    	ldr	x13, [sp, #0xb0]
10078b1a0: 3ccb8320    	ldur	q0, [x25, #0xb8]
10078b1a4: 3c8b8160    	stur	q0, [x11, #0xb8]
10078b1a8: 3ccc8320    	ldur	q0, [x25, #0xc8]
10078b1ac: 3c8c8160    	stur	q0, [x11, #0xc8]
10078b1b0: 3ccd8320    	ldur	q0, [x25, #0xd8]
10078b1b4: 3c8d8160    	stur	q0, [x11, #0xd8]
10078b1b8: 3cce8320    	ldur	q0, [x25, #0xe8]
10078b1bc: 3c8e8160    	stur	q0, [x11, #0xe8]
10078b1c0: 3cc78320    	ldur	q0, [x25, #0x78]
10078b1c4: 3c878160    	stur	q0, [x11, #0x78]
10078b1c8: 3cc88320    	ldur	q0, [x25, #0x88]
10078b1cc: 3c888160    	stur	q0, [x11, #0x88]
10078b1d0: 3cc98320    	ldur	q0, [x25, #0x98]
10078b1d4: 3c898160    	stur	q0, [x11, #0x98]
10078b1d8: 3cca8320    	ldur	q0, [x25, #0xa8]
10078b1dc: 3c8a8160    	stur	q0, [x11, #0xa8]
10078b1e0: 3cc68320    	ldur	q0, [x25, #0x68]
10078b1e4: 3c868160    	stur	q0, [x11, #0x68]
10078b1e8: f9403bee    	ldr	x14, [sp, #0x70]
10078b1ec: f90033ee    	str	x14, [sp, #0x60]
10078b1f0: f84773ee    	ldur	x14, [sp, #0x77]
10078b1f4: f80673ee    	stur	x14, [sp, #0x67]
10078b1f8: a917a7ea    	stp	x10, x9, [sp, #0x178]
10078b1fc: 390623e8    	strb	w8, [sp, #0x188]
10078b200: f94033e8    	ldr	x8, [sp, #0x60]
10078b204: f8011168    	stur	x8, [x11, #0x11]
10078b208: a91907ee    	stp	x14, x1, [sp, #0x190]
10078b20c: a91a37e0    	stp	x0, x13, [sp, #0x1a0]
10078b210: a91b53f7    	stp	x23, x20, [sp, #0x1b0]
10078b214: a91c57f8    	stp	x24, x21, [sp, #0x1c0]
10078b218: a91d33f6    	stp	x22, x12, [sp, #0x1d0]
10078b21c: 910203e8    	add	x8, sp, #0x80
10078b220: 9105e3e0    	add	x0, sp, #0x178
10078b224: 94080ef3    	bl	0x10098edf0 <__ZN76_$LT$toml_edit..de..Deserializer$u20$as$u20$serde_core..de..Deserializer$GT$15deserialize_any17h9bb712022c4c455aE>
10078b228: f94043e9    	ldr	x9, [sp, #0x80]
10078b22c: 394223e8    	ldrb	w8, [sp, #0x88]
10078b230: f840932a    	ldur	x10, [x25, #0x9]
10078b234: f9002bea    	str	x10, [sp, #0x50]
10078b238: a94907ea    	ldp	x10, x1, [sp, #0x90]
10078b23c: f80573ea    	stur	x10, [sp, #0x57]
10078b240: f94053e0    	ldr	x0, [sp, #0xa0]
10078b244: f100093f    	cmp	x9, #0x2
10078b248: 54000141    	b.ne	0x10078b270 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x1a4>
10078b24c: f100093f    	cmp	x9, #0x2
10078b250: 540001a0    	b.eq	0x10078b284 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x1b8>
10078b254: b4000061    	cbz	x1, 0x10078b260 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x194>
10078b258: 52800022    	mov	w2, #0x1                ; =1
10078b25c: 97e1d8c3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078b260: f240fabf    	tst	x21, #0x7fffffffffffffff
10078b264: 540003a1    	b.ne	0x10078b2d8 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x20c>
10078b268: b5000438    	cbnz	x24, 0x10078b2ec <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x220>
10078b26c: 1400002b    	b	0x10078b318 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x24c>
10078b270: a94b53f7    	ldp	x23, x20, [sp, #0xb0]
10078b274: a94c57f8    	ldp	x24, x21, [sp, #0xc0]
10078b278: f9406bf6    	ldr	x22, [sp, #0xd0]
10078b27c: f100093f    	cmp	x9, #0x2
10078b280: 54fffea1    	b.ne	0x10078b254 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x188>
10078b284: f9402be9    	ldr	x9, [sp, #0x50]
10078b288: f90023e9    	str	x9, [sp, #0x40]
10078b28c: f84573e9    	ldur	x9, [sp, #0x57]
10078b290: f80473e9    	stur	x9, [sp, #0x47]
10078b294: 71001d1f    	cmp	w8, #0x7
10078b298: 540004c0    	b.eq	0x10078b330 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x264>
10078b29c: f94023e9    	ldr	x9, [sp, #0x40]
10078b2a0: f80213e9    	stur	x9, [sp, #0x21]
10078b2a4: f84473e9    	ldur	x9, [sp, #0x47]
10078b2a8: a90287e9    	stp	x9, x1, [sp, #0x28]
10078b2ac: f9001fe0    	str	x0, [sp, #0x38]
10078b2b0: 390083e8    	strb	w8, [sp, #0x20]
10078b2b4: 910003e0    	mov	x0, sp
10078b2b8: 910083e1    	add	x1, sp, #0x20
10078b2bc: 940b8476    	bl	0x100a6c494 <__ZN60_$LT$alloc..string..String$u20$as$u20$toml..value..Index$GT$5index17h140688f0b3cd0ee1E>
10078b2c0: b4000540    	cbz	x0, 0x10078b368 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x29c>
10078b2c4: 39400008    	ldrb	w8, [x0]
10078b2c8: 340005a8    	cbz	w8, 0x10078b37c <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x2b0>
10078b2cc: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078b2d0: f9000268    	str	x8, [x19]
10078b2d4: 1400003d    	b	0x10078b3c8 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x2fc>
10078b2d8: aa1603e0    	mov	x0, x22
10078b2dc: aa1503e1    	mov	x1, x21
10078b2e0: 52800022    	mov	w2, #0x1                ; =1
10078b2e4: 97e1d8a1    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078b2e8: b4000198    	cbz	x24, 0x10078b318 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x24c>
10078b2ec: 91002295    	add	x21, x20, #0x8
10078b2f0: 14000004    	b	0x10078b300 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x234>
10078b2f4: 910062b5    	add	x21, x21, #0x18
10078b2f8: f1000718    	subs	x24, x24, #0x1
10078b2fc: 540000e0    	b.eq	0x10078b318 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x24c>
10078b300: f85f82a1    	ldur	x1, [x21, #-0x8]
10078b304: b4ffff81    	cbz	x1, 0x10078b2f4 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x228>
10078b308: f94002a0    	ldr	x0, [x21]
10078b30c: 52800022    	mov	w2, #0x1                ; =1
10078b310: 97e1d896    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078b314: 17fffff8    	b	0x10078b2f4 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x228>
10078b318: b40000d7    	cbz	x23, 0x10078b330 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x264>
10078b31c: 8b1706e8    	add	x8, x23, x23, lsl #1
10078b320: d37df101    	lsl	x1, x8, #3
10078b324: aa1403e0    	mov	x0, x20
10078b328: 52800102    	mov	w2, #0x8                ; =8
10078b32c: 97e1d88f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078b330: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078b334: f9000268    	str	x8, [x19]
10078b338: f94003e1    	ldr	x1, [sp]
10078b33c: b4000081    	cbz	x1, 0x10078b34c <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x280>
10078b340: f94007e0    	ldr	x0, [sp, #0x8]
10078b344: 52800022    	mov	w2, #0x1                ; =1
10078b348: 97e1d888    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078b34c: 9109c3ff    	add	sp, sp, #0x270
10078b350: a9447bfd    	ldp	x29, x30, [sp, #0x40]
10078b354: a9434ff4    	ldp	x20, x19, [sp, #0x30]
10078b358: a94257f6    	ldp	x22, x21, [sp, #0x20]
10078b35c: a9415ff8    	ldp	x24, x23, [sp, #0x10]
10078b360: a8c567fa    	ldp	x26, x25, [sp], #0x50
10078b364: d65f03c0    	ret
10078b368: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078b36c: f9000268    	str	x8, [x19]
10078b370: 910083e0    	add	x0, sp, #0x20
10078b374: 94018690    	bl	0x1007ecdb4 <__ZN4core3ptr39drop_in_place$LT$toml..value..Value$GT$17h53736b29afe70b00E>
10078b378: 17fffff0    	b	0x10078b338 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x26c>
10078b37c: f9400c14    	ldr	x20, [x0, #0x18]
10078b380: b4fffa74    	cbz	x20, 0x10078b2cc <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x200>
10078b384: f9400815    	ldr	x21, [x0, #0x10]
10078b388: 9105e3e8    	add	x8, sp, #0x178
10078b38c: aa1403e0    	mov	x0, x20
10078b390: 52800001    	mov	w1, #0x0                ; =0
10078b394: 52800022    	mov	w2, #0x1                ; =1
10078b398: 52800023    	mov	w3, #0x1                ; =1
10078b39c: 97fa1877    	bl	0x100611578 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17hac6eac5846bb4dbdE>
10078b3a0: b9417be8    	ldr	w8, [sp, #0x178]
10078b3a4: f940c3f6    	ldr	x22, [sp, #0x180]
10078b3a8: 37000168    	tbnz	w8, #0x0, 0x10078b3d4 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x308>
10078b3ac: f940c7f7    	ldr	x23, [sp, #0x188]
10078b3b0: aa1703e0    	mov	x0, x23
10078b3b4: aa1503e1    	mov	x1, x21
10078b3b8: aa1403e2    	mov	x2, x20
10078b3bc: 942918bc    	bl	0x1011d16ac <_writev+0x1011d16ac>
10078b3c0: a9005e76    	stp	x22, x23, [x19]
10078b3c4: f9000a74    	str	x20, [x19, #0x10]
10078b3c8: 910083e0    	add	x0, sp, #0x20
10078b3cc: 9401867a    	bl	0x1007ecdb4 <__ZN4core3ptr39drop_in_place$LT$toml..value..Value$GT$17h53736b29afe70b00E>
10078b3d0: 17ffffda    	b	0x10078b338 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x26c>
10078b3d4: f940c7e1    	ldr	x1, [sp, #0x188]
10078b3d8: aa1603e0    	mov	x0, x22
10078b3dc: 942914a2    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10078b3e0: d4200020    	brk	#0x1
10078b3e4: aa0003f3    	mov	x19, x0
10078b3e8: 14000004    	b	0x10078b3f8 <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x32c>
10078b3ec: aa0003f3    	mov	x19, x0
10078b3f0: 910083e0    	add	x0, sp, #0x20
10078b3f4: 94018670    	bl	0x1007ecdb4 <__ZN4core3ptr39drop_in_place$LT$toml..value..Value$GT$17h53736b29afe70b00E>
10078b3f8: f94003e1    	ldr	x1, [sp]
10078b3fc: b4000081    	cbz	x1, 0x10078b40c <__ZN13codexmate_lib4core5relay12codex_writer25extract_toml_string_value17h181b010969815faeE+0x340>
10078b400: f94007e0    	ldr	x0, [sp, #0x8]
10078b404: 52800022    	mov	w2, #0x1                ; =1
10078b408: 97e1d858    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078b40c: aa1303e0    	mov	x0, x19
10078b410: 94291751    	bl	0x1011d1154 <_writev+0x1011d1154>
10078b414: 942915cd    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

000000010078b418 <__ZN13codexmate_lib4core5relay12codex_writer26is_aimami_relay_model_slug17h5d69a07fac09ef7fE>:
10078b418: f100303f    	cmp	x1, #0xc
10078b41c: 540001a1    	b.ne	0x10078b450 <__ZN13codexmate_lib4core5relay12codex_writer26is_aimami_relay_model_slug17h5d69a07fac09ef7fE+0x38>
10078b420: f9400008    	ldr	x8, [x0]
10078b424: b9400809    	ldr	w9, [x0, #0x8]
10078b428: d28d2c2a    	mov	x10, #0x6961            ; =26977
10078b42c: f2ac2daa    	movk	x10, #0x616d, lsl #16
10078b430: f2cd2daa    	movk	x10, #0x696d, lsl #32
10078b434: f2ee4bea    	movk	x10, #0x725f, lsl #48
10078b438: eb0a011f    	cmp	x8, x10
10078b43c: 528d8ca8    	mov	w8, #0x6c65             ; =27749
10078b440: 72af2c28    	movk	w8, #0x7961, lsl #16
10078b444: 7a480120    	ccmp	w9, w8, #0x0, eq
10078b448: 1a9f17e0    	cset	w0, eq
10078b44c: d65f03c0    	ret
10078b450: f100343f    	cmp	x1, #0xd
10078b454: 54000062    	b.hs	0x10078b460 <__ZN13codexmate_lib4core5relay12codex_writer26is_aimami_relay_model_slug17h5d69a07fac09ef7fE+0x48>
10078b458: 52800000    	mov	w0, #0x0                ; =0
10078b45c: d65f03c0    	ret
10078b460: f9400008    	ldr	x8, [x0]
10078b464: f8405009    	ldur	x9, [x0, #0x5]
10078b468: d28d2c2a    	mov	x10, #0x6961            ; =26977
10078b46c: f2ac2daa    	movk	x10, #0x616d, lsl #16
10078b470: f2cd2daa    	movk	x10, #0x696d, lsl #32
10078b474: f2ee4bea    	movk	x10, #0x725f, lsl #48
10078b478: eb0a011f    	cmp	x8, x10
10078b47c: d28bed28    	mov	x8, #0x5f69             ; =24425
10078b480: f2acae48    	movk	x8, #0x6572, lsl #16
10078b484: f2cc2d88    	movk	x8, #0x616c, lsl #32
10078b488: f2ebef28    	movk	x8, #0x5f79, lsl #48
10078b48c: fa480120    	ccmp	x9, x8, #0x0, eq
10078b490: 1a9f17e0    	cset	w0, eq
10078b494: d65f03c0    	ret

000000010078b498 <__ZN13codexmate_lib4core5relay12codex_writer25read_top_level_model_line17hb785f0da46cd98feE>:
10078b498: d10543ff    	sub	sp, sp, #0x150
10078b49c: 6d0e23e9    	stp	d9, d8, [sp, #0xe0]
10078b4a0: a90f6ffc    	stp	x28, x27, [sp, #0xf0]
10078b4a4: a91067fa    	stp	x26, x25, [sp, #0x100]
10078b4a8: a9115ff8    	stp	x24, x23, [sp, #0x110]
10078b4ac: a91257f6    	stp	x22, x21, [sp, #0x120]
10078b4b0: a9134ff4    	stp	x20, x19, [sp, #0x130]
10078b4b4: a9147bfd    	stp	x29, x30, [sp, #0x140]
10078b4b8: 910503fd    	add	x29, sp, #0x140
10078b4bc: aa0803f3    	mov	x19, x8
10078b4c0: 5280001c    	mov	w28, #0x0               ; =0
10078b4c4: 52800048    	mov	w8, #0x2                ; =2
10078b4c8: 39005fe8    	strb	w8, [sp, #0x17]
10078b4cc: a90187ff    	stp	xzr, x1, [sp, #0x18]
10078b4d0: 910183f8    	add	x24, sp, #0x60
10078b4d4: 528dedb9    	mov	w25, #0x6f6d            ; =28525
10078b4d8: 72acac99    	movk	w25, #0x6564, lsl #16
10078b4dc: 910063fa    	add	x26, sp, #0x18
10078b4e0: a90287e0    	stp	x0, x1, [sp, #0x28]
10078b4e4: 0f000540    	movi.2s	v0, #0xa
10078b4e8: 5280003b    	mov	w27, #0x1               ; =1
10078b4ec: a90387ff    	stp	xzr, x1, [sp, #0x38]
10078b4f0: fd0027e0    	str	d0, [sp, #0x48]
10078b4f4: 0f0107a8    	movi.2s	v8, #0x3d
10078b4f8: 390143fb    	strb	w27, [sp, #0x50]
10078b4fc: b0005bc8    	adrp	x8, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
10078b500: 3dc04900    	ldr	q0, [x8, #0x120]
10078b504: 3d8003e0    	str	q0, [sp]
10078b508: 7900b3ff    	strh	wzr, [sp, #0x58]
10078b50c: 14000004    	b	0x10078b51c <__ZN13codexmate_lib4core5relay12codex_writer25read_top_level_model_line17hb785f0da46cd98feE+0x84>
10078b510: 5280003c    	mov	w28, #0x1               ; =1
10078b514: 394167e8    	ldrb	w8, [sp, #0x59]
10078b518: 37001c28    	tbnz	w8, #0x0, 0x10078b89c <__ZN13codexmate_lib4core5relay12codex_writer25read_top_level_model_line17hb785f0da46cd98feE+0x404>
10078b51c: f94017f4    	ldr	x20, [sp, #0x28]
