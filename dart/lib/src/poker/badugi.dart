import 'package:onejoker/src/utilities.dart';

import '../cards/cards.dart';
import 'hand_value.dart';

/// [HandValue] for Badugi hands.
/// {@category poker}
class HandValueBadugi extends HandValue<HandLevelBadugi> {
  HandValueBadugi(super.level, super.ranks, super.value);

  @override
  String fullName() {
    List<String> r1 = ranks.map((r) => r.name).toList();

    switch (level) {
      case HandLevelBadugi.FourCard:
        if (ranks[0] == Rank.Four) {
          return "perfect badugi";
        }
        return "four-card ${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}";
      case HandLevelBadugi.ThreeCard:
        return "three-card ${r1[0]}, ${r1[1]}, ${r1[2]}";
      case HandLevelBadugi.TwoCard:
        return "two-card ${r1[0]}, ${r1[1]}";
      default:
        assert(level == HandLevelBadugi.OneCard);
        return "one-card ${r1[0]}";
    }
  }
}

(int, List<Rank>) badugiPartialValue(List<Card> cl) {
  HandLevelBadugi level = [
    HandLevelBadugi.None,
    HandLevelBadugi.OneCard,
    HandLevelBadugi.TwoCard,
    HandLevelBadugi.ThreeCard,
    HandLevelBadugi.FourCard
  ][cl.length];

  List<Rank> ranks = cl.map((c) => c.rank).toList();
  ojSort(ranks);

  int v = 0;
  for (int i = 0; i < ranks.length; i += 1) {
    v <<= 4;
    v += ranks[i].index;
  }
  return (1000000 * level.index + v, ranks);
}

bool isBadugi(Hand h) {
  int rankSet = 0;
  int suitSet = 0;

  for (int i = 0; i < h.length; i += 1) {
    var pSet = rankSet;
    rankSet |= 1 << h[i].rank.index;
    if (pSet == rankSet) {
      return false;
    }
    pSet = suitSet;
    suitSet |= 1 << h[i].suit.index;
    if (pSet == suitSet) {
      return false;
    }
  }
  return true;
}

/// Evaluation functions for Badugi hands.
class HandEvaluatorBadugi extends HandEvaluator<HandValueBadugi> {
  @override
  HandValueFactory<HandValueBadugi> get worstHandValue {
    return () => HandValueBadugi(HandLevelBadugi.OneCard, [Rank.King],
        1000000 * HandLevelBadugi.OneCard.index + Rank.King.index);
  }
  HandEvaluatorBadugi();

  @override
  int completeHand() { return 4; }

  @override
  HandValueBadugi referenceEvaluator(Hand h) {
    assert(_allValidCards(h) > 0);
    assert(_allValidCards(h) <= completeHand());
    Hand h4 = h.clone();

    if (h.length == 4 && isBadugi(h4)) {
      var (v, ranks) = badugiPartialValue(h4.cards);
      return HandValueBadugi(HandLevelBadugi.FourCard, ranks, v);
    }
    Hand best = h.clone();
    best.clear();
    int bestVal = 0x7FFFFFFFFFFFFFFF;
    List<Rank> bestRanks = [];

    if (h.length >= 3) {
      for (var h3 in h.combinations(3)) {
        if (isBadugi(h3)) {
          var (v, ranks) = badugiPartialValue(h3.cards);
          if (v < bestVal) {
            bestVal = v;
            bestRanks = ranks;
            best = h3.clone();
          }
        }
      }
      if (bestVal != 0x7FFFFFFFFFFFFFFF) {
        return HandValueBadugi(HandLevelBadugi.ThreeCard, bestRanks, bestVal);
      }
    }

    if (h.length >= 2) {
      for (var h2 in h.combinations(2)) {
        if (isBadugi(h2)) {
          var (v, ranks) = badugiPartialValue(h2.cards);
          if (v < bestVal) {
            bestVal = v;
            bestRanks = ranks;
            best = h2.clone();
          }
        }
      }
      if (bestVal != 0x7FFFFFFFFFFFFFFF) {
        return HandValueBadugi(HandLevelBadugi.TwoCard, bestRanks, bestVal);
      }
    }

    Card leastCard = h[0];
    for (int i = 1; i < h.length; i += 1) {
      if (h[i].rank < leastCard.rank) {
        leastCard = h[i];
      }
    }
    return HandValueBadugi(HandLevelBadugi.OneCard, [leastCard.rank],
        1000000 * HandLevelBadugi.OneCard.index + leastCard.rank.index);
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
}
