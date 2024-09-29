# ojpoker | Updated: September 28, 2024

This is code for the [OneJoker](https://onejoker.org) project,
aiming to create libraries and other digital tools for handling playing cards
and card games in general, and more spcifically poker and its many variants.

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

This codebase produces two separate libraries in [Rust](https://rust-lang.org)
and [Dart](https://dart.dev).
If you are only interested in one of those, you will not have to deal with
the other if you don't want to.
I also use TypeScript ([Deno](https://deno.com)) for utility scripts, but
those too aren't strictly necessary to build the libraries unless you want
to change something or run tests.
All of these languages have good complete development environments on Linux.
I hear those for Windows and Mac are also quite good, but I can't vouch for
that personally.

I'm using Dart 3.5.3. The code in the repo does not use Flutter, so you can
set up Dart with or without Flutter as you prefer.
Flutter is a good mobile development platform based on Dart, but is beyond
the scope of this project.
I am using Rust/Cargo 1.80.0.
Finally, I use Deno 1.46.3 (TypeScript 5.5.2) for utility scripts.
It has all the power of TypeScript without the cruft of NodeJS.
I strongly recommend it for general-purpose scripting.

The Dart code (in the `dart` directory) serves as a "reference" implementation
for all the features of the project. Dart is strongly typed, very expressive,
and has all the features needed for that job.
The rust code (in the `rust` directory) is designed for pure performance.

There are many serialized data sets for things like test data and pre-computed
lookup tables. Each data set exists in two or more forms.
They begin life in [JSON5](https://json5.org) for easy creation, reading, and
editing by humans.
I then use a unique Deno script for each file to convert it into
[MessagePack](https://msgpack.org) binary format, consolidating duplicate data
and removing map key names for size and performance.
The final code in each language then reads that binary and re-creates objects
appropriate for its use (which may or may not resemble the original schema).
The `data` directory contains these files and scripts.
This may sound complex, but it's actually quite simple compared to managing
schemas and generated code needed by more powerful serialization formats
like ProtoBufs.

IMPORTANT: Test code for all languages relies on the existence of these
.msgpack files that are not checked into the repo, so those must be built
before any tests will run.
Go to the `data` directory and run the `packall.ts` script to build these
files before doing anything else.

There are also some source code files built by scripts, but these are checked
into the repo so you can build all the code without having to generate them.
If you do install Deno to run the scripts, the script `clean_build_all.ts` at
the root will remove all previously generated files and rebuild them.

# Data representation

Some basic data representations used across all languages:

## Card

The `Card` class is an enum in Dart and a single-member anonymous tuple
class in Rust. Both use the same values:

Value     | Card
----------|-------
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
46        | Jack of Hearts
47        | Jack of spades
48        | Knight / Cavalier of spades
49        | Knight of diamonds
50        | Knight of hearts
51        | Knight of spades
52        | Queen of clubs
53        | Queen of diamonds
54        | Queen of hearts
55        | Queen of spades
56        | King of clubs
57        | King of diamonds
58        | King of hearts
59        | King of spades
60        | Ace of clubs (high, see below)
61        | Ace of diamonds (high)
62        | Ace of hearts (high)
63        | Ace of spades (high)
64..47    | (TBD) Tarot Nouveau trumps
48..71    | (TBD) Tarot de Marseilles trumps?

The card object does NOT keep the rank and suit separate: if needed, the rank
and suit of a card can be easily calculated from the ordinals above with shift
and mask operations (no expensive division). But with the cards ordered in
this way, extracting rank and suit is rarely necessary. You can compare ranks,
for example, just by comparing full ordinals.

Note that you can use either values 8..63 for most games in which aces are
valued high, or use 4..59 in games where they are valued low, and further
speed up comparisons. The library should make it easy for you to decide which
to use for your game; see `MasterDeck`, below.

Ordinals over 71 will probably be used to index into tables for things like
card backs or other needed graphics, but I have not reserved any yet.
If you do, let me know, and we'll try to standardize.

## Suit, Rank

Ranks and suits are simple enums with a few basic methods.
The specific values are important, though, to keep algorithms and data files
compatible between languages.

Value       | Suit (French, Latin [ES, IT, PT], German, Swiss, Tarot)
------------|------
1           | Clubs / Batons / Acorns / Acorns / Wands
2           | Diamonds / Coins / Bells / Bells / Pentacles
3           | Hearts / Cups / Hearts / Roses / Cups
4           | Spades / Swords / Leaves / Shields / Swords

Value       | Rank
------------|------
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
11          | Jack / Under
12          | Knight / Cavalier / Ober
13          | Queen
14          | King
15          | Ace (high)

Jokers and tarot trumps have neither rank nor suit.
English/American decks of cards typically contain two jokers: one is drawn
in plain black ink, and the other is more colorful.
We call the former the #2 "black" joker, and the latter the #3 "red" joker,
for games like Dou Dizhou which distinguish them.
I don't know of any games requiring three distinguished jokers, but
[Unicode](https://https://en.wikipedia.org/wiki/Playing_cards_in_Unicode)
seems to think there is, so that's my joker #1.
If there is just one joker, we use the red/colorful one.

Aces are high by default; hands being read from text files, for example,
will put aces into the 60..63 slots, and must be adjusted afterwards for
games using low aces (the library mostly handles this automatically).

## MasterDeck

Every game or simulation begins with a full deck of some kind that I call a
`MasterDeck`. This determines the initial state of the "live" decks actually
used for dealing cards, and such things as which cards are allowed, whether
duplicates are allowed, etc. If you are playing Skat, for exmaple, you would
begin with a 32-card German deck with no cards below seven. Error checking in
the system would detect if, say, a five were to appear in a hand or deck
associated with that master. The master also determines whether aces are high
or low, and does appropriate adjustments and error checking.

Master decks are generally named after the place where they are commonly used,
or the specific games they are designed for playing, and you choose the master
for your game by name. The common English/American 52-card deck most of you
are familiar with can be summoned by the names "english" or "poker" or
"bridge", for example. If your game is California lowball, where aces are low
and a joker is included in the deck, ask for a "lowball" deck.

## CardStack, Deck, Hand

A `CardStack` is just a simple LIFO stack of card objects, with many of the
usual functions of arrays or stacks or queues in many languages. These can be
used for whole decks, player hands, discard piles, Texas Hold'em boards,
active tricks, solitaire tableaux, etc. `Deck`s and `Hand`s are more
spcialized objects containing a `CardStack` and other features suited to
their task. In particular, each is associated with a `MasterDeck` that
determines which cards are allowed, etc. `Deck`s are initialized and refilled
from this master. `Hand`s are given cards from a `Deck` from which they are
created. If you need "orphan" hands that have no such error checking for
simulations and such, use `CardStack`.

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
It is never produced on output.
We can also produce Unicode suit symbols and single-code cards, which may
come in handy for producing documentation.
