// Build hash tables for poker hand evaluation

use std::collections::{BinaryHeap, HashMap};
use onejoker::*;

const DECK: &str = "low";
const SCALE: HandScale = HandScale::AceToFive;
const GAME: &str = "ACE_TO_FIVE";
const HASH: fn(&[Card]) -> u32 = |c|
    ojh_positional_32cs_mp5_low(c).unwrap();

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

const RVAL: [u32; 13] = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14 ];

fn build_tables() -> Result<(), OjError> {
    let mut heap: BinaryHeap<HandAndValue> = BinaryHeap::new();
    let deck = Deck::new_by_name(DECK);

    for i in 0..13 {
        let r1 = Rank::from_u8(RVAL[i] as u8);

        for j in 0..13 {
            let r2 = Rank::from_u8(RVAL[j] as u8);

            for k in 0..13 {
                let r3 = Rank::from_u8(RVAL[k] as u8);

                for l in 0..13 {
                    let r4 = Rank::from_u8(RVAL[l] as u8);

                    for m in 0..13 {
                        let r5 = Rank::from_u8(RVAL[m] as u8);

                        let hand = deck.new_hand().init([
                            Card::from_rank_suit(r1, Suit::Spade),
                            Card::from_rank_suit(r2, Suit::Heart),
                            Card::from_rank_suit(r3, Suit::Diamond),
                            Card::from_rank_suit(r4, Suit::Club),
                            Card::from_rank_suit(r5, Suit::Spade),
                        ]);

                        let v = SCALE.eval_full()(&hand)?;
                        heap.push(HandAndValue { hand, value: v });
                    }
                }
            }
        }
    }

    let all_hands: Vec<HandAndValue> = heap.into_sorted_vec();
    let mut ec_heap: BinaryHeap<EquivClassAndHash> = BinaryHeap::new();
    let mut value_map: HashMap<u32, HandAndValue> = HashMap::new();
    let mut equiv = 0;
    let mut p_value = 0;
    let mut p_equiv = 0;

    for hv in all_hands {
        if hv.value.value != p_value {
            equiv += 1;
        }
        assert!(hv.value.value >= p_value);
        assert!(equiv >= p_equiv);
        p_value = hv.value.value;
        p_equiv = equiv;

        let hash = HASH(hv.hand.as_slice());
        ec_heap.push(EquivClassAndHash { hash, eclass: equiv });
        value_map.entry(equiv).or_insert(hv);
    }

    println!("// Do not edit: file generated from script
/// Table 1: direct map from {} perfect hashes \
to {} equivalence classes\n", ec_heap.len(), equiv);
    print!("pub static {}_TABLE_1: [u16; {}] = [
  ", GAME, ec_heap.len());

    for i in 0..ec_heap.len() {
        let entry = ec_heap.pop().unwrap();
        assert!(i == entry.hash as usize);
        print!("{},", if entry.eclass > 6175 { 0 } else { entry.eclass });

        if 11 == i % 12 {
            print!("\n  ");
        }
    }
    println!("\n];

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
  (lv!(0),rk!(0,0,0,0,0)),", GAME, 6176);

    for ec in 1..=(equiv - 13) {
        let ep = &value_map[&ec];
        println!("  (lv!({}),rk!({},{},{},{},{})),",
            ep.value.level as u32,
            ep.value.hand[0].rank() as u32,
            ep.value.hand[1].rank() as u32,
            ep.value.hand[2].rank() as u32,
            ep.value.hand[3].rank() as u32,
            ep.value.hand[4].rank() as u32);
    }
    println!("];

/// Table 3: action razz equivalence class adjustments

pub static ACTION_RAZZ_ADJUST: [u16; {}] = [ 0,
  ", 6138);

    let mut next_ec = 6176;
    for ec in 1..=(equiv - 52) {
        let ep = &value_map[&ec];

        if ep.value.hand[0].rank() > Rank::Ten ||
            ep.value.hand[1].rank() > Rank::Ten ||
            ep.value.hand[2].rank() > Rank::Ten ||
            ep.value.hand[3].rank() > Rank::Ten ||
            ep.value.hand[4].rank() > Rank::Ten {
            print!("0,");
        } else {
            print!("{},", next_ec);
            next_ec += 1;
        }
        if 11 == ec % 12 {
            print!("\n  ");
        }
    }
    println!("\n];");

    Ok(())
}

fn main() -> Result<(), OjError> {
    build_tables()
}
