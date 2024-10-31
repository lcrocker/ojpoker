import 'dart:io';
import 'package:test/test.dart';
import 'package:characters/characters.dart';
import 'package:jsonc/jsonc.dart';
import 'package:onejoker/onejoker.dart';

void main() {
  group('text i/o', () {
    test('random hands file', () async {
      var file = File('../data/json/random_hands_100k.jsonc');
      expect(file.existsSync(), true);
      final text = await file.readAsString();
      final data = jsonc.decode(text);

      final max = ONEJOKER_FULL_TESTS ? data["hands"].length : 10000;
      for (var i = 0; i < max; i += 1) {
        int index = ONEJOKER_FULL_TESTS
            ? i
            : OjRandom.rangeUniform(data["hands"].length);
        var h = data["hands"][index];

        var d = Deck(data["decks"][h[0] - 1]);
        d.shuffle();
        var hand = d.newHand();
        bool ok = hand.drawHand(cardsFromText(h[1]));
        expect(ok, true);

        var s = "";
        for (var c in (h[1] as String).characters) {
          if (' ' == c || '\t' == c || '\n' == c || '\r' == c || '[' == c) {
            continue;
          }
          if (']' == c) {
            break;
          }
          if ('1' == c) {
            s += "A";
          } else {
            s += c;
          }
        }
        // print("${index} ${hand} ${s} ${h[2]}");
        expect(hand.toString(), s);
        expect(ojhFNV32(hand.cards), h[2]);

        int len = hand.length;
        Hand clone = hand.clone();
        expect(clone.length, len);
        expect(clone.equals(hand), true);

        clone.shuffle();
        expect(clone.length, len);
        expect(clone.isEquivalentTo(hand), true);
        expect(clone.contains(hand[0]), true);
        expect(clone.contains(hand[len - 1]), true);

        hand.sort();
        expect(hand.length, len);
        for (var j = 1; j < len; j += 1) {
          expect(hand[j] <= hand[j - 1], true);
        }
        clone.sort();
        expect(hand.equals(clone), true);

        clone[0] = Card.None; // should reject
        expect(hand.equals(clone), true);
        clone.cards[0] = Card.None; // go in the back door
        expect(hand.equals(clone), false);
        clone[0] = hand.cardAt(0)!;
        expect(hand.equals(clone), true);

        if (len > 5) {
          clone.discard([0, 2, 4]);
          expect(clone.length, len - 3);
          expect(clone[0] == hand[1], true);
          expect(clone[1] == hand[3], true);
          expect(clone[2] == hand[5], true);

          clone.clear();
          clone.pushN(hand.toList());
          expect(clone.equals(hand), true);

          clone.discard([2, 4, 0]);
          expect(clone.length, len - 3);
          expect(clone[0] == hand[1], true);
          expect(clone[1] == hand[3], true);
          expect(clone[2] == hand[5], true);
        }
      }
    });
  });

  group('methods', () {
    test('constructors', () {
      var hand = Hand();
      expect(hand.length, 0);
      expect(hand.isEmpty, true);

      hand = Hand.fromIter([Card.FourOfSpades, Card.Joker]);
      expect(hand.length, 2);
      expect(hand.cardAt(0), Card.FourOfSpades);
      expect(hand.cardAt(1), Card.Joker);

      expect(hand.contains(Card.FourOfSpades), true);
      expect(hand.contains(Card.EightOfClubs), false);
      hand.clear();
      expect(hand.isEmpty, true);
      expect(hand.contains(Card.FourOfSpades), false);

      hand = Hand.fromText("4sJc9d");
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

      hand = Hand.fromIter([
        Card.LowAceOfDiamonds,
        Card.SevenOfHearts,
        Card.AceOfHearts,
        Card.KingOfClubs
      ]);
      expect(hand.cardAt(0), Card.AceOfDiamonds);
      expect(hand.cardAt(1), Card.SevenOfHearts);
      expect(hand.cardAt(2), Card.AceOfHearts);
      expect(hand.cardAt(3), Card.KingOfClubs);

      hand[0] = Card.QueenOfDiamonds;
      hand[2] = Card.FiveOfHearts;
      expect(hand.toString(), "Qd7h5hKc");
    });

    test('push and pop', () {
      var hand = Hand("onejoker");
      hand.pushN(cardsFromText("3dQc7s9h"));
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
      hand.pushN(cardsFromText("JkKh8h"));
      expect(hand.toString(), "3dQcJkKh8h");
      hand.popN(4).forEach((_) {}); // pop and drop
      expect(hand.toString(), "3d");
      hand.pushN([Card.FourOfSpades, Card.DeuceOfClubs]);
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
      var hand = Hand("onejoker");
      hand.pushN(cardsFromText("4sJc9d"));
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
  });
}
