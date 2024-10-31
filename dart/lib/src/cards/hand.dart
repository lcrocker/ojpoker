//
import '../utilities.dart';
import 'cards.dart';

class HandIterator implements Iterator<Card> {
  final List<Card> cards;
  int currentIndex = -1;

  HandIterator(this.cards) {
    currentIndex = -1;
  }

  @override
  bool moveNext() {
    if (currentIndex < cards.length - 1) {
      currentIndex += 1;
      return true;
    }
    return false;
  }

  @override
  Card get current => cards[currentIndex];
}

/// Hand of cards (or other ordered collection).
/// {@category cards}
class Hand extends Iterable<Card> {
  final MasterDeck master;
  final Deck? parent;
  final List<Card> cards;

  Hand([String dname = "default", this.parent, List<Card>? cards])
      : master = MasterDeck.byName(dname),
        cards = cards ?? [];

  /// Create new hand from list of cards, e.g.:
  /// ```
  /// var hand = CardStack.fromList([ Card.FourOfSpades, Card.Joker ]);
  /// ```
  Hand.fromIter(Iterable<Card> c)
      : master = MasterDeck.byName("default"),
        parent = null,
        cards = c.toList() {
    aceFix();
  }

  /// Create new hand from text, e.g.:
  Hand.fromText(String text)
      : master = MasterDeck.byName("default"),
        parent = null,
        cards = cardsFromText(text).toList() {
    aceFix();
  }

  Hand clone() {
    return Hand(master.name, parent, cards.toList());
  }

  Hand copyFromIter(Iterable<Card> c) {
    var h = Hand(master.name, parent, c.toList());
    h.aceFix();
    return h;
  }

  Hand copyFromText(String s) {
    var h = Hand(master.name, parent, cardsFromText(s).toList());
    h.aceFix();
    return h;
  }

  Iterable<Rank> get ranks sync* {
    for (Card c in cards) {
      yield c.rank;
    }
  }

  Iterable<Suit> get suits sync* {
    for (Card c in cards) {
      yield c.suit;
    }
  }

  @override
  Iterator<Card> get iterator => HandIterator(cards);

  /// Fix cards for decks with low/high aces.
  void aceFix() {
    if (master.lowAces) {
      for (int i = 0; i < cards.length; i += 1) {
        cards[i] = Card.lowAceFix(cards[i]);
      }
    } else {
      for (int i = 0; i < cards.length; i += 1) {
        cards[i] = Card.highAceFix(cards[i]);
      }
    }
  }

  (bool, Card) validCard(Card c) {
    Card cout = master.lowAces ? Card.lowAceFix(c) : Card.highAceFix(c);
    if (master.has(cout)) return (true, cout);
    return (false, cout);
  }

  /// How many cards currently in the stack?
  @override
  int get length {
    return cards.length;
  }

  /// Is list empty?
  @override
  bool get isEmpty {
    return cards.isEmpty;
  }

  @override
  bool get isNotEmpty {
    return cards.isNotEmpty;
  }

  /// Empty the hand.
  void clear() {
    cards.clear();
  }

  /// Return the card at position `index`.
  Card? cardAt(int index) {
    if (index < 0 || index >= cards.length) return null;
    return cards[index];
  }

  bool setCardAt(int index, Card card) {
    if (index < 0 || index >= cards.length) return false;
    var (v, c) = validCard(card);
    if (v) cards[index] = c;
    return v;
  }

  void push(Card card) {
    var (v, c) = validCard(card);
    if (v) cards.add(c);
  }

  Card? pop() {
    if (cards.isEmpty) return null;
    return cards.removeLast();
  }

  @override
  bool contains(Object? element) {
    if (element is! Card) return false;
    var (v, c) = validCard(element);
    if (v) return cards.contains(element);
    return v;
  }

  void pushN(Iterable<Card> cards) {
    for (Card c in cards) {
      push(c);
    }
  }

  Iterable<Card> popN(int n) sync* {
    if (n > cards.length) n = cards.length;
    for (int i = 0; i < n; i += 1) {
      yield cards.removeLast();
    }
  }

  /// Insert card at position `index`.
  void insertAt(int index, Card card) {
    if (index < 0 || index > cards.length) return;
    var (v, c) = validCard(card);
    if (v) cards.insert(index, card);
  }

  /// Remove card at position `index`.
  Card? removeAt(int index) {
    if (index < 0 || index >= cards.length) return null;
    return cards.removeAt(index);
  }

  /// Remove top-most specific card from list.
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

  void truncate(int n) {
    if (n < 0) n = 0;
    if (n < cards.length) cards.removeRange(n, cards.length);
  }

  /// Randomize order of cards in the stack.
  void shuffle() {
    ojShuffle(cards);
  }

  /// Sort cards in the stack in *descending* order from the top.
  void sort() {
    ojSort(cards);
  }

  Iterable<Hand> combinations([int? count]) sync* {
    count ??= cards.length;
    if (count > cards.length) return;

    if (0 == count || cards.isEmpty) {
      yield copyFromIter([]);
      return;
    }
    List<Card> cs = cards.toList();
    ojSort(cs);
    if (cs.length == count) {
      yield copyFromIter(cs);
      return;
    }

    List<int> a = List<int>.generate(count, (i) => i + 1);

    do {
      var res = copyFromIter([]);
      for (int i = 0; i < count; i += 1) {
        res.push(cs[a[i] - 1]);
      }
      yield res;
    } while (ojNextCombination(a, cs.length));
  }

  Card operator [](int index) {
    return cardAt(index)!;
  }

  void operator []=(int index, Card card) {
    setCardAt(index, card);
  }

  @override
  String toString() {
    List<String> a = [];
    for (int i = 0; i < cards.length; i += 1) {
      a.add(cards[i].toString());
    }
    return a.join('');
  }

  bool equals(Hand other) {
    if (cards.length != other.length) return false;

    for (int i = 0; i < cards.length; i += 1) {
      if (cards[i] != other.cards[i]) return false;
    }
    return true;
  }

  bool isEquivalentTo(Hand other) {
    if (cards.length != other.length) return false;

    // Have to do the hard way if duplicates are possible.
    if (master.dupsAllowed) {
      List<Card> as = cards.toList();
      List<Card> os = other.toList();
      ojSort(as);
      ojSort(os);

      for (int i = 0; i < as.length; i += 1) {
        if (as[i] != os[i]) return false;
      }
      return true;
    }
    int mask1 = 0, mask2 = 0;
    for (int i = 0; i < cards.length; i += 1) {
      mask1 |= (1 << cards[i].index);
      mask2 |= (1 << other.cards[i].index);
    }
    return mask1 == mask2;
  }

  bool draw(int n) {
    if (parent == null) return false;
    if (parent!.remaining < n) return false;
    pushN(parent!.popN(n));
    return true;
  }

  bool drawCard(Card c) {
    if (parent == null) return false;
    if (parent!.removeCard(c)) {
      push(c);
      return true;
    }
    return false;
  }

  bool drawHand(Iterable<Card> cards) {
    if (parent == null) return false;
    for (Card c in cards) {
      if (parent!.removeCard(c)) {
        push(c);
      } else {
        return false;
      }
    }
    return true;
  }

  bool discard(List<int> indices) {
    var ok = true;
    ojSort(indices); // descending

    for (var x in indices) {
      if (x >= length) {
        ok = false;
      } else {
        cards.removeAt(x);
      }
    }
    return ok;
  }
}
