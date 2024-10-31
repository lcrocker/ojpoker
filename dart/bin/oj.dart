import 'package:onejoker/onejoker.dart';

void main(List<String> arguments) {
  print("// 100000 random hands over varying lengths from all decks");
  print("{\n\"decks\":[");

  for (int i = 0; i < MasterDeck.deckCount; i += 1) {
    var d = MasterDeck.byIndex(i + 1);
    print("\"${d.name}\",");
  }
  print("],\n\"hands\":[");

  for (int i = 0; i < 100000; i += 1) {
    int dnum = 1 + OjRandom.rangeUniform(MasterDeck.deckCount);
    MasterDeck md = MasterDeck.byIndex(dnum);
    Deck d = Deck(md.name);
    d.shuffle();

    int len = 1 + // 3d4 - 2: nice bell curve
      OjRandom.rangeUniform(4) +
      OjRandom.rangeUniform(4) +
      OjRandom.rangeUniform(4);

    var h = d.newHand();
    h.draw(len);
    var hash = ojhFNV32(h.cards);

    print("[${dnum},\"${h}\",${hash}],");
  }
  print("]\n}");
}
