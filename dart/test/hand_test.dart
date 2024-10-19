import 'dart:io';
import 'package:msgpack_dart/msgpack_dart.dart' as mp;
import 'package:test/test.dart';
import 'package:onejoker/onejoker.dart';

void main() {
  group('text i/o', () {
    dynamic skip = false;
    var file = File('../data/bin/hands_text.msgpack');
    if (!file.existsSync()) {
      skip = "Test files not built, skipping...";
    }

    test('hands text data file', () async {
      var bytes = await file.readAsBytes();
      var data = mp.deserialize(bytes);

      var hands = data['hands'];
      for (int i = 0; i < data['count']; i += 1) {
        var deck = MasterDeck.byName(data['deck_names'][hands[i][0] - 1]);
        var hand = OrphanHand.fromText(hands[i][1]);

        if (deck.lowAces) {
          hand.lowAceFix();
        }
        expect(hand.length, hands[i][2]);
        for (int j = 0; j < hand.length; j += 1) {
          expect(true, deck.has(hand.cardAt(j)!));
        }
        expect(hands[i][1], hand.toString());
        expect(hands[i][3], ojhFNV32(hand));
      }
    }, skip: skip);
  });

  group('methods', () {
    test('constructors', () {
      var hand = OrphanHand();
      expect(hand.length, 0);
      expect(hand.isEmpty, true);

      hand = OrphanHand.fromIterable([Card.FourOfSpades, Card.Joker]);
      expect(hand.length, 2);
      expect(hand.cardAt(0), Card.FourOfSpades);
      expect(hand.cardAt(1), Card.Joker);

      expect(hand.contains(Card.FourOfSpades), true);
      expect(hand.contains(Card.EightOfClubs), false);
      hand.clear();
      expect(hand.isEmpty, true);
      expect(hand.contains(Card.FourOfSpades), false);

      hand = OrphanHand.fromText("4sJc9d");
      expect(3, hand.length);
      expect(hand[0], Card.FourOfSpades);
      expect(hand[1], Card.JackOfClubs);
      expect(hand[2], Card.NineOfDiamonds);

      var hand2 = hand.clone();
      expect(3, hand2.length);
      expect(hand2[0], Card.FourOfSpades);
      expect(hand2[1], Card.JackOfClubs);
      expect(hand2[2], Card.NineOfDiamonds);

      List<Card> cards = hand.toList();
      expect(cards.length, 3);
      expect(cards[0], Card.FourOfSpades);
      expect(cards[1], Card.JackOfClubs);
      expect(cards[2], Card.NineOfDiamonds);

      hand = OrphanHand.fromIterable([
        Card.LowAceOfDiamonds,
        Card.SevenOfHearts,
        Card.AceOfHearts,
        Card.KingOfClubs
      ]);
      expect(hand.cardAt(0), Card.LowAceOfDiamonds);
      expect(hand.cardAt(1), Card.SevenOfHearts);
      expect(hand.cardAt(2), Card.AceOfHearts);
      expect(hand.cardAt(3), Card.KingOfClubs);
      hand.lowAceFix();
      expect(hand.cardAt(0), Card.LowAceOfDiamonds);
      expect(hand.cardAt(1), Card.SevenOfHearts);
      expect(hand.cardAt(2), Card.LowAceOfHearts);
      expect(hand.cardAt(3), Card.KingOfClubs);

      hand[0] = Card.QueenOfDiamonds;
      hand[2] = Card.FiveOfHearts;
      expect(hand.toString(), "Qd7h5hKc");
    });

    test('push and pop', () {
      var hand = OrphanHand.fromText("3dQc7s9h");
      expect(hand.length, 4);
      expect(hand.toString(), "3dQc7s9h");
      expect(hand.pop(), Card.NineOfHearts);
      hand.push(Card.SixOfClubs);
      hand.push(Card.FiveOfDiamonds);
      expect(hand.toString(), "3dQc7s6c5d");
      List<Card> out = hand.popN(3).toList();
      expect(out.length, 3);
      expect(out[0], Card.FiveOfDiamonds);
      expect(out[1], Card.SixOfClubs);
      expect(out[2], Card.SevenOfSpades);
      hand.pushN(3, cardsFromText("JkKh8h"));
      expect(hand.toString(), "3dQcJkKh8h");
      hand.popN(4).forEach((_) {}); // pop and drop
      expect(hand.toString(), "3d");
      hand.pushN(2, [Card.FourOfSpades, Card.DeuceOfClubs]);
      expect(hand.toString(), "3d4s2c");

      out = [Card.AceOfClubs, Card.AceOfDiamonds, Card.AceOfHearts];
      int i = 0;
      for (Card c in hand) {
        out[i] = c;
        i += 1;
      }
      expect(i, 3);
      for (Card c in hand.popN(3)) {
        i -= 1;
        expect(c, out[i]);
      }
      expect(i, 0);
    });

    test('insert and remove', () {
      var hand = OrphanHand.fromText("4sJc9d");
      hand.insertAt(1, Card.Joker);
      expect(hand.toString(), "4sJkJc9d");
      hand.insertAt(0, Card.TenOfDiamonds);
      expect(hand.toString(), "Td4sJkJc9d");
      hand.insertAt(4, Card.QueenOfDiamonds);
      expect(hand.toString(), "Td4sJkJcQd9d");
      hand.insertAt(6, Card.SixOfSpades);
      expect(hand.toString(), "Td4sJkJcQd9d6s");

      expect(hand.removeAt(0), Card.TenOfDiamonds);
      expect(hand.toString(), "4sJkJcQd9d6s");
      expect(hand.removeAt(2), Card.JackOfClubs);
      expect(hand.toString(), "4sJkQd9d6s");
      expect(hand.removeAt(3), Card.NineOfDiamonds);
      expect(hand.toString(), "4sJkQd6s");
    });

    test('shuffle and sort', () {
      var hand = OrphanHand.fromText("3h5h8dTh3c4h7sJkQs7d");
      hand.shuffle();
      expect(hand.length, 10);
      expect(hand.contains(Card.FiveOfHearts), true);
      expect(hand.contains(Card.TenOfHearts), true);
      expect(hand.contains(Card.SevenOfDiamonds), true);
      expect(hand.contains(Card.NineOfClubs), false);

      hand.sort();
      expect(hand.toString(), "QsTh8d7s7d5h4h3h3cJk");

      hand.removeCard(Card.SevenOfDiamonds);
      hand.shuffle();
      expect(hand.length, 9);
      expect(hand.contains(Card.TreyOfClubs), true);
      expect(hand.contains(Card.SevenOfSpades), true);
      expect(hand.contains(Card.Joker), true);
      expect(hand.contains(Card.SevenOfDiamonds), false);

      hand.sort();
      expect(hand.toString(), "QsTh8d7s5h4h3h3cJk");
    });
  });
}
