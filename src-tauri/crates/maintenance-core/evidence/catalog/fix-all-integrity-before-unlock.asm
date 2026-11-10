
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001003a69d8 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E>:
1003a6d38: d0007b36    	adrp	x22, 0x10130c000 <_anon.164d03a2dfc98f969363eaf1c50504fd.135+0x4d6>
1003a6d3c: 913272d6    	add	x22, x22, #0xc9c
1003a6d40: 910803e8    	add	x8, sp, #0x200
1003a6d44: a94383e3    	ldp	x3, x0, [sp, #0x38]
1003a6d48: aa1603e1    	mov	x1, x22
1003a6d4c: 528001c2    	mov	w2, #0xe                ; =14
1003a6d50: f9401be4    	ldr	x4, [sp, #0x30]
1003a6d54: 9403e10d    	bl	0x10049f188 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit11RepairAudit3fix17h09ed44c66da3de57E>
1003a6d58: f94103e8    	ldr	x8, [sp, #0x200]
1003a6d5c: f1002d1f    	cmp	x8, #0xb
1003a6d60: 540013a1    	b.ne	0x1003a6fd4 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x5fc>
1003a6d64: 3cc08380    	ldur	q0, [x28, #0x8]
1003a6d68: 3cc18381    	ldur	q1, [x28, #0x18]
1003a6d6c: ad030780    	stp	q0, q1, [x28, #0x60]
1003a6d70: 3cc28380    	ldur	q0, [x28, #0x28]
1003a6d74: 3cc38381    	ldur	q1, [x28, #0x38]
1003a6d78: ad040780    	stp	q0, q1, [x28, #0x80]
1003a6d7c: f94127e8    	ldr	x8, [sp, #0x248]
1003a6d80: f90153e8    	str	x8, [sp, #0x2a0]
1003a6d84: f94137f6    	ldr	x22, [sp, #0x268]
1003a6d88: f94133f3    	ldr	x19, [sp, #0x260]
1003a6d8c: f9413bf7    	ldr	x23, [sp, #0x270]
1003a6d90: 8b1706e8    	add	x8, x23, x23, lsl #1
1003a6d94: d37df11a    	lsl	x26, x8, #3
1003a6d98: 8b1a02c8    	add	x8, x22, x26
1003a6d9c: a9365bb6    	stp	x22, x22, [x29, #-0xa0]
1003a6da0: a93723b3    	stp	x19, x8, [x29, #-0x90]
1003a6da4: f9405bf5    	ldr	x21, [sp, #0xb0]
1003a6da8: f94053e8    	ldr	x8, [sp, #0xa0]
1003a6dac: cb150108    	sub	x8, x8, x21
1003a6db0: eb0802ff    	cmp	x23, x8
1003a6db4: 54017a28    	b.hi	0x1003a9cf8 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x3320>
1003a6db8: f94057e8    	ldr	x8, [sp, #0xa8]
1003a6dbc: 52800309    	mov	w9, #0x18               ; =24
1003a6dc0: 9b0922a0    	madd	x0, x21, x9, x8
1003a6dc4: aa1603e1    	mov	x1, x22
1003a6dc8: aa1a03e2    	mov	x2, x26
1003a6dcc: 9438aa38    	bl	0x1011d16ac <_writev+0x1011d16ac>
1003a6dd0: 8b1702a8    	add	x8, x21, x23
1003a6dd4: f9005be8    	str	x8, [sp, #0xb0]
1003a6dd8: b40000d3    	cbz	x19, 0x1003a6df0 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x418>
1003a6ddc: 8b130668    	add	x8, x19, x19, lsl #1
1003a6de0: d37df101    	lsl	x1, x8, #3
1003a6de4: aa1603e0    	mov	x0, x22
1003a6de8: 52800102    	mov	w2, #0x8                ; =8
1003a6dec: 97f169df    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003a6df0: f9414ff6    	ldr	x22, [sp, #0x298]
1003a6df4: f9414bf3    	ldr	x19, [sp, #0x290]
1003a6df8: f94153f7    	ldr	x23, [sp, #0x2a0]
1003a6dfc: 8b1706e8    	add	x8, x23, x23, lsl #1
1003a6e00: d37df11a    	lsl	x26, x8, #3
1003a6e04: 8b1a02c8    	add	x8, x22, x26
1003a6e08: a9365bb6    	stp	x22, x22, [x29, #-0xa0]
1003a6e0c: a93723b3    	stp	x19, x8, [x29, #-0x90]
1003a6e10: f94073f5    	ldr	x21, [sp, #0xe0]
1003a6e14: f9406be8    	ldr	x8, [sp, #0xd0]
1003a6e18: cb150108    	sub	x8, x8, x21
1003a6e1c: eb0802ff    	cmp	x23, x8
1003a6e20: 540177c8    	b.hi	0x1003a9d18 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x3340>
1003a6e24: f9406fe8    	ldr	x8, [sp, #0xd8]
1003a6e28: 52800309    	mov	w9, #0x18               ; =24
1003a6e2c: 9b0922a0    	madd	x0, x21, x9, x8
1003a6e30: aa1603e1    	mov	x1, x22
1003a6e34: aa1a03e2    	mov	x2, x26
1003a6e38: 9438aa1d    	bl	0x1011d16ac <_writev+0x1011d16ac>
1003a6e3c: 8b1702a8    	add	x8, x21, x23
1003a6e40: f90073e8    	str	x8, [sp, #0xe0]
1003a6e44: b40000d3    	cbz	x19, 0x1003a6e5c <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x484>
1003a6e48: 8b130668    	add	x8, x19, x19, lsl #1
1003a6e4c: d37df101    	lsl	x1, x8, #3
1003a6e50: aa1603e0    	mov	x0, x22
1003a6e54: 52800102    	mov	w2, #0x8                ; =8
1003a6e58: 97f169c4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003a6e5c: f94143f6    	ldr	x22, [sp, #0x280]
1003a6e60: f94147f3    	ldr	x19, [sp, #0x288]
1003a6e64: b4001313    	cbz	x19, 0x1003a70c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x6ec>
1003a6e68: 910022d4    	add	x20, x22, #0x8
1003a6e6c: 14000004    	b	0x1003a6e7c <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x4a4>
1003a6e70: 91006294    	add	x20, x20, #0x18
1003a6e74: f1000673    	subs	x19, x19, #0x1
1003a6e78: 54001260    	b.eq	0x1003a70c4 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x6ec>
1003a6e7c: f85f8281    	ldur	x1, [x20, #-0x8]
1003a6e80: b4ffff81    	cbz	x1, 0x1003a6e70 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x498>
1003a6e84: f9400280    	ldr	x0, [x20]
1003a6e88: 52800022    	mov	w2, #0x1                ; =1
1003a6e8c: 97f169b7    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003a6e90: 17fffff8    	b	0x1003a6e70 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x498>
1003a6e94: 97f169b8    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003a6e98: 52800760    	mov	w0, #0x3b               ; =59
1003a6e9c: 52800021    	mov	w1, #0x1                ; =1
1003a6ea0: 97f169b1    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003a6ea4: b4017fa0    	cbz	x0, 0x1003a9e98 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x34c0>
1003a6ea8: d0007b48    	adrp	x8, 0x101310000 <_anon.164d03a2dfc98f969363eaf1c50504fd.774+0xeea>
1003a6eac: 91212908    	add	x8, x8, #0x84a
1003a6eb0: ad400500    	ldp	q0, q1, [x8]
1003a6eb4: ad000400    	stp	q0, q1, [x0]
1003a6eb8: 3dc00900    	ldr	q0, [x8, #0x20]
1003a6ebc: 3d800800    	str	q0, [x0, #0x20]
1003a6ec0: 3cc2b100    	ldur	q0, [x8, #0x2b]
1003a6ec4: 3c82b000    	stur	q0, [x0, #0x2b]
1003a6ec8: b0007b28    	adrp	x8, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003a6ecc: 3dc0b500    	ldr	q0, [x8, #0x2d0]
1003a6ed0: f9400be9    	ldr	x9, [sp, #0x10]
1003a6ed4: 3d800120    	str	q0, [x9]
1003a6ed8: 52800768    	mov	w8, #0x3b               ; =59
1003a6edc: a9012120    	stp	x0, x8, [x9, #0x10]
1003a6ee0: f9403ff3    	ldr	x19, [sp, #0x78]
1003a6ee4: b4000553    	cbz	x19, 0x1003a6f8c <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x5b4>
1003a6ee8: f94047f4    	ldr	x20, [sp, #0x88]
1003a6eec: b40003d4    	cbz	x20, 0x1003a6f64 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x58c>
1003a6ef0: f9403bf5    	ldr	x21, [sp, #0x70]
1003a6ef4: aa1503f6    	mov	x22, x21
1003a6ef8: fc4086c0    	ldr	d0, [x22], #0x8
1003a6efc: 2e208800    	cmge.8b	v0, v0, #0
1003a6f00: 9e660008    	fmov	x8, d0
1003a6f04: 9201c118    	and	x24, x8, #0x8080808080808080
1003a6f08: 52800317    	mov	w23, #0x18              ; =24
1003a6f0c: 14000005    	b	0x1003a6f20 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x548>
1003a6f10: d1000708    	sub	x8, x24, #0x1
1003a6f14: 8a180118    	and	x24, x8, x24
1003a6f18: d1000694    	sub	x20, x20, #0x1
1003a6f1c: b4000254    	cbz	x20, 0x1003a6f64 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x58c>
1003a6f20: b50000f8    	cbnz	x24, 0x1003a6f3c <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x564>
1003a6f24: fc4086c0    	ldr	d0, [x22], #0x8
1003a6f28: 2e208800    	cmge.8b	v0, v0, #0
1003a6f2c: 9e660008    	fmov	x8, d0
1003a6f30: d10302b5    	sub	x21, x21, #0xc0
1003a6f34: 9201c118    	and	x24, x8, #0x8080808080808080
1003a6f38: b4ffff78    	cbz	x24, 0x1003a6f24 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x54c>
1003a6f3c: dac00308    	rbit	x8, x24
1003a6f40: dac01108    	clz	x8, x8
1003a6f44: cb480fe8    	neg	x8, x8, lsr #3
1003a6f48: 9b375508    	smaddl	x8, w8, w23, x21
1003a6f4c: f85e8101    	ldur	x1, [x8, #-0x18]
1003a6f50: b4fffe01    	cbz	x1, 0x1003a6f10 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x538>
1003a6f54: f85f0100    	ldur	x0, [x8, #-0x10]
1003a6f58: 52800022    	mov	w2, #0x1                ; =1
1003a6f5c: 97f16983    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003a6f60: 17ffffec    	b	0x1003a6f10 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x538>
1003a6f64: 8b130668    	add	x8, x19, x19, lsl #1
1003a6f68: d37df108    	lsl	x8, x8, #3
1003a6f6c: 8b130109    	add	x9, x8, x19
1003a6f70: b1008521    	adds	x1, x9, #0x21
1003a6f74: 540000c0    	b.eq	0x1003a6f8c <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x5b4>
1003a6f78: f9403be9    	ldr	x9, [sp, #0x70]
1003a6f7c: cb080128    	sub	x8, x9, x8
1003a6f80: d1006100    	sub	x0, x8, #0x18
1003a6f84: 52800102    	mov	w2, #0x8                ; =8
1003a6f88: 97f16978    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003a6f8c: 910143e0    	add	x0, sp, #0x50
1003a6f90: 9406f38d    	bl	0x100563dc4 <__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4d255dfcf056ce13E>
1003a6f94: f9402be8    	ldr	x8, [sp, #0x50]
1003a6f98: b40000c8    	cbz	x8, 0x1003a6fb0 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x5d8>
1003a6f9c: f9402fe0    	ldr	x0, [sp, #0x58]
1003a6fa0: 52800d09    	mov	w9, #0x68               ; =104
1003a6fa4: 9b097d01    	mul	x1, x8, x9
1003a6fa8: 52800102    	mov	w2, #0x8                ; =8
1003a6fac: 97f1696f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003a6fb0: 910c83ff    	add	sp, sp, #0x320
1003a6fb4: a9467bfd    	ldp	x29, x30, [sp, #0x60]
1003a6fb8: a9454ff4    	ldp	x20, x19, [sp, #0x50]
1003a6fbc: a94457f6    	ldp	x22, x21, [sp, #0x40]
1003a6fc0: a9435ff8    	ldp	x24, x23, [sp, #0x30]
1003a6fc4: a94267fa    	ldp	x26, x25, [sp, #0x20]
1003a6fc8: a9416ffc    	ldp	x28, x27, [sp, #0x10]
1003a6fcc: 6cc723e9    	ldp	d9, d8, [sp], #0x70
1003a6fd0: d65f03c0    	ret
1003a6fd4: 910343e8    	add	x8, sp, #0xd0
1003a6fd8: ad4a8500    	ldp	q0, q1, [x8, #0x150]
1003a6fdc: ad040780    	stp	q0, q1, [x28, #0x80]
1003a6fe0: ad4b8500    	ldp	q0, q1, [x8, #0x170]
1003a6fe4: ad050780    	stp	q0, q1, [x28, #0xa0]
1003a6fe8: ad498500    	ldp	q0, q1, [x8, #0x130]
1003a6fec: ad030780    	stp	q0, q1, [x28, #0x60]
1003a6ff0: 97f16961    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003a6ff4: 528001c0    	mov	w0, #0xe                ; =14
1003a6ff8: 52800021    	mov	w1, #0x1                ; =1
1003a6ffc: 97f1695a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003a7000: b4017540    	cbz	x0, 0x1003a9ea8 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x34d0>
1003a7004: aa0003f5    	mov	x21, x0
1003a7008: f94002c8    	ldr	x8, [x22]
1003a700c: f9000008    	str	x8, [x0]
1003a7010: f84062c8    	ldur	x8, [x22, #0x6]
1003a7014: f8006008    	stur	x8, [x0, #0x6]
1003a7018: f94067f3    	ldr	x19, [sp, #0xc8]
1003a701c: f9405fe8    	ldr	x8, [sp, #0xb8]
1003a7020: eb08027f    	cmp	x19, x8
1003a7024: 54000061    	b.ne	0x1003a7030 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x658>
1003a7028: 9102e3e0    	add	x0, sp, #0xb8
1003a702c: 9438a375    	bl	0x1011cfe00 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hca26f846e3ace0fdE>
1003a7030: f94063e8    	ldr	x8, [sp, #0xc0]
1003a7034: 52800309    	mov	w9, #0x18               ; =24
1003a7038: 9b092268    	madd	x8, x19, x9, x8
1003a703c: 528001c9    	mov	w9, #0xe                ; =14
1003a7040: a9005509    	stp	x9, x21, [x8]
1003a7044: f9000909    	str	x9, [x8, #0x10]
1003a7048: 91000668    	add	x8, x19, #0x1
1003a704c: f90067e8    	str	x8, [sp, #0xc8]
1003a7050: 910983e8    	add	x8, sp, #0x260
1003a7054: f81603a8    	stur	x8, [x29, #-0xa0]
1003a7058: b00022a8    	adrp	x8, 0x1007fc000 <__ZN6chrono8datetime40DateTime$LT$chrono..offset..utc..Utc$GT$14from_timestamp17h2b34124b50daed04E+0x8>
1003a705c: 91343108    	add	x8, x8, #0xd0c
1003a7060: f81683a8    	stur	x8, [x29, #-0x98]
1003a7064: f0008bc0    	adrp	x0, 0x101522000 <_anon.164d03a2dfc98f969363eaf1c50504fd.794+0x931>
1003a7068: 912de400    	add	x0, x0, #0xb79
1003a706c: 910403e8    	add	x8, sp, #0x100
1003a7070: d10283a1    	sub	x1, x29, #0xa0
1003a7074: 94374208    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
1003a7078: f94073f3    	ldr	x19, [sp, #0xe0]
1003a707c: f9406be8    	ldr	x8, [sp, #0xd0]
1003a7080: eb08027f    	cmp	x19, x8
1003a7084: 54000061    	b.ne	0x1003a7090 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x6b8>
1003a7088: 910343e0    	add	x0, sp, #0xd0
1003a708c: 9438a35d    	bl	0x1011cfe00 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17hca26f846e3ace0fdE>
1003a7090: f9406fe8    	ldr	x8, [sp, #0xd8]
1003a7094: 52800309    	mov	w9, #0x18               ; =24
1003a7098: 9b092268    	madd	x8, x19, x9, x8
1003a709c: 910343e9    	add	x9, sp, #0xd0
1003a70a0: 3dc00d20    	ldr	q0, [x9, #0x30]
1003a70a4: 3d800100    	str	q0, [x8]
1003a70a8: f9408be9    	ldr	x9, [sp, #0x110]
1003a70ac: f9000909    	str	x9, [x8, #0x10]
1003a70b0: 91000668    	add	x8, x19, #0x1
1003a70b4: f90073e8    	str	x8, [sp, #0xe0]
1003a70b8: 910983e0    	add	x0, sp, #0x260
1003a70bc: 9400c33d    	bl	0x1003d7db0 <__ZN4core3ptr59drop_in_place$LT$codexmate_lib..core..models..CoreError$GT$17h90a3dad92fa7663dE>
1003a70c0: 14000008    	b	0x1003a70e0 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x708>
1003a70c4: f9413fe8    	ldr	x8, [sp, #0x278]
1003a70c8: b40000c8    	cbz	x8, 0x1003a70e0 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x708>
1003a70cc: 8b080508    	add	x8, x8, x8, lsl #1
1003a70d0: d37df101    	lsl	x1, x8, #3
1003a70d4: aa1603e0    	mov	x0, x22
1003a70d8: 52800102    	mov	w2, #0x8                ; =8
1003a70dc: 97f16923    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1003a70e0: f94047e8    	ldr	x8, [sp, #0x88]
1003a70e4: b4013308    	cbz	x8, 0x1003a9744 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x2d6c>
1003a70e8: f0007b21    	adrp	x1, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
1003a70ec: 9100ac21    	add	x1, x1, #0x2b
1003a70f0: 9101c3e8    	add	x8, sp, #0x70
1003a70f4: 91008100    	add	x0, x8, #0x20
1003a70f8: 52800342    	mov	w2, #0x1a               ; =26
1003a70fc: 940c77af    	bl	0x1006c4fb8 <__ZN4core4hash11BuildHasher8hash_one17h61e5c885b8041446E>
1003a7100: d2800008    	mov	x8, #0x0                ; =0
1003a7104: d379fc0b    	lsr	x11, x0, #57
1003a7108: a94727ea    	ldp	x10, x9, [sp, #0x70]
1003a710c: 0e010d60    	dup.8b	v0, w11
1003a7110: d100614b    	sub	x11, x10, #0x18
1003a7114: 5280030c    	mov	w12, #0x18              ; =24
1003a7118: 6f07e7e1    	movi.2d	v1, #0xffffffffffffffff
1003a711c: d28dee4d    	mov	x13, #0x6f72            ; =28530
1003a7120: f2ae8ead    	movk	x13, #0x7475, lsl #16
1003a7124: f2ce4cad    	movk	x13, #0x7265, lsl #32
1003a7128: f2eeabed    	movk	x13, #0x755f, lsl #48
1003a712c: d28d8dce    	mov	x14, #0x6c6e            ; =27758
1003a7130: f2ac6dee    	movk	x14, #0x636f, lsl #16
1003a7134: f2cbed6e    	movk	x14, #0x5f6b, lsl #32
1003a7138: f2eeac2e    	movk	x14, #0x7561, lsl #48
1003a713c: d28d0e8f    	mov	x15, #0x6874            ; =26740
1003a7140: f2ae4bef    	movk	x15, #0x725f, lsl #16
1003a7144: f2ce6caf    	movk	x15, #0x7365, lsl #32
1003a7148: f2ec8d2f    	movk	x15, #0x6469, lsl #48
1003a714c: 528caeb0    	mov	w16, #0x6575            ; =25973
1003a7150: 8a090011    	and	x17, x0, x9
1003a7154: fc716942    	ldr	d2, [x10, x17]
1003a7158: 2e208c43    	cmeq.8b	v3, v2, v0
1003a715c: 9e660060    	fmov	x0, d3
1003a7160: f201c000    	ands	x0, x0, #0x8080808080808080
1003a7164: 540002c0    	b.eq	0x1003a71bc <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x7e4>
1003a7168: dac00001    	rbit	x1, x0
1003a716c: dac01021    	clz	x1, x1
1003a7170: 8b410e21    	add	x1, x17, x1, lsr #3
1003a7174: 8a090021    	and	x1, x1, x9
1003a7178: 9b0cfc21    	mneg	x1, x1, x12
1003a717c: 8b010161    	add	x1, x11, x1
1003a7180: f9400822    	ldr	x2, [x1, #0x10]
1003a7184: f100685f    	cmp	x2, #0x1a
1003a7188: 54000141    	b.ne	0x1003a71b0 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x7d8>
1003a718c: f9400421    	ldr	x1, [x1, #0x8]
1003a7190: a9400c22    	ldp	x2, x3, [x1]
1003a7194: f9400824    	ldr	x4, [x1, #0x10]
1003a7198: 79403021    	ldrh	w1, [x1, #0x18]
1003a719c: eb0d005f    	cmp	x2, x13
1003a71a0: fa4e0060    	ccmp	x3, x14, #0x0, eq
1003a71a4: fa4f0080    	ccmp	x4, x15, #0x0, eq
1003a71a8: 7a500020    	ccmp	w1, w16, #0x0, eq
1003a71ac: 54000220    	b.eq	0x1003a71f0 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x818>
1003a71b0: d1000401    	sub	x1, x0, #0x1
1003a71b4: ea000020    	ands	x0, x1, x0
1003a71b8: 54fffd81    	b.ne	0x1003a7168 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x790>
1003a71bc: 2e218c42    	cmeq.8b	v2, v2, v1
1003a71c0: 2e30a842    	umaxv.8b	b2, v2
1003a71c4: 1e260040    	fmov	w0, s2
1003a71c8: 37001560    	tbnz	w0, #0x0, 0x1003a7474 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0xa9c>
1003a71cc: 91002108    	add	x8, x8, #0x8
1003a71d0: 8b080220    	add	x0, x17, x8
1003a71d4: 8a090011    	and	x17, x0, x9
1003a71d8: fc716942    	ldr	d2, [x10, x17]
1003a71dc: 2e208c43    	cmeq.8b	v3, v2, v0
1003a71e0: 9e660060    	fmov	x0, d3
1003a71e4: f201c000    	ands	x0, x0, #0x8080808080808080
1003a71e8: 54fffc01    	b.ne	0x1003a7168 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x790>
1003a71ec: 17fffff4    	b	0x1003a71bc <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x7e4>
1003a71f0: f0007b36    	adrp	x22, 0x10130e000 <_anon.164d03a2dfc98f969363eaf1c50504fd.630+0x2>
1003a71f4: 9100aed6    	add	x22, x22, #0x2b
1003a71f8: 910803e8    	add	x8, sp, #0x200
1003a71fc: a94383e3    	ldp	x3, x0, [sp, #0x38]
1003a7200: aa1603e1    	mov	x1, x22
1003a7204: 52800342    	mov	w2, #0x1a               ; =26
1003a7208: f9401be4    	ldr	x4, [sp, #0x30]
1003a720c: 9403e336    	bl	0x10049fee4 <__ZN13codexmate_lib4core5relay16codex_diagnostic5audit11RepairAudit3fix17h804d2ec3928990d7E>
1003a7210: f94103e8    	ldr	x8, [sp, #0x200]
1003a7214: f1002d1f    	cmp	x8, #0xb
1003a7218: 54000b61    	b.ne	0x1003a7384 <__ZN13codexmate_lib4core5relay16codex_diagnostic7fix_all17h86751db120e25625E+0x9ac>
1003a721c: 3cc08380    	ldur	q0, [x28, #0x8]
1003a7220: 3cc18381    	ldur	q1, [x28, #0x18]
1003a7224: ad030780    	stp	q0, q1, [x28, #0x60]
1003a7228: 3cc28380    	ldur	q0, [x28, #0x28]
1003a722c: 3cc38381    	ldur	q1, [x28, #0x38]
1003a7230: ad040780    	stp	q0, q1, [x28, #0x80]
1003a7234: f94127e8    	ldr	x8, [sp, #0x248]
1003a7238: f90153e8    	str	x8, [sp, #0x2a0]
1003a723c: f94137f6    	ldr	x22, [sp, #0x268]
