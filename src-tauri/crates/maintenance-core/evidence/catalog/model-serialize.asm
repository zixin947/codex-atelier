
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010079ce6c <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E>:
10079ce6c: d10103ff    	sub	sp, sp, #0x40
10079ce70: a90157f6    	stp	x22, x21, [sp, #0x10]
10079ce74: a9024ff4    	stp	x20, x19, [sp, #0x20]
10079ce78: a9037bfd    	stp	x29, x30, [sp, #0x30]
10079ce7c: 9100c3fd    	add	x29, sp, #0x30
10079ce80: aa0003f3    	mov	x19, x0
10079ce84: f9400034    	ldr	x20, [x1]
10079ce88: f9400a88    	ldr	x8, [x20, #0x10]
10079ce8c: f9400289    	ldr	x9, [x20]
10079ce90: eb08013f    	cmp	x9, x8
10079ce94: 540008e0    	b.eq	0x10079cfb0 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x144>
10079ce98: f9400689    	ldr	x9, [x20, #0x8]
10079ce9c: 52800f6a    	mov	w10, #0x7b              ; =123
10079cea0: 3828692a    	strb	w10, [x9, x8]
10079cea4: 91000508    	add	x8, x8, #0x1
10079cea8: f9000a88    	str	x8, [x20, #0x10]
10079ceac: 52802008    	mov	w8, #0x100              ; =256
10079ceb0: 790003e8    	strh	w8, [sp]
10079ceb4: f90007e1    	str	x1, [sp, #0x8]
10079ceb8: d0006061    	adrp	x1, 0x1013aa000 <_anon.6408540cdffbe318d49f52812e99783f.267+0xd1>
10079cebc: 911cc421    	add	x1, x1, #0x731
10079cec0: 910003e0    	mov	x0, sp
10079cec4: 91004263    	add	x3, x19, #0x10
10079cec8: 528000a2    	mov	w2, #0x5                ; =5
10079cecc: 97f6211c    	bl	0x10052533c <__ZN10serde_core3ser12SerializeMap15serialize_entry17hace858a778cac3daE>
10079ced0: b5000180    	cbnz	x0, 0x10079cf00 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x94>
10079ced4: 394003e8    	ldrb	w8, [sp]
10079ced8: 36000068    	tbz	w8, #0x0, 0x10079cee4 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x78>
10079cedc: 9428b623    	bl	0x1011ca768 <__ZN10serde_json3ser17invalid_raw_value17h6aa1aefc3e141c8fE>
10079cee0: 14000008    	b	0x10079cf00 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x94>
10079cee4: f0006061    	adrp	x1, 0x1013ab000 <_anon.6408540cdffbe318d49f52812e99783f.467+0x9b>
10079cee8: 9118b421    	add	x1, x1, #0x62d
10079ceec: 910003e0    	mov	x0, sp
10079cef0: 91016263    	add	x3, x19, #0x58
10079cef4: 528000e2    	mov	w2, #0x7                ; =7
10079cef8: 97f614b7    	bl	0x1005221d4 <__ZN10serde_core3ser12SerializeMap15serialize_entry17h88ba24cdac469be0E>
10079cefc: b40000c0    	cbz	x0, 0x10079cf14 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0xa8>
10079cf00: a9437bfd    	ldp	x29, x30, [sp, #0x30]
10079cf04: a9424ff4    	ldp	x20, x19, [sp, #0x20]
10079cf08: a94157f6    	ldp	x22, x21, [sp, #0x10]
10079cf0c: 910103ff    	add	sp, sp, #0x40
10079cf10: d65f03c0    	ret
10079cf14: 394003e8    	ldrb	w8, [sp]
10079cf18: 3707fe28    	tbnz	w8, #0x0, 0x10079cedc <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x70>
10079cf1c: 90006081    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079cf20: 910f8821    	add	x1, x1, #0x3e2
10079cf24: 910003e0    	mov	x0, sp
10079cf28: 528001a2    	mov	w2, #0xd                ; =13
10079cf2c: aa1303e3    	mov	x3, x19
10079cf30: 97f62f1f    	bl	0x100528bac <__ZN10serde_core3ser12SerializeMap15serialize_entry17hdc1114bed2d36da8E>
10079cf34: b5fffe60    	cbnz	x0, 0x10079cf00 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x94>
10079cf38: 394003e8    	ldrb	w8, [sp]
10079cf3c: 3707fd08    	tbnz	w8, #0x0, 0x10079cedc <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x70>
10079cf40: 90006081    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079cf44: 910fbc21    	add	x1, x1, #0x3ef
10079cf48: 910003e0    	mov	x0, sp
10079cf4c: 91010263    	add	x3, x19, #0x40
10079cf50: 52800162    	mov	w2, #0xb                ; =11
10079cf54: 97f63aa2    	bl	0x10052b9dc <__ZN10serde_core3ser12SerializeMap15serialize_entry17hf92b925f2096e89dE>
10079cf58: b5fffd40    	cbnz	x0, 0x10079cf00 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x94>
10079cf5c: 394003e8    	ldrb	w8, [sp]
10079cf60: 3707fbe8    	tbnz	w8, #0x0, 0x10079cedc <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x70>
10079cf64: 90006081    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079cf68: 910fe821    	add	x1, x1, #0x3fa
10079cf6c: 910003e0    	mov	x0, sp
10079cf70: 9100a263    	add	x3, x19, #0x28
10079cf74: 528000c2    	mov	w2, #0x6                ; =6
10079cf78: 97f620f1    	bl	0x10052533c <__ZN10serde_core3ser12SerializeMap15serialize_entry17hace858a778cac3daE>
10079cf7c: b5fffc20    	cbnz	x0, 0x10079cf00 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x94>
10079cf80: 394003e8    	ldrb	w8, [sp]
10079cf84: 3707fbe8    	tbnz	w8, #0x0, 0x10079cf00 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x94>
10079cf88: 394007e8    	ldrb	w8, [sp, #0x1]
10079cf8c: 34fffba8    	cbz	w8, 0x10079cf00 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x94>
10079cf90: f94007e8    	ldr	x8, [sp, #0x8]
10079cf94: f9400100    	ldr	x0, [x8]
10079cf98: b0006061    	adrp	x1, 0x1013a9000 <_anon.cdb272158a04aae91ce02c6e47f106c7.539+0xd4>
10079cf9c: 910a2021    	add	x1, x1, #0x288
10079cfa0: 52800022    	mov	w2, #0x1                ; =1
10079cfa4: 97f6ff4a    	bl	0x10055cccc <__ZN5alloc3vec16Vec$LT$T$C$A$GT$17extend_from_slice17h3b7683e8d5b760adE>
10079cfa8: d2800000    	mov	x0, #0x0                ; =0
10079cfac: 17ffffd5    	b	0x10079cf00 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x94>
10079cfb0: aa1403e0    	mov	x0, x20
10079cfb4: aa0103f5    	mov	x21, x1
10079cfb8: aa0803e1    	mov	x1, x8
10079cfbc: 52800022    	mov	w2, #0x1                ; =1
10079cfc0: 52800023    	mov	w3, #0x1                ; =1
10079cfc4: 52800024    	mov	w4, #0x1                ; =1
10079cfc8: 9427c643    	bl	0x10118e8d4 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he59b5ef0f52a5e29E>
10079cfcc: aa1503e1    	mov	x1, x21
10079cfd0: f9400a88    	ldr	x8, [x20, #0x10]
10079cfd4: 17ffffb1    	b	0x10079ce98 <__ZN13codexmate_lib4core5relay6models1_108_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayModelEntry$GT$9serialize17h4fb84ca608bea4f0E+0x2c>
