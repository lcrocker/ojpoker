
use std::collections::{BinaryHeap, HashMap};
use onejoker::*;

struct HandAndValue {
    hand: Hand,
    value: HandValueHigh,
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

struct EquivClassAndPerfectHash {
    eclass: u32,
    phash: u32,
}
impl PartialEq for EquivClassAndPerfectHash {
    fn eq(&self, other: &Self) -> bool {
        self.phash == other.phash
    }
}
impl Eq for EquivClassAndPerfectHash {}
impl PartialOrd for EquivClassAndPerfectHash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}
impl Ord for EquivClassAndPerfectHash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.phash.cmp(&self.phash)
    }
}

fn main() -> Result<(), OjError> {
    println!("Building lookup tables...");
    let hh = HandEvaluatorHigh::new();
    let mut heap: BinaryHeap<HandAndValue> = BinaryHeap::new();
    let deck = Deck::new("english");

    let mut total = 0;
    for hand in deck.combinations(5) {
        total += 1;

        let v = hh.value_of(&hand)?;
        heap.push(HandAndValue { hand, value: v });
    }
    println!("Evaluated {} hands", total);

    let all_hands: Vec<HandAndValue> = heap.into_sorted_vec();
    let mut ec_heap: BinaryHeap<EquivClassAndPerfectHash> = BinaryHeap::new();
    let mut value_map: HashMap<u32, HandAndValue> = HashMap::new();
    let mut equiv = 0;
    let mut p_value = 0;
    let mut p_equiv = 0;

    for hv in all_hands {
        if hv.value.value() != p_value {
            equiv += 1;
        }
        assert!(hv.value.value() >= p_value);
        assert!(equiv >= p_equiv);
        p_value = hv.value.value();
        p_equiv = equiv;

        let hash = ojh_bitfield_64co(hv.hand.as_slice()).unwrap();
        let mph = ojh_mp5_english(hash);
        ec_heap.push(EquivClassAndPerfectHash { eclass: equiv, phash: mph });

        value_map.entry(equiv).or_insert(hv);
    }
    println!("// generated file");
    println!("{{\n hash_count: {},", total);
    println!(" eclass_count: {},", equiv);
    println!(" hashes: [");

    for i in 0..ec_heap.len() {
        let entry = ec_heap.pop().unwrap();
        assert!(entry.phash == i as u32 + 1);
        println!("{},", entry.eclass);
    }
    println!("],\n eclasses: [");

    for ec in 1..=equiv {
        let ep = &value_map[&ec];
        println!("[{},[{},{},{},{},{}]], // {}",
            ep.value.level.index(),
            ep.value.ranks[0] as i32,
            ep.value.ranks[1] as i32,
            ep.value.ranks[2] as i32,
            ep.value.ranks[3] as i32,
            ep.value.ranks[4] as i32,
            ep.value.full_name());
    }
    println!("],\n}}");

    Ok(())
}
