import 'package:onejoker/cards/card.dart';

/// This file contains utility functions that are used in multiple places in
/// the library. Things like random number generation, shuffling, and sorting
/// are present in the language already, but these versions are specifically
/// tuned and optimized for playing cards and games.
class OjRandom {
  static final List<int> _s = [0, 0, 0, 0];
  static bool _seeded = false;

  /// Generate a random 32-bit integer. While Dart uses 64-bit integers,
  /// it can be compiled into JavaScript, which doesn't play nice with
  /// 64-bit ints, so we stick to 32 here.
  static int next32() {
    if (!_seeded) {
      _seeded = true;
      _s[0] = DateTime.now().millisecondsSinceEpoch;
      _s[1] = _s[0] + 1;
      _s[2] = _s[0] + 2;
      _s[3] = _s[0] + 3;
    }

    int r = (_s[0] + _s[3]) & 0xFFFFFFFF;
    final int result = ((r << 7) | (r >> 25)) + _s[0];
    final int t = _s[1] << 9;

    _s[2] ^= _s[0];
    _s[3] ^= _s[1];
    _s[1] ^= _s[2];
    _s[0] ^= _s[3];

    _s[2] ^= t;
    _s[3] = (_s[3] << 11) | ((_s[3] & 0xFFFFFFFF) >> 21);

    return result & 0xFFFFFFFF;
  }

  /// Return a random integer uniformly distributed in the range [0, limit)
  /// with no division, using rejection sampling. The mask `m` is used to
  /// minimize rejections, which will be at worst 50%.
  static int rangeUniform(int limit) {
    assert(limit > 0);
    int m = limit - 1;
    for (int i in [1, 2, 4, 8, 16]) {
      m |= m >> i;
    }

    while (true) {
      int r = next32() & m;
      if (r < limit) {
        return r;
      }
    }
  }
}

/// Standard Fisher-Yates shuffle using our own PRNG.
void ojShuffle(List<Card> cards) {
  if (cards.length < 2) {
    return;
  }
  for (int i = cards.length - 1; i > 0; i -= 1) {
    int j = OjRandom.rangeUniform(i + 1);
    if (i != j) {
      Card temp = cards[i];
      cards[i] = cards[j];
      cards[j] = temp;
    }
  }
}

void _heapify(List<Card> a, int n, int i) {
  int loopGuard = 200; // Should be about log(n) passes
  while (loopGuard > 0) {
    loopGuard -= 1;

    int max = i;
    int left = 2 * i + 1;
    int right = 2 * i + 2;

    if (left < n && a[left] > a[max]) {
      max = left;
    }
    if (right < n && a[right] > a[max]) {
      max = right;
    }
    if (max == i) break;

    Card temp = a[i];
    a[i] = a[max];
    a[max] = temp;
    i = max;
  }
}

/// Heapsort somewhat optimized for small sets, like a hand of cards.
void ojSort(List<Card> a) {
  switch (a.length) {
    case 5:
      if (a[0] > a[1]) {
        Card temp = a[0];
        a[0] = a[1];
        a[1] = temp;
      }
      if (a[3] > a[4]) {
        Card temp = a[3];
        a[3] = a[4];
        a[4] = temp;
      }
      if (a[2] > a[4]) {
        Card temp = a[2];
        a[2] = a[4];
        a[4] = temp;
      }
      if (a[2] > a[3]) {
        Card temp = a[2];
        a[2] = a[3];
        a[3] = temp;
      }
      if (a[0] > a[3]) {
        Card temp = a[0];
        a[0] = a[3];
        a[3] = temp;
      }
      if (a[0] > a[2]) {
        Card temp = a[0];
        a[0] = a[2];
        a[2] = temp;
      }
      if (a[1] > a[4]) {
        Card temp = a[1];
        a[1] = a[4];
        a[4] = temp;
      }
      if (a[1] > a[3]) {
        Card temp = a[1];
        a[1] = a[3];
        a[3] = temp;
      }
      if (a[1] > a[2]) {
        Card temp = a[1];
        a[1] = a[2];
        a[2] = temp;
      }
      break;
    case 4:
      if (a[0] > a[1]) {
        Card temp = a[0];
        a[0] = a[1];
        a[1] = temp;
      }
      if (a[2] > a[3]) {
        Card temp = a[2];
        a[2] = a[3];
        a[3] = temp;
      }
      if (a[0] > a[2]) {
        Card temp = a[0];
        a[0] = a[2];
        a[2] = temp;
      }
      if (a[1] > a[3]) {
        Card temp = a[1];
        a[1] = a[3];
        a[3] = temp;
      }
      if (a[1] > a[2]) {
        Card temp = a[1];
        a[1] = a[2];
        a[2] = temp;
      }
      break;
    case 3:
      if (a[1] > a[2]) {
        Card temp = a[1];
        a[1] = a[2];
        a[2] = temp;
      }
      if (a[0] > a[2]) {
        Card temp = a[0];
        a[0] = a[2];
        a[2] = temp;
      }
      if (a[0] > a[1]) {
        Card temp = a[0];
        a[0] = a[1];
        a[1] = temp;
      }
      break;
    case 2:
      if (a[0] > a[1]) {
        Card temp = a[0];
        a[0] = a[1];
        a[1] = temp;
      }
      break;
    case 1:
    case 0:
      break;
    default:
      for (int i = a.length ~/ 2; i >= 0; i -= 1) {
        _heapify(a, a.length, i);
      }
      for (int i = a.length - 1; i >= 1; i -= 1) {
        Card temp = a[0];
        a[0] = a[i];
        a[i] = temp;
        _heapify(a, i, 0);
      }
      break;
  }
}

bool ojNextCombination(List<int> a, int n) {
  int k = a.length;

  for (int i = k - 1; i >= 0; i -= 1) {
    if (a[i] < n - k + i + 1) {
      a[i] += 1;
      for (int j = i + 1; j < k; j += 1) {
        a[j] = a[j - 1] + 1;
      }
      return true;
    }
  }
  return false;
}
