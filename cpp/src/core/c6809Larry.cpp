#include "c6809Larry.h"

/* MC6809 Emulator */

/* void EXEC6809(char *, REGS6809 *, EMUHANDLERS *, int *, unsigned char *); */
/* void RESET6809(char *, REGS6809 *); */
/* void AEXEC6809(char *, REGS6809 *, EMUHANDLERS *, int *, unsigned char *); */
/* void ARESET6809(char *, REGS6809 *); */
/* int DIS6809(char *, REGS6809 *, int *, int *, char *); */
/* void M6809Debug(HANDLE, HWND, REGS6809 *); */

/* Structure to pass to CPU emulator with memory handler routines */
/* typedef struct tagEMUHANDLERS */
/* { */
/*    MEMRPROC pfn_read; */
/*    MEMWPROC pfn_write; */

/* } EMUHANDLERS; */

/* typedef unsigned char (_cdecl *MEMRPROC)(unsigned short); */
/* typedef void (_cdecl *MEMWPROC)(unsigned short, unsigned char); */

// Static vars

c6809Larry* c6809Larry::s_larry = nullptr;
REGS6809 c6809Larry::s_larry_regs;
unsigned char* c6809Larry::s_mem = {0};
EMUHANDLERS c6809Larry::s_emu_handlers{&c6809Larry::read_byte, &c6809Larry::write_byte};

unsigned char c6809Larry::read_byte(unsigned short _addr) {
    assert(0);
    return 0;
}

void c6809Larry::write_byte(unsigned short _addr, unsigned char _byte) {
    assert(0);
}

c6809Larry::c6809Larry(std::unique_ptr<cMemIO> _mem) : c6809Base(std::move(_mem)) {
    assert(s_larry == nullptr);
    s_larry = this;
}

c6809Larry::~c6809Larry() {
    s_larry = nullptr;
}

regs_t c6809Larry::getRegs() const {
    regs_t regs;

    regs.a = s_larry_regs.ucRegA;
    regs.b = s_larry_regs.ucRegA;

    regs.x = s_larry_regs.usRegX;
    regs.y = s_larry_regs.usRegY;

    regs.s = s_larry_regs.usRegS;
    regs.u = s_larry_regs.usRegU;

    regs.dp = s_larry_regs.ucRegDP;
    regs.cc = s_larry_regs.ucRegCC;

    return regs;
}

void c6809Larry::setRegs(regs_t const& _regs) {

    s_larry_regs.ucRegA = _regs.a;
    s_larry_regs.ucRegA = _regs.b;

    s_larry_regs.usRegX = _regs.x;
    s_larry_regs.usRegY = _regs.y;

    s_larry_regs.usRegS = _regs.s;
    s_larry_regs.usRegU = _regs.u;

    s_larry_regs.ucRegDP = _regs.dp;
    s_larry_regs.ucRegCC = _regs.cc;
}

void c6809Larry::step() {
}

void c6809Larry::reset() {
}
