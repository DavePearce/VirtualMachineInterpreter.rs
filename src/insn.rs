use num::BigUint;
use crate::domain::Countable;
use crate::domain::{Bits,Bytes};
use crate::machine::MicroCode;

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
	assert!(width.count() >= r.count());
	//
	r
    }
}

impl Countable for Format {
    fn count(&self) -> BigUint {
	let mut count = BigUint::from(self.opcode.count());
	//
	for op in &self.operands {
	    count = count * op.count();
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
    semantic: &'a [MicroCode]
}

impl<'a> Instruction<'a> {
    pub fn new(mnemonic: &'a str, format: &'a Format, semantic: &'a [MicroCode]) -> Self {
	Instruction{mnemonic,format,semantic}
    }
}

// =====================================================
// Instruction Set
// =====================================================   

/// A collection of instructions.
pub struct InstructionSet<'a> {
    insns : &'a [Instruction<'a>]
}

impl<'a> InstructionSet<'a> {
    pub fn new(insns : &'a [Instruction<'a>]) -> Self {
	InstructionSet{insns}
    }
}

