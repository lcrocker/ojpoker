//
/// General playing-card handling classes.
/// 
/// ## Cards in text
///
/// Cards are recorded to and retrieved from text (such as the JSONC files)
/// using the popular convention:
/// Each card is a 2-letter abbreviation with a one-letter uppercase rank and
/// a one-letter lowercase suit. Ranks are `2`, `3`, ... `9`, `T`, `J`, `Q`,
/// `K`, and `A`. Suits are `c`, `d`, `h`, and `s`. This format is used in
/// lots of data files of card games around the net, such as
/// PHH (<https://arxiv.org/html/2312.11753v2>) poker hand history format.
/// "AsKsQsJsTs" is a royal flush, for example.
/// I also recognize `C` for knight/cavalier, `Jk` for joker, `Jb` for the
/// black/uncolored joker in games that distinguish between them, and `Jw`
/// for the third "white" joker.
/// Whitespace between cards is ignored, but is not allowed between rank
/// and suit. It is never produced on output.
/// We can also produce Unicode suit symbols and single-code cards, which may
/// come in handy for producing documentation.
/// {@category cards}
library;

export 'suit.dart';
export 'rank.dart';
export 'card.dart';
export 'master_deck.dart';
export 'hashes.dart';
export 'deck.dart';
export 'hand.dart';