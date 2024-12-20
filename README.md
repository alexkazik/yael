[![crates.io](https://img.shields.io/crates/v/yael.svg)](https://crates.io/crates/yael)
[![License](https://img.shields.io/crates/l/yael.svg)](./LICENSE)

# Crate yael

<!-- cargo-rdme start -->

"Yet another endianness library"

This library is intended for uses where the data is not deserialized but already in ROM/RAM.

The library is `no_std`, has no features and no dependencies.

The conversion is made explicit by calling `new`, `get` and `set`.

Default is provided because 0 is identical for both byte orders.

Debug is provided because it can be useful and is expected to not be fast.

Operations which do not depend on the byte order can be performed on integers:
eq, and, or, xor, not.

But please note that a operation with a constant may be more efficient with `get`ting the value
instead of wrapping the constant with `new`, though depending on the constant and cpu.

There are several similar libraries, but they all differ in a some points.

Some alternatives:
- [bswap](https://crates.io/crates/bswap)
- [byteorder](https://crates.io/crates/byteorder)
- [endian](https://crates.io/crates/endian)
- [simple_endian](https://crates.io/crates/simple_endian)

## Examples

```rust
#[repr(C)]
struct Chunk {
    size: u32be,
    checksum: u32be,
}
let chunk = Chunk { size: u32be::new(55), checksum: u32be::new(0x52f0e743) };
let chunk_size = chunk.size.get();
```

```rust
#[repr(C)]
struct Data {
    mode: u16be,
    flags: u16be,
}

fn fix_flags1(data: &mut Data){
    // compare with a constant big-endian number, no conversion performed, can also be written as `data.mode.is_zero()`
    if data.mode == u16be::new(0) {
        // bit operations: also no conversion performed
        data.flags |= u16be::new(1);
    }
}

fn fix_flags2(data: &mut Data, flags_on_error: u16){
    // while these get and set do perform conversion
    if data.mode.get() > 80 {
        data.flags.set(flags_on_error);
    }
}
```

The `#[repr(C)]` is not required by the examples, but probably in many use cases.

## Floating Point

The floating point types do not support bit operations because a float does not either.

Comparison is not supported because it would differ from how a float behaves.

(`+0 == -0` float: equal, bits: different; `nan == nan` float: different, bits: equal)

<!-- cargo-rdme end -->
