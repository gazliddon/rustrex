/* Structures and defines for Larry's Arcade Emulator */
/* Written by Larry Bank */
/* Copyright 1998 BitBank Software, Inc. */
/* Project started 1/7/98 */

// #define PORTABLE /* Used for compiling no ASM code into the project */
//
#define _cdecl
typedef int HANDLE;
typedef int HWND;
typedef bool BOOL;
typedef int HDC;
typedef int PALETTEENTRY;
typedef HANDLE HPALETTE;
typedef unsigned int RGBQUAD;
#define TRUE true
#define FALSE false

/* Joystick bitfield values */
#define JOY_BUT1   1
#define JOY_BUT2   2
#define JOY_LEFT   4
#define JOY_RIGHT  8
#define JOY_UP     16
#define JOY_DOWN   32

/* Menu / command definitions */
#define IDM_DATA    400
#define IDM_UPDATE  401
#define IDM_FILLLIST 402

/* Pending interrupt bits */
#define INT_NMI  1
#define INT_FIRQ 2
#define INT_IRQ  4

/* Keyboard bit definitions for main program to process */
#define RKEY_0A        0x0001
#define RKEY_1A        0x0002
#define RKEY_2A        0x0004
#define RKEY_3A        0x0008
#define RKEY_4A        0x0010
#define RKEY_5A        0x0020
#define RKEY_6A        0x0040
#define RKEY_7A        0x0080
#define RKEY_0B        0x0100
#define RKEY_1B        0x0200
#define RKEY_2B        0x0400
#define RKEY_3B        0x0800
#define RKEY_4B        0x1000
#define RKEY_5B        0x2000
#define RKEY_6B        0x4000
#define RKEY_7B        0x8000
#define RKEY_0C        0x010000
#define RKEY_1C        0x020000
#define RKEY_2C        0x040000
#define RKEY_3C        0x080000
#define RKEY_4C        0x100000
#define RKEY_5C        0x200000
#define RKEY_6C        0x400000
#define RKEY_7C        0x800000
#define RKEY_0D        0x01000000
#define RKEY_1D        0x02000000
#define RKEY_2D        0x04000000
#define RKEY_3D        0x08000000
#define RKEY_4D        0x10000000
#define RKEY_5D        0x20000000
#define RKEY_6D        0x40000000
#define RKEY_7D        0x80000000

/* Memory map emulation offsets */
#define MEM_ROM   0x00000 /* Offset to first bank (ROM) */
#define MEM_RAM   0x10000 /* Offset to second bank (RAM) */
#define MEM_FLAGS 0x20000 /* Offset to flags in memory map */

/* Definitions for external memory read and write routines */
typedef unsigned char (_cdecl *MEMRPROC)(unsigned short);
typedef void (_cdecl *MEMWPROC)(unsigned short, unsigned char);
typedef void (_cdecl *INITPROC)(void);
typedef void (_cdecl *GAMEPROC)(HANDLE, HWND, char *, BOOL, BOOL);
typedef void (_cdecl *LISTROMPROC)(HWND, int);

/* Keyboard definition */
typedef struct tagEMUKEYS
{
char *szKeyName; /* ASCII name user sees */
unsigned short usScancode; /* Put a default value here */
unsigned long ulKeyMask;  /* defines a bit to return indicating key is down */
} EMUKEYS;

/* Game options structure for allowing user control of dip switches and optional flags */
typedef struct taggameoptions
{
char *szName;   /* Name of the option to present to the user */
char *szChoices[16]; /* List of up to 16 choices for the user to choose */
int iChoice; /* Choice index, init with default value */
} GAMEOPTIONS;

/* Game definition structure for simple expandability */
typedef struct taggamedef
{
char *szGameName; /* The name displayed to the user */
char *szGameDir; /* Subdirectory name where ROM images reside */
INITPROC pfnInit;  /* Game initialization function */
GAMEPROC pfnGame;  /* Main game entry point */
LISTROMPROC pfnList;  /* Procedure to list the needed ROMS */
GAMEOPTIONS *pGameOptions; /* Pointer to array of n game options, last one has name = NULL */
EMUKEYS *emuKeys; /* Keyboard definitions for each game */
int iWidth, iHeight; /* Screen width and height */
} GAMEDEF;

/* ROM Loader structure */
typedef struct tagLOADROM
{
    char *szROMName;
    int  iROMStart;
    int  iROMLen;
    int  iCheckSum;
    MEMRPROC pfn_read;
    MEMWPROC pfn_write;

} LOADROM;

/* Structure to define special memory mapped device handler routines */
typedef struct tagMEMHANDLERS
{
    unsigned short usStart;
    unsigned short usLen;
    MEMRPROC pfn_read;
    MEMWPROC pfn_write;

} MEMHANDLERS;

/* Structure to pass to CPU emulator with memory handler routines */
typedef struct tagEMUHANDLERS
{
   MEMRPROC pfn_read;
   MEMWPROC pfn_write;

} EMUHANDLERS;

/* Z80 registers */
typedef struct tagREGSZ80
{
  union
    {
    unsigned short usRegAF;
    struct
       {
       unsigned char ucRegF;       /* Main register set */
       unsigned char ucRegA;
       };
    };
  union
    {
    unsigned short usRegBC;
    struct
       {
       unsigned char ucRegC;
       unsigned char ucRegB;
       };
    };
  union
    {
    unsigned short usRegDE;
    struct
       {
       unsigned char ucRegE;
       unsigned char ucRegD;
       };
    };
  union
    {
    unsigned short usRegHL;
    struct
       {
       unsigned char ucRegL;
       unsigned char ucRegH;
       };
    };
  union
    {
    unsigned short usRegAF1;
    struct
       {
       unsigned char ucRegF1;      /* Auxillary register set */
       unsigned char ucRegA1;
       };
    };
  union
    {
    unsigned short usRegBC1;
    struct
       {
       unsigned char ucRegC1;
       unsigned char ucRegB1;
       };
    };
  union
    {
    unsigned short usRegDE1;
    struct
       {
       unsigned char ucRegE1;
       unsigned char ucRegD1;
       };
    };
  union
    {
    unsigned short usRegHL1;
    struct
       {
       unsigned char ucRegL1;
       unsigned char ucRegH1;
       };
    };
unsigned short usRegIX;
unsigned short usRegIY;
unsigned short usRegSP;
unsigned short usRegPC;
unsigned char ucRegI;
unsigned char ucRegR;
unsigned char ucRegIM;
unsigned char ucRegIFF1;
unsigned char ucRegIFF2;
} REGSZ80;

/* 6809 registers */
typedef struct tagREGS6809
{
    unsigned short usRegX;
    unsigned short usRegY;
    unsigned short usRegU;
    unsigned short usRegS;
    unsigned short usRegPC;
    union
       {
       unsigned short usRegD;
       struct
          {
          unsigned char ucRegB; /* On a big-endian machine, reverse A & B */
          unsigned char ucRegA;
          };
       };
    unsigned char ucRegCC;
    unsigned char ucRegDP;
} REGS6809;

/* 6800 registers */
typedef struct tagREGS6800
{
    unsigned short usRegX;
    unsigned short usRegS;
    unsigned short usRegPC;
    unsigned char  ucRegA;
    unsigned char  ucRegB;
    unsigned char  ucRegCC;
} REGS6800;

/* 6502 registers */
typedef struct tagREGS6502
{
    unsigned short usRegPC;
    unsigned char  ucRegX;
    unsigned char  ucRegY;
    unsigned char  ucRegS;
    unsigned char  ucRegA;
    unsigned char  ucRegP;
} REGS6502;

/* Z80 Emulator */
void RESETZ80(REGSZ80 *);
void EXECZ80(char *, REGSZ80 *, EMUHANDLERS *, int *, unsigned char *, unsigned char);
void ARESETZ80(REGSZ80 *);
void AEXECZ80(char *, REGSZ80 *, EMUHANDLERS *, int *, unsigned char *, unsigned char);

/* MC6800/6802/6808 Emulator */
void EXEC6800(char *, REGS6800 *, EMUHANDLERS *, int *, unsigned char *);
void RESET6800(char *, REGS6800 *);
void AEXEC6800(char *, REGS6800 *, EMUHANDLERS *, int *, unsigned char *);
void ARESET6800(char *, REGS6800 *);
int DIS6800(char *, REGS6800 *, int *, int *, char *);

/* MC6809 Emulator */
void EXEC6809(REGS6809 *, EMUHANDLERS *, int *, unsigned char *);
void RESET6809(REGS6809 *);
void AEXEC6809(REGS6809 *, EMUHANDLERS *, int *, unsigned char *);
void ARESET6809(REGS6809 *);
int DIS6809(char *, REGS6809 *, int *, int *, char *);
void M6809Debug(HANDLE, HWND, REGS6809 *);

/* M6502 Emulator */
void EXEC6502(char *, REGS6502 *, EMUHANDLERS *, int *, unsigned char *);
void RESET6502(char *, REGS6502 *);
void AEXEC6502(char *, REGS6502 *, EMUHANDLERS *, int *, unsigned char *);
void ARESET6502(char *, REGS6502 *);
int DIS6502(char *, REGS6502 *, int *, int *, char *);

void EMUCenterDialog(HWND);
BOOL CheckMessages(unsigned long *);
void EMUScreenUpdate(char, HDC, int, int);
HPALETTE EMUCreateIdentityPalette(RGBQUAD *, int);
HPALETTE EMUCreatePalette(void);
void EMUResize(int, int);
BOOL EMULoadHiscore(char *, int);
void EMUSaveHiscore(char *, int);
void EMUSaveState(char, char *, char *, int, char *, int);
BOOL EMULoadState(char, char *, char *, int, char *, int);
BOOL EMULoadRoms(HWND, LOADROM *, char *, int *, EMUHANDLERS *, char *);
void EMUGetJoyInfo(unsigned long *);
unsigned char EMUReadNULL(unsigned short);
void EMUWriteNULL(unsigned short, unsigned char);
void EMUCreateVideoBuffer(int, int, int, char **);
void EMUFreeVideoBuffer(char *);
void EMUUpdatePalette(int, PALETTEENTRY *);
void EMUDrawChar(int, unsigned char *, int, int, int, unsigned char *, unsigned char *);
void EMUDrawSprite(int, int, int, int, BOOL, BOOL, unsigned char *, unsigned char *);
void EMUDrawSprite2(int, int, int, int, BOOL, BOOL, unsigned char *, unsigned char *);

#ifndef MAIN_MODULE
extern int *iCharX, *iCharY;
extern unsigned char cTransparent;
#endif
