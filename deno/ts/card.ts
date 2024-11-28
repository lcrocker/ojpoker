/**
 * @file cards.ts
 */

import * as Suit from "./suit.ts";
import * as Rank from "./rank.ts";

export type T = number;

export const None = 0;
export const WhiteJoker = 1;
export const RedJoker = 2;
export const Joker = 3;
export const LowAceOfClubs = 4;
export const LowAceOfDiamonds = 5;
export const LowAceOfHearts = 6;
export const LowAceOfSpades = 7;
export const DeuceOfClubs = 8;
export const DeuceOfDiamonds = 9;
export const DeuceOfHearts = 10;
export const DeuceOfSpades = 11;
export const TreyOfClubs = 12;
export const TreyOfDiamonds = 13;
export const TreyOfHearts = 14;
export const TreyOfSpades = 15;
export const FourOfClubs = 16;
export const FourOfDiamonds = 17;
export const FourOfHearts = 18;
export const FourOfSpades = 19;
export const FiveOfClubs = 20;
export const FiveOfDiamonds = 21;
export const FiveOfHearts = 22;
export const FiveOfSpades = 23;
export const SixOfClubs = 24;
export const SixOfDiamonds = 25;
export const SixOfHearts = 26;
export const SixOfSpades = 27;
export const SevenOfClubs = 28;
export const SevenOfDiamonds = 29;
export const SevenOfHearts = 30;
export const SevenOfSpades = 31;
export const EightOfClubs = 32;
export const EightOfDiamonds = 33;
export const EightOfHearts = 34;
export const EightOfSpades = 35;
export const NineOfClubs = 36;
export const NineOfDiamonds = 37;
export const NineOfHearts = 38;
export const NineOfSpades = 39;
export const TenOfClubs = 40;
export const TenOfDiamonds = 41;
export const TenOfHearts = 42;
export const TenOfSpades = 43;
export const JackOfClubs = 44;
export const JackOfDiamonds = 45;
export const JackOfHearts = 46;
export const JackOfSpades = 47;
export const KnightOfClubs = 48;
export const KnightOfDiamonds = 49;
export const KnightOfHearts = 50;
export const KnightOfSpades = 51;
export const QueenOfClubs = 52;
export const QueenOfDiamonds = 53;
export const QueenOfHearts = 54;
export const QueenOfSpades = 55;
export const KingOfClubs = 56;
export const KingOfDiamonds = 57;
export const KingOfHearts = 58;
export const KingOfSpades = 59;
export const AceOfClubs = 60;
export const AceOfDiamonds = 61;
export const AceOfHearts = 62;
export const AceOfSpades = 63;

export function suitOf(n: T): Suit.T {
    if (n < LowAceOfClubs || n > KnightOfSpades) return Suit.None;
    return (n & 3) + 1 as Suit.T;
}

export function rankOf(n: T): Rank.T {
    if (n < LowAceOfClubs || n > KnightOfSpades) return Rank.None;
    return (n >> 2) as Rank.T;
}

// Always return high aces. Good for indexing external tables, etc.
export function highRankOf(n: T): Rank.T {
    if (n < LowAceOfClubs || n > KnightOfSpades) return Rank.None;
    if (n < DeuceOfClubs) return Rank.Ace;
    return (n >> 2) as Rank.T;
}

export function fromRankSuit(r: Rank.T, s: Suit.T): T {
    if (r < 1 || r > 15) return None;
    if (s < 1 || s > 4) return None;
    return ((r << 2) | (s - 1)) as T;
}

/**
 * These are used internally for ensuring that the right values get into
 * hands from the chosen deck. You should need them as a library user.
 */
export function lowAceFix(n: T): T {
    return ((n >= 56 && n <= 59) ? n - 52 : n) as T;
}

export function highAceFix(n: T): T {
    return ((n >= 4 && n <= 7) ? n + 52 : n) as T;
}

/**
 * Various card classifiers.
 */
export function isCard(n: T): boolean {
    return n >= LowAceOfClubs && n <= AceOfSpades;
}

export function isAce(n: T): boolean {
    return (n >= LowAceOfClubs && n <= LowAceOfSpades) ||
        (n >= AceOfClubs && n <= AceOfSpades);
}

export function isJoker(n: T): boolean {
    return n >= WhiteJoker && n <= Joker;
}

export function isRed(n: T): boolean {
    if (n === RedJoker) return true;
    const s = suitOf(n);
    return Suit.Diamond === s || Suit.Heart === s;
}

export function isBlack(n: T): boolean {
    if (n === Joker) return true;
    const s = suitOf(n);
    return Suit.Club === s || Suit.Spade === s;
}

/**
 * Single-card text IO functions. Later used by cardlist for
 * reading/writing whole hands.
 */
function commonTextOf(n: T, u: boolean): string {
    if (! isCard(n)) return "??";
    if (n === Joker) return "Jk";
    else if (n === RedJoker) return "Jr";
    else if (n === WhiteJoker) return "Jw";

    const r: Rank.T = rankOf(n);
    const s: Suit.T = suitOf(n);
    if (u) return `${Rank.charOf(r)}${Suit.symbolOf(s)}`;
    return `${Rank.charOf(r)}${Suit.charOf(s)}`;
}

export function textOf(n: T): string {
    return commonTextOf(n, false);
}

export function unicodeOf(n: T): string {
    return commonTextOf(n, true);
}

export function unicodeSingleOf(n: number): string {
    if (n < 0 || n > KnightOfSpades) n = 0;
    return [
        "⁇","🃟","🂿","🃏","🃑","🃁","🂱","🂡","🃒","🃂","🂲","🂢","🃓","🃃","🂳","🂣",
        "🃔","🃄","🂴","🂤","🃕","🃅","🂵","🂥","🃖","🃆","🂶","🂦","🃗","🃇","🂷","🂧",
        "🃘","🃈","🂸","🂨","🃙","🃉","🂹","🂩","🃚","🃊","🂺","🂪","🃛","🃋","🂻","🂫",
        "🃝","🃍","🂽","🂭","🃞","🃎","🂾","🂮","🃑","🃁","🂱","🂡","🃜","🃌","🂼","🃜",
    ][n];
}

export function fullNameOf(n: T): string {
    if (! isCard(n)) return "unknown";
    else if (n === Joker) return "joker";
    else if (n === RedJoker) return "red joker";
    else if (n === WhiteJoker) return "white joker";

    const r: Rank.T = rankOf(n);
    const s: Suit.T = suitOf(n);
    return `${Rank.nameOf(r)} of ${Suit.pluralOf(s)}`;
}

export function fromString(s: string): T {
    const [ c, _n ] = parseOne(s, 0);
    return c;
}

/**
 * Simple state machine to parse one card from a piece of text, and give us a
 * pointer past what was read for iterative use.
 */
export function parseOne(text: string, start: number): [T, number] {
    let rank: Rank.T = Rank.None;
    let suit: Suit.T = Suit.None;
    let state: number = 0;
    let done: boolean = false;

    let i = start;
    while (i < text.length) {
        const c = text.charAt(i);

        switch (state) {
        case 0:
            if (" " === c || '[' === c) { break; }
            else if ('J' == c) { state = 2; }
            else {
                rank = Rank.fromChar(c);
                if (rank === Rank.None) return [ None, i ];
                state = 1;
            }
            break;
        case 1:
            suit = Suit.fromChar(c);
            done = true;
            break;
        case 2:
            if ('k' === c) return [ Joker, i + 1 ];
            else if ('r' === c) return [ RedJoker, i + 1 ];
            else if ('w' === c) return [ WhiteJoker, i + 1 ];
            else {
                rank = Rank.Jack;
                suit = Suit.fromChar(c);
                done = true;
            }
            break;
        default:
            throw new Error(`internal`);
        }
        if (done) break;
        i += 1;
    }
    if (suit === Suit.None) return [ None, i ];
    return [ fromRankSuit(rank, suit), i + 1 ];
}

