#ifndef REGS_H_7MG3EZD9
#define REGS_H_7MG3EZD9

#include <spdlog/fmt/ostr.h>
#include <spdlog/spdlog.h>

struct regs_t {
    uint8_t a, b, cc, dp;
    uint16_t x, y, s, u, pc;

    static char const * get_regs_hdr() {
        return "PC   D    A  B  X    Y    U    S    DP : flags   ";
    }

    friend std::ostream& operator<<(std::ostream& out, regs_t const& lhs) {
        auto x = fmt::format(
            "{:04x} {:04x} {:02x} {:02x} {:04x} {:04x} {:04x} {:04x} {:02x} : {:08b}", lhs.pc,
            (lhs.a << 8) + lhs.b, lhs.a, lhs.b, lhs.x, lhs.y, lhs.u, lhs.s, lhs.dp, lhs.cc);

        out << x;

        return out;
    }
};

#endif /* end of include guard: REGS_H_7MG3EZD9 */
