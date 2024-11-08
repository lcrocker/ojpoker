#!/usr/bin/env -S deno run --allow-all test_all.ts

/**
 * @file test_all.ts
 * @brief run all Dart and Rust tests
 */

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

async function testRust() {
    const cmd = new Deno.Command(
        "cargo", {
        args: [ "test" ],
        cwd: `${sdir()}/rust`,
        stdin: "inherit",
        stdout: "inherit",
    });
    const child = cmd.spawn();
    const st = await child.status;

    if (! st.success) {
        throw new Error("Rust tests failed");
    }
}

if (import.meta.main) {
    // Not parallelizing so I can watch the output
    await testRust();
}
