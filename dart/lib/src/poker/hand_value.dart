// ignore_for_file: constant_identifier_names
import '../cards/cards.dart';
import '../poker/high_hand.dart';

abstract class HandLevelInterface {
  int get index;
  HandLevelInterface fromIndex(int x);
  HandLevelInterface get best;
  HandLevelInterface get worst;
}

enum HandLevelHigh implements HandLevelInterface {
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
  NoPair; // 10

  @override
  HandLevelHigh fromIndex(int x) {
    return values[x];
  }

  @override
  HandLevelHigh get best {
    return FiveOfAKind;
  }

  @override
  HandLevelHigh get worst {
    return NoPair;
  }
}

typedef HandLevelPaiGow = HandLevelHigh;

enum HandLevelStripped implements HandLevelInterface {
  None, // 0
  FiveOfAKind, // 1
  StraightFlush, // 2
  Quads, // 3
  Flush, // 4
  FullHouse, // 5
  Straight, // 6
  Trips, // 7
  TwoPair, // 8
  Pair, // 9
  NoPair; // 10

  @override
  HandLevelStripped fromIndex(int x) {
    return values[x];
  }

  @override
  HandLevelStripped get best {
    return FiveOfAKind;
  }

  @override
  HandLevelStripped get worst {
    return NoPair;
  }
}

typedef HandLevelManilla = HandLevelStripped;
typedef HandLevelMexican = HandLevelStripped;

enum HandLevelAceToFive implements HandLevelInterface {
  None, // 0
  NoPair, // 1
  Pair, // 2
  TwoPair, // 3
  Trips, // 4
  FullHouse, // 5
  Quads; // 6

  @override
  HandLevelAceToFive fromIndex(int x) {
    return values[x];
  }

  @override
  HandLevelAceToFive get best {
    return NoPair;
  }

  @override
  HandLevelAceToFive get worst {
    return Quads;
  }
}

enum HandLevelDeuceToSeven implements HandLevelInterface {
  None, // 0
  NoPair, // 1
  Pair, // 2
  TwoPair, // 3
  Trips, // 4
  Straight, // 5
  Flush, // 6
  FullHouse, // 7
  Quads, // 8
  StraightFlush; // 9

  @override
  HandLevelDeuceToSeven fromIndex(int x) {
    return values[x];
  }

  @override
  HandLevelDeuceToSeven get best {
    return NoPair;
  }

  @override
  HandLevelDeuceToSeven get worst {
    return Quads;
  }
}

typedef HandLevelAceToSix = HandLevelDeuceToSeven;

enum HandLevelBadugi implements HandLevelInterface {
  None, // 0
  FourCard, // 1
  ThreeCard, // 2
  TwoCard, // 3
  OneCard; // 4

  @override
  HandLevelBadugi fromIndex(int x) {
    return values[x];
  }

  @override
  HandLevelBadugi get best {
    return FourCard;
  }

  @override
  HandLevelBadugi get worst {
    return OneCard;
  }
}

enum HandLevelActionRazz implements HandLevelInterface {
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
  UnqualifiedQuads; // 12

  @override
  HandLevelActionRazz fromIndex(int x) {
    return values[x];
  }

  @override
  HandLevelActionRazz get best {
    return QualifiedNoPair;
  }

  @override
  HandLevelActionRazz get worst {
    return UnqualifiedQuads;
  }
}

/// Default function for assigning a numeric value to the [Hand] such that
/// lower number equals better hand.
///
/// Start with just the level times 10 million.
/// Then within each level, use `PositionalHash.u32csr()` to
/// express the ranks as an n-digit number in base 16 (which should be less
/// than 10000000). That number will be high for better hands within the
/// level, so we negate it.
int ojHighHandValueFunction(HandLevelInterface level, List<Rank> ranks) {
  return 10000000 * level.index - ojhPositional32cr(ranks);
}

int ojLowHandValueFunction(HandLevelInterface level, List<Rank> ranks) {
  return 10000000 * level.index + ojhPositional32cr(ranks);
}

/// All the information resulting from the evaluation of a poker hand.
///
/// This is used for comparing hands to determine a winner, and also for
/// displaying the hand appropriately.
/// Poker hands are typically evaluated into one of a small set of
/// categories we call "levels" (e.g. Pair, Flush, Straight) and then
/// within that category, the ranks of the cards are compared.
/// All the numbers here are such that lower number means better hand.
/// {@category poker}
class HandValue<L extends HandLevelInterface> {
  final L level;
  final List<Rank> ranks;
  final int value;

  /// Constructor for internal use
  HandValue(this.level, this.ranks, int v)
      : value = ((v >= 0) ? v : 
          ((-1 == v) ? ojHighHandValueFunction(level, ranks) :
           ojLowHandValueFunction(level, ranks)));

  // Another ugly hack to work around Dart's broken generics
  HandValue.best()
      : level = HandLevelHigh.StraightFlush as L,
        ranks = [],
        value = 1;

  /// Implement == for [HandValue]
  @override
  bool operator ==(Object other) {
    if (other is HandValue) {
      return value == other.value;
    }
    return false;
  }

  /// Overriding == requires overriding hashCode as well
  @override
  int get hashCode {
    return ojHighHandValueFunction(level, ranks);
  }

  /// Describe the [Hand] (we expect every implementor to override this).
  fullName() {
    return "$level $value";
  }

  /// Re-order the [Card]s for value-appropriate display
  /// (e.g. the hand "5h3cAc3h3d" will display as "3h3d3cAc5h").
  OrphanHand orderedForDisplay(HandInterface h) {
    List<Card> hIn = h.toList();
    assert(hIn.length == 5);
    assert(ranks.length == 5);
    OrphanHand hOut = OrphanHand();

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
      hOut.push(found);
    }
    assert(hOut.length == 5);
    return hOut;
  }

  /// Implementors will override this with something short for debugging.
  @override
  toString() {
    return fullName();
  }
}

typedef HandValueFactory<V> = V Function();

/// Base class for poker hand evaluation.
///
/// Provides a framework for the subclasses and provides some default
/// implementations.
/// {@category poker}
abstract class HandEvaluator<V extends HandValue> {
  // This abomination is to work around the problem that Dart generics
  // are broken, and do not allow the creation of a generic object in
  // any sensible way.
  HandValueFactory<V> get valueFactory {
    return () => HandValue.best() as V;
  }

  V referenceEvaluator(HandInterface h);

  V partialEvaluator(HandInterface h) {
    return referenceEvaluator(h);
  }

  V lookupEvaluator(HandInterface h) {
    return referenceEvaluator(h);
  }

  int fastValue(HandInterface h) {
    return referenceEvaluator(h).value;
  }

  V valueOf(HandInterface h) {
    if (h.length < 5) {
      return partialEvaluator(h);
    } else if (h.length == 5) {
      return lookupEvaluator(h);
    } else {
      return bestOf(h);
    }
  }

  V bestOf(HandInterface h) {
    V best = valueFactory();

    for (HandInterface sub in h.combinations(5)) {
      V subV = lookupEvaluator(sub);
      if (subV.value < best.value) {
        best = subV;
      }
    }
    return best;
  }
}

/// Return a [HandEvaluator] by name.
/// {@category poker}
HandEvaluator pokerEvaluator(String name) {
  switch (name) {
    case "standard":
    case "default":
    case "high":
      return HandEvaluatorHigh();
    default:
      throw ArgumentError('Unknown poker evaluator: $name');
  }
}
