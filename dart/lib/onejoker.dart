// The [OneJoker](https://onejoker.org) library for
// [Dart](https://dart/dev) is a collection of tools for working with
// playing cards in general and poker in particular.
//
// Here's a sample hand of poker:
// ```
// var d = Deck("poker");
// d.shuffle();
// var p1 = d.newHand(5);
// var p2 = d.newHand(5);
//
// var v1 = HighHand.eval();
// var v2 = HighHand.eval();
//
// if (v1.value < v2.value) {
//   print("Player 1 wins with ${v1.fullName()}");
// } else if (v2.value < v2.value) {
//   print("Player 2 wins with ${v2.fullName()}");
// } else {
//   print("Players tie with ${v1.fullName()}");
// }
// ```
library;

export 'src/troolean.dart';
export 'src/utilities.dart';
export 'src/cards/cards.dart';
export 'src/poker/poker.dart';
