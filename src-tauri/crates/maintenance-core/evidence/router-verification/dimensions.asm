
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100362c54 <__ZN13codexmate_lib4core5relay10invariants1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..invariants..InvariantDim$GT$9serialize17h4dc3e1567704ea59E>:
100362c54: 39400009    	ldrb	w9, [x0]
100362c58: b0007d40    	adrp	x0, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362c5c: 912c0800    	add	x0, x0, #0xb02
100362c60: 7100053f    	cmp	w9, #0x1
100362c64: 5400016d    	b.le	0x100362c90 <__ZN13codexmate_lib4core5relay10invariants1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..invariants..InvariantDim$GT$9serialize17h4dc3e1567704ea59E+0x3c>
100362c68: 7100093f    	cmp	w9, #0x2
100362c6c: 54000200    	b.eq	0x100362cac <__ZN13codexmate_lib4core5relay10invariants1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..invariants..InvariantDim$GT$9serialize17h4dc3e1567704ea59E+0x58>
100362c70: 71000d3f    	cmp	w9, #0x3
100362c74: 54000281    	b.ne	0x100362cc4 <__ZN13codexmate_lib4core5relay10invariants1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..invariants..InvariantDim$GT$9serialize17h4dc3e1567704ea59E+0x70>
100362c78: b0007d43    	adrp	x3, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362c7c: 912c7c63    	add	x3, x3, #0xb1f
100362c80: 52800181    	mov	w1, #0xc                ; =12
100362c84: 52800062    	mov	w2, #0x3                ; =3
100362c88: 528000c4    	mov	w4, #0x6                ; =6
100362c8c: 14399ea5    	b	0x1011ca720 <__ZN87_$LT$serde_json..value..ser..RawValueEmitter$u20$as$u20$serde_core..ser..Serializer$GT$22serialize_unit_variant17h39b1cf38b19761c3E>
100362c90: 35000269    	cbnz	w9, 0x100362cdc <__ZN13codexmate_lib4core5relay10invariants1_109_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..invariants..InvariantDim$GT$9serialize17h4dc3e1567704ea59E+0x88>
100362c94: b0007d43    	adrp	x3, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362c98: 912c3863    	add	x3, x3, #0xb0e
100362c9c: 52800181    	mov	w1, #0xc                ; =12
100362ca0: 52800002    	mov	w2, #0x0                ; =0
100362ca4: 528000c4    	mov	w4, #0x6                ; =6
100362ca8: 14399e9e    	b	0x1011ca720 <__ZN87_$LT$serde_json..value..ser..RawValueEmitter$u20$as$u20$serde_core..ser..Serializer$GT$22serialize_unit_variant17h39b1cf38b19761c3E>
100362cac: b0007d43    	adrp	x3, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362cb0: 912c6063    	add	x3, x3, #0xb18
100362cb4: 52800181    	mov	w1, #0xc                ; =12
100362cb8: 52800042    	mov	w2, #0x2                ; =2
100362cbc: 528000e4    	mov	w4, #0x7                ; =7
100362cc0: 14399e98    	b	0x1011ca720 <__ZN87_$LT$serde_json..value..ser..RawValueEmitter$u20$as$u20$serde_core..ser..Serializer$GT$22serialize_unit_variant17h39b1cf38b19761c3E>
100362cc4: b0007d43    	adrp	x3, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362cc8: 912c9463    	add	x3, x3, #0xb25
100362ccc: 52800181    	mov	w1, #0xc                ; =12
100362cd0: 52800082    	mov	w2, #0x4                ; =4
100362cd4: 528000a4    	mov	w4, #0x5                ; =5
100362cd8: 14399e92    	b	0x1011ca720 <__ZN87_$LT$serde_json..value..ser..RawValueEmitter$u20$as$u20$serde_core..ser..Serializer$GT$22serialize_unit_variant17h39b1cf38b19761c3E>
100362cdc: b0007d43    	adrp	x3, 0x10130b000 <_anon.81615f03ac42cf09f1e5e0315a2480f1.1280+0x5f4>
100362ce0: 912c5063    	add	x3, x3, #0xb14
100362ce4: 52800181    	mov	w1, #0xc                ; =12
100362ce8: 52800022    	mov	w2, #0x1                ; =1
100362cec: 52800084    	mov	w4, #0x4                ; =4
100362cf0: 14399e8c    	b	0x1011ca720 <__ZN87_$LT$serde_json..value..ser..RawValueEmitter$u20$as$u20$serde_core..ser..Serializer$GT$22serialize_unit_variant17h39b1cf38b19761c3E>
