import 'dart:io';
import 'package:test/test.dart';
import 'package:msgpack_dart/msgpack_dart.dart' as mp;

import 'package:onejoker/onejoker.dart';

void main() {
  group('evaluation', () {
    dynamic skip = false;
    var file = File('../data/bin/poker_hands_100k_eval.msgpack');
    if (!file.existsSync()) {
      skip = "Test files not built, skipping...";
    }

    test('hands eval data file', () async {
      var bytes = await file.readAsBytes();
      var data = mp.deserialize(bytes);
      var hands = data['hands'];
      var high = HandEvaluatorHigh();

      for (int i = 0; i < data['count']; i += 1) {
        var irow = hands[i];
        var j = OjRandom.rangeUniform(100000);
        var jrow = hands[j];
        var ihand = OrphanHand.fromText(irow[0]);
        var jhand = OrphanHand.fromText(jrow[0]);

        expect(ihand.length, 5);
        expect(jhand.length, 5);
        HandValueHigh iv = high.referenceEvaluator(ihand);
        HandValueHigh jv = high.referenceEvaluator(jhand);

        if (irow[1] < jrow[1]) {
          expect(iv.value < jv.value, true);
        } else if (irow[1] > jrow[1]) {
          expect(iv.value > jv.value, true);
        } else {
          expect(iv.value == jv.value, true);
        }
      }
    }, skip: skip);
  });
}
