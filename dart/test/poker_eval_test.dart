import 'dart:io';
import 'package:test/test.dart';
import 'package:jsonc/jsonc.dart';
import 'package:onejoker/onejoker.dart';

void main() {
  group('evaluation', () {
    test('hands eval data file', () async {
      var file = File('../data/json/poker_hands_100k.jsonc');
      expect(file.existsSync(), true);
      final data = jsonc.decode(await file.readAsString());

      final highDeck = Deck("poker");
      final high = HandEvaluatorHigh();
      final dto7 = HandEvaluatorDeuceToSeven();
      final lowDeck = Deck("low");
      final ato5 = HandEvaluatorAceToFive();
      // final ato6 = HandEvaluatorAceToSix();
      final badugi = HandEvaluatorBadugi();

      final max = ONEJOKER_FULL_TESTS ? data.length : 10000;
      for (var i = 0; i < max; i += 1) {
        int index =
            ONEJOKER_FULL_TESTS ? i : OjRandom.rangeUniform(data.length);
        int j = OjRandom.rangeUniform(data.length);

        Hand h1 = highDeck.newHand();
        h1.pushN(cardsFromText(data[index][0] as String));
        HandValueHigh hv1 = high.valueOf(h1);
        Hand h2 = highDeck.newHand();
        h2.pushN(cardsFromText(data[j][0] as String));
        HandValueHigh hv2 = high.valueOf(h2);

        if (data[index][1] < data[j][1]) {
          expect(hv1 < hv2, true);
        } else if (data[index][1] > data[j][1]) {
          expect(hv1 > hv2, true);
        } else {
          expect(hv1, hv2);
        }
        HandValueDeuceToSeven dv1 = dto7.valueOf(h1);
        HandValueDeuceToSeven dv2 = dto7.valueOf(h2);

        if (data[index][3] < data[j][3]) {
          expect(dv1 < dv2, true);
        } else if (data[index][3] > data[j][3]) {
          expect(dv1 > dv2, true);
        } else {
          expect(dv1, dv2);
        }
        h1 = lowDeck.newHand();
        h1.pushN(cardsFromText(data[index][0] as String));
        HandValueAceToFive av1 = ato5.valueOf(h1);
        h2 = lowDeck.newHand();
        h2.pushN(cardsFromText(data[j][0] as String));
        HandValueAceToFive av2 = ato5.valueOf(h2);

        if (data[index][2] < data[j][2]) {
          expect(av1 < av2, true);
        } else if (data[index][2] > data[j][2]) {
          expect(av1 > av2, true);
        } else {
          expect(av1, av2);
        }
        HandValueBadugi bv1 = badugi.valueOf(h1);
        HandValueBadugi bv2 = badugi.valueOf(h2);

        if (data[index][4] < data[j][4]) {
          expect(bv1 < bv2, true);
        } else if (data[index][4] > data[j][4]) {
          expect(bv1 > bv2, true);
        } else {
          expect(bv1, bv2);
        }
      }
    });
  });
}
