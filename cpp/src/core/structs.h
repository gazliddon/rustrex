#ifndef STRUCTS_H_HGCIST5J
#define STRUCTS_H_HGCIST5J

#include "mem.h"
#include "c6809base.h"

struct cpu_state_t {
    regs_t m_regs;
    std::string m_digest;
    size_t m_cycles;

    friend std::ostream& operator<<(std::ostream& out, cpu_state_t const& lhs) ;
};


struct run_log_t {
    std::vector<mem_descriptor_t> m_memory;
    std::string m_file_name;
    uint16_t m_load_addr;
    std::vector<cpu_state_t> m_states;

    run_log_t(char const* _file, uint16_t _load_addr, std::initializer_list<mem_descriptor_t> _mem);

    void do_run(c6809Base& _cpu, size_t _steps) ;
};


#endif /* end of include guard: STRUCTS_H_HGCIST5J */
