import 'package:test/test.dart';
import 'package:onejoker/onejoker.dart';

void main() {
  group('deck_and_hand', () {
    const List<Card> englishCards = [
      Card.TreyOfDiamonds,
      Card.FourOfClubs,
      Card.EightOfSpades,
      Card.TenOfClubs,
      Card.QueenOfDiamonds,
      Card.AceOfSpades
    ];
    const List<Card> spanishCards = [
      Card.DeuceOfDiamonds,
      Card.FiveOfClubs,
      Card.SevenOfHearts,
      Card.JackOfClubs,
      Card.KnightOfDiamonds,
      Card.LowAceOfSpades
    ];

    test('english deck', () async {
      var d1 = Deck("english");
      d1.shuffle();
      var (v, c) = d1.validCard(Card.LowAceOfSpades);
      expect(v, true);
      expect(c, Card.AceOfSpades);
      expect(d1.length, 52);

      for (var c in englishCards) {
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
      expect(d1.size, 52);
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

      d1.refill();
      expect(d1.length, 52);
      d1.clearAll();
      expect(h1.isEmpty, true);
      expect(h2.isEmpty, true);

      for (var c in englishCards) {
        expect(d1.contains(c), true);
      }
    });

    test('spanish deck', () async {
      var d1 = Deck("spanish");
      d1.shuffle();
      var (v, c) = d1.validCard(Card.AceOfSpades);
      expect(v, true);
      expect(c, Card.LowAceOfSpades);
      expect(d1.length, 40);
      expect(d1.size, 40);

      for (var c in spanishCards) {
        expect(d1.contains(c), true);
      }
      var h1 = d1.newHand();
      expect(h1.isEmpty, true);
      expect(d1.length, 40);
      d1.dealTo(h1);
      expect(h1.length, 1);
      expect(d1.length, 39);

      expect(d1.contains(h1[0]), false);
    });

    const pHand = [
      Card.EightOfHearts,
      Card.FiveOfSpades,
      Card.QueenOfDiamonds,
      Card.JackOfClubs,
      Card.FiveOfClubs,
    ];

    test('hand', () async {
      var d1 = Deck("onejoker");
      d1.shuffle();
      var h1 = d1.newHand();
      var h2 = d1.newHand();

      expect(d1.remaining, 53);
      h1.draw(5);
      expect(d1.remaining, 48);

      for (var c in h1) {
        expect(d1.contains(c), false);
      }
      h2.draw(5);
      for (var c in h2) {
        expect(d1.contains(c), false);
        expect(h1.contains(c), false);
      }
      d1
        ..clearAll()
        ..refill()
        ..shuffle();

      expect(h1.isEmpty, true);
      expect(d1.contains(Card.DeuceOfHearts), true);
      h1.drawCard(Card.DeuceOfHearts);
      expect(h1[0], Card.DeuceOfHearts);
      expect(d1.contains(Card.DeuceOfHearts), false);

      h2.drawHand(pHand);
      expect(h2.length, 5);
      expect(d1.remaining, 47);
      for (var c in pHand) {
        expect(h2.contains(c), true);
        expect(d1.contains(c), false);
      }
      h2.sort();
      expect(h2.toString(), "QdJc8h5s5c");

      h1.clear();
      h1.pushN(cardsFromText("8h4s4h2c5s"));
      h2.clear();
      h2.pushN(cardsFromText("4h8h5s4s2c"));

      expect(h1.length, 5);
      expect(h2.length, 5);
      expect(h1 == h2, false);
      expect(h1.isEquivalentTo(h2), true);

      var d2 = Deck("pinochle");
      var h3 = d2.newHand();
      var h4 = d2.newHand();
      var h5 = d2.newHand();

      h3.pushN(cardsFromText("TdTd9dKsJc"));
      h4.pushN(cardsFromText("TdKs9dJcTd"));
      h5.pushN(cardsFromText("Td9d9dKsJc"));

      expect(h3.isEquivalentTo(h4), true);
      expect(h3.isEquivalentTo(h5), false);
    });
  });
}
