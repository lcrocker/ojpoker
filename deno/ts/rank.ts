/**
 * @file rank.ts
 * @brief Implement rank type
 * @author Lee Daniel Crocker <lee@piclab.com>
 * @copyright https://creativecommons.org/publicdomain/zero/1.0/
 * https://onejoker.org/
 *
 * This is included in the "cards.ts" import collector.
 * If you need to import it separately, use:
 *
 * import * as Rank from "rank.ts";
 */

/**
 * As with Suit.T, Rank.T is used as a rank type, though TypeScript
 * doesn't strictly type check enums and numeric types.
 */
export type T = number;

export const None = 0;
export const LowAce = 1;
export const Deuce = 2;
export const Trey = 3;
export const Four = 4;
export const Five = 5;
export const Six = 6;
export const Seven = 7;
export const Eight = 8;
export const Nine = 9;
export const Ten = 10;
export const Jack = 11;
export const Knight = 12;
export const Queen = 13;
export const King = 14;
export const Ace = 15;

export function nameOf(n: T): string {
    if (n < 0 || n > 15) n = 0;
    return [ "?", "ace", "deuce", "trey", "four", "five", "six",
        "seven", "eight", "nine", "ten", "jack", "knight", "queen",
        "king", "ace" ][n];
}

export function pluralOf(n: T): string {
    if (n < 0 || n > 15) n = 0;
    return [ "?", "aces", "deuces", "treys", "fours", "fives",
        "sixes", "sevens", "eights", "nines", "tens", "jacks",
        "knights", "queens", "kings", "aces" ][n];
}

export function articleOf(n: T): string {
    if (n === 1 || n === 8 || n === 15) return "an";
    return "a";
}

/**
 * Functions for I/O in common text format.
 */
export function charOf(n: T): string {
    if (n < 0 || n > 15) n = 0;
    return [
        "?", "A", "2", "3", "4", "5", "6", "7", "8", "9", "T",
        "J", "C", "Q", "K", "A"
    ][n];
}

export function fromChar(s: string): T {
    // Low aces and jokers are handled at a higher level
    switch (s) {
        case "1": return LowAce;
        case "2": return Deuce;
        case "3": return Trey;
        case "4": return Four;
        case "5": return Five;
        case "6": return Six;
        case "7": return Seven;
        case "8": return Eight;
        case "9": return Nine;
        case "T": return Ten;
        case "J": return Jack;
        case "C": return Knight;
        case "Q": return Queen;
        case "K": return King;
        case "A": return Ace;
        default: return None;
    }
}
