//! # rand | [wiki](https://github.com/lcrocker/ojpoker/wiki/Rand) | Simple fast xoshiro128++ PRNG and utilities.
//! I know, I know, first rule of PRNG club is don't roll your own,
//! just use the library.
//! But I'm pretty sure I'm one of the few people who understands this
//! stuff well enough to ignore that rule, and I just don't like the rand
//! crate for this application.

use std::sync::{ Mutex, OnceLock };

static CELL: OnceLock<Mutex<[u32; 4]>> = OnceLock::new();

// Get next 32 bits from PRNG
pub fn next32() -> u32 {
    let mut s = CELL.get_or_init(|| {
        let mut bytes = [0u8; 16];
        getrandom::getrandom(&mut bytes).unwrap();

        let new_state = [
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        ];
        Mutex::new(new_state)
    }).lock().unwrap();

    let result:u32 = ((s[0].wrapping_add(s[3])).rotate_left(7))
        .wrapping_add(s[0]);
    let t: u32 = s[1] << 9;

	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];
	s[2] ^= t;
	s[3] = s[3].rotate_left(11);

    result
}

/// Return a random integer uniformly distributed in range [0, limit)
/// with no division, using rejection sampling. The mask `m` is created
/// to minimize rejections, which will be at worst 50%.
pub fn range_uniform(limit: i32) -> i32 {
    debug_assert!(limit > 0);
    let mut m: u32 = (limit - 1) as u32;
    for i in [1, 2, 4, 8, 16] { m |= m >> i; }

    loop {
        let v = next32() & m;
        if (v as i32) < limit { return v as i32; }
    }
}
