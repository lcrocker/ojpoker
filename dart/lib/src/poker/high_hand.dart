// ignore_for_file: constant_identifier_names
import '../troolean.dart';
import '../utilities.dart';
import '../cards/cards.dart';
import 'hand_value.dart';

/// [HandValue] for traditional "high" poker hands.
/// {@category poker}
class HandValueHigh extends HandValue<HandLevelHigh> {
  HandValueHigh(HandLevelHigh level, List<Rank> ranks):
    super(level, ranks, -1);

  @override
  String fullName() {
    List<String> r1 = ranks.map((r) => r.name).toList();
    List<String> r2 = ranks.map((r) => r.plural).toList();
    List<String> r3 = ranks.map((r) => r.article).toList();

    switch (level) {
      case HandLevelHigh.StraightFlush:
        if (ranks[0] == Rank.Ace) {
          return "royal flush";
        }
        return "${r1[0]}-high straight flush";
      case HandLevelHigh.Quads:
        return "four ${r2[0]} with ${r3[4]} ${r1[4]}";
      case HandLevelHigh.FullHouse:
        return "${r2[0]} full of ${r2[3]}";
      case HandLevelHigh.Flush:
        return "flush: ${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      case HandLevelHigh.Straight:
        return "${r1[0]}-high straight";
      case HandLevelHigh.Trips:
        return "three ${r2[0]}, ${r1[3]}, ${r1[4]}";
      case HandLevelHigh.TwoPair:
        return "${r2[0]} and ${r2[2]} with ${r3[4]} ${r1[4]}";
      case HandLevelHigh.Pair:
        return "pair of ${r2[0]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      default:
        assert(level == HandLevelHigh.NoPair);
        return "no pair: ${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
    }
  }
}

/// Evaluation functions for traditional "high" poker hands.
/// {@category poker}
class HandEvaluatorHigh extends HandEvaluator<HandValueHigh> {
  @override
  HandValueFactory<HandValueHigh> get valueFactory {
    return () => HandValueHigh(HandLevelHigh.StraightFlush,
        [Rank.Ace, Rank.King, Rank.Queen, Rank.Jack, Rank.Ten]);
  }

  HandEvaluatorHigh();

  @override
  HandValueHigh referenceEvaluator(HandInterface h) {
    assert(_allValidCards(h) <= 5);
    var st = _EvaluatorState(h);

    st.checkFlush();
    st.checkStraight();

    if (st.flush.isTrue() && st.straight.isTrue()) {
      return HandValueHigh(HandLevelHigh.StraightFlush, st.ranks);
    }
    if (st.flush.isTrue()) {
      assert(st.straight.isFalse());
      return HandValueHigh(HandLevelHigh.Flush, st.ranks);
    }
    if (st.straight.isTrue()) {
      assert(st.flush.isFalse());
      return HandValueHigh(HandLevelHigh.Straight, st.ranks);
    }
    st.checkQuads();

    if (st.quads.isTrue()) {
      return HandValueHigh(HandLevelHigh.Quads, st.ranks);
    }
    st.checkFullHouse();

    if (st.fullHouse.isTrue()) {
      return HandValueHigh(HandLevelHigh.FullHouse, st.ranks);
    }
    st.checkTrips();

    if (st.trips.isTrue()) {
      return HandValueHigh(HandLevelHigh.Trips, st.ranks);
    }
    st.checkTwoPair();

    if (st.twoPair.isTrue()) {
      return HandValueHigh(HandLevelHigh.TwoPair, st.ranks);
    }
    st.checkOnePair();

    if (st.pair.isTrue()) {
      return HandValueHigh(HandLevelHigh.Pair, st.ranks);
    }
    assert(st.allChecksComplete());
    return HandValueHigh(HandLevelHigh.NoPair, st.ranks);
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
