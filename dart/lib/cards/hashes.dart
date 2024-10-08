import 'package:onejoker/cards/card.dart';
import 'package:onejoker/cards/utils.dart';

class CardHashInterface {
  // 32-bit standard hash
  static int u32(Iterable<Card> cards) {
    assert(false);
    return 0;
  }

  // 32-bit collison-free
  static int u32c(Iterable<Card> cards) {
    assert(false);
    return 0;
  }

  // 32-bit collision-free order-independent
  static int u32co(Iterable<Card> cards) {
    assert(false);
    return 0;
  }

  // 32-bit collision-free order-independent suit-independent
  static int u32cos(Iterable<Card> cards) {
    assert(false);
    return 0;
  }

  // 64-bit standard hash
  static int u64(Iterable<Card> cards) {
    assert(false);
    return 0;
  }

  // 64-bit collision-free
  static int u64c(Iterable<Card> cards) {
    assert(false);
    return 0;
  }

  // 64-bit collision-free order-independent
  static int u64co(Iterable<Card> cards) {
    assert(false);
    return 0;
  }

  // 64-bit collision-free order-independent suit-independent
  static int u64cos(Iterable<Card> cards) {
    assert(false);
    return 0;
  }
}

class FNVHash implements CardHashInterface {
  static int u32(Iterable<Card> cards) {
    int h = 0x811c9dc5;
    for (Card c in cards) {
      h ^= c.index;
      h *= 0x01000193;
    }
    return h & 0xFFFFFFFF;
  }

  static int u64(Iterable<Card> cards) {
    int h = 0xcbf29ce484222325;
    for (Card c in cards) {
      h ^= c.index;
      h *= 0x100000001b3;
    }
    return h;
  }
}

class PositionalHash implements CardHashInterface {
  static int u32c(Iterable<Card> cards) {
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

  static int u32co(Iterable<Card> cards) {
    List<Card> sorted = cards.toList();
    ojSort(sorted);
    return PositionalHash.u32c(sorted);
  }

  static int u32cos(Iterable<Card> cards) {
    List<Card> sorted = cards.toList();
    ojSort(sorted);

    int max = 8;
    int h = 0;

    for (Card c in sorted) {
      max -= 1;
      assert(max >= 0);

      h <<= 4;
      h += (0x0F & (c.index >> 2));
    }
    return h & 0xFFFFFFFF;
  }

  static int u64c(Iterable<Card> cards) {
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

  static int u64co(Iterable<Card> cards) {
    List<Card> sorted = cards.toList();
    ojSort(sorted);
    return PositionalHash.u64c(sorted);
  }

  static int u64cos(Iterable<Card> cards) {
    List<Card> sorted = cards.toList();
    ojSort(sorted);

    int max = 16;
    int h = 0;

    for (Card c in sorted) {
      max -= 1;
      assert(max >= 0);

      h <<= 4;
      h += (0x0F & (c.index >> 2));
    }
    return h;
  }
}

class BitfieldHash implements CardHashInterface {
  static int u64co(Iterable<Card> cards) {
    int h = 0;

    for (Card c in cards) {
      assert(0 == (h & (1 << (0x3F & c.index))));
      h |= (1 << (0x3F & c.index));
    }
    return h;
  }
}

class PrimeHash implements CardHashInterface {
  static int u32cos(Iterable<Card> cards) {
    int max = 5;
    int h = 1;

    for (Card c in cards) {
      max -= 1;
      assert(max >= 0);

      h *= _primes[0x0F & (c.index >> 2)];
    }
    return h & 0xFFFFFFFF;
  }

  static int u64co(Iterable<Card> cards) {
    int max = 7;
    int h = 1;

    for (Card c in cards) {
      max -= 1;
      assert(max >= 0);

      h *= _primes[0x3F & c.index];
    }
    return h;
  }

  static int u64cos(Iterable<Card> cards) {
    int max = 10;
    int h = 1;

    for (Card c in cards) {
      max -= 1;
      assert(max >= 0);

      h *= _primes[0x0F & (c.index >> 2)];
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
