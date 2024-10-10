#!/usr/bin/env -S deno run --allow-all build_docs_all.ts

/*
 * Build docs and copy to server directories.
 */

import { copy } from "jsr:@std/fs";
import { exists } from "jsr:@std/fs/exists";
import { buildMasterDeckAll } from "./data/build_master_deck_code.ts";

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

async function deleteDocs() {
    const d1 = new Deno.Command("rm", {
        args: ["-rf", `${sdir()}/docs/dart`],
        cwd: sdir(),
    }).spawn();
    const d2 = new Deno.Command("rm", {
        args: ["-rf", `${sdir()}/docs/rust`],
        cwd: sdir(),
    }).spawn();
    const d3 = new Deno.Command("rm", {
        args: ["-rf", `${sdir()}/dart/doc/api/*`],
        cwd: sdir(),
    }).spawn();
    const d4 = new Deno.Command("rm", {
        args: ["-rf", `${sdir()}/rust/target/doc/onejoker/*`],
        cwd: sdir(),
    }).spawn();

    await Promise.all([
        d1.status.then(() => console.log("removed dart docs deployed")),
        d2.status.then(() => console.log("removed rust docs deployed")),
        d3.status.then(() => console.log("removed dart docs built")),
        d4.status.then(() => console.log("removed rust docs built")),
    ]);
}

async function buildDocsDart() {
    const c1 = new Deno.Command("dart", {
        args: ["doc"],
        cwd: `${sdir()}/dart`,
    }).spawn();

    await c1.status.then(() => { console.log("built dart docs"); });
    await copy(`${sdir()}/dart/doc/api`, `${sdir()}/docs/dart`)
        .then(() => { console.log("copied dart docs"); }
    );
}

async function buildDocsRust() {
    const c1 = new Deno.Command("cargo", {
        args: ["doc"],
        cwd: `${sdir()}/rust`,
    }).spawn();

    await c1.status.then(() => { console.log("built rust docs"); });
    await copy(`${sdir()}/rust/target/doc/onejoker`, `${sdir()}/docs/rust`)
        .then(() => { console.log("copied rust docs");
    });
}

async function buildMasterDeckCodeIfNeeded() {
    const e1 = await Promise.all([
        exists(`${sdir()}/dart/lib/src/cards/master_deck.dart`),
        exists(`${sdir()}/rust/src/cards/master_deck.rs`),
    ]);
    if (! (e1[0] && e1[1])) {
        await buildMasterDeckAll();
    }
}

export async function buildDocsAll() {
    await deleteDocs();
    await buildMasterDeckCodeIfNeeded();
    await Promise.all([
        buildDocsDart(),
        buildDocsRust(),
    ]);
}

if (import.meta.main) {
    buildDocsAll().then(() => { console.log("done with docs"); });
}
