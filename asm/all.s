        include	"equates.inc"

        org MEM_START

        code

start       
    sync
            lda #$55
            tfr a,cc
            lds #stack_top
            ldu #ustack_top
            lda #$0f

            ldx #pal
            bsr copy_pal

            ldb #0

1          
            sta PALETTE
            inca
            sta PALETTE+1
            inca
            sta PALETTE+2
            inca
            stb SCREEN
            stb SCREEN+1
            stb SCREEN+3
            incb
            sync
            bra 1B


pal     fcb  $00,$0,$0
        fcb  $10,$00,$00
        fcb  $20,$00,$00
        fcb  $30,$00,$00
        fcb  $40,$00,$00
        fcb  $50,$00,$00
        fcb  $60,$00,$00
        fcb  $70,$00,$00
        fcb  $80,$00,$00
        fcb  $90,$00,$00
        fcb  $a0,$00,$00
        fcb  $b0,$00,$00
        fcb  $c0,$00,$00
        fcb  $d0,$00,$00
        fcb  $e0,$00,$00
        fcb  $f0,$00,$00

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; X -> palette to read from
copy_pal    lda #(16 * 3) -1
            pshu a
            ldy #PALETTE
1           ldd ,x++
            std ,y++
            dec ,u
            bpl 1B
            rts

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

1
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

    cmpu #SCREEN + SCREEN_SIZE_BYTES
    bne 1B
    puls u
    rts

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
reserved 
sw3vec
sw2vec
frqvec
irqvec
swivec
nmivec

1    bra 1B

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
        bss
ustack  rmb $100
ustack_top

stack   rmb $100
stack_top

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		data

        org  $fff0           
        fdb  $0000 
        fdb  sw3vec
        fdb  sw2vec
        fdb  frqvec
        fdb  irqvec
        fdb  swivec
        fdb  nmivec
        fdb  start

