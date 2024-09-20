#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin

/**
 * @file pack_master_decks.ts
 * @brief Build msgpack file from master_decks.json5
 * 
 * JSON input is array of deck info objects:
 * 
 * [ {
 * "name": "english",
 * "dups_allowed": false,
 * "low_aces": false,
 * "q_is_knight": false,
 * "aliases":["default","french","poker","bridge","52","deucetoseven",
 * "tienlen","gin","spades","hearts"],
 * "card_list":[8,9,10,11, 12,13,14,15, 16,17,18,19, 20,21,22,23,
 * 24,25,26,27, 28,29,30,31, 32,33,34,35, 36,37,38,39, 40,41,42,43,
 * 44,45,46,47, 48,49,50,51, 52,53,54,55, 56,57,58,59]
 * }, . . . ]
 * 
 * MessagePack output is one struct:
 * 
 * type MasterDeckData = {
 *    count: number;
 *    aliases: Record<string, number>;
 *    info: [ string, number[], boolean, boolean, boolean ][];
 * }
 */

import * as json5 from "https://deno.land/x/json5@v1.0.0/mod.ts";
import * as mp from "https://deno.land/x/msgpack@v1.2/mod.ts";

type MasterDeckData = {
    count: number;
    aliases: Record<string, number>;
    info: [ string, number[], boolean, boolean, boolean ][];
}

export async function packMasterDecks() {
    const deckDataIn = json5.parse(await Deno.readTextFile("./json/master_decks.json5"));
    const deckDataOut: MasterDeckData = {
        count: deckDataIn.length,
        aliases: {},
        info: [],
    };

    for (let i = 0; i < deckDataIn.length; i += 1) {
        for (let j = 0; j < deckDataIn[i].aliases.length; j += 1) {
            deckDataOut.aliases[deckDataIn[i].aliases[j]] = i + 1;
        }
        deckDataOut.aliases[deckDataIn[i].name] = i + 1;

        deckDataOut.info.push([ deckDataIn[i].name, deckDataIn[i].card_list,
            deckDataIn[i].dups_allowed, deckDataIn[i].low_aces,
            deckDataIn[i].q_is_knight ]);
    }
    // console.log(deckDataOut);

    const pack = mp.encode(deckDataOut);
    await Deno.writeFile("./bin/master_decks.msgpack", pack);
}

if (import.meta.main) {
    packMasterDecks().then(() => { console.log("MasterDecks done."); });
}
