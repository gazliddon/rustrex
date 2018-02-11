#include "mem.h"
#include <spdlog/spdlog.h>

////////////////////////////////////////////////////////////////////////////////

bool cMemIO::inRange(uint16_t _addr) const {
    auto x = getRange();
    return _addr >= x.first && _addr <= x.second;
}

void cMemIO::write_word(uint16_t _addr, uint16_t _val) {
    auto lo = _val & 0xff;
    auto hi = _val >> 8;
    write_byte(_addr, hi);
    write_byte(_addr + 1, lo);
}

uint16_t cMemIO::read_word(uint16_t _addr) const {
    auto hi = read_byte(_addr + 1);
    auto lo = read_byte(_addr);
    return lo + (hi << 8);
}

void cMemIO::set_memory(uint16_t _addr, gsl::span<uint8_t> _data) {
    for (auto i : _data) {
        write_byte(_addr, i);
        _addr++;
    }
}

std::vector<uint8_t> cMemIO::get_memory(uint16_t _addr, size_t _size) const {
    assert(_addr + _size < 0x10000);
    std::vector<uint8_t> ret;
    ret.resize(_size);

    for (auto i = 0u; i < _size; i++) {
        ret[i] = read_byte(i + _addr);
    }

    return ret;
}

sha1 cMemIO::get_hash() const {
    sha1 hash;
    add_hash(hash);
    hash.finalize();
    return hash;
}

std::string cMemIO::get_hash_hex() const {
    sha1 hash;
    add_hash(hash);
    hash.finalize();
    return hash.get_hex();
}
////////////////////////////////////////////////////////////////////////////////

cMemBlock::cMemBlock(uint16_t _first, size_t _size, bool _writeable)
    : m_first(_first), m_last((_first + _size) - 1), m_size(_size), m_writeable(_writeable) {
    assert(m_last < 0x10000);
    m_mem.resize(m_size);
}

uint8_t cMemBlock::read_byte(uint16_t _addr) const {
    assert(_addr >= m_first && _addr <= m_last);
    auto phys_addr = _addr - m_first;
    return m_mem[phys_addr];
}

void cMemBlock::write_byte(uint16_t _addr, uint8_t _val) {
    assert(_addr >= m_first && _addr <= m_last);
    auto phys_addr = _addr - m_first;
    m_mem[phys_addr] = _val;
}

std::pair<uint16_t, uint16_t> cMemBlock::getRange() const {
    return {m_first, m_last};
}

void cMemBlock::add_hash(sha1& _hash) const {
    _hash.add(m_mem.data(), m_mem.size());
}

////////////////////////////////////////////////////////////////////////////////

cMemMap::cMemMap() {
}

void cMemMap::add_mem(std::unique_ptr<cMemIO> _mem) {
    m_memblocks.push_back(std::move(_mem));
}

uint8_t cMemMap::read_byte(uint16_t _addr) const {
    auto b = find_block_index(_addr);

    if (b) {
        return m_memblocks[*b]->read_byte(_addr);
    } else {
        return 0;
    }
}

void cMemMap::write_byte(uint16_t _addr, uint8_t _val) {
    auto b = find_block_index(_addr);

    if (b) {
        m_memblocks[*b]->write_byte(_addr, _val);
    }
}

bool cMemMap::inRange(uint16_t _addr) const {
    if (find_block_index(_addr)) {
        return true;
    } else {
        return false;
    }
}

std::pair<uint16_t, uint16_t> cMemMap::getRange() const {
    return {0, 0xfff};
}

opt::optional<unsigned int> cMemMap::find_block_index(uint16_t _addr) const {
    for (auto i = 0u; i < m_memblocks.size(); i++) {
        if (m_memblocks[i]->inRange(_addr)) {
            return i;
        }
    }

    return {};
}

void cMemMap::add_hash(sha1& _hash) const {
    for (auto const& b : m_memblocks) {
        b->add_hash(_hash);
    };
}
