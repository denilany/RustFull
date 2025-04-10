// Define an enum `Role` representing different roles in a mafia organization
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Soldier,
    Caporegime,
    Associate,
}

// Define the `Member` struct, which represents a member of the mafia organization
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

impl Member {
    // Method to promote a member to the next higher role
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => self.role = Role::Soldier,
            Role::Soldier => self.role = Role::Caporegime,
            Role::Caporegime => self.role = Role::Underboss,
            _ => {}
        }
    }

    // Constructor method to create a new instance of `Member`
    pub fn new(name: &str, role: Role, age: u8) -> Self {
        Self {
            name: name.to_string(),
            role,
            age,
        }
    }
}