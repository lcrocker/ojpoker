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
        CardHashBase hasher = FNVHash();

        int h1 = hasher.u32(hand1);
        int h2 = hasher.u32(hand2);
        int h3 = hasher.u32(hand3);
        int h4 = hasher.u32(hand4);

        expect(h1 == h2, hand1.equals(hand2));
        expect(h1 == h3, false);
        expect(h1 == h4, i == j);

        hasher = BitfieldHash();
        h1 = hasher.u64co(hand1);
        h2 = hasher.u64co(hand2);
        h3 = hasher.u64co(hand3);
        h4 = hasher.u64co(hand4);

        expect(h1 == h2, true);
        expect(h1 == h3, false);
        expect(h1 == h4, i == j);

        if (hand1.length <= 16 && hand4.length <= 16) {
          hasher = PositionalHash();
          h1 = hasher.u64cos(hand1);
          h2 = hasher.u64cos(hand2);
          h3 = hasher.u64cos(hand3);

          expect(h1 == h2, true);
          expect(h1 == h3, true);
        }
        if (hand1.length <= 10 && hand4.length <= 10) {
          h1 = hasher.u64c(hand1);
          h2 = hasher.u64c(hand2);
          h3 = hasher.u64c(hand3);
          h4 = hasher.u64c(hand4);

          expect(h1 == h2, hand1.equals(hand2));
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);

          h1 = hasher.u64co(hand1);
          h2 = hasher.u64co(hand2);
          h3 = hasher.u64co(hand3);
          h4 = hasher.u64co(hand4);

          expect(h1 == h2, true);
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);

          h1 = hasher.u64cos(hand1);
          h2 = hasher.u64cos(hand2);
          h3 = hasher.u64cos(hand3);

          expect(h1 == h2, true);
          expect(h1 == h3, true);

          hasher = PrimeHash();
          h1 = hasher.u64cosr(hand1.map((c) => c.rank!));
          h2 = hasher.u64cosr(hand2.map((c) => c.rank!));
          h3 = hasher.u64cosr(hand3.map((c) => c.rank!));

          expect(h1 == h2, true);
          expect(h1 == h3, true);
        }
        if (hand1.length <= 7 && hand4.length <= 7) {
          h1 = hasher.u64co(hand1);
          h2 = hasher.u64co(hand2);
          h3 = hasher.u64co(hand3);
          h4 = hasher.u64co(hand4);

          expect(h1 == h2, true);
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);
        }
        if (hand1.length <= 5 && hand4.length <= 5) {
          h1 = hasher.u32cosr(hand1.map((c) => c.rank!));
          h2 = hasher.u32cosr(hand2.map((c) => c.rank!));
          h3 = hasher.u32cosr(hand3.map((c) => c.rank!));

          expect(h1 == h2, true);
          expect(h1 == h3, true);

          hasher = PositionalHash();
          h1 = hasher.u32c(hand1);
          h2 = hasher.u32c(hand2);
          h3 = hasher.u32c(hand3);
          h4 = hasher.u32c(hand4);

          expect(h1 == h2, hand1.equals(hand2));
          expect(h1 == h3, false);
          expect(h1 == h4, i == j);
        }
      }
    }, skip: skip);
  });
}
