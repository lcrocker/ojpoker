# [wiki](https://github.com/lcrocker/ojpoker/wiki/Rust_Libraries) | Non-game-specific card handling

This module contains all the sub-modules that are not specific
to any particular game---types like `Card` and `Deck` and
the card constants like `JACK_OF_CLUBS`.

## Cards in text

Cards are recorded to and retrieved from text (such as JSON files)
using the popular convention:
Each card is a 2-letter abbreviation with a one-letter uppercase rank and a
one-letter lowercase suit. Ranks are `2`, `3`, ... `9`, `T`, `J`, `Q`, `K`,
and `A`. Suits are `c`, `d`, `h`, and `s`.
This format is used in lots of data files of card games around the net, such
as PHH (<https://arxiv.org/html/2312.11753v2>) poker hand history format.
"AsKsQsJsTs" is a royal flush, for example.
I also recognize `C` for knight/cavalier, `Jk` for joker, `Jb` for the
black/uncolored joker in games that distinguish between them, and `Jw` for
the third "white" joker.
Whitespace between cards is ignored, but is not allowed between rank and suit.
It is never produced on output.
We can also produce Unicode suit symbols and single-code cards, which may
come in handy for producing documentation.
Finally, I allow "10d", for example, on input for tens as well as "1h" for
low aces, though these will never be output.

# Examples
