//
/// Simple implementation of xoshiro128++
/// {@category utilities}
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
/// {@category utilities}
void ojShuffle<T>(List<T> items) {
  if (items.length < 2) {
    return;
  }
  for (int i = items.length - 1; i > 0; i -= 1) {
    int j = OjRandom.rangeUniform(i + 1);
    if (i != j) {
      T temp = items[i];
      items[i] = items[j];
      items[j] = temp;
    }
  }
}

void _heapify<T extends Comparable>(List<T> a, int n, int i) {
  int loopGuard = 200; // Should be about log(n) passes
  while (loopGuard > 0) {
    loopGuard -= 1;

    int min = i;
    int left = 2 * i + 1;
    int right = 2 * i + 2;

    if (left < n && a[left].compareTo(a[min]) < 0) {
      min = left;
    }
    if (right < n && a[right].compareTo(a[min]) < 0) {
      min = right;
    }
    if (min == i) break;

    T temp = a[i];
    a[i] = a[min];
    a[min] = temp;
    i = min;
  }
}

/// Heapsort somewhat optimized for small sets, like a hand of cards.
/// {@category utilities}
void ojSort<T extends Comparable>(List<T> a) {
  switch (a.length) {
    case 5:
      if (a[0].compareTo(a[1]) < 0) {
        T temp = a[0];
        a[0] = a[1];
        a[1] = temp;
      }
      if (a[3].compareTo(a[4]) < 0) {
        T temp = a[3];
        a[3] = a[4];
        a[4] = temp;
      }
      if (a[2].compareTo(a[4]) < 0) {
        T temp = a[2];
        a[2] = a[4];
        a[4] = temp;
      }
      if (a[2].compareTo(a[3]) < 0) {
        T temp = a[2];
        a[2] = a[3];
        a[3] = temp;
      }
      if (a[0].compareTo(a[3]) < 0) {
        T temp = a[0];
        a[0] = a[3];
        a[3] = temp;
      }
      if (a[0].compareTo(a[2]) < 0) {
        T temp = a[0];
        a[0] = a[2];
        a[2] = temp;
      }
      if (a[1].compareTo(a[4]) < 0) {
        T temp = a[1];
        a[1] = a[4];
        a[4] = temp;
      }
      if (a[1].compareTo(a[3]) < 0) {
        T temp = a[1];
        a[1] = a[3];
        a[3] = temp;
      }
      if (a[1].compareTo(a[2]) < 0) {
        T temp = a[1];
        a[1] = a[2];
        a[2] = temp;
      }
      break;
    case 4:
      if (a[0].compareTo(a[1]) < 0) {
        T temp = a[0];
        a[0] = a[1];
        a[1] = temp;
      }
      if (a[2].compareTo(a[3]) < 0) {
        T temp = a[2];
        a[2] = a[3];
        a[3] = temp;
      }
      if (a[0].compareTo(a[2]) < 0) {
        T temp = a[0];
        a[0] = a[2];
        a[2] = temp;
      }
      if (a[1].compareTo(a[3]) < 0) {
        T temp = a[1];
        a[1] = a[3];
        a[3] = temp;
      }
      if (a[1].compareTo(a[2]) < 0) {
        T temp = a[1];
        a[1] = a[2];
        a[2] = temp;
      }
      break;
    case 3:
      if (a[1].compareTo(a[2]) < 0) {
        T temp = a[1];
        a[1] = a[2];
        a[2] = temp;
      }
      if (a[0].compareTo(a[2]) < 0) {
        T temp = a[0];
        a[0] = a[2];
        a[2] = temp;
      }
      if (a[0].compareTo(a[1]) < 0) {
        T temp = a[0];
        a[0] = a[1];
        a[1] = temp;
      }
      break;
    case 2:
      if (a[0].compareTo(a[1]) < 0) {
        T temp = a[0];
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
        T temp = a[0];
        a[0] = a[i];
        a[i] = temp;
        _heapify(a, i, 0);
      }
      break;
  }
}

/// Generate all k-combinations of n cards
/// {@category utilities}
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
