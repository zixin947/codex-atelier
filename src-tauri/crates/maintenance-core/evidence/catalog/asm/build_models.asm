
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100367658 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE>:
100367658: 6db82beb    	stp	d11, d10, [sp, #-0x80]!
10036765c: 6d0123e9    	stp	d9, d8, [sp, #0x10]
100367660: a9026ffc    	stp	x28, x27, [sp, #0x20]
100367664: a90367fa    	stp	x26, x25, [sp, #0x30]
100367668: a9045ff8    	stp	x24, x23, [sp, #0x40]
10036766c: a90557f6    	stp	x22, x21, [sp, #0x50]
100367670: a9064ff4    	stp	x20, x19, [sp, #0x60]
100367674: a9077bfd    	stp	x29, x30, [sp, #0x70]
100367678: 9101c3fd    	add	x29, sp, #0x70
10036767c: d10703ff    	sub	sp, sp, #0x1c0
100367680: f90023e4    	str	x4, [sp, #0x40]
100367684: a9057fe0    	stp	x0, xzr, [sp, #0x50]
100367688: f90017e8    	str	x8, [sp, #0x28]
10036768c: 52800108    	mov	w8, #0x8                ; =8
100367690: a9067fe8    	stp	x8, xzr, [sp, #0x60]
100367694: b4003063    	cbz	x3, 0x100367ca0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x648>
100367698: b4003041    	cbz	x1, 0x100367ca0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x648>
10036769c: aa0503f4    	mov	x20, x5
1003676a0: aa0203f6    	mov	x22, x2
1003676a4: 52800308    	mov	w8, #0x18               ; =24
1003676a8: 9b080869    	madd	x9, x3, x8, x2
1003676ac: d378dc28    	lsl	x8, x1, #8
1003676b0: a90323e9    	stp	x9, x8, [sp, #0x30]
1003676b4: d37be8a9    	lsl	x9, x5, #5
1003676b8: f94023e8    	ldr	x8, [sp, #0x40]
1003676bc: f9000fe9    	str	x9, [sp, #0x18]
1003676c0: 8b090108    	add	x8, x8, x9
1003676c4: f90027e8    	str	x8, [sp, #0x48]
1003676c8: 0f05e7e8    	movi.8b	v8, #0xbf
1003676cc: 0f00e749    	movi.8b	v9, #0x1a
1003676d0: 0f01e40a    	movi.8b	v10, #0x20
1003676d4: f0007d59    	adrp	x25, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
1003676d8: 913c5339    	add	x25, x25, #0xf14
1003676dc: 14000006    	b	0x1003676f4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x9c>
1003676e0: 9101c3e0    	add	x0, sp, #0x70
1003676e4: 9407fd89    	bl	0x100566d08 <__ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8b7cbd6ddb489c57E>
1003676e8: f9401be8    	ldr	x8, [sp, #0x30]
1003676ec: eb0802df    	cmp	x22, x8
1003676f0: 54002d80    	b.eq	0x100367ca0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x648>
1003676f4: aa1603f3    	mov	x19, x22
1003676f8: 910062d6    	add	x22, x22, #0x18
1003676fc: f9400a75    	ldr	x21, [x19, #0x10]
100367700: f9401ff7    	ldr	x23, [sp, #0x38]
100367704: f9402bfa    	ldr	x26, [sp, #0x50]
100367708: 14000004    	b	0x100367718 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0xc0>
10036770c: 9104035a    	add	x26, x26, #0x100
100367710: f10402f7    	subs	x23, x23, #0x100
100367714: 54fffea0    	b.eq	0x1003676e8 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x90>
100367718: f9400b48    	ldr	x8, [x26, #0x10]
10036771c: eb15011f    	cmp	x8, x21
100367720: 54ffff61    	b.ne	0x10036770c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0xb4>
100367724: f9400661    	ldr	x1, [x19, #0x8]
100367728: f9400740    	ldr	x0, [x26, #0x8]
10036772c: aa1503e2    	mov	x2, x21
100367730: 9439a7dc    	bl	0x1011d16a0 <_writev+0x1011d16a0>
100367734: 35fffec0    	cbnz	w0, 0x10036770c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0xb4>
100367738: 9103c3e8    	add	x8, sp, #0xf0
10036773c: aa1a03e0    	mov	x0, x26
100367740: 9410c6e6    	bl	0x1007992d8 <__ZN13codexmate_lib4core5relay6models13RelayProvider14exposed_models17h78c65a493568f3aaE>
100367744: a94f27e8    	ldp	x8, x9, [sp, #0xf0]
100367748: f94083ea    	ldr	x10, [sp, #0x100]
10036774c: 52800c0b    	mov	w11, #0x60              ; =96
100367750: 9b0b254b    	madd	x11, x10, x11, x9
100367754: a90727e9    	stp	x9, x9, [sp, #0x70]
100367758: a9082fe8    	stp	x8, x11, [sp, #0x80]
10036775c: f90013eb    	str	x11, [sp, #0x20]
100367760: b4fffc0a    	cbz	x10, 0x1003676e0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x88>
100367764: 91018137    	add	x23, x9, #0x60
100367768: f9003ff7    	str	x23, [sp, #0x78]
10036776c: f9400128    	ldr	x8, [x9]
100367770: 3cc28120    	ldur	q0, [x9, #0x28]
100367774: 3cc38121    	ldur	q1, [x9, #0x38]
100367778: ad0587e0    	stp	q0, q1, [sp, #0xb0]
10036777c: 3cc48120    	ldur	q0, [x9, #0x48]
100367780: 3d8037e0    	str	q0, [sp, #0xd0]
100367784: f9402d2a    	ldr	x10, [x9, #0x58]
100367788: f90073ea    	str	x10, [sp, #0xe0]
10036778c: 3cc08120    	ldur	q0, [x9, #0x8]
100367790: 3cc18121    	ldur	q1, [x9, #0x18]
100367794: ad0487e0    	stp	q0, q1, [sp, #0x90]
100367798: f100091f    	cmp	x8, #0x2
10036779c: 54fffa20    	b.eq	0x1003676e0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x88>
1003677a0: ad4483e1    	ldp	q1, q0, [sp, #0x90]
1003677a4: 9103c3e9    	add	x9, sp, #0xf0
1003677a8: 3c818120    	stur	q0, [x9, #0x18]
1003677ac: ad460be0    	ldp	q0, q2, [sp, #0xc0]
1003677b0: 3c848122    	stur	q2, [x9, #0x48]
1003677b4: 3dc02fe2    	ldr	q2, [sp, #0xb0]
1003677b8: 3c838120    	stur	q0, [x9, #0x38]
1003677bc: 3c828122    	stur	q2, [x9, #0x28]
1003677c0: f9007be8    	str	x8, [sp, #0xf0]
1003677c4: f94073e8    	ldr	x8, [sp, #0xe0]
1003677c8: f8058128    	stur	x8, [x9, #0x58]
1003677cc: 3c808121    	stur	q1, [x9, #0x8]
1003677d0: a950eff8    	ldp	x24, x27, [sp, #0x108]
1003677d4: b4000334    	cbz	x20, 0x100367838 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x1e0>
1003677d8: f9400ff3    	ldr	x19, [sp, #0x18]
1003677dc: f94023fc    	ldr	x28, [sp, #0x40]
1003677e0: 14000004    	b	0x1003677f0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x198>
1003677e4: 9100839c    	add	x28, x28, #0x20
1003677e8: f1008273    	subs	x19, x19, #0x20
1003677ec: 54000260    	b.eq	0x100367838 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x1e0>
1003677f0: aa1903e0    	mov	x0, x25
1003677f4: 52800081    	mov	w1, #0x4                ; =4
1003677f8: aa1c03e2    	mov	x2, x28
1003677fc: 94369ce5    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
100367800: b4ffff20    	cbz	x0, 0x1003677e4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x18c>
100367804: 39400008    	ldrb	w8, [x0]
100367808: 71000d1f    	cmp	w8, #0x3
10036780c: 54fffec1    	b.ne	0x1003677e4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x18c>
100367810: f9400c08    	ldr	x8, [x0, #0x18]
100367814: eb1b011f    	cmp	x8, x27
100367818: 54fffe61    	b.ne	0x1003677e4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x18c>
10036781c: f9400800    	ldr	x0, [x0, #0x10]
100367820: aa1803e1    	mov	x1, x24
100367824: aa1b03e2    	mov	x2, x27
100367828: 9439a79e    	bl	0x1011d16a0 <_writev+0x1011d16a0>
10036782c: 35fffdc0    	cbnz	w0, 0x1003677e4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x18c>
100367830: aa1c03e4    	mov	x4, x28
100367834: 140000de    	b	0x100367bac <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x554>
100367838: d10383a0    	sub	x0, x29, #0xe0
10036783c: aa1803e1    	mov	x1, x24
100367840: aa1b03e2    	mov	x2, x27
100367844: 97fffee3    	bl	0x1003673d0 <__ZN13codexmate_lib4core5relay13codex_catalog16model_tail_lower17hdcbb168b8679db13E>
100367848: a972a3a0    	ldp	x0, x8, [x29, #-0xd8]
10036784c: f1001d1f    	cmp	x8, #0x7
100367850: 540001e1    	b.ne	0x10036788c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x234>
100367854: b9400008    	ldr	w8, [x0]
100367858: b8403009    	ldur	w9, [x0, #0x3]
10036785c: 528e0cea    	mov	w10, #0x7067            ; =28775
100367860: 72a5ae8a    	movk	w10, #0x2d74, lsl #16
100367864: 6b0a011f    	cmp	w8, w10
100367868: 5286a5a8    	mov	w8, #0x352d             ; =13613
10036786c: 72a6c5c8    	movk	w8, #0x362e, lsl #16
100367870: 7a480120    	ccmp	w9, w8, #0x0, eq
100367874: 54000101    	b.ne	0x100367894 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x23c>
100367878: f85203a1    	ldur	x1, [x29, #-0xe0]
10036787c: b4000181    	cbz	x1, 0x1003678ac <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x254>
100367880: 52800022    	mov	w2, #0x1                ; =1
100367884: 97f26739    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367888: 14000009    	b	0x1003678ac <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x254>
10036788c: f100211f    	cmp	x8, #0x8
100367890: 54001de2    	b.hs	0x100367c4c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x5f4>
100367894: 52800013    	mov	w19, #0x0               ; =0
100367898: f85203a1    	ldur	x1, [x29, #-0xe0]
10036789c: b4000061    	cbz	x1, 0x1003678a8 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x250>
1003678a0: 52800022    	mov	w2, #0x1                ; =1
1003678a4: 97f26731    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003678a8: 36000173    	tbz	w19, #0x0, 0x1003678d4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x27c>
1003678ac: d10303a0    	sub	x0, x29, #0xc0
1003678b0: aa1803e1    	mov	x1, x24
1003678b4: aa1b03e2    	mov	x2, x27
1003678b8: 97fffec6    	bl	0x1003673d0 <__ZN13codexmate_lib4core5relay13codex_catalog16model_tail_lower17hdcbb168b8679db13E>
1003678bc: f94027e9    	ldr	x9, [sp, #0x48]
1003678c0: d10303a8    	sub	x8, x29, #0xc0
1003678c4: a937a3a9    	stp	x9, x8, [x29, #-0x88]
1003678c8: b4001614    	cbz	x20, 0x100367b88 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x530>
1003678cc: f94023f3    	ldr	x19, [sp, #0x40]
1003678d0: 14000008    	b	0x1003678f0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x298>
1003678d4: d280001c    	mov	x28, #0x0               ; =0
1003678d8: d2800004    	mov	x4, #0x0                ; =0
1003678dc: 140000b4    	b	0x100367bac <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x554>
1003678e0: f94007f4    	ldr	x20, [sp, #0x8]
1003678e4: f94027e8    	ldr	x8, [sp, #0x48]
1003678e8: eb08027f    	cmp	x19, x8
1003678ec: 540014e0    	b.eq	0x100367b88 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x530>
1003678f0: aa1303fc    	mov	x28, x19
1003678f4: aa1903e0    	mov	x0, x25
1003678f8: 52800081    	mov	w1, #0x4                ; =4
1003678fc: aa1303e2    	mov	x2, x19
100367900: 94369ca4    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
100367904: 91008393    	add	x19, x28, #0x20
100367908: b4fffee0    	cbz	x0, 0x1003678e4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x28c>
10036790c: 39400008    	ldrb	w8, [x0]
100367910: 71000d1f    	cmp	w8, #0x3
100367914: 54fffe81    	b.ne	0x1003678e4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x28c>
100367918: a9410408    	ldp	x8, x1, [x0, #0x10]
10036791c: aa0803e0    	mov	x0, x8
100367920: 97fb1e0e    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100367924: aa0103fb    	mov	x27, x1
100367928: b7f81da1    	tbnz	x1, #0x3f, 0x100367cdc <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x684>
10036792c: aa0003f5    	mov	x21, x0
100367930: b40001fb    	cbz	x27, 0x10036796c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x314>
100367934: 97f26710    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100367938: aa1b03e0    	mov	x0, x27
10036793c: 52800021    	mov	w1, #0x1                ; =1
100367940: 97f26709    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100367944: b4001d40    	cbz	x0, 0x100367cec <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x694>
100367948: aa0003f8    	mov	x24, x0
10036794c: aa1503e1    	mov	x1, x21
100367950: aa1b03e2    	mov	x2, x27
100367954: 9439a756    	bl	0x1011d16ac <_writev+0x1011d16ac>
100367958: f100237f    	cmp	x27, #0x8
10036795c: 54000162    	b.hs	0x100367988 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x330>
100367960: d2800008    	mov	x8, #0x0                ; =0
100367964: 5280040b    	mov	w11, #0x20              ; =32
100367968: 1400003f    	b	0x100367a64 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x40c>
10036796c: 52800020    	mov	w0, #0x1                ; =1
100367970: aa1503e1    	mov	x1, x21
100367974: aa1b03e2    	mov	x2, x27
100367978: 9439a74d    	bl	0x1011d16ac <_writev+0x1011d16ac>
10036797c: 52800038    	mov	w24, #0x1               ; =1
100367980: 52800035    	mov	w21, #0x1               ; =1
100367984: 14000042    	b	0x100367a8c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x434>
100367988: f101037f    	cmp	x27, #0x40
10036798c: 5280040b    	mov	w11, #0x20              ; =32
100367990: 54000062    	b.hs	0x10036799c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x344>
100367994: d2800008    	mov	x8, #0x0                ; =0
100367998: 14000021    	b	0x100367a1c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x3c4>
10036799c: 927ae368    	and	x8, x27, #0x7fffffffffffffc0
1003679a0: 91008309    	add	x9, x24, #0x20
1003679a4: aa0803ea    	mov	x10, x8
1003679a8: 4f05e7f0    	movi.16b	v16, #0xbf
1003679ac: 4f00e751    	movi.16b	v17, #0x1a
1003679b0: 4f01e412    	movi.16b	v18, #0x20
1003679b4: ad7f0520    	ldp	q0, q1, [x9, #-0x20]
1003679b8: ad400d22    	ldp	q2, q3, [x9]
1003679bc: 4e308404    	add.16b	v4, v0, v16
1003679c0: 4e308425    	add.16b	v5, v1, v16
1003679c4: 4e308446    	add.16b	v6, v2, v16
1003679c8: 4e308467    	add.16b	v7, v3, v16
1003679cc: 6e243624    	cmhi.16b	v4, v17, v4
1003679d0: 6e253625    	cmhi.16b	v5, v17, v5
1003679d4: 6e263626    	cmhi.16b	v6, v17, v6
1003679d8: 6e273627    	cmhi.16b	v7, v17, v7
1003679dc: 4e321c84    	and.16b	v4, v4, v18
1003679e0: 4e321ca5    	and.16b	v5, v5, v18
1003679e4: 4e321cc6    	and.16b	v6, v6, v18
1003679e8: 4e321ce7    	and.16b	v7, v7, v18
1003679ec: 4ea01c80    	orr.16b	v0, v4, v0
1003679f0: 4ea11ca1    	orr.16b	v1, v5, v1
1003679f4: 4ea21cc2    	orr.16b	v2, v6, v2
1003679f8: ad3f0520    	stp	q0, q1, [x9, #-0x20]
1003679fc: 4ea31ce0    	orr.16b	v0, v7, v3
100367a00: ac820122    	stp	q2, q0, [x9], #0x40
100367a04: f101014a    	subs	x10, x10, #0x40
100367a08: 54fffd61    	b.ne	0x1003679b4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x35c>
100367a0c: eb08037f    	cmp	x27, x8
100367a10: 540003c0    	b.eq	0x100367a88 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x430>
100367a14: f27d0b7f    	tst	x27, #0x38
100367a18: 54000260    	b.eq	0x100367a64 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x40c>
100367a1c: aa0803e9    	mov	x9, x8
100367a20: 927def68    	and	x8, x27, #0x7ffffffffffffff8
100367a24: fc696b00    	ldr	d0, [x24, x9]
100367a28: 0e288401    	add.8b	v1, v0, v8
100367a2c: 2e213521    	cmhi.8b	v1, v9, v1
100367a30: 0e2a1c21    	and.8b	v1, v1, v10
100367a34: 0ea01c20    	orr.8b	v0, v1, v0
100367a38: fc296b00    	str	d0, [x24, x9]
100367a3c: 91002129    	add	x9, x9, #0x8
100367a40: eb09011f    	cmp	x8, x9
100367a44: 54ffff01    	b.ne	0x100367a24 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x3cc>
100367a48: 1400000e    	b	0x100367a80 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x428>
100367a4c: b4fff4bb    	cbz	x27, 0x1003678e0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x288>
100367a50: aa1803e0    	mov	x0, x24
100367a54: aa1b03e1    	mov	x1, x27
100367a58: 52800022    	mov	w2, #0x1                ; =1
100367a5c: 97f266c3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367a60: 17ffffa0    	b	0x1003678e0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x288>
100367a64: 38686b09    	ldrb	w9, [x24, x8]
100367a68: 5101052a    	sub	w10, w9, #0x41
100367a6c: 7100695f    	cmp	w10, #0x1a
100367a70: 1a9f316a    	csel	w10, w11, wzr, lo
100367a74: 2a090149    	orr	w9, w10, w9
100367a78: 38286b09    	strb	w9, [x24, x8]
100367a7c: 91000508    	add	x8, x8, #0x1
100367a80: eb08037f    	cmp	x27, x8
100367a84: 54ffff01    	b.ne	0x100367a64 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x40c>
100367a88: aa1803f5    	mov	x21, x24
100367a8c: d10383a0    	sub	x0, x29, #0xe0
100367a90: aa1803e1    	mov	x1, x24
100367a94: aa1b03e2    	mov	x2, x27
100367a98: 97fffe4e    	bl	0x1003673d0 <__ZN13codexmate_lib4core5relay13codex_catalog16model_tail_lower17hdcbb168b8679db13E>
100367a9c: a972a3a0    	ldp	x0, x8, [x29, #-0xd8]
100367aa0: f1001d1f    	cmp	x8, #0x7
100367aa4: a900d7f4    	stp	x20, x21, [sp, #0x8]
100367aa8: 540001e1    	b.ne	0x100367ae4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x48c>
100367aac: b9400008    	ldr	w8, [x0]
100367ab0: b8403009    	ldur	w9, [x0, #0x3]
100367ab4: 528e0cea    	mov	w10, #0x7067            ; =28775
100367ab8: 72a5ae8a    	movk	w10, #0x2d74, lsl #16
100367abc: 6b0a011f    	cmp	w8, w10
100367ac0: 5286a5a8    	mov	w8, #0x352d             ; =13613
100367ac4: 72a6c5c8    	movk	w8, #0x362e, lsl #16
100367ac8: 7a480120    	ccmp	w9, w8, #0x0, eq
100367acc: 54000101    	b.ne	0x100367aec <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x494>
100367ad0: f85203a1    	ldur	x1, [x29, #-0xe0]
100367ad4: b4000181    	cbz	x1, 0x100367b04 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x4ac>
100367ad8: 52800022    	mov	w2, #0x1                ; =1
100367adc: 97f266a3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367ae0: 14000009    	b	0x100367b04 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x4ac>
100367ae4: f100211f    	cmp	x8, #0x8
100367ae8: 540003c2    	b.hs	0x100367b60 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x508>
100367aec: 52800015    	mov	w21, #0x0               ; =0
100367af0: f85203a1    	ldur	x1, [x29, #-0xe0]
100367af4: b4000061    	cbz	x1, 0x100367b00 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x4a8>
100367af8: 52800022    	mov	w2, #0x1                ; =1
100367afc: 97f2669b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367b00: 3607fa75    	tbz	w21, #0x0, 0x100367a4c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x3f4>
100367b04: f85503b4    	ldur	x20, [x29, #-0xb0]
100367b08: eb1b029f    	cmp	x20, x27
100367b0c: 54fffa03    	b.lo	0x100367a4c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x3f4>
100367b10: f85483b5    	ldur	x21, [x29, #-0xb8]
100367b14: aa1803e0    	mov	x0, x24
100367b18: aa1503e1    	mov	x1, x21
100367b1c: aa1b03e2    	mov	x2, x27
100367b20: 9439a6e0    	bl	0x1011d16a0 <_writev+0x1011d16a0>
100367b24: 35fff940    	cbnz	w0, 0x100367a4c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x3f4>
100367b28: eb1b029f    	cmp	x20, x27
100367b2c: 540000a0    	b.eq	0x100367b40 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x4e8>
100367b30: 387b6aa8    	ldrb	w8, [x21, x27]
100367b34: 12001d08    	and	w8, w8, #0xff
100367b38: 7100b51f    	cmp	w8, #0x2d
100367b3c: 9a9f039c    	csel	x28, x28, xzr, eq
100367b40: f94007f4    	ldr	x20, [sp, #0x8]
100367b44: b40000bb    	cbz	x27, 0x100367b58 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x500>
100367b48: aa1803e0    	mov	x0, x24
100367b4c: aa1b03e1    	mov	x1, x27
100367b50: 52800022    	mov	w2, #0x1                ; =1
100367b54: 97f26685    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367b58: b4ffec7c    	cbz	x28, 0x1003678e4 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x28c>
100367b5c: 14000046    	b	0x100367c74 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x61c>
100367b60: f9400008    	ldr	x8, [x0]
100367b64: d28e0ce9    	mov	x9, #0x7067             ; =28775
100367b68: f2a5ae89    	movk	x9, #0x2d74, lsl #16
100367b6c: f2c5c6a9    	movk	x9, #0x2e35, lsl #32
100367b70: f2e5a6c9    	movk	x9, #0x2d36, lsl #48
100367b74: eb09011f    	cmp	x8, x9
100367b78: 1a9f17f5    	cset	w21, eq
100367b7c: f85203a1    	ldur	x1, [x29, #-0xe0]
100367b80: b5fffbc1    	cbnz	x1, 0x100367af8 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x4a0>
100367b84: 17ffffdf    	b	0x100367b00 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x4a8>
100367b88: d2800004    	mov	x4, #0x0                ; =0
100367b8c: f85403a1    	ldur	x1, [x29, #-0xc0]
100367b90: b40000c1    	cbz	x1, 0x100367ba8 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x550>
100367b94: f85483a0    	ldur	x0, [x29, #-0xb8]
100367b98: 52800022    	mov	w2, #0x1                ; =1
100367b9c: aa0403f3    	mov	x19, x4
100367ba0: 97f26672    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367ba4: aa1303e4    	mov	x4, x19
100367ba8: d280001c    	mov	x28, #0x0               ; =0
100367bac: d10383a0    	sub	x0, x29, #0xe0
100367bb0: 9103c3e2    	add	x2, sp, #0xf0
100367bb4: aa1a03e1    	mov	x1, x26
100367bb8: aa1c03e3    	mov	x3, x28
100367bbc: 940000df    	bl	0x100367f38 <__ZN13codexmate_lib4core5relay13codex_catalog21make_relay_model_info17h303a38e566039289E>
100367bc0: f94037f3    	ldr	x19, [sp, #0x68]
100367bc4: f9402fe8    	ldr	x8, [sp, #0x58]
100367bc8: eb08027f    	cmp	x19, x8
100367bcc: 54000061    	b.ne	0x100367bd8 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x580>
100367bd0: 910163e0    	add	x0, sp, #0x58
100367bd4: 94398af1    	bl	0x1011ca798 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h7488d3d97b01b200E>
100367bd8: f94033e8    	ldr	x8, [sp, #0x60]
100367bdc: 8b131508    	add	x8, x8, x19, lsl #5
100367be0: ad7907a0    	ldp	q0, q1, [x29, #-0xe0]
100367be4: ad000500    	stp	q0, q1, [x8]
100367be8: 91000668    	add	x8, x19, #0x1
100367bec: f90037e8    	str	x8, [sp, #0x68]
100367bf0: f94083e1    	ldr	x1, [sp, #0x100]
100367bf4: b4000081    	cbz	x1, 0x100367c04 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x5ac>
100367bf8: f94087e0    	ldr	x0, [sp, #0x108]
100367bfc: 52800022    	mov	w2, #0x1                ; =1
100367c00: 97f2665a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367c04: f9409be1    	ldr	x1, [sp, #0x130]
100367c08: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100367c0c: eb08003f    	cmp	x1, x8
100367c10: 540000a0    	b.eq	0x100367c24 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x5cc>
100367c14: b4000081    	cbz	x1, 0x100367c24 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x5cc>
100367c18: f9409fe0    	ldr	x0, [sp, #0x138]
100367c1c: 52800022    	mov	w2, #0x1                ; =1
100367c20: 97f26652    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367c24: f9408fe1    	ldr	x1, [sp, #0x118]
100367c28: b4000081    	cbz	x1, 0x100367c38 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x5e0>
100367c2c: f94093e0    	ldr	x0, [sp, #0x120]
100367c30: 52800022    	mov	w2, #0x1                ; =1
100367c34: 97f2664d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367c38: aa1703e9    	mov	x9, x23
100367c3c: f94013e8    	ldr	x8, [sp, #0x20]
100367c40: eb0802ff    	cmp	x23, x8
100367c44: 54ffd901    	b.ne	0x100367764 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x10c>
100367c48: 17fffea6    	b	0x1003676e0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x88>
100367c4c: f9400008    	ldr	x8, [x0]
100367c50: d28e0ce9    	mov	x9, #0x7067             ; =28775
100367c54: f2a5ae89    	movk	x9, #0x2d74, lsl #16
100367c58: f2c5c6a9    	movk	x9, #0x2e35, lsl #32
100367c5c: f2e5a6c9    	movk	x9, #0x2d36, lsl #48
100367c60: eb09011f    	cmp	x8, x9
100367c64: 1a9f17f3    	cset	w19, eq
100367c68: f85203a1    	ldur	x1, [x29, #-0xe0]
100367c6c: b5ffe1a1    	cbnz	x1, 0x1003678a0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x248>
100367c70: 17ffff0e    	b	0x1003678a8 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x250>
100367c74: f81703b3    	stur	x19, [x29, #-0x90]
100367c78: a9326fbb    	stp	x27, x27, [x29, #-0xe0]
100367c7c: f81303bc    	stur	x28, [x29, #-0xd0]
100367c80: d102a3a8    	sub	x8, x29, #0xa8
100367c84: d10243a0    	sub	x0, x29, #0x90
100367c88: d10383a1    	sub	x1, x29, #0xe0
100367c8c: 94126b46    	bl	0x1008029a4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h129a6c6422062f0fE>
100367c90: f85683a4    	ldur	x4, [x29, #-0x98]
100367c94: f85403a1    	ldur	x1, [x29, #-0xc0]
100367c98: b5fff7e1    	cbnz	x1, 0x100367b94 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x53c>
100367c9c: 17ffffc3    	b	0x100367ba8 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x550>
100367ca0: 3cc583e0    	ldur	q0, [sp, #0x58]
100367ca4: f94017e9    	ldr	x9, [sp, #0x28]
100367ca8: 3d800120    	str	q0, [x9]
100367cac: f94037e8    	ldr	x8, [sp, #0x68]
100367cb0: f9000928    	str	x8, [x9, #0x10]
100367cb4: 910703ff    	add	sp, sp, #0x1c0
100367cb8: a9477bfd    	ldp	x29, x30, [sp, #0x70]
100367cbc: a9464ff4    	ldp	x20, x19, [sp, #0x60]
100367cc0: a94557f6    	ldp	x22, x21, [sp, #0x50]
100367cc4: a9445ff8    	ldp	x24, x23, [sp, #0x40]
100367cc8: a94367fa    	ldp	x26, x25, [sp, #0x30]
100367ccc: a9426ffc    	ldp	x28, x27, [sp, #0x20]
100367cd0: 6d4123e9    	ldp	d9, d8, [sp, #0x10]
100367cd4: 6cc82beb    	ldp	d11, d10, [sp], #0x80
100367cd8: d65f03c0    	ret
100367cdc: d2800000    	mov	x0, #0x0                ; =0
100367ce0: f9400be1    	ldr	x1, [sp, #0x10]
100367ce4: 9439a260    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100367ce8: d4200020    	brk	#0x1
100367cec: f9000bfb    	str	x27, [sp, #0x10]
100367cf0: 52800020    	mov	w0, #0x1                ; =1
100367cf4: 17fffffb    	b	0x100367ce0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x688>
100367cf8: 14000013    	b	0x100367d44 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x6ec>
100367cfc: aa0003f3    	mov	x19, x0
100367d00: b400025b    	cbz	x27, 0x100367d48 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x6f0>
100367d04: aa1803e0    	mov	x0, x24
100367d08: aa1b03e1    	mov	x1, x27
100367d0c: 52800022    	mov	w2, #0x1                ; =1
100367d10: 97f26616    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367d14: 1400000d    	b	0x100367d48 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x6f0>
100367d18: aa0003f3    	mov	x19, x0
100367d1c: d10383a0    	sub	x0, x29, #0xe0
100367d20: 9401be28    	bl	0x1003d75c0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100367d24: 1400000e    	b	0x100367d5c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x704>
100367d28: 9439a388    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
100367d2c: aa0003f3    	mov	x19, x0
100367d30: 14000019    	b	0x100367d94 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x73c>
100367d34: 14000002    	b	0x100367d3c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x6e4>
100367d38: 14000003    	b	0x100367d44 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x6ec>
100367d3c: aa0003f3    	mov	x19, x0
100367d40: 14000007    	b	0x100367d5c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x704>
100367d44: aa0003f3    	mov	x19, x0
100367d48: f85403a1    	ldur	x1, [x29, #-0xc0]
100367d4c: b4000081    	cbz	x1, 0x100367d5c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x704>
100367d50: f85483a0    	ldur	x0, [x29, #-0xb8]
100367d54: 52800022    	mov	w2, #0x1                ; =1
100367d58: 97f26604    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367d5c: f94083e1    	ldr	x1, [sp, #0x100]
100367d60: b50002c1    	cbnz	x1, 0x100367db8 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x760>
100367d64: f9409be1    	ldr	x1, [sp, #0x130]
100367d68: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100367d6c: eb08003f    	cmp	x1, x8
100367d70: fa401824    	ccmp	x1, #0x0, #0x4, ne
100367d74: 54000321    	b.ne	0x100367dd8 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x780>
100367d78: f9408fe1    	ldr	x1, [sp, #0x118]
100367d7c: b4000081    	cbz	x1, 0x100367d8c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x734>
100367d80: f94093e0    	ldr	x0, [sp, #0x120]
100367d84: 52800022    	mov	w2, #0x1                ; =1
100367d88: 97f265f8    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367d8c: 9101c3e0    	add	x0, sp, #0x70
100367d90: 9407fbde    	bl	0x100566d08 <__ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8b7cbd6ddb489c57E>
100367d94: a94623f4    	ldp	x20, x8, [sp, #0x60]
100367d98: 91000516    	add	x22, x8, #0x1
100367d9c: aa1403e0    	mov	x0, x20
100367da0: f10006d6    	subs	x22, x22, #0x1
100367da4: 54000260    	b.eq	0x100367df0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x798>
100367da8: 91008015    	add	x21, x0, #0x20
100367dac: 9407a7a8    	bl	0x100551c4c <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100367db0: aa1503e0    	mov	x0, x21
100367db4: 17fffffb    	b	0x100367da0 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x748>
100367db8: f94087e0    	ldr	x0, [sp, #0x108]
100367dbc: 52800022    	mov	w2, #0x1                ; =1
100367dc0: 97f265ea    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367dc4: f9409be1    	ldr	x1, [sp, #0x130]
100367dc8: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100367dcc: eb08003f    	cmp	x1, x8
100367dd0: fa401824    	ccmp	x1, #0x0, #0x4, ne
100367dd4: 54fffd20    	b.eq	0x100367d78 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x720>
100367dd8: f9409fe0    	ldr	x0, [sp, #0x138]
100367ddc: 52800022    	mov	w2, #0x1                ; =1
100367de0: 97f265e2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367de4: f9408fe1    	ldr	x1, [sp, #0x118]
100367de8: b5fffcc1    	cbnz	x1, 0x100367d80 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x728>
100367dec: 17ffffe8    	b	0x100367d8c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x734>
100367df0: f9402fe8    	ldr	x8, [sp, #0x58]
100367df4: b40000a8    	cbz	x8, 0x100367e08 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x7b0>
100367df8: d37be901    	lsl	x1, x8, #5
100367dfc: aa1403e0    	mov	x0, x20
100367e00: 52800102    	mov	w2, #0x8                ; =8
100367e04: 97f265d9    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367e08: aa1303e0    	mov	x0, x19
100367e0c: 9439a4d2    	bl	0x1011d1154 <_writev+0x1011d1154>
100367e10: f10006d6    	subs	x22, x22, #0x1
100367e14: 540000c0    	b.eq	0x100367e2c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x7d4>
100367e18: 910082b3    	add	x19, x21, #0x20
100367e1c: aa1503e0    	mov	x0, x21
100367e20: 9407a78b    	bl	0x100551c4c <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100367e24: aa1303f5    	mov	x21, x19
100367e28: 17fffffa    	b	0x100367e10 <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x7b8>
100367e2c: f9402fe8    	ldr	x8, [sp, #0x58]
100367e30: b40000e8    	cbz	x8, 0x100367e4c <__ZN13codexmate_lib4core5relay13codex_catalog18build_relay_models17h86803b9a69b0918dE+0x7f4>
100367e34: d37be901    	lsl	x1, x8, #5
100367e38: aa1403e0    	mov	x0, x20
100367e3c: 52800102    	mov	w2, #0x8                ; =8
100367e40: 97f265ca    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367e44: 9439a341    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
100367e48: 9439a340    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
100367e4c: 9439a33f    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

0000000100367e50 <__ZN13codexmate_lib4core5relay13codex_catalog19is_relay_model_slug17h3d07e3b2bec682bbE>:
100367e50: d102c3ff    	sub	sp, sp, #0xb0
100367e54: a90857f6    	stp	x22, x21, [sp, #0x80]
100367e58: a9094ff4    	stp	x20, x19, [sp, #0x90]
100367e5c: a90a7bfd    	stp	x29, x30, [sp, #0xa0]
100367e60: 910283fd    	add	x29, sp, #0xa0
100367e64: aa0103f4    	mov	x20, x1
100367e68: aa0003f3    	mov	x19, x0
100367e6c: b0007d22    	adrp	x2, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
100367e70: 91030842    	add	x2, x2, #0xc2
100367e74: 910063e8    	add	x8, sp, #0x18
100367e78: 52800043    	mov	w3, #0x2                ; =2
100367e7c: 9438768f    	bl	0x1011858b8 <__ZN4core3str7pattern11StrSearcher3new17h4cabcb1377b9c47bE>
100367e80: 910003e0    	mov	x0, sp
100367e84: 910063e1    	add	x1, sp, #0x18
100367e88: 94021a33    	bl	0x1003ee754 <__ZN80_$LT$core..str..pattern..StrSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17hec5ac1ff8c43bcccE>
100367e8c: b94003e8    	ldr	w8, [sp]
100367e90: 36000228    	tbz	w8, #0x0, 0x100367ed4 <__ZN13codexmate_lib4core5relay13codex_catalog19is_relay_model_slug17h3d07e3b2bec682bbE+0x84>
100367e94: f94007e1    	ldr	x1, [sp, #0x8]
100367e98: f100343f    	cmp	x1, #0xd
100367e9c: 540001c3    	b.lo	0x100367ed4 <__ZN13codexmate_lib4core5relay13codex_catalog19is_relay_model_slug17h3d07e3b2bec682bbE+0x84>
100367ea0: f9400268    	ldr	x8, [x19]
100367ea4: f8405269    	ldur	x9, [x19, #0x5]
100367ea8: d28d2c2a    	mov	x10, #0x6961            ; =26977
100367eac: f2ac2daa    	movk	x10, #0x616d, lsl #16
100367eb0: f2cd2daa    	movk	x10, #0x696d, lsl #32
100367eb4: f2ee4bea    	movk	x10, #0x725f, lsl #48
100367eb8: eb0a011f    	cmp	x8, x10
100367ebc: d28bed28    	mov	x8, #0x5f69             ; =24425
100367ec0: f2acae48    	movk	x8, #0x6572, lsl #16
100367ec4: f2cc2d88    	movk	x8, #0x616c, lsl #32
100367ec8: f2ebef28    	movk	x8, #0x5f79, lsl #48
100367ecc: fa480120    	ccmp	x9, x8, #0x0, eq
100367ed0: 540000e0    	b.eq	0x100367eec <__ZN13codexmate_lib4core5relay13codex_catalog19is_relay_model_slug17h3d07e3b2bec682bbE+0x9c>
100367ed4: 52800000    	mov	w0, #0x0                ; =0
100367ed8: a94a7bfd    	ldp	x29, x30, [sp, #0xa0]
100367edc: a9494ff4    	ldp	x20, x19, [sp, #0x90]
100367ee0: a94857f6    	ldp	x22, x21, [sp, #0x80]
100367ee4: 9102c3ff    	add	sp, sp, #0xb0
100367ee8: d65f03c0    	ret
100367eec: f9400bf5    	ldr	x21, [sp, #0x10]
100367ef0: 910063e8    	add	x8, sp, #0x18
100367ef4: aa1303e0    	mov	x0, x19
100367ef8: 9410e4be    	bl	0x1007a11f0 <__ZN13codexmate_lib4core5relay6models26validate_relay_provider_id17h52e062fe96fe3cd7E>
100367efc: f9400fe1    	ldr	x1, [sp, #0x18]
100367f00: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100367f04: eb08003f    	cmp	x1, x8
100367f08: 540000e1    	b.ne	0x100367f24 <__ZN13codexmate_lib4core5relay13codex_catalog19is_relay_model_slug17h3d07e3b2bec682bbE+0xd4>
100367f0c: cb150281    	sub	x1, x20, x21
100367f10: 8b150260    	add	x0, x19, x21
100367f14: 97fb1c91    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100367f18: f100003f    	cmp	x1, #0x0
100367f1c: 1a9f07e0    	cset	w0, ne
100367f20: 17ffffee    	b	0x100367ed8 <__ZN13codexmate_lib4core5relay13codex_catalog19is_relay_model_slug17h3d07e3b2bec682bbE+0x88>
100367f24: b4fffd81    	cbz	x1, 0x100367ed4 <__ZN13codexmate_lib4core5relay13codex_catalog19is_relay_model_slug17h3d07e3b2bec682bbE+0x84>
100367f28: f94013e0    	ldr	x0, [sp, #0x20]
100367f2c: 52800022    	mov	w2, #0x1                ; =1
100367f30: 97f2658e    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100367f34: 17ffffe8    	b	0x100367ed4 <__ZN13codexmate_lib4core5relay13codex_catalog19is_relay_model_slug17h3d07e3b2bec682bbE+0x84>
