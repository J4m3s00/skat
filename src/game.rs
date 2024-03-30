use crate::{
    card::{get_shuffled_skat_deck, Card, CardValue},
    player::Player,
};

pub struct Game {
    players: [Player; 3],
    skat: [Card; 2],
}

impl Game {
    /// Generate a new game and give first round of cards
    pub fn new() -> Self {
        let mut res = Self {
            players: Default::default(),
            skat: [
                Card::new(CardValue::new_unchecked(1), crate::card::CardColor::Clubs),
                Card::new(CardValue::new_unchecked(1), crate::card::CardColor::Diamond),
            ],
        };
        res.reset_round();
        res
    }

    pub fn reset_round(&mut self) {
        self.players.iter_mut().for_each(Player::clear_hand);

        let mut deck = get_shuffled_skat_deck().to_vec();

        // Give everyone cards
        // We will for fun use the rules of the game
        self.players[0].give_cards(deck.drain(..3));
        self.players[1].give_cards(deck.drain(..3));
        self.players[2].give_cards(deck.drain(..3));

        self.skat = [deck.remove(0), deck.remove(0)];

        self.players[0].give_cards(deck.drain(..4));
        self.players[1].give_cards(deck.drain(..4));
        self.players[2].give_cards(deck.drain(..4));

        self.players[0].give_cards(deck.drain(..3));
        self.players[1].give_cards(deck.drain(..3));
        self.players[2].give_cards(deck.drain(..3));
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn start_game() {
        Game::new(); // Should just not crash
    }
}
