//! This is an example of one of the programs used to build the
//! lookup tables (in this case, deuce-to-seven lowball). It uses
//! binary heaps to collect all of the hands and sort them as
//! needed, then writes the tables to files.

use onejoker::prelude::*;
use onejoker::cards::hashes::*;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::BinaryHeap;

const DECK: &str = "english";
const SCALE: Scale = Scale::DeuceToSeven;
const HASH: fn(&[Card]) -> u32 = ojh_bitfield_mp5_english;
const FNAME: &str = "ojp_kc";
const CARDS: usize = 5;

struct HandAndDescription {
    hand: Hand,
    desc: HandDescription,
}
impl PartialEq for HandAndDescription {
    fn eq(&self, other: &Self) -> bool {
        self.desc.value == other.desc.value
    }
}
impl Eq for HandAndDescription {}
impl PartialOrd for HandAndDescription {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandAndDescription {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.desc.value.cmp(&self.desc.value)
    }
}

struct EquivClassAndHash {
    ec: u32,
    hash: u32,
}
impl PartialEq for EquivClassAndHash {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
impl Eq for EquivClassAndHash {}
impl PartialOrd for EquivClassAndHash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for EquivClassAndHash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.hash.cmp(&self.hash)
    }
}

fn build_tables() -> OjResult<()> {
    let mut heap: BinaryHeap<HandAndDescription> = BinaryHeap::new();
    let deck = Deck::new_by_name(DECK);

    // Enumerate all 5-card hands, evaluate them, and add
    // them to the first heap.

    let mut count = 0;
    for hand in deck.combinations(CARDS) {
        count += 1;
        if 0 == count & 0x1FFFF {
            print!("\revaluated {} hands", count);
            std::io::stdout().flush()?;
        }
        let v = SCALE.value(&hand);
        let desc = SCALE.description(&hand, v);
        heap.push(HandAndDescription { hand, desc });
    }
    println!("\revaluated {} hands", count);
    assert_eq!(count, heap.len());

    // Table 2 is built from the first heap. Iterate through the
    // hands in evaluation order, assigning equivalence classes to
    // each equal-valued group, then add an entry to the table for
    // each class with the hand level and card ranks.

    let file2 = File::create(format!("./{}_table_2.rs", FNAME))?;
    let mut writer2 = BufWriter::new(file2);
    write!(writer2, "pub const {}_TABLE_2: [u32; ??] = [\n", FNAME.to_uppercase())?;
    write!(writer2, "    0x000000, ")?;

    let mut p_val = 0;
    let mut ec = 0;
    let mut ec_heap: BinaryHeap<EquivClassAndHash> = BinaryHeap::new();

    for _i in 0..heap.len() {
        count -= 1;
        if 0 == count & 0x1FFFF {
            print!("\rtable 2:{:9} hands remaining", count);
            std::io::stdout().flush()?;
        }
        let entry = heap.pop().unwrap();
        // println!("{:9} {} {:X}", i, entry.hand, entry.desc.value);

        if p_val != entry.desc.value {
            ec += 1;
            p_val = entry.desc.value;

            write!(writer2, "{:#06X}, ", p_val)?;
            if 7 == ec & 0x7 {
                write!(writer2, "\n    ")?;
            }
        }
        // As we unpack the first heap, calculate the hash for each
        // hand and add it to the second heap.

        let hash = HASH(&entry.hand[..]);
        ec_heap.push(EquivClassAndHash { ec, hash });
        // println!("{:9} {} {} {}", i, entry.hand, ec, hash);
    }
    write!(writer2, "\n];\n")?;
    println!("\n{} equivalence classes", ec);

    // Table 1 is built from the second heap. Iterate through the
    // hands in hash order, writing the equivalence class for each.

    let file1 = File::create(format!("./{}_table_1.bin", FNAME))?;
    let mut writer1 = BufWriter::new(file1);
    let mut bytes: [u8; 2] = [0, 0];
    writer1.write_all(&bytes)?;

    for _i in 0..ec_heap.len() {
        count += 1;
        if 0 == count & 0x1FFFF {
            print!("\rtable 1:{:9} hands", count);
            std::io::stdout().flush()?;
        }
        let entry = ec_heap.pop().unwrap();
        bytes[0] = (0xFF & entry.ec) as u8;
        bytes[1] = (0xFF & (entry.ec >> 8)) as u8;
        writer1.write_all(&bytes)?;
    }
    println!("\rtable 1:{:9} hands", count);

    Ok(())
}

fn main() -> OjResult<()> {
    build_tables()
}
