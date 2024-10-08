//! # rand | [wiki](https://github.com/lcrocker/ojpoker/wiki/Rust_Rand) | Simple fast xoshiro128++ PRNG and utilities.
//! I know, I know, first rule of PRNG club is don't roll your own,
//! just use the library.
//! But I'm pretty sure I'm one of the few people who understands this
//! stuff well enough to ignore that rule, and I just don't like the rand
//! crate for this application.

use std::sync::{ Mutex, OnceLock };
use crate::cards::Card;

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
pub fn range_uniform(limit: usize) -> usize {
    debug_assert!(limit > 0);
    debug_assert!(limit < (1 << 31));

    let mut m: u32 = (limit - 1) as u32;
    for i in [1, 2, 4, 8, 16] { m |= m >> i; }

    let mut loop_guard = 100;
    while loop_guard > 0 {
        loop_guard -= 1;

        let v = next32() & m;
        if (v as usize) < limit { return v as usize; }
    }
    0
}

/// Standard Fisher-Yates shuffle using our own PRNG.
pub fn oj_shuffle(a: &mut [Card]) {
    if a.len() < 2 { return; }

    for i in (1..a.len()).rev() {
        let j = range_uniform(i + 1);
        if i != j { a.swap(i, j); }
    }
}

fn heapify(a: &mut [Card], n: usize, i: usize) {
    let mut i = i;
    let mut loop_guard = 200;

    while loop_guard > 0 {
        loop_guard -= 1;

        let mut min = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && a[left] < a[min] { min = left; }
        if right < n && a[right] < a[min] { min = right; }

        if min == i { break; }

        a.swap(i, min);
        i = min;
    }
}

/// Somewhat specialized sort optimized for small sets, like poker hands.
pub fn oj_sort(a: &mut [Card]) {
    match a.len() {
        5 => {
            if a[0] < a[1] { a.swap(0, 1); }
            if a[3] < a[4] { a.swap(3, 4); }
            if a[2] < a[4] { a.swap(2, 4); }
            if a[2] < a[3] { a.swap(2, 3); }
            if a[0] < a[3] { a.swap(0, 3); }
            if a[0] < a[2] { a.swap(0, 2); }
            if a[1] < a[4] { a.swap(1, 4); }
            if a[1] < a[3] { a.swap(1, 3); }
            if a[1] < a[2] { a.swap(1, 2); }
        },
        4 => {
            if a[0] < a[1] { a.swap(0, 1); }
            if a[2] < a[3] { a.swap(2, 3); }
            if a[0] < a[2] { a.swap(0, 2); }
            if a[1] < a[3] { a.swap(1, 3); }
            if a[1] < a[2] { a.swap(1, 2); }
        },
        3 => {
            if a[1] < a[2] { a.swap(1, 2); }
            if a[0] < a[2] { a.swap(0, 2); }
            if a[0] < a[1] { a.swap(0, 1); }    
        },
        2 => {
            if a[0] < a[1] { a.swap(0, 1); }
        },
        1 | 0 => {},
        _ => {
            for i in (0..=(a.len() / 2)).rev() {
                heapify(a, a.len(), i);
            }
            for i in (1..a.len()).rev() {
                a.swap(0, i);
                heapify(a, i, 0);
            }
        },
    }
}

pub fn oj_next_combination(a: &mut [usize], n: usize) -> bool {
    let k = a.len();

    for i in (0..k).rev() {
        if a[i] < n - k + i + 1 {
            a[i] += 1;
            for j in (i + 1)..k {
                a[j] = a[j - 1] + 1;
            }
            return true;
        }
    }
    false
}
