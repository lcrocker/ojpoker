import 'package:test/test.dart';
import 'package:onejoker/onejoker.dart';

void main() {
  group('text i/o', () {
    test('deck', () async {
      var d1 = Deck("poker");
      d1.shuffle();

      expect(d1.length, 52);
      for (var c in [
        Card.DeuceOfDiamonds,
        Card.FourOfClubs,
        Card.SevenOfHearts,
        Card.EightOfSpades,
        Card.TenOfClubs,
        Card.JackOfClubs,
        Card.QueenOfDiamonds,
        Card.AceOfSpades
      ]) {
        expect(d1.contains(c), true);
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

      h1.removeAt(2);
      expect(h1.length, 4);
      d1.dealTo(h1);
      expect(h1.length, 5);
      expect(d1.length, 41);

      print(d1);
      print(h1);
      print(h2);
    });
  });
}
