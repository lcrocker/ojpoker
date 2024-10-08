import 'package:onejoker/onejoker.dart';

void main(List<String> arguments) {
  var d = Deck('poker');

  print("$d");
  d.shuffle();
  print("$d");
  var h = d.newHand(5);
  print("$h ${h[0]} ${h[1]} ${h[2]} ${h[3]} ${h[4]}");
  print("$d");

  h.sort();
  print("$h ${h[0]} ${h[1]} ${h[2]} ${h[3]} ${h[4]}");
  for (Card c in h) {
    print(c);
  }

  print(cardsFromText("9c2dTs6h4s"));
  print(cardsFromText("9c2dTs6h4s").toList());

  print(OrphanHand.fromIterable([Card.AceOfClubs, Card.TenOfSpades, Card.SixOfHearts]));
  print(OrphanHand.fromText("AcTs6h"));
}
