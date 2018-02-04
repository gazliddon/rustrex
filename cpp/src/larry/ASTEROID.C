/********************************************/
/* Code to emulate Asteroid arcade hardware */
/* Asteroids contains a 1Mhz 6502 for all.  */
/* The display is vector, black and white   */
/*                                          */
/* Written by Larry Bank                    */
/* Copyright 1998 BitBank Software, Inc.    */
/* Driver started 1/28/98                   */
/********************************************/

#include <dos.h>
#include <time.h>
#include <direct.h>
#include <io.h>
#include <windows.h>
#include <commdlg.h>   /* Common dialogs  */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "accio.h"
#include "emu.h"
#include "williams.h"
#include "sound.h"

#define DISPLAY_X 512
#define DISPLAY_Y 512

GAMEOPTIONS AsteroidOptions[] = {"Language", {"English","German","French","Spanish",0,0,0,0}, 0,
                                 "Ships per game", {"4","3",0,0,0,0,0,0}, 1,
                                 "Game Price", {"Free","1 coin per 2 plays","1 coin per play","2 coins per play",0,0,0,0}, 1,
                                 NULL, NULL, 0};

/* Table defining keys used by Asteroids */
EMUKEYS emukAsteroid[] = {"Fire",72,RKEY_0A,
                  "Thrust",80,RKEY_1A,
                  "Hyperspace",77,RKEY_2A,
                  "Player 1",2,RKEY_3A,
                  "Player 2",3,RKEY_4A,
                  "Rotate CClockwise",33,RKEY_5A,
                  "Rotate Clockwise",34,RKEY_6A,
                  "Coin Drop",4,RKEY_7A,
                  NULL,0,0};

char *AstSounds[] = {"beep.sam","explode1.sam","explode2.sam","explode3.sam",
                     "fire.sam","life.sam","lsaucer.sam","sfire.sam",
                     "ssaucer.sam","thrust.sam","thumbhi.sam","thumplo.sam",NULL};

/* Needs to be global for faster access by all routines */
extern HPALETTE hpalEMU;
extern BOOL bUserAbort;
extern int iFrame, iPitch;
extern unsigned char *pBitmap; /* Pointer to bitmap memory */
extern BITMAPINFO *bminfo;
unsigned char AstPIA1Read(unsigned short);
unsigned char AstPIA2Read(unsigned short);
unsigned char AstDIPSWRead(unsigned short);
void AstPIA1Write(unsigned short, unsigned char);
void AstPIA2Write(unsigned short, unsigned char);
void AstBankSWWrite(unsigned short, unsigned char);
void AstExplodeWrite(unsigned short, unsigned char);
void AstThumpWrite(unsigned short, unsigned char);
void AstSoundWrite(unsigned short, unsigned char);
void AstVectorStart(unsigned short, unsigned char);

#define NUM_HARDWARE 8  /* Number of hardware sections to emulate */
MEMHANDLERS mhAsteroid[NUM_HARDWARE] = {0x2000, 8, AstPIA1Read, AstPIA1Write,
                                        0x2400, 8, AstPIA2Read, AstPIA2Write,
                                        0x2800, 4, AstDIPSWRead, RoboWriteNULL,
                                        0x3000, 1, RoboReadNULL, AstVectorStart,
                                        0x3200, 1, RoboReadRAM, AstBankSWWrite,
                                        0x3600, 1, RoboReadNULL, AstExplodeWrite,
                                        0x3a00, 1, RoboReadNULL, AstThumpWrite,
                                        0x3c00, 5, RoboReadNULL, AstSoundWrite};

LOADROM ASTEROIDROMS[] = {"035145.02", 0x6800, 0x800, 0, NULL, NULL,
                          "035144.02", 0x7000, 0x800, 0, NULL, NULL,
                          "035143.02", 0x7800, 0x800, 0, NULL, NULL,
                          "035143.02", 0xf800, 0x800, 0, NULL, NULL, /* Read again for interrupt vector table */
                          "035127.02", 0x5000, 0x800, 0, NULL, NULL,
                          NULL, 0, 0, 0, NULL, NULL};

/* List the ROM images in a listbox needed to play Asteroids */
void AsteroidList(HWND hDlg, int iControl)
{
int i;
    i = 0;
    while (ASTEROIDROMS[i].szROMName)
       {
       SendDlgItemMessage(hDlg, iControl, LB_INSERTSTRING, (UINT)-1, (long)ASTEROIDROMS[i].szROMName);
       i++;
       }
} /* AsteroidList() */
/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstDrawLine(int, int, int, int)                            *
 *                                                                          *
 *  PURPOSE    : Draw a line from point a to b.                             *
 *                                                                          *
 ****************************************************************************/
void AstDrawLine(int x1, int y1, int x2, int y2, unsigned char ucIntensity)
{
int error;
int x, y, dx, dy, xinc, yinc;
int iWidth, iHeight;
/* Slow and ugly, but start with this to get it working... */

    iWidth = DISPLAY_X;
    iHeight = DISPLAY_Y;
//   iWidth = bminfo->bmiHeader.biWidth;
//   iHeight = bminfo->bmiHeader.biHeight;
//   if (iHeight < 0) /* For top-down DIB */
//      iHeight = -iHeight;

   x1 >>= 1;
   x2 >>= 1;
   y1 >>= 1;
   y2 >>= 1;

   ucIntensity = 0xff; /* DEBUG */

   x = x1;
   y = y1;
   dx = x2 - x1;
   if (dx < 0)
      {
      xinc = -1;
      dx = -dx;
      }
   else
      xinc = 1;
   dy = y2 - y1;
   if (dy < 0)
      {
      yinc = -1;
      dy = -dy;
      }
   else
      yinc = 1;

   if (dx > dy) /* Case 1 */
      {
      error=dy*2-dx;
      dx <<= 1;
      dy <<= 1;
      while (x != x2)
         {
         if (x >=0 && x < iWidth && y >=0 && y < iHeight)
            pBitmap[(DISPLAY_Y-1-y)*iPitch + x] = ucIntensity; /* Plot this point */
         x += xinc;
         error += dy;
         if (error > 0)
            {
            y += yinc;
            error -= dx;
            }
         }
      }
   else  /* dy > dx */
      {
      error=dx*2-dy;
      dx <<= 1;
      dy <<= 1;
      while (y != y2)
         {
         if (x >=0 && x < iWidth && y >=0 && y < iHeight)
            pBitmap[(DISPLAY_Y-1-y)*iPitch + x] = ucIntensity; /* Plot this point */
         y += yinc;
         error += dx;
         if (error > 0)
            {
            x += xinc;
            error -= dy;
            }
         }
      }
/* Make sure the last point is plotted in case of a single point line */
  if (x >=0 && x < iWidth && y >=0 && y < iHeight)
     pBitmap[(DISPLAY_Y-1-y)*iPitch + x] = ucIntensity; /* Plot this point */

} /* AstDrawLine() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstVectorStart(unsigned short)                             *
 *                                                                          *
 *  PURPOSE    : When written to, this signals the vectors to be drawn.     *
 *                                                                          *
 ****************************************************************************/
void AstVectorStart(unsigned short usAddr, unsigned char ucByte)
{
unsigned short us, opcode, *p;
int iStack[10]; /* Allow nesting 10 levels deep */
int iSP;
BOOL bDone, bDraw;
int i, iScale; /* Overall scale factor (as a shift value) */
unsigned char ucIntensity;
int x, y; /* Current beam position */
int dx, dy; /* Current vector to draw */


/* Erase old stuff */
   /* DEBUG */
   for (y=0; y<DISPLAY_Y; y++)
      memset(&pBitmap[y*iPitch], 0, DISPLAY_X);

/* Process the vector table */
        bDone = FALSE;
        p = (unsigned short *)&mem_map[MEM_RAM+0x4000];
        x = y = iScale = 0;
        iSP = 10; /* Start stack pointer at top of stack */
        while (!bDone) /* Execute until halt instruction encountered */
           {
           opcode = *p++;
           bDraw = FALSE;
           switch (opcode & 0xf000)
              {
              case 0: /* This is not defined, but acts as a HALT instruction */
              case 0xB000: /* another halt instruction */
                 bDone = TRUE;
                 break;
              case 0x1000: /* 1-9 are long vectors with various scale factors */
              case 0x2000:
              case 0x3000:
              case 0x4000:
              case 0x5000:
              case 0x6000:
              case 0x7000:
              case 0x8000:
              case 0x9000:
                 i = (iScale + (opcode >> 12)) & 0xf; /* Scale factor */
                 if (i > 9)
                    i = -1;
                 dy = opcode & 0x3ff;
                 if (opcode & 0x400) /* Y sign bit */
                    dy = -dy;
                 opcode = *p++; /* Second half of 4 byte instruction */
                 dx = opcode & 0x3ff;
                 if (opcode & 0x400) /* X sign bit */
                    dx = -dx;
                 ucIntensity = opcode >> 12;
                 dx = (dx << 0) >> (9-i); /* Adjust for both scale factors */
                 dy = (dy << 0) >> (9-i);
                 bDraw = TRUE;
                 break;
              case 0xA000: /* Position beam and load overall scale factor */
                 y = opcode & 0xfff; /* Lower 12 bits are Y position */
                 opcode = *p++; /* get second half of instruction */
                 x = opcode & 0xfff; /* Lower 12 bits are X position */
                 iScale = (opcode & 0xf000) >> 12;
                 if (opcode & 0x8000) /* divisor = negative shift */
                    iScale = iScale - 16; /* two's complement negative # */
                 break;
              case 0xC000: /* Call subroutine */
                 us = opcode & 0xfff;
                 if (us == 0) /* Address of 0 same as HALT */
                    bDone = TRUE;
                 else
                    {
                    iStack[--iSP] = (int)p; /* push current position */
                    us = 0x4000 + (opcode & 0xfff)*2;
                    if (mem_map[MEM_FLAGS + us] == 1) /* Reading from ROM */
                       p = (unsigned short *)&mem_map[MEM_ROM + us];
                    else
                       p = (unsigned short *)&mem_map[MEM_RAM + us];
                    }
                 break;
              case 0xD000: /* Return from subroutine */
                 p = (unsigned short *)iStack[iSP++];
                 break;
              case 0xE000: /* Jump to address */
                 us = opcode & 0xfff;
                 if (us == 0)
                    bDone = TRUE; /* Address of 0 same as HALT */
                 else
                    us = 0x4000 + (opcode & 0xfff)*2;
                    if (mem_map[MEM_FLAGS + us] == 1) /* Reading from ROM */
                       p = (unsigned short *)&mem_map[MEM_ROM + us];
                    else
                       p = (unsigned short *)&mem_map[MEM_RAM + us];
                 break;
              case 0xF000: /* Draw relative short vector */
                 ucIntensity = (opcode & 0xf0) >> 4;
                 i = 2 + ((opcode >> 2) & 0x02) + ((opcode >> 11) & 0x01);
                 i = ((iScale + i) & 0x0f);
                 if (i > 9)
                    i = -1;
                 dx = (opcode & 3) << 8;
                 dy = opcode & 0x300;
                 if (opcode & 4) /* X sign bit */
                    dx = -dx;
                 if (opcode & 0x400) /* Y sign bit */
                    dy = -dy;
                 dx  = (dx << 0) >> (9-i);
                 dy  = (dy << 0) >> (9-i);
                 bDraw = TRUE;
                 break;
              } /* switch */
           if (bDraw) /* If there is something to draw */
              {
              if (ucIntensity > 3) /* If no intensity, just move beam */
                 AstDrawLine(x,y,x+dx,y+dy, ucIntensity);
              x += dx; /* Update beam position */
              y += dy;
              }
           }
} /* AstVectorStart() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstBankSWWrite(unsigned short)                             *
 *                                                                          *
 *  PURPOSE    : Write values into the Asteroids bank switch at 0x3200.     *
 *                                                                          *
 ****************************************************************************/
void AstBankSWWrite(unsigned short usAddr, unsigned char ucByte)
{
register unsigned char c;
int i;

   c = ucByte & 4; /* Isolate bank switch bit */
   if (c != (mem_map[MEM_RAM+usAddr] & 4)) /* If bit is changing */
      {
      /* Swap areas at $200 with $300 (player 1/2 info swap) */
      for (i=0; i<0x100; i++)
         {
         c = mem_map[MEM_RAM+0x200 + i];
         mem_map[MEM_RAM+0x200 + i] = mem_map[MEM_RAM+0x300+i];
         mem_map[MEM_RAM+0x300+i] = c;
         }
      }
   mem_map[MEM_RAM+usAddr] = ucByte; /* Store new bit setting */

} /* AstBankSWWrite() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstSoundWrite(unsigned short)                              *
 *                                                                          *
 *  PURPOSE    : Write values into the Asteroids sound generator.           *
 *                                                                          *
 ****************************************************************************/
void AstSoundWrite(unsigned short usAddr, unsigned char ucByte)
{

} /* AstSoundWrite() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstThumpWrite(unsigned short)                              *
 *                                                                          *
 *  PURPOSE    : Write values into the Asteroids thump sound port.          *
 *                                                                          *
 ****************************************************************************/
void AstThumpWrite(unsigned short usAddr, unsigned char ucByte)
{
   if (ucByte & 0x10)
      {
//      if (ucByte & 4)
//         EMUDoSound(10);
//      else
//         EMUDoSound(11);
      }
} /* AstThumpWrite() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstExplodeWrite(unsigned short)                            *
 *                                                                          *
 *  PURPOSE    : Write values into the Asteroids explosion sound generator. *
 *                                                                          *
 ****************************************************************************/
void AstExplodeWrite(unsigned short usAddr, unsigned char ucByte)
{
   switch (ucByte)
      {
      case 0xbd: /* Small explosion */
//         EMUDoSound(1);
         break;
      case 0xfd: /* Medium explosion */
//         EMUDoSound(2);
         break;
      case 0x3d: /* Large explosion */
//         EMUDoSound(3);
         break;
      }
} /* AstExplodeWrite() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstPIA2Read(unsigned short)                                *
 *                                                                          *
 *  PURPOSE    : Read the values from the Asteroids port 2400-2407.         *
 *                                                                          *
 ****************************************************************************/
unsigned char AstPIA2Read(unsigned short usAddr)
{
   switch (usAddr)
      {
      case 0x2400: /* Coin in */
         if (ulKeys & RKEY_7A)
            return 0x80;
         break;
      case 0x2403: /* 1 Player start */
         if (ulKeys & RKEY_3A)
            return 0x80;
         break;
      case 0x2404: /* 2 Player start */
         if (ulKeys & RKEY_4A)
            return 0x80;
         break;
      case 0x2405: /* Thrust */
         if (ulKeys & RKEY_1A)
            return 0x80;
         break;
      case 0x2406: /* Rotate right */
         if (ulKeys & RKEY_6A)
            return 0x80;
         break;
      case 0x2407: /* Rotate left */
         if (ulKeys & RKEY_5A)
            return 0x80;
         break;
      }
    return 0;

} /* AstPIA2Read() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstPIA1Read(unsigned short)                                *
 *                                                                          *
 *  PURPOSE    : Read the values from the Asteroids port $2000-2007.        *
 *                                                                          *
 ****************************************************************************/
unsigned char AstPIA1Read(unsigned short usAddr)
{
   switch (usAddr)
      {
      case 0x2003: /* Hyperspace */
         if (ulKeys & RKEY_2A)
            return 0x80;
         break;
      case 0x2004: /* Fire */
         if (ulKeys & RKEY_0A)
            return 0x80;
         break;
      case 0x2007: /* Self Test */
         return 0;
      }

   return 0;

} /* AstPIA1Read() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstDIPSWRead(unsigned short)                               *
 *                                                                          *
 *  PURPOSE    : Read the values from the Asteroids DIP switches.           *
 *                                                                          *
 ****************************************************************************/
unsigned char AstDIPSWRead(unsigned short usAddr)
{
   switch (usAddr)
      {
      case 0x2800: /* Switches 8 & 7 in lower 2 bits */
         return AsteroidOptions[2].iChoice; /* Number of coins per play */
      case 0x2801: /* Not used - switches 6 and 5 in lower 2 bits */
         return 0;
      case 0x2802: /* Switches 4 and 3 in lower 2 bits */
         return AsteroidOptions[1].iChoice; /* Number of ships per game */
      case 0x2803: /* Switches 1 and 0 in lower 2 bits */
         return AsteroidOptions[0].iChoice; /* Language */
      }
} /* AstDIPSWRead() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstPIA1Write(unsigned short)                               *
 *                                                                          *
 *  PURPOSE    : Write values into the Asteroids PIA #1.                    *
 *                                                                          *
 ****************************************************************************/
void AstPIA1Write(unsigned short usAddr, unsigned char ucByte)
{
char c;
  c = 0;
} /* AstPIA1Write() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AstPIA2Write(unsigned short)                               *
 *                                                                          *
 *  PURPOSE    : Write values into the Asteroids PIA #2.                    *
 *                                                                          *
 ****************************************************************************/
void AstPIA2Write(unsigned short usAddr, unsigned char ucByte)
{
char c;
   c = 0;
} /* AstPIA2Write() */

void AsteroidInit(unsigned short us, unsigned char uc)
{
char keys[256];

/* Clear out any pressed keys before beginning */
   GetAsyncKeyState(VK_ESCAPE); /* Important to clear this key */
   memset(keys, 0, sizeof(keys));
   SetKeyboardState(keys);

} /* AsteroidInit() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : AsteroidPlay(HANDLE, HWND, char *)                         *
 *                                                                          *
 *  PURPOSE    : Emulate the Atari Asteroids arcade hardware.               *
 *                                                                          *
 ****************************************************************************/
void AsteroidPlay(HANDLE hInst, HWND hWnd, char *szDir, BOOL bThrottle, BOOL bAutoState)
{
HDC hdc;
HPALETTE hpalOld;
int i, j;
long lTargTime, lCurrTime;
REGS6502 regs;
unsigned char ucIRQs = 0; /* Pending interrupt flags */
int iNumHandlers;

    emuh = AccAlloc(sizeof(EMUHANDLERS)*50); /* Allow up to 50 hardware emulation handlers */
    mem_map = AccAlloc(0x30000); /* Allocate 192K for 2 banks and flag map */
    memset(&mem_map[MEM_FLAGS], 0, 0x10000); /* Assume all of map is RAM to start */
    EMUCreateVideoBuffer(DISPLAY_X, DISPLAY_Y, 8, &pBitmap);
    hpalEMU = EMUCreateIdentityPalette(NULL, 64);
    hdc = GetDC(hWnd);
    hpalOld = SelectPalette(hdc, hpalEMU, 0);
    RealizePalette(hdc);
    ReleaseDC(hWnd, hdc);

/* Load the sound samples */
//    iNumSounds = 0;
//    while (AstSounds[iNumSounds])
//       {
//       strcpy(pszTemp, szDir);
//       strcat(pszTemp, AstSounds[iNumSounds]);
//       iHandle = AccOpen(pszTemp);
//       i = AccSeek(iHandle, 0, 2); /* Get the file length */
//       AccSeek(iHandle, 16, 0); /* Seek to start of data */
//       i -= 16; /* Skip header */
//       pSndBuf[iNumSounds] = AccAlloc(i);
//       iSndLen[iNumSounds] = i;
//       AccRead(iHandle, pSndBuf[iNumSounds], i);
//       AccClose(iHandle);
//       iNumSounds++;
//       }
/* Open the sound device */
    EMUOpenSound(11025); /* Sample rate used by the Asteroids samples */

/*--- Load the Asteroid ROMs into memory ---*/
    if (EMULoadRoms(hWnd, ASTEROIDROMS, szDir, &iNumHandlers, emuh, mem_map))
     /* Error, free everything and leave */
       goto cleanup;

/*--- Setup the hardware emulation handlers ---*/
   for (i=0; i<NUM_HARDWARE; i++)
      {
      emuh[iNumHandlers].pfn_read = mhAsteroid[i].pfn_read; /* Copy read and write routines */
      emuh[iNumHandlers++].pfn_write = mhAsteroid[i].pfn_write;
      memset(&mem_map[MEM_FLAGS + mhAsteroid[i].usStart], iNumHandlers+1, mhAsteroid[i].usLen); /* Set appropriate flag number */
      }

/* All set to start, tell the parent window to resize to what we need */
   EMUResize(DISPLAY_X, DISPLAY_Y);

   ARESET6502(mem_map, &regs);
/* Load the entire machine state from the last time it was played */
   if (bAutoState)
      {
      EMULoadState('0', &mem_map[MEM_RAM], (char *)&regs, sizeof(regs), (char *)ucColorTab, 16);
//      WilliamsRestoreVideo(&mem_map[MEM_RAM], pBitmap, ucColorTab); /* Convert video RAM into a DIB */
      cDirtyRect = -1;
      }
/* Load the hi-score table */
   EMULoadHiscore(&mem_map[MEM_RAM+0], 0x100);

   hdc = GetDC(hWnd);
   hpalOld = SelectPalette(hdc, hpalEMU, 0);
   RealizePalette(hdc);
   bUserAbort = FALSE;
   ulKeys = 0; /* Start with no keys pressed */
   iFrame = 0;
   timeBeginPeriod(1); /* Try to get 1ms resolution from the timer */
   lCurrTime = timeGetTime();
   lTargTime = lCurrTime + 33; /* Try 30 fps */
   i = 1000;
   ucIRQs = 0;
   AEXEC6502(mem_map, &regs, emuh, &i, &ucIRQs);  /* Let it do some init before first NMI */
   while (!bUserAbort)
      {
      if (CheckMessages(&ulKeys)) /* Check for keyboard activity and look for ESC to quit */
         bUserAbort = TRUE;
      for (j=0; j<8; j++)
         {
         ucIRQs = INT_NMI; /* Need to generate this 240 times a second */
         i = 6250;
         AEXEC6502(mem_map, &regs, emuh, &i, &ucIRQs);  /* Execute a 240th of a second on a 1.5Mhz 6502 */
         }
      cDirtyRect = -1;
      if (cDirtyRect) /* If we need to paint, do it */
         EMUScreenUpdate(cDirtyRect, hdc, DISPLAY_X, DISPLAY_Y);
      iFrame++; /* Count the number of frames */
/* Allow messages to flow while we waste time */
      if (bThrottle)
         {
         lCurrTime = timeGetTime();
         j = lTargTime - lCurrTime;
         if (j > 3) /* We can trust Sleep for a decent amount of time */
            Sleep(j-1);
         while (lCurrTime < lTargTime)
            {
            if (CheckMessages(&ulKeys))
               bUserAbort = TRUE;
            lCurrTime = timeGetTime();
            }
         }
      lTargTime = lCurrTime + (1000 / 30); /* Milliseconds = 1000 / 30 fps */
      }
   timeEndPeriod(1); /* Need a matching timeEndPeriod() */
   SelectPalette(hdc, hpalOld, 1);
   ReleaseDC(hWnd, hdc);
/* Save the hi-score table */
   EMUSaveHiscore(&mem_map[MEM_RAM+0], 0x100);
/* Save the entire machine state for a quick startup next time */
   if (bAutoState)
      EMUSaveState('0',&mem_map[MEM_RAM], (char *)&regs, sizeof(regs), (char *)ucColorTab, 16);
cleanup:
   AccFree(emuh); /* Free emulation handler table */
   AccFree(mem_map); /* Free memory map */
   DeleteObject(hpalEMU);
   EMUFreeVideoBuffer(pBitmap);
  /* Free the sound samples */
//   for (i=0; i<iNumSounds; i++)
//      AccFree(pSndBuf[i]);

} /* AsteroidPlay() */

