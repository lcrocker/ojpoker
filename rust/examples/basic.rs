
use onejoker::prelude::*;

fn main() -> OjResult<()> {
    let game = Scale::by_name("poker");
    let mut deck = game.new_deck().shuffled();
    let h1 = deck.new_hand().init(deck.draw(5));
    let h2 = deck.new_hand().init(deck.draw(5));

    println!("Player 1: [{}], Player 2: [{}]", h1, h2);

    let v1 = game.value(&h1);
    let v2 = game.value(&h2);

    if v1 < v2 {
        let d = game.description(&h1, v1);
        println!("Player 1 wins with [{}] ({})", v1, d.full_text());
    } else if v1 > v2 {
        let d = game.description(&h2, v2);
        println!("Player 2 wins with [{}] ({})", v2, d.full_text());
    } else {
        let d = game.description(&h1, v1);
        println!("Players tie with ({})", d.full_text());
    }
    Ok(())
}

