//@ cards/stack.rs
//! # stack | [wiki](https://github.com/lcrocker/ojpoker/wiki/CardStack) | A simple LIFO stack for cards.

// use crate::errors::*;
use onejoker::cards::*;

use std::io::{self, Write, BufReader};
use serde::Deserialize;
use std::fs::File;
use rmp_serde::decode::from_read;

#[derive(Debug, Deserialize)]
struct HandData {
    deck: i32,
    text: String,
    len: usize,
    hash: u32,
}

#[derive(Debug, Deserialize)]
struct HandDataList {
    count: usize,
    deck_names: Vec<String>,
    hands: Vec<HandData>,
}

#[test]
fn test_hand_data_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../data/bin/hands_text.msgpack");
    if file.is_err() {
        write!(&mut io::stdout(),
            "*** No test data file found; skipping...").unwrap();
        return Ok(());
    }
    let reader = BufReader::new(file.unwrap());
    let data: HandDataList = from_read(reader)?;

    for i in 0..data.count as usize {
        let deck = MasterDeck::by_name(&data.deck_names[data.hands[i].deck as usize - 1]);
        let mut h = CardStack::from_text(&data.hands[i].text);
        if deck.low_aces {
            h.low_ace_fix();
        }
        assert_eq!(h.len(), data.hands[i].len as usize);
        for j in 0..h.len() {
            assert_eq!(true, deck.has(h.card_at(j).unwrap()));
        }
        assert_eq!(h.to_string(), data.hands[i].text);
        assert_eq!(h.quick_hash(), data.hands[i].hash);
    }
    Ok(())
}
