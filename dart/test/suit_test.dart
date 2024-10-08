import 'package:test/test.dart';
import 'package:onejoker/onejoker.dart';

void main() {
  group('Suit', () {
    test('constants', () {
      expect(1, Suit.Club.index);
      expect(2, Suit.Diamond.index);
      expect(3, Suit.Heart.index);
      expect(4, Suit.Spade.index);
    });

    test('club', () {
      expect(Suit.Club, Suit.fromChar('c'));
      expect(Suit.Club, Suit.fromChar('♣'));
      expect('c', Suit.Club.toChar());
      expect('♣', Suit.Club.toUnicode());
      expect('club', Suit.Club.name);
      expect('clubs', Suit.Club.plural);
      expect('a', Suit.Club.article);
    });

    test('diamond', () {
      expect(Suit.Diamond, Suit.fromChar('d'));
      expect(Suit.Diamond, Suit.fromChar('♦'));
      expect('d', Suit.Diamond.toChar());
      expect('♦', Suit.Diamond.toUnicode());
      expect('diamond', Suit.Diamond.name);
      expect('diamonds', Suit.Diamond.plural);
      expect('a', Suit.Diamond.article);
    });

    test('heart', () {
      expect(Suit.Heart, Suit.fromChar('h'));
      expect(Suit.Heart, Suit.fromChar('♥'));
      expect('h', Suit.Heart.toChar());
      expect('♥', Suit.Heart.toUnicode());
      expect('heart', Suit.Heart.name);
      expect('hearts', Suit.Heart.plural);
      expect('a', Suit.Heart.article);
    });

    test('spade', () {
      expect(Suit.Spade, Suit.fromChar('s'));
      expect(Suit.Spade, Suit.fromChar('♠'));
      expect('s', Suit.Spade.toChar());
      expect('♠', Suit.Spade.toUnicode());
      expect('spade', Suit.Spade.name);
      expect('spades', Suit.Spade.plural);
      expect('a', Suit.Spade.article);
    });

    test('compare', () {
      expect(-1, Suit.Club.compareTo(Suit.Diamond));
      expect(0, Suit.Heart.compareTo(Suit.Heart));
      expect(1, Suit.Spade.compareTo(Suit.Heart));
      expect(true, Suit.Club < Suit.Diamond);
      expect(false, Suit.Heart < Suit.Heart);
      expect(false, Suit.Spade < Suit.Heart);
    });
  });
}
