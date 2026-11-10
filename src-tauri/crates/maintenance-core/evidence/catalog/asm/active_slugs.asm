
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001007a17cc <__ZN13codexmate_lib4core5relay6models30active_codex_relay_model_slugs17h72c809e2cf5f3b42E>:
1007a17cc: d10503ff    	sub	sp, sp, #0x140
1007a17d0: a9116ffc    	stp	x28, x27, [sp, #0x110]
1007a17d4: a9124ff4    	stp	x20, x19, [sp, #0x120]
1007a17d8: a9137bfd    	stp	x29, x30, [sp, #0x130]
1007a17dc: 9104c3fd    	add	x29, sp, #0x130
1007a17e0: aa0803f3    	mov	x19, x8
1007a17e4: 52800308    	mov	w8, #0x18               ; =24
1007a17e8: 9b080868    	madd	x8, x3, x8, x2
1007a17ec: a90023e2    	stp	x2, x8, [sp]
1007a17f0: a90107e0    	stp	x0, x1, [sp, #0x10]
1007a17f4: f90013ff    	str	xzr, [sp, #0x20]
1007a17f8: f90027ff    	str	xzr, [sp, #0x48]
1007a17fc: 90008a80    	adrp	x0, 0x1018f1000 <__ZN3std12backtrace_rs9symbolize5gimli5Cache11with_global14MAPPINGS_CACHE17hb3a3380e67302d28E>
1007a1800: 912f6000    	add	x0, x0, #0xbd8
1007a1804: f9400008    	ldr	x8, [x0]
1007a1808: d63f0100    	blr	x8
1007a180c: aa0003f4    	mov	x20, x0
1007a1810: 39404008    	ldrb	w8, [x0, #0x10]
1007a1814: 7100051f    	cmp	w8, #0x1
1007a1818: 540003a1    	b.ne	0x1007a188c <__ZN13codexmate_lib4core5relay6models30active_codex_relay_model_slugs17h72c809e2cf5f3b42E+0xc0>
1007a181c: a9400680    	ldp	x0, x1, [x20]
1007a1820: 91000408    	add	x8, x0, #0x1
1007a1824: f9000288    	str	x8, [x20]
1007a1828: b0008448    	adrp	x8, 0x10182a000 <_anon.cdb272158a04aae91ce02c6e47f106c7.1+0x6d0>
1007a182c: 91212108    	add	x8, x8, #0x848
1007a1830: ad400500    	ldp	q0, q1, [x8]
1007a1834: ad0387e0    	stp	q0, q1, [sp, #0x70]
1007a1838: a90907e0    	stp	x0, x1, [sp, #0x90]
1007a183c: ad4207e0    	ldp	q0, q1, [sp, #0x40]
1007a1840: ad3d87a0    	stp	q0, q1, [x29, #-0x50]
1007a1844: 3dc01be0    	ldr	q0, [sp, #0x60]
1007a1848: 3c9d03a0    	stur	q0, [x29, #-0x30]
1007a184c: ad4007e0    	ldp	q0, q1, [sp]
1007a1850: ad3b87a0    	stp	q0, q1, [x29, #-0x90]
1007a1854: ad4103e1    	ldp	q1, q0, [sp, #0x20]
1007a1858: ad3c83a1    	stp	q1, q0, [x29, #-0x70]
1007a185c: 9101c3e0    	add	x0, sp, #0x70
1007a1860: d10243a1    	sub	x1, x29, #0x90
1007a1864: 97f67a41    	bl	0x100540168 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE>
1007a1868: ad4387e0    	ldp	q0, q1, [sp, #0x70]
1007a186c: ad000660    	stp	q0, q1, [x19]
1007a1870: 3dc027e0    	ldr	q0, [sp, #0x90]
1007a1874: 3d800a60    	str	q0, [x19, #0x20]
1007a1878: a9537bfd    	ldp	x29, x30, [sp, #0x130]
1007a187c: a9524ff4    	ldp	x20, x19, [sp, #0x120]
1007a1880: a9516ffc    	ldp	x28, x27, [sp, #0x110]
1007a1884: 910503ff    	add	sp, sp, #0x140
1007a1888: d65f03c0    	ret
1007a188c: 9426a702    	bl	0x10114b494 <__ZN3std3sys6random19hashmap_random_keys17h8d42fafffd67301dE>
1007a1890: a9000680    	stp	x0, x1, [x20]
1007a1894: 52800028    	mov	w8, #0x1                ; =1
1007a1898: 39004288    	strb	w8, [x20, #0x10]
1007a189c: 17ffffe1    	b	0x1007a1820 <__ZN13codexmate_lib4core5relay6models30active_codex_relay_model_slugs17h72c809e2cf5f3b42E+0x54>
1007a18a0: aa0003f3    	mov	x19, x0
1007a18a4: 910003e0    	mov	x0, sp
1007a18a8: 94013541    	bl	0x1007eedac <__ZN4core3ptr624drop_in_place$LT$core..iter..adapters..flatten..FlatMap$LT$core..iter..adapters..filter_map..FilterMap$LT$core..slice..iter..Iter$LT$alloc..string..String$GT$$C$codexmate_lib..core..relay..models..active_codex_relay_model_slugs..$u7b$$u7b$closure$u7d$$u7d$$GT$$C$core..iter..adapters..map..Map$LT$alloc..vec..into_iter..IntoIter$LT$codexmate_lib..core..relay..models..RelayModelEntry$GT$$C$codexmate_lib..core..relay..models..active_codex_relay_model_slugs..$u7b$$u7b$closure$u7d$$u7d$..$u7b$$u7b$closure$u7d$$u7d$$GT$$C$codexmate_lib..core..relay..models..active_codex_relay_model_slugs..$u7b$$u7b$closure$u7d$$u7d$$GT$$GT$17h0a82a6dfffe8581cE>
1007a18ac: 14000004    	b	0x1007a18bc <__ZN13codexmate_lib4core5relay6models30active_codex_relay_model_slugs17h72c809e2cf5f3b42E+0xf0>
1007a18b0: aa0003f3    	mov	x19, x0
1007a18b4: 9101c3e0    	add	x0, sp, #0x70
1007a18b8: 94033754    	bl	0x10086f608 <__ZN79_$LT$hashbrown..raw..RawTable$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5856a37aee37aca3E>
1007a18bc: aa1303e0    	mov	x0, x19
1007a18c0: 9428be25    	bl	0x1011d1154 <_writev+0x1011d1154>
1007a18c4: 9428bca1    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

00000001007a18c8 <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE>:
1007a18c8: a9bd57f6    	stp	x22, x21, [sp, #-0x30]!
1007a18cc: a9014ff4    	stp	x20, x19, [sp, #0x10]
1007a18d0: a9027bfd    	stp	x29, x30, [sp, #0x20]
1007a18d4: 910083fd    	add	x29, sp, #0x20
1007a18d8: aa0003f3    	mov	x19, x0
1007a18dc: f0006041    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
1007a18e0: 91173021    	add	x1, x1, #0x5cc
1007a18e4: 52800222    	mov	w2, #0x11               ; =17
1007a18e8: 94258d01    	bl	0x101104cec <__ZN10serde_json5value5Value7pointer17h84916003af215a14E>
1007a18ec: b40001a0    	cbz	x0, 0x1007a1920 <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0x58>
1007a18f0: 39400008    	ldrb	w8, [x0]
1007a18f4: 71000d1f    	cmp	w8, #0x3
1007a18f8: 54000141    	b.ne	0x1007a1920 <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0x58>
1007a18fc: a9410408    	ldp	x8, x1, [x0, #0x10]
1007a1900: aa0803e0    	mov	x0, x8
1007a1904: 97ea3615    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1007a1908: aa0103f3    	mov	x19, x1
1007a190c: b6f80221    	tbz	x1, #0x3f, 0x1007a1950 <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0x88>
1007a1910: d2800015    	mov	x21, #0x0               ; =0
1007a1914: aa1503e0    	mov	x0, x21
1007a1918: aa1303e1    	mov	x1, x19
1007a191c: 9428bb52    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
1007a1920: f0006040    	adrp	x0, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
1007a1924: 91178000    	add	x0, x0, #0x5e0
1007a1928: 52800121    	mov	w1, #0x9                ; =9
1007a192c: aa1303e2    	mov	x2, x19
1007a1930: 9425b498    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
1007a1934: b40000a0    	cbz	x0, 0x1007a1948 <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0x80>
1007a1938: 39400008    	ldrb	w8, [x0]
1007a193c: 7100011f    	cmp	w8, #0x0
1007a1940: 1a9f07f5    	cset	w21, ne
1007a1944: 1400007a    	b	0x1007a1b2c <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0x264>
1007a1948: 52800055    	mov	w21, #0x2               ; =2
1007a194c: 14000078    	b	0x1007a1b2c <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0x264>
1007a1950: aa0003f6    	mov	x22, x0
1007a1954: b40001f3    	cbz	x19, 0x1007a1990 <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0xc8>
1007a1958: 97e17f07    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1007a195c: 52800035    	mov	w21, #0x1               ; =1
1007a1960: aa1303e0    	mov	x0, x19
1007a1964: 52800021    	mov	w1, #0x1                ; =1
1007a1968: 97e17eff    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1007a196c: b4fffd40    	cbz	x0, 0x1007a1914 <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0x4c>
1007a1970: aa0003f4    	mov	x20, x0
1007a1974: aa1603e1    	mov	x1, x22
1007a1978: aa1303e2    	mov	x2, x19
1007a197c: 9428bf4c    	bl	0x1011d16ac <_writev+0x1011d16ac>
1007a1980: f100227f    	cmp	x19, #0x8
1007a1984: 54000122    	b.hs	0x1007a19a8 <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0xe0>
1007a1988: d2800008    	mov	x8, #0x0                ; =0
1007a198c: 1400003c    	b	0x1007a1a7c <__ZN13codexmate_lib4core5relay8dialects19reasoning_requested17hefcaed0d8560940bE+0x1b4>
