#ifndef C6809LARRY_H_HWORZ7TI
#define C6809LARRY_H_HWORZ7TI

#include "c6809base.h"

#include <larry.h>

class c6809Larry : public c6809Base {
  public:
    c6809Larry();
    ~c6809Larry();

    regs_t get_regs() const override;

    void set_regs(regs_t const& _regs) override;
    void step(cMemIO & _mem) override;
    void reset() override;

    unsigned get_cycles() const override {
        return m_cycles;
    }

    void reset_cycles() override {
        m_cycles = 0;
    }

  protected:
    static REGS6809 s_larry_regs;
    static EMUHANDLERS s_emu_handlers;
    static c6809Larry * s_larry;

    static unsigned char read_byte(unsigned short _addr);
    static void write_byte(unsigned short _addr, unsigned char _byte);
    static cMemIO * s_mem;

    unsigned int m_cycles;

};

#endif /* end of include guard: C6809LARRY_H_HWORZ7TI */
