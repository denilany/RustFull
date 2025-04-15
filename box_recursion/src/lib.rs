#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

// Define Link as an Option<Box<Worker>> to allow recursion
pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    // Initializes a new WorkEnvironment with no workers
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    // Adds a worker to the start of the list
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker {
            role,
            name,
            next: self.grade.take(),
        });
        self.grade = Some(new_worker);
    }

    // Removes the most recently added worker and returns their name
    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(worker) = self.grade.take() {
            self.grade = worker.next;
            Some(worker.name)
        } else {
            None
        }
    }

    // Returns the last added worker’s name and role
    pub fn last_worker(&self) -> Option<(String, String)> {
        if let Some(worker) = &self.grade {
            Some((worker.name.clone(), worker.role.clone()))
        } else {
            None
        }
    }
}
