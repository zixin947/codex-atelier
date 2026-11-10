
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010068e464 <__ZN13codexmate_lib4core5relay7manager19CodexRelaunchPolicy29relaunch_after_verified_state17hdf25fa26ef14220dE>:
10068e464: a9be4ff4    	stp	x20, x19, [sp, #-0x20]!
10068e468: a9017bfd    	stp	x29, x30, [sp, #0x10]
10068e46c: 910043fd    	add	x29, sp, #0x10
10068e470: aa0003f3    	mov	x19, x0
10068e474: 34000143    	cbz	w3, 0x10068e49c <__ZN13codexmate_lib4core5relay7manager19CodexRelaunchPolicy29relaunch_after_verified_state17hdf25fa26ef14220dE+0x38>
10068e478: 36000121    	tbz	w1, #0x0, 0x10068e49c <__ZN13codexmate_lib4core5relay7manager19CodexRelaunchPolicy29relaunch_after_verified_state17hdf25fa26ef14220dE+0x38>
10068e47c: aa0203f4    	mov	x20, x2
10068e480: 9404bf59    	bl	0x1007be1e4 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE>
10068e484: 360000d4    	tbz	w20, #0x0, 0x10068e49c <__ZN13codexmate_lib4core5relay7manager19CodexRelaunchPolicy29relaunch_after_verified_state17hdf25fa26ef14220dE+0x38>
10068e488: 350000a0    	cbnz	w0, 0x10068e49c <__ZN13codexmate_lib4core5relay7manager19CodexRelaunchPolicy29relaunch_after_verified_state17hdf25fa26ef14220dE+0x38>
10068e48c: aa1303e0    	mov	x0, x19
10068e490: a9417bfd    	ldp	x29, x30, [sp, #0x10]
10068e494: a8c24ff4    	ldp	x20, x19, [sp], #0x20
10068e498: 14000266    	b	0x10068ee30 <__ZN13codexmate_lib4core5relay7manager24launch_codex_app_warning17hbb01285663aa4543E>
10068e49c: d2f00008    	mov	x8, #-0x8000000000000000 ; =-9223372036854775808
10068e4a0: f9000268    	str	x8, [x19]
10068e4a4: a9417bfd    	ldp	x29, x30, [sp, #0x10]
10068e4a8: a8c24ff4    	ldp	x20, x19, [sp], #0x20
10068e4ac: d65f03c0    	ret
