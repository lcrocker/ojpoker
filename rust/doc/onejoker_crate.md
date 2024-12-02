# [wiki](https://github.com/lcrocker/ojpoker/wiki/Rust_Libraries) | A library for handling playing cards and card games.

Last updated November 22, 2024 \
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
    let game = Scale::by_name("high-hand");
    let mut deck = game.new_deck().shuffled();
    let hand1 = deck.new_hand().init(deck.draw(5));
    let hand2 = deck.new_hand().init(deck.draw(5));

    println!("Player 1: [{}], Player 2: [{}]", hand1, hand2);

    let v1 = game.value(&hand1);
    let v2 = game.value(&hand2);

    if v1 < v2 {
        let d = game.description(&hand1, v1);
        println!("Player 1 wins with [{}] ({})", d.hand, d.full_text());
    } else if v1 > v2 {
        let d = game.description(&hand2, v2);
        println!("Player 2 wins with [{}] ({})", d.hand, d.full_text());
    } else {
        let d = game.description(&hand1, v1);
        println!("Players tie with ({})", d.full_text());
    }
    Ok(())
}
```
This should produce output similar to:
```text
Player 1: [TcTd6sQdAh], Player 2: [6d2d9c2s9h]
Player 2 wins with [9h9c2s2d6d] (nines and deuces with a six)
```
Some things to note: we begin by choosing a game.
The `Scale` type represents the various way poker hands can be
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
Hands are then created from the deck with `deck.new_hand()`, and
initialized with cards from the deck with `.init(deck.draw(5))`.
`Deck`s and `Hand`s can be created independently of a game, but then
you will have to specify what type of deck to use: (e.g.
`let d = Deck::new(DeckType::English);`) and which function to call
for evaluating hands (e.g. `ojp_high_value(&hand)`).

Each `Scale` contains an `value()` function that computes a number
that can be used to compare hands--smaller number is better.
If you want more information about the hand that just who wins,
you can use the scale's `description()` function to create a
structure that has more information. Here we use it to print the
hand and its text description.
