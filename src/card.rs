use rand::{seq::SliceRandom, thread_rng};
use std::convert::TryFrom;

fn get_skat_deck() -> [Card; 32] {
    [
        Card::new(CardValue::new_unchecked(1), CardColor::Clubs),
        Card::new(CardValue::new_unchecked(7), CardColor::Clubs),
        Card::new(CardValue::new_unchecked(8), CardColor::Clubs),
        Card::new(CardValue::new_unchecked(9), CardColor::Clubs),
        Card::new(CardValue::new_unchecked(10), CardColor::Clubs),
        Card::new(CardValue::new_unchecked(11), CardColor::Clubs),
        Card::new(CardValue::new_unchecked(12), CardColor::Clubs),
        Card::new(CardValue::new_unchecked(13), CardColor::Clubs),
        Card::new(CardValue::new_unchecked(1), CardColor::Spades),
        Card::new(CardValue::new_unchecked(7), CardColor::Spades),
        Card::new(CardValue::new_unchecked(8), CardColor::Spades),
        Card::new(CardValue::new_unchecked(9), CardColor::Spades),
        Card::new(CardValue::new_unchecked(10), CardColor::Spades),
        Card::new(CardValue::new_unchecked(11), CardColor::Spades),
        Card::new(CardValue::new_unchecked(12), CardColor::Spades),
        Card::new(CardValue::new_unchecked(13), CardColor::Spades),
        Card::new(CardValue::new_unchecked(1), CardColor::Heart),
        Card::new(CardValue::new_unchecked(7), CardColor::Heart),
        Card::new(CardValue::new_unchecked(8), CardColor::Heart),
        Card::new(CardValue::new_unchecked(9), CardColor::Heart),
        Card::new(CardValue::new_unchecked(10), CardColor::Heart),
        Card::new(CardValue::new_unchecked(11), CardColor::Heart),
        Card::new(CardValue::new_unchecked(12), CardColor::Heart),
        Card::new(CardValue::new_unchecked(13), CardColor::Heart),
        Card::new(CardValue::new_unchecked(1), CardColor::Diamond),
        Card::new(CardValue::new_unchecked(7), CardColor::Diamond),
        Card::new(CardValue::new_unchecked(8), CardColor::Diamond),
        Card::new(CardValue::new_unchecked(9), CardColor::Diamond),
        Card::new(CardValue::new_unchecked(10), CardColor::Diamond),
        Card::new(CardValue::new_unchecked(11), CardColor::Diamond),
        Card::new(CardValue::new_unchecked(12), CardColor::Diamond),
        Card::new(CardValue::new_unchecked(13), CardColor::Diamond),
    ]
}

fn get_shuffled_skat_deck() -> [Card; 32] {
    let mut rng = thread_rng();
    let mut deck = get_skat_deck();
    deck.shuffle(&mut rng);
    deck
}

pub struct Card {
    value: CardValue,
    color: CardColor,
}

impl Card {
    pub fn new(value: CardValue, color: CardColor) -> Self {
        Self { value, color }
    }
}

pub enum CardColor {
    Clubs,
    Diamond,
    Heart,
    Spades,
}

/// Possible values:
/// - 1 -> Ace
/// - 2-10 -> Number
/// - 11 -> Jack
/// - 12 -> Queen
/// - 13 -> King
pub struct CardValue(u8);

impl CardValue {
    pub fn new(value: u8) -> Option<CardValue> {
        (1..=13).contains(&value).then_some(Self(value))
    }

    /// Caller has to make sure that 1 <= value <= 13
    pub fn new_unchecked(value: u8) -> CardValue {
        Self(value)
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for CardValue {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use crate::card::CardValue;

    #[test]
    fn card_value() {
        assert!(CardValue::new(0).is_none());
        assert!(CardValue::new(14).is_none());
        assert!(CardValue::new(1).is_some());
        assert_eq!(CardValue::new(5).unwrap().inner(), 5);
    }
}
