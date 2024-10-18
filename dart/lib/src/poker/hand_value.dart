// ignore_for_file: constant_identifier_names
import 'dart:io';
import 'package:msgpack_dart/msgpack_dart.dart' as mp;
import '../cards/cards.dart';
import '../poker/high_hand.dart';

/// All the information resulting from the evaluation of a poker hand.
///
/// This is used for comparing hands to determine a winner, and also for
/// displaying the hand appropriately.
/// Poker hands are typically evaluated into one of a small set of
/// categories we call [HandLevel]s (e.g. Pair, Flush, Straight) and then
/// within that category, the ranks of the cards are compared.
/// All the numbers here are such that lower number means better hand.
/// {@category poker}
class HandValue {
  static PositionalHash hasher = PositionalHash();

  final int level;
  final List<Rank> ranks;
  final int value;

  /// Constructor for internal use
  HandValue(this.level, this.ranks, int? v)
      : value = v ?? HandValue.defaultValueFunction(level, ranks);

  /// Default function for assigning a numeric value to the [Hand] such that
  /// lower number equals better hand.
  ///
  /// Start with just the [HandLevel] times
  /// 10 million. Then within each level, use `PositionalHash.u32csr()` to
  /// express the ranks as an n-digit number in base 16 (which should be less
  /// than 10000000). That number will be high for better hands within the
  /// level, so we negate it.
  static defaultValueFunction(int level, List<Rank> ranks) {
    int h = hasher.u32csr(ranks);
    return 10000000 * level - h;
  }

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
    return HandValue.defaultValueFunction(level, ranks);
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

typedef HandValueFactory<V> = V Function(
    int level, List<Rank> ranks, int value);

/// Standard format for lookup tables for fast evaluation of poker hands.
/// {@category poker}
class HandEvaluationTables {
  int hashCount;
  int eclassCount;
  Map<int, int> hashes;
  List<int> ecLevels;
  List<List<Rank>> ecRanks;

  HandEvaluationTables()
      : hashCount = 0,
        eclassCount = 0,
        hashes = {},
        ecLevels = [],
        ecRanks = [];
}

/// Base class for poker hand evaluation.
///
/// Provides a framework for the subclasses and provides some default
/// implementations.
/// {@category poker}
abstract class HandEvaluator<V extends HandValue> {
  static CardHashBase hasher = PrimeHash();
  static bool tablesLoaded = false;
  static HandEvaluationTables tables = HandEvaluationTables();

  // This abomination is to work around the problem that Dart generics
  // are broken, and do not allow the creation of a generic object in
  // any sensible way.
  HandValueFactory<V> get valueFactory {
    return (int level, List<Rank> ranks, int value) =>
        HandValue(level, ranks, value) as V;
  }

  static bool loadMsgPackTables(String fname) {
    if (tablesLoaded) {
      return true;
    }
    var file = File('../data/bin/$fname.msgpack');
    if (!file.existsSync()) {
      return false;
    }
    var bytes = file.readAsBytesSync();
    var data = mp.deserialize(bytes);

    tables.hashCount = data['hash_count'];
    tables.eclassCount = data['eclass_count'];
    tables.hashes.addEntries(data['hashes']);
    tables.ecLevels = data['eclasses'].map((e) => e[0]).toList();
    tables.ecRanks = data['eclasses']
        .map((e) => e[1].map((r) => Rank.values[r]).toList())
        .toList();
    return true;
  }

  V referenceEvaluator(HandInterface h);

  V partialEvaluator(HandInterface h);

  V lookupEvaluator(HandInterface h) {
    if (!tablesLoaded) {
      return referenceEvaluator(h);
    }
    int hv = hasher.u64co(h);
    int ec = tables.hashes[hv]!;

    return valueFactory(tables.ecLevels[ec - 1], tables.ecRanks[ec - 1], ec);
  }

  int fastValue(HandInterface h) {
    if (tablesLoaded) {
      return tables.hashes[hasher.u64co(h)]!;
    }
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
    V best = valueFactory(99, [], 0x7FFFFFFFFFFFFFFF);

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
    case 'standard':
    case 'default':
    case 'high':
      return HighHandEvaluator();
    default:
      throw ArgumentError('Unknown poker evaluator: $name');
  }
}
