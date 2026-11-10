
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100209cec <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE>:
100209cec: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
100209cf0: a90167fa    	stp	x26, x25, [sp, #0x10]
100209cf4: a9025ff8    	stp	x24, x23, [sp, #0x20]
100209cf8: a90357f6    	stp	x22, x21, [sp, #0x30]
100209cfc: a9044ff4    	stp	x20, x19, [sp, #0x40]
100209d00: a9057bfd    	stp	x29, x30, [sp, #0x50]
100209d04: 910143fd    	add	x29, sp, #0x50
100209d08: d11443ff    	sub	sp, sp, #0x510
100209d0c: f90003ff    	str	xzr, [sp]
100209d10: aa0003f6    	mov	x22, x0
100209d14: aa0803f5    	mov	x21, x8
100209d18: 941b9101    	bl	0x1008ee11c <__ZN13codexmate_lib8platform5paths10CodexPaths18ensure_directories17hdb891f73b7db9411E>
100209d1c: b40000c0    	cbz	x0, 0x100209d34 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x48>
100209d20: f9000aa0    	str	x0, [x21, #0x10]
100209d24: f00087c8    	adrp	x8, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
100209d28: 3dc07500    	ldr	q0, [x8, #0x1d0]
100209d2c: 3d8002a0    	str	q0, [x21]
100209d30: 140002c0    	b	0x10020a830 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xb44>
100209d34: f94132d3    	ldr	x19, [x22, #0x260]
100209d38: f94136d4    	ldr	x20, [x22, #0x268]
100209d3c: 910a43e8    	add	x8, sp, #0x290
100209d40: aa1303e0    	mov	x0, x19
100209d44: aa1403e1    	mov	x1, x20
100209d48: 943cfa2c    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
100209d4c: b94293e8    	ldr	w8, [sp, #0x290]
100209d50: f9414fe1    	ldr	x1, [sp, #0x298]
100209d54: 7100051f    	cmp	w8, #0x1
100209d58: 54000501    	b.ne	0x100209df8 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x10c>
100209d5c: 52800033    	mov	w19, #0x1               ; =1
100209d60: 52800020    	mov	w0, #0x1                ; =1
100209d64: 940091b3    	bl	0x10022e430 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
100209d68: f9019bff    	str	xzr, [sp, #0x330]
100209d6c: f90197ff    	str	xzr, [sp, #0x328]
100209d70: ad5887e0    	ldp	q0, q1, [sp, #0x310]
100209d74: ad578be3    	ldp	q3, q2, [sp, #0x2f0]
100209d78: ad0882a2    	stp	q2, q0, [x21, #0x110]
100209d7c: 3d804ea1    	str	q1, [x21, #0x130]
100209d80: ad5687e0    	ldp	q0, q1, [sp, #0x2d0]
100209d84: ad558be4    	ldp	q4, q2, [sp, #0x2b0]
100209d88: ad0682a2    	stp	q2, q0, [x21, #0xd0]
100209d8c: b00087c8    	adrp	x8, 0x101302000 <_writev+0x101302000>
100209d90: 3dc03500    	ldr	q0, [x8, #0xd0]
100209d94: 3c8582a0    	stur	q0, [x21, #0x58]
100209d98: ad078ea1    	stp	q1, q3, [x21, #0xf0]
100209d9c: ad548be1    	ldp	q1, q2, [sp, #0x290]
100209da0: ad0486a0    	stp	q0, q1, [x21, #0x90]
100209da4: a9017ebf    	stp	xzr, xzr, [x21, #0x10]
100209da8: a9047ebf    	stp	xzr, xzr, [x21, #0x40]
100209dac: 52800108    	mov	w8, #0x8                ; =8
100209db0: a90022bf    	stp	xzr, x8, [x21]
100209db4: a9027ea8    	stp	x8, xzr, [x21, #0x20]
100209db8: a9034ebf    	stp	xzr, x19, [x21, #0x30]
100209dbc: f9002ab3    	str	x19, [x21, #0x50]
100209dc0: 7900f2bf    	strh	wzr, [x21, #0x78]
100209dc4: 3901eabf    	strb	wzr, [x21, #0x7a]
100209dc8: a90822bf    	stp	xzr, x8, [x21, #0x80]
100209dcc: f9419be8    	ldr	x8, [sp, #0x330]
100209dd0: f900a2a8    	str	x8, [x21, #0x140]
100209dd4: ad0592a2    	stp	q2, q4, [x21, #0xb0]
100209dd8: 528000e8    	mov	w8, #0x7                ; =7
100209ddc: 7902c2a8    	strh	w8, [x21, #0x160]
100209de0: 52800208    	mov	w8, #0x10               ; =16
100209de4: b9016aa8    	str	w8, [x21, #0x168]
100209de8: 7902dabf    	strh	wzr, [x21, #0x16c]
100209dec: 52802008    	mov	w8, #0x100              ; =256
100209df0: 7902fea8    	strh	w8, [x21, #0x17e]
100209df4: 1400028f    	b	0x10020a830 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xb44>
100209df8: d2800000    	mov	x0, #0x0                ; =0
100209dfc: 9400918d    	bl	0x10022e430 <__ZN4core3ptr90drop_in_place$LT$core..result..Result$LT$std..fs..Metadata$C$std..io..error..Error$GT$$GT$17h5a2c154c6f1229d7E>
100209e00: 910a43e8    	add	x8, sp, #0x290
100209e04: aa1303e0    	mov	x0, x19
100209e08: aa1403e1    	mov	x1, x20
100209e0c: 943ce0b4    	bl	0x1011420dc <__ZN3std2fs14read_to_string5inner17h0f85889ccb0352e2E>
100209e10: f9414bf3    	ldr	x19, [sp, #0x290]
100209e14: f9414ff4    	ldr	x20, [sp, #0x298]
100209e18: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100209e1c: eb08027f    	cmp	x19, x8
100209e20: 54000061    	b.ne	0x100209e2c <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x140>
100209e24: f9000ab4    	str	x20, [x21, #0x10]
100209e28: 17ffffbf    	b	0x100209d24 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x38>
100209e2c: f94153f8    	ldr	x24, [sp, #0x2a0]
100209e30: aa1403e0    	mov	x0, x20
100209e34: aa1803e1    	mov	x1, x24
100209e38: 940094c8    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100209e3c: b40005a1    	cbz	x1, 0x100209ef0 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x204>
100209e40: f9014bf4    	str	x20, [sp, #0x290]
100209e44: f9014ff8    	str	x24, [sp, #0x298]
100209e48: f90157ff    	str	xzr, [sp, #0x2a8]
100209e4c: f90153ff    	str	xzr, [sp, #0x2a0]
100209e50: f9015bf4    	str	x20, [sp, #0x2b0]
100209e54: f9015ff8    	str	x24, [sp, #0x2b8]
100209e58: 910143e8    	add	x8, sp, #0x50
100209e5c: 910a43e0    	add	x0, sp, #0x290
100209e60: 97ff43ed    	bl	0x1001dae14 <__ZN10serde_json2de10from_trait17h472fe9ac40d19421E>
100209e64: 394143e8    	ldrb	w8, [sp, #0x50]
100209e68: 7100191f    	cmp	w8, #0x6
100209e6c: 540006a1    	b.ne	0x100209f40 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x254>
100209e70: f9402fe8    	ldr	x8, [sp, #0x58]
100209e74: f81183a8    	stur	x8, [x29, #-0xe8]
100209e78: d103a3a8    	sub	x8, x29, #0xe8
100209e7c: d0007809    	adrp	x9, 0x10110b000 <__ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h5a90e0e0b6d91f2fE+0x50>
100209e80: 910ab129    	add	x9, x9, #0x2ac
100209e84: f9014be8    	str	x8, [sp, #0x290]
100209e88: f9014fe9    	str	x9, [sp, #0x298]
100209e8c: 900098a0    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
100209e90: 912edc00    	add	x0, x0, #0xbb7
100209e94: 910743e8    	add	x8, sp, #0x1d0
100209e98: 910a43e1    	add	x1, sp, #0x290
100209e9c: 943db67e    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
100209ea0: 3dc077e0    	ldr	q0, [sp, #0x1d0]
100209ea4: 3d80a7e0    	str	q0, [sp, #0x290]
100209ea8: f940f3e8    	ldr	x8, [sp, #0x1e0]
100209eac: f90153e8    	str	x8, [sp, #0x2a0]
100209eb0: f85183b6    	ldur	x22, [x29, #-0xe8]
100209eb4: aa1603e0    	mov	x0, x22
100209eb8: 94008116    	bl	0x10022a310 <__ZN4core3ptr49drop_in_place$LT$serde_json..error..ErrorCode$GT$17h4487869eebe89369E>
100209ebc: aa1603e0    	mov	x0, x22
100209ec0: 52800501    	mov	w1, #0x28               ; =40
100209ec4: 52800102    	mov	w2, #0x8                ; =8
100209ec8: 97f7dda8    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100209ecc: ad5483e1    	ldp	q1, q0, [sp, #0x290]
100209ed0: ad3a03a1    	stp	q1, q0, [x29, #-0xc0]
100209ed4: ad1183e1    	stp	q1, q0, [sp, #0x230]
100209ed8: ad5187e0    	ldp	q0, q1, [sp, #0x230]
100209edc: ad0086a0    	stp	q0, q1, [x21, #0x10]
100209ee0: f00087c8    	adrp	x8, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
100209ee4: 3dc07901    	ldr	q1, [x8, #0x1e0]
100209ee8: 3d8002a1    	str	q1, [x21]
100209eec: 1400024c    	b	0x10020a81c <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xb30>
100209ef0: 97f7dda1    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100209ef4: 528006c0    	mov	w0, #0x36               ; =54
100209ef8: 52800021    	mov	w1, #0x1                ; =1
100209efc: 97f7dd9a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100209f00: b4006080    	cbz	x0, 0x10020ab10 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xe24>
100209f04: b00087e8    	adrp	x8, 0x101306000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.305+0x2bd>
100209f08: 911bd508    	add	x8, x8, #0x6f5
100209f0c: ad400500    	ldp	q0, q1, [x8]
100209f10: ad000400    	stp	q0, q1, [x0]
100209f14: 3dc00900    	ldr	q0, [x8, #0x20]
100209f18: 3d800800    	str	q0, [x0, #0x20]
100209f1c: f842e108    	ldur	x8, [x8, #0x2e]
100209f20: f802e008    	stur	x8, [x0, #0x2e]
100209f24: 528006c8    	mov	w8, #0x36               ; =54
100209f28: a90102a8    	stp	x8, x0, [x21, #0x10]
100209f2c: f90012a8    	str	x8, [x21, #0x20]
100209f30: f00087c8    	adrp	x8, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
100209f34: 3dc07900    	ldr	q0, [x8, #0x1e0]
100209f38: 3d8002a0    	str	q0, [x21]
100209f3c: 14000238    	b	0x10020a81c <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xb30>
100209f40: ad4283e1    	ldp	q1, q0, [sp, #0x50]
100209f44: ad0083e1    	stp	q1, q0, [sp, #0x10]
100209f48: 900087e0    	adrp	x0, 0x101305000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.145+0x3e0>
100209f4c: 91153000    	add	x0, x0, #0x54c
100209f50: 910043e2    	add	x2, sp, #0x10
100209f54: 528001a1    	mov	w1, #0xd                ; =13
100209f58: 943c130e    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
100209f5c: 910a43f9    	add	x25, sp, #0x290
100209f60: b4000140    	cbz	x0, 0x100209f88 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x29c>
100209f64: 39400008    	ldrb	w8, [x0]
100209f68: 7100091f    	cmp	w8, #0x2
100209f6c: 54000d21    	b.ne	0x10020a110 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x424>
100209f70: f9400408    	ldr	x8, [x0, #0x8]
100209f74: b4000968    	cbz	x8, 0x10020a0a0 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x3b4>
100209f78: f100051f    	cmp	x8, #0x1
100209f7c: 54000ca1    	b.ne	0x10020a110 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x424>
100209f80: f940081c    	ldr	x28, [x0, #0x10]
100209f84: 14000049    	b	0x10020a0a8 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x3bc>
100209f88: 5280001c    	mov	w28, #0x0               ; =0
100209f8c: b90037ff    	str	wzr, [sp, #0x34]
100209f90: b00087e0    	adrp	x0, 0x101306000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.305+0x2bd>
100209f94: 91143000    	add	x0, x0, #0x50c
100209f98: 910043e2    	add	x2, sp, #0x10
100209f9c: 528001c1    	mov	w1, #0xe                ; =14
100209fa0: 943c12fc    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
100209fa4: aa0003e1    	mov	x1, x0
100209fa8: b4000461    	cbz	x1, 0x10020a034 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x348>
100209fac: 39400028    	ldrb	w8, [x1]
100209fb0: 34000428    	cbz	w8, 0x10020a034 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x348>
100209fb4: 71003f9f    	cmp	w28, #0xf
100209fb8: 5400016b    	b.lt	0x100209fe4 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x2f8>
100209fbc: 910a43e0    	add	x0, sp, #0x290
100209fc0: 9400a730    	bl	0x100233c80 <__ZN63_$LT$serde_json..value..Value$u20$as$u20$core..clone..Clone$GT$5clone17h3cd5d3d303a5b673E>
100209fc4: 910143e8    	add	x8, sp, #0x50
100209fc8: 910a43e0    	add	x0, sp, #0x290
100209fcc: 941653ff    	bl	0x10079efc8 <__ZN13codexmate_lib4core5relay6models1_111_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..RouterOperationId$GT$11deserialize17h0ead1bed9d66bab3E>
100209fd0: 394143e8    	ldrb	w8, [sp, #0x50]
100209fd4: 34000308    	cbz	w8, 0x10020a034 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x348>
100209fd8: f9402fe1    	ldr	x1, [sp, #0x58]
100209fdc: 52800020    	mov	w0, #0x1                ; =1
100209fe0: 94006d06    	bl	0x1002253f8 <__ZN4core3ptr118drop_in_place$LT$core..result..Result$LT$codexmate_lib..core..repository..ApiConfig$C$serde_json..error..Error$GT$$GT$17h30fdb21acf4167a4E>
100209fe4: 97f7dd64    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100209fe8: 52800920    	mov	w0, #0x49               ; =73
100209fec: 52800021    	mov	w1, #0x1                ; =1
100209ff0: 97f7dd5d    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100209ff4: b4005960    	cbz	x0, 0x10020ab20 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xe34>
100209ff8: b00087e8    	adrp	x8, 0x101306000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.305+0x2bd>
100209ffc: 911cad08    	add	x8, x8, #0x72b
10020a000: ad410500    	ldp	q0, q1, [x8, #0x20]
10020a004: ad010400    	stp	q0, q1, [x0, #0x20]
10020a008: 3cc39100    	ldur	q0, [x8, #0x39]
10020a00c: 3c839000    	stur	q0, [x0, #0x39]
10020a010: ad400101    	ldp	q1, q0, [x8]
10020a014: ad000001    	stp	q1, q0, [x0]
10020a018: 52800928    	mov	w8, #0x49               ; =73
10020a01c: a90102a8    	stp	x8, x0, [x21, #0x10]
10020a020: f90012a8    	str	x8, [x21, #0x20]
10020a024: d00087c8    	adrp	x8, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
10020a028: 3dc07900    	ldr	q0, [x8, #0x1e0]
10020a02c: 3d8002a0    	str	q0, [x21]
10020a030: 140001f9    	b	0x10020a814 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xb28>
10020a034: 900087e0    	adrp	x0, 0x101306000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.305+0x2bd>
10020a038: 911dd000    	add	x0, x0, #0x774
10020a03c: 910043e2    	add	x2, sp, #0x10
10020a040: 52800161    	mov	w1, #0xb                ; =11
10020a044: 943c130d    	bl	0x10110ec78 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$14index_into_mut17h00fb942b2e888945E>
10020a048: b4000bc0    	cbz	x0, 0x10020a1c0 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x4d4>
10020a04c: 38408408    	ldrb	w8, [x0], #0x8
10020a050: 7100151f    	cmp	w8, #0x5
10020a054: 54000b61    	b.ne	0x10020a1c0 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x4d4>
10020a058: 900087e1    	adrp	x1, 0x101306000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.305+0x2bd>
10020a05c: 911dfc21    	add	x1, x1, #0x77f
10020a060: 528000a2    	mov	w2, #0x5                ; =5
10020a064: 94153af0    	bl	0x100758c24 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$7get_mut17h3c79b65c876bd82fE>
10020a068: b4000ac0    	cbz	x0, 0x10020a1c0 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x4d4>
10020a06c: aa0003f7    	mov	x23, x0
10020a070: 39400008    	ldrb	w8, [x0]
10020a074: 71000d1f    	cmp	w8, #0x3
10020a078: 54000740    	b.eq	0x10020a160 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x474>
10020a07c: 35000a28    	cbnz	w8, 0x10020a1c0 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x4d4>
10020a080: aa1703e0    	mov	x0, x23
10020a084: 94007fc7    	bl	0x100229fa0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10020a088: 52800088    	mov	w8, #0x4                ; =4
10020a08c: 390002e8    	strb	w8, [x23]
10020a090: 52800108    	mov	w8, #0x8                ; =8
10020a094: a900a2ff    	stp	xzr, x8, [x23, #0x8]
10020a098: f9000eff    	str	xzr, [x23, #0x18]
10020a09c: 14000049    	b	0x10020a1c0 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x4d4>
10020a0a0: f940081c    	ldr	x28, [x0, #0x10]
10020a0a4: b7f8037c    	tbnz	x28, #0x3f, 0x10020a110 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x424>
10020a0a8: eb3cc39f    	cmp	x28, w28, sxtw
10020a0ac: 54000321    	b.ne	0x10020a110 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x424>
10020a0b0: b90037fc    	str	w28, [sp, #0x34]
10020a0b4: f100439f    	cmp	x28, #0x10
10020a0b8: 54fff6cd    	b.le	0x100209f90 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x2a4>
10020a0bc: 9100d3e8    	add	x8, sp, #0x34
10020a0c0: b0007b89    	adrp	x9, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
10020a0c4: 91043129    	add	x9, x9, #0x10c
10020a0c8: f9014be8    	str	x8, [sp, #0x290]
10020a0cc: f9014fe9    	str	x9, [sp, #0x298]
10020a0d0: d00087e8    	adrp	x8, 0x101308000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.1087+0x3d>
10020a0d4: 910ab108    	add	x8, x8, #0x2ac
10020a0d8: f90153e8    	str	x8, [sp, #0x2a0]
10020a0dc: f90157e9    	str	x9, [sp, #0x2a8]
10020a0e0: f0009880    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
10020a0e4: 912d6800    	add	x0, x0, #0xb5a
10020a0e8: 9100e3e8    	add	x8, sp, #0x38
10020a0ec: 910a43e1    	add	x1, sp, #0x290
10020a0f0: 943db5e9    	bl	0x101177894 <__ZN5alloc3fmt6format12format_inner17hd571df24e790cabcE>
10020a0f4: 3cc383e0    	ldur	q0, [sp, #0x38]
10020a0f8: f94027e8    	ldr	x8, [sp, #0x48]
10020a0fc: f90012a8    	str	x8, [x21, #0x20]
10020a100: d00087c8    	adrp	x8, 0x101304000 <_anon.0ebe3621b21ee6692feb28debc4f6fb9.1710+0x271>
10020a104: 3dc07901    	ldr	q1, [x8, #0x1e0]
10020a108: ad0002a1    	stp	q1, q0, [x21]
10020a10c: 140001c2    	b	0x10020a814 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xb28>
10020a110: 910a43e0    	add	x0, sp, #0x290
10020a114: 94000336    	bl	0x10020adec <__ZN13codexmate_lib4core5relay7storage4load28_$u7b$$u7b$closure$u7d$$u7d$17h903d2144fddb5b55E>
10020a118: f9414be8    	ldr	x8, [sp, #0x290]
10020a11c: 3cc2c320    	ldur	q0, [x25, #0x2c]
10020a120: 3c8342a0    	stur	q0, [x21, #0x34]
10020a124: 3cc3c320    	ldur	q0, [x25, #0x3c]
10020a128: 3c8442a0    	stur	q0, [x21, #0x44]
10020a12c: 3cc4c320    	ldur	q0, [x25, #0x4c]
10020a130: 3c8542a0    	stur	q0, [x21, #0x54]
10020a134: b9429be9    	ldr	w9, [sp, #0x298]
10020a138: b942efea    	ldr	w10, [sp, #0x2ec]
10020a13c: b90066aa    	str	w10, [x21, #0x64]
10020a140: 3cc0c320    	ldur	q0, [x25, #0xc]
10020a144: 3c8142a0    	stur	q0, [x21, #0x14]
10020a148: 3cc1c320    	ldur	q0, [x25, #0x1c]
10020a14c: 3c8242a0    	stur	q0, [x21, #0x24]
10020a150: b90012a9    	str	w9, [x21, #0x10]
10020a154: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
10020a158: a90022a9    	stp	x9, x8, [x21]
10020a15c: 140001ae    	b	0x10020a814 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xb28>
10020a160: a940a2e9    	ldp	x9, x8, [x23, #0x8]
10020a164: a90027e8    	stp	x8, x9, [sp]
10020a168: f9400efa    	ldr	x26, [x23, #0x18]
10020a16c: 52800028    	mov	w8, #0x1                ; =1
10020a170: a900a2ff    	stp	xzr, x8, [x23, #0x8]
10020a174: f9000eff    	str	xzr, [x23, #0x18]
10020a178: 97f7dcff    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10020a17c: 52800400    	mov	w0, #0x20               ; =32
10020a180: 52800101    	mov	w1, #0x8                ; =8
10020a184: 97f7dcf8    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10020a188: b4004ba0    	cbz	x0, 0x10020aafc <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xe10>
10020a18c: aa0003fb    	mov	x27, x0
10020a190: 52800068    	mov	w8, #0x3                ; =3
10020a194: 39000008    	strb	w8, [x0]
10020a198: a94027e8    	ldp	x8, x9, [sp]
10020a19c: a900a009    	stp	x9, x8, [x0, #0x8]
10020a1a0: f9000c1a    	str	x26, [x0, #0x18]
10020a1a4: aa1703e0    	mov	x0, x23
10020a1a8: 94007f7e    	bl	0x100229fa0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10020a1ac: 52800088    	mov	w8, #0x4                ; =4
10020a1b0: 390002e8    	strb	w8, [x23]
10020a1b4: 52800028    	mov	w8, #0x1                ; =1
10020a1b8: a900eee8    	stp	x8, x27, [x23, #0x8]
10020a1bc: f9000ee8    	str	x8, [x23, #0x18]
10020a1c0: d10303a0    	sub	x0, x29, #0xc0
10020a1c4: 910043e1    	add	x1, sp, #0x10
10020a1c8: 9400a6ae    	bl	0x100233c80 <__ZN63_$LT$serde_json..value..Value$u20$as$u20$core..clone..Clone$GT$5clone17h3cd5d3d303a5b673E>
10020a1cc: d0008d01    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10020a1d0: 910aac21    	add	x1, x1, #0x2ab
10020a1d4: 9000b103    	adrp	x3, 0x10182a000 <_anon.cdb272158a04aae91ce02c6e47f106c7.1+0x6d0>
10020a1d8: 913c2063    	add	x3, x3, #0xf08
10020a1dc: 910a43fa    	add	x26, sp, #0x290
10020a1e0: 910a43e8    	add	x8, sp, #0x290
10020a1e4: d10303a0    	sub	x0, x29, #0xc0
10020a1e8: 52800142    	mov	w2, #0xa                ; =10
10020a1ec: 52800144    	mov	w4, #0xa                ; =10
10020a1f0: 941c3afa    	bl	0x100918dd8 <__ZN10serde_json5value2de83_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..value..Value$GT$18deserialize_struct17h6af4a717083fb407E>
10020a1f4: f9414bf7    	ldr	x23, [sp, #0x290]
10020a1f8: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10020a1fc: eb0802ff    	cmp	x23, x8
10020a200: 54000301    	b.ne	0x10020a260 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x574>
10020a204: f9414fe1    	ldr	x1, [sp, #0x298]
10020a208: d10303a0    	sub	x0, x29, #0xc0
10020a20c: 940002a4    	bl	0x10020ac9c <__ZN13codexmate_lib4core5relay7storage4load28_$u7b$$u7b$closure$u7d$$u7d$17h74f1af0768461961E>
10020a210: ad7c07a0    	ldp	q0, q1, [x29, #-0x80]
10020a214: ad1387e0    	stp	q0, q1, [sp, #0x270]
10020a218: ad7a0fa2    	ldp	q2, q3, [x29, #-0xc0]
10020a21c: ad110be1    	stp	q1, q2, [sp, #0x220]
10020a220: ad7b17a4    	ldp	q4, q5, [x29, #-0xa0]
10020a224: ad0e8fe2    	stp	q2, q3, [sp, #0x1d0]
10020a228: ad1003e5    	stp	q5, q0, [sp, #0x200]
10020a22c: 3d807fe4    	str	q4, [sp, #0x1f0]
10020a230: ad4f87e0    	ldp	q0, q1, [sp, #0x1f0]
10020a234: 3c8282a0    	stur	q0, [x21, #0x28]
10020a238: 3c8382a1    	stur	q1, [x21, #0x38]
10020a23c: ad5087e0    	ldp	q0, q1, [sp, #0x210]
10020a240: 3c8482a0    	stur	q0, [x21, #0x48]
10020a244: 3c8582a1    	stur	q1, [x21, #0x58]
10020a248: ad4e87e0    	ldp	q0, q1, [sp, #0x1d0]
10020a24c: 3c8082a0    	stur	q0, [x21, #0x8]
10020a250: 3c8182a1    	stur	q1, [x21, #0x18]
10020a254: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10020a258: f90002a8    	str	x8, [x21]
10020a25c: 1400016e    	b	0x10020a814 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0xb28>
10020a260: 910143fb    	add	x27, sp, #0x50
10020a264: 3cc28340    	ldur	q0, [x26, #0x28]
10020a268: 3cc38341    	ldur	q1, [x26, #0x38]
10020a26c: ad1287e0    	stp	q0, q1, [sp, #0x250]
10020a270: 3cc48340    	ldur	q0, [x26, #0x48]
10020a274: 3cc58341    	ldur	q1, [x26, #0x58]
10020a278: ad1387e0    	stp	q0, q1, [sp, #0x270]
10020a27c: 3cc08340    	ldur	q0, [x26, #0x8]
10020a280: 3cc18341    	ldur	q1, [x26, #0x18]
10020a284: 910a43e8    	add	x8, sp, #0x290
10020a288: 910143e9    	add	x9, sp, #0x50
10020a28c: ad1187e0    	stp	q0, q1, [sp, #0x230]
10020a290: 9101a120    	add	x0, x9, #0x68
10020a294: 9101a101    	add	x1, x8, #0x68
10020a298: 52802302    	mov	w2, #0x118              ; =280
10020a29c: 943f1d04    	bl	0x1011d16ac <_writev+0x1011d16ac>
10020a2a0: ad5183e1    	ldp	q1, q0, [sp, #0x230]
10020a2a4: ad0e83e1    	stp	q1, q0, [sp, #0x1d0]
10020a2a8: ad538be3    	ldp	q3, q2, [sp, #0x270]
10020a2ac: ad108be3    	stp	q3, q2, [sp, #0x210]
10020a2b0: ad5293e5    	ldp	q5, q4, [sp, #0x250]
10020a2b4: ad0f93e5    	stp	q5, q4, [sp, #0x1f0]
10020a2b8: 3c8683e0    	stur	q0, [sp, #0x68]
10020a2bc: 3c8583e1    	stur	q1, [sp, #0x58]
10020a2c0: 3c858362    	stur	q2, [x27, #0x58]
10020a2c4: 3c848363    	stur	q3, [x27, #0x48]
10020a2c8: 3c8883e4    	stur	q4, [sp, #0x88]
10020a2cc: 3c8783e5    	stur	q5, [sp, #0x78]
10020a2d0: f9002bf7    	str	x23, [sp, #0x50]
10020a2d4: 910a43e0    	add	x0, sp, #0x290
10020a2d8: 910143e1    	add	x1, sp, #0x50
10020a2dc: 97ffec44    	bl	0x1002053ec <__ZN13codexmate_lib4core5relay7storage20validate_credentials17hc17aa034e8eca4ebE>
10020a2e0: f9414be8    	ldr	x8, [sp, #0x290]
10020a2e4: f1002d1f    	cmp	x8, #0xb
10020a2e8: 54000cc1    	b.ne	0x10020a480 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x794>
10020a2ec: a9458be1    	ldp	x1, x2, [sp, #0x58]
10020a2f0: 910a43e0    	add	x0, sp, #0x290
10020a2f4: 97fff88f    	bl	0x100208530 <__ZN13codexmate_lib4core5relay7storage27plan_provider_id_migrations17hc8d1d789c02b4ebfE>
10020a2f8: 3cc08320    	ldur	q0, [x25, #0x8]
10020a2fc: 3c9403a0    	stur	q0, [x29, #-0xc0]
10020a300: f9414be8    	ldr	x8, [sp, #0x290]
10020a304: f94157e9    	ldr	x9, [sp, #0x2a8]
10020a308: f81503a9    	stur	x9, [x29, #-0xb0]
10020a30c: f1002d1f    	cmp	x8, #0xb
10020a310: 54000d01    	b.ne	0x10020a4b0 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x7c4>
10020a314: 3cd403a0    	ldur	q0, [x29, #-0xc0]
10020a318: 3d8107e0    	str	q0, [sp, #0x410]
10020a31c: f85503bb    	ldur	x27, [x29, #-0xb0]
10020a320: f90213fb    	str	x27, [sp, #0x420]
10020a324: b40006fb    	cbz	x27, 0x10020a400 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x714>
10020a328: f94126c1    	ldr	x1, [x22, #0x248]
10020a32c: f9412ac2    	ldr	x2, [x22, #0x250]
10020a330: 910a43e0    	add	x0, sp, #0x290
10020a334: aa1403e3    	mov	x3, x20
10020a338: aa1803e4    	mov	x4, x24
10020a33c: 97fffd08    	bl	0x10020975c <__ZN13codexmate_lib4core5relay7storage43preserve_state_before_provider_id_migration17h1233478817d1963bE>
10020a340: f9414be8    	ldr	x8, [sp, #0x290]
10020a344: f9414ff8    	ldr	x24, [sp, #0x298]
10020a348: f94153f9    	ldr	x25, [sp, #0x2a0]
10020a34c: f94157f7    	ldr	x23, [sp, #0x2a8]
10020a350: f1002d1f    	cmp	x8, #0xb
10020a354: 54001681    	b.ne	0x10020a624 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x938>
10020a358: f9420fe2    	ldr	x2, [sp, #0x418]
10020a35c: 910143e1    	add	x1, sp, #0x50
10020a360: aa1603e0    	mov	x0, x22
10020a364: aa1b03e3    	mov	x3, x27
10020a368: 97fffa4e    	bl	0x100208ca0 <__ZN13codexmate_lib4core5relay7storage28apply_provider_id_migrations17hba1e798428cf4899E>
10020a36c: 9000b748    	adrp	x8, 0x1018f2000 <__ZN13codexmate_lib13EXIT_APPROVED17h1f9ee2eac97195fdE.0>
10020a370: 910fe108    	add	x8, x8, #0x3f8
10020a374: f9400108    	ldr	x8, [x8]
10020a378: f100051f    	cmp	x8, #0x1
10020a37c: 54000389    	b.ls	0x10020a3ec <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x700>
10020a380: f900ebfb    	str	x27, [sp, #0x1d0]
10020a384: f9011bf9    	str	x25, [sp, #0x230]
10020a388: f9011ff7    	str	x23, [sp, #0x238]
10020a38c: b0007b88    	adrp	x8, 0x10117b000 <__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i16$GT$3fmt17had97750c0db30cd3E+0x14>
10020a390: 910d3108    	add	x8, x8, #0x34c
10020a394: 910743e9    	add	x9, sp, #0x1d0
10020a398: a93423a9    	stp	x9, x8, [x29, #-0xc0]
10020a39c: 9108c3e8    	add	x8, sp, #0x230
10020a3a0: 90007aa9    	adrp	x9, 0x10115e000 <__ZN4core5slice4sort8unstable9quicksort9quicksort17h324c95ad922d2016E+0x1fc>
10020a3a4: 91351129    	add	x9, x9, #0xd44
10020a3a8: a93527a8    	stp	x8, x9, [x29, #-0xb0]
10020a3ac: 900087e8    	adrp	x8, 0x101306000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.305+0x2bd>
10020a3b0: 91128908    	add	x8, x8, #0x4a2
10020a3b4: 52800469    	mov	w9, #0x23               ; =35
10020a3b8: f9014be8    	str	x8, [sp, #0x290]
10020a3bc: f9014fe9    	str	x9, [sp, #0x298]
10020a3c0: f90153e8    	str	x8, [sp, #0x2a0]
10020a3c4: f90157e9    	str	x9, [sp, #0x2a8]
10020a3c8: b000b068    	adrp	x8, 0x101817000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.56+0x2d8>
10020a3cc: 91128108    	add	x8, x8, #0x4a0
10020a3d0: f9015be8    	str	x8, [sp, #0x2b0]
10020a3d4: f0009880    	adrp	x0, 0x10151d000 <_anon.dc5b8adae996cb58fd4173e320c2d1cf.283+0x7>
10020a3d8: 91276400    	add	x0, x0, #0x9d9
10020a3dc: d10303a1    	sub	x1, x29, #0xc0
10020a3e0: 910a43e3    	add	x3, sp, #0x290
10020a3e4: 52800042    	mov	w2, #0x2                ; =2
10020a3e8: 9414de4f    	bl	0x100741d24 <__ZN3log13__private_api3log17h9a1c73faaf1d7afbE>
10020a3ec: b40000b8    	cbz	x24, 0x10020a400 <__ZN13codexmate_lib4core5relay7storage4load17h01ad0305738b14dbE+0x714>
10020a3f0: aa1903e0    	mov	x0, x25
10020a3f4: aa1803e1    	mov	x1, x24
10020a3f8: 52800022    	mov	w2, #0x1                ; =1
10020a3fc: 97f7dc5b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
