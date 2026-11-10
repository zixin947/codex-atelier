
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100540168 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE>:
100540168: a9be4ff4    	stp	x20, x19, [sp, #-0x20]!
10054016c: a9017bfd    	stp	x29, x30, [sp, #0x10]
100540170: 910043fd    	add	x29, sp, #0x10
100540174: aa0103f3    	mov	x19, x1
100540178: aa0003f4    	mov	x20, x0
10054017c: f9400c09    	ldr	x9, [x0, #0x18]
100540180: f9401028    	ldr	x8, [x1, #0x20]
100540184: b4000329    	cbz	x9, 0x1005401e8 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0x80>
100540188: b4000648    	cbz	x8, 0x100540250 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0xe8>
10054018c: f9401668    	ldr	x8, [x19, #0x28]
100540190: f9401e69    	ldr	x9, [x19, #0x38]
100540194: cb080128    	sub	x8, x9, x8
100540198: d345fd08    	lsr	x8, x8, #5
10054019c: b201f3e9    	mov	x9, #-0x5555555555555556 ; =-6148914691236517206
1005401a0: f2955569    	movk	x9, #0xaaab
1005401a4: d280002a    	mov	x10, #0x1               ; =1
1005401a8: 9b092908    	madd	x8, x8, x9, x10
1005401ac: f9402669    	ldr	x9, [x19, #0x48]
1005401b0: b4000109    	cbz	x9, 0x1005401d0 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0x68>
1005401b4: f9402a69    	ldr	x9, [x19, #0x50]
1005401b8: f940326a    	ldr	x10, [x19, #0x60]
1005401bc: cb090149    	sub	x9, x10, x9
1005401c0: d345fd29    	lsr	x9, x9, #5
1005401c4: b201f3ea    	mov	x10, #-0x5555555555555556 ; =-6148914691236517206
1005401c8: f295556a    	movk	x10, #0xaaab
1005401cc: 9b0a7d29    	mul	x9, x9, x10
1005401d0: 8b080128    	add	x8, x9, x8
1005401d4: d341fd01    	lsr	x1, x8, #1
1005401d8: f9400a88    	ldr	x8, [x20, #0x10]
1005401dc: eb08003f    	cmp	x1, x8
1005401e0: 540002e9    	b.ls	0x10054023c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0xd4>
1005401e4: 1400001f    	b	0x100540260 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0xf8>
1005401e8: b4000108    	cbz	x8, 0x100540208 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0xa0>
1005401ec: f9401668    	ldr	x8, [x19, #0x28]
1005401f0: f9401e69    	ldr	x9, [x19, #0x38]
1005401f4: cb080128    	sub	x8, x9, x8
1005401f8: d345fd08    	lsr	x8, x8, #5
1005401fc: b201f3e9    	mov	x9, #-0x5555555555555556 ; =-6148914691236517206
100540200: f2955569    	movk	x9, #0xaaab
100540204: 9b097d08    	mul	x8, x8, x9
100540208: f9402669    	ldr	x9, [x19, #0x48]
10054020c: b4000109    	cbz	x9, 0x10054022c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0xc4>
100540210: f9402a69    	ldr	x9, [x19, #0x50]
100540214: f940326a    	ldr	x10, [x19, #0x60]
100540218: cb090149    	sub	x9, x10, x9
10054021c: d345fd29    	lsr	x9, x9, #5
100540220: b201f3ea    	mov	x10, #-0x5555555555555556 ; =-6148914691236517206
100540224: f295556a    	movk	x10, #0xaaab
100540228: 9b0a7d29    	mul	x9, x9, x10
10054022c: 8b080121    	add	x1, x9, x8
100540230: f9400a88    	ldr	x8, [x20, #0x10]
100540234: eb08003f    	cmp	x1, x8
100540238: 54000148    	b.hi	0x100540260 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0xf8>
10054023c: aa1303e0    	mov	x0, x19
100540240: aa1403e1    	mov	x1, x20
100540244: a9417bfd    	ldp	x29, x30, [sp, #0x10]
100540248: a8c24ff4    	ldp	x20, x19, [sp], #0x20
10054024c: 140b0ffa    	b	0x100804234 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h4fd63908faff6f51E>
100540250: 52800028    	mov	w8, #0x1                ; =1
100540254: f9402669    	ldr	x9, [x19, #0x48]
100540258: b5fffae9    	cbnz	x9, 0x1005401b4 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0x4c>
10054025c: 17ffffdd    	b	0x1005401d0 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0x68>
100540260: 91008282    	add	x2, x20, #0x20
100540264: aa1403e0    	mov	x0, x20
100540268: 52800023    	mov	w3, #0x1                ; =1
10054026c: 9431a150    	bl	0x1011a87ac <__ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h5ca31c0a6fa50467E>
100540270: 17fffff3    	b	0x10054023c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h807803937c151adfE+0xd4>
100540274: aa0003f4    	mov	x20, x0
100540278: aa1303e0    	mov	x0, x19
10054027c: 94003b89    	bl	0x10054f0a0 <__ZN4core3ptr1482drop_in_place$LT$core..iter..adapters..map..Map$LT$core..iter..adapters..flatten..FlatMap$LT$core..iter..adapters..filter_map..FilterMap$LT$core..slice..iter..Iter$LT$alloc..string..String$GT$$C$codexmate_lib..core..relay..models..active_codex_relay_model_slugs..$u7b$$u7b$closure$u7d$$u7d$$GT$$C$core..iter..adapters..map..Map$LT$alloc..vec..into_iter..IntoIter$LT$codexmate_lib..core..relay..models..RelayModelEntry$GT$$C$codexmate_lib..core..relay..models..active_codex_relay_model_slugs..$u7b$$u7b$closure$u7d$$u7d$..$u7b$$u7b$closure$u7d$$u7d$$GT$$C$codexmate_lib..core..relay..models..active_codex_relay_model_slugs..$u7b$$u7b$closure$u7d$$u7d$$GT$$C$$LT$hashbrown..set..HashSet$LT$alloc..string..String$C$std..hash..random..RandomState$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$alloc..string..String$GT$$GT$..extend$LT$core..iter..adapters..flatten..FlatMap$LT$core..iter..adapters..filter_map..FilterMap$LT$core..slice..iter..Iter$LT$alloc..string..String$GT$$C$codexmate_lib..core..relay..models..active_codex_relay_model_slugs..$u7b$$u7b$closure$u7d$$u7d$$GT$$C$core..iter..adapters..map..Map$LT$alloc..vec..into_iter..IntoIter$LT$codexmate_lib..core..relay..models..RelayModelEntry$GT$$C$codexmate_lib..core..relay..models..active_codex_relay_model_slugs..$u7b$$u7b$closure$u7d$$u7d$..$u7b$$u7b$closure$u7d$$u7d$$GT$$C$codexmate_lib..core..relay..models..active_codex_relay_model_slugs..$u7b$$u7b$closure$u7d$$u7d$$GT$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$$GT$17hc251152b23a4d46fE>
100540280: aa1403e0    	mov	x0, x20
100540284: 943243b4    	bl	0x1011d1154 <_writev+0x1011d1154>

0000000100540288 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE>:
100540288: d103c3ff    	sub	sp, sp, #0xf0
10054028c: 6d0823e9    	stp	d9, d8, [sp, #0x80]
100540290: a9096ffc    	stp	x28, x27, [sp, #0x90]
100540294: a90a67fa    	stp	x26, x25, [sp, #0xa0]
100540298: a90b5ff8    	stp	x24, x23, [sp, #0xb0]
10054029c: a90c57f6    	stp	x22, x21, [sp, #0xc0]
1005402a0: a90d4ff4    	stp	x20, x19, [sp, #0xd0]
1005402a4: a90e7bfd    	stp	x29, x30, [sp, #0xe0]
1005402a8: 910383fd    	add	x29, sp, #0xe0
1005402ac: f9000fe0    	str	x0, [sp, #0x18]
1005402b0: a9402033    	ldp	x19, x8, [x1]
1005402b4: eb08027f    	cmp	x19, x8
1005402b8: 54001420    	b.eq	0x10054053c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x2b4>
1005402bc: f9400837    	ldr	x23, [x1, #0x10]
1005402c0: cb130108    	sub	x8, x8, x19
1005402c4: d345fd15    	lsr	x21, x8, #5
1005402c8: f9400ee8    	ldr	x8, [x23, #0x18]
1005402cc: b4001148    	cbz	x8, 0x1005404f4 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x26c>
1005402d0: d2800018    	mov	x24, #0x0               ; =0
1005402d4: a94026e8    	ldp	x8, x9, [x23]
1005402d8: a902a7e8    	stp	x8, x9, [sp, #0x28]
1005402dc: d1006108    	sub	x8, x8, #0x18
1005402e0: f90013e8    	str	x8, [sp, #0x20]
1005402e4: d0006e94    	adrp	x20, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
1005402e8: 913c5294    	add	x20, x20, #0xf14
1005402ec: 14000047    	b	0x100540408 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x180>
1005402f0: a942a7e8    	ldp	x8, x9, [sp, #0x28]
1005402f4: 8a090017    	and	x23, x0, x9
1005402f8: fc776909    	ldr	d9, [x8, x23]
1005402fc: 2e288d20    	cmeq.8b	v0, v9, v8
100540300: 9e660008    	fmov	x8, d0
100540304: f201c114    	ands	x20, x8, #0x8080808080808080
100540308: 540002a0    	b.eq	0x10054035c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0xd4>
10054030c: dac00288    	rbit	x8, x20
100540310: dac01108    	clz	x8, x8
100540314: 8b480ee8    	add	x8, x23, x8, lsr #3
100540318: f9401be9    	ldr	x9, [sp, #0x30]
10054031c: 8a090108    	and	x8, x8, x9
100540320: 52800309    	mov	w9, #0x18               ; =24
100540324: 9b09fd08    	mneg	x8, x8, x9
100540328: f94013e9    	ldr	x9, [sp, #0x20]
10054032c: 8b080128    	add	x8, x9, x8
100540330: f9400909    	ldr	x9, [x8, #0x10]
100540334: eb09035f    	cmp	x26, x9
100540338: 540000c1    	b.ne	0x100540350 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0xc8>
10054033c: f9400501    	ldr	x1, [x8, #0x8]
100540340: aa1b03e0    	mov	x0, x27
100540344: aa1a03e2    	mov	x2, x26
100540348: 943244d6    	bl	0x1011d16a0 <_writev+0x1011d16a0>
10054034c: 34000180    	cbz	w0, 0x10054037c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0xf4>
100540350: d1000688    	sub	x8, x20, #0x1
100540354: ea140114    	ands	x20, x8, x20
100540358: 54fffda1    	b.ne	0x10054030c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x84>
10054035c: 6f07e7e0    	movi.2d	v0, #0xffffffffffffffff
100540360: 2e208d20    	cmeq.8b	v0, v9, v0
100540364: 2e30a800    	umaxv.8b	b0, v0
100540368: 1e260008    	fmov	w8, s0
10054036c: 37000428    	tbnz	w8, #0x0, 0x1005403f0 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x168>
100540370: 91002339    	add	x25, x25, #0x8
100540374: 8b1902e0    	add	x0, x23, x25
100540378: 17ffffde    	b	0x1005402f0 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x68>
10054037c: d0006ec0    	adrp	x0, 0x10131a000 <_anon.b5f536351c4bf1e454cf90283dd9e625.537+0x24f>
100540380: 91029800    	add	x0, x0, #0xa6
100540384: 528002e1    	mov	w1, #0x17               ; =23
100540388: aa1c03e2    	mov	x2, x28
10054038c: 942f3a01    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
100540390: b4000300    	cbz	x0, 0x1005403f0 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x168>
100540394: 39400008    	ldrb	w8, [x0]
100540398: 71000d1f    	cmp	w8, #0x3
10054039c: aa1603f7    	mov	x23, x22
1005403a0: 54000a41    	b.ne	0x1005404e8 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x260>
1005403a4: a9410408    	ldp	x8, x1, [x0, #0x10]
1005403a8: aa0803e0    	mov	x0, x8
1005403ac: 97f3bb6b    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
1005403b0: b40009c1    	cbz	x1, 0x1005404e8 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x260>
1005403b4: b7f80d7a    	tbnz	x26, #0x3f, 0x100540560 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x2d8>
1005403b8: aa0103fc    	mov	x28, x1
1005403bc: aa0003f6    	mov	x22, x0
1005403c0: b400053a    	cbz	x26, 0x100540464 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x1dc>
1005403c4: 97eb046c    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1005403c8: aa1a03e0    	mov	x0, x26
1005403cc: 52800021    	mov	w1, #0x1                ; =1
1005403d0: 97eb0465    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1005403d4: b4000d40    	cbz	x0, 0x10054057c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x2f4>
1005403d8: f9000be0    	str	x0, [sp, #0x10]
1005403dc: aa1b03e1    	mov	x1, x27
1005403e0: aa1a03e2    	mov	x2, x26
1005403e4: 943244b2    	bl	0x1011d16ac <_writev+0x1011d16ac>
1005403e8: b7f80c3c    	tbnz	x28, #0x3f, 0x10054056c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x2e4>
1005403ec: 14000024    	b	0x10054047c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x1f4>
1005403f0: d0006e94    	adrp	x20, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
1005403f4: 913c5294    	add	x20, x20, #0xf14
1005403f8: aa1603f7    	mov	x23, x22
1005403fc: 91000718    	add	x24, x24, #0x1
100540400: eb15031f    	cmp	x24, x21
100540404: 540009c0    	b.eq	0x10054053c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x2b4>
100540408: 8b18167c    	add	x28, x19, x24, lsl #5
10054040c: aa1403e0    	mov	x0, x20
100540410: 52800081    	mov	w1, #0x4                ; =4
100540414: aa1c03e2    	mov	x2, x28
100540418: 942f39de    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
10054041c: b4ffff00    	cbz	x0, 0x1005403fc <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x174>
100540420: 39400008    	ldrb	w8, [x0]
100540424: 71000d1f    	cmp	w8, #0x3
100540428: 54fffea1    	b.ne	0x1005403fc <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x174>
10054042c: a9410408    	ldp	x8, x1, [x0, #0x10]
100540430: aa0803e0    	mov	x0, x8
100540434: 97f3bb49    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100540438: aa0003fb    	mov	x27, x0
10054043c: aa0103fa    	mov	x26, x1
100540440: aa1703f6    	mov	x22, x23
100540444: 910082e0    	add	x0, x23, #0x20
100540448: aa1b03e1    	mov	x1, x27
10054044c: aa1a03e2    	mov	x2, x26
100540450: 940612da    	bl	0x1006c4fb8 <__ZN4core4hash11BuildHasher8hash_one17h61e5c885b8041446E>
100540454: d2800019    	mov	x25, #0x0               ; =0
100540458: d379fc08    	lsr	x8, x0, #57
10054045c: 0e010d08    	dup.8b	v8, w8
100540460: 17ffffa4    	b	0x1005402f0 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x68>
100540464: 52800020    	mov	w0, #0x1                ; =1
100540468: f9000be0    	str	x0, [sp, #0x10]
10054046c: aa1b03e1    	mov	x1, x27
100540470: aa1a03e2    	mov	x2, x26
100540474: 9432448e    	bl	0x1011d16ac <_writev+0x1011d16ac>
100540478: b7f807bc    	tbnz	x28, #0x3f, 0x10054056c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x2e4>
10054047c: 97eb043e    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100540480: aa1c03e0    	mov	x0, x28
100540484: 52800021    	mov	w1, #0x1                ; =1
100540488: 97eb0437    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10054048c: b4000800    	cbz	x0, 0x10054058c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x304>
100540490: aa0003f4    	mov	x20, x0
100540494: aa1603e1    	mov	x1, x22
100540498: aa1c03e2    	mov	x2, x28
10054049c: 94324484    	bl	0x1011d16ac <_writev+0x1011d16ac>
1005404a0: a94103e8    	ldp	x8, x0, [sp, #0x10]
1005404a4: a9046be8    	stp	x8, x26, [sp, #0x40]
1005404a8: f9001ffa    	str	x26, [sp, #0x38]
1005404ac: a90553fc    	stp	x28, x20, [sp, #0x50]
1005404b0: f90007f4    	str	x20, [sp, #0x8]
1005404b4: f90033fc    	str	x28, [sp, #0x60]
1005404b8: 9101a3e8    	add	x8, sp, #0x68
1005404bc: 9100e3e1    	add	x1, sp, #0x38
1005404c0: 910143e2    	add	x2, sp, #0x50
1005404c4: 9400c9a7    	bl	0x100572b60 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h7424fbb9555fb8f5E>
1005404c8: f94037e1    	ldr	x1, [sp, #0x68]
1005404cc: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1005404d0: eb08003f    	cmp	x1, x8
1005404d4: 540000a0    	b.eq	0x1005404e8 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x260>
1005404d8: b4000081    	cbz	x1, 0x1005404e8 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x260>
1005404dc: f9403be0    	ldr	x0, [sp, #0x70]
1005404e0: 52800022    	mov	w2, #0x1                ; =1
1005404e4: 97eb0421    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005404e8: d0006e94    	adrp	x20, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
1005404ec: 913c5294    	add	x20, x20, #0xf14
1005404f0: 17ffffc3    	b	0x1005403fc <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x174>
1005404f4: d0006e94    	adrp	x20, 0x101312000 <_anon.164d03a2dfc98f969363eaf1c50504fd.1415+0x278>
1005404f8: 913c5294    	add	x20, x20, #0xf14
1005404fc: 14000004    	b	0x10054050c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x284>
100540500: 91008273    	add	x19, x19, #0x20
100540504: f10006b5    	subs	x21, x21, #0x1
100540508: 540001a0    	b.eq	0x10054053c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x2b4>
10054050c: aa1403e0    	mov	x0, x20
100540510: 52800081    	mov	w1, #0x4                ; =4
100540514: aa1303e2    	mov	x2, x19
100540518: 942f399e    	bl	0x10110eb90 <__ZN55_$LT$str$u20$as$u20$serde_json..value..index..Index$GT$10index_into17h0b53ee69a544098bE>
10054051c: b4ffff20    	cbz	x0, 0x100540500 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x278>
100540520: 39400008    	ldrb	w8, [x0]
100540524: 71000d1f    	cmp	w8, #0x3
100540528: 54fffec1    	b.ne	0x100540500 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x278>
10054052c: a9410408    	ldp	x8, x1, [x0, #0x10]
100540530: aa0803e0    	mov	x0, x8
100540534: 97f3bb09    	bl	0x10022f158 <__ZN4core3str21_$LT$impl$u20$str$GT$12trim_matches17h29ad7bb01498db4bE>
100540538: 17fffff2    	b	0x100540500 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x278>
10054053c: a94e7bfd    	ldp	x29, x30, [sp, #0xe0]
100540540: a94d4ff4    	ldp	x20, x19, [sp, #0xd0]
100540544: a94c57f6    	ldp	x22, x21, [sp, #0xc0]
100540548: a94b5ff8    	ldp	x24, x23, [sp, #0xb0]
10054054c: a94a67fa    	ldp	x26, x25, [sp, #0xa0]
100540550: a9496ffc    	ldp	x28, x27, [sp, #0x90]
100540554: 6d4823e9    	ldp	d9, d8, [sp, #0x80]
100540558: 9103c3ff    	add	sp, sp, #0xf0
10054055c: d65f03c0    	ret
100540560: d2800000    	mov	x0, #0x0                ; =0
100540564: f9400be1    	ldr	x1, [sp, #0x10]
100540568: 9432403f    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10054056c: d2800000    	mov	x0, #0x0                ; =0
100540570: f94007e1    	ldr	x1, [sp, #0x8]
100540574: 9432403c    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100540578: d4200020    	brk	#0x1
10054057c: f9000bfa    	str	x26, [sp, #0x10]
100540580: 52800020    	mov	w0, #0x1                ; =1
100540584: aa1a03e1    	mov	x1, x26
100540588: 94324037    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10054058c: f90007fc    	str	x28, [sp, #0x8]
100540590: 52800020    	mov	w0, #0x1                ; =1
100540594: 17fffff7    	b	0x100540570 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x2e8>
100540598: b40000fa    	cbz	x26, 0x1005405b4 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h8a8236e86746fa9bE+0x32c>
10054059c: aa0003f3    	mov	x19, x0
1005405a0: f9400be0    	ldr	x0, [sp, #0x10]
1005405a4: aa1a03e1    	mov	x1, x26
1005405a8: 52800022    	mov	w2, #0x1                ; =1
1005405ac: 97eb03ef    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005405b0: aa1303e0    	mov	x0, x19
1005405b4: 943242e8    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001005405b8 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E>:
1005405b8: d10243ff    	sub	sp, sp, #0x90
1005405bc: a9036ffc    	stp	x28, x27, [sp, #0x30]
1005405c0: a90467fa    	stp	x26, x25, [sp, #0x40]
1005405c4: a9055ff8    	stp	x24, x23, [sp, #0x50]
1005405c8: a90657f6    	stp	x22, x21, [sp, #0x60]
1005405cc: a9074ff4    	stp	x20, x19, [sp, #0x70]
1005405d0: a9087bfd    	stp	x29, x30, [sp, #0x80]
1005405d4: 910203fd    	add	x29, sp, #0x80
1005405d8: a9402428    	ldp	x8, x9, [x1]
1005405dc: eb09011f    	cmp	x8, x9
1005405e0: 54000640    	b.eq	0x1005406a8 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E+0xf0>
1005405e4: aa0003f3    	mov	x19, x0
1005405e8: cb080129    	sub	x9, x9, x8
1005405ec: b201f3ea    	mov	x10, #-0x5555555555555556 ; =-6148914691236517206
1005405f0: f295556a    	movk	x10, #0xaaab
1005405f4: 9bca7d29    	umulh	x9, x9, x10
1005405f8: d347fd37    	lsr	x23, x9, #7
1005405fc: f9400834    	ldr	x20, [x1, #0x10]
100540600: 910003f8    	mov	x24, sp
100540604: 910063f9    	add	x25, sp, #0x18
100540608: 9102011a    	add	x26, x8, #0x80
10054060c: d2f0001b    	mov	x27, #-0x8000000000000000 ; =-9223372036854775808
100540610: 1400000c    	b	0x100540640 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E+0x88>
100540614: 3cc08300    	ldur	q0, [x24, #0x8]
100540618: 3c808320    	stur	q0, [x25, #0x8]
10054061c: f9000fe8    	str	x8, [sp, #0x18]
100540620: 910063e1    	add	x1, sp, #0x18
100540624: aa1303e0    	mov	x0, x19
100540628: aa1403e2    	mov	x2, x20
10054062c: 9400c094    	bl	0x10057087c <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17h173dec863d3bd853E>
100540630: 9103035a    	add	x26, x26, #0xc0
100540634: 91000694    	add	x20, x20, #0x1
100540638: f10006f7    	subs	x23, x23, #0x1
10054063c: 54000360    	b.eq	0x1005406a8 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E+0xf0>
100540640: a97f8740    	ldp	x0, x1, [x26, #-0x8]
100540644: 910003e8    	mov	x8, sp
100540648: 94301a6b    	bl	0x101146ff4 <__ZN3std3sys2fs12canonicalize17ha43fcd369ce83197E>
10054064c: f94003e8    	ldr	x8, [sp]
100540650: eb1b011f    	cmp	x8, x27
100540654: 54fffe01    	b.ne	0x100540614 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E+0x5c>
100540658: f94007f5    	ldr	x21, [sp, #0x8]
10054065c: 924006a8    	and	x8, x21, #0x3
100540660: f100051f    	cmp	x8, #0x1
100540664: 54fffe61    	b.ne	0x100540630 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E+0x78>
100540668: f85ffeb6    	ldr	x22, [x21, #-0x1]!
10054066c: f94006bc    	ldr	x28, [x21, #0x8]
100540670: f9400388    	ldr	x8, [x28]
100540674: b4000068    	cbz	x8, 0x100540680 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E+0xc8>
100540678: aa1603e0    	mov	x0, x22
10054067c: d63f0100    	blr	x8
100540680: f9400781    	ldr	x1, [x28, #0x8]
100540684: b4000081    	cbz	x1, 0x100540694 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E+0xdc>
100540688: f9400b82    	ldr	x2, [x28, #0x10]
10054068c: aa1603e0    	mov	x0, x22
100540690: 97eb03b6    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100540694: aa1503e0    	mov	x0, x21
100540698: 52800301    	mov	w1, #0x18               ; =24
10054069c: 52800102    	mov	w2, #0x8                ; =8
1005406a0: 97eb03b2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005406a4: 17ffffe3    	b	0x100540630 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E+0x78>
1005406a8: a9487bfd    	ldp	x29, x30, [sp, #0x80]
1005406ac: a9474ff4    	ldp	x20, x19, [sp, #0x70]
1005406b0: a94657f6    	ldp	x22, x21, [sp, #0x60]
1005406b4: a9455ff8    	ldp	x24, x23, [sp, #0x50]
1005406b8: a94467fa    	ldp	x26, x25, [sp, #0x40]
1005406bc: a9436ffc    	ldp	x28, x27, [sp, #0x30]
1005406c0: 910243ff    	add	sp, sp, #0x90
1005406c4: d65f03c0    	ret
1005406c8: aa0003f3    	mov	x19, x0
1005406cc: f9400781    	ldr	x1, [x28, #0x8]
1005406d0: b4000081    	cbz	x1, 0x1005406e0 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17h97bce6e93d165068E+0x128>
1005406d4: f9400b82    	ldr	x2, [x28, #0x10]
1005406d8: aa1603e0    	mov	x0, x22
1005406dc: 97eb03a3    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005406e0: aa1503e0    	mov	x0, x21
1005406e4: 52800301    	mov	w1, #0x18               ; =24
1005406e8: 52800102    	mov	w2, #0x8                ; =8
1005406ec: 97eb039f    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1005406f0: aa1303e0    	mov	x0, x19
1005406f4: 94324298    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001005406f8 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17hc79ff8a20413f142E>:
1005406f8: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
1005406fc: a90167fa    	stp	x26, x25, [sp, #0x10]
100540700: a9025ff8    	stp	x24, x23, [sp, #0x20]
100540704: a90357f6    	stp	x22, x21, [sp, #0x30]
100540708: a9044ff4    	stp	x20, x19, [sp, #0x40]
10054070c: a9057bfd    	stp	x29, x30, [sp, #0x50]
100540710: 910143fd    	add	x29, sp, #0x50
100540714: d106c3ff    	sub	sp, sp, #0x1b0
100540718: a9402428    	ldp	x8, x9, [x1]
10054071c: eb09011f    	cmp	x8, x9
100540720: 54000f60    	b.eq	0x10054090c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17hc79ff8a20413f142E+0x214>
100540724: aa0003f3    	mov	x19, x0
100540728: f940082a    	ldr	x10, [x1, #0x10]
10054072c: cb080129    	sub	x9, x9, x8
100540730: d343fd29    	lsr	x9, x9, #3
100540734: d2904d6b    	mov	x11, #0x826b            ; =33387
100540738: f2aade2b    	movk	x11, #0x56f1, lsl #16
10054073c: f2c73ecb    	movk	x11, #0x39f6, lsl #32
100540740: f2e4d48b    	movk	x11, #0x26a4, lsl #48
100540744: 9bcb7d29    	umulh	x9, x9, x11
100540748: d343fd37    	lsr	x23, x9, #3
10054074c: d101c3b8    	sub	x24, x29, #0x70
100540750: a94fd554    	ldp	x20, x21, [x10, #0xf8]
100540754: d10283b9    	sub	x25, x29, #0xa0
100540758: 91030116    	add	x22, x8, #0xc0
10054075c: b24107fb    	mov	x27, #-0x7fffffffffffffff ; =-9223372036854775807
100540760: d2f0001c    	mov	x28, #-0x8000000000000000 ; =-9223372036854775808
100540764: 14000004    	b	0x100540774 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17hc79ff8a20413f142E+0x7c>
100540768: 9106a2d6    	add	x22, x22, #0x1a8
10054076c: f10006f7    	subs	x23, x23, #0x1
100540770: 54000ce0    	b.eq	0x10054090c <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17hc79ff8a20413f142E+0x214>
100540774: a94206c0    	ldp	x0, x1, [x22, #0x20]
100540778: a94096c4    	ldp	x4, x5, [x22, #0x8]
10054077c: 9102c3e8    	add	x8, sp, #0xb0
100540780: aa1403e2    	mov	x2, x20
100540784: aa1503e3    	mov	x3, x21
100540788: 940bbf50    	bl	0x1008304c8 <__ZN13codexmate_lib4core10repository29load_refreshable_account_auth17h9cb7df91ccdb8a39E>
10054078c: f9405be8    	ldr	x8, [sp, #0xb0]
100540790: eb1b011f    	cmp	x8, x27
100540794: 54fffea0    	b.eq	0x100540768 <__ZN121_$LT$hashbrown..map..HashMap$LT$K$C$V$C$S$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$LP$K$C$V$RP$$GT$$GT$6extend17hc79ff8a20413f142E+0x70>
100540798: ad4987e0    	ldp	q0, q1, [sp, #0x130]
10054079c: ad0407e0    	stp	q0, q1, [sp, #0x80]
1005407a0: 3dc057e0    	ldr	q0, [sp, #0x150]
1005407a4: 3d802be0    	str	q0, [sp, #0xa0]
1005407a8: ad4787e0    	ldp	q0, q1, [sp, #0xf0]
1005407ac: ad0207e0    	stp	q0, q1, [sp, #0x40]
1005407b0: ad4883e1    	ldp	q1, q0, [sp, #0x110]
1005407b4: ad0303e1    	stp	q1, q0, [sp, #0x60]
1005407b8: ad4587e0    	ldp	q0, q1, [sp, #0xb0]
1005407bc: ad0007e0    	stp	q0, q1, [sp]
1005407c0: ad4683e1    	ldp	q1, q0, [sp, #0xd0]
1005407c4: ad0103e1    	stp	q1, q0, [sp, #0x20]
1005407c8: d101c3a8    	sub	x8, x29, #0x70
1005407cc: aa1603e0    	mov	x0, x22
1005407d0: 9430e34b    	bl	0x1011794fc <__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17hae67e35e1c94f097E>
1005407d4: 9102c3e8    	add	x8, sp, #0xb0
1005407d8: 910003e0    	mov	x0, sp
1005407dc: 97f2bc4f    	bl	0x1001ef918 <__ZN13codexmate_lib4core4auth21credential_generation17h877063463a143766E>
1005407e0: f85903ba    	ldur	x26, [x29, #-0x70]
1005407e4: 3cc08300    	ldur	q0, [x24, #0x8]
1005407e8: 3c9603a0    	stur	q0, [x29, #-0xa0]
1005407ec: 3dc02fe0    	ldr	q0, [sp, #0xb0]
1005407f0: 3c810320    	stur	q0, [x25, #0x10]
1005407f4: f94063e8    	ldr	x8, [sp, #0xc0]
1005407f8: f8020328    	stur	x8, [x25, #0x20]
1005407fc: f94003e1    	ldr	x1, [sp]
