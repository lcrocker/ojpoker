
use crate::cards::*;
use crate::poker::*;

/// Return the result of scanning the hand for information necessary
/// for replacing the bug, if present.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct BugScanResult {
    /// Index of the bug card in the hand
    pub index: u8,
    /// Card to replace it with
    pub replacement: Card
}

/// Scan a hand for the bug, and create some handy bitmaps along the way
pub fn ojp_bug_scan(h: &Hand, g: Scale) -> Option<BugScanResult> {
    debug_assert!(5 == h.len());
    debug_assert!(g.deck_type().has(JOKER));

    let mut rank_mask: u16 = 0;
    let mut suit_maps: [u16; 4] = [0; 4];
    let mut joker_index: i8 = -1;

    #[allow(unused_assignments)]
    let mut needed_rank = Rank::None;
    let mut needed_suit = Suit::None;

    for i in 0..h.len() {
        if h[i].is_joker() {
            joker_index = i as i8;
            rank_mask |= 1;
            suit_maps[0] |= 1;
            suit_maps[1] |= 1;
            suit_maps[2] |= 1;
            suit_maps[3] |= 1;
            continue;
        }
        rank_mask |= 1 << h[i].rank() as u8;
        suit_maps[h[i].suit() as usize - 1] |= 1 << h[i].rank() as u8;

        if suit_maps[h[i].suit() as usize - 1].count_ones() == 5 {
            needed_suit = h[i].suit();
        }
    }
    if -1 == joker_index {
        return None;
    }

    // Find lowest rank not already present in hand
    if Scale::AceToFiveBug == g {
        rank_mask >>= 1;
        needed_rank = Rank::LowAce;

        loop {
            if 0 == (rank_mask & 1) {   // Will always happen before 16 loops
                break;
            }
            needed_rank = Rank::from_u8(needed_rank as u8 + 1);
            rank_mask >>= 1;
        }
        return Some(BugScanResult {
            index: joker_index as u8,
            replacement: Card::from_rank_suit(needed_rank, Suit::Spade),
        });
    }
    // Handle the 2-3-4-5-Jk case: replace with 6 in poker, A in pai gow
    if Scale::PaiGow == g && 0b111101 == rank_mask {
        needed_rank = Rank::Ace;
    } else {
        needed_rank = Rank::from_u8(HIGH_BUG_STRAIGHT_PATTERNS[
            0x3F & ojh_uhash_32(rank_mask as u32, 0xBCDE_F1A5) as usize]);
    }
    // Fill straight flush
    if needed_rank != Rank::None && needed_suit != Suit::None {
        return Some(BugScanResult {
            index: joker_index as u8,
            replacement: Card::from_rank_suit(needed_rank, needed_suit),
        });
    }
    // Flush: find highest rank of that suit not already present in
    if needed_suit != Suit::None {
        let mut ranks = suit_maps[needed_suit as usize - 1];
        if g.low_aces() {
            ranks <<= 1;
            needed_rank = Rank::King;
        } else {
            needed_rank = Rank::Ace;
        }
        loop {
            if 0 == (ranks & 0b1000_0000_0000_0000) {
                break;
            }
            needed_rank = Rank::from_u8(needed_rank as u8 - 1);
            ranks <<= 1;
        }
        return Some(BugScanResult {
            index: joker_index as u8,
            replacement: Card::from_rank_suit(needed_rank, needed_suit),
        });
    }
    // Fill straight
    if needed_rank != Rank::None {
        return Some(BugScanResult {
            index: joker_index as u8,
            replacement: Card::from_rank_suit(needed_rank, Suit::Spade),
        });
    }
    // No straight or flush. Find an ace we don't have. We don't actually
    // test the final case, so A-A-A-A-Jk will have two aces of clubs.
    if 0 == suit_maps[3] & 0b1000_0000_0000_0010 {
        return Some(BugScanResult {
            index: joker_index as u8,
            replacement: Card::from_rank_suit(Rank::Ace, Suit::Spade),
        });
    }
    if 0 == suit_maps[2] & 0b1000_0000_0000_0010 {
        return Some(BugScanResult {
            index: joker_index as u8,
            replacement: Card::from_rank_suit(Rank::Ace, Suit::Heart),
        });
    }
    if 0 == suit_maps[1] & 0b1000_0000_0000_0010 {
        return Some(BugScanResult {
            index: joker_index as u8,
            replacement: Card::from_rank_suit(Rank::Ace, Suit::Diamond),
        });
    }
    Some(BugScanResult {
        index: joker_index as u8,
        replacement: Card::from_rank_suit(Rank::Ace, Suit::Club),
    })
}

// Perfect hash map from 5-bit rank mask to replacement rank.
// Indexed by 0x3F & ojh_uhash_32(bits, 0x0xBCDE_F1A5)
const HIGH_BUG_STRAIGHT_PATTERNS: [u8; 64] = [
    0,
    10, // 0b0110101000000001
    0,
    9, // 0b0010110100000001
    10, // 0b0000001111000001
    0, 0, 0, 0,
    9, // 0b0000010111000001
    14, // 0b0010111000000001
    0,
    7, // 0b0000000001111001
    8, // 0b0000011011000001
    9, // 0b0000000111100001
    7, // 0b0000000101110001
    0, 0,
    6, // 0b0000001110100001
    11, // 0b0010011100000001
    0,
    7, // 0b0000011101000001
    7, // 0b0000001101100001
    0, 0, 0,
    11, // 0b0110011000000001
    0, 0,
    6, // 0b0000000110110001
    6, // 0b0000000010111001
    0,
    15, // 0b0110110000000001
    0, 0, 0, 0, 0, 0,
    9, // 0b0000110110000001
    0,
    11, // 0b1110010000000001
    11, // 0b0000011110000001
    6, // 0b0000000000111101
    0, 0,
    13, // 0b1100110000000001
    10, // 0b0010101100000001
    8, // 0b0000111010000001
    13, // 0b0100111000000001
    10, // 0b1110100000000001
    0,
    8, // 0b0000001011100001
    10, // 0b0000101110000001
    0, 0,
    13, // 0b0000111100000001
    8, // 0b0000000011110001
    0, 0,
    14, // 0b1010110000000001
    0, 0, 0,
];

