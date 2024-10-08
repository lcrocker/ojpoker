//! # hash | [wiki](https://github.com/lcrocker/ojpoker/wiki/Hashes) | Hash functions for cards with various properties

use crate::cards::utils::*;
use crate::cards::card::*;

pub trait CardHashTrait {
    // 32-bit standard hash
    fn u32<I>(_cards: I) -> u32
    where I: IntoIterator<Item = Card> {
        panic!();
    }

    // 32-bit collison-free
    fn u32c<I>(_cards: I) -> u32
    where I: IntoIterator<Item = Card> {
        panic!();
    }

    // 32-bit collision-free order-independent
    fn u32co<I>(_cards: I) -> u32
    where I: IntoIterator<Item = Card> {
        panic!();
    }

    // 32-bit collision-free order-independent suit-independent
    fn u32cos<I>(_cards: I) -> u32
    where I: IntoIterator<Item = Card> {
        panic!();
    }

    // 64-bit standard hash
    fn u64<I>(_cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        panic!();
    }

    // 64-bit collision-free
    fn u64c<I>(_cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        panic!();
    }
    
    // 64-bit collision-free order-independent
    fn u64co<I>(_cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        panic!();
    }
    
    // 64-bit collision-free order-independent suit-independent
    fn u64cos<I>(_cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        panic!();
    } 
}

pub struct FNVHash {}

impl CardHashTrait for FNVHash {
    fn u32<I>(cards: I) -> u32
    where I: IntoIterator<Item = Card> {
        let mut h: u32 = 0x811C9DC5;

        for c in cards {
            h ^= c.0 as u32;
            h = h.wrapping_mul(0x01000193);
        }
        h
    }

    fn u64<I>(cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        let mut h: u64 = 0xCBF29CE484222325;

        for c in cards {
            h ^= c.0 as u64;
            h = h.wrapping_mul(0x100000001B3);
        }
        h
    }
}

pub struct PositionalHash {}

impl CardHashTrait for PositionalHash {
    fn u32c<I>(cards: I) -> u32
    where I: IntoIterator<Item = Card> {
        let mut max = 5;
        let mut h: u32 = 0;

        for c in cards {
            max -= 1;
            assert!(max >= 0);

            h <<= 6;
            h += (0x3F & c.0) as u32;
        }
        h
    }

    fn u32co<I>(cards: I) -> u32
    where I: IntoIterator<Item = Card> {
        let mut sorted: Vec<Card> = cards.into_iter().collect();
        oj_sort(&mut sorted[..]);
        PositionalHash::u32c(sorted)
    }

    fn u32cos<I>(cards: I) -> u32
    where I: IntoIterator<Item = Card> {
        let mut sorted: Vec<Card> = cards.into_iter().collect();
        oj_sort(&mut sorted[..]);

        let mut max = 8;
        let mut h: u32 = 0;

        for c in sorted {
            max -= 1;
            assert!(max >= 0);

            h <<= 4;
            h += 0x0F & (c.0 >> 2) as u32;
        }
        h
    }

    fn u64c<I>(cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        let mut max = 10;
        let mut h: u64 = 0;

        for c in cards {
            max -= 1;
            assert!(max >= 0);

            h <<= 6;
            h += (0x3F & c.0) as u64;
        }
        h
    }

    fn u64co<I>(cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        let mut sorted: Vec<Card> = cards.into_iter().collect();
        oj_sort(&mut sorted);
        PositionalHash::u64c(sorted)
    }

    fn u64cos<I>(cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        let mut sorted: Vec<Card> = cards.into_iter().collect();
        oj_sort(&mut sorted);

        let mut max = 16;
        let mut h: u64 = 0;

        for c in sorted {
            max -= 1;
            assert!(max >= 0);

            h <<= 4;
            h += (0x0F & (c.0 >> 2)) as u64;
        }
        h
    }
}

pub struct BitfieldHash {}

impl CardHashTrait for BitfieldHash {
    fn u64co<I>(cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        let mut h: u64 = 0;

        for c in cards {
            assert!(0 == (h & (1 << (c.0 as u64))));
            h |= 1 << (c.0 as u64);
        }
        h
    }
}

pub struct PrimeHash {}

impl CardHashTrait for PrimeHash {
    fn u32cos<I>(cards: I) -> u32
    where I: IntoIterator<Item = Card> {
        let mut max = 5;
        let mut h: u32 = 1;

        for c in cards {
            max -= 1;
            assert!(max >= 0);
        
            h *= PRIMES[0x0F & ((c.0 >> 2) as usize)];
        }
        h
    }

    fn u64co<I>(cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        let mut max = 7;
        let mut h: u64 = 1;

        for c in cards {
            max -= 1;
            assert!(max >= 0);
        
            h *= PRIMES[0x3F & (c.0 as usize)] as u64;
        }
        h
    }

    fn u64cos<I>(cards: I) -> u64
    where I: IntoIterator<Item = Card> {
        let mut max = 10;
        let mut h: u64 = 1;

        for c in cards {
            max -= 1;
            assert!(max >= 0);
        
            h *= PRIMES[0x0F & ((c.0 >> 2) as usize)] as u64;
        }
        h
    }
}

const PRIMES: [u32; 64] = [
    3, 5, 7, 11, 13, 17, 19, 23,
    29, 31, 37, 41, 43, 47, 53, 59,
    61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137,
    139, 149, 151, 157, 163, 167, 173, 179,
    181, 191, 193, 197, 199, 211, 223, 227,
    229, 233, 239, 241, 251, 257, 263, 269,
    271, 277, 281, 283, 293, 307, 311, 313,
];

