#include "main.h"

#include <assert.h>
#include <gsl/gsl>
#include <iostream>


#include <nlohmann/json.hpp>
#include <spdlog/fmt/ostr.h>

#include "json.h"
#include "c6809Larry.h"
#include <cxxopts.hpp>

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
    using json = nlohmann::json;

    using fmt::print;

    auto file = "../utils/6809/6809all.raw";

    print("starting\n");
    print("Simulating\n");

    c6809Larry cpu;

    cpu.set_regs(get_initial_regs());

    run_log_t runner(file, 0x1000, {{0, 0x10000, true}});

    runner.do_run(cpu, 100);

    print("{} : sha1\n", regs_t::get_regs_hdr());

    json j = runner;

    std::cout << j << std::endl;

    print("Done\n");

    return 0;
}
