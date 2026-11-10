
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010078ebd8 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E>:
10078fffc: 394203e8    	ldrb	w8, [sp, #0x80]
100790000: 3707dae8    	tbnz	w8, #0x0, 0x10078fb5c <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xf84>
100790004: b00060c1    	adrp	x1, 0x1013a9000 <_anon.cdb272158a04aae91ce02c6e47f106c7.539+0xd4>
100790008: 913a5c21    	add	x1, x1, #0xe97
10079000c: 910203e0    	add	x0, sp, #0x80
100790010: 91006303    	add	x3, x24, #0x18
100790014: 528000c2    	mov	w2, #0x6                ; =6
100790018: 97f66e71    	bl	0x10052b9dc <__ZN10serde_core3ser12SerializeMap15serialize_entry17hf92b925f2096e89dE>
10079001c: aa0003f7    	mov	x23, x0
100790020: b5ffdb40    	cbnz	x0, 0x10078fb88 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0xfb0>
100790024: 17ffff1f    	b	0x10078fca0 <__ZN13codexmate_lib4core5relay18transition_journal12append_event17h0573aae81068a251E+0x10c8>
