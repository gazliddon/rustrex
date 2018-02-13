	#include "main.h"

#include <assert.h>
#include <gsl/gsl>
#include <iostream>


#include <nlohmann/json.hpp>
#include <spdlog/fmt/ostr.h>

#include "json.h"
#include "c6809Larry.h"
#include <cxxopts.hpp>

#include <fstream>

////////////////////////////////////////////////////////////////////////////////

static nlohmann::json default_json = {

    {"memory", 
        { {   {"base", 0},
              {"size", 0x10000},
              {"writeable", true} }, }, },

    {"file_name", "../utils/6809/6809all.raw"},

    {"load_addr", 0x1000},

    {"states", { { 
                     {"regs", {   { "a",  0x00 },
                                  { "b",  0x44 },
                                  { "dp", 0x00 },
                                  { "cc", 0x84 },
                                  { "x",  0xabab },
                                  { "y",  0x2e0 },
                                  { "s",  0x7f34 },
                                  { "u",  0x02e0 },
                                  { "pc", 0x1000 }, }},
                     {"digest", "kjsakjsak"},

                     {"cycles", 0}, } }
    }

};

auto make_options() {

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

    opts.parse_positional({"input"});

    return opts;
}

////////////////////////////////////////////////////////////////////////////////

void write_json_file(nlohmann::json const & j, std::string const & _file_name) {

    std::ofstream out;

    out.open(_file_name, std::ios::binary | std::ios::trunc);

    if (out) {
        out << j << "\n";
    } else {
        throw("fucked");
    }
}

////////////////////////////////////////////////////////////////////////////////

int main(int argc, char* argv[]) {


    try {
        auto opts = make_options();
        auto popts =  opts.parse(argc, argv);

        bool write_json = popts.count("json") > 0;
        bool verbose = popts.count("verbose") > 0;

        if (popts.count("input") == 0 ) {

            fmt::print("You must specify an input file\n");
            exit(10);

        } else {

            using namespace std;

            auto infile = popts["input"];

            run_log_t runner;

            from_json(default_json, runner);

            c6809Larry cpu;

            runner.do_run(cpu, 100);

            if (write_json) {

                nlohmann::json j = runner;

                auto j_file = popts["json"].as<std::string>();

                write_json_file( j,j_file);
            }
            return 0;
        }
    }

    catch (const cxxopts::OptionException& e)
    {
        std::cout << "error parsing options: " << e.what() << std::endl;
        exit(1);
    }



}
