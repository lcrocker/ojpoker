//! # hash | [wiki](https://github.com/lcrocker/ojpoker/wiki/Hashes) | Hash functions for cards with various properties

use crate::cards::utils::*;
use crate::cards::stack::*;

pub fn card_hash_fnv_32<T>(cards: &T) -> u32
where T: CardStackTrait {
    let mut h: u32 = 0x811C9DC5;

    for i in 0..cards.len() {
        h ^= cards.card_at(i).unwrap().0 as u32;
        h = h.wrapping_mul(0x01000193);
    }
    h
}

pub fn card_hash_base64_64<T>(cards: &T) -> u64
where T: CardStackTrait {
    assert!(cards.len() <= 10);
    let mut h: u64 = 0;

    for i in 0..cards.len() {
        h <<= 6;
        h |= (0x3F & cards.card_at(i).unwrap().0) as u64;
    }
    h
}

pub fn card_hash_base64_u_64<T>(cards: &T) -> u64
where T: CardStackTrait {
    assert!(cards.len() <= 10);

    let mut h: u64 = 0;
    let mut s = cards.to_vec();
    oj_sort(&mut s);

    for i in 0..s.len() {
        h <<= 6;
        h |= (0x3F & s[i].0) as u64;
    }
    h
}

pub fn card_hash_base16_ru_64<T>(cards: &T) -> u64
where T: CardStackTrait {
    assert!(cards.len() <= 16);

    let mut h: u64 = 0;
    let mut s = cards.to_vec();
    oj_sort(&mut s);

    for i in 0..s.len() {
        h <<= 4;
        h |= (0x0F & (s[i].0 >> 2)) as u64;
    }
    h
}

pub fn card_hash_bit_field_u_64<T>(cards: &T) -> u64
where T: CardStackTrait {
    let mut h: u64 = 0;

    for i in 0..cards.len() {
        h |= 1 << (0x3F & cards.card_at(i).unwrap().0);
    }
    h
}

const PRIMES: [u64; 64] = [
    3, 5, 7, 11, 13, 17, 19, 23,
    29, 31, 37, 41, 43, 47, 53, 59,
    61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137,
    139, 149, 151, 157, 163, 167, 173, 179,
    181, 191, 193, 197, 199, 211, 223, 227,
    229, 233, 239, 241, 251, 257, 263, 269,
    271, 277, 281, 283, 293, 307, 311, 313,
];

pub fn card_hash_prime_u_64<T>(cards: &T) -> u64
where T: CardStackTrait {
    let mut h: u64 = 1;

    for i in 0..cards.len() {
        h *= PRIMES[(0x3F & cards.card_at(i).unwrap().0) as usize];
    }
    h
}

pub fn card_hash_prime_ru_64<T>(cards: &T) -> u64
where T: CardStackTrait {
    let mut h: u64 = 1;

    for i in 0..cards.len() {
        h *= PRIMES[(0x0F & (cards.card_at(i).unwrap().0 >> 2)) as usize];
    }
    h
}
