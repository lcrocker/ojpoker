/**
 * @file suit.ts
 * @brief Implement card suit methods
 * @author Lee Daniel Crocker <lee@piclab.com>
 * @copyright https://creativecommons.org/publicdomain/zero/1.0/
 * https://onejoker.org/
 * 
 * This is included in the "cards.ts" import collector.
 * If you need to import it separately, use:
 * 
 * import * as Suit from "suit.ts";
 */

/**
 * Suit.T is used as a suit type, though TypeScript doesn't
 * strictly type check enums and numeric types.
 */
export type T = number;

export const None: T = 0;
export const Club: T = 1;
export const Diamond: T = 2;
export const Heart: T = 3;
export const Spade: T = 4;

export function nameOf(n: T): string {
    if (n < 0 || n > 4) n = 0;
    return [ "?", "club", "diamond", "heart", "spade" ][n];
}

export function pluralOf(n: T): string {
    if (n < 0 || n > 4) n = 0;
    return [ "?", "clubs", "diamonds", "hearts", "spades" ][n];
}

export function articleOf(_n: T): string { return "a"; }

/**
 * Functions for I/O in common text format.
 */
export function charOf(n: number): string {
    if (n < 0 || n > 4) n = 0;
    return [ "?", "c", "d", "h", "s" ][n];
}

/**
 * These are Unicode suit symbols if your editor/screen can deal with them.
 * For output only.
 */
export function symbolOf(n: T): string {
    if (n < 0 || n > 4) n = 0;
    return [ "?", "♣", "♦", "♥", "♠" ][n];
}

export function fromChar(s: string): T {
    switch (s) {
        case "c": case "♣": return Club;
        case "d": case "♦": return Diamond;
        case "h": case "♥": return Heart;
        case "s": case "♠": return Spade;
        default: return None;
    }
}

