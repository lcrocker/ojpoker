#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin

/**
 * @file packall.ts
 * @brief Build msgpack file from master_decks.jsonc
 */

import { packHandsText } from "./pack_hands_text.ts";
import { packHashTestData } from "./pack_hash_tests.ts";
import { packPokerEvalHands } from "./pack_poker_hands.ts";

export function packAll() {
    return Promise.all([
        packHandsText(),
        packHashTestData(),
        packPokerEvalHands(),
    ]);
}

if (import.meta.main) {
    await packAll();
}
