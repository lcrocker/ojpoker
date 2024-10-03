#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin pack_hash_tests.ts

/**
 * @file pack_hash_tests.ts
 * @brief Build msgpack file from hash_tests.json5
 * 
 * JSON input is array of 3-tuples. Each is a hand of cards, the
 * second element being the same as the first but re-ordered, and the
 * third element being the same but with different suits.
 * 
 * [
 *  ["7sAd5s","5s7sAd","Ac5d7c"],
 *  . . .
 * ]
 * 
 * Output is struct:
 * 
 * type HashTestData = {
 *   count: number,
 *   hands: [ string, string, string ][];
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

type HashTestData = {
    count: number,
    hands: [ string, string, string ][];
}

export async function packHashTestData() {
    const dataIn = json5.parse(await Deno.readTextFile(`${sdir()}/json/hash_tests.json5`));
    const dataOut: HashTestData = {
        count: dataIn.length,
        hands: [],
    }

    for (let i = 0; i < dataIn.length; i += 1) {
        const d = dataIn[i];
        dataOut.hands.push([ d[0], d[1], d[2] ]);
    }
    // console.log(dataOut);

    const pack = mp.encode(dataOut);
    await Deno.writeFile(`${sdir()}/bin/hash_tests.msgpack`, pack);
    console.log("hash test data packed");
}

if (import.meta.main) {
    await packHashTestData();
}
