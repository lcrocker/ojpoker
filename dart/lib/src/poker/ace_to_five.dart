import '../cards/cards.dart';
import '../troolean.dart';
import 'hand_value.dart';
import 'eval_state.dart';

/// [HandValue] for ace-to-five low poker hands.
/// {@category poker}
class HandValueAceToFive extends HandValue<HandLevelAceToFive> {
  HandValueAceToFive(HandLevelAceToFive level, List<Rank> ranks)
      : super(level, ranks, ojLowHandValueFunction(level, ranks));

  @override
  String fullName() {
    List<String> r1 = ranks.map((r) => r.name).toList();
    List<String> r2 = ranks.map((r) => r.plural).toList();
    List<String> r3 = ranks.map((r) => r.article).toList();

    switch (level) {
      case HandLevelAceToFive.Quads:
        return "four ${r2[0]} with ${r3[4]} ${r1[4]}";
      case HandLevelAceToFive.FullHouse:
        return "${r2[0]} full of ${r2[3]}";
      case HandLevelAceToFive.Trips:
        return "three ${r2[0]}, ${r1[3]}, ${r1[4]}";
      case HandLevelAceToFive.TwoPair:
        return "${r2[0]} and ${r2[2]} with ${r3[4]} ${r1[4]}";
      case HandLevelAceToFive.Pair:
        return "pair of ${r2[0]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      default:
        assert(level == HandLevelAceToFive.NoPair);
        return "${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
    }
  }
}

/// Evaluation functions for traditional "high" poker hands.
/// {@category poker}
class HandEvaluatorAceToFive extends HandEvaluator<HandValueAceToFive> {
  @override
  HandValueFactory<HandValueAceToFive> get worstHandValue {
    return () => HandValueAceToFive(HandLevelAceToFive.Quads,
        [Rank.King, Rank.King, Rank.King, Rank.King, Rank.Queen]);
  }

  HandEvaluatorAceToFive();

  @override
  HandValueAceToFive referenceEvaluator(Hand h) {
    assert(_allValidCards(h) > 0);
    assert(_allValidCards(h) <= completeHand());
    var st = PokerEvaluatorState(h);

    st.straight = TFALSE;
    st.flush = TFALSE;
    st.checkQuads();

    if (st.quads.isTrue()) {
      return HandValueAceToFive(HandLevelAceToFive.Quads, st.ranks);
    }
    st.checkFullHouse();

    if (st.fullHouse.isTrue()) {
      return HandValueAceToFive(HandLevelAceToFive.FullHouse, st.ranks);
    }
    st.checkTrips();

    if (st.trips.isTrue()) {
      return HandValueAceToFive(HandLevelAceToFive.Trips, st.ranks);
    }
    st.checkTwoPair();

    if (st.twoPair.isTrue()) {
      return HandValueAceToFive(HandLevelAceToFive.TwoPair, st.ranks);
    }
    st.checkOnePair();

    if (st.pair.isTrue()) {
      return HandValueAceToFive(HandLevelAceToFive.Pair, st.ranks);
    }
    assert(st.allChecksComplete());
    assert(st.verifyNoPair());
    return HandValueAceToFive(HandLevelAceToFive.NoPair, st.ranks);
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
