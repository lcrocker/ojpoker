import 'package:onejoker/cards/card.dart';
import 'package:onejoker/cards/deck.dart';
import 'package:onejoker/cards/utils.dart';

abstract class HandInterface extends Iterable<Card> {
  @override int get length;
  @override bool get isEmpty;
  @override bool get isNotEmpty;
  @override bool contains(Object? element);
  HandInterface clone();
  void clear();
  Card? cardAt(int index);
  bool setCardAt(int index, Card card);
  void push(Card card);
  Card? pop();
  void pushN(int count, Iterable<Card> cards);
  Iterable<Card> popN(int n);
  void insertAt(int index, Card card);
  Card? removeAt(int index);
  bool removeCard(Card card);
  void shuffle();
  void sort();
  Iterable<HandInterface> combinations(int count);
  operator [](int index);
  operator []=(int index, Card card);
  bool equals(HandInterface other);
  bool isEquivalentTo(HandInterface other);
}

class HandIterator implements Iterator<Card> {
  final List<Card> _cards;
  int _currentIndex = -1;

  HandIterator(this._cards) {
    _currentIndex = -1;
  }

  @override
  bool moveNext() {
    if (_currentIndex < _cards.length - 1) {
      _currentIndex += 1;
      return true;
    }
    return false;
  }

  @override
  Card get current => _cards[_currentIndex];
}

class OrphanHand extends Iterable<Card> implements HandInterface {
  List<Card> _cards = [];

  OrphanHand();

  /// Create new stack from list of cards, e.g.:
  /// ```
  /// var hand = CardStack.fromList([ Card.FourOfSpades, Card.Joker ]);
  /// ```
  OrphanHand.fromIterable(Iterable<Card> c) {
    _cards = c.toList();
  }

  OrphanHand.fromText(String text) : this.fromIterable(cardsFromText(text));

  @override
  OrphanHand clone() {
    return OrphanHand.fromIterable(_cards.toList());
  }

  @override
  Iterator<Card> get iterator => HandIterator(_cards);

  /// Fix cards for decks with low aces.
  void lowAceFix() {
    for (int i = 0; i < _cards.length; i += 1) {
      _cards[i] = Card.lowAceFix(_cards[i]);
    }
  }

  /// Fix cards for decks with high aces.
  void highAceFix() {
    for (int i = 0; i < _cards.length; i += 1) {
      _cards[i] = Card.highAceFix(_cards[i]);
    }
  }

  /// How many cards currently in the stack?
  @override
  int get length {
    return _cards.length;
  }

  /// Is list empty?
  @override
  bool get isEmpty {
    return _cards.isEmpty;
  }

  @override
  bool get isNotEmpty {
    return _cards.isNotEmpty;
  }

  /// Empty the stack.
  @override
  void clear() {
    _cards.clear();
  }

  /// Return the card at position `index`.
  @override
  Card? cardAt(int index) {
    if (index < 0 || index >= _cards.length) return null;
    return _cards[index];
  }

  @override
  bool setCardAt(int index, Card card) {
    if (index < 0 || index >= _cards.length) return false;
    _cards[index] = card;
    return true;
  }

  @override
  void push(Card card) {
    _cards.add(card);
  }

  @override
  Card? pop() {
    if (_cards.isEmpty) return null;
    return _cards.removeLast();
  }

  @override bool contains(Object? element) {
    if (element is! Card) return false;
    return _cards.contains(element);
  }

  /// Add *n* cards to the top of the stack as a unit.
  @override
  void pushN(int count, Iterable<Card> cards) {
    for (Card c in cards.take(count)) {
      _cards.add(c);
    }
  }

  @override
  Iterable<Card> popN(int n) sync* {
    if (n > _cards.length) n = _cards.length;
    for (int i = 0; i < n; i += 1) {
      yield _cards.removeLast();
    }
  }

  /// Insert card at position `index`.
  @override
  void insertAt(int index, Card card) {
    assert(index >= 0 && index <= _cards.length);
    _cards.insert(index, card);
  }

  /// Remove card at position `index`.
  @override
  Card? removeAt(int index) {
    if (index < 0 || index >= _cards.length) return null;
    return _cards.removeAt(_cards.length - 1 - index);
  }

  /// Remove top-most specific card from list.
  @override
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
  @override
  void shuffle() {
    ojShuffle(_cards);
  }

  /// Sort cards in the stack in *descending* order from the top.
  @override
  void sort() {
    ojSort(_cards);
  }

  @override
  Iterable<HandInterface> combinations([int? count]) sync* {
    count ??= _cards.length;
    if (count > _cards.length) return;

    if (0 == count || _cards.isEmpty) {
      yield OrphanHand();
      return;
    }
    List<Card> cs = _cards.toList();
    ojSort(cs);
    if (cs.length == count) {
      yield OrphanHand.fromIterable(cs);
      return;
    }

    List<int> a = List<int>.generate(count, (i) => i + 1);

    do {
      var res = OrphanHand();
      for (int i = 0; i < count; i += 1) {
        res.push(cs[a[i] - 1]);
      }
      yield res;
    } while (ojNextCombination(a, cs.length));
  }

  @override
  operator [](int index) {
    return cardAt(index);
  }

  @override
  operator []=(int index, Card card) {
    setCardAt(index, card);
  }

  @override
  String toString() {
    List<String> a = [];
    for (int i = 0; i < _cards.length; i += 1) {
      a.add(_cards[i].toString());
    }
    return a.join('');
  }

  @override
  bool equals(HandInterface other) {
    if (_cards.length != other.length) return false;
    for (int i = 0; i < _cards.length; i += 1) {
      if (_cards[i] != other.cardAt(i)) return false;
    }
    return true;
  }

  @override
  bool isEquivalentTo(HandInterface other) {
    if (_cards.length != other.length) return false;

    // Have to do the hard way if duplicates are possible.
    List<Card> as = _cards.toList();
    List<Card> os = other.toList();
    ojSort(as);
    ojSort(os);
    for (int i = 0; i < as.length; i += 1) {
      if (as[i] != os[i]) return false;
    }
    return true;
  }
}

class Hand extends Iterable<Card> implements HandInterface {
  final Deck deck;
  final OrphanHand _cards = OrphanHand();

  Hand(this.deck);
  Hand.fromNewDeck(String dname) : deck = Deck(dname);

  @override
  Hand clone() {
    Hand res = Hand(deck);
    res._cards.pushN(_cards.length, _cards);
    return res;
  }

  @override
  Iterator<Card> get iterator => HandIterator(_cards.toList());

  bool draw(int n) {
    if (deck.length < n) return false;
    _cards.pushN(n, deck.popN(n));
    return true;
  }

  bool drawCard(Card c) {
    if (deck.removeCard(c)) {
      _cards.push(c);
      return true;
    }
    return false;
  }

  bool drawHand(List<Card> cards) {
    for (Card c in cards) {
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

  @override
  List<Card> toList({bool growable = true}) {
    return _cards.toList(growable: growable);
  }

  @override
  int get length => _cards.length;

  @override
  bool get isEmpty => _cards.isEmpty;

  @override
  bool get isNotEmpty => _cards.isNotEmpty;

  @override
  bool contains(Object? element) {
    if (element is! Card) return false;
    return _cards.contains(deck.validCard(element));
  }

  @override
  void clear() {
    _cards.clear();
  }

  @override
  Card? cardAt(int index) {
    return _cards.cardAt(index);
  }

  @override
  bool setCardAt(int index, Card card) {
    return _cards.setCardAt(index, deck.validCard(card));
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
  void pushN(int n, Iterable<Card> cards) {
    _cards.pushN(n, cards.map((c) => deck.validCard(c)));
  }

  @override
  Iterable<Card> popN(int n) sync*{
    yield* _cards.popN(n);
  }

  @override
  void insertAt(int index, Card card) {
    _cards.insertAt(index, deck.validCard(card));
  }

  @override
  Card? removeAt(int index) {
    return _cards.removeAt(index);
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
  Iterable<Hand> combinations(int count) sync* {
    if (count > _cards.length) return;

    if (0 == count || _cards.isEmpty) {
      yield Hand(deck);
      return;
    }
    List<Card> cs = _cards.toList();
    ojSort(cs);
    if (cs.length == count) {
      Hand res = Hand(deck);
      res.pushN(cs.length, cs);
      yield res;
      return;
    }
    List<int> a = List<int>.generate(count, (i) => i + 1);

    do {
      Hand res = Hand(deck);
      for (int i = count - 1; i >= 0; i -= 1) {
        res.push(cs[a[i] - 1]);
      }
      yield res;
    } while (ojNextCombination(a, cs.length));
  }

  @override
  String toString() {
    return _cards.toString();
  }

  @override
  operator [](int index) {
    return _cards[index];
  }

  @override
  operator []=(int index, Card card) {
    _cards[index] = card;
  }

  @override
  bool equals(HandInterface other) {
    return _cards.equals(other);
  }

  @override
  bool isEquivalentTo(HandInterface other) {
    if (_cards.length != other.length) return false;

    if (deck.master.dupsAllowed) {
      List<Card> as = _cards.toList();
      List<Card> os = other.toList();
      ojSort(as);
      ojSort(os);
      for (int i = 0; i < as.length; i += 1) {
        if (as[i] != os[i]) return false;
      }
      return true;
    } else {
      int mask1 = 0, mask2 = 0;
      for (int i = 0; i < _cards.length; i += 1) {
        mask1 |= (1 << _cards.cardAt(i)!.index);
        mask2 |= (1 << other.cardAt(i)!.index);
      }
      return mask1 == mask2;
    }
  }
}
