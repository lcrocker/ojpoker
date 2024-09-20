import 'package:test/test.dart';
import 'package:onejoker/card.dart';
import 'package:onejoker/masterdeck.dart';

import 'dart:io';
import 'package:msgpack_dart/msgpack_dart.dart' as mp;

void main() {
  group('Card', () {
    test('properties', () {
      for (var i = 1; i < 64; i += 1) {
        var c = Card.fromInt(i);

        if ({1, 2, 3}.contains(c.index)) {
          expect(true, c.isJoker);
          expect(Rank.None, c.rank);
          expect(Suit.None, c.suit);
        } else {
          expect(false, c.isJoker);
        }

        if ({4, 5, 6, 7, 56, 57, 58, 59}.contains(c.index)) {
          expect(true, c.isAce);
          expect(true, Card.lowAceFix(c).index < 8);
          expect(true, Card.highAceFix(c).index > 55);

          if (c.index < 8) {
            expect(Rank.LowAce, c.rank);
            expect(Rank.Ace, c.highRank);
          } else {
            expect(Rank.Ace, c.rank);
          }
        } else {
          expect(false, c.isAce);
          expect(c, Card.lowAceFix(c));
          expect(c, Card.highAceFix(c));
        }

        if ({
          3,
          5,
          6,
          9,
          10,
          13,
          14,
          17,
          18,
          21,
          22,
          25,
          26,
          29,
          30,
          33,
          34,
          37,
          38,
          41,
          42,
          45,
          46,
          49,
          50,
          53,
          54,
          57,
          58,
          61,
          62
        }.contains(c.index)) {
          expect(true, c.isRed);
          if (c.index >= 4) {
            expect(true, Suit.Diamond == c.suit || Suit.Heart == c.suit);
          }
        } else {
          expect(false, c.isRed);
        }

        if ({
          2,
          4,
          7,
          8,
          11,
          12,
          15,
          16,
          19,
          20,
          23,
          24,
          27,
          28,
          31,
          32,
          35,
          36,
          39,
          40,
          43,
          44,
          47,
          48,
          51,
          52,
          55,
          56,
          59,
          60,
          63
        }.contains(c.index)) {
          expect(true, c.isBlack);
          if (c.index >= 4) {
            expect(true, Suit.Club == c.suit || Suit.Spade == c.suit);
          }
        } else {
          expect(false, c.isBlack);
        }
      }
    });

    test('rank_and_suit', () {
      expect(Card.LowAceOfClubs, Card.fromRankSuit(Rank.LowAce, Suit.Club));
      expect(Card.TreyOfDiamonds, Card.fromRankSuit(Rank.Trey, Suit.Diamond));
      expect(Card.SevenOfHearts, Card.fromRankSuit(Rank.Seven, Suit.Heart));
      expect(Card.JackOfSpades, Card.fromRankSuit(Rank.Jack, Suit.Spade));

      expect(Rank.LowAce, Card.LowAceOfSpades.rank);
      expect(Rank.Ace, Card.LowAceOfSpades.highRank);
      expect(Rank.Four, Card.FourOfClubs.rank);
      expect(Rank.Nine, Card.NineOfDiamonds.rank);

      expect(Suit.Club, Card.DeuceOfClubs.suit);
      expect(Suit.Diamond, Card.SixOfDiamonds.suit);
      expect(Suit.Heart, Card.TenOfHearts.suit);
      expect(Suit.Spade, Card.KingOfSpades.suit);
    });

    test('io', () async {
      var data = await fetchHandData();

        for (int i = 0; i < data.count; i += 1) {
          var ht = data.hands[i];
          var deck = await MasterDeck.byName(ht.deck);
          var hand = cardsFromText(ht.hand).toList();

          if (deck.lowAces) {
            hand = hand.map((c) => Card.lowAceFix(c)).toList();
          }
          // print('${deck.name}  $hand  ${ht.hash}');
          expect(ht.len, hand.length);
          expect(ht.hash, quickHash(hand));
        }
    });
  });
}

class OneHandTest {
  final String deck;
  final String hand;
  final int len;
  final int hash;
  const OneHandTest(this.deck, this.hand, this.len, this.hash);
}

class HandTestData {
  final int count;
  List<OneHandTest> hands;
  HandTestData(this.count) : hands = [];
}

Future<HandTestData> fetchHandData() async {
  var file = File('../data/bin/hands_text.msgpack');
  var bytes = await file.readAsBytes();
  var pack = mp.deserialize(bytes.buffer.asUint8List());

  var data = HandTestData(pack['count']);
  for (var htin in pack['hands']) {
    var dname = pack['deckNames'][htin[0] - 1];
    var htout = OneHandTest(dname, htin[1], htin[2], htin[3]);
    // print("${htout.deck}  ${htout.hand}  ${htout.len}  ${htout.hash}");
    data.hands.add(htout);
  }
  return data;
}

int quickHash(List<Card> cards) {
  var h = 0x811c9dc5;

  for (int i = 0; i < cards.length; i += 1) {
    h ^= cards[i].index;
    h *= 0x01000193;
  }
  return h & 0xffffffff;
}
