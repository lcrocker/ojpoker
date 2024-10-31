//! [wiki](https://github.com/lcrocker/ojpoker/wiki/parse_cards) | A function for reading cards from text.

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
                        '1'..='9' | 'T' | 'J' | 'Q' | 'K' | 'A' => {
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
                        '1'..='9' | 'T' | 'J' | 'Q' | 'K' | 'A' => {
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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/parse_cards) | A function for reading cards from text.
pub fn parse_cards(text: &str) -> impl Iterator<Item = Card> + '_ {
    CardParseIter::new(text.chars())
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::deck::Deck;
    use crate::errors::OjError;
    use crate::utils::oj_rand_range;
 
    #[test]
    fn test_cards() -> Result<(), OjError> {
        let mut deck = Deck::new("allcards");

        for _ in 0..1000 {
            let len = 1 + oj_rand_range(4) +
                oj_rand_range(4) + oj_rand_range(4);

            deck.refill_shuffled();
            let h = deck.new_hand().init(deck.draw(len));

            let text = h.to_string();
            let mut h2 = deck.new_hand();

            h2.push_n(parse_cards(&text));
            assert!(h.equals(&h2));
        }
        Ok(())
    }
}
