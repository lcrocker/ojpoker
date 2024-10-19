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

        int h1 = ojhFNV32(hand1);
        int h2 = ojhFNV32(hand2);
        int h3 = ojhFNV32(hand3);
        int h4 = ojhFNV32(hand4);

        expect(h1 == h2, hand1.equals(hand2));
        expect(h1 == h3, false);
        expect(h1 == h4, i == j);

        h1 = ojhBitfield64co(hand1);
        h2 = ojhBitfield64co(hand2);
        h3 = ojhBitfield64co(hand3);
        h4 = ojhBitfield64co(hand4);

        expect(h1 == h2, true);
        expect(h1 == h3, false);
        expect(h1 == h4, i == j);

        if (hand1.length <= 16 && hand4.length <= 16) {
          h1 = ojhPositional64cs(hand1);
          h2 = ojhPositional64cs(hand2);
          h3 = ojhPositional64cs(hand3);

          expect(h1 == h2, true);
          expect(h1 == h3, true);
        }
        if (hand1.length <= 10 && hand4.length <= 10) {
          h1 = ojhPositional64c(hand1);
          h2 = ojhPositional64c(hand2);
          h3 = ojhPositional64c(hand3);
          h4 = ojhPositional64c(hand4);

          expect(h1 == h2, hand1.equals(hand2));
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);

          h1 = ojhPrime64cor(hand1.map((c) => c.rank!));
          h2 = ojhPrime64cor(hand2.map((c) => c.rank!));
          h3 = ojhPrime64cor(hand3.map((c) => c.rank!));

          expect(h1 == h2, true);
          expect(h1 == h3, true);
        }
        if (hand1.length <= 7 && hand4.length <= 7) {
          h1 = ojhPrime64co(hand1);
          h2 = ojhPrime64co(hand2);
          h3 = ojhPrime64co(hand3);
          h4 = ojhPrime64co(hand4);

          expect(h1 == h2, true);
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);
        }
        if (hand1.length <= 5 && hand4.length <= 5) {
          h1 = ojhPrime32cor(hand1.map((c) => c.rank!));
          h2 = ojhPrime32cor(hand2.map((c) => c.rank!));
          h3 = ojhPrime32cor(hand3.map((c) => c.rank!));

          expect(h1 == h2, true);
          expect(h1 == h3, true);

          h1 = ojhPositional32c(hand1);
          h2 = ojhPositional32c(hand2);
          h3 = ojhPositional32c(hand3);
          h4 = ojhPositional32c(hand4);

          expect(h1 == h2, hand1.equals(hand2));
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);
        }
      }
    }, skip: skip);
  });
}
