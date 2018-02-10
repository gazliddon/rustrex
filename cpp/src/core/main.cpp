#include "mem.h"

#include <larry.h>

#include "c6809Larry.h"

#include <assert.h>
#include <fstream>
#include <gsl/gsl>
#include <iostream>
#include <spdlog/spdlog.h>

////////////////////////////////////////////////////////////////////////////////

size_t get_file_size(char const* _fileName) {
    std::ifstream f1(_fileName, std::fstream::binary);
    assert(f1);
    f1.seekg(0, f1.end);
    assert(f1);
    return f1.tellg();
}

std::vector<uint8_t> load_file(char const* _name) {
    auto size = get_file_size(_name);

    std::vector<char> chars;
    chars.reserve(size);

    std::ifstream testFile(_name, std::ios::binary);
    chars.resize(size, 0);
    testFile.read(&chars[0], size);

    std::vector<uint8_t> ret;
    ret.resize(size, 0);
    ret.assign(chars.begin(), chars.end());

    return ret;
}

void load_file(char const* _fileName, cMemIO& _mem, uint16_t _addr) {
    auto mem = load_file(_fileName);
    _mem.set_memory(_addr, mem);
}

int main(int argc, char* argv[]) {

    using fmt::print;
    using std::make_unique;

    print("starting\n");

    auto file = "../utils/6809/6809all.raw";

    print("loading {}\n", file);

    auto mem = load_file(file);

    print("making memory maps\n");

    cMemMap memMap(make_unique<cMemBlock>(0, 0x10000));

    auto before = memMap.get_hash_hex();

    print("loading file {}\n", file);
    load_file(file, memMap, 0x1000);

    auto after = memMap.get_hash_hex();

    print("before: {}\n", before);
    print("after:  {}\n", after);

    print("complete\n");

    c6809Larry cpu(
            make_unique<cMemMap>(
                make_unique<cMemBlock>(0, 0x10000)));

    return 0;
}
