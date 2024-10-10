#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin pack_hash_tests.ts

/**
 * @file pack_hash_tests.ts
 * @brief Build msgpack file from hash_tests.jsonc
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

import { parse as jsonParse } from "jsr:@std/jsonc";
import { encode as mpEncode } from "jsr:@lambdalisue/messagepack";

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

type HashTestDataIn = [ string, string, string ];
function isHashTestDataIn(obj: unknown): obj is HashTestDataIn {
    if (! Array.isArray(obj)) return false;
    if (obj.length != 3) return false;
    if ("string" != typeof (obj[0])) return false;
    if ("string" != typeof (obj[1])) return false;
    return ("string" == typeof(obj[2]));
}
type HashTestDataOut = {
    count: number,
    hands: [ string, string, string ][];
}

export async function packHashTestData() {
    const dataIn = jsonParse(await
        Deno.readTextFile(`${sdir()}/json/hash_tests.jsonc`));
    if (! Array.isArray(dataIn)) {
        throw new Error("bad input file");
    }
    const dataOut: HashTestDataOut = {
        count: dataIn.length,
        hands: [],
    }

    for (let i = 0; i < dataIn.length; i += 1) {
        const d = dataIn[i];
        if (! isHashTestDataIn(d)) {
            throw new Error("bad input data");
        }
        dataOut.hands.push([ d[0], d[1], d[2] ]);
    }
    // console.log(dataOut);

    const pack = mpEncode(dataOut);
    await Deno.writeFile(`${sdir()}/bin/hash_tests.msgpack`, pack);
    console.log("hash test data packed");
}

if (import.meta.main) {
    await packHashTestData();
}
