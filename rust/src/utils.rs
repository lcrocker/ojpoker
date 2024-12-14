#![doc = include_str!("../doc/utils_module.md")]

use std::time::SystemTime;

use crate::cards::hashes::{ojh_uhash_64, OJH_SPLIT_MIX_CONSTANT};

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/PRNG) | A simple xoshiro256** PRNG
///
/// I know, I know, first rule of PRNG club is don't roll your own,
/// just use the library.
/// I'm pretty sure my Math chops are up to understanding this stuff well
/// enough to bend that rule a bit, and I just don't like the standard
/// library for this application.
/// Xoshiro crate is better, but contains a lot of stuff I don't need.
#[derive(Debug, Default, Clone)]
pub struct Random {
    s: [u64; 4],
    buf32: u64,
    buf16: u64,
    buf32state: u8,
    buf16state: u8,
    seeded: bool,
}

impl Random {
    /// Create a new randomizer object. Will be lazily initialized
    /// and seeded on first use.
    pub fn new() -> Self {
        Random::default()
    }

    /// Seed with given number for reproducibility.
    pub fn seeded(mut self, seed: u64) -> Self {
        self.s[0] = ojh_uhash_64(seed, OJH_SPLIT_MIX_CONSTANT);
        self.s[1] = ojh_uhash_64(seed + 1, OJH_SPLIT_MIX_CONSTANT);
        self.s[2] = ojh_uhash_64(seed + 2, OJH_SPLIT_MIX_CONSTANT);
        self.s[3] = ojh_uhash_64(seed + 3, OJH_SPLIT_MIX_CONSTANT);

        self.seeded = true;
        self.buf32state = 0;
        self.buf16state = 0;
        self
    }

    /// Execute one round of the xoshiro256** 64-bit PRNG.
    pub fn next64(&mut self) -> u64 {
        if ! self.seeded {
            let seed = SystemTime::now().
                duration_since(SystemTime::UNIX_EPOCH).
                expect("cannot realistically happen").as_nanos() as u64;

            self.s[0] = ojh_uhash_64(seed + 3, OJH_SPLIT_MIX_CONSTANT);
            self.s[1] = ojh_uhash_64(seed + 2, OJH_SPLIT_MIX_CONSTANT);
            self.s[2] = ojh_uhash_64(seed + 1, OJH_SPLIT_MIX_CONSTANT);
            self.s[3] = ojh_uhash_64(seed, OJH_SPLIT_MIX_CONSTANT);
            self.seeded = true;
        }
        let result = (self.s[1].wrapping_mul(5)).rotate_left(7)
            .wrapping_mul(9);
        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);

        result
    }

    /// Jump is equivalent to 2^128 calls to next(); it can be used to
    /// generate non-overlapping subsequences for parallel computations.
    pub fn jump(&mut self) {
        debug_assert!(self.seeded);

        let jump: [u64; 4] = [
            0x180EC6D33CFD0ABA, 0xD5A61266F0C9392C,
            0xA9582618E03FC9AA, 0x39ABDC4529B1661C
        ];
        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;

        for j in jump {
            for b in 0..64 {
                if 0 != j & (1 << b) {
                    s0 ^= self.s[0];
                    s1 ^= self.s[1];
                    s2 ^= self.s[2];
                    s3 ^= self.s[3];
                }
                self.next64();
            }
        }
        self.s[0] = s0;
        self.s[1] = s1;
        self.s[2] = s2;
        self.s[3] = s3;
    }

    /// Get 32 random bits.
    pub fn next32(&mut self) -> u32 {
        (match self.buf32state {
            0 => {
                self.buf32 = self.next64();
                self.buf32state = 1;
                self.buf32 >> 32
            },
            1 => {
                self.buf32state = 0;
                self.buf32
            },
            _ => unreachable!(),
        }) as u32
    }

    /// Get 16 random bits.
    pub fn next16(&mut self) -> u16 {
        (match self.buf16state {
            0 => {
                self.buf16 = self.next64();
                self.buf16state = 1;
                self.buf16 >> 48
            },
            1 => {
                self.buf16state = 2;
                self.buf16 >> 32
            },
            2 => {
                self.buf16state = 3;
                self.buf16 >> 16
            },
            3 => {
                self.buf16state = 0;
                self.buf16
            },
            _ => unreachable!(),
        }) as u16
    }

    /// Faster, but not precisely uniform, using Lemire's method.
    pub fn uniform32(&mut self, limit: usize) -> usize {
        debug_assert!(limit > 0);
        debug_assert!(limit < 0x8000_0000);

        let v: u64 = self.next32() as u64;
        ((v * (limit as u64)) >> 32) as usize
    }

    /// Slower, but precisely uniform, using rejection sampling with mask.
    pub fn uniform16(&mut self, limit: usize) -> usize {
        debug_assert!(limit > 0);
        debug_assert!(limit < 0x8000);

        let mut m: u16 = (limit - 1) as u16;
        m |= m >> 1;
        m |= m >> 2;
        m |= m >> 4;
        m |= m >> 8;

        let mut loop_guard = 100;
        loop {
            loop_guard -= 1;
            assert!(loop_guard > 0);

            let v = m & self.next16();
            if v < limit as u16 { return v as usize; }
        }
    }
}

/// # [wiki](https://github.com/lcrocker/ojpoker/wiki/oj_shuffle) | Standard Fisher-Yates shuffle
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::utils::{Random, oj_shuffle};
///
/// let mut rng = Random::new();
/// let mut v = [1,2,3,4,5,6,7,8,9,10];
/// oj_shuffle(&mut v, &mut rng);
/// ```
pub fn oj_shuffle<T>(a: &mut [T], rng: &mut Random) {
    if a.len() < 2 { return; }

    for i in (1..a.len()).rev() {
        let j = rng.uniform32(i + 1);
        if i != j { a.swap(i, j); }
    }
}

/// Non-recursive heapify function for heapsort.
fn heapify<T: PartialOrd>(a: &mut [T], n: usize, i: usize) {
    let mut i = i;

    let mut loop_guard = 200;
    loop {
        loop_guard -= 1;
        assert!(loop_guard > 0);

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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/oj_sort) | Slightly specialized heapsort
/// Heapsort optimized for small sets like poker hands, and in descending order which is
/// most useful for ranking and displaying poker hands.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::utils::oj_sort;
///
/// let mut v = [7,1,5,10,2,3,9,4,6,8];
/// oj_sort(&mut v);
/// assert_eq!(v[0], 10);
/// ```
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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/oj_next_combination) | Iterate over combinations
/// Given an array of indices into a larger array, increment the 0-based
/// indices to the next k-combination, returning true when done.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::utils::oj_next_combination;
///
/// let mut a = [0,1,2];
/// oj_next_combination(&mut a, 5);     // 0,1,3
/// assert_eq!(a[2], 3);
/// ```
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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/oj_binomial) | Calculate binomial coefficient
/// Calculate the binomial coefficient "n choose k" using a lookup table.
/// Is only valid for n, k in the range 0..=63.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::utils::oj_binomial;
///
/// assert_eq!(oj_binomial(5, 2), 10);
/// ```
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
    use crate::error::Result;

    #[test]
    fn test_rand_range() -> Result<()> {
        let mut rng = Random::new();
        let mut array = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19];
        let mut counts = [0; 20];

        for _ in 0..1000000 {
            oj_shuffle(&mut array[..], &mut rng);
            for i in 0..20 {
                if 0 == array[i] {
                    counts[i] += 1;
                    break;
                }
            }
        }
        for i in 0..20 {
            assert!(counts[i] > 49000);
            assert!(counts[i] < 51000);
        }
        Ok(())
    }

    #[test]
    fn test_binomial_coeffiecients() -> Result<()> {
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

        let mut rng = Random::new();
        for _ in 0..500 {
            let n = rng.uniform16(64) as i32;
            let k = rng.uniform32(64) as i32;

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
        Ok(())
    }
}
