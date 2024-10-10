#!/usr/bin/env -S deno run --allow-read=./json --allow-write=./json build_hands_text_data.ts

/**
 * @file build_hands_text_data.ts
 * @brief Build hand text io test file.
 */

import { parse as jsonParse } from "jsr:@std/jsonc";

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

const CARD_TEXT = [
    "", "Jw", "Jb", "Jk", "Ac", "Ad", "Ah", "As", "2c", "2d", "2h", "2s",
    "3c", "3d", "3h", "3s", "4c", "4d", "4h", "4s", "5c", "5d", "5h", "5s",
    "6c", "6d", "6h", "6s", "7c", "7d", "7h", "7s", "8c", "8d", "8h", "8s",
    "9c", "9d", "9h", "9s", "Tc", "Td", "Th", "Ts", "Jc", "Jd", "Jh", "Js",
    "Cc", "Cd", "Ch", "Cs", "Qc", "Qd", "Qh", "Qs", "Kc", "Kd", "Kh", "Ks",
    "Ac", "Ad", "Ah", "As"
];

type DeckInfo = {
    name: string;
    dups_allowed: boolean;
    low_aces: boolean;
    aliases: string[];
    card_list: number[];
};
function isDeckInfo(obj: unknown): obj is DeckInfo {
    if (typeof obj !== "object" || obj === null) return false;
    const o = obj as DeckInfo;
    return typeof o.name === "string" &&
        typeof o.dups_allowed === "boolean" &&
        typeof o.low_aces === "boolean" &&
        Array.isArray(o.aliases) &&
        o.aliases.every((x) => typeof x === "string") &&
        Array.isArray(o.card_list) &&
        o.card_list.every((x) => typeof x === "number");
} 

export async function buildHandsTextData() {
    const masterData = jsonParse(await
        Deno.readTextFile(`${sdir()}/json/master_decks.jsonc`));
    if (! (Array.isArray(masterData))) {
        throw new Error("Invalid master_decks.jsonc");
    }
    const f = await Deno.open(`${sdir()}/json/hands_text.jsonc`, {
        write: true, create: true, truncate: true });
    const enc = new TextEncoder();

    await f.write(enc.encode(
`// This file is built by the build_hands_text_data.ts script.
// It is 1000 random hands of various sizes and decks to test text I/O.
[
`));
    for (let i = 0; i < 1000; i += 1) {
        const deckNo = Math.floor(Math.random() * masterData.length);
        const deckInfo = masterData[deckNo];
        if (! isDeckInfo(deckInfo)) {
            throw new Error("Invalid deck info");
        }
        const handLen = 2 + Math.floor(Math.random() * Math.random() * 12);
        const listIn = [...deckInfo.card_list];
        const listOut = [];

        for (let j = 0; j < handLen; j += 1) {
            const cardNo = Math.floor(Math.random() * listIn.length);
            listOut.push(listIn[cardNo]);
            listIn.splice(cardNo, 1);
        }
        const text = listOut.map((x) => CARD_TEXT[x]).join("");

        await f.write(enc.encode(
`["${deckInfo.name}", "${text}", ${handLen}, ${quickHash(listOut)}],
`));
    }
    await f.write(enc.encode(
`]
`));
}

function quickHash(list: number[]): number {
    let h = 0x811c9dc5;
    list.forEach((x) => {
        h ^= x;
        h += (h << 1) + (h << 4) + (h << 7) + (h << 8) + (h << 24);
    });
    return h >>> 0;
}

/*
 *
 */

if (import.meta.main) {
    await buildHandsTextData();
}
