    /*******************************************************************/
/* 6502 CPU emulator written by Larry Bank                         */
/* Copyright 1998 BitBank Software, Inc.                           */
/*                                                                 */
/* This code was written from scratch using the 6502 data from     */
/* the various sources and other peoples code as a guide.          */
/*                                                                 */
/* Change history:                                                 */
/* 1/18/98 Wrote it - Larry B.                                     */
/*******************************************************************/
#include <windows.h>
#include <string.h>
#include "emu.h"

#define F_CARRY     1
#define F_ZERO      2
#define F_IRQMASK   4
#define F_DECIMAL   8
#define F_BREAK     16
#define F_UNUSED    32
#define F_OVERFLOW  64
#define F_NEGATIVE  128

/* Some statics */
EMUHANDLERS *mem_handlers;
unsigned char *mem_map;

unsigned char M6502ReadByte(unsigned char *, unsigned short);
unsigned short M6502ReadWord(unsigned char *, unsigned short);
__inline void M6502PUSHB(unsigned char *, REGS6502 *, unsigned char);
__inline void M6502PUSHW(unsigned char *, REGS6502 *, unsigned short);
__inline unsigned char M6502PULLB(unsigned char *, REGS6502 *);
__inline unsigned short M6502PULLW(unsigned char *, REGS6502 *);

unsigned char uc6502Cycles[256] =
                        {7,6,0,0,0,3,5,0,3,2,2,0,0,4,6,6, /* 00-0F */
                         2,5,0,0,0,4,6,6,2,4,0,0,0,5,7,7, /* 10-1F */
                         6,6,0,0,3,3,5,0,4,2,2,0,4,4,6,0, /* 20-2F */
                         2,5,0,0,0,4,6,6,2,4,0,0,0,5,7,0, /* 30-3F */
                         6,6,0,0,0,3,5,5,3,2,2,0,3,4,6,0, /* 40-4F */
                         2,5,0,0,0,4,6,6,2,4,0,0,0,5,7,7, /* 50-5F */
                         6,6,0,0,0,3,5,5,4,2,2,0,5,4,6,0, /* 60-6F */
                         2,5,0,0,0,4,6,0,2,4,0,0,0,5,7,0, /* 70-7F */
                         0,6,0,0,3,3,3,0,2,0,2,0,4,4,4,0, /* 80-8F */
                         2,6,0,0,4,4,4,0,2,5,2,0,0,5,0,0, /* 90-9F */
                         2,6,2,0,3,3,3,0,2,2,2,0,4,4,4,0, /* A0-AF */
                         2,5,0,0,4,4,4,0,2,4,2,0,4,4,4,0, /* B0-BF */
                         2,6,0,0,3,3,5,0,2,2,2,0,4,4,6,0, /* C0-CF */
                         2,5,0,0,0,4,6,0,2,4,0,0,0,5,7,0, /* D0-DF */
                         2,6,0,0,3,3,5,0,2,2,2,0,4,4,6,0, /* E0-EF */
                         2,5,0,0,0,4,6,0,2,4,0,0,0,5,7,0};/* F0-FF */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : RESET6502(char *, REGS6502 *)                              *
 *                                                                          *
 *  PURPOSE    : Get the 6502 after a reset.                                *
 *                                                                          *
 ****************************************************************************/
void RESET6502(char *mem, REGS6502 *regs)
{
   mem_map = mem;
   memset(regs, 0, sizeof(REGS6502)); /* Start with a clean slate at reset */
   regs->usRegPC = mem_map[MEM_ROM+0xfffc] + mem_map[MEM_ROM+0xfffd] * 256; /* Start execution at reset vector */
   regs->ucRegP = F_ZERO | F_UNUSED;

} /* RESET6502() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502BIT(REGS6502 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a bit test and update flags.                       *
 *                                                                          *
 ****************************************************************************/
__inline void M6502BIT(REGS6502 *regs, unsigned char ucByte)
{
unsigned char uc;

   regs->ucRegP &= ~(F_OVERFLOW | F_ZERO | F_NEGATIVE);
   uc = ucByte & (F_OVERFLOW | F_NEGATIVE); /* Top two bits of argument go right into flags */
   regs->ucRegP |= uc;
   uc = regs->ucRegA & ucByte;
   if (uc == 0)
      regs->ucRegP |= F_ZERO;

} /* M6502BIT() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502INC(REGS6502 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform an increment and update flags.                     *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502INC(REGS6502 *regs, unsigned char ucByte)
{
   ucByte++;
   regs->ucRegP &= ~(F_ZERO | F_NEGATIVE);
   if (ucByte == 0)
      regs->ucRegP |= F_ZERO;
   if (ucByte & 0x80)
       regs->ucRegP |= F_NEGATIVE;
   return ucByte;

} /* M6502INC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502DEC(REGS6502 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a decrement and update flags.                      *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502DEC(REGS6502 *regs, unsigned char ucByte)
{
   ucByte--;
   regs->ucRegP &= ~(F_ZERO | F_NEGATIVE);
   if (ucByte == 0)
      regs->ucRegP |= F_ZERO;
   if (ucByte & 0x80)
       regs->ucRegP |= F_NEGATIVE;
   return ucByte;

} /* M6502DEC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502ADC(REGS6502 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform a 8-bit addition with carry.                       *
 *                                                                          *
 ****************************************************************************/
__inline void M6502ADC(REGS6502 *regs, unsigned char ucByte)
{
register unsigned short usTemp;
register unsigned char uc, low, high;

   if (regs->ucRegP & F_DECIMAL)
      {
      uc = regs->ucRegA + ucByte + (regs->ucRegP & F_CARRY);
      regs->ucRegP &= ~(F_ZERO | F_OVERFLOW | F_NEGATIVE);
      if (uc == 0)
         regs->ucRegP |= F_ZERO;
      low = (regs->ucRegA & 0x0F) + (ucByte & 0x0F) + (regs->ucRegP & F_CARRY);
      if (low > 9)
         low += 6;
      high = (regs->ucRegA >> 4) + (ucByte >> 4) + ((low & 0x10) >> 4);
      if (high & 8)
         regs->ucRegP |= F_NEGATIVE;
      uc = ~(regs->ucRegA ^ ucByte) & (regs->ucRegA ^ (high << 4)) & 0x80;
      if (uc)
         regs->ucRegP |= F_OVERFLOW;
      if (high > 9)
         high += 6;
      regs->ucRegA = (low & 0x0F) | (high << 4);
      if (high & 0x10)
         regs->ucRegP |= F_CARRY;
      else
         regs->ucRegP &= ~F_CARRY;
      }
   else
      {
      usTemp = (unsigned short)ucByte + (unsigned short)regs->ucRegA + (regs->ucRegP & F_CARRY);
      regs->ucRegP &= ~(F_ZERO | F_OVERFLOW | F_NEGATIVE | F_CARRY);
      uc = ~(regs->ucRegA ^ ucByte) & (regs->ucRegA ^ usTemp) & 0x80;
      if (uc)
         regs->ucRegP |= F_OVERFLOW;
      if (usTemp & 0x100)
         regs->ucRegP |= F_CARRY;
      regs->ucRegA = usTemp & 0xff;
      if (regs->ucRegA & 0x80)
         regs->ucRegP |= F_NEGATIVE;
      if (regs->ucRegA == 0)
         regs->ucRegP |= F_ZERO;
      }

} /* M6502ADC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502SBC(REGS6502 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform a 8-bit subtraction with carry and update flags.   *
 *                                                                          *
 ****************************************************************************/
__inline void M6502SBC(REGS6502 *regs, unsigned char ucByte)
{
register unsigned short usTemp;
register unsigned char uc, low, high;

   if (regs->ucRegP & F_DECIMAL) /* adjust for decimal mode */
      {
      usTemp = regs->ucRegA - ucByte - (~regs->ucRegP & F_CARRY);
      regs->ucRegP &= ~(F_ZERO | F_NEGATIVE | F_OVERFLOW | F_CARRY);
      uc = (regs->ucRegA ^ ucByte) & (regs->ucRegA ^ usTemp) & 0x80;
      if (uc)
         regs->ucRegP |= F_OVERFLOW;
      low = (regs->ucRegA & 0x0F) - (ucByte & 0x0F) - (~regs->ucRegP & F_CARRY);
      if (low & 0x10)
         low -= 6;
      high = (regs->ucRegA >> 4) - (ucByte >> 4) - ((low & 0x10) >> 4);
      if (high & 0x10)
         high -= 6;
      if (!(usTemp & 0x100))
         regs->ucRegP |= F_CARRY;
      if (usTemp & 0x80)
         regs->ucRegP |= F_NEGATIVE;
      regs->ucRegA = (low & 0x0F) | (high << 4);
      if (regs->ucRegA == 0)
         regs->ucRegP |= F_ZERO;
      }
   else
      {
      usTemp = regs->ucRegA - ucByte - (~regs->ucRegP & F_CARRY);
      regs->ucRegP &= ~(F_ZERO | F_NEGATIVE | F_OVERFLOW | F_CARRY);
      uc = (regs->ucRegA ^ ucByte) & (regs->ucRegA ^ usTemp) & 0x80;
      if (uc)
         regs->ucRegP |= F_OVERFLOW;
      if (!(usTemp & 0x100))
         regs->ucRegP |= F_CARRY;
      if (usTemp & 0x80)
         regs->ucRegP |= F_NEGATIVE;
      regs->ucRegA = usTemp & 0xff;
      if (regs->ucRegA == 0)
         regs->ucRegP |= F_ZERO;
      }

} /* M6502SBC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502CMP(REGS6502 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform a 8-bit comparison.                                *
 *                                                                          *
 ****************************************************************************/
__inline void M6502CMP(REGS6502 *regs, unsigned char ucByte1, unsigned char ucByte2)
{
register signed short sTemp;
   sTemp = (signed short)ucByte1 - (signed short)ucByte2;
   regs->ucRegP &= ~(F_ZERO | F_NEGATIVE);
   if (sTemp == 0)
      regs->ucRegP |= F_ZERO;
   if (sTemp & 0x80)
       regs->ucRegP |= F_NEGATIVE;
   if (sTemp & 0x100)
       regs->ucRegP &= ~F_CARRY; /* Works backwards!! */
   else
       regs->ucRegP |= F_CARRY;

} /* M6502CMP() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502LSR(REGS6502 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a logical shift right and update flags.            *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502LSR(REGS6502 *regs, unsigned char ucByte)
{

   regs->ucRegP &= ~(F_ZERO | F_CARRY | F_NEGATIVE);
   if (ucByte & 0x01)
      regs->ucRegP |= F_CARRY;
   ucByte >>= 1;
   if (ucByte == 0)
      regs->ucRegP |= F_ZERO;
   return ucByte;

} /* M6502LSR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502ASL(REGS6502 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a arithmetic shift left and update flags.          *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502ASL(REGS6502 *regs, unsigned char ucByte)
{
unsigned short usOld = (unsigned short)ucByte;

   regs->ucRegP &= ~(F_ZERO | F_CARRY | F_NEGATIVE);
   if (ucByte & 0x80)
      regs->ucRegP |= F_CARRY;
   ucByte <<=1;
   if (ucByte == 0)
      regs->ucRegP |= F_ZERO;
   if (ucByte & 0x80)
      regs->ucRegP |= F_NEGATIVE;
   return ucByte;

} /* M6502ASL() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502ROL(REGS6502 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a rotate left and update flags.                    *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502ROL(REGS6502 *regs, unsigned char ucByte)
{
unsigned char ucOld = ucByte;
unsigned char uc;

   uc = regs->ucRegP & 1; /* Preserve old carry flag */
   regs->ucRegP &= ~(F_ZERO | F_CARRY | F_NEGATIVE);
   if (ucByte & 0x80)
      regs->ucRegP |= F_CARRY;
   ucByte = ucByte <<1 | uc;
   if (ucByte == 0)
      regs->ucRegP |= F_ZERO;
   if (ucByte & 0x80)
      regs->ucRegP |= F_NEGATIVE;
   return ucByte;

} /* M6502ROL() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502EOR(REGS6502 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform an exclusive or and update flags.                  *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502EOR(REGS6502 *regs, unsigned char ucByte1, char ucByte2)
{
register unsigned char ucTemp;

   regs->ucRegP &= ~(F_ZERO | F_NEGATIVE);
   ucTemp = ucByte1 ^ ucByte2;
   if (ucTemp == 0)
      regs->ucRegP |= F_ZERO;
   if (ucTemp & 0x80)
      regs->ucRegP |= F_NEGATIVE;
   return ucTemp;

} /* M6502EOR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502OR(REGS6502 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform an inclusive or and update flags.                  *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502OR(REGS6502 *regs, unsigned char ucByte1, char ucByte2)
{
register unsigned char ucTemp;

   regs->ucRegP &= ~(F_ZERO | F_NEGATIVE);
   ucTemp = ucByte1 | ucByte2;
   if (ucTemp == 0)
      regs->ucRegP |= F_ZERO;
   if (ucTemp & 0x80)
      regs->ucRegP |= F_NEGATIVE;
   return ucTemp;

} /* M6502OR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502AND(REGS6502 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform an AND and update flags.                           *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502AND(REGS6502 *regs, unsigned char ucByte1, char ucByte2)
{
register unsigned char ucTemp;

   regs->ucRegP &= ~(F_ZERO | F_NEGATIVE);
   ucTemp = ucByte1 & ucByte2;
   if (ucTemp == 0)
      regs->ucRegP |= F_ZERO;
   if (ucTemp & 0x80)
      regs->ucRegP |= F_NEGATIVE;
   return ucTemp;

} /* M6502AND() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502ROR(REGS6502 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a rotate right and update flags.                   *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502ROR(REGS6502 *regs, unsigned char ucByte)
{
unsigned char uc;

   uc = regs->ucRegP & 1; /* Preserve old carry flag */
   regs->ucRegP &= ~(F_ZERO | F_CARRY | F_NEGATIVE);
   if (ucByte & 0x01)
      regs->ucRegP |= F_CARRY;
   ucByte = ucByte >> 1 | uc << 7;
   if (ucByte == 0)
      regs->ucRegP |= F_ZERO;
   if (ucByte & 0x80)
      regs->ucRegP |= F_NEGATIVE;
   return ucByte;

} /* M6502ROR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502WriteByte(char *, short, char)                        *
 *                                                                          *
 *  PURPOSE    : Write a byte to memory, check for hardware.                *
 *                                                                          *
 ****************************************************************************/
__inline void M6502WriteByte(unsigned char *mem_map, unsigned short usAddr, unsigned char ucByte)
{
unsigned char c;

   switch(c = mem_map[usAddr+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         mem_map[usAddr+MEM_RAM] = ucByte;
         break;
      case 1: /* Normal ROM - nothing to do */
         break;
      default: /* Call special handler routine for this address */
         (mem_handlers[c-2].pfn_write)(usAddr, ucByte);
         break;
      }

} /* M6502WriteByte() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502FlagsNZ8(M6502REGS *, char)                           *
 *                                                                          *
 *  PURPOSE    : Set appropriate flags for 8 bit value.                     *
 *                                                                          *
 ****************************************************************************/
__inline void M6502FlagsNZ8(REGS6502 *regs, unsigned char ucByte)
{
    regs->ucRegP &= ~(F_ZERO | F_NEGATIVE);
    if (ucByte == 0)
       regs->ucRegP |= F_ZERO;
    if (ucByte & 0x80)
       regs->ucRegP |= F_NEGATIVE;

} /* M6502FlagsNZ8() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502ReadByte(char *, short)                               *
 *                                                                          *
 *  PURPOSE    : Read a byte from memory, check for hardware.               *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502ReadByte(unsigned char *mem_map, unsigned short usAddr)
{
unsigned char c;
   switch(c = mem_map[usAddr+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         return mem_map[usAddr+MEM_RAM]; /* Just return it */
         break;
      case 1: /* Normal ROM */
         return mem_map[usAddr+MEM_ROM]; /* Just return it */
         break;
      default: /* Call special handler routine for this address */
         return (mem_handlers[c-2].pfn_read)(usAddr);
         break;
      }

} /* M6502ReadByte() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502ReadWord(char *, short)                               *
 *                                                                          *
 *  PURPOSE    : Read a word from memory, check for hardware.               *
 *                                                                          *
 ****************************************************************************/
__inline unsigned short M6502ReadWord(unsigned char *mem_map, unsigned short usAddr)
{
unsigned short usWord;

   usWord = M6502ReadByte(mem_map, usAddr++);
   usWord += M6502ReadByte(mem_map, usAddr) * 256;
   return usWord;

} /* M6502ReadWord() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502PUSHB(char *, REGS6502 *, char)                       *
 *                                                                          *
 *  PURPOSE    : Push a byte to the 'S' stack.                              *
 *                                                                          *
 ****************************************************************************/
__inline void M6502PUSHB(unsigned char *mem_map, REGS6502 *regs, unsigned char ucByte)
{

   M6502WriteByte(mem_map, (unsigned short)(regs->ucRegS-- + 0x100), ucByte);

} /* M6502PUSHB() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502PUSHW(char *, REGS6502 *)                             *
 *                                                                          *
 *  PURPOSE    : Push a word to the 'S' stack.                              *
 *                                                                          *
 ****************************************************************************/
__inline void M6502PUSHW(unsigned char *mem_map, REGS6502 *regs, unsigned short usWord)
{

   M6502WriteByte(mem_map, (unsigned short)(regs->ucRegS-- + 0x100), (unsigned char)(usWord >> 8));
   M6502WriteByte(mem_map, (unsigned short)(regs->ucRegS-- + 0x100), (unsigned char)(usWord & 0xff));

} /* M6502PUSHW() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502PULLB(char *, REGS6502 *)                             *
 *                                                                          *
 *  PURPOSE    : Pull a byte from the 'S' stack.                            *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6502PULLB(unsigned char *mem_map, REGS6502 *regs)
{

   return M6502ReadByte(mem_map, (unsigned short)(0x100 + ++regs->ucRegS));

} /* M6502PULLB() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6502PULLW(char *, REGS6502 *)                             *
 *                                                                          *
 *  PURPOSE    : Pull a word from the 'S' stack.                            *
 *                                                                          *
 ****************************************************************************/
__inline unsigned short M6502PULLW(unsigned char *mem_map, REGS6502 *regs)
{
unsigned char hi, lo;

   lo = M6502ReadByte(mem_map, (unsigned short)(++regs->ucRegS + 0x100));
   hi = M6502ReadByte(mem_map, (unsigned short)(++regs->ucRegS + 0x100));
   return (hi * 256 + lo);

} /* M6502PULLW() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : EXEC6502(char *, REGS6502 *, EMUHANDLERS *, int *, char)   *
 *                                                                          *
 *  PURPOSE    : Emulate the M6502 microprocessor for N clock cycles.       *
 *                                                                          *
 ****************************************************************************/
void EXEC6502(char *mem, REGS6502 *regs, EMUHANDLERS *emuh, int *iClocks, unsigned char *ucIRQs)
{
unsigned short PC;  /* Current Program Counter address */
register unsigned short usAddr; /* Temp address */
register unsigned char ucTemp;
register signed short sTemp;

   mem_handlers = emuh; /* Assign to static for faster execution */
   mem_map = mem; /* ditto */

   PC = regs->usRegPC;
   while (*iClocks > 0) /* Execute for the amount of time alloted */
      {
      /*--- First check for any pending IRQs ---*/
      if (*ucIRQs)
         {
         if (*ucIRQs & INT_NMI) /* NMI is highest priority */
            {
            M6502PUSHW(mem_map, regs, PC);
            M6502PUSHB(mem_map, regs, regs->ucRegP);
            regs->ucRegP &= ~F_DECIMAL;
            *iClocks -= 7;
            PC = M6502ReadWord(mem_map, 0xfffa);
            *ucIRQs &= ~INT_NMI; /* clear this bit */
            goto doexecute;
            }
         if (*ucIRQs & INT_IRQ && (regs->ucRegP & F_IRQMASK) == 0) /* IRQ is lowest priority */
            {
            M6502PUSHW(mem_map, regs, PC);
            M6502PUSHB(mem_map, regs, regs->ucRegP);
            regs->ucRegP |= F_IRQMASK; /* Mask interrupts during service routine */
            regs->ucRegP &= ~F_DECIMAL;
            PC = M6502ReadWord(mem_map, 0xfffe);
            *ucIRQs &= ~INT_IRQ; /* clear this bit */
            *iClocks -= 7;
            goto doexecute;
            }
         }
doexecute:
      ucTemp = M6502ReadByte(mem_map, PC++);
      *iClocks -= (int)uc6502Cycles[ucTemp]; /* Subtract execution time of this instruction */
      switch(ucTemp)
         {
         case 0x00: /* BRK - software interrupt */
            M6502PUSHW(mem_map, regs, PC);
            regs->ucRegP |= F_BREAK;
            M6502PUSHB(mem_map, regs, regs->ucRegP);
            regs->ucRegP |= F_IRQMASK;
            regs->ucRegP &= ~F_DECIMAL;
            PC = M6502ReadWord(mem_map, 0xfffe);
            break;

         case 0x01: /* ORA - (z,x) */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            usAddr = M6502ReadByte(mem_map, ucTemp++);
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp); /* In case of wrap-around */
            ucTemp = M6502ReadByte(mem_map, usAddr);
            regs->ucRegA = M6502OR(regs, regs->ucRegA, ucTemp);
            break;

         case 0x05: /* ORA - z */
            usAddr = (unsigned short)M6502ReadByte(mem_map, PC++);
            regs->ucRegA = M6502OR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x06: /* ASL - z */
            usAddr = (unsigned short)M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ASL(regs, ucTemp));
            break;

         case 0x08: /* PHP - push flags */
            M6502PUSHB(mem_map, regs, regs->ucRegP);
            break;

         case 0x09: /* ORA - immediate */
            ucTemp = M6502ReadByte(mem_map, PC++);
            regs->ucRegA = M6502OR(regs, regs->ucRegA, ucTemp);
            break;

         case 0x0a: /* ASLA */
            regs->ucRegA = M6502ASL(regs, regs->ucRegA);
            break;

         case 0x0d: /* ORA - absolute */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            regs->ucRegA = M6502OR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x0e: /* ASL - absolute */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ASL(regs, ucTemp));
            break;

         case 0x10: /* BPL */
            sTemp = (signed short)(signed char)M6502ReadByte(mem_map, PC++);
            if (!(regs->ucRegP & F_NEGATIVE))
               {
               *iClocks -= 1;
               PC += sTemp;
               }
            break;

         case 0x11: /* ORA (z),y */
            ucTemp = M6502ReadByte(mem_map, PC++);
            usAddr = M6502ReadByte(mem_map, ucTemp++); /* Account for zp wrap-around */
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp) + regs->ucRegY;
            regs->ucRegA = M6502OR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x15: /* ORA z,x */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            regs->ucRegA = M6502OR(regs, regs->ucRegA, M6502ReadByte(mem_map, ucTemp));
            break;

         case 0x16: /* ASL z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ASL(regs, ucTemp));
            break;

         case 0x18: /* CLC */
            regs->ucRegP &= ~F_CARRY;
            break;

         case 0x19: /* ORA abs,y */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegY;
            PC += 2;
            regs->ucRegA = M6502OR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x1d: /* ORA abs,x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            regs->ucRegA = M6502OR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x1e: /* ASL abs, x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ASL(regs, ucTemp));
            break;

         case 0x20: /* JSR abs */
            M6502PUSHW(mem_map, regs, (unsigned short)(PC+1));
            PC = M6502ReadWord(mem_map, PC);
            break;

         case 0x21: /* AND (z,x) */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            usAddr = M6502ReadByte(mem_map, ucTemp++);
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp); /* In case of wrap-around */
            regs->ucRegA = M6502AND(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x24: /* BIT z */
            usAddr = M6502ReadByte(mem_map, PC++);
            M6502BIT(regs, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x25: /* AND z */
            usAddr = M6502ReadByte(mem_map, PC++);
            regs->ucRegA = M6502AND(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x26: /* ROL z */
            usAddr = M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ROL(regs, ucTemp));
            break;

         case 0x28: /* PLP */
            regs->ucRegP = M6502PULLB(mem_map, regs);
            break;

         case 0x29: /* AND - immediate */
            ucTemp = M6502ReadByte(mem_map, PC++);
            regs->ucRegA = M6502AND(regs, regs->ucRegA, ucTemp);
            break;

         case 0x2a: /* ROLA */
            regs->ucRegA = M6502ROL(regs, regs->ucRegA);
            break;

         case 0x2c: /* BIT abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            M6502BIT(regs, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x2d: /* AND abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            regs->ucRegA = M6502AND(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x2e: /* ROL abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ROL(regs, ucTemp));
            break;

         case 0x30: /* BMI */
            sTemp = (signed short)(signed char)M6502ReadByte(mem_map, PC++);
            if (regs->ucRegP & F_NEGATIVE)
               PC += sTemp;
            break;

         case 0x31: /* AND (z),y */
            ucTemp = M6502ReadByte(mem_map, PC++);
            usAddr = M6502ReadByte(mem_map, ucTemp++); /* Account for zp wrap-around */
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp) + regs->ucRegY;
            regs->ucRegA = M6502AND(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x35: /* AND z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            regs->ucRegA = M6502AND(regs, regs->ucRegA, ucTemp);
            break;

         case 0x36: /* ROL z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ROL(regs, ucTemp));
            break;

         case 0x38: /* SEC */
            regs->ucRegP |= F_CARRY;
            break;

         case 0x39: /* AND abs,y */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegY;
            PC += 2;
            regs->ucRegA = M6502AND(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x3D: /* AND abs, x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            regs->ucRegA = M6502AND(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x3E: /* ROL abs,x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ROL(regs, ucTemp));
            break;

         case 0x40: /* RTI */
            regs->ucRegP = M6502PULLB(mem_map, regs) | F_UNUSED;
            PC = M6502PULLW(mem_map, regs);
            break;

         case 0x41: /* EOR (z,x) */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            usAddr = M6502ReadByte(mem_map, ucTemp++);
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp); /* In case of wrap-around */
            regs->ucRegA = M6502EOR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x45: /* EOR z */
            usAddr = M6502ReadByte(mem_map, PC++);
            regs->ucRegA = M6502EOR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x46: /* LSR z */
            usAddr = M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502LSR(regs, ucTemp));
            break;

         case 0x48: /* PHA */
            M6502PUSHB(mem_map, regs, regs->ucRegA);
            break;

         case 0x49: /* EOR - immediate */
            ucTemp = M6502ReadByte(mem_map, PC++);
            regs->ucRegA = M6502EOR(regs, regs->ucRegA, ucTemp);
            break;

         case 0x4A: /* LSR */
            regs->ucRegA = M6502LSR(regs, regs->ucRegA);
            break;

         case 0x4C: /* JMP abs */
            PC = M6502ReadWord(mem_map, PC);
            break;

         case 0x4D: /* EOR abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            regs->ucRegA = M6502EOR(regs, ucTemp, regs->ucRegA);
            break;

         case 0x4E: /* LSR abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502LSR(regs, ucTemp));
            break;

         case 0x50: /* BVC */
            sTemp = (signed short)(signed char)M6502ReadByte(mem_map, PC++);
            if (!(regs->ucRegP & F_OVERFLOW))
               {
               PC += sTemp;
               *iClocks -= 1;
               }
            break;

         case 0x51: /* EOR (z),y */
            ucTemp = M6502ReadByte(mem_map, PC++);
            usAddr = M6502ReadByte(mem_map, ucTemp++); /* Account for zp wrap-around */
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp) + regs->ucRegY;
            regs->ucRegA = M6502EOR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x55: /* EOR z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            regs->ucRegA = M6502EOR(regs, regs->ucRegA, ucTemp);
            break;

         case 0x56: /* LSR z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502LSR(regs, ucTemp));
            break;

         case 0x58: /* CLI */
            regs->ucRegP &= ~F_IRQMASK;
            break;

         case 0x59: /* EOR abs,y */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegY;
            PC += 2;
            regs->ucRegA = M6502EOR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x5D: /* EOR abs, x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            regs->ucRegA = M6502EOR(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x5E: /* LSR abs,x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502LSR(regs, ucTemp));
            break;

         case 0x60: /* RTS */
            PC = M6502PULLW(mem_map, regs) + 1;
            break;

         case 0x61: /* ADC (z,x) */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            usAddr = M6502ReadByte(mem_map, ucTemp++);
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp); /* In case of wrap-around */
            M6502ADC(regs, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x65: /* ADC z */
            usAddr = M6502ReadByte(mem_map, PC++);
            M6502ADC(regs, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x66: /* ROR z */
            usAddr = M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ROR(regs, ucTemp));
            break;

         case 0x68: /* PLA */
            regs->ucRegA = M6502PULLB(mem_map, regs);
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0x69: /* ADC - immediate */
            ucTemp = M6502ReadByte(mem_map, PC++);
            M6502ADC(regs, ucTemp);
            break;

         case 0x6A: /* ROR */
            regs->ucRegA = M6502ROR(regs, regs->ucRegA);
            break;

         case 0x6C: /* JMP (ind) */
            usAddr = M6502ReadWord(mem_map, PC);
            PC = M6502ReadWord(mem_map, usAddr);
            break;

         case 0x6D: /* ADC abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502ADC(regs, ucTemp);
            break;

         case 0x6E: /* ROR abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ROR(regs, ucTemp));
            break;

         case 0x70: /* BVS */
            sTemp = (signed short)(signed char)M6502ReadByte(mem_map, PC++);
            if (regs->ucRegP & F_OVERFLOW)
               {
               PC += sTemp;
               *iClocks -= 1;
               }
            break;

         case 0x71: /* ADC (z),y */
            ucTemp = M6502ReadByte(mem_map, PC++);
            usAddr = M6502ReadByte(mem_map, ucTemp++); /* Account for zp wrap-around */
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp) + regs->ucRegY;
            M6502ADC(regs, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x75: /* ADC z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502ADC(regs, ucTemp);
            break;

         case 0x76: /* ROR z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ROR(regs, ucTemp));
            break;

         case 0x78: /* SEI */
            regs->ucRegP |= F_IRQMASK;
            break;

         case 0x79: /* ADC abs,y */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegY;
            PC += 2;
            M6502ADC(regs, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x7D: /* ADC abs, x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            M6502ADC(regs, M6502ReadByte(mem_map, usAddr));
            break;

         case 0x7E: /* ROR abs,x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502ROR(regs, ucTemp));
            break;

         case 0x81: /* STA (z,x) */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            usAddr = M6502ReadByte(mem_map, ucTemp++);
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp); /* In case of wrap-around */
            M6502WriteByte(mem_map, usAddr, regs->ucRegA);
            break;

         case 0x84: /* STY z */
            usAddr = M6502ReadByte(mem_map, PC++);
            M6502WriteByte(mem_map, usAddr, regs->ucRegY);
            break;

         case 0x85: /* STA z */
            usAddr = M6502ReadByte(mem_map, PC++);
            M6502WriteByte(mem_map, usAddr, regs->ucRegA);
            break;

         case 0x86: /* STX z */
            usAddr = M6502ReadByte(mem_map, PC++);
            M6502WriteByte(mem_map, usAddr, regs->ucRegX);
            break;

         case 0x88: /* DEY */
            regs->ucRegY = M6502DEC(regs, regs->ucRegY);
            break;

         case 0x8A: /* TXA */
            regs->ucRegA = regs->ucRegX;
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0x8C: /* STY abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            M6502WriteByte(mem_map, usAddr, regs->ucRegY);
            break;

         case 0x8D: /* STA abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            M6502WriteByte(mem_map, usAddr, regs->ucRegA);
            break;

         case 0x8E: /* STX abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            M6502WriteByte(mem_map, usAddr, regs->ucRegX);
            break;

         case 0x90: /* BCC */
            sTemp = (signed short)(signed char)M6502ReadByte(mem_map, PC++);
            if (!(regs->ucRegP & F_CARRY))
               {
               PC += sTemp;
               *iClocks -= 1;
               }
            break;

         case 0x91: /* STA (z),y */
            ucTemp = M6502ReadByte(mem_map, PC++);
            usAddr = M6502ReadByte(mem_map, ucTemp++); /* Account for zp wrap-around */
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp) + regs->ucRegY;
            M6502WriteByte(mem_map, usAddr, regs->ucRegA);
            break;

         case 0x94: /* STY z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            M6502WriteByte(mem_map, usAddr, regs->ucRegY);
            break;

         case 0x95: /* STA z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            M6502WriteByte(mem_map, usAddr, regs->ucRegA);
            break;

         case 0x96: /* STX z,y */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegY);
            M6502WriteByte(mem_map, usAddr, regs->ucRegX);
            break;

         case 0x98: /* TYA */
            regs->ucRegA = regs->ucRegY;
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0x99: /* STA abs,y */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegY;
            PC += 2;
            M6502WriteByte(mem_map, usAddr, regs->ucRegA);
            break;

         case 0x9A: /* TXS */
            regs->ucRegS = regs->ucRegX;
            break;

         case 0x9D: /* STA abs,x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            M6502WriteByte(mem_map, usAddr, regs->ucRegA);
            break;

         case 0xA0: /* LDY - immediate */
            regs->ucRegY = M6502ReadByte(mem_map, PC++);
            M6502FlagsNZ8(regs, regs->ucRegY);
            break;

         case 0xA1: /* LDA (z,x) */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            usAddr = M6502ReadByte(mem_map, ucTemp++);
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp); /* In case of wrap-around */
            regs->ucRegA = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xA2: /* LDX - immediate */
            regs->ucRegX = M6502ReadByte(mem_map, PC++);
            M6502FlagsNZ8(regs, regs->ucRegX);
            break;

         case 0xA4: /* LDY z */
            usAddr = M6502ReadByte(mem_map, PC++);
            regs->ucRegY = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegY);
            break;

         case 0xA5: /* LDA z */
            usAddr = M6502ReadByte(mem_map, PC++);
            regs->ucRegA = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xA6: /* LDX z */
            usAddr = M6502ReadByte(mem_map, PC++);
            regs->ucRegX = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegX);
            break;

         case 0xA8: /* TAY */
            regs->ucRegY = regs->ucRegA;
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xA9: /* LDA - immediate */
            regs->ucRegA = M6502ReadByte(mem_map, PC++);
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xAA: /* TAX */
            regs->ucRegX = regs->ucRegA;
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xAC: /* LDY abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            regs->ucRegY = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegY);
            break;

         case 0xAD: /* LDA abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            regs->ucRegA = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xAE: /* LDX abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            regs->ucRegX = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegX);
            break;

         case 0xB0: /* BCS */
            sTemp = (signed short)(signed char)M6502ReadByte(mem_map, PC++);
            if (regs->ucRegP & F_CARRY)
               {
               PC += sTemp;
               *iClocks -= 1;
               }
            break;

         case 0xB1: /* LDA (z),y */
            ucTemp = M6502ReadByte(mem_map, PC++);
            usAddr = M6502ReadByte(mem_map, ucTemp++); /* Account for zp wrap-around */
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp) + regs->ucRegY;
            regs->ucRegA = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xB4: /* LDY z,x */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            regs->ucRegY = M6502ReadByte(mem_map, ucTemp);
            M6502FlagsNZ8(regs, regs->ucRegY);
            break;

         case 0xB5: /* LDA z,x */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            regs->ucRegA = M6502ReadByte(mem_map, ucTemp);
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xB6: /* LDX z,y */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegY);
            regs->ucRegX = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegX);
            break;

         case 0xB8: /* CLV */
            regs->ucRegP &= ~F_OVERFLOW;
            break;

         case 0xB9: /* LDA abs, y */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegY;
            PC += 2;
            regs->ucRegA = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xBA: /* TSX */
            regs->ucRegX = regs->ucRegS;
            M6502FlagsNZ8(regs, regs->ucRegX);
            break;

         case 0xBC: /* LDY abs, x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            regs->ucRegY = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegY);
            break;

         case 0xBD: /* LDA abs, x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            regs->ucRegA = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegA);
            break;

         case 0xBE: /* LDX abs, y */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegY;
            PC += 2;
            regs->ucRegX = M6502ReadByte(mem_map, usAddr);
            M6502FlagsNZ8(regs, regs->ucRegX);
            break;

         case 0xC0: /* CPY - immediate */
            ucTemp = M6502ReadByte(mem_map, PC++);
            M6502CMP(regs, regs->ucRegY, ucTemp);
            break;

         case 0xC1: /* CMP (z,x) */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            usAddr = M6502ReadByte(mem_map, ucTemp++);
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp); /* In case of wrap-around */
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502CMP(regs, regs->ucRegA, ucTemp);
            break;

         case 0xC4: /* CPY z */
            usAddr = M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502CMP(regs, regs->ucRegY, ucTemp);
            break;

         case 0xC5: /* CMP z */
            usAddr = M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502CMP(regs, regs->ucRegA, ucTemp);
            break;

         case 0xC6: /* DEC z */
            usAddr = M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502DEC(regs, ucTemp));
            break;

         case 0xC8: /* INY */
            regs->ucRegY = M6502INC(regs, regs->ucRegY);
            break;

         case 0xC9: /* CMP - immediate */
            ucTemp = M6502ReadByte(mem_map, PC++);
            M6502CMP(regs, regs->ucRegA, ucTemp);
            break;

         case 0xCA: /* DEX */
            regs->ucRegX = M6502DEC(regs, regs->ucRegX);
            break;

         case 0xCC: /* CPY abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502CMP(regs, regs->ucRegY, ucTemp);
            break;

         case 0xCD: /* CMP abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502CMP(regs, regs->ucRegA, ucTemp);
            break;

         case 0xCE: /* DEC abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502DEC(regs, ucTemp));
            break;

         case 0xD0: /* BNE */
            sTemp = (signed short)(signed char)M6502ReadByte(mem_map, PC++);
            if (!(regs->ucRegP & F_ZERO))
               {
               PC += sTemp;
               *iClocks -= 1;
               }
            break;

         case 0xD1: /* CMP (z),y */
            ucTemp = M6502ReadByte(mem_map, PC++);
            usAddr = M6502ReadByte(mem_map, ucTemp++); /* Account for zp wrap-around */
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp) + regs->ucRegY;
            M6502CMP(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0xD5: /* CMP z,x */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            ucTemp = M6502ReadByte(mem_map, ucTemp);
            M6502CMP(regs, regs->ucRegA, ucTemp);
            break;

         case 0xD6: /* DEC z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502DEC(regs, ucTemp));
            break;

         case 0xD8: /* CLD */
            regs->ucRegP &= ~F_DECIMAL;
            break;

         case 0xD9: /* CMP abs,y */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegY;
            PC += 2;
            M6502CMP(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0xDD: /* CMP abs, x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            M6502CMP(regs, regs->ucRegA, M6502ReadByte(mem_map, usAddr));
            break;

         case 0xDE: /* DEC abs,x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502DEC(regs, ucTemp));
            break;

         case 0xE0: /* CPX - immediate */
            ucTemp = M6502ReadByte(mem_map, PC++);
            M6502CMP(regs, regs->ucRegX, ucTemp);
            break;

         case 0xE1: /* SBC (z,x) */
            ucTemp = M6502ReadByte(mem_map, PC++) + regs->ucRegX;
            usAddr = M6502ReadByte(mem_map, ucTemp++);
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp); /* In case of wrap-around */
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502SBC(regs, ucTemp);
            break;

         case 0xE4: /* CPX z */
            usAddr = M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502CMP(regs, regs->ucRegX, ucTemp);
            break;

         case 0xE5: /* SBC z */
            usAddr = M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502SBC(regs, ucTemp);
            break;

         case 0xE6: /* INC z */
            usAddr = M6502ReadByte(mem_map, PC++);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502INC(regs, ucTemp));
            break;

         case 0xE8: /* INX */
            regs->ucRegX = M6502INC(regs, regs->ucRegX);
            break;

         case 0xE9: /* SBC - immediate */
            ucTemp = M6502ReadByte(mem_map, PC++);
            M6502SBC(regs, ucTemp);
            break;

         case 0xEA: /* NOP */
            break;

         case 0xEC: /* CPX abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502CMP(regs, regs->ucRegX, ucTemp);
            break;

         case 0xED: /* SBC abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502SBC(regs, ucTemp);
            break;

         case 0xEE: /* INC abs */
            usAddr = M6502ReadWord(mem_map, PC);
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502INC(regs, ucTemp));
            break;

         case 0xF0: /* BEQ */
            sTemp = (signed short)(signed char)M6502ReadByte(mem_map, PC++);
            if (regs->ucRegP & F_ZERO)
               {
               PC += sTemp;
               *iClocks -= 1;
               }
            break;

         case 0xF1: /* SBC (z),y */
            ucTemp = M6502ReadByte(mem_map, PC++);
            usAddr = M6502ReadByte(mem_map, ucTemp++); /* Account for zp wrap-around */
            usAddr += 256 * M6502ReadByte(mem_map, ucTemp) + regs->ucRegY;
            M6502SBC(regs, M6502ReadByte(mem_map, usAddr));
            break;

         case 0xF5: /* SBC z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502SBC(regs, ucTemp);
            break;

         case 0xF6: /* INC z,x */
            usAddr = (unsigned char)(M6502ReadByte(mem_map, PC++) + regs->ucRegX);
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502INC(regs, ucTemp));
            break;

         case 0xF8: /* SED */
            regs->ucRegP |= F_DECIMAL;
            break;

         case 0xF9: /* SBC abs, y */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegY;
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502SBC(regs, ucTemp);
            break;

         case 0xFD: /* SBC abs, x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502SBC(regs, ucTemp);
            break;

         case 0xFE: /* INC abs, x */
            usAddr = M6502ReadWord(mem_map, PC) + regs->ucRegX;
            PC += 2;
            ucTemp = M6502ReadByte(mem_map, usAddr);
            M6502WriteByte(mem_map, usAddr, M6502INC(regs, ucTemp));
            break;

         default: /* Illegal instruction */
            *iClocks = 0;
            break;
         } /* switch */
      } /* while iClocks */

   regs->usRegPC = PC;

} /* EXEC6502() */

