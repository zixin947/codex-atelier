
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001008f166c <__ZN240_$LT$codexmate_lib..core..relay..keychain..revisions.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd8e475a392db0396E>:
1008f166c: f100283f    	cmp	x1, #0xa
1008f1670: 54000340    	b.eq	0x1008f16d8 <__ZN240_$LT$codexmate_lib..core..relay..keychain..revisions.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd8e475a392db0396E+0x6c>
1008f1674: f100203f    	cmp	x1, #0x8
1008f1678: 540004e0    	b.eq	0x1008f1714 <__ZN240_$LT$codexmate_lib..core..relay..keychain..revisions.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd8e475a392db0396E+0xa8>
1008f167c: f100183f    	cmp	x1, #0x6
1008f1680: 54000661    	b.ne	0x1008f174c <__ZN240_$LT$codexmate_lib..core..relay..keychain..revisions.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd8e475a392db0396E+0xe0>
1008f1684: b9400009    	ldr	w9, [x0]
1008f1688: 7940080a    	ldrh	w10, [x0, #0x4]
1008f168c: 528deccb    	mov	w11, #0x6f66            ; =28518
1008f1690: 72adae4b    	movk	w11, #0x6d72, lsl #16
1008f1694: 6b0b013f    	cmp	w9, w11
1008f1698: 528e8c29    	mov	w9, #0x7461             ; =29793
1008f169c: 7a490140    	ccmp	w10, w9, #0x0, eq
1008f16a0: 54000520    	b.eq	0x1008f1744 <__ZN240_$LT$codexmate_lib..core..relay..keychain..revisions.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd8e475a392db0396E+0xd8>
1008f16a4: b9400009    	ldr	w9, [x0]
1008f16a8: 7940080a    	ldrh	w10, [x0, #0x4]
1008f16ac: 528e0c2b    	mov	w11, #0x7061            ; =28769
1008f16b0: 72a96d2b    	movk	w11, #0x4b69, lsl #16
1008f16b4: 6b0b013f    	cmp	w9, w11
1008f16b8: 528f2ca9    	mov	w9, #0x7965             ; =31077
1008f16bc: 7a490140    	ccmp	w10, w9, #0x0, eq
1008f16c0: 54000461    	b.ne	0x1008f174c <__ZN240_$LT$codexmate_lib..core..relay..keychain..revisions.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd8e475a392db0396E+0xe0>
1008f16c4: 52800009    	mov	w9, #0x0                ; =0
1008f16c8: 5280006a    	mov	w10, #0x3               ; =3
1008f16cc: 3900050a    	strb	w10, [x8, #0x1]
1008f16d0: 39000109    	strb	w9, [x8]
1008f16d4: d65f03c0    	ret
1008f16d8: f9400009    	ldr	x9, [x0]
1008f16dc: 7940100a    	ldrh	w10, [x0, #0x8]
1008f16e0: d28e4e0b    	mov	x11, #0x7270            ; =29296
1008f16e4: f2aecdeb    	movk	x11, #0x766f, lsl #16
1008f16e8: f2cc8d2b    	movk	x11, #0x6469, lsl #32
1008f16ec: f2ee4cab    	movk	x11, #0x7265, lsl #48
1008f16f0: eb0b013f    	cmp	x9, x11
1008f16f4: 528c8929    	mov	w9, #0x6449             ; =25673
1008f16f8: 7a490140    	ccmp	w10, w9, #0x0, eq
1008f16fc: 54000281    	b.ne	0x1008f174c <__ZN240_$LT$codexmate_lib..core..relay..keychain..revisions.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd8e475a392db0396E+0xe0>
1008f1700: 52800009    	mov	w9, #0x0                ; =0
1008f1704: 5280002a    	mov	w10, #0x1               ; =1
1008f1708: 3900050a    	strb	w10, [x8, #0x1]
1008f170c: 39000109    	strb	w9, [x8]
1008f1710: d65f03c0    	ret
1008f1714: f9400009    	ldr	x9, [x0]
1008f1718: d28cae4a    	mov	x10, #0x6572            ; =25970
1008f171c: f2ad2eca    	movk	x10, #0x6976, lsl #16
1008f1720: f2cd2e6a    	movk	x10, #0x6973, lsl #32
1008f1724: f2edcdea    	movk	x10, #0x6e6f, lsl #48
1008f1728: eb0a013f    	cmp	x9, x10
1008f172c: 54000101    	b.ne	0x1008f174c <__ZN240_$LT$codexmate_lib..core..relay..keychain..revisions.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..keychain..revisions..StoredKey$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17hd8e475a392db0396E+0xe0>
1008f1730: 52800009    	mov	w9, #0x0                ; =0
1008f1734: 5280004a    	mov	w10, #0x2               ; =2
1008f1738: 3900050a    	strb	w10, [x8, #0x1]
1008f173c: 39000109    	strb	w9, [x8]
1008f1740: d65f03c0    	ret
1008f1744: 7900011f    	strh	wzr, [x8]
1008f1748: d65f03c0    	ret
1008f174c: a9be4ff4    	stp	x20, x19, [sp, #-0x20]!
1008f1750: a9017bfd    	stp	x29, x30, [sp, #0x10]
1008f1754: 910043fd    	add	x29, sp, #0x10
1008f1758: b00079e2    	adrp	x2, 0x10182e000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.854+0x28>
1008f175c: 910de042    	add	x2, x2, #0x378
1008f1760: 52800083    	mov	w3, #0x4                ; =4
1008f1764: aa0803f3    	mov	x19, x8
1008f1768: 94226ba7    	bl	0x10118c604 <__ZN10serde_core2de5Error13unknown_field17hadbce53ee1f5a987E>
1008f176c: aa1303e8    	mov	x8, x19
1008f1770: f9000660    	str	x0, [x19, #0x8]
1008f1774: 52800029    	mov	w9, #0x1                ; =1
1008f1778: a9417bfd    	ldp	x29, x30, [sp, #0x10]
1008f177c: a8c24ff4    	ldp	x20, x19, [sp], #0x20
1008f1780: 39000109    	strb	w9, [x8]
1008f1784: d65f03c0    	ret

00000001008f1788 <__ZN46_$LT$T$u20$as$u20$serde_core..de..Expected$GT$3fmt17hc6e84accd7419712E>:
1008f1788: aa0103e0    	mov	x0, x1
1008f178c: d0005621    	adrp	x1, 0x1013b7000 <_anon.201a6070f17c8d3e3a2dcb35fe0dabc2.1235+0x547>
1008f1790: 912f2c21    	add	x1, x1, #0xbcb
1008f1794: 528003a2    	mov	w2, #0x1d               ; =29
1008f1798: 14223a65    	b	0x10118012c <__ZN57_$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$9write_str17h42904a4ad69a17fbE>

00000001008f179c <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE>:
1008f179c: f100203f    	cmp	x1, #0x8
1008f17a0: 5400036d    	b.le	0x1008f180c <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0x70>
1008f17a4: f100243f    	cmp	x1, #0x9
1008f17a8: 54000520    	b.eq	0x1008f184c <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0xb0>
1008f17ac: f100283f    	cmp	x1, #0xa
1008f17b0: 540006a0    	b.eq	0x1008f1884 <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0xe8>
1008f17b4: f100503f    	cmp	x1, #0x14
1008f17b8: 54000921    	b.ne	0x1008f18dc <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0x140>
1008f17bc: a9402809    	ldp	x9, x10, [x0]
1008f17c0: b940100b    	ldr	w11, [x0, #0x10]
1008f17c4: d28c2c4c    	mov	x12, #0x6162            ; =24930
1008f17c8: f2ad6c6c    	movk	x12, #0x6b63, lsl #16
1008f17cc: f2cc8cac    	movk	x12, #0x6465, lsl #32
1008f17d0: f2ee0aac    	movk	x12, #0x7055, lsl #48
1008f17d4: eb0c013f    	cmp	x9, x12
1008f17d8: d28e4a09    	mov	x9, #0x7250             ; =29264
1008f17dc: f2aecca9    	movk	x9, #0x7665, lsl #16
1008f17e0: f2cded29    	movk	x9, #0x6f69, lsl #32
1008f17e4: f2ee6ea9    	movk	x9, #0x7375, lsl #48
1008f17e8: fa490140    	ccmp	x10, x9, #0x0, eq
1008f17ec: 528ea829    	mov	w9, #0x7541             ; =30017
1008f17f0: 72ad0e89    	movk	w9, #0x6874, lsl #16
1008f17f4: 7a490160    	ccmp	w11, w9, #0x0, eq
1008f17f8: 54000721    	b.ne	0x1008f18dc <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0x140>
1008f17fc: 52800089    	mov	w9, #0x4                ; =4
1008f1800: 39000509    	strb	w9, [x8, #0x1]
1008f1804: 3900011f    	strb	wzr, [x8]
1008f1808: d65f03c0    	ret
1008f180c: f100143f    	cmp	x1, #0x5
1008f1810: 54000560    	b.eq	0x1008f18bc <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0x120>
1008f1814: f100183f    	cmp	x1, #0x6
1008f1818: 54000621    	b.ne	0x1008f18dc <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0x140>
1008f181c: b9400009    	ldr	w9, [x0]
1008f1820: 7940080a    	ldrh	w10, [x0, #0x4]
1008f1824: 528e0c2b    	mov	w11, #0x7061            ; =28769
1008f1828: 72a96d2b    	movk	w11, #0x4b69, lsl #16
1008f182c: 6b0b013f    	cmp	w9, w11
1008f1830: 528f2ca9    	mov	w9, #0x7965             ; =31077
1008f1834: 7a490140    	ccmp	w10, w9, #0x0, eq
1008f1838: 54000521    	b.ne	0x1008f18dc <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0x140>
1008f183c: 52800009    	mov	w9, #0x0                ; =0
1008f1840: 39000509    	strb	w9, [x8, #0x1]
1008f1844: 3900011f    	strb	wzr, [x8]
1008f1848: d65f03c0    	ret
1008f184c: f9400009    	ldr	x9, [x0]
1008f1850: 3940200a    	ldrb	w10, [x0, #0x8]
1008f1854: d28e4c6b    	mov	x11, #0x7263            ; =29283
1008f1858: f2ac2cab    	movk	x11, #0x6165, lsl #16
1008f185c: f2ccae8b    	movk	x11, #0x6574, lsl #32
1008f1860: f2e82c8b    	movk	x11, #0x4164, lsl #48
1008f1864: eb0b013f    	cmp	x9, x11
1008f1868: 52800e89    	mov	w9, #0x74               ; =116
1008f186c: 7a490140    	ccmp	w10, w9, #0x0, eq
1008f1870: 54000361    	b.ne	0x1008f18dc <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0x140>
1008f1874: 52800069    	mov	w9, #0x3                ; =3
1008f1878: 39000509    	strb	w9, [x8, #0x1]
1008f187c: 3900011f    	strb	wzr, [x8]
1008f1880: d65f03c0    	ret
1008f1884: f9400009    	ldr	x9, [x0]
1008f1888: 7940100a    	ldrh	w10, [x0, #0x8]
1008f188c: d28c6c2b    	mov	x11, #0x6361            ; =25441
1008f1890: f2adec6b    	movk	x11, #0x6f63, lsl #16
1008f1894: f2cdceab    	movk	x11, #0x6e75, lsl #32
1008f1898: f2e96e8b    	movk	x11, #0x4b74, lsl #48
1008f189c: eb0b013f    	cmp	x9, x11
1008f18a0: 528f2ca9    	mov	w9, #0x7965             ; =31077
1008f18a4: 7a490140    	ccmp	w10, w9, #0x0, eq
1008f18a8: 540001a1    	b.ne	0x1008f18dc <__ZN251_$LT$codexmate_lib..core..relay..router_unlock_auth.._..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$codexmate_lib..core..relay..router_unlock_auth..RouterUnlockAuthMarker$GT$..deserialize..__FieldVisitor$u20$as$u20$serde_core..de..Visitor$GT$9visit_str17h4cbdb7ad7673a6adE+0x140>
1008f18ac: 52800049    	mov	w9, #0x2                ; =2
1008f18b0: 39000509    	strb	w9, [x8, #0x1]
1008f18b4: 3900011f    	strb	wzr, [x8]
1008f18b8: d65f03c0    	ret
1008f18bc: b9400009    	ldr	w9, [x0]
