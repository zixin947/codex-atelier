
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000001007a2854 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE>:
1007a2854: d10383ff    	sub	sp, sp, #0xe0
1007a2858: a90a5ff8    	stp	x24, x23, [sp, #0xa0]
1007a285c: a90b57f6    	stp	x22, x21, [sp, #0xb0]
1007a2860: a90c4ff4    	stp	x20, x19, [sp, #0xc0]
1007a2864: a90d7bfd    	stp	x29, x30, [sp, #0xd0]
1007a2868: 910343fd    	add	x29, sp, #0xd0
1007a286c: aa0803f3    	mov	x19, x8
1007a2870: 910103e8    	add	x8, sp, #0x40
1007a2874: 940ed90a    	bl	0x100b58c9c <__ZN18security_framework17passwords_options15PasswordOptions20new_generic_password17ha3128dc48542e9cbE>
1007a2878: 910103e0    	add	x0, sp, #0x40
1007a287c: 940ed9b4    	bl	0x100b58f4c <__ZN18security_framework17passwords_options15PasswordOptions22use_protected_keychain17hfb39461c3be7b537E>
1007a2880: 3dc013e0    	ldr	q0, [sp, #0x40]
1007a2884: 3c9b03a0    	stur	q0, [x29, #-0x50]
1007a2888: f9402be8    	ldr	x8, [sp, #0x50]
1007a288c: f81c03a8    	stur	x8, [x29, #-0x40]
1007a2890: 910023f7    	add	x23, sp, #0x8
1007a2894: 910023e8    	add	x8, sp, #0x8
1007a2898: d10143a0    	sub	x0, x29, #0x50
1007a289c: 940ed3c6    	bl	0x100b577b4 <__ZN18security_framework9passwords16generic_password17h473e7692ce6bf6c9E>
1007a28a0: f94007f4    	ldr	x20, [sp, #0x8]
1007a28a4: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1007a28a8: eb08029f    	cmp	x20, x8
1007a28ac: 54000121    	b.ne	0x1007a28d0 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x7c>
1007a28b0: b94013e8    	ldr	w8, [sp, #0x10]
1007a28b4: 128c5a69    	mov	w9, #-0x62d4            ; =-25300
1007a28b8: 6b09011f    	cmp	w8, w9
1007a28bc: 54000741    	b.ne	0x1007a29a4 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x150>
1007a28c0: 90005b08    	adrp	x8, 0x101302000 <_writev+0x101302000>
1007a28c4: 3dc03500    	ldr	q0, [x8, #0xd0]
1007a28c8: 3d800260    	str	q0, [x19]
1007a28cc: 14000030    	b	0x1007a298c <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x138>
1007a28d0: a9415bf5    	ldp	x21, x22, [sp, #0x10]
1007a28d4: 910103e8    	add	x8, sp, #0x40
1007a28d8: aa1503e0    	mov	x0, x21
1007a28dc: aa1603e1    	mov	x1, x22
1007a28e0: 94278e1c    	bl	0x101186150 <__ZN4core3str8converts9from_utf817h3d3f04c61e7c2c97E>
1007a28e4: b94043e8    	ldr	w8, [sp, #0x40]
1007a28e8: 7100051f    	cmp	w8, #0x1
1007a28ec: 54000481    	b.ne	0x1007a297c <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x128>
1007a28f0: 3cc483e0    	ldur	q0, [sp, #0x48]
1007a28f4: 3cc082e1    	ldur	q1, [x23, #0x8]
1007a28f8: 3c8483e1    	stur	q1, [sp, #0x48]
1007a28fc: f90023f4    	str	x20, [sp, #0x40]
1007a2900: 3c8583e0    	stur	q0, [sp, #0x58]
1007a2904: 52800028    	mov	w8, #0x1                ; =1
1007a2908: a906a3ff    	stp	xzr, x8, [sp, #0x68]
1007a290c: f9003fff    	str	xzr, [sp, #0x78]
1007a2910: 52800408    	mov	w8, #0x20               ; =32
1007a2914: 72ac0008    	movk	w8, #0x6000, lsl #16
1007a2918: f81c03a8    	stur	x8, [x29, #-0x40]
1007a291c: 9101a3e8    	add	x8, sp, #0x68
1007a2920: b0008449    	adrp	x9, 0x10182b000 <_anon.6408540cdffbe318d49f52812e99783f.784+0x28>
1007a2924: 9133e129    	add	x9, x9, #0xcf8
1007a2928: a93b27a8    	stp	x8, x9, [x29, #-0x50]
1007a292c: 910103e0    	add	x0, sp, #0x40
1007a2930: d10143a1    	sub	x1, x29, #0x50
1007a2934: 94275b59    	bl	0x101179698 <__ZN67_$LT$alloc..string..FromUtf8Error$u20$as$u20$core..fmt..Display$GT$3fmt17h59ea02b46e4c723fE>
1007a2938: 35000400    	cbnz	w0, 0x1007a29b8 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x164>
1007a293c: 3cc683e0    	ldur	q0, [sp, #0x68]
1007a2940: 3d800be0    	str	q0, [sp, #0x20]
1007a2944: f9403fe8    	ldr	x8, [sp, #0x78]
1007a2948: f9001be8    	str	x8, [sp, #0x30]
1007a294c: b40000b4    	cbz	x20, 0x1007a2960 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x10c>
1007a2950: f94027e0    	ldr	x0, [sp, #0x48]
1007a2954: aa1403e1    	mov	x1, x20
1007a2958: 52800022    	mov	w2, #0x1                ; =1
1007a295c: 97e17b03    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007a2960: 3dc00be0    	ldr	q0, [sp, #0x20]
1007a2964: 3c808260    	stur	q0, [x19, #0x8]
1007a2968: f9401be8    	ldr	x8, [sp, #0x30]
1007a296c: f9000e68    	str	x8, [x19, #0x18]
1007a2970: 52800028    	mov	w8, #0x1                ; =1
1007a2974: f9000268    	str	x8, [x19]
1007a2978: 14000005    	b	0x1007a298c <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x138>
1007a297c: 3cc083e0    	ldur	q0, [sp, #0x8]
1007a2980: 3c808260    	stur	q0, [x19, #0x8]
1007a2984: f9000e76    	str	x22, [x19, #0x18]
1007a2988: f900027f    	str	xzr, [x19]
1007a298c: a94d7bfd    	ldp	x29, x30, [sp, #0xd0]
1007a2990: a94c4ff4    	ldp	x20, x19, [sp, #0xc0]
1007a2994: a94b57f6    	ldp	x22, x21, [sp, #0xb0]
1007a2998: a94a5ff8    	ldp	x24, x23, [sp, #0xa0]
1007a299c: 910383ff    	add	sp, sp, #0xe0
1007a29a0: d65f03c0    	ret
1007a29a4: b90043e8    	str	w8, [sp, #0x40]
1007a29a8: 91002260    	add	x0, x19, #0x8
1007a29ac: 910103e1    	add	x1, sp, #0x40
1007a29b0: 9427b23d    	bl	0x10118f2a4 <__ZN49_$LT$T$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h3428d441d41b5853E>
1007a29b4: 17ffffef    	b	0x1007a2970 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x11c>
1007a29b8: 90006060    	adrp	x0, 0x1013ae000 <_anon.6408540cdffbe318d49f52812e99783f.1450+0xb0>
1007a29bc: 91173400    	add	x0, x0, #0x5cd
1007a29c0: b0008443    	adrp	x3, 0x10182b000 <_anon.6408540cdffbe318d49f52812e99783f.784+0x28>
1007a29c4: 91382063    	add	x3, x3, #0xe08
1007a29c8: b0008444    	adrp	x4, 0x10182b000 <_anon.6408540cdffbe318d49f52812e99783f.784+0x28>
1007a29cc: 9134a084    	add	x4, x4, #0xd28
1007a29d0: 910083e2    	add	x2, sp, #0x20
1007a29d4: 528006e1    	mov	w1, #0x37               ; =55
1007a29d8: 9428b808    	bl	0x1011d09f8 <__ZN4core6result13unwrap_failed17hfd1628134417d023E>
1007a29dc: d4200020    	brk	#0x1
1007a29e0: aa0003f3    	mov	x19, x0
1007a29e4: b50001f4    	cbnz	x20, 0x1007a2a20 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x1cc>
1007a29e8: 14000012    	b	0x1007a2a30 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x1dc>
1007a29ec: aa0003f3    	mov	x19, x0
1007a29f0: 910103e0    	add	x0, sp, #0x40
1007a29f4: 940140e2    	bl	0x1007f2d7c <__ZN4core3ptr75drop_in_place$LT$security_framework..passwords_options..PasswordOptions$GT$17hc301fa283ba7237cE>
1007a29f8: 1400000e    	b	0x1007a2a30 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x1dc>
1007a29fc: 9428b853    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
1007a2a00: aa0003f3    	mov	x19, x0
1007a2a04: f94037e1    	ldr	x1, [sp, #0x68]
1007a2a08: b4000081    	cbz	x1, 0x1007a2a18 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x1c4>
1007a2a0c: f9403be0    	ldr	x0, [sp, #0x70]
1007a2a10: 52800022    	mov	w2, #0x1                ; =1
1007a2a14: 97e17ad5    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007a2a18: b40000d4    	cbz	x20, 0x1007a2a30 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain3get17h34a1e026ed63f96cE+0x1dc>
1007a2a1c: f94027f5    	ldr	x21, [sp, #0x48]
1007a2a20: aa1503e0    	mov	x0, x21
1007a2a24: aa1403e1    	mov	x1, x20
1007a2a28: 52800022    	mov	w2, #0x1                ; =1
1007a2a2c: 97e17acf    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
1007a2a30: aa1303e0    	mov	x0, x19
1007a2a34: 9428b9c8    	bl	0x1011d1154 <_writev+0x1011d1154>

00000001007a2a38 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain6delete17h34da6b4470f8284fE>:
1007a2a38: d10143ff    	sub	sp, sp, #0x50
1007a2a3c: a9034ff4    	stp	x20, x19, [sp, #0x30]
1007a2a40: a9047bfd    	stp	x29, x30, [sp, #0x40]
1007a2a44: 910103fd    	add	x29, sp, #0x40
1007a2a48: aa0803f3    	mov	x19, x8
1007a2a4c: 910063e8    	add	x8, sp, #0x18
1007a2a50: 940ed893    	bl	0x100b58c9c <__ZN18security_framework17passwords_options15PasswordOptions20new_generic_password17ha3128dc48542e9cbE>
1007a2a54: 910063e0    	add	x0, sp, #0x18
1007a2a58: 940ed93d    	bl	0x100b58f4c <__ZN18security_framework17passwords_options15PasswordOptions22use_protected_keychain17hfb39461c3be7b537E>
1007a2a5c: 3cc183e0    	ldur	q0, [sp, #0x18]
1007a2a60: 3d8003e0    	str	q0, [sp]
1007a2a64: f94017e8    	ldr	x8, [sp, #0x28]
1007a2a68: f9000be8    	str	x8, [sp, #0x10]
1007a2a6c: 910003e0    	mov	x0, sp
1007a2a70: 940ed410    	bl	0x100b57ab0 <__ZN18security_framework9passwords31delete_generic_password_options17hc77e53893dfb9a25E>
1007a2a74: 128c5a68    	mov	w8, #-0x62d4            ; =-25300
1007a2a78: 6b08001f    	cmp	w0, w8
1007a2a7c: 54000040    	b.eq	0x1007a2a84 <__ZN13codexmate_lib4core5relay8keychain12mac_keychain6delete17h34da6b4470f8284fE+0x4c>
1007a2a80: 350000e0    	cbnz	w0, 0x1007a2a9c <__ZN13codexmate_lib4core5relay8keychain12mac_keychain6delete17h34da6b4470f8284fE+0x64>
1007a2a84: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
1007a2a88: f9000268    	str	x8, [x19]
1007a2a8c: a9447bfd    	ldp	x29, x30, [sp, #0x40]
1007a2a90: a9434ff4    	ldp	x20, x19, [sp, #0x30]
1007a2a94: 910143ff    	add	sp, sp, #0x50
1007a2a98: d65f03c0    	ret
1007a2a9c: b9001be0    	str	w0, [sp, #0x18]
1007a2aa0: 910063e1    	add	x1, sp, #0x18
1007a2aa4: aa1303e0    	mov	x0, x19
1007a2aa8: 9427b1ff    	bl	0x10118f2a4 <__ZN49_$LT$T$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h3428d441d41b5853E>
1007a2aac: 17fffff8    	b	0x1007a2a8c <__ZN13codexmate_lib4core5relay8keychain12mac_keychain6delete17h34da6b4470f8284fE+0x54>
1007a2ab0: aa0003f3    	mov	x19, x0
1007a2ab4: 910063e0    	add	x0, sp, #0x18
1007a2ab8: 940140b1    	bl	0x1007f2d7c <__ZN4core3ptr75drop_in_place$LT$security_framework..passwords_options..PasswordOptions$GT$17hc301fa283ba7237cE>
1007a2abc: aa1303e0    	mov	x0, x19
1007a2ac0: 9428b9a5    	bl	0x1011d1154 <_writev+0x1011d1154>
1007a2ac4: 9428b821    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

00000001007a2ac8 <__ZN13codexmate_lib4core6models1_100_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..models..AccountSummary$GT$9serialize17ha921111109d021aaE>:
1007a2ac8: d101c3ff    	sub	sp, sp, #0x70
1007a2acc: a90267fa    	stp	x26, x25, [sp, #0x20]
1007a2ad0: a9035ff8    	stp	x24, x23, [sp, #0x30]
1007a2ad4: a90457f6    	stp	x22, x21, [sp, #0x40]
1007a2ad8: a9054ff4    	stp	x20, x19, [sp, #0x50]
1007a2adc: a9067bfd    	stp	x29, x30, [sp, #0x60]
1007a2ae0: 910183fd    	add	x29, sp, #0x60
1007a2ae4: aa0003f4    	mov	x20, x0
1007a2ae8: aa0003f3    	mov	x19, x0
1007a2aec: f84a0e77    	ldr	x23, [x19, #0xa0]!
1007a2af0: f9400035    	ldr	x21, [x1]
1007a2af4: f9400c28    	ldr	x8, [x1, #0x18]
1007a2af8: 91000508    	add	x8, x8, #0x1
1007a2afc: f9000c28    	str	x8, [x1, #0x18]
1007a2b00: 3900803f    	strb	wzr, [x1, #0x20]
1007a2b04: f9400aa8    	ldr	x8, [x21, #0x10]
1007a2b08: f94002a9    	ldr	x9, [x21]
1007a2b0c: eb08013f    	cmp	x9, x8
1007a2b10: 54002140    	b.eq	0x1007a2f38 <__ZN13codexmate_lib4core6models1_100_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..models..AccountSummary$GT$9serialize17ha921111109d021aaE+0x470>
1007a2b14: f94006a9    	ldr	x9, [x21, #0x8]
1007a2b18: 52800f6a    	mov	w10, #0x7b              ; =123
1007a2b1c: 3828692a    	strb	w10, [x9, x8]
1007a2b20: 91000508    	add	x8, x8, #0x1
1007a2b24: f9000aa8    	str	x8, [x21, #0x10]
1007a2b28: 52802008    	mov	w8, #0x100              ; =256
1007a2b2c: 790003e8    	strh	w8, [sp]
1007a2b30: f90007e1    	str	x1, [sp, #0x8]
1007a2b34: d0006041    	adrp	x1, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
1007a2b38: 9118c021    	add	x1, x1, #0x630
1007a2b3c: 910003e0    	mov	x0, sp
1007a2b40: 52800142    	mov	w2, #0xa                ; =10
1007a2b44: 97f71736    	bl	0x10056881c <__ZN88_$LT$serde_json..ser..Compound$LT$W$C$F$GT$$u20$as$u20$serde_core..ser..SerializeMap$GT$13serialize_key17h36fbd8dcee114b15E>
1007a2b48: 394003e8    	ldrb	w8, [sp]
1007a2b4c: 7100051f    	cmp	w8, #0x1
1007a2b50: 54001e80    	b.eq	0x1007a2f20 <__ZN13codexmate_lib4core6models1_100_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..models..AccountSummary$GT$9serialize17ha921111109d021aaE+0x458>
1007a2b54: f94007f5    	ldr	x21, [sp, #0x8]
1007a2b58: a94d8e82    	ldp	x2, x3, [x20, #0xd8]
1007a2b5c: f94002b6    	ldr	x22, [x21]
1007a2b60: f9400ac1    	ldr	x1, [x22, #0x10]
1007a2b64: f94002c8    	ldr	x8, [x22]
1007a2b68: cb010108    	sub	x8, x8, x1
1007a2b6c: f100051f    	cmp	x8, #0x1
1007a2b70: 54001f89    	b.ls	0x1007a2f60 <__ZN13codexmate_lib4core6models1_100_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..models..AccountSummary$GT$9serialize17ha921111109d021aaE+0x498>
1007a2b74: f94006c8    	ldr	x8, [x22, #0x8]
1007a2b78: 52840749    	mov	w9, #0x203a             ; =8250
1007a2b7c: 78216909    	strh	w9, [x8, x1]
1007a2b80: f9400ac8    	ldr	x8, [x22, #0x10]
1007a2b84: 91000908    	add	x8, x8, #0x2
1007a2b88: f9000ac8    	str	x8, [x22, #0x10]
1007a2b8c: aa1503e0    	mov	x0, x21
