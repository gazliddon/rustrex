    include	"equates.inc"

        org MEM_START

        CODE

start   
        lda #$55
        tfr a,cc
        lds #stack_top
        lda #$0f
        bsr init

loop    bra loop

init    bsr screen_clear
        rts

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; clear_scren
;; Clear the screen to pixel pair in A

screen_clear

    ldu #SCREEN
    ;; fill registers with clear color
    tfr a,b ;; d = a a
    tfr a,dp
    tfr d,x
    tfr d,y

loop2
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

    CMPU #SCREEN + SCREEN_SIZE_BYTES
    BNE loop2
    rts


		CODE

RESERVED 
SW3VEC
SW2VEC
FRQVEC
IRQVEC
SWIVEC
NMIVEC
loop10 
    bra loop10
    rti

    BSS
stack
    rmb $100
stack_top

		DATA

        ORG  $FFF0           
        FDB  $0000 
        FDB  SW3VEC
        FDB  SW2VEC
        FDB  FRQVEC
        FDB  IRQVEC
        FDB  SWIVEC
        FDB  NMIVEC
        FDB  start

