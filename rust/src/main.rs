
use onejoker::*;

fn main() -> aResult<()> {
    let deck = Deck::new("default");

    for h in deck.combinations(5) {
        let hash = ojh_bitfield_u64co(h.as_slice())?;
        let mph = ojh_mp5_english(hash);
        println!("{}  {}", h, mph);
    }
    aOk(())
}

// fn main() -> Result<()> {
//     let mut d = Deck::new("default");
//     d.shuffle();
//     let h1 = d.new_hand_with(5);
//     let h2 = d.new_hand_with(5);

//     println!("Player 1: [{}], Player 2: [{}]", h1, h2);

//     let high = HandEvaluatorHigh::new();
//     let v1 = high.value_of(&h1)?;
//     let v2 = high.value_of(&h2)?;

//     if v1 < v2 {
//         println!("Player 1 wins with [{}]", v1.full_name());
//     } else if v1 > v2 {
//         println!("Player 2 wins with [{}]", v2.full_name());
//     } else {
//         println!("Players tie with [{}]", v1.full_name());
//     }
//     aOk(())
// }
