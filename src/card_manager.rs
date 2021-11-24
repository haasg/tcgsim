use std::collections::HashMap;

use crate::card::Card;

pub struct CardManager {
    pub card_storage: HashMap<String, Card>,
}

impl CardManager {

    pub fn add_card(&mut self, name: &str, cost: i32, attack: i32, health: i32) -> () {
        let card = Card { 
            name: name.to_string(),
            cost,
            attack,
            health,
        };
        self.card_storage.insert(name.to_string(), card);
    }

    pub fn get_card(&self, name: &str) -> Card {
        let stored_card_ref_option = self.card_storage.get(&name.to_string());
        let stored_card_ref = stored_card_ref_option.unwrap();
        let stored_card_copy = (*stored_card_ref).clone();
        return stored_card_copy;
    }

    pub fn init(&mut self) -> () {
        self.card_storage.clear();
        self.add_card("Elven Archer", 1, 1, 1);
        self.add_card("Goldshire Footman", 1, 1, 2);
        self.add_card("Grimscale Oracle", 1, 1, 1);
        self.add_card("Murloc Raider", 1, 2, 1);
        self.add_card("Stonetusk Boar", 1, 1, 1);
        self.add_card("Voodoo", 1, 2, 1);
        self.add_card("Acidic Swamp Ooze", 2, 3, 2);
        self.add_card("Bloodfen Raptor", 2, 3, 2);
        self.add_card("Bluegill Warrior", 2, 2, 1);
        self.add_card("Frostwolf Grunt", 2, 2, 2);
        self.add_card("Kobold Geomancer", 2, 2, 2);
        self.add_card("Morloc Tidehunter", 2, 2, 1);
        self.add_card("Novice Engineer", 2, 1, 1);
        self.add_card("River Crocolisk", 2, 2, 3);
        self.add_card("Dalaran Mage", 3, 1, 4);
        self.add_card("Ironforge Rifleman", 3, 2, 2);
        self.add_card("Ironfur Grizzly", 3, 3, 3);
        self.add_card("Magma Rager", 3, 5, 1);
        self.add_card("Raid Leader", 3, 2, 3);
        self.add_card("Razorfen Hunter", 3, 2, 3);
        self.add_card("Shattered Sun Cleric", 3, 3, 2);
        self.add_card("Silverback Patriarch", 3, 1, 4);
        self.add_card("Wolfrider", 3, 3, 1);
        self.add_card("Chillwind Yeti", 4, 4, 5);
        self.add_card("Dragonling Mechanic", 4, 2, 4);
        self.add_card("Gnomish Inventor", 4, 2, 4);
        self.add_card("Oasis Snapjaw", 4, 2, 7);
        self.add_card("Ogre Magi", 4, 4, 4);
        self.add_card("Sen'jin Shieldmasta", 4, 3, 5);
        self.add_card("Stormwind Knight", 4, 2, 5);
        self.add_card("Booty Bay Bodyguard", 5, 5, 4);
        self.add_card("Darkscale Healer", 5, 4, 5);
        self.add_card("Frostwolf Warlord", 5, 4, 4);
        self.add_card("Gurubashi Berserker", 5, 2, 8);
        self.add_card("Nightblade", 5, 4, 4);
        self.add_card("Stormpike Commando", 5, 4, 2);
        self.add_card("Archmage", 6, 4, 7);
        self.add_card("Boulderfist Ogre", 6, 6, 7);
        self.add_card("Lord of the Arena", 6, 6, 5);
        self.add_card("Rockless Rocketeer", 6, 5, 2);
        self.add_card("Core Hound", 7, 9, 5);
        self.add_card("Stormwind Champion", 7, 7, 7);
        self.add_card("War Golem", 7, 7, 7);
    }
    
}