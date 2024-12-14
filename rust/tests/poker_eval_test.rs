//@ tests/poker_eval_test.rs

#[cfg(feature = "serde")]
use serde::Deserialize;

use onejoker::prelude::*;

#[cfg(feature = "serde")]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
struct PokerHand(String, u16, u16, u16, u16, u16);

/// JSON file structure
#[cfg(feature = "serde")]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
struct PokerHandDataFile(Vec<PokerHand>);

#[test]
#[cfg(feature = "serde")]
fn test_poker_hand_file() -> OjResult<()> {
    use std::fs::File;
    use std::io::BufReader;
    use onejoker::utils::Random;

    let file = File::open("../data/json/poker_hands_100k.jsonc")?;
    let mut reader = BufReader::new(file);
    let data: PokerHandDataFile = serde_json5::from_reader(&mut reader)?;

    let deck = Deck::new_by_name("poker");
    let mut rng = Random::new();

    for i in 0..data.0.len() {
        let j = rng.uniform32(data.0.len());
        let irow = &data.0[i];
        let jrow = &data.0[j];
        let ihand = deck.new_hand().init(card_parse(&irow.0));
        let jhand = deck.new_hand().init(card_parse(&jrow.0));

        let high = Scale::HighHand;
        let ival = high.value(&ihand);
        let jval = high.value(&jhand);
        // println!("{:6} {} {:4} {:#X}", i, ihand, irow.1, ival.value());
        // println!("{:6} {} {:4} {:#X}", j, jhand, jrow.1, jval.value());

        #[cfg(feature = "high-hand-tables")]
        assert!(ival as u16 == irow.1 + 13);

        if ival < jval {
            assert!(irow.1 < jrow.1);
            assert!(high.value(&ihand) < high.value(&jhand));
        } else if ival > jval {
            assert!(irow.1 > jrow.1);
            assert!(high.value(&ihand) > high.value(&jhand));
        } else {
            assert!(irow.1 == jrow.1);
            assert!(high.value(&ihand) == high.value(&jhand));
        }
        let d27 = Scale::DeuceToSeven;
        let ival = d27.value(&ihand);
        let jval = d27.value(&jhand);

        #[cfg(feature = "deuce-to-seven-tables")]
        assert!(ival as u16 == irow.2);

        // println!("{:6} {} {:4} {:#X}", i, ihand, irow.2, ival.value());
        // println!("{:6} {} {:4} {:#X}", j, jhand, jrow.2, jval.value());

        if ival < jval {
            assert!(irow.2 < jrow.2);
            assert!(d27.value(&ihand) < d27.value(&jhand));
        } else if ival > jval {
            assert!(irow.2 > jrow.2);
            assert!(d27.value(&ihand) > d27.value(&jhand));
        } else {
            assert!(irow.2 == jrow.2);
            assert!(d27.value(&ihand) == d27.value(&jhand));
        }
        let ihand = ihand.convert_decktype(DeckType::Low);
        let jhand = jhand.convert_decktype(DeckType::Low);

        let a25 = Scale::AceToFive;
        let ival = a25.value(&ihand);
        let jval = a25.value(&jhand);

        #[cfg(feature = "ace-to-five-tables")]
        assert!(ival as u16 == irow.3);

        if ival < jval {
            assert!(irow.3 < jrow.3);
            assert!(a25.value(&ihand) < a25.value(&jhand));
        } else if ival > jval {
            assert!(irow.3 > jrow.3);
            assert!(a25.value(&ihand) > a25.value(&jhand));
        } else {
            assert!(irow.3 == jrow.3);
            assert!(a25.value(&ihand) == a25.value(&jhand));
        }

        let a26 = Scale::AceToSix;
        let ival = a26.value(&ihand);
        let jval = a26.value(&jhand);

        #[cfg(feature = "deuce-to-seven-tables")]
        assert!(ival as u16 == irow.4);

        if ival < jval {
            assert!(irow.4 < jrow.4);
            assert!(a26.value(&ihand) < a26.value(&jhand));
        } else if ival > jval {
            assert!(irow.4 > jrow.4);
            assert!(a26.value(&ihand) > a26.value(&jhand));
        } else {
            assert!(irow.4 == jrow.4);
            assert!(a26.value(&ihand) == a26.value(&jhand));
        }

        let bad = Scale::Badugi;
        let ival = bad.value(&ihand);
        let jval = bad.value(&jhand);

        #[cfg(feature = "badugi-tables")]
        assert!(ival as u16 == irow.5);

        if ival < jval {
            assert!(irow.5 < jrow.5);
            assert!(bad.value(&ihand) < bad.value(&jhand));
        } else if ival > jval {
            assert!(irow.5 > jrow.5);
            assert!(bad.value(&ihand) > bad.value(&jhand));
        } else {
            assert!(irow.5 == jrow.5);
            assert!(bad.value(&ihand) == bad.value(&jhand));
        }
    }
    Ok(())
}

#[cfg(feature = "serde")]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[allow(unused)]
struct SevenCardHand(String, u16, String);

/// JSON file structure
#[cfg(feature = "serde")]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
struct SevenCardHandDataFile(Vec<SevenCardHand>);

#[test]
#[cfg(feature = "serde")]
fn test_seven_card_hand_file() -> OjResult<()> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open("../data/json/seven_card_hands_100k.jsonc")?;
    let mut reader = BufReader::new(file);
    let data: SevenCardHandDataFile = serde_json5::from_reader(&mut reader)?;

    let deck = Deck::new_by_name("poker");

    for i in 0..data.0.len() {
        let irow = &data.0[i];
        let ihand = deck.new_hand().init(card_parse(&irow.0));

        #[cfg(feature = "high-hand-tables")]
        let ec = irow.1;

        let ohand = irow.2.clone();
        let v= Scale::HighHand.value(&ihand);
        let desc = Scale::HighHand.description(&ihand, v);

        #[cfg(feature = "high-hand-tables")]
        assert_eq!(v as u16, ec + 13);

        assert_eq!(desc.hand_to_string(), ohand);
    }
    Ok(())
}

#[test]
fn test_no_json() -> OjResult<()> {
    Ok(())
}
