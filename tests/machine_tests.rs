use virmin::machine::MicroCode;
use virmin::machine::State;
use virmin::machine::Memory;
use virmin::machine::Width::{Byte,Word,DoubleWord,QuadWord};
use virmin::machine::Sign::*;

// =====================================================
// MicroCode (Add)
// =====================================================   

#[test]
fn test_add_01() {
    let mut bytes : [u8;2] = [1,2];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Add(0,1,Byte));
    // Check what happened
    assert_eq!(state.pc,1);
    assert_eq!(bytes,[3,2]);
}

#[test]
fn test_add_02() {
    let mut bytes : [u8;2] = [255,2];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Add(0,1,Byte));
    // Check what happened
    assert_eq!(state.pc,1);
    assert_eq!(bytes,[1,2]);
}    

#[test]
fn test_add_03() {
    let mut bytes : [u8;4] = [1,2, 2,2];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Add(0,1,Word));
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
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Copy(0,1,Byte));
    // Check what happened
    assert_eq!(state.pc,1);
    assert_eq!(bytes,[2,2]);
}

#[test]
fn test_copy_02() {
    let mut bytes : [u8;4] = [1,1,2,3];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Copy(0,1,Word));
    // Check what happened
    assert_eq!(state.pc,1);
    assert_eq!(bytes,[1,2,2,3]);
}

#[test]
fn test_copy_03() {
    let mut bytes : [u8;4] = [1,1,2,3];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Copy(0,2,Word));
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
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Load(0,1,Byte));
    // Check what happened
    assert_eq!(state.pc,1);	
    assert_eq!(bytes,[1,2]);
}

#[test]
fn test_load_02() {
    let mut bytes : [u8;4] = [0,1,2,3];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Load(0,1,Word));
    // Check what happened
    assert_eq!(state.pc,1);
    assert_eq!(bytes,[1,0,2,3]);
}

#[test]
fn test_load_03() {
    let mut bytes : [u8;4] = [0,0,2,3];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Load(0,257,Word));
    // Check what happened
    assert_eq!(state.pc,1);
    assert_eq!(bytes,[1,1,2,3]);	
}

#[test]
fn test_load_04() {
    let mut bytes : [u8;4] = [0,0,1,1];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Load(0,257,DoubleWord));
    // Check what happened
    assert_eq!(state.pc,1);
    assert_eq!(bytes,[1,1,0,0]);
}

#[test]
fn test_load_05() {
    let mut bytes : [u8;8] = [2,3,4,5,6,7,8,9];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Load(0,65537,DoubleWord));
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
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Goto(2));
    // Check what happened
    assert_eq!(state.pc,2);
    assert_eq!(bytes,[1,2]);
}

#[test]
fn test_goto_02() {
    let mut bytes : [u8;2] = [1,2];
    let mut state = State::new(0,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Goto(0));
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
    let mut state = State::new(1,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Jump(2));
    // Check what happened
    assert_eq!(state.pc,3);
    assert_eq!(bytes,[1,2]);
}

#[test]
fn test_jump_02() {
    let mut bytes : [u8;2] = [1,2];
    let mut state = State::new(2,&mut bytes);
    // Execute an instruction
    state.execute(MicroCode::Jump(-1));
    // Check what happened
    assert_eq!(state.pc,1);
    assert_eq!(bytes,[1,2]);
}
