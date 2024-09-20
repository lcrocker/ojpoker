import 'package:test/test.dart';
import 'package:onejoker/card.dart';
import 'package:onejoker/masterdeck.dart';

void main() {
  group('MasterDeck', () {
    test('properties', () async {
      for (var dname in [
        "poker",
        "bug",
        "54",
        "razz",
        "40",
        "skat",
        "euchre",
        "canasta",
        "pinochle"
      ]) {
        var deck = await MasterDeck.byName(dname);
        switch (dname) {
          case "poker":
            expect("english", deck.name);
            expect(52, deck.cardList.length);
            expect(false, deck.has(Card.Joker));
            expect(true, deck.has(Card.DeuceOfClubs));
            expect(false, deck.dupsAllowed);
            expect(false, deck.lowAces);
            break;
          case "bug":
            expect("onejoker", deck.name);
            expect(53, deck.cardList.length);
            expect(true, deck.has(Card.Joker));
            expect(false, deck.dupsAllowed);
            expect(false, deck.lowAces);
            break;
          case "54":
            expect("twojokers", deck.name);
            expect(54, deck.cardList.length);
            expect(true, deck.has(Card.BlackJoker));
            expect(true, deck.has(Card.TreyOfClubs));
            expect(false, deck.dupsAllowed);
            expect(false, deck.lowAces);
            break;
          case "razz":
            expect("low", deck.name);
            expect(52, deck.cardList.length);
            expect(false, deck.has(Card.Joker));
            expect(true, deck.has(Card.LowAceOfClubs));
            expect(false, deck.dupsAllowed);
            expect(true, deck.lowAces);
            break;
          case "40":
            expect("spanish", deck.name);
            expect(40, deck.cardList.length);
            expect(false, deck.has(Card.Joker));
            expect(false, deck.has(Card.NineOfClubs));
            expect(true, deck.has(Card.FourOfClubs));
            expect(false, deck.dupsAllowed);
            expect(true, deck.lowAces);
            break;
          case "skat":
            expect("german", deck.name);
            expect(32, deck.cardList.length);
            expect(false, deck.has(Card.Joker));
            expect(true, deck.has(Card.SevenOfClubs));
            expect(false, deck.has(Card.FiveOfClubs));
            expect(false, deck.dupsAllowed);
            expect(false, deck.lowAces);
            break;
          case "euchre":
            expect("euchre", deck.name);
            expect(24, deck.cardList.length);
            expect(false, deck.has(Card.Joker));
            expect(true, deck.has(Card.NineOfClubs));
            expect(false, deck.has(Card.EightOfClubs));
            expect(false, deck.dupsAllowed);
            expect(false, deck.lowAces);
            break;
          case "canasta":
            expect("canasta", deck.name);
            expect(108, deck.cardList.length);
            expect(true, deck.has(Card.BlackJoker));
            expect(true, deck.has(Card.SixOfClubs));
            expect(true, deck.dupsAllowed);
            expect(false, deck.lowAces);
            break;
          case "pinochle":
            expect("pinochle", deck.name);
            expect(48, deck.cardList.length);
            expect(false, deck.has(Card.Joker));
            expect(true, deck.has(Card.TenOfClubs));
            expect(false, deck.has(Card.SevenOfClubs));
            expect(true, deck.dupsAllowed);
            expect(false, deck.lowAces);
            break;
          default:
            assert(false);
        }
      }
    });
  });
}
