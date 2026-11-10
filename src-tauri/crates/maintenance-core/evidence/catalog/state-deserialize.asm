
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100918dd8 <__ZN10serde_json5value2de83_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..value..Value$GT$18deserialize_struct17h6af4a717083fb407E>:
100918dd8: d100c3ff    	sub	sp, sp, #0x30
100918ddc: a9014ff4    	stp	x20, x19, [sp, #0x10]
100918de0: a9027bfd    	stp	x29, x30, [sp, #0x20]
100918de4: 910083fd    	add	x29, sp, #0x20
100918de8: aa0803f3    	mov	x19, x8
100918dec: 39400008    	ldrb	w8, [x0]
100918df0: 7100111f    	cmp	w8, #0x4
100918df4: 54000120    	b.eq	0x100918e18 <__ZN10serde_json5value2de83_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..value..Value$GT$18deserialize_struct17h6af4a717083fb407E+0x40>
100918df8: 7100151f    	cmp	w8, #0x5
100918dfc: 540001a1    	b.ne	0x100918e30 <__ZN10serde_json5value2de83_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..value..Value$GT$18deserialize_struct17h6af4a717083fb407E+0x58>
100918e00: 91002000    	add	x0, x0, #0x8
100918e04: aa1303e8    	mov	x8, x19
100918e08: a9427bfd    	ldp	x29, x30, [sp, #0x20]
100918e0c: a9414ff4    	ldp	x20, x19, [sp, #0x10]
100918e10: 9100c3ff    	add	sp, sp, #0x30
100918e14: 17eda8f9    	b	0x1004831f8 <__ZN10serde_json5value2de135_$LT$impl$u20$serde_core..de..Deserializer$u20$for$u20$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$15deserialize_any17h86b2cc9d10e0556eE>
100918e18: 91002000    	add	x0, x0, #0x8
100918e1c: aa1303e8    	mov	x8, x19
100918e20: a9427bfd    	ldp	x29, x30, [sp, #0x20]
100918e24: a9414ff4    	ldp	x20, x19, [sp, #0x10]
100918e28: 9100c3ff    	add	sp, sp, #0x30
100918e2c: 17e7ff72    	b	0x100318bf4 <__ZN10serde_json5value2de11visit_array17hbec28f07dee3cb6fE>
100918e30: f00078a2    	adrp	x2, 0x10182f000 <_anon.93008c7a652f50ea5d1586046426690a.9+0x10>
100918e34: 91150042    	add	x2, x2, #0x540
100918e38: 91003fe1    	add	x1, sp, #0xf
100918e3c: aa0003f4    	mov	x20, x0
100918e40: 94221d79    	bl	0x1011a0424 <__ZN10serde_json5value2de42_$LT$impl$u20$serde_json..value..Value$GT$12invalid_type17hb2f6e8292cb4d8b1E>
100918e44: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
100918e48: a9000268    	stp	x8, x0, [x19]
100918e4c: aa1403e0    	mov	x0, x20
100918e50: 94014c14    	bl	0x10096bea0 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100918e54: a9427bfd    	ldp	x29, x30, [sp, #0x20]
100918e58: a9414ff4    	ldp	x20, x19, [sp, #0x10]
100918e5c: 9100c3ff    	add	sp, sp, #0x30
100918e60: d65f03c0    	ret
100918e64: aa0003f3    	mov	x19, x0
