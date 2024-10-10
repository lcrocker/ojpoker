# ojpoker | Updated: October 9, 2024

This is code for the [OneJoker](https://onejoker.org) project,
aiming to create libraries and other digital tools for handling playing cards
and card games in general, and more spcifically poker and its many variants.
I am writing it to provide some advantages over other libraries I have
encountered on the net:

- Completeness: Card-handling code covers things that others don't like
  jokers and foreign decks. Poker code handles more variants. The two
  initial languages (Dart and Rust) cover more application areas.
- Correctness: Many libraries do not correctly handle things like lowball
  and badugi hands, or betting limits and procedures. Author is a long-time
  poker player and casino manager with extensive knowledge of the rules,
  and implements them carefully.
- Performance: Poker code is *fast*, taking full advantage of modern 64-bit
  machines. It does sacrifice memory efficiency for this in some cases.

# Requirements

If you already have the Dart and Rust development environments and the Deno
runtime running on your machine, you can jump right in and run the build and
test scripts:
```
./clean_build_all.ts
./test_all.ts
```
This codebase produces two separate libraries in [Rust](https://rust-lang.org)
and [Dart](https://dart.dev).
I also use TypeScript ([Deno](https://deno.com)) for utility scripts, but
those aren't strictly necessary to build the libraries unless you want
to change something.
All of these languages have good complete development environments on Linux.

I'm using Dart 3.5.3. The code in the repo does not use Flutter, so you can
set up Dart with or without Flutter as you prefer.
Flutter is a good mobile development platform based on Dart, but is beyond
the scope of this project.
I am using Rust/Cargo 1.80.0.
Finally, I use Deno 2.0.0 (TypeScript 5.6.2) for utility scripts.
I strongly recommend it for general-purpose scripting.

## Data files

I use many serialized data sets for things like test data and pre-computed
lookup tables. The smaller and more necessary ones are checked into the repo
in the `data` directory along with conversion scripts.
[JSONC](https://code.visualstudio.com/docs/languages/json#_json-with-comments)
format is used for ease of human editing and
[MessagePack](https://msgpack.org) binary format for performance.
The very large ones (such as the poker evaluator lookup tables) are not in
the repo, but can be downloaded from the repo's
[Releases](https://githib.com/lcrocker/ojpoker/releases) area.

## Cards in text

Cards are recorded to and retrieved from text (such as the JSON5 files above)
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

# More information

More detailed documentation and dicussion of the library can be found on
the [ojpoker wiki](https://github.com/lcrocker/ojpoker/wiki) on GitHub,
and on the [OneJoker](https://onejoker.org) website.
