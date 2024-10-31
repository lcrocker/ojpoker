//
import '../utilities.dart';
import 'cards.dart';

/// A "live" [Deck] of cards that is used for a game.
///
/// In a typical game, we
/// expect that the deck will be created once; [Hand]s are created from it;
/// cards are dealt from the deck; the hand is played; and the deck is
/// refilled and shuffled for the next hand of play. Nothing enforces this
/// sequence of events, but it's the one optimzed for.
/// {@category cards}
class Deck extends Iterable<Card> {
  /// Associated [MasterDeck] from which to fill this one
  final MasterDeck master;
  final List<Card> cards = [];

  /// [Hand]s spawned from this deck
  final List<Hand> hands = [];

  /// Create a new [Deck] from a [MasterDeck] name
  Deck([String dname = "default"]) : master = MasterDeck.byName(dname) {
    refill();
  }

  /// Synonym for "length"
  int get remaining => cards.length;

  /// How many cards in full deck?
  int get size => master.size;

  /// Implementation of iterator
  @override
  Iterator<Card> get iterator => HandIterator(cards.toList());

  // Create new associated [Hand]. If number given, deal that many cards.
  Hand newHand([int size = 0]) {
    var hand = Hand(this.master.name, this);
    hands.add(hand);

    if (size > 0) {
      hand.draw(size);
    }
    return hand;
  }

  /// Deal a single card to the given [Hand]
  bool dealTo(Hand h) {
    if (cards.isEmpty) return false;
    h.push(pop()!);
    return true;
  }

  /// Deal all [Hand]s created from this deck a number of cards. Useful at
  /// the start of a hand of play, e.g. `d.dealAll(5);`.
  bool dealAll(int n) {
    if (cards.length < n * hands.length) return false;
    for (var h in hands) {
      h.pushN(popN(n));
    }
    return true;
  }

  /// Clear all associated [Hand]s.
  void clearAll() {
    for (var h in hands) {
      h.clear();
    }
  }

  /// Refill the [Deck] from its associated [MasterDeck].
  void refill() {
    cards.clear();
    pushN(master.cardList);
  }

  /// Validate that the [Card] passed is legal for this deck.
  /// Convert high and low aces if necessary.
  (bool, Card) validCard(Card cin) {
    Card cout;

    if (master.lowAces) {
      cout = Card.lowAceFix(cin);
    } else {
      cout = Card.highAceFix(cin);
    }
    if (master.has(cout)) return (true, cout);
    return (false, cout);
  }

  /// Add a [Card] to the [Deck]. Not expected to be used often.
  void push(Card card) {
    var (v, c) = validCard(card);
    if (v) cards.add(c);
  }

  /// Pop a [Card] from the [Deck]. This is expected to be the primary way
  /// cards are taken (the `dealXX` functions call this).
  Card? pop() {
    return cards.removeLast();
  }

  /// Push a number of [Card]s back onto the [Deck].
  void pushN(Iterable<Card> ci) {
    for (var c in ci) {
      push(c);
    }
  }

  /// Pop a number of [Card]s from the [Deck].
  Iterable<Card> popN(int n) sync* {
    for (int i = 0; i < n; i += 1) {
      if (cards.isEmpty) break;
      yield cards.removeLast();
    }
  }

  /// Remove a specific [Card] from the [Deck] by its identity.
  /// (e.g. `d.removeCard(Card.JackOfClubs))`.
  bool removeCard(Card card) {
    for (var i = 0; i < cards.length; i += 1) {
      var (v, c) = validCard(card);

      if (v && cards[i] == c) {
        cards.removeAt(i);
        return true;
      }
    }
    return false;
  }

  /// Randomize the [Card]s remaining in the [Deck].
  void shuffle() {
    ojShuffle(cards);
  }

  /// Sort the [Deck] into the same descending order by [Rank] and [Suit] as
  /// used for [Hand]s. Note that this will make the deck appear to be in the
  /// reverse ascending order when printed (see `toString()`, below).
  void sort() {
    ojSort(cards);
  }

  /// Iterate over all distinct count-sized [Hand]s from the remaining deck.
  /// For this purpose, hands with the same cards in different order are
  /// considered "distinct". For example, on a fresh 52-card deck,
  /// `for h in d.combinations(5)` will iterate over all 2598960 5-card hands.
  Iterable<Hand> combinations(int count) sync* {
    if (count > cards.length) return;

    if (0 == count || cards.isEmpty) {
      yield newHand();
      return;
    }
    List<Card> cs = cards.toList();
    ojSort(cs);

    if (cs.length == count) {
      Hand res = newHand();
      res.pushN(cs);
      yield res;
      return;
    }
    List<int> a = List<int>.generate(count, (i) => i + 1);

    do {
      Hand res = newHand();
      for (int i = 0; i < count; i += 1) {
        res.push(cs[a[i] - 1]);
      }
      yield res;
    } while (ojNextCombination(a, cs.length));
  }

  /// Copy the remaining [Card]s onto a new `List`.
  @override
  List<Card> toList({bool growable = true}) {
    return cards.toList(growable: growable);
  }

  /// Does the [Deck] contain the given [Card]?
  @override
  bool contains(Object? element) {
    if (element is! Card) return false;
    var (v, c) = validCard(element);
    if (v) return cards.contains(c);
    return false;
  }

  /// How many [Card]s remain?
  @override
  int get length => cards.length;

  /// Have all [Card]s been dealt?
  @override
  bool get isEmpty => cards.isEmpty;

  /// Are any [Card]s remaining?
  @override
  bool get isNotEmpty => cards.isNotEmpty;

  /// Print contents of [Deck]. Note that for performance, cards are popped
  /// from the end of the list, which makes it the notional "top" of the deck.
  /// For testing and simulation purposes, deck contents are shown backwards:
  /// i.e., from the end of the list to the front.
  @override
  String toString() {
    int l = cards.length;
    int max = l;
    int tail = 0;

    if (l > 32) {
      max = 29;
      tail = l - 29;
    }
    StringBuffer sb = StringBuffer();
    sb.write("${master.name} [");
    for (int i = l - 1; i >= l - max; i -= 1) {
      sb.write(cards[i].toString());
    }
    if (tail > 0) {
      sb.write("...+$tail");
    }
    sb.write("]");
    return sb.toString();
  }
}
