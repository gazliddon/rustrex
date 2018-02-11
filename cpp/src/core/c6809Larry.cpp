#include "c6809Larry.h"
#include <spdlog/spdlog.h>


// Static vars

c6809Larry* c6809Larry::s_larry = nullptr;
REGS6809 c6809Larry::s_larry_regs;
EMUHANDLERS c6809Larry::s_emu_handlers{&c6809Larry::read_byte, &c6809Larry::write_byte};
cMemIO * c6809Larry::s_mem = nullptr;

unsigned char c6809Larry::read_byte(unsigned short _addr) {
    return s_mem->read_byte(_addr);
}

void c6809Larry::write_byte(unsigned short _addr, unsigned char _byte) {
    s_mem->write_byte(_addr, _byte);
}

c6809Larry::c6809Larry() {
    assert(s_larry == nullptr);
    s_larry = this;
}

c6809Larry::~c6809Larry() {
    s_larry = nullptr;
}

regs_t c6809Larry::get_regs() const {
    regs_t regs;

    regs.a = s_larry_regs.ucRegA;
    regs.b = s_larry_regs.ucRegB;

    regs.x = s_larry_regs.usRegX;
    regs.y = s_larry_regs.usRegY;

    regs.s = s_larry_regs.usRegS;
    regs.u = s_larry_regs.usRegU;

    regs.dp = s_larry_regs.ucRegDP;
    regs.cc = s_larry_regs.ucRegCC;

    regs.pc = s_larry_regs.usRegPC;

    return regs;
}

void c6809Larry::set_regs(regs_t const& _regs) {

    s_larry_regs.ucRegA = _regs.a;
    s_larry_regs.ucRegB = _regs.b;

    s_larry_regs.usRegX = _regs.x;
    s_larry_regs.usRegY = _regs.y;

    s_larry_regs.usRegS = _regs.s;
    s_larry_regs.usRegU = _regs.u;

    s_larry_regs.ucRegDP = _regs.dp;
    s_larry_regs.ucRegCC = _regs.cc;

    s_larry_regs.usRegPC = _regs.pc;
}

void c6809Larry::step(cMemIO & _mem, int _cycles) {
    assert(s_mem == nullptr);
    s_mem = & _mem;
    int cycles = 1;
    unsigned char irqs = 0;
    EXEC6809(&s_larry_regs, &s_emu_handlers, &cycles, &irqs);
    s_mem = nullptr;
}

void c6809Larry::reset() {
}
