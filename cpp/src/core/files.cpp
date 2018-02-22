#include "files.h"
#include <assert.h>
#include <spdlog/fmt/ostr.h>

size_t get_file_size(char const* _fileName) {
    std::ifstream f1(_fileName, std::fstream::binary);

    if (f1.fail()) {
        fmt::print("Error finding the filesize for {}\n", _fileName);
        assert(f1);
    }

    f1.seekg(0, f1.end);
    assert(f1);
    return f1.tellg();
}
std::string load_file_as_string(char const * _name) {
    auto v = load_file(_name);
    return std::string((char const *)v.data(), v.size());
}

std::vector<uint8_t> load_file(char const* _name) {
    auto size = get_file_size(_name);

    std::vector<char> chars;
    chars.reserve(size);

    std::ifstream testFile(_name, std::ios::binary);
    assert(testFile.fail() == false);

    chars.resize(size, 0);

    testFile.read(&chars[0], size);
    assert(testFile.fail() == false);

    std::vector<uint8_t> ret;
    ret.resize(size, 0);
    ret.assign(chars.begin(), chars.end());

    return ret;
}

void load_file(char const* _fileName, cMemIO& _mem, uint16_t _addr) {
    auto mem = load_file(_fileName);
    _mem.set_memory(_addr, mem);
}

