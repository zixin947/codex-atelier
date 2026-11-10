
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100376eb0 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E>:
100376eb0: d10503ff    	sub	sp, sp, #0x140
100376eb4: a90e6ffc    	stp	x28, x27, [sp, #0xe0]
100376eb8: a90f67fa    	stp	x26, x25, [sp, #0xf0]
100376ebc: a9105ff8    	stp	x24, x23, [sp, #0x100]
100376ec0: a91157f6    	stp	x22, x21, [sp, #0x110]
100376ec4: a9124ff4    	stp	x20, x19, [sp, #0x120]
100376ec8: a9137bfd    	stp	x29, x30, [sp, #0x130]
100376ecc: 9104c3fd    	add	x29, sp, #0x130
100376ed0: a90103e3    	stp	x3, x0, [sp, #0x10]
100376ed4: 52800108    	mov	w8, #0x8                ; =8
100376ed8: a90223ff    	stp	xzr, x8, [sp, #0x20]
100376edc: f9001bff    	str	xzr, [sp, #0x30]
100376ee0: 52800048    	mov	w8, #0x2                ; =2
100376ee4: 3900ffe8    	strb	w8, [sp, #0x3f]
100376ee8: 910103f8    	add	x24, sp, #0x40
100376eec: 0f000540    	movi.2s	v0, #0xa
100376ef0: a9040bff    	stp	xzr, x2, [sp, #0x40]
100376ef4: 52800034    	mov	w20, #0x1               ; =1
100376ef8: d2f00013    	mov	x19, #-0x8000000000000000 ; =-9223372036854775808
100376efc: a9050be1    	stp	x1, x2, [sp, #0x50]
100376f00: a9060bff    	stp	xzr, x2, [sp, #0x60]
100376f04: fd003be0    	str	d0, [sp, #0x70]
100376f08: 3901e3f4    	strb	w20, [sp, #0x78]
100376f0c: d2f00019    	mov	x25, #-0x8000000000000000 ; =-9223372036854775808
100376f10: 790103ff    	strh	wzr, [sp, #0x80]
100376f14: 1400001b    	b	0x100376f80 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0xd0>
100376f18: f8401368    	ldur	x8, [x27, #0x1]
100376f1c: f8409369    	ldur	x9, [x27, #0x9]
100376f20: d28dedaa    	mov	x10, #0x6f6d            ; =28525
100376f24: f2acac8a    	movk	x10, #0x6564, lsl #16
100376f28: f2cbed8a    	movk	x10, #0x5f6c, lsl #32
100376f2c: f2ee4e0a    	movk	x10, #0x7270, lsl #48
100376f30: eb0a011f    	cmp	x8, x10
100376f34: d28ecde8    	mov	x8, #0x766f             ; =30319
100376f38: f2ac8d28    	movk	x8, #0x6469, lsl #16
100376f3c: f2ce4ca8    	movk	x8, #0x7265, lsl #32
100376f40: f2e5ce68    	movk	x8, #0x2e73, lsl #48
100376f44: fa480120    	ccmp	x9, x8, #0x0, eq
100376f48: 54001400    	b.eq	0x1003771c8 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x318>
100376f4c: f8401368    	ldur	x8, [x27, #0x1]
100376f50: 39402769    	ldrb	w9, [x27, #0x9]
100376f54: d28e4e0a    	mov	x10, #0x7270            ; =29296
100376f58: f2accdea    	movk	x10, #0x666f, lsl #16
100376f5c: f2cd8d2a    	movk	x10, #0x6c69, lsl #32
100376f60: f2ee6caa    	movk	x10, #0x7365, lsl #48
100376f64: eb0a011f    	cmp	x8, x10
100376f68: 528005c8    	mov	w8, #0x2e               ; =46
100376f6c: 7a480120    	ccmp	w9, w8, #0x0, eq
100376f70: 54000dc0    	b.eq	0x100377128 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x278>
100376f74: d2f00019    	mov	x25, #-0x8000000000000000 ; =-9223372036854775808
100376f78: 394207e8    	ldrb	w8, [sp, #0x81]
100376f7c: 37001a48    	tbnz	w8, #0x0, 0x1003772c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x414>
100376f80: f9402bf5    	ldr	x21, [sp, #0x50]
100376f84: d101e3a0    	sub	x0, x29, #0x78
100376f88: 91004301    	add	x1, x24, #0x10
100376f8c: 9401de64    	bl	0x1003ee91c <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
100376f90: b85883a8    	ldur	w8, [x29, #-0x78]
100376f94: 7100051f    	cmp	w8, #0x1
100376f98: 54000101    	b.ne	0x100376fb8 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x108>
100376f9c: f85983a8    	ldur	x8, [x29, #-0x68]
100376fa0: f94023e9    	ldr	x9, [sp, #0x40]
100376fa4: cb090116    	sub	x22, x8, x9
100376fa8: 8b0902b5    	add	x21, x21, x9
100376fac: f90023e8    	str	x8, [sp, #0x40]
100376fb0: b50001d6    	cbnz	x22, 0x100376fe8 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x138>
100376fb4: 1400001a    	b	0x10037701c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x16c>
100376fb8: 394207e8    	ldrb	w8, [sp, #0x81]
100376fbc: 37001848    	tbnz	w8, #0x0, 0x1003772c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x414>
100376fc0: 390207f4    	strb	w20, [sp, #0x81]
100376fc4: 394203ea    	ldrb	w10, [sp, #0x80]
100376fc8: a94427e8    	ldp	x8, x9, [sp, #0x40]
100376fcc: 3700006a    	tbnz	w10, #0x0, 0x100376fd8 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x128>
100376fd0: eb08013f    	cmp	x9, x8
100376fd4: 54001780    	b.eq	0x1003772c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x414>
100376fd8: f9402bea    	ldr	x10, [sp, #0x50]
100376fdc: cb080136    	sub	x22, x9, x8
100376fe0: 8b080155    	add	x21, x10, x8
100376fe4: b40001d6    	cbz	x22, 0x10037701c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x16c>
100376fe8: 8b1602a8    	add	x8, x21, x22
100376fec: 385ff109    	ldurb	w9, [x8, #-0x1]
100376ff0: 7100293f    	cmp	w9, #0xa
100376ff4: 54000141    	b.ne	0x10037701c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x16c>
100376ff8: f10006c9    	subs	x9, x22, #0x1
100376ffc: 540000e0    	b.eq	0x100377018 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x168>
100377000: 385fe108    	ldurb	w8, [x8, #-0x2]
100377004: d1000aca    	sub	x10, x22, #0x2
100377008: 7100351f    	cmp	w8, #0xd
10037700c: 9a9512b5    	csel	x21, x21, x21, ne
100377010: 9a8a1136    	csel	x22, x9, x10, ne
100377014: 14000002    	b	0x10037701c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x16c>
100377018: d2800016    	mov	x22, #0x0               ; =0
10037701c: aa1503e0    	mov	x0, x21
100377020: aa1603e1    	mov	x1, x22
100377024: 97fae04d    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100377028: aa0003fb    	mov	x27, x0
10037702c: aa0103fc    	mov	x28, x1
100377030: 9100ffe0    	add	x0, sp, #0x3f
100377034: aa1503e1    	mov	x1, x21
100377038: aa1603e2    	mov	x2, x22
10037703c: 94105a78    	bl	0x10078da1c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE>
100377040: 34fff9c0    	cbz	w0, 0x100376f78 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0xc8>
100377044: b400039c    	cbz	x28, 0x1003770b4 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x204>
100377048: 39400368    	ldrb	w8, [x27]
10037704c: 71016d1f    	cmp	w8, #0x5b
100377050: 54000321    	b.ne	0x1003770b4 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x204>
100377054: 8b1c0368    	add	x8, x27, x28
100377058: 385ff108    	ldurb	w8, [x8, #-0x1]
10037705c: 7101751f    	cmp	w8, #0x5d
100377060: 540002a1    	b.ne	0x1003770b4 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x204>
100377064: f1000783    	subs	x3, x28, #0x1
100377068: 540017a0    	b.eq	0x10037735c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x4ac>
10037706c: 39c00768    	ldrsb	w8, [x27, #0x1]
100377070: 3101051f    	cmn	w8, #0x41
100377074: 5400174d    	b.le	0x10037735c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x4ac>
100377078: eb13033f    	cmp	x25, x19
10037707c: 54000100    	b.eq	0x10037709c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x1ec>
100377080: 2a3a02e8    	orn	w8, w23, w26
100377084: 360002e8    	tbz	w8, #0x0, 0x1003770e0 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x230>
100377088: b40000b9    	cbz	x25, 0x10037709c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x1ec>
10037708c: f94007e0    	ldr	x0, [sp, #0x8]
100377090: aa1903e1    	mov	x1, x25
100377094: 52800022    	mov	w2, #0x1                ; =1
100377098: 97f22934    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10037709c: d1000b88    	sub	x8, x28, #0x2
1003770a0: f100411f    	cmp	x8, #0x10
1003770a4: 54fff3a2    	b.hs	0x100376f18 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x68>
1003770a8: f100251f    	cmp	x8, #0x9
1003770ac: 54fff502    	b.hs	0x100376f4c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x9c>
1003770b0: 17ffffb1    	b	0x100376f74 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0xc4>
1003770b4: eb13033f    	cmp	x25, x19
1003770b8: 1a9f17e8    	cset	w8, eq
1003770bc: 2a1a02e9    	orr	w9, w23, w26
1003770c0: 2a090108    	orr	w8, w8, w9
1003770c4: 3707f5a8    	tbnz	w8, #0x0, 0x100376f78 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0xc8>
1003770c8: aa1b03e0    	mov	x0, x27
1003770cc: aa1c03e1    	mov	x1, x28
1003770d0: 9400719c    	bl	0x100393740 <__ZN13codexmate_lib4core5relay16codex_diagnostic30is_explicit_third_party_marker17h8ca48711281fdd83E>
1003770d4: aa0003fa    	mov	x26, x0
1003770d8: 52800017    	mov	w23, #0x0               ; =0
1003770dc: 17ffffa7    	b	0x100376f78 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0xc8>
1003770e0: f9401bf5    	ldr	x21, [sp, #0x30]
1003770e4: f94013e8    	ldr	x8, [sp, #0x20]
1003770e8: eb0802bf    	cmp	x21, x8
1003770ec: 54000061    	b.ne	0x1003770f8 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x248>
1003770f0: 910083e0    	add	x0, sp, #0x20
1003770f4: 94396343    	bl	0x1011cfe00 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hca26f846e3ace0fdE>
1003770f8: f94017e8    	ldr	x8, [sp, #0x28]
1003770fc: 52800309    	mov	w9, #0x18               ; =24
100377100: 9b0922a8    	madd	x8, x21, x9, x8
100377104: a9402be9    	ldp	x9, x10, [sp]
100377108: a9002919    	stp	x25, x10, [x8]
10037710c: f9000909    	str	x9, [x8, #0x10]
100377110: 910006a8    	add	x8, x21, #0x1
100377114: f9001be8    	str	x8, [sp, #0x30]
100377118: d1000b88    	sub	x8, x28, #0x2
10037711c: f100411f    	cmp	x8, #0x10
100377120: 54ffefc2    	b.hs	0x100376f18 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x68>
100377124: 17ffffe1    	b	0x1003770a8 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x1f8>
100377128: d1002f81    	sub	x1, x28, #0xb
10037712c: 910223e8    	add	x8, sp, #0x88
100377130: 91002b60    	add	x0, x27, #0xa
100377134: 97fff12c    	bl	0x1003735e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E>
100377138: a9495bf5    	ldp	x21, x22, [sp, #0x90]
10037713c: aa1503e0    	mov	x0, x21
100377140: aa1603e1    	mov	x1, x22
100377144: f9400be2    	ldr	x2, [sp, #0x10]
100377148: 97fff5fe    	bl	0x100374940 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE>
10037714c: 37000520    	tbnz	w0, #0x0, 0x1003771f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x340>
100377150: aa1503e0    	mov	x0, x21
100377154: aa1603e1    	mov	x1, x22
100377158: 940090cf    	bl	0x10039b494 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E>
10037715c: aa0003f7    	mov	x23, x0
100377160: 910223e8    	add	x8, sp, #0x88
100377164: f81703a8    	stur	x8, [x29, #-0x90]
100377168: d00003a8    	adrp	x8, 0x1003ed000 <__ZN5alloc11collections5btree3set21BTreeSet$LT$T$C$A$GT$10difference17h151923f22deefe03E+0x15c>
10037716c: 9133e108    	add	x8, x8, #0xcf8
100377170: f81783a8    	stur	x8, [x29, #-0x88]
100377174: d101e3a8    	sub	x8, x29, #0x78
100377178: d10243a1    	sub	x1, x29, #0x90
10037717c: b0008d40    	adrp	x0, 0x101520000 <_anon.164d03a2dfc98f969363eaf1c50504fd.175+0x1a1>
100377180: 910b7400    	add	x0, x0, #0x2dd
100377184: 943801c4    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100377188: d101e3a8    	sub	x8, x29, #0x78
10037718c: 3dc00100    	ldr	q0, [x8]
100377190: 3c9703a0    	stur	q0, [x29, #-0x90]
100377194: f85983a8    	ldur	x8, [x29, #-0x68]
100377198: f81803a8    	stur	x8, [x29, #-0x80]
10037719c: 34000817    	cbz	w23, 0x10037729c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x3ec>
1003771a0: d101e3a8    	sub	x8, x29, #0x78
1003771a4: d10243a0    	sub	x0, x29, #0x90
1003771a8: 943808d5    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
1003771ac: f9401bf5    	ldr	x21, [sp, #0x30]
1003771b0: f94013e8    	ldr	x8, [sp, #0x20]
1003771b4: eb0802bf    	cmp	x21, x8
1003771b8: 540005e1    	b.ne	0x100377274 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x3c4>
1003771bc: 910083e0    	add	x0, sp, #0x20
1003771c0: 94396310    	bl	0x1011cfe00 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hca26f846e3ace0fdE>
1003771c4: 1400002c    	b	0x100377274 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x3c4>
1003771c8: d1004b81    	sub	x1, x28, #0x12
1003771cc: 910223e8    	add	x8, sp, #0x88
1003771d0: 91004760    	add	x0, x27, #0x11
1003771d4: 97fff104    	bl	0x1003735e4 <__ZN13codexmate_lib4core5relay16codex_diagnostic18first_section_part17h07e804047dca1e88E>
1003771d8: a9495bf5    	ldp	x21, x22, [sp, #0x90]
1003771dc: aa1503e0    	mov	x0, x21
1003771e0: aa1603e1    	mov	x1, x22
1003771e4: f9400be2    	ldr	x2, [sp, #0x10]
1003771e8: 97fff5d6    	bl	0x100374940 <__ZN13codexmate_lib4core5relay16codex_diagnostic19is_known_or_builtin17h99c99dafa0fb836fE>
1003771ec: 360000a0    	tbz	w0, #0x0, 0x100377200 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x350>
1003771f0: d2f00019    	mov	x25, #-0x8000000000000000 ; =-9223372036854775808
1003771f4: f94047e1    	ldr	x1, [sp, #0x88]
1003771f8: b50005e1    	cbnz	x1, 0x1003772b4 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x404>
1003771fc: 17ffff5f    	b	0x100376f78 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0xc8>
100377200: aa1503e0    	mov	x0, x21
100377204: aa1603e1    	mov	x1, x22
100377208: 940090a3    	bl	0x10039b494 <__ZN13codexmate_lib4core5relay16codex_diagnostic32is_explicit_config_owner_residue17h3fb094d210396735E>
10037720c: aa0003f7    	mov	x23, x0
100377210: 910223e8    	add	x8, sp, #0x88
100377214: f81703a8    	stur	x8, [x29, #-0x90]
100377218: d00003a8    	adrp	x8, 0x1003ed000 <__ZN5alloc11collections5btree3set21BTreeSet$LT$T$C$A$GT$10difference17h151923f22deefe03E+0x15c>
10037721c: 9133e108    	add	x8, x8, #0xcf8
100377220: f81783a8    	stur	x8, [x29, #-0x88]
100377224: d101e3a8    	sub	x8, x29, #0x78
100377228: d10243a1    	sub	x1, x29, #0x90
10037722c: b0008d40    	adrp	x0, 0x101520000 <_anon.164d03a2dfc98f969363eaf1c50504fd.175+0x1a1>
100377230: 910bb000    	add	x0, x0, #0x2ec
100377234: 94380198    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100377238: d101e3a8    	sub	x8, x29, #0x78
10037723c: 3dc00100    	ldr	q0, [x8]
100377240: 3c9703a0    	stur	q0, [x29, #-0x90]
100377244: f85983a8    	ldur	x8, [x29, #-0x68]
100377248: f81803a8    	stur	x8, [x29, #-0x80]
10037724c: 34000297    	cbz	w23, 0x10037729c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x3ec>
100377250: d101e3a8    	sub	x8, x29, #0x78
100377254: d10243a0    	sub	x0, x29, #0x90
100377258: 943808a9    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
10037725c: f9401bf5    	ldr	x21, [sp, #0x30]
100377260: f94013e8    	ldr	x8, [sp, #0x20]
100377264: eb0802bf    	cmp	x21, x8
100377268: 54000061    	b.ne	0x100377274 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x3c4>
10037726c: 910083e0    	add	x0, sp, #0x20
100377270: 943962e4    	bl	0x1011cfe00 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hca26f846e3ace0fdE>
100377274: f94017e8    	ldr	x8, [sp, #0x28]
100377278: 52800309    	mov	w9, #0x18               ; =24
10037727c: 9b0922a8    	madd	x8, x21, x9, x8
100377280: d101e3a9    	sub	x9, x29, #0x78
100377284: 3dc00120    	ldr	q0, [x9]
100377288: 3d800100    	str	q0, [x8]
10037728c: f85983a9    	ldur	x9, [x29, #-0x68]
100377290: f9000909    	str	x9, [x8, #0x10]
100377294: 910006a8    	add	x8, x21, #0x1
100377298: f9001be8    	str	x8, [sp, #0x30]
10037729c: 5280001a    	mov	w26, #0x0               ; =0
1003772a0: a97727b9    	ldp	x25, x9, [x29, #-0x90]
1003772a4: f85803a8    	ldur	x8, [x29, #-0x80]
1003772a8: a90027e8    	stp	x8, x9, [sp]
1003772ac: f94047e1    	ldr	x1, [sp, #0x88]
1003772b0: b4ffe641    	cbz	x1, 0x100376f78 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0xc8>
1003772b4: f9404be0    	ldr	x0, [sp, #0x90]
1003772b8: 52800022    	mov	w2, #0x1                ; =1
1003772bc: 97f228ab    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003772c0: 17ffff2e    	b	0x100376f78 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0xc8>
1003772c4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1003772c8: eb08033f    	cmp	x25, x8
1003772cc: 540002e0    	b.eq	0x100377328 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x478>
1003772d0: 2a3a02e8    	orn	w8, w23, w26
1003772d4: 360000e8    	tbz	w8, #0x0, 0x1003772f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x440>
1003772d8: b4000299    	cbz	x25, 0x100377328 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x478>
1003772dc: f94007e0    	ldr	x0, [sp, #0x8]
1003772e0: aa1903e1    	mov	x1, x25
1003772e4: 52800022    	mov	w2, #0x1                ; =1
1003772e8: 97f228a0    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003772ec: 1400000f    	b	0x100377328 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x478>
1003772f0: f9401bf3    	ldr	x19, [sp, #0x30]
1003772f4: f94013e8    	ldr	x8, [sp, #0x20]
1003772f8: eb08027f    	cmp	x19, x8
1003772fc: 54000061    	b.ne	0x100377308 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x458>
100377300: 910083e0    	add	x0, sp, #0x20
100377304: 943962bf    	bl	0x1011cfe00 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hca26f846e3ace0fdE>
100377308: f94017e8    	ldr	x8, [sp, #0x28]
10037730c: 52800309    	mov	w9, #0x18               ; =24
100377310: 9b092268    	madd	x8, x19, x9, x8
100377314: a9402be9    	ldp	x9, x10, [sp]
100377318: a9002919    	stp	x25, x10, [x8]
10037731c: f9000909    	str	x9, [x8, #0x10]
100377320: 91000668    	add	x8, x19, #0x1
100377324: f9001be8    	str	x8, [sp, #0x30]
100377328: 3dc00be0    	ldr	q0, [sp, #0x20]
10037732c: f9400fe9    	ldr	x9, [sp, #0x18]
100377330: 3d800120    	str	q0, [x9]
100377334: f9401be8    	ldr	x8, [sp, #0x30]
100377338: f9000928    	str	x8, [x9, #0x10]
10037733c: a9537bfd    	ldp	x29, x30, [sp, #0x130]
100377340: a9524ff4    	ldp	x20, x19, [sp, #0x120]
100377344: a95157f6    	ldp	x22, x21, [sp, #0x110]
100377348: a9505ff8    	ldp	x24, x23, [sp, #0x100]
10037734c: a94f67fa    	ldp	x26, x25, [sp, #0xf0]
100377350: a94e6ffc    	ldp	x28, x27, [sp, #0xe0]
100377354: 910503ff    	add	sp, sp, #0x140
100377358: d65f03c0    	ret
10037735c: f000a584    	adrp	x4, 0x10182a000 <_anon.cdb272158a04aae91ce02c6e47f106c7.1+0x6d0>
100377360: 9133a084    	add	x4, x4, #0xce8
100377364: aa1b03e0    	mov	x0, x27
100377368: aa1c03e1    	mov	x1, x28
10037736c: 52800022    	mov	w2, #0x1                ; =1
100377370: 9439652a    	bl	0x1011d0818 <__ZN4core3str16slice_error_fail17hecde84adb67345c5E>
100377374: d4200020    	brk	#0x1
100377378: 14000001    	b	0x10037737c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x4cc>
10037737c: aa0003f3    	mov	x19, x0
100377380: f85883a1    	ldur	x1, [x29, #-0x78]
100377384: b50000c1    	cbnz	x1, 0x10037739c <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x4ec>
100377388: f85703a1    	ldur	x1, [x29, #-0x90]
10037738c: b50001c1    	cbnz	x1, 0x1003773c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x514>
100377390: f94047e1    	ldr	x1, [sp, #0x88]
100377394: b5000341    	cbnz	x1, 0x1003773fc <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x54c>
100377398: 1400002c    	b	0x100377448 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x598>
10037739c: f85903a0    	ldur	x0, [x29, #-0x70]
1003773a0: 52800022    	mov	w2, #0x1                ; =1
1003773a4: 97f22871    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003773a8: f85703a1    	ldur	x1, [x29, #-0x90]
1003773ac: b4ffff21    	cbz	x1, 0x100377390 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x4e0>
1003773b0: 14000005    	b	0x1003773c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x514>
1003773b4: 14000001    	b	0x1003773b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x508>
1003773b8: aa0003f3    	mov	x19, x0
1003773bc: f85703a1    	ldur	x1, [x29, #-0x90]
1003773c0: b4fffe81    	cbz	x1, 0x100377390 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x4e0>
1003773c4: f85783a0    	ldur	x0, [x29, #-0x88]
1003773c8: 52800022    	mov	w2, #0x1                ; =1
1003773cc: 97f22867    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003773d0: f94047e1    	ldr	x1, [sp, #0x88]
1003773d4: b5000141    	cbnz	x1, 0x1003773fc <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x54c>
1003773d8: 1400001c    	b	0x100377448 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x598>
1003773dc: 14000001    	b	0x1003773e0 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x530>
1003773e0: aa0003f3    	mov	x19, x0
1003773e4: b50002b9    	cbnz	x25, 0x100377438 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x588>
1003773e8: 14000018    	b	0x100377448 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x598>
1003773ec: 14000001    	b	0x1003773f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x540>
1003773f0: aa0003f3    	mov	x19, x0
1003773f4: f94047e1    	ldr	x1, [sp, #0x88]
1003773f8: b4000281    	cbz	x1, 0x100377448 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x598>
1003773fc: f9404be0    	ldr	x0, [sp, #0x90]
100377400: 14000010    	b	0x100377440 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x590>
100377404: aa0003f3    	mov	x19, x0
100377408: 910083e0    	add	x0, sp, #0x20
10037740c: 940183d0    	bl	0x1003d834c <__ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h265e755c5c638973E>
100377410: aa1303e0    	mov	x0, x19
100377414: 94396750    	bl	0x1011d1154 <_writev+0x1011d1154>
100377418: 14000003    	b	0x100377424 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x574>
10037741c: aa0003f3    	mov	x19, x0
100377420: 14000005    	b	0x100377434 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x584>
100377424: aa0003f3    	mov	x19, x0
100377428: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10037742c: eb08033f    	cmp	x25, x8
100377430: 540000c0    	b.eq	0x100377448 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x598>
100377434: b40000b9    	cbz	x25, 0x100377448 <__ZN13codexmate_lib4core5relay16codex_diagnostic22find_foreign_providers17h4ad6b67b67550126E+0x598>
100377438: f94007e0    	ldr	x0, [sp, #0x8]
10037743c: aa1903e1    	mov	x1, x25
100377440: 52800022    	mov	w2, #0x1                ; =1
100377444: 97f22849    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100377448: 910083e0    	add	x0, sp, #0x20
10037744c: 940183c0    	bl	0x1003d834c <__ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h265e755c5c638973E>
100377450: aa1303e0    	mov	x0, x19
100377454: 94396740    	bl	0x1011d1154 <_writev+0x1011d1154>
