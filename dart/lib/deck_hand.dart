import 'package:onejoker/card.dart';
import 'package:onejoker/stack.dart';
import 'package:onejoker/master_deck.dart';

/// # Deck | [wiki](https://github.com/lcrocker/ojpoker/wiki/Deck)
/// This is the "live" deck of cards that is used for a game. In a
/// typical game, we expect that the deck will be created once,
/// hands and other card stacks created from it, and then cards are
/// dealt from the deck, the hand is played, and the deck is
/// refilled and shuffled for another.
class Deck extends Iterable<Card> implements CardStackInterface {
  final MasterDeck master;
  CardStack _cards = CardStack();
  List<Hand> hands = [];

  /// Create a new deck from a master deck.
  Deck(String dname) : master = MasterDeck.byName(dname) {
    _cards = CardStack.fromList(master.cardList);
  }

  int get remaining => _cards.length;
  int get size => master.size;

  @override
  Iterator<Card> get iterator => CardStackIterator(_cards.toList());

  Hand newHand() {
    var hand = Hand(this);
    hands.add(hand);
    return hand;
  }

  bool dealTo(Hand h) {
    if (_cards.isEmpty) return false;
    h.push(_cards.pop()!);
    return true;
  }

  bool dealAll(int n) {
    if (_cards.length < n * hands.length) return false;
    for (var h in hands) {
      h.pushN(_cards.popN(n)!);
    }
    return true;
  }

  refill() {
    _cards.clear();
    _cards.pushN(master.cardList);
  }

  Card validCard(Card cin) {
    Card cout;

    if (master.lowAces) {
      cout = Card.lowAceFix(cin);
    } else {
      cout = Card.highAceFix(cin);
    }
    assert(master.has(cout));
    return cout;
  }

  void aceFix() {
    if (master.lowAces) {
      _cards.lowAceFix();
    } else {
      _cards.highAceFix();
    }
  }

  /*
   *
   */

  /// Return list copy (reversed).
  @override
  List<Card> toList({ bool growable = true }) {
    return _cards.toList(growable: growable);
  }

  @override
  int get length => _cards.length;

  @override
  bool get isEmpty => _cards.isEmpty;

  // @override
  // bool contains(Card card) {
  //   return _cards.contains(card);
  // }

  @override
  void clear() {
    _cards.clear();
  }

  @override
  Card? cardAt(int index) {
    return _cards.cardAt(index);
  }

  @override
  void push(Card card) {
    _cards.push(validCard(card));
  }

  @override
  Card? pop() {
    return _cards.pop();
  }

  @override
  void pushN(List<Card> cards) {
    for (var i = cards.length - 1; i >= 0; i -= 1) {
      push(cards[i]);
    }
  }

  @override
  List<Card>? popN(int n) {
    return _cards.popN(n);
  }

  @override
  void insertAt(int index, Card card) {
    _cards.insertAt(index, validCard(card));
  }

  @override
  void insertAtEnd(Card card) {
    _cards.insertAtEnd(validCard(card));
  }

  @override
  Card? removeAt(int index) {
    return _cards.removeAt(index);
  }

  @override
  Card? removeAtEnd() {
    return _cards.removeAtEnd();
  }

  @override
  bool removeCard(Card card) {
    return _cards.removeCard(card);
  }

  @override
  void shuffle() {
    _cards.shuffle();
  }

  @override
  void sort() {
    _cards.sort();
  }

  @override
  int quickHash() {
    return _cards.quickHash();
  }

  @override
  operator [](int index) {
    return _cards[index];
  }

  @override
  operator []=(int index, Card card) {
    _cards[index] = card;
  }
}

class Hand extends Iterable<Card> implements CardStackInterface {
  final Deck deck;
  final CardStack _cards = CardStack();

  Hand(this.deck);

  @override
  Iterator<Card> get iterator => CardStackIterator(_cards.toList());

  bool draw(int n) {
    if (deck.length < n) return false;
    _cards.pushN(deck.popN(n)!);
    return true;
  }

  bool drawCard(Card c) {
    if (deck.removeCard(c)) {
      _cards.push(c);
      return true;
    }
    return false;
  }

  bool drawHand(String text) {
    for (Card c in cardsFromText(text)) {
      if (deck.removeCard(c)) {
        _cards.push(c);
      } else {
        return false;
      }
    }
    return true;
  }

  void aceFix() {
    if (deck.master.lowAces) {
      _cards.lowAceFix();
    } else {
      _cards.highAceFix();
    }
  }

  /*
   *
   */

  /// Return list copy (reversed).
  @override
  List<Card> toList({ bool growable = true }) {
    return _cards.toList(growable: growable);
  }

  @override
  int get length => _cards.length;

  @override
  bool get isEmpty => _cards.isEmpty;

  // @override
  // bool contains(Card card) {
  //   return _cards.contains(card);
  // }

  @override
  void clear() {
    _cards.clear();
  }

  @override
  Card? cardAt(int index) {
    return _cards.cardAt(index);
  }

  @override
  void push(Card card) {
    _cards.push(deck.validCard(card));
  }

  @override
  Card? pop() {
    return _cards.pop();
  }

  @override
  void pushN(List<Card> cards) {
    for (var i = cards.length - 1; i >= 0; i -= 1) {
      push(cards[i]);
    }
  }

  @override
  List<Card>? popN(int n) {
    return _cards.popN(n);
  }

  @override
  void insertAt(int index, Card card) {
    _cards.insertAt(index, deck.validCard(card));
  }

  @override
  void insertAtEnd(Card card) {
    _cards.insertAtEnd(deck.validCard(card));
  }

  @override
  Card? removeAt(int index) {
    return _cards.removeAt(index);
  }

  @override
  Card? removeAtEnd() {
    return _cards.removeAtEnd();
  }

  @override
  bool removeCard(Card card) {
    return _cards.removeCard(card);
  }

  @override
  void shuffle() {
    _cards.shuffle();
  }

  @override
  void sort() {
    _cards.sort();
  }

  @override
  int quickHash() {
    return _cards.quickHash();
  }

  @override
  operator [](int index) {
    return _cards[index];
  }

  @override
  operator []=(int index, Card card) {
    _cards[index] = card;
  }
}
