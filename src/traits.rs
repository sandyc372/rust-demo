pub Trait Hibernate {
    pub fn enter_hibernation(&self) {
        println!("Entering hibernation");
    }
    pub fn leave_hibernation(&self) {
        println!("leaving hibernation");
    }
}

struct Animal {
    age: u32
}