import 'package:test/test.dart';
import 'package:onejoker/onejoker.dart';

void main() {
  group('text i/o', () {
    test('hands text data file', () async {
      var d1 = Deck("poker");
      d1.shuffle();

      expect(d1.length, 52);
      for (var v in [9, 11, 15, 22, 26, 31, 39, 43, 49, 52, 55, 62]) {
        expect(d1.contains(Card.fromInt(v)!), true);
      }
      var h1 = d1.newHand();
      expect(h1.isEmpty, true);
      expect(d1.length, 52);

      var h2 = d1.newHand();
      expect(h2.length, 0);
      expect(d1.length, 52);

      d1.dealAll(5);
      expect(d1.length, 42);
      expect(h1.length, 5);
      expect(h2.length, 5);

      for (var c in h1) {
        expect(d1.contains(c), false);
        expect(h2.contains(c), false);
      }
    });
  });
}
