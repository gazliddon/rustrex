#include "mem.h"

#include <larry.h>

#include "c6809Larry.h"

#include <gsl/gsl>
#include <fstream>
#include <assert.h>
#include <iostream>
#include <spdlog/spdlog.h>

////////////////////////////////////////////////////////////////////////////////

size_t get_file_size(char const * _fileName) {
    std::ifstream f1 (_fileName,std::fstream::binary);
    assert(f1);
    f1.seekg(0, f1.end);
    assert(f1);
    return f1.tellg();
}

std::vector<uint8_t> load_file(char const * _name) {

    auto size = get_file_size(_name);

    std::vector<char> chars;
    chars.reserve(size);

    std::ifstream testFile(_name, std::ios::binary);
    chars.resize(size,0);
    testFile.read( &chars[0], size );

    std::vector<uint8_t> ret;
    ret.resize(size,0);
    ret.assign(chars.begin(), chars.end());

    return ret;
}

int main(int argc, char *argv[]) { 

    using fmt::print;

    print("starting");

    auto file = "../utils/6809/6809all.raw";

    print("loading {}\n", file);

    auto mem = load_file(file);

    print("making memory maps\n" );

    cMemMap memMap;

    auto ptr =  std::make_unique<cMemBlock>(0,0x10000);

    sha1 hash;

    ptr->add_hash(hash);

    memMap.add_mem(std::move(ptr));

    auto hex = memMap.get_hash().get_hex();
    print("before {}\n", hex);

    print("transferring memory\n" );

    memMap.set_memory(0,mem);

    hex = memMap.get_hash().get_hex();
    print("after {}\n", hex);

    print("complete\n");

    return 0; 
}
