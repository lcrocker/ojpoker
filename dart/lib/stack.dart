import 'package:onejoker/card.dart';
import 'package:onejoker/utils.dart';

/// A `CardStack` is the basic card collection type for the library, used to
/// implement whole decks, player hands, discard piles, Texas Hold'em boards,
/// active tricks, solitaire tableaux, etc. It is a simple LIFO stack of
/// cards, with a subset of the sorts of methods of native arrays or stacks
/// or queues in many languages, but tuned with cards and simulations in mind.
/// The generic `CardStack` should be used sparingly: its enclosing classes,
/// `Hand` and `Deck`, are to be preferred for most uses as they have more
/// error checking and specialized shortcuts.
///
/// Cards are moved between stacks with `pop()` (which removes the top card
/// of the stack) and `push()` (which adds a card to the top of the stack).
/// Stacks are indexed and displayed top-down, so, for example:
/// ```
/// hand = CardStack();
/// hand.push(Card.FourOfSpades);
/// hand.push(Card.Joker);
///
/// print("${hand}, ${hand[0]}");
/// ```
/// will print `Jk4s, Joker`.
/// Cards added to the stack as a list will be added as a unit, so:
/// ```
/// hand = CardStack.fromText("5sJc9d");
/// hand.pushN([Card.QueenOfClubs, Card.KingOfClubs]);
/// print("${hand}");
/// ```
/// will print `QcKc5sJc9d`. There are also `insertX()` and `removeX()`
/// methods, but these are less efficient than `push()` and `pop()`.
class CardStack {
  List<Card> _cards = [];

  /// Create new empty stack.
  CardStack();

  /// Create new stack from list of cards, e.g.:
  /// ```
  /// var hand = CardStack.fromList([ Card.FourOfSpades, Card.Joker ]);
  /// ```
  CardStack.fromList(List<Card> c) {
    _cards = c.reversed.toList();
  }

  /// Create new stack from text representation, e.g.:
  /// ```
  /// var hand = CardStack.fromText("4sJk9d");
  /// ```
  CardStack.fromText(String text) {
    _cards = cardsFromText(text).toList().reversed.toList();
  }

  /// Duplicate this stack.
  CardStack clone() {
    var s = CardStack();
    s._cards = _cards.toList();
    return s;
  }

  /// Return list copy (reversed).
  List<Card> toList() {
    return _cards.reversed.toList();
  }

  /// Fix cards for decks with low aces.
  void lowAceFix() {
    for (int i = 0; i < _cards.length; i += 1) {
      _cards[i] = Card.lowAceFix(_cards[i]);
    }
  }

  /// How many cards currently in the stack?
  int len() {
    return _cards.length;
  }

  /// Is list empty?
  bool isEmpty() {
    return _cards.isEmpty;
  }

  /// Does the stack contain the specific card?
  bool contains(Card card) {
    return _cards.contains(card);
  }

  /// Empty the stack.
  void clear() {
    _cards.clear();
  }

  /// Return the card at position `index`.
  Card? cardAt(int index) {
    if (index < 0 || index >= _cards.length) return null;
    return _cards[_cards.length - 1 - index];
  }

  /// Add card to top of the stack.
  void push(Card card) {
    _cards.add(card);
  }

  /// Remove card from the top of the stack.
  Card? pop() {
    if (_cards.isEmpty) return null;
    return _cards.removeLast();
  }

  /// Add *n* cards to the top of the stack as a unit.
  void pushN(List<Card> cards) {
    for (int i = cards.length - 1; i >= 0; i -= 1) {
      _cards.add(cards[i]);
    }
  }

  /// Remove *n* cards from the top of the stack as a list.
  /// All or none: if we can't return `n`, return null.
  List<Card>? popN(int n) {
    if (_cards.length < n) return null;

    List<Card> ret = [];
    for (int i = 0; i < n; i += 1) {
      ret.add(_cards.removeLast());
    }
    return ret;
  }

  /// Insert card at position `index`.
  void insertAt(int index, Card card) {
    assert(index >= 0 && index <= _cards.length);
    _cards.insert(_cards.length - index, card);
  }

  /// Insert card at bottom of stack.
  void insertAtEnd(Card card) {
    _cards.insert(0, card);
  }

  /// Remove card at position `index`.
  Card? removeAt(int index) {
    if (index < 0 || index >= _cards.length) return null;
    return _cards.removeAt(_cards.length - 1 - index);
  }

  /// Remove card at bottom of stack.
  Card? removeAtEnd() {
    if (_cards.isEmpty) return null;
    return _cards.removeAt(0);
  }

  /// Remove top-most specific card from list.
  bool removeCard(Card card) {
    for (var i = _cards.length - 1; i >= 0; i -= 1) {
      if (_cards[i] == card) {
        _cards.removeAt(i);
        return true;
      }
    }
    return false;
  }

  /// Randomize order of cards in the stack.
  void shuffle() {
    ojShuffle(_cards);
  }

  /// Sort cards in the stack in *descending* order from the top.
  void sort() {
    ojSort(_cards);
  }

  /// Simple FNV hash useful for testing.
  int quickHash() {
    int h = 0x811c9dc5;
    for (int i = _cards.length - 1; i >= 0; i -= 1) {
      h ^= _cards[i].index;
      h *= 0x01000193;
    }
    return h & 0xFFFFFFFF;
  }

  operator [](int index) {
    return cardAt(index);
  }

  operator []=(int index, Card card) {
    _cards[_cards.length - 1 - index] = card;
  }

  @override
  String toString() {
    List<String> a = [];
    for (int i = _cards.length - 1; i >= 0; i -= 1) {
      a.add(_cards[i].toString());
    }
    return a.join('');
  }
}
