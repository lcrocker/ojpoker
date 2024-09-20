// ignore_for_file: constant_identifier_names

/// Enum for card suits and their most basic methods.
/// The actual numbers are important here: change them and you will break
/// some tests and compatibility with the Rust code.
enum Suit {
  None, // 0
  Club, // 1
  Diamond, // 2
  Heart, // 3
  Spade; // 4

  /// Create one from text character or unicode symbol.
  static Suit fromChar(String c) {
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
        return None;
    }
  }

  /// Render as text
  String toChar() {
    if (index < 0 || index > 4) return '?';
    return ['?', 'c', 'd', 'h', 's'][index];
  }

  /// Render as unicode symbol
  String toUnicode() {
    if (index < 0 || index > 4) return '?';
    return ['?', '♣', '♦', '♥', '♠'][index];
  }

  /// Full English name
  String get name {
    if (index < 0 || index > 4) return "?";
    return ["?", "club", "diamond", "heart", "spade"][index];
  }

  /// Full English plural name, not really needed but for consistency
  String get plural {
    if (index < 0 || index > 4) return "?";
    return ["?", "clubs", "diamonds", "hearts", "spades"][index];
  }

  /// For consistency with rank, not really needed
  String get article {
    return "a";
  }
}

/// Enum for card ranks and their basic methods. As with suits, do not
/// change the numbers.
enum Rank {
  None, // 0
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
  Queen, // 12
  King, // 13
  Ace, // 14
  Knight; // 15

  /// Create one from text character.
  static Rank fromChar(String c) {
    switch (c) {
      case '1': // Extension for testing never really used
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
      case 'Q':
        return Queen;
      case 'K':
        return King;
      case 'A':
        return Ace;
      case 'C':
        return Knight;
      default:
        return None;
    }
  }

  /// Render as text
  String toChar() {
    if (index < 0 || index > 15) return '?';
    return [
      '?',
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
      'Q',
      'K',
      'A',
      'C'
    ][index];
  }

  /// Full English name
  String get name {
    if (index < 0 || index > 15) return "?";
    return [
      "?",
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
      "queen",
      "king",
      "ace",
      "knight"
    ][index];
  }

  /// Full English plural name (to deal with "sixes")
  String get plural {
    if (index < 0 || index > 15) return "?";
    return [
      "?",
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
      "queens",
      "kings",
      "aces",
      "knights"
    ][index];
  }

  /// Indefinite article (to deal with "an eight, an ace")
  String get article {
    if (index == 1 || index == 8 || index == 14) return "an";
    return "a";
  }
}

/// Enum for cards and their basic methods. Numbers *very* important here,
/// not just for compatibility but specific algorithms will fail if changed.
enum Card {
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
  QueenOfClubs, // 48
  QueenOfDiamonds, // 49
  QueenOfHearts, // 50
  QueenOfSpades, // 51
  KingOfClubs, // 52
  KingOfDiamonds, // 53
  KingOfHearts, // 54
  KingOfSpades, // 55
  AceOfClubs, // 56
  AceOfDiamonds, // 57
  AceOfHearts, // 58
  AceOfSpades, // 59
  KnightOfClubs, // 60
  KnightOfDiamonds, // 61
  KnightOfHearts, // 62
  KnightOfSpades; // 63

  /// Create one from integer value
  static Card fromInt(int v) {
    if (v < 1 || v > 63) return None;
    return Card.values[v];
  }

  /// Create one from rank and suit
  static Card fromRankSuit(Rank r, Suit s) {
    if (r == Rank.None || s == Suit.None) return None;
    return Card.values[(r.index << 2) | (s.index - 1)];
  }

  /// What's the rank of this card? Note that we use a shift on the number
  /// value; that's why the numbers are important. Note that jokers do not
  /// have rank or suit.
  Rank get rank {
    if (index < 4 || index > 63) return Rank.None;
    return Rank.values[index >> 2];
  }

  /// What's the rank of this card, with low aces made high?
  Rank get highRank {
    if (index < 4 || index > 63) return Rank.None;
    if (index < 8) return Rank.Ace;
    return Rank.values[index >> 2];
  }

  /// What's the suit of this card? Jokers have no suit.
  Suit get suit {
    if (index < 4 || index > 63) return Suit.None;
    return Suit.values[1 + (index & 3)];
  }

  /// Is this an actual card (and not None or an invalid number)?
  bool get isCard {
    return index > 0 && index < 64;
  }

  /// Is this card an ace?
  bool get isAce {
    return (index >= 4 && index <= 7) || (index >= 56 && index <= 59);
  }

  /// Is this card a joker?
  bool get isJoker {
    return (index >= 1 && index <= 3);
  }

  /// Is this card red? Note that while jokers have no suit, there is a
  /// black one and a red one.
  bool get isRed {
    if (3 == index) return true;
    if (index < 4 || index > 63) return false;
    return (index & 3) == 1 || (index & 3) == 2;
  }

  /// Is this card black? Note that while jokers have no suit, there is a
  /// black one and a red one.
  bool get isBlack {
    if (2 == index) return true;
    if (index < 4 || index > 63) return false;
    return (index & 3) == 0 || (index & 3) == 3;
  }

  /// Render to two-character text in standard format
  String toText() {
    if (!isCard) return "??";
    if (1 == index) return "Jw";
    if (2 == index) return "Jb";
    if (3 == index) return "Jk";
    return "${rank.toChar()}${suit.toChar()}";
  }

  /// Render to two-character text with unicode suit symbols
  String toUnicode() {
    if (!isCard) return "??";
    if (1 == index) return "Jw";
    if (2 == index) return "Jb";
    if (3 == index) return "Jk";
    return "${rank.toChar()}${suit.toUnicode()}";
  }

  /// Full English name
  String get fullName {
    if (!isCard) return "unknown";
    if (1 == index) return "white joker";
    if (2 == index) return "black joker";
    if (3 == index) return "joker";
    return "${rank.name} of ${suit.plural}";
  }

  /// Make high aces low, leave other cards alone
  static Card lowAceFix(Card c) {
    return Card
        .values[(c.index >= 56 && c.index <= 59) ? c.index - 52 : c.index];
  }

  /// Make low aces high, leave other cards alone
  static Card highAceFix(Card c) {
    return Card.values[(c.index >= 4 && c.index <= 7) ? c.index + 52 : c.index];
  }

  @override
  String toString() {
    return toText();
  }
}

/// Parse a string of card text into a sequence of cards. Whitespace is
/// ignored between cards, but is not allowed between rank and suit.
/// Cards may be enclosed in square brackets.
Iterable<Card> cardsFromText(String text) sync* {
  int state = 0;
  bool bracketAllowed = true;
  Suit s = Suit.None;
  Rank r = Rank.None;

  for (String c in text.runes.map((r) => String.fromCharCode(r))) {
    switch (state) {
      case 0: // skip whitespace or one [, look for rank
        if ("[" == c) {
          if (!bracketAllowed) {
            return;
          }
          bracketAllowed = false;
          break;
        }
        if (" " == c) {
          break;
        }
        bracketAllowed = false;

        if ("J" == c) {
          state = 1;
          break;
        }
        r = Rank.fromChar(c);
        if (Rank.None == r) {
          return;
        }
        state = 2;
        break;
      case 1: // got J, look for joker color or jack suit
        if ("k" == c || "r" == c) {
          yield Card.Joker;
          state = 0;
          break;
        }
        if ("b" == c) {
          yield Card.BlackJoker;
          state = 0;
          break;
        }
        if ("w" == c) {
          yield Card.WhiteJoker;
          state = 0;
          break;
        }
        r = Rank.Jack;
        s = Suit.fromChar(c);
        if (Suit.None == s) {
          return;
        }
        yield Card.fromRankSuit(r, s);
        state = 0;
        break;
      case 2: // got rank, look for suit
        s = Suit.fromChar(c);
        if (Suit.None == s) {
          return;
        }
        assert(Rank.None != r);
        yield Card.fromRankSuit(r, s);
        state = 0;
        break;
      default:
        assert(false);
    }
  }
}
