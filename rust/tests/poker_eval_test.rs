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
    use onejoker::utils::oj_shuffle;
    use onejoker::utils::oj_rand_range;

    let file = File::open("../data/json/poker_hands_100k.jsonc")?;
    let mut reader = BufReader::new(file);
    let data: PokerHandDataFile = serde_json5::from_reader(&mut reader)?;

    let deck = Deck::new_by_name("poker");

    for i in 0..data.0.len() {
        let j = oj_rand_range(data.0.len());
        let irow = &data.0[i];
        let jrow = &data.0[j];
        let ihand = deck.new_hand().init(card_parse(&irow.0));
        let jhand = deck.new_hand().init(card_parse(&jrow.0));

        let high = Scale::HighHand;
        let ival = high.eval(&ihand)?;
        let jval = high.eval(&jhand)?;
        // println!("{:6} {} {:4} {:#X}", i, ihand, irow.1, ival.value());
        // println!("{:6} {} {:4} {:#X}", j, jhand, jrow.1, jval.value());

        #[cfg(feature = "high-hand-tables")]
        assert!(ival.value() as u16 == irow.1);

        if ival < jval {
            assert!(irow.1 < jrow.1);
            assert!(high.eval_quick(&ihand) < high.eval_quick(&jhand));
        } else if ival > jval {
            assert!(irow.1 > jrow.1);
            assert!(high.eval_quick(&ihand) > high.eval_quick(&jhand));
        } else {
            assert!(irow.1 == jrow.1);
            assert!(high.eval_quick(&ihand) == high.eval_quick(&jhand));
        }
        let d27 = Scale::DeuceToSeven;
        let ival = d27.eval(&ihand)?;
        let jval = d27.eval(&jhand)?;

        #[cfg(feature = "deuce-to-seven-tables")]
        assert!(ival.value as u16 == irow.2);

        // println!("{:6} {} {:4} {:#X}", i, ihand, irow.2, ival.value());
        // println!("{:6} {} {:4} {:#X}", j, jhand, jrow.2, jval.value());

        if ival < jval {
            assert!(irow.2 < jrow.2);
            assert!(d27.eval_quick(&ihand) < d27.eval_quick(&jhand));
        } else if ival > jval {
            assert!(irow.2 > jrow.2);
            assert!(d27.eval_quick(&ihand) > d27.eval_quick(&jhand));
        } else {
            assert!(irow.2 == jrow.2);
            assert!(d27.eval_quick(&ihand) == d27.eval_quick(&jhand));
        }
        let ihand = ihand.convert_decktype(DeckType::Low);
        let jhand = jhand.convert_decktype(DeckType::Low);

        let a25 = Scale::AceToFive;
        let ival = a25.eval(&ihand)?;
        let jval = a25.eval(&jhand)?;

        #[cfg(feature = "ace-to-five-tables")]
        assert!(ival.value as u16 == irow.3);

        if ival < jval {
            assert!(irow.3 < jrow.3);
            assert!(a25.eval_quick(&ihand) < a25.eval_quick(&jhand));
        } else if ival > jval {
            assert!(irow.3 > jrow.3);
            assert!(a25.eval_quick(&ihand) > a25.eval_quick(&jhand));
        } else {
            assert!(irow.3 == jrow.3);
            assert!(a25.eval_quick(&ihand) == a25.eval_quick(&jhand));
        }

        let a26 = Scale::AceToSix;
        let ival = a26.eval(&ihand)?;
        let jval = a26.eval(&jhand)?;

        #[cfg(feature = "ace-to-six-tables")]
        assert!(ival.value as u16 == irow.4);

        if ival < jval {
            assert!(irow.4 < jrow.4);
            assert!(a26.eval_quick(&ihand) < a26.eval_quick(&jhand));
        } else if ival > jval {
            assert!(irow.4 > jrow.4);
            assert!(a26.eval_quick(&ihand) > a26.eval_quick(&jhand));
        } else {
            assert!(irow.4 == jrow.4);
            assert!(a26.eval_quick(&ihand) == a26.eval_quick(&jhand));
        }

        let bad = Scale::Badugi;
        let ival = bad.eval(&ihand)?;
        let jval = bad.eval(&jhand)?;

        #[cfg(feature = "badugi-tables")]
        assert!(ival.value as u16 == irow.5);

        if ival < jval {
            assert!(irow.5 < jrow.5);
            assert!(bad.eval_quick(&ihand) < bad.eval_quick(&jhand));
        } else if ival > jval {
            assert!(irow.5 > jrow.5);
            assert!(bad.eval_quick(&ihand) > bad.eval_quick(&jhand));
        } else {
            assert!(irow.5 == jrow.5);
            assert!(bad.eval_quick(&ihand) == bad.eval_quick(&jhand));
        }
    }
    Ok(())
}

#[test]
fn test_no_json() -> OjResult<()> {
    Ok(())
}
