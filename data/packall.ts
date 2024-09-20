#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin

/**
 * @file packall.ts
 * @brief Build msgpack file from master_decks.json5
 */

import { packMasterDecks } from "./pack_master_decks.ts";
import { packHandsText } from "./pack_hands_text.ts";

Promise.all([
    packMasterDecks(),
    packHandsText()
]).then(() => { console.log("All done."); });
