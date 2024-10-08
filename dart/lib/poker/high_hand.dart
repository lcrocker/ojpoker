import 'dart:io';
import 'package:onejoker/onejoker.dart';
import 'package:msgpack_dart/msgpack_dart.dart' as mp;

Rank nextLowerRank(Rank r) {
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

class EvaluatorState {
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

  EvaluatorState(HandInterface h)
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

    assert(ranks.length == 5);
    assert(ranks[0].index >= ranks[1].index);
    assert(ranks[1].index >= ranks[2].index);
    assert(ranks[2].index >= ranks[3].index);
    assert(ranks[3].index >= ranks[4].index);
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
    flush = TTRUE;
    for (int i = 1; i < cards.length; i += 1) {
      if (cards[i].suit != cards[0].suit) {
        flush = TFALSE;
        return;
      }
    }
  }

  checkStraight() {
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
      if (ranks[i] != nextLowerRank(ranks[i - 1])) {
        straight = TFALSE;
        return;
      }
    }
    straight = TTRUE;
  }

  checkQuads() {
    assert(sorted.isTrue());

    // AAAAB
    if (ranks[0] == ranks[1] && ranks[0] == ranks[2] && ranks[0] == ranks[3]) {
      quads = TTRUE;
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
    assert(sorted.isTrue());
    assert(quads.isFalse());
    assert(ranks[0] != ranks[4]);

    // AAABB
    if (ranks[0] == ranks[1] &&
      ranks[0] == ranks[2] &&
      ranks[3] == ranks[4]) {
  
      fullHouse = TTRUE;
      return;
    }
    // AABBB
    if (ranks[0] == ranks[1] &&
      ranks[2] == ranks[3] &&
      ranks[2] == ranks[4]) {
  
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
    assert(sorted.isTrue());
    assert(quads.isFalse());
    assert(fullHouse.isFalse());

    // AAABC
    if (ranks[0] == ranks[1] && ranks[0] == ranks[2]) {
      assert(ranks[3] != ranks[4]);
      trips = TTRUE;
      return;
    }
    // ABBBC
    if (ranks[1] == ranks[2] && ranks[1] == ranks[3]) {
      ranks[3] = ranks[0];
      ranks[0] = ranks[1];

      assert(ranks[3] != ranks[4]);
      sorted = TFALSE;
      trips = TTRUE;
      return;
    }
    // ABCCC
    if (ranks[2] == ranks[3] && ranks[2] == ranks[4]) {
      ranks[4] = ranks[1];
      ranks[3] = ranks[0];
      ranks[0] = ranks[2];
      ranks[1] = ranks[2];

      assert(ranks[3] != ranks[4]);
      sorted = TFALSE;
      trips = TTRUE;
      return;
    }
    trips = TFALSE;
  }

  checkTwoPair() {
    assert(sorted.isTrue());
    assert(quads.isFalse());
    assert(fullHouse.isFalse());
    assert(trips.isFalse());

    // AABBC
    if (ranks[0] == ranks[1] && ranks[2] == ranks[3]) {
      assert(ranks[0] != ranks[4]);
      assert(ranks[2] != ranks[4]);
      twoPair = TTRUE;
      return;
    }
    // ABBCC
    if (ranks[1] == ranks[2] && ranks[3] == ranks[4]) {
      ranks[4] = ranks[0];
      ranks[0] = ranks[1];
      ranks[2] = ranks[3];

      assert(ranks[0] != ranks[4]);
      assert(ranks[2] != ranks[4]);
      sorted = TFALSE;
      twoPair = TTRUE;
      return;
    }
    // AABCC
    if (ranks[0] == ranks[1] && ranks[3] == ranks[4]) {
      ranks[4] = ranks[2];
      ranks[2] = ranks[3];

      assert(ranks[0] != ranks[4]);
      assert(ranks[2] != ranks[4]);
      sorted = TFALSE;
      twoPair = TTRUE;
      return;
    }
    twoPair = TFALSE;
  }

  checkOnePair() {
    assert(sorted.isTrue());
    assert(quads.isFalse());
    assert(fullHouse.isFalse());
    assert(trips.isFalse());
    assert(twoPair.isFalse());

    // AABCD
    if (ranks[0] == ranks[1]) {
      assert(ranks[0] != ranks[2]);
      assert(ranks[0] != ranks[3]);
      assert(ranks[0] != ranks[4]);
      assert(ranks[2] != ranks[3]);
      assert(ranks[2] != ranks[4]);
      assert(ranks[3] != ranks[4]);
      pair = TTRUE;
      return;
    }
    // ABBCD
    if (ranks[1] == ranks[2]) {
      ranks[2] = ranks[0];
      ranks[0] = ranks[1];

      assert(ranks[0] != ranks[2]);
      assert(ranks[0] != ranks[3]);
      assert(ranks[0] != ranks[4]);
      assert(ranks[2] != ranks[3]);
      assert(ranks[2] != ranks[4]);
      assert(ranks[3] != ranks[4]);
      sorted = TFALSE;
      pair = TTRUE;
      return;
    }
    // ABCCD
    if (ranks[2] == ranks[3]) {
      ranks[3] = ranks[1];
      ranks[1] = ranks[2];
      ranks[2] = ranks[0];
      ranks[0] = ranks[1];

      assert(ranks[0] != ranks[2]);
      assert(ranks[0] != ranks[3]);
      assert(ranks[0] != ranks[4]);
      assert(ranks[2] != ranks[3]);
      assert(ranks[2] != ranks[4]);
      assert(ranks[3] != ranks[4]);
      sorted = TFALSE;
      pair = TTRUE;
      return;
    }
    // ABCDD
    if (ranks[3] == ranks[4]) {
      ranks[4] = ranks[2];
      ranks[2] = ranks[0];
      ranks[0] = ranks[3];
      ranks[3] = ranks[1];
      ranks[1] = ranks[0];

      assert(ranks[0] != ranks[2]);
      assert(ranks[0] != ranks[3]);
      assert(ranks[0] != ranks[4]);
      assert(ranks[2] != ranks[3]);
      assert(ranks[2] != ranks[4]);
      assert(ranks[3] != ranks[4]);
      sorted = TFALSE;
      pair = TTRUE;
      return;
    }
    pair = TFALSE;
  }
}

bool isValidFiveCards(HandInterface h) {
  if (h.length != 5) {
    return false;
  }
  for (int i = 0; i < 5; i += 1) {
    if (h.cardAt(i)!.suit == null) {
      return false;
    }
    if (h.cardAt(i)!.rank == Rank.LowAce) {
      return false;
    }
    if (h.cardAt(i)!.rank == Rank.Knight) {
      return false;
    }
  }
  return true;
}

class HighHandHashTables {
  int hashCount;
  int eclassCount;
  Map<int, int> hashes;
  List<int> ecLevels;
  List<List<Rank>> ecRanks;

  HighHandHashTables()
      : hashCount = 0,
        eclassCount = 0,
        hashes = {},
        ecLevels = List.filled(7462, 0),
        ecRanks = List.generate(
            7462, (_) => [Rank.Ace, Rank.Ace, Rank.Ace, Rank.Ace, Rank.Ace]);
}

class HandValueHigh extends HandValueBase<HandLevelHigh> {
  static bool _tablesLoaded = false;
  static bool _tablesFailed = false;
  static final HighHandHashTables _tables = HighHandHashTables();

  HandValueHigh(HandInterface hand, HandLevelHigh level, List<Rank> ranks,
    [int? v]): super(hand, HandScale.HighHand, level, ranks, v);

  factory HandValueHigh.referenceEvaluator(HandInterface hand) {
    assert(isValidFiveCards(hand));
    var st = EvaluatorState(hand);

    st.checkFlush();
    st.checkStraight();

    if (st.flush.isTrue() && st.straight.isTrue()) {
      return HandValueHigh(hand, HandLevelHigh.StraightFlush, st.ranks);
    }
    if (st.flush.isTrue()) {
      assert(st.straight.isFalse());
      return HandValueHigh(hand, HandLevelHigh.Flush, st.ranks);
    }
    if (st.straight.isTrue()) {
      assert(st.flush.isFalse());
      return HandValueHigh(hand, HandLevelHigh.Straight, st.ranks);
    }
    st.checkQuads();
  
    if (st.quads.isTrue()) {
      return HandValueHigh(hand, HandLevelHigh.Quads, st.ranks);
    }
    st.checkFullHouse();

    if (st.fullHouse.isTrue()) {
      return HandValueHigh(hand, HandLevelHigh.FullHouse, st.ranks);
    }
    st.checkTrips();
  
    if (st.trips.isTrue()) {
      return HandValueHigh(hand, HandLevelHigh.Trips, st.ranks);
    }
    st.checkTwoPair();

    if (st.twoPair.isTrue()) {
      return HandValueHigh(hand, HandLevelHigh.TwoPair, st.ranks);
    }
    st.checkOnePair();
  
    if (st.pair.isTrue()) {
      return HandValueHigh(hand, HandLevelHigh.Pair, st.ranks);
    }
    assert(st.allChecksComplete());
    return HandValueHigh(hand, HandLevelHigh.NoPair, st.ranks);
  }

  factory HandValueHigh.lookupEvaluator(HandInterface hand) {
    if (_tablesFailed) {
      return HandValueHigh.referenceEvaluator(hand);
    }
    if (!_tablesLoaded) {
      var file = File('../data/bin/high_hand_prime_hash.msgpack');
      if (!file.existsSync()) {
        print("Lookup tables not found. Falling back to reference evaluator.");
        _tablesFailed = true;
        return HandValueHigh.referenceEvaluator(hand);
      }
      var bytes = file.readAsBytesSync();
      var data = mp.deserialize(bytes);

      _tables.hashCount = data['hash_count'];
      _tables.eclassCount = data['eclass_count'];
      _tables.hashes.addEntries(data['hashes']);
      _tables.ecLevels = data['eclasses'].map((e) => e[0]).toList();
      _tables.ecRanks = data['eclasses']
        .map((e) => e[1].map((r) => Rank.values[r]).toList()).toList();
      _tablesLoaded = true;
    }
    int h = PrimeHash.u64co(hand);
    int ec = _tables.hashes[h]!;

    return HandValueHigh(hand, HandLevelHigh.values[_tables.ecLevels[ec - 1]],
        _tables.ecRanks[ec - 1], ec);
  }

  static int fastEvaluator(HandInterface hand) {
    if (_tablesLoaded) {
      return _tables.hashes[PrimeHash.u64co(hand)]!;
    }
    var v = HandValueHigh.lookupEvaluator(hand);
    return v.value;
  }

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
      case HandLevelHigh.NoPair:
        return "no pair: ${r1[0]}, ${r1[1]}, ${r1[2]}, ${r1[3]}, ${r1[4]}";
      default:
        return "unknown hand";
    }
  }

  @override
  String toString() {
    return "$scale $level $ranks";
  }
}
