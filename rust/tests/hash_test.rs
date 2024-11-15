
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
    let mut ret = *h;
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
    let mut ret = *h;
    for i in 0..h.len() {
        let mut s = ret[i].suit() as u8;
        if s > 0 {
            s += 1;
            if s > 4 {
                s = 1;
            }
            ret[i] = Card::from_rank_suit(h[i].rank(), Suit::from_u8(s));
        }
    }
    ret
}

#[test]
fn test_hash_data_file() -> Result<(), OjError> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open("../data/json/random_hands_100k.jsonc")?;
    let mut reader = BufReader::new(file);
    let data: RandomHandDataFile = serde_json5::from_reader(&mut reader)?;

    for i in 0..data.hands.len() {
        let mut deck =
            Deck::new_by_name(data.decks[data.hands[i].0 as usize - 1].as_str());
        deck.shuffle();

        let h1 = deck.new_hand().init(
            ojc_parse(data.hands[i].1.as_str()));
        let h2 = deck.new_hand().init(
            ojc_parse(h1.to_string().as_str()));

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

        if !deck.deck_type.dups_allowed() {
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

        if h1.len() > 10 { continue; }
        assert_eq!(ojh_positional_64c(h1.as_slice())?, ojh_positional_64c(h2.as_slice())?);
        assert_eq!(ojh_positional_64c(h1.as_slice())? == ojh_positional_64c(h3.as_slice())?,
            h1.equals(&h3));
        assert_eq!(ojh_positional_64c(h1.as_slice())? == ojh_positional_64c(h4.as_slice())?,
            h1.equals(&h4));

        assert_eq!(ojh_prime_64cos(h1.as_slice())?, ojh_prime_64cos(h2.as_slice())?);
        assert_eq!(ojh_prime_64cos(h1.as_slice())?, ojh_prime_64cos(h3.as_slice())?);
        assert_eq!(ojh_prime_64cos(h1.as_slice())?, ojh_prime_64cos(h4.as_slice())?);

        if h1.len() > 8 { continue; }
        assert_eq!(ojh_positional_32cs(h1.as_slice())?, ojh_positional_32cs(h2.as_slice())?);
        assert_eq!(ojh_positional_32cs(h1.as_slice())? == ojh_positional_32cs(h3.as_slice())?,
            ranks_identical(&h1, &h3));
        assert_eq!(ojh_positional_32cs(h1.as_slice())?, ojh_positional_32cs(h4.as_slice())?);

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

        assert_eq!(ojh_prime_32cos(h1.as_slice())?, ojh_prime_32cos(h2.as_slice())?);
        assert_eq!(ojh_prime_32cos(h1.as_slice())?, ojh_prime_32cos(h3.as_slice())?);
        assert_eq!(ojh_prime_32cos(h1.as_slice())?, ojh_prime_32cos(h4.as_slice())?);
    }
    Ok(())
}
