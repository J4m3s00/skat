use crate::card::Card;

pub struct Player {
    hand: Vec<Card>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hand: Vec::with_capacity(10),
        }
    }
}

impl Player {
    pub fn give_card(&mut self, card: Card) {
        // Should we check for the num of cards on the hand?
        // Maybe return if we have 10 cards. Could be useless though
        // Maybe want to have special modies with more cards on the hand
        self.hand.push(card);
    }

    pub fn give_cards(&mut self, cards: impl Iterator<Item = Card>) {
        for c in cards {
            self.give_card(c);
        }
    }

    /// Removes card from the hand and returns it
    /// # Panics
    /// Panics if `card_index` is out of bounds
    pub fn play_card(&mut self, card_index: usize) -> Card {
        assert!(
            card_index < self.hand.len(),
            "Hand does not have enough cards to play {}",
            card_index
        );
        self.hand.remove(card_index)
    }

    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }
}
