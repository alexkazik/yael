use core::mem;
use yael::{f32be, f32le, u16be, u16le, u32be, u32le};

#[derive(Copy, Clone)]
#[repr(C)]
struct Data {
    a: u32be,
    b: u32le,
    c: u16be,
    d: u16le,
    e: f32be,
    f: f32le,
}

#[test]
fn basic() {
    // check from_be_bytes
    assert_eq!(
        u16be::from_be_bytes([0x12, 0x34]).get(),
        u16::from_be_bytes([0x12, 0x34])
    );

    let mut data = {
        union BytesAsData {
            data: Data,
            bytes: [u8; 20],
        }

        let bytes = BytesAsData {
            bytes: [
                0x11, 0x22, 0x33, 0x44, // a
                0x55, 0x66, 0x77, 0x88, // b
                0x99, 0xaa, // c
                0xbb, 0xcc, // d
                52, 0, 0, 0, // e
                0, 0, 0, 52, // f
            ],
        };

        unsafe { bytes.data }
    };

    // check if the data is read/compared as expected
    assert_eq!(data.a.get(), 0x11223344);
    assert!(data.a == u32be::new(0x11223344));
    assert!(!data.a.is_zero());
    assert_eq!(data.b.get(), 0x88776655);
    assert!(data.b == u32le::new(0x88776655));
    assert!(!data.b.is_zero());
    assert_eq!(data.c.get(), 0x99aa);
    assert!(data.c == u16be::new(0x99aa));
    assert!(!data.c.is_zero());
    assert_eq!(data.d.get(), 0xccbb);
    assert!(data.d == u16le::new(0xccbb));
    assert!(!data.d.is_zero());

    // check set
    data.a.set(0x12345678);
    assert_eq!(
        unsafe { mem::transmute::<u32be, u32>(data.a) }.to_ne_bytes(),
        [0x12, 0x34, 0x56, 0x78]
    );

    // check float
    assert_eq!(data.e.get(), f32::EPSILON);
    data.e.set(f32::NEG_INFINITY);
    unsafe {
        assert_eq!(
            mem::transmute::<f32be, u32>(data.e),
            mem::transmute::<f32be, u32>(f32be::new(f32::NEG_INFINITY))
        );
    }
    assert_eq!(data.f.get(), f32::EPSILON);
    data.f.set(f32::NEG_INFINITY);
    unsafe {
        assert_eq!(
            mem::transmute::<f32le, u32>(data.f),
            mem::transmute::<f32le, u32>(f32le::new(f32::NEG_INFINITY))
        );
    }
}
