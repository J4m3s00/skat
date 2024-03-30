use std::convert::TryFrom;

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
