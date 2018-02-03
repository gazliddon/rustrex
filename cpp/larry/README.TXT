EMU programmer,
Included in this zip file is the C source code to my cpu emulators: 6502, 6800,
6809, and Z80.  This code was written to be reasonably efficient, but not truly
optimized for performance.  The main goal was readability and ease of debug.
For performance, I rewrote everything in assembler and at the moment, I am not
ready to release these for public consumption.  If you would like to enhance
the performance of the C code on Intel machines, the first step would be to
remove all use of SHORT variables since most compilers are not very smart and
will create code which uses word sized registers (a performance no-no on Intel).
You are free to use these in your own programs as long as proper credit is
given to me.  If you have any improvements or bug fixes, please let me know and
you will receive credit.  I have not implemented any undocumented instructions
because I have not found any games that need them (so far).  The most tested
of the emulators is my 6809.  It has passed several rigorous tests and performs
flawlessly in all of the 6809 based games I currently support.  If you are
interested in using my ASM based cpu emulators in your own projects, please let
me know.  There is currently one emu author using my 6809 and he is quite
pleased with the performance and reliability; he told me that it was able to run
20 CPU's in parallel, and still run at full speed on his machine.

Also included is the ASM source code to my 6502 emulator as well as the game
driver for Asteroids which shows how it is actually used.  This 6502 emulates
the basic instruction set and not the 65C02 nor the 65816.  If you feel like
adding the 'undocumented' instructions, please send me your results.  You may
use this in your own projects (not for profit) as long as you mention that it
came from me.  Please feel free to pick at it and offer suggestions for
improvements. Some of the code may look like it's doing things backwards;
this is for Pentium and 486 optimization.  The general rule is that sequential
instructions should not reference the same register.  Another useful rules is
that when running in a 32-bit code segment, use of 16-bit registers is a no-no
because it causes a data size override byte and pipe stall to occur.

Enjoy,
Larry Bank 5/25/98
