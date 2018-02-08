#ifndef MEM_H_PCHU3R9M
#define MEM_H_PCHU3R9M

#include <assert.h>
#include <cstdint>
#include <experimental/optional>
#include <memory>
#include <vector>

#include "sha1.hpp"

#include <gsl/gsl>

namespace opt = std::experimental;

struct regs_t {
    uint8_t a, b, cc, dp;
    uint16_t x, y, s, u, pc;
};

class cMemIO {
   public:
    cMemIO() = default;
    virtual ~cMemIO() = default;

    virtual void write_byte(uint16_t _addr, uint8_t _val) = 0;
    virtual uint8_t read_byte(uint16_t _addr) const = 0;
    virtual std::pair<uint16_t, uint16_t> getRange() const = 0;

    virtual bool inRange(uint16_t _addr) const;

    virtual void write_word(uint16_t _addr, uint16_t _val);

    virtual uint16_t read_word(uint16_t _addr) const;

    virtual void add_hash(sha1& _hash) const = 0;

    /* void copy_memory(uint16_t _addr, ); */

    void set_memory(uint16_t _addr, gsl::span<uint8_t> _data) {

        for (auto i : _data) {
            write_byte(_addr, i);
            _addr++;
        }
    }

    std::vector<uint8_t> get_memory(uint16_t _addr, size_t _size) {
        assert(_addr + _size < 0x10000);
        std::vector<uint8_t> ret;
        ret.resize(_size);

        for (auto i = 0u; i < _size; i++) {
            ret[i] = read_byte(i + _addr);
        }

        return ret;
    }

    virtual sha1 get_hash() const {
        sha1 hash;
        add_hash(hash);
        return hash;
    }
};

class cMemBlock : public cMemIO {
   public:
    cMemBlock(uint16_t _first, size_t _size);

    uint8_t read_byte(uint16_t _addr) const override;
    void write_byte(uint16_t _addr, uint8_t _val) override;
    std::pair<uint16_t, uint16_t> getRange() const override;
    virtual void add_hash(sha1& _hash) const override;

   protected:
    unsigned m_first, m_last, m_size;
    std::vector<uint8_t> m_mem;
};

class cMemMap : public cMemIO {
   public:
    cMemMap();

    void add_mem(std::unique_ptr<cMemIO> _mem);

    uint8_t read_byte(uint16_t _addr) const override;

    void write_byte(uint16_t _addr, uint8_t _val) override;

    bool inRange(uint16_t _addr) const override;

    std::pair<uint16_t, uint16_t> getRange() const override;
    virtual void add_hash(sha1& _hash) const override;

   protected:
    opt::optional<unsigned int> find_block_index(
        uint16_t _addr) const;

    unsigned m_first = 0;
    unsigned m_last = 0xffff;

    std::vector<std::unique_ptr<cMemIO>> m_memblocks;
};

#endif /* end of include guard: MEM_H_PCHU3R9M */
