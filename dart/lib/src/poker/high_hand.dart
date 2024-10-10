// ignore_for_file: constant_identifier_names
import '../troolean.dart';
import '../utilities.dart';
import '../cards/cards.dart';
import 'hand_value.dart';

// const int _FIVE_OF_A_KIND = 1;
const int _STRAIGHT_FLUSH = 2;
const int _QUADS = 3;
const int _FULL_HOUSE = 4;
const int _FLUSH = 5;
const int _STRAIGHT = 6;
const int _TRIPS = 7;
const int _TWO_PAIR = 8;
const int _PAIR = 9;
const int _NO_PAIR = 10;

/// [HandValue] for traditional "high" poker hands.
/// {@category poker}
class HighHandValue extends HandValue {
  HighHandValue(super.level, super.ranks, super.value);

  @override
  String fullName() {
    List<String> r1 = ranks.map((r) => r.name).toList();
    List<String> r2 = ranks.map((r) => r.plural).toList();
    List<String> r3 = ranks.map((r) => r.article).toList();

    switch (level) {
      case _STRAIGHT_FLUSH:
        if (ranks[0] == Rank.Ace) {
          return "royal flush";
        }
        return "${r1[0]}-high straight flush";
      case _QUADS:
        return "four ${r2[0]} with ${r3[4]} ${r1[4]}";
      case _FULL_HOUSE:
        return "${r2[0]} full of ${r2[3]}";
      case _FLUSH:
        return "flush: ${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      case _STRAIGHT:
        return "${r1[0]}-high straight";
      case _TRIPS:
        return "three ${r2[0]}, ${r1[3]}, ${r1[4]}";
      case _TWO_PAIR:
        return "${r2[0]} and ${r2[2]} with ${r3[4]} ${r1[4]}";
      case _PAIR:
        return "pair of ${r2[0]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      case _NO_PAIR:
        return "no pair: ${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      default:
        return "unknown hand";
    }
  }
}

/// Evaluation functions for traditional "high" poker hands.
/// {@category poker}
class HighHandEvaluator extends HandEvaluator<HighHandValue> {
  static PrimeHash hasher = PrimeHash();

  @override
  HandValueFactory<HighHandValue> get valueFactory {
    return (int level, List<Rank> ranks, int value) =>
        HighHandValue(level, ranks, value);
  }

  HighHandEvaluator() {
    HandEvaluator.tablesLoaded =
        HandEvaluator.loadMsgPackTables("high_hand_prime_hash");
  }

  // superclass lookupEvaluator() should work
  // superclass fastValue() should work
  // superclass valueOf() should work
  // superclass bestOf() should work

  @override
  HighHandValue referenceEvaluator(HandInterface h) {
    assert(5 == _allValidCards(h));
    var st = _EvaluatorState(h);

    st.checkFlush();
    st.checkStraight();

    if (st.flush.isTrue() && st.straight.isTrue()) {
      return HighHandValue(_STRAIGHT_FLUSH, st.ranks, 0);
    }
    if (st.flush.isTrue()) {
      assert(st.straight.isFalse());
      return HighHandValue(_FLUSH, st.ranks, 0);
    }
    if (st.straight.isTrue()) {
      assert(st.flush.isFalse());
      return HighHandValue(_STRAIGHT, st.ranks, 0);
    }
    st.checkQuads();

    if (st.quads.isTrue()) {
      return HighHandValue(_QUADS, st.ranks, 0);
    }
    st.checkFullHouse();

    if (st.fullHouse.isTrue()) {
      return HighHandValue(_FULL_HOUSE, st.ranks, 0);
    }
    st.checkTrips();

    if (st.trips.isTrue()) {
      return HighHandValue(_TRIPS, st.ranks, 0);
    }
    st.checkTwoPair();

    if (st.twoPair.isTrue()) {
      return HighHandValue(_TWO_PAIR, st.ranks, 0);
    }
    st.checkOnePair();

    if (st.pair.isTrue()) {
      return HighHandValue(_PAIR, st.ranks, 0);
    }
    assert(st.allChecksComplete());
    return HighHandValue(_NO_PAIR, st.ranks, 0);
  }

  /// Evaluate partial hand, e.g. for determining first bet in stud.
  @override
  HighHandValue partialEvaluator(HandInterface h) {
    assert(h.length < 5);
    var st = _EvaluatorState(h);

    st.checkFlush();
    assert(st.flush.isFalse());
    st.checkStraight();
    assert(st.straight.isFalse());
    st.checkQuads();

    if (st.quads.isTrue()) {
      return HighHandValue(_QUADS, st.ranks, 0);
    }
    st.checkFullHouse();
    assert(st.fullHouse.isFalse());
    st.checkTrips();

    if (st.trips.isTrue()) {
      return HighHandValue(_TRIPS, st.ranks, 0);
    }
    st.checkTwoPair();

    if (st.twoPair.isTrue()) {
      return HighHandValue(_TWO_PAIR, st.ranks, 0);
    }
    st.checkOnePair();

    if (st.pair.isTrue()) {
      return HighHandValue(_PAIR, st.ranks, 0);
    }
    assert(st.allChecksComplete());
    return HighHandValue(_NO_PAIR, st.ranks, 0);
  }
}

/// Handling the Knight gap
Rank _nextLowerRank(Rank r) {
  switch (r) {
    case Rank.Deuce:
      return Rank.LowAce;
    case Rank.Trey:
      return Rank.Deuce;
    case Rank.Four:
      return Rank.Trey;
    case Rank.Five:
      return Rank.Four;
    case Rank.Six:
      return Rank.Five;
    case Rank.Seven:
      return Rank.Six;
    case Rank.Eight:
      return Rank.Seven;
    case Rank.Nine:
      return Rank.Eight;
    case Rank.Ten:
      return Rank.Nine;
    case Rank.Jack:
      return Rank.Ten;
    case Rank.Queen:
      return Rank.Jack;
    case Rank.King:
      return Rank.Queen;
    case Rank.Ace:
      return Rank.King;
    default: // Shouldn't see any knights or low aces
      throw Exception("Invalid rank");
  }
}

class _EvaluatorState {
  List<Card> cards;
  List<Rank> ranks;
  Troolean sorted;
  Troolean flush;
  Troolean straight;
  Troolean quads;
  Troolean fullHouse;
  Troolean trips;
  Troolean twoPair;
  Troolean pair;

  _EvaluatorState(HandInterface h)
      : cards = h.toList(),
        ranks = [],
        sorted = TUNKNOWN,
        flush = TUNKNOWN,
        straight = TUNKNOWN,
        quads = TUNKNOWN,
        fullHouse = TUNKNOWN,
        trips = TUNKNOWN,
        twoPair = TUNKNOWN,
        pair = TUNKNOWN {
    ojSort(cards);
    sorted = TTRUE;
    ranks = cards.map((c) => c.rank!).toList();
  }

  bool allChecksComplete() {
    return sorted.isKnown() &&
        flush.isKnown() &&
        straight.isKnown() &&
        quads.isKnown() &&
        fullHouse.isKnown() &&
        trips.isKnown() &&
        twoPair.isKnown() &&
        pair.isKnown();
  }

  checkFlush() {
    if (cards.length < 5) {
      flush = TFALSE;
      return;
    }
    assert(sorted.isTrue()); // just for display

    flush = TTRUE;
    for (int i = 1; i < cards.length; i += 1) {
      if (cards[i].suit != cards[0].suit) {
        flush = TFALSE;
        return;
      }
    }
  }

  checkStraight() {
    if (cards.length < 5) {
      straight = TFALSE;
      return;
    }
    assert(sorted.isTrue());

    // Special-case wheel
    if (ranks[0] == Rank.Ace &&
        ranks[1] == Rank.Five &&
        ranks[2] == Rank.Four &&
        ranks[3] == Rank.Trey &&
        ranks[4] == Rank.Deuce) {
      ranks[0] = Rank.Five;
      ranks[1] = Rank.Four;
      ranks[2] = Rank.Trey;
      ranks[3] = Rank.Deuce;
      ranks[4] = Rank.Ace;

      sorted = TFALSE;
      straight = TTRUE;
      return;
    }
    for (int i = 1; i < 5; i += 1) {
      if (ranks[i] != _nextLowerRank(ranks[i - 1])) {
        straight = TFALSE;
        return;
      }
    }
    straight = TTRUE;
  }

  checkQuads() {
    if (cards.length < 4) {
      quads = TFALSE;
      return;
    }
    assert(sorted.isTrue());

    // AAAAB
    if (ranks[0] == ranks[1] && ranks[0] == ranks[2] && ranks[0] == ranks[3]) {
      quads = TTRUE;
      return;
    }
    if (cards.length < 5) {
      quads = TFALSE;
      return;
    }
    // ABBBB
    if (ranks[1] == ranks[2] && ranks[1] == ranks[3] && ranks[1] == ranks[4]) {
      ranks[4] = ranks[0];
      ranks[0] = ranks[1];

      sorted = TFALSE;
      quads = TTRUE;
      return;
    }
    quads = TFALSE;
  }

  checkFullHouse() {
    if (cards.length < 5) {
      fullHouse = TFALSE;
      return;
    }
    assert(sorted.isTrue());
    assert(quads.isFalse());
    assert(ranks[0] != ranks[4]);

    // AAABB
    if (ranks[0] == ranks[1] && ranks[0] == ranks[2] && ranks[3] == ranks[4]) {
      fullHouse = TTRUE;
      return;
    }
    // AABBB
    if (ranks[0] == ranks[1] && ranks[2] == ranks[3] && ranks[2] == ranks[4]) {
      ranks[4] = ranks[0];
      ranks[3] = ranks[0];
      ranks[0] = ranks[2];
      ranks[1] = ranks[2];

      sorted = TFALSE;
      fullHouse = TTRUE;
      return;
    }
    fullHouse = TFALSE;
  }

  checkTrips() {
    if (cards.length < 3) {
      trips = TFALSE;
      return;
    }
    assert(sorted.isTrue());
    assert(quads.isFalse());
    assert(fullHouse.isFalse());

    // AAABC
    if (ranks[0] == ranks[1] && ranks[0] == ranks[2]) {
      trips = TTRUE;
      return;
    }
    if (cards.length < 4) {
      trips = TFALSE;
      return;
    }
    // ABBBC
    if (ranks[1] == ranks[2] && ranks[1] == ranks[3]) {
      ranks[3] = ranks[0];
      ranks[0] = ranks[1];

      sorted = TFALSE;
      trips = TTRUE;
      return;
    }
    if (cards.length < 5) {
      trips = TFALSE;
      return;
    }
    // ABCCC
    if (ranks[2] == ranks[3] && ranks[2] == ranks[4]) {
      ranks[4] = ranks[1];
      ranks[3] = ranks[0];
      ranks[0] = ranks[2];
      ranks[1] = ranks[2];

      sorted = TFALSE;
      trips = TTRUE;
      return;
    }
    trips = TFALSE;
  }

  checkTwoPair() {
    if (cards.length < 4) {
      twoPair = TFALSE;
      return;
    }
    assert(sorted.isTrue());
    assert(quads.isFalse());
    assert(fullHouse.isFalse());
    assert(trips.isFalse());

    // AABBC
    if (ranks[0] == ranks[1] && ranks[2] == ranks[3]) {
      twoPair = TTRUE;
      return;
    }
    if (cards.length < 5) {
      twoPair = TFALSE;
      return;
    }
    // ABBCC
    if (ranks[1] == ranks[2] && ranks[3] == ranks[4]) {
      ranks[4] = ranks[0];
      ranks[0] = ranks[1];
      ranks[2] = ranks[3];

      sorted = TFALSE;
      twoPair = TTRUE;
      return;
    }
    // AABCC
    if (ranks[0] == ranks[1] && ranks[3] == ranks[4]) {
      ranks[4] = ranks[2];
      ranks[2] = ranks[3];

      sorted = TFALSE;
      twoPair = TTRUE;
      return;
    }
    twoPair = TFALSE;
  }

  checkOnePair() {
    if (cards.length < 2) {
      pair = TFALSE;
      return;
    }
    assert(sorted.isTrue());
    assert(quads.isFalse());
    assert(fullHouse.isFalse());
    assert(trips.isFalse());
    assert(twoPair.isFalse());

    // AABCD
    if (ranks[0] == ranks[1]) {
      pair = TTRUE;
      return;
    }
    if (cards.length < 3) {
      pair = TFALSE;
      return;
    }
    // ABBCD
    if (ranks[1] == ranks[2]) {
      ranks[2] = ranks[0];
      ranks[0] = ranks[1];

      sorted = TFALSE;
      pair = TTRUE;
      return;
    }
    if (cards.length < 4) {
      pair = TFALSE;
      return;
    }
    // ABCCD
    if (ranks[2] == ranks[3]) {
      ranks[3] = ranks[1];
      ranks[1] = ranks[2];
      ranks[2] = ranks[0];
      ranks[0] = ranks[1];

      sorted = TFALSE;
      pair = TTRUE;
      return;
    }
    if (cards.length < 5) {
      pair = TFALSE;
      return;
    }
    // ABCDD
    if (ranks[3] == ranks[4]) {
      ranks[4] = ranks[2];
      ranks[2] = ranks[0];
      ranks[0] = ranks[3];
      ranks[3] = ranks[1];
      ranks[1] = ranks[0];

      sorted = TFALSE;
      pair = TTRUE;
      return;
    }
    pair = TFALSE;
  }
}

int _allValidCards(HandInterface h) {
  int ret = 0;

  for (int i = 0; i < h.length; i += 1) {
    if (h.cardAt(i)!.suit == null) {
      return 0;
    }
    if (h.cardAt(i)!.rank == Rank.LowAce) {
      return 0;
    }
    if (h.cardAt(i)!.rank == Rank.Knight) {
      return 0;
    }
    ret += 1;
  }
  return ret;
}
