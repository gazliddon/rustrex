#ifndef STRUCTS_H_HGCIST5J
#define STRUCTS_H_HGCIST5J

#include "mem.h"
#include "c6809base.h"
#include <experimental/optional>

#include <array>

struct cpu_state_t {

    using mem_array_t = std::array<uint8_t, 5>;

    template<class T>
    using optional = std::experimental::optional<T>;

    regs_t m_regs;


    size_t m_cycles;

    optional<mem_array_t> m_mem_opt;
    optional<std::string> m_digest_opt;

    friend std::ostream& operator<<(std::ostream& out, cpu_state_t const& lhs) ;
};


struct run_log_t {
    std::vector<mem_descriptor_t> m_memory;
    std::string m_file_name;
    uint16_t m_load_addr;
    std::vector<cpu_state_t> m_states;

    size_t m_instructions;

    run_log_t() {
    }

    run_log_t(char const* _file, uint16_t _load_addr, std::initializer_list<mem_descriptor_t> _mem);

    void do_run(c6809Base& _cpu, bool _do_digest) ;
};


#endif /* end of include guard: STRUCTS_H_HGCIST5J */
