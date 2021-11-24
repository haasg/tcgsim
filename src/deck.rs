

use std::collections::VecDeque;
use crate::{card::Card, card_manager::CardManager};


pub struct Deck {
    pub card_deque: VecDeque<Card>,
}
impl Deck {

    pub fn push_card(&mut self, card: Card) -> () {
        self.card_deque.push_back(card);
    }

    pub fn print_card_details(&self) -> () {
        for card in self.card_deque.iter() {
            card.print_card_details();
        }
    }

    pub fn print_card_names(&self) -> () {
        for card in self.card_deque.iter() {
            card.print_card_name();
        }
    }

    pub fn shuffle(&mut self) -> () {
        shuffle(&mut self.card_deque, rand::thread_rng());
    }

    pub fn init_test_deck1(&mut self, card_manager: &CardManager) -> () {
        self.card_deque.push_back(card_manager.get_card("Elven Archer"));
        self.card_deque.push_back(card_manager.get_card("Raid Leader"));
        self.card_deque.push_back(card_manager.get_card("Ogre Magi"));
        self.card_deque.push_back(card_manager.get_card("Bloodfen Raptor"));
        self.card_deque.push_back(card_manager.get_card("Nightblade"));
    }

    pub fn init_test_deck2(&mut self, card_manager: &CardManager) -> () {
        self.card_deque.push_back(card_manager.get_card("Boulderfist Ogre"));
        self.card_deque.push_back(card_manager.get_card("Frostwolf Warlord"));
        self.card_deque.push_back(card_manager.get_card("Dalaran Mage"));
        self.card_deque.push_back(card_manager.get_card("Voodoo"));
        self.card_deque.push_back(card_manager.get_card("Ironfur Grizzly"));
    }
}



trait LenAndSwap {
    fn len(&self) -> usize;
    fn swap(&mut self, i: usize, j: usize);
}

// An exact copy of rand::Rng::shuffle, with the signature modified to
// accept any type that implements LenAndSwap
fn shuffle<T, R>(values: &mut T, mut rng: R)
    where T: LenAndSwap,
          R: rand::Rng {
    let mut i = values.len();
    while i >= 2 {
        // invariant: elements with index >= i have been locked in place.
        i -= 1;
        // lock element i in place.
        values.swap(i, rng.gen_range(0, i + 1));
    }
}

// VecDeque trivially fulfills the LenAndSwap requirement, but
// we have to spell it out.
impl<T> LenAndSwap for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.swap(i, j)
    }
}
