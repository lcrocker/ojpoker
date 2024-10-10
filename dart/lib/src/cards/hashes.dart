//
import '../utilities.dart';
import 'cards.dart';

/// Base class for a group of hash functions based on related algorithms.
/// {@category hashes}
class CardHashBase {
  /// 32-bit standard hash
  int u32(Iterable<Card> cards) {
    return 0;
  }

  /// 32-bit collison-free
  int u32c(Iterable<Card> cards) {
    return 0;
  }

  /// 32-bit collision-free order-independent
  int u32co(Iterable<Card> cards) {
    List<Card> sorted = cards.toList();
    ojSort(sorted);
    return u32c(sorted);
  }

  /// 32-bit collision-free suit-independent
  int u32csr(Iterable<Rank> ranks) {
    return 0;
  }

  /// 32-bit collision-free order-independent suit-independent
  int u32cos(Iterable<Card> cards) {
    return u32cosr(cards.map((c) => c.rank!));
  }

  /// 32-bit collision-free order-independent suit-independent
  int u32cosr(Iterable<Rank> ranks) {
    List<Rank> sorted = ranks.toList();
    ojSort(sorted);
    return u32csr(sorted);
  }

  /// 64-bit standard hash
  int u64(Iterable<Card> cards) {
    return 0;
  }

  /// 64-bit collision-free
  int u64c(Iterable<Card> cards) {
    return 0;
  }

  /// 64-bit collision-free order-independent
  int u64co(Iterable<Card> cards) {
    List<Card> sorted = cards.toList();
    ojSort(sorted);
    return u64c(sorted);
  }

  /// 64-bit collision-free order-independent suit-independent
  int u64csr(Iterable<Rank> ranks) {
    return 0;
  }

  /// 64-bit collision-free order-independent suit-independent
  int u64cos(Iterable<Card> cards) {
    return u64cosr(cards.map((c) => c.rank!));
  }

  /// 64-bit collision-free order-independent suit-independent
  int u64cosr(Iterable<Rank> ranks) {
    List<Rank> sorted = ranks.toList();
    ojSort(sorted);
    return u64csr(sorted);
  }
}

/// Standard Fowler-Knoll-Vo.
///
/// May be useful for traditional hash tables,
/// but there are no collision-free variants.
/// {@category hashes}
class FNVHash extends CardHashBase {
  /// 32-bit standard hash
  @override
  int u32(Iterable<Card> cards) {
    int h = 0x811c9dc5;
    for (Card c in cards) {
      h ^= c.index;
      h *= 0x01000193;
    }
    return h & 0xFFFFFFFF;
  }

  /// 64-bit standard hash
  @override
  int u64(Iterable<Card> cards) {
    int h = 0xcbf29ce484222325;
    for (Card c in cards) {
      h ^= c.index;
      h *= 0x100000001b3;
    }
    return h;
  }
}

/// Hash functions based on treating each card as a base-64 (or base-16)
/// digit in a multi-digit integer.
///
/// No collisions, but limited to small sets of cards.
/// {@category hashes}
class PositionalHash extends CardHashBase {
  /// 32-bit collision-free
  @override
  int u32c(Iterable<Card> cards) {
    int max = 5;
    int h = 0;

    for (Card c in cards) {
      max -= 1;
      assert(max >= 0);

      h <<= 6;
      h += (0x3F & c.index);
    }
    return h & 0xFFFFFFFF;
  }

  /// 32-bit collision-free suit-independent
  @override
  int u32csr(Iterable<Rank> ranks) {
    int max = 8;
    int h = 0;

    for (Rank r in ranks) {
      max -= 1;
      assert(max >= 0);

      h <<= 4;
      h += (0x0F & r.index);
    }
    return h & 0xFFFFFFFF;
  }

  /// 64-bit collision-free
  @override
  int u64c(Iterable<Card> cards) {
    int max = 10;
    int h = 0;

    for (Card c in cards) {
      max -= 1;
      assert(max >= 0);

      h <<= 6;
      h += (0x3F & c.index);
    }
    return h;
  }

  /// 64-bit collision-free order-independent suit-independent
  @override
  int u64csr(Iterable<Rank> ranks) {
    int max = 16;
    int h = 0;

    for (Rank r in ranks) {
      max -= 1;
      assert(max >= 0);

      h <<= 4;
      h += (0x0F & r.index);
    }
    return h;
  }
}

/// Hash function based on a 64-bit integer where each bit indicates the
/// presence or absence of card in the set.
///
/// Inherently order-independent, cannot be used on decks with duplicate
/// cards (Pinochle, Canasta, et al.)
/// {@category hashes}
class BitfieldHash extends CardHashBase {
  /// 64-bit collision-free order-independent
  @override
  int u64co(Iterable<Card> cards) {
    int h = 0;

    for (Card c in cards) {
      assert(0 == (h & (1 << (0x3F & c.index))));
      h |= (1 << (0x3F & c.index));
    }
    return h;
  }
}

/// Hash functions based on multiplication of primes.
///
/// Inherently order-independent, can be used with duplicate cards,
/// but limited to hands with a small number of cards.
/// {@category hashes}
class PrimeHash extends CardHashBase {
  /// 32-bit collision-free order-independent suit-independent
  @override
  int u32cosr(Iterable<Rank> ranks) {
    int max = 5;
    int h = 1;

    for (Rank r in ranks) {
      max -= 1;
      assert(max >= 0);

      h *= _primes[0x0F & r.index];
    }
    return h & 0xFFFFFFFF;
  }

  /// 64-bit collision-free order-independent
  @override
  int u64co(Iterable<Card> cards) {
    int max = 7;
    int h = 1;

    for (Card c in cards) {
      max -= 1;
      assert(max >= 0);

      h *= _primes[0x3F & c.index];
    }
    return h;
  }

  /// 64-bit collision-free order-independent suit-independent
  @override
  int u64cosr(Iterable<Rank> ranks) {
    int max = 10;
    int h = 1;

    for (Rank r in ranks) {
      max -= 1;
      assert(max >= 0);

      h *= _primes[0x0F & r.index];
    }
    return h;
  }
}

const List<int> _primes = [
  3,
  5,
  7,
  11,
  13,
  17,
  19,
  23,
  29,
  31,
  37,
  41,
  43,
  47,
  53,
  59,
  61,
  67,
  71,
  73,
  79,
  83,
  89,
  97,
  101,
  103,
  107,
  109,
  113,
  127,
  131,
  137,
  139,
  149,
  151,
  157,
  163,
  167,
  173,
  179,
  181,
  191,
  193,
  197,
  199,
  211,
  223,
  227,
  229,
  233,
  239,
  241,
  251,
  257,
  263,
  269,
  271,
  277,
  281,
  283,
  293,
  307,
  311,
  313,
];
