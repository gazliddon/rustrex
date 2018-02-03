/****************************************************************/
/* Larry's Arcade Emulator (an exercise in futility)            */
/* A Win32 DirectX app to emulate my favorite arcade games      */
/* Copyright (c) 1998 BitBank Software, Inc.                    */
/*                                                              */
/* Change History:                                              */
/* 1/07/98 Larry B - wrote it.                                  */
/****************************************************************/
/* Here are some sample routines I am releasing to the public              */
/*                                                                         */
/* The assembler code is optimized for the Pentium (execution pipe interleave) */
/* It may look ugly to the uninitiated, but it goes a whole lot quicker.   */
/* Please feel free to pick at it and send me any improvements you make.   */
/* The character and sprite routines could certainly be done quicker by    */
/* Copying DWORDS to the destination bitmap, but then the transparency     */
/* and color palette conversion would not work properly.  For example,     */
/* in most character based games like PacMan, each character color is      */
/* shifted up 2 bits and added to the 2-Bpp character bitmap data.         */
/* Since each color then really selects a group of 4 colors, a dword       */
/* copy would not work well because the resultant value than looks up      */
/* the actually palette entry in another table.  PacMan can be done more   */
/* efficiently by creating a palette with all permutations of color groups */
/* but it is the exception and not the rule.  Most other character based   */
/* games would arrive at more than 256 possible colors which would not fit */
/* in a standard palette.                                                  */
/* Enjoy...                                                                */
/* Larry Bank 5/4/98                                                       */
/***************************************************************************/


/* Some general routines for understanding my brand of emulation */
/****************************************************************************
 *                                                                          *
 *  FUNCTION   : EMULoadRoms(HWND, LOADROM *, char *, int *, char *, EMUHANDLERS)*
 *                                                                          *
 *  PURPOSE    : Load the ROM images into our memory map.                   *
 *                                                                          *
 ****************************************************************************/
BOOL EMULoadRoms(HWND hWnd, LOADROM *roms, char *szDir, int *iNumHandlers, EMUHANDLERS *emuh, char *mem_map)
{
int i, iHandle;
char pszTemp[256];

    i = 0;
    if (iNumHandlers == NULL) /* Loading character ROMS */
       {
       while (roms[i].szROMName)
          {
          strcpy(pszTemp, szDir);
          strcat(pszTemp, roms[i].szROMName);
          iHandle = AccOpen(pszTemp);
          if (iHandle < 0)
             {
             MessageBox(hWnd, pszTemp, "Unable to load ROM image",MB_OK);
             return TRUE;
             }
          AccRead(iHandle, &mem_map[roms[i].iROMStart + MEM_ROM], roms[i].iROMLen);
          AccClose(iHandle);
          i++;
          }
       }
    else
       {
       *iNumHandlers = 0;
       while (roms[i].szROMName)
          {
          strcpy(pszTemp, szDir);
          strcat(pszTemp, roms[i].szROMName);
          iHandle = AccOpen(pszTemp);
          if (iHandle < 0)
             {
             MessageBox(hWnd, pszTemp, "Unable to load ROM image",MB_OK);
             return TRUE;
             }
          AccRead(iHandle, &mem_map[roms[i].iROMStart + MEM_ROM], roms[i].iROMLen);
          /* Mark this section with the proper memory handler flag */
          if (roms[i].pfn_read != NULL) /* If there is a custom read routine */
             { /* Copy handler routines and increment number */
             emuh[*iNumHandlers].pfn_read = roms[i].pfn_read;
             emuh[(*iNumHandlers)++].pfn_write = roms[i].pfn_write;
             memset(&mem_map[roms[i].iROMStart + MEM_FLAGS], (*iNumHandlers)+1, roms[i].iROMLen); /* Mark it as needing special handler */
             }
          else
             {
             if (roms[i].iROMStart < 0x10000) /* Don't mark flags for bank switched ROM */
                memset(&mem_map[roms[i].iROMStart + MEM_FLAGS], 1, roms[i].iROMLen); /* Mark it as normal ROM */
             }
          AccClose(iHandle);
          i++;
          }
       }
    return FALSE;

} /* EMULoadRoms() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : EMUDrawChar(unsigned short, unsigned char *, int)          *
 *                                                                          *
 *  PURPOSE    : Draw the bitmapped character at the given position.        *
 *                                                                          *
 ****************************************************************************/
void EMUDrawChar(int iAddr, unsigned char *cColorPROM, int iPitch, int iChar, int iColor, unsigned char *pBitmap, unsigned char *pCharData)
{
int x, y, count;
unsigned char *p, *d;

   x = iCharX[iAddr];   /* Translate address offset into x,y coordinate */
   y = iCharY[iAddr];
   if (x < 0 || y < 0) /* Non-visible stuff */
      return;

//   cDirtyRect |= cSPDirty[y]; /* Mark this area of the display as needing a repaint */
   d = &pBitmap[y * iPitch + x];
   p = &pCharData[iChar * 64];
#ifdef PORTABLE
   for (y=0; y<8; y++)
      for (x=0; x<8; x++)
         d[y*iPitch+x] = cColorPROM[iColor + *p++];
#else /* Do it the RIGHT way */
   iPitch -= 8; /* Use to advance to next line in asm code */
   _asm {
        mov  esi,p
        mov  edi,d
        dec  edi        /* Start back by 1 for better instruction interleave */
        mov  ebx,iColor
        mov  edx,cColorPROM  /* color lookup table */
        mov  count,8      /* y count */
        xor  eax,eax
drwc0:  mov  ecx,8      /* x count */
drwc1:  mov  al,[esi]
        inc  edi
        add  al,bl
        inc  esi
        mov  al,[edx+eax]  /* translate the color */
        dec  ecx
        mov  [edi],al
        jnz  drwc1
        add  edi,iPitch   /* Skip to next line */
        dec  count
        jnz  drwc0
        }
#endif /* PORTABLE */

} /* EMUDrawChar() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : EMUDrawSprite(int, int, uchar, uchar, bool bool)           *
 *                                                                          *
 *  PURPOSE    : Draw an individual sprite with the given attributes.       *
 *               The transparency color is in the sprite image data.        *
 *                                                                          *
 ****************************************************************************/
void EMUDrawSprite(int sx, int sy, int iSprite, int iColor, BOOL bFlipx, BOOL bFlipy, unsigned char *pSpriteData, unsigned char *pColorPROM)
{
unsigned char *d, *p;
int xCount, yCount, iDestPitch, iSrcPitch;
#ifdef PORTABLE
unsigned char c;
register int x, y;
#endif

/* Adjust for clipped sprite */
   xCount = yCount = 16; /* Assume full size to draw */
   p = &pSpriteData[iSprite * 256];
   if (sy < 0)
      {
      yCount += sy; /* Shrink height to draw */
      if (bFlipy)
         p += (sy*16);
      else
         p -= (sy*16); /* Adjust sprite pointer also */
      sy = 0; /* Start at 0 */
      }
   if (sx < 0)
      {
      xCount += sx; /* Shrink width to draw */
      if (bFlipx)
         p += sx;
      else
         p -= sx; /* Adjust sprite pointer */
      sx = 0; /* Start at 0 */
      }
   if (sx > 208) /* Part of it is off the right edge */
      xCount = (224-sx); /* Only draw part of it */
   if (sy > 272) /* Part of it is off the bottom edge */
      yCount = (288-sy); /* Only draw part of it */
   if (xCount < 1 || yCount < 1)
      return; /* Nothing to do! */
   d = &pBitmap[sy*iPitch + sx];
   iDestPitch = iPitch - xCount;

/* 4 possible flip cases */
   if (bFlipx && bFlipy) /* Both directions flipped */
      {
#ifdef PORTABLE
      for (y=0; y<yCount; y++)
         for (x=0; x<xCount; x++)
            {
            c = p[(15-y)*16+(15-x)];
            if (c != cTransparent)
               d[y*iPitch+x] = pColorPROM[iColor + c];
            }
#else
   /* Use this to speed up asm code */
   iSrcPitch = xCount - 16;
    _asm {
         mov  esi,p
         add  esi,15*16+15  /* Start from bottom right */
         mov  edi,d
         dec  edi       /* Adjust for pentium optimization */
         mov  ebx,iColor    /* Only low byte used */
         mov  bh,cTransparent
         mov  edx,pColorPROM
         xor  eax,eax
drwsfxy0: mov  ecx,xCount
drwsfxy1: mov  al,[esi]
         inc  edi
         dec  esi
         cmp  al,bh
         jz   drwtfxy0    /* transparent, don't draw it */
         add  al,bl
         mov  al,[edx+eax]
         mov  [edi],al
drwtfxy0: dec  ecx
         jnz  drwsfxy1
         add  edi,iDestPitch
         add  esi,iSrcPitch
         dec  yCount
         jnz  drwsfxy0
         }
#endif
      }
   if (bFlipx && !bFlipy)
      {
#ifdef PORTABLE
      for (y=0; y<yCount; y++)
         for (x=0; x<xCount; x++)
            {
            c = p[y*16+(15-x)];
            if (c != cTransparent)
               d[y*iPitch+x] = pColorPROM[iColor + c];
            }
#else
   /* Use this to speed up asm code */
   iSrcPitch = 16 + xCount;
    _asm {
         mov  esi,p
         add  esi,15  /* Start from opposite direction right */
         mov  edi,d
         dec  edi       /* Adjust for pentium optimization */
         mov  ebx,iColor    /* Only low byte used */
         mov  bh,cTransparent
         mov  edx,pColorPROM
         xor  eax,eax
drwsfx0: mov  ecx,xCount
drwsfx1: mov  al,[esi]
         inc  edi
         dec  esi
         cmp  al,bh
         jz   drwtfx0    /* transparent, don't draw it */
         add  al,bl
         mov  al,[edx+eax]
         mov  [edi],al
drwtfx0: dec  ecx
         jnz  drwsfx1
         add  edi,iDestPitch
         add  esi,iSrcPitch
         dec  yCount
         jnz  drwsfx0
         }
#endif
      }
   if (!bFlipx && !bFlipy) /* Normal direction */
      {
#ifdef PORTABLE
      for (y=0; y<yCount; y++)
         for (x=0; x<xCount; x++)
            {
            c = p[y*16+x];
            if (c != cTransparent)
               d[y*iPitch+x] = pColorPROM[iColor + c];
            }
#else
   /* Use this to speed up asm code */
   iSrcPitch = 16 - xCount;
    _asm {
         mov  esi,p
         mov  edi,d
         dec  edi       /* Adjust for pentium optimization */
         mov  ebx,iColor    /* Only low byte used */
         mov  bh,cTransparent
         mov  edx,pColorPROM
         xor  eax,eax
drws0:   mov  ecx,xCount
drws1:   mov  al,[esi]
         inc  edi
         inc  esi
         cmp  al,bh
         jz   drwt0    /* transparent, don't draw it */
         add  al,bl
         mov  al,[edx+eax]
         mov  [edi],al
drwt0:   dec  ecx
         jnz  drws1
         add  edi,iDestPitch
         add  esi,iSrcPitch
         dec  yCount
         jnz  drws0
         }
#endif
      }
   if (!bFlipx && bFlipy)
      {
#ifdef PORTABLE
      for (y=0; y<yCount; y++)
         for (x=0; x<xCount; x++)
            {
            c = p[(15-y)*16+x];
            if (c != cTransparent)
               d[y*iPitch+x] = pColorPROM[iColor + c];
            }
#else
   /* Use this to speed up asm code */
   iSrcPitch = 16 + xCount;
    _asm {
         mov  esi,p
         add  esi,15*16  /* Start from bottom */
         mov  edi,d
         dec  edi       /* Adjust for pentium optimization */
         mov  ebx,iColor    /* Only low byte used */
         mov  bh,cTransparent
         mov  edx,pColorPROM
         xor  eax,eax
drwsfy0: mov  ecx,xCount
drwsfy1: mov  al,[esi]
         inc  edi
         inc  esi
         cmp  al,bh
         jz   drwtfy0    /* transparent, don't draw it */
         add  al,bl
         mov  al,[edx+eax]
         mov  [edi],al
drwtfy0: dec  ecx
         jnz  drwsfy1
         add  edi,iDestPitch
         sub  esi,iSrcPitch
         dec  yCount
         jnz  drwsfy0
         }
#endif
      }

} /* EMUDrawSprite() */

/****************************************************************************
 *                                                                          *
 *  FUNCTION   : EMUDrawSprite2(int, int, uchar, uchar, bool bool)          *
 *                                                                          *
 *  PURPOSE    : Draw an individual sprite with the given attributes.       *
 *               The transparency color is taken after color translation.   *
 *                                                                          *
 ****************************************************************************/
void EMUDrawSprite2(int sx, int sy, int iSprite, int iColor, BOOL bFlipx, BOOL bFlipy, unsigned char *pSpriteData, unsigned char *pColorPROM)
{
unsigned char *d, *p;
int xCount, yCount, iDestPitch, iSrcPitch;
#ifdef PORTABLE
unsigned char c;
register int x, y;
#endif

/* Adjust for clipped sprite */
   xCount = yCount = 16; /* Assume full size to draw */
   p = &pSpriteData[iSprite * 256];
   if (sy < 0)
      {
      yCount += sy; /* Shrink height to draw */
      if (bFlipy)
         p += (sy*16);
      else
         p -= (sy*16); /* Adjust sprite pointer also */
      sy = 0; /* Start at 0 */
      }
   if (sx < 0)
      {
      xCount += sx; /* Shrink width to draw */
      if (bFlipx)
         p += sx;
      else
         p -= sx; /* Adjust sprite pointer */
      sx = 0; /* Start at 0 */
      }
   if (sx > 208) /* Part of it is off the right edge */
      xCount = (224-sx); /* Only draw part of it */
   if (sy > 272) /* Part of it is off the bottom edge */
      yCount = (288-sy); /* Only draw part of it */
   if (xCount < 1 || yCount < 1)
      return; /* Nothing to do! */
   d = &pBitmap[sy*iPitch + sx];
   iDestPitch = iPitch - xCount;

/* 4 possible flip cases */
   if (bFlipx && bFlipy) /* Both directions flipped */
      {
#ifdef PORTABLE
      for (y=0; y<yCount; y++)
         for (x=0; x<xCount; x++)
            {
            c = pColorPROM[iColor + p[(15-y)*16+(15-x)]];
            if (c != cTransparent)
               d[y*iPitch+x] = c;
            }
#else
   /* Use this to speed up asm code */
   iSrcPitch = xCount - 16;
    _asm {
         mov  esi,p
         add  esi,15*16+15  /* Start from bottom right */
         mov  edi,d
         dec  edi       /* Adjust for pentium optimization */
         mov  ebx,iColor    /* Only low byte used */
         mov  bh,cTransparent
         mov  edx,pColorPROM
         xor  eax,eax
drwsfxy0: mov  ecx,xCount
drwsfxy1: mov  al,[esi]
         inc  edi
         dec  esi
         add  al,bl
         mov  al,[edx+eax]
         cmp  al,bh
         jz   drwtfxy0    /* transparent, don't draw it */
         mov  [edi],al
drwtfxy0: dec  ecx
         jnz  drwsfxy1
         add  edi,iDestPitch
         add  esi,iSrcPitch
         dec  yCount
         jnz  drwsfxy0
         }
#endif
      }
   if (bFlipx && !bFlipy)
      {
#ifdef PORTABLE
      for (y=0; y<yCount; y++)
         for (x=0; x<xCount; x++)
            {
            c = pColorPROM[iColor + p[y*16+(15-x)]];
            if (c != cTransparent)
               d[y*iPitch+x] = c;
            }
#else
   /* Use this to speed up asm code */
   iSrcPitch = 16 + xCount;
    _asm {
         mov  esi,p
         add  esi,15  /* Start from right */
         mov  edi,d
         dec  edi       /* Adjust for pentium optimization */
         mov  ebx,iColor    /* Only low byte used */
         mov  bh,cTransparent
         mov  edx,pColorPROM
         xor  eax,eax
drwsfx0: mov  ecx,xCount
drwsfx1: mov  al,[esi]
         inc  edi
         dec  esi
         add  al,bl
         mov  al,[edx+eax]
         cmp  al,bh
         jz   drwtfx0    /* transparent, don't draw it */
         mov  [edi],al
drwtfx0: dec  ecx
         jnz  drwsfx1
         add  edi,iDestPitch
         add  esi,iSrcPitch
         dec  yCount
         jnz  drwsfx0
         }
#endif
      }
   if (!bFlipx && !bFlipy) /* Normal direction */
      {
#ifdef PORTABLE
      for (y=0; y<yCount; y++)
         for (x=0; x<xCount; x++)
            {
            c = pColorPROM[iColor + p[y*16+x]];
            if (c != cTransparent)
               d[y*iPitch+x] = c;
            }
#else
   /* Use this to speed up asm code */
   iSrcPitch = 16 - xCount;
    _asm {
         mov  esi,p
         mov  edi,d
         dec  edi       /* Adjust for pentium optimization */
         mov  ebx,iColor    /* Only low byte used */
         mov  bh,cTransparent
         mov  edx,pColorPROM
         xor  eax,eax
drws0:   mov  ecx,xCount
drws1:   mov  al,[esi]
         inc  edi
         inc  esi
         add  al,bl
         mov  al,[edx+eax]
         cmp  al,bh
         jz   drwt0    /* transparent, don't draw it */
         mov  [edi],al
drwt0:   dec  ecx
         jnz  drws1
         add  edi,iDestPitch
         add  esi,iSrcPitch
         dec  yCount
         jnz  drws0
         }
#endif
      }
   if (!bFlipx && bFlipy)
      {
#ifdef PORTABLE
      for (y=0; y<yCount; y++)
         for (x=0; x<xCount; x++)
            {
            c = pColorPROM[iColor + p[(15-y)*16+x]];
            if (c != cTransparent)
               d[y*iPitch+x] = c;
            }
#else
   /* Use this to speed up asm code */
   iSrcPitch = 16 + xCount;
    _asm {
         mov  esi,p
         add  esi,15*16  /* Start from bottom */
         mov  edi,d
         dec  edi       /* Adjust for pentium optimization */
         mov  ebx,iColor    /* Only low byte used */
         mov  bh,cTransparent
         mov  edx,pColorPROM
         xor  eax,eax
drwsfy0: mov  ecx,xCount
drwsfy1: mov  al,[esi]
         inc  edi
         inc  esi
         add  al,bl
         mov  al,[edx+eax]
         cmp  al,bh
         jz   drwtfy0    /* transparent, don't draw it */
         mov  [edi],al
drwtfy0: dec  ecx
         jnz  drwsfy1
         add  edi,iDestPitch
         sub  esi,iSrcPitch
         dec  yCount
         jnz  drwsfy0
         }
#endif
      }

} /* EMUDrawSprite2() */
