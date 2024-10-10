#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin pack_hands_text.ts

/**
 * @file pack_hands_text.ts
 * @brief Build msgpack file from hands_text.jsonc
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

import { parse as jsonParse } from "jsr:@std/jsonc";
import { encode as mpEncode } from "jsr:@lambdalisue/messagepack";

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

type HandTestDataIn = [ string, string, number, number ];
function isHandTestDataIn(obj: unknown): obj is HandTestDataIn {
    if (! Array.isArray(obj)) return false;
    if (obj.length != 4) return false;
    if ("string" != typeof (obj[0])) return false;
    if ("string" != typeof (obj[1])) return false;
    if ("number" != typeof (obj[2])) return false;
    return ("number" == typeof(obj[3]));
}

type HandTestDataOut = {
    count: number,
    deck_names: string[],
    hands: [ number, string, number, number ][];
}

export async function packHandsText() {
    let deckNoNext = 1;
    const deckMap: Map<string, number> = new Map();

    const handDataIn = jsonParse(await
        Deno.readTextFile(`${sdir()}/json/hands_text.jsonc`));
    if (! Array.isArray(handDataIn)) {
        throw new Error("bad input file");
    }
    const handDataOut: HandTestDataOut = {
        count: handDataIn.length,
        deck_names: [],
        hands: [],
    };

    for (let i = 0; i < handDataIn.length; i += 1) {
        const hd = handDataIn[i];
        if (! isHandTestDataIn(hd)) {
            throw new Error("bad input file");
        }
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

    const pack = mpEncode(handDataOut);
    await Deno.writeFile(`${sdir()}/bin/hands_text.msgpack`, pack);
    console.log("hand data packed");
}

if (import.meta.main) {
    await packHandsText();
}
