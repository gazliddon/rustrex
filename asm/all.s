        include	"equates.inc"

        org MEM_START

start      lda #$55
            tfr a,cc
            lds #stack_top
            ldu #ustack_top
            lda #$0f

            lda #0
            sync

            ldx #pal
            bsr copy_pal

            ldb #0
@loop       sta PALETTE
            inca
            sta PALETTE+1
            inca
            sta PALETTE+2
            inca
            ldy #SCREEN
@loop2      stb ,y+
            cmpy #SCREEN+256
            bne @loop2
            incb
            sync
            bra @loop


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

task    STRUCT

next        rmd 1
func        rmd 1
tick        rmb 1
sleep       rmb 1
d           rmd 1   
y           rmd 1
x           rmd 1
temp0       rmd 1
temp1       rmd 1 

        ENDSTRUCT

tsk_size        equ sizeof{task}

num_of_tasks    equ 100

tasks           rmb tsk_size * num_of_tasks

tasks_end       rmb 0

active_tasks    rmd 1
free_tasks      rmd 1
current_task    rmd 2

task_init_system
    ;; no active tasks
    ;; x is my zero
    ldx #0
    stx active_tasks
    stx current_task

    ldu #tasks
    stu free_tasks
@loop
    ;; get ptr to next task
    leay tsk_size,u
    cmpy #tasks_end
    bne @no_task
    tfr y,x
    ;;
@no_task
    ;; Store set z flags
    ;; if x was zero then we're the end of the list
    sty ,u
    bne @loop
    rts


;; U -> current task
task_exec 
    lda task.tick,u
    beq @run_task
    dec task.tick,u
    rts
    ;;
@run_task
    lda task.sleep,u
    sta task.tick,u


    ldd task.d,u
    ldx task.x,u
    ldy task.y,u
    
    jsr [task.func,u]

    leau tsk_size,u 
    pshu d,x,y
    leau -tsk_size+6,y

    rts

task_run_tasks
    ;; u -> active task list
    ldu  #active_tasks
@loop
    ;; u -> next task, if zero end of list
    ldu ,u
    beq @done
    ;; set this as the current task
    stu current_task
    ;; run the task
    bsr task_exec
    ;; do the next one
    bra @loop
@done
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
    bne     @got_task
    ;;
    ;; no tasks!
    swi2
    ;; func, tick, sleep for new task
@got_task
    stx task.func,y
    sta task.tick,y
    clr task.sleep,y

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

