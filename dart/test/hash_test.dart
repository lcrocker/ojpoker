import 'dart:io';
import 'package:collection/collection.dart';
import 'package:test/test.dart';
import 'package:msgpack_dart/msgpack_dart.dart' as mp;

import 'package:onejoker/onejoker.dart';

void main() {
  group('hashes', () {
    dynamic skip = false;
    var file = File('../data/bin/hash_tests.msgpack');
    if (!file.existsSync()) {
      skip = "Test files not built, skipping...";
    }

    Function leq = ListEquality().equals;

    test('hash test data file', () async {
      var bytes = await file.readAsBytes();
      var data = mp.deserialize(bytes);

      var hands = data['hands'];
      for (int i = 0; i < data['count']; i += 1) {
        var hand1 = CardStack.fromText(hands[i][0]);
        var hand2 = CardStack.fromText(hands[i][1]);
        var hand3 = CardStack.fromText(hands[i][2]);

        expect(hand1.length == hand2.length, true);
        expect(hand1.length == hand3.length, true);

        int j = OjRandom.rangeUniform(data['count']);
        var hand4 = CardStack.fromText(hands[j][0]);

        int h1 = cardHashFNV_32(hand1);
        int h2 = cardHashFNV_32(hand2);
        int h3 = cardHashFNV_32(hand3);
        int h4 = cardHashFNV_32(hand4);

        expect(h1 == h2, leq(hand1.toList(), hand2.toList()));
        expect(h1 == h3, false);
        expect(h1 == h4, i == j);

        h1 = cardHashBitFieldU_64(hand1);
        h2 = cardHashBitFieldU_64(hand2);
        h3 = cardHashBitFieldU_64(hand3);
        h4 = cardHashBitFieldU_64(hand4);

        expect(h1 == h2, true);
        expect(h1 == h3, false);
        expect(h1 == h4, i == j);

        if (hand1.length <= 16 && hand4.length <= 16) {
          h1 = cardHashBase16RU_64(hand1);
          h2 = cardHashBase16RU_64(hand2);
          h3 = cardHashBase16RU_64(hand3);

          expect(h1 == h2, true);
          expect(h1 == h3, true);
        }
        if (hand1.length <= 10 && hand4.length <= 10) {
          h1 = cardHashBase64_64(hand1);
          h2 = cardHashBase64_64(hand2);
          h3 = cardHashBase64_64(hand3);
          h4 = cardHashBase64_64(hand4);

          expect(h1 == h2, leq(hand1.toList(), hand2.toList()));
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);

          h1 = cardHashBase64U_64(hand1);
          h2 = cardHashBase64U_64(hand2);
          h3 = cardHashBase64U_64(hand3);
          h4 = cardHashBase64U_64(hand4);

          expect(h1 == h2, true);
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);

          h1 = cardHashPrimeRU_64(hand1);
          h2 = cardHashPrimeRU_64(hand2);
          h3 = cardHashPrimeRU_64(hand3);

          expect(h1 == h2, true);
          expect(h1 == h3, true);
        }
        if (hand1.length <= 7 && hand4.length <= 7) {
          h1 = cardHashPrimeU_64(hand1);
          h2 = cardHashPrimeU_64(hand2);
          h3 = cardHashPrimeU_64(hand3);
          h4 = cardHashPrimeU_64(hand4);

          expect(h1 == h2, true);
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);
        }
      }
    }, skip: skip);
  });
}
