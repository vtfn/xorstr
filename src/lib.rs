#![no_std]
#![feature(decl_macro)]

pub macro xor($s:literal) {{
    const LEN: usize = $s.len();

    static mut BYTES: [u8; LEN] = const {
        let mut bytes = [0u8; LEN];

        let mut i = 0;
        while i < LEN {
            bytes[i] = $s.as_bytes()[i] ^ 0x69;
            i += 1;
        }

        bytes
    };

    unsafe {
        ::core::hint::black_box(BYTES);

        for x in BYTES.iter_mut() {
            *x ^= 0x69;
        }

        ::core::str::from_utf8_unchecked(&BYTES)
    }
}}
