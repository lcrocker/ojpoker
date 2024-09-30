// ignore_for_file: constant_identifier_names

/// Enum for card suits and their most basic methods.
/// The actual numbers are important here: change them and you will break
/// some tests and compatibility with the Rust code.
enum Suit {
  None, // 0 not a valid suit, but Dart won't let me assign the numbers
  Club, // 1
  Diamond, // 2
  Heart, // 3
  Spade; // 4

  /// Create one from text character or unicode symbol.
  static Suit? fromChar(String c) {
    switch (c) {
      case 'c':
      case '♣':
        return Club;
      case 'd':
      case '♦':
        return Diamond;
      case 'h':
      case '♥':
        return Heart;
      case 's':
      case '♠':
        return Spade;
      default:
        return null;
    }
  }

  /// Render as text
  String toChar() {
    assert(index >= Suit.Club.index && index <= Suit.Spade.index);
    return ['c', 'd', 'h', 's'][index - 1];
  }

  /// Render as unicode symbol
  String toUnicode() {
    assert(index >= Suit.Club.index && index <= Suit.Spade.index);
    return ['♣', '♦', '♥', '♠'][index - 1];
  }

  /// Full English name
  String get name {
    assert(index >= Suit.Club.index && index <= Suit.Spade.index);
    return ["club", "diamond", "heart", "spade"][index - 1];
  }

  /// Full English plural name, not really needed but for consistency
  String get plural {
    assert(index >= Suit.Club.index && index <= Suit.Spade.index);
    return ["clubs", "diamonds", "hearts", "spades"][index - 1];
  }

  /// For consistency with rank, not really needed
  String get article {
    return "a";
  }
}

/// Enum for card ranks and their basic methods. As with suits, do not
/// change the numbers.
enum Rank {
  None, // 0 not a valid rank
  LowAce, // 1
  Deuce, // 2
  Trey, // 3
  Four, // 4
  Five, // 5
  Six, // 6
  Seven, // 7
  Eight, // 8
  Nine, // 9
  Ten, // 10
  Jack, // 11
  Knight, // 12
  Queen, // 13
  King, // 14
  Ace; // 15

  /// Create one from text character.
  static Rank? fromChar(String c) {
    switch (c) {
      case '1':
        return LowAce;
      case '2':
        return Deuce;
      case '3':
        return Trey;
      case '4':
        return Four;
      case '5':
        return Five;
      case '6':
        return Six;
      case '7':
        return Seven;
      case '8':
        return Eight;
      case '9':
        return Nine;
      case 'T':
        return Ten;
      case 'J':
        return Jack;
      case 'C':
        return Knight;
      case 'Q':
        return Queen;
      case 'K':
        return King;
      case 'A':
        return Ace;
      default:
        return null;
    }
  }

  /// Render as text
  String toChar() {
    assert(index >= Rank.LowAce.index && index <= Rank.Ace.index);
    return [
      'A',
      '2',
      '3',
      '4',
      '5',
      '6',
      '7',
      '8',
      '9',
      'T',
      'J',
      'C',
      'Q',
      'K',
      'A',
    ][index - 1];
  }

  /// Full English name
  String get name {
    assert(index >= Rank.LowAce.index && index <= Rank.Ace.index);
    return [
      "ace",
      "deuce",
      "trey",
      "four",
      "five",
      "six",
      "seven",
      "eight",
      "nine",
      "ten",
      "jack",
      "knight",
      "queen",
      "king",
      "ace",
    ][index - 1];
  }

  /// Full English plural name (to deal with "sixes")
  String get plural {
    assert(index >= Rank.LowAce.index && index <= Rank.Ace.index);
    return [
      "aces",
      "deuces",
      "treys",
      "fours",
      "fives",
      "sixes",
      "sevens",
      "eights",
      "nines",
      "tens",
      "jacks",
      "knights",
      "queens",
      "kings",
      "aces",
    ][index - 1];
  }

  /// Indefinite article (to deal with "an eight, an ace")
  String get article {
    if (index == Rank.LowAce.index ||
        index == Rank.Eight.index ||
        index == Rank.Ace.index) return "an";
    return "a";
  }
}

// Patterns for matching cards as text
final _brackets = RegExp(r'^\s*\[([^\]]+)\]');
final _oneCard = RegExp(r'\s*(Jk|Jb|Jw|([1-9TJCQKA])([cdhs]))');

/// Enum for cards and their basic methods. Numbers *very* important here,
/// not just for compatibility but specific algorithms will fail if changed
/// (see README).
enum Card implements Comparable<Card> {
  None, // 0
  WhiteJoker, // 1
  BlackJoker, // 2
  Joker, // 3
  LowAceOfClubs, // 4
  LowAceOfDiamonds, // 5
  LowAceOfHearts, // 6
  LowAceOfSpades, // 7
  DeuceOfClubs, // 8
  DeuceOfDiamonds, // 9
  DeuceOfHearts, // 10
  DeuceOfSpades, // 11
  TreyOfClubs, // 12
  TreyOfDiamonds, // 13
  TreyOfHearts, // 14
  TreyOfSpades, // 15
  FourOfClubs, // 16
  FourOfDiamonds, // 17
  FourOfHearts, // 18
  FourOfSpades, // 19
  FiveOfClubs, // 20
  FiveOfDiamonds, // 21
  FiveOfHearts, // 22
  FiveOfSpades, // 23
  SixOfClubs, // 24
  SixOfDiamonds, // 25
  SixOfHearts, // 26
  SixOfSpades, // 27
  SevenOfClubs, // 28
  SevenOfDiamonds, // 29
  SevenOfHearts, // 30
  SevenOfSpades, // 31
  EightOfClubs, // 32
  EightOfDiamonds, // 33
  EightOfHearts, // 34
  EightOfSpades, // 35
  NineOfClubs, // 36
  NineOfDiamonds, // 37
  NineOfHearts, // 38
  NineOfSpades, // 39
  TenOfClubs, // 40
  TenOfDiamonds, // 41
  TenOfHearts, // 42
  TenOfSpades, // 43
  JackOfClubs, // 44
  JackOfDiamonds, // 45
  JackOfHearts, // 46
  JackOfSpades, // 47
  KnightOfClubs, // 48
  KnightOfDiamonds, // 49
  KnightOfHearts, // 50
  KnightOfSpades, // 51
  QueenOfClubs, // 52
  QueenOfDiamonds, // 53
  QueenOfHearts, // 54
  QueenOfSpades, // 55
  KingOfClubs, // 56
  KingOfDiamonds, // 57
  KingOfHearts, // 58
  KingOfSpades, // 59
  AceOfClubs, // 60
  AceOfDiamonds, // 61
  AceOfHearts, // 62
  AceOfSpades; // 63

  /// Create one from integer value
  static Card? fromInt(int v) {
    if (v < 1 || v > 63) return null;
    return Card.values[v];
  }

  /// Create one from rank and suit
  static Card fromRankSuit(Rank r, Suit s) {
    return Card.values[(r.index << 2) | (s.index - 1)];
  }

  /// Create one from text representation
  static Card? fromText(String text) {
    var match = _oneCard.firstMatch(text);
    if (match == null) return null;

    if (match.group(1) != null) {
      switch (match.group(1)) {
        case "Jk":
          return Joker;
        case "Jb":
          return BlackJoker;
        case "Jw":
          return WhiteJoker;
        default:
          if (match.group(2) != null && match.group(3) != null) {
            var r = Rank.fromChar(match.group(2)!);
            var s = Suit.fromChar(match.group(3)!);
            assert(r != null && s != null);
            return fromRankSuit(r!, s!);
          }
      }
    }
    return null;
  }

  /// What's the rank of this card? Note that we use a shift on the number
  /// value; that's why the numbers are important. Note that jokers do not
  /// have rank or suit.
  Rank? get rank {
    if (index < Card.LowAceOfClubs.index || index > Card.AceOfSpades.index) {
      return null;
    }
    return Rank.values[index >> 2];
  }

  /// What's the rank of this card, with low aces made high?
  Rank? get highRank {
    if (index < Card.LowAceOfClubs.index || index > Card.AceOfSpades.index) {
      return null;
    }
    if (index < Card.DeuceOfClubs.index) return Rank.Ace;
    return Rank.values[index >> 2];
  }

  /// What's the suit of this card? Jokers have no suit.
  Suit? get suit {
    if (index < Card.LowAceOfClubs.index || index > Card.AceOfSpades.index) {
      return null;
    }
    return Suit.values[1 + (index & 3)];
  }

  /// Is this an actual card (and not None or an invalid number)?
  bool get isCard {
    return index >= Card.WhiteJoker.index && index <= Card.AceOfSpades.index;
  }

  /// Is this card an ace?
  bool get isAce {
    return (index >= Card.LowAceOfClubs.index &&
            index <= Card.LowAceOfSpades.index) ||
        (index >= Card.AceOfClubs.index && index <= Card.AceOfSpades.index);
  }

  /// Is this card a joker?
  bool get isJoker {
    return (index >= Card.WhiteJoker.index && index <= Card.Joker.index);
  }

  /// Is this card red? Note that while jokers have no suit, there is a
  /// black one and a red one.
  bool get isRed {
    if (index == Card.Joker.index) return true;
    if (index < Card.LowAceOfClubs.index || index > Card.AceOfSpades.index) {
      return false;
    }
    return (index & 3) == Suit.Diamond.index - 1 ||
        (index & 3) == Suit.Heart.index - 1;
  }

  /// Is this card black? Note that while jokers have no suit, there is a
  /// black one and a red one.
  bool get isBlack {
    if (index == Card.BlackJoker.index) return true;
    if (index < Card.LowAceOfClubs.index || index > Card.AceOfSpades.index) {
      return false;
    }
    return (index & 3) == Suit.Club.index - 1 ||
        (index & 3) == Suit.Spade.index - 1;
  }

  /// Render to two-character text in standard format
  String toText() {
    assert(isCard);
    if (Card.WhiteJoker.index == index) return "Jw";
    if (Card.BlackJoker.index == index) return "Jb";
    if (Card.Joker.index == index) return "Jk";
    return "${rank!.toChar()}${suit!.toChar()}";
  }

  /// Render to two-character text with unicode suit symbols
  String toUnicode() {
    assert(isCard);
    if (Card.WhiteJoker.index == index) return "Jw";
    if (Card.BlackJoker.index == index) return "Jb";
    if (Card.Joker.index == index) return "Jk";
    return "${rank!.toChar()}${suit!.toUnicode()}";
  }

  static final List<String> unicodeSingles = [
    "🃟",
    "🂿",
    "🃏",
    "🃑",
    "🃁",
    "🂱",
    "🂡",
    "🃒",
    "🃂",
    "🂲",
    "🂢",
    "🃓",
    "🃃",
    "🂳",
    "🂣",
    "🃔",
    "🃄",
    "🂴",
    "🂤",
    "🃕",
    "🃅",
    "🂵",
    "🂥",
    "🃖",
    "🃆",
    "🂶",
    "🂦",
    "🃗",
    "🃇",
    "🂷",
    "🂧",
    "🃘",
    "🃈",
    "🂸",
    "🂨",
    "🃙",
    "🃉",
    "🂹",
    "🂩",
    "🃚",
    "🃊",
    "🂺",
    "🂪",
    "🃛",
    "🃋",
    "🂻",
    "🂫",
    "🃜",
    "🃌",
    "🂼",
    "🃜",
    "🃝",
    "🃍",
    "🂽",
    "🂭",
    "🃞",
    "🃎",
    "🂾",
    "🂮",
    "🃑",
    "🃁",
    "🂱",
    "🂡",
  ];

  /// Render to single-character unicode card image.
  String toUnicodeSingle() {
    assert(isCard);
    return unicodeSingles[index - 1];
  }

  /// Full English name
  String get fullName {
    assert(isCard);
    if (Card.WhiteJoker.index == index) return "white joker";
    if (Card.BlackJoker.index == index) return "black joker";
    if (Card.Joker.index == index) return "joker";
    return "${rank!.name} of ${suit!.plural}";
  }

  /// Make high aces low, leave other cards alone
  static Card lowAceFix(Card c) {
    if (c.index >= Card.AceOfClubs.index && c.index <= Card.AceOfSpades.index) {
      return Card
          .values[c.index - (Card.AceOfClubs.index - Card.LowAceOfClubs.index)];
    }
    return c;
  }

  /// Make low aces high, leave other cards alone
  static Card highAceFix(Card c) {
    if (c.index >= Card.LowAceOfClubs.index &&
        c.index <= Card.LowAceOfSpades.index) {
      return Card
          .values[c.index + (Card.AceOfClubs.index - Card.LowAceOfClubs.index)];
    }
    return c;
  }

  @override
  String toString() {
    return toText();
  }

  @override
  int compareTo(Card other) {
    return index - other.index;
  }

  bool operator <(Card other) {
    return index < other.index;
  }

  bool operator <=(Card other) {
    return index <= other.index;
  }

  bool operator >(Card other) {
    return index > other.index;
  }

  bool operator >=(Card other) {
    return index >= other.index;
  }
}

/// Parse a string of card text into a sequence of cards. Whitespace is
/// ignored between cards, but is not allowed between rank and suit.
/// Cards may be enclosed in square brackets.
Iterable<Card> cardsFromText(String text) sync* {
  var input = text;

  var match = _brackets.firstMatch(input);
  if (match != null && match.group(1)!.isNotEmpty) {
    input = match.group(1)!;
  }
  var matches = _oneCard.allMatches(input);
  for (var match in matches) {
    if (match.group(1) == null) {
      return;
    }
    if (match.group(1) == "Jk") {
      yield Card.Joker;
      continue;
    }
    if (match.group(1) == "Jb") {
      yield Card.BlackJoker;
      continue;
    }
    if (match.group(1) == "Jw") {
      yield Card.WhiteJoker;
      continue;
    }
    if (match.group(2) == null || match.group(3) == null) {
      return;
    }
    var r = Rank.fromChar(match.group(2)!);
    var s = Suit.fromChar(match.group(3)!);
    if (r == null || s == null) {
      return;
    }
    yield Card.fromRankSuit(r, s);
  }
}
