#ifndef C6809BASE_H_T51IQVEN
#define C6809BASE_H_T51IQVEN

#include "mem.h"

class c6809Base {
    public:
        c6809Base(std::unique_ptr<cMemIO> _mem)  :
            m_mem(std::move(_mem)) {
                    };

        virtual ~c6809Base() {}
        virtual regs_t getRegs() const = 0;
        virtual void setRegs(regs_t const &_regs) = 0;
        virtual void step() = 0;
        virtual void reset() = 0;

    protected:

        std::unique_ptr<cMemIO> m_mem;
};


#endif /* end of include guard: C6809BASE_H_T51IQVEN */
