// ignore_for_file: constant_identifier_names

// A HandValue represents the result of evaluating poker hand, and is used
// for comparing hands to determine a winner, and also for displaying the
// hand appropriately.

import 'package:onejoker/onejoker.dart';

enum HandScale {
  None, // 0
  HighHand, // 1
  AceToFive, // 2
  DeuceToSeven, // 3
  AceToSix, // 4
  Badugi, // 5
  PaiGow, // 6
  Manilla, // 7
  Mexican, // 8
  ActionRazz, // 9
}

abstract class HandLevel {
  int get index;
}

enum HandLevelHigh implements HandLevel {
  None, // 0
  FiveOfAKind, // 1
  StraightFlush, // 2
  Quads, // 3
  FullHouse, // 4
  Flush, // 5
  Straight, // 6
  Trips, // 7
  TwoPair, // 8
  Pair, // 9
  NoPair, // 10
}

typedef HandLevelPaiGow = HandLevelHigh;
typedef HandLevelManilla = HandLevelHigh;
typedef HandLevelMexican = HandLevelHigh;

enum HandLevelAceToFive implements HandLevel {
  None, // 0
  NoPair, // 1
  Pair, // 2
  TwoPair, // 3
  Trips, // 4
  FullHouse, // 5
  Quads, // 6
}

enum HandLevelDeuceToSeven implements HandLevel {
  None, // 0
  NoPair, // 1
  Pair, // 2
  TwoPair, // 3
  Trips, // 4
  Straight, // 5
  Flush, // 6
  FullHouse, // 7
  Quads, // 8
  StraightFlush, // 9
}

typedef HandLevelAceToSix = HandLevelDeuceToSeven;

enum HandLevelBadugi implements HandLevel {
  None, // 0
  FourCard, // 1
  ThreeCard, // 2
  TwoCard, // 3
  OneCard, // 4
}

enum HandLevelActionRazz implements HandLevel {
  None, // 0
  QualifiedNoPair, // 1
  QualifiedPair, // 2
  QualifiedTwoPair, // 3
  QualifiedTrips, // 4
  QualifiedFullHouse, // 5
  QualifiedQuads, // 6
  UnqualifiedNoPair, // 7
  UnqualifiedPair, // 8
  UnqualifiedTwoPair, // 9
  UnqualifiedTrips, // 10
  UnqualifiedFullHouse, // 11
  UnqualifiedQuads, // 12
}

class HandValueBase<T extends HandLevel> {
  final HandInterface hand;
  final HandScale scale;
  final T level;
  final List<Rank> ranks;
  final int value;

  HandValueBase(this.hand, this.scale, this.level, this.ranks, [int? v])
    : value = v ??
      HandValueBase.defaultValueFunction(scale.index, level.index, ranks);

  @override
  bool operator ==(Object other) {
    if (other is HandValueBase) {
      return scale == other.scale &&
        level == other.level &&
        value == other.value;
    }
    return false;
  }

  static defaultValueFunction(int scale, int level, List<Rank> ranks) {
    int v = 0;
    for (int i = 0; i < ranks.length; i += 1) {
      v *= 16;
      v += (15 - ranks[i].index); // Lower numbers, higher hand
    }
    return 100000000 * scale + 5000000 * level + v;
  }

  @override
  int get hashCode {
    return HandValueBase.defaultValueFunction(scale.index,
      level.index, ranks);
  }

  fullName() {
    return "$hand $scale $level $value";
  }

  HandInterface orderedForDisplay() {
    List<Card> hIn = hand.toList();
    assert(hIn.length == 5);
    assert(ranks.length == 5);
    List<Card> hOut = [];

    for (int i = 0; i < ranks.length; i += 1) {
      Rank r = ranks[i];
      Card found = Card.None;
      int fIndex = -1;

      for (int j = 0; j < hIn.length; j += 1) {
        if (hIn[j].rank == r && hIn[j] > found) {
          found = hIn[j];
          fIndex = j;
        }
      }
      assert(fIndex >= 0);
      assert(found == hIn.removeAt(fIndex));
      hOut.add(found);
    }
    assert(hOut.length == 5);
    HandInterface ret = hand.clone();
    ret.clear();
    ret.pushN(5, hOut);
    return ret;
  }

  @override
  toString() {
    return this.fullName();
  }
}
