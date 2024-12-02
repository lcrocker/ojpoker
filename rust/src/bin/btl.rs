// Build hash tables for poker hand evaluation

use std::collections::{BinaryHeap, HashMap};

use onejoker::prelude::*;
use onejoker::cards::hashes::*;

const DECK: &str = "low";
const SCALE: Scale = Scale::AceToFive;
const GAME: &str = "ACE_TO_FIVE";
const HASH: fn(&[Card]) -> u32 = ojh_positional_mp5_low;

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

const RVAL: [Rank; 13] = [ Rank::LowAce, Rank::Deuce, Rank::Trey, Rank::Four,
    Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
    Rank::Jack, Rank::Queen, Rank::King ];

fn build_tables() -> Result<(), OjError> {
    let mut heap: BinaryHeap<HandAndDescription> = BinaryHeap::new();
    let deck = Deck::new_by_name(DECK);

    for r1 in &RVAL {
        for r2 in &RVAL {
            for r3 in &RVAL {
                for r4 in &RVAL {
                    for r5 in &RVAL {
                        let hand = deck.new_hand().init([
                            Card::from_rank_suit(*r1, Suit::Spade),
                            Card::from_rank_suit(*r2, Suit::Heart),
                            Card::from_rank_suit(*r3, Suit::Diamond),
                            Card::from_rank_suit(*r4, Suit::Club),
                            Card::from_rank_suit(*r5, Suit::Spade),
                        ]);

                        let v = SCALE.value(&hand);
                        let desc = SCALE.description(&hand, v);
                        heap.push(HandAndDescription { hand, desc });
                    }
                }
            }
        }
    }

    let all_hands: Vec<HandAndDescription> = heap.into_sorted_vec();
    let mut ec_heap: BinaryHeap<EquivClassAndHash> = BinaryHeap::new();
    let mut value_map: HashMap<u32, HandAndDescription> = HashMap::new();
    let mut equiv = 0;
    let mut p_value = 0;
    let mut p_equiv = 0;

    for hv in all_hands {
        if hv.desc.value != p_value {
            equiv += 1;
        }
        assert!(hv.desc.value >= p_value);
        assert!(equiv >= p_equiv);
        p_value = hv.desc.value;
        p_equiv = equiv;

        let hash = HASH(&hv.hand[..]);
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
        let hand = ep.desc.hand;

        println!("  (lv!({}),rk!({},{},{},{},{})),",
            ep.desc.level as u32,
            hand[0].rank() as u32,
            hand[1].rank() as u32,
            hand[2].rank() as u32,
            hand[3].rank() as u32,
            hand[4].rank() as u32);
    }
    println!("];

/// Table 3: action razz equivalence class adjustments

pub static ACTION_RAZZ_ADJUST: [u16; {}] = [ 0,
  ", 6138);

    let mut next_ec = 6176;
    for ec in 1..=(equiv - 52) {
        let ep = &value_map[&ec];
        let hand = ep.desc.hand;

        if hand[0].rank() > Rank::Ten ||
            hand[1].rank() > Rank::Ten ||
            hand[2].rank() > Rank::Ten ||
            hand[3].rank() > Rank::Ten ||
            hand[4].rank() > Rank::Ten {
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
