#include "mem.h"

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

////////////////////////////////////////////////////////////////////////////////

cMemBlock::cMemBlock(uint16_t _first, size_t _size)
    : m_first(_first), m_last(( _first + _size ) -1), m_size(_size) {
    assert(m_last < 0x10000);
    m_mem.resize(m_size);
}

uint8_t cMemBlock::read_byte(uint16_t _addr) const {
    assert(_addr >= m_first && _addr <= m_last);
    return m_mem[_addr - m_first];
}

void cMemBlock::write_byte(uint16_t _addr, uint8_t _val) {
    assert(_addr >= m_first && _addr <= m_last);
    m_mem[_addr - m_first] = _val;
}

std::pair<uint16_t, uint16_t> cMemBlock::getRange() const {
    return {m_first, m_last};
}

void cMemBlock::add_hash(sha1& _hash) const {
    _hash.add(m_mem.data(), m_mem.size());
}

////////////////////////////////////////////////////////////////////////////////

cMemMap::cMemMap() {}

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

std::pair<uint16_t, uint16_t> cMemMap::getRange() const { return {0, 0xfff}; }

opt::optional<unsigned int> cMemMap::find_block_index(
    uint16_t _addr) const {
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
