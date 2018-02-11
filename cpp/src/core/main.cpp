#include "mem.h"

#include <assert.h>
#include <gsl/gsl>
#include <iostream>

#include "c6809Larry.h"
#include "files.h"

////////////////////////////////////////////////////////////////////////////////
struct cpu_state_t {
    regs_t m_regs;
    std::string m_digest;
    size_t m_cycles;

    friend std::ostream& operator<<(std::ostream& out, cpu_state_t const& lhs) {
        out << lhs.m_regs << " : " << lhs.m_digest;
        return out;
    }
};

cpu_state_t get_state(c6809Base const& _cpu, cMemIO const& _mem) {
    cpu_state_t ret{
        _cpu.get_regs(),
        _mem.get_hash_hex(),
        0,
    };

    return ret;
}

struct run_log_t {
    std::vector<mem_descriptor_t> m_memory;
    std::string m_file_name;
    uint16_t m_load_addr;
    std::vector<cpu_state_t> m_states;

    run_log_t(char const* _file, uint16_t _load_addr, std::initializer_list<mem_descriptor_t> _mem)
        : m_memory(_mem), m_file_name(_file), m_load_addr(_load_addr) {
    }

    void do_run(c6809Base& _cpu, size_t _steps) {

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
};

std::vector<cpu_state_t> do_run(c6809Base& _cpu, cMemIO& _mem, size_t _steps) {

    std::vector<cpu_state_t> ret;

    for (auto i = 0u; i < _steps; i++) {
        auto state = get_state(_cpu, _mem);
        ret.push_back(state);
        _cpu.step(_mem, 1);
    }

    ret.push_back(get_state(_cpu, _mem));

    return ret;
}

////////////////////////////////////////////////////////////////////////////////

regs_t get_initial_regs() {
    regs_t regs;

    regs.pc = 0x1000;
    regs.a  = 0x00;
    regs.b  = 0x44;
    regs.x  = 0xabab;
    regs.y  = 0x02e0;
    regs.u  = 0x02e0;
    regs.s  = 0x7f34;
    regs.dp = 0;
    regs.cc = 0x84;

    return regs;
}

////////////////////////////////////////////////////////////////////////////////
int main(int argc, char* argv[]) {

    using fmt::print;

    auto file = "../utils/6809/6809all.raw";

    print("starting\n");
    print("Simulating\n");

    c6809Larry cpu;
    cpu.set_regs(get_initial_regs());

    run_log_t runner(file, 0x1000, {{0, 0x10000, true}});

    runner.do_run(cpu, 100);

    print("{} : sha1\n", regs_t::get_regs_hdr());

    for (auto const& s : runner.m_states) {
        print("{}\n", s);
    }

    print("Done\n");

    return 0;
}
