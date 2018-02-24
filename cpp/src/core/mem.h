#ifndef MEM_H_PCHU3R9M
#define MEM_H_PCHU3R9M

#include <assert.h>
#include <cstdint>
#include <experimental/optional>
#include <memory>
#include <vector>


#include <gsl/gsl>

#include "sha1.hpp"
#include "regs.h"

namespace opt = std::experimental;

struct mem_descriptor_t {
    uint16_t m_base;
    size_t m_size;
    bool m_writeable;
};


class cMemIO {
  public:
      cMemIO() : m_dirty(false) {
      }

    virtual ~cMemIO() = default;

    virtual void write_byte(uint16_t _addr, uint8_t _val)  = 0;
    virtual uint8_t read_byte(uint16_t _addr) const        = 0;
    virtual std::pair<uint16_t, uint16_t> getRange() const = 0;
    virtual void add_hash(sha1& _hash) const               = 0;

    virtual bool is_dirty() const {
        return m_dirty;
    }

    virtual void clear_dirty() {
        m_dirty = false;
    };

    virtual bool inRange(uint16_t _addr) const;
    virtual void write_word(uint16_t _addr, uint16_t _val);
    virtual uint16_t read_word(uint16_t _addr) const;
    virtual void set_memory(uint16_t _addr, gsl::span<uint8_t> _data);
    virtual std::vector<uint8_t> get_memory(uint16_t _addr, size_t _size) const;
    virtual sha1 get_hash() const;
    virtual std::string get_hash_hex() const;

  protected:
    bool m_dirty;

};

class cMemBlock : public cMemIO {

  public:

    cMemBlock(uint16_t _first, size_t _size, bool _writeable = true);

    cMemBlock(mem_descriptor_t const & _m) : cMemBlock(_m.m_base, _m.m_size, _m.m_writeable) {
    }

    uint8_t read_byte(uint16_t _addr) const override;
    void write_byte(uint16_t _addr, uint8_t _val) override;
    std::pair<uint16_t, uint16_t> getRange() const override;
    virtual void add_hash(sha1& _hash) const override;


  protected:
    unsigned m_first, m_last, m_size;
    bool m_writeable;
    std::vector<uint8_t> m_mem;
};

class cMemMap : public cMemIO {
  public:
    cMemMap();



    cMemMap(std::unique_ptr<cMemIO> _mem) : cMemMap() {
        add_mem(std::move(_mem));
    }

    cMemMap(uint16_t _base, size_t _size) {
        add_mem(std::make_unique<cMemBlock>(_base, _size));
    }

    void add_mem(std::unique_ptr<cMemIO> _mem);

    uint8_t read_byte(uint16_t _addr) const override;
    void write_byte(uint16_t _addr, uint8_t _val) override;
    bool inRange(uint16_t _addr) const override;
    std::pair<uint16_t, uint16_t> getRange() const override;
    virtual void add_hash(sha1& _hash) const override;

    virtual bool is_dirty() const override ;
    virtual void clear_dirty() override ;

  protected:
    opt::optional<unsigned int> find_block_index(uint16_t _addr) const;

    unsigned m_first = 0;
    unsigned m_last  = 0xffff;

    std::vector<std::unique_ptr<cMemIO>> m_memblocks;
};

#endif /* end of include guard: MEM_H_PCHU3R9M */
