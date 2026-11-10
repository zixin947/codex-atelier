
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001005c1f80 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E>:
1005c1f80: a9bc5ff8    	stp	x24, x23, [sp, #-0x40]!
1005c1f84: a90157f6    	stp	x22, x21, [sp, #0x10]
1005c1f88: a9024ff4    	stp	x20, x19, [sp, #0x20]
1005c1f8c: a9037bfd    	stp	x29, x30, [sp, #0x30]
1005c1f90: 9100c3fd    	add	x29, sp, #0x30
1005c1f94: d10c43ff    	sub	sp, sp, #0x310
1005c1f98: aa0803f3    	mov	x19, x8
1005c1f9c: 910863f4    	add	x20, sp, #0x218
1005c1fa0: 910483e8    	add	x8, sp, #0x120
1005c1fa4: 9416c87c    	bl	0x100b74194 <__ZN74_$LT$toml_edit..de..Deserializer$u20$as$u20$core..str..traits..FromStr$GT$8from_str17h13cc4747bad7bb97E>
1005c1fa8: a95223e9    	ldp	x9, x8, [sp, #0x120]
1005c1fac: ad4987e0    	ldp	q0, q1, [sp, #0x130]
1005c1fb0: ad0807e0    	stp	q0, q1, [sp, #0x100]
1005c1fb4: ad4a87e0    	ldp	q0, q1, [sp, #0x150]
1005c1fb8: ad0607e0    	stp	q0, q1, [sp, #0xc0]
1005c1fbc: 3dc05fe0    	ldr	q0, [sp, #0x170]
1005c1fc0: 3d803be0    	str	q0, [sp, #0xe0]
1005c1fc4: f940c3ea    	ldr	x10, [sp, #0x180]
1005c1fc8: f9007bea    	str	x10, [sp, #0xf0]
1005c1fcc: f100093f    	cmp	x9, #0x2
1005c1fd0: 540001c1    	b.ne	0x1005c2008 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x88>
1005c1fd4: ad4807e0    	ldp	q0, q1, [sp, #0x100]
1005c1fd8: ad0507e0    	stp	q0, q1, [sp, #0xa0]
1005c1fdc: ad460fe2    	ldp	q2, q3, [sp, #0xc0]
1005c1fe0: ad030fe2    	stp	q2, q3, [sp, #0x60]
1005c1fe4: 3dc03be4    	ldr	q4, [sp, #0xe0]
1005c1fe8: 3d8023e4    	str	q4, [sp, #0x80]
1005c1fec: f9407be9    	ldr	x9, [sp, #0xf0]
1005c1ff0: f9004be9    	str	x9, [sp, #0x90]
1005c1ff4: ad0207e0    	stp	q0, q1, [sp, #0x40]
1005c1ff8: f9001be9    	str	x9, [sp, #0x30]
1005c1ffc: ad0093e3    	stp	q3, q4, [sp, #0x10]
1005c2000: 3d8003e2    	str	q2, [sp]
1005c2004: 14000034    	b	0x1005c20d4 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x154>
1005c2008: 910483f5    	add	x21, sp, #0x120
1005c200c: 3ccb82a0    	ldur	q0, [x21, #0xb8]
1005c2010: 3c8b8280    	stur	q0, [x20, #0xb8]
1005c2014: 3ccc82a0    	ldur	q0, [x21, #0xc8]
1005c2018: 3c8c8280    	stur	q0, [x20, #0xc8]
1005c201c: 3ccd82a0    	ldur	q0, [x21, #0xd8]
1005c2020: 3c8d8280    	stur	q0, [x20, #0xd8]
1005c2024: 3cce82a0    	ldur	q0, [x21, #0xe8]
1005c2028: 3c8e8280    	stur	q0, [x20, #0xe8]
1005c202c: 3cc782a0    	ldur	q0, [x21, #0x78]
1005c2030: 3c878280    	stur	q0, [x20, #0x78]
1005c2034: 3cc882a0    	ldur	q0, [x21, #0x88]
1005c2038: 3c888280    	stur	q0, [x20, #0x88]
1005c203c: 3cc982a0    	ldur	q0, [x21, #0x98]
1005c2040: 3c898280    	stur	q0, [x20, #0x98]
1005c2044: 3cca82a0    	ldur	q0, [x21, #0xa8]
1005c2048: 3c8a8280    	stur	q0, [x20, #0xa8]
1005c204c: 3cc682a0    	ldur	q0, [x21, #0x68]
1005c2050: 3c868280    	stur	q0, [x20, #0x68]
1005c2054: ad4807e0    	ldp	q0, q1, [sp, #0x100]
1005c2058: ad0507e0    	stp	q0, q1, [sp, #0xa0]
1005c205c: ad460fe2    	ldp	q2, q3, [sp, #0xc0]
1005c2060: ad030fe2    	stp	q2, q3, [sp, #0x60]
1005c2064: 3dc03be2    	ldr	q2, [sp, #0xe0]
1005c2068: 3d8023e2    	str	q2, [sp, #0x80]
1005c206c: f9407bea    	ldr	x10, [sp, #0xf0]
1005c2070: f9004bea    	str	x10, [sp, #0x90]
1005c2074: f9010fe9    	str	x9, [sp, #0x218]
1005c2078: f90113e8    	str	x8, [sp, #0x220]
1005c207c: ad008680    	stp	q0, q1, [x20, #0x10]
1005c2080: ad4307e0    	ldp	q0, q1, [sp, #0x60]
1005c2084: 3dc023e2    	ldr	q2, [sp, #0x80]
1005c2088: f9013fea    	str	x10, [sp, #0x278]
1005c208c: ad020a81    	stp	q1, q2, [x20, #0x40]
1005c2090: 3d800e80    	str	q0, [x20, #0x30]
1005c2094: 910483e8    	add	x8, sp, #0x120
1005c2098: 910863e0    	add	x0, sp, #0x218
1005c209c: 940f3355    	bl	0x10098edf0 <__ZN76_$LT$toml_edit..de..Deserializer$u20$as$u20$serde_core..de..Deserializer$GT$15deserialize_any17h9bb712022c4c455aE>
1005c20a0: f94093e8    	ldr	x8, [sp, #0x120]
1005c20a4: 3cc082a0    	ldur	q0, [x21, #0x8]
1005c20a8: 3cc182a1    	ldur	q1, [x21, #0x18]
1005c20ac: ad0207e0    	stp	q0, q1, [sp, #0x40]
1005c20b0: f100091f    	cmp	x8, #0x2
1005c20b4: 54000100    	b.eq	0x1005c20d4 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x154>
1005c20b8: 3cc282a0    	ldur	q0, [x21, #0x28]
1005c20bc: 3cc382a1    	ldur	q1, [x21, #0x38]
1005c20c0: ad0007e0    	stp	q0, q1, [sp]
1005c20c4: 3cc482a0    	ldur	q0, [x21, #0x48]
1005c20c8: 3d800be0    	str	q0, [sp, #0x20]
1005c20cc: f940bfe9    	ldr	x9, [sp, #0x178]
1005c20d0: f9001be9    	str	x9, [sp, #0x30]
1005c20d4: f100091f    	cmp	x8, #0x2
1005c20d8: 540000c1    	b.ne	0x1005c20f0 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x170>
1005c20dc: 910103e0    	add	x0, sp, #0x40
1005c20e0: 9400ef99    	bl	0x1005fdf44 <__ZN4core3ptr39drop_in_place$LT$toml..value..Value$GT$17h53736b29afe70b00E>
1005c20e4: 52800168    	mov	w8, #0xb                ; =11
1005c20e8: f9000268    	str	x8, [x19]
1005c20ec: 14000045    	b	0x1005c2200 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x280>
1005c20f0: ad4207e0    	ldp	q0, q1, [sp, #0x40]
1005c20f4: 3c808280    	stur	q0, [x20, #0x8]
1005c20f8: 3c818281    	stur	q1, [x20, #0x18]
1005c20fc: ad4007e0    	ldp	q0, q1, [sp]
1005c2100: 3c828280    	stur	q0, [x20, #0x28]
1005c2104: 3c838281    	stur	q1, [x20, #0x38]
1005c2108: 3dc00be0    	ldr	q0, [sp, #0x20]
1005c210c: 3c848280    	stur	q0, [x20, #0x48]
1005c2110: f9401be9    	ldr	x9, [sp, #0x30]
1005c2114: f9013be9    	str	x9, [sp, #0x270]
1005c2118: f9010fe8    	str	x8, [sp, #0x218]
1005c211c: 97e8fd16    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1005c2120: 52800d00    	mov	w0, #0x68               ; =104
1005c2124: 52800021    	mov	w1, #0x1                ; =1
1005c2128: 97e8fd0f    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1005c212c: b4000760    	cbz	x0, 0x1005c2218 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x298>
1005c2130: aa0003f4    	mov	x20, x0
1005c2134: f0006ec8    	adrp	x8, 0x10139d000 <_anon.62728874ff9674d7d8f5de25cb1e85f1.196+0x81f4a>
1005c2138: 912e2908    	add	x8, x8, #0xb8a
1005c213c: ad420500    	ldp	q0, q1, [x8, #0x40]
1005c2140: ad020400    	stp	q0, q1, [x0, #0x40]
1005c2144: f9403109    	ldr	x9, [x8, #0x60]
1005c2148: f9003009    	str	x9, [x0, #0x60]
1005c214c: ad400500    	ldp	q0, q1, [x8]
1005c2150: ad000400    	stp	q0, q1, [x0]
1005c2154: ad410101    	ldp	q1, q0, [x8, #0x20]
1005c2158: ad010001    	stp	q1, q0, [x0, #0x20]
1005c215c: f9411be1    	ldr	x1, [sp, #0x230]
1005c2160: b4000081    	cbz	x1, 0x1005c2170 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x1f0>
1005c2164: f9411fe0    	ldr	x0, [sp, #0x238]
1005c2168: 52800022    	mov	w2, #0x1                ; =1
1005c216c: 97e8fcff    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005c2170: f94133e1    	ldr	x1, [sp, #0x260]
1005c2174: f240f83f    	tst	x1, #0x7fffffffffffffff
1005c2178: 540000a1    	b.ne	0x1005c218c <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x20c>
1005c217c: f9412bf5    	ldr	x21, [sp, #0x250]
1005c2180: f9412ff6    	ldr	x22, [sp, #0x258]
1005c2184: b5000116    	cbnz	x22, 0x1005c21a4 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x224>
1005c2188: 14000012    	b	0x1005c21d0 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x250>
1005c218c: f94137e0    	ldr	x0, [sp, #0x268]
1005c2190: 52800022    	mov	w2, #0x1                ; =1
1005c2194: 97e8fcf5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005c2198: f9412bf5    	ldr	x21, [sp, #0x250]
1005c219c: f9412ff6    	ldr	x22, [sp, #0x258]
1005c21a0: b4000196    	cbz	x22, 0x1005c21d0 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x250>
1005c21a4: 910022b7    	add	x23, x21, #0x8
1005c21a8: 14000004    	b	0x1005c21b8 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x238>
1005c21ac: 910062f7    	add	x23, x23, #0x18
1005c21b0: f10006d6    	subs	x22, x22, #0x1
1005c21b4: 540000e0    	b.eq	0x1005c21d0 <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x250>
1005c21b8: f85f82e1    	ldur	x1, [x23, #-0x8]
1005c21bc: b4ffff81    	cbz	x1, 0x1005c21ac <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x22c>
1005c21c0: f94002e0    	ldr	x0, [x23]
1005c21c4: 52800022    	mov	w2, #0x1                ; =1
1005c21c8: 97e8fce8    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005c21cc: 17fffff8    	b	0x1005c21ac <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x22c>
1005c21d0: f94127e8    	ldr	x8, [sp, #0x248]
1005c21d4: b40000c8    	cbz	x8, 0x1005c21ec <__ZN13codexmate_lib4core12codex_config18validate_candidate17h7e43e53c51b47a30E+0x26c>
1005c21d8: 8b080508    	add	x8, x8, x8, lsl #1
1005c21dc: d37df101    	lsl	x1, x8, #3
1005c21e0: aa1503e0    	mov	x0, x21
1005c21e4: 52800102    	mov	w2, #0x8                ; =8
1005c21e8: 97e8fce0    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005c21ec: f0006ee8    	adrp	x8, 0x1013a1000 <_anon.62728874ff9674d7d8f5de25cb1e85f1.1777+0x221>
1005c21f0: 3dc01900    	ldr	q0, [x8, #0x60]
1005c21f4: 3d800260    	str	q0, [x19]
1005c21f8: 52800d08    	mov	w8, #0x68               ; =104
1005c21fc: a9012274    	stp	x20, x8, [x19, #0x10]
1005c2200: 910c43ff    	add	sp, sp, #0x310
1005c2204: a9437bfd    	ldp	x29, x30, [sp, #0x30]
1005c2208: a9424ff4    	ldp	x20, x19, [sp, #0x20]
1005c220c: a94157f6    	ldp	x22, x21, [sp, #0x10]
1005c2210: a8c45ff8    	ldp	x24, x23, [sp], #0x40
1005c2214: d65f03c0    	ret
1005c2218: 52800020    	mov	w0, #0x1                ; =1
1005c221c: 52800d01    	mov	w1, #0x68               ; =104
1005c2220: 94303911    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1005c2224: d4200020    	brk	#0x1
1005c2228: aa0003f3    	mov	x19, x0
1005c222c: 910863e0    	add	x0, sp, #0x218
1005c2230: 9400f15e    	bl	0x1005fe7a8 <__ZN4core3ptr41drop_in_place$LT$toml_edit..de..Error$GT$17h502d262ad67279c3E>
1005c2234: aa1303e0    	mov	x0, x19
1005c2238: 94303bc7    	bl	0x1011d1154 <_writev+0x1011d1154>
