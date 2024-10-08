import 'package:onejoker/cards/card.dart';
import 'package:onejoker/cards/master_deck.dart';
import 'package:onejoker/cards/hand.dart';
import 'package:onejoker/cards/utils.dart';

abstract class DeckInterface extends Iterable<Card> {
  @override int get length;
  @override bool get isEmpty;
  @override bool get isNotEmpty;
  @override bool contains(Object? element);
  int get remaining;
  int get size;
  Hand newHand([int? size]);
  bool dealTo(Hand h);
  bool dealAll(int n);
  void clearAll();
  void refill();
  Card validCard(Card cin);
  void push(Card card);
  Card? pop();
  void pushN(int count, Iterable<Card> cards);
  Iterable<Card> popN(int n);
  bool removeCard(Card card);
  void shuffle();
  void sort();
  Iterable<Hand> combinations(int count);
}

/// # Deck | [wiki](https://github.com/lcrocker/ojpoker/wiki/Deck)
/// This is the "live" deck of cards that is used for a game. In a
/// typical game, we expect that the deck will be created once,
/// hands and other card stacks created from it, and then cards are
/// dealt from the deck, the hand is played, and the deck is
/// refilled and shuffled for another.
class Deck extends Iterable<Card> implements DeckInterface {
  final MasterDeck master;
  OrphanHand _cards = OrphanHand();
  List<Hand> hands = [];

  /// Create a new deck from a master deck name.
  Deck(String dname) : master = MasterDeck.byName(dname) {
    _cards = OrphanHand.fromIterable(master.cardList);
  }

  @override
  int get remaining => _cards.length;

  @override
  int get size => master.size;

  @override
  Iterator<Card> get iterator => HandIterator(_cards.toList());

  @override
  Hand newHand([int? size]) {
    var hand = Hand(this);
    hands.add(hand);

    if (size != null) {
      hand.draw(size);
    }
    return hand;
  }

  @override
  bool dealTo(Hand h) {
    if (_cards.isEmpty) return false;
    h.push(_cards.pop()!);
    return true;
  }

  @override
  bool dealAll(int n) {
    if (_cards.length < n * hands.length) return false;
    for (var h in hands) {
      h.pushN(n, _cards.popN(n));
    }
    return true;
  }

  @override
  void clearAll() {
    for (var h in hands) {
      h.clear();
    }
  }

  @override
  void refill() {
    _cards.clear();
    _cards.pushN(master.cardList.length, master.cardList);
  }

  @override
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

  @override
  void push(Card card) {
    _cards.push(validCard(card));
  }

  @override
  Card? pop() {
    return _cards.pop();
  }

  @override
  void pushN(int n, Iterable<Card> cards) {
    _cards.pushN(n, cards.map((c) => validCard(c)));
  }

  @override
  Iterable<Card> popN(int n) sync* {
    yield* _cards.popN(n);
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
      yield newHand();
      return;
    }
    List<Card> cs = _cards.toList();
    ojSort(cs);

    if (cs.length == count) {
      Hand res = Hand(this);
      res.pushN(cs.length, cs);
      yield res;
      return;
    }
    List<int> a = List<int>.generate(count, (i) => i + 1);

    do {
      Hand res = Hand(this);
      for (int i = 0; i < count; i += 1) {
        res.push(cs[a[i] - 1]);
      }
      yield res;
    } while (ojNextCombination(a, cs.length));
  }

  @override
  List<Card> toList({bool growable = true}) {
    return _cards.toList(growable: growable).toList();
  }

  @override
  bool contains(Object? element) {
    if (element is! Card) return false;
    return _cards.contains(validCard(element));
  }

  @override
  int get length => _cards.length;

  @override
  bool get isEmpty => _cards.isEmpty;

  @override
  bool get isNotEmpty => _cards.isNotEmpty;

  @override
  String toString() {
    int l = _cards.length;
    int max = l;
    int tail = 0;

    if (l > 32) {
      max = 29;
      tail = l - 29;
    }
    StringBuffer sb = StringBuffer();
    sb.write("${master.name} [");
    for (int i = l - 1; i >= l - max; i -= 1) {
      sb.write(_cards[i].toString());
    }
    if (tail > 0) {
      sb.write("...+$tail");
    }
    sb.write("]");
    return sb.toString();
  }
}
