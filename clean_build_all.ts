#!/usr/bin/env -S deno run --allow-all clean_build_all.ts

/**
 * @file clean_build_all.ts
 * @brief Remove old generated files and build everything from scratch.
 */

import { buildMasterDeckAll } from "./data/build_master_deck_code.ts";
import { packAll } from "./data/pack_all.ts";

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

function rm(fname: string): Promise<void> {
    return Deno.remove(fname).then(() => {
        console.log(`removed ${fname}`);
    }, (e) => {
        if (e.name !== "NotFound") {
            console.log(e);
        }
    });
}

async function deleteAll() {
    const promises = [];

    promises.push(rm(`${sdir()}/dart/lib/master_deck.dart`));
    promises.push(rm(`${sdir()}/rust/src/cards/master_deck.rs`));

    for await(const f of Deno.readDir(`${sdir()}/data/bin`)) {
        if (! f.isFile) continue;
        if (! f.name.match(/.*\.msgpack$/)) continue;
        promises.push(rm(`${sdir()}/data/bin/${f.name}`));
    }
    return Promise.all(promises);
}

if (import.meta.main) {
    await deleteAll();
    await buildMasterDeckAll();
    await packAll();
}
