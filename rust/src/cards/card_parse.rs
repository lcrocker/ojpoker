//! [wiki](https://github.com/lcrocker/ojpoker/wiki/card_parse) | Reading cards from text

use crate::cards::{Card, Rank, Suit, JOKER, BLACK_JOKER, WHITE_JOKER};

enum CardParseState {
    Initial,
    PreCard,
    OneChar(char),
    TwoChars(char, char),
    Done,
}

struct CardParseIter<'a> {
    source: std::str::Chars<'a>,
    state: CardParseState,
}

impl<'a> CardParseIter<'a> {
    pub fn new(chars: std::str::Chars<'a>) -> Self {
        CardParseIter {
            source: chars,
            state: CardParseState::Initial,
        }
    }
}

impl<'a> Iterator for CardParseIter<'a> {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        // No loop guard here. If the first character of the first card
        // appears after 12GB of whitespace, we'll find it.
        loop {
            match self.state {
                CardParseState::Initial => {
                    let Some(r) = self.source.next() else {
                        self.state = CardParseState::Done;
                        continue;
                    };
                    match r {
                        ' ' | '\t' | '\n' | '\r' => {
                            self.state = CardParseState::Initial;
                        },
                        '1'..='9' | 'T' | 'J' | 'C' | 'Q' | 'K' | 'A' => {
                            self.state = CardParseState::OneChar(r);
                        },
                        '[' => {
                            self.state = CardParseState::PreCard;
                        },
                        _ => {
                            self.state = CardParseState::Done;
                        },
                    }
                },
                CardParseState::PreCard => {
                    let Some(r) = self.source.next() else {
                        self.state = CardParseState::Done;
                        continue;
                    };
                    match r {
                        ' ' | '\t' | '\n' | '\r' => {
                            self.state = CardParseState::PreCard;
                        },
                        '1'..='9' | 'T' | 'J' | 'C' | 'Q' | 'K' | 'A' => {
                            self.state = CardParseState::OneChar(r);
                        },
                        _ => {
                            self.state = CardParseState::Done;
                        },
                    }
                },
                CardParseState::OneChar(r) => {
                    if let Some(s) = self.source.next() {
                        if '1' == r && '0' == s {
                            self.state = CardParseState::OneChar('T');
                        } else {
                            self.state = CardParseState::TwoChars(r, s);
                        }
                    } else {
                        self.state = CardParseState::Done;
                    }
                },
                CardParseState::TwoChars(r, s) => {
                    self.state = CardParseState::PreCard;

                    if 'J' == r && 'k' == s { return Some(JOKER); }
                    if 'J' == r && 'b' == s { return Some(BLACK_JOKER); }
                    if 'J' == r && 'w' == s { return Some(WHITE_JOKER); }

                    let card = Card::from_rank_suit(
                        Rank::from_char(r), Suit::from_char(s));
                    if card.is_card() {
                        return Some(card);
                    }
                    self.state = CardParseState::Done;
                },
                CardParseState::Done => {
                    return None;
                },
            }
        }
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/card_parse) | Convert text to iterator of [Card]s
///
/// Simple state-machine parser for strings that contain cards in "As" form.
/// We accept a few extras here above the standard: "Jk", "Jb", and "Jw" for the
/// red, black, and white jokers, respectively; "10" for the ten as well as "T",
/// and "1" for low ace (should only be used for testing).
///
/// ```rust
/// use onejoker::prelude::*;
///
/// let text = "  [2h 3s Td Kc Jk 7c Ad]";
/// let cards: Vec<Card> = card_parse(text).collect();
/// assert_eq!(cards.len(), 7);
/// assert_eq!(cards[0], DEUCE_OF_HEARTS);
/// assert_eq!(cards[4], JOKER);
/// assert_eq!(cards[6], ACE_OF_DIAMONDS);
/// ```
pub fn card_parse(text: &str) -> impl Iterator<Item = Card> + '_ {
    CardParseIter::new(text.chars())
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use crate::utils::oj_rand_range;
    use crate::cards::*;

    #[test]
    fn test_cards() -> Result<()> {
        let mut deck = Deck::new(DeckType::TwoJokers);

        for _ in 0..100 {
            let len = 1 + oj_rand_range(4) +
                oj_rand_range(4) + oj_rand_range(4);

            deck.refill_and_shuffle();
            let h = deck.new_hand().init(deck.draw(len));

            let text = h.to_string();
            let h2 = deck.new_hand().init(card_parse(&text));

            assert!(h.equals(&h2));
        }
        let deck = Deck::new(DeckType::AllCards);
        let h1: Hand = deck.new_hand().init(
            card_parse("  [2h 10s TdKcJb7cCsAd]"));
        let h2: Hand = deck.new_hand().init([
            DEUCE_OF_HEARTS, TEN_OF_SPADES, TEN_OF_DIAMONDS, KING_OF_CLUBS,
            BLACK_JOKER, SEVEN_OF_CLUBS, KNIGHT_OF_SPADES, ACE_OF_DIAMONDS
        ]);
        assert!(h1.equals(&h2));
        Ok(())
    }
}
