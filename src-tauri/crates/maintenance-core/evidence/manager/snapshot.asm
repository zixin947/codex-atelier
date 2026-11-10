
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010068cdb4 <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE>:
10068cdb4: d10143ff    	sub	sp, sp, #0x50
10068cdb8: a9015ff8    	stp	x24, x23, [sp, #0x10]
10068cdbc: a90257f6    	stp	x22, x21, [sp, #0x20]
10068cdc0: a9034ff4    	stp	x20, x19, [sp, #0x30]
10068cdc4: a9047bfd    	stp	x29, x30, [sp, #0x40]
10068cdc8: 910103fd    	add	x29, sp, #0x40
10068cdcc: aa0803f4    	mov	x20, x8
10068cdd0: f9400416    	ldr	x22, [x0, #0x8]
10068cdd4: 910042d3    	add	x19, x22, #0x10
10068cdd8: d94102c0    	ldapur	x0, [x22, #0x10]
10068cddc: b4000300    	cbz	x0, 0x10068ce3c <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE+0x88>
10068cde0: 942af763    	bl	0x10114ab6c <__ZN3std3sys3pal4unix4sync5mutex5Mutex4lock17h8bb1e91d7d7c25e1E>
10068cde4: 90009357    	adrp	x23, 0x1018f4000 <__ZN15crossbeam_utils6atomic11atomic_cell4lock5LOCKS17h71d6ab0bc78afdcbE+0x1b00>
10068cde8: 912662f7    	add	x23, x23, #0x998
10068cdec: f94002e8    	ldr	x8, [x23]
10068cdf0: f240f91f    	tst	x8, #0x7fffffffffffffff
10068cdf4: 54000341    	b.ne	0x10068ce5c <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE+0xa8>
10068cdf8: 52800015    	mov	w21, #0x0               ; =0
10068cdfc: 394062c8    	ldrb	w8, [x22, #0x18]
10068ce00: 35000368    	cbnz	w8, 0x10068ce6c <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE+0xb8>
10068ce04: 910082c1    	add	x1, x22, #0x20
10068ce08: aa1403e0    	mov	x0, x20
10068ce0c: 9401e924    	bl	0x10070729c <__ZN85_$LT$codexmate_lib..core..relay..models..RelayState$u20$as$u20$core..clone..Clone$GT$5clone17hfd25db8fb8e42792E>
10068ce10: 37000095    	tbnz	w21, #0x0, 0x10068ce20 <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE+0x6c>
10068ce14: f94002e8    	ldr	x8, [x23]
10068ce18: f240f91f    	tst	x8, #0x7fffffffffffffff
10068ce1c: 54000401    	b.ne	0x10068ce9c <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE+0xe8>
10068ce20: f9400260    	ldr	x0, [x19]
10068ce24: a9447bfd    	ldp	x29, x30, [sp, #0x40]
10068ce28: a9434ff4    	ldp	x20, x19, [sp, #0x30]
10068ce2c: a94257f6    	ldp	x22, x21, [sp, #0x20]
10068ce30: a9415ff8    	ldp	x24, x23, [sp, #0x10]
10068ce34: 910143ff    	add	sp, sp, #0x50
10068ce38: 142af754    	b	0x10114ab88 <__ZN3std3sys3pal4unix4sync5mutex5Mutex6unlock17hb12cc63852d40a98E>
10068ce3c: aa1303e0    	mov	x0, x19
10068ce40: 942c4db6    	bl	0x1011a0518 <__ZN3std3sys4sync8once_box16OnceBox$LT$T$GT$10initialize17h95d0a095904ca1eaE>
10068ce44: 942af74a    	bl	0x10114ab6c <__ZN3std3sys3pal4unix4sync5mutex5Mutex4lock17h8bb1e91d7d7c25e1E>
10068ce48: 90009357    	adrp	x23, 0x1018f4000 <__ZN15crossbeam_utils6atomic11atomic_cell4lock5LOCKS17h71d6ab0bc78afdcbE+0x1b00>
10068ce4c: 912662f7    	add	x23, x23, #0x998
10068ce50: f94002e8    	ldr	x8, [x23]
10068ce54: f240f91f    	tst	x8, #0x7fffffffffffffff
10068ce58: 54fffd00    	b.eq	0x10068cdf8 <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE+0x44>
10068ce5c: 942d001c    	bl	0x1011ccecc <__ZN3std9panicking11panic_count17is_zero_slow_path17hbe4f1645e48c7385E>
10068ce60: 52000015    	eor	w21, w0, #0x1
10068ce64: 394062c8    	ldrb	w8, [x22, #0x18]
10068ce68: 34fffce8    	cbz	w8, 0x10068ce04 <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE+0x50>
10068ce6c: f90003f3    	str	x19, [sp]
10068ce70: 390023f5    	strb	w21, [sp, #0x8]
10068ce74: b00068a0    	adrp	x0, 0x1013a1000 <_anon.62728874ff9674d7d8f5de25cb1e85f1.1777+0x221>
10068ce78: 913fd800    	add	x0, x0, #0xff6
10068ce7c: f0008cc3    	adrp	x3, 0x101827000 <_anon.1f4a05fd1dc96b9982818105670d4dce.474+0x460>
10068ce80: 91300063    	add	x3, x3, #0xc00
10068ce84: f0008cc4    	adrp	x4, 0x101827000 <_anon.1f4a05fd1dc96b9982818105670d4dce.474+0x460>
10068ce88: 91048084    	add	x4, x4, #0x120
10068ce8c: 910003e2    	mov	x2, sp
10068ce90: 52800281    	mov	w1, #0x14               ; =20
10068ce94: 942d0ed9    	bl	0x1011d09f8 <__ZN4core6result13unwrap_failed17hfd1628134417d023E>
10068ce98: d4200020    	brk	#0x1
10068ce9c: 942d000c    	bl	0x1011ccecc <__ZN3std9panicking11panic_count17is_zero_slow_path17hbe4f1645e48c7385E>
10068cea0: 3707fc00    	tbnz	w0, #0x0, 0x10068ce20 <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE+0x6c>
10068cea4: 52800028    	mov	w8, #0x1                ; =1
10068cea8: 390062c8    	strb	w8, [x22, #0x18]
