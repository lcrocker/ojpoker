// import 'dart:io';
// import 'package:min_max_heap/min_max_heap.dart';
// import 'package:onejoker/onejoker.dart';

// Build tables for high hands, prime hash

void main(List<String> arguments) {
  // List<HighHand> allHands = [];
  // Map<int, int> hashMap = {};
  // Map<int, HighHand> valueMap = {};

  // var heap = MinMaxHeap<HighHand>(criteria: (HighHand v) => v.value);
  // var deck = Deck("poker");

  // int total = 0;
  // for (Hand hand in deck.combinations(5)) {
  //   total += 1;

  //   HighHand v = HighHand.referenceEvaluator(hand);
  //   heap.add(v);
  // }
  // if (2598960 != total) {
  //   print("Error: total hands = $total");
  // }
  // while (heap.isNotEmpty) {
  //   HighHand v = heap.removeMin();
  //   allHands.add(v);
  // }
  // int equiv = 0;
  // int pValue = 0;
  // int pEquiv = 0;

  // for (HighHand v in allHands) {
  //   if (v.value != pValue) {
  //     equiv += 1;
  //   }
  //   assert(v.value >= pValue && equiv >= pEquiv);
  //   pValue = v.value;
  //   pEquiv = equiv;

  //   int hash = PrimeHash.u64co(v.hand);
  //   if (hashMap.containsKey(hash)) {
  //     assert(hashMap[hash] == equiv);
  //   } else {
  //     hashMap[hash] = equiv;
  //   }
  //   if (!valueMap.containsKey(equiv)) {
  //     valueMap[equiv] = v;
  //   }
  // }

  // var file = File('../data/json/high_hand_prime_hash.jsonc');

  // out(String s) {
  //   file.writeAsStringSync("$s\n", mode: FileMode.append);
  // }
  // file.writeAsStringSync("// Generated file -- do not edit\n");
  // out("{\n hash_count: $total,\n eclass_count: $equiv,");
  // out(" hashes: [");

  // for (var entry in hashMap.entries) {
  //   out("[${entry.key},${entry.value}],");
  // }
  // out("],\n eclasses: [");

  // for (var entry in valueMap.entries) {
  //   out("[${entry.value.level.index},[${entry.value.ranks[0].index},"
  //       "${entry.value.ranks[1].index},${entry.value.ranks[2].index},"
  //       "${entry.value.ranks[3].index},${entry.value.ranks[4].index}]],"
  //       " // ${entry.value.fullName()}");
  // }
  // out("],\n}");
}
