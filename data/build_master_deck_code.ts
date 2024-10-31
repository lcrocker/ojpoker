#!/usr/bin/env -S deno run --allow-read=./json --allow-write=../dart/lib/src/cards,../rust/src/cards build_master_deck_code.ts

/**
 * @file build_master_deck_code.ts
 * @brief Build Dart and Rust code for MasterDeck object from JSON data.
 * 
 * The amount of data in all the master decks is small enough that it
 * makes sense to just include it in all the executables rather than
 * reading it from a file, which would make the library more awkward to
 * use. But it's nice to have this data in JSON that can be easily added
 * to and edited as new games are added. So, to keep the code up to date
 * and in sync between languages, we use this script to build the code
 * from the JSON file. We also check in the generated code so that new
 * users of the library don't have to do this unless they make a change.
 * 
 * JSON5 input is array of deck info objects:
 * 
 * [ {
 * "name": "english",
 * "dups_allowed": false,
 * "low_aces": false,
 * "aliases":["default","french","poker","bridge","52","deucetoseven",
 * "tienlen","gin","spades","hearts"],
 * "card_list":[8,9,10,11, 12,13,14,15, 16,17,18,19, 20,21,22,23,
 * 24,25,26,27, 28,29,30,31, 32,33,34,35, 36,37,38,39, 40,41,42,43,
 * 44,45,46,47, 48,49,50,51, 52,53,54,55, 56,57,58,59]
 * }, . . . ]
 * 
 * Output is code for MasterDeck object and its static data.
 */

import { parse as jsonParse } from "jsr:@std/jsonc";

let SCRIPT_DIR: string | undefined = undefined;

function sdir(): string {
    if (SCRIPT_DIR !== undefined) return SCRIPT_DIR;
    SCRIPT_DIR = import.meta.dirname;
    if (SCRIPT_DIR === undefined) SCRIPT_DIR = ".";
    return SCRIPT_DIR;
}

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

const DART_CARD_NAMES = [ "",
  "WhiteJoker", "BlackJoker", "Joker",
  "LowAceOfClubs", "LowAceOfDiamonds", "LowAceOfHearts", "LowAceOfSpades",
  "DeuceOfClubs", "DeuceOfDiamonds", "DeuceOfHearts", "DeuceOfSpades",
  "TreyOfClubs", "TreyOfDiamonds", "TreyOfHearts", "TreyOfSpades",
  "FourOfClubs", "FourOfDiamonds", "FourOfHearts", "FourOfSpades",
  "FiveOfClubs", "FiveOfDiamonds", "FiveOfHearts", "FiveOfSpades",
  "SixOfClubs", "SixOfDiamonds", "SixOfHearts", "SixOfSpades",
  "SevenOfClubs", "SevenOfDiamonds", "SevenOfHearts", "SevenOfSpades",
  "EightOfClubs", "EightOfDiamonds", "EightOfHearts", "EightOfSpades",
  "NineOfClubs", "NineOfDiamonds", "NineOfHearts", "NineOfSpades",
  "TenOfClubs", "TenOfDiamonds", "TenOfHearts", "TenOfSpades",
  "JackOfClubs", "JackOfDiamonds", "JackOfHearts", "JackOfSpades",
  "KnightOfClubs", "KnightOfDiamonds", "KnightOfHearts", "KnightOfSpades",
  "QueenOfClubs", "QueenOfDiamonds", "QueenOfHearts", "QueenOfSpades",
  "KingOfClubs", "KingOfDiamonds", "KingOfHearts", "KingOfSpades",
  "AceOfClubs", "AceOfDiamonds", "AceOfHearts", "AceOfSpades", ];

export async function buildMasterDeckDart() {
    const deckDataIn = jsonParse(await
        Deno.readTextFile(`${sdir()}/json/master_decks.jsonc`));
    if (! Array.isArray(deckDataIn)) {
        throw new Error("Invalid master_decks.jsonc");
    }
    const enc = new TextEncoder();
    const f = await Deno.open(`${sdir()}/../dart/lib/src/cards/master_deck.dart`, {
        write: true, create: true, truncate: true
    });
    await f.write(enc.encode(
`// Do not edit: File generated with build_master_deck_code.ts
import 'cards.dart';

/// # MasterDeck | [wiki](https://github.com/lcrocker/ojpoker/wiki/MasterDeck)
/// A static object that describes the properties of a new deck of cards for a
/// certain game. 
///
/// For example, the "English" master deck has 52 cards, no jokers, aces are
/// high, and no duplicate cards are allowed. The "Canasta" deck has 108 cards
/// including jokers and duplicates. Since this is all unchanging information,
/// \`MasterDeck.byName()\` just returns a pointer to an existing static object
/// based on the name you pass in.
/// {@category cards}
class MasterDeck {
    final String name;
    final int cardSet;
    final List<Card> cardList;
    final bool dupsAllowed;
    final bool lowAces;

    const MasterDeck._(this.name, this.cardSet, this.cardList,
        this.dupsAllowed, this.lowAces);

    /// Get a MasterDeck object by name or alias.
    factory MasterDeck.byName(String dname) {
        int id = aliases[dname]!;
        return decks[id - 1];
    }

    factory MasterDeck.byIndex(int idx) {
        return decks[idx - 1];
    }

    static const deckCount = ${deckDataIn.length};

    int get size => cardList.length;

    /// Does this deck contain the given card?
    bool has(Card c) { return 0 != (cardSet & (1 << c.index)); }

    @override
    String toString() {
        int len = cardList.length;
        String ret = "\$name deck: \${lowAces ? 'LA' : 'HA'} "
        "\${dupsAllowed ? 'DY' : 'DN'} "
        "\${cardList[len-1]}\${cardList[len-2]}\${cardList[len-3]}\${cardList[len-4]}"
        "(+\${cardList.length - 4})...";
        return ret;
    }
`));

    const aliases: Record<string, number> = {};
    for (let i = 0; i < deckDataIn.length; i += 1) {
        const dd = deckDataIn[i];
        if (! isDeckInfo(dd)) {
            throw new Error("bad deck data");
        }
        for (let j = 0; j < dd.aliases.length; j += 1) {
            aliases[dd.aliases[j]] = i + 1;
        }
        aliases[dd.name] = i + 1;
    }
    await f.write(enc.encode(
`
    static const Map<String, int> aliases = {
`));
    for (const [key, value] of Object.entries(aliases)) {
        await f.write(enc.encode(`        "${key}": ${value},\n`));
    }

    await f.write(enc.encode(
`    };

    static const List<MasterDeck> decks = [
`));
    for (let i = 0; i < deckDataIn.length; i += 1) {
        const dd = deckDataIn[i];
        if (! isDeckInfo(dd)) {
            throw new Error("bad deck data");
        }
        let cset: bigint = 0n;
        for (let j = 0; j < dd.card_list.length; j += 1) {
            cset |= (1n << BigInt(dd.card_list[j]));
        }
        await f.write(enc.encode(
`        MasterDeck._("${dd.name}",
            0x${cset.toString(16)},
            [
`));
            const cards = dd.card_list;
            for (let j = cards.length - 1; j >= 0; j -= 1) {
                await f.write(enc.encode(
`                Card.${DART_CARD_NAMES[cards[j]]},
`));
            }
            await f.write(enc.encode(
`            ],
            ${dd.dups_allowed},
            ${dd.low_aces}),\n`));
    }
    await f.write(enc.encode(
`    ];
}
`));
    console.log("built master_deck.dart");
}

export async function buildMasterDeckRust() {
    const deckDataIn = jsonParse(await
        Deno.readTextFile(`${sdir()}/json/master_decks.jsonc`));
    if (! Array.isArray(deckDataIn)) {
        throw new Error("bad input file master_decks.jsonc");
    }
    const enc = new TextEncoder();
    const f = await Deno.open(`${sdir()}/../rust/src/cards/master_deck.rs`, {
        write: true, create: true, truncate: true
    });
    await f.write(enc.encode(
`// Do not edit: File generated with build_master_deck_code.ts
//! [wiki](https://github.com/lcrocker/ojpoker/wiki/MasterDeck) | Represents a new, full deck

use crate::cards::*;

/// [wiki](https://github.com/lcrocker/tspoker/wiki/MasterDeck) | A new full deck of cards
///
/// A static object that describes the properties of a new deck of cards for a
/// certain game or set of games.
/// For example, the "English" master deck has 52 cards, no jokers, aces are high,
/// and no duplicate cards are allowed.
/// The "Canasta" deck has 108 cards including jokers and duplicates.
///
/// Since this is all unchanging information, \`MasterDeck::new()\`
/// just returns a pointer to an existing static object based on the name you
/// pass in.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MasterDeck {
    /// Canonical name of deck
    pub name: &'static str,
    /// Bitset of cards in deck for quick lookup
    pub card_set: u64,
    /// List of cards in full deck
    pub card_list: &'static [Card],
    /// Are duplicate cards allowed?
    pub dups_allowed: bool,
    /// Are aces low?
    pub low_aces: bool,
}

impl MasterDeck {
    /// Retrieve pointer to [MasterDeck] by name (or alias).
    pub fn by_name(dname: &str) -> &'static Self { masterdeck_by_name(dname) }

    /// Does this deck contain the given card?
    pub fn has(&self, c: Card) -> bool { 0 != (self.card_set & (1 << c.0)) }

    /// How many cards in full deck?
    pub fn size(&self) -> usize { self.card_list.len() }
}

impl core::fmt::Debug for MasterDeck {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("MasterDeck")
            .field("name", &format_args!("{}", self.name))
            .field("card_set", &format_args!("0x{:X}", self.card_set))
            .field("card_list", &(self.card_list.len()))
            .field("dups", &format_args!("{}", if self.dups_allowed { "Yes" } else { "No" }))
            .field("aces", &format_args!("{}", if self.low_aces { "Low" } else { "High" }))
            .finish()
    }
}

impl std::fmt::Display for MasterDeck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{} ({})", self.name.chars().next().unwrap().to_uppercase(),
            self.name.chars().skip(1).collect::<String>(),
        self.card_list.len())
    }
}

fn masterdeck_by_name(alias: &str) -> &'static MasterDeck {
    match &alias.to_lowercase()[..] {
`));
    for (let i = 0; i < deckDataIn.length; i += 1) {
        const dd = deckDataIn[i];
        if (! isDeckInfo(dd)) {
            throw new Error("bad input file");
        }
        const aliases: string[] = [ `"${dd.name}"` ];

        for (let j = 0; j < dd.aliases.length; j += 1) {
            aliases.push(`"${dd.aliases[j]}"`);
        }
        f.write(enc.encode(
`        ${aliases.join(" | ")} => &DECK_INFO[${i}],
`));
    }
    f.write(enc.encode(
`        _ => &DECK_INFO[0],
    }
}

/// Retrieve pointer to [MasterDeck] by index (1-based)
pub fn masterdeck_by_index(idx: usize) -> &'static MasterDeck {
    &DECK_INFO[idx - 1]
}
/// How many decks are there?
pub fn deck_count() -> usize { DECK_INFO.len() }

macro_rules! masterdeck {
    ( $name:literal, $set:literal, $list:expr,
        $d:literal, $la:literal ) => {
        MasterDeck {
            name: $name,
            card_set: $set,
            card_list: $list,
            dups_allowed: $d,
            low_aces: $la,
        }
    };
}

const DECK_INFO: [MasterDeck; ${deckDataIn.length}] = [
`));
    for (let i = 0; i < deckDataIn.length; i += 1) {
        const dd = deckDataIn[i];
        if (! isDeckInfo(dd)) {
            throw new Error("bad input file");
        }
        let cset: bigint = 0n;
        for (let j = 0; j < dd.card_list.length; j += 1) {
            cset |= (1n << BigInt(dd.card_list[j]));
        }
        const lname = `${dd.name}_cards`.toLocaleUpperCase();

        f.write(enc.encode(
`    masterdeck!("${dd.name}",
         0x${cset.toString(16)},
         &${lname},
         ${dd.dups_allowed},
         ${dd.low_aces}),
`));
    }
    f.write(enc.encode(
`];

macro_rules! card_array {
    ( $( $x:expr ),* ) => {
        [
            $(
                Card($x)
            ),*
        ]
    };
}

`));
    for (let i = 0; i < deckDataIn.length; i += 1) {
        const dd = deckDataIn[i];
        if (! isDeckInfo(dd)) {
            throw new Error("bad input file");
        }
        const lname = `${dd.name}_cards`.toLocaleUpperCase();
        f.write(enc.encode(
`const ${lname}: [Card; ${dd.card_list.length}] =
card_array!(${dd.card_list.reverse().join(",")});

`));
    }
    console.log("built master_deck.rs");
}

export function buildMasterDeckAll() {
    return Promise.all([
        buildMasterDeckDart(),
        buildMasterDeckRust(),
    ]);
}

/*
 *
 */

if (import.meta.main) {
    await buildMasterDeckAll();
}
