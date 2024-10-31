//@ tests/hash_test.rs

use std::fs;
use serde::Deserialize;
use onejoker::*;

type HandData = (i32, String, u64);

/// JSON file structure
#[derive(Deserialize, Debug)]
struct RandomHandDataFile {
    decks: Vec<String>,
    hands: Vec<HandData>,
}

fn ranks_identical(h1: &Hand, h2: &Hand) -> bool {
    if h1.len() != h2.len() {
        return false;
    }
    for i in 0..h1.len() {
        if h1[i].rank() != h2[i].rank() {
            return false;
        }
    }
    true
}

fn reorder(h: &Hand) -> Hand {
    let mut ret = h.clone();
    if h.len() < 2 {
        return ret;
    }
    let mut max = 10;
    loop {
        max -= 1;
        if max <= 0 {
            break;
        }
        oj_shuffle(ret.as_mut_slice());
        if ! h.equals(&ret) {
            break;
        }
    }
    ret
}

fn resuit(h: &Hand) -> Hand {
    let mut ret = h.clone();
    for i in 0..h.len() {
        let mut s = ret[i].suit() as i32;
        if s > 0 {
            s += 1;
            if s > 4 {
                s = 1;
            }
            ret[i] = Card::from_rank_suit(h[i].rank(), Suit::from_i32(s));
        }
    }
    ret
}


#[test]
fn test_hash_data_file() -> Result<(), OjError> {
    let text = fs::read_to_string("../data/json/random_hands_100k.jsonc")?;
    let data = json5::from_str::<RandomHandDataFile>(&text[..])?;

    for i in 0..data.hands.len() {
        let mut deck =
            Deck::new(data.decks[data.hands[i].0 as usize - 1].as_str());
        deck.shuffle();

        let mut h1 = deck.new_hand();
        let mut h2 = deck.new_hand();
        h1.push_n(parse_cards(data.hands[i].1.as_str()));
        h2.push_n(parse_cards(h1.to_string().as_str()));

        let h3 = reorder(&h1);
        let h4 = resuit(&h1);

        assert!(h1.equals(&h2));
        assert!(h1.is_equivalent_to(&h3));

        assert_eq!(ojh_fnv_32(h1.as_slice())?, ojh_fnv_32(h2.as_slice())?);
        assert_eq!(ojh_fnv_32(h1.as_slice())? == ojh_fnv_32(h3.as_slice())?,
            h1.equals(&h3));
        assert_eq!(ojh_fnv_32(h1.as_slice())? == ojh_fnv_32(h4.as_slice())?,
            h1.equals(&h4));

        assert_eq!(ojh_fnv_64(h1.as_slice())?, ojh_fnv_64(h2.as_slice())?);
        assert_eq!(ojh_fnv_64(h1.as_slice())? == ojh_fnv_64(h3.as_slice())?,
            h1.equals(&h3));
        assert_eq!(ojh_fnv_64(h1.as_slice())? == ojh_fnv_64(h4.as_slice())?,
            h1.equals(&h4));

        if !deck.master.dups_allowed {
            assert_eq!(ojh_bitfield_64co(h1.as_slice())?, ojh_bitfield_64co(h2.as_slice())?);
            assert_eq!(ojh_bitfield_64co(h1.as_slice())?, ojh_bitfield_64co(h3.as_slice())?);
            assert_eq!(ojh_bitfield_64co(h1.as_slice())? == ojh_bitfield_64co(h4.as_slice())?,
                h1.is_equivalent_to(&h4));
        }
        if h1.len() > 16 { continue; }
        assert_eq!(ojh_positional_64cs(h1.as_slice())?, ojh_positional_64cs(h2.as_slice())?);
        assert_eq!(ojh_positional_64cs(h1.as_slice())? == ojh_positional_64cs(h3.as_slice())?,
            ranks_identical(&h1, &h3));
        assert_eq!(ojh_positional_64cs(h1.as_slice())?, ojh_positional_64cs(h4.as_slice())?);

        assert_eq!(ojh_positional_64cr(&h1.ranks())?, ojh_positional_64cr(&h2.ranks())?);
        assert_eq!(ojh_positional_64cr(&h1.ranks())? == ojh_positional_64cr(&h3.ranks())?,
            ranks_identical(&h1, &h3));
        assert_eq!(ojh_positional_64cr(&h1.ranks())?, ojh_positional_64cr(&h4.ranks())?);

        if h1.len() > 10 { continue; }
        assert_eq!(ojh_positional_64c(h1.as_slice())?, ojh_positional_64c(h2.as_slice())?);
        assert_eq!(ojh_positional_64c(h1.as_slice())? == ojh_positional_64c(h3.as_slice())?,
            h1.equals(&h3));
        assert_eq!(ojh_positional_64c(h1.as_slice())? == ojh_positional_64c(h4.as_slice())?,
            h1.equals(&h4));

        assert_eq!(ojh_prime_64cor(&h1.ranks())?, ojh_prime_64cor(&h2.ranks())?);
        assert_eq!(ojh_prime_64cor(&h1.ranks())?, ojh_prime_64cor(&h3.ranks())?);
        assert_eq!(ojh_prime_64cor(&h1.ranks())?, ojh_prime_64cor(&h4.ranks())?);

        if h1.len() > 8 { continue; }
        assert_eq!(ojh_positional_32cs(h1.as_slice())?, ojh_positional_32cs(h2.as_slice())?);
        assert_eq!(ojh_positional_32cs(h1.as_slice())? == ojh_positional_32cs(h3.as_slice())?,
            ranks_identical(&h1, &h3));
        assert_eq!(ojh_positional_32cs(h1.as_slice())?, ojh_positional_32cs(h4.as_slice())?);

        assert_eq!(ojh_positional_32cr(&h1.ranks())?, ojh_positional_32cr(&h2.ranks())?);
        assert_eq!(ojh_positional_32cr(&h1.ranks())? == ojh_positional_32cr(&h3.ranks())?,
            ranks_identical(&h1, &h3));
        assert_eq!(ojh_positional_32cr(&h1.ranks())?, ojh_positional_32cr(&h4.ranks())?);

        if h1.len() > 7 { continue; }
        assert_eq!(ojh_prime_64co(h1.as_slice())?, ojh_prime_64co(h2.as_slice())?);
        assert_eq!(ojh_prime_64co(h1.as_slice())?, ojh_prime_64co(h3.as_slice())?);
        assert_eq!(ojh_prime_64co(h1.as_slice())? == ojh_prime_64co(h4.as_slice())?,
            h1.is_equivalent_to(&h4));

        if h1.len() > 5 { continue; }
        assert_eq!(ojh_positional_32c(h1.as_slice())?, ojh_positional_32c(h2.as_slice())?);
        assert_eq!(ojh_positional_32c(h1.as_slice())? == ojh_positional_32c(h3.as_slice())?,
            h1.equals(&h3));
        assert_eq!(ojh_positional_32c(h1.as_slice())? == ojh_positional_32c(h4.as_slice())?,
            h1.equals(&h4));

        assert_eq!(ojh_prime_32cor(&h1.ranks())?, ojh_prime_32cor(&h2.ranks())?);
        assert_eq!(ojh_prime_32cor(&h1.ranks())?, ojh_prime_32cor(&h3.ranks())?);
        assert_eq!(ojh_prime_32cor(&h1.ranks())?, ojh_prime_32cor(&h4.ranks())?);
    }
    Ok(())
}
