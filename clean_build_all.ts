#!/usr/bin/env -S deno run --allow-all clean_build_all.ts

/**
 * @file clean_build_all.ts
 * @brief Remove old generated files and build everything from scratch.
 */

import { buildDocsAll } from "./build_docs_all.ts";

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

if (import.meta.main) {
    await buildDocsAll();
}

