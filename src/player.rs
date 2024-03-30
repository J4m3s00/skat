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

    pub fn play_card(&mut self, card_index: usize) -> Card {
        assert!(
            card_index < self.hand.len(),
            "Hand does not have enough cards to play {}",
            card_index
        );
        self.hand.remove(card_index)
    }
}
