import 'dart:io';
import 'package:test/test.dart';
import 'package:msgpack_dart/msgpack_dart.dart' as mp;

import 'package:onejoker/card.dart';
import 'package:onejoker/master_deck.dart';
import 'package:onejoker/stack.dart';

void main() {
  group('text i/o', () {
    dynamic skip = false;
    var file = File('../data/bin/hands_text.msgpack');
    if (! file.existsSync()) {
      skip = "Test files not built, skipping...";
    }

    test('hands text data file', () async {
      var bytes = await file.readAsBytes();
      var data = mp.deserialize(bytes);

      var hands = data['hands'];
      for (int i = 0; i < data['count']; i += 1) {
        var deck = MasterDeck.byName(data['deck_names'][hands[i][0] - 1]);
        var hand = CardStack.fromText(hands[i][1]);
        if (deck.lowAces) {
          hand.lowAceFix();
        }
        expect(hand.length, hands[i][2]);
        for (int j = 0; j < hand.length; j += 1) {
          expect(true, deck.has(hand.cardAt(j)!));
        }
        expect(hands[i][1], hand.toString());
        expect(hands[i][3], hand.quickHash());
      }
    }, skip: skip);
  });

  group('methods', () {
    test('constructors', () {
      var hand = CardStack();
      expect(hand.length, 0);
      expect(hand.isEmpty, true);

      hand = CardStack.fromList([Card.FourOfSpades, Card.Joker]);
      expect(hand.length, 2);
      expect(hand.cardAt(0), Card.FourOfSpades);
      expect(hand.cardAt(1), Card.Joker);

      expect(hand.contains(Card.FourOfSpades), true);
      expect(hand.contains(Card.EightOfClubs), false);
      hand.clear();
      expect(hand.isEmpty, true);
      expect(hand.contains(Card.FourOfSpades), false);

      hand = CardStack.fromText("4sJc9d");
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

      hand = CardStack.fromList([
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
      var hand = CardStack();
      hand.push(Card.FourOfSpades);
      expect(hand.length, 1);
      expect(hand.cardAt(0), Card.FourOfSpades);
      hand.push(Card.Joker);
      expect(hand.length, 2);
      expect(hand.cardAt(0), Card.Joker);
      expect(hand.cardAt(1), Card.FourOfSpades);
      expect(hand.toString(), "Jk4s");
      expect(hand.pop(), Card.Joker);
      expect(hand.length, 1);
      expect(hand.cardAt(0), Card.FourOfSpades);
      expect(hand.pop(), Card.FourOfSpades);
      expect(hand.length, 0);
      expect(hand.isEmpty, true);

      hand.push(Card.fromText("9d")!);
      hand.push(Card.fromText("Qs")!);
      expect(hand.toString(), "Qs9d");

      hand = CardStack.fromList([Card.KingOfClubs, Card.AceOfClubs]);
      hand.pushN([
        Card.TenOfClubs,
        Card.JackOfClubs,
        Card.QueenOfClubs,
      ]);
      expect(hand.toString(), "TcJcQcKcAc");
      var list = hand.popN(2)!;
      expect(list[0], Card.TenOfClubs);
      expect(list[1], Card.JackOfClubs);
      expect(hand.toString(), "QcKcAc");

      hand.pushN(cardsFromText("TcJc").toList());
      expect(hand.toString(), "TcJcQcKcAc");
    });

    test('insert and remove', () {
      var hand = CardStack.fromText("4sJc9d");
      hand.insertAt(1, Card.Joker);
      expect(hand.toString(), "4sJkJc9d");
      hand.insertAt(0, Card.TenOfDiamonds);
      expect(hand.toString(), "Td4sJkJc9d");
      hand.insertAt(4, Card.QueenOfDiamonds);
      expect(hand.toString(), "Td4sJkJcQd9d");
      hand.insertAtEnd(Card.AceOfClubs);
      expect(hand.toString(), "Td4sJkJcQd9dAc");
      hand.insertAt(7, Card.SixOfSpades);
      expect(hand.toString(), "Td4sJkJcQd9dAc6s");

      expect(hand.removeAt(0), Card.TenOfDiamonds);
      expect(hand.toString(), "4sJkJcQd9dAc6s");
      expect(hand.removeAt(2), Card.JackOfClubs);
      expect(hand.toString(), "4sJkQd9dAc6s");
      expect(hand.removeCard(Card.AceOfClubs), true);
      expect(hand.toString(), "4sJkQd9d6s");
      expect(hand.removeAtEnd(), Card.SixOfSpades);
      expect(hand.toString(), "4sJkQd9d");
      expect(hand.removeAt(3), Card.NineOfDiamonds);
      expect(hand.toString(), "4sJkQd");
    });

    test('shuffle and sort', () {
      var hand = CardStack.fromText("3h5h8dTh3c4h7sJkQs7d");
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

      /// Test the randomness of shuffle() by playing "find the ace".
      /// Make a deck of 20 cards, shuffle it a million times, and count the
      /// number of times the ace of spades falls in each position. Should be
      /// close to 50000 per bucket. Of course, being random, the test may
      /// fail rarely. This is a simple quick-and-dirty "is it broken", test.
      /// The PRNG itself I ran through a battery of tests called "dieharder"
      /// that checks for all kinds of statistical bias.

      List<int> counts = List.filled(20, 0);
      hand = CardStack.fromText("As2s3s4s5s6s7s8s9sTsAh2h3h4h5h6h7h8h9hTh");
      for (int i = 0; i < 1000000; i += 1) {
        hand.shuffle();
        for (int j = 0; j < 20; j += 1) {
          if (hand[j] == Card.AceOfSpades) {
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
  });
}
