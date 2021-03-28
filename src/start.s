.cpu mpcore
.section ".crt0","ax"

.align 2
.arm

_start:
    b pre_main

.ascii "_prm"
__service_ptr:
    .word 0
__apt_appid:
    .word 0x300
__heap_size:
    .word 24*1024*1024
__linear_heap_size:
	.word 32*1024*1024
__system_arglist:
    .word 0
__system_runflags:
	.word 0
