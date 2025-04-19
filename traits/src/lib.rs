use std::fmt;
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub strength: f64,
    pub score: i32,
    pub money: i32,
    pub weapons: Vec<String>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

impl Player {
    pub fn eat<T: Food>(&mut self, food: T) {
        // Simply add the food's strength contribution
        self.strength += food.gives();
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Strength: {}, Score: {}, Money: {}", 
            self.strength, self.score, self.money)?;
        write!(f, "Weapons: {:?}", self.weapons)
    }
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        // Each kg of fruit gives 4 units of strength
        self.weight_in_kg * 4.0
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        // Calculate protein content (weight minus fat)
        let protein = self.weight_in_kg - (self.weight_in_kg * self.fat_content);
        let fat = self.weight_in_kg * self.fat_content;
        
        // Protein gives 4 units per kg, fat gives 9 units per kg
        protein * 4.0 + fat * 9.0
    }
}