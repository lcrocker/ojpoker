import '../cards/cards.dart';
import 'hand_value.dart';
import 'eval_state.dart';

/// [HandValue] for traditional "high" poker hands.
/// {@category poker}
class HandValueHigh extends HandValue<HandLevelHigh> {
  HandValueHigh(HandLevelHigh level, List<Rank> ranks)
      : super(level, ranks, ojHighHandValueFunction(level, ranks));

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
  HandValueFactory<HandValueHigh> get worstHandValue {
    return () => HandValueHigh(HandLevelHigh.NoPair,
        [Rank.Seven, Rank.Five, Rank.Four, Rank.Trey, Rank.Deuce]);
  }

  HandEvaluatorHigh();

  @override
  HandValueHigh referenceEvaluator(Hand h) {
    assert(_allValidCards(h) > 0);
    assert(_allValidCards(h) <= completeHand());
    var st = PokerEvaluatorState(h);

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
    assert(st.verifyNoPair());
    return HandValueHigh(HandLevelHigh.NoPair, st.ranks);
  }
}

int _allValidCards(Hand h) {
  int ret = 0;

  for (int i = 0; i < h.length; i += 1) {
    if (h.cardAt(i)!.suit == Suit.None) {
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
