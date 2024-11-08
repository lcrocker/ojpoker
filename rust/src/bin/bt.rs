// Build hash tables for poker hand evaluation

// const DECK: &str = "english";
// const SCALE: HandScale = HandScale::HighHand;
// const GAME: &str = "HIGH_HAND";
// const HASH: fn(&[Card]) -> u32 = |c|
//     ojh_mp5_english(ojh_bitfield_64co(c).unwrap());

// const DECK: &str = "english";
// const SCALE: HandScale = HandScale::DeuceToSeven;
// const GAME: &str = "DEUCE_TO_SEVEN";
// const HASH: fn(&[Card]) -> u32 = |c|
//    ojh_mp5_english(ojh_bitfield_64co(c).unwrap());

// const DECK: &str = "low";
// const SCALE: HandScale = HandScale::AceToSix;
// const GAME: &str = "ACE_TO_SIX";
// const HASH: fn(&[Card]) -> u32 = |c|
//     ojh_mp5_low(ojh_bitfield_64co(c).unwrap());

const DECK: &str = "low";
const SCALE: HandScale = HandScale::Badugi;
const GAME: &str = "BADUGI";
const HASH: fn(&[Card]) -> u32 = |c|
    ojh_mp4_low(ojh_bitfield_64co(c).unwrap());
    
use std::collections::{BinaryHeap, HashMap};
use onejoker::*;

struct HandAndValue {
    hand: Hand,
    value: HandValue,
}
impl PartialEq for HandAndValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for HandAndValue {}
impl PartialOrd for HandAndValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandAndValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
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
    let mut heap: BinaryHeap<HandAndValue> = BinaryHeap::new();
    let deck = Deck::new_by_name(DECK);

    for hand in deck.combinations(SCALE.complete_hand()) {
        let v = SCALE.eval_full()(&hand)?;
        heap.push(HandAndValue { hand, value: v });
    }
    let all_hands: Vec<HandAndValue> = heap.into_sorted_vec();
    let mut ec_heap: BinaryHeap<EquivClassAndHash> = BinaryHeap::new();
    let mut value_map: HashMap<u32, HandAndValue> = HashMap::new();
    let mut equiv = 0;
    let mut p_value = 0;
    let mut p_equiv = 0;

    let total = all_hands.len();
    for hv in all_hands {
        if hv.value.value != p_value {
            equiv += 1;
        }
        assert!(hv.value.value >= p_value);
        assert!(equiv >= p_equiv);
        p_value = hv.value.value;
        p_equiv = equiv;

        let hash = HASH(hv.hand.as_slice());
        ec_heap.push(EquivClassAndHash { eclass: equiv, hash });
        value_map.entry(equiv).or_insert(hv);
    }

    println!("// Do not edit: file generated from script
/// Table 1: direct map from {} perfect hashes \
to {} equivalence classes\n", total, equiv);
    print!("pub static {}_TABLE_1: [u16; {}] = [ 0,
  ", GAME, total + 1);

    let mut p_hash = 0xFFFFFFFF;

    for i in 0..ec_heap.len() {
        let entry = ec_heap.pop().unwrap();
        if p_hash != entry.hash {
            print!("{},", entry.eclass);
            p_hash = entry.hash;
        }
        if 11 == i % 12 {
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
            println!("  (lv!({}),rk!({},{},{},{},{})),",
                ep.value.level as u32,
                ep.value.hand[0].rank() as u32,
                ep.value.hand[1].rank() as u32,
                ep.value.hand[2].rank() as u32,
                ep.value.hand[3].rank() as u32,
                ep.value.hand[4].rank() as u32);
        } else {
            if ep.value.level as u32 > 11 { ep.value.hand[3] = Card(0); }
            if ep.value.level as u32 > 12 { ep.value.hand[2] = Card(0); }
            if ep.value.level as u32 > 13 { ep.value.hand[1] = Card(0); }
                
            println!("  (lv!({}),rk!({},{},{},{})),",
                ep.value.level as u32,
                ep.value.hand[0].rank() as u32,
                ep.value.hand[1].rank() as u32,
                ep.value.hand[2].rank() as u32,
                ep.value.hand[3].rank() as u32);
        }
    }
    println!("];");

    Ok(())
}

fn main() -> Result<(), OjError> {
    build_tables()
}