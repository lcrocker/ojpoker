//!
//! Use of lookup tables must be enabled in `Cargo.toml`, and the binary
//! files used must be downloaded from the "releases" area of the repository.
//! See [README](https://github.com/lcrocker/ojpoker/blob/main/data/releases/README.md)
//! for details.

#[cfg(feature = "high-hand-tables")]
pub mod high_tables;

#[cfg(feature = "ace-to-five-tables")]
pub mod ace_to_five_tables;

#[cfg(feature = "deuce-to-seven-tables")]
pub mod deuce_to_seven_tables;

#[cfg(feature = "badugi-tables")]
pub mod badugi_tables;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_load_binary_table) | Decompress and load binary loopup table
///
/// Yes, I'm using "expect" here.
/// If the tables are not present, I cannot fall back to to the reference
/// evaluators because they produce different values, violating user
/// expectation and complicating the tests.
/// Also I want the user the know that the tables are missing.
#[cfg(any(feature = "high-hand-tables",
    feature = "ace-to-five-tables",
    feature = "deuce-to-seven-tables",
    feature = "badugi-tables"))]
pub fn ojp_load_binary_table(name: &str, size: usize) -> Vec<u16> {
    use std::fs;
    use std::io::{Read, BufReader};
    use std::path::PathBuf;
    use home::cargo_home;
    use flate2::read::GzDecoder;

    let mut path: PathBuf = cargo_home()
        .expect("failed to get cargo home directory");

    path.push("onejoker");
    path.push(name);

    let file = fs::File::open(&path)
        .expect("failed to open binary file for lookup table");

    let mut reader = BufReader::new(file);
    let mut decoder = GzDecoder::new(&mut reader);

    let mut table: Vec<u16> = Vec::with_capacity(size);
    let ptr = table.as_mut_ptr();
    let cap = table.capacity() * 2;

    std::mem::forget(table);
    let mut bytes: Vec<u8> = unsafe {
        Vec::from_raw_parts(ptr as *mut u8, 0, cap)
    };
    let nread = decoder.read_to_end(&mut bytes).unwrap();
    debug_assert!(nread == 2 * size);

    let len = bytes.len() / 2;
    let cap = bytes.capacity() / 2;
    let ptr = bytes.as_mut_ptr() as *mut u16;

    std::mem::forget(bytes);
    unsafe { Vec::from_raw_parts(ptr, len, cap) }
}
