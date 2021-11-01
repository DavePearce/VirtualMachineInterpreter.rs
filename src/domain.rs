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

pub const ONE_BITS : Bits = Bits{value:1};
pub const TWO_BITS : Bits = Bits{value:2};
pub const THREE_BITS : Bits = Bits{value:3};
pub const FOUR_BITS : Bits = Bits{value:4};
pub const FIVE_BITS : Bits = Bits{value:5};
pub const SIX_BITS : Bits = Bits{value:6};
pub const SEVEN_BITS : Bits = Bits{value:7};
pub const EIGHT_BITS : Bits = Bits{value:8};
pub const NINE_BITS : Bits = Bits{value:9};
pub const TEN_BITS : Bits = Bits{value:10};

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

pub const ONE_BYTE : Bytes = Bytes{value:1};
pub const TWO_BYTES : Bytes = Bytes{value:2};

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
