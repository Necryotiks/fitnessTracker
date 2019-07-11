use super::foods::*;
use super::utils::*;

#[derive(Default)]
pub struct Person {
    name: String,
    age: u32,
    tdee: u32,
    gender: Gender,
    mass: u32,
    height: u32,
    remaining_cal: u32,
    act_lvl: ActivityLevel,
    daily_meals: Vec<Food>,
}

pub enum Gender {
    Male,
    Female,
}

pub enum ActivityLevel {
    Sedentary,
    Light,
    Moderate,
    Heavy,
    Extreme,
}

impl Person {
    pub fn new() -> Person {
        Person {
            name: String::new(),
            ..Default::default()
        }
    }
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
    fn set_tdee(&mut self, new_tdee: u32) {
        self.tdee = new_tdee;
    }
    pub fn set_gender(&mut self, new_gender: String) {
        let mut new_gender = new_gender;
        new_gender.make_ascii_lowercase();
        match new_gender.as_ref() {
            "male" => self.gender = Gender::Male,
            "female" => self.gender = Gender::Female,
            _ => panic!(),

        }
    }
    pub fn set_mass(&mut self, new_mass: u32) {
        self.mass = new_mass;
    }
    pub fn set_height(&mut self, new_height: u32) {
        self.height = new_height
    }
    fn set_remaining_cal(&mut self, new_rc: u32) {
        self.remaining_cal = new_rc;
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_age(&self) -> u32 {
        self.age
    }
    pub fn get_tdee(&self) -> u32 {
        self.tdee
    }
    pub fn get_gender(&self) -> &Gender {
        &self.gender
    }
    pub fn get_mass(&self) -> u32 {
        self.mass
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_remaining_cals(&self) -> u32 {
        self.remaining_cal
    }
    pub fn get_act_lvl(&self) -> &ActivityLevel {
        &self.act_lvl
    }
    pub fn get_daily_meals(&self) -> &Vec<Food> {
        &self.daily_meals
    }
    pub fn calc_tdee(&mut self) {
        match self.get_gender() {
            Gender::Male => {
                let bmr = 13.397 * (self.get_mass() as f32) + 4.799 * (self.get_height() as f32) -
                    5.667 * (self.get_age() as f32) + 88.362;
                match self.get_act_lvl() {
                    ActivityLevel::Sedentary => self.set_tdee((bmr * 1.2) as u32),
                    ActivityLevel::Light => self.set_tdee((bmr * 1.375) as u32),
                    ActivityLevel::Moderate => self.set_tdee((bmr * 1.55) as u32),
                    ActivityLevel::Heavy => self.set_tdee((bmr * 1.725) as u32),
                    ActivityLevel::Extreme => self.set_tdee((bmr * 1.9) as u32),
                };
            }
            Gender::Female => {
                let bmr = 9.247 * (self.get_mass() as f32) + 3.098 * (self.get_height() as f32) -
                    4.370 * (self.get_age() as f32) + 447.593;
                match self.get_act_lvl() {
                    ActivityLevel::Sedentary => self.set_tdee((bmr * 1.2) as u32),
                    ActivityLevel::Light => self.set_tdee((bmr * 1.375) as u32),
                    ActivityLevel::Moderate => self.set_tdee((bmr * 1.55) as u32),
                    ActivityLevel::Heavy => self.set_tdee((bmr * 1.725) as u32),
                    ActivityLevel::Extreme => self.set_tdee((bmr * 1.9) as u32),
                };
            }
        };
    }
    fn read_usr_input(&mut self,new_item:&mut Food)
    {
                let mut input = String::new();
                println!("Enter number of grams of carbs: ");
                std::io::stdin().read_line(&mut input).expect("Failed to read input!");
                new_item.set_carbs(input.trim().parse().unwrap());
                input.clear();

                println!("Enter number of grams of protein: ");
                std::io::stdin().read_line(&mut input).expect("Failed to read input!");
                new_item.set_protein(input.trim().parse().unwrap());
                input.clear();

                println!("Enter number of grams of fat: ");
                std::io::stdin().read_line(&mut input).expect("Failed to read input!");
                new_item.set_fat(input.trim().parse().unwrap());
                input.clear();
    }
    pub fn add_meal_entry(&mut self) {
        let mut new_item = Food::new();
        let mut input = String::with_capacity(32);

        //Clear screen with ANSI commands
        print!("{}[2J{}[;H", 27 as char, 27 as char);
        println!("Enter the name of the food item: ");
        std::io::stdin().read_line(&mut input).expect("Failed to read input!");
        new_item.set_name(&input.trim().to_string());
                input.clear();

        println!("Is nutritional information already provided? [Y/N]");
        loop  {
        std::io::stdin().read_line(&mut input).expect("Failed to read input!");
        input.make_ascii_lowercase();
        if input.trim().len() == 1 {
            match input.trim(){
                "y" => break,
                "n" => break,
                _ => input.clear(),
            };
        }
        else
        {
            input.clear();
            ();
        }
        };
        match input.trim() {
            "y" => {
                input.clear();
                println!("Enter caloric information:");
                std::io::stdin().read_line(&mut input).expect("Failed to read input!");
                new_item.set_calories(input.trim().parse().unwrap());
                self.read_usr_input(&mut new_item);
                self.daily_meals.push(new_item);
                println!("Entry saved!");
                wait_for_key_press();
            }
            "n" => {
                self.read_usr_input(&mut new_item);
                new_item.finalize();
                self.daily_meals.push(new_item);
                self.print_meals();
                wait_for_key_press();
                //TODO:refactor this

            }
            _ => panic!(), //TODO: make this a do-while loop.

        };
    }
    pub fn remove_meal_entry(&mut self) {
        self.daily_meals.pop();
        //Clear screen with ANSI commands
        print!("{}[2J{}[;H", 27 as char, 27 as char);
        println!("Meal entry deleted!\n");
        wait_for_key_press();

    }
    pub fn print_meals(&self) {
        for item in &self.daily_meals {
            println!("{}", item);
        }
        println!("\n");
    }
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

impl Default for ActivityLevel {
    fn default() -> Self {
        ActivityLevel::Sedentary
    }
}


#[cfg(test)]
mod person_tests {
    use super::*;
    #[test]
    fn test_add_meal() {
        let mut test_obj = Person::new();
        test_obj.add_meal_entry();
        let food_obj = test_obj.get_daily_meals();
        assert_ne!(food_obj.len(), 0);
        //TODO:test that item stored properly
    }
    #[test]
    #[ignore]
    fn test_remove_meal() {
        let mut test_obj = Person::new();
        test_obj.remove_meal_entry();
    }
}
