
AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001007be1e4 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE>:
1007be1e4: d10643ff    	sub	sp, sp, #0x190
1007be1e8: a91467fa    	stp	x26, x25, [sp, #0x140]
1007be1ec: a9155ff8    	stp	x24, x23, [sp, #0x150]
1007be1f0: a91657f6    	stp	x22, x21, [sp, #0x160]
1007be1f4: a9174ff4    	stp	x20, x19, [sp, #0x170]
1007be1f8: a9187bfd    	stp	x29, x30, [sp, #0x180]
1007be1fc: 910603fd    	add	x29, sp, #0x180
1007be200: 9101e3e8    	add	x8, sp, #0x78
1007be204: 940005e8    	bl	0x1007bf9a4 <__ZN13codexmate_lib8platform7process21codex_app_bundle_path17h6054073d42452650E>
1007be208: f9403ff4    	ldr	x20, [sp, #0x78]
1007be20c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1007be210: eb08029f    	cmp	x20, x8
1007be214: 54000360    	b.eq	0x1007be280 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x9c>
1007be218: a94807f5    	ldp	x21, x1, [sp, #0x80]
1007be21c: 910103e8    	add	x8, sp, #0x40
1007be220: aa1503e0    	mov	x0, x21
1007be224: 9400200c    	bl	0x1007c6254 <__ZN13codexmate_lib8platform7process34codex_macos_bundle_main_executable17h8de2843a138076adE>
1007be228: b40000b4    	cbz	x20, 0x1007be23c <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x58>
1007be22c: aa1503e0    	mov	x0, x21
1007be230: aa1403e1    	mov	x1, x20
1007be234: 52800022    	mov	w2, #0x1                ; =1
1007be238: 97e10ccc    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be23c: f94023f4    	ldr	x20, [sp, #0x40]
1007be240: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1007be244: eb08029f    	cmp	x20, x8
1007be248: 540001c0    	b.eq	0x1007be280 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x9c>
1007be24c: a94487f5    	ldp	x21, x1, [sp, #0x48]
1007be250: aa1503e0    	mov	x0, x21
1007be254: 94001344    	bl	0x1007c2f64 <__ZN13codexmate_lib8platform7process28codex_macos_main_process_pid17h0f1304b7ddc8901aE>
1007be258: aa0003f3    	mov	x19, x0
1007be25c: b40000b4    	cbz	x20, 0x1007be270 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x8c>
1007be260: aa1503e0    	mov	x0, x21
1007be264: aa1403e1    	mov	x1, x20
1007be268: 52800022    	mov	w2, #0x1                ; =1
1007be26c: 97e10cbf    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be270: 7100067f    	cmp	w19, #0x1
1007be274: 54000061    	b.ne	0x1007be280 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x9c>
1007be278: 52800020    	mov	w0, #0x1                ; =1
1007be27c: 14000043    	b	0x1007be388 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x1a4>
1007be280: f0005f60    	adrp	x0, 0x1013ad000 <_anon.6408540cdffbe318d49f52812e99783f.1143+0x1>
1007be284: 911f4000    	add	x0, x0, #0x7d0
1007be288: 9101e3e8    	add	x8, sp, #0x78
1007be28c: 52800241    	mov	w1, #0x12               ; =18
1007be290: 94263d39    	bl	0x10114d774 <__ZN3std3sys7process4unix6common7Command3new17hd0b854c5bb2c218dE>
1007be294: f0005f61    	adrp	x1, 0x1013ad000 <_anon.6408540cdffbe318d49f52812e99783f.1143+0x1>
1007be298: 911f8821    	add	x1, x1, #0x7e2
1007be29c: 9101e3e0    	add	x0, sp, #0x78
1007be2a0: 52800042    	mov	w2, #0x2                ; =2
1007be2a4: 94263cc5    	bl	0x10114d5b8 <__ZN3std3sys7process4unix6common7Command3arg17hb74bd81733e98035E>
1007be2a8: f0005f61    	adrp	x1, 0x1013ad000 <_anon.6408540cdffbe318d49f52812e99783f.1143+0x1>
1007be2ac: 9120f421    	add	x1, x1, #0x83d
1007be2b0: 9101e3e0    	add	x0, sp, #0x78
1007be2b4: 52800582    	mov	w2, #0x2c               ; =44
1007be2b8: 94263cc0    	bl	0x10114d5b8 <__ZN3std3sys7process4unix6common7Command3arg17hb74bd81733e98035E>
1007be2bc: 910103f3    	add	x19, sp, #0x40
1007be2c0: 910103e8    	add	x8, sp, #0x40
1007be2c4: 9101e3e0    	add	x0, sp, #0x78
1007be2c8: 94264f74    	bl	0x101152098 <__ZN3std7process7Command6output17h603175eca38db81eE>
1007be2cc: f94023f6    	ldr	x22, [sp, #0x40]
1007be2d0: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1007be2d4: eb0802df    	cmp	x22, x8
1007be2d8: 540002e1    	b.ne	0x1007be334 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x150>
1007be2dc: f94027f4    	ldr	x20, [sp, #0x48]
1007be2e0: 92400688    	and	x8, x20, #0x3
1007be2e4: f100051f    	cmp	x8, #0x1
1007be2e8: 54000201    	b.ne	0x1007be328 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x144>
1007be2ec: f85ffe95    	ldr	x21, [x20, #-0x1]!
1007be2f0: f9400696    	ldr	x22, [x20, #0x8]
1007be2f4: f94002c8    	ldr	x8, [x22]
1007be2f8: b4000068    	cbz	x8, 0x1007be304 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x120>
1007be2fc: aa1503e0    	mov	x0, x21
1007be300: d63f0100    	blr	x8
1007be304: f94006c1    	ldr	x1, [x22, #0x8]
1007be308: b4000081    	cbz	x1, 0x1007be318 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x134>
1007be30c: f9400ac2    	ldr	x2, [x22, #0x10]
1007be310: aa1503e0    	mov	x0, x21
1007be314: 97e10c95    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be318: aa1403e0    	mov	x0, x20
1007be31c: 52800301    	mov	w1, #0x18               ; =24
1007be320: 52800102    	mov	w2, #0x8                ; =8
1007be324: 97e10c91    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be328: 9101e3e0    	add	x0, sp, #0x78
1007be32c: 9400bbfe    	bl	0x1007ed324 <__ZN4core3ptr42drop_in_place$LT$std..process..Command$GT$17hb86c8f07849f97abE>
1007be330: 14000015    	b	0x1007be384 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x1a0>
1007be334: 3cc08261    	ldur	q1, [x19, #0x8]
1007be338: 3cc18260    	ldur	q0, [x19, #0x18]
1007be33c: ad0083e1    	stp	q1, q0, [sp, #0x10]
1007be340: 3cc28260    	ldur	q0, [x19, #0x28]
1007be344: 3c8303e0    	stur	q0, [sp, #0x30]
1007be348: f90007f6    	str	x22, [sp, #0x8]
1007be34c: 9101e3e0    	add	x0, sp, #0x78
1007be350: 9400bbf5    	bl	0x1007ed324 <__ZN4core3ptr42drop_in_place$LT$std..process..Command$GT$17hb86c8f07849f97abE>
1007be354: b9403be8    	ldr	w8, [sp, #0x38]
1007be358: 34000268    	cbz	w8, 0x1007be3a4 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x1c0>
1007be35c: b40000b6    	cbz	x22, 0x1007be370 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x18c>
1007be360: f9400be0    	ldr	x0, [sp, #0x10]
1007be364: aa1603e1    	mov	x1, x22
1007be368: 52800022    	mov	w2, #0x1                ; =1
1007be36c: 97e10c7f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be370: f94013e1    	ldr	x1, [sp, #0x20]
1007be374: b4000081    	cbz	x1, 0x1007be384 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x1a0>
1007be378: f94017e0    	ldr	x0, [sp, #0x28]
1007be37c: 52800022    	mov	w2, #0x1                ; =1
1007be380: 97e10c7a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be384: 52800000    	mov	w0, #0x0                ; =0
1007be388: a9587bfd    	ldp	x29, x30, [sp, #0x180]
1007be38c: a9574ff4    	ldp	x20, x19, [sp, #0x170]
1007be390: a95657f6    	ldp	x22, x21, [sp, #0x160]
1007be394: a9555ff8    	ldp	x24, x23, [sp, #0x150]
1007be398: a95467fa    	ldp	x26, x25, [sp, #0x140]
1007be39c: 910643ff    	add	sp, sp, #0x190
1007be3a0: d65f03c0    	ret
1007be3a4: a94107f7    	ldp	x23, x1, [sp, #0x10]
1007be3a8: 9101e3e8    	add	x8, sp, #0x78
1007be3ac: aa1703e0    	mov	x0, x23
1007be3b0: 9426eb8c    	bl	0x1011791e0 <__ZN5alloc6string6String15from_utf8_lossy17h03e7be16830322ceE>
1007be3b4: a94807f5    	ldp	x21, x1, [sp, #0x80]
1007be3b8: aa1503e0    	mov	x0, x21
1007be3bc: 97e9c367    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1007be3c0: aa0103f3    	mov	x19, x1
1007be3c4: b6f800c1    	tbz	x1, #0x3f, 0x1007be3dc <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x1f8>
1007be3c8: d2800019    	mov	x25, #0x0               ; =0
1007be3cc: aa1903e0    	mov	x0, x25
1007be3d0: aa1303e1    	mov	x1, x19
1007be3d4: 942848a4    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1007be3d8: d4200020    	brk	#0x1
1007be3dc: aa0003f8    	mov	x24, x0
1007be3e0: b4000373    	cbz	x19, 0x1007be44c <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x268>
1007be3e4: 97e10c64    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1007be3e8: 52800039    	mov	w25, #0x1               ; =1
1007be3ec: aa1303e0    	mov	x0, x19
1007be3f0: 52800021    	mov	w1, #0x1                ; =1
1007be3f4: 97e10c5c    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1007be3f8: b4fffea0    	cbz	x0, 0x1007be3cc <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x1e8>
1007be3fc: aa0003f4    	mov	x20, x0
1007be400: aa1803e1    	mov	x1, x24
1007be404: aa1303e2    	mov	x2, x19
1007be408: 94284ca9    	bl	0x1011d16ac <_writev+0x1011d16ac>
1007be40c: b40000b6    	cbz	x22, 0x1007be420 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x23c>
1007be410: aa1703e0    	mov	x0, x23
1007be414: aa1603e1    	mov	x1, x22
1007be418: 52800022    	mov	w2, #0x1                ; =1
1007be41c: 97e10c53    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be420: f94013e1    	ldr	x1, [sp, #0x20]
1007be424: b4000081    	cbz	x1, 0x1007be434 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x250>
1007be428: f94017e0    	ldr	x0, [sp, #0x28]
1007be42c: 52800022    	mov	w2, #0x1                ; =1
1007be430: 97e10c4e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be434: f9403fe1    	ldr	x1, [sp, #0x78]
1007be438: f240f83f    	tst	x1, #0x7fffffffffffffff
1007be43c: 54000161    	b.ne	0x1007be468 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x284>
1007be440: f100127f    	cmp	x19, #0x4
1007be444: 540005a1    	b.ne	0x1007be4f8 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x314>
1007be448: 1400000d    	b	0x1007be47c <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x298>
1007be44c: 52800034    	mov	w20, #0x1               ; =1
1007be450: aa1403e0    	mov	x0, x20
1007be454: aa1803e1    	mov	x1, x24
1007be458: aa1303e2    	mov	x2, x19
1007be45c: 94284c94    	bl	0x1011d16ac <_writev+0x1011d16ac>
1007be460: b5fffd96    	cbnz	x22, 0x1007be410 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x22c>
1007be464: 17ffffef    	b	0x1007be420 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x23c>
1007be468: aa1503e0    	mov	x0, x21
1007be46c: 52800022    	mov	w2, #0x1                ; =1
1007be470: 97e10c3e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be474: f100127f    	cmp	x19, #0x4
1007be478: 54000401    	b.ne	0x1007be4f8 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x314>
1007be47c: 39400289    	ldrb	w9, [x20]
1007be480: 5101052a    	sub	w10, w9, #0x41
1007be484: 52800408    	mov	w8, #0x20               ; =32
1007be488: 7100695f    	cmp	w10, #0x1a
1007be48c: 1a9f310a    	csel	w10, w8, wzr, lo
1007be490: 2a090149    	orr	w9, w10, w9
1007be494: 7101d13f    	cmp	w9, #0x74
1007be498: 54000301    	b.ne	0x1007be4f8 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x314>
1007be49c: 39400689    	ldrb	w9, [x20, #0x1]
1007be4a0: 5101052a    	sub	w10, w9, #0x41
1007be4a4: 7100695f    	cmp	w10, #0x1a
1007be4a8: 1a9f3108    	csel	w8, w8, wzr, lo
1007be4ac: 2a090108    	orr	w8, w8, w9
1007be4b0: 7101c91f    	cmp	w8, #0x72
1007be4b4: 54000221    	b.ne	0x1007be4f8 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x314>
1007be4b8: 39400a89    	ldrb	w9, [x20, #0x2]
1007be4bc: 5101052a    	sub	w10, w9, #0x41
1007be4c0: 52800408    	mov	w8, #0x20               ; =32
1007be4c4: 7100695f    	cmp	w10, #0x1a
1007be4c8: 1a9f310a    	csel	w10, w8, wzr, lo
1007be4cc: 2a090149    	orr	w9, w10, w9
1007be4d0: 7101d53f    	cmp	w9, #0x75
1007be4d4: 54000121    	b.ne	0x1007be4f8 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x314>
1007be4d8: 39400e89    	ldrb	w9, [x20, #0x3]
1007be4dc: 5101052a    	sub	w10, w9, #0x41
1007be4e0: 7100695f    	cmp	w10, #0x1a
1007be4e4: 1a9f3108    	csel	w8, w8, wzr, lo
1007be4e8: 2a090108    	orr	w8, w8, w9
1007be4ec: 7101951f    	cmp	w8, #0x65
1007be4f0: 1a9f17e0    	cset	w0, eq
1007be4f4: 14000002    	b	0x1007be4fc <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x318>
1007be4f8: 52800000    	mov	w0, #0x0                ; =0
1007be4fc: b4fff473    	cbz	x19, 0x1007be388 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x1a4>
1007be500: aa0003f5    	mov	x21, x0
1007be504: aa1403e0    	mov	x0, x20
1007be508: aa1303e1    	mov	x1, x19
1007be50c: 52800022    	mov	w2, #0x1                ; =1
1007be510: 97e10c16    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be514: aa1503e0    	mov	x0, x21
1007be518: 17ffff9c    	b	0x1007be388 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x1a4>
1007be51c: aa0003f3    	mov	x19, x0
1007be520: f94006c1    	ldr	x1, [x22, #0x8]
1007be524: b4000081    	cbz	x1, 0x1007be534 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x350>
1007be528: f9400ac2    	ldr	x2, [x22, #0x10]
1007be52c: aa1503e0    	mov	x0, x21
1007be530: 97e10c0e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be534: aa1403e0    	mov	x0, x20
1007be538: 52800301    	mov	w1, #0x18               ; =24
1007be53c: 52800102    	mov	w2, #0x8                ; =8
1007be540: 97e10c0a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be544: 14000010    	b	0x1007be584 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x3a0>
1007be548: 14000001    	b	0x1007be54c <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x368>
1007be54c: aa0003f3    	mov	x19, x0
1007be550: b40002f4    	cbz	x20, 0x1007be5ac <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x3c8>
1007be554: aa1503e0    	mov	x0, x21
1007be558: aa1403e1    	mov	x1, x20
1007be55c: 52800022    	mov	w2, #0x1                ; =1
1007be560: 97e10c02    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be564: aa1303e0    	mov	x0, x19
1007be568: 94284afb    	bl	0x1011d1154 <_writev+0x1011d1154>
1007be56c: aa0003f3    	mov	x19, x0
1007be570: 910023e0    	add	x0, sp, #0x8
1007be574: 9400cd25    	bl	0x1007f1a08 <__ZN4core3ptr72drop_in_place$LT$codexmate_lib..core..models..McpServerRemovePayload$GT$17hb32cb7675c618949E>
1007be578: aa1303e0    	mov	x0, x19
1007be57c: 94284af6    	bl	0x1011d1154 <_writev+0x1011d1154>
1007be580: aa0003f3    	mov	x19, x0
1007be584: 9101e3e0    	add	x0, sp, #0x78
1007be588: 9400bb67    	bl	0x1007ed324 <__ZN4core3ptr42drop_in_place$LT$std..process..Command$GT$17hb86c8f07849f97abE>
1007be58c: 14000008    	b	0x1007be5ac <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x3c8>
1007be590: 9428496e    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
1007be594: aa0003f3    	mov	x19, x0
1007be598: 910023e0    	add	x0, sp, #0x8
1007be59c: 9400cd1b    	bl	0x1007f1a08 <__ZN4core3ptr72drop_in_place$LT$codexmate_lib..core..models..McpServerRemovePayload$GT$17hb32cb7675c618949E>
1007be5a0: f9403fe1    	ldr	x1, [sp, #0x78]
1007be5a4: f240f83f    	tst	x1, #0x7fffffffffffffff
1007be5a8: 54000061    	b.ne	0x1007be5b4 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE+0x3d0>
1007be5ac: aa1303e0    	mov	x0, x19
1007be5b0: 94284ae9    	bl	0x1011d1154 <_writev+0x1011d1154>
1007be5b4: aa1503e0    	mov	x0, x21
1007be5b8: 52800022    	mov	w2, #0x1                ; =1
1007be5bc: 97e10beb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007be5c0: aa1303e0    	mov	x0, x19
1007be5c4: 94284ae4    	bl	0x1011d1154 <_writev+0x1011d1154>
