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
/// ```rust
/// use onejoker::*;
///
/// let h = Hand::default().init(cards!("Ac","Kc","Qc","Jc","Tc"));
/// assert_eq!(309437067, ojh_fnv_32(h.as_slice()).unwrap());
/// ```
pub fn ojh_fnv_32(cards: &[Card]) -> Result<u32, OjError> {
    let mut h: u32 = 0x811C_9DC5;

    for c in cards {
        h ^= c.0 as u32;
        h = h.wrapping_mul(0x0100_0193);
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/FNV_Hash) | 64-bit FNV-1a hash
/// ```rust
/// use onejoker::*;
///
/// let h = Hand::default().init(cards!("Ac","Kc","Qc","Jc","Tc"));
/// assert_eq!(18055603845018456331, ojh_fnv_64(h.as_slice()).unwrap());
/// ```
pub fn ojh_fnv_64(cards: &[Card]) -> Result<u64, OjError> {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;

    for c in cards {
        h ^= c.0 as u64;
        h = h.wrapping_mul(0x0100_0000_01B3);
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 32-bit positional hash
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | MPH for ace-to-five low
///
/// Turns out a base-13 positional rank hash is a damn near minimal perfect hash
/// for 5-card ace-to-five low hands
/// ```rust
/// use onejoker::*;
///
/// let d = Deck::new(DeckType::Low);
/// let h = d.new_hand().init(cards!("7c","6d","4h","3s","2s"));
/// assert_eq!(182885, ojh_positional_32cs_mp5_low(h.as_slice()).unwrap());
/// ```
pub fn ojh_positional_32cs_mp5_low(cards: &[Card]) -> Result<u32, OjError> {
    let mut max = 8;
    let mut h: u32 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(OjError::HashDomain(String::from("8 cards max")));
        }
        let mut r = c.rank() as u32 - 1;
        if r > 10 { r -= 1; }

        h *= 13;
        h += r;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 32-bit positional hash
/// ```rust
/// use onejoker::*;
///
/// let h = Hand::default().init(cards!("2s","2h","2c","2d","Kc"));
/// assert_eq!(187204216, ojh_positional_32c(h.as_slice()).unwrap());
/// ```
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
/// ```rust
/// use onejoker::*;
///
/// let h = Hand::default().init(cards!("2s","2h","2c","2d","Kc"));
/// assert_eq!(0x0002_222E, ojh_positional_32cs(h.as_slice()).unwrap());
/// ```
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
/// ```rust
/// use onejoker::*;
///
/// let h = Hand::default().init(cards!("Qc","Qd","Qs","Tc","Th"));
/// assert_eq!(886536746, ojh_positional_64c(h.as_slice()).unwrap());
/// ```
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
/// ```rust
/// use onejoker::*;
///
/// let h = Hand::default().init(cards!("Qc","Qd","Qs","Tc","Th"));
/// assert_eq!(0x000D_DDAA, ojh_positional_64cs(h.as_slice()).unwrap());
/// ```
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
/// ```rust
/// use onejoker::*;
///
/// let h = Hand::default().init(cards!("Jc","9c","7c","5c","3c"));
/// assert_eq!(0x1010_1010_1000, ojh_bitfield_64co(h.as_slice()).unwrap());
/// ```
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
/// ```rust
/// use onejoker::*;
///
/// let d = Deck::new(DeckType::English);
/// let h = d.new_hand().init(cards!("2s","2d","2c","2h","3c"));
/// let b = ojh_bitfield_64co(h.as_slice()).unwrap();
/// assert_eq!(1, ojh_mp5_english(b));
/// ```
pub fn ojh_mp5_english(f: u64) -> u32 {
    // make ranks contiguous
    let mut b = f >> 8;
    b = (b & 0x00FF_FFFF_FFFF) | ((b & 0x00FF_F000_0000_0000) >> 4);

    let mut h: u64 = oj_binomial(52, 5);
    let mut mask = 0x0008_0000_0000_0000;
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
/// Requires Stripped deck, 5 cards.
/// ```rust
/// use onejoker::*;
///
/// let d = Deck::new(DeckType::Stripped);
/// let h = d.new_hand().init(cards!("7s","7d","7c","7h","8c"));
/// let b = ojh_bitfield_64co(h.as_slice()).unwrap();
/// assert_eq!(1, ojh_mp5_stripped(b));
/// ```
pub fn ojh_mp5_stripped(f: u64) -> u32 {
    // make ranks contiguous
    let mut b = f >> 28;
    b = (b & 0x0000_000F_FFFF) | ((b & 0x0000_000F_FF00_0000) >> 4);

    let mut h: u64 = oj_binomial(32, 5);
    let mut mask = 0x0000_0000_8000_0000;
    let mut m = 1;

    for j in 0..32 {
        if 0 != (b & mask) {
            h -= oj_binomial(j, m);
            m += 1;
            if m > 5 { break; }
        }
        mask >>= 1;
    }
    debug_assert!(h <= 201376);
    h as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | Convert bitfield to MPH
/// Given a bitfield with exactly 5 bits set, return the lexicographic
/// index of that particular set of bits for minimal perfect hash.
/// Low-ace deck version, 5 cards.
/// ```rust
/// use onejoker::*;
///
/// let d = Deck::new(DeckType::Low);
/// let h = d.new_hand().init(cards!("Ac","Ad","Ah","As","2c"));
/// let b = ojh_bitfield_64co(h.as_slice()).unwrap();
/// assert_eq!(1, ojh_mp5_low(b));
/// ```
pub fn ojh_mp5_low(f: u64) -> u32 {
    // make ranks contiguous
    let mut b = f >> 4;
    b = (b & 0x0FFF_FFFF_FFFF) | ((b & 0x00FF_0000_0000_0000) >> 4);

    let mut h: u64 = oj_binomial(52, 5);
    let mut mask = 0x0008_0000_0000_0000;
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
/// Given a bitfield with exactly 4 bits set, return the lexicographic
/// index of that particular set of bits for minimal perfect hash.
/// High-ace deck version, 4 cards (for Badugi).
/// ```rust
/// use onejoker::*;
///
/// let d = Deck::new(DeckType::English);
/// let h = d.new_hand().init(cards!("2c","2d","2h","2s"));
/// let b = ojh_bitfield_64co(h.as_slice()).unwrap();
/// assert_eq!(1, ojh_mp4_english(b));
/// ```
pub fn ojh_mp4_english(f: u64) -> u32 {
    let mut b = f >> 8;
    b = (b & 0x00FF_FFFF_FFFF) | ((b & 0x00FF_F000_0000_0000) >> 4);

    let mut h: u64 = oj_binomial(52, 4);
    let mut mask = 0x0008_0000_0000_0000;
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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | Convert bitfield to MPH
/// Given a bitfield with exactly 4 bits set, return the lexicographic
/// index of that particular set of bits for minimal perfect hash.
/// Low-ace deck version, 4 cards (for Badugi).
/// ```rust
/// use onejoker::*;
///
/// let d = Deck::new(DeckType::Low);
/// let h = d.new_hand().init(cards!("Ac","Ad","Ah","As"));
/// let b = ojh_bitfield_64co(h.as_slice()).unwrap();
/// assert_eq!(1, ojh_mp4_low(b));
/// ```
pub fn ojh_mp4_low(f: u64) -> u32 {
    let mut b = f >> 4;
    b = (b & 0x0FFF_FFFF_FFFF) | ((b & 0x00FF_0000_0000_0000) >> 4);

    let mut h: u64 = oj_binomial(52, 4);
    let mut mask = 0x0008_0000_0000_0000;
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
/// ```rust
/// use onejoker::*;
///
/// let h = Hand::default().init(cards!("Kc","Qd","Jh","Ts","9h"));
/// assert_eq!(117144257, ojh_prime_32cos(h.as_slice()).unwrap());
/// ```
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
/// ```rust
/// use onejoker::*;
///
/// let h = Hand::default().init(cards!("Kc","Qd","Jh","Ts","9h"));
/// assert_eq!(529321587761, ojh_prime_64co(h.as_slice()).unwrap());
/// ```
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
/// ```rust
/// use onejoker::*;
///
/// let d = Deck::new(DeckType::Low);
/// let h = d.new_hand().init(cards!("Ac","2d","3h","4s","5h"));
/// assert_eq!(5 * 7 * 11 * 13 * 17, ojh_prime_64cos(h.as_slice()).unwrap());
/// ```
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

