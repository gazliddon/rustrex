;*************************************************************************
;* 6502 emulator                                                         *
;* written by Larry Bank                                                 *
;* Copyright (c) 1998 BitBank Software, Inc.                             *
;* project start 1/30/98                                                 *
;*                                                                       *
;* Since optimizing C compilers crank out crap, it becomes               *
;* necessary to write it in asm to get any speed out of it.              *
;*************************************************************************

.386p

MEM_ROM   equ 0        ; Offset to first bank (ROM)
MEM_RAM   equ 10000h   ; Offset to second bank (RAM)
MEM_FLAGS equ 20000h   ; Offset to flags in memory map

; 6502 interrupt flags
INT_NMI  equ  1
INT_FIRQ equ  2  ; not used on 6502
INT_IRQ  equ  4

; 6502 status flags
F_CARRY    equ  1
F_ZERO     equ  2
F_IRQMASK  equ  4
F_DECIMAL  equ  8
F_BREAK    equ  16
F_UNUSED   equ  32
F_OVERFLOW equ  64
F_NEGATIVE equ  128

_DATA     segment dword public use32 'DATA'

;
; Opcode jump table
;
optable  dd offset flat:op_brk,offset flat:op_or_zxind,offset flat:op_illegal,offset flat:op_illegal         ; 00-03
         dd offset flat:op_illegal,offset flat:op_or_z,offset flat:op_asl_z,offset flat:op_illegal           ; 04-07
         dd offset flat:op_php,offset flat:op_or_i,offset flat:op_asla,offset flat:op_illegal                ; 08-0B
         dd offset flat:op_illegal,offset flat:op_or_a,offset flat:op_asl_a,offset flat:op_illegal           ; 0C-0F
         dd offset flat:op_bpl,offset flat:op_or_zyind,offset flat:op_illegal,offset flat:op_illegal         ; 10-13
         dd offset flat:op_illegal,offset flat:op_or_zx,offset flat:op_asl_zx,offset flat:op_illegal         ; 14-17
         dd offset flat:op_clc,offset flat:op_or_ay,offset flat:op_illegal,offset flat:op_illegal            ; 18-1B
         dd offset flat:op_illegal,offset flat:op_or_ax,offset flat:op_asl_ax,offset flat:op_illegal         ; 1C-1F
         dd offset flat:op_jsr_a,offset flat:op_and_zxind,offset flat:op_illegal,offset flat:op_illegal       ; 20-23
         dd offset flat:op_bit_z,offset flat:op_and_z,offset flat:op_rol_z,offset flat:op_illegal             ; 24-27
         dd offset flat:op_plp,offset flat:op_and_i,offset flat:op_rola,offset flat:op_illegal                ; 28-2B
         dd offset flat:op_bit_a,offset flat:op_and_a,offset flat:op_rol_a,offset flat:op_illegal             ; 2C-2F
         dd offset flat:op_bmi,offset flat:op_and_zyind,offset flat:op_illegal,offset flat:op_illegal         ; 30-33
         dd offset flat:op_illegal,offset flat:op_and_zx,offset flat:op_rol_zx,offset flat:op_illegal         ; 34-37
         dd offset flat:op_sec,offset flat:op_and_ay,offset flat:op_illegal,offset flat:op_illegal            ; 38-3B
         dd offset flat:op_illegal,offset flat:op_and_ax,offset flat:op_rol_ax,offset flat:op_illegal         ; 3C-3F
         dd offset flat:op_rti,offset flat:op_eor_zxind,offset flat:op_illegal,offset flat:op_illegal         ; 40-43
         dd offset flat:op_illegal,offset flat:op_eor_z,offset flat:op_lsr_z,offset flat:op_illegal           ; 44-47
         dd offset flat:op_pha,offset flat:op_eor_i,offset flat:op_lsr,offset flat:op_illegal                 ; 48-4B
         dd offset flat:op_jmp_a,offset flat:op_eor_a,offset flat:op_lsr_a,offset flat:op_illegal             ; 4C-4F
         dd offset flat:op_bvc,offset flat:op_eor_zyind,offset flat:op_illegal,offset flat:op_illegal         ; 50-53
         dd offset flat:op_illegal,offset flat:op_eor_zx,offset flat:op_lsr_zx,offset flat:op_illegal         ; 54-57
         dd offset flat:op_cli,offset flat:op_eor_ay,offset flat:op_illegal,offset flat:op_illegal            ; 58-5B
         dd offset flat:op_illegal,offset flat:op_eor_ax,offset flat:op_lsr_ax,offset flat:op_illegal         ; 5C-5F
         dd offset flat:op_rts,offset flat:op_adc_zxind,offset flat:op_illegal,offset flat:op_illegal         ; 60-63
         dd offset flat:op_illegal,offset flat:op_adc_z,offset flat:op_ror_z,offset flat:op_illegal           ; 64-67
         dd offset flat:op_pla,offset flat:op_adc_i,offset flat:op_ror,offset flat:op_illegal                 ; 68-6B
         dd offset flat:op_jmp_ind,offset flat:op_adc_a,offset flat:op_ror_a,offset flat:op_illegal           ; 6C-6F
         dd offset flat:op_bvs,offset flat:op_adc_zyind,offset flat:op_illegal,offset flat:op_illegal         ; 70-73
         dd offset flat:op_illegal,offset flat:op_adc_zx,offset flat:op_ror_zx,offset flat:op_illegal         ; 74-77
         dd offset flat:op_sei,offset flat:op_adc_ay,offset flat:op_illegal,offset flat:op_illegal            ; 78-7B
         dd offset flat:op_illegal,offset flat:op_adc_ax,offset flat:op_ror_ax,offset flat:op_illegal         ; 7C-7F
         dd offset flat:op_illegal,offset flat:op_sta_zxind,offset flat:op_illegal,offset flat:op_illegal     ; 80-83
         dd offset flat:op_sty_z,offset flat:op_sta_z,offset flat:op_stx_z,offset flat:op_illegal             ; 84-87
         dd offset flat:op_dey,offset flat:op_illegal,offset flat:op_txa,offset flat:op_illegal               ; 88-8B
         dd offset flat:op_sty_a,offset flat:op_sta_a,offset flat:op_stx_a,offset flat:op_illegal             ; 8C-8F
         dd offset flat:op_bcc,offset flat:op_sta_zyind,offset flat:op_illegal, offset flat:op_illegal        ; 90-93
         dd offset flat:op_sty_zx,offset flat:op_sta_zx,offset flat:op_stx_zy,offset flat:op_illegal          ; 94-97
         dd offset flat:op_tya,offset flat:op_sta_ay,offset flat:op_txs,offset flat:op_illegal                ; 98-9B
         dd offset flat:op_illegal,offset flat:op_sta_ax,offset flat:op_illegal,offset flat:op_illegal        ; 9C-9F
         dd offset flat:op_ldy_i,offset flat:op_lda_zxind,offset flat:op_ldx_i,offset flat:op_illegal         ; A0-A3
         dd offset flat:op_ldy_z,offset flat:op_lda_z,offset flat:op_ldx_z,offset flat:op_illegal             ; A4-A7
         dd offset flat:op_tay,offset flat:op_lda_i,offset flat:op_tax,offset flat:op_illegal                 ; A8-AB
         dd offset flat:op_ldy_a,offset flat:op_lda_a,offset flat:op_ldx_a,offset flat:op_illegal             ; AC-AF
         dd offset flat:op_bcs,offset flat:op_lda_zyind,offset flat:op_illegal,offset flat:op_illegal         ; B0-B3
         dd offset flat:op_ldy_zx,offset flat:op_lda_zx,offset flat:op_ldx_zy,offset flat:op_illegal          ; B4-B7
         dd offset flat:op_clv,offset flat:op_lda_ay,offset flat:op_tsx,offset flat:op_illegal                ; B8-BB
         dd offset flat:op_ldy_ax,offset flat:op_lda_ax,offset flat:op_ldx_ay,offset flat:op_illegal          ; BC-BF
         dd offset flat:op_cpy_i,offset flat:op_cmp_zxind,offset flat:op_illegal,offset flat:op_illegal       ; C0-C3
         dd offset flat:op_cpy_z,offset flat:op_cmp_z,offset flat:op_dec_z,offset flat:op_illegal             ; C4-C7
         dd offset flat:op_iny,offset flat:op_cmp_i,offset flat:op_dex,offset flat:op_illegal                 ; C8-CB
         dd offset flat:op_cpy_a,offset flat:op_cmp_a,offset flat:op_dec_a,offset flat:op_illegal             ; CC-CF
         dd offset flat:op_bne,offset flat:op_cmp_zyind,offset flat:op_illegal,offset flat:op_illegal         ; D0-D3
         dd offset flat:op_illegal,offset flat:op_cmp_zx,offset flat:op_dec_zx,offset flat:op_illegal         ; D4-D7
         dd offset flat:op_cld,offset flat:op_cmp_ay,offset flat:op_illegal,offset flat:op_illegal            ; D8-DB
         dd offset flat:op_illegal,offset flat:op_cmp_ax,offset flat:op_dec_ax,offset flat:op_illegal         ; DC-DF
         dd offset flat:op_cpx_i,offset flat:op_sbc_zxind,offset flat:op_illegal,offset flat:op_illegal       ; E0-E3
         dd offset flat:op_cpx_z,offset flat:op_sbc_z,offset flat:op_inc_z,offset flat:op_illegal             ; E4-E7
         dd offset flat:op_inx,offset flat:op_sbc_i,offset flat:op_nop,offset flat:op_illegal                 ; E8-EB
         dd offset flat:op_cpx_a,offset flat:op_sbc_a,offset flat:op_inc_a,offset flat:op_illegal             ; EC-EF
         dd offset flat:op_beq,offset flat:op_sbc_zyind,offset flat:op_illegal,offset flat:op_illegal         ; F0-F3
         dd offset flat:op_illegal,offset flat:op_sbc_zx,offset flat:op_inc_zx,offset flat:op_illegal         ; F4-F7
         dd offset flat:op_sed,offset flat:op_sbc_ay,offset flat:op_illegal,offset flat:op_illegal            ; F8-FB
         dd offset flat:op_illegal,offset flat:op_sbc_ax,offset flat:op_inc_ax,offset flat:op_illegal         ; FC-FF

mem_map  dd 0  ; points to CPU map + flags
pregs    dd 0  ; points to register structure passed in
pemuh    dd 0  ; points to memory handler structure
clocks   dd 0  ; number of clock cycles to execute
temp     dd 0  ; temp storage

regs struct
   sreg_pc   dw 0
   sreg_x    db 0
   sreg_y    db 0
   sreg_s    db 0
   sreg_a    db 0
   sreg_p    db 0
regs  ends

align 4
pending dd 0            ; address of pending interrupt flag
; 6502 registers, global vars for speed
reg_x  db 0
reg_y  db 0
reg_s  db 0
intflag db 0       ; interrupt pending flag

; Cycle times by opcode
m6502Cycles db  7,6,0,0,0,3,5,0,3,2,2,0,0,4,6,6  ; 00-0F
            db  2,5,0,0,0,4,6,6,2,4,0,0,0,5,7,7  ; 10-1F
            db  6,6,0,0,3,3,5,0,4,2,2,0,4,4,6,0  ; 20-2F
            db  2,5,0,0,0,4,6,6,2,4,0,0,0,5,7,0  ; 30-3F
            db  6,6,0,0,0,3,5,5,3,2,2,0,3,4,6,0  ; 40-4F
            db  2,5,0,0,0,4,6,6,2,4,0,0,0,5,7,7  ; 50-5F
            db  6,6,0,0,0,3,5,5,4,2,2,0,5,4,6,0  ; 60-6F
            db  2,5,0,0,0,4,6,0,2,4,0,0,0,5,7,0  ; 70-7F
            db  0,6,0,0,3,3,3,0,2,0,2,0,4,4,4,0  ; 80-8F
            db  2,6,0,0,4,4,4,0,2,5,2,0,0,5,0,0  ; 90-9F
            db  2,6,2,0,3,3,3,0,2,2,2,0,4,4,4,0  ; A0-AF
            db  2,5,0,0,4,4,4,0,2,4,2,0,4,4,4,0  ; B0-BF
            db  2,6,0,0,3,3,5,0,2,2,2,0,4,4,6,0  ; C0-CF
            db  2,5,0,0,0,4,6,0,2,4,0,0,0,5,7,0  ; D0-DF
            db  2,6,0,0,3,3,5,0,2,2,2,0,4,4,6,0  ; E0-EF
            db  2,5,0,0,0,4,6,0,2,4,0,0,0,5,7,0  ; F0-FF
;
; Zero and negative flags (quick lookup)
;
m6502NZ db  02h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h   ; /* 00-0F */
        db  00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h   ; /* 10-1F */
        db  00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h   ; /* 20-2F */
        db  00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h   ; /* 30-3F */
        db  00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h   ; /* 40-4F */
        db  00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h   ; /* 50-5F */
        db  00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h   ; /* 60-6F */
        db  00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h,00h   ; /* 70-7F */
        db  80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h   ; /* 80-8F */
        db  80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h   ; /* 90-9F */
        db  80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h   ; /* A0-AF */
        db  80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h   ; /* B0-BF */
        db  80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h   ; /* C0-CF */
        db  80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h   ; /* D0-DF */
        db  80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h   ; /* E0-EF */
        db  80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h,80h   ; /* F0-FF */

_DATA     ends
_TEXT   segment dword public use32 'CODE'
        assume ds:flat, ss:flat
        public _aexec6502, _areset6502
;
; Effective address macros
; assume: AX= 8 or 16-bit operand, edx=PC
;
M_EA_AX macro       ;  absolute offset from x
    add  al,reg_x
    adc  ah,0
    add  edx,2      ; PC += 2
        endm

M_EA_AY macro       ;  absolute offset from y
    add  al,reg_y
    adc  ah,0
    add  edx,2      ; PC += 2
        endm

M_EA_ZX macro       ; zero page + x
    add  al,reg_x
    inc  edx        ; PC++
    xor  ah,ah      ; only need 8 bits from (PC)
        endm

M_EA_ZY macro       ; zero page + y
    add  al,reg_y
    inc  edx        ; PC++
    xor  ah,ah      ; only need 8 bits from (PC)
        endm

M_EA_ZXIND macro    ; zero page + x indirect
    add  al,reg_x   ; zero page + x
    xor  ah,ah      ; zero page
    inc  edx        ; PC++
    mov  bl,[esi+eax+MEM_RAM]   ; need to do this in case of wrap-around
    inc  al
    mov  ah,[esi+eax+MEM_RAM]
    mov  al,bl
         endm

M_EA_ZYIND macro    ; zero page + y indirect
    xor  ah,ah      ; zero page
    inc  edx        ; PC++
    mov  bl,[esi+eax+MEM_RAM]   ; need to do this in case of wrap-around
    inc  al
    mov  ah,[esi+eax+MEM_RAM]
    add  bl,reg_y   ; add y
    adc  ah,0
    mov  al,bl
           endm

M_EA_REL macro      ; relative addressing (conditional)
    inc  edx        ; PC++
    cbw             ; sign extend the offset
         endm

M_EA_Z   macro          ; zero page addressing
    xor  ah,ah          ; kill the high byte
    inc  edx            ; PC++
         endm

M_EA_A  macro       ; absolute addressing
    add  edx,2          ; PC += 2
        endm

M_EA_I  macro       ; immediate value
    mov  bl,al      ; get in temp reg
    inc  edx        ; PC++
        endm
;
; Memory handling macros
;
M_RDB   macro      ; read a byte and check for hardware
        local @1,@2,@3
        test  byte ptr [esi+eax+MEM_FLAGS],-1   ; normal RAM?
        jnz   @1
        mov   bl,[esi+eax+MEM_RAM]
        jmp short @3
@1:     mov   bl,[esi+eax+MEM_FLAGS]
		cmp   bl,1      ; ROM?
        jnz   @2
        mov   bl,[esi+eax+MEM_ROM]
        jmp short @3
; call hardware handler
@2:     call  read_slow
@3:     nop
        endm

M_WRB   macro      ; write a byte and check for hardware
        local @1,@2,@3
        test  byte ptr [esi+eax+MEM_FLAGS],-1      ; normal RAM?
        jnz   @1
        mov   [esi+eax+MEM_RAM],bl
        jmp short @3
@1:     cmp   byte ptr [esi+eax+MEM_FLAGS],1      ; ROM?
        jz    @3          ; can't write to ROM, leave
; call hardware handler
@2:     call  write_slow
@3:     nop
        endm

M_RDBZ  macro      ; read a byte from zero page (no need to check hardware)
        mov  bl,[esi+eax+MEM_RAM]       ; store in RAM
        endm

M_WRBZ  macro      ; write a byte to zero page (no need to check hardware)
        mov  [esi+eax+MEM_RAM],bl       ; store in RAM
        endm

M_PUSHA  macro     ; push register A onto the stack
        mov  bl,reg_s
        mov  [esi+ebx+MEM_RAM+100h],cl       ; store in RAM
        dec  reg_s
         endm

M_PUSHP  macro     ; push the flags onto the stack
        mov  bl,reg_s
        mov  [esi+ebx+MEM_RAM+100h],ch       ; store in RAM
        dec  reg_s
         endm

M_PUSHPC  macro     ; push PC on the stack
        mov  bl,reg_s
        mov  [esi+ebx+MEM_RAM+100h],dh
        mov  [esi+ebx+MEM_RAM+0ffh],dl
        sub  reg_s,2
          endm

M_PULLPC macro
        mov  bl,reg_s
        mov  dl,[esi+ebx+MEM_RAM+101h]
        mov  dh,[esi+ebx+MEM_RAM+102h]
        add  reg_s,2
         endm

M_PULLA  macro     ; pull register A from the stack
        mov  bl,reg_s
        mov  cl,[esi+ebx+MEM_RAM+101h]
        inc  reg_s
        mov  bl,cl
        and  ch,255-(F_ZERO + F_NEGATIVE)
        or   ch,m6502NZ[ebx]
         endm

M_PULLP  macro     ; pull the flags register from the stack
        mov  bl,reg_s
        mov  ch,[esi+ebx+MEM_RAM+101h]
        inc  reg_s
         endm
;
; Flags and operations macros
;
M_FLAGSNZ  macro        ; set the zero and negative flags according to BL
     and  ch,255-(F_ZERO + F_NEGATIVE)
     or   ch,m6502NZ[ebx]
           endm

M_OR  macro
     and  ch,255-(F_ZERO + F_NEGATIVE)
     or   cl,bl
     mov  bl,cl
     or   ch,m6502NZ[ebx]
      endm

M_EOR  macro
     and  ch,255-(F_ZERO + F_NEGATIVE)
     xor  cl,bl
     mov  bl,cl
     or   ch,m6502NZ[ebx]
       endm

M_AND  macro
     and  ch,255-(F_ZERO + F_NEGATIVE)
     and  cl,bl
     mov  bl,cl
     or   ch,m6502NZ[ebx]
       endm

M_DEC  macro
     and  ch,255-(F_ZERO + F_NEGATIVE)
     dec  bl
     or   ch,m6502NZ[ebx]
       endm

M_INC  macro
     and  ch,255-(F_ZERO + F_NEGATIVE)
     inc  bl
     or   ch,m6502NZ[ebx]
       endm

M_BIT macro
      and  ch,255-(F_OVERFLOW + F_ZERO + F_NEGATIVE)
      mov  al,bl           ; keep a copy for later
      and  al,F_OVERFLOW + F_NEGATIVE
      or   ch,al        ; get the top two bits as-is
      and  bl,cl        ; AND with accumulator
	  jnz  @F
      or   ch,F_ZERO    ; don't disturb N bit since it was set from operand directly
@@:   nop
      endm

M_SBC macro
    local do_bcd, m_sbc_z
    mov   ah,ch				; Set AH = 6502 flags
	and   ch,255-(F_OVERFLOW + F_ZERO + F_CARRY + F_NEGATIVE)
    test  ah,F_DECIMAL      ; Doing BCD?
    jnz   do_bcd
    shr   ah,1              ; Move 6502 Carry to 80x86's
    cmc                     ; 6502 uses backwards logic for subtraction
    sbb   cl,bl             ; Do the SBC
    cmc                     ; 6502 backwards logic
    mov   ah,0              ; use AH to set overflow without affecting carry
	mov   bl,cl             ; get result in BL for setting flags
	jno   @F
    mov   ah,F_OVERFLOW
@@: adc   ch,0              ; set carry flag 
    or    ch,ah             ; set V if needed
    or    ch,m6502NZ[ebx]   ; set N and Z
    jmp  short m_sbc_z      ; done!
do_bcd: shr ah,1            ; Move 6502 Carry to 80x86's
    cmc                     ; 6502 uses backwards logic for subtraction
    sbb   cl,bl             ; Do the SBC
    mov   al,cl             ; 80x86 accum to 6502's
    das                     ; Decimal adjust
    cmc                     ; 6502 uses backwards logic for subtraction
    mov   ah,0              ; use AH to set overflow without affecting carry
    mov   bl,al             ; Back to 6502 accum
	jno   @F
    mov   ah,F_OVERFLOW
@@: adc   ch,0              ; set carry flag
    or    ch,ah             ; set V if needed
    or    ch,m6502NZ[ebx]   ; set N and Z
m_sbc_z: mov cl,bl          ; put result in accumulator
      endm

M_ADC macro
    local do_bcd, m_adc_z
	mov	  ah,ch				; Set AH = 6502 flags
	and   ch,255-(F_OVERFLOW + F_ZERO + F_CARRY + F_NEGATIVE)
	test  ah,F_DECIMAL		; Doing BCD?
	jnz	  do_bcd
	shr   ah,1				; Move 6502 Carry to 80x86's
	adc	  bl,cl				; Do the ADC
    mov   ah,0              ; use AH to set overflow without affecting carry
	jno   @F	            ; Reset V flag
	mov   ah,F_OVERFLOW		; SET V flag
@@: adc   ch,0				; Set carry bit according to result
    or    ch,ah             ; set V flag if needed
    or    ch,m6502NZ[ebx]   ; Set N and Z
	jmp short m_adc_z
do_bcd:	shr	ah,1			; Move 6502 Carry to 80x86's
	adc	bl,cl               ; Do the ADC
	mov	al,bl				; 80x86 accum to 6502's
	daa		                ; Decimal adjust
    mov   ah,0              ; use AH to set overflow without affecting carry
	jno   @F	            ; Reset V flag
	mov   ah,F_OVERFLOW     ; SET V flag
@@: adc   ch,0				; Set carry bit according to result
    or    ch,ah             ; set V flag if needed
    mov   bl,al
    or    ch,m6502NZ[ebx]   ; Set N and Z
m_adc_z: mov  cl,bl			; get result back to accumulator
    endm

M_ROL macro
      mov  bh,ch            ; flags
      and  ch,255-(F_CARRY + F_NEGATIVE + F_ZERO)
      shr  bh,1             ; get carry flag
      adc  bl,bl            ; ROL
      adc  ch,0             ; get carry flag
	  xor  bh,bh
      or   ch,m6502NZ[ebx]  ; get N and Z
      endm

M_ROR macro
      mov  bh,ch            ; get flags
      and  ch,255-(F_CARRY + F_ZERO + F_NEGATIVE)
      shr  bh,1             ; get 6502's CARRY
      rcr  bl,1             ; ROR
      adc  ch,0             ; set carry flag
	  xor  bh,bh
	  or   ch,m6502NZ[ebx]  ; set N and Z
      endm


M_CMP macro
      xor  ah,ah
      and  ch,255-(F_ZERO + F_NEGATIVE + F_CARRY)
      mov  al,cl
      sub  al,bl        ; do subtraction of accumulator and operand
      cmc
      adc  ch,0         ; set carry flag - WORKS BACKWARDS!!
      or   ch,m6502NZ[eax]
      endm

M_CPX macro
      xor  ah,ah
      and  ch,255-(F_ZERO + F_NEGATIVE + F_CARRY)
      mov  al,reg_x
      sub  al,bl           ; do subtraction of accumulator and operand
	  cmc
      adc  ch,0            ; set carry flag - WORKS BACKWARDS!!
      or   ch,m6502NZ[eax]
      endm

M_CPY macro
      xor  ah,ah
      and  ch,255-(F_ZERO + F_NEGATIVE + F_CARRY)
      mov  al,reg_y
      sub  al,bl            ; do subtraction of accumulator and operand
	  cmc
      adc  ch,0             ; set carry flag - WORKS BACKWARDS!!
      or   ch,m6502NZ[eax]
      endm

M_ASL macro
      and  ch,255-(F_CARRY + F_NEGATIVE + F_ZERO)
	  add  bl,bl
      adc  ch,0			; set carry flag
      or   ch,m6502NZ[ebx]
      endm

M_LSR macro
      and  ch,255-(F_CARRY + F_NEGATIVE + F_ZERO)
      shr  bl,1
	  adc  ch,0			; set carry flag
      or   ch,m6502NZ[ebx]
      endm

M_LSRA macro
      and  ch,255-(F_CARRY + F_NEGATIVE + F_ZERO)
      shr  cl,1
      adc  ch,0         ; set carry flag
	  mov  bl,cl
      or   ch,m6502NZ[ebx]
      endm

M_ASLA macro
      and  ch,255-(F_CARRY + F_NEGATIVE + F_ZERO)
      add  cl,cl
      adc  ch,0         ; set carry flag
	  mov  bl,cl
      or   ch,m6502NZ[ebx]
      endm

M_ROLA macro
      mov  bh,ch            ; flags
      and  ch,255-(F_CARRY + F_NEGATIVE + F_ZERO)
      shr  bh,1             ; get carry flag
      adc  cl,cl            ; ROL
      adc  ch,0             ; get carry flag
	  xor  bh,bh
	  mov  bl,cl
      or   ch,m6502NZ[ebx]  ; get N and Z
      endm

M_RORA macro
      mov  bh,ch            ; get flags
      and  ch,255-(F_CARRY + F_ZERO + F_NEGATIVE)
      shr  bh,1             ; get 6502's CARRY
      rcr  cl,1             ; ROR
      adc  ch,0             ; set carry flag
	  xor  bh,bh
	  mov  bl,cl
	  or   ch,m6502NZ[ebx]  ; set N and Z
      endm
;
; Read a byte from one of the hardware handlers
; EBX contains read handler #
;
read_slow proc near
        push  edx    ; save PC
        push  ecx    ; save accumulator and flags
        mov   esi,pemuh
        push  eax    ; address to read from
        call  DWORD PTR [esi+ebx*8-16]
        xor   ebx,ebx   ; in case it got trashed
        mov   esi,mem_map   ; restore memory map pointer
        mov   bl,al
        mov   edi,clocks        ; restore clock tick pointer
        pop   eax       ; restore address and correct stack
        pop   ecx       ; restore accumulator and flags
		pop   edx       ; restore PC
        ret
read_slow endp
;
; Write a byte to one of the hardware handlers
;
write_slow proc near
        push  edx				; save PC
        push  ecx               ; save accumulator and flags
        push  ebx               ; push byte to write
        xor   ecx,ecx
        mov   cl,[esi+eax+MEM_FLAGS]
        mov   esi,pemuh         ; point to emulation handler structure
        push  eax               ; push address
        call  DWORD PTR [esi+ecx*8-12]
        xor   ebx,ebx           ; in case EBX got clobbered
        add   esp,8
        mov   esi,mem_map       ; restore memory map pointer
        pop   ecx               ; restore accumulator and flags
		pop   edx				; restore PC
        mov   edi,clocks        ; in case EDI got clobbered
        ret
write_slow endp
;
; Call from C as RESET6502(char *mem, REGS6502 *regs)
;
_areset6502  proc  near
          push      ebx
          push      esi
          push      edi
          enter     0,0

          mov       esi,[ebp+20]        ; get memory map pointer
          mov       edi,[ebp+24]        ; get registers structure

          xor       eax,eax
          mov       [edi].sreg_a,al
          mov       [edi].sreg_x,al
          mov       [edi].sreg_y,al
          mov       [edi].sreg_s,al
          mov       [edi].sreg_p,F_ZERO + F_UNUSED
          mov       al,[esi+MEM_ROM+0fffch]  ; Get reset vector
          mov       ah,[esi+MEM_ROM+0fffdh]
          mov       [edi].sreg_pc,ax

          leave
          pop       edi
          pop       esi
          pop       ebx
          ret
_areset6502  endp

;
; Execute n clock cycles of 6502 instructions
; call from C as:
; void EXEC6502(char *mem, REGS6502 *regs, EMUHANDLERS *emuh, int *iClocks, unsigned char ucIRQs)
;
_aexec6502 proc      near
          push      ebx
          push      esi
          push      edi
          enter     0,0

          mov       esi,[ebp+20] ; get memory map pointer
          mov       mem_map,esi
          mov       edi,[ebp+24] ; register structure
          mov       pregs,edi
          mov       eax,[ebp+28] ; emulation handler pointers
          mov       pemuh,eax
          mov       eax,[ebp+32] ; pointer to total clock cycles to execute
          mov       clocks,eax
          mov       eax,[ebp+36] ; pending interrupt flags
          mov       pending,eax
          mov       al,[eax]     ; get the flag into a global for fast access
          mov       intflag,al
          mov       al,[edi].sreg_x    ; copy register values to my structure
          mov       reg_x,al
          mov       al,[edi].sreg_y
          mov       reg_y,al
          mov       al,[edi].sreg_s
          mov       reg_s,al
          movzx     edx,word ptr [edi].sreg_pc           ; ugly, but gets PC in a register
          mov       cl,[edi].sreg_a    ; accumulator keep in CL
          mov       ch,[edi].sreg_p    ; flags keep in CH
          mov       edi,clocks
          xor       ebx,ebx     ; keep EBX clear, use only BL
;
; Top of execution loop
;
; EAX used for effective address
; EBX used for temp byte values
; EDX used for program counter
; ESI points to memory map
; EDI points to clock count variable
; CL = Accumulator, CH = flags
;
top6502:  test      intflag,-1  ; any interrupts pending?
          jnz       int6502     ; yes, try to handle
; DEBUG - we are assuming code is executing in ROM - change in future
top2:     mov       eax,[esi+edx+MEM_ROM]       ; read an instruction and arguments
          mov       bl,al       ; get opcode in EBX for jump table
          mov       bl,m6502Cycles[ebx]
          sub       [edi],ebx   ; subtract instruction length from total
          js        bot6502     ; if < 0, we are done
          mov       bl,al       ; get opcode again
          shr       eax,8       ; prepare operand for use
          inc       edx         ; point PC to next instruction/arg
          and       eax,0ffffh  ; prepare for use as a 16-bit address
          jmp       optable[ebx*4]      ; execute the instruction
;
; Interrupt handling
;
int6502:  test      intflag,INT_NMI     ; NMI pending?
          jz        @F
          M_PUSHPC                      ; push PC and flags
          M_PUSHP
          and       ch,255-F_DECIMAL    ; clear the decimal flag
          sub       dword ptr [edi],7   ; subtract time to execute
          mov       dl,[esi+MEM_ROM+0fffah] ; must be in ROM
          and       intflag,255-INT_NMI ; clear pending interrupt flag
          mov       dh,[esi+MEM_ROM+0fffbh] ; PC = [FFFA]  interrupt vector
          jmp       top2                ; continue execution
; Check for IRQ
@@:       test      intflag,INT_IRQ     ; IRQ pending?
          jz        top2                ; continue with normal execution
          test      ch,F_IRQMASK        ; interrupts are masked?
          jnz       top2                ; yes, can't execute yet
          M_PUSHPC                      ; push PC on the stack
          M_PUSHP                       ; push the flags
          and       intflag,255-INT_IRQ ; clear pending interrupt flag
          and       ch,255-F_DECIMAL    ; clear the decimal flag
          sub       dword ptr [edi],7   ; subtract time to execute
          mov       dl,[esi+MEM_ROM+0fffeh] ; must be in ROM
          or        ch,F_IRQMASK        ; mask further interrupts
          mov       dh,[esi+MEM_ROM+0ffffh] ; PC = [FFFE]  interrupt vector
          jmp       top2                ; continue execution


;
; Opcode implementations follow in opcode order
;
; 00 - BRK
op_brk:   or    ch,F_BREAK        ; set the break flag
          M_PUSHPC                ; push the PC
          M_PUSHP                 ; push the flags
          mov   dl,[esi+MEM_ROM+0fffeh] ; must be in ROM
          or    ch,F_IRQMASK        ; mask further interrupts
          mov   dh,[esi+MEM_ROM+0ffffh] ; PC = [FFFE]  interrupt vector
          and   ch,255-F_DECIMAL  ; clear decimal mode
          jmp   top6502           ; do next instruction

; 01 - OR (z,x)
op_or_zxind: M_EA_ZXIND          ; get the EA
          M_RDB
          M_OR                    ; OR with A
          jmp   top6502
; 05 - OR z
op_or_z: M_EA_Z
          M_RDBZ
          M_OR
          jmp   top6502
; 06 - ASL z
op_asl_z: M_EA_Z
          M_RDBZ
          M_ASL
          M_WRBZ
          jmp   top6502
; 08 - PHP
op_php:   M_PUSHP
          jmp   top6502
; 09 - ORA immediate
op_or_i:  M_EA_I
          M_OR
          jmp   top6502
; 0A - ASLA
op_asla:  M_ASLA
          jmp   top6502
; 0D - ORA abs
op_or_a:  M_EA_A
          M_RDB
          M_OR
          jmp   top6502
; 0E - ASL abs
op_asl_a: M_EA_A
          M_RDB
          M_ASL
          M_WRB
          jmp   top6502
; 10 - BPL
op_bpl:   M_EA_REL
          test  ch,F_NEGATIVE
          jnz   top6502
          add   edx,eax
          dec   dword ptr [edi]  ; minus 1 clock tick
		  and   edx,0ffffh		; keep within 16 bits
          jmp   top6502
; 11 - OR (z),y
op_or_zyind: M_EA_ZYIND
          M_RDB
          M_OR
          jmp   top6502
; 15 - OR z,x
op_or_zx: M_EA_ZX
          M_RDBZ
          M_OR
          jmp   top6502
; 16 - ASL z,x
op_asl_zx: M_EA_ZX
          M_RDBZ
          M_ASL
          M_WRBZ
          jmp   top6502
; 18 - CLC
op_clc:   and   ch,255-F_CARRY
          jmp   top6502
; 19 - ORA abs,y
op_or_ay:  M_EA_AY
           M_RDB
           M_OR
           jmp  top6502
; 1D - ORA abs,x
op_or_ax:  M_EA_AX
           M_RDB
           M_OR
           jmp  top6502
; 1E - ASL abs,x
op_asl_ax: M_EA_AX
           M_RDB
           M_ASL
           M_WRB
           jmp  top6502
; 20 - JSR abs
op_jsr_a:  inc  edx     ; push PC + 1
           M_PUSHPC
           mov  edx,eax  ; jmp to EA
           jmp  top6502
; 21 - AND (z,x)
op_and_zxind: M_EA_ZXIND
           M_RDB
           M_AND
           jmp  top6502
; 24 - BIT z
op_bit_z:  M_EA_Z
           M_RDBZ
           M_BIT
           jmp  top6502
; 25 - AND z
op_and_z:  M_EA_Z
           M_RDBZ
           M_AND
           jmp  top6502
; 26 - ROL z
op_rol_z:  M_EA_Z
           M_RDBZ
           M_ROL
           M_WRBZ
           jmp  top6502
; 28 - PLP
op_plp:    M_PULLP
           jmp  top6502
; 29 - AND - immediate
op_and_i:  M_EA_I
           M_AND
           jmp  top6502
; 2A - ROLA
op_rola:   M_ROLA
           jmp  top6502
; 2C - BIT abs
op_bit_a:  M_EA_A
           M_RDB
           M_BIT
           jmp  top6502
; 2D - AND abs
op_and_a:  M_EA_A
           M_RDB
           M_AND
           jmp  top6502
; 2E - ROL abs
op_rol_a:  M_EA_A
           M_RDB
           M_ROL
           M_WRB
           jmp  top6502
; 30 - BMI
op_bmi:   M_EA_REL
          test  ch,F_NEGATIVE
          jz    top6502
          add   edx,eax
          dec   dword ptr [edi]  ; minus 1 clock tick
		  and   edx,0ffffh
          jmp   top6502
; 31 - AND (z),y
op_and_zyind: M_EA_ZYIND
           M_RDB
           M_AND
           jmp  top6502
; 35 - AND z,x
op_and_zx: M_EA_ZX
           M_RDBZ
           M_AND
           jmp  top6502
; 36 - ROL z,x
op_rol_zx: M_EA_ZX
           M_RDBZ
           M_ROL
           M_WRBZ
           jmp  top6502
; 38 - SEC
op_sec:    or   ch,F_CARRY
           jmp  top6502
; 39 - AND abs,y
op_and_ay: M_EA_AY
           M_RDB
           M_AND
           jmp  top6502
; 3D - AND abs,x
op_and_ax: M_EA_AX
           M_RDB
           M_AND
           jmp  top6502
; 3E - ROL abs,x
op_rol_ax: M_EA_AX
           M_RDB
           M_ROL
           M_WRB
           jmp  top6502
; 40 - RTI
op_rti:    M_PULLP
           or   ch,F_UNUSED
           M_PULLPC
           jmp  top6502
; 41 - EOR (z,x)
op_eor_zxind: M_EA_ZXIND
           M_RDB
           M_EOR
           jmp  top6502
; 45 - EOR z
op_eor_z:  M_EA_Z
           M_RDBZ
           M_EOR
           jmp  top6502
; 46 - LSR z
op_lsr_z:  M_EA_Z
           M_RDBZ
           M_LSR
           M_WRBZ
           jmp  top6502
; 48 - PHA
op_pha:    M_PUSHA
           jmp  top6502
; 49 - EOR immediate
op_eor_i:  M_EA_I
           M_EOR
           jmp  top6502
; 4A - LSRA
op_lsr:    M_LSRA
           jmp  top6502
; 4C - JMP abs
op_jmp_a:  mov  edx,eax
           jmp  top6502
; 4D - EOR abs
op_eor_a:  M_EA_A
           M_RDB
           M_EOR
           jmp  top6502
; 4E - LSR abs
op_lsr_a:  M_EA_A
           M_RDB
           M_LSR
           M_WRB
           jmp  top6502
; 50 - BVC
op_bvc:    M_EA_REL
           test ch,F_OVERFLOW
           jnz  top6502
           add  edx,eax
           dec  dword ptr [edi]  ; minus 1 clock tick
		   and  edx,0ffffh		; don't let negative values hose it
           jmp  top6502
; 51 - EOR (z),y
op_eor_zyind: M_EA_ZYIND
           M_RDB
           M_EOR
           jmp  top6502
; 55 - EOR z,x
op_eor_zx: M_EA_ZX
           M_RDBZ
           M_EOR
           jmp  top6502
; 56 - LSR z,x
op_lsr_zx: M_EA_ZX
           M_RDBZ
           M_LSR
           M_WRBZ
           jmp  top6502
; 58 - CLI
op_cli:    and  ch,255-F_IRQMASK
           jmp  top6502
; 59 - EOR abs,y
op_eor_ay: M_EA_AY
           M_RDB
           M_EOR
           jmp  top6502
; 5D - EOR abs,x
op_eor_ax: M_EA_AX
           M_RDB
           M_EOR
           jmp  top6502
; 5E - LSR abs,x
op_lsr_ax: M_EA_AX
           M_RDB
           M_LSR
           M_WRB
           jmp  top6502
; 60 - RTS
op_rts:    M_PULLPC
           inc  edx         ; PC++
           jmp  top6502
; 61 - ADC (z,x)
op_adc_zxind: M_EA_ZXIND
           M_RDB
           M_ADC
           jmp  top6502
; 65 - ADC z
op_adc_z:  M_EA_Z
           M_RDBZ
           M_ADC
           jmp  top6502
; 66 - ROR z
op_ror_z:  M_EA_Z
           M_RDBZ
           M_ROR
           M_WRBZ
           jmp  top6502
; 68 - PLA
op_pla:    M_PULLA
           jmp  top6502
; 69 - ADC immediate
op_adc_i:  M_EA_I
           M_ADC
           jmp  top6502
; 6A - RORA
op_ror:    M_RORA
           jmp  top6502
; 6C - JMP (ind)
op_jmp_ind: mov dl,[esi+eax+MEM_ROM] ; safe to assume executing in ROM
           mov  dh,[esi+eax+MEM_ROM+1] ; DEBUG should check for code in RAM
           jmp  top6502
; 6D - ADC abs
op_adc_a:  M_EA_A
           M_RDB
           M_ADC
           jmp  top6502
; 6E - ROR abs
op_ror_a:  M_EA_A
           M_RDB
           M_ROR
           M_WRB
           jmp  top6502
; 70 - BVS
op_bvs:    M_EA_REL
           test ch,F_OVERFLOW
           jz   top6502
           add  edx,eax
           dec  dword ptr [edi]  ; minus 1 clock tick
		   and  edx,0ffffh		; don't let negative values hose it
           jmp  top6502
; 71 - ADC (z),y
op_adc_zyind: M_EA_ZYIND
           M_RDB
           M_ADC
           jmp  top6502
; 75 - ADC z,x
op_adc_zx: M_EA_ZX
           M_RDBZ
           M_ADC
           jmp  top6502
; 76 - ROR z,x
op_ror_zx: M_EA_ZX
           M_RDBZ
           M_ROR
           M_WRBZ
           jmp  top6502
; 78 - SEI
op_sei:    or   ch,F_IRQMASK
           jmp  top6502
; 79 - ADC abs,y
op_adc_ay: M_EA_AY
           M_RDB
           M_ADC
           jmp  top6502
; 7D - ADC abs,x
op_adc_ax: M_EA_AX
           M_RDB
           M_ADC
           jmp  top6502
; 7E - ROR abs,x
op_ror_ax: M_EA_AX
           M_RDB
           M_ROR
           M_WRB
           jmp  top6502
; 81 - STA (z,x)
op_sta_zxind: M_EA_ZXIND
		   mov  bl,cl
           M_WRB
           jmp  top6502
; 84 - STY z
op_sty_z:  M_EA_Z
           mov  bl,reg_y
           M_WRBZ
           jmp  top6502
; 85 - STA z
op_sta_z:  mov  bl,cl
           M_EA_Z
           M_WRBZ
           jmp  top6502
; 86 - STX z
op_stx_z:  M_EA_Z
           mov  bl,reg_x
           M_WRBZ
           jmp  top6502
; 88 - DEY
op_dey:    dec  reg_y
		   mov  bl,reg_y
           M_FLAGSNZ
           jmp  top6502
; 8A - TXA
op_txa:    mov  cl,reg_x
		   mov  bl,cl
           M_FLAGSNZ
           jmp  top6502
; 8C - STY abs
op_sty_a:  M_EA_A
           mov  bl,reg_y
           M_WRB
           jmp  top6502
; 8D - STA abs
op_sta_a:  mov  bl,cl
           M_EA_A
           M_WRB
           jmp  top6502
; 8E - STX abs
op_stx_a:  M_EA_A
           mov  bl,reg_x
           M_WRB
           jmp  top6502
; 90 - BCC
op_bcc:    M_EA_REL
           test ch,F_CARRY
           jnz  top6502
           add  edx,eax
           dec  dword ptr [edi]  ; minus 1 clock tick
		   and  edx,0ffffh		; don't let negative values hose it
           jmp  top6502
; 91 - STA (z),y
op_sta_zyind: M_EA_ZYIND
		   mov  bl,cl
           M_WRB
           jmp  top6502
; 94 - STY z,x
op_sty_zx: M_EA_ZX
           mov  bl,reg_y
           M_WRBZ
           jmp  top6502
; 95 - STA z,x
op_sta_zx: M_EA_ZX
           mov  bl,cl
           M_WRBZ
           jmp  top6502
; 96 - STX z,y
op_stx_zy: M_EA_ZY
           mov  bl,reg_x
           M_WRBZ
           jmp  top6502
; 98 - TYA
op_tya:    mov  bl,reg_y
           M_FLAGSNZ
           mov  cl,bl
           jmp  top6502
; 99 - STA abs,y
op_sta_ay: M_EA_AY
           mov  bl,cl
           M_WRB
           jmp  top6502
; 9A - TXS
op_txs:    mov  al,reg_x
           mov  reg_s,al
           jmp  top6502
; 9D - STA abs,x
op_sta_ax: M_EA_AX
           mov  bl,cl
           M_WRB
           jmp  top6502
; A0 - LDY immediate
op_ldy_i:  M_EA_I
           mov  reg_y,al
           M_FLAGSNZ
           jmp  top6502
; A1 - LDA (z,x)
op_lda_zxind: M_EA_ZXIND
           M_RDB
           mov  cl,bl
           M_FLAGSNZ
           jmp  top6502
; A2 - LDX immediate
op_ldx_i:  M_EA_I
           mov  reg_x,al
           M_FLAGSNZ
           jmp  top6502
; A4 - LDY z
op_ldy_z:  M_EA_Z
           M_RDBZ
           mov  reg_y,bl
           M_FLAGSNZ
           jmp  top6502
; A5 - LDA z
op_lda_z:  M_EA_Z
           M_RDBZ
           mov  cl,bl
           M_FLAGSNZ
           jmp  top6502
; A6 - LDX z
op_ldx_z:  M_EA_Z
           M_RDBZ
           mov  reg_x,bl
           M_FLAGSNZ
           jmp  top6502
; A8 - TAY
op_tay:    mov  reg_y,cl
           mov  bl,cl
           M_FLAGSNZ
           jmp  top6502
; A9 - LDA immediate
op_lda_i:  M_EA_I
           mov  cl,bl
           M_FLAGSNZ
           jmp  top6502
; AA - TAX
op_tax:    mov  reg_x,cl
           mov  bl,cl
           M_FLAGSNZ
           jmp  top6502
; AC - LDY abs
op_ldy_a:  M_EA_A
           M_RDB
           mov  reg_y,bl
           M_FLAGSNZ
           jmp  top6502
; AD - LDA abs
op_lda_a:  M_EA_A
           M_RDB
           mov  cl,bl
           M_FLAGSNZ
           jmp  top6502
; AE - LDX abs
op_ldx_a:  M_EA_A
           M_RDB
           mov  reg_x,bl
           M_FLAGSNZ
           jmp  top6502
; B0 - BCS
op_bcs:    M_EA_REL
           test  ch,F_CARRY
           jz   top6502
           add  edx,eax
           dec  dword ptr [edi]  ; minus 1 clock tick
		   and  edx,0ffffh		; don't let negative values hose it
           jmp  top6502
; B1 - LDA (z),y
op_lda_zyind: M_EA_ZYIND
           M_RDB
           mov  cl,bl
           M_FLAGSNZ
           jmp  top6502
; B4 - LDY z,x
op_ldy_zx: M_EA_ZX
           M_RDBZ
           mov  reg_y,bl
           M_FLAGSNZ
           jmp  top6502
; B5 - LDA z,x
op_lda_zx: M_EA_ZX
           M_RDBZ
           mov  cl,bl
           M_FLAGSNZ
           jmp  top6502
; B6 - LDX z,y
op_ldx_zy: M_EA_ZY
           M_RDBZ
           mov  reg_x,bl
           M_FLAGSNZ
           jmp  top6502
; B8 - CLV
op_clv:    and  ch,255-F_OVERFLOW
           jmp  top6502
; B9 - LDA abs,y
op_lda_ay: M_EA_AY
           M_RDB
           mov  cl,bl
           M_FLAGSNZ
           jmp  top6502
; BA - TSX
op_tsx:    mov  bl,reg_s
           M_FLAGSNZ
           mov  reg_x,bl
           jmp  top6502
; BC - LDY abs,x
op_ldy_ax: M_EA_AX
           M_RDB
           mov  reg_y,bl
           M_FLAGSNZ
           jmp  top6502
; BD - LDA abs,x
op_lda_ax: M_EA_AX
           M_RDB
           mov  cl,bl
           M_FLAGSNZ
           jmp  top6502
; BE - LDX abs,y
op_ldx_ay: M_EA_AY
           M_RDB
           M_FLAGSNZ
           mov  reg_x,bl
           jmp  top6502
; C0 - CPY immediate
op_cpy_i:  M_EA_I
           M_CPY
           jmp  top6502
; C1 - CMP (z,x)
op_cmp_zxind: M_EA_ZXIND
           M_RDB
           M_CMP
           jmp  top6502
; C4 - CPY z
op_cpy_z:  M_EA_Z
           M_RDBZ
           M_CPY
           jmp  top6502
; C5 - CMP z
op_cmp_z:  M_EA_Z
           M_RDBZ
           M_CMP
           jmp  top6502
; C6 - DEC z
op_dec_z:  M_EA_Z
           M_RDBZ
           M_DEC
           M_WRBZ
           jmp  top6502
; C8 - INY
op_iny:    inc  reg_y
           mov  bl,reg_y
           M_FLAGSNZ
           jmp  top6502
; C9 - CMP immediate
op_cmp_i:  M_EA_I
           M_CMP
           jmp  top6502
; CA - DEX
op_dex:    dec   reg_x
           mov  bl,reg_x
		   M_FLAGSNZ
           jmp  top6502
; CC - CPY abs
op_cpy_a:  M_EA_A
           M_RDB
           M_CPY
           jmp  top6502
; CD - CMP abs
op_cmp_a:  M_EA_A
           M_RDB
           M_CMP
           jmp  top6502
; CE - DEC abs
op_dec_a:  M_EA_A
           M_RDB
           M_DEC
           M_WRB
           jmp  top6502
; D0 - BNE
op_bne:    M_EA_REL
           test ch,F_ZERO
           jnz  top6502
           add  edx,eax
           dec  dword ptr [edi]  ; minus 1 clock tick
		   and  edx,0ffffh		; don't let negative values hose it
           jmp  top6502
; D1 - CMP (z),y
op_cmp_zyind: M_EA_ZYIND
           M_RDB
           M_CMP
           jmp  top6502
; D5 - CMP z,x
op_cmp_zx: M_EA_ZX
           M_RDBZ
           M_CMP
           jmp  top6502
; D6 - DEC z,x
op_dec_zx: M_EA_ZX
           M_RDBZ
           M_DEC
           M_WRBZ
           jmp  top6502
; D8 - CLD
op_cld:    and  ch,255-F_DECIMAL
           jmp  top6502
; D9 - CMP abs,y
op_cmp_ay: M_EA_AY
           M_RDB
           M_CMP
           jmp  top6502
; DD - CMP abs,x
op_cmp_ax: M_EA_AX
           M_RDB
           M_CMP
           jmp  top6502
; DE - DEC abs,x
op_dec_ax: M_EA_AX
           M_RDB
           M_DEC
           M_WRB
           jmp  top6502
; E0 - CPX immediate
op_cpx_i:  M_EA_I
           M_CPX
           jmp  top6502
; E1 - SBC (z,x)
op_sbc_zxind: M_EA_ZXIND
           M_RDB
           M_SBC
           jmp  top6502
; E4 - CPX z
op_cpx_z:  M_EA_Z
           M_RDBZ
           M_CPX
           jmp  top6502
; E5 - SBC z
op_sbc_z:  M_EA_Z
           M_RDBZ
           M_SBC
           jmp  top6502
; E6 - INC z
op_inc_z:  M_EA_Z
           M_RDBZ
           M_INC
           M_WRBZ
           jmp  top6502
; E8 - INX
op_inx:    inc  reg_x
           mov  bl,reg_x
           M_FLAGSNZ
           jmp  top6502
; E9 - SBC immediate
op_sbc_i:  M_EA_I
           M_SBC
           jmp  top6502
; EA - NOP
op_nop:    jmp  top6502
; EC - CPX abs
op_cpx_a:  M_EA_A
           M_RDB
           M_CPX
           jmp  top6502
; ED - SBC abs
op_sbc_a:  M_EA_A
           M_RDB
           M_SBC
           jmp  top6502
; EE - INC abs
op_inc_a:  M_EA_A
           M_RDB
           M_INC
           M_WRB
           jmp  top6502
; F0 - BEQ
op_beq:    M_EA_REL
           test ch,F_ZERO
           jz   top6502
           add  edx,eax
           dec  dword ptr [edi]  ; minus 1 clock tick
		   and  edx,0ffffh		; don't let negative values hose it
           jmp  top6502
; F1 - SBC (z),y
op_sbc_zyind: M_EA_ZYIND
           M_RDB
           M_SBC
           jmp  top6502
; F5 - SBC z,x
op_sbc_zx: M_EA_ZX
           M_RDBZ
           M_SBC
           jmp  top6502
; F6 - INC z,x
op_inc_zx: M_EA_ZX
           M_RDBZ
           M_INC
           M_WRBZ
           jmp  top6502
; F8 - SED
op_sed:    or   ch,F_DECIMAL
           jmp  top6502
; F9 - SBC abs,y
op_sbc_ay: M_EA_AY
           M_RDB
           M_SBC
           jmp  top6502
; FD - SBC abs,x
op_sbc_ax: M_EA_AX
           M_RDB
           M_SBC
           jmp  top6502
; FE - INC abs,x
op_inc_ax: M_EA_AX
           M_RDB
           M_INC
           M_WRB
           jmp  top6502
op_illegal: mov [edi],0         ; stop execution and return
           jmp  top6502
bot6502:  leave
; copy the register values back to the calling program
          mov       edi,pending
          mov       al,intflag
          mov       [edi],al    ; store the updated interrupt flags
          mov       esi,pregs   ; put register values back in 'C' structures
          mov       [esi].sreg_a,cl
          mov       [esi].sreg_p,ch
          mov       [esi].sreg_pc,dx
          mov       al,reg_x
          mov       [esi].sreg_x,al
          mov       al,reg_y
          mov       [esi].sreg_y,al
          mov       al,reg_s
          mov       [esi].sreg_s,al
          pop       edi
          pop       esi
          pop       ebx
          ret
_aexec6502 endp
_TEXT     ends
          end
