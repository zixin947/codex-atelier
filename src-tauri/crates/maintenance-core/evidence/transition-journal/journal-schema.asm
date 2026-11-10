
AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010079211c <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E>:
10079211c: d10243ff    	sub	sp, sp, #0x90
100792120: a90657f6    	stp	x22, x21, [sp, #0x60]
100792124: a9074ff4    	stp	x20, x19, [sp, #0x70]
100792128: a9087bfd    	stp	x29, x30, [sp, #0x80]
10079212c: 910203fd    	add	x29, sp, #0x80
100792130: aa0003f4    	mov	x20, x0
100792134: aa0803f3    	mov	x19, x8
100792138: 900060c0    	adrp	x0, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10079213c: 913c5400    	add	x0, x0, #0xf15
100792140: 528000d5    	mov	w21, #0x6               ; =6
100792144: 9100c3e8    	add	x8, sp, #0x30
100792148: 52800221    	mov	w1, #0x11               ; =17
10079214c: 528000c2    	mov	w2, #0x6                ; =6
100792150: 9425e915    	bl	0x10110c5a4 <__ZN82_$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$16serialize_struct17h9def4837d60a4ab2E>
100792154: a94323e9    	ldp	x9, x8, [sp, #0x30]
100792158: d280004a    	mov	x10, #0x2               ; =2
10079215c: f2f0000a    	movk	x10, #0x8000, lsl #48
100792160: eb0a013f    	cmp	x9, x10
100792164: 54000101    	b.ne	0x100792184 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x68>
100792168: f9000668    	str	x8, [x19, #0x8]
10079216c: 39000275    	strb	w21, [x19]
100792170: a9487bfd    	ldp	x29, x30, [sp, #0x80]
100792174: a9474ff4    	ldp	x20, x19, [sp, #0x70]
100792178: a94657f6    	ldp	x22, x21, [sp, #0x60]
10079217c: 910243ff    	add	sp, sp, #0x90
100792180: d65f03c0    	ret
100792184: ad4207e0    	ldp	q0, q1, [sp, #0x40]
100792188: ad0087e0    	stp	q0, q1, [sp, #0x10]
10079218c: 910003f5    	mov	x21, sp
100792190: a90023e9    	stp	x9, x8, [sp]
100792194: 900060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100792198: 91385021    	add	x1, x1, #0xe14
10079219c: 910003e0    	mov	x0, sp
1007921a0: 91010683    	add	x3, x20, #0x41
1007921a4: 528001a2    	mov	w2, #0xd                ; =13
1007921a8: 97ead0c2    	bl	0x1002464b0 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17hff9306b651d0e9beE>
1007921ac: b5000480    	cbnz	x0, 0x10079223c <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x120>
1007921b0: 900060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
1007921b4: 913b5c21    	add	x1, x1, #0xed7
1007921b8: 910003e0    	mov	x0, sp
1007921bc: 91010283    	add	x3, x20, #0x40
1007921c0: 528000c2    	mov	w2, #0x6                ; =6
1007921c4: 97eac248    	bl	0x100242ae4 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h4ce81db2bcd95770E>
1007921c8: b50003a0    	cbnz	x0, 0x10079223c <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x120>
1007921cc: 900060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
1007921d0: 913b7421    	add	x1, x1, #0xedd
1007921d4: 910003e0    	mov	x0, sp
1007921d8: 528000a2    	mov	w2, #0x5                ; =5
1007921dc: aa1403e3    	mov	x3, x20
1007921e0: 97eac31a    	bl	0x100242e48 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h531e6971296a2d24E>
1007921e4: b50002c0    	cbnz	x0, 0x10079223c <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x120>
1007921e8: 900060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
1007921ec: 913c9821    	add	x1, x1, #0xf26
1007921f0: 910003e0    	mov	x0, sp
1007921f4: 9100c283    	add	x3, x20, #0x30
1007921f8: 52800162    	mov	w2, #0xb                ; =11
1007921fc: 97eac8d0    	bl	0x10024453c <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h93565849ccd292d4E>
100792200: b50001e0    	cbnz	x0, 0x10079223c <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x120>
100792204: 900060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100792208: 913cc421    	add	x1, x1, #0xf31
10079220c: 910003e0    	mov	x0, sp
100792210: 9100e283    	add	x3, x20, #0x38
100792214: 52800162    	mov	w2, #0xb                ; =11
100792218: 97eac8c9    	bl	0x10024453c <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17h93565849ccd292d4E>
10079221c: b5000100    	cbnz	x0, 0x10079223c <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x120>
100792220: 900060c1    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
100792224: 913bb821    	add	x1, x1, #0xeee
100792228: 910003e0    	mov	x0, sp
10079222c: 91006283    	add	x3, x20, #0x18
100792230: 528000c2    	mov	w2, #0x6                ; =6
100792234: 97eacf19    	bl	0x100245e98 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$15serialize_field17hf8f139222e299adbE>
100792238: b4000340    	cbz	x0, 0x1007922a0 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x184>
10079223c: f9000660    	str	x0, [x19, #0x8]
100792240: 528000c8    	mov	w8, #0x6                ; =6
100792244: 39000268    	strb	w8, [x19]
100792248: f94003f3    	ldr	x19, [sp]
10079224c: b24107e8    	mov	x8, #-0x7fffffffffffffff ; =-9223372036854775807
100792250: eb08027f    	cmp	x19, x8
100792254: 540000e1    	b.ne	0x100792270 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x154>
100792258: 394023e8    	ldrb	w8, [sp, #0x8]
10079225c: 7100191f    	cmp	w8, #0x6
100792260: 54fff880    	b.eq	0x100792170 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x54>
100792264: 910022a0    	add	x0, x21, #0x8
100792268: 94016ddb    	bl	0x1007ed9d4 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10079226c: 17ffffc1    	b	0x100792170 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x54>
100792270: 910003e8    	mov	x8, sp
100792274: 91006100    	add	x0, x8, #0x18
100792278: 97ff77fb    	bl	0x100770264 <__ZN99_$LT$alloc..collections..btree..map..BTreeMap$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h25de00eca2f35937E>
10079227c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100792280: eb08027f    	cmp	x19, x8
100792284: 54fff760    	b.eq	0x100792170 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x54>
100792288: b4fff753    	cbz	x19, 0x100792170 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x54>
10079228c: f94007e0    	ldr	x0, [sp, #0x8]
100792290: aa1303e1    	mov	x1, x19
100792294: 52800022    	mov	w2, #0x1                ; =1
100792298: 97e1bcb4    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10079229c: 17ffffb5    	b	0x100792170 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x54>
1007922a0: ad4007e0    	ldp	q0, q1, [sp]
1007922a4: ad0187e0    	stp	q0, q1, [sp, #0x30]
1007922a8: 3dc00be0    	ldr	q0, [sp, #0x20]
1007922ac: 3d8017e0    	str	q0, [sp, #0x50]
1007922b0: 9100c3e0    	add	x0, sp, #0x30
1007922b4: aa1303e8    	mov	x8, x19
1007922b8: 9425e918    	bl	0x10110c718 <__ZN89_$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$3end17h3354b3c9e0455d21E>
1007922bc: 17ffffad    	b	0x100792170 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x54>
1007922c0: aa0003f4    	mov	x20, x0
1007922c4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1007922c8: eb08027f    	cmp	x19, x8
1007922cc: 54000160    	b.eq	0x1007922f8 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x1dc>
1007922d0: b4000153    	cbz	x19, 0x1007922f8 <__ZN13codexmate_lib4core5relay18transition_journal1_122_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..transition_journal..TransitionJournal$GT$9serialize17h2a74589f6212bec0E+0x1dc>
1007922d4: f94007e0    	ldr	x0, [sp, #0x8]
1007922d8: aa1303e1    	mov	x1, x19
1007922dc: 52800022    	mov	w2, #0x1                ; =1
1007922e0: 97e1bca2    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007922e4: aa1403e0    	mov	x0, x20
1007922e8: 9428fb9b    	bl	0x1011d1154 <_writev+0x1011d1154>
1007922ec: aa0003f4    	mov	x20, x0
1007922f0: 910003e0    	mov	x0, sp
1007922f4: 94016f6b    	bl	0x1007ee0a0 <__ZN4core3ptr57drop_in_place$LT$serde_json..value..ser..SerializeMap$GT$17he7d746eed2e7e905E>
1007922f8: aa1403e0    	mov	x0, x20
1007922fc: 9428fb96    	bl	0x1011d1154 <_writev+0x1011d1154>
100792300: 9428fa12    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
