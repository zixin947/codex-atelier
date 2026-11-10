
/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/AiMaMi.arm64:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100680a24 <__ZN13codexmate_lib4core5relay7manager12RelayManager24set_codex_router_enabled17h8beef6301b4830bdE>:
100680d90: 911a03e8    	add	x8, sp, #0x680
100680d94: aa1703e0    	mov	x0, x23
100680d98: 94003007    	bl	0x10068cdb4 <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE>
100680d9c: 3941dfe8    	ldrb	w8, [sp, #0x77]
100680da0: b90053e8    	str	w8, [sp, #0x50]
100680da4: 7100051f    	cmp	w8, #0x1
100680da8: b90063f9    	str	w25, [sp, #0x60]
100680dac: f9002ffc    	str	x28, [sp, #0x58]
100680db0: 540004a1    	b.ne	0x100680e44 <__ZN13codexmate_lib4core5relay7manager12RelayManager24set_codex_router_enabled17h8beef6301b4830bdE+0x420>
100680db4: 9404f50c    	bl	0x1007be1e4 <__ZN13codexmate_lib8platform7process20is_codex_app_running17h76824f60ab18d1bcE>
100680db8: b9004fe0    	str	w0, [sp, #0x4c]
100680dbc: 395fb3e9    	ldrb	w9, [sp, #0x7ec]
100680dc0: 3941dbe8    	ldrb	w8, [sp, #0x76]
100680dc4: 6b08013f    	cmp	w9, w8
100680dc8: 54000481    	b.ne	0x100680e58 <__ZN13codexmate_lib4core5relay7manager12RelayManager24set_codex_router_enabled17h8beef6301b4830bdE+0x434>
100680dcc: 39423ff8    	ldrb	w24, [sp, #0x8f]
100680dd0: f9400ef9    	ldr	x25, [x23, #0x18]
100680dd4: 912b83e0    	add	x0, sp, #0xae0
100680dd8: 9102a3e2    	add	x2, sp, #0xa8
100680ddc: aa1903e1    	mov	x1, x25
100680de0: aa1803e3    	mov	x3, x24
100680de4: 52800004    	mov	w4, #0x0                ; =0
100680de8: 94001899    	bl	0x10068704c <__ZN13codexmate_lib4core5relay7manager12RelayManager33verify_target_state_at_with_depth17h97cab5c85896f945E>
100680dec: 910e43e8    	add	x8, sp, #0x390
100680df0: aa1703e0    	mov	x0, x23
100680df4: 94002ff0    	bl	0x10068cdb4 <__ZN13codexmate_lib4core5relay7manager12RelayManager8snapshot17h585677820424dd0dE>
100680df8: 912fc3e8    	add	x8, sp, #0xbf0
100680dfc: 910e43e1    	add	x1, sp, #0x390
100680e00: aa1703e0    	mov	x0, x23
100680e04: 94001a7f    	bl	0x100687800 <__ZN13codexmate_lib4core5relay7manager12RelayManager38codex_config_stale_reason_for_snapshot17hdd31c82dbcb1ff40E>
100680e08: 910e43e0    	add	x0, sp, #0x390
100680e0c: 9400fc32    	bl	0x1006bfed4 <__ZN4core3ptr67drop_in_place$LT$codexmate_lib..core..relay..models..RelayState$GT$17he3bbc346fbe4bfa7E>
100680e10: 912fc3ea    	add	x10, sp, #0xbf0
100680e14: f9400148    	ldr	x8, [x10]
100680e18: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
100680e1c: eb09011f    	cmp	x8, x9
100680e20: b9004bf8    	str	w24, [sp, #0x48]
100680e24: f90023f9    	str	x25, [sp, #0x40]
100680e28: 54000a81    	b.ne	0x100680f78 <__ZN13codexmate_lib4core5relay7manager12RelayManager24set_codex_router_enabled17h8beef6301b4830bdE+0x554>
100680e2c: f94126a1    	ldr	x1, [x21, #0x248]
100680e30: f9412aa2    	ldr	x2, [x21, #0x250]
100680e34: 912e03e0    	add	x0, sp, #0xb80
100680e38: 911a03e3    	add	x3, sp, #0x680
100680e3c: 940019a6    	bl	0x1006874d4 <__ZN13codexmate_lib4core5relay7manager12RelayManager38card_catalog_stale_reason_for_snapshot17hb48b4cf25213a526E>
100680e40: 14000052    	b	0x100680f88 <__ZN13codexmate_lib4core5relay7manager12RelayManager24set_codex_router_enabled17h8beef6301b4830bdE+0x564>
100680e44: b9004fff    	str	wzr, [sp, #0x4c]
100680e48: 395fb3e9    	ldrb	w9, [sp, #0x7ec]
100680e4c: 3941dbe8    	ldrb	w8, [sp, #0x76]
100680e50: 6b08013f    	cmp	w9, w8
100680e54: 54fffbc0    	b.eq	0x100680dcc <__ZN13codexmate_lib4core5relay7manager12RelayManager24set_codex_router_enabled17h8beef6301b4830bdE+0x3a8>
100680e58: 912403f9    	add	x25, sp, #0x900
100680e5c: 395ffbfc    	ldrb	w28, [sp, #0x7fe]
