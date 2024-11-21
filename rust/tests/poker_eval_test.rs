//@ tests/poker_eval_test.rs
#[allow(dead_code)]

use serde::Deserialize;

use onejoker::prelude::*;
use onejoker::utils::oj_rand_range;

#[derive(Deserialize, Debug)]
struct PokerHand(String, u16, u16, u16, u16, u16);

/// JSON file structure
#[derive(Deserialize, Debug)]
struct PokerHandDataFile(Vec<PokerHand>);

#[test]
fn test_poker_hand_file() -> OjResult<()> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open("../data/json/poker_hands_100k.jsonc")?;
    let mut reader = BufReader::new(file);
    let data: PokerHandDataFile = serde_json5::from_reader(&mut reader)?;

    let high_deck = Deck::new_by_name("poker");
    let low_deck = Deck::new_by_name("low");

    for i in 0..data.0.len() {
        let j = oj_rand_range(data.0.len());
        let irow = &data.0[i];
        let jrow = &data.0[j];
        let ihand = high_deck.new_hand().init(card_parse(&irow.0));
        let jhand = high_deck.new_hand().init(card_parse(&jrow.0));

        let high = HandScale::HighHand;
        let mut ival = high.eval(&ihand)?;
        let mut jval = high.eval(&jhand)?;
        // println!("{:6} {} {:4} {:#X}", i, ihand, irow.1, ival.value());
        // println!("{:6} {} {:4} {:#X}", j, jhand, jrow.1, jval.value());

        #[cfg(feature = "high-hand-tables")]
        assert!(ival.value() as u16 == irow.1);

        if ival.value() < jval.value() {
            assert!(irow.1 < jrow.1);
            assert!(high.eval_quick(&ihand) < high.eval_quick(&jhand));
        } else if ival.value() > jval.value() {
            assert!(irow.1 > jrow.1);
            assert!(high.eval_quick(&ihand) > high.eval_quick(&jhand));
        } else {
            assert!(irow.1 == jrow.1);
            assert!(high.eval_quick(&ihand) == high.eval_quick(&jhand));
        }
        // let d27 = HandScale::DeuceToSeven;
        // ival = d27.eval(&ihand)?;
        // jval = d27.eval(&jhand)?;

        // #[cfg(feature = "deuce-to-seven-tables")]
        // assert!(ival.value as u16 == irow.2);

        // if ival < jval {
        //     assert!(irow.2 < jrow.2);
        //     assert!(d27.eval_quick(&ihand) < d27.eval_quick(&jhand));
        // } else if ival > jval {
        //     assert!(irow.2 > jrow.2);
        //     assert!(d27.eval_quick(&ihand) > d27.eval_quick(&jhand));
        // } else {
        //     assert!(irow.2 == jrow.2);
        //     assert!(d27.eval_quick(&ihand) == d27.eval_quick(&jhand));
        // }
        // let ihand = low_deck.new_hand().init(ojc_parse(&irow.0));
        // let jhand = low_deck.new_hand().init(ojc_parse(&jrow.0));

        // let a25 = HandScale::AceToFive;
        // ival = a25.eval(&ihand)?;
        // jval = a25.eval(&jhand)?;

        // #[cfg(feature = "ace-to-five-tables")]
        // assert!(ival.value as u16 == irow.3);

        // if ival < jval {
        //     assert!(irow.3 < jrow.3);
        //     assert!(a25.eval_quick(&ihand) < a25.eval_quick(&jhand));
        // } else if ival > jval {
        //     assert!(irow.3 > jrow.3);
        //     assert!(a25.eval_quick(&ihand) > a25.eval_quick(&jhand));
        // } else {
        //     assert!(irow.3 == jrow.3);
        //     assert!(a25.eval_quick(&ihand) == a25.eval_quick(&jhand));
        // }

        // let a26 = HandScale::AceToSix;
        // ival = a26.eval(&ihand)?;
        // jval = a26.eval(&jhand)?;

        // #[cfg(feature = "ace-to-six-tables")]
        // assert!(ival.value as u16 == irow.4);

        // if ival < jval {
        //     assert!(irow.4 < jrow.4);
        //     assert!(a26.eval_quick(&ihand) < a26.eval_quick(&jhand));
        // } else if ival > jval {
        //     assert!(irow.4 > jrow.4);
        //     assert!(a26.eval_quick(&ihand) > a26.eval_quick(&jhand));
        // } else {
        //     assert!(irow.4 == jrow.4);
        //     assert!(a26.eval_quick(&ihand) == a26.eval_quick(&jhand));
        // }

        // let bad = HandScale::Badugi;
        // ival = bad.eval(&ihand)?;
        // jval = bad.eval(&jhand)?;

        // #[cfg(feature = "badugi-tables")]
        // assert!(ival.value as u16 == irow.5);

        // if ival < jval {
        //     assert!(irow.5 < jrow.5);
        //     assert!(bad.eval_quick(&ihand) < bad.eval_quick(&jhand));
        // } else if ival > jval {
        //     assert!(irow.5 > jrow.5);
        //     assert!(bad.eval_quick(&ihand) > bad.eval_quick(&jhand));
        // } else {
        //     assert!(irow.5 == jrow.5);
        //     assert!(bad.eval_quick(&ihand) == bad.eval_quick(&jhand));
        // }
    }
    Ok(())
}
