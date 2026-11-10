
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078ae60 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E>:
10078ae60: d102c3ff    	sub	sp, sp, #0xb0
10078ae64: a9075ff8    	stp	x24, x23, [sp, #0x70]
10078ae68: a90857f6    	stp	x22, x21, [sp, #0x80]
10078ae6c: a9094ff4    	stp	x20, x19, [sp, #0x90]
10078ae70: a90a7bfd    	stp	x29, x30, [sp, #0xa0]
10078ae74: 910283fd    	add	x29, sp, #0xa0
10078ae78: b4000082    	cbz	x2, 0x10078ae88 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x28>
10078ae7c: 39400028    	ldrb	w8, [x1]
10078ae80: 71008d1f    	cmp	w8, #0x23
10078ae84: 54000121    	b.ne	0x10078aea8 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x48>
10078ae88: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078ae8c: f9000008    	str	x8, [x0]
10078ae90: a94a7bfd    	ldp	x29, x30, [sp, #0xa0]
10078ae94: a9494ff4    	ldp	x20, x19, [sp, #0x90]
10078ae98: a94857f6    	ldp	x22, x21, [sp, #0x80]
10078ae9c: a9475ff8    	ldp	x24, x23, [sp, #0x70]
10078aea0: 9102c3ff    	add	sp, sp, #0xb0
10078aea4: d65f03c0    	ret
10078aea8: f1003c5f    	cmp	x2, #0xf
10078aeac: 54000128    	b.hi	0x10078aed0 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x70>
10078aeb0: aa0103e8    	mov	x8, x1
10078aeb4: aa0203e9    	mov	x9, x2
10078aeb8: 3840150a    	ldrb	w10, [x8], #0x1
10078aebc: 7100f55f    	cmp	w10, #0x3d
10078aec0: 540001e0    	b.eq	0x10078aefc <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x9c>
10078aec4: f1000529    	subs	x9, x9, #0x1
10078aec8: 54ffff81    	b.ne	0x10078aeb8 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x58>
10078aecc: 17ffffef    	b	0x10078ae88 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x28>
10078aed0: aa0003f5    	mov	x21, x0
10078aed4: 528007a0    	mov	w0, #0x3d               ; =61
10078aed8: aa0103f3    	mov	x19, x1
10078aedc: aa0203f4    	mov	x20, x2
10078aee0: 9427ee02    	bl	0x1011866e8 <__ZN4core5slice6memchr14memchr_aligned17h58876bc074a374c5E>
10078aee4: aa1403e2    	mov	x2, x20
10078aee8: aa1303e1    	mov	x1, x19
10078aeec: aa0003e8    	mov	x8, x0
10078aef0: aa1503e0    	mov	x0, x21
10078aef4: f100051f    	cmp	x8, #0x1
10078aef8: 54fffc81    	b.ne	0x10078ae88 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x28>
10078aefc: aa0003f7    	mov	x23, x0
10078af00: 910003e8    	mov	x8, sp
10078af04: a90107e2    	stp	x2, x1, [sp, #0x10]
10078af08: aa0103f3    	mov	x19, x1
10078af0c: a9027fe2    	stp	x2, xzr, [sp, #0x20]
10078af10: f9001be2    	str	x2, [sp, #0x30]
10078af14: 0f0107a0    	movi.2s	v0, #0x3d
10078af18: fd001fe0    	str	d0, [sp, #0x38]
10078af1c: 52800029    	mov	w9, #0x1                ; =1
10078af20: 390103e9    	strb	w9, [sp, #0x40]
10078af24: 790093e9    	strh	w9, [sp, #0x48]
10078af28: d0005bc9    	adrp	x9, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
10078af2c: 3dc04920    	ldr	q0, [x9, #0x120]
10078af30: 3d8003e0    	str	q0, [sp]
10078af34: d10123a0    	sub	x0, x29, #0x48
10078af38: 91006101    	add	x1, x8, #0x18
10078af3c: 9401d014    	bl	0x1007fef8c <__ZN81_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$10next_match17h057e0822e43a1174E>
10078af40: b85b83a8    	ldur	w8, [x29, #-0x48]
10078af44: 7100051f    	cmp	w8, #0x1
10078af48: 54000761    	b.ne	0x10078b034 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x1d4>
10078af4c: f85c03a8    	ldur	x8, [x29, #-0x40]
10078af50: f94007e9    	ldr	x9, [sp, #0x8]
10078af54: cb090101    	sub	x1, x8, x9
10078af58: 8b090260    	add	x0, x19, x9
10078af5c: 97ea907f    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
10078af60: aa1703e8    	mov	x8, x23
10078af64: b4000921    	cbz	x1, 0x10078b088 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x228>
10078af68: aa0103f3    	mov	x19, x1
10078af6c: 8b010009    	add	x9, x0, x1
10078af70: aa0003ea    	mov	x10, x0
10078af74: eb09015f    	cmp	x10, x9
10078af78: 54000720    	b.eq	0x10078b05c <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x1fc>
10078af7c: aa0a03ec    	mov	x12, x10
10078af80: 38c0158d    	ldrsb	w13, [x12], #0x1
10078af84: 12001dab    	and	w11, w13, #0xff
10078af88: 37f800ad    	tbnz	w13, #0x1f, 0x10078af9c <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x13c>
10078af8c: aa0c03ea    	mov	x10, x12
10078af90: 7100b57f    	cmp	w11, #0x2d
10078af94: 54000401    	b.ne	0x10078b014 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x1b4>
10078af98: 17fffff7    	b	0x10078af74 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x114>
10078af9c: 3940054c    	ldrb	w12, [x10, #0x1]
10078afa0: 1200158d    	and	w13, w12, #0x3f
10078afa4: 7103817f    	cmp	w11, #0xe0
10078afa8: 54000223    	b.lo	0x10078afec <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x18c>
10078afac: 1200116c    	and	w12, w11, #0x1f
10078afb0: 3940094e    	ldrb	w14, [x10, #0x2]
10078afb4: 120015ce    	and	w14, w14, #0x3f
10078afb8: 2a0d19cd    	orr	w13, w14, w13, lsl #6
10078afbc: 7103c17f    	cmp	w11, #0xf0
10078afc0: 54000223    	b.lo	0x10078b004 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x1a4>
10078afc4: 39400d4b    	ldrb	w11, [x10, #0x3]
10078afc8: 1200156b    	and	w11, w11, #0x3f
10078afcc: 2a0d196b    	orr	w11, w11, w13, lsl #6
10078afd0: 330e098b    	bfi	w11, w12, #18, #3
10078afd4: 7144417f    	cmp	w11, #0x110, lsl #12    ; =0x110000
10078afd8: 54000420    	b.eq	0x10078b05c <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x1fc>
10078afdc: 9100114a    	add	x10, x10, #0x4
10078afe0: 7100b57f    	cmp	w11, #0x2d
10078afe4: 54000181    	b.ne	0x10078b014 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x1b4>
10078afe8: 17ffffe3    	b	0x10078af74 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x114>
10078afec: 9100094a    	add	x10, x10, #0x2
10078aff0: 331a116d    	bfi	w13, w11, #6, #5
10078aff4: aa0d03eb    	mov	x11, x13
10078aff8: 7100b57f    	cmp	w11, #0x2d
10078affc: 540000c1    	b.ne	0x10078b014 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x1b4>
10078b000: 17ffffdd    	b	0x10078af74 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x114>
10078b004: 91000d4a    	add	x10, x10, #0x3
10078b008: 2a0c31ab    	orr	w11, w13, w12, lsl #12
10078b00c: 7100b57f    	cmp	w11, #0x2d
10078b010: 54fffb20    	b.eq	0x10078af74 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x114>
10078b014: 5100e96c    	sub	w12, w11, #0x3a
10078b018: 121a796d    	and	w13, w11, #0xffffffdf
10078b01c: 51016dad    	sub	w13, w13, #0x5b
10078b020: 71017d7f    	cmp	w11, #0x5f
10078b024: 3a4b1982    	ccmn	w12, #0xb, #0x2, ne
10078b028: 3a5a99a2    	ccmn	w13, #0x1a, #0x2, ls
10078b02c: 54fffa42    	b.hs	0x10078af74 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x114>
10078b030: 14000016    	b	0x10078b088 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x228>
10078b034: 394127e8    	ldrb	w8, [sp, #0x49]
10078b038: 370000c8    	tbnz	w8, #0x0, 0x10078b050 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x1f0>
10078b03c: 394123ea    	ldrb	w10, [sp, #0x48]
10078b040: a940a7e8    	ldp	x8, x9, [sp, #0x8]
10078b044: 3700016a    	tbnz	w10, #0x0, 0x10078b070 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x210>
10078b048: eb08013f    	cmp	x9, x8
10078b04c: 54000121    	b.ne	0x10078b070 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x210>
10078b050: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10078b054: f90002e8    	str	x8, [x23]
10078b058: 17ffff8e    	b	0x10078ae90 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x30>
10078b05c: b6f801d3    	tbz	x19, #0x3f, 0x10078b094 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x234>
10078b060: d2800015    	mov	x21, #0x0               ; =0
10078b064: aa1503e0    	mov	x0, x21
10078b068: aa1303e1    	mov	x1, x19
10078b06c: 9429157e    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10078b070: f9400fea    	ldr	x10, [sp, #0x18]
10078b074: cb080121    	sub	x1, x9, x8
10078b078: 8b080140    	add	x0, x10, x8
10078b07c: 97ea9037    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
10078b080: aa1703e8    	mov	x8, x23
10078b084: b5fff721    	cbnz	x1, 0x10078af68 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x108>
10078b088: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10078b08c: f9000109    	str	x9, [x8]
10078b090: 17ffff80    	b	0x10078ae90 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x30>
10078b094: aa0003f4    	mov	x20, x0
10078b098: 97e1d937    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10078b09c: 52800035    	mov	w21, #0x1               ; =1
10078b0a0: aa1303e0    	mov	x0, x19
10078b0a4: 52800021    	mov	w1, #0x1                ; =1
10078b0a8: 97e1d92f    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10078b0ac: b4fffdc0    	cbz	x0, 0x10078b064 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x204>
10078b0b0: aa0003f6    	mov	x22, x0
10078b0b4: aa1403e1    	mov	x1, x20
10078b0b8: aa1303e2    	mov	x2, x19
10078b0bc: 9429197c    	bl	0x1011d16ac <_writev+0x1011d16ac>
10078b0c0: a9005af3    	stp	x19, x22, [x23]
10078b0c4: f9000af3    	str	x19, [x23, #0x10]
10078b0c8: 17ffff72    	b	0x10078ae90 <__ZN13codexmate_lib4core5relay12codex_writer24top_level_assignment_key17ha36acef3dd8c28e5E+0x30>
