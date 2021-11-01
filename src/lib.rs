use num::BigUint;

/// Used for converting a given domain into a physical count of
/// elements in that domain.  For example, the domain of 2bits would
/// convert into a count of 4 (i.e. since that is the number of
/// distinct elements in the domain).
trait DomainSize {
    /// Calculate the number of elements in the given domain.
    fn to_domsize(&self) -> BigUint;
}

// ================================================================
// Bits
// ================================================================

#[derive(Clone,Copy,PartialEq)]
struct Bits {
    // INVARIANT: value > 0
    value : u8,
}

impl From<u8> for Bits {
    fn from(value:u8) -> Self {
	assert!(value != 0);
	Bits{value}
    }
}

impl DomainSize for Bits {
    fn to_domsize(&self) -> BigUint {
	let mut count = BigUint::from(1u32);
	//
	for i in 0 .. self.value {
	    count = count * 2u32;
	}
	//
	count
    }
}

// ================================================================
// Bytes
// ================================================================

#[derive(Clone,Copy,PartialEq)]
struct Bytes {
    // INVARIANT: value > 0    
    value : u8,
}

impl From<u8> for Bytes {
    fn from(value:u8) -> Self {
	assert!(value != 0);
	Bytes{value}
    }
}

impl DomainSize for Bytes {
    fn to_domsize(&self) -> BigUint {
	let mut count = BigUint::from(1u32);
	//
	for i in 0 .. self.value {
	    count = count * 256u32;
	}
	//
	count
    }
}

// ================================================================
// Format
// ================================================================

/// Defines a format for a class of related instructions.  For
/// example, some instructions might not take any operands.  Others
/// might take, say, three register operands, etc.  The following
/// illustrates:
///
/// ```text
///    +-+-+-+-+-+-+-+-+
///    |7   5|    2|1 0|
///    +-+-+-+-+-+-+-+-+
///    | #2  | #1  |Op |
///    +-+-+-+-+-+-+-+-+
/// ```
/// Here, we see one possible layout for an instruction class which
/// includes two three-bit operands, and a two bit opcode.  This means
/// we can have at most four instructions in this class, and each
/// operand can take on eight distinct values.
#[derive(PartialEq)]
struct Format {
    /// Determines the overall width (in bytes) of an instruction in
    /// this class.  Generally speaking, virtual machines normally
    /// have all instructions of the same width (e.g. 32bits) and, in
    /// such case, `width` will be the same for all formats.  However,
    /// it is possible that a virtual machine has different sized
    /// instructions (e.g. 16bit instructions, and 32bit "double"
    /// instructions).
    width: Bytes,
    /// Human-readable label for this format
    label: String,
    /// Determine the number of distinct instructions in this class.
    opcode : Bits,
    /// Determine the number and size of operands for all instructions
    /// in this class.
    operands: Vec<Bits>
}

impl Format {
    pub fn new(width:Bytes, label: &str, opcode: Bits, operands: Vec<Bits>) -> Format {	
	let r = Format{width,label:label.to_string(),opcode,operands};
	// Sanity check there is enough space
	assert!(width.to_domsize() >= r.to_domsize());
	//
	r
    }
}

impl DomainSize for Format {
    fn to_domsize(&self) -> BigUint {
	let mut count = BigUint::from(self.opcode.to_domsize());
	//
	for op in &self.operands {
	    count = count * op.to_domsize();
	}
	//
	count
    }
}

// =====================================================
// (Random Access) Memory
// =====================================================

/// Describes a fixed-size array of bytes.
struct Memory<'a> {
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
enum Width {
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
enum Sign {
    // Indicates an unsigned operation
    Unsigned,
    // Indicates a signed operation
    Signed
}

/// Microcode is used to define the semantics of virtual machine
/// instructions.  This means, for example, they can be executed using
/// a "virtual machine interpreter".
#[derive(Clone,Copy,PartialEq)]
enum MachineCode {
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

struct MachineState<'a> {
    /// Program counter.  This determines where in the instruction
    /// memory the machine is currently executing.  The program
    /// counter always points to the *next* instruction to be
    /// executed.
    pc: usize,
    /// Available memory
    data: Memory<'a>,
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


// =====================================================
// Instruction
// =====================================================

struct Instruction<'a> {
    /// Mnemonic for referring to the instruction.  Every instruction
    /// should have a unique mnemonic.
    mnemonic: &'a str,
    /// Format associated with this instruction.
    format: &'a Format,
    /// Machine semantics associated with instruction.
    semantic: &'a [MachineCode]
}

impl<'a> Instruction<'a> {
    pub fn new(mnemonic: &'a str, format: &'a Format, semantic: &'a [MachineCode]) -> Self {
	Instruction{mnemonic,format,semantic}
    }

    /// Apply a given instruction to a given machine state.
    pub fn execute(&self, state: &mut MachineState) {
	for insn in self.semantic {
	    state.execute(*insn);
	}
    }
}

// =====================================================
// Instruction Set
// =====================================================   

/// A collection of instructions.
struct InstructionSet<'a> {
    insns : Vec<Instruction<'a>>
}

// =====================================================
// Tests
// =====================================================   

#[cfg(test)]
mod tests {
    use num::BigUint;
    use crate::Bits;
    use crate::Bytes;    
    use crate::Format;
    use crate::DomainSize;
    use crate::MachineCode;
    use crate::MachineState;
    use crate::Memory;
    use crate::Width::*;
    use crate::Sign::*;
    
    // =====================================================
    // Bits
    // =====================================================   

    #[test]
    #[should_panic]
    fn test_bites_00() {
	let _b = Bits::from(0);
    }    

    #[test]
    fn test_bits_01() {
	let b = Bits::from(1);
	assert_eq!(b.to_domsize(),BigUint::from(2u32));
    }

    #[test]
    fn test_bits_02() {
	let b = Bits::from(2);
	assert_eq!(b.to_domsize(),BigUint::from(4u32));
    }

    #[test]
    fn test_bits_03() {
	let b = Bits::from(3);
	assert_eq!(b.to_domsize(),BigUint::from(8u32));
    }

    // =====================================================
    // Bytes
    // =====================================================   

    #[test]
    #[should_panic]
    fn test_bytes_00() {
	let _b = Bytes::from(0);
    }    
    
    #[test]
    fn test_bytes_01() {
	let b = Bytes::from(1);
	assert_eq!(b.to_domsize(),BigUint::from(256u32));
    }

    #[test]
    fn test_bytes_02() {
	let b = Bytes::from(2);
	assert_eq!(b.to_domsize(),BigUint::from(65536u32));
    }

    #[test]
    fn test_bytes_03() {
	let b = Bytes::from(3);
	assert_eq!(b.to_domsize(),BigUint::from(16777216u32));
    }

    // =====================================================
    // Formats
    // =====================================================   

    #[test]
    #[should_panic]
    fn test_format_01() {
	// Check that 10bits does not fit into one byte
	let width = Bytes::from(1);
	let opcode = Bits::from(10);
        Format::new(width,"fmt",opcode,vec![]);
    }

    #[test]
    #[should_panic]
    fn test_format_02() {
	// Check that 10bits does not fit into one byte
	let width = Bytes::from(1);
	let opcode = Bits::from(4);
	let operand = Bits::from(3);
        Format::new(width,"fmt",opcode,vec![operand,operand]);
    }   
    
    #[test]
    fn test_format_03() {
	// Check that 8 bits fits into one byte
	let width = Bytes::from(1);
	let opcode = Bits::from(4);
	let operand = Bits::from(4);
        let fmt = Format::new(width,"fmt",opcode,vec![operand]);
	assert_eq!(fmt.to_domsize(),BigUint::from(256u32));
    }

    #[test]
    fn test_format_04() {
	// Check that 8 bits fits into one byte
	let width : Bytes = Bytes::from(1);	
	let opcode : Bits = Bits::from(4);
	let operand : Bits = Bits::from(2);
        let fmt = Format::new(width,"fmt",opcode,vec![operand,operand]);
	assert_eq!(fmt.to_domsize(),BigUint::from(256u32));	
    }

    #[test]
    fn test_format_05() {
	// Check that 6 bits fit into one byte with spare
	let width : Bytes = Bytes::from(1);	
	let opcode : Bits = Bits::from(6);
        let fmt = Format::new(width,"fmt",opcode,vec![]);
	assert_eq!(fmt.to_domsize(),BigUint::from(64u32));	
    }
    
    #[test]
    fn test_format_06() {
	// Check that 6 bits fits into one byte with space
	let width : Bytes = Bytes::from(1);	
	let opcode : Bits = Bits::from(2);
	let operand : Bits = Bits::from(2);
        let fmt = Format::new(width,"fmt",opcode,vec![operand,operand]);
	assert_eq!(fmt.to_domsize(),BigUint::from(64u32));	
    }

    // =====================================================
    // Machine Codes (Add)
    // =====================================================   

    #[test]
    fn test_add_01() {
	let mut bytes : [u8;2] = [1,2];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Add(0,1,Byte));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[3,2]);
    }

    #[test]
    fn test_add_02() {
	let mut bytes : [u8;2] = [255,2];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Add(0,1,Byte));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[1,2]);
    }    

    #[test]
    fn test_add_03() {
	let mut bytes : [u8;4] = [1,2, 2,2];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Add(0,1,Word));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[3,4,2,2]);
    }    
    
    // =====================================================
    // Machine Codes (Copy)
    // =====================================================   
    
    #[test]
    fn test_copy_01() {
	let mut bytes : [u8;2] = [1,2];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Copy(0,1,Byte));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[2,2]);
    }

    #[test]
    fn test_copy_02() {
	let mut bytes : [u8;4] = [1,1,2,3];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Copy(0,1,Word));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[1,2,2,3]);
    }

    #[test]
    fn test_copy_03() {
	let mut bytes : [u8;4] = [1,1,2,3];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Copy(0,2,Word));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[2,3,2,3]);
    }

    // =====================================================
    // Machine Codes (Load)
    // =====================================================   
    
    #[test]
    fn test_load_01() {
	let mut bytes : [u8;2] = [0,2];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Load(0,1,Byte));
	// Check what happened
	assert_eq!(state.pc,1);	
	assert_eq!(bytes,[1,2]);
    }

    #[test]
    fn test_load_02() {
	let mut bytes : [u8;4] = [0,1,2,3];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Load(0,1,Word));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[1,0,2,3]);
    }

    #[test]
    fn test_load_03() {
	let mut bytes : [u8;4] = [0,0,2,3];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Load(0,257,Word));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[1,1,2,3]);	
    }

    #[test]
    fn test_load_04() {
	let mut bytes : [u8;4] = [0,0,1,1];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Load(0,257,DoubleWord));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[1,1,0,0]);
    }

    #[test]
    fn test_load_05() {
	let mut bytes : [u8;8] = [2,3,4,5,6,7,8,9];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Load(0,65537,DoubleWord));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[1,0,1,0,6,7,8,9]);
    }

    // =====================================================
    // Machine Codes (Goto)
    // =====================================================   
    
    #[test]
    fn test_goto_01() {
	let mut bytes : [u8;2] = [1,2];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Goto(2));
	// Check what happened
	assert_eq!(state.pc,2);
	assert_eq!(bytes,[1,2]);
    }

    #[test]
    fn test_goto_02() {
	let mut bytes : [u8;2] = [1,2];
	let mut state = MachineState::new(0,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Goto(0));
	// Check what happened
	assert_eq!(state.pc,0);
	assert_eq!(bytes,[1,2]);
    }

    // =====================================================
    // Machine Codes (Jump)
    // =====================================================       
    
    #[test]
    fn test_jump_01() {
	let mut bytes : [u8;2] = [1,2];
	let mut state = MachineState::new(1,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Jump(2));
	// Check what happened
	assert_eq!(state.pc,3);
	assert_eq!(bytes,[1,2]);
    }

    #[test]
    fn test_jump_02() {
	let mut bytes : [u8;2] = [1,2];
	let mut state = MachineState::new(2,&mut bytes);
	// Execute an instruction
	state.execute(MachineCode::Jump(-1));
	// Check what happened
	assert_eq!(state.pc,1);
	assert_eq!(bytes,[1,2]);
    }
   
}
