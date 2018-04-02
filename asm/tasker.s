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
