
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008fbb64 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E>:
1008fbb64: d10303ff    	sub	sp, sp, #0xc0
1008fbb68: a90767fa    	stp	x26, x25, [sp, #0x70]
1008fbb6c: a9085ff8    	stp	x24, x23, [sp, #0x80]
1008fbb70: a90957f6    	stp	x22, x21, [sp, #0x90]
1008fbb74: a90a4ff4    	stp	x20, x19, [sp, #0xa0]
1008fbb78: a90b7bfd    	stp	x29, x30, [sp, #0xb0]
1008fbb7c: 9102c3fd    	add	x29, sp, #0xb0
1008fbb80: aa0003f3    	mov	x19, x0
1008fbb84: d0007fa0    	adrp	x0, 0x1018f1000 <__ZN3std12backtrace_rs9symbolize5gimli5Cache11with_global14MAPPINGS_CACHE17hb3a3380e67302d28E>
1008fbb88: 912f6000    	add	x0, x0, #0xbd8
1008fbb8c: f9400008    	ldr	x8, [x0]
1008fbb90: d63f0100    	blr	x8
1008fbb94: 39404008    	ldrb	w8, [x0, #0x10]
1008fbb98: 7100051f    	cmp	w8, #0x1
1008fbb9c: 54000d81    	b.ne	0x1008fbd4c <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x1e8>
1008fbba0: a9402408    	ldp	x8, x9, [x0]
1008fbba4: 9100050a    	add	x10, x8, #0x1
1008fbba8: f900000a    	str	x10, [x0]
1008fbbac: d000798a    	adrp	x10, 0x10182d000 <_anon.973b090bee24c06fdb949486dbea4242.549+0x1e8>
1008fbbb0: 9118014a    	add	x10, x10, #0x600
1008fbbb4: ad400540    	ldp	q0, q1, [x10]
1008fbbb8: ad0007e0    	stp	q0, q1, [sp]
1008fbbbc: a90227e8    	stp	x8, x9, [sp, #0x20]
1008fbbc0: a9405434    	ldp	x20, x21, [x1]
1008fbbc4: d2f00017    	mov	x23, #-0x8000000000000000 ; =-9223372036854775808
1008fbbc8: eb15029f    	cmp	x20, x21
1008fbbcc: 540003a0    	b.eq	0x1008fbc40 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0xdc>
1008fbbd0: f9400836    	ldr	x22, [x1, #0x10]
1008fbbd4: 9100c3f8    	add	x24, sp, #0x30
1008fbbd8: 14000004    	b	0x1008fbbe8 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x84>
1008fbbdc: 91040294    	add	x20, x20, #0x100
1008fbbe0: eb15029f    	cmp	x20, x21
1008fbbe4: 540002a0    	b.eq	0x1008fbc38 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0xd4>
1008fbbe8: f9406288    	ldr	x8, [x20, #0xc0]
1008fbbec: b7ffff88    	tbnz	x8, #0x3f, 0x1008fbbdc <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x78>
1008fbbf0: a9409283    	ldp	x3, x4, [x20, #0x8]
1008fbbf4: f94126c1    	ldr	x1, [x22, #0x248]
1008fbbf8: f9412ac2    	ldr	x2, [x22, #0x250]
1008fbbfc: a94c9a85    	ldp	x5, x6, [x20, #0xc8]
1008fbc00: 9100c3e0    	add	x0, sp, #0x30
1008fbc04: 97ffa530    	bl	0x1008e50c4 <__ZN13codexmate_lib4core5relay8keychain9revisions8key_path17hd5fc4e16eadecf41E>
1008fbc08: f9401be8    	ldr	x8, [sp, #0x30]
1008fbc0c: f100091f    	cmp	x8, #0x2
1008fbc10: 54fffe60    	b.eq	0x1008fbbdc <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x78>
1008fbc14: 370001a8    	tbnz	w8, #0x0, 0x1008fbc48 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0xe4>
1008fbc18: 3cc08300    	ldur	q0, [x24, #0x8]
1008fbc1c: 3d8017e0    	str	q0, [sp, #0x50]
1008fbc20: f8418308    	ldur	x8, [x24, #0x18]
1008fbc24: f90033e8    	str	x8, [sp, #0x60]
1008fbc28: 910003e0    	mov	x0, sp
1008fbc2c: 910143e1    	add	x1, sp, #0x50
1008fbc30: 97f1e670    	bl	0x1005755f0 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17hcff5980c95ff4507E>
1008fbc34: 17ffffea    	b	0x1008fbbdc <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x78>
1008fbc38: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008fbc3c: 14000005    	b	0x1008fbc50 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0xec>
1008fbc40: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1008fbc44: 14000003    	b	0x1008fbc50 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0xec>
1008fbc48: a943abe8    	ldp	x8, x10, [sp, #0x38]
1008fbc4c: f94027e9    	ldr	x9, [sp, #0x48]
1008fbc50: 910003eb    	mov	x11, sp
1008fbc54: a94057f4    	ldp	x20, x21, [sp]
1008fbc58: 3cc083e0    	ldur	q0, [sp, #0x8]
1008fbc5c: f9400ff6    	ldr	x22, [sp, #0x18]
1008fbc60: 3cc20161    	ldur	q1, [x11, #0x20]
1008fbc64: 3d800fe1    	str	q1, [sp, #0x30]
1008fbc68: eb17011f    	cmp	x8, x23
1008fbc6c: 540000e1    	b.ne	0x1008fbc88 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x124>
1008fbc70: f9000274    	str	x20, [x19]
1008fbc74: 3c808260    	stur	q0, [x19, #0x8]
1008fbc78: f9000e76    	str	x22, [x19, #0x18]
1008fbc7c: 3dc00fe0    	ldr	q0, [sp, #0x30]
1008fbc80: 3d800a60    	str	q0, [x19, #0x20]
1008fbc84: 1400002b    	b	0x1008fbd30 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x1cc>
1008fbc88: a901266a    	stp	x10, x9, [x19, #0x10]
1008fbc8c: a900227f    	stp	xzr, x8, [x19]
1008fbc90: b4000515    	cbz	x21, 0x1008fbd30 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x1cc>
1008fbc94: b40003d6    	cbz	x22, 0x1008fbd0c <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x1a8>
1008fbc98: aa1403f3    	mov	x19, x20
1008fbc9c: fc408660    	ldr	d0, [x19], #0x8
1008fbca0: 2e208800    	cmge.8b	v0, v0, #0
1008fbca4: 9e660008    	fmov	x8, d0
1008fbca8: 9201c119    	and	x25, x8, #0x8080808080808080
1008fbcac: 52800317    	mov	w23, #0x18              ; =24
1008fbcb0: aa1403f8    	mov	x24, x20
1008fbcb4: 14000005    	b	0x1008fbcc8 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x164>
1008fbcb8: d1000728    	sub	x8, x25, #0x1
1008fbcbc: 8a190119    	and	x25, x8, x25
1008fbcc0: d10006d6    	sub	x22, x22, #0x1
1008fbcc4: b4000256    	cbz	x22, 0x1008fbd0c <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x1a8>
1008fbcc8: b50000f9    	cbnz	x25, 0x1008fbce4 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x180>
1008fbccc: fc408660    	ldr	d0, [x19], #0x8
1008fbcd0: 2e208800    	cmge.8b	v0, v0, #0
1008fbcd4: 9e660008    	fmov	x8, d0
1008fbcd8: d1030318    	sub	x24, x24, #0xc0
1008fbcdc: 9201c119    	and	x25, x8, #0x8080808080808080
1008fbce0: b4ffff79    	cbz	x25, 0x1008fbccc <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x168>
1008fbce4: dac00328    	rbit	x8, x25
1008fbce8: dac01108    	clz	x8, x8
1008fbcec: cb480fe8    	neg	x8, x8, lsr #3
1008fbcf0: 9b376108    	smaddl	x8, w8, w23, x24
1008fbcf4: f85e8101    	ldur	x1, [x8, #-0x18]
1008fbcf8: b4fffe01    	cbz	x1, 0x1008fbcb8 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x154>
1008fbcfc: f85f0100    	ldur	x0, [x8, #-0x10]
1008fbd00: 52800022    	mov	w2, #0x1                ; =1
1008fbd04: 97dc1619    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008fbd08: 17ffffec    	b	0x1008fbcb8 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x154>
1008fbd0c: 8b1506a8    	add	x8, x21, x21, lsl #1
1008fbd10: d37df108    	lsl	x8, x8, #3
1008fbd14: 8b150109    	add	x9, x8, x21
1008fbd18: b1008521    	adds	x1, x9, #0x21
1008fbd1c: 540000a0    	b.eq	0x1008fbd30 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x1cc>
1008fbd20: cb080288    	sub	x8, x20, x8
1008fbd24: d1006100    	sub	x0, x8, #0x18
1008fbd28: 52800102    	mov	w2, #0x8                ; =8
1008fbd2c: 97dc160f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1008fbd30: a94b7bfd    	ldp	x29, x30, [sp, #0xb0]
1008fbd34: a94a4ff4    	ldp	x20, x19, [sp, #0xa0]
1008fbd38: a94957f6    	ldp	x22, x21, [sp, #0x90]
1008fbd3c: a9485ff8    	ldp	x24, x23, [sp, #0x80]
1008fbd40: a94767fa    	ldp	x26, x25, [sp, #0x70]
1008fbd44: 910303ff    	add	sp, sp, #0xc0
1008fbd48: d65f03c0    	ret
1008fbd4c: aa0103f4    	mov	x20, x1
1008fbd50: aa0003f5    	mov	x21, x0
1008fbd54: 94213dd0    	bl	0x10114b494 <__ZN3std3sys6random19hashmap_random_keys17h8d42fafffd67301dE>
1008fbd58: aa0003e8    	mov	x8, x0
1008fbd5c: aa1503e0    	mov	x0, x21
1008fbd60: aa0103e9    	mov	x9, x1
1008fbd64: aa1403e1    	mov	x1, x20
1008fbd68: a90026a8    	stp	x8, x9, [x21]
1008fbd6c: 5280002a    	mov	w10, #0x1               ; =1
1008fbd70: 390042aa    	strb	w10, [x21, #0x10]
1008fbd74: 17ffff8c    	b	0x1008fbba4 <__ZN4core4iter8adapters11try_process17h60aa98f71b85c907E+0x40>
1008fbd78: aa0003f3    	mov	x19, x0
1008fbd7c: 910003e0    	mov	x0, sp
1008fbd80: 97fdd0e5    	bl	0x100870114 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9aa9677d07c4e8b5E>
1008fbd84: aa1303e0    	mov	x0, x19
1008fbd88: 942354f3    	bl	0x1011d1154 <_writev+0x1011d1154>
1008fbd8c: 9423536f    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

00000001008fbd90 <__ZN4core4iter8adapters11try_process17ha8eac7c8ac7ff1b2E>:
1008fbd90: d10403ff    	sub	sp, sp, #0x100
1008fbd94: a90b67fa    	stp	x26, x25, [sp, #0xb0]
1008fbd98: a90c5ff8    	stp	x24, x23, [sp, #0xc0]
1008fbd9c: a90d57f6    	stp	x22, x21, [sp, #0xd0]
1008fbda0: a90e4ff4    	stp	x20, x19, [sp, #0xe0]
1008fbda4: a90f7bfd    	stp	x29, x30, [sp, #0xf0]
1008fbda8: 9103c3fd    	add	x29, sp, #0xf0
1008fbdac: aa0803f3    	mov	x19, x8
1008fbdb0: d28002d5    	mov	x21, #0x16              ; =22
1008fbdb4: f2f00015    	movk	x21, #0x8000, lsl #48
1008fbdb8: f90007f5    	str	x21, [sp, #0x8]
1008fbdbc: 910023e8    	add	x8, sp, #0x8
1008fbdc0: a90483e8    	stp	x8, x0, [sp, #0x48]
1008fbdc4: f9002fe1    	str	x1, [sp, #0x58]
1008fbdc8: d0007fa0    	adrp	x0, 0x1018f1000 <__ZN3std12backtrace_rs9symbolize5gimli5Cache11with_global14MAPPINGS_CACHE17hb3a3380e67302d28E>
