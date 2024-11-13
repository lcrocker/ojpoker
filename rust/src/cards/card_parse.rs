//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Parsing_Cards) | A function for reading cards from text.

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
                        self.state = CardParseState::TwoChars(r, s);
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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojc_parse) | A function for reading cards from text
/// ```rust
/// use onejoker::*;
///
/// let text = "  [2h 3s Td Kc Jk 7c Ad]";
/// let cards: Vec<Card> = ojc_parse(text).collect();
/// assert_eq!(cards.len(), 7);
/// assert_eq!(cards[0], card!("2h"));
/// assert_eq!(cards[4], card!("Jk"));
/// assert_eq!(cards[6], card!("Ad"));
/// ```
pub fn ojc_parse(text: &str) -> impl Iterator<Item = Card> + '_ {
    CardParseIter::new(text.chars())
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::OjError;
    use crate::utils::oj_rand_range;
    use crate::cards::{Deck, DeckType};

    #[test]
    fn test_cards() -> Result<(), OjError> {
        let mut deck = Deck::new(DeckType::AllCards);

        for _ in 0..1000 {
            let len = 1 + oj_rand_range(4) +
                oj_rand_range(4) + oj_rand_range(4);

            deck.refill_and_shuffle();
            let h = deck.new_hand().init(deck.draw(len));

            let text = h.to_string();
            let h2 = deck.new_hand().init(ojc_parse(&text));

            assert!(h.equals(&h2));
        }
        Ok(())
    }
}
