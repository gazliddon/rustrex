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
    using std::cout;
    using std::endl;
    using std::make_unique;

    print("starting\n");

    auto file = "../utils/6809/6809all.raw";

    print("loading {}\n", file);

    auto mem = load_file(file);

    print("making memory maps\n");

    auto mm = make_unique<cMemMap>(make_unique<cMemBlock>(0, 0x10000));

    auto before = mm->get_hash_hex();

    print("loading file {}\n", file);
    load_file(file, *mm, 0x1000);

    auto after = mm->get_hash_hex();

    print("before: {}\n", before);
    print("after:  {}\n", after);

    print("complete\n");

    print("Simulating\n");

    c6809Larry cpu(std::move(mm));

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
    cpu.set_regs(regs);

    while (true) {

        if (regs.pc == 0x13f6) {
            break;
        } else {
            cout << "        PC   D    A  B  X    Y    U    S    DP : flags" << endl;
            print("before: ");
            cout << regs << endl;

            cpu.step(1);

            regs = cpu.get_regs();
            print("after:  ");
            cout << regs << endl << endl;
        }
    }

    print("Done\n");

    return 0;
}
