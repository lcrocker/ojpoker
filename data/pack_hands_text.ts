#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin

/**
 * @file pack_hands_text.ts
 * @brief Build msgpack file from hands_text.json5
 * 
 * JSON input is array of 4-tuples, each containing deck name, hand,
 * length, and hash:
 * 
 * [
 *  [ "onejoker", "7c4sAh", 3, 1820849030 ],
 *  . . .
 * ]
 * 
 * Output is struct with similar array, but deck names are consolidated
 * into numbers for compactness:
 * 
 * type HandTestData = {
 *   count: number,
 *   deck_names: string[],
 *   hands: [ number, string, number, number ][];
 * }
 */

import * as json5 from "https://deno.land/x/json5@v1.0.0/mod.ts";
import * as mp from "https://deno.land/x/msgpack@v1.2/mod.ts";

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

type HandTestData = {
    count: number,
    deck_names: string[],
    hands: [ number, string, number, number ][];
}

export async function packHandsText() {
    let deckNoNext = 1;
    const deckMap: Map<string, number> = new Map();

    const handDataIn = json5.parse(await Deno.readTextFile(`${sdir()}/json/hands_text.json5`));
    const handDataOut: HandTestData = {
        count: handDataIn.length,
        deck_names: [],
        hands: [],
    };

    for (let i = 0; i < handDataIn.length; i += 1) {
        const hd = handDataIn[i];
        let dn = deckMap.get(hd[0]);
        if (! dn) {
            dn = deckNoNext;
            deckNoNext += 1;
            deckMap.set(hd[0], dn);
            handDataOut.deck_names.push(hd[0]);
        }
        handDataOut.hands.push([ dn, hd[1], hd[2], hd[3], ]);
    }
    // console.log(handDataOut);

    const pack = mp.encode(handDataOut);
    await Deno.writeFile(`${sdir()}/bin/hands_text.msgpack`, pack);
    console.log("hand data packed");
}

if (import.meta.main) {
    await packHandsText();
}
