#ifndef C6809LARRY_H_HWORZ7TI
#define C6809LARRY_H_HWORZ7TI

#include "c6809base.h"

#include <larry.h>

class c6809Larry : public c6809Base {
    public:
        c6809Larry(std::unique_ptr<cMemIO> _mem);
        ~c6809Larry();

        regs_t getRegs() const override;

        void setRegs(regs_t const &_regs) override;
        void step() override;
        void reset() override;

    protected:
        REGS6809 m_larryRegs;
};



#endif /* end of include guard: C6809LARRY_H_HWORZ7TI */
