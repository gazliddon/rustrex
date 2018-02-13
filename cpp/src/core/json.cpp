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


void from_json(nlohmann::json const & j, regs_t & _r) {
    _r.a = j.at("a").get<uint8_t>();
    _r.b = j.at("b").get<uint8_t>();
    _r.cc = j.at("cc").get<uint8_t>();
    _r.dp = j.at("dp").get<uint8_t>();

    _r.x = j.at("x").get<uint16_t>();
    _r.y = j.at("y").get<uint16_t>();
    _r.s = j.at("s").get<uint16_t>();
    _r.u = j.at("u").get<uint16_t>();
    _r.pc = j.at("pc").get<uint16_t>();
}

void from_json(nlohmann::json const & j, mem_descriptor_t & _mem) {
    _mem.m_base = j.at("base").get<uint16_t>();
    _mem.m_size = j.at("size").get<size_t>();
    _mem.m_writeable = j.at("writeable").get<bool>();

}

void from_json(nlohmann::json const & j, cpu_state_t & _s) {
    _s.m_regs=j.at("regs").get<regs_t>();
    _s.m_digest=j.at("digest").get<std::string>();
    _s.m_cycles=j.at("cycles").get<size_t>();

}

void from_json(nlohmann::json const & j, run_log_t & _r) {
    _r.m_file_name = j.at("file_name").get<std::string>();
    _r.m_load_addr  = j.at("load_addr").get<uint16_t>();
    _r.m_memory = j.at("memory").get<std::vector<mem_descriptor_t>>();
    _r.m_states = j.at("states").get<std::vector<cpu_state_t>>();

}


