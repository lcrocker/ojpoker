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

#[cfg(target_arch = "x86_64")]
use std::arch::asm;

use crate::error::{Error,Result};
use crate::cards::*;
use crate::utils::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/FNV_Hash) | 32-bit FNV-1a hash
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let h = Hand::default().init(hand!("Ac","Kc","Qc","Jc","Tc"));
/// assert_eq!(309437067, ojh_fnv_32(&h[..]).unwrap());
/// ```
pub fn ojh_fnv_32(cards: &[Card]) -> Result<u32> {
    let mut h: u32 = 0x811C_9DC5;

    for c in cards {
        h ^= c.0 as u32;
        h = h.wrapping_mul(0x0100_0193);
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/FNV_Hash) | 64-bit FNV-1a hash
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let h = Hand::default().init(hand!("Ac","Kc","Qc","Jc","Tc"));
/// assert_eq!(18055603845018456331, ojh_fnv_64(&h[..]).unwrap());
/// ```
pub fn ojh_fnv_64(cards: &[Card]) -> Result<u64> {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;

    for c in cards {
        h ^= c.0 as u64;
        h = h.wrapping_mul(0x0100_0000_01B3);
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Universal_Hash) | 32-bit universal hash
///
/// Simple and fast 32-bit integer-to-integer universal hash function,
/// useful for making perfect hashes for small tables.
/// Based on Chris Wellons' prospector.
pub const fn ojh_uhash_32(inp: u32, param: u32) -> u32 {
    let mut v = param.wrapping_add(inp);
    v ^= v >> 15;
    v = v.wrapping_mul(0x2C1B_3C6D);
    v ^= v >> 12;
    v = v.wrapping_mul(0x297A_2D39);
    v ^= v >> 15;
    v
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Universal_Hash) | 64-bit universal hash
///
/// Simple and fast 64-bit integer-to-integer universal hash function.
/// Based on Sebastiano Vigna's SplitMix64.
pub const fn ojh_uhash_64(inp: u64, param: u64) -> u64 {
    let mut v = param.wrapping_add(inp);
    v ^= v >> 30;
    v = v.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    v ^= v >> 27;
    v = v.wrapping_mul(0x94D0_49BB_1331_11EB);
    v ^= v >> 31;
    v
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Universal_Hash) | 64-to-32-bit mixer
///
/// 64-to-32-bit mixer (Thomas Wang)
pub const fn ojh_mix_64_32(inp: u64) -> u32 {
    let mut v: u64 = (!inp) + (inp << 18);
    v ^= v >> 31;
    v *= 21;
    v ^= v >> 11;
    v += v << 6;
    v ^= v >> 22;
    v as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 32-bit positional hash
///
/// Base-13 positional rank hash -- low ace decks, no jokers.
/// Turns out a base-13 positional rank hash is a damn near minimal perfect
/// hash for 5-card ace-to-five low hands.
pub fn ojh_positional_32cs_low(cards: &[Card]) -> Result<u32> {
     let mut max = 8;
     let mut h: u32 = 0;

     for c in cards {
         max -= 1;
         if max < 0 {
             return Err(Error::HashDomain(String::from("8 cards max")));
         }
         let mut r = c.rank() as u32 - 1;
         if r > 10 { r -= 1; }

         h *= 13;
         h += r;
     }
     Ok(h)
}

#[cfg(not(target_arch = "x86_64"))]
pub fn ojh_positional_mp5_low(cards: &[Card]) -> u32 {
    ojh_positional_32cs_low(cards).unwrap() as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | MPH for ace-to-five low
///
/// This branchless assembly version of teh base-13 positional rank hash
/// should produce the same value as the above Rust version for 5 cards.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let d = Deck::new(DeckType::Low);
/// let h = d.new_hand().init(hand!("7c","6d","4h","3s","2s"));
/// assert_eq!(182885, ojh_positional_mp5_low(&h[..]));
/// ```
#[cfg(target_arch = "x86_64")]
pub fn ojh_positional_mp5_low(cards: &[Card]) -> u32 {
    let mut hash: u64 = 0;
    debug_assert!(5 == cards.len());
    debug_assert!(cards[0].0 > 3 && cards[0].0 < 60);
    debug_assert!(cards[1].0 > 3 && cards[1].0 < 60);
    debug_assert!(cards[2].0 > 3 && cards[2].0 < 60);
    debug_assert!(cards[3].0 > 3 && cards[3].0 < 60);
    debug_assert!(cards[4].0 > 3 && cards[4].0 < 60);

    unsafe {
        asm!(
            "movzx {tmp}, byte ptr [{cards}]",
            "shr {tmp}, 2",
            "dec {tmp}",
            "cmp {tmp:l}, 11",
            "cmc",
            "sbb {tmp}, 0",
            "mov {hash}, {tmp}",

            "movzx {tmp}, byte ptr [{cards} + 1]",
            "shr {tmp}, 2",
            "dec {tmp}",
            "cmp {tmp:l}, 11",
            "cmc",
            "sbb {tmp}, 0",
            "imul {hash}, 13",
            "add {hash}, {tmp}",

            "movzx {tmp}, byte ptr [{cards} + 2]",
            "shr {tmp}, 2",
            "dec {tmp}",
            "cmp {tmp:l}, 11",
            "cmc",
            "sbb {tmp}, 0",
            "imul {hash}, 13",
            "add {hash}, {tmp}",

            "movzx {tmp}, byte ptr [{cards} + 3]",
            "shr {tmp}, 2",
            "dec {tmp}",
            "cmp {tmp:l}, 11",
            "cmc",
            "sbb {tmp}, 0",
            "imul {hash}, 13",
            "add {hash}, {tmp}",

            "movzx {tmp}, byte ptr [{cards} + 4]",
            "shr {tmp}, 2",
            "dec {tmp}",
            "cmp {tmp:l}, 11",
            "cmc",
            "sbb {tmp}, 0",
            "imul {hash}, 13",
            "add {hash}, {tmp}",

            cards = in(reg) cards.as_ptr(),
            hash = inout(reg) hash,
            tmp = out(reg) _,
            options(nostack),
        );
    }
    hash as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 32-bit positional hash
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let h = Hand::default().init(hand!("2s","2h","2c","2d","Kc"));
/// assert_eq!(187204216, ojh_positional_32c(&h[..]).unwrap());
/// ```
pub fn ojh_positional_32c(cards: &[Card]) -> Result<u32> {
    let mut max = 5;
    let mut h: u32 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(Error::HashDomain(String::from("5 cards max")));
        }
        h <<= 6;
        h += (0x3F & c.0) as u32;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 32-bit positional rank hash
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let h = Hand::default().init(hand!("2s","2h","2c","2d","Kc"));
/// assert_eq!(0x0002_222E, ojh_positional_32cs(&h[..]).unwrap());
/// ```
pub fn ojh_positional_32cs(cards: &[Card]) -> Result<u32>{
    let mut max = 8;
    let mut h: u32 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(Error::HashDomain(String::from("8 ranks max")));
        }
        h <<= 4;
        h += 0x0F & (c.rank() as u32);
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 64-bit positional hash
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let h = Hand::default().init(hand!("Qc","Qd","Qs","Tc","Th"));
/// assert_eq!(886536746, ojh_positional_64c(&h[..]).unwrap());
/// ```
pub fn ojh_positional_64c(cards: &[Card]) -> Result<u64> {
    let mut max = 10;
    let mut h: u64 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(Error::HashDomain(String::from("10 cards max")));
        }
        h <<= 6;
        h += (0x3F & c.0) as u64;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Positional_Hash) | 64-bit positional rank hash
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let h = Hand::default().init(hand!("Qc","Qd","Qs","Tc","Th"));
/// assert_eq!(0x000D_DDAA, ojh_positional_64cs(&h[..]).unwrap());
/// ```
pub fn ojh_positional_64cs(cards: &[Card]) -> Result<u64> {
    let mut max = 16;
    let mut h: u64 = 0;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(Error::HashDomain(String::from("16 ranks max")));
        }
        h <<= 4;
        h += (0x0F & (c.rank() as u8)) as u64;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Bitfield_Hash) | 64-bit bitfield hash
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let h = Hand::default().init(hand!("Jc","9c","7c","5c","3c"));
/// assert_eq!(0x1010_1010_1000, ojh_bitfield_64co(&h[..]).unwrap());
/// ```
pub fn ojh_bitfield_64co(cards: &[Card]) -> Result<u64> {
    let mut h: u64 = 0;

    for c in cards {
        #[cfg(debug_assertions)]
        if 0 != (h & (1 << (c.0 as u64))) {
            return Err(Error::HashDomain(String::from("duplicate card")));
        }
        h |= 1 << (c.0 as u64);
    }
    Ok(h)
}

#[cfg(not(target_arch = "x86_64"))]
pub fn ojh_mp5_english(cards: &[Card]) -> u32 {
    let mut bf: u64 = 0;
    for c in cards {
        bf |= 1 << c.0;
    }
    // make ranks contiguous
    bf >>= 8;
    bf = (bf & 0x00FF_FFFF_FFFF) | ((bf & 0x00FF_F000_0000_0000) >> 4);

    let mut hash: u64 = oj_binomial(52, 5);
    let mut mask = 0x0008_0000_0000_0000;
    let mut count = 1;

    for pos in 0..52 {
        if 0 != (bf & mask) {
            hash -= oj_binomial(pos, count);
            count += 1;
            if count > 5 { break; }
        }
        mask >>= 1;
    }
    debug_assert!(hash <= 2598960);
    hash as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | Convert bitfield to MPH
///
/// Start by creating a bitfield in the same manner as ojh_bitfield_64co().
/// This is essentially a radix sort of the cards. Then compute the index of
/// particular set of bits for minimal perfect hash. Note: doesn't check for
/// errors. Whatever 5 bytes happen to be at the address you pass, that's
/// what it will use, and will return a value.
/// Requires English deck, 5 cards.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let d = Deck::new(DeckType::English);
/// let h = d.new_hand().init(hand!("2s","2d","2c","2h","3c"));
/// assert_eq!(1, ojh_mp5_english(&h[..]));
/// ```
#[cfg(target_arch = "x86_64")]
pub fn ojh_mp5_english(cards: &[Card]) -> u32 {
    let mut hash: u64 = 2598960;
    debug_assert!(5 == cards.len());
    debug_assert!(cards[0].0 > 7 && cards[0].0 < 64);
    debug_assert!(cards[1].0 > 7 && cards[1].0 < 64);
    debug_assert!(cards[2].0 > 7 && cards[2].0 < 64);
    debug_assert!(cards[3].0 > 7 && cards[3].0 < 64);
    debug_assert!(cards[4].0 > 7 && cards[4].0 < 64);

    unsafe {
        asm!(
            "xor {bf}, {bf}",
            "movzx {mask}, byte ptr [{cards}]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 1]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 2]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 3]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 4]",
            "bts {bf}, {mask}",

            "shr {bf}, 8",
            "mov {tmp}, {bf}",
            "xor {mask}, {mask}",
            "mov {mask}, 0x000000FFFFFFFFFF",
            "and {bf}, {mask}",
            "mov {mask}, 0x00FFF00000000000",
            "and {tmp}, {mask}",
            "shr {tmp}, 4",
            "or {bf}, {tmp}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "sub {hash}, {tmp}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 16",
            "sub {hash}, {mask}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 24",
            "sub {hash}, {mask}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 32",
            "sbb {hash}, {mask}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 40",
            "sbb {hash}, {mask}",

            cards = in(reg) cards.as_ptr(),
            coefs = in(reg) COEFFICIENTS.as_ptr(),
            hash = inout(reg) hash,
            tmp = out(reg) _,
            bf = out(reg) _,
            mask = out(reg) _,
            options(nostack),
        );
        hash as u32
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn ojh_mp7_english(cards: &[Card]) -> u32 {
    let mut bf: u64 = 0;
    for c in cards {
        bf |= 1 << c.0;
    }
    // make ranks contiguous
    bf >>= 8;
    bf = (bf & 0x00FF_FFFF_FFFF) | ((bf & 0x00FF_F000_0000_0000) >> 4);

    let mut hash: u64 = oj_binomial(52, 7);
    let mut mask = 0x0008_0000_0000_0000;
    let mut count = 1;

    for pos in 0..52 {
        if 0 != (bf & mask) {
            hash -= oj_binomial(pos, count);
            count += 1;
            if count > 7 { break; }
        }
        mask >>= 1;
    }
    debug_assert!(hash <= 133784560);
    hash as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | Convert bitfield to MPH
/// 7-card minimal perfect hash for English deck
#[cfg(target_arch = "x86_64")]
pub fn ojh_mp7_english(cards: &[Card]) -> u32 {
    let mut hash: u64 = 133784560;
    debug_assert!(7 == cards.len());

    unsafe {
        asm!(
            "xor {bf}, {bf}",
            "movzx {mask}, byte ptr [{cards}]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 1]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 2]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 3]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 4]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 5]",
            "bts {bf}, {mask}",
            "movzx {mask}, byte ptr [{cards} + 6]",
            "bts {bf}, {mask}",

            "shr {bf}, 8",
            "mov {tmp}, {bf}",
            "xor {mask}, {mask}",
            "mov {mask}, 0x000000FFFFFFFFFF",
            "and {bf}, {mask}",
            "mov {mask}, 0x00FFF00000000000",
            "and {tmp}, {mask}",
            "shr {tmp}, 4",
            "or {bf}, {tmp}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "sub {hash}, {tmp}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 16]",
            "sub {hash}, {mask}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 24]",
            "sub {hash}, {mask}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 32]",
            "sbb {hash}, {mask}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 40]",
            "sbb {hash}, {mask}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 48]",
            "sbb {hash}, {mask}",

            "bsr {mask}, {bf}",
            "btr {bf}, {mask}",
            "mov {tmp}, 51",
            "sub {tmp}, {mask}",
            "shl {tmp}, 9",
            "mov {mask}, [{coefs} + {tmp} + 56]",
            "sbb {hash}, {mask}",

            cards = in(reg) cards.as_ptr(),
            coefs = in(reg) COEFFICIENTS.as_ptr(),
            hash = inout(reg) hash,
            tmp = out(reg) _,
            bf = out(reg) _,
            mask = out(reg) _,
            options(nostack),
        );
        hash as u32
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Perfect_Hash) | Convert bitfield to MPH
/// Given a bitfield with exactly 5 bits set, return the lexicographic
/// index of that particular set of bits for minimal perfect hash.
/// Requires Stripped deck, 5 cards.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let d = Deck::new(DeckType::Stripped);
/// let h = d.new_hand().init(hand!("7s","7d","7c","7h","8c"));
/// let b = ojh_bitfield_64co(&h[..]).unwrap();
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
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let d = Deck::new(DeckType::Low);
/// let h = d.new_hand().init(hand!("Ac","Ad","Ah","As","2c"));
/// let b = ojh_bitfield_64co(&h[..]).unwrap();
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
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let d = Deck::new(DeckType::English);
/// let h = d.new_hand().init(hand!("2c","2d","2h","2s"));
/// let b = ojh_bitfield_64co(&h[..]).unwrap();
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
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let d = Deck::new(DeckType::Low);
/// let h = d.new_hand().init(hand!("Ac","Ad","Ah","As"));
/// let b = ojh_bitfield_64co(&h[..]).unwrap();
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
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let h = Hand::default().init(hand!("Kc","Qd","Jh","Ts","9h"));
/// assert_eq!(117144257, ojh_prime_32cos(&h[..]).unwrap());
/// ```
pub fn ojh_prime_32cos(cards: &[Card]) -> Result<u32> {
    let mut max = 5;
    let mut h: u32 = 1;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(Error::HashDomain(String::from("5 cards max")));
        }
        h *= PRIMES[0x0F & (c.rank() as usize)];
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Prime_Hash) | 64-bit prime hash
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let h = Hand::default().init(hand!("Kc","Qd","Jh","Ts","9h"));
/// assert_eq!(529321587761, ojh_prime_64co(&h[..]).unwrap());
/// ```
pub fn ojh_prime_64co(cards: &[Card]) -> Result<u64> {
    let mut max = 7;
    let mut h: u64 = 1;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(Error::HashDomain(String::from("7 cards max")));
        }
        h *= PRIMES[0x3F & (c.0 as usize)] as u64;
    }
    Ok(h)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Prime_Hash) | 64-bit prime rank hash
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::cards::hashes::*;
///
/// let d = Deck::new(DeckType::Low);
/// let h = d.new_hand().init(hand!("Ac","2d","3h","4s","5h"));
/// assert_eq!(5 * 7 * 11 * 13 * 17, ojh_prime_64cos(&h[..]).unwrap());
/// ```
pub fn ojh_prime_64cos(cards: &[Card]) -> Result<u64> {
    let mut max = 10;
    let mut h: u64 = 1;

    for c in cards {
        max -= 1;
        if max < 0 {
            return Err(Error::HashDomain(String::from("10 cards max")));
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
    use crate::cards::hand::Hand;

    #[test]
    fn test_hashes() -> Result<()> {
        let h1 = Hand::default().init(
            hand!("Ac","Kc","Qc","Jc","Tc"));

        assert_eq!(309437067, ojh_fnv_32(&h1[..])?);
        assert_eq!(18055603845018456331, ojh_fnv_64(&h1[..])?);
        assert_eq!(1021528872, ojh_positional_32c(&h1[..])?);
        assert_eq!(1043898, ojh_positional_32cs(&h1[..])?);
        assert_eq!(1021528872, ojh_positional_64c(&h1[..])?);
        assert_eq!(1043898, ojh_positional_64cs(&h1[..])?);
        assert_eq!(1229501389969817600, ojh_bitfield_64co(&h1[..])?);
        assert_eq!(222951973, ojh_prime_32cos(&h1[..])?);
        assert_eq!(717864180907, ojh_prime_64co(&h1[..])?);
        assert_eq!(222951973, ojh_prime_64cos(&h1[..])?);

        Ok(())
    }
}
