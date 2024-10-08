#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./bin pack_high_hand_tables.ts

/**
 * @file pack_high_hand_tables.ts
 * @brief Build msgpack file from high_hand_prime_hash.json5
 * 
 * JSON input is an object with two huge tables:
 * {
 *      hash_count: 2598960,
 *      eclass_count: 7462,
 *      hashes: [
 * [820441918433, 1],
 * [760189980863, 1],
 * . . .
 * ],
 *     eclasses: [
 * [1,[14,13,12,11,10]],
 * . . .
 * ]
 * }
 * 
 * Output is identical.
 * 
 * type HighHandPrimeHashes = {
 *   hash_count: number,
 *   eclass_count: number,
 *   hashes: [ number, number ][],
 *   eclasses: [ number, [number, number, number, number, number ] ][];
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

type HighHandPrimeHashes = {
    hash_count: number,
    eclass_count: number,
    hashes: [ number, number ][],
    eclasses: [ number, [number, number, number, number, number ] ][],
}

export async function packHighHandTables() {
    const dataIn = json5.parse(await Deno.readTextFile(`${sdir()}/json/high_hand_prime_hash.json5`));
    // console.log(dataIn);

    const dataOut: HighHandPrimeHashes = {
        hash_count: dataIn.hash_count,
        eclass_count: dataIn.eclass_count,
        hashes: [],
        eclasses: [],
    }

    for (let i = 0; i < dataIn.hash_count; i += 1) {
        const row = dataIn.hashes[i];
        dataOut.hashes.push(row);
    }
    for (let i = 0; i < dataIn.eclass_count; i += 1) {
        const row = dataIn.eclasses[i];
        dataOut.eclasses.push(row);
    }
    // console.log(dataOut);

    const pack = mp.encode(dataOut);
    await Deno.writeFile(`${sdir()}/bin/high_hand_prime_hash.msgpack`, pack);
    console.log("high hand tables packed");
}

if (import.meta.main) {
    await packHighHandTables();
}
