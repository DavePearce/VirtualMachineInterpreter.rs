use num::BigUint;
use virmin::domain::Bits;
use virmin::domain::Bytes;
use virmin::domain::Countable;
use virmin::insn::Format;
use virmin::insn::Instruction;
use virmin::machine::MicroCode;
use virmin::machine::Width::{Byte,Word};

// =====================================================
// Bits
// =====================================================   

#[test]
#[should_panic]
fn test_bits_00() {
    let _b = Bits::from(0);
}    

#[test]
fn test_bits_01() {
    let b = Bits::from(1);
    assert_eq!(b.count(),BigUint::from(2u32));
}

#[test]
fn test_bits_02() {
    let b = Bits::from(2);
    assert_eq!(b.count(),BigUint::from(4u32));
}

#[test]
fn test_bits_03() {
    let b = Bits::from(3);
    assert_eq!(b.count(),BigUint::from(8u32));
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
    assert_eq!(b.count(),BigUint::from(256u32));
}

#[test]
fn test_bytes_02() {
    let b = Bytes::from(2);
    assert_eq!(b.count(),BigUint::from(65536u32));
}

#[test]
fn test_bytes_03() {
    let b = Bytes::from(3);
    assert_eq!(b.count(),BigUint::from(16777216u32));
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
    assert_eq!(fmt.count(),BigUint::from(256u32));
}

#[test]
fn test_format_04() {
    // Check that 8 bits fits into one byte
    let width : Bytes = Bytes::from(1);	
    let opcode : Bits = Bits::from(4);
    let operand : Bits = Bits::from(2);
    let fmt = Format::new(width,"fmt",opcode,vec![operand,operand]);
    assert_eq!(fmt.count(),BigUint::from(256u32));	
}

#[test]
fn test_format_05() {
    // Check that 6 bits fit into one byte with spare
    let width : Bytes = Bytes::from(1);	
    let opcode : Bits = Bits::from(6);
    let fmt = Format::new(width,"fmt",opcode,vec![]);
    assert_eq!(fmt.count(),BigUint::from(64u32));	
}

#[test]
fn test_format_06() {
    // Check that 6 bits fits into one byte with space
    let width : Bytes = Bytes::from(1);	
    let opcode : Bits = Bits::from(2);
    let operand : Bits = Bits::from(2);
    let fmt = Format::new(width,"fmt",opcode,vec![operand,operand]);
    assert_eq!(fmt.count(),BigUint::from(64u32));	
}

// =====================================================
// Instructions
// =====================================================   

#[test]
fn test_insn_01() {
    // 4 bits opcode, no operands
    let fmt = Format::new(Bytes::from(1),"fmt",Bits::from(4),vec![]);
    // FIXME: currently a disconnect here, because the microcode is
    // concrete whereas we are in a symbolic world.
    let microcode = [MicroCode::Load(0,0,Byte)];
    let insn = Instruction::new("insn", &fmt, &microcode);
}
