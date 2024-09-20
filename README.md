# ojpoker | Updated: September 20, 2024

This is code for the `OneJoker` project, aiming to create libraries for
handling playing cards and card games in general, and more spcifically poker
and its many variants.

I wrote it to provide some advantages over other libraries you may encounter
on the net:

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

I use three languages in the code, but all have very good development
environments that are easy to get started with on all platforms and do not
interfere with each other.

I'm using Dart 3.5.3. The code in the repo does not use Flutter, so you can
set up Dart with or without Flutter as you prefer. Flutter is a good mobile
development platform based on Dart, but is beyond the scope of this project.
I am using Rust/Cargo 1.80.0. It's a very simple and complete development
environment and I use very few external libraries.
Finally, I use Deno 1.46.3 (TypeScript 5.5.2) for utility scripts. It has all
the power of TypeScript without the overwhelming complexity of NodeJS. I
strongly recommend it for general-purpose scripting.

The Dart code (in the `dart` directory) serves as a "reference" implementation
for all the features of the project. Dart is strongly typed, very expressive,
and has all the features needed for that job.
The rust code (in the `rs` directory) is designed for pure performance.

There are many serialized data sets for things like test data and pre-computed
lookup tables. Each data set exists in two or more forms. They begin life in
JSON5 for easy creation, reading, and editing by humans. I then use a unique
Deno script for each file to convert it into MessagePack format, consolidating
duplicate data and removing map key names for size and performance. The final
code in each language then reads that binary and re-creates objects appropriate
for its use (which may or may not resemble the original JSON5 schema).
The `data` directory contains these files and scripts. This may sound complex,
but it's actually quite simple compared to managing schemas and generated code
needed by more powerful serialization formats like ProtoBufs.

IMPORTANT: Test code for all laguages relies on the existence of these
.msgpack files that are not checked into the repo, so those must be built
before any tests will run. Go to the `data` directory and run the `packall.ts`
script to build these files before doing anything else.

<https://deno.com> \
<https://rust-lang.org> \
<https://dart.dev> \
<https://json5.org> \
<https://msgpack.org> \

# Data representation

Some basic data representations used across all languages:

## Card

The `Card` class is an enum in Dart and a single-member anonymous tuple
class in Rust. Both use the same values:

Value     | Card
----------|-------
0         | None/unknown
1         | White/Blue third joker
2         | Black/Uncolored second joker
3         | Red/Colored default joker
4         | Ace of clubs (low, see below)
5         | Ace of diamonds (low)
6         | Ace of hearts (low)
7         | Ace of spades (low)
8         | Deuce of clubs
9         | Deuce of diamonds
10        | Deuce of hearts
11        | Deuce of spades
12        | Trey of clubs
13        | Trey of diamonds
.         | .
.         | .
.         | .
51        | Queen of spades
52        | King of clubs
53        | King of diamonds
54        | King of hearts
55        | King of spades
56        | Ace of clubs (high, see below)
57        | Ace of diamonds (high)
58        | Ace of hearts (high)
59        | Ace of spades (high)
60        | Knight/Cavalier of clubs
61        | Knight of diamonds
62        | Knight of hearts
63        | Knight of spades

The card object does NOT keep the rank and suit separate: if needed, the rank
and suit of a card can be easily calculated from the ordinals above with shift
amd mask operations (no expensive division). But with the cards ordered in
this way, extracting rank and suit is rarely necessary. You can compare ranks,
for example, just by comparing full ordinals.

Note that you can use either values 8..59 for most games in which aces are
valued high, or use 4..55 in games where they are valued low, and further
speed up comparisons. The library should make it easy for you to decide which
to use for your game; see `MasterDeck`, below.

Ordinals over 63 will probably be used to index into tables for things like
card backs or other needed graphics, but I have not reserved any yet. If you do,
let me know, and we'll try to standardize.

## Rank, Suit

Ranks and suits are simple enums with a few basic methods. The specific values
are important, though, to keep algorithms and data files compatible among all
the languages.

Value       | Suit
------------|------
0           | None/unknown
1           | Club
2           | Diamond
3           | Heart
4           | Spade

Value       | Rank
------------|------
0           | None/Unknown
1           | Ace (low)
2           | Deuce
3           | Trey
4           | Four
5           | Five
6           | Six
7           | Seven
8           | Eight
9           | Nine
10          | Ten
11          | Jack
12          | Queen
13          | King
14          | Ace (high)
15          | Knight/Cavalier

Jokers have neither rank nor suit. American decks of cards typically contain
two jokers: one is drawn in plain black ink, and the other is more colorful.
We call the former the #2 "black" joker, and the latter the #3 "red" joker,
for games like Dou Dizhou which distinguish them. I don't know of any games
requiring three distinguished jokers, but Unicode seems to think there is, so
that's my joker #1. If there is just one joker, we use the red/colorful one.

Games that use the Knight/Cavalier usually have only three face cards, using
the Knight in place of the Queen. For those games it's probably best to just
use the 12 rank and adjust visually on output. But the graphic tables still
need a location to put the image, so I put them up higher, and there are games
and decks that use all four (e.g. the Tarot de Marseille).

Aces are high by default; hands being read from text files, for example, will
put aces into the 56..59 slots, and must be adjusted afterwards for games
using low aces.

No special provisions are made for German, Swiss, or Italian-suited playing
cards, or for Uber and Under face cards. Using the existing rank and suit
numbers with just different visuals should suffice. The same perhaps cannot
be said for my exclusion of pip cards above ten and Tarot trumps. If there is
sufficient interest in standardizing these and providing code for them, I'd
be happy to add them.

## MasterDeck

Every game or simulation begins with a full deck of some kind that I call a
"master" deck. This determines the initial state of the "live" decks actually
used for dealing cards, and such things as which cards are allowed, whether
duplicates are allowed, etc. If you are playing Skat, for exmaple, you would
begin with a 32-card German deck with no cards below seven. Error checking in
the system would detect if, say, a five were to appear in a hand or deck
associated with that master.

Master decks are generally named after the place where they are commonly used,
or the specific games they are designed for playing, and you choose the master
for your game by name. The common English/American 52-card deck most of you
are familiar with can be summoned by the names "english" or "poker" or
"bridge", for example. If your game is California lowball, where aces are low
and a joker is included in the deck, ask for a "lowball" deck.

## CardList, Deck, Hand

A `CardList` is just a simple array of card objects, with the usual functions
of expandable arrays (`List` for Dart, `Vec` for Rust). `Deck`s and `Hand`s
are more spcialized objects containing a `CardList`, and features more suited
to their task. In particular, each is associated with a `MasterDeck` that
determines which cards are allowed, etc. `Deck`s are initialized and refilled
from this master. `Hand`s are given cards from a `Deck` from which they are
created, or they can be "orphans", giving themselves whatever cards they want
for simulations and such. The `Hand` object can be used for any other group
of cards that's not a deck, such as a Gin discard pile, a Texas Hold'em board,
a solitaire tableau, etc.

## Cards in text

Cards are recorded to and retrieved from text using the popular convention:
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
It is never produced on output. Invalid values produce `??`.
We can also produce Unicode suit symbols and single-code cards, which may
come in handy for producing documentation.
