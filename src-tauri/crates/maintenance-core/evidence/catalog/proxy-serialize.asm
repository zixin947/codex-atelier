
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010079e0e8 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E>:
10079e0e8: d10103ff    	sub	sp, sp, #0x40
10079e0ec: a90157f6    	stp	x22, x21, [sp, #0x10]
10079e0f0: a9024ff4    	stp	x20, x19, [sp, #0x20]
10079e0f4: a9037bfd    	stp	x29, x30, [sp, #0x30]
10079e0f8: 9100c3fd    	add	x29, sp, #0x30
10079e0fc: aa0003f3    	mov	x19, x0
10079e100: f9400034    	ldr	x20, [x1]
10079e104: f9400a88    	ldr	x8, [x20, #0x10]
10079e108: f9400289    	ldr	x9, [x20]
10079e10c: eb08013f    	cmp	x9, x8
10079e110: 54000940    	b.eq	0x10079e238 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x150>
10079e114: f9400689    	ldr	x9, [x20, #0x8]
10079e118: 52800f6a    	mov	w10, #0x7b              ; =123
10079e11c: 3828692a    	strb	w10, [x9, x8]
10079e120: 91000508    	add	x8, x8, #0x1
10079e124: f9000a88    	str	x8, [x20, #0x10]
10079e128: 52802008    	mov	w8, #0x100              ; =256
10079e12c: 790003e8    	strh	w8, [sp]
10079e130: f90007e1    	str	x1, [sp, #0x8]
10079e134: d0006061    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079e138: 9111f021    	add	x1, x1, #0x47c
10079e13c: 910003e0    	mov	x0, sp
10079e140: 91012a63    	add	x3, x19, #0x4a
10079e144: 528000e2    	mov	w2, #0x7                ; =7
10079e148: 97f62bb3    	bl	0x100529014 <__ZN10serde_core3ser12SerializeMap15serialize_entry17hdf5d036ab6deacfdE>
10079e14c: b40000c0    	cbz	x0, 0x10079e164 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x7c>
10079e150: a9437bfd    	ldp	x29, x30, [sp, #0x30]
10079e154: a9424ff4    	ldp	x20, x19, [sp, #0x20]
10079e158: a94157f6    	ldp	x22, x21, [sp, #0x10]
10079e15c: 910103ff    	add	sp, sp, #0x40
10079e160: d65f03c0    	ret
10079e164: 394003e8    	ldrb	w8, [sp]
10079e168: 360000c8    	tbz	w8, #0x0, 0x10079e180 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x98>
10079e16c: a9437bfd    	ldp	x29, x30, [sp, #0x30]
10079e170: a9424ff4    	ldp	x20, x19, [sp, #0x20]
10079e174: a94157f6    	ldp	x22, x21, [sp, #0x10]
10079e178: 910103ff    	add	sp, sp, #0x40
10079e17c: 1428b17b    	b	0x1011ca768 <__ZN10serde_json3ser17invalid_raw_value17h6aa1aefc3e141c8fE>
10079e180: b0006061    	adrp	x1, 0x1013ab000 <_anon.6408540cdffbe318d49f52812e99783f.467+0x9b>
10079e184: 91079821    	add	x1, x1, #0x1e6
10079e188: 910003e0    	mov	x0, sp
10079e18c: 91012263    	add	x3, x19, #0x48
10079e190: 52800082    	mov	w2, #0x4                ; =4
10079e194: 97f63711    	bl	0x10052bdd8 <__ZN10serde_core3ser12SerializeMap15serialize_entry17hfdd4d227f5c19b25E>
10079e198: b5fffdc0    	cbnz	x0, 0x10079e150 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x68>
10079e19c: 394003e8    	ldrb	w8, [sp]
10079e1a0: 3707fe68    	tbnz	w8, #0x0, 0x10079e16c <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x84>
10079e1a4: b0006061    	adrp	x1, 0x1013ab000 <_anon.6408540cdffbe318d49f52812e99783f.467+0x9b>
10079e1a8: 91188021    	add	x1, x1, #0x620
10079e1ac: 910003e0    	mov	x0, sp
10079e1b0: 528000e2    	mov	w2, #0x7                ; =7
10079e1b4: aa1303e3    	mov	x3, x19
10079e1b8: 97f61c61    	bl	0x10052533c <__ZN10serde_core3ser12SerializeMap15serialize_entry17hace858a778cac3daE>
10079e1bc: b5fffca0    	cbnz	x0, 0x10079e150 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x68>
10079e1c0: 394003e8    	ldrb	w8, [sp]
10079e1c4: 3707fd48    	tbnz	w8, #0x0, 0x10079e16c <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x84>
10079e1c8: d0006061    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079e1cc: 91120c21    	add	x1, x1, #0x483
10079e1d0: 910003e0    	mov	x0, sp
10079e1d4: 91006263    	add	x3, x19, #0x18
10079e1d8: 52800182    	mov	w2, #0xc                ; =12
10079e1dc: 97f61c58    	bl	0x10052533c <__ZN10serde_core3ser12SerializeMap15serialize_entry17hace858a778cac3daE>
10079e1e0: b5fffb80    	cbnz	x0, 0x10079e150 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x68>
10079e1e4: 394003e8    	ldrb	w8, [sp]
10079e1e8: 3707fc28    	tbnz	w8, #0x0, 0x10079e16c <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x84>
10079e1ec: d0006061    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079e1f0: 910ea421    	add	x1, x1, #0x3a9
10079e1f4: 910003e0    	mov	x0, sp
10079e1f8: 9100c263    	add	x3, x19, #0x30
10079e1fc: 52800122    	mov	w2, #0x9                ; =9
10079e200: 97f635f7    	bl	0x10052b9dc <__ZN10serde_core3ser12SerializeMap15serialize_entry17hf92b925f2096e89dE>
10079e204: b5fffa60    	cbnz	x0, 0x10079e150 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x68>
10079e208: 394003e8    	ldrb	w8, [sp]
10079e20c: 3707fa28    	tbnz	w8, #0x0, 0x10079e150 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x68>
10079e210: 394007e8    	ldrb	w8, [sp, #0x1]
10079e214: 34fff9e8    	cbz	w8, 0x10079e150 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x68>
10079e218: f94007e8    	ldr	x8, [sp, #0x8]
10079e21c: f9400100    	ldr	x0, [x8]
10079e220: f0006041    	adrp	x1, 0x1013a9000 <_anon.cdb272158a04aae91ce02c6e47f106c7.539+0xd4>
10079e224: 910a2021    	add	x1, x1, #0x288
10079e228: 52800022    	mov	w2, #0x1                ; =1
10079e22c: 97f6faa8    	bl	0x10055cccc <__ZN5alloc3vec16Vec$LT$T$C$A$GT$17extend_from_slice17h3b7683e8d5b760adE>
10079e230: d2800000    	mov	x0, #0x0                ; =0
10079e234: 17ffffc7    	b	0x10079e150 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x68>
10079e238: aa1403e0    	mov	x0, x20
10079e23c: aa0103f5    	mov	x21, x1
10079e240: aa0803e1    	mov	x1, x8
10079e244: 52800022    	mov	w2, #0x1                ; =1
10079e248: 52800023    	mov	w3, #0x1                ; =1
10079e24c: 52800024    	mov	w4, #0x1                ; =1
10079e250: 9427c1a1    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
10079e254: aa1503e1    	mov	x1, x21
10079e258: f9400a88    	ldr	x8, [x20, #0x10]
10079e25c: 17ffffae    	b	0x10079e114 <__ZN13codexmate_lib4core5relay6models1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayProxyStatus$GT$9serialize17h07a52d20860eb124E+0x2c>
