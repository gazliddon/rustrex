// memory trait
use std::vec::Vec;
use std;
use sha1::Sha1;




fn to_mem_range( address : u16, size :u16 ) -> std::ops::Range<u32> {
    use std::cmp::min;
    let last_mem = address as u32 + size as u32;
    (address as u32 .. min(0x10000, last_mem) )
}

pub fn as_word(lo : u8, hi : u8) -> u16 {
    lo as u16 | (hi as u16) << 8
}

pub fn as_bytes(val : u16) -> (u8,u8) {
    ( (val&0xff) as u8, (val>>8) as u8 )
}

pub trait MemoryIO {
    fn upload(&mut self, addr : u16, data : &[u8]);

    fn get_range(&self) -> (u16, u16);

    fn update_sha1(&self, digest : &mut Sha1);

    fn load_byte(&self, addr:u16) -> u8;
        
    fn store_byte(&mut self, addr:u16, val:u8);

    fn get_sha1_string(&self) -> String {
        let mut m = Sha1::new();
        self.update_sha1(&mut m);
        m.digest().to_string()
    }

    fn get_name(&self) -> String {
        String::from("NO NAME")
    }

    fn is_in_range(&self, val : u16) -> bool {
        let (base, last) = self.get_range();
        (val >= base) && (val <= last)
    }


    fn store_word(&mut self, addr:u16, val:u16) {
        let (lo,hi) = as_bytes(val);
        self.store_byte(addr, hi);
        self.store_byte(addr.wrapping_add(1), lo);
    }

    fn load_word(&self, addr:u16) -> u16 {
        let lo = self.load_byte(addr.wrapping_add(1));
        let hi = self.load_byte(addr);
        as_word(lo, hi)
    }

    fn get_mem_as_str(&self, addr:u16, size:u16 ) -> String {
        let a32 = addr as u32;

        let r = to_mem_range( addr, size);

        let mut v : Vec<String> = Vec::new();

        for a in r {
            let b = self.load_byte(a as u16);
            let t = format!("{:02X}", b);
            v.push(t);
        }

        v.join(" ")
    }
}

