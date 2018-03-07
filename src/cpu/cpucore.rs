// Handles CPU emulation

use mem::MemoryIO;
use cpu::{Regs, RegEnum, Flags, InstructionDecoder};
use cpu::{AddressLines, Direct, Extended, Immediate, Inherent, Relative, Indexed};
use cpu::{Clock};

use cpu::alu::{GazAlu};
use cpu::alu;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Host<M: MemoryIO, C : Clock> {
    fn mem(&mut self) -> &mut M;
    fn clock(&mut self) -> &Rc<RefCell<C>>;
    fn regs(&mut self) -> &mut Regs;
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

    fn fetch_byte_as_i16<A : AddressLines>(&mut self) -> i16 {
        self.fetch_byte::<A>() as i8 as i16
    }

    fn store_byte<A: AddressLines>(&mut self, v : u8) {
        A::store_byte(self.mem, self.regs, &mut self.ins, v);
    }

    fn store_word<A: AddressLines>(&mut self, v : u16) {
        A::store_word(self.mem, self.regs, &mut self.ins, v);
    }

    fn fetch_word_as_i16<A : AddressLines>(&mut self) -> i16 {
        self.fetch_word::<A>() as i16
    }

    fn fetch_byte<A : AddressLines>(&mut self) -> u8 {
        A::fetch_byte(self.mem, self.regs, &mut self.ins)
    }

    fn fetch_word<A : AddressLines>(&mut self) -> u16 {
        A::fetch_word(self.mem, self.regs, &mut self.ins)
    }

    fn ea<A : AddressLines>(&mut self) -> u16 {
        A::ea(self.mem, self.regs, &mut self.ins)
    }

    fn op8_2<A : AddressLines>( &mut self, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8, i0 : u8 ) -> u8{
        let i1 = self.fetch_byte::<A>() as u32;
        func(&mut self.regs.flags, write_mask, i0 as u32, i1)
    }

    fn op16_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u16, i0 : u16 ) -> u16 {
        let i1 = self.fetch_word::<A>() as u32;
        func(&mut self.regs.flags, write_mask, i0 as u32,i1)
    }

    fn opd_2< A : AddressLines>( &mut self, write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u16 ) -> u16 {
        let i0 = self.regs.get_d() ;
        self.op16_2::<A>(write_mask, func,i0)
    }

    fn modd_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u16 ) {
        let r = self.opd_2::<A>(write_mask, func);
        self.regs.set_d(r);
    }

    fn opa_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) -> u8 {
        let i0 = self.regs.a as u32;
        let i1 = self.fetch_byte::<A>() as u32;
        func(&mut self.regs.flags, write_mask, i0,i1)
    }

    fn opb_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) -> u8{
        let i0 = self.regs.b as u32;
        let i1 = self.fetch_byte::<A>() as u32;
        func(&mut self.regs.flags, write_mask, i0,i1)
    }

    fn moda_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) {
        let r = self.opa_2::<A>(write_mask, func);
        self.regs.a = r;
    }

    fn modb_2< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32, u32) -> u8 ) {
        let r = self.opb_2::<A>(write_mask, func);
        self.regs.b = r;
    }

    fn opa< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) -> u8{
        let i0 = self.regs.a as u32;
        func(&mut self.regs.flags, write_mask, i0)
    }

    fn opb< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) -> u8 {
        let i0 = self.regs.b as u32;
        func(&mut self.regs.flags, write_mask, i0)
    }

    fn moda< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) {
        let r = self.opa::<A>(write_mask, func);
        self.regs.a = r
    }

    fn modb< A : AddressLines>( &mut self,  write_mask : u8, func : fn(&mut Flags,u8, u32) -> u8 ) {
        let r = self.opb::<A>(write_mask, func);
        self.regs.b = r
    }

    fn rwmod8< A : AddressLines>(&mut self,  write_mask : u8, func : fn(&mut Flags, u8, u32) -> u8) {
        let ea = self.ea::<A>();
        let v = self.mem.load_byte(ea) as u32;
        let r = func(&mut self.regs.flags, write_mask, v );
        self.mem.store_byte(ea,r);
    }

    fn branch< A : AddressLines>(&mut self, v : bool)  {
        let offset = self.fetch_byte_as_i16::<A>();

        if v {
            self.set_pc_rel(offset)
        }
    }

    fn lbranch< A : AddressLines>(&mut self,v : bool)  {
        let offset = self.fetch_word_as_i16::<A>();
        if v {
            self.set_pc_rel(offset)
        }
    }

    fn post_clear(&mut self) {
        self.regs.flags.set(Flags::Z, true );
        self.regs.flags.set(Flags::N | Flags::V | Flags::C, false );
    }

    fn st8< A : AddressLines>(&mut self, v : u8)  {
        self.store_byte::<A>(v);
        alu::nz::<u8>(&mut self.regs.flags,Flags::NZ.bits(), v as u32);
        self.regs.flags.set(Flags::V, false);
    }

    fn st16< A : AddressLines>(&mut self, v : u16)  {
        self.store_word::<A>(v);
        alu::nz::<u16>(&mut self.regs.flags,Flags::NZ.bits(), v as u32);
        self.regs.flags.set(Flags::V, false);
    }
}

impl<'a, C : 'a + Clock, M : 'a + MemoryIO> Context<'a, C, M> {
    fn orcc<A : AddressLines>(&mut self) {
        let v = self.fetch_byte::<A>();
        let cc = self.regs.flags.bits();
        self.regs.flags.set_flags(v | cc);
        self.inc_cycles()
    }

    fn stx<A : AddressLines>(&mut self)  {
        let x = self.regs.x;
        self.st16::<A>(x);
    }

    fn sta<A : AddressLines>(&mut self)  {
        let r = self.regs.a;
        self.st8::<A>(r);
    }

    fn stb<A : AddressLines>(&mut self)  {
        let r = self.regs.b;
        self.st8::<A>(r);
    }

    fn std<A : AddressLines>(&mut self)  {
        let r = self.regs.get_d();
        self.st16::<A>(r);
    }

    fn stu<A : AddressLines>(&mut self)  {
        let r = self.regs.u;
        self.st16::<A>(r);
    }

    fn sty<A : AddressLines>(&mut self)  {
        let r = self.regs.y;
        self.st16::<A>(r);
    }

    fn sts<A : AddressLines>(&mut self)  {
        let r = self.regs.s;
        self.st16::<A>(r);
    }

    fn lsla_asla<A : AddressLines>(&mut self)  {
        self.moda::<A>( Flags::NZVC.bits(), u8::asl);
    }
    
    fn lslb_aslb<A : AddressLines>(&mut self)  {
        self.modb::<A>( Flags::NZVC.bits(), u8::asl);
    }

    fn asra<A : AddressLines>(&mut self)  {
        self.moda::<A>( Flags::NZVC.bits(), u8::asr);
    }

    fn asrb<A : AddressLines>(&mut self)  {
        self.modb::<A>( Flags::NZVC.bits(), u8::asr);
    }

    fn asr<A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::asr);
    }

    fn tfr<A : AddressLines>(&mut self)  {
        self.add_cycles(4);
        let operand = self.fetch_byte::<A>();
        let (a,b) = get_tfr_regs(operand as u8);
        let av = self.regs.get(&a);
        self.regs.set(&b, av);
    }

    fn abx<A : AddressLines>(&mut self)  {
        self.add_cycles(1);
        let x = self.regs.x;
        self.regs.x = x.wrapping_add(self.regs.b as u16);
    }


    fn beq<A : AddressLines>(&mut self)  {
        let z = self.regs.flags.contains(Flags::Z);
        self.branch::<A>(z);
    }

    fn bge<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.ge();
        self.branch::<A>(cond);
    }

    fn bgt<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.gt();
        self.branch::<A>(cond);
    }

    fn blo_bcs<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::C);
        self.branch::<A>(cond);
    }

    fn brn<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::N);
        self.branch::<A>(cond);
    }

    fn bhs_bcc<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::C);
        self.branch::<A>(!cond);
    }

    fn bhi<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.hi();
        self.branch::<A>(cond);
    }

    fn ble<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.le();
        self.branch::<A>(cond);
    }

    fn bls<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.ls();
        self.branch::<A>(cond);
    }

    fn blt<A : AddressLines>(&mut self )  {
        let cond = self.regs.flags.lt();
        self.branch::<A>(cond);
    }

    fn pushu_byte(&mut self, v : u8) {
        let u = self.regs.u.wrapping_sub(1);
        self.mem.store_byte(u,v);
        self.regs.u = u;
    }

    fn pushu_word(&mut self, v : u16) {
        let u = self.regs.u.wrapping_sub(2);
        self.mem.store_word(u,v);
        self.regs.u = u 
    }

    fn popu_byte(&mut self) -> u8 {
        let r = self.mem.load_byte(self.regs.u);
        self.regs.u = self.regs.u.wrapping_add(1);
        r
    }

    fn popu_word(&mut self) -> u16 {
        let r = self.mem.load_word(self.regs.u);
        self.regs.u = self.regs.u.wrapping_add(2);
        r
    }

    fn pushs_byte(&mut self, v : u8) {
        let s = self.regs.s.wrapping_sub(1);
        self.mem.store_byte(s,v);
        self.regs.s = s;
    }

    fn pushs_word(&mut self, v : u16) {
        let s = self.regs.s.wrapping_sub(2);
        self.mem.store_word(s,v);
        self.regs.s = s 
    }

    fn pops_byte(&mut self) -> u8 {
        let r = self.mem.load_byte(self.regs.s);
        self.regs.s = self.regs.s.wrapping_add(1);
        r
    }

    fn pops_word(&mut self) -> u16 {
        let r = self.mem.load_word(self.regs.s);
        self.regs.s = self.regs.s.wrapping_add(2);
        r
    }

    fn rts<A : AddressLines>(&mut self)  {
        let pc = self.pops_word();
        self.set_pc(pc);
    }

    fn bsr< A : AddressLines>(&mut self)  {
        let offset = self.fetch_byte_as_i16::<A>();
        let next_op = self.get_pc();
        self.pushs_word( next_op);
        self.set_pc_rel(offset);
    }

    fn bvc< A : AddressLines>(&mut self)  {
        let cond = !self.regs.flags.contains(Flags::V);
        self.branch::<A>(cond);
    }

    fn bne< A : AddressLines>(&mut self)  {
        let cond = !self.regs.flags.contains(Flags::Z);
        self.branch::<A>(cond);
    }

    fn bvs< A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::V);
        self.branch::<A>(cond);
    }

    fn bmi< A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::N);
        self.branch::<A>(cond);
    }

    fn bra< A : AddressLines>(&mut self)  {
        self.branch::<A>(true);
    }

    fn bpl<A : AddressLines>(&mut self)  {
        let cond = !self.regs.flags.contains(Flags::N);
        self.branch::<A>(cond);
    }


    fn andcc<A : AddressLines>(&mut self)  {
        self.inc_cycles();

        let i0 = self.regs.flags.bits() as u32;
        let i1 = self.fetch_byte::<A>() as u32;
        let new_f = u8::and(&mut self.regs.flags, 0, i0, i1);
        self.regs.flags.set_flags(new_f);
    }

    fn lsl_asl<A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::asl);
    }

    //////////////////////////////////////////////////////////////////////////////// 

    fn adda<A : AddressLines>(&mut self)  {
        self.moda_2::<A>( Flags::NZVCH.bits(), u8::add);
    }

    fn adca<A : AddressLines>(&mut self)  {
        self.moda_2::<A>( Flags::NZVCH.bits(), u8::adc);
    }

    fn adcb<A : AddressLines>(&mut self)  {
        self.modb_2::<A>( Flags::NZVCH.bits(), u8::adc);
    }


    fn addb<A : AddressLines>(&mut self)  {
        self.modb_2::<A>( Flags::NZVCH.bits(), u8::add);
    }

    fn addd<A : AddressLines>(&mut self)  {
        self.inc_cycles();
        self.modd_2::<A>(Flags::NZVC.bits(), u16::add);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn anda<A : AddressLines>(&mut self)  {
        self.moda_2::<A>( Flags::NZV.bits(), u8::and);
    }

    fn andb<A : AddressLines>(&mut self)  {
        self.modb_2::<A>( Flags::NZV.bits(), u8::and);
    }

    fn bita<A : AddressLines>(&mut self)  {
        self.opa_2::<A>(Flags::NZ.bits(), u8::and);
    }

    fn bitb<A : AddressLines>(&mut self)  {
        self.opb_2::<A>(Flags::NZ.bits(), u8::and);
    }

    //////////////////////////////////////////////////////////////////////////////// 

    fn clra<A : AddressLines>(&mut self)  {
        self.regs.a = 0;
        self.post_clear();
    }

    fn clrb<A : AddressLines>(&mut self)  {
        self.regs.b = 0;
        self.post_clear();
    }

    fn clr<A : AddressLines>(&mut self)  {
        self.store_byte::<A>(0);
        self.post_clear();
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn cmpa<A : AddressLines>(&mut self)  {
        self.opa_2::<A>(Flags::NZVC.bits(), u8::sub);
    }

    fn cmpb<A : AddressLines>(&mut self)  {
        self.opb_2::<A>(Flags::NZVC.bits(), u8::sub);
    }

    fn cmpd< A : AddressLines>(&mut self)  {
        let i0 = self.regs.get_d();
        self.op16_2::<A>(Flags::NZVC.bits(), u16::sub, i0);
    }

    fn cmpu< A : AddressLines>(&mut self)  {
        let i0 = self.regs.u;
        self.op16_2::<A>( Flags::NZVC.bits(), u16::sub, i0);
    }


    fn cmps< A : AddressLines>(&mut self)  {
        let i0 = self.regs.s;
        self.op16_2::<A>( Flags::NZVC.bits(), u16::sub, i0);
    }

    fn cmpx< A : AddressLines>(&mut self)  {
        let i0 = self.regs.x;
        self.op16_2::<A>( Flags::NZVC.bits(), u16::sub, i0);
    }

    fn cmpy< A : AddressLines>(&mut self)  {
        let i0 = self.regs.y;
        self.op16_2::<A>( Flags::NZVC.bits(), u16::sub, i0);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn coma< A : AddressLines>(&mut self)  {
        self.moda::<A>( Flags::NZVC.bits(), u8::com);
    }

    fn comb< A : AddressLines>(&mut self)  {
        self.modb::<A>( Flags::NZVC.bits(), u8::com);
    }

    fn com< A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::com);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn deca< A : AddressLines>(&mut self)  {
        self.moda::<A>( Flags::NZV.bits(), u8::dec);
    }

    fn decb< A : AddressLines>(&mut self)  {
        self.modb::<A>( Flags::NZV.bits(), u8::dec);
    }

    fn dec< A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZV.bits(), u8::dec);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn inca< A : AddressLines>(&mut self)  {
        self.moda::<A>( Flags::NZV.bits(), u8::inc);
    }
    fn incb< A : AddressLines>(&mut self)  {
        self.modb::<A>( Flags::NZV.bits(), u8::inc);
    }
    fn inc< A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZV.bits(), u8::inc);
    }

    //////////////////////////////////////////////////////////////////////////////// 

    fn lsra< A : AddressLines>(&mut self)  {
        self.moda::<A>( Flags::NZC.bits(), u8::lsr);
    }

    fn lsrb< A : AddressLines>(&mut self)  {
        self.modb::<A>( Flags::NZC.bits(), u8::lsr);
    }

    fn lsr< A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZC.bits(), u8::lsr);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn eora< A : AddressLines>(&mut self)  {
        self.moda_2::<A>( Flags::NZV.bits(), u8::eor);
    }
    fn eorb< A : AddressLines>(&mut self)  {
        self.modb_2::<A>( Flags::NZV.bits(), u8::eor);
    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn ora< A : AddressLines>(&mut self)  {
        self.moda_2::<A>( Flags::NZV.bits(), u8::or);
    }
    fn orb< A : AddressLines>(&mut self)  {
        self.modb_2::<A>( Flags::NZV.bits(), u8::or);
    }


    //////////////////////////////////////////////////////////////////////////////// 
    fn daa<A : AddressLines>(&mut self)  {
        // fuck sakes
        let a = self.regs.a as u32;

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

        self.regs.a = new_a

    }

    //////////////////////////////////////////////////////////////////////////////// 
    fn exg<A : AddressLines>(&mut self)  {
        let operand = self.fetch_byte::<A>(); 
        let (a,b) = get_tfr_regs(operand as u8);
        let temp = self.regs.get(&b);
        let av = self.regs.get(&a);
        let bv = self.regs.get(&b);
        self.regs.set(&b, av);
        self.regs.set(&a, bv)
    }

    fn jsr<A : AddressLines>(&mut self)  {
        let dest = self.ea::<A>();
        let next_op = self.get_pc();
        self.pushs_word(next_op);
        self.set_pc(dest)
    }

    // {{{ Long Branches

    fn lbsr<A : AddressLines>(&mut self)  {
        let offset = self.fetch_word_as_i16::<A>();
        let next_op = self.get_pc();
        self.pushs_word(next_op);
        self.set_pc_rel(offset);
    }
    
    fn lbrn<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::N);
        self.lbranch::<A>(cond);
    }
    fn lbhi<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.hi();
        self.lbranch::<A>(cond);
    }

    fn lbra<A : AddressLines>(&mut self)  {
        self.lbranch::<A>(true);
    }

    fn lbls<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.ls();
        self.lbranch::<A>(cond);
    }

    fn lble<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.le();
        self.lbranch::<A>(cond);
    }

    fn lbge<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.ge();
        self.lbranch::<A>(cond);
    }
    fn lblt<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.lt();
        self.lbranch::<A>(cond);
    }
    fn lbgt<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.gt();
        self.lbranch::<A>(cond);
    }

    fn lbvc<A : AddressLines>(&mut self)  {
        let cond = !self.regs.flags.contains(Flags::V);
        self.lbranch::<A>(cond);
    }

    fn lbvs<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::V);
        self.lbranch::<A>(cond);
    }
    fn lbpl<A : AddressLines>(&mut self)  {
        let cond = !self.regs.flags.contains(Flags::N);
        self.lbranch::<A>(cond);
    }
    fn lbmi<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::N);
        self.lbranch::<A>(cond);
    }

    fn lbhs_lbcc<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::C);
        self.lbranch::<A>(!cond);
    }

    fn lblo_lbcs<A : AddressLines>(&mut self)  {
        let cond = self.regs.flags.contains(Flags::C);
        self.lbranch::<A>(cond);
    }

    fn lbne<A : AddressLines>(&mut self)  {
        let cond = !self.regs.flags.contains(Flags::Z);
        self.lbranch::<A>(cond);
    }
    fn lbeq<A : AddressLines>(&mut self)  {
        let z = self.regs.flags.contains(Flags::Z);
        self.lbranch::<A>(z);
    }
    // }}}

    ////////////////////////////////////////////////////////////////////////////////
    // {{{ Register loads
    fn load_reg_byte<A : AddressLines>(&mut self)  -> u8 {
        let v = self.fetch_byte::<A>();
        alu::nz::<u8>(&mut self.regs.flags, Flags::NZ.bits(), v as u32);
        self.regs.flags.set(Flags::V, false);
        v
    }
    fn load_reg_word<A : AddressLines>(&mut self)  -> u16 {
        let v = self.fetch_word::<A>();
        alu::nz::<u16>(&mut self.regs.flags, Flags::NZ.bits(), v as u32);
        self.regs.flags.set(Flags::V, false);
        v
    }

    fn lda<A : AddressLines>(&mut self)  {
        let i0 = self.load_reg_byte::<A>();
        self.regs.a = i0
    }

    fn ldb<A : AddressLines>(&mut self)  {
        let i0 = self.load_reg_byte::<A>();
        self.regs.b = i0
    }

    fn ldd<A : AddressLines>(&mut self)  {
        let i0 = self.load_reg_word::<A>();
        self.regs.set_d(i0)
    }

    fn ldx<A : AddressLines>(&mut self)  {
        let i0 = self.load_reg_word::<A>();
        self.regs.x = i0
    }

    fn ldy<A : AddressLines>(&mut self)  {
        let i0 = self.load_reg_word::<A>();
        self.regs.y = i0
    }

    fn lds<A : AddressLines>(&mut self)  {
        let i0 = self.load_reg_word::<A>();
        self.regs.s = i0;
    }

    fn ldu<A : AddressLines>(&mut self)  {
        let i0 = self.load_reg_word::<A>();
        self.regs.u = i0;
    }
    // }}}

    ////////////////////////////////////////////////////////////////////////////////

    fn pshs<A : AddressLines>(&mut self)  {

        let op = self.fetch_byte::<A>();

        let is_set = |m : u8| (op & m) == m;

        if is_set(0x80) {
            let i0 = self.get_pc();
            self.pushs_word( i0);
        }

        if is_set( 0x40 ) {
            let i0 = self.regs.u;
            self.pushs_word( i0);
        }

        if is_set( 0x20 ) {
            let i0 = self.regs.y;
            self.pushs_word( i0);
        }

        if is_set( 0x10 ) {
            let i0 = self.regs.x;
            self.pushs_word( i0);
        }

        if is_set( 0x08 ) {
            let i0 = self.regs.dp;
            self.pushs_byte( i0);
        }

        if is_set( 0x04 ) {
            let i0 = self.regs.b;
            self.pushs_byte( i0);
        }

        if is_set( 0x02 ) {
            let i0 = self.regs.a;
            self.pushs_byte( i0);
        }

        if is_set( 0x01 ) {
            let i0 = self.regs.flags.bits();
            self.pushs_byte( i0);
        }
    }

    fn pshu<A : AddressLines>(&mut self)  {

        let op = self.fetch_byte::<A>();

        let is_set = |m : u8| (op & m) == m;

        if is_set(0x80) {
            let i0 = self.get_pc();
            self.pushu_word( i0);
        }

        if is_set( 0x40 ) {
            let i0 = self.regs.s;
            self.pushu_word( i0);
        }

        if is_set( 0x20 ) {
            let i0 = self.regs.y;
            self.pushu_word( i0);
        }

        if is_set( 0x10 ) {
            let i0 = self.regs.x;
            self.pushu_word( i0);
        }

        if is_set( 0x08 ) {
            let i0 = self.regs.dp;
            self.pushu_byte( i0);
        }

        if is_set( 0x04 ) {
            let i0 = self.regs.b;
            self.pushu_byte( i0);
        }

        if is_set( 0x02 ) {
            let i0 = self.regs.a;
            self.pushu_byte( i0);
        }

        if is_set( 0x01 ) {
            let i0 = self.regs.flags.bits();
            self.pushu_byte( i0);
        }
    }

    fn puls<A : AddressLines>(&mut self)  {
        let op = self.fetch_byte::<A>();

        if ( op & 0x1 ) == 0x1  {
            let i0 = self.pops_byte();
            self.regs.flags.set_flags(i0);
        }

        if ( op & 0x2 ) == 0x2  {
            let i0 = self.pops_byte();
            self.regs.a = i0;
        }

        if ( op & 0x4 ) == 0x4  {
            let i0 = self.pops_byte();
            self.regs.b = i0;
        }
        if ( op & 0x8 ) == 0x8  {
            let i0 = self.pops_byte();
            self.regs.dp = i0;
        }

        if ( op & 0x10 ) == 0x10  {
            let i0 = self.pops_word();
            self.regs.x = i0;
        }

        if ( op & 0x20 ) == 0x20  {
            let i0 = self.pops_word();
            self.regs.y = i0;
        }

        if ( op & 0x40 ) == 0x40  {
            let i0 = self.pops_word();
            self.regs.u = i0;
        }

        if (op & 0x80) == 0x80 {
            let i0 = self.pops_word();
            self.set_pc(i0);
        }
    }
    fn pulu<A : AddressLines>(&mut self)  {
        let op = self.fetch_byte::<A>();

        if ( op & 0x1 ) == 0x1  {
            let i0 = self.popu_byte();
            self.regs.flags.set_flags(i0);
        }

        if ( op & 0x2 ) == 0x2  {
            let i0 = self.popu_byte();
            self.regs.a = i0;
        }

        if ( op & 0x4 ) == 0x4  {
            let i0 = self.popu_byte();
            self.regs.b = i0;
        }
        if ( op & 0x8 ) == 0x8  {
            let i0 = self.popu_byte();
            self.regs.dp = i0;
        }

        if ( op & 0x10 ) == 0x10  {
            let i0 = self.popu_word();
            self.regs.x = i0;
        }

        if ( op & 0x20 ) == 0x20  {
            let i0 = self.popu_word();
            self.regs.y = i0;
        }

        if ( op & 0x40 ) == 0x40  {
            let i0 = self.popu_word();
            self.regs.s = i0;
        }

        if (op & 0x80) == 0x80 {
            let i0 = self.popu_word();
            self.regs.pc = i0;
        }
    }


    fn mul<A : AddressLines>(&mut self)  {

        let i0 = self.regs.a as u32;
        let i1 = self.regs.b as u32;

        let r = u16::mul(&mut self.regs.flags, Flags::NZC.bits(), i0, i1);

        self.regs.set_d(r);

    }


////////////////////////////////////////////////////////////////////////////////

    fn leax<A : AddressLines>(&mut self)  {
        let ea = self.ea::<A>();
        self.regs.flags.set(Flags::Z, ea == 0);
        self.regs.x = ea
    }

    fn leay<A : AddressLines>(&mut self)  {
        let ea = self.ea::<A>();
        self.regs.flags.set(Flags::Z, ea == 0);
        self.regs.y = ea
    }

    fn leas<A : AddressLines>(&mut self)  {
        let ea = self.ea::<A>();
        self.regs.s = ea
    }

    fn leau<A : AddressLines>(&mut self)  {
        let ea = self.ea::<A>();
        self.regs.u = ea
    }

////////////////////////////////////////////////////////////////////////////////

    fn neg<A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::neg);
    }

    fn nega<A : AddressLines>(&mut self)  {
        self.moda::<A>(Flags::NZVC.bits(), u8::neg);
    }

    fn negb<A : AddressLines>(&mut self)  {
        self.modb::<A>(Flags::NZVC.bits(), u8::neg);
    }

    ////////////////////////////////////////////////////////////////////////////////

    fn nop<A : AddressLines>(&mut self)  {
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn rol<A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZVC.bits(), u8::rol);
    }
    fn rola<A : AddressLines>(&mut self)  {
        self.moda::<A>(Flags::NZVC.bits(), u8::rol);
    }
    fn rolb<A : AddressLines>(&mut self)  {
        self.modb::<A>(Flags::NZVC.bits(), u8::rol);
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn ror<A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZC.bits(), u8::ror);
    }
    fn rora<A : AddressLines>(&mut self)  {
        self.moda::<A>(Flags::NZC.bits(), u8::ror);
    }
    fn rorb<A : AddressLines>(&mut self)  {
        self.modb::<A>(Flags::NZC.bits(), u8::ror);
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn sbca<A : AddressLines>(&mut self)  {
        self.moda_2::<A>(Flags::NZVC.bits(), u8::sbc);
    }

    fn sbcb<A : AddressLines>(&mut self)  {
        self.modb_2::<A>(Flags::NZVC.bits(), u8::sbc);
    }
    fn suba<A : AddressLines>(&mut self)  {
        self.moda_2::<A>(Flags::NZVC.bits(), u8::sub);
    }
    fn subb<A : AddressLines>(&mut self)  {
        self.modb_2::<A>(Flags::NZVC.bits(), u8::sub);
    }

    fn tsta<A : AddressLines>(&mut self)  {
        self.moda::<A>(Flags::NZV.bits(), u8::tst);
    }

    fn tstb<A : AddressLines>(&mut self)  {
        self.modb::<A>(Flags::NZV.bits(), u8::tst);
    }

    fn tst<A : AddressLines>(&mut self)  {
        self.rwmod8::<A>( Flags::NZV.bits(), u8::tst);
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn sex<A : AddressLines>(&mut self)  {
        if self.regs.b & 0x80 == 0x80 {
            self.regs.a = 0xff;
        } else {
            self.regs.a = 0;
        }

        let d = self.regs.get_d() as u32;

        alu::nz::<u16>(&mut self.regs.flags,Flags::NZ.bits(),d);
    }


    fn swi_base<A : AddressLines>(&mut self, vec : u16, flags : Flags)  {

        macro_rules! push8 {
            ($val:expr) => (
                { let i0 = $val; self.pushs_byte(i0) })}

        macro_rules! push16 {
            ($val:expr) => (
                { let i0 = $val; self.pushs_word(i0) })}

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
        self.set_pc(pc)
    }

    fn swi<A : AddressLines>(&mut self)  {
        self.swi_base::<A>(0xfffa, Flags::E | Flags::F);
    }

    fn swi2<A : AddressLines>(&mut self)  {
        self.swi_base::<A>(0xfff4, Flags::E);
    }

    fn swi3<A : AddressLines>(&mut self)  {
        self.swi_base::<A>(0xfff2, Flags::E);
    }

    fn subd<A : AddressLines>(&mut self)  {
        let i0 = self.regs.get_d();
        let r = self.op16_2::<A>(Flags::NZVC.bits(), u16::sub, i0);
        self.regs.set_d(r);
    }

    fn jmp<A : AddressLines>(&mut self)  {
        let a = self.ea::<A>();
        self.set_pc(a)
    }

    fn rti<A : AddressLines>(&mut self)  {
        macro_rules! pop8 {
            () => { self.pops_byte() };

            ($val:expr) => ( { let i0 =  pop8!(); $val = i0 })}

        macro_rules! pop16 {
            () => { self.pops_word() };
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

        self.set_pc(pc)
    }

    fn cwai< A : AddressLines>(&mut self)  {
        panic!("cwai NO!")
    }

    fn reset< A : AddressLines>(&mut self)  {
        panic!("reset NO!")
    }

    fn sync< A : AddressLines>(&mut self)  {
        panic!("sync NO!")
    }

    fn unimplemented(&mut self) {
        // panic!("unimplemnted op code")
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

pub fn step<M: MemoryIO, C : Clock>(regs : &mut Regs, mem : &mut M, ref_clock : &Rc<RefCell<C>>) -> InstructionDecoder {

    let mut ctx = Context::new(mem,regs,ref_clock);

    macro_rules! handle_op {
        ($addr:ident, $action:ident) => ({ ctx.$action::<$addr>(); }) }

    op_table!(ctx.fetch_instruction(), { ctx.unimplemented() });

    ctx.regs.pc =  ctx.ins.next_addr;
    ctx.ins.clone()
}

pub fn step_host<M: MemoryIO, C: Clock>(host : &mut Host<M,C>) -> InstructionDecoder {

    //let mem = {host.mem() };

    //let regs = host.regs();

    //let clock = host.clock();

    //let ctx = Context::new(mem, regs, clock);

    ////
    unimplemented!();
}

//
// }}}

