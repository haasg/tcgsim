use crate::deck::Deck;
use crate::hand::Hand;

pub struct Player {
    pub deck: Deck,
    pub hand: Option<Hand>,
    pub health: i32,
    pub mana: i32,
}
impl Player {

    pub fn new(deck: Deck) -> Self {
        Self {
            deck,
            hand: None,
            health: 30,
            mana: 0,
        }
    }

    pub fn get_deck(&self) -> &Deck {
        &self.deck
    }

    pub fn get_deck_mut(&mut self) -> &mut Deck {
        &mut self.deck
    }

    pub fn set_health(&mut self, health: i32) -> () {
        self.health = health;
    }
}