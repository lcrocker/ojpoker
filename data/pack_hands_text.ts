#!/usr/bin/env -S deno run --allow-write --allow-read

/**
 * @file pack_hands_text.ts
 * @brief Build msgpack file from hands_text.json5
 */

import * as json5 from "https://deno.land/x/json5@v1.0.0/mod.ts";
import * as mp from "https://deno.land/x/msgpack@v1.2/mod.ts";

type HandTestData = {
    count: number,
    deckNames: string[],
    hands: {
        deck: number,
        hand: string,
        len: number,
        hash: number,
    }[];
}

function run() {
    let deckNoNext = 1;
    const deckMap: Map<string, number> = new Map();
    const handDataIn = json5.parse(Deno.readTextFileSync("./json/hands_text.json5"));
    const handDataOut: HandTestData = {
        count: handDataIn.length,
        deckNames: [],
        hands: [],
    };

    for (let i = 0; i < handDataIn.length; i += 1) {
        const hd = handDataIn[i];
        let dn = deckMap.get(hd[0]);
        if (! dn) {
            dn = deckNoNext;
            deckNoNext += 1;
            deckMap.set(hd[0], dn);
            handDataOut.deckNames.push(hd[0]);
        }
        handDataOut.hands.push([ dn, hd[1], hd[2], hd[3], ]);
    }
    // console.log(handDataOut);

    const pack = mp.encode(handDataOut);
    Deno.writeFileSync("./bin/hands_text.msgpack", pack);
}

run();
