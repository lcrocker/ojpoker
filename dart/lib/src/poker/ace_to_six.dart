import '../troolean.dart';
import '../cards/cards.dart';
import 'hand_value.dart';
import 'eval_state.dart';

/// [HandValue] for ace-to-six "London" low poker hands.
/// {@category poker}
class HandValueAceToSix extends HandValue<HandLevelAceToSix> {
  HandValueAceToSix(HandLevelAceToSix level, List<Rank> ranks, int vAdj)
    : super(level, ranks, vAdj + ojLowHandValueFunction(level, ranks));

  @override
  String fullName() {
    List<String> r1 = ranks.map((r) => r.name).toList();
    List<String> r2 = ranks.map((r) => r.plural).toList();
    List<String> r3 = ranks.map((r) => r.article).toList();

    switch (level) {
      case HandLevelAceToSix.StraightFlush:
        if (ranks[0] == Rank.Ace) {
          return "royal flush";
        }
        return "${r1[0]}-high straight flush";
      case HandLevelAceToSix.Quads:
        return "four ${r2[0]} with ${r3[4]} ${r1[4]}";
      case HandLevelAceToSix.FullHouse:
        return "${r2[0]} full of ${r2[3]}";
      case HandLevelAceToSix.Flush:
        return "flush: ${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      case HandLevelAceToSix.Straight:
        return "${r1[0]}-high straight";
      case HandLevelAceToSix.Trips:
        return "three ${r2[0]}, ${r1[3]}, ${r1[4]}";
      case HandLevelAceToSix.TwoPair:
        return "${r2[0]} and ${r2[2]} with ${r3[4]} ${r1[4]}";
      case HandLevelAceToSix.Pair:
        return "pair of ${r2[0]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      default:
        assert(level == HandLevelAceToSix.NoPair);
        return "${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
    }
  }
}

/// Evaluation functions for ace-to-six low poker hands.
/// {@category poker}
class HandEvaluatorAceToSix extends HandEvaluator<HandValueAceToSix> {
  @override
  HandValueFactory<HandValueAceToSix> get worstHandValue {
    return () => HandValueAceToSix(HandLevelAceToSix.StraightFlush,
        [Rank.LowAce, Rank.King, Rank.Queen, Rank.Jack, Rank.Ten], 0xE0000);
  }

  HandEvaluatorAceToSix();

  @override
  HandValueAceToSix referenceEvaluator(Hand h) {
    assert(_allValidCards(h) > 0);
    assert(_allValidCards(h) <= completeHand());
    var st = PokerEvaluatorState(h);

    st.checkFlush();
    st.checkStraight();

    // NOTE: I cannot find a definitive source for whether or not broadway
    // counts as a straight in London lowball. The only source I did find
    // says that it does, but that there is "some controversy" over it.
    // It should never come up in practice, but I'd like to get it right.

    bool lowBroadway = st.ranks[0] == Rank.King &&
        st.ranks[1] == Rank.Queen &&
        st.ranks[2] == Rank.Jack &&
        st.ranks[3] == Rank.Ten &&
        st.ranks[4] == Rank.LowAce;

    if (lowBroadway) {
      st.ranks[0] = Rank.LowAce;
      st.ranks[1] = Rank.King;
      st.ranks[2] = Rank.Queen;
      st.ranks[3] = Rank.Jack;
      st.ranks[4] = Rank.Ten;

      st.straight = TTRUE;
      st.sorted = TFALSE;
    }

    if (st.flush.isTrue() && st.straight.isTrue()) {
      return HandValueAceToSix(HandLevelAceToSix.StraightFlush, st.ranks,
        lowBroadway ? 0xE0000 : 0); // to adjust for low ace
    }
    if (st.flush.isTrue()) {
      assert(st.straight.isFalse());
      return HandValueAceToSix(HandLevelAceToSix.Flush, st.ranks, 0);
    }
    if (st.straight.isTrue()) {
      assert(st.flush.isFalse());
      return HandValueAceToSix(HandLevelAceToSix.Straight, st.ranks,
        lowBroadway ? 0xE0000 : 0);
    }
    st.checkQuads();

    if (st.quads.isTrue()) {
      return HandValueAceToSix(HandLevelAceToSix.Quads, st.ranks, 0);
    }
    st.checkFullHouse();

    if (st.fullHouse.isTrue()) {
      return HandValueAceToSix(HandLevelAceToSix.FullHouse, st.ranks, 0);
    }
    st.checkTrips();

    if (st.trips.isTrue()) {
      return HandValueAceToSix(HandLevelAceToSix.Trips, st.ranks, 0);
    }
    st.checkTwoPair();

    if (st.twoPair.isTrue()) {
      return HandValueAceToSix(HandLevelAceToSix.TwoPair, st.ranks, 0);
    }
    st.checkOnePair();

    if (st.pair.isTrue()) {
      return HandValueAceToSix(HandLevelAceToSix.Pair, st.ranks, 0);
    }
    assert(st.allChecksComplete());
    assert(st.verifyNoPair());
    return HandValueAceToSix(HandLevelAceToSix.NoPair, st.ranks, 0);
  }
}

int _allValidCards(Hand h) {
  int ret = 0;

  for (int i = 0; i < h.length; i += 1) {
    if (h.cardAt(i)!.suit == Suit.None) {
      return 0;
    }
    if (h.cardAt(i)!.rank == Rank.Ace) {
      return 0;
    }
    if (h.cardAt(i)!.rank == Rank.Knight) {
      return 0;
    }
    ret += 1;
  }
  return ret;
}
