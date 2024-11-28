/*
 * Build tables 1 and 2 for 5-card high hands.
 * Tweak tables to include 13 slots for 5-of-a-kind that we will need
 * for other games.
 */

use onejoker::prelude::*;
use onejoker::cards::hashes::*;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::BinaryHeap;

const DECK: &str = "english";
const SCALE: Scale = Scale::HighHand;
const HASH: fn(&[Card]) -> u32 = ojh_mp5_english;
const FNAME: &str = "high_mp5";
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

    let mut count = 0;
    for hand in deck.combinations(CARDS) {
        count += 1;
        if 0 == (count & 0x1FFFF) {
            print!("\revaluated {} hands", count);
            std::io::stdout().flush()?;
        }
        let desc = SCALE.eval(&hand)?;
        heap.push(HandAndDescription { hand, desc });
    }
    println!("\revaluated  {} hands", count);
    assert_eq!(count, heap.len());

    let file2 = File::create(format!("./{}_table_2.rs", FNAME))?;
    let mut writer2 = BufWriter::new(file2);
    writeln!(writer2, "pub const {}_TABLE_2: [u32; 7476] = [", FNAME.to_uppercase())?;
    writeln!(writer2, "    0x000000, 0x1FFFFF, 0x1EEEEE, 0x1DDDDD, 0x1BBBBB, 0x1AAAAA, 0x199999, 0x188888,")?;
    write!(writer2, "    0x177777, 0x166666, 0x155555, 0x144444, 0x133333, 0x122222, ")?;

    let mut p_val = 0;
    let mut ec = 0;
    let mut ec_heap: BinaryHeap<EquivClassAndHash> = BinaryHeap::new();

    for _ in 0..heap.len() {
        count -= 1;
        if 0 == (count & 0x1FFFF) {
            print!("\rtable 2: {:9} hands remaining", count);
            std::io::stdout().flush()?;
        }
        let hd = heap.pop().unwrap();

        if p_val != hd.desc.value {
            ec += 1;
            p_val = hd.desc.value;

            let mut rval: u32 = 0;
            for c in hd.desc.hand {
                rval <<= 4;
                rval += c.rank() as u32;
            }
            rval |= 0x100000 * hd.desc.level as u32;
            write!(writer2, "{:#06X}, ", rval)?;
            if 2 == (ec & 0x07) {
                write!(writer2, "\n    ")?;
            }
        }
        let hash = HASH(&hd.hand[..]);
        ec_heap.push(EquivClassAndHash { ec: ec + 13, hash });
        // println!("{} {:5} {}", hd.desc, hd.desc.value, hd.desc.full_name());
    }
    write!(writer2, "\n];")?;
    println!("\n{} equivalence classes", ec);

    let file1 = File::create(format!("./{}_table_1.bin", FNAME))?;
    let mut writer1 = BufWriter::new(file1);
    let mut bytes: [u8; 2] = [0; 2];
    writer1.write_all(&bytes)?;

    for i in 0..ec_heap.len() {
        count += 1;
        if 0 == (count & 0x3FFFF) {
            print!("\rtable 1: {:9} hands", count);
            std::io::stdout().flush()?;
        }
        let entry = ec_heap.pop().unwrap();
        // println!("{:7} {:5}", entry.hash, entry.ec);
        assert_eq!(entry.hash as usize, i + 1);

        bytes[0] = (0xFF & entry.ec) as u8;
        bytes[1] = (0xFF & (entry.ec >> 8)) as u8;
        writer1.write_all(&bytes)?;
    }
    println!("\rtable 1: {:9} hands\nDone.", count);
    Ok(())
}

fn main() -> OjResult<()> {
    build_tables()?;
    Ok(())
}
