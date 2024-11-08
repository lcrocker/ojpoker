//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Hashes) | Various hash functions for cards
//! 
//! The common FNV hashes are often used for implementing hash tables and
//! doing quick checksumming for tests. They are not collision-free, but are
//! fast and simple.
//!
//! Positional hashes treat cards (or ranks) as digits of a base-n number,
//! They are therefore order-dependent and limited in size, but inherently
//! collision-free and useful for ranking hands.
//! 
//! Bitfield hashes represent each card as a bit in a 64-bit integer.
//! This is inherently order-independent and collision-free, and very fast,
//! but can't handle duplicate cards and produces huge numbers.
//! 
//! Prime hashes based on the product of prime numbers are inherently
//! collision-free, order-independent, handle duplicates, and produce
//! smaller numbers, but can only handle very small sets.
//! 
//! The "mp" functions create minimal perfect hashes for particular games
//! and are very specific to number of cards and type of deck.

use crate::errors::*;
use crate::cards::*;
use crate::utils::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/FNV_Hash) | 32-bit FNV-1a hash
pub fn ojh_fnv_32(cards: &[Card]) -> Result<u32, OjError> {
    let mut h: u32 = 0x811C9DC5;

    for c in cards {
        h ^= c.0 as u32;
        h = h.wrapping_mul(0x01000193);
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/FNV_Hash) | 64-bit FNV-1a hash
pub fn ojh_fnv_64(cards: &[Card]) -> Result<u64, OjError> {
    let mut h: u64 = 0xCBF29CE484222325;

    for c in cards {
        h ^= c.0 as u64;
        h = h.wrapping_mul(0x100000001B3);
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 32-bit positional hash
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | MPH for ace-to-five low
// Turns out a base-13 positional rank hash is a damn near minimal perfect hash
// for 5-card ace-to-five low hands.
pub fn ojh_positional_32cs_mp5_low(cards: &[Card]) -> Result<u32, OjError> {
    let mut max = 5;
    let mut h: u32 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(OjError::HashDomain(String::from("5 cards max")));
        }
        let mut r = c.rank() as u32 - 1;
        if r > 10 { r -= 1; }

        h *= 13;
        h += r;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 32-bit positional hash
pub fn ojh_positional_32c(cards: &[Card]) -> Result<u32, OjError> {
    let mut max = 5;
    let mut h: u32 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(OjError::HashDomain(String::from("5 cards max")));
        }
        h <<= 6;
        h += (0x3F & c.0) as u32;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 32-bit positional rank hash
pub fn ojh_positional_32cs(cards: &[Card]) -> Result<u32, OjError>{
    let mut max = 8;
    let mut h: u32 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(OjError::HashDomain(String::from("8 ranks max")));
        }
        h <<= 4;
        h += 0x0F & (c.rank() as u32);
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 64-bit positional hash
pub fn ojh_positional_64c(cards: &[Card]) -> Result<u64, OjError> {
    let mut max = 10;
    let mut h: u64 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(OjError::HashDomain(String::from("10 cards max")));
        }
        h <<= 6;
        h += (0x3F & c.0) as u64;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 64-bit positional rank hash
pub fn ojh_positional_64cs(cards: &[Card]) -> Result<u64, OjError> {
    let mut max = 16;
    let mut h: u64 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(OjError::HashDomain(String::from("16 ranks max")));
        }
        h <<= 4;
        h += (0x0F & (c.rank() as u8)) as u64;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Bitfield_Hash) | 64-bit bitfield hash
pub fn ojh_bitfield_64co(cards: &[Card]) -> Result<u64, OjError> {
    let mut h: u64 = 0;

    for c in cards {
        #[cfg(debug_assertions)]
        if 0 != (h & (1 << (c.0 as u64))) {
            return Err(OjError::HashDomain(String::from("duplicate card")));
        }
        h |= 1 << (c.0 as u64);
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | Convert bitfield to MPH
/// Given a bitfield with exactly 5 bits set, return the lexicographic
/// index of that particular set of bits for minimal perfect hash.
/// Requires English deck, 5 cards.
pub fn ojh_mp5_english(f: u64) -> u32 {
    // make ranks contiguous
    let mut b = f >> 8;
    b = (b & 0xFFFFFFFFFF) | ((b & 0x00FFF00000000000) >> 4);

    let mut h: u64 = oj_binomial(52, 5);
    let mut mask = 0x0008000000000000;
    let mut m = 1;

    for j in 0..52 {
        if 0 != (b & mask) {
            h -= oj_binomial(j, m);
            m += 1;
            if m > 5 { break; }
        }
        mask >>= 1;
    }
    debug_assert!(h <= 2598960);
    h as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | Convert bitfield to MPH
/// Given a bitfield with exactly 5 bits set, return the lexicographic
/// index of that particular set of bits for minimal perfect hash.
/// Low-ace deck version, 5 cards.
pub fn ojh_mp5_low(f: u64) -> u32 {
    // make ranks contiguous
    let mut b = f >> 4;
    b = (b & 0xFFFFFFFFFFF) | ((b & 0x00FF000000000000) >> 4);

    let mut h: u64 = oj_binomial(52, 5);
    let mut mask = 0x0008000000000000;
    let mut m = 1;

    for j in 0..52 {
        if 0 != (b & mask) {
            h -= oj_binomial(j, m);
            m += 1;
            if m > 5 { break; }
        }
        mask >>= 1;
    }
    debug_assert!(h <= 2598960);
    h as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | Convert bitfield to MPH
/// Given a bitfield with exactly 5 bits set, return the lexicographic
/// index of that particular set of bits for minimal perfect hash.
/// Low-ace deck version, 4 cards (for Badugi).
pub fn ojh_mp4_low(f: u64) -> u32 {
    let mut b = f >> 4;
    b = (b & 0xFFFFFFFFFFF) | ((b & 0x00FF000000000000) >> 4);

    let mut h: u64 = oj_binomial(52, 4);
    let mut mask = 0x0008000000000000;
    let mut m = 1;

    for j in 0..52 {
        if 0 != (b & mask) {
            h -= oj_binomial(j, m);
            m += 1;
            if m > 4 { break; }
        }
        mask >>= 1;
    }
    debug_assert!(h <= 270725);
    h as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Prime_Hash) | 32-bit prime rank hash
pub fn ojh_prime_32cos(cards: &[Card]) -> Result<u32, OjError> {
    let mut max = 5;
    let mut h: u32 = 1;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(OjError::HashDomain(String::from("5 cards max")));
        }
        h *= PRIMES[0x0F & (c.rank() as usize)];
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Prime_Hash) | 64-bit prime hash
pub fn ojh_prime_64co(cards: &[Card]) -> Result<u64, OjError> {
    let mut max = 7;
    let mut h: u64 = 1;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(OjError::HashDomain(String::from("7 cards max")));
        }
        h *= PRIMES[0x3F & (c.0 as usize)] as u64;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Prime_Hash) | 64-bit prime rank hash
pub fn ojh_prime_64cos(cards: &[Card]) -> Result<u64, OjError> {
    let mut max = 10;
    let mut h: u64 = 1;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(OjError::HashDomain(String::from("10 cards max")));
        }      
        h *= PRIMES[0x0F & (c.rank() as usize)] as u64;
    }
    Ok(h)
}

/// List of first 64 odd primes
const PRIMES: [u32; 64] = [
    3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
    61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137,
    139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227,
    229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::OjError;
    use crate::cards::hand::Hand;

    #[test]
    fn test_hashes() -> Result<(), OjError> {
        let h1 = Hand::default().init(
            cards!("Ac","Kc","Qc","Jc","Tc"));

        assert_eq!(309437067, ojh_fnv_32(h1.as_slice())?);
        assert_eq!(18055603845018456331, ojh_fnv_64(h1.as_slice())?);
        assert_eq!(1021528872, ojh_positional_32c(h1.as_slice())?);
        assert_eq!(1043898, ojh_positional_32cs(h1.as_slice())?);
        assert_eq!(1021528872, ojh_positional_64c(h1.as_slice())?);
        assert_eq!(1043898, ojh_positional_64cs(h1.as_slice())?);
        assert_eq!(1229501389969817600, ojh_bitfield_64co(h1.as_slice())?);
        assert_eq!(222951973, ojh_prime_32cos(h1.as_slice())?);
        assert_eq!(717864180907, ojh_prime_64co(h1.as_slice())?);
        assert_eq!(222951973, ojh_prime_64cos(h1.as_slice())?);

        Ok(())
    }
}

