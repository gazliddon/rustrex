#include "json.h"

using json = nlohmann::json;


void to_json(json & j, regs_t const & _r) {
    j = json{ 
        {"a", _r.a},
        {"b", _r.b},
        {"cc", _r.cc},
        {"dp", _r.dp},
        {"x", _r.x},
        {"y", _r.y},
        {"s", _r.s},
        {"u", _r.u},
        {"pc", _r.pc},
    };

}

void to_json(json & j, mem_descriptor_t const & _mem) {
    j = json {
        {"base", _mem.m_base},
        {"size", _mem.m_size},
        {"writeable", _mem.m_writeable},
    };

}
void to_json(json & j, cpu_state_t const & _s) {
    j = json {
        {"regs", _s.m_regs},
        {"digest", _s.m_digest},
        {"cycles", _s.m_cycles},
    };
}

void to_json(json & j, run_log_t const & _r) {
    j = json {
        {"file_name", _r.m_file_name },
        {"load_addr", _r.m_load_addr },
        {"memory", _r.m_memory },
        {"states", _r.m_states },
    };
}

