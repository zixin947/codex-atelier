
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100805608 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE>:
100805608: d10683ff    	sub	sp, sp, #0x1a0
10080560c: a9146ffc    	stp	x28, x27, [sp, #0x140]
100805610: a91567fa    	stp	x26, x25, [sp, #0x150]
100805614: a9165ff8    	stp	x24, x23, [sp, #0x160]
100805618: a91757f6    	stp	x22, x21, [sp, #0x170]
10080561c: a9184ff4    	stp	x20, x19, [sp, #0x180]
100805620: a9197bfd    	stp	x29, x30, [sp, #0x190]
100805624: 910643fd    	add	x29, sp, #0x190
100805628: a9406c48    	ldp	x8, x27, [x2]
10080562c: f90003e8    	str	x8, [sp]
100805630: eb01001f    	cmp	x0, x1
100805634: 54001e80    	b.eq	0x100805a04 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x3fc>
100805638: cb000028    	sub	x8, x1, x0
10080563c: f9400849    	ldr	x9, [x2, #0x10]
100805640: d344fd1a    	lsr	x26, x8, #4
100805644: 8b1b1528    	add	x8, x9, x27, lsl #5
100805648: 91000513    	add	x19, x8, #0x1
10080564c: 9100201c    	add	x28, x0, #0x8
100805650: a97fa788    	ldp	x8, x9, [x28, #-0x8]
100805654: f100113f    	cmp	x9, #0x4
100805658: 540001cc    	b.gt	0x100805690 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x88>
10080565c: f1000d3f    	cmp	x9, #0x3
100805660: 54000380    	b.eq	0x1008056d0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0xc8>
100805664: f100113f    	cmp	x9, #0x4
100805668: 54000681    	b.ne	0x100805738 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x130>
10080566c: b9400108    	ldr	w8, [x8]
100805670: 528d2d09    	mov	w9, #0x6968             ; =26984
100805674: 72ad0ce9    	movk	w9, #0x6867, lsl #16
100805678: 6b09011f    	cmp	w8, w9
10080567c: 540005e1    	b.ne	0x100805738 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x130>
100805680: f0005d55    	adrp	x21, 0x1013b0000 <_anon.973b090bee24c06fdb949486dbea4242.108+0x3d8>
100805684: 912deab5    	add	x21, x21, #0xb7a
100805688: 52800594    	mov	w20, #0x2c              ; =44
10080568c: 1400002e    	b	0x100805744 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x13c>
100805690: f100153f    	cmp	x9, #0x5
100805694: 54000420    	b.eq	0x100805718 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x110>
100805698: f100193f    	cmp	x9, #0x6
10080569c: 540004e1    	b.ne	0x100805738 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x130>
1008056a0: b9400109    	ldr	w9, [x8]
1008056a4: 79400908    	ldrh	w8, [x8, #0x4]
1008056a8: 528cadaa    	mov	w10, #0x656d            ; =25965
1008056ac: 72ad2c8a    	movk	w10, #0x6964, lsl #16
1008056b0: 6b0a013f    	cmp	w9, w10
1008056b4: 528daea9    	mov	w9, #0x6d75             ; =28021
1008056b8: 7a490100    	ccmp	w8, w9, #0x0, eq
1008056bc: 540003e1    	b.ne	0x100805738 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x130>
1008056c0: f0005d55    	adrp	x21, 0x1013b0000 <_anon.973b090bee24c06fdb949486dbea4242.108+0x3d8>
1008056c4: 912e9ab5    	add	x21, x21, #0xba6
1008056c8: 528006b4    	mov	w20, #0x35              ; =53
1008056cc: 1400001e    	b	0x100805744 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x13c>
1008056d0: 79400109    	ldrh	w9, [x8]
1008056d4: 3940090a    	ldrb	w10, [x8, #0x2]
1008056d8: 528ded8b    	mov	w11, #0x6f6c            ; =28524
1008056dc: 6b0b013f    	cmp	w9, w11
1008056e0: 52800ee9    	mov	w9, #0x77               ; =119
1008056e4: 7a490140    	ccmp	w10, w9, #0x0, eq
1008056e8: 54001760    	b.eq	0x1008059d4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x3cc>
1008056ec: 79400109    	ldrh	w9, [x8]
1008056f0: 39400908    	ldrb	w8, [x8, #0x2]
1008056f4: 528c2daa    	mov	w10, #0x616d            ; =24941
1008056f8: 6b0a013f    	cmp	w9, w10
1008056fc: 52800f09    	mov	w9, #0x78               ; =120
100805700: 7a490100    	ccmp	w8, w9, #0x0, eq
100805704: 540001a1    	b.ne	0x100805738 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x130>
100805708: f0005d55    	adrp	x21, 0x1013b0000 <_anon.973b090bee24c06fdb949486dbea4242.108+0x3d8>
10080570c: 912c46b5    	add	x21, x21, #0xb11
100805710: 528005b4    	mov	w20, #0x2d              ; =45
100805714: 1400000c    	b	0x100805744 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x13c>
100805718: b9400109    	ldr	w9, [x8]
10080571c: 39401108    	ldrb	w8, [x8, #0x4]
100805720: 528d0f0a    	mov	w10, #0x6878            ; =26744
100805724: 72aced2a    	movk	w10, #0x6769, lsl #16
100805728: 6b0a013f    	cmp	w9, w10
10080572c: 52800d09    	mov	w9, #0x68               ; =104
100805730: 7a490100    	ccmp	w8, w9, #0x0, eq
100805734: 54001580    	b.eq	0x1008059e4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x3dc>
100805738: f0005d55    	adrp	x21, 0x1013b0000 <_anon.973b090bee24c06fdb949486dbea4242.108+0x3d8>
10080573c: 912bdeb5    	add	x21, x21, #0xaf7
100805740: 52800354    	mov	w20, #0x1a              ; =26
100805744: f90017ff    	str	xzr, [sp, #0x28]
100805748: f9001fff    	str	xzr, [sp, #0x38]
10080574c: 97dfef8a    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100805750: 528000c0    	mov	w0, #0x6                ; =6
100805754: 52800021    	mov	w1, #0x1                ; =1
100805758: 97dfef83    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
10080575c: 528000c1    	mov	w1, #0x6                ; =6
100805760: b4001660    	cbz	x0, 0x100805a2c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x424>
100805764: aa0003f6    	mov	x22, x0
100805768: 528e8e48    	mov	w8, #0x7472             ; =29810
10080576c: 79000808    	strh	w8, [x0, #0x4]
100805770: 528ccca8    	mov	w8, #0x6665             ; =26213
100805774: 72adecc8    	movk	w8, #0x6f66, lsl #16
100805778: b9000008    	str	w8, [x0]
10080577c: 528000c8    	mov	w8, #0x6                ; =6
100805780: a90403e8    	stp	x8, x0, [sp, #0x40]
100805784: f9002be8    	str	x8, [sp, #0x50]
100805788: f9400397    	ldr	x23, [x28]
10080578c: b7f81357    	tbnz	x23, #0x3f, 0x1008059f4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x3ec>
100805790: f85f8398    	ldur	x24, [x28, #-0x8]
100805794: b4000137    	cbz	x23, 0x1008057b8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x1b0>
100805798: 97dfef77    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
10080579c: aa1703e0    	mov	x0, x23
1008057a0: 52800021    	mov	w1, #0x1                ; =1
1008057a4: 97dfef70    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008057a8: aa0003f9    	mov	x25, x0
1008057ac: 52800020    	mov	w0, #0x1                ; =1
1008057b0: b5000079    	cbnz	x25, 0x1008057bc <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x1b4>
1008057b4: 14000091    	b	0x1008059f8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x3f0>
1008057b8: 52800039    	mov	w25, #0x1               ; =1
1008057bc: aa1903e0    	mov	x0, x25
1008057c0: aa1803e1    	mov	x1, x24
1008057c4: aa1703e2    	mov	x2, x23
1008057c8: 94272fb9    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008057cc: 52800068    	mov	w8, #0x3                ; =3
1008057d0: 390163e8    	strb	w8, [sp, #0x58]
1008057d4: a90667f7    	stp	x23, x25, [sp, #0x60]
1008057d8: f9003bf7    	str	x23, [sp, #0x70]
1008057dc: 9101e3e8    	add	x8, sp, #0x78
1008057e0: 9100a3e0    	add	x0, sp, #0x28
1008057e4: 910103e1    	add	x1, sp, #0x40
1008057e8: 97fd4414    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
1008057ec: f9403fe8    	ldr	x8, [sp, #0x78]
1008057f0: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
1008057f4: eb09011f    	cmp	x8, x9
1008057f8: 9101e3f8    	add	x24, sp, #0x78
1008057fc: 910027f9    	add	x25, sp, #0x9
100805800: 54000201    	b.ne	0x100805840 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x238>
100805804: f94043e8    	ldr	x8, [sp, #0x80]
100805808: f9400f09    	ldr	x9, [x24, #0x18]
10080580c: 8b091508    	add	x8, x8, x9, lsl #5
100805810: ad400500    	ldp	q0, q1, [x8]
100805814: 3c898300    	stur	q0, [x24, #0x98]
100805818: 3c8a8301    	stur	q1, [x24, #0xa8]
10080581c: 3cc683e0    	ldur	q0, [sp, #0x68]
100805820: 3cc583e1    	ldur	q1, [sp, #0x58]
100805824: ad000101    	stp	q1, q0, [x8]
100805828: 385803a8    	ldurb	w8, [x29, #-0x80]
10080582c: 7100191f    	cmp	w8, #0x6
100805830: 54000280    	b.eq	0x100805880 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x278>
100805834: d10203a0    	sub	x0, x29, #0x80
100805838: 9401788d    	bl	0x100863a6c <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
10080583c: 14000011    	b	0x100805880 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x278>
100805840: 3cc783e0    	ldur	q0, [sp, #0x78]
100805844: 3cc883e1    	ldur	q1, [sp, #0x88]
100805848: 3c838300    	stur	q0, [x24, #0x38]
10080584c: 3c848301    	stur	q1, [x24, #0x48]
100805850: 3dc00b00    	ldr	q0, [x24, #0x20]
100805854: 3c858300    	stur	q0, [x24, #0x58]
100805858: f9401b08    	ldr	x8, [x24, #0x30]
10080585c: f9003708    	str	x8, [x24, #0x68]
100805860: 3cc583e0    	ldur	q0, [sp, #0x58]
100805864: 3cc683e1    	ldur	q1, [sp, #0x68]
100805868: 3c878300    	stur	q0, [x24, #0x78]
10080586c: 3c888301    	stur	q1, [x24, #0x88]
100805870: d10203a8    	sub	x8, x29, #0x80
100805874: 9102c3e0    	add	x0, sp, #0xb0
100805878: d10283a1    	sub	x1, x29, #0xa0
10080587c: 97f821a1    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
100805880: 97dfef3d    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100805884: 52800160    	mov	w0, #0xb                ; =11
100805888: 52800021    	mov	w1, #0x1                ; =1
10080588c: 97dfef36    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100805890: 52800161    	mov	w1, #0xb                ; =11
100805894: b4000cc0    	cbz	x0, 0x100805a2c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x424>
100805898: aa0003f6    	mov	x22, x0
10080589c: 528d2e88    	mov	w8, #0x6974             ; =26996
1008058a0: 72adcde8    	movk	w8, #0x6e6f, lsl #16
1008058a4: b8007008    	stur	w8, [x0, #0x7]
1008058a8: f0005d48    	adrp	x8, 0x1013b0000 <_anon.973b090bee24c06fdb949486dbea4242.108+0x3d8>
1008058ac: 91301908    	add	x8, x8, #0xc06
1008058b0: f9400108    	ldr	x8, [x8]
1008058b4: f9000008    	str	x8, [x0]
1008058b8: 52800168    	mov	w8, #0xb                ; =11
1008058bc: a90403e8    	stp	x8, x0, [sp, #0x40]
1008058c0: f9002be8    	str	x8, [sp, #0x50]
1008058c4: 97dfef2c    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
1008058c8: aa1403e0    	mov	x0, x20
1008058cc: 52800021    	mov	w1, #0x1                ; =1
1008058d0: 97dfef25    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
1008058d4: b4000b20    	cbz	x0, 0x100805a38 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x430>
1008058d8: aa0003f7    	mov	x23, x0
1008058dc: aa1503e1    	mov	x1, x21
1008058e0: aa1403e2    	mov	x2, x20
1008058e4: 94272f72    	bl	0x1011d16ac <_writev+0x1011d16ac>
1008058e8: 52800068    	mov	w8, #0x3                ; =3
1008058ec: 390163e8    	strb	w8, [sp, #0x58]
1008058f0: a9065ff4    	stp	x20, x23, [sp, #0x60]
1008058f4: f9003bf4    	str	x20, [sp, #0x70]
1008058f8: 9101e3e8    	add	x8, sp, #0x78
1008058fc: 9100a3e0    	add	x0, sp, #0x28
100805900: 910103e1    	add	x1, sp, #0x40
100805904: 97fd43cd    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
100805908: f9403fe8    	ldr	x8, [sp, #0x78]
10080590c: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
100805910: eb09011f    	cmp	x8, x9
100805914: 54000201    	b.ne	0x100805954 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x34c>
100805918: f94043e8    	ldr	x8, [sp, #0x80]
10080591c: f9400f09    	ldr	x9, [x24, #0x18]
100805920: 8b091508    	add	x8, x8, x9, lsl #5
100805924: ad400500    	ldp	q0, q1, [x8]
100805928: 3c898300    	stur	q0, [x24, #0x98]
10080592c: 3c8a8301    	stur	q1, [x24, #0xa8]
100805930: 3cc683e0    	ldur	q0, [sp, #0x68]
100805934: 3cc583e1    	ldur	q1, [sp, #0x58]
100805938: ad000101    	stp	q1, q0, [x8]
10080593c: 385803a8    	ldurb	w8, [x29, #-0x80]
100805940: 7100191f    	cmp	w8, #0x6
100805944: 54000280    	b.eq	0x100805994 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x38c>
100805948: d10203a0    	sub	x0, x29, #0x80
10080594c: 94017848    	bl	0x100863a6c <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100805950: 14000011    	b	0x100805994 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x38c>
100805954: 3cc783e0    	ldur	q0, [sp, #0x78]
100805958: 3cc883e1    	ldur	q1, [sp, #0x88]
10080595c: 3c838300    	stur	q0, [x24, #0x38]
100805960: 3c848301    	stur	q1, [x24, #0x48]
100805964: 3dc00b00    	ldr	q0, [x24, #0x20]
100805968: 3c858300    	stur	q0, [x24, #0x58]
10080596c: f9401b08    	ldr	x8, [x24, #0x30]
100805970: f9003708    	str	x8, [x24, #0x68]
100805974: 3cc583e0    	ldur	q0, [sp, #0x58]
100805978: 3cc683e1    	ldur	q1, [sp, #0x68]
10080597c: 3c878300    	stur	q0, [x24, #0x78]
100805980: 3c888301    	stur	q1, [x24, #0x88]
100805984: d10203a8    	sub	x8, x29, #0x80
100805988: 9102c3e0    	add	x0, sp, #0xb0
10080598c: d10283a1    	sub	x1, x29, #0xa0
100805990: 97f8215c    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
100805994: 3cc283e0    	ldur	q0, [sp, #0x28]
100805998: 3c807320    	stur	q0, [x25, #0x7]
10080599c: f9401fe8    	ldr	x8, [sp, #0x38]
1008059a0: f8017328    	stur	x8, [x25, #0x17]
1008059a4: 528000a8    	mov	w8, #0x5                ; =5
1008059a8: 381ff268    	sturb	w8, [x19, #-0x1]
1008059ac: 3cc093e0    	ldur	q0, [sp, #0x9]
1008059b0: 3d800260    	str	q0, [x19]
1008059b4: 3cc183e0    	ldur	q0, [sp, #0x18]
1008059b8: 3c80f260    	stur	q0, [x19, #0xf]
1008059bc: 9100077b    	add	x27, x27, #0x1
1008059c0: 91008273    	add	x19, x19, #0x20
1008059c4: 9100439c    	add	x28, x28, #0x10
1008059c8: f100075a    	subs	x26, x26, #0x1
1008059cc: 54ffe421    	b.ne	0x100805650 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x48>
1008059d0: 1400000d    	b	0x100805a04 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x3fc>
1008059d4: f0005d55    	adrp	x21, 0x1013b0000 <_anon.973b090bee24c06fdb949486dbea4242.108+0x3d8>
1008059d8: 912f6eb5    	add	x21, x21, #0xbdb
1008059dc: 528004b4    	mov	w20, #0x25              ; =37
1008059e0: 17ffff59    	b	0x100805744 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x13c>
1008059e4: f0005d55    	adrp	x21, 0x1013b0000 <_anon.973b090bee24c06fdb949486dbea4242.108+0x3d8>
1008059e8: 912cfab5    	add	x21, x21, #0xb3e
1008059ec: 52800794    	mov	w20, #0x3c              ; =60
1008059f0: 17ffff55    	b	0x100805744 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x13c>
1008059f4: d2800000    	mov	x0, #0x0                ; =0
1008059f8: aa1703e1    	mov	x1, x23
1008059fc: 94272b1a    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100805a00: 14000011    	b	0x100805a44 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x43c>
100805a04: f94003e8    	ldr	x8, [sp]
100805a08: f900011b    	str	x27, [x8]
100805a0c: a9597bfd    	ldp	x29, x30, [sp, #0x190]
100805a10: a9584ff4    	ldp	x20, x19, [sp, #0x180]
100805a14: a95757f6    	ldp	x22, x21, [sp, #0x170]
100805a18: a9565ff8    	ldp	x24, x23, [sp, #0x160]
100805a1c: a95567fa    	ldp	x26, x25, [sp, #0x150]
100805a20: a9546ffc    	ldp	x28, x27, [sp, #0x140]
100805a24: 910683ff    	add	sp, sp, #0x1a0
100805a28: d65f03c0    	ret
100805a2c: 52800020    	mov	w0, #0x1                ; =1
100805a30: 94272b0d    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100805a34: 14000004    	b	0x100805a44 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x43c>
100805a38: 52800020    	mov	w0, #0x1                ; =1
100805a3c: aa1403e1    	mov	x1, x20
100805a40: 94272b09    	bl	0x1011d0664 <__ZN5alloc7raw_vec12handle_error17hd9deaf7dc19ebb3fE>
100805a44: d4200020    	brk	#0x1
100805a48: 14000011    	b	0x100805a8c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x484>
100805a4c: 14000010    	b	0x100805a8c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x484>
100805a50: 1400000f    	b	0x100805a8c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x484>
100805a54: aa0003f3    	mov	x19, x0
100805a58: aa1603e0    	mov	x0, x22
100805a5c: 52800161    	mov	w1, #0xb                ; =11
100805a60: 14000010    	b	0x100805aa0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x498>
100805a64: aa0003f3    	mov	x19, x0
100805a68: 910163e0    	add	x0, sp, #0x58
100805a6c: 97fcfe3f    	bl	0x100745368 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100805a70: 1400000e    	b	0x100805aa8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x4a0>
100805a74: 94272c35    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
100805a78: aa0003f3    	mov	x19, x0
100805a7c: 910163e0    	add	x0, sp, #0x58
100805a80: 97fcfe3a    	bl	0x100745368 <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100805a84: 14000009    	b	0x100805aa8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x4a0>
100805a88: 94272c30    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>
100805a8c: aa0003f3    	mov	x19, x0
100805a90: 14000006    	b	0x100805aa8 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9037e112e844613cE+0x4a0>
100805a94: aa0003f3    	mov	x19, x0
100805a98: aa1603e0    	mov	x0, x22
100805a9c: 528000c1    	mov	w1, #0x6                ; =6
100805aa0: 52800022    	mov	w2, #0x1                ; =1
100805aa4: 97dfeeb1    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100805aa8: 9100a3e0    	add	x0, sp, #0x28
100805aac: 97fda9ee    	bl	0x100770264 <__ZN99_$LT$alloc..collections..btree..map..BTreeMap$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h25de00eca2f35937E>
100805ab0: f94003e8    	ldr	x8, [sp]
100805ab4: f900011b    	str	x27, [x8]
100805ab8: aa1303e0    	mov	x0, x19
100805abc: 94272da6    	bl	0x1011d1154 <_writev+0x1011d1154>
100805ac0: 94272c22    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

0000000100805ac4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h972e43de0ad34e80E>:
100805ac4: d10443ff    	sub	sp, sp, #0x110
100805ac8: a90c67fa    	stp	x26, x25, [sp, #0xc0]
100805acc: a90d5ff8    	stp	x24, x23, [sp, #0xd0]
100805ad0: a90e57f6    	stp	x22, x21, [sp, #0xe0]
100805ad4: a90f4ff4    	stp	x20, x19, [sp, #0xf0]
100805ad8: a9107bfd    	stp	x29, x30, [sp, #0x100]
100805adc: 910403fd    	add	x29, sp, #0x100
100805ae0: aa0203f3    	mov	x19, x2
100805ae4: d28002d5    	mov	x21, #0x16              ; =22
100805ae8: f2f00015    	movk	x21, #0x8000, lsl #48
100805aec: 910003f6    	mov	x22, sp
100805af0: a90007e0    	stp	x0, x1, [sp]
100805af4: 910103f7    	add	x23, sp, #0x40
100805af8: 910203f8    	add	x24, sp, #0x80
100805afc: d2f00019    	mov	x25, #-0x8000000000000000 ; =-9223372036854775808
100805b00: f0008134    	adrp	x20, 0x10182c000 <_anon.6408540cdffbe318d49f52812e99783f.1678+0xb8>
100805b04: 91386294    	add	x20, x20, #0xe18
100805b08: 14000003    	b	0x100805b14 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h972e43de0ad34e80E+0x50>
100805b0c: 910203e0    	add	x0, sp, #0x80
100805b10: 940176a6    	bl	0x1008635a8 <__ZN4core3ptr43drop_in_place$LT$rusqlite..error..Error$GT$17h09391278bf407148E>
100805b14: 910103e0    	add	x0, sp, #0x40
100805b18: 910003e1    	mov	x1, sp
100805b1c: 9401bf25    	bl	0x1008757b0 <__ZN94_$LT$rusqlite..row..Rows$u20$as$u20$fallible_streaming_iterator..FallibleStreamingIterator$GT$7advance17hdebe552720793c1eE>
100805b20: f94023e9    	ldr	x9, [sp, #0x40]
100805b24: eb15013f    	cmp	x9, x21
100805b28: 54000221    	b.ne	0x100805b6c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h972e43de0ad34e80E+0xa8>
100805b2c: f94007e8    	ldr	x8, [sp, #0x8]
100805b30: b4000508    	cbz	x8, 0x100805bd0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h972e43de0ad34e80E+0x10c>
100805b34: 910203e8    	add	x8, sp, #0x80
100805b38: 910022c0    	add	x0, x22, #0x8
100805b3c: 52800021    	mov	w1, #0x1                ; =1
100805b40: aa1403e2    	mov	x2, x20
100805b44: 97ffea34    	bl	0x100800414 <__ZN8rusqlite3row3Row3get17hd7be767ace66ceaeE>
100805b48: a94823e9    	ldp	x9, x8, [sp, #0x80]
100805b4c: ad408702    	ldp	q2, q1, [x24, #0x10]
100805b50: 3cc30300    	ldur	q0, [x24, #0x30]
100805b54: 3d8007e1    	str	q1, [sp, #0x10]
100805b58: ad010be0    	stp	q0, q2, [sp, #0x20]
100805b5c: 910006aa    	add	x10, x21, #0x1
100805b60: eb0a013f    	cmp	x9, x10
100805b64: 54000161    	b.ne	0x100805b90 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h972e43de0ad34e80E+0xcc>
100805b68: 1400001a    	b	0x100805bd0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h972e43de0ad34e80E+0x10c>
100805b6c: f94027e8    	ldr	x8, [sp, #0x48]
100805b70: ad4086e0    	ldp	q0, q1, [x23, #0x10]
100805b74: ad008700    	stp	q0, q1, [x24, #0x10]
100805b78: 3cc302e0    	ldur	q0, [x23, #0x30]
100805b7c: 3c830300    	stur	q0, [x24, #0x30]
100805b80: ad408702    	ldp	q2, q1, [x24, #0x10]
100805b84: 3cc30300    	ldur	q0, [x24, #0x30]
100805b88: 3d8007e1    	str	q1, [sp, #0x10]
100805b8c: ad010be0    	stp	q0, q2, [sp, #0x20]
100805b90: ad4087e0    	ldp	q0, q1, [sp, #0x10]
100805b94: ad010700    	stp	q0, q1, [x24, #0x20]
100805b98: a90823e9    	stp	x9, x8, [sp, #0x80]
100805b9c: 3dc00fe1    	ldr	q1, [sp, #0x30]
100805ba0: 3c810301    	stur	q1, [x24, #0x10]
100805ba4: eb15013f    	cmp	x9, x21
100805ba8: 54fffb21    	b.ne	0x100805b0c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h972e43de0ad34e80E+0x48>
100805bac: eb19011f    	cmp	x8, x25
100805bb0: 54fffb20    	b.eq	0x100805b14 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h972e43de0ad34e80E+0x50>
100805bb4: 3dc00fe0    	ldr	q0, [sp, #0x30]
100805bb8: 3c808300    	stur	q0, [x24, #0x8]
100805bbc: f90043e8    	str	x8, [sp, #0x80]
100805bc0: 910203e1    	add	x1, sp, #0x80
100805bc4: aa1303e0    	mov	x0, x19
100805bc8: 97f5c17f    	bl	0x1005761c4 <__ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17hdb67a7b69d944eb9E>
100805bcc: 17ffffd2    	b	0x100805b14 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h972e43de0ad34e80E+0x50>
100805bd0: 910003e0    	mov	x0, sp
100805bd4: 94017318    	bl	0x100862834 <__ZN4core3ptr40drop_in_place$LT$rusqlite..row..Rows$GT$17h29214cf941cfc830E>
100805bd8: a9507bfd    	ldp	x29, x30, [sp, #0x100]
100805bdc: a94f4ff4    	ldp	x20, x19, [sp, #0xf0]
100805be0: a94e57f6    	ldp	x22, x21, [sp, #0xe0]
100805be4: a94d5ff8    	ldp	x24, x23, [sp, #0xd0]
100805be8: a94c67fa    	ldp	x26, x25, [sp, #0xc0]
100805bec: 910443ff    	add	sp, sp, #0x110
100805bf0: d65f03c0    	ret
100805bf4: aa0003f3    	mov	x19, x0
100805bf8: 910003e0    	mov	x0, sp
100805bfc: 9401730e    	bl	0x100862834 <__ZN4core3ptr40drop_in_place$LT$rusqlite..row..Rows$GT$17h29214cf941cfc830E>
100805c00: aa1303e0    	mov	x0, x19
100805c04: 94272d54    	bl	0x1011d1154 <_writev+0x1011d1154>
100805c08: 94272bd0    	bl	0x1011d0b48 <__ZN4core9panicking16panic_in_cleanup17h2d3328184a42aad6E>

0000000100805c0c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17ha622b1dbf730d5ddE>:
100805c0c: d10203ff    	sub	sp, sp, #0x80
100805c10: a90367fa    	stp	x26, x25, [sp, #0x30]
100805c14: a9045ff8    	stp	x24, x23, [sp, #0x40]
100805c18: a90557f6    	stp	x22, x21, [sp, #0x50]
100805c1c: a9064ff4    	stp	x20, x19, [sp, #0x60]
100805c20: a9077bfd    	stp	x29, x30, [sp, #0x70]
100805c24: 9101c3fd    	add	x29, sp, #0x70
100805c28: a9402408    	ldp	x8, x9, [x0]
100805c2c: a9405835    	ldp	x21, x22, [x1]
100805c30: eb09011f    	cmp	x8, x9
100805c34: 54000480    	b.eq	0x100805cc4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17ha622b1dbf730d5ddE+0xb8>
100805c38: f940082a    	ldr	x10, [x1, #0x10]
100805c3c: f9400814    	ldr	x20, [x0, #0x10]
100805c40: cb080129    	sub	x9, x9, x8
100805c44: b201f3eb    	mov	x11, #-0x5555555555555556 ; =-6148914691236517206
100805c48: f295556b    	movk	x11, #0xaaab
100805c4c: 9bcb7d29    	umulh	x9, x9, x11
100805c50: d344fd37    	lsr	x23, x9, #4
100805c54: 52800309    	mov	w9, #0x18               ; =24
100805c58: 9b092ad8    	madd	x24, x22, x9, x10
100805c5c: 91004119    	add	x25, x8, #0x10
100805c60: 1400000a    	b	0x100805c88 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17ha622b1dbf730d5ddE+0x7c>
100805c64: 91006339    	add	x25, x25, #0x18
100805c68: 3dc003e0    	ldr	q0, [sp]
100805c6c: 3d800300    	str	q0, [x24]
100805c70: f9400be8    	ldr	x8, [sp, #0x10]
100805c74: f9000b08    	str	x8, [x24, #0x10]
100805c78: 910006d6    	add	x22, x22, #0x1
100805c7c: 91006318    	add	x24, x24, #0x18
100805c80: f10006f7    	subs	x23, x23, #0x1
100805c84: 54000200    	b.eq	0x100805cc4 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17ha622b1dbf730d5ddE+0xb8>
100805c88: a97f8720    	ldp	x0, x1, [x25, #-0x8]
100805c8c: 910063e8    	add	x8, sp, #0x18
100805c90: 9425cd54    	bl	0x1011791e0 <__ZN5alloc6string6String15from_utf8_lossy17h03e7be16830322ceE>
100805c94: a9420bf3    	ldp	x19, x2, [sp, #0x20]
100805c98: a9409283    	ldp	x3, x4, [x20, #0x8]
100805c9c: 910003e0    	mov	x0, sp
100805ca0: aa1303e1    	mov	x1, x19
100805ca4: 97fa32c2    	bl	0x1006927ac <__ZN13codexmate_lib8platform9debug_log17redact_text_inner17hbd7295fd7808b1b2E>
100805ca8: f9400fe1    	ldr	x1, [sp, #0x18]
100805cac: f240f83f    	tst	x1, #0x7fffffffffffffff
100805cb0: 54fffda0    	b.eq	0x100805c64 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17ha622b1dbf730d5ddE+0x58>
100805cb4: aa1303e0    	mov	x0, x19
100805cb8: 52800022    	mov	w2, #0x1                ; =1
100805cbc: 97dfee2b    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100805cc0: 17ffffe9    	b	0x100805c64 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17ha622b1dbf730d5ddE+0x58>
100805cc4: f90002b6    	str	x22, [x21]
100805cc8: a9477bfd    	ldp	x29, x30, [sp, #0x70]
100805ccc: a9464ff4    	ldp	x20, x19, [sp, #0x60]
100805cd0: a94557f6    	ldp	x22, x21, [sp, #0x50]
100805cd4: a9445ff8    	ldp	x24, x23, [sp, #0x40]
100805cd8: a94367fa    	ldp	x26, x25, [sp, #0x30]
100805cdc: 910203ff    	add	sp, sp, #0x80
100805ce0: d65f03c0    	ret
100805ce4: f90002b6    	str	x22, [x21]
100805ce8: 94272d1b    	bl	0x1011d1154 <_writev+0x1011d1154>
100805cec: aa0003f4    	mov	x20, x0
100805cf0: f9400fe1    	ldr	x1, [sp, #0x18]
100805cf4: f240f83f    	tst	x1, #0x7fffffffffffffff
100805cf8: 54000080    	b.eq	0x100805d08 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17ha622b1dbf730d5ddE+0xfc>
100805cfc: aa1303e0    	mov	x0, x19
100805d00: 52800022    	mov	w2, #0x1                ; =1
100805d04: 97dfee19    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
100805d08: f90002b6    	str	x22, [x21]
100805d0c: aa1403e0    	mov	x0, x20
100805d10: 94272d11    	bl	0x1011d1154 <_writev+0x1011d1154>

0000000100805d14 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E>:
100805d14: a9ba6ffc    	stp	x28, x27, [sp, #-0x60]!
100805d18: a90167fa    	stp	x26, x25, [sp, #0x10]
100805d1c: a9025ff8    	stp	x24, x23, [sp, #0x20]
100805d20: a90357f6    	stp	x22, x21, [sp, #0x30]
100805d24: a9044ff4    	stp	x20, x19, [sp, #0x40]
100805d28: a9057bfd    	stp	x29, x30, [sp, #0x50]
100805d2c: 910143fd    	add	x29, sp, #0x50
100805d30: d10a43ff    	sub	sp, sp, #0x290
100805d34: a940201b    	ldp	x27, x8, [x0]
100805d38: a940283a    	ldp	x26, x10, [x1]
100805d3c: eb08037f    	cmp	x27, x8
100805d40: 54009760    	b.eq	0x10080702c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x1318>
100805d44: d2800013    	mov	x19, #0x0               ; =0
100805d48: 910403f7    	add	x23, sp, #0x100
100805d4c: f9400829    	ldr	x9, [x1, #0x10]
100805d50: f9003be9    	str	x9, [sp, #0x70]
100805d54: cb1b0108    	sub	x8, x8, x27
100805d58: d343fd08    	lsr	x8, x8, #3
100805d5c: f940080b    	ldr	x11, [x0, #0x10]
100805d60: d28b1a29    	mov	x9, #0x58d1             ; =22737
100805d64: f2a343e9    	movk	x9, #0x1a1f, lsl #16
100805d68: f2c87d69    	movk	x9, #0x43eb, lsl #32
100805d6c: f2efac69    	movk	x9, #0x7d63, lsl #48
100805d70: 9b097d08    	mul	x8, x8, x9
100805d74: a9062fe8    	stp	x8, x11, [sp, #0x60]
100805d78: 52803108    	mov	w8, #0x188              ; =392
100805d7c: 5280190c    	mov	w12, #0xc8              ; =200
100805d80: 52800114    	mov	w20, #0x8               ; =8
100805d84: d28d0e99    	mov	x25, #0x6874            ; =26740
100805d88: f2acae59    	movk	x25, #0x6572, lsl #16
100805d8c: f2cc8c39    	movk	x25, #0x6461, lsl #32
100805d90: f2ec8939    	movk	x25, #0x6449, lsl #48
100805d94: f9003fea    	str	x10, [sp, #0x78]
100805d98: aa1303f6    	mov	x22, x19
100805d9c: 9b086e73    	madd	x19, x19, x8, x27
100805da0: a9422568    	ldp	x8, x9, [x11, #0x20]
100805da4: 9b0c2129    	madd	x9, x9, x12, x8
100805da8: a90ba7e8    	stp	x8, x9, [sp, #0xb8]
100805dac: f90067f3    	str	x19, [sp, #0xc8]
100805db0: 910283e8    	add	x8, sp, #0xa0
100805db4: 9102e3e0    	add	x0, sp, #0xb8
100805db8: 97f4dba1    	bl	0x10053cc3c <__ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17hd74a628107ffc5c0E>
100805dbc: f9006bff    	str	xzr, [sp, #0xd0]
100805dc0: f90073ff    	str	xzr, [sp, #0xe0]
100805dc4: 97dfedec    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100805dc8: 52800100    	mov	w0, #0x8                ; =8
100805dcc: 52800021    	mov	w1, #0x1                ; =1
100805dd0: 97dfede5    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100805dd4: 52800101    	mov	w1, #0x8                ; =8
100805dd8: b4009a40    	cbz	x0, 0x100807120 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x140c>
100805ddc: aa0003f8    	mov	x24, x0
100805de0: f9000019    	str	x25, [x0]
100805de4: a90682f4    	stp	x20, x0, [x23, #0x68]
100805de8: f9003ef4    	str	x20, [x23, #0x78]
100805dec: f9401679    	ldr	x25, [x19, #0x28]
100805df0: b7f89319    	tbnz	x25, #0x3f, 0x100807050 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x133c>
100805df4: f9401274    	ldr	x20, [x19, #0x20]
100805df8: b4000139    	cbz	x25, 0x100805e1c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x108>
100805dfc: 97dfedde    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100805e00: aa1903e0    	mov	x0, x25
100805e04: 52800021    	mov	w1, #0x1                ; =1
100805e08: 97dfedd7    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100805e0c: aa0003f5    	mov	x21, x0
100805e10: 52800020    	mov	w0, #0x1                ; =1
100805e14: b5000075    	cbnz	x21, 0x100805e20 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x10c>
100805e18: 1400048f    	b	0x100807054 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x1340>
100805e1c: 52800035    	mov	w21, #0x1               ; =1
100805e20: aa1503e0    	mov	x0, x21
100805e24: aa1403e1    	mov	x1, x20
100805e28: aa1903e2    	mov	x2, x25
100805e2c: 94272e20    	bl	0x1011d16ac <_writev+0x1011d16ac>
100805e30: 52800068    	mov	w8, #0x3                ; =3
100805e34: 390683e8    	strb	w8, [sp, #0x1a0]
100805e38: a90ad6f9    	stp	x25, x21, [x23, #0xa8]
100805e3c: f9005ef9    	str	x25, [x23, #0xb8]
100805e40: 910723e8    	add	x8, sp, #0x1c8
100805e44: 910343e0    	add	x0, sp, #0xd0
100805e48: 9105a3e1    	add	x1, sp, #0x168
100805e4c: 97fd427b    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
100805e50: f94066e8    	ldr	x8, [x23, #0xc8]
100805e54: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
100805e58: eb09011f    	cmp	x8, x9
100805e5c: 54000201    	b.ne	0x100805e9c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x188>
100805e60: f9406ae8    	ldr	x8, [x23, #0xd0]
100805e64: f94072e9    	ldr	x9, [x23, #0xe0]
100805e68: 8b091508    	add	x8, x8, x9, lsl #5
100805e6c: 3dc00100    	ldr	q0, [x8]
100805e70: 3d805ae0    	str	q0, [x23, #0x160]
100805e74: 3dc00500    	ldr	q0, [x8, #0x10]
100805e78: 3d805ee0    	str	q0, [x23, #0x170]
100805e7c: ad4502e1    	ldp	q1, q0, [x23, #0xa0]
100805e80: ad000101    	stp	q1, q0, [x8]
100805e84: 385803a8    	ldurb	w8, [x29, #-0x80]
100805e88: 7100191f    	cmp	w8, #0x6
100805e8c: 54000220    	b.eq	0x100805ed0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x1bc>
100805e90: d10203a0    	sub	x0, x29, #0x80
100805e94: 940176f6    	bl	0x100863a6c <__ZN4core3ptr45drop_in_place$LT$serde_json..value..Value$GT$17h92d2879214bf60a3E>
100805e98: 1400000e    	b	0x100805ed0 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x1bc>
100805e9c: 3ccc82e0    	ldur	q0, [x23, #0xc8]
100805ea0: 3ccd82e1    	ldur	q1, [x23, #0xd8]
100805ea4: ad0806e0    	stp	q0, q1, [x23, #0x100]
100805ea8: 3cce82e0    	ldur	q0, [x23, #0xe8]
100805eac: 3d804ae0    	str	q0, [x23, #0x120]
100805eb0: f9407ee8    	ldr	x8, [x23, #0xf8]
100805eb4: f9009ae8    	str	x8, [x23, #0x130]
100805eb8: ad4506e0    	ldp	q0, q1, [x23, #0xa0]
100805ebc: ad0a06e0    	stp	q0, q1, [x23, #0x140]
100805ec0: d10203a8    	sub	x8, x29, #0x80
100805ec4: d10383a0    	sub	x0, x29, #0xe0
100805ec8: d10283a1    	sub	x1, x29, #0xa0
100805ecc: 97f8200d    	bl	0x10060df00 <__ZN5alloc11collections5btree3map5entry28VacantEntry$LT$K$C$V$C$A$GT$12insert_entry17h956a274bfccbaa6eE>
100805ed0: 97dfeda9    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100805ed4: 52800160    	mov	w0, #0xb                ; =11
100805ed8: 52800021    	mov	w1, #0x1                ; =1
100805edc: 97dfeda2    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100805ee0: 52800161    	mov	w1, #0xb                ; =11
100805ee4: b40091e0    	cbz	x0, 0x100807120 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x140c>
100805ee8: aa0003fc    	mov	x28, x0
100805eec: 528c2a08    	mov	w8, #0x6150             ; =24912
100805ef0: 72ad0e88    	movk	w8, #0x6874, lsl #16
100805ef4: b8007008    	stur	w8, [x0, #0x7]
100805ef8: f0005d48    	adrp	x8, 0x1013b0000 <_anon.973b090bee24c06fdb949486dbea4242.108+0x3d8>
100805efc: 913c6108    	add	x8, x8, #0xf18
100805f00: f9400108    	ldr	x8, [x8]
100805f04: f9000008    	str	x8, [x0]
100805f08: 52800168    	mov	w8, #0xb                ; =11
100805f0c: f90083e8    	str	x8, [sp, #0x100]
100805f10: a900a2e0    	stp	x0, x8, [x23, #0x8]
100805f14: a9408660    	ldp	x0, x1, [x19, #0x8]
100805f18: 9105a3e8    	add	x8, sp, #0x168
100805f1c: 9425ccb1    	bl	0x1011791e0 <__ZN5alloc6string6String15from_utf8_lossy17h03e7be16830322ceE>
100805f20: a94752f8    	ldp	x24, x20, [x23, #0x70]
100805f24: b7f889f4    	tbnz	x20, #0x3f, 0x100807060 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x134c>
100805f28: b4000134    	cbz	x20, 0x100805f4c <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x238>
100805f2c: 97dfed92    	bl	0x100001574 <__RNvCs1Y7DaGC1cwg_7___rustc35___rust_no_alloc_shim_is_unstable_v2>
100805f30: aa1403e0    	mov	x0, x20
100805f34: 52800021    	mov	w1, #0x1                ; =1
100805f38: 97dfed8b    	bl	0x100001564 <__RNvCs1Y7DaGC1cwg_7___rustc12___rust_alloc>
100805f3c: aa0003f5    	mov	x21, x0
100805f40: 52800020    	mov	w0, #0x1                ; =1
100805f44: b5000075    	cbnz	x21, 0x100805f50 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x23c>
100805f48: 14000447    	b	0x100807064 <__ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h9bc790e5b164b940E+0x1350>
100805f4c: 52800035    	mov	w21, #0x1               ; =1
100805f50: aa1503e0    	mov	x0, x21
100805f54: aa1803e1    	mov	x1, x24
100805f58: aa1403e2    	mov	x2, x20
100805f5c: 94272dd4    	bl	0x1011d16ac <_writev+0x1011d16ac>
100805f60: 52800068    	mov	w8, #0x3                ; =3
100805f64: 390683e8    	strb	w8, [sp, #0x1a0]
100805f68: a90ad6f4    	stp	x20, x21, [x23, #0xa8]
100805f6c: f9005ef4    	str	x20, [x23, #0xb8]
100805f70: 910723e8    	add	x8, sp, #0x1c8
100805f74: 910343e0    	add	x0, sp, #0xd0
100805f78: 910403e1    	add	x1, sp, #0x100
100805f7c: 97fd422f    	bl	0x100756838 <__ZN5alloc11collections5btree3map25BTreeMap$LT$K$C$V$C$A$GT$5entry17h926085ab87dc1eedE>
