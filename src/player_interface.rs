use crate::card::Card;

pub trait PlayerInterface {
    /// Get bid value of the hand
    fn get_bid_value(&self, hand: &[Card; 10]) -> u8;
    /// Decide which card to play.
    ///
    /// Return the index of the hand to play
    fn make_move(&self, hand: &[Card], table: &[Card]) -> usize;
}
