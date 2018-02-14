#include "main.h"

#include <assert.h>
#include <gsl/gsl>
#include <iostream>

#include <nlohmann/json.hpp>
#include <spdlog/fmt/ostr.h>

#include "c6809Larry.h"
#include "json.h"
#include <cxxopts.hpp>

#include <fstream>

////////////////////////////////////////////////////////////////////////////////

static auto efault_regs = regs_t {
    0x00,
        0x44,
        0x84,
        0x00,
        0xabab,
        0x2e0,
        0x7f34,
        0x02e0,
        0x1000,
};

auto default_json = R"( 
{
    "file_name": "../utils/6809/6809all.raw",
    "load_addr": 4096,
    "memory": [
        {
            "base": 0,
            "size": 65536,
            "writeable": true
        }
    ],
    "states": [
        {
            "cycles": 0,
            "digest": "93ce8198e7bcfbfa8b6fa58b9e6ff9caec9a7c70",
            "regs": {
                "a": 0,
                "b": 68,
                "dp": 0,
                "flags": { "bits" : 132 },
                "pc" : 4096,
                "s": 32564,
                "u": 736,
                "x": 43947,
                "y": 736
            }
        }
           ]
}
)"_json;

auto make_options() {

    using namespace cxxopts;
    using std::string;
    using std::vector;

    Options opts("Core", "6809 logger");

    opts.add_options()("v,verbose", "verbose mode")("j,json", "write json file", value<string>())(
            "c,cpu", "write json file", value<string>()->default_value("larry"))(
            "positional", "", value<vector<string>>())("input", "input file", value<string>());

    opts.parse_positional({"input"});

    return opts;
}

////////////////////////////////////////////////////////////////////////////////

void write_json_file(nlohmann::json const& j, std::string const& _file_name) {

    std::ofstream out;

    out.open(_file_name, std::ios::binary | std::ios::trunc);

    if (out) {
        out << std::setw(4) << j << "\n";
    } else {
        throw(fmt::format("can't open file {} for writing", _file_name));
    }
}

////////////////////////////////////////////////////////////////////////////////

int main(int argc, char* argv[]) {

    try {
        auto opts  = make_options();
        auto popts = opts.parse(argc, argv);

        bool write_json = popts.count("json") > 0;
        bool verbose    = popts.count("verbose") > 0;

        if (popts.count("input") == 0) {

            fmt::print("You must specify an input file\n");
            exit(10);

        } else {

            using namespace std;
            using fmt::print;

            auto infile = popts["input"];

            run_log_t runner;

            print("about to do it!\n");

            from_json(default_json, runner);

            print("about to done it!\n");

            c6809Larry cpu;

            runner.do_run(cpu, 100);

            if (write_json) {

                nlohmann::json j = runner;

                auto j_file = popts["json"].as<std::string>();

                write_json_file(j, j_file);
            }
            return 0;
        }
    } catch (const cxxopts::OptionException& e) {
        std::cout << "error parsing options: " << e.what() << std::endl;
        exit(1);
    }
}
