//
import '../utilities.dart';
import 'cards.dart';

/// Common behavior for [Deck]-like objects.
/// 
/// They generally expect to be dealt
/// from, with cards going to [Hand]-like objects that are created from it and
/// thus remain associated with it for the duration of the game (or at least
/// the hand of play). See [Deck] for details on methods.
/// {@category cards}
abstract class DeckInterface extends Iterable<Card> {
  @override
  int get length;
  @override
  bool get isEmpty;
  @override
  bool get isNotEmpty;
  @override
  bool contains(Object? element);
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

/// A "live" [Deck] of cards that is used for a game.
/// 
/// In a typical game, we
/// expect that the deck will be created once; [Hand]s are created from it;
/// cards are dealt from the deck; the hand is played; and the deck is
/// refilled and shuffled for the next hand of play. Nothing enforces this
/// sequence of events, but it's the one optimzed for.
/// {@category cards}
class Deck extends Iterable<Card> implements DeckInterface {
  /// Associated [MasterDeck] from which to fill this one
  final MasterDeck master;

  /// Current contents of the [Deck]
  OrphanHand _cards = OrphanHand();

  /// [Hand]s spawned from this deck
  List<Hand> hands = [];

  /// Create a new [Deck] from a [MasterDeck] name
  Deck(String dname) : master = MasterDeck.byName(dname) {
    _cards = OrphanHand.fromIterable(master.cardList);
  }

  /// Handy synonym for "length"
  @override
  int get remaining => _cards.length;

  /// How many cards in full deck?
  @override
  int get size => master.size;

  /// Implementation of iterator
  @override
  Iterator<Card> get iterator => HandIterator(_cards.toList());

  // Create new associated [Hand]. If number given, deal that many cards.
  @override
  Hand newHand([int? size]) {
    var hand = Hand(this);
    hands.add(hand);

    if (size != null) {
      hand.draw(size);
    }
    return hand;
  }

  /// Deal a single card to the given [Hand]
  @override
  bool dealTo(Hand h) {
    if (_cards.isEmpty) return false;
    h.push(_cards.pop()!);
    return true;
  }

  /// Deal all [Hand]s created from this deck a number of cards. Useful at
  /// the start of a hand of play, e.g. `d.dealAll(5);`.
  @override
  bool dealAll(int n) {
    if (_cards.length < n * hands.length) return false;
    for (var h in hands) {
      h.pushN(n, _cards.popN(n));
    }
    return true;
  }
  /// Clear all associated [Hand]s.
  @override
  void clearAll() {
    for (var h in hands) {
      h.clear();
    }
  }
  /// Refill the [Deck] from its associated [MasterDeck].
  @override
  void refill() {
    _cards.clear();
    _cards.pushN(master.cardList.length, master.cardList);
  }
  /// Validate that the [Card] passed is legal for this deck. Convert high
  /// and low aces if necessary.
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
  /// Add a [Card] to the [Deck]. Not expected to be used often.
  @override
  void push(Card card) {
    _cards.push(validCard(card));
  }
  /// Pop a [Card] from the [Deck]. This is expected to be the primary way
  /// cards are taken (the `dealXX` functions call this).
  @override
  Card? pop() {
    return _cards.pop();
  }
  /// Push a number of [Card]s back onto the [Deck].
  @override
  void pushN(int n, Iterable<Card> cards) {
    _cards.pushN(n, cards.map((c) => validCard(c)));
  }
  /// Pop a number of [Card]s from the [Deck].
  @override
  Iterable<Card> popN(int n) sync* {
    yield* _cards.popN(n);
  }
  /// Remove a specific [Card] from the [Deck] by its identity.
  /// (e.g. `d.removeCard(Card.JackOfClubs))`.
  @override
  bool removeCard(Card card) {
    return _cards.removeCard(card);
  }
  /// Randomize the [Card]s remaining in the [Deck].
  @override
  void shuffle() {
    _cards.shuffle();
  }
  /// Sort the [Deck] into the same descending order by [Rank] and [Suit] as
  /// used for [Hand]s. Note that this will make the deck appear to be in the
  /// reverse ascending order when printed (see `toString()`, below).
  @override
  void sort() {
    _cards.sort();
  }
  /// Iterate over all distinct count-sized [Hand]s from the remaining deck.
  /// For this purpose, hands with the same cards in different order are
  /// considered "distinct". For example, on a fresh 52-card deck,
  /// `for h in d.combinations(5)` will iterate over all 2598960 5-card hands.
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
  /// Copy the remaining [Card]s onto a new `List`.
  @override
  List<Card> toList({bool growable = true}) {
    return _cards.toList(growable: growable).toList();
  }
  /// Does the [Deck] contain the given [Card]?
  @override
  bool contains(Object? element) {
    if (element is! Card) return false;
    return _cards.contains(validCard(element));
  }
  /// How many [Card]s remain?
  @override
  int get length => _cards.length;
  /// Have all [Card]s been dealt?
  @override
  bool get isEmpty => _cards.isEmpty;
  /// Are any [Card]s remaining?
  @override
  bool get isNotEmpty => _cards.isNotEmpty;
  /// Print contents of [Deck]. Note that for performance, cards are popped
  /// from the end of the list, which makes it the notional "top" of the deck.
  /// For testing and simulation purposes, deck contents are shown backwards:
  /// i.e., from the end of the list to the front.
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
