; $100-$106		basic hooks
; $1000-$2000	code
; $2000-$20ff	direct page
; $2100-$2eff	data
; $2f00-$2fff	stack
; $feee-$fefc	ROM interrupt redirects

		org    $1000
START	equ    *

		orcc	#$50

;; init 6809 vectors

		ldx		#$fefa
		stx		$fffa		swi

		ldx		#$fef1
		stx		$fff4		swi2

		ldx		#$feee
		stx		$fff2		swi3

		lda		#$16		
		sta		$fefa
		sta		$fef1
		sta		$fefe
		ldx		#$0209		fefa->	LBRA	$106
		stx		$fefb		
		ldx		#$020f		fef1->	LBRA	$103
		stx		$fef2
		stx		$feef		fefe->	LBRA	$100

		lda		#$3b		rti
		sta		$100
		sta		$103
		sta		$106

;; setup registers for test

		lda		#DIRECT/256
		tfr		a,dp
;;		setdp	DIRECT/256
		setdp	$20

		ldu		#SCRATCH+96
		tfr		u,x
		tfr		u,y

		sts		save_s
		lds		#STACK
			
*------------------------------------------------------------------------------
* 6809 instruction tests
*------------------------------------------------------------------------------
                   ABX
*
                   ADCA   #IMM
                   ADCA   <DIR
                   ADCA   EXT
                   ADCA   [IND]
                   ADCA   ,X
                   ADCA   ,Y++
                   ADCA   [,--U]
                   ADCB   #IMM
                   ADCB   <DIR
                   ADCB   EXT
                   ADCB   [IND]
                   ADCB   ,X
                   ADCB   ,Y++
                   ADCB   [,--U]
*
                   ADDA   #IMM
                   ADDA   <DIR
                   ADDA   EXT
                   ADDA   [IND]
                   ADDA   ,X
                   ADDA   ,Y++
                   ADDA   [,--U]
                   ADDB   #IMM
                   ADDB   <DIR
                   ADDB   EXT
                   ADDB   [IND]
                   ADDB   ,X
                   ADDB   ,Y++
                   ADDB   [,--U]
                   ADDD   #IMM
                   ADDD   <DIR
                   ADDD   EXT
                   ADDD   [IND]
                   ADDD   ,X
                   ADDD   ,Y++
                   ADDD   [,--U]
*
                   ANDA   #IMM
                   ANDA   <DIR
                   ANDA   EXT
                   ANDA   [IND]
                   ANDA   ,X
                   ANDA   ,Y++
                   ANDA   [,--U]
                   ANDB   #IMM
                   ANDB   <DIR
                   ANDB   EXT
                   ANDB   [IND]
                   ANDB   ,X
                   ANDB   ,Y++
                   ANDB   [,--U]
                   ANDCC  #$FF
*
                   ASLA
                   ASLB
                   ASL    <DIR
                   ASL    EXT
                   ASL    [IND]
                   ASL    ,X
                   ASL    ,Y++
                   ASL    [,--U]
*
                   ASRA
                   ASRB
                   ASR    <DIR
                   ASR    EXT
                   ASR    [IND]
                   ASR    ,X
                   ASR    ,Y++
                   ASR    [,--U]
*
                   BCC    *+2
                   BCS    *+2
                   BEQ    *+2
                   BGE    *+2
                   BGT    *+2
                   BHI    *+2
                   BHS    *+2
                   BLE    *+2
                   BLO    *+2
                   BLS    *+2
                   BLT    *+2
                   BMI    *+2
                   BNE    *+2
                   BPL    *+2
                   BRA    *+2
                   BRN    *+2
                   BVC    *+2
                   BVS    *+2
                   BSR    a@
				   BRA    b@
a@				   RTS
b@

                   BITA   #IMM
                   BITA   <DIR
                   BITA   EXT
                   BITA   [IND]
                   BITA   ,X
                   BITA   ,Y++
                   BITA   [,--U]
                   BITB   #IMM
                   BITB   <DIR
                   BITB   EXT
                   BITB   [IND]
                   BITB   ,X
                   BITB   ,Y++
                   BITB   [,--U]
*
                   CLRA
                   CLRB
                   CLR    <DIR
                   CLR    EXT
                   CLR    [IND]
                   CLR    ,X
                   CLR    ,Y++
                   CLR    [,--U]
*
                   CMPA   #IMM
                   CMPA   <DIR
                   CMPA   EXT
                   CMPA   [IND]
                   CMPA   ,X
                   CMPA   ,Y++
                   CMPA   [,--U]
                   CMPB   #IMM
                   CMPB   <DIR
                   CMPB   EXT
                   CMPB   [IND]
                   CMPB   ,X
                   CMPB   ,Y++
                   CMPB   [,--U]
                   CMPD   #IMM
                   CMPD   <DIR
                   CMPD   EXT
                   CMPD   [IND]
                   CMPD   ,X
                   CMPD   ,Y++
                   CMPD   [,--U]
                   CMPX   #IMM
                   CMPX   <DIR
                   CMPX   EXT
                   CMPX   [IND]
                   CMPX   ,X
                   CMPX   ,Y++
                   CMPX   [,--U]
                   CMPY   #IMM
                   CMPY   <DIR
                   CMPY   EXT
                   CMPY   [IND]
                   CMPY   ,X
                   CMPY   ,Y++
                   CMPY   [,--U]
                   CMPU   #IMM
                   CMPU   <DIR
                   CMPU   EXT
                   CMPU   [IND]
                   CMPU   ,X
                   CMPU   ,Y++
                   CMPU   [,--U]
*
                   COMA
                   COMB
                   COM    <DIR
                   COM    EXT
                   COM    [IND]
                   COM    ,X
                   COM    ,Y++
                   COM    [,--U]
*
;;                   CWAI   #IMM
                   DAA
*
                   DECA
                   DECB
                   DEC    <DIR
                   DEC    EXT
                   DEC    [IND]
                   DEC    ,X
                   DEC    ,Y++
                   DEC    [,--U]
*
                   EORA   #IMM
                   EORA   <DIR
                   EORA   EXT
                   EORA   [IND]
                   EORA   ,X
                   EORA   ,Y++
                   EORA   [,--U]
                   EORB   #IMM
                   EORB   <DIR
                   EORB   EXT
                   EORB   [IND]
                   EORB   ,X
                   EORB   ,Y++
                   EORB   [,--U]
*
                   EXG    A,B
                   EXG    X,Y
*
                   INCA
                   INCB
                   INC    <DIR
                   INC    EXT
                   INC    [IND]
                   INC    ,X
                   INC    ,Y++
                   INC    [,--U]
*
;;                   JMP    <DIR
;;                   JMP    EXT
;;                   JMP    [IND]
;;                   JMP    ,X
;;                   JMP    ,X++
;;                   JMP    [,--Y]
;;                   JSR    <DIR
;;                   JSR    EXT
;;                   JSR    [IND]
;;                   JSR    ,X
;;                   JSR    ,X++
;;                   JSR    [,--Y]
*
                   LBCC   *+4
                   LBCS   *+4
                   LBEQ   *+4
                   LBGE   *+4
                   LBGT   *+4
                   LBHI   *+4
                   LBHS   *+4
                   LBLE   *+4
                   LBLO   *+4
                   LBLS   *+4
                   LBLT   *+4
                   LBMI   *+4
                   LBNE   *+4
                   LBPL   *+4
                   LBRA   *+3
                   LBRN   *+4
                   LBVC   *+4
                   LBVS   *+4
                   LBSR   a@
				   bra    b@
a@				   rts
b@
*
                   LDA    #IMM
                   LDA    <DIR
                   LDA    EXT
                   LDA    [IND]
                   LDA    ,X
                   LDA    ,Y++
                   LDA    [,--U]
                   LDB    #IMM
                   LDB    <DIR
                   LDB    EXT
                   LDB    [IND]
                   LDB    ,X
                   LDB    ,Y++
                   LDB    [,--U]
                   LDD    #IMM
                   LDD    <DIR
                   LDD    EXT
                   LDD    [IND]
                   LDD    ,X
                   LDD    ,Y++
                   LDD    [,--U]
*
		pshs x,y,u
		sts save
                   LEAX   5,X
                   LEAX   ,Y++
                   LEAX   ,--U
                   LEAX   [,S++]
                   LEAY   5,X
                   LEAY   ,Y++
                   LEAY   ,--U
                   LEAY   [,S++]
                   LEAU   5,X
                   LEAU   ,Y++
                   LEAU   ,--U
                   LEAU   [,S++]
                   LEAS   5,X
                   LEAS   ,Y++
                   LEAS   ,--U
                   LEAS   [,S++]

; check Z flag on index registers

		ldx		#2
		leax	-1,x
		leax	-1,x

		ldy		#2
		leay	-1,y
		leay	-1,y

		lds		#2
		leas	-1,s
		leas	-1,s

		ldu		#2
		leau	-1,u
		leau	-1,u

		lds		save
		puls	x,y,u

*
                   LSLA
                   LSLB
                   LSL    <DIR
                   LSL    EXT
                   LSL    [IND]
                   LSL    ,X
                   LSL    ,Y++
                   LSL    [,--U]
*
                   LSRA
                   LSRB
                   LSR    <DIR
                   LSR    EXT
                   LSR    [IND]
                   LSR    ,X
                   LSR    ,Y++
                   LSR    [,--U]
*
                   MUL
*
                   NEGA
                   NEGB
                   NEG    <DIR
                   NEG    EXT
                   NEG    [IND]
                   NEG    ,X
                   NEG    ,Y++
                   NEG    [,--U]
*
                   NOP
*
                   ORA    #IMM
                   ORA    <DIR
                   ORA    EXT
                   ORA    [IND]
                   ORA    ,X
                   ORA    ,Y++
                   ORA    [,--U]
                   ORB    #IMM
                   ORB    <DIR
                   ORB    EXT
                   ORB    [IND]
                   ORB    ,X
                   ORB    ,Y++
                   ORB    [,--U]
                   ORCC   #0
*

	pshs	a,b,cc,dp,x,y,u,pc
	puls	a,b,cc,dp,x,y,u
	puls	x

	pshu	a,b,cc,dp,x,y,s,pc
	pulu	a,b,cc,dp,x,y,s
	pulu	x
	tfr		y,x

*
                   ROLA
                   ROLB
                   ROL    <DIR
                   ROL    EXT
                   ROL    [IND]
                   ROL    ,X
                   ROL    ,Y++
                   ROL    [,--U]
*
                   RORA
                   RORB
                   ROR    <DIR
                   ROR    EXT
                   ROR    [IND]
                   ROR    ,X
                   ROR    ,Y++
                   ROR    [,--U]
*
;;                   RTI
;;                   RTS
*
                   SBCA   #IMM
                   SBCA   <DIR
                   SBCA   EXT
                   SBCA   [IND]
                   SBCA   ,X
                   SBCA   ,Y++
                   SBCA   [,--U]
                   SBCB   #IMM
                   SBCB   <DIR
                   SBCB   EXT
                   SBCB   [IND]
                   SBCB   ,X
                   SBCB   ,Y++
                   SBCB   [,--U]
*
                   SEX
*
                   STA    <DIR
                   STA    EXT
                   STA    [IND]
                   STA    ,X
                   STA    ,Y++
                   STA    [,--U]
                   STB    <DIR
                   STB    EXT
                   STB    [IND]
                   STB    ,X
                   STB    ,Y++
                   STB    [,--U]
                   STD    <DIR
                   STD    EXT
                   STD    [IND]
                   STD    ,X
                   STD    ,Y++
                   STD    [,--U]
*
                   SUBA   #IMM
                   SUBA   <DIR
                   SUBA   EXT
                   SUBA   [IND]
                   SUBA   ,X
                   SUBA   ,Y++
                   SUBA   [,--U]
                   SUBB   #IMM
                   SUBB   <DIR
                   SUBB   EXT
                   SUBB   [IND]
                   SUBB   ,X
                   SUBB   ,Y++
                   SUBB   [,--U]
                   SUBD   #IMM
                   SUBD   <DIR
                   SUBD   EXT
                   SUBD   [IND]
                   SUBD   ,X
                   SUBD   ,Y++
                   SUBD   [,--U]
*
                   SWI
                   SWI2
                   SWI3
;;                   SYNC
*
                   TFR    A,B
                   TFR    X,Y
*
                   TSTA
                   TSTB
                   TST    <DIR
                   TST    EXT
                   TST    [IND]
                   TST    ,X
                   TST    ,Y++
                   TST    [,--U]

*------------------------------------------------------------------------------
* indexed memory access
*------------------------------------------------------------------------------
                   lda    ,-x     automatic decrement before read
                   lda    ,x+     automatic increment after read
                   lda    1,x     constant offset from x
                   lda    -1,x    negative constant offset from x
                   lda    16,x
                   lda    -16,x
                   lda    15,x
                   lda    -15,x
	ldx #SCRATCH-127
                   lda    127,x
                   lda    128,x
	ldx #SCRATCH+128
                   lda    -127,x
                   lda    -128,x
	ldx #SCRATCH-256
                   lda    256,x
                   lda    *+2
                   lda    *-2
	ldx #SCRATCH+256
                   lda    -256,x
	ldx #SCRATCH-32767
                   lda    32767,x
                   LEAX   -$4000,X
                   leax   $8000,x
                   leay   -$40,y
                   LEAX   -350,X
                   leax   1,x
                   LEAU   512,U
                   LEAU   32,U
                   leay   32,Y
                   LEAS   2,S
				   LEAS   -2,S
                   LEAU   D,U
                   leax   -1,X
                   leay   B,Y
                   leax   -350,X

*------------------------------------------------------------------------------
* in<DIRect addressing, indexing, pointers
*------------------------------------------------------------------------------
	ldx		#SCRATCH+64

                   lda    [,--x]  auto-decrement 16-bit pointer before reading byte
                   lda    [,x++]  auto-increment 16-bit pointer after reading byte
                   jsr    [IND_RTS] call 16-bit address
;;                   lda    [,x+]   possible on 6809, but probably useless [,r+]
;;                   lda    [,-x]   possible on 6809, but probably useless [,-r]

*------------------------------------------------------------------------------
* relative addressing, position-independent memory access
*------------------------------------------------------------------------------
                   leax   *,pcr       point X to the address of symbol "start"
                   leax   *,pcr       load into register X the address of "apple"
                   ldx    *,pcr       load X with the contents of the address of symbol "start"
                   ldx    *,pcr       load into register X the contents of address "apple"
                   leax   *+1,pcr     point X to the address of symbol "start" + 1
                   lda    *,pcr   load the first opcode of the instruction "lda *,pcr"
                   lda    *+2,pc  "pc" can be used for "pcr"


		clra
		tfr		a,dp
		lds		save_s
		andcc	#$af

A_RTS	rts

A_RTI	rti


;;		org		$2000

		bsz		$2000-*

DIRECT	equ		*

;;		org		$2100

		bsz		$2100-*

save_s	rmb		2
save	rmb		2			  
IMM		equ		$12
DIR		equ		$34
EXT		rmb		2
indval	rmb		2
IND		fdb		indval
IND_RTS	fdb		A_RTS

		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
SCRATCH fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e
		fdb		$2300,$2302,$2304,$2306,$2308,$230a,$230c,$230e

;;		org		$2f80
		bsz		$2f80-*

STACK	equ		*

		if		0

		org		$100
		rti				; swi3
		org		$103
		rti				; swi2
		org		$106
		rti				; swi

		org		$fefa
		lbra	$106
		org		$fef1
		lbra	$103
		org		$feee
		lbra	$100

		endc

		end		START


