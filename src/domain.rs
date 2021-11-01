use num::BigUint;

/// Used for converting a given domain into a physical count of
/// elements in that domain.  For example, the domain of 2bits would
/// convert into a count of 4 (i.e. since that is the number of
/// distinct elements in the domain).
pub trait Countable {
    /// Calculate the number of elements in the given domain.
    fn count(&self) -> BigUint;
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

impl Countable for Bits {
    fn count(&self) -> BigUint {
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

impl Countable for Bytes {
    fn count(&self) -> BigUint {
	let mut count = BigUint::from(1u32);
	//
	for i in 0 .. self.value {
	    count = count * 256u32;
	}
	//
	count
    }
}
