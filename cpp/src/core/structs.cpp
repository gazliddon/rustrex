#include "structs.h"

#include "files.h"

#include <spdlog/fmt/ostr.h>

static cpu_state_t get_state(c6809Base const& _cpu, cMemIO const& _mem, bool _disable_hash) {

    cpu_state_t ret;

    ret.m_regs = _cpu.get_regs();

    if (_disable_hash == false) {
        if (_mem.is_dirty()) {
            ret.m_digest = _mem.get_hash_hex();
        }
    }

    ret.m_cycles = 0;

    for(auto i = 0u; i < 5; i++) {
        ret.m_mem[i] = _mem.read_byte(ret.m_regs.pc + i);
    }

    return ret;
}

std::ostream& operator<<(std::ostream& out, cpu_state_t const& lhs) {
    out << lhs.m_regs << " : " << lhs.m_digest;
    return out;
}


run_log_t::run_log_t(char const* _file, uint16_t _load_addr, std::initializer_list<mem_descriptor_t> _mem)
    : m_memory(_mem), m_file_name(_file), m_load_addr(_load_addr) {
    }

void run_log_t::do_run(c6809Base& _cpu, bool _disable_hash) {
    using fmt::print;

    if (!m_states.empty()) {
        auto regs = m_states[0].m_regs;
        _cpu.set_regs(regs);
        m_states.clear();

        print("Initial state set, PC = {:04X}\n", regs.pc );
    } else {
        print("no initial state\n" );
    }

    m_states.reserve(m_instructions);

    cMemMap mem;

    print("adding memory regions\n");

    for (auto const& mb : m_memory) {
        mem.add_mem(std::make_unique<cMemBlock>(mb));
    }

    load_file(m_file_name.c_str(), mem, m_load_addr);

    print("{} loaded to {:04x}\n", m_file_name, m_load_addr );

    print("Running for {} instructions\n", m_instructions );

    for (auto i = 0u; i < m_instructions; i++) {
        auto state = get_state(_cpu, mem, _disable_hash);
        mem.clear_dirty();
        m_states.push_back(state);
        _cpu.step(mem, 1);
    }

    print("run complete\n" );
}



