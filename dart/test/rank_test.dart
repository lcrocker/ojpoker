import 'package:test/test.dart';
import 'package:onejoker/onejoker.dart';

void main() {
   group('Rank', () {
    test('constants', () {
      expect(1, Rank.LowAce.index);
      expect(2, Rank.Deuce.index);
      expect(3, Rank.Trey.index);
      expect(4, Rank.Four.index);
      expect(5, Rank.Five.index);
      expect(6, Rank.Six.index);
      expect(7, Rank.Seven.index);
      expect(8, Rank.Eight.index);
      expect(9, Rank.Nine.index);
      expect(10, Rank.Ten.index);
      expect(11, Rank.Jack.index);
      expect(12, Rank.Knight.index);
      expect(13, Rank.Queen.index);
      expect(14, Rank.King.index);
      expect(15, Rank.Ace.index);
    });

    test('low_ace', () {
      // fromChar doesn't do low aces
      expect('A', Rank.LowAce.toChar());
      expect('ace', Rank.LowAce.name);
      expect('aces', Rank.LowAce.plural);
      expect('an', Rank.LowAce.article);
    });

    test('deuce', () {
      expect(Rank.Deuce, Rank.fromChar('2'));
      expect('2', Rank.Deuce.toChar());
      expect('deuce', Rank.Deuce.name);
      expect('deuces', Rank.Deuce.plural);
      expect('a', Rank.Deuce.article);
    });

    test('trey', () {
      expect(Rank.Trey, Rank.fromChar('3'));
      expect('3', Rank.Trey.toChar());
      expect('trey', Rank.Trey.name);
      expect('treys', Rank.Trey.plural);
      expect('a', Rank.Trey.article);
    });

    test('four', () {
      expect(Rank.Four, Rank.fromChar('4'));
      expect('4', Rank.Four.toChar());
      expect('four', Rank.Four.name);
      expect('fours', Rank.Four.plural);
      expect('a', Rank.Four.article);
    });

    test('five', () {
      expect(Rank.Five, Rank.fromChar('5'));
      expect('5', Rank.Five.toChar());
      expect('five', Rank.Five.name);
      expect('fives', Rank.Five.plural);
      expect('a', Rank.Five.article);
    });

    test('six', () {
      expect(Rank.Six, Rank.fromChar('6'));
      expect('6', Rank.Six.toChar());
      expect('six', Rank.Six.name);
      expect('sixes', Rank.Six.plural);
      expect('a', Rank.Six.article);
    });

    test('seven', () {
      expect(Rank.Seven, Rank.fromChar('7'));
      expect('7', Rank.Seven.toChar());
      expect('seven', Rank.Seven.name);
      expect('sevens', Rank.Seven.plural);
      expect('a', Rank.Seven.article);
    });

    test('eight', () {
      expect(Rank.Eight, Rank.fromChar('8'));
      expect('8', Rank.Eight.toChar());
      expect('eight', Rank.Eight.name);
      expect('eights', Rank.Eight.plural);
      expect('an', Rank.Eight.article);
    });

    test('nine', () {
      expect(Rank.Nine, Rank.fromChar('9'));
      expect('9', Rank.Nine.toChar());
      expect('nine', Rank.Nine.name);
      expect('nines', Rank.Nine.plural);
      expect('a', Rank.Nine.article);
    });

    test('ten', () {
      expect(Rank.Ten, Rank.fromChar('T'));
      expect('T', Rank.Ten.toChar());
      expect('ten', Rank.Ten.name);
      expect('tens', Rank.Ten.plural);
      expect('a', Rank.Ten.article);
    });

    test('jack', () {
      expect(Rank.Jack, Rank.fromChar('J'));
      expect('J', Rank.Jack.toChar());
      expect('jack', Rank.Jack.name);
      expect('jacks', Rank.Jack.plural);
      expect('a', Rank.Jack.article);
    });

    test('knight', () {
      expect(Rank.Knight, Rank.fromChar('C'));
      expect('C', Rank.Knight.toChar());
      expect('knight', Rank.Knight.name);
      expect('knights', Rank.Knight.plural);
      expect('a', Rank.Knight.article);
    });

    test('queen', () {
      expect(Rank.Queen, Rank.fromChar('Q'));
      expect('Q', Rank.Queen.toChar());
      expect('queen', Rank.Queen.name);
      expect('queens', Rank.Queen.plural);
      expect('a', Rank.Queen.article);
    });

    test('king', () {
      expect(Rank.King, Rank.fromChar('K'));
      expect('K', Rank.King.toChar());
      expect('king', Rank.King.name);
      expect('kings', Rank.King.plural);
      expect('a', Rank.King.article);
    });

    test('ace', () {
      expect(Rank.Ace, Rank.fromChar('A'));
      expect('A', Rank.Ace.toChar());
      expect('ace', Rank.Ace.name);
      expect('aces', Rank.Ace.plural);
      expect('an', Rank.Ace.article);
    });

    test('compare', () {
      expect(-1, Rank.LowAce.compareTo(Rank.Deuce));
      expect(0, Rank.Seven.compareTo(Rank.Seven));
      expect(1, Rank.King.compareTo(Rank.Queen));
      expect(true, Rank.LowAce < Rank.Deuce);
      expect(false, Rank.Seven < Rank.Seven);
      expect(false, Rank.King < Rank.Queen);
    });
  });
}
