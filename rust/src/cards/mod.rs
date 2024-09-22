//@ cards/mod.rs
//@ Lee Daniel Crocker <lee@piclab.com>

//! # cards | [wiki](https://github.com/lcrocker/tspoker/wiki/Cards) | Non-game-specific card handling.

pub use crate::errors::*;
pub use crate::rand::*;

pub mod suit;
pub use suit::Suit;

pub mod rank;
pub use rank::Rank;

pub mod card;
pub use card::{ Card, cards_from_text };

pub mod master_deck;
pub use master_deck::MasterDeck;

// pub mod lists;
// pub use lists::{ Hand, Deck };
// pub use std::str::FromStr;

pub use card::{ WHITE_JOKER, BLACK_JOKER, JOKER,
    LOW_ACE_OF_CLUBS, LOW_ACE_OF_DIAMONDS, LOW_ACE_OF_HEARTS, LOW_ACE_OF_SPADES,
    DEUCE_OF_CLUBS, DEUCE_OF_DIAMONDS, DEUCE_OF_HEARTS, DEUCE_OF_SPADES,
    TREY_OF_CLUBS, TREY_OF_DIAMONDS, TREY_OF_HEARTS, TREY_OF_SPADES,
    FOUR_OF_CLUBS, FOUR_OF_DIAMONDS, FOUR_OF_HEARTS, FOUR_OF_SPADES,
    FIVE_OF_CLUBS, FIVE_OF_DIAMONDS, FIVE_OF_HEARTS, FIVE_OF_SPADES,
    SIX_OF_CLUBS, SIX_OF_DIAMONDS, SIX_OF_HEARTS, SIX_OF_SPADES,
    SEVEN_OF_CLUBS, SEVEN_OF_DIAMONDS, SEVEN_OF_HEARTS, SEVEN_OF_SPADES,
    EIGHT_OF_CLUBS, EIGHT_OF_DIAMONDS, EIGHT_OF_HEARTS, EIGHT_OF_SPADES,
    NINE_OF_CLUBS, NINE_OF_DIAMONDS, NINE_OF_HEARTS, NINE_OF_SPADES,
    TEN_OF_CLUBS, TEN_OF_DIAMONDS, TEN_OF_HEARTS, TEN_OF_SPADES,
    JACK_OF_CLUBS, JACK_OF_DIAMONDS, JACK_OF_HEARTS, JACK_OF_SPADES,
    QUEEN_OF_CLUBS, QUEEN_OF_DIAMONDS, QUEEN_OF_HEARTS, QUEEN_OF_SPADES,
    KING_OF_CLUBS, KING_OF_DIAMONDS, KING_OF_HEARTS, KING_OF_SPADES,
    ACE_OF_CLUBS, ACE_OF_DIAMONDS, ACE_OF_HEARTS, ACE_OF_SPADES,
    KNIGHT_OF_CLUBS, KNIGHT_OF_DIAMONDS, KNIGHT_OF_HEARTS, KNIGHT_OF_SPADES,
};

/// Standard Fisher-Yates shuffle using our own PRNG.
pub fn oj_shuffle(a: &mut [Card]) {
    if a.len() < 2 { return; }

    for i in 0..(a.len() - 1) {
        let j = i + (range_uniform((a.len() - i) as i32) as usize);
        if i != j { a.swap(i, j); }
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
            for i in (0..(a.len() / 2)).rev() {
                heapify(a, a.len(), i);
            }
            for i in (1..a.len()).rev() {
                a.swap(0, i);
                heapify(a, i, 0);
            }
        },
    }
}

fn heapify(a: &mut [Card], n: usize, i: usize) {
    let mut min = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && a[left] < a[min] { min = left; }
    if right < n && a[right] < a[min] { min = right; }

    if min != i {
        a.swap(i, min);
        heapify(a, n, min);
    }
}

pub fn oj_next_combination(a: &mut [i32], n: i32) -> bool {
    let k: i32 = a.len() as i32;

    for i in (0..k).rev() {
        if a[i as usize] < n - k + i + 1 {
            a[i as usize] += 1;
            for j in (i + 1)..k {
                a[j as usize] = a[(j - 1) as usize] + 1;
            }
            return true;
        }
    }
    false
}
