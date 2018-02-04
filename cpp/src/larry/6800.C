/*******************************************************************/
/* 6800 CPU emulator written by Larry Bank                         */
/* Copyright 1998 BitBank Software, Inc.                           */
/*                                                                 */
/* This code was written from scratch using the 6800 data from     */
/* the Motorola databook "8-BIT MICROPROCESSOR & PERIPHERAL DATA". */
/*                                                                 */
/* Change history:                                                 */
/* 1/22/98 Wrote it - Larry B.                                     */
/*******************************************************************/
#include <windows.h>
#include <string.h>
#include "emu.h"

#define F_CARRY     1
#define F_OVERFLOW  2
#define F_ZERO      4
#define F_NEGATIVE  8
#define F_IRQMASK   16
#define F_HALFCARRY 32

#define SET_V8(a,b,r) regs->ucRegCC |= (((a^b^r^(r>>1))&0x80)>>6)
#define SET_V16(a,b,r) regs->ucRegCC |= (((a^b^r^(r>>1))&0x8000)>>14)

void TRACE6800(REGS6800 *);
//#define TRACE

/* Some statics */
EMUHANDLERS *mem_handlers;
unsigned char *m_map;
/* Negative and zero flags for quicker flag settings */
unsigned char c6800NZ[256]={
      4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 00-0F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 10-1F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 20-2F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 30-3F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 40-4F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 50-5F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 60-6F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 70-7F */
      8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,          /* 80-8F */
      8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,          /* 90-9F */
      8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,          /* A0-AF */
      8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,          /* B0-BF */
      8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,          /* C0-CF */
      8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,          /* D0-DF */
      8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,          /* E0-EF */
      8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8};         /* F0-FF */

unsigned char M6800ReadByte(unsigned char *, unsigned short);
unsigned short M6800ReadWord(unsigned char *, unsigned short);
__inline void M6800PUSHB(unsigned char *, REGS6800 *, unsigned char);
__inline void M6800PUSHW(unsigned char *, REGS6800 *, unsigned short);
__inline unsigned char M6800PULLB(unsigned char *, REGS6800 *);
__inline unsigned short M6800PULLW(unsigned char *, REGS6800 *);

char c6800Cycles[256] = {0,2,0,0,0,0,2,2,4,4,2,2,2,2,2,2,   /* 00-0F */
                         2,2,0,0,0,0,2,2,0,2,0,2,0,0,0,0,   /* 10-1F */
                         4,0,4,4,4,4,4,4,4,4,4,4,4,4,4,4,   /* 20-2F */
                         4,4,4,4,4,4,4,4,0,5,0,10,0,0,9,12, /* 30-3F */
                         2,0,0,2,2,0,2,2,2,2,2,0,2,2,0,2,   /* 40-4F */
                         2,0,0,2,2,0,2,2,2,2,2,0,2,2,0,2,   /* 50-5F */
                         7,0,0,7,7,0,7,7,7,7,7,0,7,7,4,7,   /* 60-6F */
                         6,0,0,6,6,0,6,6,6,6,6,0,6,6,3,6,   /* 70-7F */
                         2,2,2,0,2,2,2,0,2,2,2,2,3,8,3,0,   /* 80-8F */
                         3,3,3,0,3,3,3,4,3,3,3,3,4,0,4,5,   /* 90-9F */
                         5,5,5,0,5,5,5,6,5,5,5,5,6,8,6,7,   /* A0-AF */
                         4,4,4,0,4,4,4,5,4,4,4,4,5,9,5,6,   /* B0-BF */
                         2,2,2,0,2,2,2,0,2,2,2,2,0,0,3,0,   /* C0-CF */
                         3,3,3,0,3,3,3,4,3,3,3,3,0,0,4,5,   /* D0-DF */
                         5,5,5,0,5,5,5,6,5,5,5,5,4,0,6,7,   /* E0-EF */
                         4,4,4,0,4,4,4,5,4,4,4,4,0,0,5,6};  /* F0-FF */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : RESET6800(char *, REGS6800 *)                              *
 *                                                                          *
 *  PURPOSE    : Get the 6800 after a reset.                                *
 *                                                                          *
 ****************************************************************************/
void RESET6800(char *mem, REGS6800 *regs)
{
   m_map = mem;
   memset(regs, 0, sizeof(REGS6800)); /* Start with a clean slate at reset */
   regs->usRegPC = m_map[MEM_ROM+0xfffe] * 256 + m_map[MEM_ROM+0xffff]; /* Start execution at reset vector */
   regs->ucRegCC = F_IRQMASK; /* Start with IRQ disabled */

} /* RESET6800() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800INC(REGS6800 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform an increment and update flags.                     *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800INC(REGS6800 *regs, unsigned char ucByte)
{
   ucByte++;
   regs->ucRegCC &= ~(F_ZERO | F_OVERFLOW | F_NEGATIVE);
   regs->ucRegCC |= c6800NZ[ucByte];
   if (ucByte == 0x80)
      regs->ucRegCC |= F_OVERFLOW;
   return ucByte;

} /* M6800INC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800DEC(REGS6800 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a decrement and update flags.                      *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800DEC(REGS6800 *regs, unsigned char ucByte)
{
   ucByte--;
   regs->ucRegCC &= ~(F_ZERO | F_OVERFLOW | F_NEGATIVE);
   regs->ucRegCC |= c6800NZ[ucByte];
   if (ucByte == 0x7f)
      regs->ucRegCC |= F_OVERFLOW;
   return ucByte;

} /* M6800DEC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800SUB(REGS6800 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform a 8-bit subtraction and update flags.              *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800SUB(REGS6800 *regs, unsigned char ucByte1, unsigned char ucByte2)
{
register unsigned short sTemp;
   sTemp = (unsigned short)ucByte1 - (unsigned short)ucByte2;
   regs->ucRegCC &= ~(F_ZERO | F_CARRY | F_OVERFLOW | F_NEGATIVE);
   regs->ucRegCC |= c6800NZ[sTemp & 0xff];
   if (sTemp & 0x100)
       regs->ucRegCC |= F_CARRY;
   SET_V8(ucByte1, ucByte2, sTemp);
//   if ((sTemp ^ ucByte1 ^ ucByte2 ^ (sTemp>>1)) & 0x80)
//      regs->ucRegCC |= F_OVERFLOW;
   return (unsigned char)(sTemp & 0xff);

} /* M6800SUB() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800ADD(REGS6800 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform a 8-bit addition and update flags.                 *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800ADD(REGS6800 *regs, unsigned char ucByte1, unsigned char ucByte2)
{
register unsigned short sTemp;
   sTemp = (unsigned short)ucByte1 + (unsigned short)ucByte2;
   regs->ucRegCC &= ~(F_HALFCARRY | F_CARRY | F_OVERFLOW | F_NEGATIVE | F_ZERO);
   regs->ucRegCC |= c6800NZ[sTemp & 0xff];
   if (sTemp & 0x100)
       regs->ucRegCC |= F_CARRY;
   SET_V8(ucByte1, ucByte2, sTemp);
//   if ((sTemp ^ ucByte1 ^ ucByte2 ^ (sTemp>>1)) & 0x80)
//      regs->ucRegCC |= F_OVERFLOW;
   if ((sTemp ^ ucByte1 ^ ucByte2) & 0x10)
      regs->ucRegCC |= F_HALFCARRY;
   return (unsigned char)(sTemp & 0xff);

} /* M6800ADD() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800ADC(REGS6800 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform a 8-bit addition with carry.                       *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800ADC(REGS6800 *regs, unsigned char ucByte1, unsigned char ucByte2)
{
register unsigned short sTemp;
   sTemp = (unsigned short)ucByte1 + (unsigned short)ucByte2 + (regs->ucRegCC & 1);
   regs->ucRegCC &= ~(F_HALFCARRY | F_ZERO | F_CARRY | F_OVERFLOW | F_NEGATIVE);
   regs->ucRegCC |= c6800NZ[sTemp & 0xff];
   if (sTemp & 0x100)
       regs->ucRegCC |= F_CARRY;
   SET_V8(ucByte1, ucByte2, sTemp);
//   if ((sTemp ^ ucByte1 ^ ucByte2 ^ (sTemp>>1)) & 0x80)
//      regs->ucRegCC |= F_OVERFLOW;
   if ((sTemp ^ ucByte1 ^ ucByte2) & 0x10)
      regs->ucRegCC |= F_HALFCARRY;
   return (unsigned char)(sTemp & 0xff);

} /* M6800ADC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800SBC(REGS6800 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform a 8-bit subtraction with carry and update flags.   *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800SBC(REGS6800 *regs, unsigned char ucByte1, unsigned char ucByte2)
{
register unsigned short sTemp;
   sTemp = (unsigned short)ucByte1 - (unsigned short)ucByte2 - (regs->ucRegCC & 1);
   regs->ucRegCC &= ~(F_ZERO | F_CARRY | F_OVERFLOW | F_NEGATIVE);
   regs->ucRegCC |= c6800NZ[sTemp & 0xff];
   if (sTemp & 0x100)
       regs->ucRegCC |= F_CARRY;
   SET_V8(ucByte1, ucByte2, sTemp);
//   if ((sTemp ^ ucByte1 ^ ucByte2 ^ (sTemp>>1)) & 0x80)
//      regs->ucRegCC |= F_OVERFLOW;
   return (unsigned char)(sTemp & 0xff);

} /* M6800SBC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800CMP(REGS6800 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform a 8-bit comparison.                                *
 *                                                                          *
 ****************************************************************************/
__inline void M6800CMP(REGS6800 *regs, unsigned char ucByte1, unsigned char ucByte2)
{
register unsigned short sTemp;
   sTemp = (unsigned short)ucByte1 - (unsigned short)ucByte2;
   regs->ucRegCC &= ~(F_ZERO | F_CARRY | F_OVERFLOW | F_NEGATIVE);
   regs->ucRegCC |= c6800NZ[sTemp & 0xff];
   if (sTemp & 0x100)
       regs->ucRegCC |= F_CARRY;
//   if ((sTemp ^ ucByte1 ^ ucByte2 ^ (sTemp>>1)) & 0x80)
//      regs->ucRegCC |= F_OVERFLOW;
   SET_V8(ucByte1, ucByte2, sTemp);

} /* M6800CMP() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800CMP16(REGS6800 *, char, char)                         *
 *                                                                          *
 *  PURPOSE    : Perform a 16-bit comparison.                               *
 *                                                                          *
 ****************************************************************************/
__inline void M6800CMP16(REGS6800 *regs, unsigned short usWord1, unsigned short usWord2)
{
register unsigned long lTemp;
   lTemp = (unsigned long)usWord1 - (unsigned long)usWord2;
   regs->ucRegCC &= ~(F_ZERO | F_CARRY | F_OVERFLOW | F_NEGATIVE);
   if (lTemp == 0)
      regs->ucRegCC |= F_ZERO;
   if (lTemp & 0x8000)
       regs->ucRegCC |= F_NEGATIVE;
   if (lTemp & 0x10000)
       regs->ucRegCC |= F_CARRY;
   SET_V16(usWord1, usWord2, lTemp);
//   if ((lTemp ^ usWord1 ^ usWord2 ^ (lTemp>>1)) & 0x8000)
//      regs->ucRegCC |= F_OVERFLOW;

} /* M6800CMP16() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800LSR(REGS6800 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a logical shift right and update flags.            *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800LSR(REGS6800 *regs, unsigned char ucByte)
{

   regs->ucRegCC &= ~(F_ZERO | F_CARRY | F_NEGATIVE | F_OVERFLOW);
   if (ucByte & 0x01)
      regs->ucRegCC |= (F_CARRY + F_OVERFLOW); /* funny rule */
   ucByte >>= 1;
   if (ucByte == 0)
      regs->ucRegCC |= F_ZERO;
   return ucByte;

} /* M6800LSR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800ASR(REGS6800 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a arithmetic shift right and update flags.         *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800ASR(REGS6800 *regs, unsigned char ucByte)
{

   regs->ucRegCC &= ~(F_ZERO | F_CARRY | F_NEGATIVE | F_OVERFLOW);
   if (ucByte & 0x01)
      regs->ucRegCC |= F_CARRY;
   ucByte = ucByte & 0x80 | ucByte >>1;
   regs->ucRegCC |= c6800NZ[ucByte];
   if ((regs->ucRegCC & F_NEGATIVE) ^ ((regs->ucRegCC & F_CARRY)<< 3))
      regs->ucRegCC |= F_OVERFLOW; /* funny rule */

   return ucByte;

} /* M6800ASR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800ASL(REGS6800 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a arithmetic shift left and update flags.          *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800ASL(REGS6800 *regs, unsigned char ucByte)
{
unsigned short usOld = (unsigned short)ucByte;

   regs->ucRegCC &= ~(F_ZERO | F_CARRY | F_OVERFLOW | F_NEGATIVE);
   if (ucByte & 0x80)
      regs->ucRegCC |= F_CARRY;
   ucByte <<=1;
   regs->ucRegCC |= c6800NZ[ucByte];
   if ((regs->ucRegCC & F_NEGATIVE) ^ ((regs->ucRegCC & F_CARRY)<< 3))
      regs->ucRegCC |= F_OVERFLOW; /* funny rule */
   return ucByte;

} /* M6800ASL() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800ROL(REGS6800 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a rotate left and update flags.                    *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800ROL(REGS6800 *regs, unsigned char ucByte)
{
unsigned char ucOld = ucByte;
unsigned char uc;

   uc = regs->ucRegCC & 1; /* Preserve old carry flag */
   regs->ucRegCC &= ~(F_ZERO | F_CARRY | F_OVERFLOW | F_NEGATIVE);
   if (ucByte & 0x80)
      regs->ucRegCC |= F_CARRY;
   ucByte = ucByte <<1 | uc;
   regs->ucRegCC |= c6800NZ[ucByte];
   if ((regs->ucRegCC & F_NEGATIVE) ^ ((regs->ucRegCC & F_CARRY)<< 3))
      regs->ucRegCC |= F_OVERFLOW; /* funny rule */
   return ucByte;

} /* M6800ROL() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800EOR(REGS6800 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform an exclusive or and update flags.                  *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800EOR(REGS6800 *regs, unsigned char ucByte1, char ucByte2)
{
register unsigned char ucTemp;

   regs->ucRegCC &= ~(F_ZERO | F_OVERFLOW | F_NEGATIVE);
   ucTemp = ucByte1 ^ ucByte2;
   regs->ucRegCC |= c6800NZ[ucTemp];
   return ucTemp;

} /* M6800EOR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800OR(REGS6800 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform an inclusive or and update flags.                  *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800OR(REGS6800 *regs, unsigned char ucByte1, char ucByte2)
{
register unsigned char ucTemp;

   regs->ucRegCC &= ~(F_ZERO | F_OVERFLOW | F_NEGATIVE);
   ucTemp = ucByte1 | ucByte2;
   regs->ucRegCC |= c6800NZ[ucTemp];
   return ucTemp;

} /* M6800OR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800AND(REGS6800 *, char, char)                           *
 *                                                                          *
 *  PURPOSE    : Perform an AND and update flags.                           *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800AND(REGS6800 *regs, unsigned char ucByte1, char ucByte2)
{
register unsigned char ucTemp;

   regs->ucRegCC &= ~(F_ZERO | F_OVERFLOW | F_NEGATIVE);
   ucTemp = ucByte1 & ucByte2;
   regs->ucRegCC |= c6800NZ[ucTemp];
   return ucTemp;

} /* M6800AND() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800COM(REGS6800 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a 1's complement and update flags.                 *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800COM(REGS6800 *regs, unsigned char ucByte)
{

   regs->ucRegCC &= ~(F_ZERO | F_OVERFLOW | F_NEGATIVE);
   regs->ucRegCC |= F_CARRY;
   ucByte = ~ucByte;
   regs->ucRegCC |= c6800NZ[ucByte];
   return ucByte;

} /* M6800COM() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800NEG(REGS6800 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a 2's complement and update flags.                 *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800NEG(REGS6800 *regs, unsigned char ucByte)
{

   regs->ucRegCC &= ~(F_CARRY | F_ZERO | F_OVERFLOW | F_NEGATIVE);
   ucByte = ~ucByte + 1;
   if (ucByte == 0x80)      /* Some strange flag rules */
      regs->ucRegCC |= F_OVERFLOW;
   if (ucByte == 0)
      regs->ucRegCC |= (F_ZERO + F_CARRY);
   if (ucByte & 0x80)
      regs->ucRegCC |= F_NEGATIVE;
   return ucByte;

} /* M6800NEG() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800ROR(REGS6800 *, char)                                 *
 *                                                                          *
 *  PURPOSE    : Perform a rotate right and update flags.                   *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800ROR(REGS6800 *regs, unsigned char ucByte)
{
unsigned char uc;

   uc = regs->ucRegCC & 1; /* Preserve old carry flag */
   regs->ucRegCC &= ~(F_ZERO | F_CARRY | F_NEGATIVE | F_OVERFLOW);
   if (ucByte & 0x01)
      regs->ucRegCC |= F_CARRY;
   ucByte = ucByte >> 1 | uc << 7;
   regs->ucRegCC |= c6800NZ[ucByte];
   if ((regs->ucRegCC & F_NEGATIVE) ^ ((regs->ucRegCC & F_CARRY)<< 3))
      regs->ucRegCC |= F_OVERFLOW; /* funny rule */
   return ucByte;

} /* M6800ROR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800WriteByte(char *, short, char)                        *
 *                                                                          *
 *  PURPOSE    : Write a byte to memory, check for hardware.                *
 *                                                                          *
 ****************************************************************************/
__inline void M6800WriteByte(unsigned char *m_map, unsigned short usAddr, unsigned char ucByte)
{
unsigned char c;

   switch(c = m_map[usAddr+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         m_map[usAddr+MEM_RAM] = ucByte;
         break;
      case 1: /* Normal ROM - nothing to do */
         break;
      default: /* Call special handler routine for this address */
         (mem_handlers[c-2].pfn_write)(usAddr, ucByte);
         break;
      }

} /* M6800WriteByte() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800WriteWord(char *, short, short)                       *
 *                                                                          *
 *  PURPOSE    : Write a word to memory, check for hardware.                *
 *                                                                          *
 ****************************************************************************/
__inline void M6800WriteWord(unsigned char *m_map, unsigned short usAddr, unsigned short usWord)
{
unsigned char c;

   switch(c = m_map[usAddr+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         m_map[usAddr+MEM_RAM] = usWord >> 8;
         break;
      case 1: /* Normal ROM - nothing to do */
         break;
      default: /* Call special handler routine for this address */
         (mem_handlers[c-2].pfn_write)(usAddr, (unsigned char)(usWord >> 8));
         break;
      }
   usAddr++;
   switch(c = m_map[usAddr+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         m_map[usAddr+MEM_RAM] = usWord & 0xff;
         break;
      case 1: /* Normal ROM - nothing to do */
         break;
      default: /* Call special handler routine for this address */
         (mem_handlers[c-2].pfn_write)(usAddr, (unsigned char)(usWord & 0xff));
         break;
      }

} /* M6800WriteWord() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800FlagsNZ16(M6800REGS *, short)                         *
 *                                                                          *
 *  PURPOSE    : Set appropriate flags for 16 bit value.                    *
 *                                                                          *
 ****************************************************************************/
__inline void M6800FlagsNZ16(REGS6800 *regs, unsigned short usWord)
{
    regs->ucRegCC &= ~(F_ZERO | F_NEGATIVE);
    if (usWord == 0)
       regs->ucRegCC |= F_ZERO;
    if (usWord & 0x8000)
       regs->ucRegCC |= F_NEGATIVE;

} /* M6800FlagsNZ16() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800ReadByte(char *, short)                               *
 *                                                                          *
 *  PURPOSE    : Read a byte from memory, check for hardware.               *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800ReadByte(unsigned char *m_map, unsigned short usAddr)
{
unsigned char c;
   switch(c = m_map[usAddr+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         return m_map[usAddr+MEM_RAM]; /* Just return it */
         break;
      case 1: /* Normal ROM */
         return m_map[usAddr+MEM_ROM]; /* Just return it */
         break;
      default: /* Call special handler routine for this address */
         return (mem_handlers[c-2].pfn_read)(usAddr);
         break;
      }

} /* M6800ReadByte() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800ReadWord(char *, short)                               *
 *                                                                          *
 *  PURPOSE    : Read a word from memory, check for hardware.               *
 *                                                                          *
 ****************************************************************************/
__inline unsigned short M6800ReadWord(unsigned char *m_map, unsigned short usAddr)
{
unsigned short usWord;
unsigned char c;
/* Re-code this to make the compliler inline it better */
   switch(c = m_map[usAddr+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         usWord = m_map[usAddr+MEM_RAM] << 8;
         break;
      case 1: /* Normal ROM */
         usWord = m_map[usAddr+MEM_ROM] << 8;
         break;
      default: /* Call special handler routine for this address */
         usWord = (mem_handlers[c-2].pfn_read)(usAddr) << 8;
         break;
      }
   usAddr++;
/* Check flags again in case someone is being tricky */
   switch(c = m_map[usAddr+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         usWord += m_map[usAddr+MEM_RAM];
         break;
      case 1: /* Normal ROM */
         usWord += m_map[usAddr+MEM_ROM];
         break;
      default: /* Call special handler routine for this address */
         usWord += (mem_handlers[c-2].pfn_read)(usAddr);
         break;
      }
   return usWord;

} /* M6800ReadWord() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800PUSHB(char *, REGS6800 *, char)                       *
 *                                                                          *
 *  PURPOSE    : Push a byte to the 'S' stack.                              *
 *                                                                          *
 ****************************************************************************/
__inline void M6800PUSHB(unsigned char *m_map, REGS6800 *regs, unsigned char ucByte)
{
unsigned char c;

   switch(c = m_map[--regs->usRegS + MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         m_map[regs->usRegS + MEM_RAM] = ucByte;
         break;
      case 1: /* Normal ROM - nothing to do */
         break;
      default: /* Call special handler routine for this address */
         (mem_handlers[c-2].pfn_write)(regs->usRegS, ucByte);
         break;
      }

} /* M6800PUSHB() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800PUSHW(char *, REGS6800 *)                             *
 *                                                                          *
 *  PURPOSE    : Push a word to the 'S' stack.                              *
 *                                                                          *
 ****************************************************************************/
__inline void M6800PUSHW(unsigned char *m_map, REGS6800 *regs, unsigned short usWord)
{

unsigned char c;

   switch(c = m_map[--regs->usRegS + MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         m_map[regs->usRegS + MEM_RAM] = usWord & 0xff;
         break;
      case 1: /* Normal ROM - nothing to do */
         break;
      default: /* Call special handler routine for this address */
         (mem_handlers[c-2].pfn_write)(regs->usRegS, (unsigned char)(usWord & 0xff));
         break;
      }
/* Check the flags again in case someone is trying to be tricky */
   switch(c = m_map[--regs->usRegS + MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         m_map[regs->usRegS + MEM_RAM] = usWord >> 8;
         break;
      case 1: /* Normal ROM - nothing to do */
         break;
      default: /* Call special handler routine for this address */
         (mem_handlers[c-2].pfn_write)(regs->usRegS, (unsigned char)(usWord >> 8));
         break;
      }

} /* M6800PUSHW() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800PULLB(char *, REGS6800 *)                             *
 *                                                                          *
 *  PURPOSE    : Pull a byte from the 'S' stack.                            *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char M6800PULLB(unsigned char *m_map, REGS6800 *regs)
{
unsigned char c;
   switch(c = m_map[regs->usRegS+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         return m_map[regs->usRegS++ + MEM_RAM]; /* Just return it */
         break;
      case 1: /* Normal ROM */
         return m_map[regs->usRegS++ + MEM_ROM]; /* Just return it */
         break;
      default: /* Call special handler routine for this address */
         return (mem_handlers[c-2].pfn_read)(regs->usRegS++);
         break;
      }

} /* M6800PULLB() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : M6800PULLW(char *, REGS6800 *)                             *
 *                                                                          *
 *  PURPOSE    : Pull a word from the 'S' stack.                            *
 *                                                                          *
 ****************************************************************************/
__inline unsigned short M6800PULLW(unsigned char *m_map, REGS6800 *regs)
{
unsigned char c, hi, lo;

   switch(c = m_map[regs->usRegS+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         hi = m_map[regs->usRegS++ + MEM_RAM]; /* Just return it */
         break;
      case 1: /* Normal ROM */
         hi = m_map[regs->usRegS++ + MEM_ROM]; /* Just return it */
         break;
      default: /* Call special handler routine for this address */
         hi = (mem_handlers[c-2].pfn_read)(regs->usRegS++);
         break;
      }
/* I'll check the flag again in case someone is trying to be tricky */
   switch(c = m_map[regs->usRegS+MEM_FLAGS]) /* If special flag (ROM or hardware) */
      {
      case 0: /* Normal RAM */
         lo = m_map[regs->usRegS++ + MEM_RAM]; /* Just return it */
         break;
      case 1: /* Normal ROM */
         lo = m_map[regs->usRegS++ + MEM_ROM]; /* Just return it */
         break;
      default: /* Call special handler routine for this address */
         lo = (mem_handlers[c-2].pfn_read)(regs->usRegS++);
         break;
      }
   return (hi * 256 + lo);

} /* M6800PULLW() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : EXEC6800(char *, REGS6800 *, EMUHANDLERS *, int *, char *) *
 *                                                                          *
 *  PURPOSE    : Emulate the M6800 microprocessor for N clock cycles.       *
 *                                                                          *
 ****************************************************************************/
void EXEC6800(char *mem, REGS6800 *regs, EMUHANDLERS *emuh, int *iClocks, unsigned char *ucIRQs)
{
unsigned short PC;  /* Current Program Counter address */
register unsigned short usAddr; /* Temp address */
register unsigned char ucTemp;
register unsigned short usTemp;
register signed short sTemp;

   mem_handlers = emuh; /* Assign to static for faster execution */
   m_map = mem; /* ditto */

   PC = regs->usRegPC;
   while (*iClocks > 0) /* Execute for the amount of time alloted */
      {
      /*--- First check for any pending IRQs ---*/
checkpending:
      if (*ucIRQs)
         {
         if (*ucIRQs & INT_NMI) /* NMI is highest priority */
            {
            M6800PUSHW(m_map, regs, PC);
            M6800PUSHW(m_map, regs, regs->usRegX);
            M6800PUSHB(m_map, regs, regs->ucRegA);
            M6800PUSHB(m_map, regs, regs->ucRegB);
            M6800PUSHB(m_map, regs, regs->ucRegCC);
            regs->ucRegCC |= F_IRQMASK; /* Mask interrupts during service routine */
            *iClocks -= 19;
            PC = M6800ReadWord(m_map, 0xfffc);
            *ucIRQs &= ~INT_NMI; /* clear this bit */
            goto doexecute;
            }
         if (*ucIRQs & INT_IRQ && (regs->ucRegCC & F_IRQMASK) == 0) /* IRQ is lowest priority */
            {
            M6800PUSHW(m_map, regs, PC);
            M6800PUSHW(m_map, regs, regs->usRegX);
            M6800PUSHB(m_map, regs, regs->ucRegA);
            M6800PUSHB(m_map, regs, regs->ucRegB);
            M6800PUSHB(m_map, regs, regs->ucRegCC);
            regs->ucRegCC |= F_IRQMASK; /* Mask interrupts during service routine */
            PC = M6800ReadWord(m_map, 0xfff8);
            *ucIRQs &= ~INT_IRQ; /* clear this bit */
            *iClocks -= 19;
            goto doexecute;
            }
         }
doexecute:
#ifdef TRACE
      regs->usRegPC = PC;
      TRACE6800(regs);
#endif
      ucTemp = M6800ReadByte(m_map, PC++);
      *iClocks -= c6800Cycles[ucTemp];
      switch (ucTemp)
         {
         case 0x01: /* NOP */
            break;

         case 0x06: /* TAP - transfer A to flags */
            regs->ucRegCC = regs->ucRegA;
            break;

         case 0x07: /* TPA - transfer flags to A */
            regs->ucRegA = regs->ucRegCC;
            break;

         case 0x08: /* INX - increment index */
            regs->usRegX++;
            regs->ucRegCC &= ~F_ZERO;
            if (regs->usRegX == 0)
               regs->ucRegCC |= F_ZERO;
            break;

         case 0x09: /* DEX - decrement index register */
            regs->usRegX--;
            regs->ucRegCC &= ~F_ZERO;
            if (regs->usRegX == 0)
               regs->ucRegCC |= F_ZERO;
            break;

         case 0x0A: /* CLV - Clear overflow flag */
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0x0B: /* SEV - Set overflow flag */
            regs->ucRegCC |= F_OVERFLOW;
            break;

         case 0x0C: /* CLC - Clear carry flag */
            regs->ucRegCC &= ~F_CARRY;
            break;

         case 0x0D: /* SEC - Set carry flag */
            regs->ucRegCC |= F_CARRY;
            break;

         case 0x0E: /* CLI - clear interrupt flag */
            regs->ucRegCC &= ~F_IRQMASK;
            break;

         case 0x0F: /* SEI - set interrupt flag */
            regs->ucRegCC |= F_IRQMASK;
            break;

         case 0x10: /* SBA - subtract B from A */
            regs->ucRegA = M6800SUB(regs, regs->ucRegA, regs->ucRegB);
            break;

         case 0x11: /* CBA - compare B to A */
            M6800CMP(regs, regs->ucRegA, regs->ucRegB);
            break;

         case 0x16: /* TAB - transfer A to B */
            regs->ucRegB = regs->ucRegA;
            regs->ucRegCC &= ~(F_OVERFLOW | F_ZERO | F_NEGATIVE);
            regs->ucRegCC |= c6800NZ[regs->ucRegB];
            break;

         case 0x17: /* TBA - transfer B to A */
            regs->ucRegA = regs->ucRegB;
            regs->ucRegCC &= ~(F_OVERFLOW | F_ZERO | F_NEGATIVE);
            regs->ucRegCC |= c6800NZ[regs->ucRegA];
            break;

         case 0x19: /* DAA */
            {
            unsigned char msn, lsn;
            unsigned short cf = 0;
            msn=regs->ucRegA & 0xf0; lsn=regs->ucRegA & 0x0f;
            if( lsn>0x09 || regs->ucRegCC&0x20 ) cf |= 0x06;
            if( msn>0x80 && lsn>0x09 ) cf |= 0x60;
            if( msn>0x90 || regs->ucRegCC&0x01 ) cf |= 0x60;
            usTemp = cf + regs->ucRegA;
            regs->ucRegCC &= ~(F_CARRY | F_NEGATIVE | F_ZERO | F_OVERFLOW);
            if (usTemp & 0x100)
               regs->ucRegCC |= F_CARRY;
            SET_V8(regs->ucRegA, cf, usTemp);
            regs->ucRegA = (unsigned char)usTemp;
            regs->ucRegCC |= c6800NZ[usTemp & 0xff];
            }
            break;

         case 0x1B: /* ABA - add B to A */
            regs->ucRegA = M6800ADD(regs, regs->ucRegA, regs->ucRegB);
            break;

         /* Relative conditional branches */
         case 0x20: /* BRA */
            PC += (signed short)(signed char)M6800ReadByte(m_map, PC++);
            break;

         case 0x21: /* BRN - a two byte NOP */
            PC++;
            break;
         case 0x22: /* BHI */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (!(regs->ucRegCC & (F_CARRY | F_ZERO)))
               PC += sTemp;
            break;

         case 0x23: /* BLS */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (regs->ucRegCC & (F_CARRY | F_ZERO))
               PC += sTemp;
            break;

         case 0x24: /* BCC */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (!(regs->ucRegCC & F_CARRY))
               PC += sTemp;
            break;

         case 0x25: /* BCS */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (regs->ucRegCC & F_CARRY)
               PC += sTemp;
            break;

         case 0x26: /* BNE */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (!(regs->ucRegCC & F_ZERO))
               PC += sTemp;
            break;

         case 0x27: /* BEQ */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (regs->ucRegCC & F_ZERO)
               PC += sTemp;
            break;

         case 0x28: /* BVC */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (!(regs->ucRegCC & F_OVERFLOW))
               PC += sTemp;
            break;

         case 0x29: /* BVS */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (regs->ucRegCC & F_OVERFLOW)
               PC += sTemp;
            break;

         case 0x2A: /* BPL */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (!(regs->ucRegCC & F_NEGATIVE))
               PC += sTemp;
            break;

         case 0x2B: /* BMI */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (regs->ucRegCC & F_NEGATIVE)
               PC += sTemp;
            break;

         case 0x2C: /* BGE */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (!((regs->ucRegCC & F_NEGATIVE) ^ (regs->ucRegCC & F_OVERFLOW)<<2))
               PC += sTemp;
            break;

         case 0x2D: /* BLT */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if ((regs->ucRegCC & F_NEGATIVE) ^ (regs->ucRegCC & F_OVERFLOW)<<2)
               PC += sTemp;
            break;

         case 0x2E: /* BGT */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if (!((regs->ucRegCC & F_NEGATIVE) ^ (regs->ucRegCC & F_OVERFLOW)<<2 || regs->ucRegCC & F_ZERO))
               PC += sTemp;
            break;

         case 0x2F: /* BLE */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            if ((regs->ucRegCC & F_NEGATIVE) ^ (regs->ucRegCC & F_OVERFLOW)<<2 || regs->ucRegCC & F_ZERO)
               PC += sTemp;
            break;

         case 0x30: /* TSX - transfer stack to X */
            regs->usRegX = regs->usRegS + 1;
            break;

         case 0x31: /* INS - increment stack pointer */
            regs->usRegS++;
            break;

         case 0x32: /* PULA */
            regs->ucRegA = M6800PULLB(m_map, regs);
            break;

         case 0x33: /* PULB */
            regs->ucRegB = M6800PULLB(m_map, regs);
            break;

         case 0x34: /* DES - decrement stack pointer */
            regs->usRegS--;
            break;

         case 0x35: /* TXS - transfer X to stack */
            regs->usRegS = regs->usRegX - 1;
            break;

         case 0x36: /* PSHA */
            M6800PUSHB(m_map, regs, regs->ucRegA);
            break;

         case 0x37: /* PSHB */
            M6800PUSHB(m_map, regs, regs->ucRegB);
            break;

         case 0x39: /* RTS */
            PC = M6800PULLW(m_map, regs);
            break;

         case 0x3B: /* RTI */
            regs->ucRegCC = M6800PULLB(m_map, regs);
            regs->ucRegB = M6800PULLB(m_map, regs);
            regs->ucRegA = M6800PULLB(m_map, regs);
            regs->usRegX = M6800PULLW(m_map, regs);
            PC = M6800PULLW(m_map, regs);
            break;

         case 0x3E: /* WAI */
            if (*ucIRQs) /* If there are pending interrupts, handle them */
               goto checkpending;
            else /* otherwise, make sure that the next time through */
               { /* it does not advance to the next instruction */
               PC--;
               *iClocks = 0; /* exit until next interrupt */
               }
            break;

         case 0x3F: /* SWI */
            M6800PUSHW(m_map, regs, PC);
            M6800PUSHW(m_map, regs, regs->usRegX);
            M6800PUSHB(m_map, regs, regs->ucRegA);
            M6800PUSHB(m_map, regs, regs->ucRegB);
            M6800PUSHB(m_map, regs, regs->ucRegCC);
            regs->ucRegCC |= F_IRQMASK;  /* Disable further interrupts */
            PC = M6800ReadWord(m_map, 0xfffa);
            break;

         case 0x40: /* NEGA */
            regs->ucRegA = M6800NEG(regs, regs->ucRegA);
            break;

         case 0x43: /* COMA */
            regs->ucRegA = M6800COM(regs, regs->ucRegA);
            break;

         case 0x44: /* LSRA */
            regs->ucRegA = M6800LSR(regs, regs->ucRegA);
            break;

         case 0x46: /* RORA */
            regs->ucRegA = M6800ROR(regs, regs->ucRegA);
            break;

         case 0x47: /* ASRA */
            regs->ucRegA = M6800ASR(regs, regs->ucRegA);
            break;

         case 0x48: /* ASLA */
            regs->ucRegA = M6800ASL(regs, regs->ucRegA);
            break;

         case 0x49: /* ROLA */
            regs->ucRegA = M6800ROL(regs, regs->ucRegA);
            break;

         case 0x4A: /* DECA */
            regs->ucRegA = M6800DEC(regs, regs->ucRegA);
            break;

         case 0x4C: /* INCA */
            regs->ucRegA = M6800INC(regs, regs->ucRegA);
            break;

         case 0x4D: /* TSTA */
            regs->ucRegCC &= ~(F_OVERFLOW | F_ZERO | F_NEGATIVE | F_CARRY);
            regs->ucRegCC |= c6800NZ[regs->ucRegA];
            break;

         case 0x4F: /* CLRA */
            regs->ucRegA = 0;
            regs->ucRegCC &= ~(F_NEGATIVE | F_OVERFLOW | F_CARRY);
            regs->ucRegCC |= F_ZERO;
            break;

         case 0x50: /* NEGB */
            regs->ucRegB = M6800NEG(regs, regs->ucRegB);
            break;

         case 0x53: /* COMB */
            regs->ucRegB = M6800COM(regs, regs->ucRegB);
            break;

         case 0x54: /* LSRB */
            regs->ucRegB = M6800LSR(regs, regs->ucRegB);
            break;

         case 0x56: /* RORB */
            regs->ucRegB = M6800ROR(regs, regs->ucRegB);
            break;

         case 0x57: /* ASRB */
            regs->ucRegB = M6800ASR(regs, regs->ucRegB);
            break;

         case 0x58: /* ASLB */
            regs->ucRegB = M6800ASL(regs, regs->ucRegB);
            break;

         case 0x59: /* ROLB */
            regs->ucRegB = M6800ROL(regs, regs->ucRegB);
            break;

         case 0x5A: /* DECB */
            regs->ucRegB = M6800DEC(regs, regs->ucRegB);
            break;

         case 0x5C: /* INCB */
            regs->ucRegB = M6800INC(regs, regs->ucRegB);
            break;

         case 0x5D: /* TSTB */
            regs->ucRegCC &= ~(F_OVERFLOW | F_ZERO | F_NEGATIVE | F_CARRY);
            regs->ucRegCC |= c6800NZ[regs->ucRegB];
            break;

         case 0x5F: /* CLRB */
            regs->ucRegB = 0;
            regs->ucRegCC &= ~(F_NEGATIVE | F_OVERFLOW | F_CARRY);
            regs->ucRegCC |= F_ZERO;
            break;

         case 0x60: /* NEG - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800WriteByte(m_map, usAddr, M6800NEG(regs, ucTemp));
            break;

         case 0x63: /* COM - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800WriteByte(m_map, usAddr, M6800COM(regs, ucTemp));
            break;

         case 0x64: /* LSR - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800WriteByte(m_map, usAddr, M6800LSR(regs, ucTemp));
            break;

         case 0x66: /* ROR - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800WriteByte(m_map, usAddr, M6800ROR(regs, ucTemp));
            break;

         case 0x67: /* ASR - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800WriteByte(m_map, usAddr, M6800ASR(regs, ucTemp));
            break;

         case 0x68: /* ASL - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800WriteByte(m_map, usAddr, M6800ASL(regs, ucTemp));
            break;

         case 0x69: /* ROL - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800WriteByte(m_map, usAddr, M6800ROL(regs, ucTemp));
            break;

         case 0x6A: /* DEC - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800WriteByte(m_map, usAddr, M6800DEC(regs, ucTemp));
            break;

         case 0x6C: /* INC - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800WriteByte(m_map, usAddr, M6800INC(regs, ucTemp));
            break;

         case 0x6D: /* TST - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            regs->ucRegCC &= ~(F_OVERFLOW | F_ZERO | F_NEGATIVE | F_CARRY);
            regs->ucRegCC |= c6800NZ[M6800ReadByte(m_map, usAddr)];
            break;

         case 0x6E: /* JMP - indexed */
            PC = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC);
            break;

         case 0x6F: /* CLR - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            M6800WriteByte(m_map, usAddr, 0);
            regs->ucRegCC &= ~(F_OVERFLOW | F_CARRY | F_NEGATIVE);
            regs->ucRegCC |= F_ZERO;
            break;

         case 0x70: /* NEG - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, M6800NEG(regs, M6800ReadByte(m_map, usAddr)));
            break;

         case 0x73: /* COM - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, M6800COM(regs, M6800ReadByte(m_map, usAddr)));
            break;

         case 0x74: /* LSR - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, M6800LSR(regs, M6800ReadByte(m_map, usAddr)));
            break;

         case 0x76: /* ROR - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, M6800ROR(regs, M6800ReadByte(m_map, usAddr)));
            break;

         case 0x77: /* ASR - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, M6800ASR(regs, M6800ReadByte(m_map, usAddr)));
            break;

         case 0x78: /* ASL - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, M6800ASL(regs, M6800ReadByte(m_map, usAddr)));
            break;

         case 0x79: /* ROL - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, M6800ROL(regs, M6800ReadByte(m_map, usAddr)));
            break;

         case 0x7A: /* DEC - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, M6800DEC(regs, M6800ReadByte(m_map, usAddr)));
            break;

         case 0x7C: /* INC - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, M6800INC(regs, M6800ReadByte(m_map, usAddr)));
            break;

         case 0x7D: /* TST - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegCC &= ~(F_CARRY | F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[ucTemp];
            break;

         case 0x7E: /* JMP - extended */
            PC = M6800ReadWord(m_map, PC);
            break;

         case 0x7F: /* CLR - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, 0);
            regs->ucRegCC &= ~(F_CARRY | F_OVERFLOW | F_NEGATIVE);
            regs->ucRegCC |= F_ZERO;
            break;

         case 0x80: /* SUBA - immediate */
            regs->ucRegA = M6800SUB(regs, regs->ucRegA, M6800ReadByte(m_map, PC++));
            break;

         case 0x81: /* CMPA - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            M6800CMP(regs, regs->ucRegA, ucTemp);
            break;

         case 0x82: /* SBCA - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegA = M6800SBC(regs, regs->ucRegA, ucTemp);
            break;

         case 0x84: /* ANDA - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegA = M6800AND(regs, regs->ucRegA, ucTemp);
            break;

         case 0x85: /* BITA - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            M6800AND(regs, regs->ucRegA, ucTemp);
            break;

         case 0x86: /* LDA - immediate */
            regs->ucRegA = M6800ReadByte(m_map, PC++);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegA];
            break;

         case 0x88: /* EORA - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegA = M6800EOR(regs, regs->ucRegA, ucTemp);
            break;

         case 0x89: /* ADCA - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegA = M6800ADC(regs, regs->ucRegA, ucTemp);
            break;

         case 0x8A: /* ORA - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegA = M6800OR(regs, regs->ucRegA, ucTemp);
            break;

         case 0x8B: /* ADDA - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegA = M6800ADD(regs, regs->ucRegA, ucTemp);
            break;

         case 0x8C: /* CPX - immediate */
            usTemp = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800CMP16(regs, regs->usRegX, usTemp);
            break;

         case 0x8D: /* BSR */
            sTemp = (signed short)(signed char)M6800ReadByte(m_map, PC++);
            M6800PUSHW(m_map, regs, PC);
            PC += sTemp;
            break;

         case 0x8E: /* LDS - immediate */
            usTemp = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->usRegS = usTemp;
            M6800FlagsNZ16(regs, usTemp);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0x90: /* SUBA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800SUB(regs, regs->ucRegA, ucTemp);
            break;

         case 0x91: /* CMPA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800CMP(regs, regs->ucRegA, ucTemp);
            break;

         case 0x92: /* SBCA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800SBC(regs, regs->ucRegA, ucTemp);
            break;

         case 0x94: /* ANDA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800AND(regs, regs->ucRegA, ucTemp);
            break;

         case 0x95: /* BITA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800AND(regs, regs->ucRegA, ucTemp);
            break;

         case 0x96: /* LDA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            regs->ucRegA = M6800ReadByte(m_map, usAddr);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegA];
            break;

         case 0x97: /* STA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            M6800WriteByte(m_map, usAddr, regs->ucRegA);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegA];
            break;

         case 0x98: /* EORA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800EOR(regs, regs->ucRegA, ucTemp);
            break;

         case 0x99: /* ADCA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800ADC(regs, regs->ucRegA, ucTemp);
            break;

         case 0x9A: /* ORA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800OR(regs, regs->ucRegA, ucTemp);
            break;

         case 0x9B: /* ADDA - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800ADD(regs, regs->ucRegA, ucTemp);
            break;

         case 0x9C: /* CMPX - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            usTemp = M6800ReadWord(m_map, usAddr);
            M6800CMP16(regs, regs->usRegX, usTemp);
            break;

         case 0x9E: /* LDS - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            regs->usRegS = M6800ReadWord(m_map, usAddr);
            M6800FlagsNZ16(regs, regs->usRegS);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0x9F: /* STS - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++); /* Address of byte to negate */
            M6800WriteWord(m_map, usAddr, regs->usRegS);
            M6800FlagsNZ16(regs, regs->usRegS);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xA0: /* SUBA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800SUB(regs, regs->ucRegA, ucTemp);
            break;

         case 0xA1: /* CMPA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800CMP(regs, regs->ucRegA, ucTemp);
            break;

         case 0xA2: /* SBCA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800SBC(regs, regs->ucRegA, ucTemp);
            break;

         case 0xA4: /* ANDA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800AND(regs, regs->ucRegA, ucTemp);
            break;

         case 0xA5: /* BITA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800AND(regs, regs->ucRegA, ucTemp);
            break;

         case 0xA6: /* LDA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            regs->ucRegA = M6800ReadByte(m_map, usAddr);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegA];
            break;

         case 0xA7: /* STA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            M6800WriteByte(m_map, usAddr, regs->ucRegA);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegA];
            break;

         case 0xA8: /* EORA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800EOR(regs, regs->ucRegA, ucTemp);
            break;

         case 0xA9: /* ADCA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800ADC(regs, regs->ucRegA, ucTemp);
            break;

         case 0xAA: /* ORA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800OR(regs, regs->ucRegA, ucTemp);
            break;

         case 0xAB: /* ADDA - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegA = M6800ADD(regs, regs->ucRegA, ucTemp);
            break;

         case 0xAC: /* CMPX - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            usTemp = M6800ReadWord(m_map, usAddr);
            M6800CMP16(regs, regs->usRegX, usTemp);
            break;

         case 0xAD: /* JSR - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            M6800PUSHW(m_map, regs, PC);
            PC = usAddr;
            break;

         case 0xAE: /* LDS - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            regs->usRegS = M6800ReadWord(m_map, usAddr);
            M6800FlagsNZ16(regs, regs->usRegS);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xAF: /* STS - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            M6800WriteWord(m_map, usAddr, regs->usRegS);
            M6800FlagsNZ16(regs, regs->usRegS);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xB0: /* SUBA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->ucRegA = M6800SUB(regs, regs->ucRegA, M6800ReadByte(m_map, usAddr));
            break;

         case 0xB1: /* CMPA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800CMP(regs, regs->ucRegA, M6800ReadByte(m_map, usAddr));
            break;

         case 0xB2: /* SBCA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->ucRegA = M6800SBC(regs, regs->ucRegA, M6800ReadByte(m_map, usAddr));
            break;

         case 0xB4: /* ANDA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->ucRegA = M6800AND(regs, regs->ucRegA, M6800ReadByte(m_map, usAddr));
            break;

         case 0xB5: /* BITA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800AND(regs, regs->ucRegA, M6800ReadByte(m_map, usAddr));
            break;

         case 0xB6: /* LDA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->ucRegA = M6800ReadByte(m_map, usAddr);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegA];
            break;

         case 0xB7: /* STA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, regs->ucRegA);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegA];
            break;

         case 0xB8: /* EORA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->ucRegA = M6800EOR(regs, regs->ucRegA, M6800ReadByte(m_map, usAddr));
            break;

         case 0xB9: /* ADCA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->ucRegA = M6800ADC(regs, regs->ucRegA, M6800ReadByte(m_map, usAddr));
            break;

         case 0xBA: /* ORA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->ucRegA = M6800OR(regs, regs->ucRegA, M6800ReadByte(m_map, usAddr));
            break;

         case 0xBB: /* ADDA - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->ucRegA = M6800ADD(regs, regs->ucRegA, M6800ReadByte(m_map, usAddr));
            break;

         case 0xBC: /* CMPX - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800CMP16(regs, regs->usRegX, M6800ReadWord(m_map, usAddr));
            break;

         case 0xBD: /* JSR - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800PUSHW(m_map, regs, PC);
            PC = usAddr;
            break;

         case 0xBE: /* LDS - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->usRegS = M6800ReadWord(m_map, usAddr);
            M6800FlagsNZ16(regs, regs->usRegS);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xBF: /* STS - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteWord(m_map, usAddr, regs->usRegS);
            M6800FlagsNZ16(regs, regs->usRegS);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xC0: /* SUBB - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegB = M6800SUB(regs, regs->ucRegB, ucTemp);
            break;

         case 0xC1: /* CMPB - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            M6800CMP(regs, regs->ucRegB, ucTemp);
            break;

         case 0xC2: /* SBCB - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegB = M6800SBC(regs, regs->ucRegB, ucTemp);
            break;

         case 0xC4: /* ANDB - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegB = M6800AND(regs, regs->ucRegB, ucTemp);
            break;

         case 0xC5: /* BITB - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            M6800AND(regs, regs->ucRegB, ucTemp);
            break;

         case 0xC6: /* LDB - immediate */
            regs->ucRegB = M6800ReadByte(m_map, PC++);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegB];
            break;

         case 0xC8: /* EORB - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegB = M6800EOR(regs, regs->ucRegB, ucTemp);
            break;

         case 0xC9: /* ADCB - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegB = M6800ADC(regs, regs->ucRegB, ucTemp);
            break;

         case 0xCA: /* ORB - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegB = M6800OR(regs, regs->ucRegB, ucTemp);
            break;

         case 0xCB: /* ADDB - immediate */
            ucTemp = M6800ReadByte(m_map, PC++);
            regs->ucRegB = M6800ADD(regs, regs->ucRegB, ucTemp);
            break;

         case 0xCE: /* LDX - immediate */
            regs->usRegX = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800FlagsNZ16(regs, regs->usRegX);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xD0: /* SUBB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800SUB(regs, regs->ucRegB, ucTemp);
            break;

         case 0xD1: /* CMPB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800CMP(regs, regs->ucRegB, ucTemp);
            break;

         case 0xD2: /* SBCB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800SBC(regs, regs->ucRegB, ucTemp);
            break;

         case 0xD4: /* ANDB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800AND(regs, regs->ucRegB, ucTemp);
            break;

         case 0xD5: /* BITB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800AND(regs, regs->ucRegB, ucTemp);
            break;

         case 0xD6: /* LDB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            regs->ucRegB = M6800ReadByte(m_map, usAddr);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegB];
            break;

         case 0xD7: /* STB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            M6800WriteByte(m_map, usAddr, regs->ucRegB);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegB];
            break;

         case 0xD8: /* EORB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800EOR(regs, regs->ucRegB, ucTemp);
            break;

         case 0xD9: /* ADCB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800ADC(regs, regs->ucRegB, ucTemp);
            break;

         case 0xDA: /* ORB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800OR(regs, regs->ucRegB, ucTemp);
            break;

         case 0xDB: /* ADDB - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800ADD(regs, regs->ucRegB, ucTemp);
            break;

         case 0xDE: /* LDX - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            regs->usRegX = M6800ReadWord(m_map, usAddr);
            M6800FlagsNZ16(regs, regs->usRegX);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xDF: /* STX - direct */
            usAddr = (unsigned short)M6800ReadByte(m_map, PC++);
            M6800WriteWord(m_map, usAddr, regs->usRegX);
            M6800FlagsNZ16(regs, regs->usRegX);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xE0: /* SUBB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800SUB(regs, regs->ucRegB, ucTemp);
            break;

         case 0xE1: /* CMPB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800CMP(regs, regs->ucRegB, ucTemp);
            break;

         case 0xE2: /* SBCB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800SBC(regs, regs->ucRegB, ucTemp);
            break;

         case 0xE4: /* ANDB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800AND(regs, regs->ucRegB, ucTemp);
            break;

         case 0xE5: /* BITB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800AND(regs, regs->ucRegB, ucTemp);
            break;

         case 0xE6: /* LDB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            regs->ucRegB = M6800ReadByte(m_map, usAddr);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegB];
            break;

         case 0xE7: /* STB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            M6800WriteByte(m_map, usAddr, regs->ucRegB);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegB];
            break;

         case 0xE8: /* EORB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800EOR(regs, regs->ucRegB, ucTemp);
            break;

         case 0xE9: /* ADCB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800ADC(regs, regs->ucRegB, ucTemp);
            break;

         case 0xEA: /* ORB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800OR(regs, regs->ucRegB, ucTemp);
            break;

         case 0xEB: /* ADDB - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800ADD(regs, regs->ucRegB, ucTemp);
            break;

         case 0xEE: /* LDX - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            regs->usRegX = M6800ReadWord(m_map, usAddr);
            M6800FlagsNZ16(regs, regs->usRegX);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xEF: /* STX - indexed */
            usAddr = regs->usRegX + (unsigned short)M6800ReadByte(m_map, PC++);
            M6800WriteWord(m_map, usAddr, regs->usRegX);
            M6800FlagsNZ16(regs, regs->usRegX);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xF0: /* SUBB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800SUB(regs, regs->ucRegB, ucTemp);
            break;

         case 0xF1: /* CMPB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800CMP(regs, regs->ucRegB, ucTemp);
            break;

         case 0xF2: /* SBCB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800SBC(regs, regs->ucRegB, ucTemp);
            break;

         case 0xF4: /* ANDB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800AND(regs, regs->ucRegB, ucTemp);
            break;

         case 0xF5: /* BITB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            M6800AND(regs, regs->ucRegB, ucTemp);
            break;

         case 0xF6: /* LDB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->ucRegB = M6800ReadByte(m_map, usAddr);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegB];
            break;

         case 0xF7: /* STB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteByte(m_map, usAddr, regs->ucRegB);
            regs->ucRegCC &= ~(F_OVERFLOW | F_NEGATIVE | F_ZERO);
            regs->ucRegCC |= c6800NZ[regs->ucRegB];
            break;

         case 0xF8: /* EORB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800EOR(regs, regs->ucRegB, ucTemp);
            break;

         case 0xF9: /* ADCB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800ADC(regs, regs->ucRegB, ucTemp);
            break;

         case 0xFA: /* ORB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800OR(regs, regs->ucRegB, ucTemp);
            break;

         case 0xFB: /* ADDB - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            ucTemp = M6800ReadByte(m_map, usAddr);
            regs->ucRegB = M6800ADD(regs, regs->ucRegB, ucTemp);
            break;

         case 0xFE: /* LDX - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            regs->usRegX = M6800ReadWord(m_map, usAddr);
            M6800FlagsNZ16(regs, regs->usRegX);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         case 0xFF: /* STX - extended */
            usAddr = M6800ReadWord(m_map, PC);
            PC += 2;
            M6800WriteWord(m_map, usAddr, regs->usRegX);
            M6800FlagsNZ16(regs, regs->usRegX);
            regs->ucRegCC &= ~F_OVERFLOW;
            break;

         default: /* Illegal instruction */
            *iClocks = 0;
            break;
         } /* switch */
      } /* while *iClocks */

   regs->usRegPC = PC;

} /* EXEC6800() */
