use super::person::*;

pub struct Profile {
    profile: Person,
    is_metric: bool,
    //TODO: add metadata
}
impl Profile {
    pub fn create_profile() {
        let mut new_obj = Person::new();
        let mut input = String::with_capacity(64);
        print!("{}[2J{}[;H", 27 as char, 27 as char);
        println!("Enter your name: ");
        std::io::stdin().read_line(&mut input).expect("Failed to read input!");
        new_obj.set_name(input.trim().to_string());
        input.clear();

        println!("Enter your age: ");
        std::io::stdin().read_line(&mut input).expect("Failed to read input!");
        new_obj.set_age(input.trim().parse().unwrap());
        
        println!("Enter your gender: ");//TODO: make gender parse system
        std::io::stdin().read_line(&mut input).expect("Failed to read input!");
        new_obj.set_gender(input.trim().parse().unwrap());

        println!("Enter your mass: ");
        std::io::stdin().read_line(&mut input).expect("Failed to read input!");
        new_obj.set_mass(input.trim().parse().unwrap());
        println!("Enter your height: ");
        std::io::stdin().read_line(&mut input).expect("Failed to read input!");
        new_obj.set_height(input.trim().parse().unwrap());

        
        //TODO:validate user input
        //TODO:save data to file
        unimplemented!();
    }

    pub fn load_profile() {
        unimplemented!();
    }
    pub fn save_profile() {
        //TODO: call internal save function
        unimplemented!();
    }

    fn save(self) {
        unimplemented!();
    }
}
