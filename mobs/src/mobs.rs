pub mod boss;
pub mod member;

// Importing `Boss`, `Member`, and `Role` from the respective modules
use boss::Boss;
use member::{Member, Role};

// Define the `Mob` struct representing a mafia organization
#[derive(Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    // Method to recruit a new member into the mob
    pub fn recruit(&mut self, member_name: &str, member_age: u8) {
        self.members
            .push(Member::new(member_name, Role::Associate, member_age));
    }

    // Method to initiate an attack on another mob
    pub fn attack(&mut self, target: &mut Mob) {
        if calculate_power(self) > calculate_power(target) {
            target.members.pop();
        } else {
            self.members.pop();
        }

        // If the current mob's members are all gone, transfer cities and wealth to the target mob
        if self.members.is_empty() {
            switch_cities(target, self);
            target.wealth += self.wealth;
            self.cities = vec![];
            self.wealth = 0;
        } else if target.members.is_empty() {
            switch_cities(self, target);
            self.wealth += target.wealth;
            target.cities = vec![];
            target.wealth = 0;
        }
    }

    // Method to steal wealth from another mob
    pub fn steal(&mut self, target: &mut Mob, value: u32) {
        if target.wealth >= value {
            self.wealth += value;
            target.wealth -= value;
        } else {
            self.wealth += target.wealth;
            target.wealth = 0;
        }
    }

    // Method to conquer a city from other mobs
    pub fn conquer_city(&mut self, mobs: Vec<Mob>, wanted_city: String, value: u8) {
        if !mobs
            .into_iter()
            .any(|ele| ele.cities.iter().any(|(city, _)| city == &wanted_city))
        {
            self.cities.push((wanted_city, value));
        }
    }
}

// Helper function to calculate the "power" of a mob based on its members' roles
fn calculate_power(mob: &Mob) -> usize {
    let mut result = 0;
    for member in &mob.members {
        match member.role {
            Role::Underboss => result += 4,
            Role::Caporegime => result += 3,
            Role::Soldier => result += 2,
            Role::Associate => result += 1,
        }
    }
    result
}

// Helper function to transfer cities from one mob to another
fn switch_cities(winner: &mut Mob, loser: &Mob) {
    for city in &loser.cities {
        winner.cities.push(city.clone());
    }
}