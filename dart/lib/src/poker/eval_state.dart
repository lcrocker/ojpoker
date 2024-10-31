import '../troolean.dart';
import '../utilities.dart';
import '../cards/cards.dart';

// ignore: constant_identifier_names
const List<int> POKER_RANK_ORDER =
    [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, -1, 12, 13, 14 ];

class PokerEvaluatorState {
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

  PokerEvaluatorState(Hand h)
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
    ranks = cards.map((c) => c.rank).toList();
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

  checkStraight({ bool wheelIsStraight = true }) {
    if (cards.length < 5) {
      straight = TFALSE;
      return;
    }
    assert(sorted.isTrue());

    // Special-case wheel
    if (wheelIsStraight) {
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
    }
    for (int i = 1; i < 5; i += 1) {
      if (POKER_RANK_ORDER[ranks[i].index] + 1 !=
        POKER_RANK_ORDER[ranks[i - 1].index]) {

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

  bool verifyNoPair() {
    if (cards.length < 2) {
      return true;
    }
    if (cards.length < 3) {
      return ranks[1] != ranks[0];
    }
    if (cards.length < 4) {
      return ranks[2] != ranks[1] && ranks[2] != ranks[0];
    }
    if (cards.length < 5) {
      return ranks[3] != ranks[2] &&
          ranks[3] != ranks[1] &&
          ranks[3] != ranks[0];
    }
    assert(cards.length == 5);
    return ranks[4] != ranks[3] &&
        ranks[4] != ranks[2] &&
        ranks[4] != ranks[1] &&
        ranks[4] != ranks[0];
  }
}
