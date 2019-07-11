use chrono::{Datelike, Timelike, Utc};
#[derive(Default, Debug)]
pub struct Food {
    name: String,
    calories: u32,
    carbs: u32,
    protein: u32,
    fat: u32,
}

impl Food {
    pub fn new() -> Food {
        Food {
            name: String::new(),
            ..Default::default()
        }
    }

    pub fn set_name(&mut self, new_name: &String) {
        self.name = new_name.to_string();

    }
    pub fn set_calories(&mut self, new_cal: u32) {
        self.calories = new_cal;
    }
    pub fn set_carbs(&mut self, new_carbs: u32) {
        self.carbs = new_carbs;
    }
    pub fn set_protein(&mut self, new_prot: u32) {
        self.protein = new_prot;
    }
    pub fn set_fat(&mut self, new_fat: u32) {
        self.fat = new_fat;
    }
    pub fn finalize(&mut self) {
        let new_total = 4 * self.protein + 4 * self.carbs + 9 * self.fat;
        self.set_calories(new_total);
    }
    //TODO: finish
}

impl std::fmt::Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let now = Utc::now();
        let (_, year) = now.year_ce();
        write!(
            f,
            "
        ********************\n
        Date: {}-{:02}-{:02} {:?} \n
        Name: {} \n
        Calories: {} \n
        Carbs: {} g\n
        Protein: {} g\n
        Fat: {} g\n
        ********************\n",
            year,
            now.month(),
            now.day(),
            now.weekday(),
            self.name,
            self.calories,
            self.carbs,
            self.protein,
            self.fat
        )
    }
}
