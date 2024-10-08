//@ cards/stack.rs
//! # stack | [wiki](https://github.com/lcrocker/ojpoker/wiki/CardStack) | A simple LIFO stack for cards.

// use crate::errors::*;
use onejoker::cards::*;

use std::io::{self, Write, BufReader};
use serde::Deserialize;
use std::fs::File;
use rmp_serde::decode::from_read;

#[derive(Debug, Deserialize)]
struct HashDataList {
    count: usize,
    hands: Vec<Vec<String>>,
}

#[test]
fn test_hash_data_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../data/bin/hash_tests.msgpack");
    if file.is_err() {
        write!(&mut io::stdout(),
            "***\n   No test data file found; skipping...\n***").unwrap();
        return Ok(());
    }
    let reader = BufReader::new(file.unwrap());
    let data: HashDataList = from_read(reader)?;

    for i in 0..data.count {
        let s1 = OrphanHand::from_text(&data.hands[i][0]);
        let s2 = OrphanHand::from_text(&data.hands[i][1]);
        let s3 = OrphanHand::from_text(&data.hands[i][2]);

        assert_eq!(s1.len(), s2.len());
        assert_eq!(s1.len(), s3.len());

        let j = range_uniform(data.count);
        let s4 = OrphanHand::from_text(&data.hands[j][0]);

        let h1 = FNVHash::u32(&s1);
        let h2 = FNVHash::u32(&s2);
        let h3 = FNVHash::u32(&s3);
        let h4 = FNVHash::u32(&s4);

        assert_eq!(h1 == h2, s1.equals(&s2));
        assert!(h1 != h3);
        assert_eq!(h1 == h4, i == j);

        let mut h1 = BitfieldHash::u64co(&s1);
        let mut h2 = BitfieldHash::u64co(&s2);
        let mut h3 = BitfieldHash::u64co(&s3);
        let mut h4 = BitfieldHash::u64co(&s4);

        assert!(h1 == h2);
        assert!(h1 != h3);
        assert_eq!(h1 == h4, i == j);

        if s1.len() <= 16 && s4.len() <= 16 {
            h1 = PositionalHash::u64cos(&s1);
            h2 = PositionalHash::u64cos(&s2);
            h3 = PositionalHash::u64cos(&s3);

            assert!(h1 == h2);
            assert!(h1 == h3);
        }
        if s1.len() <= 10 && s4.len() <= 10 {
            h1 = PositionalHash::u64c(&s1);
            h2 = PositionalHash::u64c(&s2);
            h3 = PositionalHash::u64c(&s3);
            h4 = PositionalHash::u64c(&s4);

            assert_eq!(h1 == h2, s1.equals(&s2));
            assert!(h1 != h3);
            assert_eq!(h1 == h4, i == j);

            h1 = PositionalHash::u64co(&s1);
            h2 = PositionalHash::u64co(&s2);
            h3 = PositionalHash::u64co(&s3);
            h4 = PositionalHash::u64co(&s4);

            assert!(h1 == h2);
            assert!(h1 != h3);
            assert_eq!(h1 == h4, i == j);

            h1 = PrimeHash::u64cos(&s1);
            h2 = PrimeHash::u64cos(&s2);
            h3 = PrimeHash::u64cos(&s3);

            assert!(h1 == h2);
            assert!(h1 == h3);
        }
        if s1.len() <= 7 && s4.len() <= 7 {
            h1 = PrimeHash::u64co(&s1);
            h2 = PrimeHash::u64co(&s2);
            h3 = PrimeHash::u64co(&s3);
            h4 = PrimeHash::u64co(&s4);

            assert!(h1 == h2);
            assert!(h1 != h3);
            assert_eq!(h1 == h4, i == j);
        }
    }
    Ok(())
}
