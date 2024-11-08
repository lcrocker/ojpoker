#!/usr/bin/env -S deno run --allow-all build_docs_all.ts

/*
 * Build docs and copy to server directories.
 */

import { copy } from "jsr:@std/fs";
import { exists } from "jsr:@std/fs/exists";

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

async function deleteDocs() {
    const d2 = new Deno.Command("rm", {
        args: ["-rf", `${sdir()}/docs/rust`],
        cwd: sdir(),
    }).spawn();
    const d4 = new Deno.Command("rm", {
        args: ["-rf", `${sdir()}/rust/target/doc/onejoker/*`],
        cwd: sdir(),
    }).spawn();

    await Promise.all([
        d2.status.then(() => console.log("removed rust docs deployed")),
        d4.status.then(() => console.log("removed rust docs built")),
    ]);
}

async function buildDocsRust() {
    const c1 = new Deno.Command("cargo", {
        args: ["doc", "--no-deps"],
        cwd: `${sdir()}/rust`,
    }).spawn();

    await c1.status.then(() => { console.log("built rust docs"); });
    await copy(`${sdir()}/rust/target/doc/onejoker`, `${sdir()}/docs/rust`)
        .then(() => { console.log("copied rust docs");
    });
}

export async function buildDocsAll() {
    await deleteDocs();
    await Promise.all([
        buildDocsRust(),
    ]);
}

if (import.meta.main) {
    buildDocsAll().then(() => { console.log("done with docs"); });
}
