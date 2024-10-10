// ignore_for_file: constant_identifier_names

/// Enum for card ranks and their basic methods.
/// 
/// As with suits, do not
/// change the numbers.
/// {@category cards}
enum Rank implements Comparable<Rank> {
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

  @override
  int compareTo(Rank other) {
    return index - other.index;
  }

  bool operator <(Rank other) {
    return index < other.index;
  }

  bool operator <=(Rank other) {
    return index <= other.index;
  }

  bool operator >(Rank other) {
    return index > other.index;
  }

  bool operator >=(Rank other) {
    return index >= other.index;
  }
}
