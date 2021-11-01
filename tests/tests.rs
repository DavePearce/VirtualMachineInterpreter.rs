use num::BigUint;
use virmin::insn::Bits;
use virmin::insn::Bytes;    
use virmin::insn::Format;
use virmin::insn::DomainSize;
use virmin::machine::MachineCode;
use virmin::machine::MachineState;
use virmin::machine::Memory;
use virmin::machine::Width::{Byte,Word,DoubleWord,QuadWord};
use virmin::machine::Sign::*;

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
// MicroCode (Add)
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
// MicroCode (Copy)
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
// MicroCode (Load)
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
// MicroCode (Goto)
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
// MicroCode (Jump)
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
