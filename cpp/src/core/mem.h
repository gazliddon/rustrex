#ifndef MEM_H_PCHU3R9M
#define MEM_H_PCHU3R9M

#include <cstdint>
#include <memory>
#include <vector>
#include <assert.h>
#include <experimental/optional>

#include "sha1.hpp"

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

        virtual sha1 get_hash() const = 0;
};

class cMemBlock : public cMemIO {

    public:
        cMemBlock(uint16_t _first, uint16_t _size);

        uint8_t read_byte(uint16_t _addr) const override; 
        void write_byte(uint16_t _addr, uint8_t _val) override; 
        std::pair<uint16_t, uint16_t> getRange() const override; 
        sha1 get_hash() const override;

    protected:
        unsigned m_first, m_last, m_size;
        std::vector<uint8_t> m_mem;
};

class cMemMap : public cMemIO {
    public:
        cMemMap() ;

        void add_mem(std::unique_ptr<cMemIO> _mem) ;

        uint8_t read_byte(uint16_t _addr) const override ;

        void write_byte(uint16_t _addr, uint8_t _val) override ;

        bool inRange(uint16_t _addr) const override ;

        std::pair<uint16_t, uint16_t> getRange() const override ;
        sha1 get_hash() const override;

    protected:
        std::experimental::optional<unsigned int> find_block_index(
                uint16_t _addr) const ;

        unsigned m_first = 0;
        unsigned m_last = 0xffff;

        std::vector<std::unique_ptr<cMemIO>> m_memblocks;
};

#endif /* end of include guard: MEM_H_PCHU3R9M */
