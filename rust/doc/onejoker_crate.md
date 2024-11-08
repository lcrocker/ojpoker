# [wiki](https://github.com/lcrocker/ojpoker/wiki/Rust_Libraries) | A library for handling playing cards and card games.

Last updated October 28, 2024 \
\
This crate is part of the [OneJoker](https://onejoker.org) project
to create free software for handling playing cards and card games
in general, and many poker variants in particular.

Lee Daniel Crocker <lee@onejoker.org> \
Licensed <https://creativecommons.org/publicdomain/zero/1.0/>

# Example
```
use onejoker::*;

fn main() -> Result<(), OjError> {
    let mut d = Deck::new(DeckType::English).shuffled();
    let h1 = d.new_hand().init(d.draw(5));
    let h2 = d.new_hand().init(d.draw(5));

    println!("Player 1: [{}], Player 2: [{}]", h1, h2);

    let eval = HandScale::by_name("poker").eval_full();
    let v1 = eval(&h1)?;
    let v2 = eval(&h2)?;

    if v1 < v2 {
        println!("Player 1 wins with [{}]", v1.full_name());
    } else if v1 > v2 {
        println!("Player 2 wins with [{}]", v2.full_name());
    } else {
        println!("Players tie with [{}]", v1.full_name());
    }
    Ok(())
}
```
This should produce output similar to:
```text
Player 1: [4cJc7s4h6s], Player 2: [Kd6sJdAsKh]
Player 2 wins with [pair of kings, ace, jack, six]
```
