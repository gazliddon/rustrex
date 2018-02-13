#include "structs.h"

#include "files.h"

static cpu_state_t get_state(c6809Base const& _cpu, cMemIO const& _mem) {
    cpu_state_t ret{
        _cpu.get_regs(),
        _mem.get_hash_hex(),
        0,
    };

    return ret;
}

std::ostream& operator<<(std::ostream& out, cpu_state_t const& lhs) {
    out << lhs.m_regs << " : " << lhs.m_digest;
    return out;
}


run_log_t::run_log_t(char const* _file, uint16_t _load_addr, std::initializer_list<mem_descriptor_t> _mem)
    : m_memory(_mem), m_file_name(_file), m_load_addr(_load_addr) {
    }

void run_log_t::do_run(c6809Base& _cpu, size_t _steps) {

    m_states.clear();

    cMemMap mem;

    for (auto const& mb : m_memory) {
        mem.add_mem(std::make_unique<cMemBlock>(mb));
    }

    load_file(m_file_name.c_str(), mem, 0x1000);

    for (auto i = 0u; i < _steps; i++) {
        auto state = get_state(_cpu, mem);
        m_states.push_back(state);
        _cpu.step(mem, 1);
    }
}



