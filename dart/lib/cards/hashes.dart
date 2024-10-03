import 'package:onejoker/cards/card.dart';
import 'package:onejoker/cards/utils.dart';

/// This file contains hash functions for sets of cards with
/// varying abilities.

/// FNV hash. Can take any number of cards of any values, but
/// will produce collisions. Useful mostly for unit tests.
int cardHashFNV_32(Iterable<Card> cards) {
  int h = 0x811c9dc5;
  for (Card c in cards) {
    h ^= c.index;
    h *= 0x01000193;
  }
  return h & 0xFFFFFFFF;
}

int _base64(Iterable<int> ordinals) {
  int max = 10;
  int h = 0;
  for (int d in ordinals) {
    assert(max > 0);
    max -= 1;

    h <<= 6;
    h += 0x3F & d;
  }
  return h;
}

int _base16(Iterable<int> ordinals) {
  int max = 16;
  int h = 0;
  for (int d in ordinals) {
    assert(max > 0);
    max -= 1;

    h <<= 4;
    h += (0x0F & (d >> 2));
  }
  return h;
}

int cardHashBase64_64(Iterable<Card> cards) {
  return _base64(cards.map((c) => c.index));
}

int cardHashBase64U_64(Iterable<Card> cards) {
  List<Card> s = cards.toList();
  ojSort(s);
  return _base64(s.map((c) => c.index));
}

int cardHashBase16RU_64(Iterable<Card> cards) {
  List<Card> s = cards.toList();
  ojSort(s);
  return _base16(s.map((c) => c.index));
}

int cardHashBitFieldU_64(Iterable<Card> cards) {
  int h = 0;
  for (Card c in cards) {
    h |= (1 << c.index);
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

int cardHashPrimeU_64(Iterable<Card> cards) {
  int max = 7;
  int h = 1;

  for (Card c in cards) {
    assert(max > 0);
    max -= 1;

    h *= _primes[c.index];
  }
  return h;
}

int cardHashPrimeRU_64(Iterable<Card> cards) {
  int max = 10;
  int h = 1;

  for (Card c in cards) {
    assert(max > 0);
    max -= 1;

    h *= _primes[c.index >> 2];
  }
  return h;
}
