rem
rem Command line options to build A6502.ASM using Microsoft's MASM 6.11
rem The ASM source was coded using the old 5.1 style (-Zm)
rem and is assumed to be linked with VC++ output (-coff)
rem
rem
d:\masm611\bin\ml -c -Zm -Zi -coff %1.asm
