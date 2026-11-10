
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010079c2e0 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E>:
10079c2e0: d10103ff    	sub	sp, sp, #0x40
10079c2e4: a9024ff4    	stp	x20, x19, [sp, #0x20]
10079c2e8: a9037bfd    	stp	x29, x30, [sp, #0x30]
10079c2ec: 9100c3fd    	add	x29, sp, #0x30
10079c2f0: f9400008    	ldr	x8, [x0]
10079c2f4: d2410109    	eor	x9, x8, #0x8000000000000000
10079c2f8: 5280004a    	mov	w10, #0x2               ; =2
10079c2fc: f100011f    	cmp	x8, #0x0
10079c300: 9a8ab128    	csel	x8, x9, x10, lt
10079c304: b4000348    	cbz	x8, 0x10079c36c <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x8c>
10079c308: f100051f    	cmp	x8, #0x1
10079c30c: 540007e1    	b.ne	0x10079c408 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x128>
10079c310: f9400033    	ldr	x19, [x1]
10079c314: f9400a68    	ldr	x8, [x19, #0x10]
10079c318: f9400269    	ldr	x9, [x19]
10079c31c: eb08013f    	cmp	x9, x8
10079c320: 54000d80    	b.eq	0x10079c4d0 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x1f0>
10079c324: f9400669    	ldr	x9, [x19, #0x8]
10079c328: 52800f6a    	mov	w10, #0x7b              ; =123
10079c32c: 3828692a    	strb	w10, [x9, x8]
10079c330: 91000508    	add	x8, x8, #0x1
10079c334: f9000a68    	str	x8, [x19, #0x10]
10079c338: 52802008    	mov	w8, #0x100              ; =256
10079c33c: 790023e8    	strh	w8, [sp, #0x10]
10079c340: f9000fe1    	str	x1, [sp, #0x18]
10079c344: 90006081    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079c348: 910f0421    	add	x1, x1, #0x3c1
10079c34c: d0006063    	adrp	x3, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10079c350: 91192063    	add	x3, x3, #0x648
10079c354: 910043e0    	add	x0, sp, #0x10
10079c358: 52800082    	mov	w2, #0x4                ; =4
10079c35c: 52800084    	mov	w4, #0x4                ; =4
10079c360: 97f637ac    	bl	0x10052a210 <__ZN10serde_core3ser12SerializeMap15serialize_entry17hec930f8dfface222E>
10079c364: b5000980    	cbnz	x0, 0x10079c494 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x1b4>
10079c368: 14000017    	b	0x10079c3c4 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0xe4>
10079c36c: f9400033    	ldr	x19, [x1]
10079c370: f9400a68    	ldr	x8, [x19, #0x10]
10079c374: f9400269    	ldr	x9, [x19]
10079c378: eb08013f    	cmp	x9, x8
10079c37c: 54000be0    	b.eq	0x10079c4f8 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x218>
10079c380: f9400669    	ldr	x9, [x19, #0x8]
10079c384: 52800f6a    	mov	w10, #0x7b              ; =123
10079c388: 3828692a    	strb	w10, [x9, x8]
10079c38c: 91000508    	add	x8, x8, #0x1
10079c390: f9000a68    	str	x8, [x19, #0x10]
10079c394: 52802008    	mov	w8, #0x100              ; =256
10079c398: 790023e8    	strh	w8, [sp, #0x10]
10079c39c: f9000fe1    	str	x1, [sp, #0x18]
10079c3a0: 90006081    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079c3a4: 910f0421    	add	x1, x1, #0x3c1
10079c3a8: 90006083    	adrp	x3, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079c3ac: 910f1463    	add	x3, x3, #0x3c5
10079c3b0: 910043e0    	add	x0, sp, #0x10
10079c3b4: 52800082    	mov	w2, #0x4                ; =4
10079c3b8: 528000c4    	mov	w4, #0x6                ; =6
10079c3bc: 97f63795    	bl	0x10052a210 <__ZN10serde_core3ser12SerializeMap15serialize_entry17hec930f8dfface222E>
10079c3c0: b50006a0    	cbnz	x0, 0x10079c494 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x1b4>
10079c3c4: 394043e8    	ldrb	w8, [sp, #0x10]
10079c3c8: 370001c8    	tbnz	w8, #0x0, 0x10079c400 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x120>
10079c3cc: 394047e8    	ldrb	w8, [sp, #0x11]
10079c3d0: 34000188    	cbz	w8, 0x10079c400 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x120>
10079c3d4: f9400fe8    	ldr	x8, [sp, #0x18]
10079c3d8: f9400113    	ldr	x19, [x8]
10079c3dc: f9400a61    	ldr	x1, [x19, #0x10]
10079c3e0: f9400268    	ldr	x8, [x19]
10079c3e4: eb01011f    	cmp	x8, x1
10079c3e8: 54000b00    	b.eq	0x10079c548 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x268>
10079c3ec: f9400668    	ldr	x8, [x19, #0x8]
10079c3f0: 52800fa9    	mov	w9, #0x7d               ; =125
10079c3f4: 38216909    	strb	w9, [x8, x1]
10079c3f8: 91000428    	add	x8, x1, #0x1
10079c3fc: f9000a68    	str	x8, [x19, #0x10]
10079c400: d2800000    	mov	x0, #0x0                ; =0
10079c404: 14000024    	b	0x10079c494 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x1b4>
10079c408: aa0003f3    	mov	x19, x0
10079c40c: f9400034    	ldr	x20, [x1]
10079c410: f9400a88    	ldr	x8, [x20, #0x10]
10079c414: f9400289    	ldr	x9, [x20]
10079c418: eb08013f    	cmp	x9, x8
10079c41c: 54000820    	b.eq	0x10079c520 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x240>
10079c420: f9400689    	ldr	x9, [x20, #0x8]
10079c424: 52800f6a    	mov	w10, #0x7b              ; =123
10079c428: 3828692a    	strb	w10, [x9, x8]
10079c42c: 91000508    	add	x8, x8, #0x1
10079c430: f9000a88    	str	x8, [x20, #0x10]
10079c434: 52802008    	mov	w8, #0x100              ; =256
10079c438: 790023e8    	strh	w8, [sp, #0x10]
10079c43c: f9000fe1    	str	x1, [sp, #0x18]
10079c440: 90006081    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079c444: 910f0421    	add	x1, x1, #0x3c1
10079c448: 90006083    	adrp	x3, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079c44c: 910f2c63    	add	x3, x3, #0x3cb
10079c450: 910043e0    	add	x0, sp, #0x10
10079c454: 52800082    	mov	w2, #0x4                ; =4
10079c458: 52800104    	mov	w4, #0x8                ; =8
10079c45c: 97f6376d    	bl	0x10052a210 <__ZN10serde_core3ser12SerializeMap15serialize_entry17hec930f8dfface222E>
10079c460: b50001a0    	cbnz	x0, 0x10079c494 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x1b4>
10079c464: 394043e8    	ldrb	w8, [sp, #0x10]
10079c468: 7100051f    	cmp	w8, #0x1
10079c46c: 54000061    	b.ne	0x10079c478 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x198>
10079c470: 9428b8be    	bl	0x1011ca768 <__ZN10serde_json3ser17invalid_raw_value17h6aa1aefc3e141c8fE>
10079c474: 14000008    	b	0x10079c494 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x1b4>
10079c478: b0006061    	adrp	x1, 0x1013a9000 <_anon.cdb272158a04aae91ce02c6e47f106c7.539+0xd4>
10079c47c: 912b7821    	add	x1, x1, #0xade
10079c480: 910043e0    	add	x0, sp, #0x10
10079c484: 52800042    	mov	w2, #0x2                ; =2
10079c488: aa1303e3    	mov	x3, x19
10079c48c: 97f5f42c    	bl	0x10051953c <__ZN10serde_core3ser12SerializeMap15serialize_entry17h32c6e6db3815e075E>
10079c490: b40000a0    	cbz	x0, 0x10079c4a4 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x1c4>
10079c494: a9437bfd    	ldp	x29, x30, [sp, #0x30]
10079c498: a9424ff4    	ldp	x20, x19, [sp, #0x20]
10079c49c: 910103ff    	add	sp, sp, #0x40
10079c4a0: d65f03c0    	ret
10079c4a4: 394043e8    	ldrb	w8, [sp, #0x10]
10079c4a8: 3707fac8    	tbnz	w8, #0x0, 0x10079c400 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x120>
10079c4ac: 394047e8    	ldrb	w8, [sp, #0x11]
10079c4b0: 34fffa88    	cbz	w8, 0x10079c400 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x120>
10079c4b4: f9400fe8    	ldr	x8, [sp, #0x18]
10079c4b8: f9400100    	ldr	x0, [x8]
10079c4bc: b0006061    	adrp	x1, 0x1013a9000 <_anon.cdb272158a04aae91ce02c6e47f106c7.539+0xd4>
10079c4c0: 910a2021    	add	x1, x1, #0x288
10079c4c4: 52800022    	mov	w2, #0x1                ; =1
10079c4c8: 97f70201    	bl	0x10055cccc <__ZN5alloc3vec16Vec$LT$T$C$A$GT$17extend_from_slice17h3b7683e8d5b760adE>
10079c4cc: 17ffffcd    	b	0x10079c400 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x120>
10079c4d0: aa1303e0    	mov	x0, x19
10079c4d4: aa0103f4    	mov	x20, x1
10079c4d8: aa0803e1    	mov	x1, x8
10079c4dc: 52800022    	mov	w2, #0x1                ; =1
10079c4e0: 52800023    	mov	w3, #0x1                ; =1
10079c4e4: 52800024    	mov	w4, #0x1                ; =1
10079c4e8: 9427c8fb    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
10079c4ec: aa1403e1    	mov	x1, x20
10079c4f0: f9400a68    	ldr	x8, [x19, #0x10]
10079c4f4: 17ffff8c    	b	0x10079c324 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x44>
10079c4f8: aa1303e0    	mov	x0, x19
10079c4fc: aa0103f4    	mov	x20, x1
10079c500: aa0803e1    	mov	x1, x8
10079c504: 52800022    	mov	w2, #0x1                ; =1
10079c508: 52800023    	mov	w3, #0x1                ; =1
10079c50c: 52800024    	mov	w4, #0x1                ; =1
10079c510: 9427c8f1    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
10079c514: aa1403e1    	mov	x1, x20
10079c518: f9400a68    	ldr	x8, [x19, #0x10]
10079c51c: 17ffff99    	b	0x10079c380 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0xa0>
10079c520: aa1403e0    	mov	x0, x20
10079c524: f90007e1    	str	x1, [sp, #0x8]
10079c528: aa0803e1    	mov	x1, x8
10079c52c: 52800022    	mov	w2, #0x1                ; =1
10079c530: 52800023    	mov	w3, #0x1                ; =1
10079c534: 52800024    	mov	w4, #0x1                ; =1
10079c538: 9427c8e7    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
10079c53c: f94007e1    	ldr	x1, [sp, #0x8]
10079c540: f9400a88    	ldr	x8, [x20, #0x10]
10079c544: 17ffffb7    	b	0x10079c420 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x140>
10079c548: aa1303e0    	mov	x0, x19
10079c54c: 52800022    	mov	w2, #0x1                ; =1
10079c550: 52800023    	mov	w3, #0x1                ; =1
10079c554: 52800024    	mov	w4, #0x1                ; =1
10079c558: 9427c8df    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
10079c55c: f9400a61    	ldr	x1, [x19, #0x10]
10079c560: 17ffffa3    	b	0x10079c3ec <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayCredential$GT$9serialize17h0ab6a70f7d80ea23E+0x10c>
