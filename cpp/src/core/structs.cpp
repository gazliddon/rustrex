#include "structs.h"

#include "files.h"

#include <spdlog/fmt/ostr.h>


static cpu_state_t get_state(c6809Base const& _cpu, cMemIO const& _mem, bool _add_hash, bool _add_mem) {

    cpu_state_t ret;

    ret.m_regs = _cpu.get_regs();

    if (_add_hash) {
        auto hash = _mem.get_hash_hex();
        ret.m_digest_opt = hash;
    }

    ret.m_cycles = 0;

    if (_add_mem) {
        std::array<uint8_t, 5> mem;

        for(auto i = 0u; i < 5; i++) {
            mem[i] = _mem.read_byte(ret.m_regs.pc + i);
        }
        ret.m_mem_opt = mem;
    }


    return ret;
}

std::ostream& operator<<(std::ostream& out, cpu_state_t const& lhs) {
    out << lhs.m_regs << " : " << *lhs.m_digest_opt;
    return out;
}


run_log_t::run_log_t(char const* _file, uint16_t _load_addr, std::initializer_list<mem_descriptor_t> _mem)
    : m_memory(_mem), m_file_name(_file), m_load_addr(_load_addr) {
    }

void run_log_t::do_run(c6809Base& _cpu, bool _do_digest) {
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

        bool first = i == 0;
        bool last = (i == m_instructions -1);
        bool add_hash;

        if (first || last || (_do_digest && mem.is_dirty())) {

            add_hash = true;
            mem.clear_dirty();

        } else {
            add_hash = false;
        }

        auto state = get_state(_cpu, mem, add_hash, false);

        m_states.push_back(state);

        _cpu.step(mem, 1);
    }

    print("run complete\n" );
}



