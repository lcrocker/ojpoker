
use crate::prelude::*;

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

// No straight or flush. Find an ace we don't have. We don't actually
// test the final case, so A-A-A-A-Jk will have two aces of clubs.
fn first_ace(mask: u8, index: i8) -> Option<BugScanResult> {
    if 0 == mask & 0b1000 {
        return Some(BugScanResult {
            index: index as u8,
            replacement: Card::from_rank_suit(Rank::Ace, Suit::Spade),
        });
    }
    if 0 == mask & 0b0100 {
        return Some(BugScanResult {
            index: index as u8,
            replacement: Card::from_rank_suit(Rank::Ace, Suit::Heart),
        });
    }
    if 0 == mask & 0b0010 {
        return Some(BugScanResult {
            index: index as u8,
            replacement: Card::from_rank_suit(Rank::Ace, Suit::Diamond),
        });
    }
    Some(BugScanResult {
        index: index as u8,
        replacement: Card::from_rank_suit(Rank::Ace, Suit::Club),
    })
}

/// Scan incomplete hand for a bug,
pub fn ojp_bug_scan_p_1(h: &Hand, g: Scale) -> Option<BugScanResult> {
    debug_assert!(h.len() < 5);
    debug_assert!(g.deck_type().has(JOKER));

    let mut ace_mask: u8 = 0;
    let mut joker_index: i8 = -1;

    for i in 0..h.len() {
        if h[i].is_ace() {
            ace_mask |= 1 << (3 & h[i].0);
        } else if h[i].is_joker() {
            joker_index = i as i8;
        }
    }
    if -1 == joker_index {
        return None;
    }
    first_ace(ace_mask, joker_index)
}

/// Scan a five-card hand for one bug
pub fn ojp_bug_scan_5_1(h: &Hand, g: Scale) -> Option<BugScanResult> {
    debug_assert!(5 == h.len());
    debug_assert!(g.deck_type().has(JOKER));

    let mut rank_mask: u16 = 0;
    let mut ace_mask: u8 = 0;
    let mut suit_counts: [u16; 4] = [0; 4];
    let mut joker_index: i8 = -1;

    #[allow(unused_assignments)]
    let mut needed_rank = Rank::None;
    let mut needed_suit = Suit::None;

    for i in 0..5 {
        let r = 0x0F & (h[i].0 >> 2);
        rank_mask |= 1 << r;

        if 0 == r {
            joker_index = i as i8;
        } else {
            let s = 0x03 & h[i].0;
            suit_counts[s as usize] += 1;
            if 4 == suit_counts[s as usize] {
                needed_suit = Suit::from_u8(s + 1);
            }
            if 1 == r || 15 == r {
                ace_mask |= 1 << s;
            }
        }
    }
    if -1 == joker_index {
        return None;
    }
    // Find lowest rank not already present in hand
    if Scale::AceToFiveBug == g {
        rank_mask >>= 1;
        needed_rank = Rank::LowAce;

        let mut loop_guard = 20;
        loop {
            loop_guard -= 1;
            assert!(loop_guard > 0);

            if 0 == (rank_mask & 1) {
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
    // Handle the 2-3-4-5-Jk case: replace with A in pai gow, 6 in other games
    if Scale::PaiGow == g && 0b111101 == rank_mask {
        needed_rank = Rank::Ace;
    } else {
        needed_rank = straight_rank_needed_bsearch(rank_mask);
    }
    // Fill straight flush
    if needed_rank != Rank::None && needed_suit != Suit::None {
        return Some(BugScanResult {
            index: joker_index as u8,
            replacement: Card::from_rank_suit(needed_rank, needed_suit),
        });
    }
    // Flush: find highest rank not already present in hand
    if needed_suit != Suit::None {
        if g.low_aces() {
            rank_mask <<= 1;
            needed_rank = Rank::King;
        } else {
            needed_rank = Rank::Ace;
        }
        let mut loop_guard = 20;
        loop {
            loop_guard -= 1;
            assert!(loop_guard > 0);

            if 0 == (rank_mask & 0b1000_0000_0000_0000) {
                break;
            }
            needed_rank = Rank::from_u8(needed_rank as u8 - 1);
            rank_mask <<= 1;
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
    first_ace(ace_mask, joker_index)
}

// For five-card hands, the rank mask must exactly equal one of the
// target bit patterns, so we can use a binary search.
fn straight_rank_needed_bsearch(rank_mask: u16) -> Rank {
    let mut start: usize = 0;
    let mut end: usize = HIGH_BUG_STRAIGHT_PATTERNS.len();

    let mut loop_guard = 10;
    loop {
        loop_guard -= 1;
        assert!(loop_guard > 0);

        let mid = (start + end) / 2;
        let p = HIGH_BUG_STRAIGHT_PATTERNS[mid];
        if p.0 == rank_mask {
            return p.1;
        }
        if p.0 < rank_mask {
            start = mid + 1;
        } else {
            end = mid;
        }
        if end <= start {
            break;
        }
    }
    Rank::None
}

const HIGH_BUG_STRAIGHT_PATTERNS: [(u16, Rank); 41] = [
    (0b0000000000111101, Rank::Six),
    (0b0000000001011101, Rank::Five),
    (0b0000000001101101, Rank::Four),
    (0b0000000001110101, Rank::Trey),
    (0b0000000001111001, Rank::Seven),
    (0b0000000010111001, Rank::Six),
    (0b0000000011011001, Rank::Five),
    (0b0000000011101001, Rank::Four),
    (0b0000000011110001, Rank::Eight),
    (0b0000000101110001, Rank::Seven),
    (0b0000000110110001, Rank::Six),
    (0b0000000111010001, Rank::Five),
    (0b0000000111100001, Rank::Nine),
    (0b0000001011100001, Rank::Eight),
    (0b0000001101100001, Rank::Seven),
    (0b0000001110100001, Rank::Six),
    (0b0000001111000001, Rank::Ten),
    (0b0000010111000001, Rank::Nine),
    (0b0000011011000001, Rank::Eight),
    (0b0000011101000001, Rank::Seven),
    (0b0000011110000001, Rank::Jack),
    (0b0000101110000001, Rank::Ten),
    (0b0000110110000001, Rank::Nine),
    (0b0000111010000001, Rank::Eight),
    (0b0000111100000001, Rank::Queen),
    (0b0010011100000001, Rank::Jack),
    (0b0010101100000001, Rank::Ten),
    (0b0010110100000001, Rank::Nine),
    (0b0010111000000001, Rank::King),
    (0b0100111000000001, Rank::Queen),
    (0b0110011000000001, Rank::Jack),
    (0b0110101000000001, Rank::Ten),
    (0b0110110000000001, Rank::Ace),
    (0b1000000000011101, Rank::Five),
    (0b1000000000101101, Rank::Four),
    (0b1000000000110101, Rank::Trey),
    (0b1000000000111001, Rank::Deuce),
    (0b1010110000000001, Rank::King),
    (0b1100110000000001, Rank::Queen),
    (0b1110010000000001, Rank::Jack),
    (0b1110100000000001, Rank::Ten),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_table() -> OjResult<()> {
        for i in 1..HIGH_BUG_STRAIGHT_PATTERNS.len() {
            let p = HIGH_BUG_STRAIGHT_PATTERNS[i];
            assert!(p > HIGH_BUG_STRAIGHT_PATTERNS[i - 1]);
        }
        Ok(())
    }
}


