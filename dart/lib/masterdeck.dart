import 'package:onejoker/card.dart';

import 'dart:io';
import 'package:msgpack_dart/msgpack_dart.dart' as mp;

// Incoming structure:
// {
//     count: number;
//     aliases: Record<string, number>;
//     info: [ string, number[], boolean, boolean, boolean ][];
// }

class MasterDeck {
  final String name;
  final int cardSet;
  final List<Card> cardList;
  final bool dupsAllowed;
  final bool lowAces;
  final bool qIsKnight;

  static bool loaded = false;
  static Map<String, int> aliases = <String, int>{};
  static dynamic masterData;

  MasterDeck._(this.name, this.cardSet, this.cardList, this.dupsAllowed,
      this.lowAces, this.qIsKnight);

  static Future<void> load() async {
    if (loaded) return;
    var file = File('../data/bin/master_decks.msgpack');
    var bytes = await file.readAsBytes();
    masterData = mp.deserialize(bytes.buffer.asUint8List());

    for (var entry in masterData['aliases'].entries) {
      aliases[entry.key] = entry.value;
    }
    for (int i = 0; i < masterData['count']; i += 1) {
      var mi = masterData['info'][i];
      aliases[mi[0]] = i + 1;
    }
    loaded = true;
  }

  static Future<MasterDeck> byName(String name) async {
    await MasterDeck.load();
    var id = aliases[name] ?? 0;
    if (id == 0) return throw 'Unknown deck: $name';

    var mi = masterData['info'][id - 1];
    var cardSet = 0;
    var cardList = <Card>[];

    for (int j = 0; j < mi[1].length; j += 1) {
      cardSet |= (1 << mi[1][j]);
      cardList.add(Card.fromInt(mi[1][j]));
    }
    return MasterDeck._(mi[0], cardSet, cardList, mi[2], mi[3], mi[4]);
  }

  bool has(Card c) {
    return (cardSet & (1 << c.index)) != 0;
  }
}
