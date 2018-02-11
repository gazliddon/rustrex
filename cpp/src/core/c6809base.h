#ifndef C6809BASE_H_T51IQVEN
#define C6809BASE_H_T51IQVEN

#include "mem.h"

class c6809Base {
    public:
        c6809Base() = default;
        virtual ~c6809Base() = default;
        virtual regs_t get_regs() const = 0;
        virtual void set_regs(regs_t const &_regs) = 0;
        virtual void step(cMemIO & _mem, int _cycles = 0) = 0;
        virtual void reset() = 0;

    protected:
};


#endif /* end of include guard: C6809BASE_H_T51IQVEN */
