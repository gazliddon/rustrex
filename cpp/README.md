# 6809 Tracer

Provides a harness to create a trace file for a 6809 program. Trace file output is in JSON and contains machine state for each CPU step.This can include:

* Registers
* Flags
* SHA1 check sum of memory if enabled
    * Included for initial step
    * Emitted after any memory writes

Output used to test rustrex CPU against another emulator.



