import 'dart:io';
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

    test('hash test data file', () async {
      var bytes = await file.readAsBytes();
      var data = mp.deserialize(bytes);

      var hands = data['hands'];
      for (int i = 0; i < data['count']; i += 1) {
        var hand1 = OrphanHand.fromText(hands[i][0]);
        var hand2 = OrphanHand.fromText(hands[i][1]);
        var hand3 = OrphanHand.fromText(hands[i][2]);

        expect(hand1.length == hand2.length, true);
        expect(hand1.length == hand3.length, true);

        int j = OjRandom.rangeUniform(data['count']);
        var hand4 = OrphanHand.fromText(hands[j][0]);

        int h1 = FNVHash.u32(hand1);
        int h2 = FNVHash.u32(hand2);
        int h3 = FNVHash.u32(hand3);
        int h4 = FNVHash.u32(hand4);

        expect(h1 == h2, hand1.equals(hand2));
        expect(h1 == h3, false);
        expect(h1 == h4, i == j);

        h1 = BitfieldHash.u64co(hand1);
        h2 = BitfieldHash.u64co(hand2);
        h3 = BitfieldHash.u64co(hand3);
        h4 = BitfieldHash.u64co(hand4);

        expect(h1 == h2, true);
        expect(h1 == h3, false);
        expect(h1 == h4, i == j);

        if (hand1.length <= 16 && hand4.length <= 16) {
          h1 = PositionalHash.u64cos(hand1);
          h2 = PositionalHash.u64cos(hand2);
          h3 = PositionalHash.u64cos(hand3);

          expect(h1 == h2, true);
          expect(h1 == h3, true);
        }
        if (hand1.length <= 10 && hand4.length <= 10) {
          h1 = PositionalHash.u64c(hand1);
          h2 = PositionalHash.u64c(hand2);
          h3 = PositionalHash.u64c(hand3);
          h4 = PositionalHash.u64c(hand4);

          expect(h1 == h2, hand1.equals(hand2));
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);

          h1 = PositionalHash.u64co(hand1);
          h2 = PositionalHash.u64co(hand2);
          h3 = PositionalHash.u64co(hand3);
          h4 = PositionalHash.u64co(hand4);

          expect(h1 == h2, true);
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);

          h1 = PositionalHash.u64cos(hand1);
          h2 = PositionalHash.u64cos(hand2);
          h3 = PositionalHash.u64cos(hand3);

          expect(h1 == h2, true);
          expect(h1 == h3, true);

          h1 = PrimeHash.u64cos(hand1);
          h2 = PrimeHash.u64cos(hand2);
          h3 = PrimeHash.u64cos(hand3);

          expect(h1 == h2, true);
          expect(h1 == h3, true);
        }
        if (hand1.length <= 7 && hand4.length <= 7) {
          h1 = PrimeHash.u64co(hand1);
          h2 = PrimeHash.u64co(hand2);
          h3 = PrimeHash.u64co(hand3);
          h4 = PrimeHash.u64co(hand4);

          expect(h1 == h2, true);
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);
        }
        if (hand1.length <= 5 && hand4.length <= 5) {
          h1 = PrimeHash.u32cos(hand1);
          h2 = PrimeHash.u32cos(hand2);
          h3 = PrimeHash.u32cos(hand3);

          expect(h1 == h2, true);
          expect(h1 == h3, true);

          h1 = PositionalHash.u32c(hand1);
          h2 = PositionalHash.u32c(hand2);
          h3 = PositionalHash.u32c(hand3);
          h4 = PositionalHash.u32c(hand4);

          expect(h1 == h2, hand1.equals(hand2));
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);
        }
      }
    }, skip: skip);
  });
}
