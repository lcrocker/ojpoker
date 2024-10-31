import 'dart:io';
import 'package:test/test.dart';
import 'package:jsonc/jsonc.dart';
import 'package:onejoker/onejoker.dart';

bool ranksIdentical(Hand h1, Hand h2) {
  if (h1.length != h2.length) {
    return false;
  }
  for (var i = 0; i < h1.length; i += 1) {
    if (h1[i].rank != h2[i].rank) {
      return false;
    }
  }
  return true;
}

Hand reorder(Hand hand) {
  var ret = hand.clone();
  if (hand.length < 2) {
    return ret;
  }
  int max_tries = 10; // for hands like "7d7d"
  do {
    ojShuffle(ret.cards);
    max_tries -= 1;
    if (max_tries <= 0) {
      break;
    }
  } while (ret.equals(hand));
  return ret;
}

Hand resuit(Hand hand) {
  var ret = hand.clone();
  for (var i = 0; i < ret.length; i += 1) {
    int s = ret[i].suit.index;
    if (s > 0) {
      s += 1;
      if (s > 4) {
        s = 1;
      }
      ret[i] = Card.fromRankSuit(ret[i].rank, Suit.fromInt(s));
    }
  }
  return ret;
}

void main() {
  group('hashes', () {
    test('random hands file', () async {
      var file = File('../data/json/random_hands_100k.jsonc');
      expect(file.existsSync(), true);
      final data = jsonc.decode(await file.readAsString());

      final max = ONEJOKER_FULL_TESTS ? data["hands"].length : 10000;
      for (var i = 0; i < max; i += 1) {
        int index = ONEJOKER_FULL_TESTS
            ? i
            : OjRandom.rangeUniform(data["hands"].length);

        Deck d = Deck(data["decks"][data["hands"][index][0] - 1]);
        d.shuffle();
        Hand h1 = d.newHand();
        h1.pushN(cardsFromText(data["hands"][index][1]));
        Hand h2 = d.newHand();
        h2.pushN(cardsFromText(h1.toString()));
        Hand h3 = reorder(h1);
        Hand h4 = resuit(h1);

        // print("$index ${h1} ${h3} ${h4}");
        expect(h1.equals(h2), true);
        expect(h1.isEquivalentTo(h3), true);

        expect(ojhFNV32(h1.cards), ojhFNV32(h2.cards));
        expect(ojhFNV32(h1.cards) == ojhFNV32(h3.cards), h1.equals(h3));
        expect(ojhFNV32(h1.cards) == ojhFNV32(h4.cards), h1.equals(h4));

        expect(ojhFNV64(h1.cards), ojhFNV64(h2.cards));
        expect(ojhFNV64(h1.cards) == ojhFNV64(h3.cards), h1.equals(h3));
        expect(ojhFNV64(h1.cards) == ojhFNV64(h4.cards), h1.equals(h4));

        if (!d.master.dupsAllowed) {
          expect(ojhBitfield64co(h1.cards), ojhBitfield64co(h2.cards));
          expect(ojhBitfield64co(h1.cards), ojhBitfield64co(h3.cards));
          expect(ojhBitfield64co(h1.cards) == ojhBitfield64co(h4.cards),
              h1.isEquivalentTo(h4));
        }
        if (h1.length > 16) continue;
        expect(ojhPositional64cs(h1.cards), ojhPositional64cs(h2.cards));
        expect(ojhPositional64cs(h1.cards) == ojhPositional64cs(h3.cards),
            ranksIdentical(h1, h3));
        expect(ojhPositional64cs(h1.cards), ojhPositional64cs(h4.cards));

        expect(ojhPositional64cr(h1.ranks), ojhPositional64cr(h2.ranks));
        expect(ojhPositional64cr(h1.ranks) == ojhPositional64cr(h3.ranks),
            ranksIdentical(h1, h3));
        expect(ojhPositional64cr(h1.ranks), ojhPositional64cr(h4.ranks));

        if (h1.length > 10) continue;
        expect(ojhPositional64c(h1.cards), ojhPositional64c(h2.cards));
        expect(ojhPositional64c(h1.cards) == ojhPositional64c(h3.cards),
            h1.equals(h3));
        expect(ojhPositional64c(h1.cards) == ojhPositional64c(h4.cards),
            h1.equals(h4));

        expect(ojhPrime64cor(h1.ranks), ojhPrime64cor(h2.ranks));
        expect(ojhPrime64cor(h1.ranks), ojhPrime64cor(h3.ranks));
        expect(ojhPrime64cor(h1.ranks), ojhPrime64cor(h4.ranks));

        if (h1.length > 8) continue;
        expect(ojhPositional32cs(h1.cards), ojhPositional32cs(h2.cards));
        expect(ojhPositional32cs(h1.cards) == ojhPositional32cs(h3.cards),
            ranksIdentical(h1, h3));
        expect(ojhPositional32cs(h1.cards), ojhPositional32cs(h4.cards));

        expect(ojhPositional32cr(h1.ranks), ojhPositional32cr(h2.ranks));
        expect(ojhPositional32cr(h1.ranks) == ojhPositional32cr(h3.ranks),
            ranksIdentical(h1, h3));
        expect(ojhPositional32cr(h1.ranks), ojhPositional32cr(h4.ranks));

        if (h1.length > 7) continue;
        expect(ojhPrime64co(h1.cards), ojhPrime64co(h2.cards));
        expect(ojhPrime64co(h1.cards), ojhPrime64co(h3.cards));
        expect(ojhPrime64co(h1.cards) == ojhPrime64co(h4.cards),
            h1.isEquivalentTo(h4));

        if (h1.length > 5) continue;
        expect(ojhPositional32c(h1.cards), ojhPositional32c(h2.cards));
        expect(ojhPositional32c(h1.cards) == ojhPositional32c(h3.cards),
            h1.equals(h3));
        expect(ojhPositional32c(h1.cards) == ojhPositional32c(h4.cards),
            h1.equals(h4));

        expect(ojhPrime32cor(h1.ranks), ojhPrime32cor(h2.ranks));
        expect(ojhPrime32cor(h1.ranks), ojhPrime32cor(h3.ranks));
        expect(ojhPrime32cor(h1.ranks), ojhPrime32cor(h4.ranks));
      }
    });
  });
}
