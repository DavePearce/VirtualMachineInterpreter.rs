use num::BigUint;
use virmin::domain::*;
use virmin::insn::Format;
use virmin::insn::Instruction;
use virmin::insn::AbstractMicroCode::*;
use virmin::insn::Operand::*;
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
    Format::new(ONE_BYTE,"fmt",TEN_BITS, &[]);
}

#[test]
#[should_panic]
fn test_format_02() {
    // Check that 10bits does not fit into one byte
    Format::new(ONE_BYTE,"fmt",FOUR_BITS, &[THREE_BITS,THREE_BITS]);
}   

#[test]
fn test_format_03() {
    // Check that 8 bits fits into one byte
    let fmt = Format::new(ONE_BYTE,"fmt", FOUR_BITS, &[FOUR_BITS]);
    assert_eq!(fmt.count(),BigUint::from(256u32));
}

#[test]
fn test_format_04() {
    // Check that 8 bits fits into one byte
    let fmt = Format::new(ONE_BYTE,"fmt", FOUR_BITS, &[TWO_BITS, TWO_BITS]);
    assert_eq!(fmt.count(),BigUint::from(256u32));	
}

#[test]
fn test_format_05() {
    // Check that 6 bits fit into one byte with spare
    let fmt = Format::new(ONE_BYTE,"fmt",SIX_BITS, &[]);
    assert_eq!(fmt.count(),BigUint::from(64u32));	
}

#[test]
fn test_format_06() {
    // Check that 6 bits fits into one byte with space
    let fmt = Format::new(ONE_BYTE,"fmt",TWO_BITS, &[TWO_BITS,TWO_BITS]);
    assert_eq!(fmt.count(),BigUint::from(64u32));	
}

// =====================================================
// Instructions
// =====================================================   

#[test]
fn test_insn_01() {
    let fmt = Format::new(ONE_BYTE,"fmt",FOUR_BITS, &[FOUR_BITS]);
    let microcode = [Load(Var(0),0,Byte)];
    let insn = Instruction::new("insn", &fmt, &microcode);
    //
    assert!(insn.to_microcode(&[1]) == vec![MicroCode::Load(1,0,Byte)])
}

#[test]
fn test_insn_02() {
    let fmt = Format::new(ONE_BYTE,"fmt",FOUR_BITS, &[FOUR_BITS]);
    let microcode = [Load(Const(123),0,Byte)];
    let insn = Instruction::new("insn", &fmt, &microcode);
    //
    assert!(insn.to_microcode(&[1]) == vec![MicroCode::Load(123,0,Byte)])
}

#[test]
#[should_panic]
fn test_insn_03() {
    let fmt = Format::new(ONE_BYTE,"fmt",FOUR_BITS, &[]);
    let microcode = [Load(Var(0),0,Byte)];
    // Microcode expects operand, but format has none.
    let insn = Instruction::new("insn", &fmt, &microcode);
}

#[test]
#[should_panic]
fn test_insn_04() {
    let fmt = Format::new(ONE_BYTE,"fmt",FOUR_BITS, &[FOUR_BITS]);
    let microcode = [Load(Var(1),0,Byte)];
    // Microcode expects two operands, but format has one.
    let insn = Instruction::new("insn", &fmt, &microcode);
}
