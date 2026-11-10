
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010079b0d0 <__ZN13codexmate_lib4core5relay6models1_105_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayWireApi$GT$9serialize17h3af7934dbb265090E>:
10079b0d0: a9be4ff4    	stp	x20, x19, [sp, #-0x20]!
10079b0d4: a9017bfd    	stp	x29, x30, [sp, #0x10]
10079b0d8: 910043fd    	add	x29, sp, #0x10
10079b0dc: aa0803f3    	mov	x19, x8
10079b0e0: 39400014    	ldrb	w20, [x0]
10079b0e4: 97e19924    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10079b0e8: 340001b4    	cbz	w20, 0x10079b11c <__ZN13codexmate_lib4core5relay6models1_105_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayWireApi$GT$9serialize17h3af7934dbb265090E+0x4c>
10079b0ec: 7100069f    	cmp	w20, #0x1
10079b0f0: 54000301    	b.ne	0x10079b150 <__ZN13codexmate_lib4core5relay6models1_105_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayWireApi$GT$9serialize17h3af7934dbb265090E+0x80>
10079b0f4: 52800200    	mov	w0, #0x10               ; =16
10079b0f8: 52800021    	mov	w1, #0x1                ; =1
10079b0fc: 97e1991a    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10079b100: b40004c0    	cbz	x0, 0x10079b198 <__ZN13codexmate_lib4core5relay6models1_105_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayWireApi$GT$9serialize17h3af7934dbb265090E+0xc8>
10079b104: b0006088    	adrp	x8, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079b108: 910d1108    	add	x8, x8, #0x344
10079b10c: 3dc00100    	ldr	q0, [x8]
10079b110: 3d800000    	str	q0, [x0]
10079b114: 52800208    	mov	w8, #0x10               ; =16
10079b118: 14000019    	b	0x10079b17c <__ZN13codexmate_lib4core5relay6models1_105_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayWireApi$GT$9serialize17h3af7934dbb265090E+0xac>
10079b11c: 52800160    	mov	w0, #0xb                ; =11
10079b120: 52800021    	mov	w1, #0x1                ; =1
10079b124: 97e19910    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10079b128: b40003e0    	cbz	x0, 0x10079b1a4 <__ZN13codexmate_lib4core5relay6models1_105_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayWireApi$GT$9serialize17h3af7934dbb265090E+0xd4>
10079b12c: 528d0c68    	mov	w8, #0x6863             ; =26723
10079b130: 72ae8c28    	movk	w8, #0x7461, lsl #16
10079b134: b8007008    	stur	w8, [x0, #0x7]
10079b138: b0006088    	adrp	x8, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079b13c: 910ce508    	add	x8, x8, #0x339
10079b140: f9400108    	ldr	x8, [x8]
10079b144: f9000008    	str	x8, [x0]
10079b148: 52800168    	mov	w8, #0xb                ; =11
10079b14c: 1400000c    	b	0x10079b17c <__ZN13codexmate_lib4core5relay6models1_105_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayWireApi$GT$9serialize17h3af7934dbb265090E+0xac>
10079b150: 52800120    	mov	w0, #0x9                ; =9
10079b154: 52800021    	mov	w1, #0x1                ; =1
10079b158: 97e19903    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10079b15c: b40002a0    	cbz	x0, 0x10079b1b0 <__ZN13codexmate_lib4core5relay6models1_105_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$codexmate_lib..core..relay..models..RelayWireApi$GT$9serialize17h3af7934dbb265090E+0xe0>
10079b160: 52800c68    	mov	w8, #0x63               ; =99
10079b164: 39002008    	strb	w8, [x0, #0x8]
10079b168: b0006088    	adrp	x8, 0x1013ac000 <_anon.6408540cdffbe318d49f52812e99783f.712+0x619>
10079b16c: 910d5108    	add	x8, x8, #0x354
10079b170: f9400108    	ldr	x8, [x8]
10079b174: f9000008    	str	x8, [x0]
10079b178: 52800128    	mov	w8, #0x9                ; =9
10079b17c: 52800069    	mov	w9, #0x3                ; =3
10079b180: 39000269    	strb	w9, [x19]
10079b184: a9008268    	stp	x8, x0, [x19, #0x8]
10079b188: f9000e68    	str	x8, [x19, #0x18]
10079b18c: a9417bfd    	ldp	x29, x30, [sp, #0x10]
10079b190: a8c24ff4    	ldp	x20, x19, [sp], #0x20
10079b194: d65f03c0    	ret
10079b198: 52800020    	mov	w0, #0x1                ; =1
10079b19c: 52800201    	mov	w1, #0x10               ; =16
10079b1a0: 9428d531    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
10079b1a4: 52800020    	mov	w0, #0x1                ; =1
10079b1a8: 52800161    	mov	w1, #0xb                ; =11
10079b1ac: 9428d52e    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
