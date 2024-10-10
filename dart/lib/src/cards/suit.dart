// ignore_for_file: constant_identifier_names

/// Enum for card suits and their most basic methods.
/// 
/// The actual numbers are important here: change them and you will break
/// some tests and compatibility with the Rust code.
/// {@category cards}
enum Suit implements Comparable<Suit> {
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

  @override
  int compareTo(Suit other) {
    return index - other.index;
  }

  bool operator <(Suit other) {
    return index < other.index;
  }

  bool operator <=(Suit other) {
    return index <= other.index;
  }

  bool operator >(Suit other) {
    return index > other.index;
  }

  bool operator >=(Suit other) {
    return index >= other.index;
  }
}
