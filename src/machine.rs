use num::BigUint;

// =====================================================
// (Random Access) Memory
// =====================================================

/// Describes a fixed-size array of bytes.
pub struct Memory<'a> {
    contents: &'a mut [u8]
}

impl<'a> Memory<'a> {
    pub fn new(contents: &'a mut [u8]) -> Self {
	Memory{contents}
    }
    pub fn read_u8(&self, address : usize) -> u8 {
	self.contents[address]
    }
    pub fn read_u16(&self, address : usize) -> u16 {
	let b0 = self.contents[address];
	let b1 = self.contents[address+1];	
	return u16::from_le_bytes([b0,b1]);
    }
    pub fn read_u32(&self, address : usize) -> u32 {
	let b0 = self.contents[address+0];
	let b1 = self.contents[address+1];
	let b2 = self.contents[address+2];
	let b3 = self.contents[address+3];
	return u32::from_le_bytes([b0,b1,b2,b3]);
    }
    pub fn read_u64(&self, address : usize) -> u64 {
	let b0 = self.contents[address+0];
	let b1 = self.contents[address+1];
	let b2 = self.contents[address+2];
	let b3 = self.contents[address+3];
	let b4 = self.contents[address+4];
	let b5 = self.contents[address+5];
	let b6 = self.contents[address+6];
	let b7 = self.contents[address+7];
	return u64::from_le_bytes([b0,b1,b2,b3,b4,b5,b6,b7]);
    }
    pub fn write_u8(&mut self, address : usize, value: u8) {
	self.contents[address] = value; 
    }
    pub fn write_u16(&mut self, address : usize, value: u16) {
	let bytes = value.to_le_bytes();
	self.contents[address+0] = bytes[0];
	self.contents[address+1] = bytes[1];
    }
    pub fn write_u32(&mut self, address : usize, value: u32) {
	let bytes = value.to_le_bytes();
	self.contents[address+0] = bytes[0];
	self.contents[address+1] = bytes[1];
	self.contents[address+2] = bytes[2];
	self.contents[address+3] = bytes[3];	
    }
    pub fn write_u64(&mut self, address : usize, value: u64) {
	let bytes = value.to_le_bytes();
	self.contents[address+0] = bytes[0];
	self.contents[address+1] = bytes[1];
	self.contents[address+2] = bytes[2];
	self.contents[address+3] = bytes[3];
	self.contents[address+4] = bytes[4];
	self.contents[address+5] = bytes[5];
	self.contents[address+6] = bytes[6];
	self.contents[address+7] = bytes[7];	
    }
}

// =====================================================
// Machine Codes
// =====================================================

#[derive(Clone,Copy,PartialEq)]
pub enum Width {
    /// 8 bits
    Byte,
    /// 16 bits    
    Word,
    /// 32 bits    
    DoubleWord,
    /// 64 bits    
    QuadWord	    
}

#[derive(Clone,Copy,PartialEq)]
pub enum Sign {
    // Indicates an unsigned operation
    Unsigned,
    // Indicates a signed operation
    Signed
}

/// Microcode is used to define the semantics of virtual machine
/// instructions.  This means, for example, they can be executed using
/// a "virtual machine interpreter".
#[derive(Clone,Copy,PartialEq)]
pub enum MachineCode {
    /// x := x + y (w bits signed or unsigned)
    Add(usize,usize,Width),    
    /// x := y (w bits)
    Copy(usize,usize,Width),
    /// pc := i
    Goto(usize),    
    /// pc := pc + i
    Jump(isize),
    /// x := i
    Load(usize,u64,Width),
}

// =====================================================
// Machine State
// =====================================================

pub struct MachineState<'a> {
    /// Program counter.  This determines where in the instruction
    /// memory the machine is currently executing.  The program
    /// counter always points to the *next* instruction to be
    /// executed.
    pub pc: usize,
    /// Available memory
    pub data: Memory<'a>,
}

impl<'a> MachineState<'a> {
    pub fn new(pc: usize, bytes: &'a mut [u8]) -> Self {
	MachineState{pc,data: Memory::new(bytes)}
    }
    pub fn execute(&mut self, insn: MachineCode) {
	match insn {
	    MachineCode::Add(x,y,Width::Byte) => {
		let v = self.data.read_u8(x);
		let w = self.data.read_u8(y);
		let r = v.wrapping_add(w);
		// Note, must allow wrap around semantics so that
		// signed arithmetic works as expected.
		self.data.write_u8(x,r);
		self.pc += 1;
	    }
	    MachineCode::Add(x,y,Width::Word) => {
		let v = self.data.read_u16(x);
		let w = self.data.read_u16(y);
		let r = v.wrapping_add(w);
		// Note, must allow wrap around semantics so that
		// signed arithmetic works as expected.
		self.data.write_u16(x,r);
		self.pc += 1;
	    }
	    MachineCode::Add(x,y,Width::DoubleWord) => {
		let v = self.data.read_u32(x);
		let w = self.data.read_u32(y);
		let r = v.wrapping_add(w);
		// Note, must allow wrap around semantics so that
		// signed arithmetic works as expected.
		self.data.write_u32(x,r);
		self.pc += 1;
	    }
	    MachineCode::Add(x,y,Width::QuadWord) => {
		let v = self.data.read_u64(x);
		let w = self.data.read_u64(y);
		let r = v.wrapping_add(w);
		// Note, must allow wrap around semantics so that
		// signed arithmetic works as expected.
		self.data.write_u64(x,r);
		self.pc += 1;
	    }
	    MachineCode::Copy(x,y,Width::Byte) => {
		let v = self.data.read_u8(y);
		self.data.write_u8(x,v);
		self.pc += 1;
	    }
	    MachineCode::Copy(x,y,Width::Word) => {
		let v = self.data.read_u16(y);
		self.data.write_u16(x,v);
		self.pc += 1;
	    }
	    MachineCode::Copy(x,y,Width::DoubleWord) => {
		let v = self.data.read_u32(y);
		self.data.write_u32(x,v);
		self.pc += 1;
	    }
	    MachineCode::Copy(x,y,Width::QuadWord) => {
		let v = self.data.read_u64(y);
		self.data.write_u64(x,v);
		self.pc += 1;
	    }
	    MachineCode::Goto(i) => {
		self.pc = i;
	    }
	    MachineCode::Jump(i) => {
		if i < 0 {
		    self.pc -= -i as usize;
		} else {
		    self.pc += i as usize;
		}
	    }	    
	    MachineCode::Load(x,i,Width::Byte) => {
		self.data.write_u8(x,i.try_into().unwrap());
		self.pc += 1;
	    }
	    MachineCode::Load(x,i,Width::Word) => {
		self.data.write_u16(x,i.try_into().unwrap());
		self.pc += 1;
	    }
	    MachineCode::Load(x,i,Width::DoubleWord) => {
		self.data.write_u32(x,i.try_into().unwrap());
		self.pc += 1;
	    }
	    MachineCode::Load(x,i,Width::QuadWord) => {
		self.data.write_u64(x,i);
		self.pc += 1;
	    }
	    _ => {
		todo!("Implement more instructions")
	    }
	}
    }
}
