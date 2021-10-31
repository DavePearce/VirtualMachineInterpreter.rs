use num::BigUint;
// instruction format

// --> opcodes / operands
//
// operand
// --> immediate
// --> register?
// --> ???

/// Used for converting a given domain into a physical count of
/// elements in that domain.  For example, the domain of 2bits would
/// convert into a count of 4 (i.e. since that is the number of
/// distinct elements in the domain).
trait DomainSize {
    /// Calculate the number of elements in the given domain.
    fn to_domain_size(&self) -> BigUint;
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
    fn to_domain_size(&self) -> BigUint {
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
    value : u8,
}

impl From<u8> for Bytes {
    fn from(value:u8) -> Self {
	Bytes{value}
    }
}

impl DomainSize for Bytes {
    fn to_domain_size(&self) -> BigUint {
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
struct Format {
    /// Determines the overall width (in bytes) of an instruction in
    /// this class.  Generally speaking, virtual machines normally
    /// have all instructions of the same width (e.g. 32bits) and, in
    /// such case, `width` will be the same for all formats.  However,
    /// it is possible that a virtual machine has different sized
    /// instructions (e.g. 16bit instructions, and 32bit "double"
    /// instructions).
    width: Bytes,
    /// Determine the number of distinct instructions in this class.
    opcode : Bits,
    /// Determine the number and size of operands for all instructions
    /// in this class.
    operands: Vec<Bits>
}

impl Format {
    pub fn new(width:Bytes, opcode: Bits, operands: Vec<Bits>) -> Format {	
	Format{width,opcode,operands}
    }
}

// instruction semantics

// machine state

#[cfg(test)]
mod tests {
    use num::BigUint;
    use crate::Bits;
    use crate::Bytes;    
    use crate::Format;
    use crate::DomainSize;

    // =====================================================
    // Bits
    // =====================================================   
    
    #[test]
    fn test_bits_01() {
	let b = Bits::from(1);
	assert_eq!(b.to_domain_size(),BigUint::from(2u32));
    }

    #[test]
    fn test_bits_02() {
	let b = Bits::from(2);
	assert_eq!(b.to_domain_size(),BigUint::from(4u32));
    }

    #[test]
    fn test_bits_03() {
	let b = Bits::from(3);
	assert_eq!(b.to_domain_size(),BigUint::from(8u32));
    }

    // =====================================================
    // Bytes
    // =====================================================   
    
    #[test]
    fn test_bytes_01() {
	let b = Bytes::from(1);
	assert_eq!(b.to_domain_size(),BigUint::from(256u32));
    }

    #[test]
    fn test_bytes_02() {
	let b = Bytes::from(2);
	assert_eq!(b.to_domain_size(),BigUint::from(65536u32));
    }

    #[test]
    fn test_bytes_03() {
	let b = Bytes::from(3);
	assert_eq!(b.to_domain_size(),BigUint::from(16777216u32));
    }

    // =====================================================
    // Formats
    // =====================================================   
    
    #[test]
    fn test() {
	let op4 : Bits = Bits::from(4);
	let b1 : Bytes = Bytes::from(1);	
        let fmt_rr = Format::new(b1,op4,vec![op4]);
    }
}
