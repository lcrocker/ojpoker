#![doc = include_str!("../doc/utils_module.md")]

use std::sync::{ Mutex, OnceLock };
use std::time::SystemTime;

static SEED: OnceLock<Mutex<[u32; 4]>> = OnceLock::new();

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Rand) | A simple xoshiro128++ PRNG
/// I know, I know, first rule of PRNG club is don't roll your own,
/// just use the library.
/// But I'm pretty sure I'm one of the few people who understands this stuff
/// well enough to bend that rule a bit, and I just don't like the standard
/// library of either Dart or Rust for this application.
pub fn oj_rand_next32() -> u32 {
    let mut s = SEED.get_or_init(|| {
        let _seed = SystemTime::now().
            duration_since(SystemTime::UNIX_EPOCH).
            expect("cannot realistically happen").as_millis();

        let new_state = [
            _seed as u32, _seed as u32 + 1, _seed as u32 + 2, _seed as u32 + 3,
        ];
        Mutex::new(new_state)
    }).lock().expect("Failed to lock random seed mutex");

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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Range_Uniform) | Random integer within range
/// Return a random integer uniformly distributed in range [0, limit)
/// with no division, using rejection sampling. The mask `m` is created
/// to minimize rejections, which will be at worst 50%.
pub fn oj_rand_range(limit: usize) -> usize {
    debug_assert!(limit > 0);
    debug_assert!(limit < (1 << 31));

    let mut m: u32 = (limit - 1) as u32;
    for i in [1, 2, 4, 8, 16] { m |= m >> i; }

    let mut loop_guard = 100;
    loop {
        let v = oj_rand_next32() & m;
        if (v as usize) < limit { return v as usize; }

        loop_guard -= 1;
        if loop_guard <= 0 { break; }
    }
    0
}

/// # [wiki](https://github.com/lcrocker/ojpoker/wiki/Shuffle) | Standard Fisher-Yates shuffle
/// Standard shuffle using our own PRNG.
pub fn oj_shuffle<T>(a: &mut [T]) {
    if a.len() < 2 { return; }

    for i in (1..a.len()).rev() {
        let j = oj_rand_range(i + 1);
        if i != j { a.swap(i, j); }
    }
}

/// Non-recursive heapify function for heapsort.
fn heapify<T: PartialOrd>(a: &mut [T], n: usize, i: usize) {
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

macro_rules! compare_and_swap {
    ( $a:ident, $x:expr, $y:expr ) => {
        if $a[$x] < $a[$y] { $a.swap($x, $y); }
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Sort) | Slightly specialized heapsort
/// Heapsort optimized for small sets like poker hands, and in descending order which is
/// most useful for ranking and displaying poker hands.
pub fn oj_sort<T: PartialOrd>(a: &mut [T]) {
    match a.len() {
        5 => {
            compare_and_swap!(a, 0, 1);
            compare_and_swap!(a, 3, 4);
            compare_and_swap!(a, 2, 4);
            compare_and_swap!(a, 2, 3);
            compare_and_swap!(a, 0, 3);
            compare_and_swap!(a, 0, 2);
            compare_and_swap!(a, 1, 4);
            compare_and_swap!(a, 1, 3);
            compare_and_swap!(a, 1, 2);
        },
        4 => {
            compare_and_swap!(a, 0, 1);
            compare_and_swap!(a, 2, 3);
            compare_and_swap!(a, 0, 2);
            compare_and_swap!(a, 1, 3);
            compare_and_swap!(a, 1, 2);
        },
        3 => {
            compare_and_swap!(a, 1, 2);
            compare_and_swap!(a, 0, 2);
            compare_and_swap!(a, 0, 1);
        },
        2 => {
            compare_and_swap!(a, 0, 1);
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

// pub fn oj_next_combination(a: &mut [usize], n: usize) -> bool {
//     let k = a.len();

//     for i in (0..k).rev() {
//         if a[i] < n - k + i + 1 {
//             a[i] += 1;
//             for j in (i + 1)..k {
//                 a[j] = a[j - 1] + 1;
//             }
//             return true;
//         }
//     }
//     false
// }

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Next_Combination) | Iterate over combinations
/// Given an array of indices into a larger array, increment the 0-based
/// indices to the next k-combination, returning true when done.
pub fn oj_next_combination(a: &mut [usize], n: usize) -> bool {
    let k = a.len();

    let mut found: i32 = -1;
    for j in (0..k).rev() {
        if a[j] < n - k + j {
            found = j as i32;
            break;
        }
    }
    if -1 == found { return true; }

    a[found as usize] += 1;
    for j in ((found + 1) as usize)..k {
        a[j] = a[j - 1] + 1;
    }
    false
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Binomial) | Calculate binomial coefficient
/// Calculate the binomial coefficient "n choose k" using a lookup table.
/// Is only valid for n, k in the range 0..=63.
#[inline(always)]
pub const fn oj_binomial(n: i32, k: i32) -> u64 {
    debug_assert!(n >= 0 && n < 64 && k >= 0 && k < 64);
    COEFFICIENTS[n as usize][k as usize]
}

/// Lookup table for binomial coefficients up to 63c63, calculated
/// additively with Pascal's triangle method at compile time.
pub const COEFFICIENTS: [[u64; 64]; 64] = {
    let mut n = 0;
    let mut table = [[0; 64]; 64];

    while n < 64 {
        let mut k = 0;

        while k <= n {
            table[n][k] = if k == 0 || k == n {
                1 
            } else {
                table[n - 1][k] + table[n - 1][k - 1]
            };
            k += 1;
        }
        n += 1;
    }
    table
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn test_binomial_coeffiecients() -> aResult<()> {
        assert_eq!(1, oj_binomial(0,0));
        assert_eq!(0, oj_binomial(0,1));
        assert_eq!(1, oj_binomial(1,0));
        assert_eq!(1, oj_binomial(63,63));
        assert_eq!(435, oj_binomial(30,2));
        assert_eq!(749398, oj_binomial(41,5));
        assert_eq!(2869685, oj_binomial(53,5));
        assert_eq!(17672631900, oj_binomial(37,18));
        assert_eq!(330, oj_binomial(11,7));
        assert_eq!(5200300, oj_binomial(25,12));
        assert_eq!(4272048, oj_binomial(33,7));
        assert_eq!(1, oj_binomial(4,0));
        assert_eq!(2707475148, oj_binomial(38,12));
        assert_eq!(11440, oj_binomial(16,9));
        assert_eq!(54264, oj_binomial(21,15));
        assert_eq!(636763050, oj_binomial(51,8));
        assert_eq!(1, oj_binomial(12,12));
        assert_eq!(5752004349, oj_binomial(43,32));
        assert_eq!(231917400, oj_binomial(56,7));
        assert_eq!(76904685, oj_binomial(40,8));
        assert_eq!(296010, oj_binomial(27,21));
        assert_eq!(0, oj_binomial(46,47));
        assert_eq!(74613, oj_binomial(22,6));
        assert_eq!(58, oj_binomial(58,57));
        assert_eq!(4, oj_binomial(4,1));
        assert_eq!(4280561376, oj_binomial(42,11));
        assert_eq!(6566222272575, oj_binomial(61,13));
        assert_eq!(43, oj_binomial(43,1));
        assert_eq!(3003, oj_binomial(15,10));
        assert_eq!(265182525, oj_binomial(31,17));
        assert_eq!(3819816, oj_binomial(56,5));
        assert_eq!(40920, oj_binomial(33,29));
        assert_eq!(435897, oj_binomial(37,5));
        assert_eq!(314457495, oj_binomial(47,39));
        assert_eq!(97997533741800, oj_binomial(56,17));
        assert_eq!(254186856, oj_binomial(36,26));
        assert_eq!(3, oj_binomial(3,1));
        assert_eq!(3268760, oj_binomial(25,10));
        assert_eq!(90177170226, oj_binomial(61,51));
        assert_eq!(3060, oj_binomial(18,14));
        assert_eq!(1, oj_binomial(13,13));
        assert_eq!(1040465790, oj_binomial(54,46));
        assert_eq!(1370754, oj_binomial(46,5));
        assert_eq!(56, oj_binomial(8,3));
        assert_eq!(91390, oj_binomial(40,36));
        assert_eq!(15084504396, oj_binomial(39,14));
        assert_eq!(406, oj_binomial(29,27));
        assert_eq!(57902201338905, oj_binomial(57,41));
        assert_eq!(6188, oj_binomial(17,5));
        assert_eq!(319770, oj_binomial(22,8));

        for _ in 0..500 {
            let n = oj_rand_range(64) as i32;
            let k = oj_rand_range(64) as i32;
    
            if k > n {
                assert_eq!(0, oj_binomial(n, k));
            } else if k == n {
                assert_eq!(1, oj_binomial(n, k));
            } else if k == 1 {
                assert_eq!(n, oj_binomial(n, k) as i32);
            } else {
                assert_eq!(oj_binomial(n, k), oj_binomial(n, n - k));
            }
            if n > 0 && k > 0 {
                assert_eq!(oj_binomial(n - 1, k) + oj_binomial(n - 1, k - 1),
                    oj_binomial(n, k));
            }
        }
        aOk(())
    }
}
