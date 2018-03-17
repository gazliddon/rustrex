use cpu::Flags;

extern crate num;
use std;

fn a_or_b<T>(f : bool, a : T, b : T) -> T {
    if f {a} else {b}
}

fn one_zero<T : num::One + num::Zero>(f : bool) -> T {
    a_or_b(f, T::one(), T::zero())
}

fn true_false<T : num::Zero + std::cmp::PartialEq>(v : &T) -> bool {
    *v != T::zero()
}

pub fn test_negative<T : GazAlu>(v : u32) -> bool {
    (v & T::hi_bit_mask()) != 0
}

pub fn test_zero<T : GazAlu>(v : u32) -> bool {
    T::from_u32(v) == T::zero()
}

pub fn test_overflow<T : GazAlu>(a : u32, b : u32, r : u32) -> bool {
    ( ( a ^ b ^ r ^ (r >> 1) ) & T::hi_bit_mask() ) != 0
}

pub fn test_carry<T : GazAlu>(_a : u32, _b : u32, r : u32) -> bool {
    (r & (T::hi_bit_mask()<<1)) != 0
}

pub fn test_half<T : GazAlu>(a : u32, b : u32, r : u32) -> bool {
    (a ^ b ^ r) & (T::half_bit_mask()<<1) != 0
}

pub fn get_negative<T : GazAlu>(v : u32) -> u8 {
    a_or_b( test_negative::<T>(v), Flags::N.bits(), 0)
}

pub fn get_zero<T : GazAlu>(v : u32) -> u8 {
    a_or_b( test_zero::<T>(v), Flags::Z.bits(), 0)
}

pub fn get_overflow<T : GazAlu>(a : u32, b : u32, r : u32) -> u8 {
    a_or_b( test_overflow::<T>(a,b,r), Flags::V.bits(), 0)
}

pub fn get_carry<T : GazAlu>(a : u32, b : u32, r : u32) -> u8 {
    a_or_b( test_carry::<T>(a,b,r), Flags::C.bits(), 0)
}

pub fn get_half<T : GazAlu>(a : u32, b : u32, r : u32) -> u8 {
    a_or_b( test_half::<T>(a,b,r), Flags::H.bits(), 0)
}

pub fn nzvch<T : GazAlu>(f : &mut Flags, write_mask : u8, a : u32, b: u32, r: u32) -> T {
    let my_mask = (Flags::N | Flags::Z | Flags::V | Flags::C | Flags::H).bits();

    let new_bits = 
        get_negative::<T>(r) |
        get_zero::<T>(r) |
        get_overflow::<T>(a,b,r) |
        get_carry::<T>(a,b,r) |
        get_half::<T>(a,b,r);

    f.set_w_mask(write_mask & my_mask, new_bits);

    T::from_u32(r)
}

pub fn nzv<T : GazAlu>(f : &mut Flags, write_mask : u8, a : u32, b: u32, r: u32) -> T {
    let my_mask = (Flags::N | Flags::Z | Flags::V).bits();

    let new_bits = 
        get_negative::<T>(r) |
        get_zero::<T>(r) |
        get_overflow::<T>(a,b,r);

    f.set_w_mask(write_mask & my_mask, new_bits);

    T::from_u32(r)
}


pub fn nz<T : GazAlu>(f : &mut Flags, write_mask : u8,r: u32) -> T {

    let write_mask = (Flags::N | Flags::Z).bits() & write_mask;

    f.set_w_mask(write_mask, 
                 get_negative::<T>(r) |
                 get_zero::<T>(r));

    T::from_u32(r)
}

pub trait GazAlu : num::PrimInt + num::traits::WrappingAdd + num::traits::WrappingMul + std::fmt::LowerHex {
    fn hi_bit_mask() -> u32;
    fn from_u32(v : u32) -> Self;
    fn half_bit_mask() -> u32;
    fn mask() -> u32;

    fn add(f : &mut Flags, write_mask : u8, a : u32, b: u32) -> Self {
        f.set(Flags::C, false);
        let c =  one_zero::<u32>(f.contains(Flags::C));
        let r = a.wrapping_add(b).wrapping_add(c);
        nzvch::<Self>(f,write_mask, a, b,r)
    }

    fn eor(f : &mut Flags, write_mask : u8, a : u32, b: u32) -> Self {

        f.set_w_mask(write_mask,0);

        let r = a ^ b;

        nz::<Self>(f, write_mask, r)
    }

    fn dec(f : &mut Flags, write_mask : u8, a : u32) -> Self {

        let r = a.wrapping_sub(1) & Self::mask();

        let v = r == (Self::mask()>>1) || r == Self::mask();

        f.set_w_mask(write_mask, a_or_b(v, Flags::V.bits(), 0));

        nz::<Self>(f, write_mask, r);

        Self::from_u32(r)
    }

    fn inc(f : &mut Flags, write_mask : u8, a : u32) -> Self {

        let r = a.wrapping_add(1) & Self::mask();
        let v = ( r == 0) || r == Self::hi_bit_mask();

        f.set_w_mask(write_mask, a_or_b(v, Flags::V.bits(), 0));

        nz::<Self>(f, write_mask, r);

        Self::from_u32(r)
    }



    fn adc(f : &mut Flags, write_mask : u8, a : u32, b: u32) -> Self {
        let c =  one_zero::<u32>(f.contains(Flags::C));
        let r = a.wrapping_add(b).wrapping_add(c);
        nzvch::<Self>(f,write_mask, a, b,r)
    }

    fn sbc( f : &mut Flags, write_mask : u8, a : u32, b : u32 ) -> Self {
        let c =  one_zero::<u32>(f.contains(Flags::C));
        let r = a.wrapping_sub(b).wrapping_sub(c);
        nzvch::<Self>(f,write_mask, a, b,r)
    }

    fn sub( f : &mut Flags, write_mask : u8, a : u32, b : u32 ) -> Self {
        f.set(Flags::C, false);
        let c =  one_zero::<u32>(f.contains(Flags::C));
        let r = a.wrapping_sub(b).wrapping_sub(c);
        nzvch::<Self>(f,write_mask, a, b,r)
    }

    fn tst(f : &mut Flags, write_mask : u8, a : u32) -> Self {
        f.set_w_mask(write_mask, 0);
        nz::<Self>(f,write_mask, a)
    }

    fn asl(f : &mut Flags, write_mask : u8, a : u32) -> Self {

        let r = a << 1;

        let c_bits = a_or_b(test_negative::<Self>(a), Flags::C.bits(), 0);
        let v_bits = a_or_b(test_negative::<Self>(a ^ r), Flags::V.bits(), 0);

        f.set_w_mask(write_mask, v_bits | c_bits);
        nz::<Self>(f,write_mask, r)
    } 

    fn com(f : &mut Flags, write_mask : u8, a : u32) -> Self {
        let r = (!a) & Self::mask();

        let r = nz::<Self>(f,write_mask, r);

        f.set(Flags::C, true);
        f.set(Flags::V, false);

        r
    } 


    fn and( f : &mut Flags, write_mask : u8, a : u32, b : u32 ) -> Self {
        let r = a & b;
        f.set_w_mask(write_mask, 0);
        nz::<Self>(f, write_mask,r)
    }

    fn asr(f : &mut Flags, write_mask : u8, a : u32) -> Self {

        let mut new_f = Flags::new(0);

        let r = a >> 1 | a_or_b(test_negative::<Self>(a), Self::hi_bit_mask(), 0);

        let c = a & 1 != 0;
        let n = test_negative::<Self>(r);
        let z = test_zero::<Self>(r);

        new_f.set(Flags::C, c);
        new_f.set(Flags::N, n);
        new_f.set(Flags::Z, z);

        f.set_w_mask(write_mask, new_f.bits());

        Self::from_u32(r)
    }

    fn mul(f : &mut Flags, write_mask : u8, a : u32, b :u32) -> Self {

        let r = a.wrapping_mul(b);

        let cbits = a_or_b(true_false(&(r & 0x80)), Flags::C.bits(),0);

        f.set_w_mask(write_mask, cbits);
        nz::<Self>(f, write_mask, r)
    }

    fn lsr(f : &mut Flags, write_mask : u8, a : u32) -> Self {

        let cbits = a_or_b(true_false(&( a & 1 )), Flags::C.bits(),0);

        f.set_w_mask(write_mask & Flags::C.bits(), cbits);

        nz::<Self>(f, write_mask, a >> 1)
    }

    fn or(f : &mut Flags, write_mask : u8, a : u32, b: u32) -> Self {
        f.set_w_mask(write_mask, 0);
        nz::<Self>(f, write_mask, a | b)
    }

    fn neg(f : &mut Flags, write_mask : u8, a : u32) -> Self {

        let r = (a ^ 0xffff).wrapping_add(1);

        let mut fl = 0;

        if test_negative::<Self>(a) {
            fl |= Flags::V.bits()
        }

        if test_negative::<Self>(r) {
            fl |= (Flags::N | Flags::C).bits()
        }

        fl |= get_zero::<Self>(r);

        f.set_w_mask(write_mask, fl);

        Self::from_u32(r)

    }

    fn rol(f : &mut Flags, write_mask : u8, a : u32) -> Self {

        let mut new_bits = 0u8;

        if test_negative::<Self>(a)  { new_bits |= Flags::C.bits(); }

        let r = a << 1 | a_or_b(f.contains(Flags::C), 1,0);

        if test_negative::<Self>(a ^ r) { new_bits |= Flags::V.bits(); }

        f.set_w_mask(write_mask, new_bits);

        nz::<Self>(f, write_mask, r)

    }

    fn ror(f : &mut Flags, write_mask : u8, a : u32) -> Self {

        let mut new_bits = 0u8;

        if a&1 ==1  { new_bits |= Flags::C.bits(); }

        let r = a >> 1 | a_or_b(f.contains(Flags::C), Self::hi_bit_mask(),0);

        f.set_w_mask(write_mask, new_bits);

        nz::<Self>(f, write_mask, r)
    }
}


impl GazAlu for u8 {

    fn mask() -> u32 { 0xff }
    fn hi_bit_mask() -> u32 { 0x80u32 }
    fn from_u32(v : u32) -> u8 { v as u8 }
    fn half_bit_mask() -> u32 {
        0x08
    }
}

impl GazAlu for u16 {

    fn mask() -> u32 { 0xffff }

    fn hi_bit_mask() -> u32 { 0x8000u32 }
    fn from_u32(v : u32) -> u16 { v as u16 }

    fn half_bit_mask() -> u32 {
        0x0800
    }
}

