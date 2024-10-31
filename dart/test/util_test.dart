import 'package:test/test.dart';
import 'package:onejoker/onejoker.dart';

void main() {
  group('Utils', () {
    test('random', () {
      List<int> array = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19
      ];
      List<int> counts = List.filled(20, 0);

      for (int i = 0; i < 1000000; i += 1) {
        ojShuffle(array);
        for (int j = 0; j < 20; j += 1) {
          if (array[j] == 0) {
            counts[j] += 1;
            break;
          }
        }
      }
      for (int i = 0; i < 20; i += 1) {
        expect(counts[i], greaterThan(49000));
        expect(counts[i], lessThan(51000));
      }
    });

    test('binomial', () {
      expect(1, ojBinomial(0, 0));
      expect(0, ojBinomial(0, 1));
      expect(1, ojBinomial(1, 0));
      expect(1, ojBinomial(63, 63));
      expect(7770, ojBinomial(37, 34));
      expect(1107568, ojBinomial(33, 27));
      expect(15471286560, ojBinomial(38, 23));
      expect(7898654920, ojBinomial(41, 12));
      expect(65780, ojBinomial(26, 21));
      expect(780, ojBinomial(40, 38));
      expect(99884400, ojBinomial(50, 7));
      expect(137846528820, ojBinomial(40, 20));
      expect(12176310231149295, ojBinomial(61, 40));
      expect(735471, ojBinomial(24, 8));
      expect(1, ojBinomial(23, 0));
      expect(46376, ojBinomial(34, 4));
      expect(86493225, ojBinomial(30, 12));
      expect(54264, ojBinomial(21, 15));
      expect(286, ojBinomial(13, 10));
      expect(20160075, ojBinomial(31, 22));
      expect(31, ojBinomial(31, 1));
      expect(118755, ojBinomial(29, 24));
      expect(1, ojBinomial(13, 13));
      expect(36052387482172425, ojBinomial(60, 24));
      expect(77520, ojBinomial(20, 13));
      expect(10, ojBinomial(5, 3));
      expect(36576848168, ojBinomial(43, 30));
      expect(346104, ojBinomial(24, 17));
      expect(40475358, ojBinomial(58, 52));
      expect(30067266499541040, ojBinomial(58, 29));
      expect(37711260990, ojBinomial(39, 16));
      expect(14031391033119152, ojBinomial(57, 27));
      expect(736281, ojBinomial(31, 6));
      expect(4154246671960, ojBinomial(46, 19));
      expect(0, ojBinomial(16, 17));
      expect(314457495, ojBinomial(47, 39));
      expect(563921995, ojBinomial(43, 9));
      expect(91390, ojBinomial(40, 4));
      expect(46, ojBinomial(46, 45));
      expect(62891499, ojBinomial(47, 40));
      expect(129024480, ojBinomial(32, 21));
      expect(8, ojBinomial(8, 7));
      expect(1677106640, ojBinomial(48, 9));
      expect(646646, ojBinomial(22, 10));
      expect(1001, ojBinomial(14, 4));
      expect(4537567650, ojBinomial(35, 18));
      expect(184756, ojBinomial(20, 10));
      expect(575757, ojBinomial(39, 34));
      expect(1476337800, ojBinomial(35, 22));
      expect(10, ojBinomial(10, 9));

      for (var i = 0; i < 500; i += 1) {
        var n = OjRandom.rangeUniform(64);
        var k = OjRandom.rangeUniform(64);

        if (k > n) {
          expect(0, ojBinomial(n, k));
        } else if (k == n) {
          expect(1, ojBinomial(n, k));
        } else if (k == 1) {
          expect(n, ojBinomial(n, k));
        } else {
          expect(ojBinomial(n, k), ojBinomial(n, n - k));
        }
        if (n > 0 && k > 0) {
          expect(ojBinomial(n - 1, k) + ojBinomial(n - 1, k - 1),
              ojBinomial(n, k));
        }
      }
    });
  });
}
