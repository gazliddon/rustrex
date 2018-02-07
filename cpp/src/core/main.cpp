#include "mem.h"

#include <larry.h>

#include "c6809Larry.h"

#include <gsl/gsl>
#include <fstream>
#include <assert.h>
#include <iostream>

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
    using std::cout;

    cout << "starting!\n";

    auto file = "../utils/6809/6809all.raw";
    cout << "loading " << file << "\n";
    auto mem = load_file(file);

    cout << "making memory maps "<< "\n";
    cMemMap memMap;

    memMap.add_mem( std::make_unique<cMemBlock>(0,0x10000));

    /* cout << "getting hashes"<< "\n"; */
    /* char txt[1000]; */
    /* memMap.get_hash().print_hex(txt); */
    /* std::cout << "Before: " << txt << "\n"; */

    cout << "transferring memory"<< "\n";

    memMap.transfer_memory(0,mem);

    /* memMap.get_hash().print_hex(txt); */
    /* std::cout << "after: " << txt << "\n"; */

    cout << "complete"<< "\n";

    return 0; 
}
