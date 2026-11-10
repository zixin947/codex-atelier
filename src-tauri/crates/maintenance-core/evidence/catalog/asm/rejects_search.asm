
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100203d68 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E>:
100203d68: d10283ff    	sub	sp, sp, #0xa0
100203d6c: a9065ff8    	stp	x24, x23, [sp, #0x60]
100203d70: a90757f6    	stp	x22, x21, [sp, #0x70]
100203d74: a9084ff4    	stp	x20, x19, [sp, #0x80]
100203d78: a9097bfd    	stp	x29, x30, [sp, #0x90]
100203d7c: 910243fd    	add	x29, sp, #0x90
100203d80: 12001c68    	and	w8, w3, #0xff
100203d84: 7100051f    	cmp	w8, #0x1
100203d88: 54000101    	b.ne	0x100203da8 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x40>
100203d8c: 3943fc08    	ldrb	w8, [x0, #0xff]
100203d90: 340001c8    	cbz	w8, 0x100203dc8 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x60>
100203d94: 51000d08    	sub	w8, w8, #0x3
100203d98: 7100091f    	cmp	w8, #0x2
100203d9c: 54000322    	b.hs	0x100203e00 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x98>
100203da0: 52800035    	mov	w21, #0x1               ; =1
100203da4: 14000002    	b	0x100203dac <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x44>
100203da8: 52800015    	mov	w21, #0x0               ; =0
100203dac: aa1503e0    	mov	x0, x21
100203db0: a9497bfd    	ldp	x29, x30, [sp, #0x90]
100203db4: a9484ff4    	ldp	x20, x19, [sp, #0x80]
100203db8: a94757f6    	ldp	x22, x21, [sp, #0x70]
100203dbc: a9465ff8    	ldp	x24, x23, [sp, #0x60]
100203dc0: 910283ff    	add	sp, sp, #0xa0
100203dc4: d65f03c0    	ret
100203dc8: aa0003f3    	mov	x19, x0
100203dcc: aa0103e0    	mov	x0, x1
100203dd0: aa0103f4    	mov	x20, x1
100203dd4: aa0203e1    	mov	x1, x2
100203dd8: aa0203f5    	mov	x21, x2
100203ddc: 94165575    	bl	0x1007993b0 <__ZN13codexmate_lib4core5relay6models15brand_for_model17hf2740b13b2d55153E>
100203de0: aa1403e1    	mov	x1, x20
100203de4: aa1503e2    	mov	x2, x21
100203de8: aa0003e8    	mov	x8, x0
100203dec: aa1303e0    	mov	x0, x19
100203df0: 12001d08    	and	w8, w8, #0xff
100203df4: 51000d08    	sub	w8, w8, #0x3
100203df8: 7100091f    	cmp	w8, #0x2
100203dfc: 54fffd23    	b.lo	0x100203da0 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x38>
100203e00: aa0003f7    	mov	x23, x0
100203e04: aa0103e0    	mov	x0, x1
100203e08: aa0203e1    	mov	x1, x2
100203e0c: 9400acd3    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100203e10: aa0103f3    	mov	x19, x1
100203e14: b6f800a1    	tbz	x1, #0x3f, 0x100203e28 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0xc0>
100203e18: d2800016    	mov	x22, #0x0               ; =0
100203e1c: aa1603e0    	mov	x0, x22
100203e20: aa1303e1    	mov	x1, x19
100203e24: 943f3210    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100203e28: aa0003f5    	mov	x21, x0
100203e2c: b40001f3    	cbz	x19, 0x100203e68 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x100>
100203e30: 97f7f5d1    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100203e34: 52800036    	mov	w22, #0x1               ; =1
100203e38: aa1303e0    	mov	x0, x19
100203e3c: 52800021    	mov	w1, #0x1                ; =1
100203e40: 97f7f5c9    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100203e44: b4fffec0    	cbz	x0, 0x100203e1c <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0xb4>
100203e48: aa0003f4    	mov	x20, x0
100203e4c: aa1503e1    	mov	x1, x21
100203e50: aa1303e2    	mov	x2, x19
100203e54: 943f3616    	bl	0x1011d16ac <_writev+0x1011d16ac>
100203e58: f100227f    	cmp	x19, #0x8
100203e5c: 54000122    	b.hs	0x100203e80 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x118>
100203e60: d2800008    	mov	x8, #0x0                ; =0
100203e64: 1400003b    	b	0x100203f50 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x1e8>
100203e68: 52800034    	mov	w20, #0x1               ; =1
100203e6c: 52800020    	mov	w0, #0x1                ; =1
100203e70: aa1503e1    	mov	x1, x21
100203e74: aa1303e2    	mov	x2, x19
100203e78: 943f360d    	bl	0x1011d16ac <_writev+0x1011d16ac>
100203e7c: 1400003f    	b	0x100203f78 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x210>
100203e80: f101027f    	cmp	x19, #0x40
100203e84: 54000062    	b.hs	0x100203e90 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x128>
100203e88: d2800008    	mov	x8, #0x0                ; =0
100203e8c: 14000021    	b	0x100203f10 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x1a8>
100203e90: 927ae268    	and	x8, x19, #0x7fffffffffffffc0
100203e94: 91008289    	add	x9, x20, #0x20
100203e98: 4f05e7e0    	movi.16b	v0, #0xbf
100203e9c: 4f00e741    	movi.16b	v1, #0x1a
100203ea0: 4f01e402    	movi.16b	v2, #0x20
100203ea4: aa0803ea    	mov	x10, x8
100203ea8: ad7f1123    	ldp	q3, q4, [x9, #-0x20]
100203eac: ad401925    	ldp	q5, q6, [x9]
100203eb0: 4e208467    	add.16b	v7, v3, v0
100203eb4: 4e208490    	add.16b	v16, v4, v0
100203eb8: 4e2084b1    	add.16b	v17, v5, v0
100203ebc: 4e2084d2    	add.16b	v18, v6, v0
100203ec0: 6e273427    	cmhi.16b	v7, v1, v7
100203ec4: 6e303430    	cmhi.16b	v16, v1, v16
100203ec8: 6e313431    	cmhi.16b	v17, v1, v17
100203ecc: 6e323432    	cmhi.16b	v18, v1, v18

/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100203d68 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E>:
100203f78: a900cfff    	stp	xzr, x19, [sp, #0x8]
100203f7c: a901cff4    	stp	x20, x19, [sp, #0x18]
100203f80: a902cfff    	stp	xzr, x19, [sp, #0x28]
100203f84: 0f0105e0    	movi.2s	v0, #0x2f
100203f88: fd001fe0    	str	d0, [sp, #0x38]
100203f8c: 52800028    	mov	w8, #0x1                ; =1
100203f90: 390103e8    	strb	w8, [sp, #0x40]
100203f94: 790093e8    	strh	w8, [sp, #0x48]
100203f98: 910023e0    	add	x0, sp, #0x8
100203f9c: 9400b008    	bl	0x10022ffbc <__ZN4core3str4iter22SplitInternal$LT$P$GT$9next_back17h919969fa09ebfb78E>
100203fa0: f100001f    	cmp	x0, #0x0
100203fa4: 9a810269    	csel	x9, x19, x1, eq
100203fa8: 9a800288    	csel	x8, x20, x0, eq
100203fac: f100113f    	cmp	x9, #0x4
100203fb0: 54000103    	b.lo	0x100203fd0 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x268>
100203fb4: b940010a    	ldr	w10, [x8]
100203fb8: 528d2dab    	mov	w11, #0x696d            ; =26989
100203fbc: 72adedab    	movk	w11, #0x6f6d, lsl #16
100203fc0: 6b0b015f    	cmp	w10, w11
100203fc4: 540006e0    	b.eq	0x1002040a0 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x338>
100203fc8: f1001d3f    	cmp	x9, #0x7
100203fcc: 54000282    	b.hs	0x10020401c <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x2b4>
100203fd0: a9438ae1    	ldp	x1, x2, [x23, #0x38]
100203fd4: 910023e0    	add	x0, sp, #0x8
100203fd8: 9400b7c1    	bl	0x100231edc <__ZN5alloc3str21_$LT$impl$u20$str$GT$18to_ascii_lowercase17h7ba3cfb8f7ad4e6bE>
100203fdc: 9000b0a8    	adrp	x8, 0x101817000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.56+0x2d8>
100203fe0: 91118108    	add	x8, x8, #0x460
100203fe4: 91010109    	add	x9, x8, #0x40
100203fe8: a93c27a8    	stp	x8, x9, [x29, #-0x40]
100203fec: a9410bf6    	ldp	x22, x2, [sp, #0x10]
100203ff0: d10103a0    	sub	x0, x29, #0x40
100203ff4: aa1603e1    	mov	x1, x22
100203ff8: 94010a67    	bl	0x100246994 <__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3any17hd644707e188aeb59E>
100203ffc: aa0003f5    	mov	x21, x0
100204000: f94007e1    	ldr	x1, [sp, #0x8]
100204004: b4000081    	cbz	x1, 0x100204014 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x2ac>
100204008: aa1603e0    	mov	x0, x22
10020400c: 52800022    	mov	w2, #0x1                ; =1
100204010: 97f7f556    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100204014: b50004b3    	cbnz	x19, 0x1002040a8 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x340>
100204018: 17ffff65    	b	0x100203dac <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x44>
10020401c: b940010a    	ldr	w10, [x8]
100204020: b840310b    	ldur	w11, [x8, #0x3]
100204024: 528d2dac    	mov	w12, #0x696d            ; =26989
100204028: 72ad2dcc    	movk	w12, #0x696e, lsl #16
10020402c: 6b0c015f    	cmp	w10, w12
100204030: 528dad2a    	mov	w10, #0x6d69            ; =28009
100204034: 72af0c2a    	movk	w10, #0x7861, lsl #16
100204038: 7a4a0160    	ccmp	w11, w10, #0x0, eq
10020403c: 54000320    	b.eq	0x1002040a0 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x338>
100204040: b940010a    	ldr	w10, [x8]
100204044: b840310b    	ldur	w11, [x8, #0x3]
100204048: 528ded8c    	mov	w12, #0x6f6c            ; =28524
10020404c: 72acedcc    	movk	w12, #0x676e, lsl #16
100204050: 6b0c015f    	cmp	w10, w12
100204054: 528c6cea    	mov	w10, #0x6367            ; =25447
100204058: 72ae8c2a    	movk	w10, #0x7461, lsl #16
10020405c: 7a4a0160    	ccmp	w11, w10, #0x0, eq
100204060: 54000200    	b.eq	0x1002040a0 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x338>
100204064: f1002d3f    	cmp	x9, #0xb
100204068: 54fffb43    	b.lo	0x100203fd0 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x268>
10020406c: f9400109    	ldr	x9, [x8]
100204070: f8403108    	ldur	x8, [x8, #0x3]
100204074: d28eee2a    	mov	x10, #0x7771            ; =30577
100204078: f2adccaa    	movk	x10, #0x6e65, lsl #16
10020407c: f2c5a66a    	movk	x10, #0x2d33, lsl #32
100204080: f2edec6a    	movk	x10, #0x6f63, lsl #48
100204084: eb0a013f    	cmp	x9, x10
100204088: d2866dc9    	mov	x9, #0x336e             ; =13166
10020408c: f2ac65a9    	movk	x9, #0x632d, lsl #16
100204090: f2cc8de9    	movk	x9, #0x646f, lsl #32
100204094: f2ee4ca9    	movk	x9, #0x7265, lsl #48
100204098: fa490100    	ccmp	x8, x9, #0x0, eq
10020409c: 54fff9a1    	b.ne	0x100203fd0 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x268>
1002040a0: 52800035    	mov	w21, #0x1               ; =1
1002040a4: b4ffe853    	cbz	x19, 0x100203dac <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x44>
1002040a8: aa1403e0    	mov	x0, x20
1002040ac: aa1303e1    	mov	x1, x19
1002040b0: 52800022    	mov	w2, #0x1                ; =1
1002040b4: 97f7f52d    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1002040b8: 17ffff3d    	b	0x100203dac <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x44>
1002040bc: aa0003f5    	mov	x21, x0
1002040c0: f94007e1    	ldr	x1, [sp, #0x8]
1002040c4: b40000c1    	cbz	x1, 0x1002040dc <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x374>
1002040c8: aa1603e0    	mov	x0, x22
1002040cc: 52800022    	mov	w2, #0x1                ; =1
1002040d0: 97f7f526    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1002040d4: 14000002    	b	0x1002040dc <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x374>
1002040d8: aa0003f5    	mov	x21, x0
1002040dc: b40000b3    	cbz	x19, 0x1002040f0 <__ZN13codexmate_lib4core5relay23native_responses_compat25rejects_hosted_web_search17hc9cad7aca7ecba75E+0x388>
1002040e0: aa1403e0    	mov	x0, x20
1002040e4: aa1303e1    	mov	x1, x19
1002040e8: 52800022    	mov	w2, #0x1                ; =1
1002040ec: 97f7f51f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1002040f0: aa1503e0    	mov	x0, x21
1002040f4: 943f3418    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001002040f8 <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E>:
1002040f8: d105c3ff    	sub	sp, sp, #0x170
1002040fc: a9116ffc    	stp	x28, x27, [sp, #0x110]
100204100: a91267fa    	stp	x26, x25, [sp, #0x120]
100204104: a9135ff8    	stp	x24, x23, [sp, #0x130]
100204108: a91457f6    	stp	x22, x21, [sp, #0x140]
10020410c: a9154ff4    	stp	x20, x19, [sp, #0x150]
100204110: a9167bfd    	stp	x29, x30, [sp, #0x160]
100204114: 910583fd    	add	x29, sp, #0x160
100204118: aa0103f4    	mov	x20, x1
10020411c: aa0003f5    	mov	x21, x0
100204120: aa0803f3    	mov	x19, x8
100204124: 941ba7fe    	bl	0x1008ee11c <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E>
100204128: b4000080    	cbz	x0, 0x100204138 <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x40>
10020412c: 52800048    	mov	w8, #0x2                ; =2
100204130: a9000268    	stp	x8, x0, [x19]
100204134: 140000f6    	b	0x10020450c <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x414>
100204138: f94132b6    	ldr	x22, [x21, #0x260]
10020413c: f94136b8    	ldr	x24, [x21, #0x268]
100204140: aa1603e0    	mov	x0, x22
100204144: aa1803e1    	mov	x1, x24
100204148: 943d2ca7    	bl	0x10114f3e4 <__ZN3std4path4Path6is_dir17h05d88b8a2e7370d3E>
10020414c: 360001c0    	tbz	w0, #0x0, 0x100204184 <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x8c>
100204150: a90563f6    	stp	x22, x24, [sp, #0x50]
100204154: 910143e8    	add	x8, sp, #0x50
100204158: d0007ac9    	adrp	x9, 0x10115e000 <__ZN4core5slice4sort8unstable9quicksort9quicksort17h324c95ad922d2016E+0x1fc>
10020415c: 91351129    	add	x9, x9, #0xd44
100204160: a90727e8    	stp	x8, x9, [sp, #0x70]
100204164: b00098c0    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
100204168: 9119ec00    	add	x0, x0, #0x67b
10020416c: 91002268    	add	x8, x19, #0x8
100204170: 9101c3e1    	add	x1, sp, #0x70
100204174: 943dcdc8    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100204178: 52800148    	mov	w8, #0xa                ; =10
10020417c: f9000268    	str	x8, [x19]
100204180: 140000e3    	b	0x10020450c <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x414>
100204184: 9101c3fa    	add	x26, sp, #0x70
100204188: 9101c3e8    	add	x8, sp, #0x70
10020418c: aa1603e0    	mov	x0, x22
100204190: aa1803e1    	mov	x1, x24
100204194: 943d1119    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
100204198: b94073e8    	ldr	w8, [sp, #0x70]
10020419c: f9403fe1    	ldr	x1, [sp, #0x78]
1002041a0: 7100051f    	cmp	w8, #0x1
1002041a4: 54000081    	b.ne	0x1002041b4 <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0xbc>
1002041a8: 52800020    	mov	w0, #0x1                ; =1
1002041ac: 9400a8a1    	bl	0x10022e430 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
1002041b0: 14000045    	b	0x1002042c4 <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x1cc>
1002041b4: d2800000    	mov	x0, #0x0                ; =0
1002041b8: 9400a89e    	bl	0x10022e430 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
1002041bc: b9000bff    	str	wzr, [sp, #0x8]
1002041c0: 528036c8    	mov	w8, #0x1b6              ; =438
1002041c4: 79001be8    	strh	w8, [sp, #0xc]
1002041c8: b800e3ff    	stur	wzr, [sp, #0xe]
1002041cc: 790027ff    	strh	wzr, [sp, #0x12]
1002041d0: 52800028    	mov	w8, #0x1                ; =1
1002041d4: 39003fe8    	strb	w8, [sp, #0xf]
1002041d8: 910063e8    	add	x8, sp, #0x18
1002041dc: 910023e0    	add	x0, sp, #0x8
1002041e0: aa1603e1    	mov	x1, x22
1002041e4: aa1803e2    	mov	x2, x24
1002041e8: 943cf77e    	bl	0x101141fe0 <__ZN3std2fs11OpenOptions5_open17hfd14fc0134faefc7E>
1002041ec: b9401be8    	ldr	w8, [sp, #0x18]
1002041f0: 7100051f    	cmp	w8, #0x1
1002041f4: 54000641    	b.ne	0x1002042bc <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x1c4>
1002041f8: f94013e8    	ldr	x8, [sp, #0x20]
1002041fc: f90027e8    	str	x8, [sp, #0x48]
100204200: 910123e8    	add	x8, sp, #0x48
100204204: b0007b29    	adrp	x9, 0x101169000 <__ZN5gimli4read8rnglists20RngListIter$LT$R$GT$4next17ha0add38c23e78896E+0x510>
100204208: 91211129    	add	x9, x9, #0x844
10020420c: a90727e8    	stp	x8, x9, [sp, #0x70]
100204210: b00098c0    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
100204214: 911b6400    	add	x0, x0, #0x6d9
100204218: 910143e8    	add	x8, sp, #0x50
10020421c: 9101c3e1    	add	x1, sp, #0x70
100204220: 943dcd9d    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100204224: b94053f6    	ldr	w22, [sp, #0x50]
100204228: 3cc543e0    	ldur	q0, [sp, #0x54]
10020422c: 3d801fe0    	str	q0, [sp, #0x70]
100204230: b94067e8    	ldr	w8, [sp, #0x64]
100204234: b90083e8    	str	w8, [sp, #0x80]
100204238: f94027f4    	ldr	x20, [sp, #0x48]
10020423c: 92400688    	and	x8, x20, #0x3
100204240: f100051f    	cmp	x8, #0x1
100204244: 54000201    	b.ne	0x100204284 <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x18c>
100204248: f85ffe95    	ldr	x21, [x20, #-0x1]!
10020424c: f9400697    	ldr	x23, [x20, #0x8]
100204250: f94002e8    	ldr	x8, [x23]
100204254: b4000068    	cbz	x8, 0x100204260 <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x168>
100204258: aa1503e0    	mov	x0, x21
10020425c: d63f0100    	blr	x8
100204260: f94006e1    	ldr	x1, [x23, #0x8]
100204264: b4000081    	cbz	x1, 0x100204274 <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x17c>
100204268: f9400ae2    	ldr	x2, [x23, #0x10]
10020426c: aa1503e0    	mov	x0, x21
100204270: 97f7f4be    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100204274: aa1403e0    	mov	x0, x20
100204278: 52800301    	mov	w1, #0x18               ; =24
10020427c: 52800102    	mov	w2, #0x8                ; =8
100204280: 97f7f4ba    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100204284: ad410740    	ldp	q0, q1, [x26, #0x20]
100204288: 3c82c260    	stur	q0, [x19, #0x2c]
10020428c: 3c83c261    	stur	q1, [x19, #0x3c]
100204290: 3dc01340    	ldr	q0, [x26, #0x40]
100204294: 3c84c260    	stur	q0, [x19, #0x4c]
100204298: ad4387e0    	ldp	q0, q1, [sp, #0x70]
10020429c: 3c80c260    	stur	q0, [x19, #0xc]
1002042a0: b940c3e8    	ldr	w8, [sp, #0xc0]
1002042a4: b9005e68    	str	w8, [x19, #0x5c]
1002042a8: 3c81c261    	stur	q1, [x19, #0x1c]
1002042ac: 52800148    	mov	w8, #0xa                ; =10
1002042b0: f9000268    	str	x8, [x19]
1002042b4: b9000a76    	str	w22, [x19, #0x8]
1002042b8: 14000095    	b	0x10020450c <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x414>
1002042bc: b9401fe0    	ldr	w0, [sp, #0x1c]
1002042c0: 943f340b    	bl	0x1011d12ec <_writev+0x1011d12ec>
1002042c4: 9101c3e0    	add	x0, sp, #0x70
1002042c8: aa1403e1    	mov	x1, x20
1002042cc: 97ff7326    	bl	0x1001e0f64 <__ZN10serde_json3ser13to_vec_pretty17hf4ba7587cfff0ac9E>
1002042d0: a94753f5    	ldp	x21, x20, [sp, #0x70]
1002042d4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1002042d8: eb0802bf    	cmp	x21, x8
1002042dc: 54000081    	b.ne	0x1002042ec <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x1f4>
1002042e0: 52800068    	mov	w8, #0x3                ; =3
1002042e4: a9005268    	stp	x8, x20, [x19]
1002042e8: 14000089    	b	0x10020450c <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x414>
1002042ec: f94043f7    	ldr	x23, [sp, #0x80]
1002042f0: aa1603e0    	mov	x0, x22
1002042f4: aa1803e1    	mov	x1, x24
1002042f8: 943d2cad    	bl	0x10114f5ac <__ZN3std4path4Path6parent17h2f7f79d216a1f739E>
1002042fc: aa0003f6    	mov	x22, x0
100204300: b4000aa0    	cbz	x0, 0x100204454 <__ZN13codexmate_lib4core5relay7storage14preflight_save17h85bcb79118088712E+0x35c>
100204304: aa0103f8    	mov	x24, x1
100204308: 943d362a    	bl	0x101151bb0 <__ZN3std7process2id17h3613cdd14e59bdb4E>
10020430c: b9000be0    	str	w0, [sp, #0x8]
100204310: 9101c3e8    	add	x8, sp, #0x70
100204314: 943986ae    	bl	0x101065dcc <__ZN4uuid2v428_$LT$impl$u20$uuid..Uuid$GT$6new_v417h6ad5de516c3d59ecE>
100204318: 3dc01fe0    	ldr	q0, [sp, #0x70]
10020431c: 3d8017e0    	str	q0, [sp, #0x50]
100204320: 910023e8    	add	x8, sp, #0x8
100204324: f0007ba9    	adrp	x9, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
100204328: 910bb129    	add	x9, x9, #0x2ec
10020432c: a90727e8    	stp	x8, x9, [sp, #0x70]
100204330: 910143e8    	add	x8, sp, #0x50
100204334: b0000169    	adrp	x9, 0x100231000 <__ZN4core5slice4sort6stable5merge5merge17he20bffacf3d2822eE+0x208>
100204338: 91188129    	add	x9, x9, #0x620
10020433c: a90827e8    	stp	x8, x9, [sp, #0x80]
100204340: b00098c0    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
100204344: 91197800    	add	x0, x0, #0x65e
100204348: 9100c3e8    	add	x8, sp, #0x30
10020434c: 9101c3e1    	add	x1, sp, #0x70
