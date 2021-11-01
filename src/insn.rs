use std::cmp;
use num::BigUint;
use crate::domain::Countable;
use crate::domain::{Bits,Bytes};
use crate::machine::Width;
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
    pub fn new(width:Bytes, label: &str, opcode: Bits, operands: &[Bits]) -> Format {	
	let r = Format{width,label:label.to_string(),opcode,operands:operands.to_vec()};
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
// Abstract Microcode
// =====================================================

/// Represents an abstract microcode instruction.  This is
/// (effectively) a template for constructing a concrete microcode
/// instruction from a concrete instantiation of an instruction
/// (i.e. where all operands have known values).
pub enum AbstractMicroCode {
    /// X := Y (w bits)    
    Copy(Operand,Operand,Width),
    /// pc := I
    Goto(Operand),    
    /// pc := pc + I
    Jump(Operand),
    /// X := i
    Load(Operand,u64,Width)	
}

impl AbstractMicroCode {
    /// Determine how many operands this microcode requires.  This is
    /// necessary to sanity check that, for a given instruction
    /// format, this microcode instruction makes sense.
    pub fn arity(&self) -> usize {
	match &self {
	    AbstractMicroCode::Copy(x,y,w) => {
		cmp::max(x.arity(),y.arity())
	    }
	    AbstractMicroCode::Load(x,i,w) => {
		x.arity()
	    }
	    _ => {
		todo!("implement more instructions")
	    }
	}
    }
    /// Given a set of concrete operands, reduce this abstract
    /// microcode instruction into a concrete microcode instruction.    
    pub fn to_microcode(&self, operands: &[usize]) -> MicroCode {
	match &self {
	    AbstractMicroCode::Copy(x,y,w) => {
		let l = x.as_usize(operands);
		let r = y.as_usize(operands);
		MicroCode::Copy(l,r,*w)
	    }
	    AbstractMicroCode::Load(x,i,w) => {
		let l = x.as_usize(operands);
		MicroCode::Load(l,*i,*w)
	    }
	    _ => {
		todo!("implement more instructions")
	    }
	}
    }
}

/// Represents an arbitrary expression over one or more instruction
/// operands.  For each instruction instantiation, an operand
/// expression can be evaluated to a constant.
pub enum Operand {
    /// A constant value which can be used in various ways.  For
    /// example, it can be used to identify a fixed location in the
    /// underlying machine; or, it could be used as part of a more
    /// complex operand expression.
    Const(usize),
    /// An operand value read from the instantiated instruction.
    Var(usize)
}

impl Operand {
    /// Determine the arity of an operand expression.  That is, how
    /// many operands are needed for it to evaluate.
    pub fn arity(&self) -> usize {
	match &self {
	    Operand::Const(i) => {
		0
	    }
	    Operand::Var(v) => {
		v + 1
	    }
	}
    }

    pub fn as_usize(&self, operands: &[usize]) -> usize {
	match &self {
	    Operand::Const(i) => {
		*i
	    }
	    Operand::Var(v) => {
		operands[*v]
	    }
	}
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
    semantic: &'a [AbstractMicroCode]
}

impl<'a> Instruction<'a> {
    pub fn new(mnemonic: &'a str, format: &'a Format, semantic: &'a [AbstractMicroCode]) -> Self {
	for code in semantic {
	    assert!(code.arity() <= format.operands.len());
	}
	Instruction{mnemonic,format,semantic}
    }

    pub fn to_microcode(&self, operands: &[usize]) -> Vec<MicroCode> {
	let mut microcode = Vec::new();
	for c in self.semantic {
	    microcode.push(c.to_microcode(operands));
	}
	microcode
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

