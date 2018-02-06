#include "c6809Larry.h"

c6809Larry::c6809Larry(std::unique_ptr<cMemIO> _mem)
    : c6809Base(std::move(_mem)) 
{
}

c6809Larry::~c6809Larry() {
}

regs_t c6809Larry::getRegs() const {

    regs_t regs;

    regs.a = m_larryRegs.ucRegA;
    regs.b = m_larryRegs.ucRegA;

    regs.x = m_larryRegs.usRegX;
    regs.y = m_larryRegs.usRegY;

    regs.s = m_larryRegs.usRegS;
    regs.u = m_larryRegs.usRegU;

    regs.dp = m_larryRegs.ucRegDP;
    regs.cc = m_larryRegs.ucRegCC;

    return regs;
}

void c6809Larry::setRegs(regs_t const &_regs) {

    m_larryRegs.ucRegA = _regs.a ;
    m_larryRegs.ucRegA = _regs.b ;

    m_larryRegs.usRegX = _regs.x ;
    m_larryRegs.usRegY = _regs.y ;

    m_larryRegs.usRegS = _regs.s ;
    m_larryRegs.usRegU = _regs.u ;

    m_larryRegs.ucRegDP = _regs.dp ;
    m_larryRegs.ucRegCC = _regs.cc ;
}
/* MC6809 Emulator */

/* void EXEC6809(char *, REGS6809 *, EMUHANDLERS *, int *, unsigned char *); */
/* void RESET6809(char *, REGS6809 *); */
/* void AEXEC6809(char *, REGS6809 *, EMUHANDLERS *, int *, unsigned char *); */
/* void ARESET6809(char *, REGS6809 *); */
/* int DIS6809(char *, REGS6809 *, int *, int *, char *); */
/* void M6809Debug(HANDLE, HWND, REGS6809 *); */

void c6809Larry::step() {

}

void c6809Larry::reset() {

}
