#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin pack_poker_hands.ts

/**
 * @file pack_poker_hands.ts
 * @brief Build msgpack file from poker_hands_100k_eval.jsonc
 * 
 * JSON input is array of 5-tuples, each containing a hand and four
 * equivalence class rankings.
 * 
 * [
 *  [ "6h9s4s5h3c", 7440, 71, 23, 725 ],
 *  . . .
 * ]
 * 
 * Output is struct with same similar array.
 * 
 * type PokerEvalTestData = {
 *   count: number,
 *   hands: [ string, number, number, number, number ][];
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

type PokerEvalTestDataIn = [ string, number, number, number, number ];
function isPokerEvalTestDataIn(obj: unknown): obj is PokerEvalTestDataIn {
    if (! Array.isArray(obj)) return false;
    if (obj.length != 5) return false;
    if ("string" != typeof (obj[0])) return false;
    if ("number" != typeof (obj[1])) return false;
    if ("number" != typeof (obj[2])) return false;
    if ("number" != typeof (obj[3])) return false;
    return ("number" == typeof(obj[4]));
}
type PokerEvalTestDataOut = {
    count: number,
    hands: [ string, number, number, number, number ][];
}

export async function packPokerEvalHands() {
    const dataIn = jsonParse(await
        Deno.readTextFile(`${sdir()}/json/poker_hands_100k_eval.jsonc`));
    if (! Array.isArray(dataIn)) {
        throw new Error("bad input file");
    }
    // console.log(dataIn);
    const dataOut: PokerEvalTestDataOut = {
        count: dataIn.length,
        hands: [],
    };

    for (let i = 0; i < dataIn.length; i += 1) {
        const row = dataIn[i];
        if (! isPokerEvalTestDataIn(row)) {
            throw new Error("bad input data");
        }
        dataOut.hands.push([ row[0], row[1], row[2], row[3], row[4] ]);
    }
    // console.log(dataOut);

    const pack = mpEncode(dataOut);
    await Deno.writeFile(`${sdir()}/bin/poker_hands_100k_eval.msgpack`, pack);
    console.log("poker hand eval data packed");
}

if (import.meta.main) {
    await packPokerEvalHands();
}
