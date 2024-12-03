#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::pedantic)]

//! "Yet another endianness library"
//!
//! This library is intended for uses where the data is not deserialized but already in ROM/RAM.
//!
//! The library is `no_std`, has no features and no dependencies.
//!
//! The conversion is made explicit by calling `new`, `get` and `set`.
//!
//! Operations which do not depend on the byte order can be performed on integers:
//! eq, and, or, xor, not.
//!
//! But please note that a operation with a constant may be more efficient with `get`ting the value
//! instead of wrapping the constant with `new`, though depending on the constant and cpu.
//!
//! There are several similar libraries, but they all differ in a some points.
//!
//! Some alternatives:
//! - [bswap](https://crates.io/crates/bswap)
//! - [byteorder](https://crates.io/crates/byteorder)
//! - [endian](https://crates.io/crates/endian)
//! - [simple_endian](https://crates.io/crates/simple_endian)
//!
//! # Examples
//!
//! ```rust
//! # use yael::u32be;
//! #[repr(C)]
//! struct Chunk {
//!     size: u32be,
//!     checksum: u32be,
//! }
//! let chunk = Chunk { size: u32be::new(55), checksum: u32be::new(0x52f0e743) };
//! let chunk_size = chunk.size.get();
//! ```
//!
//! ```rust
//! # use yael::u16be;
//! #[repr(C)]
//! struct Data {
//!     mode: u16be,
//!     flags: u16be,
//! }
//!
//! fn fix_flags1(data: &mut Data){
//!     // compare with a constant big-endian number, no conversion performed, can also be written as `data.mode.is_zero()`
//!     if data.mode == u16be::new(0) {
//!         // bit operations: also no conversion performed
//!         data.flags |= u16be::new(1);
//!     }
//! }
//!
//! fn fix_flags2(data: &mut Data, flags_on_error: u16){
//!     // while these get and set do perform conversion
//!     if data.mode.get() > 80 {
//!         data.flags.set(flags_on_error);
//!     }
//! }
//! ```
//!
//! The `#[repr(C)]` is not required by the examples, but probably in many use cases.
//!
//! # Floating Point
//!
//! The floating point types do not support bit operations because a float does not either.
//!
//! Comparison is not supported because it would differ from how a float behaves.
//!
//! (`+0 == -0` float: equal, bits: different; `nan == nan` float: different, bits: equal)

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

macro_rules! create_int {
    ($name:ident, $type:ident, $from:ident, $to:ident, $bytes:expr, $doc:literal) => {
        #[doc = $doc]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $name($type);
        impl $name {
            /// Convert a native byte order integer into an endianness specific integer.
            #[inline]
            #[must_use]
            pub const fn new(value: $type) -> $name {
                $name(value.$to())
            }

            /// Convert the endianness specific integer into native byte order.
            #[inline]
            #[must_use]
            pub const fn get(&self) -> $type {
                $type::$from(self.0)
            }

            /// Convert a native byte order integer and store it in this endianness specific integer.
            #[inline]
            pub fn set(&mut self, value: $type) {
                self.0 = value.$to();
            }

            /// Check if the value is zero.
            #[inline]
            #[must_use]
            pub const fn is_zero(&self) -> bool {
                // Safety: big and little endian encode a 0 the same way
                self.0 == 0
            }

            /// Creates an integer value from its representation as a byte array in big endian.
            #[inline]
            #[must_use]
            pub const fn from_be_bytes(bytes: [u8; $bytes]) -> $name {
                $name($type::from_be_bytes(bytes).$to())
            }
        }

        impl BitAnd for $name {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl BitOr for $name {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl BitXor for $name {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl Not for $name {
            type Output = Self;
            #[inline]
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
    };
}

create_int!(
    u16le,
    u16,
    from_le,
    to_le,
    2,
    "The 16-bit little endian unsigned integer type."
);
create_int!(
    u32le,
    u32,
    from_le,
    to_le,
    4,
    "The 32-bit little endian unsigned integer type."
);
create_int!(
    u64le,
    u64,
    from_le,
    to_le,
    8,
    "The 64-bit little endian unsigned integer type."
);
create_int!(
    u128le,
    u128,
    from_le,
    to_le,
    16,
    "The 128-bit little endian unsigned integer type."
);

create_int!(
    u16be,
    u16,
    from_be,
    to_be,
    2,
    "The 16-bit big endian unsigned integer type."
);
create_int!(
    u32be,
    u32,
    from_be,
    to_be,
    4,
    "The 32-bit big endian unsigned integer type."
);
create_int!(
    u64be,
    u64,
    from_be,
    to_be,
    8,
    "The 64-bit big endian unsigned integer type."
);
create_int!(
    u128be,
    u128,
    from_be,
    to_be,
    16,
    "The 128-bit big endian unsigned integer type."
);

create_int!(
    i16le,
    i16,
    from_le,
    to_le,
    2,
    "The 16-bit little endian signed integer type."
);
create_int!(
    i32le,
    i32,
    from_le,
    to_le,
    4,
    "The 32-bit little endian signed integer type."
);
create_int!(
    i64le,
    i64,
    from_le,
    to_le,
    8,
    "The 64-bit little endian signed integer type."
);
create_int!(
    i128le,
    i128,
    from_le,
    to_le,
    16,
    "The 128-bit little endian signed integer type."
);

create_int!(
    i16be,
    i16,
    from_be,
    to_be,
    2,
    "The 16-bit big endian signed integer type."
);
create_int!(
    i32be,
    i32,
    from_be,
    to_be,
    4,
    "The 32-bit big endian signed integer type."
);
create_int!(
    i64be,
    i64,
    from_be,
    to_be,
    8,
    "The 64-bit big endian signed integer type."
);
create_int!(
    i128be,
    i128,
    from_be,
    to_be,
    16,
    "The 128-bit big endian signed integer type."
);

macro_rules! create_float {
    ($name:ident, $float_type:ident, $int_type:ident, $from:ident, $to:ident, $bytes:expr, $doc:literal) => {
        #[doc = $doc]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct $name($int_type);
        impl $name {
            /// Convert the endianness specific float into native byte order.
            #[inline]
            #[must_use]
            pub const fn get(&self) -> $float_type {
                $float_type::from_bits($int_type::$from(self.0))
            }

            /// Convert a native byte order float and store it in this endianness specific float.
            #[inline]
            pub fn set(&mut self, value: $float_type) {
                self.0 = value.to_bits().$to();
            }

            /// Convert a native byte order float into an endianness specific float.
            #[inline]
            #[must_use]
            pub const fn new(value: $float_type) -> $name {
                Self($int_type::$from(value.to_bits()))
            }

            /// Creates an float value from its representation as a byte array in big endian.
            #[inline]
            #[must_use]
            pub const fn from_be_bytes(bytes: [u8; $bytes]) -> $name {
                $name($int_type::from_be_bytes(bytes).$to())
            }
        }
    };
}

create_float!(
    f32be,
    f32,
    u32,
    from_be,
    to_be,
    4,
    "The 32-bit big endian floating-point type."
);
create_float!(
    f64be,
    f64,
    u64,
    from_be,
    to_be,
    8,
    "The 64-bit big endian floating-point type."
);
create_float!(
    f32le,
    f32,
    u32,
    from_le,
    to_le,
    4,
    "The 32-bit little endian floating-point type."
);
create_float!(
    f64le,
    f64,
    u64,
    from_le,
    to_le,
    8,
    "The 64-bit little endian floating-point type."
);
