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
use onejoker::prelude::*;

fn main() -> OjResult<()> {
    let game = HandScale::by_name("high-hand");
    let mut d = game.new_deck().shuffled();
    let hand1 = d.new_hand().init(d.draw(5));
    let hand2 = d.new_hand().init(d.draw(5));

    println!("Player 1: [{}], Player 2: [{}]", hand1, hand2);

    let desc1 = game.eval(&hand1)?;
    let desc2 = game.eval(&hand2)?;

    if desc1.value() < desc2.value() {
        println!("Player 1 wins with [{}]", desc1.full_name());
    } else if desc1.value() > desc2.value() {
        println!("Player 2 wins with [{}]", desc2.full_name());
    } else {
        println!("Players tie with [{}]", desc1.full_name());
    }
    Ok(())
}
```
This should produce output similar to:
```text
Player 1: [4cJc7s4h6s], Player 2: [Kd6sJdAsKh]
Player 2 wins with [pair of kings, ace, jack, six]
```
Some things to note: we begin by choosing a game.
The [HandScale] type represents the various way poker hands can be
compared against each other.
The "high-hand" scale is for traditional high poker hands: pair,
two pair, trips, etc.
Other options include "ace-to-five" low hands, "deuce-to-seven" low
hands, "pai gow", "badugi", and others.
By choosing the game first and creating the deck from it, the system
will ensure that the correct deck of cards is chosen.

The deck is then created with `game.new_deck().shuffled()`, which
creates a deck suitable for the chosen game and gives it an initial
shuffle.
Hands are then created from the deck with `d.new_hand()`, and
initialized with cards from the deck with `.init(d.draw(5))`.
[Deck]s and [Hand]s can be created independently of a game, but then
you will have to specify what type of deck to use: (e.g.
`let d = Deck::new(DeckType::English);`) and which function to call
for evaluating hands (e.g. `ojp_high_eval_full(&hand)`).

Each `HandScale` contains an `eval()` function that creates an info
structure about the value of the hand in that game.
That structure has a method `value()` that returns a single number
that can be used to compare two hands: lower number wins.
If you just want that number without all the other info, there's also
an `eval_quick()` function.
Those are compared here to determine a winner, and then the hands
are displayed along with the full text description of the winning
hand (note that this is a function of the full hand description object,
and so would be unavailable if we used `eval_quick()`).
