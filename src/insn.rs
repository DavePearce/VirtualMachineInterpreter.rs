use num::BigUint;
use crate::machine::MachineCode;
use crate::machine::MachineState;

/// Used for converting a given domain into a physical count of
/// elements in that domain.  For example, the domain of 2bits would
/// convert into a count of 4 (i.e. since that is the number of
/// distinct elements in the domain).
pub trait DomainSize {
    /// Calculate the number of elements in the given domain.
    fn to_domsize(&self) -> BigUint;
}

// ================================================================
// Bits
// ================================================================

#[derive(Clone,Copy,PartialEq)]
pub struct Bits {
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
pub struct Bytes {
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
pub struct Format {
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
// Instruction
// =====================================================

pub struct Instruction<'a> {
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
pub struct InstructionSet<'a> {
    insns : Vec<Instruction<'a>>
}

impl<'a> InstructionSet<'a> {
    
}

