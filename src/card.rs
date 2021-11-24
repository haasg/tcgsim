
#[derive(Clone)]
pub struct Card {
    pub name: String,
    pub cost: i32,
    pub attack: i32,
    pub health: i32, 
}

impl Card {
    pub fn print_card_details(&self) -> () {
        println!("Name: {}", self.name);
        println!("Cost: {}", self.cost);
        println!("Attack: {}", self.attack);
        println!("Health: {}", self.health);
    }

    pub fn print_card_name(&self) -> () {
        println!("Name: {}", self.name);
    }

    pub fn print_card_fancy(&self) -> () { 
        let mut pad_name = format!("{:<19}", self.name);
        pad_name.truncate(18);
        let slice1 = &pad_name[..6];
        let slice2 = &pad_name[7..13];
        let slice3 = &pad_name[13..19];
        println!(" ______");
        println!("| {0}    |", self.cost);
        println!("|{0}|", slice1);
        println!("|{0}|", slice2);
        println!("|{0}|", slice3);
        println!("|_{0}__{1}_|", self.attack, self.health);
    }

    pub fn attack (&self) -> i32 {
        self.attack
    }
}


