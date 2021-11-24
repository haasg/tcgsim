use std::collections::VecDeque;

use crate::player::Player;
use crate::board::Board;
use crate::deck::Deck;
use crate::card_manager::CardManager;

pub struct Game {
    pub card_manager: CardManager,
    pub player1: Option<Player>, 
    pub player2: Option<Player>,
    pub game_board: Option<Board>,  
}

impl Game {

    pub fn new(card_manager: CardManager) -> Self {
        Self {
            card_manager,
            player1: None, 
            player2: None,
            game_board: None,
        }
    }
    pub fn init_new_test_game(&mut self) {
        println!("Initalizing Player 1 Deck");
        let mut deck1 = Deck {card_deque: VecDeque::new()};
        deck1.init_test_deck1(&self.card_manager);
        self.player1 = Some(Player::new(deck1));
        self.player1.as_ref().unwrap().get_deck().print_card_names();

        println!("Initalizing Player 2 Deck");
        let mut deck2 = Deck {card_deque: VecDeque::new()};
        deck2.init_test_deck2(&self.card_manager);
        self.player2 = Some(Player::new(deck2));
        self.player2.as_ref().unwrap().get_deck().print_card_names();

        println!("Shuffling Decks");
        self.player1.as_mut().unwrap().get_deck_mut().shuffle();
        self.player2.as_mut().unwrap().get_deck_mut().shuffle();
        
        println!("Printing Decks Again");
        self.player1.as_ref().unwrap().get_deck().print_card_names();
        self.player2.as_ref().unwrap().get_deck().print_card_names();

        //init board
    }
    
    pub fn start(&self) -> () {
        println!("Game Started");

    }
}
