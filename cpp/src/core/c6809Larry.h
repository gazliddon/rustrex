#ifndef C6809LARRY_H_HWORZ7TI
#define C6809LARRY_H_HWORZ7TI

#include "c6809base.h"

#include <larry.h>

class c6809Larry : public c6809Base {
  public:
    c6809Larry(std::unique_ptr<cMemIO> _mem);
    ~c6809Larry();

    regs_t getRegs() const override;

    void setRegs(regs_t const& _regs) override;
    void step() override;
    void reset() override;

  protected:
    static REGS6809 s_larry_regs;
    static EMUHANDLERS s_emu_handlers;
    static c6809Larry * s_larry;
    static unsigned char * s_mem;

    static unsigned char read_byte(unsigned short _addr);
    static void write_byte(unsigned short _addr, unsigned char _byte);

};

#endif /* end of include guard: C6809LARRY_H_HWORZ7TI */
