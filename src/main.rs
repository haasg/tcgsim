pub mod deck;
pub mod card;
pub mod card_manager;
pub mod game;
pub mod hand;
pub mod player;
pub mod board;

extern crate rand;

use std::collections::HashMap;
use crate::card_manager::CardManager;
use crate::game::Game;

fn main() {
    println!("Hello, world!");

    let mut card_manager = CardManager {card_storage: HashMap::new()};
    card_manager.init();

    let mut game = Game::new(card_manager);
    game.init_new_test_game();
    game.start();
    // let new_card = card_manager.get_card("Bloodfen Raptor");
    // new_card.print_card_fancy();

    

    println!("Goodbye, world!");
}