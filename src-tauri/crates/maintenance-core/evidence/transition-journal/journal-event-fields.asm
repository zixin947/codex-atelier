
AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078ebd8 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E>:
10078fac0: 72a86369    	movk	w9, #0x431b, lsl #16
10078fac4: 5280002a    	mov	w10, #0x1               ; =1
10078fac8: 381913aa    	sturb	w10, [x29, #-0x6f]
10078facc: 381903bc    	sturb	w28, [x29, #-0x70]
10078fad0: a93453b3    	stp	x19, x20, [x29, #-0xc0]
10078fad4: a93557b3    	stp	x19, x21, [x29, #-0xb0]
10078fad8: 9ba97f29    	umull	x9, w25, w9
10078fadc: a93657ab    	stp	x11, x21, [x29, #-0xa0]
10078fae0: a9376bb6    	stp	x22, x26, [x29, #-0x90]
10078fae4: 8b49c908    	add	x8, x8, x9, lsr #50
10078fae8: a93823b6    	stp	x22, x8, [x29, #-0x80]
10078faec: 97e1c6a2    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10078faf0: 52801000    	mov	w0, #0x80               ; =128
10078faf4: 52800021    	mov	w1, #0x1                ; =1
10078faf8: 97e1c69b    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10078fafc: b40042e0    	cbz	x0, 0x100790358 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0x1780>
10078fb00: d10303b8    	sub	x24, x29, #0xc0
10078fb04: 52801008    	mov	w8, #0x80               ; =128
10078fb08: a90983e8    	stp	x8, x0, [sp, #0x98]
10078fb0c: 910263e8    	add	x8, sp, #0x98
10078fb10: f90037e8    	str	x8, [sp, #0x68]
10078fb14: 52800f68    	mov	w8, #0x7b               ; =123
10078fb18: 39000008    	strb	w8, [x0]
10078fb1c: 52800028    	mov	w8, #0x1                ; =1
10078fb20: f90057e8    	str	x8, [sp, #0xa8]
10078fb24: 52802008    	mov	w8, #0x100              ; =256
10078fb28: 790103e8    	strh	w8, [sp, #0x80]
10078fb2c: 9101a3e8    	add	x8, sp, #0x68
10078fb30: f90047e8    	str	x8, [sp, #0x88]
10078fb34: f00060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10078fb38: 91385021    	add	x1, x1, #0xe14
10078fb3c: 910203e0    	add	x0, sp, #0x80
10078fb40: 91014703    	add	x3, x24, #0x51
10078fb44: 528001a2    	mov	w2, #0xd                ; =13
10078fb48: 97f6455c    	bl	0x1005210b8 <__ZN10serde_core3ser12SerializeMap15serialize_entry17h822ff9448b5ccaa4E>
10078fb4c: aa0003f7    	mov	x23, x0
10078fb50: b50001c0    	cbnz	x0, 0x10078fb88 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xfb0>
10078fb54: 394203e8    	ldrb	w8, [sp, #0x80]
10078fb58: 36000088    	tbz	w8, #0x0, 0x10078fb68 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xf90>
10078fb5c: 9428eb03    	bl	0x1011ca768 <__ZN10serde_json3ser17invalid_raw_value17h6aa1aefc3e141c8fE>
10078fb60: aa0003f7    	mov	x23, x0
10078fb64: 14000009    	b	0x10078fb88 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xfb0>
10078fb68: f00060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10078fb6c: 913b5c21    	add	x1, x1, #0xed7
10078fb70: 910203e0    	add	x0, sp, #0x80
10078fb74: 91014303    	add	x3, x24, #0x50
10078fb78: 528000c2    	mov	w2, #0x6                ; =6
10078fb7c: 97f64af3    	bl	0x100522748 <__ZN10serde_core3ser12SerializeMap15serialize_entry17h8df270f28279a598E>
10078fb80: aa0003f7    	mov	x23, x0
10078fb84: b4000600    	cbz	x0, 0x10078fc44 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0x106c>
10078fb88: f9404fe1    	ldr	x1, [sp, #0x98]
10078fb8c: b4000081    	cbz	x1, 0x10078fb9c <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xfc4>
10078fb90: f94053e0    	ldr	x0, [sp, #0xa0]
10078fb94: 52800022    	mov	w2, #0x1                ; =1
10078fb98: 97e1c674    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078fb9c: f94002e8    	ldr	x8, [x23]
10078fba0: f100051f    	cmp	x8, #0x1
10078fba4: 54000100    	b.eq	0x10078fbc4 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xfec>
10078fba8: b5000128    	cbnz	x8, 0x10078fbcc <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xff4>
10078fbac: f9400ae1    	ldr	x1, [x23, #0x10]
10078fbb0: b40000e1    	cbz	x1, 0x10078fbcc <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xff4>
10078fbb4: f94006e0    	ldr	x0, [x23, #0x8]
10078fbb8: 52800022    	mov	w2, #0x1                ; =1
10078fbbc: 97e1c66b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078fbc0: 14000003    	b	0x10078fbcc <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xff4>
10078fbc4: 910022e0    	add	x0, x23, #0x8
10078fbc8: 940175ac    	bl	0x1007ed278 <__ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h8fc6649f423239fcE>
10078fbcc: aa1703e0    	mov	x0, x23
10078fbd0: 52800501    	mov	w1, #0x28               ; =40
10078fbd4: 52800102    	mov	w2, #0x8                ; =8
10078fbd8: 97e1c664    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078fbdc: b40000b3    	cbz	x19, 0x10078fbf0 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0x1018>
10078fbe0: aa1403e0    	mov	x0, x20
10078fbe4: aa1303e1    	mov	x1, x19
10078fbe8: 52800022    	mov	w2, #0x1                ; =1
10078fbec: 97e1c65f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078fbf0: d2f00013    	mov	x19, #-0x8000000000000000 ; =-9223372036854775808
10078fbf4: eb1302bf    	cmp	x21, x19
10078fbf8: 540000a0    	b.eq	0x10078fc0c <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0x1034>
10078fbfc: f94027e0    	ldr	x0, [sp, #0x48]
10078fc00: aa1503e1    	mov	x1, x21
10078fc04: 52800022    	mov	w2, #0x1                ; =1
10078fc08: 97e1c658    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078fc0c: eb1302df    	cmp	x22, x19
10078fc10: 540000a0    	b.eq	0x10078fc24 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0x104c>
10078fc14: aa1a03e0    	mov	x0, x26
10078fc18: aa1603e1    	mov	x1, x22
10078fc1c: 52800022    	mov	w2, #0x1                ; =1
10078fc20: 97e1c652    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10078fc24: a95a7bfd    	ldp	x29, x30, [sp, #0x1a0]
10078fc28: a9594ff4    	ldp	x20, x19, [sp, #0x190]
10078fc2c: a95857f6    	ldp	x22, x21, [sp, #0x180]
10078fc30: a9575ff8    	ldp	x24, x23, [sp, #0x170]
10078fc34: a95667fa    	ldp	x26, x25, [sp, #0x160]
10078fc38: a9556ffc    	ldp	x28, x27, [sp, #0x150]
10078fc3c: 9106c3ff    	add	sp, sp, #0x1b0
10078fc40: d65f03c0    	ret
10078fc44: 394203e8    	ldrb	w8, [sp, #0x80]
10078fc48: 3707f8a8    	tbnz	w8, #0x0, 0x10078fb5c <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xf84>
10078fc4c: f00060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10078fc50: 913b7421    	add	x1, x1, #0xedd
10078fc54: 910203e0    	add	x0, sp, #0x80
10078fc58: d10303a3    	sub	x3, x29, #0xc0
10078fc5c: 528000a2    	mov	w2, #0x5                ; =5
10078fc60: 97f655b7    	bl	0x10052533c <__ZN10serde_core3ser12SerializeMap15serialize_entry17hace858a778cac3daE>
10078fc64: aa0003f7    	mov	x23, x0
10078fc68: b5fff900    	cbnz	x0, 0x10078fb88 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xfb0>
10078fc6c: 394203e8    	ldrb	w8, [sp, #0x80]
10078fc70: 3707f768    	tbnz	w8, #0x0, 0x10078fb5c <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xf84>
10078fc74: f00060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10078fc78: 913b8821    	add	x1, x1, #0xee2
10078fc7c: 910203e0    	add	x0, sp, #0x80
10078fc80: 91012303    	add	x3, x24, #0x48
10078fc84: 52800182    	mov	w2, #0xc                ; =12
10078fc88: 97f61418    	bl	0x100514ce8 <__ZN10serde_core3ser12SerializeMap15serialize_entry17h13f8e651b5097d1cE>
10078fc8c: aa0003f7    	mov	x23, x0
10078fc90: b5fff7c0    	cbnz	x0, 0x10078fb88 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xfb0>
10078fc94: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078fc98: eb0802bf    	cmp	x21, x8
10078fc9c: 54001b01    	b.ne	0x10078fffc <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0x1424>
10078fca0: 394203e8    	ldrb	w8, [sp, #0x80]
10078fca4: 3707f5c8    	tbnz	w8, #0x0, 0x10078fb5c <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xf84>
10078fca8: f00060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10078fcac: 913bb821    	add	x1, x1, #0xeee
10078fcb0: 910203e0    	add	x0, sp, #0x80
10078fcb4: 9100c303    	add	x3, x24, #0x30
10078fcb8: 528000c2    	mov	w2, #0x6                ; =6
10078fcbc: 97f66f48    	bl	0x10052b9dc <__ZN10serde_core3ser12SerializeMap15serialize_entry17hf92b925f2096e89dE>
10078fcc0: aa0003f7    	mov	x23, x0
10078fcc4: b5fff620    	cbnz	x0, 0x10078fb88 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xfb0>
10078fcc8: 394203e8    	ldrb	w8, [sp, #0x80]
10078fccc: 37000128    	tbnz	w8, #0x0, 0x10078fcf0 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0x1118>
10078fcd0: 394207e8    	ldrb	w8, [sp, #0x81]
10078fcd4: 340000e8    	cbz	w8, 0x10078fcf0 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0x1118>
10078fcd8: f94047e8    	ldr	x8, [sp, #0x88]
10078fcdc: f9400100    	ldr	x0, [x8]
10078fce0: d00060c1    	adrp	x1, 0x1013a9000 <_anon.cdb272158a04aae91ce02c6e47f106c7.539+0xd4>
10078fce4: 910a2021    	add	x1, x1, #0x288
10078fce8: 52800022    	mov	w2, #0x1                ; =1
10078fcec: 97f733f8    	bl	0x10055cccc <__ZN5alloc3vec16Vec$LT$T$C$A$GT$17extend_from_slice17h3b7683e8d5b760adE>
10078fcf0: a949dfe9    	ldp	x9, x23, [sp, #0x98]
10078fcf4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078fcf8: f90023e9    	str	x9, [sp, #0x40]
10078fcfc: eb08013f    	cmp	x9, x8
10078fd00: 54fff4e0    	b.eq	0x10078fb9c <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xfc4>
10078fd04: f94057f9    	ldr	x25, [sp, #0xa8]
10078fd08: f00060c2    	adrp	x2, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10078fd0c: 91399842    	add	x2, x2, #0xe66
10078fd10: 910143e8    	add	x8, sp, #0x50
10078fd14: a94207e0    	ldp	x0, x1, [sp, #0x20]
10078fd18: 52800263    	mov	w3, #0x13               ; =19
10078fd1c: 9426fd51    	bl	0x10114f260 <__ZN3std4path4Path5_join17h46a64ed49ec529e9E>
