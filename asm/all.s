        org MEM_START

        include	"equates.s"

start      lda #$55
           tfr a,cc
           lds #stack_top

            jsr clear_screen

            lda #0
            sync

            ;; copy the palette over

            lda #255
            sta PALETTE+3
            lda #127
            sta PALETTE+4
            lda #0
            sta PALETTE+5
            ;;
@loop
            ;;
            inc PALETTE+15*3
            inc PALETTE+15*3
            inc PALETTE+15*3
            lda PALETTE+15
            lsra
            sta PALETTE+15*3+1
            ;;
            ldx #0
            bsr draw_box
            ;;
            jsr print_a
            ;;
            sync
            nop
            nop
            bra @loop

clear_screen
    ldy #SCREEN
    ldd #0
@loop
    std ,y++
    cmpy #SCREEN+SCREEN_SIZE_BYTES
    bne @loop
    rts
font
    fdb small_a
    fdb small_b
    fdb small_c

small_a
    fdb $fff0
    fdb $f0f0
    fdb $fff0
    fdb $f0f0
    fdb $f0f0
    fdb $0000

small_a_2
    fdb $fff0,$f0f0,$fff0
    fdb $f0f0,$ff00,$0000
    fdb $0,$0,$0



small_b
    fdb $fff0
    fdb $f0f0
    fdb $ff00
    fdb $f0f0
    fdb $fff0
    fdb $0000
small_c
    fdb $0f00
    fdb $f0f0
    fdb $f000
    fdb $f0f0
    fdb $0f00
    fdb $0000


;; Draw a block
;; a = col
;; x = addr

;;yxba
draw_box
    pshs u
    ldy small_a_2
    ldx small_a_2+2
    ldd small_a_2+4

    ldu #SCREEN+6
    pshu a,b,x,y

    ldy small_a_2+0+6
    ldx small_a_2+2+6
    ldd small_a_2+4+6
    ldu #SCREEN+6+0x100
    pshu a,b,x,y
    
    ldy small_a_2+0+12
    ldx small_a_2+2+12
    ldd small_a_2+4+12
    ldu #SCREEN+6+0x200
    pshu a,b,x,y

    puls u
    rts

    

    


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; x -> palette entry
;; y -> table
;; dd = speed through table

init_x equ 0
init_y equ 2
init_d equ 4

palette_cycler
        std task.temp0,u
        ldd #@resume
        std task.func,u
        ldd #0
@resume

        rts

;; x -> reg_init
;; dd = speed through table

alloc_with_init
        pshs x
        jsr task_alloc
        puls x

        ldd init_d,x
        std task.d,y

        ldd init_x,x
        std task.x,y

        ldd init_y,x
        std task.y,y
        rts

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; X -> palette to read from

copy_pal

            lda #61*3-1
            pshu a
            ldy #PALETTE

@loop       ldd ,x++
            std ,y++
            dec ,u
            bpl @loop
            rts
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; u -> data
;; x -> screen
lett_a
    fdb 0x00
    fdb 0xf0
    fdb 0xf0
    fdb 0xf0
    fdb 0xf0
    fdb 0x00

    fdb 0x00
    fdb 0x00
    fdb 0x00
    fdb 0xff
    fdb 0x00
    fdb 0xff

    fdb 0x00
    fdb 0xf0
    fdb 0xf0
    fdb 0xff
    fdb 0x00
    fdb 0x0f

orig_scr   rmb 2
save_s     rmb 2

print_6x6
    sts save_s

    stx orig_scr
    tfr x,s

    pulu x,y,a,b
    pshs x,y,a,b

    dec orig_scr
    lds orig_scr
    pulu x,y,a,b
    pshs x,y,a,b

    dec orig_scr
    lds orig_scr
    pulu x,y,a,b
    pshs x,y,a,b

    lds save_s
    rts

print_a
    sts save_s
    ldx #SCREEN+0x800+6*3
    ldu #lett_a
    lds save_s
    rts
    jmp print_6x6

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; clear_scren
;; Clear the screen to pixel pair in A
screen_clear
    pshs u

    ldu #SCREEN
    ;; fill registers with clear color
    tfr a,b ;; d = a a
    tfr a,dp
    tfr d,x
    tfr d,y

@loop
    ;; stack blasts 36 * 7 bytes + 4 bytes = 256 bytes
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu a,b,d,x,y
    pshu x,y
    ;;
    cmpu #SCREEN + SCREEN_SIZE_BYTES
    bne @loop

    puls u
    rts


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

    include "tasker.s"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
reserved 
sw3vec
sw2vec
frqvec
irqvec
swivec
nmivec

@loop
    bra @loop

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
ustack  rmb $100
ustack_top

stack   rmb $100
stack_top

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

        org  $fff0           
        fdb  $0000 
        fdb  sw3vec
        fdb  sw2vec
        fdb  frqvec
        fdb  irqvec
        fdb  swivec
        fdb  nmivec
        fdb  start
