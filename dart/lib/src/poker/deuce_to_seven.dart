import '../cards/cards.dart';
import 'hand_value.dart';
import 'eval_state.dart';

/// [HandValue] for deuce-to-seven "Kansas City" low poker hands.
/// {@category poker}
class HandValueDeuceToSeven extends HandValue<HandLevelDeuceToSeven> {
  HandValueDeuceToSeven(HandLevelDeuceToSeven level, List<Rank> ranks)
      : super(level, ranks, ojLowHandValueFunction(level, ranks));

  @override
  String fullName() {
    List<String> r1 = ranks.map((r) => r.name).toList();
    List<String> r2 = ranks.map((r) => r.plural).toList();
    List<String> r3 = ranks.map((r) => r.article).toList();

    switch (level) {
      case HandLevelDeuceToSeven.StraightFlush:
        if (ranks[0] == Rank.Ace) {
          return "royal flush";
        }
        return "${r1[0]}-high straight flush";
      case HandLevelDeuceToSeven.Quads:
        return "four ${r2[0]} with ${r3[4]} ${r1[4]}";
      case HandLevelDeuceToSeven.FullHouse:
        return "${r2[0]} full of ${r2[3]}";
      case HandLevelDeuceToSeven.Flush:
        return "flush: ${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      case HandLevelDeuceToSeven.Straight:
        return "${r1[0]}-high straight";
      case HandLevelDeuceToSeven.Trips:
        return "three ${r2[0]}, ${r1[3]}, ${r1[4]}";
      case HandLevelDeuceToSeven.TwoPair:
        return "${r2[0]} and ${r2[2]} with ${r3[4]} ${r1[4]}";
      case HandLevelDeuceToSeven.Pair:
        return "pair of ${r2[0]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      default:
        assert(level == HandLevelDeuceToSeven.NoPair);
        return "${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
    }
  }
}

/// Evaluation functions for deuce-to-seven low poker hands.
/// {@category poker}
class HandEvaluatorDeuceToSeven extends HandEvaluator<HandValueDeuceToSeven> {
  @override
  HandValueFactory<HandValueDeuceToSeven> get worstHandValue {
    return () => HandValueDeuceToSeven(HandLevelDeuceToSeven.StraightFlush,
        [Rank.Ace, Rank.King, Rank.Queen, Rank.Jack, Rank.Ten]);
  }

  HandEvaluatorDeuceToSeven();

  @override
  HandValueDeuceToSeven referenceEvaluator(Hand h) {
    assert(_allValidCards(h) > 0);
    assert(_allValidCards(h) <= completeHand());
    var st = PokerEvaluatorState(h);

    st.checkFlush();
    st.checkStraight(wheelIsStraight: false);

    if (st.flush.isTrue() && st.straight.isTrue()) {
      return HandValueDeuceToSeven(
          HandLevelDeuceToSeven.StraightFlush, st.ranks);
    }
    if (st.flush.isTrue()) {
      assert(st.straight.isFalse());
      return HandValueDeuceToSeven(HandLevelDeuceToSeven.Flush, st.ranks);
    }
    if (st.straight.isTrue()) {
      assert(st.flush.isFalse());
      return HandValueDeuceToSeven(HandLevelDeuceToSeven.Straight, st.ranks);
    }
    st.checkQuads();

    if (st.quads.isTrue()) {
      return HandValueDeuceToSeven(HandLevelDeuceToSeven.Quads, st.ranks);
    }
    st.checkFullHouse();

    if (st.fullHouse.isTrue()) {
      return HandValueDeuceToSeven(HandLevelDeuceToSeven.FullHouse, st.ranks);
    }
    st.checkTrips();

    if (st.trips.isTrue()) {
      return HandValueDeuceToSeven(HandLevelDeuceToSeven.Trips, st.ranks);
    }
    st.checkTwoPair();

    if (st.twoPair.isTrue()) {
      return HandValueDeuceToSeven(HandLevelDeuceToSeven.TwoPair, st.ranks);
    }
    st.checkOnePair();

    if (st.pair.isTrue()) {
      return HandValueDeuceToSeven(HandLevelDeuceToSeven.Pair, st.ranks);
    }
    assert(st.allChecksComplete());
    assert(st.verifyNoPair());
    return HandValueDeuceToSeven(HandLevelDeuceToSeven.NoPair, st.ranks);
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
