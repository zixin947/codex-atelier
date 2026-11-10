
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010079f9a4 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE>:
10079f9a4: d10103ff    	sub	sp, sp, #0x40
10079f9a8: a9024ff4    	stp	x20, x19, [sp, #0x20]
10079f9ac: a9037bfd    	stp	x29, x30, [sp, #0x30]
10079f9b0: 9100c3fd    	add	x29, sp, #0x30
10079f9b4: aa0803f3    	mov	x19, x8
10079f9b8: 910023e8    	add	x8, sp, #0x8
10079f9bc: 97eac0d3    	bl	0x10024fd08 <__ZN98_$LT$$RF$mut$u20$serde_json..de..Deserializer$LT$R$GT$$u20$as$u20$serde_core..de..Deserializer$GT$18deserialize_string17hcb7db36cab4976ffE>
10079f9c0: a94083e1    	ldp	x1, x0, [sp, #0x8]
10079f9c4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10079f9c8: eb08003f    	cmp	x1, x8
10079f9cc: 54001d80    	b.eq	0x10079fd7c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3d8>
10079f9d0: f9400fe8    	ldr	x8, [sp, #0x18]
10079f9d4: f100811f    	cmp	x8, #0x20
10079f9d8: 54001c21    	b.ne	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079f9dc: 39400008    	ldrb	w8, [x0]
10079f9e0: 5100e909    	sub	w9, w8, #0x3a
10079f9e4: 51019d08    	sub	w8, w8, #0x67
10079f9e8: 31002d3f    	cmn	w9, #0xb
10079f9ec: 3a469902    	ccmn	w8, #0x6, #0x2, ls
10079f9f0: 54001b63    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079f9f4: 39400408    	ldrb	w8, [x0, #0x1]
10079f9f8: 5100e909    	sub	w9, w8, #0x3a
10079f9fc: 51019d08    	sub	w8, w8, #0x67
10079fa00: 31002d3f    	cmn	w9, #0xb
10079fa04: 3a469902    	ccmn	w8, #0x6, #0x2, ls
10079fa08: 54001aa3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fa0c: 39400808    	ldrb	w8, [x0, #0x2]
10079fa10: 5100e909    	sub	w9, w8, #0x3a
10079fa14: 51019d08    	sub	w8, w8, #0x67
10079fa18: 31002d3f    	cmn	w9, #0xb
10079fa1c: 3a469902    	ccmn	w8, #0x6, #0x2, ls
10079fa20: 540019e3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fa24: 39400c08    	ldrb	w8, [x0, #0x3]
10079fa28: 5100e909    	sub	w9, w8, #0x3a
10079fa2c: 51019d08    	sub	w8, w8, #0x67
10079fa30: 31002d3f    	cmn	w9, #0xb
10079fa34: 3a469902    	ccmn	w8, #0x6, #0x2, ls
10079fa38: 54001923    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fa3c: 39401008    	ldrb	w8, [x0, #0x4]
10079fa40: 5100e909    	sub	w9, w8, #0x3a
10079fa44: 31002d3f    	cmn	w9, #0xb
10079fa48: 54000088    	b.hi	0x10079fa58 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0xb4>
10079fa4c: 51019d08    	sub	w8, w8, #0x67
10079fa50: 3100191f    	cmn	w8, #0x6
10079fa54: 54001843    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fa58: 39401408    	ldrb	w8, [x0, #0x5]
10079fa5c: 5100e909    	sub	w9, w8, #0x3a
10079fa60: 31002d3f    	cmn	w9, #0xb
10079fa64: 54000088    	b.hi	0x10079fa74 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0xd0>
10079fa68: 51019d08    	sub	w8, w8, #0x67
10079fa6c: 3100191f    	cmn	w8, #0x6
10079fa70: 54001763    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fa74: 39401808    	ldrb	w8, [x0, #0x6]
10079fa78: 5100e909    	sub	w9, w8, #0x3a
10079fa7c: 31002d3f    	cmn	w9, #0xb
10079fa80: 54000088    	b.hi	0x10079fa90 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0xec>
10079fa84: 51019d08    	sub	w8, w8, #0x67
10079fa88: 3100191f    	cmn	w8, #0x6
10079fa8c: 54001683    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fa90: 39401c08    	ldrb	w8, [x0, #0x7]
10079fa94: 5100e909    	sub	w9, w8, #0x3a
10079fa98: 31002d3f    	cmn	w9, #0xb
10079fa9c: 54000088    	b.hi	0x10079faac <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x108>
10079faa0: 51019d08    	sub	w8, w8, #0x67
10079faa4: 3100191f    	cmn	w8, #0x6
10079faa8: 540015a3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079faac: 39402008    	ldrb	w8, [x0, #0x8]
10079fab0: 5100e909    	sub	w9, w8, #0x3a
10079fab4: 31002d3f    	cmn	w9, #0xb
10079fab8: 54000088    	b.hi	0x10079fac8 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x124>
10079fabc: 51019d08    	sub	w8, w8, #0x67
10079fac0: 3100191f    	cmn	w8, #0x6
10079fac4: 540014c3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fac8: 39402408    	ldrb	w8, [x0, #0x9]
10079facc: 5100e909    	sub	w9, w8, #0x3a
10079fad0: 31002d3f    	cmn	w9, #0xb
10079fad4: 54000088    	b.hi	0x10079fae4 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x140>
10079fad8: 51019d08    	sub	w8, w8, #0x67
10079fadc: 3100191f    	cmn	w8, #0x6
10079fae0: 540013e3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fae4: 39402808    	ldrb	w8, [x0, #0xa]
10079fae8: 5100e909    	sub	w9, w8, #0x3a
10079faec: 31002d3f    	cmn	w9, #0xb
10079faf0: 54000088    	b.hi	0x10079fb00 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x15c>
10079faf4: 51019d08    	sub	w8, w8, #0x67
10079faf8: 3100191f    	cmn	w8, #0x6
10079fafc: 54001303    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fb00: 39402c08    	ldrb	w8, [x0, #0xb]
10079fb04: 5100e909    	sub	w9, w8, #0x3a
10079fb08: 31002d3f    	cmn	w9, #0xb
10079fb0c: 54000088    	b.hi	0x10079fb1c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x178>
10079fb10: 51019d08    	sub	w8, w8, #0x67
10079fb14: 3100191f    	cmn	w8, #0x6
10079fb18: 54001223    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fb1c: 39403008    	ldrb	w8, [x0, #0xc]
10079fb20: 5100e909    	sub	w9, w8, #0x3a
10079fb24: 31002d3f    	cmn	w9, #0xb
10079fb28: 54000088    	b.hi	0x10079fb38 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x194>
10079fb2c: 51019d08    	sub	w8, w8, #0x67
10079fb30: 3100191f    	cmn	w8, #0x6
10079fb34: 54001143    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fb38: 39403408    	ldrb	w8, [x0, #0xd]
10079fb3c: 5100e909    	sub	w9, w8, #0x3a
10079fb40: 31002d3f    	cmn	w9, #0xb
10079fb44: 54000088    	b.hi	0x10079fb54 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x1b0>
10079fb48: 51019d08    	sub	w8, w8, #0x67
10079fb4c: 3100191f    	cmn	w8, #0x6
10079fb50: 54001063    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fb54: 39403808    	ldrb	w8, [x0, #0xe]
10079fb58: 5100e909    	sub	w9, w8, #0x3a
10079fb5c: 31002d3f    	cmn	w9, #0xb
10079fb60: 54000088    	b.hi	0x10079fb70 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x1cc>
10079fb64: 51019d08    	sub	w8, w8, #0x67
10079fb68: 3100191f    	cmn	w8, #0x6
10079fb6c: 54000f83    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fb70: 39403c08    	ldrb	w8, [x0, #0xf]
10079fb74: 5100e909    	sub	w9, w8, #0x3a
10079fb78: 31002d3f    	cmn	w9, #0xb
10079fb7c: 54000088    	b.hi	0x10079fb8c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x1e8>
10079fb80: 51019d08    	sub	w8, w8, #0x67
10079fb84: 3100191f    	cmn	w8, #0x6
10079fb88: 54000ea3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fb8c: 39404008    	ldrb	w8, [x0, #0x10]
10079fb90: 5100e909    	sub	w9, w8, #0x3a
10079fb94: 31002d3f    	cmn	w9, #0xb
10079fb98: 54000088    	b.hi	0x10079fba8 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x204>
10079fb9c: 51019d08    	sub	w8, w8, #0x67
10079fba0: 3100191f    	cmn	w8, #0x6
10079fba4: 54000dc3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fba8: 39404408    	ldrb	w8, [x0, #0x11]
10079fbac: 5100e909    	sub	w9, w8, #0x3a
10079fbb0: 31002d3f    	cmn	w9, #0xb
10079fbb4: 54000088    	b.hi	0x10079fbc4 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x220>
10079fbb8: 51019d08    	sub	w8, w8, #0x67
10079fbbc: 3100191f    	cmn	w8, #0x6
10079fbc0: 54000ce3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fbc4: 39404808    	ldrb	w8, [x0, #0x12]
10079fbc8: 5100e909    	sub	w9, w8, #0x3a
10079fbcc: 31002d3f    	cmn	w9, #0xb
10079fbd0: 54000088    	b.hi	0x10079fbe0 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x23c>
10079fbd4: 51019d08    	sub	w8, w8, #0x67
10079fbd8: 3100191f    	cmn	w8, #0x6
10079fbdc: 54000c03    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fbe0: 39404c08    	ldrb	w8, [x0, #0x13]
10079fbe4: 5100e909    	sub	w9, w8, #0x3a
10079fbe8: 31002d3f    	cmn	w9, #0xb
10079fbec: 54000088    	b.hi	0x10079fbfc <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x258>
10079fbf0: 51019d08    	sub	w8, w8, #0x67
10079fbf4: 3100191f    	cmn	w8, #0x6
10079fbf8: 54000b23    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fbfc: 39405008    	ldrb	w8, [x0, #0x14]
10079fc00: 5100e909    	sub	w9, w8, #0x3a
10079fc04: 31002d3f    	cmn	w9, #0xb
10079fc08: 54000088    	b.hi	0x10079fc18 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x274>
10079fc0c: 51019d08    	sub	w8, w8, #0x67
10079fc10: 3100191f    	cmn	w8, #0x6
10079fc14: 54000a43    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fc18: 39405408    	ldrb	w8, [x0, #0x15]
10079fc1c: 5100e909    	sub	w9, w8, #0x3a
10079fc20: 31002d3f    	cmn	w9, #0xb
10079fc24: 54000088    	b.hi	0x10079fc34 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x290>
10079fc28: 51019d08    	sub	w8, w8, #0x67
10079fc2c: 3100191f    	cmn	w8, #0x6
10079fc30: 54000963    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fc34: 39405808    	ldrb	w8, [x0, #0x16]
10079fc38: 5100e909    	sub	w9, w8, #0x3a
10079fc3c: 31002d3f    	cmn	w9, #0xb
10079fc40: 54000088    	b.hi	0x10079fc50 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x2ac>
10079fc44: 51019d08    	sub	w8, w8, #0x67
10079fc48: 3100191f    	cmn	w8, #0x6
10079fc4c: 54000883    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fc50: 39405c08    	ldrb	w8, [x0, #0x17]
10079fc54: 5100e909    	sub	w9, w8, #0x3a
10079fc58: 31002d3f    	cmn	w9, #0xb
10079fc5c: 54000088    	b.hi	0x10079fc6c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x2c8>
10079fc60: 51019d08    	sub	w8, w8, #0x67
10079fc64: 3100191f    	cmn	w8, #0x6
10079fc68: 540007a3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fc6c: 39406008    	ldrb	w8, [x0, #0x18]
10079fc70: 5100e909    	sub	w9, w8, #0x3a
10079fc74: 31002d3f    	cmn	w9, #0xb
10079fc78: 54000088    	b.hi	0x10079fc88 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x2e4>
10079fc7c: 51019d08    	sub	w8, w8, #0x67
10079fc80: 3100191f    	cmn	w8, #0x6
10079fc84: 540006c3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fc88: 39406408    	ldrb	w8, [x0, #0x19]
10079fc8c: 5100e909    	sub	w9, w8, #0x3a
10079fc90: 31002d3f    	cmn	w9, #0xb
10079fc94: 54000088    	b.hi	0x10079fca4 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x300>
10079fc98: 51019d08    	sub	w8, w8, #0x67
10079fc9c: 3100191f    	cmn	w8, #0x6
10079fca0: 540005e3    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fca4: 39406808    	ldrb	w8, [x0, #0x1a]
10079fca8: 5100e909    	sub	w9, w8, #0x3a
10079fcac: 31002d3f    	cmn	w9, #0xb
10079fcb0: 54000088    	b.hi	0x10079fcc0 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x31c>
10079fcb4: 51019d08    	sub	w8, w8, #0x67
10079fcb8: 3100191f    	cmn	w8, #0x6
10079fcbc: 54000503    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fcc0: 39406c08    	ldrb	w8, [x0, #0x1b]
10079fcc4: 5100e909    	sub	w9, w8, #0x3a
10079fcc8: 31002d3f    	cmn	w9, #0xb
10079fccc: 54000088    	b.hi	0x10079fcdc <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x338>
10079fcd0: 51019d08    	sub	w8, w8, #0x67
10079fcd4: 3100191f    	cmn	w8, #0x6
10079fcd8: 54000423    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fcdc: 39407008    	ldrb	w8, [x0, #0x1c]
10079fce0: 5100e909    	sub	w9, w8, #0x3a
10079fce4: 31002d3f    	cmn	w9, #0xb
10079fce8: 54000088    	b.hi	0x10079fcf8 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x354>
10079fcec: 51019d08    	sub	w8, w8, #0x67
10079fcf0: 3100191f    	cmn	w8, #0x6
10079fcf4: 54000343    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fcf8: 39407408    	ldrb	w8, [x0, #0x1d]
10079fcfc: 5100e909    	sub	w9, w8, #0x3a
10079fd00: 31002d3f    	cmn	w9, #0xb
10079fd04: 54000088    	b.hi	0x10079fd14 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x370>
10079fd08: 51019d08    	sub	w8, w8, #0x67
10079fd0c: 3100191f    	cmn	w8, #0x6
10079fd10: 54000263    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fd14: 39407808    	ldrb	w8, [x0, #0x1e]
10079fd18: 5100e909    	sub	w9, w8, #0x3a
10079fd1c: 31002d3f    	cmn	w9, #0xb
10079fd20: 54000088    	b.hi	0x10079fd30 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x38c>
10079fd24: 51019d08    	sub	w8, w8, #0x67
10079fd28: 3100191f    	cmn	w8, #0x6
10079fd2c: 54000183    	b.lo	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fd30: 39407c08    	ldrb	w8, [x0, #0x1f]
10079fd34: 5100e909    	sub	w9, w8, #0x3a
10079fd38: 31002d3f    	cmn	w9, #0xb
10079fd3c: 54000088    	b.hi	0x10079fd4c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3a8>
10079fd40: 51019d08    	sub	w8, w8, #0x67
10079fd44: 31001d1f    	cmn	w8, #0x7
10079fd48: 540000a9    	b.ls	0x10079fd5c <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3b8>
10079fd4c: a9000261    	stp	x1, x0, [x19]
10079fd50: 52800408    	mov	w8, #0x20               ; =32
10079fd54: f9000a68    	str	x8, [x19, #0x10]
10079fd58: 1400000a    	b	0x10079fd80 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3dc>
10079fd5c: b4000061    	cbz	x1, 0x10079fd68 <__ZN13codexmate_lib4core5relay6models1_112_$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..models..CredentialRevision$GT$11deserialize17hbdaaa9403842fb0eE+0x3c4>
10079fd60: 52800022    	mov	w2, #0x1                ; =1
10079fd64: 97e18601    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10079fd68: d0006040    	adrp	x0, 0x1013a9000 <_anon.cdb272158a04aae91ce02c6e47f106c7.539+0xd4>
10079fd6c: 910bbc00    	add	x0, x0, #0x2ef
10079fd70: 52800421    	mov	w1, #0x21               ; =33
10079fd74: 9427b3b0    	bl	0x10118cc34 <__ZN67_$LT$serde_json..error..Error$u20$as$u20$serde_core..ser..Error$GT$6custom17hd308ce5bb9ca103cE>
10079fd78: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10079fd7c: a9000268    	stp	x8, x0, [x19]
10079fd80: a9437bfd    	ldp	x29, x30, [sp, #0x30]
10079fd84: a9424ff4    	ldp	x20, x19, [sp, #0x20]
10079fd88: 910103ff    	add	sp, sp, #0x40
10079fd8c: d65f03c0    	ret
