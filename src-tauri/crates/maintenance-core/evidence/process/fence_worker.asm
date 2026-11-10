
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010044aaec <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E>:
10044aaec: 6db82beb    	stp	d11, d10, [sp, #-0x80]!
10044aaf0: 6d0123e9    	stp	d9, d8, [sp, #0x10]
10044aaf4: a9026ffc    	stp	x28, x27, [sp, #0x20]
10044aaf8: a90367fa    	stp	x26, x25, [sp, #0x30]
10044aafc: a9045ff8    	stp	x24, x23, [sp, #0x40]
10044ab00: a90557f6    	stp	x22, x21, [sp, #0x50]
10044ab04: a9064ff4    	stp	x20, x19, [sp, #0x60]
10044ab08: a9077bfd    	stp	x29, x30, [sp, #0x70]
10044ab0c: 9101c3fd    	add	x29, sp, #0x70
10044ab10: d109c3ff    	sub	sp, sp, #0x270
10044ab14: aa0003f3    	mov	x19, x0
10044ab18: 52800088    	mov	w8, #0x4                ; =4
10044ab1c: a90223ff    	stp	xzr, x8, [sp, #0x20]
10044ab20: f9001bff    	str	xzr, [sp, #0x30]
10044ab24: 5280003a    	mov	w26, #0x1               ; =1
10044ab28: 790073fa    	strh	w26, [sp, #0x38]
10044ab2c: 3900ebff    	strb	wzr, [sp, #0x3a]
10044ab30: f9400008    	ldr	x8, [x0]
10044ab34: 19410108    	ldapurb	w8, [x8, #0x10]
10044ab38: 34000268    	cbz	w8, 0x10044ab84 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x98>
10044ab3c: f94013e8    	ldr	x8, [sp, #0x20]
10044ab40: b40000a8    	cbz	x8, 0x10044ab54 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x68>
10044ab44: f94017e0    	ldr	x0, [sp, #0x28]
10044ab48: d37ef501    	lsl	x1, x8, #2
10044ab4c: 52800082    	mov	w2, #0x4                ; =4
10044ab50: 97eeda86    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044ab54: aa1303e0    	mov	x0, x19
10044ab58: 94004870    	bl	0x10045cd18 <__ZN4core3ptr125drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence..acquire_with_policy..$u7b$$u7b$closure$u7d$$u7d$$GT$17h897d5d6841df97bfE>
10044ab5c: 9109c3ff    	add	sp, sp, #0x270
10044ab60: a9477bfd    	ldp	x29, x30, [sp, #0x70]
10044ab64: a9464ff4    	ldp	x20, x19, [sp, #0x60]
10044ab68: a94557f6    	ldp	x22, x21, [sp, #0x50]
10044ab6c: a9445ff8    	ldp	x24, x23, [sp, #0x40]
10044ab70: a94367fa    	ldp	x26, x25, [sp, #0x30]
10044ab74: a9426ffc    	ldp	x28, x27, [sp, #0x20]
10044ab78: 6d4123e9    	ldp	d9, d8, [sp, #0x10]
10044ab7c: 6cc82beb    	ldp	d11, d10, [sp], #0x80
10044ab80: d65f03c0    	ret
10044ab84: d103a3b9    	sub	x25, x29, #0xe8
10044ab88: 9000a55b    	adrp	x27, 0x1018f2000 <__ZN13codexmate_lib13EXIT_APPROVED17h1f9ee2eac97195fdE.0>
10044ab8c: 910fe37b    	add	x27, x27, #0x3f8
10044ab90: d00075c8    	adrp	x8, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
10044ab94: 3dc01500    	ldr	q0, [x8, #0x50]
10044ab98: 3d8007e0    	str	q0, [sp, #0x10]
10044ab9c: d00075c8    	adrp	x8, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
10044aba0: 3dc01900    	ldr	q0, [x8, #0x60]
10044aba4: 3d8003e0    	str	q0, [sp]
10044aba8: b0007648    	adrp	x8, 0x101313000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1578+0x9b4>
10044abac: fd44e108    	ldr	d8, [x8, #0x9c0]
10044abb0: b0007648    	adrp	x8, 0x101313000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1578+0x9b4>
10044abb4: fd44e509    	ldr	d9, [x8, #0x9c8]
10044abb8: b0007648    	adrp	x8, 0x101313000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1578+0x9b4>
10044abbc: fd44dd0a    	ldr	d10, [x8, #0x9b8]
10044abc0: 1400000a    	b	0x10044abe8 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0xfc>
10044abc4: 910643e0    	add	x0, sp, #0x190
10044abc8: 940063ee    	bl	0x100463b80 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
10044abcc: d2800000    	mov	x0, #0x0                ; =0
10044abd0: 52968001    	mov	w1, #0xb400             ; =46080
10044abd4: 72a09881    	movk	w1, #0x4c4, lsl #16
10044abd8: 94341a88    	bl	0x1011515f8 <__ZN3std6thread9functions5sleep17hea2e257bc341ef60E>
10044abdc: f9400268    	ldr	x8, [x19]
10044abe0: 19410108    	ldapurb	w8, [x8, #0x10]
10044abe4: 35fffac8    	cbnz	w8, 0x10044ab3c <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x50>
10044abe8: 910103e0    	add	x0, sp, #0x40
10044abec: 940dd9cb    	bl	0x1007c1318 <__ZN13codexmate_lib8platform7process26collect_codex_kill_targets17h72d6737d13674bd4E>
10044abf0: f94023e8    	ldr	x8, [sp, #0x40]
10044abf4: f1002d1f    	cmp	x8, #0xb
10044abf8: 54000781    	b.ne	0x10044ace8 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x1fc>
10044abfc: 910103e8    	add	x8, sp, #0x40
10044ac00: 3cc08100    	ldur	q0, [x8, #0x8]
10044ac04: 3d802be0    	str	q0, [sp, #0xa0]
10044ac08: f8418117    	ldur	x23, [x8, #0x18]
10044ac0c: f9005bf7    	str	x23, [sp, #0xb0]
10044ac10: f94057f6    	ldr	x22, [sp, #0xa8]
10044ac14: 910083e0    	add	x0, sp, #0x20
10044ac18: aa1603e1    	mov	x1, x22
10044ac1c: aa1703e2    	mov	x2, x23
10044ac20: 940dd669    	bl	0x1007c05c4 <__ZN13codexmate_lib8platform7process23WriterGenerationTracker7observe17h61090192a63532d3E>
10044ac24: 12001c1c    	and	w28, w0, #0xff
10044ac28: 7100139f    	cmp	w28, #0x4
10044ac2c: 1adc2348    	lsl	w8, w26, w28
10044ac30: 528002c9    	mov	w9, #0x16               ; =22
10044ac34: 0a090108    	and	w8, w8, w9
10044ac38: 7a409904    	ccmp	w8, #0x0, #0x4, ls
10044ac3c: 54000da0    	b.eq	0x10044adf0 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x304>
10044ac40: 7100139f    	cmp	w28, #0x4
10044ac44: 540000c1    	b.ne	0x10044ac5c <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x170>
10044ac48: f9400668    	ldr	x8, [x19, #0x8]
10044ac4c: 91004108    	add	x8, x8, #0x10
10044ac50: b8fa0108    	ldaddal	w26, w8, [x8]
10044ac54: 11000515    	add	w21, w8, #0x1
10044ac58: a94adff6    	ldp	x22, x23, [sp, #0xa8]
10044ac5c: 3940ebe8    	ldrb	w8, [sp, #0x3a]
10044ac60: 11000509    	add	w9, w8, #0x1
10044ac64: 7103fd3f    	cmp	w9, #0xff
10044ac68: 52801fe9    	mov	w9, #0xff               ; =255
10044ac6c: 1a882528    	csinc	w8, w9, w8, hs
10044ac70: 3902ffe8    	strb	w8, [sp, #0xbf]
10044ac74: 8b1716c1    	add	x1, x22, x23, lsl #5
10044ac78: 910643e8    	add	x8, sp, #0x190
10044ac7c: aa1603e0    	mov	x0, x22
10044ac80: 94039337    	bl	0x10052f95c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17haff5edd42b8c568cE>
10044ac84: d103a3b4    	sub	x20, x29, #0xe8
10044ac88: a959e7f8    	ldp	x24, x25, [sp, #0x198]
10044ac8c: d103a3a8    	sub	x8, x29, #0xe8
10044ac90: aa1803e0    	mov	x0, x24
10044ac94: aa1903e1    	mov	x1, x25
10044ac98: b0007662    	adrp	x2, 0x101317000 <_anon.a9e2253c03df711f51070ce6102367ea.92+0x218c>
10044ac9c: 913a7c42    	add	x2, x2, #0xe9f
10044aca0: 52800063    	mov	w3, #0x3                ; =3
10044aca4: 97f79ba8    	bl	0x100231b44 <__ZN5alloc3str17join_generic_copy17hc3fffb246d6f0b82E>
10044aca8: 3dc00280    	ldr	q0, [x20]
10044acac: 3d8033e0    	str	q0, [sp, #0xc0]
10044acb0: f85283a8    	ldur	x8, [x29, #-0xd8]
10044acb4: f9006be8    	str	x8, [sp, #0xd0]
10044acb8: b4000639    	cbz	x25, 0x10044ad7c <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x290>
10044acbc: 91002314    	add	x20, x24, #0x8
10044acc0: 14000004    	b	0x10044acd0 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x1e4>
10044acc4: 91006294    	add	x20, x20, #0x18
10044acc8: f1000739    	subs	x25, x25, #0x1
10044accc: 54000580    	b.eq	0x10044ad7c <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x290>
10044acd0: f85f8281    	ldur	x1, [x20, #-0x8]
10044acd4: b4ffff81    	cbz	x1, 0x10044acc4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x1d8>
10044acd8: f9400280    	ldr	x0, [x20]
10044acdc: 52800022    	mov	w2, #0x1                ; =1
10044ace0: 97eeda22    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044ace4: 17fffff8    	b	0x10044acc4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x1d8>
10044ace8: ad4307e0    	ldp	q0, q1, [sp, #0x60]
10044acec: ad0d87e0    	stp	q0, q1, [sp, #0x1b0]
10044acf0: ad4407e0    	ldp	q0, q1, [sp, #0x80]
10044acf4: ad0e87e0    	stp	q0, q1, [sp, #0x1d0]
10044acf8: ad4207e0    	ldp	q0, q1, [sp, #0x40]
10044acfc: ad0c87e0    	stp	q0, q1, [sp, #0x190]
10044ad00: f9400a68    	ldr	x8, [x19, #0x10]
10044ad04: 1901011a    	stlurb	w26, [x8, #0x10]
10044ad08: f9400368    	ldr	x8, [x27]
10044ad0c: f100051f    	cmp	x8, #0x1
10044ad10: 54fff5a9    	b.ls	0x10044abc4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0xd8>
10044ad14: 910643e8    	add	x8, sp, #0x190
10044ad18: f90073e8    	str	x8, [sp, #0xe0]
10044ad1c: d0001d88    	adrp	x8, 0x1007fc000 <__ZN6chrono8datetime40DateTime$LT$chrono..offset..utc..Utc$GT$14from_timestamp17h2b34124b50daed04E+0x8>
10044ad20: 91343108    	add	x8, x8, #0xd0c
10044ad24: f90077e8    	str	x8, [sp, #0xe8]
10044ad28: 52800408    	mov	w8, #0x20               ; =32
10044ad2c: f81583a8    	stur	x8, [x29, #-0xa8]
10044ad30: d00086c8    	adrp	x8, 0x101524000 <_anon.164d03a2dfc98f969363eaf1c50504fd.794+0x2931>
10044ad34: 91385909    	add	x9, x8, #0xe16
10044ad38: 910383e8    	add	x8, sp, #0xe0
10044ad3c: a936a3a9    	stp	x9, x8, [x29, #-0x98]
10044ad40: b0007668    	adrp	x8, 0x101317000 <_anon.a9e2253c03df711f51070ce6102367ea.92+0x218c>
10044ad44: 913a8d08    	add	x8, x8, #0xea3
10044ad48: a931a3bf    	stp	xzr, x8, [x29, #-0xe8]
10044ad4c: ad4007e0    	ldp	q0, q1, [sp]
10044ad50: 3d800721    	str	q1, [x25, #0x10]
10044ad54: f81503a8    	stur	x8, [x29, #-0xb0]
10044ad58: d00086c8    	adrp	x8, 0x101524000 <_anon.164d03a2dfc98f969363eaf1c50504fd.794+0x2931>
10044ad5c: 91368508    	add	x8, x8, #0xda1
10044ad60: f81383a8    	stur	x8, [x29, #-0xc8]
10044ad64: 3c828320    	stur	q0, [x25, #0x28]
10044ad68: fc1603aa    	stur	d10, [x29, #-0xa0]
10044ad6c: d10207a0    	sub	x0, x29, #0x81
10044ad70: d103a3a1    	sub	x1, x29, #0xe8
10044ad74: 942e27cb    	bl	0x100fd4ca0 <__ZN61_$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$3log17he2f3dc859ac0f3c8E>
10044ad78: 17ffff93    	b	0x10044abc4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0xd8>
10044ad7c: f940cbe8    	ldr	x8, [sp, #0x190]
10044ad80: b40000c8    	cbz	x8, 0x10044ad98 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x2ac>
10044ad84: 8b080508    	add	x8, x8, x8, lsl #1
10044ad88: d37df101    	lsl	x1, x8, #3
10044ad8c: aa1803e0    	mov	x0, x24
10044ad90: 52800102    	mov	w2, #0x8                ; =8
10044ad94: 97eed9f5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044ad98: f9400368    	ldr	x8, [x27]
10044ad9c: f100051f    	cmp	x8, #0x1
10044ada0: d103a3b9    	sub	x25, x29, #0xe8
10044ada4: 54000a69    	b.ls	0x10044aef0 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x404>
10044ada8: 7100139f    	cmp	w28, #0x4
10044adac: 54000501    	b.ne	0x10044ae4c <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x360>
10044adb0: b900e3f5    	str	w21, [sp, #0xe0]
10044adb4: 910383e8    	add	x8, sp, #0xe0
10044adb8: f900cbe8    	str	x8, [sp, #0x190]
10044adbc: b0006988    	adrp	x8, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
10044adc0: 910bb108    	add	x8, x8, #0x2ec
10044adc4: f900cfe8    	str	x8, [sp, #0x198]
10044adc8: d103a3a8    	sub	x8, x29, #0xe8
10044adcc: 910643e1    	add	x1, sp, #0x190
10044add0: d00086c0    	adrp	x0, 0x101524000 <_anon.164d03a2dfc98f969363eaf1c50504fd.794+0x2931>
10044add4: 91393c00    	add	x0, x0, #0xe4f
10044add8: 9434b2af    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10044addc: 3dc00320    	ldr	q0, [x25]
10044ade0: 3d803be0    	str	q0, [sp, #0xe0]
10044ade4: f85283a8    	ldur	x8, [x29, #-0xd8]
10044ade8: f9007be8    	str	x8, [sp, #0xf0]
10044adec: 1400001a    	b	0x10044ae54 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x368>
10044adf0: d2800000    	mov	x0, #0x0                ; =0
10044adf4: 52968001    	mov	w1, #0xb400             ; =46080
10044adf8: 72a09881    	movk	w1, #0x4c4, lsl #16
10044adfc: 943419ff    	bl	0x1011515f8 <__ZN3std6thread9functions5sleep17hea2e257bc341ef60E>
10044ae00: b4000197    	cbz	x23, 0x10044ae30 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x344>
10044ae04: 910042d4    	add	x20, x22, #0x10
10044ae08: 14000004    	b	0x10044ae18 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x32c>
10044ae0c: 91008294    	add	x20, x20, #0x20
10044ae10: f10006f7    	subs	x23, x23, #0x1
10044ae14: 540000e0    	b.eq	0x10044ae30 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x344>
10044ae18: f85f8281    	ldur	x1, [x20, #-0x8]
10044ae1c: b4ffff81    	cbz	x1, 0x10044ae0c <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x320>
10044ae20: f9400280    	ldr	x0, [x20]
10044ae24: 52800022    	mov	w2, #0x1                ; =1
10044ae28: 97eed9d0    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044ae2c: 17fffff8    	b	0x10044ae0c <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x320>
10044ae30: f94053e8    	ldr	x8, [sp, #0xa0]
10044ae34: b4ffed48    	cbz	x8, 0x10044abdc <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0xf0>
10044ae38: d37be901    	lsl	x1, x8, #5
10044ae3c: aa1603e0    	mov	x0, x22
10044ae40: 52800102    	mov	w2, #0x8                ; =8
10044ae44: 97eed9c9    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044ae48: 17ffff65    	b	0x10044abdc <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0xf0>
10044ae4c: a90e6bff    	stp	xzr, x26, [sp, #0xe0]
10044ae50: f9007bff    	str	xzr, [sp, #0xf0]
10044ae54: 910383e8    	add	x8, sp, #0xe0
10044ae58: f900cbe8    	str	x8, [sp, #0x190]
10044ae5c: d00000e8    	adrp	x8, 0x100468000 <__ZN4core5error5Error7type_id17h3be870d237e4c2a8E+0x10>
10044ae60: 912ec108    	add	x8, x8, #0xbb0
10044ae64: 9102ffe9    	add	x9, sp, #0xbf
10044ae68: a919a7e8    	stp	x8, x9, [sp, #0x198]
10044ae6c: 90006989    	adrp	x9, 0x10117a000 <__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd1cd0d46b73abe55E+0x114>
10044ae70: 913cf12b    	add	x11, x9, #0xf3c
10044ae74: b000766a    	adrp	x10, 0x101317000 <_anon.a9e2253c03df711f51070ce6102367ea.92+0x218c>
10044ae78: 913a894a    	add	x10, x10, #0xea2
10044ae7c: a91aabeb    	stp	x11, x10, [sp, #0x1a8]
10044ae80: 910303e9    	add	x9, sp, #0xc0
10044ae84: a91ba7eb    	stp	x11, x9, [sp, #0x1b8]
10044ae88: f900e7e8    	str	x8, [sp, #0x1c8]
10044ae8c: 52800408    	mov	w8, #0x20               ; =32
10044ae90: f81583a8    	stur	x8, [x29, #-0xa8]
10044ae94: d00086c8    	adrp	x8, 0x101524000 <_anon.164d03a2dfc98f969363eaf1c50504fd.794+0x2931>
10044ae98: 9134d509    	add	x9, x8, #0xd35
10044ae9c: 910643e8    	add	x8, sp, #0x190
10044aea0: a936a3a9    	stp	x9, x8, [x29, #-0x98]
10044aea4: b0007668    	adrp	x8, 0x101317000 <_anon.a9e2253c03df711f51070ce6102367ea.92+0x218c>
10044aea8: 913a8d08    	add	x8, x8, #0xea3
10044aeac: a931a3bf    	stp	xzr, x8, [x29, #-0xe8]
10044aeb0: ad4007e0    	ldp	q0, q1, [sp]
10044aeb4: 3d800721    	str	q1, [x25, #0x10]
10044aeb8: f81503a8    	stur	x8, [x29, #-0xb0]
10044aebc: d00086c8    	adrp	x8, 0x101524000 <_anon.164d03a2dfc98f969363eaf1c50504fd.794+0x2931>
10044aec0: 91368508    	add	x8, x8, #0xda1
10044aec4: f81383a8    	stur	x8, [x29, #-0xc8]
10044aec8: 3c828320    	stur	q0, [x25, #0x28]
10044aecc: fc1603a8    	stur	d8, [x29, #-0xa0]
10044aed0: d10207a0    	sub	x0, x29, #0x81
10044aed4: d103a3a1    	sub	x1, x29, #0xe8
10044aed8: 942e2772    	bl	0x100fd4ca0 <__ZN61_$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$3log17he2f3dc859ac0f3c8E>
10044aedc: f94073e1    	ldr	x1, [sp, #0xe0]
10044aee0: b4000081    	cbz	x1, 0x10044aef0 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x404>
10044aee4: f94077e0    	ldr	x0, [sp, #0xe8]
10044aee8: 52800022    	mov	w2, #0x1                ; =1
10044aeec: 97eed99f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044aef0: 39406262    	ldrb	w2, [x19, #0x18]
10044aef4: 910383e8    	add	x8, sp, #0xe0
10044aef8: aa1603e0    	mov	x0, x22
10044aefc: aa1703e1    	mov	x1, x23
10044af00: d2800003    	mov	x3, #0x0                ; =0
10044af04: 940dec37    	bl	0x1007c5fe0 <__ZN13codexmate_lib8platform7process32quiesce_codex_transition_targets17hf308d2f82053aedcE>
10044af08: f94073e8    	ldr	x8, [sp, #0xe0]
10044af0c: f1002d1f    	cmp	x8, #0xb
10044af10: 54000061    	b.ne	0x10044af1c <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x430>
10044af14: 780393fa    	sturh	w26, [sp, #0x39]
10044af18: 1400003b    	b	0x10044b004 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x518>
10044af1c: ad4807e0    	ldp	q0, q1, [sp, #0x100]
10044af20: ad0d87e0    	stp	q0, q1, [sp, #0x1b0]
10044af24: ad4907e0    	ldp	q0, q1, [sp, #0x120]
10044af28: ad0e87e0    	stp	q0, q1, [sp, #0x1d0]
10044af2c: ad4707e0    	ldp	q0, q1, [sp, #0xe0]
10044af30: ad0c87e0    	stp	q0, q1, [sp, #0x190]
10044af34: 3940ebe8    	ldrb	w8, [sp, #0x3a]
10044af38: 11000509    	add	w9, w8, #0x1
10044af3c: 7103fd3f    	cmp	w9, #0xff
10044af40: 52801fe9    	mov	w9, #0xff               ; =255
10044af44: 1a882528    	csinc	w8, w9, w8, hs
10044af48: 3900ebe8    	strb	w8, [sp, #0x3a]
10044af4c: 7100091f    	cmp	w8, #0x2
10044af50: 1a9f97e8    	cset	w8, hi
10044af54: 39053fe8    	strb	w8, [sp, #0x14f]
10044af58: f9400a68    	ldr	x8, [x19, #0x10]
10044af5c: 1901011a    	stlurb	w26, [x8, #0x10]
10044af60: f9400368    	ldr	x8, [x27]
10044af64: f100051f    	cmp	x8, #0x1
10044af68: 540004a9    	b.ls	0x10044affc <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x510>
10044af6c: 9102ffe8    	add	x8, sp, #0xbf
10044af70: f900abe8    	str	x8, [sp, #0x150]
10044af74: 90006988    	adrp	x8, 0x10117a000 <__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd1cd0d46b73abe55E+0x114>
10044af78: 913cf10a    	add	x10, x8, #0xf3c
10044af7c: b0007669    	adrp	x9, 0x101317000 <_anon.a9e2253c03df711f51070ce6102367ea.92+0x218c>
10044af80: 913a8929    	add	x9, x9, #0xea2
10044af84: a915a7ea    	stp	x10, x9, [sp, #0x158]
10044af88: 91053fe8    	add	x8, sp, #0x14f
10044af8c: a916a3ea    	stp	x10, x8, [sp, #0x168]
10044af90: 90006988    	adrp	x8, 0x10117a000 <__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd1cd0d46b73abe55E+0x114>
10044af94: 91019109    	add	x9, x8, #0x64
10044af98: 910643e8    	add	x8, sp, #0x190
10044af9c: a917a3e9    	stp	x9, x8, [sp, #0x178]
10044afa0: d0001d88    	adrp	x8, 0x1007fc000 <__ZN6chrono8datetime40DateTime$LT$chrono..offset..utc..Utc$GT$14from_timestamp17h2b34124b50daed04E+0x8>
10044afa4: 91343108    	add	x8, x8, #0xd0c
10044afa8: f900c7e8    	str	x8, [sp, #0x188]
10044afac: 52800408    	mov	w8, #0x20               ; =32
10044afb0: f81583a8    	stur	x8, [x29, #-0xa8]
10044afb4: d00086c8    	adrp	x8, 0x101524000 <_anon.164d03a2dfc98f969363eaf1c50504fd.794+0x2931>
10044afb8: 9136e509    	add	x9, x8, #0xdb9
10044afbc: 910543e8    	add	x8, sp, #0x150
10044afc0: a936a3a9    	stp	x9, x8, [x29, #-0x98]
10044afc4: b0007668    	adrp	x8, 0x101317000 <_anon.a9e2253c03df711f51070ce6102367ea.92+0x218c>
10044afc8: 913a8d08    	add	x8, x8, #0xea3
10044afcc: a931a3bf    	stp	xzr, x8, [x29, #-0xe8]
10044afd0: ad4007e0    	ldp	q0, q1, [sp]
10044afd4: 3d800721    	str	q1, [x25, #0x10]
10044afd8: f81503a8    	stur	x8, [x29, #-0xb0]
10044afdc: d00086c8    	adrp	x8, 0x101524000 <_anon.164d03a2dfc98f969363eaf1c50504fd.794+0x2931>
10044afe0: 91368508    	add	x8, x8, #0xda1
10044afe4: f81383a8    	stur	x8, [x29, #-0xc8]
10044afe8: 3c828320    	stur	q0, [x25, #0x28]
10044afec: fc1603a9    	stur	d9, [x29, #-0xa0]
10044aff0: d10207a0    	sub	x0, x29, #0x81
10044aff4: d103a3a1    	sub	x1, x29, #0xe8
10044aff8: 942e272a    	bl	0x100fd4ca0 <__ZN61_$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$3log17he2f3dc859ac0f3c8E>
10044affc: 910643e0    	add	x0, sp, #0x190
10044b000: 940062e0    	bl	0x100463b80 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
10044b004: f94063e1    	ldr	x1, [sp, #0xc0]
10044b008: b4000081    	cbz	x1, 0x10044b018 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x52c>
10044b00c: f94067e0    	ldr	x0, [sp, #0xc8]
10044b010: 52800022    	mov	w2, #0x1                ; =1
10044b014: 97eed955    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044b018: b4000197    	cbz	x23, 0x10044b048 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x55c>
10044b01c: 910042d4    	add	x20, x22, #0x10
10044b020: 14000004    	b	0x10044b030 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x544>
10044b024: 91008294    	add	x20, x20, #0x20
10044b028: f10006f7    	subs	x23, x23, #0x1
10044b02c: 540000e0    	b.eq	0x10044b048 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x55c>
10044b030: f85f8281    	ldur	x1, [x20, #-0x8]
10044b034: b4ffff81    	cbz	x1, 0x10044b024 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x538>
10044b038: f9400280    	ldr	x0, [x20]
10044b03c: 52800022    	mov	w2, #0x1                ; =1
10044b040: 97eed94a    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044b044: 17fffff8    	b	0x10044b024 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x538>
10044b048: f94053e8    	ldr	x8, [sp, #0xa0]
10044b04c: b4ffdc08    	cbz	x8, 0x10044abcc <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0xe0>
10044b050: d37be901    	lsl	x1, x8, #5
10044b054: aa1603e0    	mov	x0, x22
10044b058: 52800102    	mov	w2, #0x8                ; =8
10044b05c: 97eed943    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044b060: 17fffedb    	b	0x10044abcc <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0xe0>
10044b064: aa0003f4    	mov	x20, x0
10044b068: 910643e0    	add	x0, sp, #0x190
10044b06c: 940062c5    	bl	0x100463b80 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
10044b070: 14000012    	b	0x10044b0b8 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x5cc>
10044b074: aa0003f4    	mov	x20, x0
10044b078: f94073e1    	ldr	x1, [sp, #0xe0]
10044b07c: b40001e1    	cbz	x1, 0x10044b0b8 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x5cc>
10044b080: f94077e0    	ldr	x0, [sp, #0xe8]
10044b084: 52800022    	mov	w2, #0x1                ; =1
10044b088: 97eed938    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044b08c: 1400000b    	b	0x10044b0b8 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x5cc>
10044b090: 14000009    	b	0x10044b0b4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x5c8>
10044b094: aa0003f4    	mov	x20, x0
10044b098: 910643e0    	add	x0, sp, #0x190
10044b09c: 940062b9    	bl	0x100463b80 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
10044b0a0: 14000011    	b	0x10044b0e4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x5f8>
10044b0a4: aa0003f4    	mov	x20, x0
10044b0a8: 910643e0    	add	x0, sp, #0x190
10044b0ac: 9400658f    	bl	0x1004646e8 <__ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17h265e755c5c638973E>
10044b0b0: 14000009    	b	0x10044b0d4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x5e8>
10044b0b4: aa0003f4    	mov	x20, x0
10044b0b8: f94063e1    	ldr	x1, [sp, #0xc0]
10044b0bc: b40000c1    	cbz	x1, 0x10044b0d4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x5e8>
10044b0c0: f94067e0    	ldr	x0, [sp, #0xc8]
10044b0c4: 52800022    	mov	w2, #0x1                ; =1
10044b0c8: 97eed928    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044b0cc: 14000002    	b	0x10044b0d4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x5e8>
10044b0d0: aa0003f4    	mov	x20, x0
10044b0d4: 910283e0    	add	x0, sp, #0xa0
10044b0d8: 94006bbc    	bl	0x100465fc8 <__ZN4core3ptr79drop_in_place$LT$alloc..vec..Vec$LT$$LP$u32$C$alloc..string..String$RP$$GT$$GT$17ha31420ebb7698db2E>
10044b0dc: 14000002    	b	0x10044b0e4 <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x5f8>
10044b0e0: aa0003f4    	mov	x20, x0
10044b0e4: f94013e8    	ldr	x8, [sp, #0x20]
10044b0e8: b40000a8    	cbz	x8, 0x10044b0fc <__ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf5f6cf5d28e72cc1E+0x610>
10044b0ec: f94017e0    	ldr	x0, [sp, #0x28]
10044b0f0: d37ef501    	lsl	x1, x8, #2
10044b0f4: 52800082    	mov	w2, #0x4                ; =4
10044b0f8: 97eed91c    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10044b0fc: aa1303e0    	mov	x0, x19
10044b100: 94004706    	bl	0x10045cd18 <__ZN4core3ptr125drop_in_place$LT$codexmate_lib..platform..process..CodexTransitionFence..acquire_with_policy..$u7b$$u7b$closure$u7d$$u7d$$GT$17h897d5d6841df97bfE>
10044b104: aa1403e0    	mov	x0, x20
10044b108: 94361813    	bl	0x1011d1154 <_writev+0x1011d1154>
10044b10c: 9436168f    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
