#include "main.h"

#include <assert.h>
#include <gsl/gsl>
#include <iostream>

#include <nlohmann/json.hpp>
#include <spdlog/fmt/ostr.h>

#include "c6809Larry.h"
#include "json.h"
#include "files.h"

#include <cxxopts.hpp>
#include <fstream>

////////////////////////////////////////////////////////////////////////////////

auto make_options() {

    using namespace cxxopts;
    using std::string;
    using std::vector;

    Options opts("Core", "6809 logger");

    // clang-format 

    opts.add_options()
        ("d,disable-hash", "disable hash generation")
        ("v,verbose", "verbose mode")
        ("j,json", "write json file", value<string>())

        ("c,cpu", "set cpu to test", value<string>()->default_value("larry"))

        ("positional", "", value<vector<string>>())("input", "input file", value<string>());

    opts.parse_positional({"input"});

    // clang-format on

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
        using fmt::print;

        auto opts  = make_options();
        auto popts = opts.parse(argc, argv);

        bool write_json   = popts.count("json") > 0;
        bool verbose      = popts.count("verbose") > 0;
        bool disable_hash = popts.count("disable-hash") > 0;

        if (popts.count("input") == 0) {
            print("You must specify an input file\n");
            exit(10);

        } else {

            using namespace std;

            print("hash_disable: {}\n", disable_hash);
            print("verbose:      {}\n", verbose);

            auto infile = popts["input"].as<std::string>();

            auto text = load_file_as_string(infile.c_str());
            print("{} : loaded\n", infile);

            auto json = nlohmann::json::parse(text);
            print("{}: parsed\n", infile);

            run_log_t runner;

            from_json(json, runner);
            print("{} : parsed into runner file\n", infile);

            c6809Larry cpu;

            runner.do_run(cpu, disable_hash);

            print("test run complete\n");

            if (write_json) {

                nlohmann::json j = runner;

                auto j_file = popts["json"].as<std::string>();

                write_json_file(j, j_file);

                print("written json log file {}\n", j_file);
            }
            return 0;
        }
    } catch (const cxxopts::OptionException& e) {
        std::cout << "error parsing options: " << e.what() << std::endl;
        exit(1);
    }
}
