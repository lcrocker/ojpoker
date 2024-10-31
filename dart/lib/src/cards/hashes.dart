//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Hashes) | Various hash functions for cards
//!
//! The common FNV hashes are often used for implementing hash tables and
//! doing quick checksumming for tests. They are not collision-free.
//!
//! Positional hashes treat cards (or ranks) as digits of a base-64
//! (or base-16) number. They therefore order-dependent and limited in size,
//! but inherently collision-free and useful for ranking hands.
//!
//! Bitfield hashes represent each card as a bit in a 64-bit integer.
//! This is inherently collision-free and order-independent, and very fast,
//! but can't handle duplicate cards and produces huge numbers. Also can't
//! be compiled to JavaScript.
//!
//! Prime hashes based on the product of prime numbers are inherently
//! collision-free, order-independent, handle duplicates, and produce
//! smaller numbers, but can only handle very small sets.
//!
//! The "mp" functions convert a bitfield to a minimal perfect hash,
//! and are very specific to number of cards and type of deck.

import 'package:onejoker/src/utilities.dart';

import 'cards.dart';

/// 32-bit FNV-1a
/// {@category hashes}
int ojhFNV32(Iterable<Card> cards) {
  int h = 0x811c9dc5;
  for (Card c in cards) {
    h ^= c.index;
    h *= 0x01000193;
  }
  return h & 0xFFFFFFFF;
}

/// 64-bit FNV-1a
/// {@category hashes}
int ojhFNV64(Iterable<Card> cards) {
  int h = 0xcbf29ce484222325;
  for (Card c in cards) {
    h ^= c.index;
    h *= 0x100000001b3;
  }
  return h;
}

/// 32-bit Positional hash
/// {@category hashes}
int ojhPositional32c(Iterable<Card> cards) {
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

/// 32-bit Positional suit-independent
/// {@category hashes}
int ojhPositional32cs(Iterable<Card> cards) {
  int max = 8;
  int h = 0;

  for (Card c in cards) {
    max -= 1;
    assert(max >= 0);

    h <<= 4;
    h += (0x0F & c.rank.index);
  }
  return h & 0xFFFFFFFF;
}

/// 32-bit Positional ranks only
/// {@category hashes}
int ojhPositional32cr(Iterable<Rank> ranks) {
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

/// 64-bit Positional hash
/// {@category hashes}
int ojhPositional64c(Iterable<Card> cards) {
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

/// 64-bit Positional hash
/// {@category hashes}
int ojhPositional64cs(Iterable<Card> cards) {
  int max = 16;
  int h = 0;

  for (Card c in cards) {
    max -= 1;
    assert(max >= 0);

    h <<= 4;
    h += (0x0F & c.rank.index);
  }
  return h;
}

/// 64-bit Positional ranks only
/// {@category hashes}
int ojhPositional64cr(Iterable<Rank> ranks) {
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

/// 64-bit Bitfield hash
/// {@category hashes}
int ojhBitfield64co(Iterable<Card> cards) {
  int h = 0;

  for (Card c in cards) {
    assert(0 == (h & (1 << (0x3F & c.index))));
    h |= (1 << (0x3F & c.index));
  }
  return h;
}

/// Convert bitfiled into minimal perfect hash
/// {@category hashes}
int ojhMp5English(int f) {
  // make ranks contiguous
  int b = f >> 8;
  b = (b & 0x000000FFFFFFFFFF) | ((b & 0x00FFF00000000000) >> 8);

  int h = ojBinomial(52, 5);
  int mask = 0x0008000000000000;
  int m = 1;

  for (int j = 0; j < 52; j += 1) {
    if (0 != (b & mask)) {
      h -= ojBinomial(j, m);
      m += 1;
      if (m > 5) break;
    }
    mask >>= 1;
  }
  return h;
}

/// 32-bit Prime hash
/// {@category hashes}
int ojhPrime32cor(Iterable<Rank> ranks) {
  int max = 5;
  int h = 1;

  for (Rank r in ranks) {
    max -= 1;
    assert(max >= 0);

    h *= _primes[0x0F & r.index];
  }
  return h & 0xFFFFFFFF;
}

/// 64-bit Prime hash
/// {@category hashes}
int ojhPrime64co(Iterable<Card> cards) {
  int max = 7;
  int h = 1;

  for (Card c in cards) {
    max -= 1;
    assert(max >= 0);

    h *= _primes[0x3F & c.index];
  }
  return h;
}

/// 64-bit Prime ranks hash
/// {@category hashes}
int ojhPrime64cor(Iterable<Rank> ranks) {
  int max = 10;
  int h = 1;

  for (Rank r in ranks) {
    max -= 1;
    assert(max >= 0);

    h *= _primes[0x0F & r.index];
  }
  return h;
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
