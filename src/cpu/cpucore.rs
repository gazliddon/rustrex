// Handles CPU emulation

use crate::mem::{ MemoryIO, MemError };
use crate::cpu::{Regs, RegEnum, Flags, InstructionDecoder};
use crate::cpu::{AddressLines, Direct, Extended, Immediate, Inherent, Relative, Indexed};
use crate::cpu::{Clock, alu};

use crate::cpu::alu::{GazAlu};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone )]
pub enum CpuErr {
    UnknownInstruction,
    Unimplemented(InstructionDecoder),
    IllegalAddressingMode,
    Memory(MemError),
}
// use cpu::alu;

fn get_tfr_reg(op : u8 ) -> RegEnum {
    match op {
        0 => RegEnum::D,
        1 => RegEnum::X,
        2 => RegEnum::Y,
        3 => RegEnum::U,
        4 => RegEnum::S,
        5 => RegEnum::PC,
        8 => RegEnum::A,
        9 => RegEnum::B,
        10 =>RegEnum::CC,
        11 =>RegEnum::DP,
        _ => {
            println!("op of {:02X}", op);
            panic!("illegal tfr regs")
        },
    }
}

pub fn get_tfr_regs(op : u8) -> (RegEnum, RegEnum) {
    ( get_tfr_reg(op>>4), get_tfr_reg(op&0xf) ) 
}

pub struct Context<'a, C : 'a + Clock, M : 'a + MemoryIO> {
    regs : &'a mut Regs,
    mem : &'a mut M,
    ref_clock : &'a Rc<RefCell<C>>,
    ins : InstructionDecoder,
}

impl<'a, C : 'a + Clock, M : 'a + MemoryIO> Context<'a, C, M> {
    fn inc_cycles(&mut self) {
        // self.ins.inc_cycles();
        self.ref_clock.borrow_mut().inc_cycles();
    }

    fn add_cycles(&mut self, i0 : usize) {
        // self.ins.add_cycles(i0 as u32);
        self.ref_clock.borrow_mut().add_cycles(i0);
    }
}

impl<'a, C : 'a + Clock, M : 'a + MemoryIO> Context<'a, C, M> {

    fn set_pc(&mut self, v : u16) {
        self.ins.next_addr = v;
    }

    fn get_pc(&self) -> u16 {
        self.ins.next_addr
    }

    fn set_pc_rel(&mut self, v : i16) {
        let pc = self.ins.next_addr.wrapping_add(v as u16);
        self.set_pc(pc)
    }

    fn fetch_byte_as_i16<A : AddressLines>(&mut self) -> Result<i16, CpuErr> {
        let ret = self.fetch_byte::<A>()?;
        Ok(ret as i8 as i16)
    }

    fn store_byte<A: AddressLines>(&mut self, v : u8) -> Result<u16, CpuErr> {
        A::store_byte(self.mem, self.regs, &mut self.ins, v)
    }

    fn store_word<A: AddressLines>(&mut self, v : u16) -> Result<u16, CpuErr> {
        A::store_word(self.mem, self.regs, &mut self.ins, v)
    }

    fn fetch_word_as_i16<A : AddressLines>(&mut self) -> Result<i16, CpuErr> {
        let ret = self.fetch_word::<A>()?;
        Ok( ret as i16 )
    }

    fn fetch_byte<A : AddressLines>(&mut self) -> Result<u8, CpuErr> {
        A::fetch_byte(self.mem, self.regs, &mut self.ins) 
    }

    fn fetch_word<A : AddressLines>(&mut self) -> Result<u16, CpuErr> {
        A::fetch_word(self.mem, self.regs, &mut self.ins)
    }

    fn ea<A : AddressLines>(&mut self) -> Result<u16, CpuErr> {
        A::ea(self.mem, self.regs, &mut self.ins)
    }

    fn op16_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u16, i0 : u16 ) -> Result<u16, CpuErr> {
        let i1 =  self.fetch_word::<A>()?;
        Ok(func(&mut self.regs.flags,
                write_mask,
                u32::from(i0),
                u32::from(i1)))
    }

    fn opd_2< A : AddressLines>( &mut self, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u16 ) -> Result<u16,CpuErr> {
        let i0 = self.regs.get_d();
        self.op16_2::<A>(write_mask, func,i0)
    }

    fn modd_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u16 ) -> Result<u16,CpuErr> { 
        let r = self.opd_2::<A>(write_mask, func)?;
        self.regs.set_d(r);
        Ok(r)
    }

    fn opa_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) -> Result<u8, CpuErr> {
        let i0 = self.regs.a;
        let i1 = self.fetch_byte::<A>()?;
        Ok(func(&mut self.regs.flags, write_mask,u32::from(i0),u32::from(i1)))
    }

    fn opb_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) -> Result<u8, CpuErr> {
        let i0 = self.regs.b;
        let i1 = self.fetch_byte::<A>()?;
        Ok(func(&mut self.regs.flags, write_mask,u32::from(i0),u32::from(i1)))
    }

    fn moda_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) -> Result<u8, CpuErr> {
        let r = self.opa_2::<A>(write_mask, func)?;
        self.regs.a = r;
        Ok(r)
    }

    fn modb_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) -> Result<u8, CpuErr> {
        let r = self.opb_2::<A>(write_mask, func)?;
        self.regs.b = r;
        Ok(r)
    }

    fn opa( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) -> u8 {
        let i0 = self.regs.a;
        func(&mut self.regs.flags, write_mask, u32::from(i0))
    }

    fn opb( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) -> u8 {
        let i0 = self.regs.b;
        func(&mut self.regs.flags, write_mask, u32::from(i0))
    }

    fn moda( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) -> u8 {
        let r = self.opa(write_mask, func);
        self.regs.a = r;
        r
    }

    fn modb( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) -> u8 {
        let r = self.opb(write_mask, func);
        self.regs.b = r;
        r
    }

    fn rwmod8< A : AddressLines>(&mut self,  write_mask : u8, func : fn(&mut Flags, u8, u32) -> u8) -> Result<u8, CpuErr> {
        let ea = self.ea::<A>()?;
        let v = u32::from(self.mem.load_byte(ea));

        let r = func(&mut self.regs.flags, write_mask, v );

        self.mem.store_byte(ea,r);

        Ok(r)
    }

    fn branch< A : AddressLines>(&mut self, v : bool)  -> Result<(),CpuErr> {
        let offset = self.fetch_byte_as_i16::<A>()?;

        if v {
            self.set_pc_rel(offset)
        }
        Ok(())
    }

    fn lbranch< A : AddressLines>(&mut self,v : bool)  -> Result<(), CpuErr> {
        let offset =  self.fetch_word_as_i16::<A>() ?;

        if v {
            self.set_pc_rel(offset)
        }

        Ok(())
    }

    fn post_clear(&mut self) {
        self.regs.flags.set(Flags::Z, true );
        self.regs.flags.set(Flags::N | Flags::V | Flags::C, false );
    }

    fn st8< A : AddressLines>(&mut self, v : u8)  -> Result<(), CpuErr> {
        self.store_byte::<A>(v)?;
        alu::nz::<u8>(&mut self.regs.flags,Flags::NZ.bits(), u32::from(v));
        self.regs.flags.set(Flags::V, false);
        Ok(())
    }

    fn st16< A : AddressLines>(&mut self, v : u16)  -> Result<(), CpuErr> {
        self.store_word::<A>(v)?;
        alu::nz::<u16>(&mut self.regs.flags,Flags::NZ.bits(), u32::from(v ));
        self.regs.flags.set(Flags::V, false);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Stakc functions

impl<'a, C : 'a + Clock, M : 'a + MemoryIO> Context<'a, C, M> {

    fn pushu_byte(&mut self, v : u8) -> Result<(), CpuErr> {
        let u = self.regs.u.wrapping_sub(1);
        self.mem.store_byte(u,v);
        self.regs.u = u;
        Ok(())
    }

    fn pushu_word(&mut self, v : u16) -> Result<(), CpuErr> {
        let u = self.regs.u.wrapping_sub(2);
        self.mem.store_word(u,v);
        self.regs.u = u ;
        Ok(())
    }

    fn popu_byte(&mut self) -> Result<u8, CpuErr> {
        let r = self.mem.load_byte(self.regs.u);
        self.regs.u = self.regs.u.wrapping_add(1);
        Ok(r)
    }

    fn popu_word(&mut self) -> Result<u16, CpuErr> {
        let r = self.mem.load_word(self.regs.u);
        self.regs.u = self.regs.u.wrapping_add(2);
        Ok(r)
    }

    fn pushs_byte(&mut self, v : u8) -> Result<(), CpuErr>{
        let s = self.regs.s.wrapping_sub(1);
        self.mem.store_byte(s,v);
        self.regs.s = s;
        Ok(())
    }

    fn pushs_word(&mut self, v : u16) -> Result<(), CpuErr> {
        let s = self.regs.s.wrapping_sub(2);
        self.mem.store_word(s,v);
        self.regs.s = s ;
        Ok(())
    }

    fn pops_byte(&mut self) -> Result<u8,CpuErr> {
        let r = self.mem.load_byte(self.regs.s);
        self.regs.s = self.regs.s.wrapping_add(1);
        Ok(r)
    }

    fn pops_word(&mut self) -> Result<u16, CpuErr> {
        let r = self.mem.load_word(self.regs.s);
        self.regs.s = self.regs.s.wrapping_add(2);
        Ok(r)
    }

}

////////////////////////////////////////////////////////////////////////////////

impl<'a, C : 'a + Clock, M : 'a + MemoryIO> Context<'a, C, M> {
    fn orcc<A : AddressLines>(&mut self) -> Result<(), CpuErr> {
        let v =  self.fetch_byte::<A>()?;
        let cc = self.regs.flags.bits();
        self.regs.flags.set_flags(v | cc);
        self.inc_cycles();
        Ok(())
    }

    fn stx<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let x = self.regs.x;
        self.st16::<A>(x)
    }

    fn sta<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let r = self.regs.a;
        self.st8::<A>(r)
    }

    fn stb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let r = self.regs.b;
        self.st8::<A>(r)
    }

    fn std<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let r = self.regs.get_d();
        self.st16::<A>(r)
    }

    fn stu<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let r = self.regs.u;
        self.st16::<A>(r)
    }

    fn sty<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let r = self.regs.y;
        self.st16::<A>(r)
    }

    fn sts<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let r = self.regs.s;
        self.st16::<A>(r)
    }

    fn lsla_asla<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda( Flags::NZVC.bits(), u8::asl);
        Ok(())
    }

    fn lslb_aslb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb( Flags::NZVC.bits(), u8::asl);
        Ok(())
    }

    fn asra<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda( Flags::NZVC.bits(), u8::asr);
        Ok(())
    }

    fn asrb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb( Flags::NZVC.bits(), u8::asr);
        Ok(())
    }

    fn asr<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::asr)?;
        Ok(())
    }

    fn tfr<A : AddressLines>(&mut self) -> Result<(), CpuErr> {
        self.add_cycles(4);
        let operand = self.fetch_byte::<A>()?;
        let (a,b) = get_tfr_regs(operand as u8);
        let av = self.regs.get(&a);
        self.regs.set(&b, av);
        Ok(())
    }

    fn abx<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.add_cycles(1);
        let x = self.regs.x;
        self.regs.x = x.wrapping_add(u16::from(self.regs.b));
        Ok(())
    }


    fn beq<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let z = self.regs.flags.contains(Flags::Z);
        self.branch::<A>(z)
    }

    fn bge<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.ge();
        self.branch::<A>(cond)
    }

    fn bgt<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.gt();
        self.branch::<A>(cond)
    }

    fn blo_bcs<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::C);
        self.branch::<A>(cond)
    }

    fn brn<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::N);
        self.branch::<A>(cond)
    }

    fn bhs_bcc<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::C);
        self.branch::<A>(!cond)
    }

    fn bhi<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.hi();
        self.branch::<A>(cond)
    }

    fn ble<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.le();
        self.branch::<A>(cond)
    }

    fn bls<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.ls();
        self.branch::<A>(cond)
    }

    fn blt<A : AddressLines>(&mut self )  -> Result<(), CpuErr> {
        let cond = self.regs.flags.lt();
        self.branch::<A>(cond)
    }

    fn rts<A : AddressLines>(&mut self) -> Result<(), CpuErr> {
        let pc = self.pops_word()?;
        self.set_pc(pc);
        Ok(())
    }

    fn bsr< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let offset = self.fetch_byte_as_i16::<A>()?;
        let next_op = self.get_pc();
        self.pushs_word( next_op)?;
        self.set_pc_rel(offset);
        Ok(())
    }

    fn bvc< A : AddressLines>(&mut self) -> Result<(), CpuErr> {
        let cond = !self.regs.flags.contains(Flags::V);
        self.branch::<A>(cond)
    }

    fn bne< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = !self.regs.flags.contains(Flags::Z);
        self.branch::<A>(cond)
    }

    fn bvs< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::V);
        self.branch::<A>(cond)
    }

    fn bmi< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::N);
        self.branch::<A>(cond)
    }

    fn bra< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.branch::<A>(true)
    }

    fn bpl<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = !self.regs.flags.contains(Flags::N);
        self.branch::<A>(cond)
    }


    fn andcc<A : AddressLines>(&mut self) -> Result<(), CpuErr> {
        self.inc_cycles();
        let i0 = self.regs.flags.bits();
        let i1 = self.fetch_byte::<A>()?;
        let new_f = u8::and(&mut self.regs.flags, 0, u32::from(i0), u32::from(i1));
        self.regs.flags.set_flags(new_f);
        Ok(())
    }

    fn lsl_asl<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::asl)?;
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 

    fn adda<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda_2::<A>( Flags::NZVCH.bits(), u8::add)?;
        Ok(())
    }

    fn adca<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda_2::<A>( Flags::NZVCH.bits(), u8::adc)?;
        Ok(())
    }

    fn adcb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb_2::<A>( Flags::NZVCH.bits(), u8::adc)?;
        Ok(())
    }


    fn addb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb_2::<A>( Flags::NZVCH.bits(), u8::add)?;
        Ok(())
    }

    fn addd<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.inc_cycles();
        self.modd_2::<A>(Flags::NZVC.bits(), u16::add)?;
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn anda<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda_2::<A>( Flags::NZV.bits(), u8::and)?;
        Ok(())
    }

    fn andb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb_2::<A>( Flags::NZV.bits(), u8::and)?;
        Ok(())
    }

    fn bita<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.opa_2::<A>(Flags::NZ.bits(), u8::and)?;
        Ok(())
    }

    fn bitb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.opb_2::<A>(Flags::NZ.bits(), u8::and)?;
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 

    fn clra<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.regs.a = 0;
        self.post_clear();
        Ok(())
    }

    fn clrb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.regs.b = 0;
        self.post_clear();
        Ok(())
    }

    fn clr<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.store_byte::<A>(0)?;
        self.post_clear();
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn cmpa<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.opa_2::<A>(Flags::NZVC.bits(), u8::sub)?;
        Ok(())
    }

    fn cmpb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.opb_2::<A>(Flags::NZVC.bits(), u8::sub)?;
        Ok(())
    }

    fn cmpd< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.regs.get_d();
        self.op16_2::<A>(Flags::NZVC.bits(), u16::sub, i0)?;
        Ok(())
    }

    fn cmpu< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.regs.u;
        self.op16_2::<A>( Flags::NZVC.bits(), u16::sub, i0)?;
        Ok(())
    }


    fn cmps< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.regs.s;
        self.op16_2::<A>( Flags::NZVC.bits(), u16::sub, i0)?;
        Ok(())
    }

    fn cmpx< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.regs.x;
        self.op16_2::<A>( Flags::NZVC.bits(), u16::sub, i0)?;
        Ok(())
    }

    fn cmpy< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.regs.y;
        self.op16_2::<A>( Flags::NZVC.bits(), u16::sub, i0)?;
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn coma< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda( Flags::NZVC.bits(), u8::com);
        Ok(())
    }

    fn comb< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb( Flags::NZVC.bits(), u8::com);
        Ok(())
    }

    fn com< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::com)?;
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn deca< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda( Flags::NZV.bits(), u8::dec);
        Ok(())
    }

    fn decb< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb( Flags::NZV.bits(), u8::dec);
        Ok(())
    }

    fn dec< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZV.bits(), u8::dec)?;
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn inca< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda( Flags::NZV.bits(), u8::inc);
        Ok(())
    }
    fn incb< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb( Flags::NZV.bits(), u8::inc);
        Ok(())
    }
    fn inc< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZV.bits(), u8::inc)?;
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 

    fn lsra< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda( Flags::NZC.bits(), u8::lsr);
        Ok(())
    }

    fn lsrb< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb( Flags::NZC.bits(), u8::lsr);
        Ok(())
    }

    fn lsr< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZC.bits(), u8::lsr)?;
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn eora< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda_2::<A>( Flags::NZV.bits(), u8::eor)?;
        Ok(())
    }
    fn eorb< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb_2::<A>( Flags::NZV.bits(), u8::eor)?;
        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn ora< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda_2::<A>( Flags::NZV.bits(), u8::or)?;
        Ok(())
    }

    fn orb< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb_2::<A>( Flags::NZV.bits(), u8::or)?;
        Ok(())
    }


    //////////////////////////////////////////////////////////////////////////////// 
    fn daa<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        // fuck sakes
        let a = u32::from(self.regs.a);

        let msn = a & 0xf0;
        let lsn = a & 0xf0;

        let mut cf= 0u32;

        if lsn > 0x09 || self.regs.flags.contains(Flags::H) {
            cf |= 0x06;
        }

        if msn > 0x80 && lsn >0x09 {
            cf |= 0x60;
        }

        if msn > 0x90 || self.regs.flags.contains(Flags::C) {
            cf |= 0x60;
        }

        let temp = cf.wrapping_add(a);

        self.regs.flags.set(Flags::C, temp & 0x100 != 0 );
        self.regs.flags.set(Flags::V  | Flags::N, false);


        let new_a = alu::nz::<u8>(&mut self.regs.flags,Flags::NZ.bits(), temp);

        self.regs.a = new_a;

        Ok(())
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn exg<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let operand = self.fetch_byte::<A>()?;
        let (a,b) = get_tfr_regs(operand as u8);
        let av = self.regs.get(&a);
        let bv = self.regs.get(&b);
        self.regs.set(&b, av);
        self.regs.set(&a, bv);
        Ok(())
    }

    fn jsr<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let dest = self.ea::<A>()?;
        let next_op = self.get_pc();
        self.pushs_word(next_op)?;
        self.set_pc(dest);
        Ok(())
    }

    // {{{ Long Branches

    fn lbsr<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let offset = self.fetch_word_as_i16::<A>()?;
        let next_op = self.get_pc();
        self.pushs_word(next_op)?;
        self.set_pc_rel(offset);
        Ok(())
    }

    fn lbrn<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::N);
        self.lbranch::<A>(cond)
    }
    fn lbhi<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.hi();
        self.lbranch::<A>(cond)
    }

    fn lbra<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.lbranch::<A>(true)
    }

    fn lbls<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.ls();
        self.lbranch::<A>(cond)
    }

    fn lble<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.le();
        self.lbranch::<A>(cond)
    }

    fn lbge<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.ge();
        self.lbranch::<A>(cond)
    }
    fn lblt<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.lt();
        self.lbranch::<A>(cond)
    }
    fn lbgt<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.gt();
        self.lbranch::<A>(cond)
    }

    fn lbvc<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = !self.regs.flags.contains(Flags::V);
        self.lbranch::<A>(cond)
    }

    fn lbvs<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::V);
        self.lbranch::<A>(cond)
    }
    fn lbpl<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = !self.regs.flags.contains(Flags::N);
        self.lbranch::<A>(cond)
    }
    fn lbmi<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::N);
        self.lbranch::<A>(cond)
    }

    fn lbhs_lbcc<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::C);
        self.lbranch::<A>(!cond)
    }

    fn lblo_lbcs<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = self.regs.flags.contains(Flags::C);
        self.lbranch::<A>(cond)
    }

    fn lbne<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let cond = !self.regs.flags.contains(Flags::Z);
        self.lbranch::<A>(cond)
    }
    fn lbeq<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let z = self.regs.flags.contains(Flags::Z);
        self.lbranch::<A>(z)
    }
    // }}}

    ////////////////////////////////////////////////////////////////////////////////
    // {{{ Register loads
    fn load_reg_byte<A : AddressLines>(&mut self)  -> Result<u8, CpuErr> {
        let v = self.fetch_byte::<A>()?;
        alu::nz::<u8>(&mut self.regs.flags, Flags::NZ.bits(), u32::from(v));
        self.regs.flags.set(Flags::V, false);
        Ok(v)
    }
    fn load_reg_word<A : AddressLines>(&mut self)  -> Result<u16,CpuErr> {
        let v = self.fetch_word::<A>()?;
        alu::nz::<u16>(&mut self.regs.flags, Flags::NZ.bits(), u32::from(v));
        self.regs.flags.set(Flags::V, false);
        Ok(v)
    }

    fn lda<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.load_reg_byte::<A>()?;
        self.regs.a = i0;
        Ok(())
    }

    fn ldb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.load_reg_byte::<A>()?;
        self.regs.b = i0;
        Ok(())
    }

    fn ldd<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.load_reg_word::<A>()?;
        self.regs.set_d(i0);
        Ok(())
    }

    fn ldx<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.load_reg_word::<A>()?;
        self.regs.x = i0;
        Ok(())
    }

    fn ldy<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.load_reg_word::<A>()?;
        self.regs.y = i0;
        Ok(())
    }

    fn lds<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.load_reg_word::<A>()?;
        self.regs.s = i0;
        Ok(())
    }

    fn ldu<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.load_reg_word::<A>()?;
        self.regs.u = i0;
        Ok(())
    }
    // }}}

    ////////////////////////////////////////////////////////////////////////////////

    fn pshs<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {

        let op = self.fetch_byte::<A>()?;

        let is_set = |m : u8| (op & m) == m;

        if is_set(0x80) {
            let i0 = self.get_pc();
            self.pushs_word( i0)?;
        }

        if is_set( 0x40 ) {
            let i0 = self.regs.u;
            self.pushs_word( i0)?;
        }

        if is_set( 0x20 ) {
            let i0 = self.regs.y;
            self.pushs_word( i0)?;
        }

        if is_set( 0x10 ) {
            let i0 = self.regs.x;
            self.pushs_word( i0)?;
        }

        if is_set( 0x08 ) {
            let i0 = self.regs.dp;
            self.pushs_byte( i0)?;
        }

        if is_set( 0x04 ) {
            let i0 = self.regs.b;
            self.pushs_byte( i0)?;
        }

        if is_set( 0x02 ) {
            let i0 = self.regs.a;
            self.pushs_byte( i0)?;
        }

        if is_set( 0x01 ) {
            let i0 = self.regs.flags.bits();
            self.pushs_byte( i0)?;
        }
        Ok(())
    }

    fn pshu<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {

        let op = self.fetch_byte::<A>()?;

        let is_set = |m : u8| (op & m) == m;

        if is_set(0x80) {
            let i0 = self.get_pc();
            self.pushu_word( i0)?;
        }

        if is_set( 0x40 ) {
            let i0 = self.regs.s;
            self.pushu_word( i0)?;
        }

        if is_set( 0x20 ) {
            let i0 = self.regs.y;
            self.pushu_word( i0)?;
        }

        if is_set( 0x10 ) {
            let i0 = self.regs.x;
            self.pushu_word( i0)?;
        }

        if is_set( 0x08 ) {
            let i0 = self.regs.dp;
            self.pushu_byte( i0)?;
        }

        if is_set( 0x04 ) {
            let i0 = self.regs.b;
            self.pushu_byte( i0)?;
        }

        if is_set( 0x02 ) {
            let i0 = self.regs.a;
            self.pushu_byte( i0)?;
        }

        if is_set( 0x01 ) {
            let i0 = self.regs.flags.bits();
            self.pushu_byte( i0)?;
        }
        Ok(())
    }

    fn puls<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let op = self.fetch_byte::<A>()?;

        if ( op & 0x1 ) == 0x1  {
            let i0 = self.pops_byte()?;
            self.regs.flags.set_flags(i0);
        }

        if ( op & 0x2 ) == 0x2  {
            let i0 = self.pops_byte()?;
            self.regs.a = i0;
        }

        if ( op & 0x4 ) == 0x4  {
            let i0 = self.pops_byte()?;
            self.regs.b = i0;
        }
        if ( op & 0x8 ) == 0x8  {
            let i0 = self.pops_byte()?;
            self.regs.dp = i0;
        }

        if ( op & 0x10 ) == 0x10  {
            let i0 = self.pops_word()?;
            self.regs.x = i0;
        }

        if ( op & 0x20 ) == 0x20  {
            let i0 = self.pops_word()?;
            self.regs.y = i0;
        }

        if ( op & 0x40 ) == 0x40  {
            let i0 = self.pops_word()?;
            self.regs.u = i0;
        }

        if (op & 0x80) == 0x80 {
            let i0 = self.pops_word()?;
            self.set_pc(i0);
        }
        Ok(())
    }
    fn pulu<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let op = self.fetch_byte::<A>()?;

        if ( op & 0x1 ) == 0x1  {
            let i0 = self.popu_byte()?;
            self.regs.flags.set_flags(i0);
        }

        if ( op & 0x2 ) == 0x2  {
            let i0 = self.popu_byte()?;
            self.regs.a = i0;
        }

        if ( op & 0x4 ) == 0x4  {
            let i0 = self.popu_byte()?;
            self.regs.b = i0;
        }
        if ( op & 0x8 ) == 0x8  {
            let i0 = self.popu_byte()?;
            self.regs.dp = i0;
        }

        if ( op & 0x10 ) == 0x10  {
            let i0 = self.popu_word()?;
            self.regs.x = i0;
        }

        if ( op & 0x20 ) == 0x20  {
            let i0 = self.popu_word()?;
            self.regs.y = i0;
        }

        if ( op & 0x40 ) == 0x40  {
            let i0 = self.popu_word()?;
            self.regs.s = i0;
        }

        if (op & 0x80) == 0x80 {
            let i0 = self.popu_word()?;
            self.set_pc(i0);
        }
        Ok(())
    }


    fn mul<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {

        let i0 = self.regs.a;
        let i1 = self.regs.b;

        let r = u16::mul(&mut self.regs.flags, Flags::NZC.bits(), u32::from(i0), u32::from(i1));

        self.regs.set_d(r);
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////

    fn leax<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let ea = self.ea::<A>()?;
        self.regs.flags.set(Flags::Z, ea == 0);
        self.regs.x = ea;
        Ok(())
    }

    fn leay<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let ea = self.ea::<A>()?;
        self.regs.flags.set(Flags::Z, ea == 0);
        self.regs.y = ea;;
        Ok(())
    }

    fn leas<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let ea = self.ea::<A>()?;
        self.regs.s = ea;;
        Ok(())
    }

    fn leau<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let ea = self.ea::<A>()?;
        self.regs.u = ea;;
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////

    fn neg<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::neg)?;
        Ok(())
    }

    fn nega<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda(Flags::NZVC.bits(), u8::neg);
        Ok(())
    }

    fn negb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb(Flags::NZVC.bits(), u8::neg);
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////

    fn nop<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn rol<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::rol)?;
        Ok(())
    }

    fn rola<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda(Flags::NZVC.bits(), u8::rol);
        Ok(())
    }

    fn rolb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb(Flags::NZVC.bits(), u8::rol);
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn ror<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZC.bits(), u8::ror)?;
        Ok(())
    }

    fn rora<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda(Flags::NZC.bits(), u8::ror);
        Ok(())
    }
    fn rorb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb(Flags::NZC.bits(), u8::ror);
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn sbca<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda_2::<A>(Flags::NZVC.bits(), u8::sbc)?;
        Ok(())
    }

    fn sbcb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb_2::<A>(Flags::NZVC.bits(), u8::sbc)?;
        Ok(())

    }
    fn suba<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda_2::<A>(Flags::NZVC.bits(), u8::sub)?;
        Ok(())
    }
    fn subb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb_2::<A>(Flags::NZVC.bits(), u8::sub)?;
        Ok(())
    }


    fn tsta<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.moda(Flags::NZV.bits(), u8::tst);
        Ok(())
    }

    fn tstb<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.modb(Flags::NZV.bits(), u8::tst);
        Ok(())
    }

    fn tst<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.rwmod8::<A>( Flags::NZV.bits(), u8::tst)?;
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn sex<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        if self.regs.b & 0x80 == 0x80 {
            self.regs.a = 0xff;
        } else {
            self.regs.a = 0;
        }

        let d = self.regs.get_d();

        alu::nz::<u16>(&mut self.regs.flags,Flags::NZ.bits(),u32::from(d));
        Ok(())
    }


    fn swi_base<A : AddressLines>(&mut self, vec : u16, flags : Flags)  -> Result<(), CpuErr> {

        macro_rules! push8 {
            ($val:expr) => (
                { let i0 = $val; self.pushs_byte(i0 )? })}

        macro_rules! push16 {
            ($val:expr) => (
                { let i0 = $val; self.pushs_word(i0)? })}

        self.regs.flags |= flags;

        push16!(self.get_pc());
        push16!(self.regs.u);
        push16!(self.regs.y);
        push16!(self.regs.x);

        push8!(self.regs.dp);
        push8!(self.regs.b);
        push8!(self.regs.a);

        push8!(self.regs.flags.bits());

        let pc = self.mem.load_word(vec);
        self.set_pc(pc);
        Ok(())
    }

    fn swi<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.swi_base::<A>(0xfffa, Flags::E | Flags::F)
    }

    fn swi2<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.swi_base::<A>(0xfff4, Flags::E)
    }

    fn swi3<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        self.swi_base::<A>(0xfff2, Flags::E)
    }

    fn subd<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let i0 = self.regs.get_d();
        let r = self.op16_2::<A>(Flags::NZVC.bits(), u16::sub, i0)?;
        self.regs.set_d(r);
        Ok(())
    }

    fn jmp<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        let a = self.ea::<A>()?;
        self.set_pc(a);
        Ok(())
    }

    fn rti<A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        macro_rules! pop8 {
            () => { self.pops_byte()? };

            ($val:expr) => ( { let i0 =  pop8!(); $val = i0 })}

        macro_rules! pop16 {
            () => { self.pops_word()? };
            ($val:expr) => ( { let i0 = pop16!(); $val = i0 })}

        let cc = pop8!();

        self.regs.flags.set_flags(cc);

        if self.regs.flags.contains(Flags::E) {

            pop8!(self.regs.a);
            pop8!(self.regs.b);
            pop8!(self.regs.dp);
            pop16!(self.regs.x);
            pop16!(self.regs.y);
            pop16!(self.regs.u);
        }

        let pc = pop16!();

        self.set_pc(pc);
        Ok(())
    }

    fn cwai< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        Err(CpuErr::Unimplemented(self.ins.clone()))
    }

    fn reset< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        Err(CpuErr::Unimplemented(self.ins.clone()))
    }

    fn sync< A : AddressLines>(&mut self)  -> Result<(), CpuErr> {
        Ok(())
    }

    fn unimplemented(&mut self) -> Result<(), CpuErr> {
        Err(CpuErr::Unimplemented(self.ins.clone()))
    }
}

impl<'a, C : 'a + Clock, M : 'a + MemoryIO> Context<'a, C, M> {

    fn new(mem : &'a mut M, regs : &'a mut Regs, ref_clock: &'a Rc<RefCell<C>>) -> Context<'a, C,M> {
        let ins = InstructionDecoder::new(regs.pc);
        Context { regs, mem, ref_clock, ins, }
    }

    pub fn fetch_instruction(&mut self) -> u16 {
        self.ins.fetch_instruction(self.mem)
    }
}

pub fn reset<M: MemoryIO>(regs : &mut Regs, mem : &mut M) {
    *regs = Regs {
        pc : mem.load_word(0xfffe),
        flags : Flags::I | Flags::F,
        .. Default::default()
    };
}

pub fn step<M: MemoryIO, C : Clock>(regs : &mut Regs, mem : &mut M, ref_clock : &Rc<RefCell<C>>) -> Result<InstructionDecoder, CpuErr> {

    let mut ctx = Context::new(mem,regs,ref_clock);

    macro_rules! handle_op {
        ($addr:ident, $action:ident) => ({ ctx.$action::<$addr>() }) }

    op_table!(ctx.fetch_instruction(), { ctx.unimplemented() })?;

    ctx.regs.pc =  ctx.ins.next_addr;

    Ok(ctx.ins.clone())
}


//
// }}}

