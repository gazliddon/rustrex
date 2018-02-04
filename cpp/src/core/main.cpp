#include <assert.h>
#include <cstdint>
#include <experimental/optional>
#include <memory>
#include <vector>

#include <larry.h>

struct regs_t {
    uint8_t a, b, cc, dp;

    uint16_t x, y, s, u, pc;
};

////////////////////////////////////////////////////////////////////////////////
class cMemIO {
    public:
        cMemIO() {}

        virtual ~cMemIO() {}

        virtual void write_byte(uint16_t _addr, uint8_t _val) = 0;
        virtual uint8_t read_byte(uint16_t _addr) const = 0;
        virtual std::pair<uint16_t, uint16_t> getRange() const = 0;

        virtual bool inRange(uint16_t _addr) const {
            auto x = getRange();
            return _addr >= x.first && _addr <= x.second;
        }

        virtual void write_word(uint16_t _addr, uint16_t _val) {
            auto lo = _val & 0xff;
            auto hi = _val >> 8;
            write_byte(_addr, hi);
            write_byte(_addr + 1, lo);
        }

        virtual uint16_t read_word(uint16_t _addr) const {
            auto hi = read_byte(_addr + 1);
            auto lo = read_byte(_addr);
            return lo + (hi << 8);
        }
};

////////////////////////////////////////////////////////////////////////////////
class cMemBlock : public cMemIO {
    unsigned m_first, m_last, m_size;

    std::vector<uint8_t> m_mem;

    public:
    cMemBlock(uint16_t _first, uint16_t _size)
        : m_first(_first), m_last(_first + _size), m_size(_size) {
            assert(m_last < 0x10000);
            m_mem.resize(m_size);
        }

    ~cMemBlock() override{
    }

    uint8_t read_byte(uint16_t _addr) const override {
        assert(_addr >= m_first && _addr <= m_last);
        return m_mem[_addr - m_first];
    }

    void write_byte(uint16_t _addr, uint8_t _val) override {
        assert(_addr >= m_first && _addr <= m_last);
        m_mem[_addr - m_first] = _val;
    }

    std::pair<uint16_t, uint16_t> getRange() const override {
        return {m_first, m_last};
    }
};

////////////////////////////////////////////////////////////////////////////////
class cMemMap : public cMemIO {
    public:
        cMemMap() {}
        ~cMemMap() override {}

        void add_mem(std::unique_ptr<cMemIO> _mem) {
            m_memblocks.push_back(std::move(_mem));
        }

        uint8_t read_byte(uint16_t _addr) const override {
            auto b = find_block_index(_addr);

            if (b) {
                return m_memblocks[*b]->read_byte(_addr);
            } else {
                return 0;
            }
        }

        void write_byte(uint16_t _addr, uint8_t _val) override {
            auto b = find_block_index(_addr);

            if (b) {
                m_memblocks[*b]->write_byte(_addr, _val);
            }
        }

        bool inRange(uint16_t _addr) const override {
            if (find_block_index(_addr)) {
                return true;
            } else {
                return false;
            }
        }

        std::pair<uint16_t, uint16_t> getRange() const override {
            return {0, 0xfff};
        }

    protected:
        std::experimental::optional<unsigned int> find_block_index(
                uint16_t _addr) const {
            for (auto i = 0u; i < m_memblocks.size(); i++) {
                if (m_memblocks[i]->inRange(_addr)) {
                    return i;
                }
            }

            return {};
        }

        unsigned m_first = 0;
        unsigned m_last = 0xffff;

        std::vector<std::unique_ptr<cMemIO>> m_memblocks;
};

////////////////////////////////////////////////////////////////////////////////
class c6809Base {
    public:
        c6809Base(std::unique_ptr<cMemIO> _mem);

        virtual ~c6809Base() {}
        virtual regs_t getRegs() const = 0;
        virtual void setRegs(regs_t const &_regs) = 0;
        virtual void step() = 0;
        virtual void reset() = 0;

    protected:
        std::unique_ptr<cMemIO> m_mem;
};

////////////////////////////////////////////////////////////////////////////////

class c6809Larry : public c6809Base {
    public:
        c6809Larry(std::unique_ptr<cMemIO> _mem);
        ~c6809Larry() override;
        regs_t getRegs() const override;
        void setRegs(regs_t const &_regs) override;
        void step() override;
        void reset() override;

    protected:
        REGS6809 m_larryRegs;
};

c6809Larry::c6809Larry(std::unique_ptr<cMemIO> _mem)
    : c6809Base(std::move(_mem)) 
{
}

c6809Larry::~c6809Larry() {
}

regs_t c6809Larry::getRegs() const {

    regs_t regs;

    regs.a = m_larryRegs.ucRegA;
    regs.b = m_larryRegs.ucRegA;

    regs.x = m_larryRegs.usRegX;
    regs.y = m_larryRegs.usRegY;

    regs.s = m_larryRegs.usRegS;
    regs.u = m_larryRegs.usRegU;

    regs.dp = m_larryRegs.ucRegDP;
    regs.cc = m_larryRegs.ucRegCC;

    return regs;
}

void c6809Larry::setRegs(regs_t const &_regs) {

    m_larryRegs.ucRegA = _regs.a ;
    m_larryRegs.ucRegA = _regs.b ;

    m_larryRegs.usRegX = _regs.x ;
    m_larryRegs.usRegY = _regs.y ;

    m_larryRegs.usRegS = _regs.s ;
    m_larryRegs.usRegU = _regs.u ;

    m_larryRegs.ucRegDP = _regs.dp ;
    m_larryRegs.ucRegCC = _regs.cc ;
}

void c6809Larry::step() {}
void c6809Larry::reset() {}

////////////////////////////////////////////////////////////////////////////////

int main(int argc, char *argv[]) { return 0; }
