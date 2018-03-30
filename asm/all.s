        include	"equates.inc"

        org MEM_START

        code

start      
            lda #$55
            tfr a,cc
            lds #stack_top
            ldu #ustack_top
            lda #$0f

            lda #0
            sync

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

            ldy #SCREEN
2           stb ,y+
            cmpy #SCREEN+256
            bne 2B
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

;; x -> palette entry
;; y -> table
;; dd = speed through table
palette_cycler
        ldx #1f
        stx tsk_func,u
        rts

1
        
        rts

;; x -> palette entry
;; y -> table
;; dd = speed through table

alloc_cycler
        pshs  x,y,a,b
        jsr task_alloc
        puls x,y,a,b

        stx  tsk_x,u
        stx  tsk_y,u
        std  tsk_d,u

        rts



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

task_size       equ 16
num_of_tasks    equ 100

tsk_next        equ 0
tsk_func        equ 2
tsk_tick        equ 4
tsk_sleep       equ 5
tsk_d           equ 7
tsk_y           equ 9
tsk_x           equ 11

tasks           rmb task_size * num_of_tasks
tasks_end       rmb 0

active_tasks    rmb 2
free_tasks      rmb 2
current_task    rmb 2

task_init_system
    ;; no active tasks
    ;; x is my zero
    ldx #0
    stx active_tasks
    stx current_task

    ldu #tasks
    stu free_tasks
1
    ;; get ptr to next task
    leay task_size,u
    cmpy #tasks_end
    bne 2f
    tfr y,x

    ;; init the task
    ;; ???
2
    ;; Store set z flags
    ;; if x was zero then we're the end of the list
    sty ,u
    bne 1b
    rts


;; U -> current task
task_exec 
    lda tsk_tick,u
    beq 1f
    dec tsk_tick,u
    rts

1
    lda tsk_sleep,u
    sta tsk_tick,u

    ldd tsk_d,u
    ldx tsk_x,u
    ldy tsk_y,u
    
    jsr [tsk_func,u]

    std tsk_d,u
    stx tsk_x,u
    sty tsk_y,u

    rts

task_run_tasks
    ;; u -> active task list
    ldu  #active_tasks
2
    ;; u -> next task, if zero end of list
    ldu ,u
    beq 1f
    ;; set this as the current task
    stu current_task
    ;; run the task
    bsr task_exec
    ;; do the next one
    bra 2b
1
    rts


;; Allocates a task, will be executed after the current
;; task
;; X -> func
;; A = time till execute
;; U -> current task
;; preserves u
;; y -> new task
task_alloc
    ;; y -> free task
    ldy     free_tasks
    bne     1f

    ;; no tasks!
    swi2
    ;; func, tick, sleep for new task
1
    stx tsk_func,y
    sta tsk_tick,y
    clr tsk_sleep,y

    ;; Get next free task
    ldd ,y
    ;; free tasks now -> to that
    std free_tasks

    ;; insert myself into linked list
    ;; will execute after current task
    ldx ,u
    stx ,y
    sty ,u

    rts

;; x -> task to free
task_free
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

