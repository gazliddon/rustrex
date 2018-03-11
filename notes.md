# 6522 Notes

## Rom Start
Interactions with VIA on ROM boot


**Setup the data direction for the ports.**

```
W  0xD002 DdrB       : 0x9f 0b10011111
W  0xD003 DdrA       : 0xff 0b11111111
$f151   std   <$02           : f153 9fff 9f ff c800 0000 0000 cbe6 d0 : 01011001 : F | I | N | C 

```

* DDRB Write to
    * multiplexer enable and select
    * Soundchip
    * Comparator
    * Ramp
* DDRA Ouput all

**Now write to selected ports**
```
word
W  0xD000 PortB      : 0x01 0b00000001
W  0xD001 PortA      : 0x00 0b00000000
$f156   std   <$00           : f158 0100 01 00 c800 0000 0000 cbe6 d0 : 01010001 : F | I | C 
```
SH = 1
MPLEXER = 00 == y axisS
!RAMP = active (seems off)

**Set up counters**
```
W  0xD00B AuxCntl    : 0x98 0b1 0 0 110 00
$f15b   sta   <$0B           : f15d 987f 98 7f c800 0000 0000 cbe6 d0 : 01011001 : F | I | N | C 
```

0 - 1 = 00 Port A/b latch disable
2 - 4 = 011 = output to cb2 by phase 2 clock (always set to this)
5     = 0 t2 as one shot timer
6     = 0 t1 as one shot
7     = 1 enable PB7 output???

```
W  0xD004 T1CntH     : 0x7f 0b01111111
$f15d   stb   <$04           : f15f 987f 98 7f c800 0000 0000 cbe6 d0 : 01010001 : F | I | C 

W  0xD00C Cnt1       : 0xcc 0b11001100
$f357   stb   <$0C           : f359 00cc 00 cc c800 0000 0000 cbe4 d0 : 01011001 : F | I | N | C 

W  0xD00A ShiftReg   : 0x00 0b00000000
$f359   sta   <$0A           : f35b 00cc 00 cc c800 0000 0000 cbe4 d0 : 01010101 : F | I | Z | C 

W  0xD001 PortA      : 0x00 0b00000000
$f35e   clr   <$01           : f360 0302 03 02 c800 0000 0000 cbe4 d0 : 01010100 : F | I | Z 

W  0xD000 PortB      : 0x03 0b00000011
$f360   sta   <$00           : f362 0302 03 02 c800 0000 0000 cbe4 d0 : 01010000 : F | I 

W  0xD000 PortB      : 0x02 0b00000010
$f362   stb   <$00           : f364 0302 03 02 c800 0000 0000 cbe4 d0 : 01010000 : F | I 

W  0xD000 PortB      : 0x02 0b00000010
$f364   stb   <$00           : f366 0302 03 02 c800 0000 0000 cbe4 d0 : 01010000 : F | I 

W  0xD000 PortB      : 0x01 0b00000001
$f368   stb   <$00           : f36a 0301 03 01 c800 0000 0000 cbe4 d0 : 01010000 : F | I 

```


## IntEnable

* When writing bit 7 indicates if this is a clear or set operation
* bits 0 - 6 clear / set depending on 7



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
