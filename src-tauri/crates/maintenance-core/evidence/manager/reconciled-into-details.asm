
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010068ed6c <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE>:
10068ed6c: a9bd57f6    	stp	x22, x21, [sp, #-0x30]!
10068ed70: a9014ff4    	stp	x20, x19, [sp, #0x10]
10068ed74: a9027bfd    	stp	x29, x30, [sp, #0x20]
10068ed78: 910083fd    	add	x29, sp, #0x20
10068ed7c: b9400028    	ldr	w8, [x1]
10068ed80: a942d433    	ldp	x19, x21, [x1, #0x28]
10068ed84: 3cc08020    	ldur	q0, [x1, #0x8]
10068ed88: 3d800000    	str	q0, [x0]
10068ed8c: a941d029    	ldp	x9, x20, [x1, #0x18]
10068ed90: f9000809    	str	x9, [x0, #0x10]
10068ed94: 7100051f    	cmp	w8, #0x1
10068ed98: 540001a1    	b.ne	0x10068edcc <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x60>
10068ed9c: b4000315    	cbz	x21, 0x10068edfc <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x90>
10068eda0: 91002276    	add	x22, x19, #0x8
10068eda4: 14000004    	b	0x10068edb4 <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x48>
10068eda8: 910062d6    	add	x22, x22, #0x18
10068edac: f10006b5    	subs	x21, x21, #0x1
10068edb0: 54000260    	b.eq	0x10068edfc <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x90>
10068edb4: f85f82c1    	ldur	x1, [x22, #-0x8]
10068edb8: b4ffff81    	cbz	x1, 0x10068eda8 <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x3c>
10068edbc: f94002c0    	ldr	x0, [x22]
10068edc0: 52800022    	mov	w2, #0x1                ; =1
10068edc4: 97e5c9e9    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068edc8: 17fffff8    	b	0x10068eda8 <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x3c>
10068edcc: b4000195    	cbz	x21, 0x10068edfc <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x90>
10068edd0: 91002276    	add	x22, x19, #0x8
10068edd4: 14000004    	b	0x10068ede4 <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x78>
10068edd8: 910062d6    	add	x22, x22, #0x18
10068eddc: f10006b5    	subs	x21, x21, #0x1
10068ede0: 540000e0    	b.eq	0x10068edfc <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x90>
10068ede4: f85f82c1    	ldur	x1, [x22, #-0x8]
10068ede8: b4ffff81    	cbz	x1, 0x10068edd8 <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x6c>
10068edec: f94002c0    	ldr	x0, [x22]
10068edf0: 52800022    	mov	w2, #0x1                ; =1
10068edf4: 97e5c9dd    	bl	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068edf8: 17fffff8    	b	0x10068edd8 <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0x6c>
10068edfc: b4000134    	cbz	x20, 0x10068ee20 <__ZN13codexmate_lib4core5relay7manager22ReconciledRouterTarget12into_details17h259883809833bbfdE+0xb4>
10068ee00: 8b140688    	add	x8, x20, x20, lsl #1
10068ee04: d37df101    	lsl	x1, x8, #3
10068ee08: aa1303e0    	mov	x0, x19
10068ee0c: 52800102    	mov	w2, #0x8                ; =8
10068ee10: a9427bfd    	ldp	x29, x30, [sp, #0x20]
10068ee14: a9414ff4    	ldp	x20, x19, [sp, #0x10]
10068ee18: a8c357f6    	ldp	x22, x21, [sp], #0x30
10068ee1c: 17e5c9d3    	b	0x100001568 <__RNvCs1Y7DaGC1cwg_7___rustc14___rust_dealloc>
10068ee20: a9427bfd    	ldp	x29, x30, [sp, #0x20]
10068ee24: a9414ff4    	ldp	x20, x19, [sp, #0x10]
10068ee28: a8c357f6    	ldp	x22, x21, [sp], #0x30
10068ee2c: d65f03c0    	ret
