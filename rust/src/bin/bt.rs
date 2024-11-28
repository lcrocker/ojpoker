// Build hash tables for poker hand evaluation
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::io::Write;

use onejoker::prelude::*;
use onejoker::cards::hashes::*;

// const DECK: &str = "english";
// const SCALE: HandScale = HandScale::HighHand;
// const GAME: &str = "HIGH_HAND";
// const HASH: fn(&[Card]) -> u32 = ojh_mp5_english;

// const DECK: &str = "english";
// const SCALE: HandScale = HandScale::HighHand;
// const GAME: &str = "HIGH_HAND";
// const HASH: fn(&[Card]) -> u32 = ojh_mp7_english;

const DECK: &str = "english";
const SCALE: Scale = Scale::DeuceToSeven;
const GAME: &str = "DEUCE_TO_SEVEN";
const HASH: fn(&[Card]) -> u32 = ojh_mp5_english;
// const FILE: &str = "./ojp_deuce_to_seven_mp5_table_1.bin";

// const DECK: &str = "low";
// const SCALE: HandScale = HandScale::AceToSix;
// const GAME: &str = "ACE_TO_SIX";
// const HASH: fn(&[Card]) -> u32 = |c|
//     ojh_mp5_low(ojh_bitfield_64co(c).unwrap());

// const DECK: &str = "low";
// const SCALE: HandScale = HandScale::Badugi;
// const GAME: &str = "BADUGI";
// const HASH: fn(&[Card]) -> u32 = |c|
//     ojh_mp4_low(ojh_bitfield_64co(c).unwrap());

// const DECK: &str = "english";
// const SCALE: HandScale = HandScale::Badeucy;
// const GAME: &str = "BADEUCY";
// const HASH: fn(&[Card]) -> u32 = |c|
//     ojh_mp4_english(ojh_bitfield_64co(c).unwrap());

// const DECK: &str = "stripped";
// const SCALE: HandScale = HandScale::Stripped;
// const GAME: &str = "STRIPPED_DECK";
// const HASH: fn(&[Card]) -> u32 = |c|
//     ojh_mp5_stripped(ojh_bitfield_64co(c).unwrap());

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
        self.desc.value.cmp(&other.desc.value)
    }
}

struct EquivClassAndHash {
    eclass: u32,
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

fn build_tables() -> Result<(), OjError> {
    let mut heap: BinaryHeap<HandAndDescription> = BinaryHeap::new();
    let deck = Deck::new_by_name(DECK);

    for hand in deck.combinations(7) {
        println!("// {:?}", hand);
        let desc = SCALE.eval(&hand)?;
        heap.push(HandAndDescription { hand, desc });
    }
    let all_hands: Vec<HandAndDescription> = heap.into_sorted_vec();
    let mut ec_heap: BinaryHeap<EquivClassAndHash> = BinaryHeap::new();
    let mut value_map: HashMap<u32, HandAndDescription> = HashMap::new();
    let mut equiv = 0;
    let mut p_value = 0;
    let mut p_equiv = 0;

    let total = all_hands.len();
    for hv in all_hands {
        if hv.desc.value != p_value {
            equiv += 1;
        }
        assert!(hv.desc.value >= p_value);
        assert!(equiv >= p_equiv);
        p_value = hv.desc.value;
        p_equiv = equiv;

        let hash = HASH(&hv.hand[..]);
        ec_heap.push(EquivClassAndHash { eclass: equiv, hash });
        value_map.entry(equiv).or_insert(hv);
    }

    let mut file = fs::File::create("./mp7.bin")?;
    let mut bytes: [u8; 2] = [0; 2];

    println!("// Do not edit: file generated from script
/// Table 1: direct map from {} perfect hashes \
to {} equivalence classes\n", total, equiv);
    print!("pub static {}_TABLE_1: [u16; {}] = [ 0,
  ", GAME, total + 1);

    let mut p_hash = 0xFFFF_FFFF;

    for i in 0..ec_heap.len() {
        let entry = ec_heap.pop().unwrap();
        if p_hash != entry.hash {
            print!("{},", entry.eclass);
            p_hash = entry.hash;

            bytes[0] = (0xFF & entry.eclass) as u8;
            bytes[1] = (0xFF & (entry.eclass >> 8)) as u8;
            file.write_all(&bytes)?;
        }
        if 99 == i % 100 {
            print!("\n  ");
        }
    }
    println!("
];

use crate::cards::rank::Rank;
use crate::poker::hand_evaluation::HandLevel;

macro_rules! lv {{ ($a:expr) => {{ HandLevel::from_u8($a) }} }}
macro_rules! rk {{
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        [Rank::from_u8($a),Rank::from_u8($b),Rank::from_u8($c),Rank::from_u8($d),Rank::from_u8($e)]
    }};
}}

/// Table 2: from equivalence class to hand level and rank order
");
    println!("pub static {}_TABLE_2: [(HandLevel, [Rank; 5]); {}] = [
  (lv!(0),rk!(0,0,0,0{})),", GAME, equiv + 1,
        if 5 == SCALE.complete_hand() { ",0" } else { "" });

    for ec in 1..=equiv {
        let ep = value_map.get_mut(&ec).unwrap();
        if 5 == SCALE.complete_hand() {
            let hand = ep.desc.hand;
            println!("  (lv!({}),rk!({},{},{},{},{})),",
                ep.desc.level as u32,
                hand[0].rank() as u32,
                hand[1].rank() as u32,
                hand[2].rank() as u32,
                hand[3].rank() as u32,
                hand[4].rank() as u32);
        } else {
            let lv = ep.desc.level as u32;
            let hand = &mut ep.desc.hand;
            if lv > 11 { hand[3] = Card(0); }
            if lv > 12 { hand[2] = Card(0); }
            if lv > 13 { hand[1] = Card(0); }

            println!("  (lv!({}),rk!({},{},{},{})),",
                lv,
                hand[0].rank() as u32,
                hand[1].rank() as u32,
                hand[2].rank() as u32,
                hand[3].rank() as u32);
        }
    }
    println!("];");

    Ok(())
}

use std::time::Instant;

fn main() -> Result<(), OjError> {
    let start = Instant::now();
    build_tables()?;
    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);

    Ok(())
}
