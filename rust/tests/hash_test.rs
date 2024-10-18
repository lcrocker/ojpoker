//@ tests/hash_test.rs

use onejoker::*;

// use std::io::{self, Write, BufReader};
// use serde::Deserialize;
// use std::fs::File;
// use rmp_serde::decode::from_read;

// #[derive(Debug, Deserialize)]
// struct HashDataList {
//     count: usize,
//     hands: Vec<Vec<String>>,
// }

#[test]
fn test_hash_data_file() -> aResult<()> {
    // let file = File::open("../data/bin/hash_tests.msgpack");
    // if file.is_err() {
    //     write!(&mut io::stdout(),
    //         "***\n   No test data file found; skipping...\n***").unwrap();
    //     return aOk(());
    // }
    // let reader = BufReader::new(file.unwrap());
    // let data: HashDataList = from_read(reader)?;
    // let mut hasher: Hasher = FNVHash::new();

    // for i in 0..data.count {
    //     let s1 = Hand::new_from_text(&data.hands[i][0]);
    //     let s2 = Hand::new_from_text(&data.hands[i][1]);
    //     let s3 = Hand::new_from_text(&data.hands[i][2]);

    //     assert_eq!(s1.len(), s2.len());
    //     assert_eq!(s1.len(), s3.len());

    //     let j = range_uniform(data.count);
    //     let s4 = Hand::new_from_text(&data.hands[j][0]);

    //     let h1 = hasher.u32(s1.as_slice()).unwrap();
    //     let h2 = hasher.u32(s2.as_slice()).unwrap();
    //     let h3 = hasher.u32(s3.as_slice()).unwrap();
    //     let h4 = hasher.u32(s4.as_slice()).unwrap();

    //     assert_eq!(h1 == h2, s1.equals(&s2));
    //     assert!(h1 != h3);
    //     assert_eq!(h1 == h4, i == j);

    //     hasher = BitfieldHash::new();
    //     let mut h1 = hasher.u64co(s1.as_slice()).unwrap();
    //     let mut h2 = hasher.u64co(s2.as_slice()).unwrap();
    //     let mut h3 = hasher.u64co(s3.as_slice()).unwrap();
    //     let mut h4 = hasher.u64co(s4.as_slice()).unwrap();

    //     assert!(h1 == h2);
    //     assert!(h1 != h3);
    //     assert_eq!(h1 == h4, i == j);

    //     hasher = PositionalHash::new();
    //     if s1.len() <= 16 && s4.len() <= 16 {
    //         h1 = hasher.u64cos(s1.as_slice()).unwrap();
    //         h2 = hasher.u64cos(s2.as_slice()).unwrap();
    //         h3 = hasher.u64cos(s3.as_slice()).unwrap();

    //         assert!(h1 == h2);
    //         assert!(h1 == h3);
    //     }
    //     if s1.len() <= 10 && s4.len() <= 10 {
    //         h1 = hasher.u64c(s1.as_slice()).unwrap();
    //         h2 = hasher.u64c(s2.as_slice()).unwrap();
    //         h3 = hasher.u64c(s3.as_slice()).unwrap();
    //         h4 = hasher.u64c(s4.as_slice()).unwrap();

    //         assert_eq!(h1 == h2, s1.equals(&s2));
    //         assert!(h1 != h3);
    //         assert_eq!(h1 == h4, i == j);

    //         h1 = hasher.u64co(s1.as_slice()).unwrap();
    //         h2 = hasher.u64co(s2.as_slice()).unwrap();
    //         h3 = hasher.u64co(s3.as_slice()).unwrap();
    //         h4 = hasher.u64co(s4.as_slice()).unwrap();

    //         assert!(h1 == h2);
    //         assert!(h1 != h3);
    //         assert_eq!(h1 == h4, i == j);

    //         hasher = PrimeHash::new();
    //         h1 = hasher.u64cos(s1.as_slice()).unwrap();
    //         h2 = hasher.u64cos(s2.as_slice()).unwrap();
    //         h3 = hasher.u64cos(s3.as_slice()).unwrap();

    //         assert!(h1 == h2);
    //         assert!(h1 == h3);
    //     }
    //     if s1.len() <= 7 && s4.len() <= 7 {
    //         h1 = hasher.u64co(s1.as_slice()).unwrap();
    //         h2 = hasher.u64co(s2.as_slice()).unwrap();
    //         h3 = hasher.u64co(s3.as_slice()).unwrap();
    //         h4 = hasher.u64co(s4.as_slice()).unwrap();

    //         assert!(h1 == h2);
    //         assert!(h1 != h3);
    //         assert_eq!(h1 == h4, i == j);
    //     }
    // }
    aOk(())
}
