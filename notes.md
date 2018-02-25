# Todo

* More emulators to test against
    * Isolate and include mame
    * If I change it to a C library can I just check against this stuff in directly in Rust?
    * Also my emulator could be a library as well

* Cycle counting
    * Output cycle timings from framework

* Cycle accurate bus access?

* Illegal instructions



# C Api

```

typedef void(*)(uint16_t _addr, uint8_t _val) mem_write_byte_t;
typedef uint8_t (*)(uint16_t _addr, uint8_t _val) mem_read_byte_t;
typedef void(*)(uint16_t _addr, uint16_t _val) mem_write_word_t;
typedef uint16_t (*)(uint16_t _addr, uint8_t _val) mem_read_word_t;

struct emu_context_t {

    mem_write_byte_t m_write_byte;
    mem_read_byte_t m_read_byte;
    mem_write_word_t m_write_word;
    mem_read_word_t m_read_word;

    regs_t m_regs;

    uint32_t m_cycles;

    uint32_t m_interrupt;
};

void init();
void step(emu_context_t & context);
char const * get_name();


```
