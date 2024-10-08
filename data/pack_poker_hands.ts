#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin pack_poker_hands.ts

/**
 * @file pack_poker_hands.ts
 * @brief Build msgpack file from poker_hands_100k_eval.json5
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

import * as json5 from "https://deno.land/x/json5@v1.0.0/mod.ts";
import * as mp from "https://deno.land/x/msgpack@v1.2/mod.ts";

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

type PokerEvalTestData = {
    count: number,
    hands: [ string, number, number, number, number ][];
}

export async function packPokerEvalHands() {
    const dataIn = json5.parse(await Deno.readTextFile(`${sdir()}/json/poker_hands_100k_eval.json5`));
    // console.log(dataIn);
    const dataOut: PokerEvalTestData = {
        count: dataIn.length,
        hands: [],
    };

    for (let i = 0; i < dataIn.length; i += 1) {
        const row = dataIn[i];
        dataOut.hands.push([ row[0], row[1], row[2], row[3], row[4] ]);
    }
    // console.log(dataOut);

    const pack = mp.encode(dataOut);
    await Deno.writeFile(`${sdir()}/bin/poker_hands_100k_eval.msgpack`, pack);
    console.log("poker hand eval data packed");
}

if (import.meta.main) {
    await packPokerEvalHands();
}
