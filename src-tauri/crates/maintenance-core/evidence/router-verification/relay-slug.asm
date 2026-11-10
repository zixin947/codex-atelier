
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

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
