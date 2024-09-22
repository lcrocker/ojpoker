#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin

/**
 * @file packall.ts
 * @brief Build msgpack file from master_decks.json5
 */

import { packHandsText } from "./pack_hands_text.ts";

export function packAll() {
    return Promise.all([
        packHandsText(),
    ]);
}

if (import.meta.main) {
    await packAll();
}
