

;; Screen Data
MEM_START              equ 0x9900

;; Screen Data

SCREEN                 equ $a000
SCREEN_W               equ 302
SCREEN_H               equ 256
SCREEN_W_BYTES         equ SCREEN_W / 2
SCREEN_SIZE_BYTES      equ SCREEN_W_BYTES * SCREEN_H