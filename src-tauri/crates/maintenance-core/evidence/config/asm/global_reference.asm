
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100395fb8 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E>:
100395fb8: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
100395fbc: a90167fa    	stp	x26, x25, [sp, #0x10]
100395fc0: a9025ff8    	stp	x24, x23, [sp, #0x20]
100395fc4: a90357f6    	stp	x22, x21, [sp, #0x30]
100395fc8: a9044ff4    	stp	x20, x19, [sp, #0x40]
100395fcc: a9057bfd    	stp	x29, x30, [sp, #0x50]
100395fd0: 910143fd    	add	x29, sp, #0x50
100395fd4: d108c3ff    	sub	sp, sp, #0x230
100395fd8: aa0403f4    	mov	x20, x4
100395fdc: aa0303f5    	mov	x21, x3
100395fe0: aa0003f3    	mov	x19, x0
100395fe4: 910103e8    	add	x8, sp, #0x40
100395fe8: aa0103e0    	mov	x0, x1
100395fec: aa0203e1    	mov	x1, x2
100395ff0: 941f7869    	bl	0x100b74194 <__ZN74_$LT$toml_edit..de..Deserializer$u20$as$u20$core..str..traits..FromStr$GT$8from_str17h13cc4747bad7bb97E>
100395ff4: a94427ea    	ldp	x10, x9, [sp, #0x40]
100395ff8: 394143e8    	ldrb	w8, [sp, #0x50]
100395ffc: f84513eb    	ldur	x11, [sp, #0x51]
100396000: f9001beb    	str	x11, [sp, #0x30]
100396004: a94587eb    	ldp	x11, x1, [sp, #0x58]
100396008: f80373eb    	stur	x11, [sp, #0x37]
10039600c: f94037e0    	ldr	x0, [sp, #0x68]
100396010: a947dbf9    	ldp	x25, x22, [sp, #0x78]
100396014: a948dffa    	ldp	x26, x23, [sp, #0x88]
100396018: f9404ff8    	ldr	x24, [sp, #0x98]
10039601c: f100095f    	cmp	x10, #0x2
100396020: 54000181    	b.ne	0x100396050 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x98>
100396024: f9401bea    	ldr	x10, [sp, #0x30]
100396028: f90013ea    	str	x10, [sp, #0x20]
10039602c: f84373ea    	ldur	x10, [sp, #0x37]
100396030: f80273ea    	stur	x10, [sp, #0x27]
100396034: f94013ea    	ldr	x10, [sp, #0x20]
100396038: f9000bea    	str	x10, [sp, #0x10]
10039603c: f84273ea    	ldur	x10, [sp, #0x27]
100396040: f80173ea    	stur	x10, [sp, #0x17]
100396044: f100093f    	cmp	x9, #0x2
100396048: 54000661    	b.ne	0x100396114 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x15c>
10039604c: 1400003e    	b	0x100396144 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x18c>
100396050: 9104e3eb    	add	x11, sp, #0x138
100396054: 910103ec    	add	x12, sp, #0x40
100396058: f94053ed    	ldr	x13, [sp, #0xa0]
10039605c: f9403bee    	ldr	x14, [sp, #0x70]
100396060: 3ccb8180    	ldur	q0, [x12, #0xb8]
100396064: 3c8b8160    	stur	q0, [x11, #0xb8]
100396068: 3ccc8180    	ldur	q0, [x12, #0xc8]
10039606c: 3c8c8160    	stur	q0, [x11, #0xc8]
100396070: 3ccd8180    	ldur	q0, [x12, #0xd8]
100396074: 3c8d8160    	stur	q0, [x11, #0xd8]
100396078: 3cce8180    	ldur	q0, [x12, #0xe8]
10039607c: 3c8e8160    	stur	q0, [x11, #0xe8]
100396080: 3cc78180    	ldur	q0, [x12, #0x78]
100396084: 3c878160    	stur	q0, [x11, #0x78]
100396088: 3cc88180    	ldur	q0, [x12, #0x88]
10039608c: 3c888160    	stur	q0, [x11, #0x88]
100396090: 3cc98180    	ldur	q0, [x12, #0x98]
100396094: 3c898160    	stur	q0, [x11, #0x98]
100396098: 3cca8180    	ldur	q0, [x12, #0xa8]
10039609c: 3c8a8160    	stur	q0, [x11, #0xa8]
1003960a0: 3cc68180    	ldur	q0, [x12, #0x68]
1003960a4: 3c868160    	stur	q0, [x11, #0x68]
1003960a8: f9401bec    	ldr	x12, [sp, #0x30]
1003960ac: f90013ec    	str	x12, [sp, #0x20]
1003960b0: f84373ec    	ldur	x12, [sp, #0x37]
1003960b4: f80273ec    	stur	x12, [sp, #0x27]
1003960b8: a913a7ea    	stp	x10, x9, [sp, #0x138]
1003960bc: 390523e8    	strb	w8, [sp, #0x148]
1003960c0: f94013e8    	ldr	x8, [sp, #0x20]
1003960c4: f8011168    	stur	x8, [x11, #0x11]
1003960c8: a91507ec    	stp	x12, x1, [sp, #0x150]
1003960cc: a9163be0    	stp	x0, x14, [sp, #0x160]
1003960d0: a9175bf9    	stp	x25, x22, [sp, #0x170]
1003960d4: a9185ffa    	stp	x26, x23, [sp, #0x180]
1003960d8: a91937f8    	stp	x24, x13, [sp, #0x190]
1003960dc: 910103e8    	add	x8, sp, #0x40
1003960e0: 9104e3e0    	add	x0, sp, #0x138
1003960e4: 9417e343    	bl	0x10098edf0 <__ZN76_$LT$toml_edit..de..Deserializer$u20$as$u20$serde_core..de..Deserializer$GT$15deserialize_any17h9bb712022c4c455aE>
1003960e8: f94023e9    	ldr	x9, [sp, #0x40]
1003960ec: 394123e8    	ldrb	w8, [sp, #0x48]
1003960f0: f84493ea    	ldur	x10, [sp, #0x49]
1003960f4: f9000bea    	str	x10, [sp, #0x10]
1003960f8: a94507ea    	ldp	x10, x1, [sp, #0x50]
1003960fc: f80173ea    	stur	x10, [sp, #0x17]
100396100: f94033e0    	ldr	x0, [sp, #0x60]
100396104: f100093f    	cmp	x9, #0x2
100396108: 54000141    	b.ne	0x100396130 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x178>
10039610c: f100093f    	cmp	x9, #0x2
100396110: 540001a0    	b.eq	0x100396144 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x18c>
100396114: b4000061    	cbz	x1, 0x100396120 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x168>
100396118: 52800022    	mov	w2, #0x1                ; =1
10039611c: 97f1ad13    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100396120: f240faff    	tst	x23, #0x7fffffffffffffff
100396124: 54000861    	b.ne	0x100396230 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x278>
100396128: b50008fa    	cbnz	x26, 0x100396244 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x28c>
10039612c: 14000051    	b	0x100396270 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x2b8>
100396130: a9475bf9    	ldp	x25, x22, [sp, #0x70]
100396134: a9485ffa    	ldp	x26, x23, [sp, #0x80]
100396138: f9404bf8    	ldr	x24, [sp, #0x90]
10039613c: f100093f    	cmp	x9, #0x2
100396140: 54fffea1    	b.ne	0x100396114 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x15c>
100396144: f9400be9    	ldr	x9, [sp, #0x10]
100396148: f90003e9    	str	x9, [sp]
10039614c: f84173e9    	ldur	x9, [sp, #0x17]
100396150: f80073e9    	stur	x9, [sp, #0x7]
100396154: 71001d1f    	cmp	w8, #0x7
100396158: 54000980    	b.eq	0x100396288 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x2d0>
10039615c: f94003e9    	ldr	x9, [sp]
100396160: f80413e9    	stur	x9, [sp, #0x41]
100396164: f84073e9    	ldur	x9, [sp, #0x7]
100396168: a90487e9    	stp	x9, x1, [sp, #0x48]
10039616c: f9002fe0    	str	x0, [sp, #0x58]
100396170: 390103e8    	strb	w8, [sp, #0x40]
100396174: b0007bc0    	adrp	x0, 0x10130f000 <_anon.164d03a2dfc98f969363eaf1c50504fd.640+0xf08>
100396178: 9118b000    	add	x0, x0, #0x62c
10039617c: 910103e2    	add	x2, sp, #0x40
100396180: 528002c1    	mov	w1, #0x16               ; =22
100396184: 941b3e64    	bl	0x100a65b14 <__ZN42_$LT$str$u20$as$u20$toml..value..Index$GT$5index17h27c7cfcb38a2f81dE>
100396188: b40000a0    	cbz	x0, 0x10039619c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x1e4>
10039618c: 39400008    	ldrb	w8, [x0]
100396190: 35000068    	cbnz	w8, 0x10039619c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x1e4>
100396194: f9400c08    	ldr	x8, [x0, #0x18]
100396198: b40008c8    	cbz	x8, 0x1003962b0 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x2f8>
10039619c: b0007ba0    	adrp	x0, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
1003961a0: 912b3c00    	add	x0, x0, #0xacf
1003961a4: 910103e2    	add	x2, sp, #0x40
1003961a8: 52800241    	mov	w1, #0x12               ; =18
1003961ac: 941b3e5a    	bl	0x100a65b14 <__ZN42_$LT$str$u20$as$u20$toml..value..Index$GT$5index17h27c7cfcb38a2f81dE>
1003961b0: b4000360    	cbz	x0, 0x10039621c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x264>
1003961b4: 39400008    	ldrb	w8, [x0]
1003961b8: 35000328    	cbnz	w8, 0x10039621c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x264>
1003961bc: a9415c16    	ldp	x22, x23, [x0, #0x10]
1003961c0: aa1603e0    	mov	x0, x22
1003961c4: aa1703e1    	mov	x1, x23
1003961c8: 9436e261    	bl	0x10114eb4c <__ZN3std4path4Path11is_absolute17h236556a872cb0c37E>
1003961cc: 34000280    	cbz	w0, 0x10039621c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x264>
1003961d0: aa1603e0    	mov	x0, x22
1003961d4: aa1703e1    	mov	x1, x23
1003961d8: aa1503e2    	mov	x2, x21
1003961dc: aa1403e3    	mov	x3, x20
1003961e0: 97ff70b6    	bl	0x1003724b8 <__ZN13codexmate_lib4core5relay16codex_diagnostic17path_is_under_dir17h23434ea2725d7a60E>
1003961e4: 370001c0    	tbnz	w0, #0x0, 0x10039621c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x264>
1003961e8: 9104e3e8    	add	x8, sp, #0x138
1003961ec: aa1603e0    	mov	x0, x22
1003961f0: aa1703e1    	mov	x1, x23
1003961f4: 9436c901    	bl	0x1011485f8 <__ZN3std3sys2fs8metadata17h62db632ff6f3f8f6E>
1003961f8: b9413be8    	ldr	w8, [sp, #0x138]
1003961fc: 36000108    	tbz	w8, #0x0, 0x10039621c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x264>
100396200: f940a3f4    	ldr	x20, [sp, #0x140]
100396204: aa1403e0    	mov	x0, x20
100396208: 9400f3e5    	bl	0x1003d319c <__ZN3std2io5error5Error4kind17h247456c019dc3d6bE>
10039620c: 72001c1f    	tst	w0, #0xff
100396210: 54000720    	b.eq	0x1003962f4 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x33c>
100396214: aa1403e0    	mov	x0, x20
100396218: 940103a2    	bl	0x1003d70a0 <__ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h8fc6649f423239fcE>
10039621c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100396220: f9000268    	str	x8, [x19]
100396224: 910103e0    	add	x0, sp, #0x40
100396228: 9401027f    	bl	0x1003d6c24 <__ZN4core3ptr39drop_in_place$LT$toml..value..Value$GT$17h53736b29afe70b00E>
10039622c: 14000019    	b	0x100396290 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x2d8>
100396230: aa1803e0    	mov	x0, x24
100396234: aa1703e1    	mov	x1, x23
100396238: 52800022    	mov	w2, #0x1                ; =1
10039623c: 97f1accb    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100396240: b400019a    	cbz	x26, 0x100396270 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x2b8>
100396244: 910022d4    	add	x20, x22, #0x8
100396248: 14000004    	b	0x100396258 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x2a0>
10039624c: 91006294    	add	x20, x20, #0x18
100396250: f100075a    	subs	x26, x26, #0x1
100396254: 540000e0    	b.eq	0x100396270 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x2b8>
100396258: f85f8281    	ldur	x1, [x20, #-0x8]
10039625c: b4ffff81    	cbz	x1, 0x10039624c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x294>
100396260: f9400280    	ldr	x0, [x20]
100396264: 52800022    	mov	w2, #0x1                ; =1
100396268: 97f1acc0    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10039626c: 17fffff8    	b	0x10039624c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x294>
100396270: b40000d9    	cbz	x25, 0x100396288 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x2d0>
100396274: 8b190728    	add	x8, x25, x25, lsl #1
100396278: d37df101    	lsl	x1, x8, #3
10039627c: aa1603e0    	mov	x0, x22
100396280: 52800102    	mov	w2, #0x8                ; =8
100396284: 97f1acb9    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100396288: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10039628c: f9000268    	str	x8, [x19]
100396290: 9108c3ff    	add	sp, sp, #0x230
100396294: a9457bfd    	ldp	x29, x30, [sp, #0x50]
100396298: a9444ff4    	ldp	x20, x19, [sp, #0x40]
10039629c: a94357f6    	ldp	x22, x21, [sp, #0x30]
1003962a0: a9425ff8    	ldp	x24, x23, [sp, #0x20]
1003962a4: a94167fa    	ldp	x26, x25, [sp, #0x10]
1003962a8: a8c66ffc    	ldp	x28, x27, [sp], #0x60
1003962ac: d65f03c0    	ret
1003962b0: 97f1acb1    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1003962b4: 52800474    	mov	w20, #0x23              ; =35
1003962b8: 52800460    	mov	w0, #0x23               ; =35
1003962bc: 52800021    	mov	w1, #0x1                ; =1
1003962c0: 97f1aca9    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1003962c4: b40003c0    	cbz	x0, 0x10039633c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x384>
1003962c8: 529cf748    	mov	w8, #0xe7ba             ; =59322
1003962cc: 72b75528    	movk	w8, #0xbaa9, lsl #16
1003962d0: b0007bc9    	adrp	x9, 0x10130f000 <_anon.164d03a2dfc98f969363eaf1c50504fd.640+0xf08>
1003962d4: 91190929    	add	x9, x9, #0x642
1003962d8: b801f008    	stur	w8, [x0, #0x1f]
1003962dc: ad400520    	ldp	q0, q1, [x9]
1003962e0: ad000400    	stp	q0, q1, [x0]
1003962e4: 52800468    	mov	w8, #0x23               ; =35
1003962e8: a9000268    	stp	x8, x0, [x19]
1003962ec: f9000a68    	str	x8, [x19, #0x10]
1003962f0: 17ffffcd    	b	0x100396224 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x26c>
1003962f4: aa1403e0    	mov	x0, x20
1003962f8: 9401036a    	bl	0x1003d70a0 <__ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h8fc6649f423239fcE>
1003962fc: 97f1ac9e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100396300: 52800634    	mov	w20, #0x31              ; =49
100396304: 52800620    	mov	w0, #0x31               ; =49
100396308: 52800021    	mov	w1, #0x1                ; =1
10039630c: 97f1ac96    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100396310: b4000160    	cbz	x0, 0x10039633c <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x384>
100396314: 52801088    	mov	w8, #0x84               ; =132
100396318: 3900c008    	strb	w8, [x0, #0x30]
10039631c: b0007bc8    	adrp	x8, 0x10130f000 <_anon.164d03a2dfc98f969363eaf1c50504fd.640+0xf08>
100396320: 91199508    	add	x8, x8, #0x665
100396324: ad400500    	ldp	q0, q1, [x8]
100396328: ad000400    	stp	q0, q1, [x0]
10039632c: 3dc00900    	ldr	q0, [x8, #0x20]
100396330: 3d800800    	str	q0, [x0, #0x20]
100396334: 52800628    	mov	w8, #0x31               ; =49
100396338: 17ffffec    	b	0x1003962e8 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x330>
10039633c: 52800020    	mov	w0, #0x1                ; =1
100396340: aa1403e1    	mov	x1, x20
100396344: 9438e8c8    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100396348: d4200020    	brk	#0x1
10039634c: 14000001    	b	0x100396350 <__ZN13codexmate_lib4core5relay16codex_diagnostic31invalid_global_config_reference17hf6a547c86adebe18E+0x398>
100396350: aa0003f3    	mov	x19, x0
100396354: 910103e0    	add	x0, sp, #0x40
100396358: 94010233    	bl	0x1003d6c24 <__ZN4core3ptr39drop_in_place$LT$toml..value..Value$GT$17h53736b29afe70b00E>
10039635c: aa1303e0    	mov	x0, x19
100396360: 9438eb7d    	bl	0x1011d1154 <_writev+0x1011d1154>
100396364: 9438e9f9    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

