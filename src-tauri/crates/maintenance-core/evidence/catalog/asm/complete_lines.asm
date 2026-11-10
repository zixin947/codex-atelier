
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078dbe4 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E>:
10078dbe4: d10143ff    	sub	sp, sp, #0x50
10078dbe8: a90257f6    	stp	x22, x21, [sp, #0x20]
10078dbec: a9034ff4    	stp	x20, x19, [sp, #0x30]
10078dbf0: a9047bfd    	stp	x29, x30, [sp, #0x40]
10078dbf4: 910103fd    	add	x29, sp, #0x40
10078dbf8: f9400813    	ldr	x19, [x0, #0x10]
10078dbfc: 52800048    	mov	w8, #0x2                ; =2
10078dc00: 39001fe8    	strb	w8, [sp, #0x7]
10078dc04: 39416408    	ldrb	w8, [x0, #0x59]
10078dc08: 37000648    	tbnz	w8, #0x0, 0x10078dcd0 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0xec>
10078dc0c: aa0003f4    	mov	x20, x0
10078dc10: 52800036    	mov	w22, #0x1               ; =1
10078dc14: f8428295    	ldur	x21, [x20, #0x28]
10078dc18: 910023e0    	add	x0, sp, #0x8
10078dc1c: 9100a281    	add	x1, x20, #0x28
10078dc20: 9401c4db    	bl	0x1007fef8c <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
10078dc24: b9400be8    	ldr	w8, [sp, #0x8]
10078dc28: 7100051f    	cmp	w8, #0x1
10078dc2c: 540000e1    	b.ne	0x10078dc48 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x64>
10078dc30: f9400fe8    	ldr	x8, [sp, #0x18]
10078dc34: f9400e89    	ldr	x9, [x20, #0x18]
10078dc38: cb090102    	sub	x2, x8, x9
10078dc3c: 8b0902a1    	add	x1, x21, x9
10078dc40: f9000e88    	str	x8, [x20, #0x18]
10078dc44: 1400000c    	b	0x10078dc74 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x90>
10078dc48: 39416688    	ldrb	w8, [x20, #0x59]
10078dc4c: 370003c8    	tbnz	w8, #0x0, 0x10078dcc4 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0xe0>
10078dc50: 39016696    	strb	w22, [x20, #0x59]
10078dc54: 3941628a    	ldrb	w10, [x20, #0x58]
10078dc58: a941a688    	ldp	x8, x9, [x20, #0x18]
10078dc5c: 3700006a    	tbnz	w10, #0x0, 0x10078dc68 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x84>
10078dc60: eb08013f    	cmp	x9, x8
10078dc64: 54000300    	b.eq	0x10078dcc4 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0xe0>
10078dc68: f842828a    	ldur	x10, [x20, #0x28]
10078dc6c: cb080122    	sub	x2, x9, x8
10078dc70: 8b080141    	add	x1, x10, x8
10078dc74: f9400a88    	ldr	x8, [x20, #0x10]
10078dc78: 8b020115    	add	x21, x8, x2
10078dc7c: f9000a95    	str	x21, [x20, #0x10]
10078dc80: 91001fe0    	add	x0, sp, #0x7
10078dc84: 97ffff66    	bl	0x10078da1c <__ZN13codexmate_lib4core5relay14managed_blocks16TomlLexicalState18line_is_structural17hfb6641ed5255844fE>
10078dc88: 39401fe8    	ldrb	w8, [sp, #0x7]
10078dc8c: 7100091f    	cmp	w8, #0x2
10078dc90: 54000060    	b.eq	0x10078dc9c <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0xb8>
10078dc94: 39416688    	ldrb	w8, [x20, #0x59]
10078dc98: 3607fbe8    	tbz	w8, #0x0, 0x10078dc14 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x30>
10078dc9c: eb1302a9    	subs	x9, x21, x19
10078dca0: 54000189    	b.ls	0x10078dcd0 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0xec>
10078dca4: a9400688    	ldp	x8, x1, [x20]
10078dca8: b40001b3    	cbz	x19, 0x10078dcdc <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0xf8>
10078dcac: eb01027f    	cmp	x19, x1
10078dcb0: 54000142    	b.hs	0x10078dcd8 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0xf4>
10078dcb4: 38f3690a    	ldrsb	w10, [x8, x19]
10078dcb8: 3101015f    	cmn	w10, #0x40
10078dcbc: 5400010a    	b.ge	0x10078dcdc <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0xf8>
10078dcc0: 1400000c    	b	0x10078dcf0 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x10c>
10078dcc4: f9400a95    	ldr	x21, [x20, #0x10]
10078dcc8: eb1302a9    	subs	x9, x21, x19
10078dccc: 54fffec8    	b.hi	0x10078dca4 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0xc0>
10078dcd0: d2800000    	mov	x0, #0x0                ; =0
10078dcd4: 1400001c    	b	0x10078dd44 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x160>
10078dcd8: 540000c1    	b.ne	0x10078dcf0 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x10c>
10078dcdc: eb0102bf    	cmp	x21, x1
10078dce0: 54000142    	b.hs	0x10078dd08 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x124>
10078dce4: 38f5690a    	ldrsb	w10, [x8, x21]
10078dce8: 3101055f    	cmn	w10, #0x41
10078dcec: 5400010c    	b.gt	0x10078dd0c <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x128>
10078dcf0: b00084e4    	adrp	x4, 0x10182a000 <_anon.cdb272158a04aae91ce02c6e47f106c7.1+0x6d0>
10078dcf4: 91340084    	add	x4, x4, #0xd00
10078dcf8: aa0803e0    	mov	x0, x8
10078dcfc: aa1303e2    	mov	x2, x19
10078dd00: aa1503e3    	mov	x3, x21
10078dd04: 94290ac5    	bl	0x1011d0818 <__ZN4core3str16slice_error_fail17hecde84adb67345c5E>
10078dd08: 54ffff41    	b.ne	0x10078dcf0 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x10c>
10078dd0c: 8b130100    	add	x0, x8, x19
10078dd10: 8b150108    	add	x8, x8, x21
10078dd14: 385ff108    	ldurb	w8, [x8, #-0x1]
10078dd18: 7100291f    	cmp	w8, #0xa
10078dd1c: 1a9f17e8    	cset	w8, eq
10078dd20: eb080128    	subs	x8, x9, x8
10078dd24: 540000c0    	b.eq	0x10078dd3c <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x158>
10078dd28: 8b080009    	add	x9, x0, x8
10078dd2c: 385ff129    	ldurb	w9, [x9, #-0x1]
10078dd30: 7100353f    	cmp	w9, #0xd
10078dd34: da9f13e9    	csetm	x9, eq
10078dd38: 14000002    	b	0x10078dd40 <__ZN13codexmate_lib4core5relay14managed_blocks27lines_with_complete_strings28_$u7b$$u7b$closure$u7d$$u7d$17hfda4d0b01ccc2d28E+0x15c>
10078dd3c: d2800009    	mov	x9, #0x0                ; =0
10078dd40: 8b080121    	add	x1, x9, x8
10078dd44: a9447bfd    	ldp	x29, x30, [sp, #0x40]
10078dd48: a9434ff4    	ldp	x20, x19, [sp, #0x30]
10078dd4c: a94257f6    	ldp	x22, x21, [sp, #0x20]
10078dd50: 910143ff    	add	sp, sp, #0x50
10078dd54: d65f03c0    	ret

000000010078dd58 <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E>:
10078dd58: d10683ff    	sub	sp, sp, #0x1a0
10078dd5c: a9146ffc    	stp	x28, x27, [sp, #0x140]
10078dd60: a91567fa    	stp	x26, x25, [sp, #0x150]
10078dd64: a9165ff8    	stp	x24, x23, [sp, #0x160]
10078dd68: a91757f6    	stp	x22, x21, [sp, #0x170]
10078dd6c: a9184ff4    	stp	x20, x19, [sp, #0x180]
10078dd70: a9197bfd    	stp	x29, x30, [sp, #0x190]
10078dd74: 910643fd    	add	x29, sp, #0x190
10078dd78: aa0203f6    	mov	x22, x2
10078dd7c: aa0103f5    	mov	x21, x1
10078dd80: aa0003f4    	mov	x20, x0
10078dd84: 910243fa    	add	x26, sp, #0x90
10078dd88: 910243e8    	add	x8, sp, #0x90
10078dd8c: aa0103e0    	mov	x0, x1
10078dd90: 94000b76    	bl	0x100790b68 <__ZN13codexmate_lib4core5relay18transition_journal12read_checked17hd86b1646f346cffbE>
10078dd94: a9494fe8    	ldp	x8, x19, [sp, #0x90]
10078dd98: ad408740    	ldp	q0, q1, [x26, #0x10]
10078dd9c: ad0287e0    	stp	q0, q1, [sp, #0x50]
10078dda0: ad418740    	ldp	q0, q1, [x26, #0x30]
10078dda4: ad0387e0    	stp	q0, q1, [sp, #0x70]
10078dda8: f1002d1f    	cmp	x8, #0xb
10078ddac: 540000e1    	b.ne	0x10078ddc8 <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x70>
10078ddb0: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078ddb4: eb08027f    	cmp	x19, x8
10078ddb8: 54000261    	b.ne	0x10078de04 <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0xac>
10078ddbc: 52800168    	mov	w8, #0xb                ; =11
10078ddc0: f9000288    	str	x8, [x20]
10078ddc4: 14000008    	b	0x10078dde4 <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x8c>
10078ddc8: 3dc01742    	ldr	q2, [x26, #0x50]
10078ddcc: ad4287e0    	ldp	q0, q1, [sp, #0x50]
10078ddd0: ad008680    	stp	q0, q1, [x20, #0x10]
10078ddd4: ad4387e0    	ldp	q0, q1, [sp, #0x70]
10078ddd8: 3d800e80    	str	q0, [x20, #0x30]
10078dddc: ad020a81    	stp	q1, q2, [x20, #0x40]
10078dde0: a9004e88    	stp	x8, x19, [x20]
10078dde4: a9597bfd    	ldp	x29, x30, [sp, #0x190]
10078dde8: a9584ff4    	ldp	x20, x19, [sp, #0x180]
10078ddec: a95757f6    	ldp	x22, x21, [sp, #0x170]
10078ddf0: a9565ff8    	ldp	x24, x23, [sp, #0x160]
10078ddf4: a95567fa    	ldp	x26, x25, [sp, #0x150]
10078ddf8: a9546ffc    	ldp	x28, x27, [sp, #0x140]
10078ddfc: 910683ff    	add	sp, sp, #0x1a0
10078de00: d65f03c0    	ret
10078de04: 12001ec8    	and	w8, w22, #0xff
10078de08: f90007f3    	str	x19, [sp, #0x8]
10078de0c: ad4287e0    	ldp	q0, q1, [sp, #0x50]
10078de10: ad0087e0    	stp	q0, q1, [sp, #0x10]
10078de14: ad4387e0    	ldp	q0, q1, [sp, #0x70]
10078de18: ad0187e0    	stp	q0, q1, [sp, #0x30]
10078de1c: f9400fe9    	ldr	x9, [sp, #0x18]
10078de20: f100293f    	cmp	x9, #0xa
10078de24: 540001e1    	b.ne	0x10078de60 <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x108>
10078de28: f9400be9    	ldr	x9, [sp, #0x10]
10078de2c: f940012a    	ldr	x10, [x9]
10078de30: 79401129    	ldrh	w9, [x9, #0x8]
10078de34: d28dec6b    	mov	x11, #0x6f63            ; =28515
10078de38: f2adadab    	movk	x11, #0x6d6d, lsl #16
10078de3c: f2ce8d2b    	movk	x11, #0x7469, lsl #32
10078de40: f2ed2e8b    	movk	x11, #0x6974, lsl #48
10078de44: eb0b015f    	cmp	x10, x11
10078de48: 528cedca    	mov	w10, #0x676e            ; =26478
10078de4c: 7a4a0120    	ccmp	w9, w10, #0x0, eq
10078de50: 1a9f07e9    	cset	w9, ne
10078de54: 7100091f    	cmp	w8, #0x2
10078de58: 54000041    	b.ne	0x10078de60 <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x108>
10078de5c: 34000a89    	cbz	w9, 0x10078dfac <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x254>
10078de60: 7100091f    	cmp	w8, #0x2
10078de64: 54000080    	b.eq	0x10078de74 <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x11c>
10078de68: 394123e9    	ldrb	w9, [sp, #0x48]
10078de6c: 6b08013f    	cmp	w9, w8
10078de70: 540009e1    	b.ne	0x10078dfac <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x254>
10078de74: 394123f7    	ldrb	w23, [sp, #0x48]
10078de78: f94126b9    	ldr	x25, [x21, #0x248]
10078de7c: f9412ab8    	ldr	x24, [x21, #0x250]
10078de80: b00060e2    	adrp	x2, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10078de84: 91361442    	add	x2, x2, #0xd85
10078de88: d10283a8    	sub	x8, x29, #0xa0
10078de8c: aa1903e0    	mov	x0, x25
10078de90: aa1803e1    	mov	x1, x24
10078de94: 528003c3    	mov	w3, #0x1e               ; =30
10078de98: 942704f2    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
10078de9c: a97687b6    	ldp	x22, x1, [x29, #-0x98]
10078dea0: aa1603e0    	mov	x0, x22
10078dea4: 97ee5ec8    	bl	0x1003259c4 <__ZN13codexmate_lib4core5relay12atomic_write7removal18remove_file_report17ha1da458ac3549cb8E>
10078dea8: aa0103f5    	mov	x21, x1
10078deac: 36000b40    	tbz	w0, #0x0, 0x10078e014 <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x2bc>
10078deb0: f81783b5    	stur	x21, [x29, #-0x88]
10078deb4: aa1503e0    	mov	x0, x21
10078deb8: 94013600    	bl	0x1007db6b8 <__ZN3std2io5error5Error4kind17h247456c019dc3d6bE>
10078debc: 12001c08    	and	w8, w0, #0xff
10078dec0: 381983a8    	sturb	w8, [x29, #-0x68]
10078dec4: d101a3a8    	sub	x8, x29, #0x68
10078dec8: d0000369    	adrp	x9, 0x1007fb000 <__ZN5rayon4iter7collect21collect_with_consumer17hf967c36702cf3c01E+0x30>
10078decc: 912e2129    	add	x9, x9, #0xb88
10078ded0: a90927e8    	stp	x8, x9, [sp, #0x90]
10078ded4: d0006ce0    	adrp	x0, 0x10152b000 <_anon.6408540cdffbe318d49f52812e99783f.200+0x186>
10078ded8: 91104800    	add	x0, x0, #0x412
10078dedc: d10203b7    	sub	x23, x29, #0x80
10078dee0: d10203a8    	sub	x8, x29, #0x80
10078dee4: 910243e1    	add	x1, sp, #0x90
10078dee8: 9427a66b    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10078deec: d0000368    	adrp	x8, 0x1007fb000 <__ZN5rayon4iter7collect21collect_with_consumer17hf967c36702cf3c01E+0x30>
10078def0: 91095108    	add	x8, x8, #0x254
10078def4: a90923f7    	stp	x23, x8, [sp, #0x90]
10078def8: d0006ce0    	adrp	x0, 0x10152b000 <_anon.6408540cdffbe318d49f52812e99783f.200+0x186>
10078defc: 910c1800    	add	x0, x0, #0x306
10078df00: d101a3a8    	sub	x8, x29, #0x68
10078df04: 910243e1    	add	x1, sp, #0x90
10078df08: 9427a663    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10078df0c: f85983b8    	ldur	x24, [x29, #-0x68]
10078df10: 3cca0340    	ldur	q0, [x26, #0xa0]
10078df14: 3d800340    	str	q0, [x26]
10078df18: f85803a1    	ldur	x1, [x29, #-0x80]
10078df1c: b4000081    	cbz	x1, 0x10078df2c <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x1d4>
10078df20: f85883a0    	ldur	x0, [x29, #-0x78]
10078df24: 52800022    	mov	w2, #0x1                ; =1
10078df28: 97e1cd90    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078df2c: 924006a8    	and	x8, x21, #0x3
10078df30: f100051f    	cmp	x8, #0x1
10078df34: 54000201    	b.ne	0x10078df74 <__ZN13codexmate_lib4core5relay18transition_journal11clear_inner17h69195af99754c7c1E+0x21c>
10078df38: f85ffeb7    	ldr	x23, [x21, #-0x1]!
10078df3c: f94006b9    	ldr	x25, [x21, #0x8]
