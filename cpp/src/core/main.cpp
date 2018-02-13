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

/* static auto parse(int argc, char* argv[]) { */
/*     using namespace cxxopts; */
/*     using std::string; */
/*     using std::vector; */

/*     Options opts("Core", "6809 logger"); */

/*     opts.add_options() */
/*         ("v,verbose", "verbose mode") */
/*         ("j,json", "write json file", value<string>()) */
/*         ("c,cpu", "write json file", value<string>()->default_value("larry")) */
/*         ("positional", "", value<vector<string>>()) */
/*         ("input", "input file",  value<string>()) */
/*         ; */


/* } */


////////////////////////////////////////////////////////////////////////////////
int main(int argc, char* argv[]) {
    using namespace cxxopts;
    using std::string;
    using std::vector;

    Options opts("Core", "6809 logger");

    opts.add_options()
        ("v,verbose", "verbose mode")
        ("j,json", "write json file", value<string>())
        ("c,cpu", "write json file", value<string>()->default_value("larry"))
        ("positional", "", value<vector<string>>())
        ("input", "input file",  value<string>())
        ;

    opts.parse_positional({"input", "positional"});

    auto popts =  opts.parse(argc, argv);

    bool write_json = popts.count("json") > 0;
    bool verbose = popts.count("verbose") > 0;

    if (popts.count("input") == 0 ) {

        fmt::print("You must specify an input file\n");
        exit(10);

    } else {

        using json = nlohmann::json;

        using fmt::print;

        auto file = "../utils/6809/6809all.raw";

        c6809Larry cpu;

        cpu.set_regs(get_initial_regs());

        run_log_t runner(file, 0x1000, {{0, 0x10000, true}});

        runner.do_run(cpu, 100);

        json j = runner;

        std::cout << j << std::endl;

        return 0;
    }



}
