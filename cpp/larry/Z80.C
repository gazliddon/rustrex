/*******************************************************************/
/* Z80 CPU emulator written by Larry Bank                          */
/* Copyright 1998 BitBank Software, Inc.                           */
/*                                                                 */
/* This code was written from scratch using the Z80  data from     */
/* the Zilog databook "Components".                                */
/*                                                                 */
/* Change history:                                                 */
/* 2/8/98 Wrote it - Larry B.                                      */
/*******************************************************************/
#include <windows.h>
#include <string.h>
#include "emu.h"

#define F_CARRY     1
#define F_ADDSUB    2
#define F_OVERFLOW  4
#define F_HALFCARRY 16
#define F_ZERO      64
#define F_SIGN      128

//#define TRACE 1
void TRACEZ80(REGSZ80 *);

/* Some statics */
EMUHANDLERS *mem_handlers_z80;
unsigned char *m_map_z80;

#define SET_V8(a,b,r) regs->ucRegF |= (((a^b^r^(r>>1))&0x80)>>5)
#define SET_V16(a,b,r) regs->ucRegF |= (((a^b^r^(r>>1))&0x8000)>>13)

/* Parity bit table */
unsigned char cZ80Parity[256] = {
                4,0,0,4,0,4,4,0,0,4,4,0,4,0,0,4,     /* 00-0F */
                0,4,4,0,4,0,0,4,4,0,0,4,0,4,4,0,     /* 10-1F */
                0,4,4,0,4,0,0,4,4,0,0,4,0,4,4,0,     /* 20-2F */
                4,0,0,4,0,4,4,0,0,4,4,0,4,0,0,4,     /* 30-3F */
                0,4,4,0,4,0,0,4,4,0,0,4,0,4,4,0,     /* 40-4F */
                4,0,0,4,0,4,4,0,0,4,4,0,4,0,0,4,     /* 50-5F */
                4,0,0,4,0,4,4,0,0,4,4,0,4,0,0,4,     /* 60-6F */
                0,4,4,0,4,0,0,4,4,0,0,4,0,4,4,0,     /* 70-7F */
                0,4,4,0,4,0,0,4,4,0,0,4,0,4,4,0,     /* 80-8F */
                4,0,0,4,0,4,4,0,0,4,4,0,4,0,0,4,     /* 90-9F */
                4,0,0,4,0,4,4,0,0,4,4,0,4,0,0,4,     /* A0-AF */
                0,4,4,0,4,0,0,4,4,0,0,4,0,4,4,0,     /* B0-BF */
                4,0,0,4,0,4,4,0,0,4,4,0,4,0,0,4,     /* C0-CF */
                0,4,4,0,4,0,0,4,4,0,0,4,0,4,4,0,     /* D0-DF */
                0,4,4,0,4,0,0,4,4,0,0,4,0,4,4,0,     /* E0-EF */
                4,0,0,4,0,4,4,0,0,4,4,0,4,0,0,4};    /* F0-FF */

/* Instruction t-states by opcode for 1 byte opcodes */
unsigned char cZ80Cycles[256] = {4,10,7,6,4,4,7,4,4,11,7,6,4,4,7,4,      /* 00-0F */
                                 8,10,7,6,4,4,7,4,12,11,7,6,4,4,7,4,     /* 10-1F */
                                 7,10,16,6,4,4,7,4,7,11,16,6,4,4,7,4,    /* 20-2F */
                                 7,10,13,6,11,11,10,4,7,11,13,6,4,4,7,4, /* 30-3F */
                                 4,4,4,4,4,4,7,4,4,4,4,4,4,4,7,4,        /* 40-4F */
                                 4,4,4,4,4,4,7,4,4,4,4,4,4,4,7,4,        /* 50-5F */
                                 4,4,4,4,4,4,7,4,4,4,4,4,4,4,7,4,        /* 60-6F */
                                 7,7,7,7,7,7,4,7,4,4,4,4,4,4,7,4,        /* 70-7F */
                                 4,4,4,4,4,4,7,4,4,4,4,4,4,4,7,4,        /* 80-8F */
                                 4,4,4,4,4,4,7,4,4,4,4,4,4,4,7,4,        /* 90-9F */
                                 4,4,4,4,4,4,7,4,4,4,4,4,4,4,7,4,        /* A0-AF */
                                 4,4,4,4,4,4,7,4,4,4,4,4,4,4,7,4,        /* B0-BF */
                                 5,10,10,10,10,11,7,11,5,10,10,0,10,17,7,11, /* C0-CF */
                                 5,10,10,11,10,11,7,11,5,4,10,11,10,0,0,11, /* D0-DF */
                                 5,10,10,19,0,0,0,0,0,0,0,0,0,0,0,0,     /* E0-EF */
                                 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};       /* F0-FF */

/* Instruction t-states by opcode for 2 byte opcodes  DDXX & FDXX */
unsigned char cZ80Cycles2[256] = {
                0,0,0,0,0,0,0,0,0,15,0,0,0,0,0,0,        /* 00-0F */
                0,0,0,0,0,0,0,0,0,15,0,0,0,0,0,0,        /* 10-1F */
                0,14,20,10,0,0,0,0,0,15,20,10,0,0,0,0,   /* 20-2F */
                0,0,0,0,23,23,19,0,0,15,0,0,0,0,0,0,     /* 30-3F */
                0,0,0,0,0,0,19,0,0,0,0,0,0,0,19,0,       /* 40-4F */
                0,0,0,0,0,0,19,0,0,0,0,0,0,0,19,0,       /* 50-5F */
                0,0,0,0,0,0,19,0,0,0,0,0,0,0,19,0,       /* 60-6F */
                19,19,19,19,19,19,0,19,0,0,0,0,0,0,19,0, /* 70-7F */
                0,0,0,0,0,0,19,0,0,0,0,0,0,0,19,0,       /* 80-8F */
                0,0,0,0,0,0,19,0,0,0,0,0,0,0,19,0,       /* 90-9F */
                0,0,0,0,0,0,19,0,0,0,0,0,0,0,19,0,       /* A0-AF */
                0,0,0,0,0,0,19,0,0,0,0,0,0,0,19,0,       /* B0-BF */
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,         /* C0-CF */
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,         /* D0-DF */
                0,14,0,23,0,15,0,0,0,8,0,0,0,0,0,0,      /* E0-EF */
                0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,0};       /* F0-FF */

/* Instruction t-states by opcode for 3 byte opcodes  DDCBXX & FDCBXX */
unsigned char cZ80Cycles3[256] = {
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* 00-0F */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* 10-1F */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* 20-2F */
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,23,0,      /* 30-3F */
                0,0,0,0,0,0,20,0,0,0,0,0,0,0,20,0,     /* 40-4F */
                0,0,0,0,0,0,20,0,0,0,0,0,0,0,20,0,     /* 50-5F */
                0,0,0,0,0,0,20,0,0,0,0,0,0,0,20,0,     /* 60-6F */
                0,0,0,0,0,0,20,0,0,0,0,0,0,0,20,0,     /* 70-7F */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* 80-8F */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* 90-9F */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* A0-AF */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* B0-BF */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* C0-CF */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* D0-DF */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0,     /* E0-EF */
                0,0,0,0,0,0,23,0,0,0,0,0,0,0,23,0};    /* F0-FF */

/* Sign and zero flags for quicker flag settings */
unsigned char cZ80SZ[256]={
      F_ZERO,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,     /* 00-0F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 10-1F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 20-2F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 30-3F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 40-4F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 50-5F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 60-6F */
      0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,          /* 70-7F */
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,          /* 80-8F */
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,          /* 90-9F */
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,          /* A0-AF */
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,          /* B0-BF */
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,          /* C0-CF */
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,          /* D0-DF */
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,          /* E0-EF */
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,
      F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN,F_SIGN};         /* F0-FF */

/* Register pointer tables for easier instruction decoding */
unsigned short *pusReg[4];
unsigned char *pucReg[8];

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80ADD(REGSZ80 *, unsigned char)                           *
 *                                                                          *
 *  PURPOSE    : Add an 8-bit value to A.                                   *
 *                                                                          *
 ****************************************************************************/
void Z80ADD(REGSZ80 *regs, unsigned char ucByte)
{
unsigned short us;

   us = regs->ucRegA + ucByte;
   regs->ucRegF &= ~(F_CARRY | F_ADDSUB | F_SIGN | F_ZERO | F_OVERFLOW | F_HALFCARRY);
   if (us & 0x100)
      regs->ucRegF |= F_CARRY;
   regs->ucRegF |= cZ80SZ[us & 0xff];
   regs->ucRegF |= ((regs->ucRegA ^ ucByte ^ us) & F_HALFCARRY);
   SET_V8(regs->ucRegA, ucByte, us);
   regs->ucRegA = (unsigned char)us;

} /* Z80ADD() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80ADC(REGSZ80 *, unsigned char)                           *
 *                                                                          *
 *  PURPOSE    : Add an 8-bit value to A with carry.                        *
 *                                                                          *
 ****************************************************************************/
void Z80ADC(REGSZ80 *regs, unsigned char ucByte)
{
unsigned short us;

   us = regs->ucRegA + ucByte + (regs->ucRegF & F_CARRY);
   regs->ucRegF &= ~(F_CARRY | F_ADDSUB | F_SIGN | F_ZERO | F_OVERFLOW | F_HALFCARRY);
   if (us & 0x100)
      regs->ucRegF |= F_CARRY;
   regs->ucRegF |= cZ80SZ[us & 0xff];
   regs->ucRegF |= ((regs->ucRegA ^ ucByte ^ us) & F_HALFCARRY);
   SET_V8(regs->ucRegA, ucByte, us);
   regs->ucRegA = (unsigned char)us;

} /* Z80ADC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80SUB(REGSZ80 *, unsigned char)                           *
 *                                                                          *
 *  PURPOSE    : Sub an 8-bit value to A.                                   *
 *                                                                          *
 ****************************************************************************/
void Z80SUB(REGSZ80 *regs, unsigned char ucByte)
{
unsigned short us;

   us = regs->ucRegA - ucByte;
   regs->ucRegF &= ~(F_CARRY | F_SIGN | F_ZERO | F_OVERFLOW | F_HALFCARRY);
   regs->ucRegF |= F_ADDSUB;
   if (us & 0x100)
      regs->ucRegF |= F_CARRY;
   regs->ucRegF |= cZ80SZ[us & 0xff];
   regs->ucRegF |= ((regs->ucRegA ^ ucByte ^ us) & F_HALFCARRY);
   SET_V8(regs->ucRegA, ucByte, us);
   regs->ucRegA = (unsigned char)us;

} /* Z80SUB() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80SBC(REGSZ80 *, unsigned char)                           *
 *                                                                          *
 *  PURPOSE    : Sub an 8-bit value to A with carry.                        *
 *                                                                          *
 ****************************************************************************/
void Z80SBC(REGSZ80 *regs, unsigned char ucByte)
{
unsigned short us;

   us = regs->ucRegA - ucByte - (regs->ucRegF & F_CARRY);
   regs->ucRegF &= ~(F_CARRY | F_SIGN | F_ZERO | F_OVERFLOW | F_HALFCARRY);
   regs->ucRegF |= F_ADDSUB;
   if (us & 0x100)
      regs->ucRegF |= F_CARRY;
   regs->ucRegF |= cZ80SZ[us & 0xff];
   regs->ucRegF |= ((regs->ucRegA ^ ucByte ^ us) & F_HALFCARRY);
   SET_V8(regs->ucRegA, ucByte, us);
   regs->ucRegA = (unsigned char)us;

} /* Z80SBC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80AND(REGSZ80 *, unsigned char)                           *
 *                                                                          *
 *  PURPOSE    : Logical AND an 8-bit value and update the flags.           *
 *                                                                          *
 ****************************************************************************/
void Z80AND(REGSZ80 *regs, unsigned char ucByte)
{

   regs->ucRegA &= ucByte;
   regs->ucRegF &= ~(F_SIGN | F_ZERO | F_OVERFLOW | F_CARRY |F_ADDSUB);
   regs->ucRegF |= cZ80SZ[regs->ucRegA];
   regs->ucRegF |= F_HALFCARRY;
   regs->ucRegF |= cZ80Parity[regs->ucRegA];

} /* Z80AND() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80XOR(REGSZ80 *, unsigned char)                           *
 *                                                                          *
 *  PURPOSE    : Logical XOR an 8-bit value and update the flags.           *
 *                                                                          *
 ****************************************************************************/
void Z80XOR(REGSZ80 *regs, unsigned char ucByte)
{
   regs->ucRegA ^= ucByte;
   regs->ucRegF &= ~(F_SIGN | F_ZERO | F_OVERFLOW | F_CARRY |F_ADDSUB | F_HALFCARRY);
   regs->ucRegF |= cZ80SZ[regs->ucRegA];
   regs->ucRegF |= cZ80Parity[regs->ucRegA];
} /* Z80XOR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80OR(REGSZ80 *, unsigned char)                            *
 *                                                                          *
 *  PURPOSE    : Logical OR an 8-bit value and update the flags.            *
 *                                                                          *
 ****************************************************************************/
void Z80OR(REGSZ80 *regs, unsigned char ucByte)
{
   regs->ucRegA |= ucByte;
   regs->ucRegF &= ~(F_SIGN | F_ZERO | F_OVERFLOW | F_CARRY |F_ADDSUB | F_HALFCARRY);
   regs->ucRegF |= cZ80SZ[regs->ucRegA];
   regs->ucRegF |= cZ80Parity[regs->ucRegA];
} /* Z80OR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80CMP(REGSZ80 *, unsigned char)                           *
 *                                                                          *
 *  PURPOSE    : Compare and 8-bit value to A.                              *
 *                                                                          *
 ****************************************************************************/
void Z80CMP(REGSZ80 *regs, unsigned char ucByte)
{
unsigned short us;

   us = regs->ucRegA - ucByte;
   regs->ucRegF &= ~(F_CARRY | F_SIGN | F_ZERO | F_OVERFLOW | F_HALFCARRY);
   regs->ucRegF |= F_ADDSUB;
   if (us & 0x100)
      regs->ucRegF |= F_CARRY;
   regs->ucRegF |= cZ80SZ[us & 0xff];
   regs->ucRegF |= ((regs->ucRegA ^ ucByte ^ us) & F_HALFCARRY);
   SET_V8(regs->ucRegA, ucByte, us);

} /* Z80CMP() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80INC(REGSZ80 *, unsigned char)                           *
 *                                                                          *
 *  PURPOSE    : Increment an 8-bit value and update the flags.             *
 *                                                                          *
 ****************************************************************************/
unsigned char Z80INC(REGSZ80 *regs, unsigned char ucByte)
{
unsigned char c = ucByte;

   ucByte++;
   regs->ucRegF &= ~(F_SIGN | F_ZERO | F_HALFCARRY | F_OVERFLOW | F_ADDSUB);
   regs->ucRegF |= cZ80SZ[ucByte]; /* Set sign and zero flags */
   regs->ucRegF |= ((c ^ ucByte) & F_HALFCARRY);
   if ((c ^ ucByte) & 0x80)
      regs->ucRegF |= F_OVERFLOW;

   return ucByte;
} /* Z80INC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80DEC(REGSZ80 *, unsigned char)                           *
 *                                                                          *
 *  PURPOSE    : Decrement an 8-bit value and update the flags.             *
 *                                                                          *
 ****************************************************************************/
unsigned char Z80DEC(REGSZ80 *regs, unsigned char ucByte)
{
unsigned char c = ucByte;

   ucByte--;
   regs->ucRegF &= ~(F_SIGN | F_ZERO | F_HALFCARRY | F_OVERFLOW);
   regs->ucRegF |= F_ADDSUB;
   regs->ucRegF |= cZ80SZ[ucByte]; /* Set sign and zero flags */
   regs->ucRegF |= ((c ^ ucByte) & F_HALFCARRY);
   if ((c ^ ucByte) & 0x80)
      regs->ucRegF |= F_OVERFLOW;

   return ucByte;
} /* Z80DEC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80ADD16(REGSZ80 *, unsigned word, unsigned word)          *
 *                                                                          *
 *  PURPOSE    : Add two 16-bit values and update the flags.                *
 *                                                                          *
 ****************************************************************************/
unsigned short Z80ADD16(REGSZ80 *regs, unsigned short usWord1, unsigned short usWord2)
{
unsigned long ul = usWord1 + usWord2;

   regs->ucRegF &= ~(F_CARRY | F_ADDSUB);
   if (ul & 0x10000)
      regs->ucRegF |= F_CARRY;

   return (unsigned short)ul;

} /* Z80ADD16() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80ADC16(REGSZ80 *, unsigned word, unsigned word)          *
 *                                                                          *
 *  PURPOSE    : Add two 16-bit values and update the flags.                *
 *                                                                          *
 ****************************************************************************/
unsigned short Z80ADC16(REGSZ80 *regs, unsigned short usWord1, unsigned short usWord2)
{
unsigned long ul;

   ul = usWord1 + usWord2 + (regs->ucRegF & F_CARRY);
   regs->ucRegF &= ~(F_CARRY | F_ADDSUB | F_SIGN | F_ZERO | F_OVERFLOW);
   if (ul & 0x10000)
      regs->ucRegF |= F_CARRY;
   if (ul & 0x8000)
      regs->ucRegF |= F_SIGN;
   if (ul == 0 || ul == 0x10000)
      regs->ucRegF |= F_ZERO;
   SET_V16(usWord1, usWord2, ul);

   return (unsigned short)ul;

} /* Z80ADC16() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80SBC16(REGSZ80 *, unsigned word, unsigned word)          *
 *                                                                          *
 *  PURPOSE    : Sub two 16-bit values and update the flags.                *
 *                                                                          *
 ****************************************************************************/
unsigned short Z80SBC16(REGSZ80 *regs, unsigned short usWord1, unsigned short usWord2)
{
unsigned long ul;

   ul = usWord1 - usWord2 - (regs->ucRegF & F_CARRY);
   regs->ucRegF &= ~(F_CARRY | F_SIGN | F_ZERO | F_OVERFLOW);
   regs->ucRegF |= F_ADDSUB;
   if (ul & 0x10000)
      regs->ucRegF |= F_CARRY;
   if (ul & 0x8000)
      regs->ucRegF |= F_SIGN;
   if (ul == 0 || ul == 0x10000)
      regs->ucRegF |= F_ZERO;
   SET_V16(usWord1, usWord2, ul);

   return (unsigned short)ul;

} /* Z80SBC16() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80RL(REGSZ80 *, unsigned word, unsigned word)             *
 *                                                                          *
 *  PURPOSE    : Rotate left through carry.                                 *
 *                                                                          *
 ****************************************************************************/
unsigned char Z80RL(REGSZ80 *regs, unsigned char ucByte)
{
unsigned char uc;

   uc = regs->ucRegF & F_CARRY; /* Preserve old carry flag */
   regs->ucRegF &= ~(F_ZERO | F_CARRY | F_ADDSUB | F_OVERFLOW | F_SIGN | F_HALFCARRY);
   if (ucByte & 0x80)
      regs->ucRegF |= F_CARRY;
   ucByte = (ucByte <<1) | uc;
   regs->ucRegF |= cZ80SZ[ucByte];
   regs->ucRegF |= cZ80Parity[ucByte];
   return ucByte;

} /* Z80RL() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80SLA(REGSZ80 *, unsigned word, unsigned word)            *
 *                                                                          *
 *  PURPOSE    : Shift left arithmetic.                                     *
 *                                                                          *
 ****************************************************************************/
unsigned char Z80SLA(REGSZ80 *regs, unsigned char ucByte)
{
unsigned char uc;

   uc = ucByte & 0x80;
   regs->ucRegF &= ~(F_ZERO | F_CARRY | F_ADDSUB | F_OVERFLOW | F_SIGN | F_HALFCARRY);
   if (uc)
      regs->ucRegF |= F_CARRY;
   ucByte = ucByte <<1;
   regs->ucRegF |= cZ80SZ[ucByte];
   regs->ucRegF |= cZ80Parity[ucByte];
   return ucByte;

} /* Z80SLA() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80SRA(REGSZ80 *, unsigned word, unsigned word)            *
 *                                                                          *
 *  PURPOSE    : Shift right arithmetic.                                    *
 *                                                                          *
 ****************************************************************************/
unsigned char Z80SRA(REGSZ80 *regs, unsigned char ucByte)
{
unsigned char uc;
unsigned char ucOld = ucByte;

   uc = ucByte & 1;
   regs->ucRegF &= ~(F_ZERO | F_CARRY | F_ADDSUB | F_OVERFLOW | F_SIGN | F_HALFCARRY);
   regs->ucRegF |= uc;
   ucByte = (ucByte >>1) | (ucOld & 0x80);
   regs->ucRegF |= cZ80SZ[ucByte];
   regs->ucRegF |= cZ80Parity[ucByte];
   return ucByte;

} /* Z80SRA() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80SRL(REGSZ80 *, unsigned word, unsigned word)            *
 *                                                                          *
 *  PURPOSE    : Shift right logical.                                       *
 *                                                                          *
 ****************************************************************************/
unsigned char Z80SRL(REGSZ80 *regs, unsigned char ucByte)
{
unsigned char uc;

   uc = ucByte & 1;
   regs->ucRegF &= ~(F_ZERO | F_CARRY | F_ADDSUB | F_OVERFLOW | F_SIGN | F_HALFCARRY);
   regs->ucRegF |= uc;
   ucByte = ucByte >>1;
   regs->ucRegF |= cZ80SZ[ucByte];
   regs->ucRegF |= cZ80Parity[ucByte];
   return ucByte;

} /* Z80SRL() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80RR(REGSZ80 *, unsigned word, unsigned word)            *
 *                                                                          *
 *  PURPOSE    : Rotate right through carry.                                *
 *                                                                          *
 ****************************************************************************/
unsigned char Z80RR(REGSZ80 *regs, unsigned char ucByte)
{
unsigned char uc;

   uc = regs->ucRegF & F_CARRY; /* Preserve old carry flag */
   regs->ucRegF &= ~(F_ZERO | F_CARRY | F_ADDSUB | F_OVERFLOW | F_SIGN | F_HALFCARRY);
   regs->ucRegF |= (ucByte & F_CARRY);
   ucByte = (ucByte >>1) | (uc << 7);
   regs->ucRegF |= cZ80SZ[ucByte];
   regs->ucRegF |= cZ80Parity[ucByte];
   return ucByte;

} /* Z80RR() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80RRA(REGSZ80 *)                                          *
 *                                                                          *
 *  PURPOSE    : Rotate right A through carry.                              *
 *                                                                          *
 ****************************************************************************/
void Z80RRA(REGSZ80 *regs)
{
unsigned char uc;

   uc = regs->ucRegF & F_CARRY; /* Preserve old carry flag */
   regs->ucRegF &= ~(F_CARRY | F_ADDSUB | F_HALFCARRY);
   regs->ucRegF |= (regs->ucRegA & F_CARRY);
   regs->ucRegA = (regs->ucRegA >>1) | (uc << 7);

} /* Z80RRA() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80RLA(REGSZ80 *)                                          *
 *                                                                          *
 *  PURPOSE    : Rotate left A through carry.                               *
 *                                                                          *
 ****************************************************************************/
void Z80RLA(REGSZ80 *regs)
{
unsigned char uc;

   uc = regs->ucRegF & F_CARRY; /* Preserve old carry flag */
   regs->ucRegF &= ~(F_CARRY | F_ADDSUB | F_HALFCARRY);
   if (regs->ucRegA & 0x80)
      regs->ucRegF |= F_CARRY;
   regs->ucRegA = regs->ucRegA <<1 | uc;

} /* Z80RLA() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80RLC(REGSZ80 *, unsigned word, unsigned word)            *
 *                                                                          *
 *  PURPOSE    : Rotate left.                                               *
 *                                                                          *
 ****************************************************************************/
unsigned char Z80RLC(REGSZ80 *regs, unsigned char ucByte)
{
unsigned char uc;

   uc = ucByte & 0x80;
   regs->ucRegF &= ~(F_ZERO | F_CARRY | F_ADDSUB | F_OVERFLOW | F_SIGN | F_HALFCARRY);
   if (uc)
      regs->ucRegF |= F_CARRY;
   ucByte = (ucByte <<1) | (uc >> 7);
   regs->ucRegF |= cZ80SZ[ucByte];
   regs->ucRegF |= cZ80Parity[ucByte];
   return ucByte;

} /* Z80RLC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80RLCA(REGSZ80 *)                                         *
 *                                                                          *
 *  PURPOSE    : Rotate left A.                                             *
 *                                                                          *
 ****************************************************************************/
void Z80RLCA(REGSZ80 *regs)
{
unsigned char uc;

   uc = regs->ucRegA & 0x80;
   regs->ucRegF &= ~(F_CARRY | F_ADDSUB | F_HALFCARRY);
   if (uc)
      regs->ucRegF |= F_CARRY;
   regs->ucRegA = (regs->ucRegA <<1) | (uc >> 7);

} /* Z80RLCA() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80RRC(REGSZ80 *, unsigned word, unsigned word)            *
 *                                                                          *
 *  PURPOSE    : Rotate right.                                              *
 *                                                                          *
 ****************************************************************************/
unsigned char Z80RRC(REGSZ80 *regs, unsigned char ucByte)
{
unsigned char uc;

   uc = ucByte & 1;
   regs->ucRegF &= ~(F_ZERO | F_CARRY | F_ADDSUB | F_OVERFLOW | F_SIGN | F_HALFCARRY);
   regs->ucRegF |= uc;
   ucByte = (ucByte >>1) | (uc << 7);
   regs->ucRegF |= cZ80SZ[ucByte];
   regs->ucRegF |= cZ80Parity[ucByte];
   return ucByte;

} /* Z80RRC() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80RRCA(REGSZ80 *)                                         *
 *                                                                          *
 *  PURPOSE    : Rotate A right.                                            *
 *                                                                          *
 ****************************************************************************/
void Z80RRCA(REGSZ80 *regs)
{
unsigned char uc;

   uc = regs->ucRegA & 1;
   regs->ucRegF &= ~(F_CARRY | F_ADDSUB | F_HALFCARRY);
   regs->ucRegF |= uc;
   regs->ucRegA = (regs->ucRegA >>1) | (uc << 7);

} /* Z80RRCA() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80WriteByte(char *, short, char)                          *
 *                                                                          *
 *  PURPOSE    : Write a byte to memory, check for hardware.                *
 *                                                                          *
 ****************************************************************************/
__inline void Z80WriteByte(unsigned char *mem_map, unsigned short usAddr, unsigned char ucByte)
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
         (mem_handlers_z80[c-2].pfn_write)(usAddr, ucByte);
         break;
      }

} /* Z80WriteByte() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80WriteWord(char *, short, short)                         *
 *                                                                          *
 *  PURPOSE    : Write a word to memory, check for hardware.                *
 *                                                                          *
 ****************************************************************************/
__inline void Z80WriteWord(unsigned char *mem_map, unsigned short usAddr, unsigned short usWord)
{

   Z80WriteByte(mem_map, usAddr++, (unsigned char)(usWord & 0xff));
   Z80WriteByte(mem_map, usAddr, (unsigned char)(usWord >> 8));

} /* Z80WriteWord() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80IN(short)                                               *
 *                                                                          *
 *  PURPOSE    : Read a byte from a port, check for hardware.               *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char Z80IN(unsigned short usAddr)
{
   return (mem_handlers_z80[0].pfn_read)(usAddr);  /* Handler #0 is reserved for port access */
} /* Z80IN() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80OUT(short, char)                                        *
 *                                                                          *
 *  PURPOSE    : Write a byte to a port, check for hardware.                *
 *                                                                          *
 ****************************************************************************/
__inline void Z80OUT(unsigned short usAddr, unsigned char ucByte)
{
   (mem_handlers_z80[0].pfn_write)(usAddr, ucByte);  /* Handler #0 is reserved for port access */
} /* Z80OUT() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80ReadByte(char *, short)                                 *
 *                                                                          *
 *  PURPOSE    : Read a byte from memory, check for hardware.               *
 *                                                                          *
 ****************************************************************************/
__inline unsigned char Z80ReadByte(unsigned char *mem_map, unsigned short usAddr)
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
         return (mem_handlers_z80[c-2].pfn_read)(usAddr);
         break;
      }

} /* Z80ReadByte() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80ReadWord(char *, short)                                 *
 *                                                                          *
 *  PURPOSE    : Read a word from memory, check for hardware.               *
 *                                                                          *
 ****************************************************************************/
__inline unsigned short Z80ReadWord(unsigned char *mem_map, unsigned short usAddr)
{
unsigned short usWord;

   usWord = Z80ReadByte(mem_map, usAddr++);
   usWord += Z80ReadByte(mem_map, usAddr) * 256;
   return usWord;

} /* Z80ReadWord() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80PUSHW(char *, REGS6502 *)                               *
 *                                                                          *
 *  PURPOSE    : Push a word to the 'SP' stack.                             *
 *                                                                          *
 ****************************************************************************/
__inline void Z80PUSHW(unsigned char *mem_map, REGSZ80 *regs, unsigned short usWord)
{

   Z80WriteByte(mem_map, --regs->usRegSP, (unsigned char)(usWord >> 8));
   Z80WriteByte(mem_map, --regs->usRegSP, (unsigned char)(usWord & 0xff));

} /* Z80PUSHW() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : Z80POPW(char *, REGSZ80 *)                                 *
 *                                                                          *
 *  PURPOSE    : Pull a word from the 'SP' stack.                           *
 *                                                                          *
 ****************************************************************************/
__inline unsigned short Z80POPW(unsigned char *mem_map, REGSZ80 *regs)
{
unsigned char hi, lo;

   lo = Z80ReadByte(mem_map, regs->usRegSP++);
   hi = Z80ReadByte(mem_map, regs->usRegSP++);
   return (hi * 256 + lo);

} /* Z80POPW() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : RESETZ80(REGSZ80 *)                                        *
 *                                                                          *
 *  PURPOSE    : Setup the Z80 after a reset.                               *
 *                                                                          *
 ****************************************************************************/
void RESETZ80(REGSZ80 *regs)
{
   memset(regs, 0, sizeof(REGSZ80)); /* Start with a clean slate at reset */
   regs->usRegPC = 0; /* Start execution at reset vector */
   regs->usRegSP = 0xf000; /* Start stack at a reasonable place */
   regs->ucRegR = rand();

} /* RESETZ80() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : EXECZ80(char *, REGSZ80 *, EMUHANDLERS *, int *, char *, char)   *
 *                                                                          *
 *  PURPOSE    : Emulate the Z80 microprocessor for N clock cycles.         *
 *                                                                          *
 ****************************************************************************/
void EXECZ80(char *mem, REGSZ80 *regs, EMUHANDLERS *emuh, int *iClocks, unsigned char *ucIRQs, unsigned char cIRQValue)
{
unsigned short PC;  /* Current Program Counter address */
register unsigned short usAddr; /* Temp address */
register unsigned char ucTemp;
register unsigned short usTemp;
register signed short sTemp;
unsigned char ucOpcode, c;
unsigned short *pus;
unsigned char *puc, *puc2;

/* Set up opcode register indirection tables for easier instruction decode */
pusReg[0] = &regs->usRegBC;
pusReg[1] = &regs->usRegDE;
pusReg[2] = &regs->usRegHL;
pusReg[3] = &regs->usRegSP;

pucReg[0] = &regs->ucRegB;
pucReg[1] = &regs->ucRegC;
pucReg[2] = &regs->ucRegD;
pucReg[3] = &regs->ucRegE;
pucReg[4] = &regs->ucRegH;
pucReg[5] = &regs->ucRegL;
pucReg[6] = &regs->ucRegF;
pucReg[7] = &regs->ucRegA;


   mem_handlers_z80 = emuh; /* Assign to static for faster execution */
   m_map_z80 = mem; /* ditto */

   PC = regs->usRegPC;
   while (*iClocks > 0) /* Execute for the amount of time alloted */
      {
#ifdef TRACE
      regs->usRegPC = PC;
      TRACEZ80(regs);
#endif
      regs->ucRegR++;
      if (*ucIRQs)
         {
         if (*ucIRQs & INT_NMI)
            {
            *ucIRQs &= ~INT_NMI; /* acknowledge this NMI */
            Z80PUSHW(mem, regs, PC); /* Push PC and jump to 66H */
            PC = 0x66;
            ucOpcode = Z80ReadByte(mem, PC++);
            regs->ucRegIFF2 = regs->ucRegIFF1; /* Preserve maskable interrupt flag */
            regs->ucRegIFF1 = 0; /* Disable maskable interrupts */
            }
         if ((*ucIRQs & INT_IRQ) && regs->ucRegIFF1) /* If there is an interrupt pending */
            {
            *ucIRQs &= ~INT_IRQ;
            regs->ucRegIFF1 = regs->ucRegIFF2 = 0; /* Disable maskable interrupts */
            switch (regs->ucRegIM) /* Different interrupt mode handling */
               {
               case 0: /* Mode 0 has instruction placed on data bus */
                  ucOpcode = cIRQValue;
                  break;
               case 1: /* Mode 1 acts like an 8080 */
                  ucOpcode = 0xff; /* Do a RST 38H */
                  break;
               case 2: /* Mode 2 has a vector put on the data bus */
                  usAddr = regs->ucRegI * 256 + cIRQValue;
                  Z80PUSHW(mem, regs, PC);
                  PC = Z80ReadWord(mem, usAddr);  /* Jump to address */
                  ucOpcode = Z80ReadByte(mem, PC++);
                  break;
               }
            }
         else /* If can't do the interrupt, execute an instruction */
            ucOpcode = Z80ReadByte(mem, PC++);
         } /* Interrupt pending */
      else
         ucOpcode = Z80ReadByte(mem, PC++);

      *iClocks -= cZ80Cycles[ucOpcode];
      switch (ucOpcode)
         {
         case 0x00: /* NOP */
            break;

         case 0x01: /* LD BC,nn */
         case 0x11: /* LD DE,nn */
         case 0x21: /* LD HL,nn */
         case 0x31: /* LD SP,nn */
            pus = pusReg[ucOpcode >> 4];
            *pus = Z80ReadWord(mem, PC);
            PC += 2;
            break;

         case 0x02: /* LD (BC),A */
            Z80WriteByte(mem, regs->usRegBC, regs->ucRegA);
            break;

         case 0x03: /* INC BC */
         case 0x13: /* INC DE */
         case 0x23: /* INC HL */
         case 0x33: /* INC SP */
            pus = pusReg[ucOpcode >> 4];
            (*pus)++; /* No flags affected */
            break;

         case 0x04: /* INC B */
         case 0x0C: /* INC C */
         case 0x14: /* INC D */
         case 0x1C: /* INC E */
         case 0x24: /* INC H */
         case 0x2C: /* INC L */
         case 0x3C: /* INC A */
            puc = pucReg[ucOpcode >> 3];
            *puc = Z80INC(regs, *puc);
            break;

         case 0x05: /* DEC B */
         case 0x0D: /* DEC C */
         case 0x15: /* DEC D */
         case 0x1D: /* DEC E */
         case 0x25: /* DEC H */
         case 0x2D: /* DEC L */
         case 0x3D: /* DEC A */
            puc = pucReg[ucOpcode >> 3];
            *puc = Z80DEC(regs, *puc);
            break;

         case 0x06: /* LD B,n */
         case 0x0E: /* LD C,n */
         case 0x16: /* LD D,n */
         case 0x1E: /* LD E,n */
         case 0x26: /* LD H,n */
         case 0x2E: /* LD L,n */
         case 0x3E: /* LD A,n */
            puc = pucReg[ucOpcode >> 3];
            *puc = Z80ReadByte(mem, PC++);
            break;

         case 0x07: /* RLCA */
            Z80RLCA(regs);
            break;

         case 0x08: /* EX AF,AF' */
            usTemp = regs->usRegAF;
            regs->usRegAF = regs->usRegAF1;
            regs->usRegAF1 = usTemp;
            break;

         case 0x09: /* ADD HL,BC */
         case 0x19: /* ADD HL,DE */
         case 0x29: /* ADD HL,HL */
         case 0x39: /* ADD HL,SP */
            pus = pusReg[ucOpcode >> 4];
            regs->usRegHL = Z80ADD16(regs, regs->usRegHL, *pus);
            break;

         case 0x0A: /* LD A,(BC) */
            regs->ucRegA = Z80ReadByte(mem, regs->usRegBC);
            break;

         case 0x0B: /* DEC BC */
         case 0x1B: /* DEC DE */
         case 0x2B: /* DEC HL */
         case 0x3B: /* DEC SP */
            pus = pusReg[ucOpcode >> 4];
            (*pus)--; /* No flags affected */
            break;
         case 0x0F: /* RRCA */
            Z80RRCA(regs);
            break;
         case 0x10: /* DJNZ (PC+e) */
            sTemp = (signed short)(signed char)Z80ReadByte(mem, PC++);
            regs->ucRegB--;
            if (regs->ucRegB != 0)
               {
               PC += sTemp;
               *iClocks -= 5;
               }
            break;
         case 0x12: /* LD (DE),A */
            Z80WriteByte(mem, regs->usRegDE, regs->ucRegA);
            break;
         case 0x17: /* RLA */
            Z80RLA(regs);
            break;
         case 0x18: /* JR e */
            sTemp = (signed short)(signed char)Z80ReadByte(mem, PC++);
            PC += sTemp;
            break;
         case 0x1A: /* LD A,(DE) */
            regs->ucRegA = Z80ReadByte(mem, regs->usRegDE);
            break;
         case 0x1F: /* RRA */
            Z80RRA(regs);
            break;
         case 0x20: /* JR NZ,e */
            sTemp = (signed short)(signed char)Z80ReadByte(mem, PC++);
            if (!(regs->ucRegF & F_ZERO))
               {
               *iClocks -= 5;
               PC += sTemp;
               }
            break;
         case 0x22: /* LD (nn),HL */
            usAddr = Z80ReadWord(mem, PC);
            PC += 2;
            Z80WriteWord(mem, usAddr, regs->usRegHL);
            break;
         case 0x27: /* DAA */
            {
            unsigned char msn, lsn;
            unsigned short cf = 0;
            msn=regs->ucRegA & 0xf0; lsn=regs->ucRegA & 0x0f;
            if( lsn>0x09 || regs->ucRegF&0x10 ) cf |= 0x06;
            if( msn>0x80 && lsn>0x09 ) cf |= 0x60;
            if( msn>0x90 || regs->ucRegF&0x01 ) cf |= 0x60;
            usTemp = cf + regs->ucRegA;
            regs->ucRegF &= ~(F_HALFCARRY | F_CARRY | F_SIGN | F_ZERO | F_OVERFLOW);
            if (usTemp & 0x100)
               regs->ucRegF |= F_CARRY;
            regs->ucRegA = (unsigned char)usTemp;
            regs->ucRegF |= cZ80SZ[regs->ucRegA];
            regs->ucRegF |= cZ80Parity[regs->ucRegA];
            }
            break;
         case 0x28: /* JR Z,e */
            sTemp = (signed short)(signed char)Z80ReadByte(mem, PC++);
            if (regs->ucRegF & F_ZERO)
               {
               *iClocks -= 5;
               PC += sTemp;
               }
            break;
         case 0x2A: /* LD HL,(nn) */
            usAddr = Z80ReadWord(mem, PC);
            PC += 2;
            regs->usRegHL = Z80ReadWord(mem, usAddr);
            break;
         case 0x2F: /* CPL */
            regs->ucRegA = 255 - regs->ucRegA;
            regs->ucRegF |= (F_HALFCARRY | F_ADDSUB);
            break;
         case 0x30: /* JR NC,e */
            sTemp = (signed short)(signed char)Z80ReadByte(mem, PC++);
            if (!(regs->ucRegF & F_CARRY))
               {
               *iClocks -= 5;
               PC += sTemp;
               }
            break;
         case 0x32: /* LD (nn),A */
            usAddr = Z80ReadWord(mem, PC);
            PC += 2;
            Z80WriteByte(mem, usAddr, regs->ucRegA);
            break;
         case 0x34: /* INC (HL) */
            ucTemp = Z80ReadByte(mem, regs->usRegHL);
            Z80WriteByte(mem, regs->usRegHL, Z80INC(regs, ucTemp));
            break;
         case 0x35: /* DEC (HL) */
            ucTemp = Z80ReadByte(mem, regs->usRegHL);
            Z80WriteByte(mem, regs->usRegHL, Z80DEC(regs, ucTemp));
            break;
         case 0x36: /* LD (HL),n */
            ucTemp = Z80ReadByte(mem, PC++);
            Z80WriteByte(mem, regs->usRegHL, ucTemp);
            break;
         case 0x37: /* SCF */
            regs->ucRegF |= F_CARRY;
            regs->ucRegF &= ~(F_HALFCARRY | F_ADDSUB);
            break;
         case 0x38: /* JR C,e */
            sTemp = (signed short)(signed char)Z80ReadByte(mem, PC++);
            if (regs->ucRegF & F_CARRY)
               {
               *iClocks -= 5;
               PC += sTemp;
               }
            break;
         case 0x3A: /* LD A,(nn) */
            usAddr = Z80ReadWord(mem, PC);
            PC += 2;
            regs->ucRegA = Z80ReadByte(mem, usAddr);
            break;
         case 0x3F: /* CCF */
            regs->ucRegF ^= F_CARRY;
            regs->ucRegF &= ~F_ADDSUB;
            break;
         case 0x40: /* LD B,B */
         case 0x41: /* LD B,C */
         case 0x42: /* LD B,D */
         case 0x43: /* LD B,E */
         case 0x44: /* LD B,H */
         case 0x45: /* LD B,L */
         case 0x47: /* LD B,A */
         case 0x48: /* LD C,B */
         case 0x49: /* LD C,C */
         case 0x4A: /* LD C,D */
         case 0x4B: /* LD C,E */
         case 0x4C: /* LD C,H */
         case 0x4D: /* LD C,L */
         case 0x4F: /* LD C,A */
         case 0x50: /* LD D,B */
         case 0x51: /* LD D,C */
         case 0x52: /* LD D,D */
         case 0x53: /* LD D,E */
         case 0x54: /* LD D,H */
         case 0x55: /* LD D,L */
         case 0x57: /* LD D,A */
         case 0x58: /* LD E,B */
         case 0x59: /* LD E,C */
         case 0x5A: /* LD E,D */
         case 0x5B: /* LD E,E */
         case 0x5C: /* LD E,H */
         case 0x5D: /* LD E,L */
         case 0x5F: /* LD E,A */
         case 0x60: /* LD H,B */
         case 0x61: /* LD H,C */
         case 0x62: /* LD H,D */
         case 0x63: /* LD H,E */
         case 0x64: /* LD H,H */
         case 0x65: /* LD H,L */
         case 0x67: /* LD H,A */
         case 0x68: /* LD L,B */
         case 0x69: /* LD L,C */
         case 0x6A: /* LD L,D */
         case 0x6B: /* LD L,E */
         case 0x6C: /* LD L,H */
         case 0x6D: /* LD L,L */
         case 0x6F: /* LD L,A */
         case 0x78: /* LD A,B */
         case 0x79: /* LD A,C */
         case 0x7A: /* LD A,D */
         case 0x7B: /* LD A,E */
         case 0x7C: /* LD A,H */
         case 0x7D: /* LD A,L */
         case 0x7F: /* LD A,A */
            puc = pucReg[ucOpcode & 7]; /* Source register */
            puc2 = pucReg[(ucOpcode >> 3) & 7]; /* Destination register */
            *puc2 = *puc;
            break;
         case 0x46: /* LD B,(HL) */
         case 0x4E: /* LD C,(HL) */
         case 0x56: /* LD D,(HL) */
         case 0x5E: /* LD E,(HL) */
         case 0x66: /* LD H,(HL) */
         case 0x6E: /* LD L,(HL) */
         case 0x7E: /* LD A,(HL) */
            puc = pucReg[(ucOpcode >> 3) & 7];
            *puc = Z80ReadByte(mem, regs->usRegHL);
            break;
         case 0x70: /* LD (HL),B */
         case 0x71: /* LD (HL),C */
         case 0x72: /* LD (HL),D */
         case 0x73: /* LD (HL),E */
         case 0x74: /* LD (HL),H */
         case 0x75: /* LD (HL),L */
         case 0x77: /* LD (HL),A */
            puc = pucReg[ucOpcode & 7];
            Z80WriteByte(mem, regs->usRegHL, *puc);
            break;
         case 0x76: /* HALT */
            break;
         case 0x80: /* ADD A,B */
         case 0x81: /* ADD A,C */
         case 0x82: /* ADD A,D */
         case 0x83: /* ADD A,E */
         case 0x84: /* ADD A,H */
         case 0x85: /* ADD A,L */
         case 0x87: /* ADD A,A */
            puc = pucReg[ucOpcode & 7];
            Z80ADD(regs, *puc);
            break;
         case 0x86: /* ADD A,(HL) */
            Z80ADD(regs, Z80ReadByte(mem, regs->usRegHL));
            break;
         case 0x88: /* ADC A,B */
         case 0x89: /* ADC A,C */
         case 0x8A: /* ADC A,D */
         case 0x8B: /* ADC A,E */
         case 0x8C: /* ADC A,H */
         case 0x8D: /* ADC A,L */
         case 0x8F: /* ADC A,A */
            puc = pucReg[ucOpcode & 7];
            Z80ADC(regs, *puc);
            break;
         case 0x8E: /* ADC A,(HL) */
            Z80ADC(regs, Z80ReadByte(mem, regs->usRegHL));
            break;
         case 0x90: /* SUB B */
         case 0x91: /* SUB C */
         case 0x92: /* SUB D */
         case 0x93: /* SUB E */
         case 0x94: /* SUB H */
         case 0x95: /* SUB L */
         case 0x97: /* SUB A */
            puc = pucReg[ucOpcode & 7];
            Z80SUB(regs, *puc);
            break;
         case 0x96: /* SUB (HL) */
            Z80SUB(regs, Z80ReadByte(mem, regs->usRegHL));
            break;
         case 0x98: /* SBC A,B */
         case 0x99: /* SBC A,C */
         case 0x9A: /* SBC A,D */
         case 0x9B: /* SBC A,E */
         case 0x9C: /* SBC A,H */
         case 0x9D: /* SBC A,L */
         case 0x9F: /* SBC A,A */
            puc = pucReg[ucOpcode & 7];
            Z80SBC(regs, *puc);
            break;
         case 0x9E: /* SBC A,(HL) */
            Z80SBC(regs, Z80ReadByte(mem, regs->usRegHL));
            break;
         case 0xA0: /* AND B */
         case 0xA1: /* AND C */
         case 0xA2: /* AND D */
         case 0xA3: /* AND E */
         case 0xA4: /* AND H */
         case 0xA5: /* AND L */
         case 0xA7: /* AND A */
            puc = pucReg[ucOpcode & 7];
            Z80AND(regs, *puc);
            break;
         case 0xA6: /* AND (HL) */
            Z80AND(regs, Z80ReadByte(mem, regs->usRegHL));
            break;
         case 0xA8: /* XOR B */
         case 0xA9: /* XOR C */
         case 0xAA: /* XOR D */
         case 0xAB: /* XOR E */
         case 0xAC: /* XOR H */
         case 0xAD: /* XOR L */
         case 0xAF: /* XOR A */
            puc = pucReg[ucOpcode & 7];
            Z80XOR(regs, *puc);
            break;
         case 0xAE: /* XOR (HL) */
            Z80XOR(regs, Z80ReadByte(mem, regs->usRegHL));
            break;
         case 0xB0: /* OR B */
         case 0xB1: /* OR C */
         case 0xB2: /* OR D */
         case 0xB3: /* OR E */
         case 0xB4: /* OR H */
         case 0xB5: /* OR L */
         case 0xB7: /* OR A */
            puc = pucReg[ucOpcode & 7];
            Z80OR(regs, *puc);
            break;
         case 0xB6: /* OR (HL) */
            Z80OR(regs, Z80ReadByte(mem, regs->usRegHL));
            break;
         case 0xB8: /* CP B */
         case 0xB9: /* CP C */
         case 0xBA: /* CP D */
         case 0xBB: /* CP E */
         case 0xBC: /* CP H */
         case 0xBD: /* CP L */
         case 0xBF: /* CP A */
            puc = pucReg[ucOpcode & 7];
            Z80CMP(regs, *puc);
            break;
         case 0xBE: /* CP (HL) */
            Z80CMP(regs, Z80ReadByte(mem, regs->usRegHL));
            break;
         case 0xC0: /* RET NZ */
            if (!(regs->ucRegF & F_ZERO))
               {
               PC = Z80POPW(mem, regs);
               *iClocks -= 6;
               }
            break;
         case 0xC1: /* POP BC */
            regs->usRegBC = Z80POPW(mem, regs);
            break;
         case 0xC2: /* JP NZ,(nn) */
            if (!(regs->ucRegF & F_ZERO))
               {
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 5;
               }
            else
               PC += 2;
            break;
         case 0xC3: /* JP (nn) */
            PC = Z80ReadWord(mem, PC);
            break;
         case 0xC4: /* CALL NZ,(nn) */
            if (!(regs->ucRegF & F_ZERO))
               {
               Z80PUSHW(mem, regs, (unsigned short)(PC+2));
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 7;
               }
            else
               PC += 2;
            break;
         case 0xC5: /* PUSH BC */
            Z80PUSHW(mem, regs, regs->usRegBC);
            break;
         case 0xC6: /* ADD A,n */
            ucTemp = Z80ReadByte(mem, PC++);
            Z80ADD(regs, ucTemp);
            break;
         case 0xC7: /* RST 0H */
         case 0xCF: /* RST 8H */
         case 0xD7: /* RST 10H */
         case 0xDF: /* RST 18H */
         case 0xE7: /* RST 20H */
         case 0xEF: /* RST 28H */
         case 0xF7: /* RST 30H */
         case 0xFF: /* RST 38H */
            Z80PUSHW(mem, regs, PC);
            PC = ucOpcode & 0x38;
            break;
         case 0xC8: /* RET Z */
            if (regs->ucRegF & F_ZERO)
               {
               PC = Z80POPW(mem, regs);
               *iClocks -= 6;
               }
            break;
         case 0xC9: /* RET */
            PC = Z80POPW(mem, regs);
            break;
         case 0xCA: /* JP Z,(nn) */
            if (regs->ucRegF & F_ZERO)
               {
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 5;
               }
            else
               PC += 2;
            break;
         case 0xCB: /* New set of opcodes branches here */
            ucOpcode = Z80ReadByte(mem, PC++);
            regs->ucRegR++;
            switch(ucOpcode)
               {
               case 0x00: /* RLC B */
               case 0x01: /* RLC C */
               case 0x02: /* RLC D */
               case 0x03: /* RLC E */
               case 0x04: /* RLC H */
               case 0x05: /* RLC L */
               case 0x07: /* RLC A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  *puc = Z80RLC(regs, *puc);
                  break;
               case 0x06: /* RLC (HL) */
                  *iClocks -= 15;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegHL,Z80RLC(regs, ucTemp));
                  break;
               case 0x08: /* RRC B */
               case 0x09: /* RRC C */
               case 0x0A: /* RRC D */
               case 0x0B: /* RRC E */
               case 0x0C: /* RRC H */
               case 0x0D: /* RRC L */
               case 0x0F: /* RRC A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  *puc = Z80RLC(regs, *puc);
                  break;
               case 0x0E: /* RRC (HL) */
                  *iClocks -= 15;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegHL,Z80RRC(regs, ucTemp));
                  break;
               case 0x10: /* RL B */
               case 0x11: /* RL C */
               case 0x12: /* RL D */
               case 0x13: /* RL E */
               case 0x14: /* RL H */
               case 0x15: /* RL L */
               case 0x17: /* RL A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  *puc = Z80RL(regs, *puc);
                  break;
               case 0x16: /* RL (HL) */
                  *iClocks -= 15;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegHL,Z80RL(regs, ucTemp));
                  break;
               case 0x18: /* RR B */
               case 0x19: /* RR C */
               case 0x1A: /* RR D */
               case 0x1B: /* RR E */
               case 0x1C: /* RR H */
               case 0x1D: /* RR L */
               case 0x1F: /* RR A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  *puc = Z80RR(regs, *puc);
                  break;
               case 0x1E: /* RR (HL) */
                  *iClocks -= 15;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegHL,Z80RR(regs, ucTemp));
                  break;
               case 0x20: /* SLA B */
               case 0x21: /* SLA C */
               case 0x22: /* SLA D */
               case 0x23: /* SLA E */
               case 0x24: /* SLA H */
               case 0x25: /* SLA L */
               case 0x27: /* SLA A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  *puc = Z80SLA(regs, *puc);
                  break;
               case 0x26: /* SLA (HL) */
                  *iClocks -= 15;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegHL, Z80SLA(regs, ucTemp));
                  break;
               case 0x28: /* SRA B */
               case 0x29: /* SRA C */
               case 0x2A: /* SRA D */
               case 0x2B: /* SRA E */
               case 0x2C: /* SRA H */
               case 0x2D: /* SRA L */
               case 0x2F: /* SRA A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  *puc = Z80SRA(regs, *puc);
                  break;
               case 0x2E: /* SRA (HL) */
                  *iClocks -= 15;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegHL,Z80SRA(regs, ucTemp));
                  break;
               case 0x38: /* SRL B */
               case 0x39: /* SRL C */
               case 0x3A: /* SRL D */
               case 0x3B: /* SRL E */
               case 0x3C: /* SRL H */
               case 0x3D: /* SRL L */
               case 0x3F: /* SRL A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  *puc = Z80SRL(regs, *puc);
                  break;
               case 0x3E: /* SRL (HL) */
                  *iClocks -= 15;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegHL,Z80SRL(regs, ucTemp));
                  break;

               case 0x40: /* BIT 0,B */
               case 0x41: /* BIT 0,C */
               case 0x42: /* BIT 0,D */
               case 0x43: /* BIT 0,H */
               case 0x44: /* BIT 0,H */
               case 0x45: /* BIT 0,L */
               case 0x47: /* BIT 0,A */
               case 0x48: /* BIT 1,B */
               case 0x49: /* BIT 1,C */
               case 0x4A: /* BIT 1,D */
               case 0x4B: /* BIT 1,E */
               case 0x4C: /* BIT 1,H */
               case 0x4D: /* BIT 1,L */
               case 0x4F: /* BIT 1,A */
               case 0x50: /* BIT 2,B */
               case 0x51: /* BIT 2,C */
               case 0x52: /* BIT 2,D */
               case 0x53: /* BIT 2,H */
               case 0x54: /* BIT 2,H */
               case 0x55: /* BIT 2,L */
               case 0x57: /* BIT 2,A */
               case 0x58: /* BIT 3,B */
               case 0x59: /* BIT 3,C */
               case 0x5A: /* BIT 3,D */
               case 0x5B: /* BIT 3,E */
               case 0x5C: /* BIT 3,H */
               case 0x5D: /* BIT 3,L */
               case 0x5F: /* BIT 3,A */
               case 0x60: /* BIT 4,B */
               case 0x61: /* BIT 4,C */
               case 0x62: /* BIT 4,D */
               case 0x63: /* BIT 4,H */
               case 0x64: /* BIT 4,H */
               case 0x65: /* BIT 4,L */
               case 0x67: /* BIT 4,A */
               case 0x68: /* BIT 5,B */
               case 0x69: /* BIT 5,C */
               case 0x6A: /* BIT 5,D */
               case 0x6B: /* BIT 5,E */
               case 0x6C: /* BIT 5,H */
               case 0x6D: /* BIT 5,L */
               case 0x6F: /* BIT 5,A */
               case 0x70: /* BIT 6,B */
               case 0x71: /* BIT 6,C */
               case 0x72: /* BIT 6,D */
               case 0x73: /* BIT 6,H */
               case 0x74: /* BIT 6,H */
               case 0x75: /* BIT 6,L */
               case 0x77: /* BIT 6,A */
               case 0x78: /* BIT 7,B */
               case 0x79: /* BIT 7,C */
               case 0x7A: /* BIT 7,D */
               case 0x7B: /* BIT 7,E */
               case 0x7C: /* BIT 7,H */
               case 0x7D: /* BIT 7,L */
               case 0x7F: /* BIT 7,A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  ucTemp = (ucOpcode >> 3) & 7;
                  regs->ucRegF &= ~(F_ADDSUB | F_ZERO);
                  regs->ucRegF |= F_HALFCARRY;
                  if (!(*puc & (1<<ucTemp)))
                     regs->ucRegF |= F_ZERO;
                  break;

               case 0x46: /* BIT 0,(HL) */
               case 0x4E: /* BIT 1,(HL) */
               case 0x56: /* BIT 2,(HL) */
               case 0x5E: /* BIT 3,(HL) */
               case 0x66: /* BIT 4,(HL) */
               case 0x6E: /* BIT 5,(HL) */
               case 0x76: /* BIT 6,(HL) */
               case 0x7E: /* BIT 7,(HL) */
                  *iClocks -= 12;
                  ucTemp = (ucOpcode >> 3) & 7;
                  c = Z80ReadByte(mem, regs->usRegHL);
                  regs->ucRegF &= ~(F_ADDSUB | F_ZERO);
                  regs->ucRegF |= F_HALFCARRY;
                  if (!(c & (1<<ucTemp)))
                     regs->ucRegF |= F_ZERO;
                  break;

               case 0x80: /* RES 0,B */
               case 0x81: /* RES 0,C */
               case 0x82: /* RES 0,D */
               case 0x83: /* RES 0,E */
               case 0x84: /* RES 0,H */
               case 0x85: /* RES 0,L */
               case 0x87: /* RES 0,A */
               case 0x88: /* RES 1,B */
               case 0x89: /* RES 1,C */
               case 0x8A: /* RES 1,D */
               case 0x8B: /* RES 1,E */
               case 0x8C: /* RES 1,H */
               case 0x8D: /* RES 1,L */
               case 0x8F: /* RES 1,A */
               case 0x90: /* RES 2,B */
               case 0x91: /* RES 2,C */
               case 0x92: /* RES 2,D */
               case 0x93: /* RES 2,E */
               case 0x94: /* RES 2,H */
               case 0x95: /* RES 2,L */
               case 0x97: /* RES 2,A */
               case 0x98: /* RES 3,B */
               case 0x99: /* RES 3,C */
               case 0x9A: /* RES 3,D */
               case 0x9B: /* RES 3,E */
               case 0x9C: /* RES 3,H */
               case 0x9D: /* RES 3,L */
               case 0x9F: /* RES 3,A */
               case 0xA0: /* RES 4,B */
               case 0xA1: /* RES 4,C */
               case 0xA2: /* RES 4,D */
               case 0xA3: /* RES 4,E */
               case 0xA4: /* RES 4,H */
               case 0xA5: /* RES 4,L */
               case 0xA7: /* RES 4,A */
               case 0xA8: /* RES 5,B */
               case 0xA9: /* RES 5,C */
               case 0xAA: /* RES 5,D */
               case 0xAB: /* RES 5,E */
               case 0xAC: /* RES 5,H */
               case 0xAD: /* RES 5,L */
               case 0xAF: /* RES 5,A */
               case 0xB0: /* RES 6,B */
               case 0xB1: /* RES 6,C */
               case 0xB2: /* RES 6,D */
               case 0xB3: /* RES 6,E */
               case 0xB4: /* RES 6,H */
               case 0xB5: /* RES 6,L */
               case 0xB7: /* RES 6,A */
               case 0xB8: /* RES 7,B */
               case 0xB9: /* RES 7,C */
               case 0xBA: /* RES 7,D */
               case 0xBB: /* RES 7,E */
               case 0xBC: /* RES 7,H */
               case 0xBD: /* RES 7,L */
               case 0xBF: /* RES 7,A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  ucTemp = (ucOpcode >> 3) & 7;
                  *puc &= ~(1<<ucTemp);
                  break;

               case 0x86: /* RES 0,(HL) */
               case 0x8E: /* RES 1,(HL) */
               case 0x96: /* RES 2,(HL) */
               case 0x9E: /* RES 3,(HL) */
               case 0xA6: /* RES 4,(HL) */
               case 0xAE: /* RES 5,(HL) */
               case 0xB6: /* RES 6,(HL) */
               case 0xBE: /* RES 7,(HL) */
                  *iClocks -= 15;
                  ucTemp = (ucOpcode >> 3) & 7;
                  c = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegHL, (unsigned char)(c & ~(1<<ucTemp)));
                  break;

               case 0xC0: /* SET 0,B */
               case 0xC1: /* SET 0,C */
               case 0xC2: /* SET 0,D */
               case 0xC3: /* SET 0,E */
               case 0xC4: /* SET 0,H */
               case 0xC5: /* SET 0,L */
               case 0xC7: /* SET 0,A */
               case 0xC8: /* SET 1,B */
               case 0xC9: /* SET 1,C */
               case 0xCA: /* SET 1,D */
               case 0xCB: /* SET 1,E */
               case 0xCC: /* SET 1,H */
               case 0xCD: /* SET 1,L */
               case 0xCF: /* SET 1,A */
               case 0xD0: /* SET 2,B */
               case 0xD1: /* SET 2,C */
               case 0xD2: /* SET 2,D */
               case 0xD3: /* SET 2,E */
               case 0xD4: /* SET 2,H */
               case 0xD5: /* SET 2,L */
               case 0xD7: /* SET 2,A */
               case 0xD8: /* SET 3,B */
               case 0xD9: /* SET 3,C */
               case 0xDA: /* SET 3,D */
               case 0xDB: /* SET 3,E */
               case 0xDC: /* SET 3,H */
               case 0xDD: /* SET 3,L */
               case 0xDF: /* SET 3,A */
               case 0xE0: /* SET 4,B */
               case 0xE1: /* SET 4,C */
               case 0xE2: /* SET 4,D */
               case 0xE3: /* SET 4,E */
               case 0xE4: /* SET 4,H */
               case 0xE5: /* SET 4,L */
               case 0xE7: /* SET 4,A */
               case 0xE8: /* SET 5,B */
               case 0xE9: /* SET 5,C */
               case 0xEA: /* SET 5,D */
               case 0xEB: /* SET 5,E */
               case 0xEC: /* SET 5,H */
               case 0xED: /* SET 5,L */
               case 0xEF: /* SET 5,A */
               case 0xF0: /* SET 6,B */
               case 0xF1: /* SET 6,C */
               case 0xF2: /* SET 6,D */
               case 0xF3: /* SET 6,E */
               case 0xF4: /* SET 6,H */
               case 0xF5: /* SET 6,L */
               case 0xF7: /* SET 6,A */
               case 0xF8: /* SET 7,B */
               case 0xF9: /* SET 7,C */
               case 0xFA: /* SET 7,D */
               case 0xFB: /* SET 7,E */
               case 0xFC: /* SET 7,H */
               case 0xFD: /* SET 7,L */
               case 0xFF: /* SET 7,A */
                  *iClocks -= 8;
                  puc = pucReg[ucOpcode & 7];
                  ucTemp = (ucOpcode >> 3) & 7;
                  *puc |= (1<<ucTemp);
                  break;

               case 0xC6: /* SET 0,(HL) */
               case 0xCE: /* SET 1,(HL) */
               case 0xD6: /* SET 2,(HL) */
               case 0xDE: /* SET 3,(HL) */
               case 0xE6: /* SET 4,(HL) */
               case 0xEE: /* SET 5,(HL) */
               case 0xF6: /* SET 6,(HL) */
               case 0xFE: /* SET 7,(HL) */
                  *iClocks -= 15;
                  ucTemp = (ucOpcode >> 3) & 7;
                  c = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegHL, (unsigned char)(c | (1<<ucTemp)));
                  break;
               default: /* ILLEGAL */
                  *iClocks = 0;
                  break;
               } /* switch on CB */
            break;
         case 0xCC: /* CALL Z,(nn) */
            if (regs->ucRegF & F_ZERO)
               {
               Z80PUSHW(mem, regs, (unsigned short)(PC+2));
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 7;
               }
            else
               PC += 2;
            break;
         case 0xCD: /* CALL (nn) */
            Z80PUSHW(mem, regs, (unsigned short)(PC+2));
            PC = Z80ReadWord(mem, PC);
            break;
         case 0xCE: /* ADC A,n */
            Z80ADC(regs, Z80ReadByte(mem, PC++));
            break;
         case 0xD0: /* RET NC */
            if (!(regs->ucRegF & F_CARRY))
               {
               PC = Z80POPW(mem, regs);
               *iClocks -= 6;
               }
            break;
         case 0xD1: /* POP DE */
            regs->usRegDE = Z80POPW(mem, regs);
            break;
         case 0xD2: /* JP NC,(nn) */
            if (!(regs->ucRegF & F_CARRY))
               {
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 5;
               }
            else
               PC += 2;
            break;
         case 0xD3: /* OUT (n),A */
            Z80OUT(Z80ReadByte(mem, PC++), regs->ucRegA);
            break;
         case 0xD4: /* CALL NC,(nn) */
            if (!(regs->ucRegF & F_CARRY))
               {
               Z80PUSHW(mem, regs, (unsigned short)(PC+2));
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 7;
               }
            else
               PC += 2;
            break;
         case 0xD5: /* PUSH DE */
            Z80PUSHW(mem, regs, regs->usRegDE);
            break;
         case 0xD6: /* SUB n */
            Z80SUB(regs, Z80ReadByte(mem, PC++));
            break;
         case 0xD8: /* RET C */
            if (regs->ucRegF & F_CARRY)
               {
               PC = Z80POPW(mem, regs);
               *iClocks -= 6;
               }
            break;
         case 0xD9: /* EXX */
            usTemp = regs->usRegBC;
            regs->usRegBC = regs->usRegBC1;
            regs->usRegBC1 = usTemp;
            usTemp = regs->usRegDE;
            regs->usRegDE = regs->usRegDE1;
            regs->usRegDE1 = usTemp;
            usTemp = regs->usRegHL;
            regs->usRegHL = regs->usRegHL1;
            regs->usRegHL1 = usTemp;
            break;
         case 0xDA: /* JP C,(nn) */
            if (regs->ucRegF & F_CARRY)
               {
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 5;
               }
            else
               PC += 2;
            break;
         case 0xDB: /* IN A,(n) */
            regs->ucRegA = Z80IN(Z80ReadByte(mem, PC++));
            break;
         case 0xDC: /* CALL C,(nn) */
            if (regs->ucRegF & F_CARRY)
               {
               Z80PUSHW(mem, regs, (unsigned short)(PC+2));
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 7;
               }
            else
               PC += 2;
            break;
         case 0xDD: /* A bunch of opcodes branch from here */
            ucOpcode = Z80ReadByte(mem, PC++);
            regs->ucRegR++;
            *iClocks -= cZ80Cycles2[ucOpcode];
            switch(ucOpcode)
               {
               case 0x09: /* ADD IX,BC */
                  regs->usRegIX = Z80ADD16(regs, regs->usRegIX, regs->usRegBC);
                  break;
               case 0x19: /* ADD IX,DE */
                  regs->usRegIX = Z80ADD16(regs, regs->usRegIX, regs->usRegDE);
                  break;
               case 0x29: /* ADD IX,IX */
                  regs->usRegIX = Z80ADD16(regs, regs->usRegIX, regs->usRegIX);
                  break;
               case 0x39: /* ADD IX,SP */
                  regs->usRegIX = Z80ADD16(regs, regs->usRegIX, regs->usRegSP);
                  break;
               case 0x21: /* LD IX,nn */
                  regs->usRegIX = Z80ReadWord(mem, PC);
                  PC += 2;
                  break;
               case 0x22: /* LD (nn),IX */
                  usAddr = Z80ReadWord(mem, PC);
                  PC += 2;
                  Z80WriteWord(mem, usAddr, regs->usRegIX);
                  break;
               case 0x23: /* INC IX */
                  regs->usRegIX++; /* No flags affected */
                  break;
               case 0x2A: /* LD IX,(nn) */
                  usAddr = Z80ReadWord(mem, PC);
                  PC += 2;
                  regs->usRegIX = Z80ReadWord(mem, usAddr);
                  break;
               case 0x2B: /* DEC IX */
                  regs->usRegIX--; /* No flags affected */
                  break;
               case 0x34: /* INC (IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  ucTemp = Z80ReadByte(mem, usAddr);
                  Z80WriteByte(mem, usAddr, Z80INC(regs, ucTemp));
                  break;
               case 0x35: /* DEC (IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  ucTemp = Z80ReadByte(mem, usAddr);
                  Z80WriteByte(mem, usAddr, Z80DEC(regs, ucTemp));
                  break;
               case 0x36: /* LD (IX+d),n */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  ucTemp = Z80ReadByte(mem, PC++);
                  Z80WriteByte(mem, usAddr, ucTemp);
                  break;
               case 0x46: /* LD B,(IX+d) */
               case 0x4E: /* LD C,(IX+d) */
               case 0x56: /* LD D,(IX+d) */
               case 0x5E: /* LD E,(IX+d) */
               case 0x66: /* LD H,(IX+d) */
               case 0x6E: /* LD L,(IX+d) */
               case 0x7E: /* LD A,(IX+d) */
                  puc = pucReg[(ucOpcode >> 3) & 7];
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  *puc = Z80ReadByte(mem, usAddr);
                  break;
               case 0x70: /* LD (IX+d),B */
               case 0x71: /* LD (IX+d),C */
               case 0x72: /* LD (IX+d),D */
               case 0x73: /* LD (IX+d),E */
               case 0x74: /* LD (IX+d),H */
               case 0x75: /* LD (IX+d),L */
               case 0x77: /* LD (IX+d),A */
                  puc = pucReg[ucOpcode & 7];
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80WriteByte(mem, usAddr, *puc);
                  break;
               case 0x86: /* ADD A,(IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80ADD(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0x8E: /* ADC A,(IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80ADC(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0x96: /* SUB A,(IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80SUB(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0x9E: /* SBC A,(IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80SBC(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xA6: /* AND (IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80AND(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xAE: /* XOR (IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80XOR(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xB6: /* OR (IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80OR(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xBE: /* CP (IX+d) */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80CMP(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xE1: /* POP IX */
                  regs->usRegIX = Z80POPW(mem, regs);
                  break;
               case 0xE3: /* EX (SP),IX */
                  usTemp = Z80ReadWord(mem, regs->usRegSP);
                  Z80WriteWord(mem, regs->usRegSP, regs->usRegIX);
                  regs->usRegIX = usTemp;
                  break;
               case 0xE5: /* PUSH IX */
                  Z80PUSHW(mem, regs, regs->usRegIX);
                  break;
               case 0xE9: /* JP (IX) */
                  PC = regs->usRegIX;
                  break;
               case 0xF9: /* LD SP,IX */
                  regs->usRegSP = regs->usRegIX;
                  break;
               case 0xCB: /* Another set of opcodes branches from here */
               /* They all use (IX+d), so load the address */
                  usAddr = regs->usRegIX + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  ucTemp = Z80ReadByte(mem, usAddr);
                  ucOpcode = Z80ReadByte(mem, PC++);
                  *iClocks -= cZ80Cycles3[ucOpcode];
                  switch (ucOpcode)
                     {
                     case 0x06: /* RLC (IX+d) */
                        Z80WriteByte(mem, usAddr, Z80RLC(regs, ucTemp));
                        break;
                     case 0x0E: /* RRC (IX+d) */
                        Z80WriteByte(mem, usAddr, Z80RRC(regs, ucTemp));
                        break;
                     case 0x16: /* RL (IX+d) */
                        Z80WriteByte(mem, usAddr, Z80RL(regs, ucTemp));
                        break;
                     case 0x1E: /* RR (IX+d) */
                        Z80WriteByte(mem, usAddr, Z80RR(regs, ucTemp));
                        break;
                     case 0x26: /* SLA (IX+d) */
                        Z80WriteByte(mem, usAddr, Z80SLA(regs, ucTemp));
                        break;
                     case 0x2E: /* SRA (IX+d) */
                        Z80WriteByte(mem, usAddr, Z80SRA(regs, ucTemp));
                        break;
                     case 0x3E: /* SRL (IX+d) */
                        Z80WriteByte(mem, usAddr, Z80SRL(regs, ucTemp));
                        break;
                     case 0x46: /* BIT 0,(IX+d) */
                     case 0x4E: /* BIT 1,(IX+d) */
                     case 0x56: /* BIT 2,(IX+d) */
                     case 0x5E: /* BIT 3,(IX+d) */
                     case 0x66: /* BIT 4,(IX+d) */
                     case 0x6E: /* BIT 5,(IX+d) */
                     case 0x76: /* BIT 6,(IX+d) */
                     case 0x7E: /* BIT 7,(IX+d) */
                        c = (ucOpcode >> 3) & 7;
                        regs->ucRegF &= ~(F_ADDSUB | F_ZERO);
                        regs->ucRegF |= F_HALFCARRY;
                        if (!(ucTemp & (1<<c)))
                           regs->ucRegF |= F_ZERO;
                        break;
                     case 0x86: /* RES 0,(IX+d) */
                     case 0x8E: /* RES 1,(IX+d) */
                     case 0x96: /* RES 2,(IX+d) */
                     case 0x9E: /* RES 3,(IX+d) */
                     case 0xA6: /* RES 4,(IX+d) */
                     case 0xAE: /* RES 5,(IX+d) */
                     case 0xB6: /* RES 6,(IX+d) */
                     case 0xBE: /* RES 7,(IX+d) */
                        c = (ucOpcode >> 3) & 7;
                        ucTemp &= ~(1<<c);
                        Z80WriteByte(mem, usAddr, ucTemp);
                        break;
                     case 0xC6: /* SET 0,(IX+d) */
                     case 0xCE: /* SET 1,(IX+d) */
                     case 0xD6: /* SET 2,(IX+d) */
                     case 0xDE: /* SET 3,(IX+d) */
                     case 0xE6: /* SET 4,(IX+d) */
                     case 0xEE: /* SET 5,(IX+d) */
                     case 0xF6: /* SET 6,(IX+d) */
                     case 0xFE: /* SET 7,(IX+d) */
                        c = (ucOpcode >> 3) & 7;
                        ucTemp |= (1<<c);
                        Z80WriteByte(mem, usAddr, ucTemp);
                        break;
                     default: /* ILLEGAL */
                        *iClocks = 0;
                        break;
                     } /* switch on DDCB */
                  break;
               default: /* ILLEGAL */
                  *iClocks = 0;
                  break;
               } /* switch on DD */
            break;
         case 0xDE: /* SBC A,n */
            ucTemp = Z80ReadByte(mem, PC++);
            Z80SBC(regs, ucTemp);
            break;
         case 0xE0: /* RET PO */
            if (!(regs->ucRegF & F_OVERFLOW))
               {
               PC = Z80POPW(mem, regs);
               *iClocks -= 6;
               }
            break;
         case 0xE1: /* POP HL */
            regs->usRegHL = Z80POPW(mem, regs);
            break;
         case 0xE2: /* JP PO,(nn) */
            if (!(regs->ucRegF & F_OVERFLOW))
               {
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 5;
               }
            else
               PC += 2;
            break;
         case 0xE3: /* EX (SP),HL */
            usTemp = Z80ReadWord(mem, regs->usRegSP);
            Z80WriteWord(mem, regs->usRegSP, regs->usRegHL);
            regs->usRegHL = usTemp;
            break;
         case 0xE4: /* CALL PO,(nn) */
            if (!(regs->ucRegF & F_OVERFLOW))
               {
               Z80PUSHW(mem, regs, (unsigned short)(PC+2));
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 7;
               }
            else
               PC += 2;
            break;
         case 0xE5: /* PUSH HL */
            Z80PUSHW(mem, regs, regs->usRegHL);
            break;
         case 0xE6: /* AND n */
            ucTemp = Z80ReadByte(mem, PC++);
            Z80AND(regs, ucTemp);
            break;
         case 0xE8: /* RET PE */
            if (regs->ucRegF & F_OVERFLOW)
               {
               PC = Z80POPW(mem, regs);
               *iClocks -= 6;
               }
            break;
         case 0xE9: /* JP (HL) */
            PC = regs->usRegHL;
            break;
         case 0xEA: /* JP PE,(nn) */
            if (regs->ucRegF & F_OVERFLOW)
               {
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 5;
               }
            else
               PC += 2;
            break;
         case 0xEB: /* EX DE,HL */
            usTemp = regs->usRegDE;
            regs->usRegDE = regs->usRegHL;
            regs->usRegHL = usTemp;
            break;
         case 0xEC: /* CALL PE,(nn) */
            if (regs->ucRegF & F_OVERFLOW)
               {
               Z80PUSHW(mem, regs, (unsigned short)(PC+2));
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 7;
               }
            else
               PC += 2;
            break;
         case 0xED: /* lots of opcodes here */
            ucOpcode = Z80ReadByte(mem, PC++);
            regs->ucRegR++;
            switch (ucOpcode)
               {
               case 0x40: /* IN B,(C) */
               case 0x48: /* IN C,(C) */
               case 0x50: /* IN D,(C) */
               case 0x58: /* IN E,(C) */
               case 0x60: /* IN H,(C) */
               case 0x68: /* IN L,(C) */
               case 0x70: /* IN F,(C) */
               case 0x78: /* IN A,(C) */
                  *iClocks -= 12;
                  puc = pucReg[(ucOpcode >> 3) & 7];
                  *puc = Z80IN(regs->ucRegC);
                  break;
               case 0x41: /* OUT (C),B */
               case 0x49: /* OUT (C),C */
               case 0x51: /* OUT (C),D */
               case 0x59: /* OUT (C),E */
               case 0x61: /* OUT (C),H */
               case 0x69: /* OUT (C),L */
               case 0x79: /* OUT (C),A */
                  *iClocks -= 12;
                  puc = pucReg[(ucOpcode >> 3) & 7];
                  Z80OUT(regs->ucRegC, *puc);
                  break;
               case 0x42: /* SBC HL,BC */
               case 0x52: /* SBC HL,DE */
               case 0x62: /* SBC HL,HL */
               case 0x72: /* SBC HL,SP */
                  *iClocks -= 15;
                  pus = pusReg[(ucOpcode >> 4) & 3];
                  regs->usRegHL = Z80SBC16(regs, regs->usRegHL, *pus);
                  break;
               case 0x43: /* LD (nn),BC */
               case 0x53: /* LD (nn),DE */
               case 0x63: /* LD (nn),HL */
               case 0x73: /* LD (nn),SP */
                  *iClocks -= 20;
                  pus = pusReg[(ucOpcode >> 4) & 3];
                  usAddr = Z80ReadWord(mem, PC);
                  PC += 2;
                  Z80WriteWord(mem, usAddr, *pus);
                  break;
               case 0x44: /* NEG */
                  *iClocks -= 8;
                  ucTemp = regs->ucRegA;
                  regs->ucRegF |= F_ADDSUB;
                  regs->ucRegF &= ~(F_SIGN | F_ZERO | F_CARRY | F_HALFCARRY | F_OVERFLOW);
                  regs->ucRegA = 0 - regs->ucRegA;
                  regs->ucRegF |= cZ80SZ[regs->ucRegA];
                  if (regs->ucRegA == 0x80) /* overflow condition */
                     regs->ucRegF |= F_OVERFLOW;
                  regs->ucRegF |= ((ucTemp ^ regs->ucRegA) & F_HALFCARRY);
                  break;
               case 0x45: /* RETN */
                  *iClocks -= 14;
                  regs->ucRegIFF1 = regs->ucRegIFF2;
                  PC = Z80POPW(mem, regs);
                  break;
               case 0x46: /* IM 0 */
                  *iClocks -= 8;
                  regs->ucRegIM = 0;
                  break;
               case 0x47: /* LD I,A */
                  *iClocks -= 9;
                  regs->ucRegI = regs->ucRegA;
                  break;
               case 0x4A: /* ADC HL,BC */
               case 0x5A: /* ADC HL,DE */
               case 0x6A: /* ADC HL,HL */
               case 0x7A: /* ADC HL,SP */
                  pus = pusReg[(ucOpcode >> 4) & 3];
                  *iClocks -= 15;
                  regs->usRegHL = Z80ADC16(regs, regs->usRegHL, *pus);
                  break;
               case 0x4B: /* LD BC,(nn) */
               case 0x5B: /* LD DE,(nn) */
               case 0x6B: /* LD HL,(nn) */
               case 0x7B: /* LD SP,(nn) */
                  *iClocks -= 20;
                  pus = pusReg[(ucOpcode >> 4) & 3];
                  usAddr = Z80ReadWord(mem, PC);
                  PC += 2;
                  *pus = Z80ReadWord(mem, usAddr);
                  break;
               case 0x4D: /* RETI */
                  *iClocks -= 14;
                  PC = Z80POPW(mem, regs);
                  break;
               case 0x4F: /* LD R,A */
                  *iClocks -= 9;
                  regs->ucRegR = regs->ucRegA;
                  break;
               case 0x56: /* IM 1 */
                  *iClocks -= 8;
                  regs->ucRegIM = 1;
                  break;
               case 0x57: /* LD A,I */
                  *iClocks -= 9;
                  regs->ucRegA = regs->ucRegI;
                  regs->ucRegF &= ~(F_SIGN | F_ZERO | F_OVERFLOW | F_HALFCARRY | F_ADDSUB);
                  regs->ucRegF |= cZ80SZ[regs->ucRegA];
                  if (regs->ucRegIFF2)
                     regs->ucRegF |= F_OVERFLOW;
                  break;
               case 0x5E: /* IM 2 */
                  *iClocks -= 8;
                  regs->ucRegIM = 2;
                  break;
               case 0x5F: /* LD A,R */
                  *iClocks -= 9;
                  regs->ucRegA = regs->ucRegR;
                  regs->ucRegF &= ~(F_SIGN | F_ZERO | F_OVERFLOW | F_HALFCARRY | F_ADDSUB);
                  regs->ucRegF |= cZ80SZ[regs->ucRegA];
                  if (regs->ucRegIFF2)
                     regs->ucRegF |= F_OVERFLOW;
                  break;
               case 0x67: /* RRD */
                  *iClocks -= 18;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  c = regs->ucRegA & 0xf;
                  regs->ucRegA &= 0xf0;
                  regs->ucRegA |= (ucTemp & 0xf);
                  ucTemp >>= 4;
                  ucTemp |= (c << 4);
                  Z80WriteByte(mem, regs->usRegHL, ucTemp);
                  regs->ucRegF &= ~(F_HALFCARRY | F_ADDSUB | F_OVERFLOW);
                  regs->ucRegF |= cZ80SZ[ucTemp];
                  regs->ucRegF |= cZ80Parity[ucTemp];
                  break;
               case 0x6F: /* RLD */
                  *iClocks -= 18;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  c = regs->ucRegA & 0xf;
                  regs->ucRegA &= 0xf0;
                  regs->ucRegA |= (ucTemp >> 4);
                  ucTemp <<= 4;
                  ucTemp |= c;
                  Z80WriteByte(mem, regs->usRegHL, ucTemp);
                  regs->ucRegF &= ~(F_HALFCARRY | F_ADDSUB | F_OVERFLOW);
                  regs->ucRegF |= cZ80SZ[ucTemp];
                  regs->ucRegF |= cZ80Parity[ucTemp];
                  break;
               case 0xA0: /* LDI */
                  *iClocks -= 16;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegDE, ucTemp);
                  regs->usRegDE++;
                  regs->usRegHL++;
                  regs->usRegBC--;
                  regs->ucRegF &= ~(F_HALFCARRY | F_ADDSUB);
                  if (regs->usRegBC == 0)
                     regs->ucRegF &= ~F_OVERFLOW;
                  else
                     regs->ucRegF |= F_OVERFLOW;
                  break;
               case 0xA1: /* CPI */
                  *iClocks -= 16;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80CMP(regs, ucTemp);
                  regs->usRegHL++;
                  regs->usRegBC--;
                  if (regs->usRegBC == 0)
                     regs->ucRegF &= ~F_OVERFLOW;
                  else
                     regs->ucRegF |= F_OVERFLOW;
                  break;
               case 0xA2: /* INI */
                  *iClocks -= 16;
                  Z80WriteByte(mem, regs->usRegHL, Z80IN(regs->ucRegC));
                  regs->usRegHL++;
                  regs->ucRegB--;
                  regs->ucRegF |= F_ADDSUB;
                  if (regs->ucRegB == 0)
                     regs->ucRegF |= F_ZERO;
                  else
                     regs->ucRegF &= ~F_ZERO;
                  break;

               case 0xA3: /* OUTI */
                  *iClocks -= 16;
                  Z80OUT(regs->ucRegC, Z80ReadByte(mem, regs->usRegHL));
                  regs->usRegHL++;
                  regs->ucRegB--;
                  regs->ucRegF |= F_ADDSUB;
                  if (regs->ucRegB == 0)
                     regs->ucRegF |= F_ZERO;
                  else
                     regs->ucRegF &= ~F_ZERO;
                  break;
               case 0xA8: /* LDD */
                  *iClocks -= 16;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegDE, ucTemp);
                  regs->usRegDE--;
                  regs->usRegHL--;
                  regs->usRegBC--;
                  regs->ucRegF &= ~(F_HALFCARRY | F_ADDSUB);
                  if (regs->usRegBC == 0)
                     regs->ucRegF &= ~F_OVERFLOW;
                  else
                     regs->ucRegF |= F_OVERFLOW;
                  break;
               case 0xA9: /* CPD */
                  *iClocks -= 16;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80CMP(regs, ucTemp);
                  regs->usRegHL--;
                  regs->usRegBC--;
                  if (regs->usRegBC == 0)
                     regs->ucRegF &= ~F_OVERFLOW;
                  else
                     regs->ucRegF |= F_OVERFLOW;
                  break;
               case 0xAA: /* IND */
                  *iClocks -= 16;
                  Z80WriteByte(mem, regs->usRegHL, Z80IN(regs->ucRegC));
                  regs->usRegHL--;
                  regs->ucRegB--;
                  regs->ucRegF |= F_ADDSUB;
                  if (regs->ucRegB == 0)
                     regs->ucRegF |= F_ZERO;
                  else
                     regs->ucRegF &= ~F_ZERO;
                  break;
               case 0xAB: /* OUTD */
                  *iClocks -= 16;
                  Z80OUT(regs->ucRegC, Z80ReadByte(mem, regs->usRegHL));
                  regs->usRegHL--;
                  regs->ucRegB--;
                  regs->ucRegF |= F_ADDSUB;
                  if (regs->ucRegB == 0)
                     regs->ucRegF |= F_ZERO;
                  else
                     regs->ucRegF &= ~F_ZERO;
                  break;
               case 0xB0: /* LDIR */
                  *iClocks -= 16;
                  regs->ucRegF &= ~(F_HALFCARRY | F_ADDSUB | F_OVERFLOW);
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegDE, ucTemp);
                  regs->usRegDE++;
                  regs->usRegHL++;
                  regs->usRegBC--;
                  if (regs->usRegBC != 0)
                     {
                     *iClocks -= 5;
                     PC -= 2; /* Repeat this instruction until BC == 0 */
                     }
                  break;
               case 0xB1: /* CPIR */
                  *iClocks -= 16;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80CMP(regs, ucTemp);
                  regs->usRegHL++;
                  regs->usRegBC--;
                  if (regs->usRegBC != 0 && ucTemp != regs->ucRegA)
                     {
                     *iClocks -= 5;
                     PC -= 2; /* Repeat this until BC==0 || A==(HL) */
                     }
                  break;
               case 0xB2: /* INIR */
                  *iClocks -= 16;
                  Z80WriteByte(mem, regs->usRegHL, Z80IN(regs->ucRegC));
                  regs->usRegHL++;
                  regs->ucRegB--;
                  regs->ucRegF |= F_ADDSUB;
                  if (regs->ucRegB == 0)
                     regs->ucRegF |= F_ZERO;
                  else
                     {
                     regs->ucRegF &= ~F_ZERO;
                     *iClocks -= 5;
                     PC -= 2; /* Repeat this instruction until B==0 */
                     }
                  break;
               case 0xB3: /* OTIR */
                  *iClocks -= 16;
                  Z80OUT(regs->ucRegC, Z80ReadByte(mem, regs->usRegHL));
                  regs->usRegHL++;
                  regs->ucRegB--;
                  regs->ucRegF |= F_ADDSUB;
                  if (regs->ucRegB == 0)
                     regs->ucRegF |= F_ZERO;
                  else
                     {
                     regs->ucRegF &= ~F_ZERO;
                     *iClocks -= 5;
                     PC -= 2; /* Repeat until B==0 */
                     }
                  break;
               case 0xB8: /* LDDR */
                  *iClocks -= 16;
                  regs->ucRegF &= ~(F_HALFCARRY | F_ADDSUB | F_OVERFLOW);
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80WriteByte(mem, regs->usRegDE, ucTemp);
                  regs->usRegDE--;
                  regs->usRegHL--;
                  regs->usRegBC--;
                  if (regs->usRegBC != 0)
                     {
                     *iClocks -= 5;
                     PC -= 2; /* Repeat this instruction until BC == 0 */
                     }
                  break;
               case 0xB9: /* CPDR */
                  *iClocks -= 16;
                  ucTemp = Z80ReadByte(mem, regs->usRegHL);
                  Z80CMP(regs, ucTemp);
                  regs->usRegHL--;
                  regs->usRegBC--;
                  if (regs->usRegBC != 0 && ucTemp != regs->ucRegA)
                     {
                     *iClocks -= 5;
                     PC -= 2; /* Repeat this until BC==0 || A==(HL) */
                     }
                  break;
               case 0xBA: /* INDR */
                  *iClocks -= 16;
                  Z80WriteByte(mem, regs->usRegHL, Z80IN(regs->ucRegC));
                  regs->usRegHL--;
                  regs->ucRegB--;
                  regs->ucRegF |= F_ADDSUB;
                  if (regs->ucRegB == 0)
                     regs->ucRegF |= F_ZERO;
                  else
                     {
                     regs->ucRegF &= ~F_ZERO;
                     *iClocks -= 5;
                     PC -= 2; /* Repeat this instruction until B==0 */
                     }
                  break;
               case 0xBB: /* OTDR */
                  *iClocks -= 16;
                  Z80OUT(regs->ucRegC, Z80ReadByte(mem, regs->usRegHL));
                  regs->usRegHL--;
                  regs->ucRegB--;
                  regs->ucRegF |= F_ADDSUB;
                  if (regs->ucRegB == 0)
                     regs->ucRegF |= F_ZERO;
                  else
                     {
                     regs->ucRegF &= ~F_ZERO;
                     *iClocks -= 5;
                     PC -= 2; /* Repeat until B==0 */
                     }
                  break;
               default: /* ILLEGAL */
                  *iClocks = 0;
                  break;
               }
            break;
         case 0xEE: /* XOR n */
            ucTemp = Z80ReadByte(mem, PC++);
            Z80XOR(regs, ucTemp);
            break;
         case 0xF0: /* RET P */
            if (!(regs->ucRegF & F_SIGN))
               {
               PC = Z80POPW(mem, regs);
               *iClocks -= 6;
               }
            break;
         case 0xF1: /* POP AF */
            regs->usRegAF = Z80POPW(mem, regs);
            break;
         case 0xF2: /* JP P,(nn) */
            if (!(regs->ucRegF & F_SIGN))
               {
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 5;
               }
            else
               PC += 2;
            break;
         case 0xF3: /* DI */
            regs->ucRegIFF1 = regs->ucRegIFF2 = 0;
            break;
         case 0xF4: /* CALL P,(nn) */
            if (!(regs->ucRegF & F_SIGN))
               {
               Z80PUSHW(mem, regs, (unsigned short)(PC+2));
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 7;
               }
            else
               PC += 2;
            break;
         case 0xF5: /* PUSH AF */
            Z80PUSHW(mem, regs, regs->usRegAF);
            break;
         case 0xF6: /* OR n */
            ucTemp = Z80ReadByte(mem, PC++);
            Z80OR(regs, ucTemp);
            break;
         case 0xF8: /* RET M */
            if (regs->ucRegF & F_SIGN)
               {
               PC = Z80POPW(mem, regs);
               *iClocks -= 6;
               }
            break;
         case 0xF9: /* LD SP,HL */
            regs->usRegSP = regs->usRegHL;
            break;
         case 0xFA: /* JP M,(nn) */
            if (regs->ucRegF & F_SIGN)
               {
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 5;
               }
            else
               PC += 2;
            break;
         case 0xFB: /* EI */
            regs->ucRegIFF1 = regs->ucRegIFF2 = 1;
            regs->ucRegR++; /* DEBUG ? */
            break;
         case 0xFC: /* CALL M,(nn) */
            if (regs->ucRegF & F_SIGN)
               {
               Z80PUSHW(mem, regs, (unsigned short)(PC+2));
               PC = Z80ReadWord(mem, PC);
               *iClocks -= 7;
               }
            else
               PC += 2;
            break;
         case 0xFD: /* lots of opcodes */
            ucOpcode = Z80ReadByte(mem, PC++);
            regs->ucRegR++;
            *iClocks -= cZ80Cycles2[ucOpcode];
            switch (ucOpcode)
               {
               case 0x09: /* ADD IY,BC */
                  regs->usRegIY = Z80ADD16(regs, regs->usRegIY, regs->usRegBC);
                  break;
               case 0x19: /* ADD IY,DE */
                  regs->usRegIY = Z80ADD16(regs, regs->usRegIY, regs->usRegDE);
                  break;
               case 0x29: /* ADD IY,IY */
                  regs->usRegIY = Z80ADD16(regs, regs->usRegIY, regs->usRegIY);
                  break;
               case 0x39: /* ADD IY,SP */
                  regs->usRegIY = Z80ADD16(regs, regs->usRegIY, regs->usRegSP);
                  break;
               case 0x21: /* LD IY,nn */
                  regs->usRegIY = Z80ReadWord(mem, PC);
                  PC += 2;
                  break;
               case 0x22: /* LD (nn),IY */
                  usAddr = Z80ReadWord(mem, PC);
                  PC += 2;
                  Z80WriteWord(mem, usAddr, regs->usRegIY);
                  break;
               case 0x23: /* INC IY */
                  regs->usRegIY++; /* No flags affected */
                  break;
               case 0x2A: /* LD IY,(nn) */
                  usAddr = Z80ReadWord(mem, PC);
                  PC += 2;
                  regs->usRegIY = Z80ReadWord(mem, usAddr);
                  break;
               case 0x2B: /* DEC IY */
                  regs->usRegIY--; /* No flags affected */
                  break;
               case 0x34: /* INC (IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  ucTemp = Z80ReadByte(mem, usAddr);
                  Z80WriteByte(mem, usAddr, Z80INC(regs, ucTemp));
                  break;
               case 0x35: /* DEC (IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  ucTemp = Z80ReadByte(mem, usAddr);
                  Z80WriteByte(mem, usAddr, Z80DEC(regs, ucTemp));
                  break;
               case 0x36: /* LD (IY+d),n */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  ucTemp = Z80ReadByte(mem, PC++);
                  Z80WriteByte(mem, usAddr, ucTemp);
                  break;
               case 0x46: /* LD B,(IY+d) */
               case 0x4E: /* LD C,(IY+d) */
               case 0x56: /* LD D,(IY+d) */
               case 0x5E: /* LD E,(IY+d) */
               case 0x66: /* LD H,(IY+d) */
               case 0x6E: /* LD L,(IY+d) */
               case 0x7E: /* LD A,(IY+d) */
                  puc = pucReg[(ucOpcode >> 3) & 7];
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  *puc = Z80ReadByte(mem, usAddr);
                  break;
               case 0x70: /* LD (IY+d),B */
               case 0x71: /* LD (IY+d),C */
               case 0x72: /* LD (IY+d),D */
               case 0x73: /* LD (IY+d),E */
               case 0x74: /* LD (IY+d),H */
               case 0x75: /* LD (IY+d),L */
               case 0x77: /* LD (IY+d),A */
                  puc = pucReg[ucOpcode & 7];
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80WriteByte(mem, usAddr, *puc);
                  break;
               case 0x86: /* ADD A,(IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80ADD(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0x8E: /* ADC A,(IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80ADC(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0x96: /* SUB A,(IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80SUB(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0x9E: /* SBC A,(IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80SBC(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xA6: /* AND (IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80AND(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xAE: /* XOR (IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80XOR(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xB6: /* OR (IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80OR(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xBE: /* CP (IY+d) */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  Z80CMP(regs, Z80ReadByte(mem, usAddr));
                  break;
               case 0xE1: /* POP IY */
                  regs->usRegIY = Z80POPW(mem, regs);
                  break;
               case 0xE3: /* EX (SP),IY */
                  usTemp = Z80ReadWord(mem, regs->usRegSP);
                  Z80WriteWord(mem, regs->usRegSP, regs->usRegIY);
                  regs->usRegIY = usTemp;
                  break;
               case 0xE5: /* PUSH IY */
                  Z80PUSHW(mem, regs, regs->usRegIY);
                  break;
               case 0xE9: /* JP (IY) */
                  PC = regs->usRegIY;
                  break;
               case 0xF9: /* LD SP,IY */
                  regs->usRegSP = regs->usRegIY;
                  break;
               case 0xCB: /* Another set of opcodes branches from here */
               /* They all use (IY+d), so load the address */
                  usAddr = regs->usRegIY + (signed short)(signed char)Z80ReadByte(mem, PC++);
                  ucTemp = Z80ReadByte(mem, usAddr);
                  ucOpcode = Z80ReadByte(mem, PC++);
                  *iClocks -= cZ80Cycles3[ucOpcode];
                  switch (ucOpcode)
                     {
                     case 0x06: /* RLC (IY+d) */
                        Z80WriteByte(mem, usAddr, Z80RLC(regs, ucTemp));
                        break;
                     case 0x0E: /* RRC (IY+d) */
                        Z80WriteByte(mem, usAddr, Z80RRC(regs, ucTemp));
                        break;
                     case 0x16: /* RL (IY+d) */
                        Z80WriteByte(mem, usAddr, Z80RL(regs, ucTemp));
                        break;
                     case 0x1E: /* RR (IY+d) */
                        Z80WriteByte(mem, usAddr, Z80RR(regs, ucTemp));
                        break;
                     case 0x26: /* SLA (IY+d) */
                        Z80WriteByte(mem, usAddr, Z80SLA(regs, ucTemp));
                        break;
                     case 0x2E: /* SRA (IY+d) */
                        Z80WriteByte(mem, usAddr, Z80SRA(regs, ucTemp));
                        break;
                     case 0x3E: /* SRL (IY+d) */
                        Z80WriteByte(mem, usAddr, Z80SRL(regs, ucTemp));
                        break;
                     case 0x46: /* BIT 0,(IY+d) */
                     case 0x4E: /* BIT 1,(IY+d) */
                     case 0x56: /* BIT 2,(IY+d) */
                     case 0x5E: /* BIT 3,(IY+d) */
                     case 0x66: /* BIT 4,(IY+d) */
                     case 0x6E: /* BIT 5,(IY+d) */
                     case 0x76: /* BIT 6,(IY+d) */
                     case 0x7E: /* BIT 7,(IY+d) */
                        c = (ucOpcode >> 3) & 7;
                        regs->ucRegF &= ~(F_ADDSUB | F_ZERO);
                        regs->ucRegF |= F_HALFCARRY;
                        if (!(ucTemp & (1<<c)))
                           regs->ucRegF |= F_ZERO;
                        break;
                     case 0x86: /* RES 0,(IY+d) */
                     case 0x8E: /* RES 1,(IY+d) */
                     case 0x96: /* RES 2,(IY+d) */
                     case 0x9E: /* RES 3,(IY+d) */
                     case 0xA6: /* RES 4,(IY+d) */
                     case 0xAE: /* RES 5,(IY+d) */
                     case 0xB6: /* RES 6,(IY+d) */
                     case 0xBE: /* RES 7,(IY+d) */
                        c = (ucOpcode >> 3) & 7;
                        ucTemp &= ~(1<<c);
                        Z80WriteByte(mem, usAddr, ucTemp);
                        break;
                     case 0xC6: /* SET 0,(IY+d) */
                     case 0xCE: /* SET 1,(IY+d) */
                     case 0xD6: /* SET 2,(IY+d) */
                     case 0xDE: /* SET 3,(IY+d) */
                     case 0xE6: /* SET 4,(IY+d) */
                     case 0xEE: /* SET 5,(IY+d) */
                     case 0xF6: /* SET 6,(IY+d) */
                     case 0xFE: /* SET 7,(IY+d) */
                        c = (ucOpcode >> 3) & 7;
                        ucTemp |= (1<<c);
                        Z80WriteByte(mem, usAddr, ucTemp);
                        break;
                     default: /* ILLEGAL */
                        *iClocks = 0;
                        break;
                     } /* switch on FDCB */
                  break;
               default: /* ILLEGAL */
                  *iClocks = 0;
                  break;
               } /* switch on FD */
            break;
         case 0xFE: /* CP n */
            ucTemp = Z80ReadByte(mem, PC++);
            Z80CMP(regs, ucTemp);
            break;
         }   /* switch */
      } /* while */
   regs->usRegPC = PC;

} /* EXECZ80() */
