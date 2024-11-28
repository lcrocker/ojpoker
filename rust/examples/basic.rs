
use onejoker::prelude::*;

fn main() -> OjResult<()> {
    let mut d = Deck::new(DeckType::English).shuffled();
    let h1 = d.new_hand().init(d.draw(5));
    let h2 = d.new_hand().init(d.draw(5));

    println!("Player 1: [{}], Player 2: [{}]", h1, h2);

    let game = Scale::by_name("high-hand");
    let v1 = game.eval(&h1)?;
    let v2 = game.eval(&h2)?;

    if v1 < v2 {
        println!("Player 1 wins with [{}] ({})", v1, v1.full_name());
    } else if v1 > v2 {
        println!("Player 2 wins with [{}] ({})", v2, v2.full_name());
    } else {
        println!("Players tie with [{}] ({})", v1, v1.full_name());
    }
    Ok(())
}

