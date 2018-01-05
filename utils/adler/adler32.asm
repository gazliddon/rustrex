
			org		$1000

Start:
			lda		#$d4
			tfr		a,cc
			lda		#$20
			tfr		a,dp
			setdp	$20
			sts		Stack
			lds		#$1fff

			jsr		RndInit
			jsr		Adler32Init

			ldy		#4				; loop count
Loop:
			ldu		#Data
			ldx		#DataLen
			jsr		Adler32Update

			ldx		#Data
			ldu		#Data+1
a@			lda		,x+
			sta		,u+
			cmpx	#Data+DataLen
			bne		a@

			jsr		Adler32GetChecksum		D=Adler32 checksum
			ldx		#Data
			sta		,x
			eorb	1,x

			bcc		a@
			daa
a@			bcs		b@
			inca
b@			beq		c@
			deca
c@			bge		d@
			jsr		RndNext
d@			bgt		e@
			adda	,x
e@			bhi		f@
			adca	,x
f@			bhs		g@
			mul
g@			ble		h@
			suba	,x
h@			blo		i@
			sbca	,x
i@			bls		j@
			lsla
j@			blt		k@
			rola
k@			bmi		l@
			lsra
l@			bne		m@
			rora
m@			bpl		n@
			anda	#$0f
n@			bvc		q@
			ora		#$f0
q@			bvs		r@
			eora	#$aa
r@

			eora	,x
			leay	-1,y
			bne		Loop

			jsr		Adler32GetChecksum		; final value

			clra
			tfr		a,dp
			lds		Stack

			rts

Data		fcc		'justsomerandomdata'
DataLen		equ		*-Data

AdlerA		bsz		4
AdlerB		bsz		4
AdlerCount	bsz		2
AdlerMod	equ		65521 
AdlerMax	equ		5550
Stack		bsz		2

; direct page stuff
mod32		equ		$2000
seed		equ		mod32+4
last		equ		seed+2

RndInit:
			ldx		#$7ffe
			stx		<seed
			ldd		#1
			std		<last
			rts

RndNext:
			ldd		<last
			lsra
			rorb
			bcc		a@
			eora	<seed
			eorb	<seed+1
a@			std		<last
			rts		

Adler32Init:	
			pshs	x 
			ldx		#0 
			stx		AdlerCount 
			stx		AdlerB 
			stx		AdlerB+2 
			stx		AdlerA 
			leax	1,x 
			stx		AdlerA+2 
			puls	x,pc 

* ,u = buffer
* x = count
Adler32Update:
			pshs	d,x,u 
a@			clra	
			ldb		,u+			adlera=adlera+(byte)
			addd	AdlerA+2 
			std		AdlerA+2 
			bcc		b@ 
			ldd		AdlerA 
			addd	#1			adcd
			std		AdlerA 
b@			ldd		AdlerA+2 	adlerb=adlerb+adlera
			addd	AdlerB+2 
			std		AdlerB+2 
			ldd		AdlerA 
			bcc		c@ 
			addd	#1			adcd
c@			addd	AdlerB 
			std		AdlerB 
			ldd		AdlerCount 
			addd	#1 
			cmpd	#AdlerMax 
			bne		d@ 
			bsr		Adler32UpdateModulus 
			ldd		#0 
d@			std		AdlerCount 
			leax	-1,x 
			bne		a@ 
			puls	d,x,u,pc 

Adler32UpdateModulus:	
			pshs	x,u 
			ldu		#AdlerA 
			ldx		#AdlerMod 
			bsr		Adler32CalcModulus 
			ldu		#AdlerB 
			bsr		Adler32CalcModulus 
			puls	x,u,pc 

* calculate modulo
* input:	,u = numerator		(32 bits)
*			x = denominator		(16 bits)
* output:	modulo stored ,u	(16 bits promoted to 32)
Adler32CalcModulus:
			pshs	d,x 
			stx		mod32+2 
			ldx		#0 
			stx		mod32 
a@			ldd		,u 
			cmpd	mod32 
			bhi		b@ 
			blo		c@ 
			ldd		2,u 
			cmpd	mod32+2 
			bhi		b@ 
			blo		c@ 
b@			lsl		mod32+3 
			rol		mod32+2 
			rol		mod32+1 
			rol		mod32 
			leax	1,x 
			bra		a@ 
c@			

			cmpx	#0 
			beq		g@ 
a@			lsr		mod32 
			ror		mod32+1 
			ror		mod32+2 
			ror		mod32+3 
			ldd		mod32 
			cmpd	,u 
			blo		b@ 
			bhi		c@ 
			ldd		mod32+2 
			cmpd	2,u 
			bhi		c@ 
b@			ldd		2,u 
			subd	mod32+2 
			std		2,u 
			ldd		,u 
			bcc		d@ 
			subd	#1 
d@			subd	mod32		sbcd on 6309
			std		,u 
c@			leax	-1,x 
			bne		a@ 
g@			puls	d,x,pc 

* stores final value in adlera memory location
Adler32GetChecksum:	
			ldd		AdlerCount 
			beq		a@ 
			bsr		Adler32UpdateModulus 
a@			ldd		AdlerB+2 
			std		AdlerA 
			rts		

			end		Start