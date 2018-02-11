#include "files.h"


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

